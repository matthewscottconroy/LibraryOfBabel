use std::io::{self, BufRead, Write};

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

// ── James Construction ─────────────────────────────────────────────────────────
//
// J(X) = ∐ₙ Xⁿ / ~  where (x₁,…,*,…,xₙ) ~ (x₁,…,x̂ᵢ,…,xₙ)
// (basepoint acts like a unit / identity element)
//
// Key theorem: ΩΣX ≃ J(X)  for connected X
// Stable splitting: Σ∞J(X) ≃ ∨ₙ Σ∞Xⁿ/Xⁿ⁻¹

// Stable homotopy groups of spheres (selected)
fn stable_pi(k: i64) -> &'static str {
    match k {
        0 => "ℤ",
        1 => "ℤ/2ℤ",
        2 => "ℤ/2ℤ",
        3 => "ℤ/24ℤ",
        4 => "0",
        5 => "0",
        6 => "ℤ/2ℤ",
        7 => "ℤ/240ℤ",
        8 => "ℤ/2ℤ ⊕ ℤ/2ℤ",
        9 => "ℤ/2ℤ ⊕ ℤ/2ℤ ⊕ ℤ/2ℤ",
        _ => "(see tables)",
    }
}

#[derive(Clone, Debug)]
struct JWord {
    letters: Vec<String>,
    base: String,
}

impl JWord {
    fn new(base: &str) -> Self { JWord { letters: Vec::new(), base: base.into() } }

    fn push(&mut self, x: &str) {
        if x != self.base { self.letters.push(x.into()); }
    }

    fn reduce(&mut self) {
        self.letters.retain(|x| x != &self.base);
    }

    fn display(&self) -> String {
        if self.letters.is_empty() {
            format!("* (basepoint)")
        } else {
            format!("[{}]", self.letters.join(","))
        }
    }

    fn length(&self) -> usize { self.letters.len() }
}

struct Sandbox {
    base: String,
    words: Vec<JWord>,
    space: String,
}

impl Sandbox {
    fn new() -> Self {
        Sandbox { base: "*".into(), words: Vec::new(), space: "X".into() }
    }

    fn print_help() {
        println!("{}", bold("── James Construction ──────────────────────────────────────────────"));
        println!("  {}  <sp>         — set the space (e.g. S1, S2)", cyan("space"));
        println!("  {}  <base>       — set the basepoint symbol", cyan("base"));
        println!("  {}  <x1> <x2>…  — create a word in J(X)", cyan("word"));
        println!("  {}          — list all words built so far", cyan("list"));
        println!("  {}  <i> <j>      — concatenate words i and j", cyan("concat"));
        println!("{}", bold("── Theory ─────────────────────────────────────────────────────────"));
        println!("  {}  <sp>         — ΩΣX ≃ J(X) theorem", cyan("loop-susp"));
        println!("  {}         — filtration J₁⊂J₂⊂J₃⊂…", cyan("filtration"));
        println!("  {}   — stable splitting of ΣJ(X)", cyan("splitting"));
        println!("  {}  <k>          — stable homotopy group πₖˢ", cyan("stable-pi"));
        println!("{}", bold("── Examples ───────────────────────────────────────────────────────"));
        println!("  J(S⁰): {}  S0      J(S¹): {}  S1", cyan("loop-susp"), cyan("loop-susp"));
        println!("  {}         — James-Hopf invariant map", cyan("james-hopf"));
        println!("  {}         — EHP sequence", cyan("ehp"));
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
            "space" => {
                self.space = if rest.is_empty() { "X".into() } else { rest.into() };
                println!("  Space set to {}", cyan(&self.space));
                println!("  J({}) = free topological monoid on {} (modulo basepoint)", self.space, self.space);
            }
            "base" => {
                self.base = if rest.is_empty() { "*".into() } else { rest.into() };
                println!("  Basepoint set to {}", cyan(&self.base));
            }
            "word" => {
                let mut w = JWord::new(&self.base);
                for tok in rest.split_whitespace() { w.push(tok); }
                w.reduce();
                let idx = self.words.len();
                println!("  Word {}: {} ∈ J{}({})", cyan(&format!("#{idx}")),
                    cyan(&w.display()), w.length(), self.space);
                if w.length() == 0 {
                    println!("  {} This is the identity element (empty word).", dim("Note:"));
                }
                self.words.push(w);
            }
            "list" => {
                if self.words.is_empty() {
                    println!("  No words yet. Use {} to create some.", cyan("word"));
                } else {
                    println!("  Words in J({}):", self.space);
                    for (i, w) in self.words.iter().enumerate() {
                        println!("  #{i}: {} ∈ J{}({})", cyan(&w.display()), w.length(), self.space);
                    }
                }
            }
            "concat" => {
                let args: Vec<&str> = rest.split_whitespace().collect();
                if args.len() < 2 {
                    println!("  Usage: concat <i> <j>");
                    return true;
                }
                let i: usize = args[0].parse().unwrap_or(0);
                let j: usize = args[1].parse().unwrap_or(0);
                if i >= self.words.len() || j >= self.words.len() {
                    println!("  {} Index out of range.", red("✗"));
                    return true;
                }
                let mut result = self.words[i].clone();
                for l in &self.words[j].letters.clone() { result.push(l); }
                result.reduce();
                println!("  #{i} · #{j} = {} ∈ J{}({})",
                    cyan(&result.display()), result.length(), self.space);
                println!("  {} Concatenation is the monoid operation in J(X).", dim("Note:"));
                self.words.push(result);
            }
            "loop-susp" => {
                let sp = if rest.is_empty() { self.space.as_str() } else { rest };
                println!("{}", bold(&format!("── ΩΣ({sp}) ≃ J({sp}) ────────────────────────────────────────────────")));
                println!("  J({sp}) = the James construction on {sp}:");
                println!("    Jₙ({sp}) = ({sp})ⁿ / (sequences with a basepoint collapsed)");
                println!("    J({sp}) = colim Jₙ({sp}) = free topological monoid on {sp}");
                println!();
                println!("  ΩΣ({sp}) = loop space of the suspension of {sp}");
                println!();
                println!("  James theorem (1955): {} ΩΣ({sp}) ≃ J({sp}) for connected {sp}", green("✓"));
                println!();
                match sp {
                    "S0" => {
                        println!("  J(S⁰) ≃ ΩΣ(S⁰) = ΩS¹ ≃ ℤ  (discrete)");
                        println!("  Words in J(S⁰): sequences of {{+1,-1}} up to *)");
                    }
                    "S1" => {
                        println!("  J(S¹) ≃ ΩΣ(S¹) = ΩS²");
                        println!("  Filtration: J₁=S¹, J₂=S³, J₃=S⁵, J_{{2k-1}}=S^{{2k-1}}, …");
                        println!("  Used by James to compute early stable homotopy groups.");
                    }
                    "S2" => {
                        println!("  J(S²) ≃ ΩS³  (after localization)");
                        println!("  Filtration: S², S² ∪η e⁴, S² ∪η e⁴ ∪ e⁶, …");
                    }
                    _ => {
                        println!("  J({sp}) = ΩΣ({sp})");
                        println!("  This is the free associative H-space generated by {sp}.");
                    }
                }
            }
            "filtration" => {
                println!("{}", bold("── Filtration of J(X) ──────────────────────────────────────────────"));
                println!("  J₀(X) = {{*}}  (basepoint only)");
                println!("  J₁(X) = X      (1-letter words ≃ X)");
                println!("  J₂(X) = (X×X)/~ where (x,*)~(x) and (*,y)~(y)");
                println!("  Jₙ(X) = (Xⁿ)/~  (n-letter words, basepoint units out)");
                println!("  J(X) = ∪ₙ Jₙ(X)  with CW topology");
                println!();
                println!("  For X = Sᵐ:");
                println!("  J₁(Sᵐ) = Sᵐ");
                println!("  J₂(Sᵐ) = Sᵐ ∪ e²ᵐ  (attaching map related to Whitehead product)");
                println!("  Jₙ(Sᵐ) = J_{{n-1}}(Sᵐ) ∪ eⁿᵐ");
                println!();
                println!("  For X = S¹:");
                println!("  J(S¹) ≃ ΩS² and the filtration gives cell structure of ΩS².");
            }
            "splitting" => {
                println!("{}", bold("── Stable Splitting of ΣJ(X) ──────────────────────────────────────"));
                println!("  Snaith splitting (1974):");
                println!("    Σ∞J(X) ≃ ∨_{{n≥1}} Σ∞(Xⁿ/Xⁿ⁻¹)  (as spectra)");
                println!();
                println!("  For X = Sᵐ:");
                println!("    Xⁿ/Xⁿ⁻¹ = Smash product (Sᵐ)^∧ⁿ = Sⁿᵐ");
                println!("    Σ∞ΩΣSᵐ ≃ ∨_{{n≥1}} Σ∞Sⁿᵐ  (as spectra)");
                println!();
                println!("  Consequence: stable homotopy of ΩΣSᵐ decomposes nicely.");
                println!("  This is one of the key tools in stable homotopy theory.");
                println!();
                println!("  The splitting maps are the James-Hopf invariants:");
                println!("    j_n: J(X) → Xⁿ/Xⁿ⁻¹");
            }
            "stable-pi" => {
                let k: i64 = rest.parse().unwrap_or(0);
                println!("  πₖˢ = π_{{n+k}}(Sⁿ) for large n (stable range)");
                println!("  π{}ˢ = {}", k, cyan(stable_pi(k)));
                if k == 0 {
                    println!("  {} π₀ˢ = ℤ = degree of maps S^n → S^n", green("✓"));
                } else if k == 3 {
                    println!("  {} π₃ˢ = ℤ/24ℤ relates to the J-homomorphism and spin structures", green("✓"));
                }
            }
            "james-hopf" => {
                println!("{}", bold("── James-Hopf Invariant ─────────────────────────────────────────────"));
                println!("  H: ΩΣX → ΩΣ(X∧X)   (the James-Hopf map)");
                println!();
                println!("  Under ΩΣX ≃ J(X):");
                println!("    H([x₁,…,xₙ]) = ∑_{{i<j}} [xᵢ, xⱼ]  (sum of pairs)");
                println!();
                println!("  For X = S¹:");
                println!("    H: ΩS² → ΩS³");
                println!("    This is part of the EHP sequence.");
                println!();
                println!("  Relationship to Hopf invariant:");
                println!("    The Hopf invariant of f: S³→S² is the degree of H(f): S³→S³");
                println!("    where we view S²=ΣS¹, S³=ΣS¹∧S¹=ΣS².");
            }
            "ehp" => {
                println!("{}", bold("── EHP Sequence ────────────────────────────────────────────────────"));
                println!("  For S^n (n≥1), there is a long exact sequence:");
                println!("    … → πₖ(Sⁿ⁻¹) →^E πₖ₊₁(Sⁿ) →^H πₖ₊₁(S²ⁿ⁻¹) →^P πₖ₋₁(Sⁿ⁻¹) → …");
                println!();
                println!("  E = suspension map (Einhängung)");
                println!("  H = James-Hopf invariant");
                println!("  P = boundary/attaching map (Pairenabbildung)");
                println!();
                println!("  For n=2:");
                println!("    … → πₖ(S¹) →^E πₖ₊₁(S²) →^H πₖ₊₁(S³) →^P πₖ₋₁(S¹) → …");
                println!("  At k=3:");
                println!("    π₃(S¹)=0 →^E π₄(S²)=ℤ/2ℤ →^H π₄(S³)=ℤ/2ℤ →^P π₂(S¹)=0");
                println!("  Exact!  η²: π₄(S²) is the suspended Hopf map.");
                println!();
                println!("  EHP works 2-locally (away from odd primes things are simpler).");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    James Construction — Interactive Sandbox               ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build words in J(X), the free topological monoid on X.");
    println!("  Explore ΩΣX ≃ J(X) and the stable homotopy world.");
    println!("  Type {} to begin, {} for the EHP sequence.\n",
        cyan("loop-susp S1"), cyan("ehp"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}james{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
