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

// ── BHK (Brouwer-Heyting-Kolmogorov) interpretation ──────────────────────────
//
// Under BHK, a proof of a proposition is a construction:
//   - proof of P ∧ Q = pair (proof of P, proof of Q)
//   - proof of P ∨ Q = choice of P or Q + proof of chosen
//   - proof of P → Q = function converting proofs of P to proofs of Q
//   - proof of ¬P   = function converting any proof of P to absurdity
//   - proof of ∀x.P(x) = function assigning proof of P(a) to each a
//   - proof of ∃x.P(x) = pair (witness a, proof of P(a))

// ── Proof objects ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
enum Proof {
    Trivial,                        // proof of ⊤
    Pair(Box<Proof>, Box<Proof>),   // proof of A ∧ B
    Left(Box<Proof>),               // proof of A ∨ B via A
    Right(Box<Proof>),              // proof of A ∨ B via B
    Fun(String, Box<Proof>),        // proof of A → B: λx.proof
    Apply(Box<Proof>, Box<Proof>),  // apply function proof to argument proof
    Witness(String, Box<Proof>),    // proof of ∃x.P: (witness, proof)
    Named(String),                  // a named proof object
    Var(String),                    // a proof variable (hypothesis)
}

impl Proof {
    fn display(&self) -> String {
        match self {
            Proof::Trivial => "()".into(),
            Proof::Pair(a, b) => format!("⟨{}, {}⟩", a.display(), b.display()),
            Proof::Left(p) => format!("left({})", p.display()),
            Proof::Right(p) => format!("right({})", p.display()),
            Proof::Fun(x, b) => format!("λ{}. {}", x, b.display()),
            Proof::Apply(f, a) => format!("{} ∘ {}", f.display(), a.display()),
            Proof::Witness(w, p) => format!("⟨{}, {}⟩", w, p.display()),
            Proof::Named(s) | Proof::Var(s) => s.clone(),
        }
    }
}

// ── Proposition ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum Prop {
    Top,
    Bot,
    Var(String),
    And(Box<Prop>, Box<Prop>),
    Or(Box<Prop>, Box<Prop>),
    Implies(Box<Prop>, Box<Prop>),
    Not(Box<Prop>),
    Forall(String, Box<Prop>),
    Exists(String, Box<Prop>),
}

impl Prop {
    fn display(&self) -> String {
        match self {
            Prop::Top => "⊤".into(),
            Prop::Bot => "⊥".into(),
            Prop::Var(s) => s.clone(),
            Prop::And(a, b) => format!("{} ∧ {}", a.display_atom(), b.display_atom()),
            Prop::Or(a, b) => format!("{} ∨ {}", a.display_atom(), b.display_atom()),
            Prop::Implies(a, b) => format!("{} → {}", a.display_atom(), b.display()),
            Prop::Not(a) => format!("¬{}", a.display_atom()),
            Prop::Forall(x, p) => format!("∀{x}. {}", p.display()),
            Prop::Exists(x, p) => format!("∃{x}. {}", p.display()),
        }
    }

    fn display_atom(&self) -> String {
        match self { Prop::Top | Prop::Bot | Prop::Var(_) => self.display(), _ => format!("({})", self.display()) }
    }

    fn bhk_construction(&self) -> String {
        match self {
            Prop::Top => "The trivial construction ()".into(),
            Prop::Bot => "No construction exists (⊥ is unprovable)".into(),
            Prop::Var(s) => format!("A given proof of {s}"),
            Prop::And(a, b) => format!("A pair ⟨p, q⟩ where p proves {} and q proves {}", a.display(), b.display()),
            Prop::Or(a, b) => format!("Either left(p) where p proves {} or right(q) where q proves {}", a.display(), b.display()),
            Prop::Implies(a, b) => format!("A function f that converts any proof of {} into a proof of {}", a.display(), b.display()),
            Prop::Not(a) => format!("A function f that converts any proof of {} into a proof of ⊥", a.display()),
            Prop::Forall(x, p) => format!("A function assigning to each {x} a proof of {}", p.display()),
            Prop::Exists(_x, p) => format!("A pair ⟨a, p⟩ where a is the witness and p proves {}", p.display()),
        }
    }

    fn is_classical(&self) -> Option<&str> {
        match self {
            Prop::Or(a, b) => {
                if let (Prop::Var(va), Prop::Not(nb)) = (a.as_ref(), b.as_ref()) {
                    if let Prop::Var(vb) = nb.as_ref() {
                        if va == vb { return Some("LEM (P ∨ ¬P) — not provable intuitionistically"); }
                    }
                }
                None
            }
            Prop::Implies(a, _b) => {
                if let Prop::Not(na) = a.as_ref() {
                    if let Prop::Not(_) = na.as_ref() {
                        return Some("¬¬P → P (double negation elim) — classically valid, not intuitionistically");
                    }
                }
                None
            }
            _ => None,
        }
    }
}

fn parse_prop(s: &str) -> Option<Prop> {
    let s = s.trim();
    // Implication (right-assoc)
    if let Some(i) = find_op_str(s, "->") {
        let a = parse_prop(&s[..i])?;
        let b = parse_prop(&s[i+2..])?;
        return Some(Prop::Implies(Box::new(a), Box::new(b)));
    }
    // Or
    if let Some(i) = find_op_str(s, "|") {
        let a = parse_prop(&s[..i])?;
        let b = parse_prop(&s[i+1..])?;
        return Some(Prop::Or(Box::new(a), Box::new(b)));
    }
    // And
    if let Some(i) = find_op_str(s, "&") {
        let a = parse_prop(&s[..i])?;
        let b = parse_prop(&s[i+1..])?;
        return Some(Prop::And(Box::new(a), Box::new(b)));
    }
    // Not
    if s.starts_with('!') || s.starts_with('~') { return Some(Prop::Not(Box::new(parse_prop(&s[1..])?))) }
    if s.starts_with("not ") { return Some(Prop::Not(Box::new(parse_prop(&s[4..])?))) }
    // Quantifiers
    if s.starts_with("forall ") || s.starts_with("all ") {
        let rest = s.trim_start_matches("forall ").trim_start_matches("all ");
        let dot = rest.find('.')?;
        let x = rest[..dot].trim().to_string();
        let body = parse_prop(rest[dot+1..].trim())?;
        return Some(Prop::Forall(x, Box::new(body)));
    }
    if s.starts_with("exists ") || s.starts_with("ex ") {
        let rest = s.trim_start_matches("exists ").trim_start_matches("ex ");
        let dot = rest.find('.')?;
        let x = rest[..dot].trim().to_string();
        let body = parse_prop(rest[dot+1..].trim())?;
        return Some(Prop::Exists(x, Box::new(body)));
    }
    // Parens
    if s.starts_with('(') && s.ends_with(')') { return parse_prop(&s[1..s.len()-1]); }
    match s {
        "T" | "true" | "True" | "top" => Some(Prop::Top),
        "F" | "false" | "False" | "bot" => Some(Prop::Bot),
        v if v.chars().all(|c| c.is_alphanumeric() || c == '_') => Some(Prop::Var(v.to_string())),
        _ => None,
    }
}

fn find_op_str(s: &str, op: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c { '(' => depth += 1, ')' => depth -= 1, _ => {} }
        if depth == 0 && s[i..].starts_with(op) { return Some(i); }
    }
    None
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    proofs: HashMap<String, (Prop, Proof)>,
    hypotheses: HashMap<String, Prop>,
}

impl Sandbox {
    fn new() -> Self { Sandbox { proofs: HashMap::new(), hypotheses: HashMap::new() } }

    fn print_help() {
        println!("{}", bold("── BHK Interpretation ──────────────────────────────────────────────"));
        println!("  {}  P & Q           — explain BHK construction for P", cyan("bhk"));
        println!("  {}  P               — analyze whether P is intuitionistically provable", cyan("check"));
        println!("{}", bold("── Build Proofs ────────────────────────────────────────────────────"));
        println!("  {}  p P            — assume proof p of proposition P", cyan("assume"));
        println!("  {}  name P proof   — record proof of proposition", cyan("prove"));
        println!("  {}  p q            — introduce a pair (proof of P∧Q)", cyan("pair"));
        println!("  {}  p              — left injection (proof of P∨Q from P)", cyan("left"));
        println!("  {}  p              — right injection", cyan("right"));
        println!("  {}  f a            — apply function proof to argument", cyan("apply"));
        println!("{}", bold("── Classical vs Intuitionistic ─────────────────────────────────────"));
        println!("  {}          — compare classical and intuitionistic logic", cyan("classical"));
        println!("  {}  P ∨ ¬P         — try to prove LEM (will show impossibility)", cyan("lem"));
        println!("{}", bold("── Syntax ──────────────────────────────────────────────────────────"));
        println!("  P & Q   P | Q   !P   P -> Q   forall x. P   exists x. P");
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
            "bhk" => {
                match parse_prop(rest) {
                    Some(p) => {
                        println!("  {}", bold("BHK Construction:"));
                        println!("  Proposition: {}", cyan(&p.display()));
                        println!("  A proof is: {}", p.bhk_construction());
                        if let Some(warning) = p.is_classical() {
                            println!("  {} {}", yellow("⚠"), warning);
                        }
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "check" => {
                match parse_prop(rest) {
                    Some(p) => {
                        println!("  Proposition: {}", cyan(&p.display()));
                        match p.is_classical() {
                            Some(msg) => println!("  {} {}", yellow("Classical only:"), msg),
                            None => match &p {
                                Prop::Top => println!("  {} Trivially provable: () : ⊤", green("✓")),
                                Prop::Bot => println!("  {} Not provable: ⊥ has no proof", red("✗")),
                                Prop::Implies(a, b) if a.as_ref() == b.as_ref() => {
                                    println!("  {} Tautology A→A: proof = λx.x (identity)", green("✓"));
                                }
                                _ => println!("  {} Depends on hypotheses — use 'assume' then 'prove'", yellow("?")),
                            }
                        }
                        println!("  BHK: {}", p.bhk_construction());
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "assume" => {
                let ws: Vec<&str> = rest.splitn(2, ' ').collect();
                if ws.len() < 2 { println!("  {} Use: assume name Proposition", red("✗")); return true; }
                match parse_prop(ws[1]) {
                    Some(p) => {
                        println!("  {} : {} assumed", cyan(ws[0]), p.display());
                        self.hypotheses.insert(ws[0].to_string(), p);
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "prove" => {
                let ws: Vec<&str> = rest.splitn(3, ' ').collect();
                if ws.len() < 2 { println!("  {} Use: prove name Proposition [proof-term]", red("✗")); return true; }
                let (name, prop_str) = (ws[0], ws[1]);
                let proof_str = ws.get(2).copied().unwrap_or("(provided)");
                match parse_prop(prop_str) {
                    Some(p) => {
                        let proof = Proof::Named(proof_str.to_string());
                        println!("  {} : {}", cyan(name), p.display());
                        println!("  Construction: {}", proof.display());
                        println!("  BHK meaning:  {}", p.bhk_construction());
                        self.proofs.insert(name.to_string(), (p, proof));
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "pair" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: pair p q", red("✗")); return true; }
                let hp = self.hypotheses.get(ws[0]).or_else(|| self.proofs.get(ws[0]).map(|(p,_)| p)).cloned();
                let hq = self.hypotheses.get(ws[1]).or_else(|| self.proofs.get(ws[1]).map(|(p,_)| p)).cloned();
                match (hp, hq) {
                    (Some(p), Some(q)) => {
                        let conj = Prop::And(Box::new(p.clone()), Box::new(q.clone()));
                        println!("  ⟨{}, {}⟩ : {} ∧ {}", ws[0], ws[1], p.display(), q.display());
                        println!("  This is a proof of: {}", cyan(&conj.display()));
                        println!("  BHK: A pair — both components must be proofs.");
                    }
                    _ => println!("  {} Proof(s) not found. Use 'assume' first.", red("✗")),
                }
            }
            "left" | "right" => {
                let hp = self.hypotheses.get(rest).or_else(|| self.proofs.get(rest).map(|(p,_)| p)).cloned();
                match hp {
                    Some(p) => {
                        if cmd == "left" {
                            println!("  left({}) : {} ∨ B   (for any B)", rest, p.display());
                            println!("  BHK: We chose the left disjunct, providing a proof of {}.", p.display());
                        } else {
                            println!("  right({}) : A ∨ {}   (for any A)", rest, p.display());
                            println!("  BHK: We chose the right disjunct, providing a proof of {}.", p.display());
                        }
                    }
                    None => println!("  {} Proof {} not found", red("✗"), rest),
                }
            }
            "apply" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: apply f a", red("✗")); return true; }
                let hf = self.hypotheses.get(ws[0]).or_else(|| self.proofs.get(ws[0]).map(|(p,_)| p)).cloned();
                let ha = self.hypotheses.get(ws[1]).or_else(|| self.proofs.get(ws[1]).map(|(p,_)| p)).cloned();
                match (hf, ha) {
                    (Some(Prop::Implies(a, b)), Some(pa)) if *a == pa => {
                        println!("  {} applied to {} : {}", ws[0], ws[1], b.display());
                        println!("  BHK: The function {} converts proof of {} to proof of {}.", ws[0], a.display(), b.display());
                    }
                    (Some(Prop::Implies(a, _)), Some(pa)) => {
                        println!("  {} Type mismatch: {} expects {} but got {}", red("✗"), ws[0], a.display(), pa.display());
                    }
                    _ => println!("  {} Proof(s) not found or not an implication", red("✗")),
                }
            }
            "classical" => {
                println!("{}", bold("── Classical vs Intuitionistic Logic ───────────────────────────────"));
                println!("  {}", bold("Intuitionistic (constructive) logic requires:"));
                println!("  A proof must be an explicit construction.");
                println!("  You cannot assume 'either P or ¬P' without a witness.");
                println!();
                println!("  {} Classically valid but NOT intuitionistically:", bold("Examples:"));
                let classical = [
                    ("P ∨ ¬P", "LEM (Excluded Middle)"),
                    ("¬¬P → P", "Double Negation Elimination"),
                    ("((P → Q) → P) → P", "Peirce's Law"),
                    ("¬(P ∧ Q) → ¬P ∨ ¬Q", "de Morgan's Law (classical form)"),
                ];
                for (prop, name) in &classical {
                    println!("  {} {} — {}",  red("✗"), cyan(prop), dim(name));
                }
                println!();
                println!("  {} Always intuitionistically valid:", bold("Examples:"));
                let intuit = [
                    ("P → ¬¬P",   "Double negation introduction"),
                    ("¬(P ∨ Q) ↔ (¬P ∧ ¬Q)", "de Morgan's Law (constructive form)"),
                    ("P ∧ (Q ∨ R) ↔ (P∧Q) ∨ (P∧R)", "Distributivity"),
                    ("(P → Q → R) ↔ (P∧Q → R)", "Currying"),
                ];
                for (prop, name) in &intuit {
                    println!("  {} {} — {}", green("✓"), cyan(prop), dim(name));
                }
            }
            "lem" => {
                println!("  {} P ∨ ¬P  (Law of Excluded Middle)", bold("Trying to prove:"));
                println!();
                println!("  Under BHK, we need either:");
                println!("    left(proof_of_P)    — but we don't have a proof of P");
                println!("    right(proof_of_¬P)  — but we don't have a refutation either");
                println!();
                println!("  Without knowing which case holds, we CANNOT construct a proof.");
                println!("  LEM is not intuitionistically valid!");
                println!();
                println!("  In HoTT: LEM is not provable in general, but it can be added");
                println!("  as an axiom (giving classical HoTT) or follows for decidable types.");
                println!();
                println!("  However: {}  IS provable intuitionistically.", cyan("P → ¬¬P"));
                println!("  Proof: λp. λf. f p   (apply refutation f to our proof p)");
            }
            "show" => {
                println!("{}", bold("  Proofs:")); for (n, (p, t)) in &self.proofs { println!("  {} : {}  [{}]", cyan(n), p.display(), t.display()); }
                println!("{}", bold("  Hypotheses:")); for (n, p) in &self.hypotheses { println!("  {} : {}", cyan(n), p.display()); }
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    BHK Interpretation — Constructive Logic Sandbox       ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore the Brouwer-Heyting-Kolmogorov interpretation of logic.");
    println!("  Every proof is a construction. Build proof objects step by step.");
    println!("  Compare classical and intuitionistic validity.");
    println!("  Type {} for commands, {} for the classical comparison.\n", cyan("help"), cyan("classical"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}BHK{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
