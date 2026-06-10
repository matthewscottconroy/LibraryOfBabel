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

// ── Curry-Howard correspondence ───────────────────────────────────────────────
//
// Propositions = Types, Proofs = Programs
//
// We build a small bidirectional explorer:
//   - Enter a logical formula → get the corresponding type + term skeleton
//   - Enter a type → get the corresponding proposition
//   - Enter a term → get its type/proposition and check if it's a proof

// ── Types / Propositions ──────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum Ty {
    Unit,                           // ⊤ / True
    Void,                           // ⊥ / False
    Var(String),                    // A, B, ...
    Prod(Box<Ty>, Box<Ty>),         // A × B / P ∧ Q
    Sum(Box<Ty>, Box<Ty>),          // A + B / P ∨ Q
    Fun(Box<Ty>, Box<Ty>),          // A → B / P ⊃ Q
    Not(Box<Ty>),                   // ¬A = A → ⊥
}

impl Ty {
    fn as_prop(&self) -> String {
        match self {
            Ty::Unit => "⊤".into(),
            Ty::Void => "⊥".into(),
            Ty::Var(s) => s.clone(),
            Ty::Prod(a, b) => format!("{} ∧ {}", a.as_prop_atom(), b.as_prop_atom()),
            Ty::Sum(a, b) => format!("{} ∨ {}", a.as_prop_atom(), b.as_prop_atom()),
            Ty::Fun(a, b) => format!("{} ⊃ {}", a.as_prop_atom(), b.as_prop()),
            Ty::Not(a) => format!("¬{}", a.as_prop_atom()),
        }
    }

    fn as_type(&self) -> String {
        match self {
            Ty::Unit => "𝟙".into(),
            Ty::Void => "𝟘".into(),
            Ty::Var(s) => s.clone(),
            Ty::Prod(a, b) => format!("{} × {}", a.as_type_atom(), b.as_type_atom()),
            Ty::Sum(a, b) => format!("{} + {}", a.as_type_atom(), b.as_type_atom()),
            Ty::Fun(a, b) => format!("{} → {}", a.as_type_atom(), b.as_type()),
            Ty::Not(a) => format!("{} → 𝟘", a.as_type_atom()),
        }
    }

    fn as_prop_atom(&self) -> String {
        match self { Ty::Unit | Ty::Void | Ty::Var(_) | Ty::Not(_) => self.as_prop(), _ => format!("({})", self.as_prop()) }
    }
    fn as_type_atom(&self) -> String {
        match self { Ty::Unit | Ty::Void | Ty::Var(_) => self.as_type(), _ => format!("({})", self.as_type()) }
    }

    fn proof_term(&self) -> String {
        match self {
            Ty::Unit => "tt".into(),
            Ty::Void => "absurd".into(),
            Ty::Var(s) => format!("proof_{s}"),
            Ty::Prod(a, b) => format!("({}, {})", a.proof_term(), b.proof_term()),
            Ty::Sum(a, _) => format!("inl({})", a.proof_term()),
            Ty::Fun(a, b) => {
                let param = match a.as_ref() { Ty::Var(s) => s.clone(), _ => "h".into() };
                format!("λ{param}. {}", b.proof_term())
            }
            Ty::Not(a) => {
                let param = match a.as_ref() { Ty::Var(s) => s.clone(), _ => "h".into() };
                format!("λ{param}. absurd")
            }
        }
    }
}

fn parse_ty(s: &str) -> Option<Ty> {
    let s = s.trim();
    // → (right-assoc, lowest precedence)
    if let Some(i) = find_arrow(s) {
        let a = parse_ty(&s[..i])?;
        let b = parse_ty(&s[i+2..])?;
        return Some(Ty::Fun(Box::new(a), Box::new(b)));
    }
    // × or *
    for sep in [" x ", " * ", " × "] {
        if let Some(i) = s.find(sep) {
            let a = parse_ty(&s[..i])?;
            let b = parse_ty(&s[i+sep.len()..])?;
            return Some(Ty::Prod(Box::new(a), Box::new(b)));
        }
    }
    // +
    if let Some(i) = s.find(" + ") {
        let a = parse_ty(&s[..i])?;
        let b = parse_ty(&s[i+3..])?;
        return Some(Ty::Sum(Box::new(a), Box::new(b)));
    }
    // ¬ or Not
    if s.starts_with('!') || s.starts_with('~') {
        let a = parse_ty(&s[1..])?;
        return Some(Ty::Not(Box::new(a)));
    }
    // Parens
    if s.starts_with('(') && s.ends_with(')') { return parse_ty(&s[1..s.len()-1]); }
    match s {
        "1" | "Unit" | "True" | "T" | "top" => Some(Ty::Unit),
        "0" | "Void" | "False" | "F" | "bot" | "Empty" => Some(Ty::Void),
        v if v.chars().all(|c| c.is_alphanumeric() || c == '_') => Some(Ty::Var(v.to_string())),
        _ => None,
    }
}

fn find_arrow(s: &str) -> Option<usize> {
    let mut depth = 0;
    let bytes = s.as_bytes();
    for i in 0..bytes.len().saturating_sub(1) {
        match bytes[i] { b'(' => depth += 1, b')' => depth -= 1, _ => {} }
        if depth == 0 && bytes[i] == b'-' && bytes[i+1] == b'>' { return Some(i); }
    }
    None
}

// ── Term checking ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
enum Term {
    Var(String),
    Tt,                             // tt : 𝟙
    Pair(Box<Term>, Box<Term>),
    Fst(Box<Term>),
    Snd(Box<Term>),
    Inl(Box<Term>),
    Inr(Box<Term>),
    Lam(String, Box<Term>),
    App(Box<Term>, Box<Term>),
    Absurd(Box<Term>),
}

impl Term {
    fn display(&self) -> String {
        match self {
            Term::Var(s) => s.clone(),
            Term::Tt => "tt".into(),
            Term::Pair(a, b) => format!("({}, {})", a.display(), b.display()),
            Term::Fst(p) => format!("fst({})", p.display()),
            Term::Snd(p) => format!("snd({})", p.display()),
            Term::Inl(a) => format!("inl({})", a.display()),
            Term::Inr(b) => format!("inr({})", b.display()),
            Term::Lam(x, b) => format!("λ{x}. {}", b.display()),
            Term::App(f, a) => format!("({} {})", f.display(), a.display()),
            Term::Absurd(e) => format!("absurd({})", e.display()),
        }
    }

    fn infer(&self, ctx: &HashMap<String, Ty>) -> Option<Ty> {
        match self {
            Term::Var(s) => ctx.get(s).cloned(),
            Term::Tt => Some(Ty::Unit),
            Term::Pair(a, b) => {
                let ta = a.infer(ctx)?;
                let tb = b.infer(ctx)?;
                Some(Ty::Prod(Box::new(ta), Box::new(tb)))
            }
            Term::Fst(p) => {
                if let Ty::Prod(a, _) = p.infer(ctx)? { Some(*a) } else { None }
            }
            Term::Snd(p) => {
                if let Ty::Prod(_, b) = p.infer(ctx)? { Some(*b) } else { None }
            }
            Term::Inl(a) => {
                let ta = a.infer(ctx)?;
                Some(Ty::Sum(Box::new(ta), Box::new(Ty::Var("B".into()))))
            }
            Term::Inr(b) => {
                let tb = b.infer(ctx)?;
                Some(Ty::Sum(Box::new(Ty::Var("A".into())), Box::new(tb)))
            }
            Term::Lam(x, body) => {
                // Can't infer without annotation — return placeholder
                let _ = (x, body);
                None
            }
            Term::App(f, a) => {
                if let Ty::Fun(dom, cod) = f.infer(ctx)? {
                    let ta = a.infer(ctx)?;
                    if ta == *dom { Some(*cod) } else { None }
                } else { None }
            }
            Term::Absurd(_) => None, // needs target type
        }
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    types: HashMap<String, Ty>,
    ctx: HashMap<String, Ty>,
}

impl Sandbox {
    fn new() -> Self { Sandbox { types: HashMap::new(), ctx: HashMap::new() } }

    fn print_help() {
        println!("{}", bold("── Type ↔ Proposition ──────────────────────────────────────────────"));
        println!("  {}  A -> B           — analyze type as proposition", cyan("type"));
        println!("  {}  P & Q            — analyze proposition as type", cyan("prop"));
        println!("  {}  A -> B           — show the proof term skeleton", cyan("term"));
        println!("{}", bold("── Context ─────────────────────────────────────────────────────────"));
        println!("  {}  x A             — add x:A to context", cyan("assume"));
        println!("  {}                   — show current context", cyan("ctx"));
        println!("{}", bold("── Correspondence Table ────────────────────────────────────────────"));
        println!("  {}                   — show the full C-H table", cyan("table"));
        println!("  {}         — show classic tautologies + their types", cyan("tautologies"));
        println!("{}", bold("── Syntax ──────────────────────────────────────────────────────────"));
        println!("  A -> B   A * B   A + B   !A   1 (Unit)   0 (Void)");
    }

    fn show_type(&self, ty: &Ty) {
        println!("  {}", bold("Curry-Howard Correspondence:"));
        println!("  Proposition:  {}", cyan(&ty.as_prop()));
        println!("  Type:         {}", cyan(&ty.as_type()));
        println!("  Proof/Term:   {}", dim(&ty.proof_term()));
        // Discuss the structure
        match ty {
            Ty::Unit => println!("  {} Always provable — the trivial proposition.", green("→")),
            Ty::Void => println!("  {} Unprovable — no inhabitant of the empty type.", red("→")),
            Ty::Fun(_, _) => println!("  {} Proofs are functions — introduce via λ-abstraction.", green("→")),
            Ty::Prod(_, _) => println!("  {} Proofs are pairs — introduce both components.", green("→")),
            Ty::Sum(_, _) => println!("  {} Proofs pick a side — inl or inr.", yellow("→")),
            Ty::Not(_) => println!("  {} Refutation = function to 𝟘.", green("→")),
            Ty::Var(_) => println!("  {} A hypothesis — provable only if assumed.", yellow("→")),
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
            "type" | "prop" => {
                // Both do the same: parse a type/prop and show correspondence
                match parse_ty(rest) {
                    Some(ty) => self.show_type(&ty),
                    None => println!("  {} Parse error in: {rest}", red("✗")),
                }
            }
            "term" => {
                match parse_ty(rest) {
                    Some(ty) => {
                        println!("  To prove: {}", ty.as_prop());
                        println!("  Type:     {}", ty.as_type());
                        println!("  Skeleton: {}", bold(&cyan(&ty.proof_term())));
                        println!("  {} Fill in the _ holes with appropriate proofs", dim("Hint:"));
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "assume" => {
                let ws: Vec<&str> = rest.splitn(2, ' ').collect();
                if ws.len() < 2 { println!("  {} Use: assume x Type", red("✗")); return true; }
                match parse_ty(ws[1]) {
                    Some(ty) => {
                        println!("  {} : {} ({} assumed)", cyan(ws[0]), ty.as_type(), ty.as_prop());
                        self.ctx.insert(ws[0].to_string(), ty);
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "ctx" => {
                if self.ctx.is_empty() { println!("  {}", dim("(empty context)")); }
                else {
                    println!("  {}", bold("Context:"));
                    for (name, ty) in &self.ctx {
                        println!("  {} : {}   ({})", cyan(name), ty.as_type(), dim(&ty.as_prop()));
                    }
                }
            }
            "table" => {
                println!("{}", bold("── Curry-Howard Correspondence Table ───────────────────────────────"));
                let rows = [
                    ("Logic",            "Type Theory",        "Programming"),
                    ("Proposition P",    "Type A",             "Specification"),
                    ("Proof of P",       "Term t : A",         "Program of type A"),
                    ("⊤ (True)",         "𝟙 (Unit)",           "()"),
                    ("⊥ (False)",        "𝟘 (Empty)",          "absurd"),
                    ("P ∧ Q",            "A × B (Product)",    "(a, b)"),
                    ("P ∨ Q",            "A + B (Sum)",        "inl a | inr b"),
                    ("P ⊃ Q (P → Q)",   "A → B (Function)",   "λx. f x"),
                    ("¬P",              "A → 𝟘",              "λx. absurd x"),
                    ("∀x:A. P(x)",       "Π(x:A). B(x)",       "λx. f x"),
                    ("∃x:A. P(x)",       "Σ(x:A). B(x)",       "(a, proof)"),
                    ("Implication elim", "Function application","f a"),
                    ("∧ introduction",   "Pair construction",  "(a, b)"),
                    ("∧ elimination",    "Projection",         "fst p, snd p"),
                ];
                let w = [22, 22, 20];
                println!("  {:w0$} {:w1$} {:w2$}", rows[0].0, rows[0].1, rows[0].2, w0=w[0], w1=w[1], w2=w[2]);
                println!("  {}", dim(&"─".repeat(w[0]+w[1]+w[2]+4)));
                for row in &rows[1..] {
                    println!("  {:w0$} {:w1$} {}", cyan(row.0), green(row.1), dim(row.2), w0=w[0], w1=w[1]);
                }
            }
            "tautologies" => {
                println!("{}", bold("── Classic Tautologies and their Proof Terms ───────────────────────"));
                let examples = [
                    ("A → A",          "λa. a",              "identity"),
                    ("A → B → A",      "λa. λb. a",          "K combinator (const)"),
                    ("(A→B→C) → (A→B) → A → C", "λf. λg. λa. f a (g a)", "S combinator"),
                    ("A ∧ B → A",      "λp. fst p",          "conjunction elimination"),
                    ("A ∧ B → B",      "λp. snd p",          "conjunction elimination"),
                    ("A → A ∨ B",      "λa. inl a",          "disjunction introduction"),
                    ("B → A ∨ B",      "λb. inr b",          "disjunction introduction"),
                    ("A ∧ B ↔ B ∧ A",  "(λp.(snd p,fst p), λp.(snd p,fst p))", "commutativity of ∧"),
                    ("¬¬A → A",        "NOT provable intuitionistically!", "double negation"),
                    ("A ∨ ¬A",         "NOT provable intuitionistically!", "LEM"),
                ];
                for (prop, term, name) in &examples {
                    println!("  {} {}", cyan(prop), dim(&format!("({})", name)));
                    println!("    λ-term: {}", green(term));
                    println!();
                }
            }
            _ => {
                // Try to parse as a type and show correspondence
                match parse_ty(trimmed) {
                    Some(ty) => self.show_type(&ty),
                    None => println!("  {} Unknown command. Type {} for help.", red("✗"), cyan("help")),
                }
            }
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Curry-Howard Correspondence Sandbox                   ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore the isomorphism between proofs and programs.");
    println!("  Enter a type or proposition to see both sides of the correspondence.");
    println!("  Type {} for the full table, {} for commands.\n", cyan("table"), cyan("help"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}CH{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
