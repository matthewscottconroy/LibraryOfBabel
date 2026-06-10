use std::io::{self, BufRead, Write};
use std::collections::HashMap;

const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn bold(s: &str) -> String { format!("{BOLD}{s}{RESET}") }
fn cyan(s: &str) -> String { format!("{CYAN}{s}{RESET}") }
fn green(s: &str) -> String { format!("{GREEN}{s}{RESET}") }
fn yellow(s: &str) -> String { format!("{YELLOW}{s}{RESET}") }
fn red(s: &str) -> String { format!("{RED}{s}{RESET}") }
fn dim(s: &str) -> String { format!("{DIM}{s}{RESET}") }

// ── Real Analysis in HoTT ─────────────────────────────────────────────────────
//
// Constructive real analysis in Homotopy Type Theory.
// The reals can be constructed as:
//   1. Cauchy completions (Bishop-style)
//   2. Dedekind cuts
//   3. HoTT reals (Auke Booij et al., using both + HITs)
//
// Key result: ℝ_Cauchy ≃ ℝ_Dedekind (assuming countable choice or in HoTT)

// Simple exact rational arithmetic for demonstration
#[derive(Clone, Debug, PartialEq)]
struct Rat {
    num: i64,
    den: i64,
}

impl Rat {
    fn new(n: i64, d: i64) -> Self {
        if d == 0 { panic!("Zero denominator"); }
        let g = gcd(n.abs(), d.abs());
        let sign = if d < 0 { -1 } else { 1 };
        Rat { num: sign * n / g, den: sign * d / g }
    }
    fn zero() -> Self { Rat { num: 0, den: 1 } }
    fn one() -> Self { Rat { num: 1, den: 1 } }
    fn add(&self, other: &Rat) -> Rat { Rat::new(self.num * other.den + other.num * self.den, self.den * other.den) }
    fn sub(&self, other: &Rat) -> Rat { Rat::new(self.num * other.den - other.num * self.den, self.den * other.den) }
    fn mul(&self, other: &Rat) -> Rat { Rat::new(self.num * other.num, self.den * other.den) }
    fn lt(&self, other: &Rat) -> bool { self.num * other.den < other.num * self.den }
    fn display(&self) -> String {
        if self.den == 1 { format!("{}", self.num) }
        else { format!("{}/{}", self.num, self.den) }
    }
}

fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b) } }

fn parse_rat(s: &str) -> Option<Rat> {
    let s = s.trim();
    if let Some(i) = s.find('/') {
        let n: i64 = s[..i].trim().parse().ok()?;
        let d: i64 = s[i+1..].trim().parse().ok()?;
        if d == 0 { return None; }
        Some(Rat::new(n, d))
    } else {
        let n: i64 = s.parse().ok()?;
        Some(Rat::new(n, 1))
    }
}

// Cauchy sequence: approximations a_n with |a_m - a_n| < 1/m + 1/n
#[derive(Clone, Debug)]
struct CauchySeq {
    name: String,
    terms: Vec<Rat>,
    description: String,
}

impl CauchySeq {
    fn display(&self, n: usize) {
        let show: Vec<String> = self.terms.iter().take(n)
            .enumerate()
            .map(|(i, r)| format!("a_{} = {}", i+1, r.display()))
            .collect();
        println!("  {} [{}]:", cyan(&self.name), dim(&self.description));
        println!("  {}", show.join(",  "));
    }

    fn is_cauchy(&self) -> bool {
        for m in 0..self.terms.len() {
            for n in 0..self.terms.len() {
                let diff = self.terms[m].sub(&self.terms[n]);
                let bound = Rat::new(1, (m+1) as i64).add(&Rat::new(1, (n+1) as i64));
                let abs_diff = if diff.num < 0 { Rat::new(-diff.num, diff.den) } else { diff };
                if !abs_diff.lt(&bound) && abs_diff != bound { return false; }
            }
        }
        true
    }
}

fn sqrt2_approx(n: usize) -> Vec<Rat> {
    let mut v = vec![Rat::new(1, 1)];
    let _two = Rat::new(2, 1);
    for _ in 1..n {
        let last = v.last().unwrap().clone();
        let next = Rat::new(last.num + 2 * last.den, 2 * last.num);
        if next.den == 0 { break; }
        v.push(next);
    }
    v
}

fn pi_approx(n: usize) -> Vec<Rat> {
    let mut v = Vec::new();
    let mut sum = Rat::zero();
    for k in 0..n {
        let term = Rat::new(if k % 2 == 0 { 4 } else { -4 }, 2 * k as i64 + 1);
        sum = sum.add(&term);
        v.push(sum.clone());
    }
    v
}

struct Sandbox {
    sequences: HashMap<String, CauchySeq>,
    cuts: HashMap<String, (String, String)>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { sequences: HashMap::new(), cuts: HashMap::new() };
        sb.sequences.insert("sqrt2".into(), CauchySeq {
            name: "sqrt2".into(),
            terms: sqrt2_approx(8),
            description: "Newton's method for √2".into(),
        });
        sb.sequences.insert("pi".into(), CauchySeq {
            name: "pi".into(),
            terms: pi_approx(8),
            description: "Leibniz series for π (slow)".into(),
        });
        sb.cuts.insert("sqrt2".into(),
            ("q : ℚ | q² < 2  ∨  q ≤ 0".into(), "q : ℚ | q² > 2  ∧  q > 0".into()));
        sb
    }

    fn print_help() {
        println!("{}", bold("── Real Analysis in HoTT Sandbox ───────────────────────────────────"));
        println!("  {}  <name>     — show a Cauchy sequence", cyan("seq"));
        println!("  {}  <name> <n> — show first n terms", cyan("terms"));
        println!("  {}  <name>     — check Cauchy condition", cyan("cauchy"));
        println!("  {}  <name>     — show Dedekind cut", cyan("cut"));
        println!("{}", bold("── Constructions of ℝ ────────────────────────────────────────────────"));
        println!("  {}      — Cauchy reals construction", cyan("cauchy-reals"));
        println!("  {}     — Dedekind reals construction", cyan("dedekind-reals"));
        println!("  {}      — HoTT reals (Booij et al.)", cyan("hott-reals"));
        println!("  {}      — equivalence of constructions", cyan("comparison"));
        println!("{}", bold("── Analysis ──────────────────────────────────────────────────────────"));
        println!("  {}  <thm>      — classical theorems in HoTT", cyan("theorem"));
        println!("  {}   — intermediate value theorem", cyan("ivt"));
        println!("  {}   — completeness of ℝ", cyan("completeness"));
        println!("  {}   — constructive issues in analysis", cyan("constructive"));
        println!("  {}   — modalities and analysis", cyan("modalities"));
        println!("  Available sequences: sqrt2, pi");
    }

    fn handle(&mut self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() { return true; }
        if trimmed == "quit" || trimmed == "exit" || trimmed == "q" { return false; }
        if trimmed == "help" || trimmed == "?" { Self::print_help(); return true; }

        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        let cmd = parts[0];
        let rest = if parts.len() > 1 { parts[1].trim() } else { "" };

        match cmd {
            "seq" | "show" => {
                match self.sequences.get(rest) {
                    Some(s) => { let s = s.clone(); s.display(6); }
                    None => println!("  {} Unknown sequence: {rest}. Available: sqrt2, pi", red("✗")),
                }
            }
            "terms" => {
                let args: Vec<&str> = rest.split_whitespace().collect();
                let name = args.first().copied().unwrap_or("sqrt2");
                let n: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(5);
                match self.sequences.get(name) {
                    Some(s) => { let s = s.clone(); s.display(n.min(s.terms.len())); }
                    None => println!("  {} Unknown: {name}", red("✗")),
                }
            }
            "cauchy" => {
                match self.sequences.get(rest) {
                    Some(s) => {
                        let is_c = s.is_cauchy();
                        let s = s.clone();
                        if is_c {
                            println!("  {} {} satisfies the Cauchy condition on stored terms.", green("✓"), s.name);
                        } else {
                            println!("  {} {} does NOT satisfy Cauchy on stored terms.", red("✗"), s.name);
                        }
                        println!("  Condition: |a_m - a_n| < 1/m + 1/n  for all m,n");
                    }
                    None => println!("  {} Unknown sequence: {rest}", red("✗")),
                }
            }
            "cut" => {
                match self.cuts.get(rest) {
                    Some((lower, upper)) => {
                        println!("  Dedekind cut for {}:", cyan(rest));
                        println!("  Lower: {{ {} }}", cyan(lower));
                        println!("  Upper: {{ {} }}", cyan(upper));
                        println!("  A real x = the pair (Lower, Upper) satisfying:");
                        println!("    1. Lower and Upper are inhabited (non-empty)");
                        println!("    2. Lower is downward-closed,  Upper is upward-closed");
                        println!("    3. Lower and Upper are disjoint");
                        println!("    4. Lower and Upper are located: ∀ε>0, ∃q∈Upper, q-ε∈Lower");
                    }
                    None => {
                        println!("  Available cuts: sqrt2");
                        println!("  In general, a Dedekind cut for x is:");
                        println!("    L_x = {{q:ℚ | q < x}},  U_x = {{q:ℚ | q > x}}");
                    }
                }
            }
            "cauchy-reals" => {
                println!("{}", bold("── Cauchy Reals in HoTT ─────────────────────────────────────────────"));
                println!("  ℝ_C = Cauchy sequences of rationals modulo equivalence");
                println!("  (a_n) ~ (b_n)  iff  ∀ε>0, ∃N, ∀n≥N, |a_n - b_n| < ε");
                println!();
                println!("  In HoTT: ℝ_C = quotient type");
                println!("    CauchySeq := {{a : ℕ→ℚ | ∀mn, |a_m-a_n| < 1/m+1/n}}");
                println!("    ℝ_C := CauchySeq / ~");
                println!();
                println!("  Operations defined on representatives:");
                println!("    (a_n) + (b_n) = (a_n + b_n)");
                println!("    (a_n) × (b_n) = (a_n × b_n)");
                println!("    |(a_n)| = (|a_n|)");
                println!();
                println!("  {} ℝ_C is a complete Archimedean ordered field.", green("Theorem:"));
                println!("  Proof requires countable choice (or higher inductive types in HoTT).");
            }
            "dedekind-reals" => {
                println!("{}", bold("── Dedekind Reals in HoTT ───────────────────────────────────────────"));
                println!("  ℝ_D = Dedekind cuts = pairs (L, U) where L,U ⊆ ℚ:");
                println!("    1. L and U are inhabited");
                println!("    2. L is rounded down: q∈L ↔ ∃r>q, r∈L");
                println!("       U is rounded up: q∈U ↔ ∃r<q, r∈U");
                println!("    3. L ∩ U = ∅");
                println!("    4. L ∪ U is dense: ∀p<q:ℚ, p∈L ∨ q∈U");
                println!();
                println!("  In HoTT: Dedekind cuts are a Σ-type with propositional conditions.");
                println!("  Each condition is a proposition ((-1)-truncated), ensuring ℝ_D is a set.");
                println!();
                println!("  {} ℝ_D is a complete Archimedean ordered field.", green("Theorem:"));
                println!("  No countable choice needed — the cut conditions are constructive.");
            }
            "hott-reals" => {
                println!("{}", bold("── HoTT Reals (Booij et al.) ───────────────────────────────────────"));
                println!("  A higher inductive approach combining Cauchy and Dedekind:");
                println!();
                println!("  ℝ is defined as a QIT (quotient inductive type):");
                println!("    rat : ℚ → ℝ  (rationals embed into ℝ)");
                println!("    lim : CauchyApprox → ℝ  (limits of approximations)");
                println!("    eq  : ∀(x y:ℝ)(∀ε>0, |x-y|<ε), x = y  (path constructor)");
                println!();
                println!("  Key feature: ℝ has decidable apartness:");
                println!("    x # y := ∃ε>0, |x-y| > ε");
                println!("  which is stronger than ¬(x=y) in constructive mathematics.");
                println!();
                println!("  Results (Booij 2020):");
                println!("    ℝ_HoTT ≃ ℝ_Cauchy ≃ ℝ_Dedekind  (as sets with field structure)");
                println!("    No choice axiom needed for the equivalence.");
            }
            "comparison" => {
                println!("{}", bold("── Comparing ℝ_Cauchy and ℝ_Dedekind ────────────────────────────────"));
                println!("  Both are complete Archimedean ordered fields.");
                println!("  By the uniqueness theorem: any two such fields are isomorphic.");
                println!();
                println!("  The map ℝ_C → ℝ_D:");
                println!("    [(a_n)] ↦ ({{q:ℚ|∃n,q<a_n}}, {{q:ℚ|∃n,q>a_n}})");
                println!();
                println!("  The map ℝ_D → ℝ_C:");
                println!("    (L,U) ↦ [(q_n)]  where q_n ∈ U with q_n - 1/n ∈ L");
                println!("    (this requires countable choice to define q_n)");
                println!();
                println!("  In HoTT (no AC): ℝ_D is preferred since no choice is needed.");
                println!("  In Booij's HoTT reals: both coincide via the QIT construction.");
            }
            "theorem" => {
                let thm = rest;
                match thm {
                    "ivt" | "intermediate" => {
                        println!("{}", bold("── Intermediate Value Theorem ────────────────────────────────────────"));
                        println!("  Classical IVT: if f:[a,b]→ℝ is continuous and f(a)<0<f(b),");
                        println!("  then ∃x∈(a,b), f(x)=0.");
                        println!();
                        println!("  In constructive math (Bishop): IVT fails in general!");
                        println!("  Counterexample: f uniformly continuous, a=0, b=1,");
                        println!("  we can't always find the root constructively.");
                        println!();
                        println!("  Constructive replacement: if f(a) # 0 and f(b) # 0 (apart from 0)");
                        println!("  and they have opposite signs, then ∃ε, ∃x:f(x) # 0 or f(x)=0.");
                        println!();
                        println!("  With LEM: classical IVT is provable in HoTT.");
                        println!("  Type {} for more on constructive analysis.", cyan("constructive"));
                    }
                    "bolzano-weierstrass" | "bw" => {
                        println!("  Bolzano-Weierstrass: every bounded sequence in ℝ has a convergent subsequence.");
                        println!("  Constructively: fails without countable choice.");
                        println!("  In HoTT with LEM: holds (uses propositional truncation for the limit).");
                    }
                    _ => {
                        println!("  Available theorems: ivt, bolzano-weierstrass (bw)");
                        println!("  Or use {} for an interactive list.", cyan("ivt"));
                    }
                }
            }
            "ivt" => {
                println!("{}", bold("── Intermediate Value Theorem in HoTT ──────────────────────────────"));
                println!("  Classical: if f continuous on [a,b] and f(a)<0<f(b), then ∃c, f(c)=0.");
                println!();
                println!("  In HoTT (Booij 2020):");
                println!("    if f:[a,b]→ℝ uniformly continuous and f has a sign change,");
                println!("    then ‖Σx:ℝ, a≤x≤b ∧ f(x)=0‖  (truncated existence)");
                println!();
                println!("  The truncation is necessary: without LEM, we can't choose the root.");
                println!("  With LEM: ‖∃x, f(x)=0‖ ≃ ∃x, f(x)=0  (props are classical).");
                println!();
                println!("  Proof idea (bisection method):");
                println!("    Split [a,b] into halves; one half has a sign change.");
                println!("    Inductively refine; the limit is a root.");
                println!("    In HoTT: use Cauchy completeness + propositional truncation.");
            }
            "completeness" => {
                println!("{}", bold("── Completeness of ℝ in HoTT ───────────────────────────────────────"));
                println!("  ℝ is Cauchy complete: every Cauchy sequence converges.");
                println!("  Proof: [(a_n)] already is a Cauchy sequence — its limit IS the class.");
                println!();
                println!("  More precisely:");
                println!("    Given a Cauchy sequence (x_n) in ℝ_C = {{a:ℕ→ℚ|Cauchy}}/~");
                println!("    Choose representatives: x_n = [(a^n_k)_k]");
                println!("    The diagonal sequence b_n = a^n_n is Cauchy in ℚ");
                println!("    The limit is [(b_n)] ∈ ℝ_C");
                println!();
                println!("  For ℝ_D: completeness = every Dedekind cut that is the");
                println!("  supremum of a Cauchy sequence actually exists as a real.");
                println!();
                println!("  {} ℝ is the unique complete Archimedean ordered field.", green("Theorem:"));
            }
            "constructive" => {
                println!("{}", bold("── Constructive Analysis in HoTT ───────────────────────────────────"));
                println!("  In HoTT (without LEM or AC), analysis is constructive.");
                println!();
                println!("  What works constructively:");
                println!("  {} Continuous functions are uniformly continuous on compact sets", green("✓"));
                println!("  {} Cauchy sequences have limits (in ℝ, by construction)", green("✓"));
                println!("  {} The reals are Archimedean ordered", green("✓"));
                println!("  {} Basic algebra and calculus", green("✓"));
                println!();
                println!("  What fails without LEM:");
                println!("  {} IVT in existential form (need truncation)", yellow("△"));
                println!("  {} Every bounded sequence has a convergent subsequence", yellow("△"));
                println!("  {} Bolzano-Weierstrass in choosing form", yellow("△"));
                println!();
                println!("  What fails without choice:");
                println!("  {} Tychonoff theorem for infinite products", red("✗"));
                println!("  {} The equivalence ℝ_C ≃ ℝ_D (needs countable choice)", red("✗"));
                println!("    unless using Booij's QIT approach.");
            }
            "modalities" => {
                println!("{}", bold("── Modalities and Real Analysis ────────────────────────────────────"));
                println!("  In Cohesive HoTT (Shulman), there are modalities:");
                println!("    ʃ  = shape / discretization");
                println!("    ♭  = flat / discrete");
                println!("    ♯  = sharp / codiscrete");
                println!();
                println!("  The reals ℝ appear in two ways:");
                println!("    ℝ_disc = discrete reals (points of ℝ as a set)");
                println!("    ℝ_cohesive = ℝ with its smooth/topological cohesion");
                println!();
                println!("  Cohesive HoTT can reason about:");
                println!("    • Differential forms, smooth maps");
                println!("    • De Rham cohomology (synthetically)");
                println!("    • Principal bundles and connections");
                println!("  This is Schreiber's program: differential cohomology in HoTT.");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Real Analysis in HoTT — Interactive Sandbox            ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore constructive real analysis in homotopy type theory.");
    println!("  See Cauchy sequences, Dedekind cuts, and the HoTT reals.");
    println!("  Type {} or {}\n",
        cyan("cauchy-reals"), cyan("seq sqrt2"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}ℝ-HoTT{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
