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

// ── Universe hierarchy ─────────────────────────────────────────────────────────
//
// 𝒰₀ : 𝒰₁ : 𝒰₂ : ⋯
//
// Types in 𝒰ₙ are also in 𝒰ₙ₊₁ (cumulative)
// 𝒰ₙ itself is in 𝒰ₙ₊₁ but NOT in 𝒰ₙ (avoids Girard's paradox)
//
// Resizing: some propositions can be lowered to any universe level.

#[derive(Clone, Debug)]
struct TypeEntry {
    name: String,
    level: usize,
    kind: TypeKind,
}

#[derive(Clone, Debug)]
enum TypeKind {
    Small,      // a concrete type like Bool, ℕ
    Universe(usize), // 𝒰ₙ itself
    Pi(String, String), // Π(x:A).B
    Sigma(String, String),
    Prop,       // a proposition (lives in all universes via propositional resizing)
}

impl TypeEntry {
    fn display(&self) -> String {
        format!("{} : 𝒰{}", self.name, self.level)
    }

    fn kind_str(&self) -> &str {
        match &self.kind {
            TypeKind::Small => "small type",
            TypeKind::Universe(_n) => "universe",
            TypeKind::Pi(_, _) => "Π-type (function/dependent product)",
            TypeKind::Sigma(_, _) => "Σ-type (dependent sum)",
            TypeKind::Prop => "proposition (resizable)",
        }
    }
}

// ── Level inference rules ─────────────────────────────────────────────────────

fn level_of_pi(dom_level: usize, cod_level: usize) -> usize {
    // Π(x:A).B where A:𝒰ₙ, B:𝒰ₘ gives Π:𝒰_{max(n,m)}
    dom_level.max(cod_level)
}

fn level_of_sigma(fst_level: usize, snd_level: usize) -> usize {
    fst_level.max(snd_level)
}

// ── Paradox checks ────────────────────────────────────────────────────────────

fn check_girard(ty: &str, level_claimed: usize) -> bool {
    // 𝒰ₙ : 𝒰ₙ would be Girard's paradox
    ty.starts_with('U') && ty[1..].parse::<usize>().ok() == Some(level_claimed)
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    types: HashMap<String, TypeEntry>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { types: HashMap::new() };
        // Pre-populate
        for name in ["Bool", "Nat", "Int", "Unit", "Void"] {
            sb.types.insert(name.to_string(), TypeEntry { name: name.to_string(), level: 0, kind: TypeKind::Small });
        }
        sb.types.insert("U0".into(), TypeEntry { name: "𝒰₀".into(), level: 1, kind: TypeKind::Universe(0) });
        sb.types.insert("U1".into(), TypeEntry { name: "𝒰₁".into(), level: 2, kind: TypeKind::Universe(1) });
        sb.types.insert("Prop".into(), TypeEntry { name: "Prop".into(), level: 1, kind: TypeKind::Prop });
        sb
    }

    fn print_help() {
        println!("{}", bold("── Universe Levels ─────────────────────────────────────────────────"));
        println!("  {}  A n             — define type A at level n", cyan("type"));
        println!("  {}  A               — show universe level of A", cyan("level"));
        println!("  {}  A n             — check if A can live in 𝒰n", cyan("inhabits"));
        println!("{}", bold("── Level Inference ─────────────────────────────────────────────────"));
        println!("  {}  A B             — level of Π(x:A).B", cyan("pi"));
        println!("  {}  A B             — level of Σ(x:A).B", cyan("sigma"));
        println!("  {}  A               — level of A→B (non-dep)", cyan("fun"));
        println!("{}", bold("── Paradoxes & Consistency ─────────────────────────────────────────"));
        println!("  {}  A n             — check if A:𝒰n is paradoxical", cyan("check"));
        println!("  {}          — Girard's paradox explained", cyan("girard"));
        println!("  {}          — Russell's paradox connection", cyan("russell"));
        println!("  {}          — propositional resizing explained", cyan("resize"));
        println!("{}", bold("── Preloaded types ─────────────────────────────────────────────────"));
        println!("  Bool, Nat, Int, Unit, Void (level 0)   U0 (level 1)   Prop (level 1)");
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
            "type" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.is_empty() { println!("  {} Use: type Name level", red("✗")); return true; }
                let name = ws[0];
                let level: usize = ws.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                if check_girard(name, level) {
                    println!("  {} {} : 𝒰{} would be Girard's paradox! (𝒰ₙ cannot be in 𝒰ₙ)", red("⚠"), name, level);
                    println!("  {} lives in 𝒰{}", name, level + 1);
                    return true;
                }
                let entry = TypeEntry { name: name.to_string(), level, kind: TypeKind::Small };
                println!("  {} : 𝒰{}", cyan(name), level);
                self.types.insert(name.to_string(), entry);
            }
            "level" => {
                if let Some(t) = self.types.get(rest) {
                    println!("  {} : 𝒰{}  ({})", cyan(&t.name), t.level, t.kind_str());
                    println!("  Also lives in: 𝒰{} ⊂ 𝒰{} ⊂ 𝒰{} ⊂ ⋯  (cumulativity)", t.level, t.level+1, t.level+2);
                } else { println!("  {} Type {rest} not found", red("✗")); }
            }
            "inhabits" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: inhabits A n", red("✗")); return true; }
                let n: usize = ws[1].parse().unwrap_or(0);
                if let Some(t) = self.types.get(ws[0]) {
                    if t.level <= n {
                        println!("  {} {} : 𝒰{}  (lives in 𝒰{} since {} ≤ {})", green("✓"), t.name, t.level, n, t.level, n);
                    } else {
                        println!("  {} {} : 𝒰{} cannot be moved down to 𝒰{}", red("✗"), t.name, t.level, n);
                    }
                } else { println!("  {} Type {} not found", red("✗"), ws[0]); }
            }
            "pi" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: pi A B", red("✗")); return true; }
                let (la, lb) = match (self.types.get(ws[0]), self.types.get(ws[1])) {
                    (Some(a), Some(b)) => (a.level, b.level),
                    _ => { println!("  {} Type(s) not found", red("✗")); return true; }
                };
                let lv = level_of_pi(la, lb);
                println!("  Π(x:{}). {} : 𝒰{}", ws[0], ws[1], lv);
                println!("  = max({}, {}) = {}", la, lb, lv);
            }
            "sigma" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: sigma A B", red("✗")); return true; }
                let (la, lb) = match (self.types.get(ws[0]), self.types.get(ws[1])) {
                    (Some(a), Some(b)) => (a.level, b.level),
                    _ => { println!("  {} Type(s) not found", red("✗")); return true; }
                };
                let lv = level_of_sigma(la, lb);
                println!("  Σ(x:{}). {} : 𝒰{}", ws[0], ws[1], lv);
                println!("  = max({}, {}) = {}", la, lb, lv);
            }
            "fun" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: fun A B", red("✗")); return true; }
                let (la, lb) = match (self.types.get(ws[0]), self.types.get(ws[1])) {
                    (Some(a), Some(b)) => (a.level, b.level),
                    _ => { println!("  {} Type(s) not found", red("✗")); return true; }
                };
                let lv = la.max(lb);
                println!("  {} → {} : 𝒰{}  (same rule as Π)", ws[0], ws[1], lv);
            }
            "check" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: check A n", red("✗")); return true; }
                let n: usize = ws[1].parse().unwrap_or(0);
                if check_girard(ws[0], n) {
                    println!("  {} PARADOXICAL: 𝒰ₙ : 𝒰ₙ leads to Girard's paradox!", red("⚠"));
                    println!("  Must use 𝒰ₙ : 𝒰ₙ₊₁ instead.");
                } else { println!("  {} No immediate paradox detected.", green("✓")); }
            }
            "girard" => {
                println!("{}", bold("── Girard's Paradox (1972) ─────────────────────────────────────────"));
                println!("  If we had 𝒰 : 𝒰 (type-in-itself), we could construct:");
                println!("  A retraction of 𝒰 onto any subset via self-referential types.");
                println!("  This allows encoding of Burali-Forti's paradox (no largest ordinal).");
                println!();
                println!("  Resolution: The universe hierarchy 𝒰₀ : 𝒰₁ : 𝒰₂ : ⋯");
                println!("  Each 𝒰ₙ contains all the types, but 𝒰ₙ : 𝒰ₙ₊₁ (not 𝒰ₙ itself).");
                println!();
                println!("  In Lean 4:  Sort 0 = Prop, Sort 1 = Type, Sort (n+1) = Type n");
                println!("  In Agda:    Set₀ : Set₁ : Set₂ : ⋯ (by default, universe-polymorphic)");
                println!("  In Coq:     Prop : Set : Type₀ : Type₁ : ⋯");
                println!("  In HoTT:    𝒰₀ : 𝒰₁ : 𝒰₂ : ⋯  (cumulative by default)");
            }
            "russell" => {
                println!("{}", bold("── Russell's Paradox Connection ────────────────────────────────────"));
                println!("  Russell's paradox: R = {{x | x ∉ x}}  leads to R ∈ R ↔ R ∉ R");
                println!();
                println!("  In naive type theory: define V : 𝒰 as 'the type of all types'.");
                println!("  Then V : V (V is a member of itself) — leads to paradox.");
                println!();
                println!("  Fix: no 'type of all types'. Instead, universe hierarchy.");
                println!("  𝒰₀ contains small types. 𝒰₁ contains 𝒰₀ and types-of-types.");
                println!("  No single universe contains itself.");
                println!();
                println!("  Universe polymorphism: define Type : 𝒰ₙ for any n,");
                println!("  with n inferred automatically (Agda/Lean approach).");
            }
            "resize" => {
                println!("{}", bold("── Propositional Resizing ───────────────────────────────────────────"));
                println!("  A proposition P (h-level -1) has at most one element.");
                println!("  It can be made small: P : 𝒰₀ even if defined using 𝒰₁.");
                println!();
                println!("  Propositional Resizing Axiom: every P : Prop 𝒰₁ is equivalent");
                println!("  to some Q : Prop 𝒰₀ (there's a 𝒰₀-small representation).");
                println!();
                println!("  This is an additional axiom (not provable from univalence alone).");
                println!("  It allows: Prop 𝒰₀ ≃ Prop 𝒰₁ ≃ ⋯ (all prop-universes collapse).");
                println!();
                println!("  Consequence: Ω𝒰₀ (propositions in 𝒰₀) is a complete Boolean algebra.");
                println!("  Classical mathematics can be developed in 𝒰₀ with this axiom.");
            }
            "hierarchy" => {
                println!("{}", bold("── Universe Hierarchy ───────────────────────────────────────────────"));
                println!("        𝒰₀  ⊂  𝒰₁  ⊂  𝒰₂  ⊂  ⋯  (cumulativity)");
                println!();
                println!("  𝒰₀ : 𝒰₁");
                println!("  𝒰₁ : 𝒰₂");
                println!("  𝒰ₙ : 𝒰ₙ₊₁");
                println!();
                let types = [("Bool", 0), ("ℕ", 0), ("𝟙", 0), ("𝟘", 0), ("𝒰₀", 1), ("𝒰₁", 2)];
                for (t, lv) in &types {
                    println!("  {} : 𝒰{}", cyan(t), lv);
                }
                println!();
                println!("  Π(A:𝒰₀).(A→A) : 𝒰₀  (level 0, small type)");
                println!("  𝒰₀ : 𝒰₁              (universe is in next level)");
                println!("  Σ(A:𝒰₀).A : 𝒰₁       (contains 𝒰₀ → one level up)");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Universes — 𝒰₀:𝒰₁:⋯, Girard's Paradox Sandbox      ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore the universe hierarchy in HoTT. Define types at levels,");
    println!("  compute levels of Π and Σ types, and understand why 𝒰 ∉ 𝒰.");
    println!("  Type {} for the hierarchy, {} for Girard's paradox, {} for commands.\n", cyan("hierarchy"), cyan("girard"), cyan("help"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}𝒰{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
