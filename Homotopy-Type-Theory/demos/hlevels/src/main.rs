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

// ── H-level hierarchy ─────────────────────────────────────────────────────────
//
// h-level(A) = -2: contractible   — uniquely inhabited
// h-level(A) = -1: proposition    — all elements equal (proof-irrelevant)
// h-level(A) =  0: set            — all paths are trivial (UIP holds)
// h-level(A) =  1: groupoid       — paths form a set
// h-level(A) =  n: n-groupoid
//
// Equivalently (shifted convention): is-contr=(-2), is-prop=(-1), is-set=(0), ...

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HLevel {
    Contractible,  // -2
    Proposition,   // -1
    Set,           // 0
    Groupoid,      // 1
    TwoGroupoid,   // 2
    Infinity,      // ∞
}

impl HLevel {
    fn name(&self) -> &str {
        match self {
            HLevel::Contractible => "contractible (-2)",
            HLevel::Proposition  => "proposition  (-1)",
            HLevel::Set          => "set          ( 0)",
            HLevel::Groupoid     => "1-groupoid   ( 1)",
            HLevel::TwoGroupoid  => "2-groupoid   ( 2)",
            HLevel::Infinity     => "∞-groupoid   (∞)",
        }
    }

    fn n(&self) -> i64 {
        match self { HLevel::Contractible => -2, HLevel::Proposition => -1,
            HLevel::Set => 0, HLevel::Groupoid => 1, HLevel::TwoGroupoid => 2,
            HLevel::Infinity => i64::MAX }
    }

    fn from_n(n: i64) -> Self {
        match n { i64::MIN..=-2 => HLevel::Contractible, -1 => HLevel::Proposition,
            0 => HLevel::Set, 1 => HLevel::Groupoid, 2 => HLevel::TwoGroupoid, _ => HLevel::Infinity }
    }

    fn description(&self) -> &str {
        match self {
            HLevel::Contractible => "Uniquely inhabited. All elements equal to a center. Identity type is also contractible.",
            HLevel::Proposition  => "At most one element. All proofs of P are equal. 'Proof-irrelevant'.",
            HLevel::Set          => "Elements may differ, but any two paths between elements are equal (UIP).",
            HLevel::Groupoid     => "Paths form a set; homotopies between paths are trivial.",
            HLevel::TwoGroupoid  => "Paths form a 1-groupoid; homotopies may differ but 2-homotopies are trivial.",
            HLevel::Infinity     => "Arbitrarily complex higher structure. General ∞-groupoid.",
        }
    }
}

// ── Type descriptions ─────────────────────────────────────────────────────────

struct TypeInfo {
    name: String,
    level: HLevel,
    reason: String,
}

fn builtin_types() -> Vec<TypeInfo> {
    vec![
        TypeInfo { name: "𝟙 (Unit)".into(), level: HLevel::Contractible, reason: "Unique inhabitant tt; all paths are refl.".into() },
        TypeInfo { name: "𝟘 (Void)".into(), level: HLevel::Contractible, reason: "Empty — vacuously contractible (no elements to distinguish).".into() },
        TypeInfo { name: "⊤ (True)".into(), level: HLevel::Proposition, reason: "Exactly one proof; any two proofs are equal.".into() },
        TypeInfo { name: "⊥ (False)".into(), level: HLevel::Proposition, reason: "No proofs; vacuously all proofs are equal.".into() },
        TypeInfo { name: "P ∧ Q".into(), level: HLevel::Proposition, reason: "If P, Q are props, P∧Q is a prop.".into() },
        TypeInfo { name: "P ∨ Q".into(), level: HLevel::Proposition, reason: "Only if truncated: ‖P∨Q‖₋₁.".into() },
        TypeInfo { name: "ℕ".into(), level: HLevel::Set, reason: "Discrete type; no non-trivial paths between distinct naturals.".into() },
        TypeInfo { name: "ℤ".into(), level: HLevel::Set, reason: "Discrete type; UIP holds.".into() },
        TypeInfo { name: "Bool".into(), level: HLevel::Set, reason: "Finite discrete type; only paths are refl.".into() },
        TypeInfo { name: "S¹".into(), level: HLevel::Groupoid, reason: "π₁(S¹)=ℤ is non-trivial; paths form a set (ℤ), 2-paths are trivial.".into() },
        TypeInfo { name: "S²".into(), level: HLevel::TwoGroupoid, reason: "π₂(S²)=ℤ; 2-paths are non-trivial.".into() },
        TypeInfo { name: "𝒰 (Universe)".into(), level: HLevel::Infinity, reason: "Contains types of all h-levels; has arbitrarily complex path structure.".into() },
        TypeInfo { name: "A → B".into(), level: HLevel::Set, reason: "Function type inherits h-level of B (if B is a set, A→B is a set).".into() },
        TypeInfo { name: "Σ(x:A).B".into(), level: HLevel::Set, reason: "Sigma type: level = max(level(A), level(B(x))).".into() },
    ]
}

// ── Rules for computing h-levels ─────────────────────────────────────────────

fn hlevel_product(a: &HLevel, b: &HLevel) -> HLevel {
    HLevel::from_n(a.n().max(b.n()))
}

fn hlevel_function(_dom: &HLevel, cod: &HLevel) -> HLevel {
    // A → B has the h-level of B (the codomain dominates for functions)
    cod.clone()
}

fn hlevel_sum(a: &HLevel, b: &HLevel) -> HLevel {
    // A + B: level = max(level(A), level(B)) + 1 if both are sets or higher
    // (coproducts raise level by 1 when components aren't props)
    let m = a.n().max(b.n());
    if m <= -1 { HLevel::from_n(-1) } // prop ∨ prop = prop (when truncated)
    else { HLevel::from_n(m + 1) }
}

fn hlevel_identity(a: &HLevel) -> HLevel {
    // The identity type a=b has level one lower than A
    HLevel::from_n(a.n() - 1)
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    types: HashMap<String, HLevel>,
}

impl Sandbox {
    fn new() -> Self { Sandbox { types: HashMap::new() } }

    fn print_help() {
        println!("{}", bold("── H-level Hierarchy ───────────────────────────────────────────────"));
        println!("  {}  n            — explain h-level n", cyan("level"));
        println!("  {}             — show the full ladder", cyan("ladder"));
        println!("  {}  typename    — look up h-level of a built-in type", cyan("check"));
        println!("  {}  list        — list all known types", cyan("types"));
        println!("{}", bold("── Assign and Combine ──────────────────────────────────────────────"));
        println!("  {}  A = set     — assign h-level to a type", cyan("set"));
        println!("  {}  A * B       — h-level of product", cyan("prod"));
        println!("  {}  A -> B      — h-level of function type", cyan("fun"));
        println!("  {}  A + B       — h-level of coproduct", cyan("sum"));
        println!("  {}  A           — h-level of identity type", cyan("id"));
        println!("{}", bold("── H-level names ───────────────────────────────────────────────────"));
        println!("  contr | prop | set | groupoid | 2groupoid | inf");
    }

    fn parse_level(s: &str) -> Option<HLevel> {
        match s.trim() {
            "contr" | "contractible" | "-2" => Some(HLevel::Contractible),
            "prop" | "proposition" | "-1" => Some(HLevel::Proposition),
            "set" | "0" => Some(HLevel::Set),
            "groupoid" | "1" => Some(HLevel::Groupoid),
            "2groupoid" | "2" => Some(HLevel::TwoGroupoid),
            "inf" | "infty" | "omega" => Some(HLevel::Infinity),
            n => n.parse::<i64>().ok().map(HLevel::from_n),
        }
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
            "ladder" => {
                println!("{}", bold("  H-level Ladder in HoTT"));
                println!("  {}", dim("────────────────────────────────────────────────────────────────────"));
                let levels = [HLevel::Contractible, HLevel::Proposition, HLevel::Set,
                    HLevel::Groupoid, HLevel::TwoGroupoid, HLevel::Infinity];
                for lv in &levels {
                    println!("  {} {}", bold(&cyan(&format!("{:28}", lv.name()))), dim(&lv.description()));
                    println!("  {}", dim("┃"));
                }
                println!("  Each level is a subclass of the next.");
                println!("  Contr ⊂ Prop ⊂ Set ⊂ Groupoid ⊂ 2-Groupoid ⊂ … ⊂ ∞-Groupoid");
            }
            "level" => {
                match Self::parse_level(rest) {
                    Some(lv) => {
                        println!("  {} — {}", bold(&cyan(lv.name())), lv.description());
                        println!("  {}", dim("Examples:"));
                        let builtins = builtin_types();
                        for t in builtins.iter().filter(|t| t.level == lv) {
                            println!("    {} : {}", cyan(&t.name), dim(&t.reason));
                        }
                    }
                    None => println!("  {} Valid levels: contr, prop, set, groupoid, 2groupoid, inf, or a number", red("✗")),
                }
            }
            "check" => {
                let builtins = builtin_types();
                let matches: Vec<_> = builtins.iter()
                    .filter(|t| t.name.to_lowercase().contains(&rest.to_lowercase())).collect();
                if matches.is_empty() {
                    // Try user-defined
                    if let Some(lv) = self.types.get(rest) {
                        println!("  {} : {}", cyan(rest), bold(&cyan(lv.name())));
                    } else {
                        println!("  {} Type {rest} not found. Use 'set' to define it.", yellow("?"));
                    }
                } else {
                    for t in matches {
                        println!("  {} : {}", cyan(&t.name), bold(&cyan(t.level.name())));
                        println!("    {}", dim(&t.reason));
                    }
                }
            }
            "types" => {
                println!("{}", bold("  Built-in type h-levels:"));
                for t in builtin_types() {
                    println!("  {:20} : {}", cyan(&t.name), t.level.name());
                }
                if !self.types.is_empty() {
                    println!("{}", bold("  User-defined:"));
                    for (name, lv) in &self.types {
                        println!("  {:20} : {}", cyan(name), lv.name());
                    }
                }
            }
            "set" => {
                let ws: Vec<&str> = rest.splitn(2, ' ').collect();
                if ws.len() < 2 { println!("  {} Use: set TypeName level", red("✗")); return true; }
                match Self::parse_level(ws[1]) {
                    Some(lv) => {
                        println!("  {} : {}", cyan(ws[0]), bold(&cyan(lv.name())));
                        self.types.insert(ws[0].to_string(), lv);
                    }
                    None => println!("  {} Invalid level", red("✗")),
                }
            }
            "prod" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: prod A B", red("✗")); return true; }
                let get = |s: &str| -> Option<HLevel> {
                    self.types.get(s).cloned()
                        .or_else(|| builtin_types().into_iter().find(|t| t.name.to_lowercase().contains(&s.to_lowercase())).map(|t| t.level))
                        .or_else(|| Self::parse_level(s))
                };
                match (get(ws[0]), get(ws[1])) {
                    (Some(a), Some(b)) => {
                        let result = hlevel_product(&a, &b);
                        println!("  level({}) = {}", ws[0], a.name());
                        println!("  level({}) = {}", ws[1], b.name());
                        println!("  level({} × {}) = {}", ws[0], ws[1], bold(&green(result.name())));
                        println!("  {} Product h-level = max of components", dim("Rule:"));
                    }
                    _ => println!("  {} Type(s) not found. Use 'set' to define them.", red("✗")),
                }
            }
            "fun" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: fun A B", red("✗")); return true; }
                let get = |s: &str| self.types.get(s).cloned()
                    .or_else(|| builtin_types().into_iter().find(|t| t.name.to_lowercase().contains(&s.to_lowercase())).map(|t| t.level))
                    .or_else(|| Self::parse_level(s));
                match (get(ws[0]), get(ws[1])) {
                    (Some(a), Some(b)) => {
                        let result = hlevel_function(&a, &b);
                        println!("  level({}) = {}", ws[0], a.name());
                        println!("  level({}) = {}", ws[1], b.name());
                        println!("  level({} → {}) = {}", ws[0], ws[1], bold(&green(result.name())));
                        println!("  {} Function h-level = h-level of codomain", dim("Rule:"));
                    }
                    _ => println!("  {} Type(s) not found.", red("✗")),
                }
            }
            "sum" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: sum A B", red("✗")); return true; }
                let get = |s: &str| self.types.get(s).cloned()
                    .or_else(|| builtin_types().into_iter().find(|t| t.name.to_lowercase().contains(&s.to_lowercase())).map(|t| t.level))
                    .or_else(|| Self::parse_level(s));
                match (get(ws[0]), get(ws[1])) {
                    (Some(a), Some(b)) => {
                        let result = hlevel_sum(&a, &b);
                        println!("  level({}) = {}", ws[0], a.name());
                        println!("  level({}) = {}", ws[1], b.name());
                        println!("  level({} + {}) = {}", ws[0], ws[1], bold(&green(result.name())));
                        println!("  {} Coproduct raises level by 1 above max (for non-props)", dim("Rule:"));
                    }
                    _ => println!("  {} Type(s) not found.", red("✗")),
                }
            }
            "id" => {
                let lv = self.types.get(rest).cloned()
                    .or_else(|| builtin_types().into_iter().find(|t| t.name.to_lowercase().contains(&rest.to_lowercase())).map(|t| t.level))
                    .or_else(|| Self::parse_level(rest));
                match lv {
                    Some(a) => {
                        let result = hlevel_identity(&a);
                        println!("  level({}) = {}", rest, a.name());
                        println!("  level(a = b for a,b:{}) = {}", rest, bold(&green(result.name())));
                        println!("  {} Identity type has level one below the type", dim("Rule:"));
                    }
                    None => println!("  {} Type not found.", red("✗")),
                }
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║     H-level Hierarchy — Truncation Sandbox              ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore the ladder from contractible types to ∞-groupoids.");
    println!("  Assign h-levels to types and compute levels of combinations.");
    println!("  Type {} to see the full ladder, {} for commands.\n", cyan("ladder"), cyan("help"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}hlevel{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
