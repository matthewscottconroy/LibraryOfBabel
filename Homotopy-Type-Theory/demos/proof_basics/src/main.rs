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

// ── Propositions ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum Prop {
    Var(String),
    True,
    False,
    Not(Box<Prop>),
    And(Box<Prop>, Box<Prop>),
    Or(Box<Prop>, Box<Prop>),
    Implies(Box<Prop>, Box<Prop>),
    Iff(Box<Prop>, Box<Prop>),
    Forall(String, Box<Prop>),  // quantified variable + body (displayed only)
    Exists(String, Box<Prop>),
}

impl Prop {
    fn display(&self) -> String {
        match self {
            Prop::Var(s) => s.clone(),
            Prop::True => "⊤".into(),
            Prop::False => "⊥".into(),
            Prop::Not(p) => format!("¬{}", p.display_atom()),
            Prop::And(a, b) => format!("{} ∧ {}", a.display_atom(), b.display_atom()),
            Prop::Or(a, b) => format!("{} ∨ {}", a.display_atom(), b.display_atom()),
            Prop::Implies(a, b) => format!("{} → {}", a.display_atom(), b.display()),
            Prop::Iff(a, b) => format!("{} ↔ {}", a.display_atom(), b.display_atom()),
            Prop::Forall(x, p) => format!("∀{x}. {}", p.display()),
            Prop::Exists(x, p) => format!("∃{x}. {}", p.display()),
        }
    }

    fn display_atom(&self) -> String {
        match self {
            Prop::Var(_) | Prop::True | Prop::False => self.display(),
            _ => format!("({})", self.display()),
        }
    }

    fn eval(&self, env: &HashMap<String, bool>) -> Option<bool> {
        match self {
            Prop::Var(s) => env.get(s).copied(),
            Prop::True => Some(true),
            Prop::False => Some(false),
            Prop::Not(p) => Some(!p.eval(env)?),
            Prop::And(a, b) => Some(a.eval(env)? && b.eval(env)?),
            Prop::Or(a, b) => Some(a.eval(env)? || b.eval(env)?),
            Prop::Implies(a, b) => Some(!a.eval(env)? || b.eval(env)?),
            Prop::Iff(a, b) => Some(a.eval(env)? == b.eval(env)?),
            Prop::Forall(_, _) | Prop::Exists(_, _) => None, // require domain
        }
    }

    fn free_vars(&self) -> Vec<String> {
        let mut vs = vec![];
        self.collect_vars(&mut vs);
        vs.sort();
        vs.dedup();
        vs
    }

    fn collect_vars(&self, out: &mut Vec<String>) {
        match self {
            Prop::Var(s) => out.push(s.clone()),
            Prop::True | Prop::False => {}
            Prop::Not(p) => p.collect_vars(out),
            Prop::And(a, b) | Prop::Or(a, b) | Prop::Implies(a, b) | Prop::Iff(a, b) => {
                a.collect_vars(out);
                b.collect_vars(out);
            }
            Prop::Forall(x, p) | Prop::Exists(x, p) => {
                p.collect_vars(out);
                out.retain(|v| v != x);
            }
        }
    }

    fn is_tautology(&self) -> bool {
        let vars = self.free_vars();
        let n = vars.len();
        if n > 8 { return false; }
        for mask in 0u32..(1 << n) {
            let mut env = HashMap::new();
            for (i, v) in vars.iter().enumerate() {
                env.insert(v.clone(), (mask >> i) & 1 == 1);
            }
            if self.eval(&env) != Some(true) { return false; }
        }
        true
    }

    fn is_contradiction(&self) -> bool {
        let vars = self.free_vars();
        let n = vars.len();
        if n > 8 { return false; }
        for mask in 0u32..(1 << n) {
            let mut env = HashMap::new();
            for (i, v) in vars.iter().enumerate() {
                env.insert(v.clone(), (mask >> i) & 1 == 1);
            }
            if self.eval(&env) != Some(false) { return false; }
        }
        true
    }
}

// ── Parser ────────────────────────────────────────────────────────────────────

struct Parser {
    tokens: Vec<String>,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        let mut tokens = vec![];
        let mut cur = String::new();
        for ch in input.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                cur.push(ch);
            } else {
                if !cur.is_empty() { tokens.push(cur.drain(..).collect()); }
                if !ch.is_whitespace() { tokens.push(ch.to_string()); }
            }
        }
        if !cur.is_empty() { tokens.push(cur); }
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&str> { self.tokens.get(self.pos).map(|s| s.as_str()) }
    fn next(&mut self) -> Option<&str> {
        let t = self.tokens.get(self.pos).map(|s| s.as_str());
        self.pos += 1;
        t
    }
    fn expect(&mut self, s: &str) -> bool {
        if self.peek() == Some(s) { self.pos += 1; true } else { false }
    }

    fn parse_iff(&mut self) -> Option<Prop> {
        let mut left = self.parse_implies()?;
        while self.peek() == Some("<") {
            self.next(); self.expect(">");
            self.expect("-"); // allow <->
            let right = self.parse_implies()?;
            left = Prop::Iff(Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_implies(&mut self) -> Option<Prop> {
        let left = self.parse_or()?;
        if self.peek() == Some("-") {
            self.next(); self.expect(">");
            let right = self.parse_implies()?;
            return Some(Prop::Implies(Box::new(left), Box::new(right)));
        }
        Some(left)
    }

    fn parse_or(&mut self) -> Option<Prop> {
        let mut left = self.parse_and()?;
        while self.peek() == Some("|") {
            self.next();
            let right = self.parse_and()?;
            left = Prop::Or(Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_and(&mut self) -> Option<Prop> {
        let mut left = self.parse_not()?;
        while self.peek() == Some("&") {
            self.next();
            let right = self.parse_not()?;
            left = Prop::And(Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_not(&mut self) -> Option<Prop> {
        if self.peek() == Some("!") || self.peek() == Some("~") {
            self.next();
            let p = self.parse_not()?;
            return Some(Prop::Not(Box::new(p)));
        }
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> Option<Prop> {
        match self.peek()? {
            "(" => {
                self.next();
                let p = self.parse_iff()?;
                self.expect(")");
                Some(p)
            }
            "T" | "true" | "True" => { self.next(); Some(Prop::True) }
            "F" | "false" | "False" => { self.next(); Some(Prop::False) }
            "forall" | "all" => {
                self.next();
                let x = self.next()?.to_string();
                self.expect(".");
                let body = self.parse_iff()?;
                Some(Prop::Forall(x, Box::new(body)))
            }
            "exists" | "ex" => {
                self.next();
                let x = self.next()?.to_string();
                self.expect(".");
                let body = self.parse_iff()?;
                Some(Prop::Exists(x, Box::new(body)))
            }
            t if t.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) => {
                let s = t.to_string(); self.next(); Some(Prop::Var(s))
            }
            _ => None,
        }
    }
}

fn parse_prop(s: &str) -> Option<Prop> {
    let mut p = Parser::new(s);
    let prop = p.parse_iff()?;
    if p.pos < p.tokens.len() { None } else { Some(prop) }
}

// ── Natural deduction proof state ────────────────────────────────────────────

#[derive(Clone)]
struct ProofState {
    goal: Prop,
    hypotheses: Vec<(String, Prop)>,
    done: bool,
}

impl ProofState {
    fn new(goal: Prop) -> Self {
        ProofState { goal, hypotheses: vec![], done: false }
    }

    fn display(&self) {
        if self.done {
            println!("  {} Proof complete!", green("✓"));
            return;
        }
        println!("  {}", dim("─────────────────────────────────────────"));
        for (name, hyp) in &self.hypotheses {
            println!("  {} : {}", cyan(name), hyp.display());
        }
        println!("  {}", dim("─────────────────────────────────────────"));
        println!("  ⊢ {}", bold(&self.goal.display()));
        println!("  {}", dim("─────────────────────────────────────────"));
    }

    fn apply_rule(&mut self, rule: &str, args: &[&str]) -> Result<String, String> {
        match rule {
            "intro" => self.rule_intro(args),
            "apply" => self.rule_apply(args),
            "exact" => self.rule_exact(args),
            "split" => self.rule_split(),
            "left" => self.rule_left(),
            "right" => self.rule_right(),
            "destruct" => self.rule_destruct(args),
            "exfalso" => self.rule_exfalso(),
            "trivial" => self.rule_trivial(),
            _ => Err(format!("Unknown rule: {rule}")),
        }
    }

    fn rule_intro(&mut self, args: &[&str]) -> Result<String, String> {
        match &self.goal.clone() {
            Prop::Implies(a, b) => {
                let name = args.first().unwrap_or(&"H").to_string();
                self.hypotheses.push((name.clone(), *a.clone()));
                self.goal = *b.clone();
                Ok(format!("Introduced {name} : {}", a.display()))
            }
            Prop::Forall(x, body) => {
                let fresh = args.first().unwrap_or(&x.as_str()).to_string();
                self.hypotheses.push((fresh.clone(), Prop::Var(format!("[{fresh} : term]"))));
                self.goal = *body.clone();
                Ok(format!("Introduced term {fresh}"))
            }
            _ => Err("intro: goal is not an implication or ∀".into()),
        }
    }

    fn rule_apply(&mut self, args: &[&str]) -> Result<String, String> {
        let name = args.first().ok_or("apply: need hypothesis name")?;
        let hyp = self.hypotheses.iter().find(|(n, _)| n == name)
            .ok_or_else(|| format!("No hypothesis {name}"))?
            .1.clone();
        match hyp {
            Prop::Implies(a, b) if *b == self.goal => {
                self.goal = *a;
                Ok(format!("Applied {name}: new goal is {}", self.goal.display()))
            }
            ref h if h == &self.goal => {
                self.done = true;
                Ok(format!("Applied {name}: goal matches hypothesis — QED!"))
            }
            _ => Err(format!("{name} does not apply to current goal")),
        }
    }

    fn rule_exact(&mut self, args: &[&str]) -> Result<String, String> {
        let name = args.first().ok_or("exact: need hypothesis name")?;
        let hyp = self.hypotheses.iter().find(|(n, _)| n == name)
            .ok_or_else(|| format!("No hypothesis {name}"))?
            .1.clone();
        if hyp == self.goal {
            self.done = true;
            Ok(format!("Exact match with {name} — QED!"))
        } else {
            Err(format!("{name} ({}) does not match goal ({})", hyp.display(), self.goal.display()))
        }
    }

    fn rule_split(&mut self) -> Result<String, String> {
        match &self.goal.clone() {
            Prop::And(a, b) => {
                println!("  {} Split into two subgoals:", green("→"));
                println!("    1. {}", a.display());
                println!("    2. {}", b.display());
                println!("  (Use 'subgoal 1' / 'subgoal 2' to work on each)");
                Ok("Split conjunction".into())
            }
            Prop::Iff(a, b) => {
                println!("  {} Split into two implications:", green("→"));
                println!("    1. {} → {}", a.display(), b.display());
                println!("    2. {} → {}", b.display(), a.display());
                Ok("Split iff".into())
            }
            _ => Err("split: goal is not ∧ or ↔".into()),
        }
    }

    fn rule_left(&mut self) -> Result<String, String> {
        match &self.goal.clone() {
            Prop::Or(a, _) => {
                self.goal = *a.clone();
                Ok(format!("Left branch: prove {}", self.goal.display()))
            }
            _ => Err("left: goal is not ∨".into()),
        }
    }

    fn rule_right(&mut self) -> Result<String, String> {
        match &self.goal.clone() {
            Prop::Or(_, b) => {
                self.goal = *b.clone();
                Ok(format!("Right branch: prove {}", self.goal.display()))
            }
            _ => Err("right: goal is not ∨".into()),
        }
    }

    fn rule_destruct(&mut self, args: &[&str]) -> Result<String, String> {
        let name = args.first().ok_or("destruct: need hypothesis name")?;
        let hyp = self.hypotheses.iter().find(|(n, _)| n == name)
            .ok_or_else(|| format!("No hypothesis {name}"))?
            .1.clone();
        match hyp {
            Prop::And(a, b) => {
                self.hypotheses.push((format!("{name}_l"), *a.clone()));
                self.hypotheses.push((format!("{name}_r"), *b.clone()));
                Ok(format!("Destructed {name} into {name}_l : {} and {name}_r : {}",
                    a.display(), b.display()))
            }
            Prop::Or(a, b) => {
                println!("  {} Case split on {name} ∨:", green("→"));
                println!("    Case 1: assume {name}_l : {}", a.display());
                println!("    Case 2: assume {name}_r : {}", b.display());
                Ok("Case split (work each branch separately)".into())
            }
            _ => Err(format!("{name} is not a conjunction or disjunction")),
        }
    }

    fn rule_exfalso(&mut self) -> Result<String, String> {
        let old = self.goal.display();
        self.goal = Prop::False;
        Ok(format!("Exfalso: was {old}, now prove ⊥"))
    }

    fn rule_trivial(&mut self) -> Result<String, String> {
        if self.goal == Prop::True {
            self.done = true;
            Ok("Trivial — ⊤ holds by definition — QED!".into())
        } else if self.hypotheses.iter().any(|(_, h)| h == &self.goal) {
            self.done = true;
            Ok("Trivial — goal is a hypothesis — QED!".into())
        } else {
            Err("trivial: cannot discharge this goal automatically".into())
        }
    }
}

// ── Main sandbox ──────────────────────────────────────────────────────────────

struct Sandbox {
    env: HashMap<String, bool>,
    named: HashMap<String, Prop>,
    proof: Option<ProofState>,
    history: Vec<String>,
}

impl Sandbox {
    fn new() -> Self {
        Sandbox {
            env: HashMap::new(),
            named: HashMap::new(),
            proof: None,
            history: vec![],
        }
    }

    fn print_help() {
        println!("{}", bold("── Propositional Logic ────────────────────────────────────────────"));
        println!("  {}  set P true|false      — assign truth value to atom", cyan("set"));
        println!("  {}  name = P & Q -> R    — name a proposition", cyan("let"));
        println!("  {}  P & Q -> R            — evaluate a formula", cyan("eval"));
        println!("  {}  name                  — check tautology/contradiction", cyan("check"));
        println!("  {}  name                  — print truth table", cyan("table"));
        println!("  {}  list named props", cyan("show"));
        println!("{}", bold("── Natural Deduction ───────────────────────────────────────────────"));
        println!("  {}  P -> Q               — start a proof", cyan("prove"));
        println!("  {}                         — show current goal", cyan("goal"));
        println!("  {}  [name]              — introduce hypothesis", cyan("intro"));
        println!("  {}  H                   — apply hypothesis to goal", cyan("apply"));
        println!("  {}  H                   — close with hypothesis", cyan("exact"));
        println!("  {}               — split ∧ or ↔ goal", cyan("split"));
        println!("  {}                  — prove left branch of ∨", cyan("left"));
        println!("  {}                 — prove right branch of ∨", cyan("right"));
        println!("  {}  H               — destruct ∧/∨ hypothesis", cyan("destruct"));
        println!("  {}         — change goal to ⊥ (ex falso)", cyan("exfalso"));
        println!("  {}          — close trivial goal", cyan("trivial"));
        println!("  {}                        — abandon proof", cyan("abort"));
        println!("{}", bold("── Syntax ──────────────────────────────────────────────────────────"));
        println!("  P & Q   P | Q   !P   P -> Q   P <> Q");
        println!("  T (true)   F (false)   forall x. P   exists x. P");
    }

    fn handle(&mut self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() { return true; }
        if trimmed == "quit" || trimmed == "exit" || trimmed == "q" { return false; }
        if trimmed == "help" || trimmed == "?" { Self::print_help(); return true; }

        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        let cmd = parts[0];
        let rest = if parts.len() > 1 { parts[1].trim() } else { "" };

        // Proof mode commands
        if let Some(ref mut proof) = self.proof {
            match cmd {
                "goal" => { proof.display(); return true; }
                "abort" => { self.proof = None; println!("  Proof aborted."); return true; }
                "intro" | "apply" | "exact" | "split" | "left" | "right" |
                "destruct" | "exfalso" | "trivial" => {
                    let args: Vec<&str> = rest.split_whitespace().collect();
                    match proof.apply_rule(cmd, &args) {
                        Ok(msg) => {
                            println!("  {} {}", green("✓"), msg);
                            if proof.done {
                                self.proof = None;
                            } else {
                                proof.display();
                            }
                        }
                        Err(e) => println!("  {} {}", red("✗"), e),
                    }
                    return true;
                }
                _ => {}
            }
        }

        match cmd {
            "set" => {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                if parts.len() >= 2 {
                    let atom = parts[0].to_string();
                    match parts[1] {
                        "true" | "1" | "T" => { self.env.insert(atom.clone(), true); println!("  {} = true", cyan(&atom)); }
                        "false" | "0" | "F" => { self.env.insert(atom.clone(), false); println!("  {} = false", cyan(&atom)); }
                        _ => println!("  {} Use: set P true|false", red("✗")),
                    }
                } else {
                    println!("  {} Use: set P true|false", red("✗"));
                }
            }
            "let" => {
                if let Some((name, expr)) = rest.split_once('=') {
                    let name = name.trim().to_string();
                    match parse_prop(expr.trim()) {
                        Some(p) => {
                            println!("  {} : {}", cyan(&name), p.display());
                            self.named.insert(name, p);
                        }
                        None => println!("  {} Parse error in: {expr}", red("✗")),
                    }
                } else {
                    println!("  {} Use: let name = proposition", red("✗"));
                }
            }
            "eval" => {
                let expr = if rest.is_empty() { cmd } else { rest };
                let prop = if let Some(named) = self.named.get(expr) {
                    named.clone()
                } else {
                    match parse_prop(expr) {
                        Some(p) => p,
                        None => { println!("  {} Parse error", red("✗")); return true; }
                    }
                };
                match prop.eval(&self.env) {
                    Some(true) => println!("  {} = {}", prop.display(), green("true")),
                    Some(false) => println!("  {} = {}", prop.display(), red("false")),
                    None => println!("  {} = {} (has unassigned atoms)", prop.display(), yellow("?")),
                }
            }
            "check" => {
                let expr = if rest.is_empty() { return true; } else { rest };
                let prop = if let Some(named) = self.named.get(expr) {
                    named.clone()
                } else {
                    match parse_prop(expr) {
                        Some(p) => p,
                        None => { println!("  {} Parse error", red("✗")); return true; }
                    }
                };
                print!("  {}: ", prop.display());
                if prop.is_tautology() { println!("{}", green("TAUTOLOGY")); }
                else if prop.is_contradiction() { println!("{}", red("CONTRADICTION")); }
                else { println!("{}", yellow("CONTINGENT")); }
            }
            "table" => {
                let expr = if rest.is_empty() { return true; } else { rest };
                let prop = if let Some(named) = self.named.get(expr) {
                    named.clone()
                } else {
                    match parse_prop(expr) {
                        Some(p) => p,
                        None => { println!("  {} Parse error", red("✗")); return true; }
                    }
                };
                let vars = prop.free_vars();
                if vars.len() > 6 { println!("  {} Too many variables (max 6)", red("✗")); return true; }
                let n = vars.len();
                // Header
                let mut header = String::from("  ");
                for v in &vars { header.push_str(&format!("  {v}")); }
                header.push_str(&format!("  │  {}", prop.display()));
                println!("{}", bold(&header));
                println!("  {}", dim(&"─".repeat(header.len() / 2 + 10)));
                for mask in 0u32..(1 << n) {
                    let mut env = HashMap::new();
                    let mut row = String::from("  ");
                    for (i, v) in vars.iter().enumerate() {
                        let val = (mask >> i) & 1 == 1;
                        env.insert(v.clone(), val);
                        row.push_str(&format!("  {}", if val { green("T") } else { red("F") }));
                    }
                    let result = prop.eval(&env);
                    row.push_str("  │  ");
                    row.push_str(&match result {
                        Some(true) => green("T"),
                        Some(false) => red("F"),
                        None => yellow("?"),
                    });
                    println!("{row}");
                }
            }
            "show" => {
                if self.named.is_empty() {
                    println!("  {}", dim("(no named propositions)"));
                } else {
                    for (name, prop) in &self.named {
                        println!("  {} : {}", cyan(name), prop.display());
                    }
                }
            }
            "prove" => {
                if rest.is_empty() { println!("  {} Use: prove P -> Q", red("✗")); return true; }
                match parse_prop(rest) {
                    Some(goal) => {
                        println!("  Starting proof of: {}", bold(&goal.display()));
                        let ps = ProofState::new(goal);
                        ps.display();
                        self.proof = Some(ps);
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "goal" => {
                if let Some(ref p) = self.proof {
                    p.display();
                } else {
                    println!("  {} Not in proof mode. Use: prove <goal>", red("✗"));
                }
            }
            "abort" => {
                self.proof = None;
                println!("  Proof aborted.");
            }
            "examples" => {
                println!("{}", bold("── Classic tautologies to explore ─────────────────────────────────"));
                println!("  check P -> P                     (identity)");
                println!("  check (P -> Q) -> (Q -> R) -> P -> R   (hypothetical syllogism)");
                println!("  check P | !P                     (excluded middle)");
                println!("  check !(P & !P)                  (non-contradiction)");
                println!("  check (P -> Q) <> (!Q -> !P)    (contrapositive)");
                println!("  check (P & Q) -> (Q & P)        (commutativity of ∧)");
                println!("  check P -> P | Q                 (addition)");
                println!("{}", bold("── Natural deduction examples ─────────────────────────────────────"));
                println!("  prove P -> P");
                println!("    intro H  →  exact H");
                println!("  prove (P -> Q) -> P -> Q");
                println!("    intro H  →  intro HP  →  apply H  →  exact HP");
                println!("  prove P & Q -> Q & P");
                println!("    intro H  →  destruct H  →  split  →  exact H_r / exact H_l");
            }
            _ => {
                // Try to eval it as a bare expression
                match parse_prop(trimmed) {
                    Some(prop) => {
                        match prop.eval(&self.env) {
                            Some(true) => println!("  {} = {}", prop.display(), green("true")),
                            Some(false) => println!("  {} = {}", prop.display(), red("false")),
                            None => {
                                // Auto-check
                                if prop.is_tautology() { println!("  {}: {}", prop.display(), green("TAUTOLOGY")); }
                                else if prop.is_contradiction() { println!("  {}: {}", prop.display(), red("CONTRADICTION")); }
                                else { println!("  {}: {} (set atoms to evaluate)", prop.display(), yellow("CONTINGENT")); }
                            }
                        }
                    }
                    None => println!("  {} Unknown command. Type {} for help.", red("✗"), cyan("help")),
                }
            }
        }
        self.history.push(trimmed.to_string());
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║        Logic Explorer — Proof Basics Sandbox             ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));

    println!("  Build propositions, evaluate them, check tautologies,");
    println!("  and practice natural deduction proofs step by step.");
    println!("  Type {} for commands, {} for guided examples.\n", cyan("help"), cyan("examples"));

    let stdin = io::stdin();
    let mut sandbox = Sandbox::new();

    loop {
        let prompt = if sandbox.proof.is_some() {
            format!("  {}proof{} > ", CYAN, RESET)
        } else {
            format!("  {}logic{} > ", GREEN, RESET)
        };
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                if !sandbox.handle(&line) { break; }
            }
        }
    }
    println!("  Goodbye.");
}
