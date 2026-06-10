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

// ── Induction vs Recursion ─────────────────────────────────────────────────────
//
// Induction: eliminators that produce types (dependent, may return proofs)
// Recursion: eliminators that produce terms (non-dependent, compute values)
//
// Key theme: induction = recursion + uniqueness/coherence
//   Rec_ℕ(z, s) : C   produces a function ℕ → C
//   Ind_ℕ(z, s) : Π(n:ℕ), C(n)  produces a section of a type family
//
// For HITs: there are additional path/higher induction hypotheses.

// Small term language
#[derive(Clone, Debug)]
enum Term {
    Zero,
    Succ(Box<Term>),
    Rec { zero: Box<Term>, succ: Box<Term>, arg: Box<Term> },
    Add(Box<Term>, Box<Term>),
    Mul(Box<Term>, Box<Term>),
    Var(String),
    Num(u64),
}

impl Term {
    fn eval(&self, env: &HashMap<String, u64>) -> Option<u64> {
        match self {
            Term::Zero => Some(0),
            Term::Succ(t) => t.eval(env).map(|n| n + 1),
            Term::Num(n) => Some(*n),
            Term::Var(x) => env.get(x).copied(),
            Term::Add(a, b) => Some(a.eval(env)? + b.eval(env)?),
            Term::Mul(a, b) => Some(a.eval(env)? * b.eval(env)?),
            Term::Rec { zero, succ, arg } => {
                let n = arg.eval(env)?;
                let z = zero.eval(env)?;
                // succ is a term representing the step function as a constant shift
                let s = succ.eval(env)?;
                Some(z + s * n)
            }
        }
    }

    fn display(&self) -> String {
        match self {
            Term::Zero => "zero".into(),
            Term::Succ(t) => format!("succ({})", t.display()),
            Term::Num(n) => n.to_string(),
            Term::Var(x) => x.clone(),
            Term::Add(a, b) => format!("{} + {}", a.display(), b.display()),
            Term::Mul(a, b) => format!("{} × {}", a.display(), b.display()),
            Term::Rec { zero, succ, arg } =>
                format!("rec({},{},{})", zero.display(), succ.display(), arg.display()),
        }
    }
}

fn parse_nat(s: &str) -> Option<u64> { s.trim().parse().ok() }

struct Sandbox {
    history: Vec<(String, String)>,
    vars: HashMap<String, u64>,
}

impl Sandbox {
    fn new() -> Self { Sandbox { history: Vec::new(), vars: HashMap::new() } }

    fn print_help() {
        println!("{}", bold("── Induction vs Recursion Sandbox ──────────────────────────────────"));
        println!("  {}  <n>      — recursion principle for ℕ", cyan("rec-nat"));
        println!("  {}  <n>      — induction principle for ℕ", cyan("ind-nat"));
        println!("  {}  <name>   — define a function by recursion", cyan("define"));
        println!("  {}  <n>      — evaluate last defined function at n", cyan("eval"));
        println!("{}", bold("── Comparison ─────────────────────────────────────────────────────"));
        println!("  {}   — rec vs ind for ℕ in detail", cyan("compare"));
        println!("  {}   — uniqueness of recursive functions", cyan("uniqueness"));
        println!("  {}     — induction for Bool, List, Tree", cyan("other"));
        println!("{}", bold("── Higher Inductive Types ──────────────────────────────────────────"));
        println!("  {}    — S¹ has point + path inductors", cyan("circle-ind"));
        println!("  {}    — path induction for identity types", cyan("path-ind"));
        println!("  {}    — large vs small elimination", cyan("elimination"));
        println!("  {}   — structural vs well-founded recursion", cyan("well-founded"));
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
            "rec-nat" => {
                let n: u64 = parse_nat(rest).unwrap_or(5);
                println!("{}", bold("── Recursion Principle for ℕ ────────────────────────────────────────"));
                println!("  rec_ℕ : C → (C → C) → ℕ → C");
                println!("  rec_ℕ(c, f, zero)   = c");
                println!("  rec_ℕ(c, f, succ n) = f(rec_ℕ(c, f, n))");
                println!();
                println!("  Example — addition:  add(m) = rec_ℕ(m, succ, −)");
                println!("  Target type C = ℕ  (non-dependent)");
                println!();
                let add_n: Vec<String> = (0..=n).map(|k| format!("add({k}) = {}", n + k)).collect();
                println!("  add({n}, k) for k=0..{n}: {}", add_n.join(", "));
                println!();
                println!("  Recursion is a {}.", dim("special case of induction where C is non-dependent"));
            }
            "ind-nat" => {
                let n: u64 = parse_nat(rest).unwrap_or(4);
                println!("{}", bold("── Induction Principle for ℕ ────────────────────────────────────────"));
                println!("  ind_ℕ : (C : ℕ → 𝒰) → C(0) → (Π n:ℕ, C(n) → C(S n)) → Π n:ℕ, C(n)");
                println!("  ind_ℕ(C, c₀, cs, zero)   = c₀");
                println!("  ind_ℕ(C, c₀, cs, succ n) = cs(n, ind_ℕ(C, c₀, cs, n))");
                println!();
                println!("  Example — even: C(n) = (n is even)");
                println!("  c₀ = even_zero : even(0)");
                println!("  cs(n, h) = even_succ_succ : even(n) → even(S(S n))");
                println!();
                for k in 0..=n {
                    let parity = if k % 2 == 0 { "even" } else { "odd" };
                    println!("  C({k}) = is_even({k}) = {}", cyan(parity));
                }
                println!();
                println!("  {} Induction gives proofs of C(n) for each n.", green("Key:"));
                println!("  The induction hypothesis at step n: we have C(n), not just C.");
            }
            "define" => {
                let args: Vec<&str> = rest.splitn(2, ' ').collect();
                let name = args.first().copied().unwrap_or("f");
                match name {
                    "add" => {
                        println!("  Defining add by recursion on first argument:");
                        println!("  add : ℕ → ℕ → ℕ");
                        println!("  add(zero,   m) = m");
                        println!("  add(succ n, m) = succ(add(n, m))");
                        println!("  = rec_ℕ(id, (r,m)↦succ(r(m)), n)(m)  (curried)");
                        self.vars.insert("last_fn".into(), 0);
                        self.history.push(("add".into(), "add(n,m) = n+m".into()));
                    }
                    "mul" => {
                        println!("  Defining mul by recursion:");
                        println!("  mul : ℕ → ℕ → ℕ");
                        println!("  mul(zero,   m) = zero");
                        println!("  mul(succ n, m) = add(mul(n,m), m)");
                        println!("  = rec_ℕ(0, (r,m)↦r+m, n)(m)");
                        self.history.push(("mul".into(), "mul(n,m) = n*m".into()));
                    }
                    "pred" => {
                        println!("  Defining pred by recursion:");
                        println!("  pred : ℕ → ℕ");
                        println!("  pred(zero)   = zero   (truncated at 0)");
                        println!("  pred(succ n) = n");
                        println!("  = rec_ℕ(0, (r,n)↦n, −)  [need to track predecessor]");
                        self.history.push(("pred".into(), "pred(n) = max(n-1,0)".into()));
                    }
                    "fib" => {
                        println!("  Fibonacci by double recursion (via rec on pairs):");
                        println!("  fib_pair : ℕ → ℕ×ℕ");
                        println!("  fib_pair(zero)   = (0, 1)");
                        println!("  fib_pair(succ n) = let (a,b) = fib_pair(n) in (b, a+b)");
                        println!("  fib(n) = fst(fib_pair(n))");
                        let fibs: Vec<u64> = {
                            let mut v = vec![0u64, 1];
                            for i in 2..10 { v.push(v[i-1]+v[i-2]); }
                            v
                        };
                        println!("  fib(0..9) = {:?}", &fibs[..10]);
                        self.history.push(("fib".into(), "fibonacci sequence".into()));
                    }
                    _ => println!("  Available: add, mul, pred, fib"),
                }
            }
            "eval" => {
                let args: Vec<&str> = rest.split_whitespace().collect();
                let fname = args.first().copied().unwrap_or("add");
                let n: u64 = args.get(1).and_then(|s| parse_nat(s)).unwrap_or(5);
                let m: u64 = args.get(2).and_then(|s| parse_nat(s)).unwrap_or(3);
                match fname {
                    "add" => println!("  add({n}, {m}) = {}", n + m),
                    "mul" => println!("  mul({n}, {m}) = {}", n * m),
                    "pred" => println!("  pred({n}) = {}", n.saturating_sub(1)),
                    "fib" => {
                        let mut a = 0u64; let mut b = 1u64;
                        for _ in 0..n { let c = a+b; a = b; b = c; }
                        println!("  fib({n}) = {a}");
                    }
                    _ => println!("  Unknown function. Try: eval add 5 3"),
                }
            }
            "compare" => {
                println!("{}", bold("── Recursion vs Induction ───────────────────────────────────────────"));
                println!("  Both are elimination principles for inductive types.");
                println!();
                println!("  {}:", bold("Recursion (non-dependent eliminator)"));
                println!("  rec_ℕ : C → (C → C) → ℕ → C");
                println!("  • Motive C is a fixed type (not depending on n)");
                println!("  • Produces a function ℕ → C");
                println!("  • Used for computation (defines new functions)");
                println!();
                println!("  {}:", bold("Induction (dependent eliminator)"));
                println!("  ind_ℕ : (C:ℕ→𝒰) → C(0) → (Πn,C(n)→C(Sn)) → Πn, C(n)");
                println!("  • Motive C depends on n");
                println!("  • Produces a section of a family");
                println!("  • Used for proofs (shows P holds for all n)");
                println!();
                println!("  {}:", green("Key relationship"));
                println!("  rec_ℕ(c, f, n) = ind_ℕ(λ_.C, c, λ_,r.f(r), n)");
                println!("  Induction is strictly stronger: you can derive rec from ind.");
                println!("  You cannot derive ind from rec alone (would need proof irrelevance).");
            }
            "uniqueness" => {
                println!("{}", bold("── Uniqueness of Recursive Functions ───────────────────────────────"));
                println!("  The recursion principle gives a {} for recursive functions:", cyan("unique"));
                println!();
                println!("  If h: ℕ → C satisfies:");
                println!("    h(0) = c");
                println!("    h(succ n) = f(h(n))");
                println!("  Then h = rec_ℕ(c, f)  (uniqueness via path induction)");
                println!();
                println!("  Proof: show ∀n, h(n) = rec_ℕ(c,f)(n) by induction.");
                println!("    Base: h(0) = c = rec(c,f)(0)");
                println!("    Step: h(succ n) = f(h(n)) = f(rec(c,f)(n)) = rec(c,f)(succ n)");
                println!();
                println!("  In HoTT: this uniqueness is a proof term (path), not just a fact.");
                println!("  The type of recursive functions is contractible!");
                println!("  {} UniqRec : isContr({{h:ℕ→C | h(0)=c ∧ ∀n,h(Sn)=f(h(n)}}))", green("Theorem:"));
            }
            "other" => {
                println!("{}", bold("── Induction for Other Types ────────────────────────────────────────"));
                println!("  {}:", bold("Bool"));
                println!("  ind_Bool : (C:Bool→𝒰) → C(tt) → C(ff) → Πb:Bool, C(b)");
                println!("  rec_Bool : C → C → Bool → C  (if-then-else)");
                println!();
                println!("  {}:", bold("List(A)"));
                println!("  ind_List : (C:List(A)→𝒰) → C([]) → (Πa,l,C(l)→C(a::l)) → Πl, C(l)");
                println!("  rec_List : C → (A → C → C) → List(A) → C  (fold)");
                println!();
                println!("  {}:", bold("Tree(A) (binary)"));
                println!("  ind_Tree : (C:Tree→𝒰) → (Πa, C(leaf(a))) →");
                println!("             (Πl,r, C(l)→C(r)→C(node(l,r))) → Πt, C(t)");
                println!();
                println!("  {} For each inductive type T, there is one eliminator.", dim("Pattern:"));
                println!("  The step case gets the induction hypothesis C(sub-term).");
            }
            "circle-ind" => {
                println!("{}", bold("── Circle Induction (S¹ as HIT) ────────────────────────────────────"));
                println!("  S¹ has constructors:");
                println!("    base : S¹");
                println!("    loop : base = base");
                println!();
                println!("  Non-dependent recursion:");
                println!("  rec_S¹ : (b:C) → (l:b=b) → S¹ → C");
                println!("  rec_S¹(b, l, base) = b");
                println!("  ap(rec_S¹(b,l), loop) = l  (computation rule for path)");
                println!();
                println!("  Dependent induction:");
                println!("  ind_S¹ : (C:S¹→𝒰) → (b:C(base)) → (transport(loop,b)=b) → Πx:S¹,C(x)");
                println!("  ind_S¹(C, b, l, base) = b");
                println!("  apd(ind_S¹(C,b,l), loop) = l  (dependent path computation rule)");
                println!();
                println!("  {} HITs have both point and path constructors.", green("Key:"));
                println!("  The induction hypothesis for path constructors involves transport.");
            }
            "path-ind" => {
                println!("{}", bold("── Path Induction (J Eliminator) ───────────────────────────────────"));
                println!("  The identity type has one constructor: refl_a : a = a");
                println!();
                println!("  Path induction (J):");
                println!("  J : (C : Πx:A, a=x → 𝒰) → C(a, refl_a) → Π(x:A)(p:a=x), C(x,p)");
                println!("  J(C, c, a, refl) = c");
                println!();
                println!("  Intuition: all paths are 'equal' to refl, up to higher structure.");
                println!("  J says: if C holds at refl, it holds at all paths.");
                println!();
                println!("  Based path induction (fixing the base):");
                println!("  J' : (C : a=x → 𝒰) → C(refl_a) → Π(p:a=x), C(p)");
                println!("  J and J' are equivalent (by Σ-types).");
                println!();
                println!("  {} J is the eliminator for identity types.", green("Key:"));
                println!("  Substitution (transport) follows from J.");
            }
            "elimination" => {
                println!("{}", bold("── Large vs Small Elimination ──────────────────────────────────────"));
                println!("  Small elimination: the motive C : T → Set  (produces sets)");
                println!("  Large elimination: the motive C : T → Type  (produces types)");
                println!();
                println!("  Large elimination is more powerful:");
                println!("    rec_Bool(A, B, b) : Type = if b then A else B");
                println!("    This defines a new TYPE depending on a boolean value.");
                println!();
                println!("  Used in HoTT for:");
                println!("    • Defining type families C(n) by recursion on n");
                println!("    • Encoding W-types, dependent types");
                println!("    • The universe hierarchy (𝒰₀ : 𝒰₁ : 𝒰₂ : …)");
                println!();
                println!("  {} In some systems (e.g. Lean 3 Prop), large elimination", yellow("Warning:"));
                println!("  is restricted for proof-irrelevant Prop to ensure extraction.");
                println!("  In HoTT, all types can be used as motives.");
            }
            "well-founded" => {
                println!("{}", bold("── Structural vs Well-Founded Recursion ────────────────────────────"));
                println!("  Structural recursion: recurse on strict sub-terms (always terminates)");
                println!("    Good: guarantees termination by type structure");
                println!("    Bad: can't express algorithms that recurse on derived smaller values");
                println!();
                println!("  Well-founded recursion: recurse when measure strictly decreases");
                println!("    A well-founded order R on A: every ∈-chain is finite");
                println!("    rec_wf : (∀x, (∀y, R(y,x) → C(y)) → C(x)) → Πx:A, C(x)");
                println!();
                println!("  In HoTT: well-founded recursion derived from accessibility:");
                println!("    Acc(R, x) := Π y, R(y,x) → Acc(R,y)  (accessible elements)");
                println!("    Wf(R) := Πx, Acc(R,x)  (the relation is well-founded)");
                println!();
                println!("  Examples: Ackermann function, Euclidean algorithm (on pair size).");
                println!("  These need well-founded recursion, not just structural.");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Induction vs Recursion — Interactive Sandbox           ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Compare induction and recursion as type-theoretic elimination principles.");
    println!("  Define functions by recursion, prove properties by induction.");
    println!("  Try {} or {}\n",
        cyan("rec-nat 5"), cyan("compare"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}ind/rec{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
