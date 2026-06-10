use std::collections::{HashMap, HashSet};
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

// ── Kripke frame ──────────────────────────────────────────────────────────────

struct KripkeFrame {
    worlds: Vec<String>,
    // accessibility: w -> set of accessible worlds
    access: HashMap<String, HashSet<String>>,
    // valuation: atom -> set of worlds where it's true
    val: HashMap<String, HashSet<String>>,
}

impl KripkeFrame {
    fn new() -> Self {
        KripkeFrame {
            worlds: vec![],
            access: HashMap::new(),
            val: HashMap::new(),
        }
    }

    fn add_world(&mut self, w: &str) {
        if !self.worlds.contains(&w.to_string()) {
            self.worlds.push(w.to_string());
            self.access.insert(w.to_string(), HashSet::new());
        }
    }

    fn add_access(&mut self, from: &str, to: &str) -> Result<(), String> {
        if !self.worlds.contains(&from.to_string()) {
            return Err(format!("World {from} not defined"));
        }
        if !self.worlds.contains(&to.to_string()) {
            return Err(format!("World {to} not defined"));
        }
        self.access.get_mut(from).unwrap().insert(to.to_string());
        Ok(())
    }

    fn set_true(&mut self, atom: &str, world: &str) -> Result<(), String> {
        if !self.worlds.contains(&world.to_string()) {
            return Err(format!("World {world} not defined"));
        }
        self.val.entry(atom.to_string()).or_default().insert(world.to_string());
        Ok(())
    }

    fn set_false(&mut self, atom: &str, world: &str) -> Result<(), String> {
        if !self.worlds.contains(&world.to_string()) {
            return Err(format!("World {world} not defined"));
        }
        if let Some(s) = self.val.get_mut(atom) { s.remove(world); }
        Ok(())
    }

    fn is_true_at(&self, world: &str, formula: &Formula) -> bool {
        match formula {
            Formula::Atom(a) => self.val.get(a).map(|s| s.contains(world)).unwrap_or(false),
            Formula::Top => true,
            Formula::Bot => false,
            Formula::Not(p) => !self.is_true_at(world, p),
            Formula::And(a, b) => self.is_true_at(world, a) && self.is_true_at(world, b),
            Formula::Or(a, b) => self.is_true_at(world, a) || self.is_true_at(world, b),
            Formula::Implies(a, b) => {
                // Intuitionistic: w ⊩ A→B iff ∀v≥w, w⊩A ⟹ w⊩B
                // Here we use classical forcing (Kripke for classical modal logic)
                !self.is_true_at(world, a) || self.is_true_at(world, b)
            }
            Formula::Nec(p) => {
                // □P is true at w iff P is true at all accessible worlds
                self.access.get(world).map(|succs| {
                    succs.iter().all(|v| self.is_true_at(v, p))
                }).unwrap_or(true) // vacuously true if no accessible worlds
            }
            Formula::Pos(p) => {
                // ◇P is true at w iff P is true at some accessible world
                self.access.get(world).map(|succs| {
                    succs.iter().any(|v| self.is_true_at(v, p))
                }).unwrap_or(false)
            }
        }
    }

    fn truth_set(&self, formula: &Formula) -> Vec<String> {
        self.worlds.iter()
            .filter(|w| self.is_true_at(w, formula))
            .cloned()
            .collect()
    }

    fn is_valid(&self, formula: &Formula) -> bool {
        self.worlds.iter().all(|w| self.is_true_at(w, formula))
    }

    fn check_reflexive(&self) -> bool {
        self.worlds.iter().all(|w| self.access.get(w).map(|s| s.contains(w)).unwrap_or(false))
    }

    fn check_transitive(&self) -> bool {
        for w in &self.worlds {
            if let Some(succs) = self.access.get(w) {
                for v in succs {
                    if let Some(succs2) = self.access.get(v) {
                        for u in succs2 {
                            if !succs.contains(u) { return false; }
                        }
                    }
                }
            }
        }
        true
    }

    fn check_symmetric(&self) -> bool {
        for w in &self.worlds {
            if let Some(succs) = self.access.get(w) {
                for v in succs {
                    if !self.access.get(v).map(|s| s.contains(w)).unwrap_or(false) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn show(&self) {
        if self.worlds.is_empty() {
            println!("  {}", dim("(no worlds)"));
            return;
        }
        println!("  {} {}", bold("Worlds:"), self.worlds.join(", "));
        println!("  {}", bold("Accessibility:"));
        for w in &self.worlds {
            let succs = self.access.get(w).map(|s| {
                let mut v: Vec<_> = s.iter().collect();
                v.sort();
                v.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            }).unwrap_or_default();
            println!("    {} → {{{}}}", cyan(w), succs);
        }
        println!("  {}", bold("Valuation:"));
        let mut atoms: Vec<_> = self.val.keys().collect();
        atoms.sort();
        if atoms.is_empty() {
            println!("    {}", dim("(no atoms assigned)"));
        }
        for atom in atoms {
            let ws = self.val.get(atom).unwrap();
            let mut wlist: Vec<_> = ws.iter().collect();
            wlist.sort();
            println!("    {} true at: {{{}}}", cyan(atom), wlist.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
        }
        let reflex = if self.check_reflexive() { green("yes") } else { red("no") };
        let trans  = if self.check_transitive() { green("yes") } else { red("no") };
        let sym    = if self.check_symmetric()  { green("yes") } else { red("no") };
        println!("  {} reflexive: {}  transitive: {}  symmetric: {}", bold("Frame:"), reflex, trans, sym);
    }
}

// ── Formula ───────────────────────────────────────────────────────────────────

#[derive(Clone)]
enum Formula {
    Atom(String),
    Top,
    Bot,
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Nec(Box<Formula>),  // □
    Pos(Box<Formula>),  // ◇
}

impl Formula {
    fn display(&self) -> String {
        match self {
            Formula::Atom(a) => a.clone(),
            Formula::Top => "⊤".into(),
            Formula::Bot => "⊥".into(),
            Formula::Not(p) => format!("¬{}", p.display_atom()),
            Formula::And(a, b) => format!("{} ∧ {}", a.display_atom(), b.display_atom()),
            Formula::Or(a, b) => format!("{} ∨ {}", a.display_atom(), b.display_atom()),
            Formula::Implies(a, b) => format!("{} → {}", a.display_atom(), b.display()),
            Formula::Nec(p) => format!("□{}", p.display_atom()),
            Formula::Pos(p) => format!("◇{}", p.display_atom()),
        }
    }
    fn display_atom(&self) -> String {
        match self { Formula::Atom(_) | Formula::Top | Formula::Bot
            | Formula::Nec(_) | Formula::Pos(_) => self.display(),
            _ => format!("({})", self.display()) }
    }
}

fn parse_formula(s: &str) -> Option<Formula> {
    let tokens = tokenize(s);
    let mut pos = 0;
    let f = parse_implies_f(&tokens, &mut pos)?;
    if pos == tokens.len() { Some(f) } else { None }
}

fn tokenize(s: &str) -> Vec<String> {
    let mut tokens = vec![];
    let mut cur = String::new();
    for ch in s.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            cur.push(ch);
        } else {
            if !cur.is_empty() { tokens.push(cur.drain(..).collect()); }
            if !ch.is_whitespace() { tokens.push(ch.to_string()); }
        }
    }
    if !cur.is_empty() { tokens.push(cur); }
    tokens
}

fn parse_implies_f(tokens: &[String], pos: &mut usize) -> Option<Formula> {
    let left = parse_or_f(tokens, pos)?;
    if tokens.get(*pos).map(|s| s == "-").unwrap_or(false) {
        *pos += 1;
        if tokens.get(*pos).map(|s| s == ">").unwrap_or(false) { *pos += 1; }
        let right = parse_implies_f(tokens, pos)?;
        return Some(Formula::Implies(Box::new(left), Box::new(right)));
    }
    Some(left)
}

fn parse_or_f(tokens: &[String], pos: &mut usize) -> Option<Formula> {
    let mut left = parse_and_f(tokens, pos)?;
    while tokens.get(*pos).map(|s| s == "|").unwrap_or(false) {
        *pos += 1;
        let right = parse_and_f(tokens, pos)?;
        left = Formula::Or(Box::new(left), Box::new(right));
    }
    Some(left)
}

fn parse_and_f(tokens: &[String], pos: &mut usize) -> Option<Formula> {
    let mut left = parse_unary_f(tokens, pos)?;
    while tokens.get(*pos).map(|s| s == "&").unwrap_or(false) {
        *pos += 1;
        let right = parse_unary_f(tokens, pos)?;
        left = Formula::And(Box::new(left), Box::new(right));
    }
    Some(left)
}

fn parse_unary_f(tokens: &[String], pos: &mut usize) -> Option<Formula> {
    match tokens.get(*pos).map(|s| s.as_str()) {
        Some("!") | Some("~") => { *pos += 1; Some(Formula::Not(Box::new(parse_unary_f(tokens, pos)?))) }
        Some("[]") | Some("box") | Some("nec") => { *pos += 1; Some(Formula::Nec(Box::new(parse_unary_f(tokens, pos)?))) }
        Some("<>") | Some("dia") | Some("pos") => { *pos += 1; Some(Formula::Pos(Box::new(parse_unary_f(tokens, pos)?))) }
        _ => parse_atom_f(tokens, pos),
    }
}

fn parse_atom_f(tokens: &[String], pos: &mut usize) -> Option<Formula> {
    match tokens.get(*pos).map(|s| s.as_str()) {
        Some("(") => {
            *pos += 1;
            let f = parse_implies_f(tokens, pos)?;
            if tokens.get(*pos).map(|s| s == ")").unwrap_or(false) { *pos += 1; }
            Some(f)
        }
        Some("T") | Some("true") => { *pos += 1; Some(Formula::Top) }
        Some("F") | Some("false") => { *pos += 1; Some(Formula::Bot) }
        Some(t) if t.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false) => {
            let s = t.to_string(); *pos += 1; Some(Formula::Atom(s))
        }
        _ => None,
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    frame: KripkeFrame,
    named: HashMap<String, Formula>,
}

impl Sandbox {
    fn new() -> Self { Sandbox { frame: KripkeFrame::new(), named: HashMap::new() } }

    fn print_help() {
        println!("{}", bold("── Frame Construction ─────────────────────────────────────────────"));
        println!("  {}  w1 w2 w3          — add worlds", cyan("world"));
        println!("  {}  w1 w2             — add accessibility edge w1→w2", cyan("access"));
        println!("  {}  P w1 w2           — P is true at these worlds", cyan("true"));
        println!("  {}  P w1              — P is false at this world", cyan("false"));
        println!("  {}                    — show the frame", cyan("show"));
        println!("  {}               — clear and start over", cyan("reset"));
        println!("{}", bold("── Evaluation ─────────────────────────────────────────────────────"));
        println!("  {}  w □P              — check if formula holds at world w", cyan("force"));
        println!("  {}  □P               — find worlds where formula holds", cyan("where"));
        println!("  {}  □P               — check if valid in entire frame", cyan("valid"));
        println!("  {}  name = □P -> ◇P — name a formula", cyan("let"));
        println!("{}", bold("── Frame Properties ───────────────────────────────────────────────"));
        println!("  {}            — check reflexive/transitive/symmetric", cyan("props"));
        println!("  {}            — add reflexive closure", cyan("reflexive"));
        println!("  {}           — add transitive closure", cyan("transitive"));
        println!("{}", bold("── Syntax ─────────────────────────────────────────────────────────"));
        println!("  P & Q  P | Q  !P  P -> Q");
        println!("  [] P  (box, necessity □)    <> P  (dia, possibility ◇)");
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
            "world" | "worlds" => {
                for w in rest.split_whitespace() {
                    self.frame.add_world(w);
                    println!("  Added world {}", cyan(w));
                }
            }
            "access" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 {
                    println!("  {} Use: access w1 w2", red("✗"));
                } else {
                    for i in 0..ws.len()-1 {
                        match self.frame.add_access(ws[i], ws[i+1]) {
                            Ok(()) => println!("  {} → {}", cyan(ws[i]), cyan(ws[i+1])),
                            Err(e) => println!("  {} {}", red("✗"), e),
                        }
                    }
                }
            }
            "true" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 {
                    println!("  {} Use: true P w1 w2 ...", red("✗"));
                } else {
                    let atom = ws[0];
                    for w in &ws[1..] {
                        match self.frame.set_true(atom, w) {
                            Ok(()) => println!("  {} true at {}", cyan(atom), cyan(w)),
                            Err(e) => println!("  {} {}", red("✗"), e),
                        }
                    }
                }
            }
            "false" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 {
                    println!("  {} Use: false P w1 ...", red("✗"));
                } else {
                    let atom = ws[0];
                    for w in &ws[1..] {
                        match self.frame.set_false(atom, w) {
                            Ok(()) => println!("  {} false at {}", cyan(atom), cyan(w)),
                            Err(e) => println!("  {} {}", red("✗"), e),
                        }
                    }
                }
            }
            "show" => self.frame.show(),
            "reset" => {
                self.frame = KripkeFrame::new();
                println!("  Frame cleared.");
            }
            "props" => {
                let r = if self.frame.check_reflexive() { green("yes (T axiom □P→P valid)") } else { red("no") };
                let t = if self.frame.check_transitive() { green("yes (4 axiom □P→□□P valid)") } else { red("no") };
                let s = if self.frame.check_symmetric() { green("yes (B axiom P→□◇P valid)") } else { red("no") };
                println!("  Reflexive:   {r}");
                println!("  Transitive:  {t}");
                println!("  Symmetric:   {s}");
            }
            "reflexive" => {
                let ws = self.frame.worlds.clone();
                for w in &ws { self.frame.access.get_mut(w).unwrap().insert(w.clone()); }
                println!("  Added reflexive closure (w→w for all worlds)");
            }
            "transitive" => {
                // Warshall's algorithm
                let ws = self.frame.worlds.clone();
                for k in &ws {
                    let wk = ws.clone();
                    for i in &wk {
                        if self.frame.access.get(i).map(|s| s.contains(k)).unwrap_or(false) {
                            let reachable_from_k: Vec<_> = self.frame.access.get(k)
                                .map(|s| s.iter().cloned().collect()).unwrap_or_default();
                            for j in reachable_from_k {
                                self.frame.access.get_mut(i).unwrap().insert(j);
                            }
                        }
                    }
                }
                println!("  Added transitive closure");
            }
            "let" => {
                if let Some((name, expr)) = rest.split_once('=') {
                    match parse_formula(expr.trim()) {
                        Some(f) => {
                            println!("  {} : {}", cyan(name.trim()), f.display());
                            self.named.insert(name.trim().to_string(), f);
                        }
                        None => println!("  {} Parse error", red("✗")),
                    }
                }
            }
            "force" => {
                let parts: Vec<&str> = rest.splitn(2, ' ').collect();
                if parts.len() < 2 {
                    println!("  {} Use: force <world> <formula>", red("✗"));
                    return true;
                }
                let w = parts[0];
                let f = if let Some(named) = self.named.get(parts[1]) { named.clone() }
                    else { match parse_formula(parts[1]) { Some(f) => f, None => { println!("  {} Parse error", red("✗")); return true; } } };
                if !self.frame.worlds.contains(&w.to_string()) {
                    println!("  {} World {w} not defined", red("✗")); return true;
                }
                let holds = self.frame.is_true_at(w, &f);
                if holds { println!("  {} ⊩ {} : {}", cyan(w), f.display(), green("true")); }
                else { println!("  {} ⊮ {} : {}", cyan(w), f.display(), red("false")); }
            }
            "where" => {
                let f = if let Some(named) = self.named.get(rest) { named.clone() }
                    else { match parse_formula(rest) { Some(f) => f, None => { println!("  {} Parse error", red("✗")); return true; } } };
                let worlds = self.frame.truth_set(&f);
                if worlds.is_empty() { println!("  {} holds nowhere", f.display()); }
                else { println!("  {} holds at: {{{}}}", f.display(), worlds.join(", ")); }
            }
            "valid" => {
                let f = if let Some(named) = self.named.get(rest) { named.clone() }
                    else { match parse_formula(rest) { Some(f) => f, None => { println!("  {} Parse error", red("✗")); return true; } } };
                if self.frame.is_valid(&f) { println!("  {} : {} (valid in this frame)", f.display(), green("✓")); }
                else { println!("  {} : {} (not valid — fails at some world)", f.display(), red("✗")); }
            }
            "example" | "examples" => {
                println!("{}", bold("── Example frames ─────────────────────────────────────────────────"));
                println!("  Linear: world w1 w2 w3 → access w1 w2 → access w2 w3 → reflexive");
                println!("  Diamond: world a b c d → access a b → access a c → access b d → access c d");
                println!("{}", bold("── Key modal validities to test ───────────────────────────────────"));
                println!("  valid [] P -> P          (T axiom — needs reflexive)");
                println!("  valid [] P -> [][] P     (4 axiom — needs transitive)");
                println!("  valid P -> [] <> P       (B axiom — needs symmetric)");
                println!("  valid [] (P -> Q) -> [] P -> [] Q   (K axiom — always valid)");
                println!("  valid [] P | [] !P       (not always valid — try refuting it!)");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║        Kripke Semantics — Modal Logic Sandbox            ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build Kripke frames, assign valuations, force modal formulas.");
    println!("  Explore which modal axioms hold based on frame properties.");
    println!("  Type {} for commands, {} for examples.\n", cyan("help"), cyan("examples"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}kripke{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
