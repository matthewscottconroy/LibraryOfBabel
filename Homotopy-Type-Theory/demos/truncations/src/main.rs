use std::collections::HashMap;
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

// ── H-level truncation ────────────────────────────────────────────────────────
//
// ‖A‖ₙ is the n-truncation of A: the "most general type of h-level n
// that A surjects onto." Concretely:
//
// ‖A‖₋₁ = propositional truncation (at most one element; "A is nonempty")
// ‖A‖₀  = set truncation (paths collapse; UIP holds)
// ‖A‖₁  = 1-groupoid truncation
// etc.
//
// We model truncations by tracking:
//   - The type A (as a description)
//   - Its h-level
//   - Effect of truncation at various levels

#[derive(Clone, Debug)]
struct TypeDesc {
    name: String,
    elements: Option<Vec<String>>,   // if finite
    hlevel: i64,
    description: String,
}

impl TypeDesc {
    fn prop_truncation(&self) -> TypeDesc {
        let elems = self.elements.as_ref().map(|es| {
            if es.is_empty() { vec![] } else { vec!["*".into()] } // collapse to one element
        });
        TypeDesc {
            name: format!("‖{}‖₋₁", self.name),
            elements: elems,
            hlevel: -1,
            description: format!("Propositional truncation of {}. At most one element.", self.name),
        }
    }

    fn set_truncation(&self) -> TypeDesc {
        // Set truncation: collapse all paths, keep elements
        let elems = self.elements.clone().map(|es| {
            let mut seen = std::collections::HashSet::new();
            es.into_iter().filter(|e| seen.insert(e.clone())).collect()
        });
        TypeDesc {
            name: format!("‖{}‖₀", self.name),
            elements: elems,
            hlevel: 0,
            description: format!("Set truncation of {}. Paths become propositional.", self.name),
        }
    }

    fn show(&self) {
        println!("  {} : h-level {}", bold(&cyan(&self.name)), self.hlevel);
        println!("  {}", self.description);
        if let Some(elems) = &self.elements {
            match elems.len() {
                0 => println!("  Elements: {} (empty)", red("∅")),
                1 => println!("  Elements: {{{}}}", green(&elems[0])),
                n if n <= 8 => println!("  Elements: {{{}}}", elems.join(", ")),
                n => println!("  Elements: {{{}, ... ({n} total)}}", elems[..3].join(", ")),
            }
        } else {
            println!("  Elements: {} (infinite or unspecified)", yellow("∞"));
        }
    }
}

// ── Sigma and Pi with truncation effects ──────────────────────────────────────

fn sigma_type(a: &TypeDesc, b: &str) -> TypeDesc {
    TypeDesc {
        name: format!("Σ(x:{}).{b}", a.name),
        elements: None,
        hlevel: a.hlevel.max(-1), // simplification
        description: format!("Σ-type: dependent pairs. Existential when B(x) is prop."),
    }
}

fn pi_type(a: &TypeDesc, b: &str) -> TypeDesc {
    TypeDesc {
        name: format!("Π(x:{}).{b}", a.name),
        elements: None,
        hlevel: -1, // simplification: Pi of props is prop
        description: format!("Π-type: dependent functions. Universal quantification."),
    }
}

// ── Existential vs truncated Sigma ────────────────────────────────────────────

fn show_exists_vs_sigma() {
    println!("{}", bold("── ∃ vs Σ in HoTT ──────────────────────────────────────────────────"));
    println!();
    println!("  Σ(x:A). B(x)  — a dependent pair (x, proof_of_B(x))");
    println!("  Contains the WITNESS x explicitly — computational content.");
    println!("  h-level = max(h-level(A), h-level(B))");
    println!();
    println!("  ‖Σ(x:A). B(x)‖₋₁  — propositionally truncated pair");
    println!("  = ∃(x:A). B(x)  — just says 'some x exists satisfying B'");
    println!("  h-level = -1  (proposition)");
    println!("  The witness is HIDDEN — you can't extract it computationally!");
    println!();
    println!("  Example:");
    println!("  Σ(n:ℕ). isPrime(n)   — gives you a specific prime + proof");
    println!("  ∃(n:ℕ). isPrime(n)   — just tells you some prime exists");
    println!();
    println!("  In classical math: ∃ = ‖Σ‖₋₁ (squashed pair)");
    println!("  In constructive:   Σ is stronger — you need an actual witness");
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    types: HashMap<String, TypeDesc>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { types: HashMap::new() };
        // Pre-define common types
        sb.types.insert("Bool".into(), TypeDesc { name: "Bool".into(), elements: Some(vec!["false".into(), "true".into()]), hlevel: 0, description: "Two-element discrete type (set).".into() });
        sb.types.insert("Unit".into(), TypeDesc { name: "Unit".into(), elements: Some(vec!["tt".into()]), hlevel: -2, description: "Contractible type (one element).".into() });
        sb.types.insert("Void".into(), TypeDesc { name: "Void".into(), elements: Some(vec![]), hlevel: -2, description: "Empty type (vacuously contractible).".into() });
        sb.types.insert("Nat".into(), TypeDesc { name: "ℕ".into(), elements: None, hlevel: 0, description: "Natural numbers (discrete set).".into() });
        sb.types.insert("S1".into(), TypeDesc { name: "S¹".into(), elements: None, hlevel: 1, description: "Circle (1-groupoid: π₁=ℤ, paths form a set).".into() });
        sb.types.insert("S2".into(), TypeDesc { name: "S²".into(), elements: None, hlevel: 2, description: "2-Sphere (2-groupoid: π₂=ℤ).".into() });
        sb.types.insert("Prop".into(), TypeDesc { name: "P (some prop)".into(), elements: None, hlevel: -1, description: "A proposition: at most one proof.".into() });
        sb
    }

    fn print_help() {
        println!("{}", bold("── Type Operations ─────────────────────────────────────────────────"));
        println!("  {}  A               — show a type's h-level and elements", cyan("show"));
        println!("  {}  A               — propositional truncation ‖A‖₋₁", cyan("prop-trunc"));
        println!("  {}  A               — set truncation ‖A‖₀", cyan("set-trunc"));
        println!("  {}  n A             — n-truncation ‖A‖ₙ", cyan("trunc"));
        println!("  {}  Name A          — define a type", cyan("type"));
        println!("  {}               — list known types", cyan("types"));
        println!("{}", bold("── Key Concepts ────────────────────────────────────────────────────"));
        println!("  {}          — explain ∃ vs Σ (squashing)", cyan("exists"));
        println!("  {}          — propositional vs non-propositional existence", cyan("witness"));
        println!("  {}          — show the truncation ladder", cyan("ladder"));
        println!("  {}  A       — does A satisfy LEM (is it a prop)?", cyan("lem"));
        println!("{}", bold("── Preloaded types ─────────────────────────────────────────────────"));
        println!("  Bool, Unit, Void, Nat, S1, S2, Prop");
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
            "show" => {
                if let Some(ty) = self.types.get(rest) { ty.clone().show(); }
                else { println!("  {} Type {rest} not found. Use 'type' to define it.", red("✗")); }
            }
            "prop-trunc" | "prop_trunc" => {
                if let Some(ty) = self.types.get(rest) {
                    let trunc = ty.clone().prop_truncation();
                    trunc.show();
                    println!("  {}", dim("‖A‖₋₁: collapse all proofs — only 'A is inhabited' remains"));
                    self.types.insert(trunc.name.clone(), trunc);
                } else { println!("  {} Type {rest} not found", red("✗")); }
            }
            "set-trunc" | "set_trunc" => {
                if let Some(ty) = self.types.get(rest) {
                    let trunc = ty.clone().set_truncation();
                    trunc.show();
                    println!("  {}", dim("‖A‖₀: collapse paths — elements remain, UIP holds"));
                    self.types.insert(trunc.name.clone(), trunc);
                } else { println!("  {} Type {rest} not found", red("✗")); }
            }
            "trunc" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: trunc n TypeName", red("✗")); return true; }
                let n: i64 = ws[0].parse().unwrap_or(0);
                if let Some(ty) = self.types.get(ws[1]) {
                    let ty = ty.clone();
                    println!("  ‖{}‖{n} (h-level {} truncation):", ty.name, n);
                    if n >= ty.hlevel { println!("  {} {} is already of h-level {} ≤ {n}, no change.", green("✓"), ty.name, ty.hlevel); }
                    else {
                        let new_level = n;
                        let desc = match new_level {
                            -2 => "contractible — all elements become equal",
                            -1 => "proposition — paths collapse to single element (if inhabited)",
                            0 => "set — paths become propositions",
                            1 => "1-groupoid — 2-paths become trivial",
                            k => return { println!("  ‖{}‖{k}: {k}-groupoid truncation", ty.name); true },
                        };
                        println!("  Truncate from h-level {} to h-level {n}: {desc}", ty.hlevel);
                    }
                } else { println!("  {} Type {} not found", red("✗"), ws[1]); }
            }
            "type" => {
                let ws: Vec<&str> = rest.splitn(3, ' ').collect();
                if ws.is_empty() { println!("  {} Use: type Name hlevel [description]", red("✗")); return true; }
                let name = ws[0];
                let hlevel: i64 = ws.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                let desc = ws.get(2).copied().unwrap_or("user-defined type");
                let ty = TypeDesc { name: name.to_string(), elements: None, hlevel, description: desc.to_string() };
                ty.show();
                self.types.insert(name.to_string(), ty);
            }
            "types" => {
                let mut tys: Vec<_> = self.types.values().collect();
                tys.sort_by(|a, b| a.hlevel.cmp(&b.hlevel).then(a.name.cmp(&b.name)));
                println!("{}", bold("  Known types by h-level:"));
                for ty in tys { println!("  {:20} h-level {}", cyan(&ty.name), ty.hlevel); }
            }
            "exists" => show_exists_vs_sigma(),
            "witness" => {
                println!("{}", bold("── Witness Extraction & Propositional Truncation ───────────────────"));
                println!("  In HoTT, ‖A‖₋₁ (propositional truncation) satisfies:");
                println!("  • If a : A, then |a| : ‖A‖₋₁  (introduction)");
                println!("  • All elements of ‖A‖₋₁ are equal (h-level -1)");
                println!("  • If B is a prop and f : A → B, then ‖A‖₋₁ → B  (elimination)");
                println!();
                println!("  KEY: You CANNOT extract a witness from ‖A‖₋₁ in general!");
                println!("  ‖Σ(n:ℕ).P(n)‖₋₁ → ℕ  is NOT provable (in pure HoTT).");
                println!("  You would need choice principles (ACCC, global choice).");
                println!();
                println!("  This is the constructive content distinction:");
                println!("  Σ(n:ℕ).P(n)  gives you the witness n explicitly.");
                println!("  ∃(n:ℕ).P(n)  only says n exists — you can't get n out.");
                println!();
                println!("  Example: is there an even prime?");
                println!("  Σ(p:ℕ). isPrime(p) ∧ isEven(p)  — gives you p = 2 and proofs");
                println!("  ‖Σ(p:ℕ). isPrime(p) ∧ isEven(p)‖₋₁ — just says yes, there is one");
            }
            "ladder" => {
                println!("{}", bold("── Truncation Ladder ───────────────────────────────────────────────"));
                println!("  ‖A‖₋₂  contractible   — A is contractible (unique element)");
                println!("  ‖A‖₋₁  proposition    — ∃ or not; proof-irrelevant");
                println!("  ‖A‖₀   set            — elements distinguishable; UIP");
                println!("  ‖A‖₁   1-groupoid     — paths form a set");
                println!("  ‖A‖₂   2-groupoid     — 2-paths form a set");
                println!("    ⋮");
                println!("  ‖A‖ₙ   n-groupoid     — n-paths form a set");
                println!("    A  itself  ∞-groupoid    — all higher paths may be non-trivial");
                println!();
                println!("  Functoriality: if A is h-level ≤ n, then ‖A‖ₙ = A");
                println!("  Universality: ‖A‖ₙ → B  iff  A → B  when B is h-level n");
                println!();
                let builtins = [("Bool", 0), ("Nat", 0), ("S¹", 1), ("S²", 2), ("𝒰₀", i64::MAX)];
                for (name, lv) in &builtins {
                    println!("  {} at h-level {} → ‖{}‖₋₁ collapses to prop", cyan(name), lv, name);
                }
            }
            "lem" => {
                if let Some(ty) = self.types.get(rest) {
                    let hlv = ty.hlevel;
                    if hlv <= -1 { println!("  {} is a {} — LEM holds: either inhabited or empty, and we can decide.", cyan(&ty.name), green("proposition")); }
                    else { println!("  {} is h-level {hlv} — LEM does not follow automatically for non-props.", cyan(&ty.name)); }
                } else { println!("  {} Type {rest} not found", red("✗")); }
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║     Truncations — ‖A‖ₙ and ∃ vs Σ Sandbox              ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore truncations: collapsing types to various h-levels.");
    println!("  Understand the difference between ∃ (truncated) and Σ (with witness).");
    println!("  Type {} for commands, {} for the ladder, {} for ∃ vs Σ.\n", cyan("help"), cyan("ladder"), cyan("exists"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}trunc{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
