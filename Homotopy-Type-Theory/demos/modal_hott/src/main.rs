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

// ── Modal HoTT ────────────────────────────────────────────────────────────────
//
// Modalities in HoTT: monads on the universe that satisfy:
//   η_A : A → ○A  (unit)
//   μ_A : ○○A → ○A  (multiplication / uniqueness for idempotent modalities)
//
// An idempotent modality: η_{{○A}} : ○A ≃ ○○A  (○○A = ○A)
//
// Key examples:
//   ‖A‖₋₁  = propositional truncation (Squash / -1-truncation)
//   ‖A‖ₙ   = n-truncation
//   ♭A     = flat (discrete / codiscrete in cohesive HoTT)
//   ◯A     = shape (shape modality in cohesive HoTT)
//   ◻A     = necessity (modal logic)

#[derive(Clone, Debug, PartialEq)]
enum ModalTy {
    Base(String),
    Trunc(i64, Box<ModalTy>),
    Box_(Box<ModalTy>),
    Diamond(Box<ModalTy>),
    Flat(Box<ModalTy>),
    Sharp(Box<ModalTy>),
    Shape(Box<ModalTy>),
    Prod(Box<ModalTy>, Box<ModalTy>),
    Arrow(Box<ModalTy>, Box<ModalTy>),
}

impl ModalTy {
    fn display(&self) -> String {
        match self {
            ModalTy::Base(s) => s.clone(),
            ModalTy::Trunc(n, t) => format!("‖{}‖_{}", t.display(), n),
            ModalTy::Box_(t) => format!("□({})", t.display()),
            ModalTy::Diamond(t) => format!("◇({})", t.display()),
            ModalTy::Flat(t) => format!("♭({})", t.display()),
            ModalTy::Sharp(t) => format!("♯({})", t.display()),
            ModalTy::Shape(t) => format!("ʃ({})", t.display()),
            ModalTy::Prod(a, b) => format!("{} × {}", a.display(), b.display()),
            ModalTy::Arrow(a, b) => format!("{} → {}", a.display(), b.display()),
        }
    }

    fn is_modal_for(&self, modal: &str) -> bool {
        match (modal, self) {
            ("trunc" | "prop", ModalTy::Trunc(-1, _)) => true,
            ("set", ModalTy::Trunc(0, _)) => true,
            ("flat" | "♭", ModalTy::Flat(_)) => true,
            ("sharp" | "♯", ModalTy::Sharp(_)) => true,
            _ => false,
        }
    }
}

fn parse_modal(s: &str) -> Option<ModalTy> {
    let s = s.trim();
    if s.starts_with("trunc(") && s.ends_with(')') {
        let inner = &s[6..s.len()-1];
        return Some(ModalTy::Trunc(-1, Box::new(parse_modal(inner)?)));
    }
    if s.starts_with("prop(") && s.ends_with(')') {
        let inner = &s[5..s.len()-1];
        return Some(ModalTy::Trunc(-1, Box::new(parse_modal(inner)?)));
    }
    if s.starts_with("box(") && s.ends_with(')') {
        let inner = &s[4..s.len()-1];
        return Some(ModalTy::Box_(Box::new(parse_modal(inner)?)));
    }
    if s.starts_with("flat(") && s.ends_with(')') {
        let inner = &s[5..s.len()-1];
        return Some(ModalTy::Flat(Box::new(parse_modal(inner)?)));
    }
    if s.starts_with("shape(") && s.ends_with(')') {
        let inner = &s[6..s.len()-1];
        return Some(ModalTy::Shape(Box::new(parse_modal(inner)?)));
    }
    if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Some(ModalTy::Base(s.into()));
    }
    None
}

struct Sandbox {
    named: HashMap<String, ModalTy>,
    history: Vec<String>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { named: HashMap::new(), history: Vec::new() };
        sb.named.insert("Prop".into(), ModalTy::Trunc(-1, Box::new(ModalTy::Base("A".into()))));
        sb
    }

    fn print_help() {
        println!("{}", bold("── Modal HoTT Sandbox ──────────────────────────────────────────────"));
        println!("  {}  <A>       — propositional truncation ‖A‖₋₁", cyan("prop"));
        println!("  {}  <A> <n>   — n-truncation ‖A‖ₙ", cyan("trunc"));
        println!("  {}  <A>       — apply □ (necessity) modality", cyan("box"));
        println!("  {}  <A>       — apply ♭ (flat/discrete) modality", cyan("flat"));
        println!("  {}  <A>       — apply ʃ (shape) modality", cyan("shape"));
        println!("{}", bold("── Theory ──────────────────────────────────────────────────────────"));
        println!("  {}    — what is a modality?", cyan("modality"));
        println!("  {}    — idempotent monads", cyan("idempotent"));
        println!("  {}    — truncations as modalities", cyan("truncation"));
        println!("  {}   — S4 modal logic in HoTT", cyan("s4"));
        println!("  {}    — cohesive HoTT modalities", cyan("cohesive"));
        println!("{}", bold("── Examples ────────────────────────────────────────────────────────"));
        println!("  {}    — double negation ¬¬ modality", cyan("double-neg"));
        println!("  {}  <n>      — n-connected cover modality", cyan("connected"));
        println!("  {}    — local types and lex modalities", cyan("lex"));
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
            "prop" | "‖-‖" => {
                let ty = if rest.is_empty() { "A" } else { rest };
                println!("  ‖{}‖₋₁ = propositional truncation of {}", cyan(ty), ty);
                println!("  • Unit: η : {ty} → ‖{ty}‖₋₁");
                println!("  • Elimination: if P is a proposition and f: {ty}→P, then lift ‖{ty}‖₋₁→P");
                println!("  • ‖{ty}‖₋₁ is the smallest proposition containing {ty}");
                println!("  • Erases all path structure, keeps only existence");
                self.history.push(format!("‖{ty}‖₋₁"));
            }
            "trunc" => {
                let args: Vec<&str> = rest.split_whitespace().collect();
                let ty = args.first().copied().unwrap_or("A");
                let n: i64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                println!("  ‖{}‖_{n} = {n}-truncation of {ty}", cyan(ty));
                let desc = match n {
                    -2 => "contractibility (collapses to a point)",
                    -1 => "proposition (all paths erased)",
                    0 => "set (all higher paths erased, identity types are props)",
                    1 => "groupoid (all 2-paths erased)",
                    _k => "higher groupoid level (erases above dimension k+2)",
                };
                println!("  {n}-truncation: {desc}");
                println!("  Unit: η : {ty} → ‖{ty}‖_{n}");
                println!("  Elimination: to map ‖{ty}‖_{n} → B (for B an {n}-type), it suffices to give {ty} → B");
                self.history.push(format!("‖{ty}‖_{n}"));
            }
            "box" => {
                let ty = if rest.is_empty() { "A" } else { rest };
                println!("  □({}) = box/necessity modality applied to {ty}", cyan(ty));
                println!("  • In modal logic: □P = 'necessarily P' = P in all accessible worlds");
                println!("  • In HoTT (Lawvere-Tierney): □ = j∘− for a topology j:Ω→Ω");
                println!("  • Sheaves for □: the ¬¬-sheaves (for j=¬¬), or n-truncated types");
                println!("  • □ is a comonad (unit: □A → A, not A → □A!)");
                println!("    Actually □ = necessity is left adjoint (comonad), ◇ = right");
                self.history.push(format!("□({ty})"));
            }
            "flat" => {
                let ty = if rest.is_empty() { "A" } else { rest };
                println!("  ♭({}) = flat modality on {ty} (cohesive HoTT)", cyan(ty));
                println!("  • ♭A = 'A with discrete topology' = constant/flat sheaf");
                println!("  • Terms of ♭A: 'crisp' terms, independent of cohesive structure");
                println!("  • There is a counit ♭A → A (every discrete thing is cohesive)");
                println!("  • ♭ is a comonad; ♯ is the right adjoint");
                println!("  • In differential cohomology: ♭A = locally constant sections");
                self.history.push(format!("♭({ty})"));
            }
            "shape" => {
                let ty = if rest.is_empty() { "A" } else { rest };
                println!("  ʃ({}) = shape modality on {ty} (cohesive HoTT)", cyan(ty));
                println!("  • ʃA = the underlying homotopy type (shape) of A");
                println!("  • Unit: A → ʃA (every space has an underlying shape)");
                println!("  • ʃA is discrete (all cohesion forgotten)");
                println!("  • π₀(ʃA) = connected components of A");
                println!("  • ʃ(ℝ) = * (ℝ is contractible topologically)");
                println!("  • ʃ(S¹) = S¹ (circle is its own shape)");
                self.history.push(format!("ʃ({ty})"));
            }
            "modality" => {
                println!("{}", bold("── What is a Modality? ──────────────────────────────────────────────"));
                println!("  A modality in HoTT (Rijke-Shulman-Spitters) is:");
                println!("    ○ : 𝒰 → 𝒰  (an operator on types)");
                println!("    η_A : A → ○A  for every type A  (the unit)");
                println!("  such that:");
                println!("    • For every f : A → ○B, there is a unique lift ○A → ○B");
                println!("      (i.e., ○B is ○-local, and ○ is left adjoint to the inclusion)");
                println!();
                println!("  Idempotent modality (most important case in HoTT):");
                println!("    η_{{○A}} : ○A → ○○A  is an equivalence");
                println!("    (applying ○ twice is the same as once)");
                println!();
                println!("  ○-local type: A is ○-local if η_A : A → ○A is an equivalence.");
                println!("  The unit η_A : A → ○A is the ○-localization of A.");
                println!();
                println!("  Examples: n-truncation ‖−‖ₙ, propositional truncation ‖−‖₋₁,");
                println!("            shape ʃ, flat ♭, double negation ¬¬, …");
            }
            "idempotent" => {
                println!("{}", bold("── Idempotent Monads as Modalities ──────────────────────────────────"));
                println!("  An idempotent monad (○, η, μ) on 𝒰 has:");
                println!("    η_A : A → ○A   (unit)");
                println!("    μ_A : ○○A → ○A  (multiplication)  but μ_A = η_{{○A}}⁻¹");
                println!();
                println!("  Idempotent means: ○η_A = η_{{○A}} : ○A → ○○A is an equivalence.");
                println!("  So ○○A ≃ ○A — applying twice doesn't do more.");
                println!();
                println!("  This corresponds to a reflective subcategory:");
                println!("    ○-local types ↪ 𝒰  with left adjoint ○: 𝒰 → ○-local");
                println!("    (every type has a best approximation by a ○-local type)");
                println!();
                println!("  In category theory: a left-exact (lex) localization.");
                println!("  In topology: a localization of spaces (e.g., p-localization).");
            }
            "truncation" => {
                println!("{}", bold("── Truncation as a Modality ─────────────────────────────────────────"));
                println!("  The n-truncation ‖−‖ₙ is an idempotent modality:");
                println!();
                for n in -2i64..=3 {
                    let desc = match n {
                        -2 => "⟨−2⟩ = contractibility  (trivial groupoid)",
                        -1 => "⟨−1⟩ = propositionality  (erase all paths)",
                        0  => "⟨0⟩  = set-ness          (erase paths between paths)",
                        1  => "⟨1⟩  = groupoid-ness     (erase 2-paths)",
                        2  => "⟨2⟩  = 2-groupoid-ness   (erase 3-paths)",
                        _  => "⟨n⟩  = n-type-ness",
                    };
                    println!("  {}", desc);
                }
                println!();
                println!("  Truncation modality tower:");
                println!("    … → ‖A‖₂ → ‖A‖₁ → ‖A‖₀ → ‖A‖₋₁ → ‖A‖₋₂ = *");
                println!("  Each map is the universal n-type approximation.");
                println!();
                println!("  Lex (left exact) modalities preserve Σ-types and pullbacks:");
                println!("  ‖−‖ₙ is lex for all n ≥ -1.");
            }
            "s4" => {
                println!("{}", bold("── S4 Modal Logic in HoTT ───────────────────────────────────────────"));
                println!("  S4 = modal logic with:");
                println!("    K: □(P→Q) → □P → □Q  (distribution)");
                println!("    T: □P → P             (reflexivity)");
                println!("    4: □P → □□P           (transitivity / idempotence)");
                println!();
                println!("  In HoTT: □ = propositional truncation ‖−‖₋₁ satisfies:");
                println!("    K: if ‖P→Q‖₋₁ and ‖P‖₋₁, then ‖Q‖₋₁  {}", green("✓"));
                println!("    T: NOT valid — ‖P‖₋₁ does not imply P  {}", red("✗"));
                println!("       (P might be inhabited non-propositionally)");
                println!("    4: ‖‖P‖₋₁‖₋₁ = ‖P‖₋₁  {}", green("✓ (idempotent)"));
                println!();
                println!("  A better model: □P = P (when P is a proposition)");
                println!("  Then T holds trivially and 4 is idempotence of prop types.");
                println!();
                println!("  Fitch-style modal type theory:");
                println!("  A HoTT variant where □A is the 'crisp/sharp' version of A.");
            }
            "cohesive" => {
                println!("{}", bold("── Cohesive HoTT ────────────────────────────────────────────────────"));
                println!("  Introduced by Shulman (2018), building on Lawvere and Schreiber.");
                println!();
                println!("  Cohesive HoTT has THREE modalities forming a triple adjunction:");
                println!("    ʃ ⊣ ♭ ⊣ ♯");
                println!("    (shape) ⊣ (flat) ⊣ (sharp)");
                println!();
                println!("  Their types:");
                println!("    ʃ : 𝒰 → 𝒰  (shape, left adjoint)     — discrete target");
                println!("    ♭ : 𝒰 → 𝒰  (flat, middle)            — discrete source");
                println!("    ♯ : 𝒰 → 𝒰  (sharp, right adjoint)    — codiscrete target");
                println!();
                println!("  Axioms:");
                println!("    ♭A → A  (flat types embed into all types)");
                println!("    A → ♯A  (all types embed into sharp types)");
                println!("    ʃ(♭A) ≃ ♭A  (flat types are already discrete)");
                println!();
                println!("  Application: synthetic differential geometry, de Rham cohomology,");
                println!("  connections on bundles — all internal to cohesive HoTT.");
            }
            "double-neg" => {
                println!("{}", bold("── Double Negation Modality ─────────────────────────────────────────"));
                println!("  ¬¬ : 𝒰 → 𝒰  defined by  ¬¬A = (A → ⊥) → ⊥");
                println!();
                println!("  This IS a modality (idempotent monad on propositions):");
                println!("  • Unit: η: A → ¬¬A  (double negation introduction)");
                println!("  • Idempotent: ¬¬¬¬A ≃ ¬¬A  (triple neg = single neg)");
                println!();
                println!("  ¬¬-sheaves = Boolean propositions:");
                println!("    A is ¬¬-local iff (¬¬A → A), i.e., ¬¬A = A");
                println!("    These are propositions P where ¬¬P ≃ P (stable propositions)");
                println!();
                println!("  Lawvere-Tierney topology j=¬¬: sheaves for this = Boolean algebra.");
                println!("  Classical logic = working with ¬¬-sheaves.");
                println!();
                println!("  In HoTT: if we assume LEM (A ∨ ¬A), then ¬¬A → A for all propositions.");
            }
            "connected" => {
                let n: i64 = rest.parse().unwrap_or(0);
                println!("  {n}-connected cover modality:");
                println!("  τ_{n} : 𝒰 → 𝒰  (kills all homotopy groups up to degree n)");
                println!();
                println!("  τ_{n}(A) is defined so that:");
                println!("    πₖ(τ_{n}(A)) = 0  for k ≤ {n}");
                println!("    πₖ(τ_{n}(A)) = πₖ(A)  for k > {n}");
                println!();
                match n {
                    -1 => println!("  τ₋₁ = identity (no truncation below π₀)"),
                    0 => println!("  τ₀(A) = connected cover of A  (removes π₀, keeps π₁,π₂,…)"),
                    1 => println!("  τ₁(A) = universal cover of A  (removes π₀ and π₁)"),
                    _ => println!("  τ_{n}(A) = {n}-connected cover (removes π₀ through π_{n})"),
                }
                println!();
                println!("  This is the Whitehead tower: … → τ_{n}A → τ_{{n-1}}A → … → A");
            }
            "lex" => {
                println!("{}", bold("── Lex Modalities ───────────────────────────────────────────────────"));
                println!("  A modality ○ is lex (left exact) if it preserves pullbacks:");
                println!("    If A = B ×_C D  then  ○A = ○B ×_{{○C}} ○D");
                println!();
                println!("  Equivalently: ○ preserves finite limits / Σ-types.");
                println!("  lex + idempotent = 'topological localization'");
                println!();
                println!("  Lex modalities include:");
                println!("    • n-truncation ‖−‖ₙ  for n ≥ -1");
                println!("    • Propositional truncation ‖−‖₋₁");
                println!("    • Shape ʃ (in cohesive HoTT)");
                println!("    • Localizations at a map f: A → B  (○ = f-local)");
                println!();
                println!("  Non-lex: double negation ¬¬ (doesn't preserve products in general).");
                println!();
                println!("  Lex modalities correspond to ∞-topos theoretic localizations.");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Modal HoTT — Interactive Sandbox                       ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore modalities: truncations, necessity, cohesive structure.");
    println!("  A modality is an idempotent monad on types.");
    println!("  Type {} or {} to begin.\n",
        cyan("modality"), cyan("cohesive"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}modal{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
