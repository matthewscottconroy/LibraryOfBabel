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

// ── Formulas ──────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum Formula {
    Top,
    Bot,
    Var(String),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Not(Box<Formula>),
}

impl Formula {
    fn display(&self) -> String {
        match self {
            Formula::Top => "⊤".into(),
            Formula::Bot => "⊥".into(),
            Formula::Var(s) => s.clone(),
            Formula::And(a, b) => format!("{} ∧ {}", a.display_a(), b.display_a()),
            Formula::Or(a, b) => format!("{} ∨ {}", a.display_a(), b.display_a()),
            Formula::Implies(a, b) => format!("{} → {}", a.display_a(), b.display()),
            Formula::Not(a) => format!("¬{}", a.display_a()),
        }
    }

    fn display_a(&self) -> String {
        match self { Formula::Var(_) | Formula::Top | Formula::Bot | Formula::Not(_) => self.display(), _ => format!("({})", self.display()) }
    }

    fn subformulas(&self) -> Vec<Formula> {
        let mut v = vec![self.clone()];
        match self {
            Formula::And(a, b) | Formula::Or(a, b) | Formula::Implies(a, b) => {
                v.extend(a.subformulas()); v.extend(b.subformulas());
            }
            Formula::Not(a) => v.extend(a.subformulas()),
            _ => {}
        }
        v
    }

    fn size(&self) -> usize {
        match self {
            Formula::Top | Formula::Bot | Formula::Var(_) => 1,
            Formula::Not(a) => 1 + a.size(),
            Formula::And(a, b) | Formula::Or(a, b) | Formula::Implies(a, b) => 1 + a.size() + b.size(),
        }
    }
}

fn parse_formula(s: &str) -> Option<Formula> {
    let s = s.trim();
    if let Some(i) = find_op(s, "->") {
        let a = parse_formula(&s[..i])?;
        let b = parse_formula(&s[i+2..])?;
        return Some(Formula::Implies(Box::new(a), Box::new(b)));
    }
    if let Some(i) = find_op(s, "|") {
        let a = parse_formula(&s[..i])?;
        let b = parse_formula(&s[i+1..])?;
        return Some(Formula::Or(Box::new(a), Box::new(b)));
    }
    if let Some(i) = find_op(s, "&") {
        let a = parse_formula(&s[..i])?;
        let b = parse_formula(&s[i+1..])?;
        return Some(Formula::And(Box::new(a), Box::new(b)));
    }
    if s.starts_with('!') || s.starts_with('~') { return Some(Formula::Not(Box::new(parse_formula(&s[1..])?))) }
    if s.starts_with('(') && s.ends_with(')') { return parse_formula(&s[1..s.len()-1]); }
    match s { "T" | "true" => Some(Formula::Top), "F" | "false" => Some(Formula::Bot), v => Some(Formula::Var(v.to_string())) }
}

fn find_op(s: &str, op: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c { '(' => depth += 1, ')' => depth -= 1, _ => {} }
        if depth == 0 && s[i..].starts_with(op) { return Some(i); }
    }
    None
}

// ── Sequent ───────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct Sequent {
    antecedents: Vec<Formula>,
    consequents: Vec<Formula>,
}

impl Sequent {
    fn display(&self) -> String {
        let ant: Vec<_> = self.antecedents.iter().map(|f| f.display()).collect();
        let con: Vec<_> = self.consequents.iter().map(|f| f.display()).collect();
        format!("{} ⊢ {}", ant.join(", "), con.join(", "))
    }

    fn is_provable(&self) -> bool {
        // Check if any formula in antecedents matches one in consequents (axiom rule)
        for a in &self.antecedents {
            for c in &self.consequents {
                if a == c { return true; }
            }
        }
        // Special: ⊤ is always provable on right
        if self.consequents.contains(&Formula::Top) { return true; }
        // ⊥ is always provable on left
        if self.antecedents.contains(&Formula::Bot) { return true; }
        false
    }
}

// ── Natural deduction derivations ─────────────────────────────────────────────

struct Derivation {
    rule: String,
    conclusion: String,
    premises: Vec<String>,
}

impl Derivation {
    fn display(&self) {
        let w = self.conclusion.len().max(self.premises.iter().map(|s| s.len()).max().unwrap_or(0));
        for p in &self.premises { println!("  {}", p); }
        println!("  {}", "─".repeat(w + 4));
        println!("  {}  [{}]", self.conclusion, cyan(&self.rule));
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    context: Vec<Formula>,
    named: HashMap<String, Formula>,
    rules_mode: String,
}

impl Sandbox {
    fn new() -> Self { Sandbox { context: vec![], named: HashMap::new(), rules_mode: "ND".into() } }

    fn print_help() {
        println!("{}", bold("── Formula Entry ───────────────────────────────────────────────────"));
        println!("  {}  P & Q -> R       — parse and analyze a formula", cyan("formula"));
        println!("  {}  A = P & Q        — name a formula", cyan("let"));
        println!("{}", bold("── Sequents ────────────────────────────────────────────────────────"));
        println!("  {}  P, Q |- R        — build and check a sequent", cyan("sequent"));
        println!("  {}  |- P -> P        — check if provable (simple axiom/⊤/⊥ check)", cyan("proves"));
        println!("{}", bold("── Proof Rules ─────────────────────────────────────────────────────"));
        println!("  {}                   — show Natural Deduction rules", cyan("nd"));
        println!("  {}                   — show Sequent Calculus (LK) rules", cyan("lk"));
        println!("  {}  rule             — explain a rule", cyan("rule"));
        println!("{}", bold("── Derivation Examples ─────────────────────────────────────────────"));
        println!("  {}                   — show example derivations", cyan("examples"));
        println!("  {}  P & Q -> P       — build a derivation interactively", cyan("derive"));
    }

    fn show_nd_rules() {
        println!("{}", bold("── Natural Deduction Rules ─────────────────────────────────────────"));
        println!("  {}", cyan("Introduction rules (⊢ ___):"));
        println!("    ──────  ⊤-I          P  Q             P");
        println!("     ⊢ ⊤              ───────  ∧-I      ─────  ∨-I_l");
        println!("                       ⊢ P∧Q             ⊢ P∨Q");
        println!();
        println!("    [x:P]              [x:P]");
        println!("     Q                 ⊥");
        println!("    ──────  →-I       ──────  ⊥-E (ex falso)");
        println!("    ⊢ P→Q              ⊢ Q");
        println!();
        println!("  {}", cyan("Elimination rules (___ ⊢):"));
        println!("    P∧Q               P∧Q             P→Q   P");
        println!("    ───  ∧-E_1        ───  ∧-E_2      ─────────  →-E (MP)");
        println!("     P                 Q                   Q");
        println!();
        println!("    P∨Q  [P]Q  [Q]R");
        println!("    ───────────────  ∨-E (case analysis)");
        println!("            R");
    }

    fn show_lk_rules() {
        println!("{}", bold("── Sequent Calculus LK Rules ───────────────────────────────────────"));
        println!("  {}", cyan("Structural rules:"));
        println!("    Axiom:   Γ,A ⊢ A,Δ");
        println!("    Cut:     Γ ⊢ A,Δ    Γ',A ⊢ Δ'  ⟹  Γ,Γ' ⊢ Δ,Δ'");
        println!("    Weakening, Contraction, Exchange (structural)");
        println!();
        println!("  {}", cyan("Left rules (introduce in antecedent):"));
        println!("    ∧-L:  Γ,A,B ⊢ Δ  ⟹  Γ,A∧B ⊢ Δ");
        println!("    ∨-L:  Γ,A ⊢ Δ  and  Γ,B ⊢ Δ  ⟹  Γ,A∨B ⊢ Δ");
        println!("    →-L:  Γ ⊢ A,Δ  and  Γ,B ⊢ Δ  ⟹  Γ,A→B ⊢ Δ");
        println!("    ¬-L:  Γ ⊢ A,Δ  ⟹  Γ,¬A ⊢ Δ");
        println!();
        println!("  {}", cyan("Right rules (introduce in consequent):"));
        println!("    ∧-R:  Γ ⊢ A,Δ  and  Γ ⊢ B,Δ  ⟹  Γ ⊢ A∧B,Δ");
        println!("    ∨-R:  Γ ⊢ A,B,Δ  ⟹  Γ ⊢ A∨B,Δ");
        println!("    →-R:  Γ,A ⊢ B,Δ  ⟹  Γ ⊢ A→B,Δ");
        println!("    ¬-R:  Γ,A ⊢ Δ  ⟹  Γ ⊢ ¬A,Δ");
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
            "formula" => {
                match parse_formula(rest) {
                    Some(f) => {
                        println!("  Formula: {}", cyan(&f.display()));
                        println!("  Size: {}  Subformulas: {}", f.size(), f.subformulas().len());
                        let subs: Vec<_> = f.subformulas().iter().map(|s| s.display()).collect();
                        println!("  Subformulas: {}", subs.join(", "));
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "let" => {
                if let Some((name, expr)) = rest.split_once('=') {
                    match parse_formula(expr.trim()) {
                        Some(f) => {
                            println!("  {} = {}", cyan(name.trim()), f.display());
                            self.named.insert(name.trim().to_string(), f);
                        }
                        None => println!("  {} Parse error", red("✗")),
                    }
                }
            }
            "sequent" => {
                let parts: Vec<&str> = rest.splitn(2, "|-").collect();
                if parts.len() < 2 { println!("  {} Use: sequent P, Q |- R", red("✗")); return true; }
                let ants: Vec<Formula> = parts[0].split(',').filter_map(|s| parse_formula(s.trim())).collect();
                let cons: Vec<Formula> = parts[1].split(',').filter_map(|s| parse_formula(s.trim())).collect();
                let seq = Sequent { antecedents: ants, consequents: cons };
                println!("  {}", seq.display());
                if seq.is_provable() {
                    println!("  {} Provable (by axiom/⊤/⊥ rule)", green("✓"));
                } else {
                    println!("  {} Cannot establish provability automatically (manual derivation needed)", yellow("?"));
                }
            }
            "proves" | "check" => {
                let parts: Vec<&str> = rest.splitn(2, "|-").collect();
                let (ants, cons_str) = if parts.len() >= 2 { (parts[0], parts[1]) } else { ("", rest) };
                let ant_fs: Vec<Formula> = ants.split(',').filter_map(|s| { let s = s.trim(); if s.is_empty() { None } else { parse_formula(s) } }).collect();
                match parse_formula(cons_str.trim()) {
                    Some(goal) => {
                        let seq = Sequent { antecedents: ant_fs, consequents: vec![goal.clone()] };
                        println!("  {}", seq.display());
                        if seq.is_provable() { println!("  {} Provable (axiom/⊤/⊥)", green("✓")); }
                        else { println!("  {} Not immediately obvious — would need full derivation", yellow("?")); }
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "nd" => Self::show_nd_rules(),
            "lk" => Self::show_lk_rules(),
            "rule" => {
                match rest {
                    "mp" | "→-E" | "modus ponens" => {
                        println!("  {} (Modus Ponens / →-Elimination):", bold("→-E"));
                        println!("    P → Q     P");
                        println!("    ─────────────");
                        println!("          Q");
                        println!("  If we have P→Q and a proof of P, we get a proof of Q.");
                        println!("  λ-term: if f : P→Q and a : P, then f(a) : Q");
                    }
                    "→-I" | "intro" => {
                        println!("  {} (→-Introduction):", bold("→-I"));
                        println!("    [x : P]");
                        println!("       Q");
                        println!("    ───────");
                        println!("    P → Q");
                        println!("  Assume P (as hypothesis x), derive Q, then discharge hypothesis.");
                        println!("  λ-term: λx. proof_of_Q : P → Q");
                    }
                    "∧-I" | "pair" => {
                        println!("  {} (∧-Introduction):", bold("∧-I"));
                        println!("    P   Q");
                        println!("    ─────");
                        println!("    P ∧ Q");
                        println!("  Have proofs of both P and Q — construct the pair.");
                        println!("  λ-term: (p, q) : P × Q");
                    }
                    "cut" => {
                        println!("  {} (Cut rule in LK):", bold("Cut"));
                        println!("    Γ ⊢ A, Δ      Γ', A ⊢ Δ'");
                        println!("    ───────────────────────────");
                        println!("         Γ, Γ' ⊢ Δ, Δ'");
                        println!("  Key: Cut-elimination theorem (Gentzen) shows cut is admissible.");
                        println!("  This gives subformula property: every derivation without cut");
                        println!("  only uses subformulas of the goal — the basis of proof search.");
                    }
                    _ => println!("  {} Rules: mp, →-I, ∧-I, cut", red("✗")),
                }
            }
            "derive" => {
                match parse_formula(rest) {
                    Some(f) => {
                        println!("  {}", bold(&format!("Deriving: {}", f.display())));
                        // Show a schematic derivation based on the formula shape
                        let deriv = make_derivation(&f);
                        deriv.display();
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "examples" => {
                println!("{}", bold("── Example Derivations ─────────────────────────────────────────────"));
                println!("  {}", cyan("Identity (P → P):"));
                println!("    [x : P]         (hypothesis)");
                println!("    ────────  →-I  (discharge x)");
                println!("    P → P");
                println!();
                println!("  {}", cyan("Modus Ponens application:"));
                println!("    P → Q   P       (both in context)");
                println!("    ─────────  →-E");
                println!("        Q");
                println!();
                println!("  {}", cyan("And-Elim (P ∧ Q → P):"));
                println!("    [h : P∧Q]        (hypothesis)");
                println!("    ──────────  ∧-E_1");
                println!("    P");
                println!("    ──────────────  →-I (discharge h)");
                println!("    P∧Q → P");
                println!();
                println!("  {} Try:", bold("Interactive:"));
                println!("  derive P & Q -> P");
                println!("  derive P -> P");
                println!("  derive (P -> Q) -> (Q -> R) -> P -> R");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn make_derivation(f: &Formula) -> Derivation {
    match f {
        Formula::Implies(a, b) if *a == *b => {
            Derivation {
                rule: "→-I".into(),
                conclusion: format!("⊢ {} → {}", a.display(), b.display()),
                premises: vec![format!("[x : {}]  ─────  Ax", a.display()), format!("⊢ {}", b.display())],
            }
        }
        Formula::Implies(a, b) => {
            Derivation {
                rule: "→-I".into(),
                conclusion: format!("⊢ {} → {}", a.display(), b.display()),
                premises: vec![format!("[h : {}], ... ⊢ {}", a.display(), b.display())],
            }
        }
        Formula::And(a, b) => {
            Derivation {
                rule: "∧-I".into(),
                conclusion: format!("⊢ {} ∧ {}", a.display(), b.display()),
                premises: vec![format!("⊢ {}", a.display()), format!("⊢ {}", b.display())],
            }
        }
        _ => {
            Derivation {
                rule: "Ax?".into(),
                conclusion: format!("⊢ {}", f.display()),
                premises: vec!["(need hypothesis or subderivations)".into()],
            }
        }
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Proof Theory — Natural Deduction & Sequent Calculus   ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore proof systems: Natural Deduction (ND) and Sequent Calculus (LK).");
    println!("  Build sequents, check provability, and see derivation rules in action.");
    println!("  Type {} for commands, {} for rules, {} for examples.\n", cyan("help"), cyan("nd"), cyan("examples"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}proof{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
