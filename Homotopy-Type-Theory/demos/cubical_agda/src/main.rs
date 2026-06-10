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

// ── Cubical Agda ──────────────────────────────────────────────────────────────
//
// Cubical Agda: an extension of Agda with cubical type theory (CCHM/ABCFHL).
//
// Key features:
//   PathP: dependent paths   PathP A p a b where p : I → A
//   hcomp: homogeneous composition
//   transp: transport (replaces J)
//   Glue types: univalence as a definitional computation rule
//
// Advantages over Book HoTT:
//   • Univalence computes (not just an axiom)
//   • Function extensionality holds definitionally
//   • Higher inductive types have computational eliminators

fn show_cubical_syntax(concept: &str) {
    match concept {
        "path" | "PathP" => {
            println!("{}", bold("── PathP in Cubical Agda ─────────────────────────────────────────────"));
            println!("  PathP : (A : I → Type) → A i0 → A i1 → Type");
            println!("  A path in a type family A over the interval.");
            println!();
            println!("  Non-dependent path (=):  Path A a b = PathP (λ _ → A) a b");
            println!("  Reflexivity:             refl : Path A a a");
            println!("                           refl = λ i → a");
            println!();
            println!("  Dependent path example (transport):");
            println!("    p : Path 𝒰 A B   (a path of types)");
            println!("    subst p a : B    where a : A");
            println!("    = transp (λ i → p i) i0 a");
            println!();
            println!("  Function extensionality:");
            println!("    funExt : (∀ x, f x = g x) → f = g");
            println!("    funExt p = λ i → λ x → p x i  (just swap arguments!)");
        }
        "transp" => {
            println!("{}", bold("── transp in Cubical Agda ────────────────────────────────────────────"));
            println!("  transp : (A : I → Type) → (i : I) → A i0 → A i");
            println!("  transp A i0 a = a  (definitional computation rule!)");
            println!();
            println!("  This replaces the J eliminator with a computational rule.");
            println!("  Unlike Book HoTT J, transp reduces definitionally.");
            println!();
            println!("  Example:");
            println!("    transport : A = B → A → B");
            println!("    transport p a = transp (λ i → p i) i0 a");
        }
        "hcomp" => {
            println!("{}", bold("── hcomp in Cubical Agda ─────────────────────────────────────────────"));
            println!("  hcomp : {{A : Type}} {{φ : I}} → (I → Partial φ A) → A → A");
            println!("  hcomp u a : A  is the composition of a partial path u");
            println!("  with base a : A, where u is defined on the face φ.");
            println!();
            println!("  Intuition: completing a 'lid' of a cube.");
            println!("  Given 3 faces of a square in A, hcomp gives the 4th face.");
            println!();
            println!("  Used to construct:");
            println!("    • Path composition: p ∙ q");
            println!("    • Kan filling (horn fillers)");
            println!("    • Higher inductive type constructors");
        }
        "glue" | "Glue" => {
            println!("{}", bold("── Glue Types and Univalence ─────────────────────────────────────────"));
            println!("  Glue types implement univalence computationally:");
            println!();
            println!("  Glue : (φ : I) → Partial φ (Σ T, T ≃ A) → Type");
            println!("  GlueElem : (t : PartialP φ (fst ∘ A)) → (a : A[φ ↦ fst (snd (A φ)) (t φ)]) → Glue φ A");
            println!();
            println!("  This makes ua (univalence) a definitional computation rule:");
            println!("    ua e : A = B");
            println!("    transport (ua e) a = fst e a  (computes!)");
            println!();
            println!("  {} Univalence is no longer an axiom: it computes!", green("Key:"));
            println!("  This was the main motivation for cubical type theory.");
        }
        "hits" | "HIT" => {
            println!("{}", bold("── HITs in Cubical Agda ──────────────────────────────────────────────"));
            println!("  Higher inductive types are natively supported.");
            println!();
            println!("  Circle S¹:");
            println!("    data S¹ : Type where");
            println!("      base : S¹");
            println!("      loop : base = base  -- (a PathP)");
            println!();
            println!("  Interval I (already primitive):");
            println!("    i0 i1 : I");
            println!("    i ∧ j, i ∨ j, ~ i : I  (De Morgan operations)");
            println!();
            println!("  Suspension ΣA:");
            println!("    data Susp A : Type where");
            println!("      north south : Susp A");
            println!("      merid : A → north = south");
            println!();
            println!("  Quotient A/R:");
            println!("    data A/R : Type where");
            println!("      [_] : A → A/R");
            println!("      eq  : (a b : A)(r : R a b) → [ a ] = [ b ]");
        }
        _ => {}
    }
}

struct Sandbox {
    modules: HashMap<String, Vec<String>>,
    proofs: Vec<(String, String)>,
}

impl Sandbox {
    fn new() -> Self {
        Sandbox { modules: HashMap::new(), proofs: Vec::new() }
    }

    fn print_help() {
        println!("{}", bold("── Cubical Agda Sandbox ────────────────────────────────────────────"));
        println!("  {}  <concept>  — show syntax for a concept", cyan("syntax"));
        println!("  {}  <thm>      — show a theorem in cubical Agda", cyan("theorem"));
        println!("  {}             — compare with Book HoTT", cyan("compare"));
        println!("{}", bold("── Core Concepts ──────────────────────────────────────────────────"));
        println!("  {}     — interval type I and its operations", cyan("interval"));
        println!("  {}       — dependent paths in type families", cyan("PathP"));
        println!("  {}      — transport (replaces J)", cyan("transp"));
        println!("  {}       — homogeneous composition", cyan("hcomp"));
        println!("  {}        — Glue types and ua computation", cyan("Glue"));
        println!("  {}         — higher inductive types", cyan("hits"));
        println!("{}", bold("── Results Proven in Cubical Agda ──────────────────────────────────"));
        println!("  {}     — function extensionality", cyan("funext"));
        println!("  {}  — Brunerie number β = 2  (π₄(S³)=ℤ/2)", cyan("brunerie"));
        println!("  {}     — Blakers-Massey theorem", cyan("blakers"));
        println!("  {}   — ∞-categorical syntax (in progress)", cyan("infinity"));
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
            "syntax" => {
                let concept = if rest.is_empty() { "path" } else { rest };
                show_cubical_syntax(concept);
                if concept != "path" && concept != "PathP" && concept != "transp"
                    && concept != "hcomp" && concept != "glue" && concept != "Glue"
                    && concept != "hits" && concept != "HIT" {
                    println!("  Available: path/PathP, transp, hcomp, Glue/glue, hits/HIT");
                }
            }
            "interval" => {
                println!("{}", bold("── The Interval I in Cubical Agda ───────────────────────────────────"));
                println!("  I : IUniv  (the interval, not in Type!)");
                println!("  i0 i1 : I  (endpoints)");
                println!();
                println!("  De Morgan algebra operations on I:");
                println!("    _∧_ : I → I → I   (meet / conjunction)");
                println!("    _∨_ : I → I → I   (join / disjunction)");
                println!("    ~_  : I → I        (negation / reversal)");
                println!();
                println!("  Laws:");
                println!("    i ∧ j = j ∧ i       (commutativity)");
                println!("    i ∧ i0 = i0          (absorption)");
                println!("    i ∧ i1 = i           (unit)");
                println!("    ~ i0 = i1,  ~ i1 = i0");
                println!("    ~ ~ i = i            (involution)");
                println!();
                println!("  A path A is a function I → A with A i0, A i1 as endpoints.");
                println!("  This is the key insight: paths ARE functions out of I.");
            }
            "PathP" | "path" => show_cubical_syntax("PathP"),
            "transp" => show_cubical_syntax("transp"),
            "hcomp" => show_cubical_syntax("hcomp"),
            "Glue" | "glue" => show_cubical_syntax("Glue"),
            "hits" | "HIT" => show_cubical_syntax("hits"),
            "theorem" | "thm" => {
                let thm = if rest.is_empty() { "funext" } else { rest };
                match thm {
                    "funext" => {
                        println!("  {}", bold("funExt in Cubical Agda:"));
                        println!("  funExt : (∀ x, f x = g x) → f = g");
                        println!("  funExt p = λ i → λ x → p x i");
                        println!();
                        println!("  This is definitionally an equality (not just propositionally).");
                        println!("  Proof: just swap i and x using function application.");
                        self.proofs.push(("funext".into(), "λ i x → p x i".into()));
                    }
                    "ua" => {
                        println!("  {}", bold("Univalence (ua) in Cubical Agda:"));
                        println!("  ua : A ≃ B → A = B");
                        println!("  ua e i = Glue B (λ{{ (i = i0) → A, e; (i = i1) → B, idEquiv }})");
                        println!();
                        println!("  Computation rule:");
                        println!("  transport (ua e) a = e .fst a  (definitional!)");
                        self.proofs.push(("ua".into(), "Glue construction".into()));
                    }
                    "uaβ" => {
                        println!("  {}", bold("uaβ — beta reduction for ua:"));
                        println!("  uaβ : (e : A ≃ B)(a : A) → transport (ua e) a = e .fst a");
                        println!("  uaβ e a = refl");
                        println!();
                        println!("  This is definitional (refl)! In Book HoTT it would require a path.");
                    }
                    _ => println!("  Available theorems: funext, ua, uaβ"),
                }
            }
            "compare" => {
                println!("{}", bold("── Cubical Agda vs Book HoTT ────────────────────────────────────────"));
                println!("  Feature                    Book HoTT       Cubical Agda");
                println!("  ─────────────────────────────────────────────────────────");
                println!("  Univalence                 axiom           computable (Glue)");
                println!("  Function extensionality    axiom           definitional");
                println!("  J eliminator               primitive       derived from transp");
                println!("  HIT computation rules      propositional   definitional");
                println!("  Canonicity                 unknown         {}  (for closed terms)", green("✓"));
                println!("  Proof relevance            optional        optional");
                println!("  Extraction to programs     not standard    {}  (with MLTT core)", green("✓"));
                println!();
                println!("  {} Cubical is strictly more computational.", green("Advantage:"));
                println!("  {} Some constructions are more complex in cubical.", yellow("Tradeoff:"));
                println!("    Cohesion, modalities, and some synthetic results are harder.");
            }
            "funext" => {
                println!("  funExt : (∀ x, f x = g x) → f = g");
                println!("  funExt h = λ i x → h x i   (just swap arguments)");
                println!("  {} This is definitional: it's a path by construction.", green("✓"));
                self.proofs.push(("funext".into(), "λ i x → h x i".into()));
            }
            "brunerie" => {
                println!("{}", bold("── Brunerie Number β = 2 ────────────────────────────────────────────"));
                println!("  Brunerie (2016) defined a number β : ℤ in HoTT such that");
                println!("    π₄(S³) = ℤ/βℤ");
                println!("  but couldn't compute it (it was a non-constructive existence proof).");
                println!();
                println!("  Ljungström and Mörtberg (2022):");
                println!("  Using Cubical Agda, they computed β = 2 definitionally.");
                println!("  This confirmed π₄(S³) = ℤ/2ℤ with a machine-verified proof.");
                println!();
                println!("  Method: abstract the proof into cubes that Agda can normalize.");
                println!("  The computation took O(minutes) of unfolding.");
                println!();
                println!("  {} This is a landmark result for cubical HoTT:", green("Significance:"));
                println!("  Univalence and HITs are computational tools for concrete results.");
            }
            "blakers" => {
                println!("{}", bold("── Blakers-Massey in Cubical Agda ──────────────────────────────────"));
                println!("  Blakers-Massey theorem: if f: A→B is m-connected and g: A→C is n-connected,");
                println!("  then the comparison map Pushout(f,g) → Pullback(B,C,−,−) is");
                println!("  (m+n-1)-connected.");
                println!();
                println!("  First proved in HoTT by:");
                println!("    • Lumsdaine-Finster-Licata (2013): in Book HoTT");
                println!("    • Hou (Favonia) et al. (2016): formalized in Lean + HoTT");
                println!("    • Finster et al.: in Cubical Agda (2019)");
                println!();
                println!("  The cubical proof is shorter and fully computational.");
                println!("  Key tool: the encode-decode method using HITs.");
            }
            "infinity" => {
                println!("{}", bold("── ∞-Categorical Syntax in Cubical Agda ─────────────────────────────"));
                println!("  Ongoing research: can we express (∞,1)-category theory in Cubical Agda?");
                println!();
                println!("  Approaches:");
                println!("    1. Simplicial HoTT (Riehl-Shulman): add directed interval");
                println!("    2. Cubical ∞-cats: use the cube category directly");
                println!("    3. Rezk completion: build complete Segal objects in Cubical Agda");
                println!();
                println!("  Current work:");
                println!("    • Barras-Coquand et al.: formalize Rezk completion in Cubical Agda");
                println!("    • Weaver-Licata: 2-dimensional directed type theory");
                println!("    • Sterling: XTT (extension type theory)");
                println!();
                println!("  Open problem: a single type theory that is both univalent (cubical)");
                println!("  AND has directed morphisms (simplicial/Segal) without redundancy.");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Cubical Agda — Interactive Sandbox                    ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore cubical type theory as implemented in Cubical Agda.");
    println!("  Paths are functions out of I, univalence computes via Glue types.");
    println!("  Type {} or {} to begin.\n",
        cyan("interval"), cyan("compare"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}cubical{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
