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

// ── Function Extensionality ───────────────────────────────────────────────────
//
// funext : (∀ x, f x = g x) → f = g
// happly : f = g → ∀ x, f x = g x
//
// These are mutually inverse. In HoTT, funext follows from univalence.
// In Cubical HoTT, it is directly provable using path abstraction.
//
// We model functions as Rust functions on a finite domain, and
// demonstrate funext by checking pointwise equality.

// ── Finite functions ──────────────────────────────────────────────────────────

#[derive(Clone)]
struct FinFun {
    name: String,
    domain: Vec<String>,
    table: HashMap<String, String>,
}

impl FinFun {
    fn new(name: &str, domain: Vec<String>) -> Self {
        FinFun { name: name.to_string(), domain, table: HashMap::new() }
    }

    fn set(&mut self, x: &str, y: &str) -> Result<(), String> {
        if !self.domain.contains(&x.to_string()) { return Err(format!("{x} not in domain")); }
        self.table.insert(x.to_string(), y.to_string());
        Ok(())
    }

    fn apply(&self, x: &str) -> Option<&str> { self.table.get(x).map(|s| s.as_str()) }

    fn is_total(&self) -> bool { self.domain.iter().all(|x| self.table.contains_key(x)) }

    fn pointwise_eq(&self, other: &FinFun) -> Option<Vec<(String, bool)>> {
        if self.domain != other.domain { return None; }
        Some(self.domain.iter().map(|x| {
            let a = self.apply(x);
            let b = other.apply(x);
            (x.clone(), a == b && a.is_some())
        }).collect())
    }

    fn display(&self) {
        println!("  {} : {} → ?", cyan(&self.name), "{".to_string() + &self.domain.join(", ") + "}");
        for x in &self.domain {
            match self.table.get(x) {
                Some(y) => println!("    {} ↦ {}", x, cyan(y)),
                None => println!("    {} ↦ {}", x, dim("?")),
            }
        }
    }
}

// ── Homotopy (pointwise path) ─────────────────────────────────────────────────

struct Homotopy {
    from: String,
    to: String,
    name: String,
}

impl Homotopy {
    fn display(&self) {
        println!("  {} : ∀ x, {}(x) = {}(x)", cyan(&self.name), self.from, self.to);
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    funs: HashMap<String, FinFun>,
    domain: Vec<String>,
    homotopies: Vec<Homotopy>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { funs: HashMap::new(), domain: vec![], homotopies: vec![] };
        // Pre-populate with a domain and some functions
        sb.domain = vec!["a".into(), "b".into(), "c".into()];
        let mut id = FinFun::new("id", sb.domain.clone());
        for x in &sb.domain { let _ = id.set(x, x); }
        sb.funs.insert("id".into(), id);
        sb
    }

    fn print_help() {
        println!("{}", bold("── Domain Setup ────────────────────────────────────────────────────"));
        println!("  {}  x y z            — set the domain", cyan("domain"));
        println!("  {}  f               — create a function on the domain", cyan("fun"));
        println!("  {}  f x y           — set f(x) = y", cyan("map"));
        println!("  {}  f               — display function table", cyan("show"));
        println!("{}", bold("── Function Extensionality ─────────────────────────────────────────"));
        println!("  {}  f g             — happly: check pointwise equality", cyan("happly"));
        println!("  {}  f g             — funext: if pointwise equal, they're equal", cyan("funext"));
        println!("  {}  f x             — apply f to x", cyan("apply"));
        println!("  {}  f g             — compose g ∘ f", cyan("compose"));
        println!("  {}  f g             — check if f = g definitionally", cyan("defeq"));
        println!("{}", bold("── η-laws ──────────────────────────────────────────────────────────"));
        println!("  {}  f               — check η: f = λx. f(x)", cyan("eta"));
        println!("{}", bold("── Theory ───────────────────────────────────────────────────────────"));
        println!("  {}         — explain funext and happly", cyan("explain"));
        println!("  {}         — show funext from univalence", cyan("from-ua"));
        println!("  {}         — show function type h-level rules", cyan("levels"));
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
            "domain" => {
                self.domain = rest.split_whitespace().map(|s| s.to_string()).collect();
                println!("  Domain = {{{}}}", self.domain.join(", "));
            }
            "fun" => {
                if rest.is_empty() { println!("  {} Use: fun name", red("✗")); return true; }
                let f = FinFun::new(rest, self.domain.clone());
                println!("  {} : {{{}}} → ?  (set values with 'map {} x y')", cyan(rest), self.domain.join(", "), rest);
                self.funs.insert(rest.to_string(), f);
            }
            "map" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 3 { println!("  {} Use: map f x y", red("✗")); return true; }
                if let Some(f) = self.funs.get_mut(ws[0]) {
                    match f.set(ws[1], ws[2]) {
                        Ok(()) => println!("  {}({}) = {}", ws[0], ws[1], cyan(ws[2])),
                        Err(e) => println!("  {} {}", red("✗"), e),
                    }
                } else { println!("  {} Function {} not found", red("✗"), ws[0]); }
            }
            "show" => {
                if rest.is_empty() {
                    for (_, f) in &self.funs { f.display(); println!(); }
                } else if let Some(f) = self.funs.get(rest) {
                    f.display();
                    if f.is_total() { println!("  {} Total function", green("✓")); }
                    else { println!("  {} Partial (some values unset)", yellow("!")); }
                } else { println!("  {} Function {rest} not found", red("✗")); }
            }
            "happly" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: happly f g", red("✗")); return true; }
                let f = self.funs.get(ws[0]).cloned();
                let g = self.funs.get(ws[1]).cloned();
                match (f, g) {
                    (Some(f), Some(g)) => {
                        println!("  happly(f=g) : ∀ x, {}(x) = {}(x)?", ws[0], ws[1]);
                        match f.pointwise_eq(&g) {
                            Some(eqs) => {
                                let all_eq = eqs.iter().all(|(_, eq)| *eq);
                                for (x, eq) in &eqs {
                                    let fx = f.apply(x).unwrap_or("?");
                                    let gx = g.apply(x).unwrap_or("?");
                                    if *eq { println!("  {}({x}) = {}({x}) = {}  {}", ws[0], ws[1], cyan(fx), green("✓")); }
                                    else { println!("  {}({x}) = {} ≠ {} = {}({x})  {}", ws[0], fx, gx, ws[1], red("✗")); }
                                }
                                if all_eq { println!("  {} Pointwise equal — funext gives {} = {} (as functions)", green("✓"), ws[0], ws[1]); }
                                else { println!("  {} Not pointwise equal — {} ≠ {} as functions", red("✗"), ws[0], ws[1]); }
                            }
                            None => println!("  {} Functions have different domains", red("✗")),
                        }
                    }
                    _ => println!("  {} Function(s) not found", red("✗")),
                }
            }
            "funext" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: funext f g", red("✗")); return true; }
                let f = self.funs.get(ws[0]).cloned();
                let g = self.funs.get(ws[1]).cloned();
                match (f, g) {
                    (Some(f), Some(g)) => {
                        match f.pointwise_eq(&g) {
                            Some(eqs) if eqs.iter().all(|(_, eq)| *eq) => {
                                println!("  {} funext({}, {}) : {} = {}  as functions", green("✓"), ws[0], ws[1], ws[0], ws[1]);
                                println!("  A path between {} and {} in (A→B).", ws[0], ws[1]);
                                println!("  This path is: λ a. <pointwise path at a>");
                                self.homotopies.push(Homotopy { from: ws[0].to_string(), to: ws[1].to_string(), name: format!("funext({},{})", ws[0], ws[1]) });
                            }
                            Some(_) => println!("  {} Cannot apply funext: not pointwise equal", red("✗")),
                            None => println!("  {} Different domains", red("✗")),
                        }
                    }
                    _ => println!("  {} Function(s) not found", red("✗")),
                }
            }
            "apply" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: apply f x", red("✗")); return true; }
                if let Some(f) = self.funs.get(ws[0]) {
                    match f.apply(ws[1]) {
                        Some(y) => println!("  {}({}) = {}", ws[0], ws[1], cyan(y)),
                        None => println!("  {} {}({}) undefined or {} not in domain", red("✗"), ws[0], ws[1], ws[1]),
                    }
                } else { println!("  {} Function {} not found", red("✗"), ws[0]); }
            }
            "compose" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: compose f g  (computes g∘f)", red("✗")); return true; }
                let f = self.funs.get(ws[0]).cloned();
                let g = self.funs.get(ws[1]).cloned();
                match (f, g) {
                    (Some(f), Some(g)) => {
                        let name = format!("{}∘{}", ws[1], ws[0]);
                        let mut comp = FinFun::new(&name, f.domain.clone());
                        for x in &f.domain {
                            if let Some(fx) = f.apply(x) {
                                if let Some(gfx) = g.apply(fx) {
                                    let _ = comp.set(x, gfx);
                                }
                            }
                        }
                        comp.display();
                        self.funs.insert(name, comp);
                    }
                    _ => println!("  {} Function(s) not found", red("✗")),
                }
            }
            "defeq" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: defeq f g", red("✗")); return true; }
                if ws[0] == ws[1] { println!("  {} {} and {} are definitionally equal (same name)", green("✓"), ws[0], ws[1]); }
                else { println!("  {} {} and {} may be propositionally equal (check with happly/funext)", yellow("?"), ws[0], ws[1]); }
            }
            "eta" => {
                if let Some(f) = self.funs.get(rest) {
                    let f = f.clone();
                    println!("  {} η-law: {} = λx. {}(x)?", bold("Checking"), rest, rest);
                    println!("  The η-law says every function equals its η-expansion.");
                    if f.is_total() { println!("  {} {} satisfies η-reduction: f = λx. f x  (by definition)", green("✓"), rest); }
                    else { println!("  {} {} is partial — η holds for defined values", yellow("?"), rest); }
                    println!("  In HoTT: η : f = λx. f x holds definitionally (or by funext)");
                } else { println!("  {} Function {rest} not found", red("✗")); }
            }
            "explain" => {
                println!("{}", bold("── Function Extensionality in HoTT ─────────────────────────────────"));
                println!("  happly : (f = g) → ∀ x, f x = g x");
                println!("  funext : (∀ x, f x = g x) → f = g");
                println!();
                println!("  In Book HoTT: funext is an axiom (Axiom 2.9.3)");
                println!("  In Cubical HoTT: funext is provable!");
                println!("  Proof: given h : ∀ x, f x = g x, define");
                println!("         p : f = g  as  λ i. λ x. h x i   (path abstraction)");
                println!();
                println!("  happly and funext are quasi-inverse:");
                println!("  happly(funext(h)) x = h x   (η-law for happly)");
                println!("  funext(happly(p)) = p       (η-law for funext)");
                println!();
                println!("  Consequence: the function type A → B is (essentially) a Π-type");
                println!("  and paths in A → B are exactly pointwise paths.");
            }
            "from-ua" => {
                println!("{}", bold("── funext from Univalence ───────────────────────────────────────────"));
                println!("  Theorem (Voevodsky): funext follows from univalence.");
                println!();
                println!("  Proof sketch:");
                println!("  1. Consider the type Q(f) = Σ(g : A→B). f = g  (singleton)");
                println!("  2. Q(f) is contractible (center = (f, refl))");
                println!("  3. By univalence, the 'codomain' transport carries this");
                println!("  4. happly is a quasi-inverse of funext");
                println!("  5. Both are equivalences");
                println!();
                println!("  In Cubical HoTT: funext is more direct via path abstraction,");
                println!("  so we don't need univalence as a stepping stone.");
            }
            "levels" => {
                println!("{}", bold("── H-levels of Function Types ───────────────────────────────────────"));
                println!("  If B is a proposition (h-level -1):");
                println!("    A → B is also a proposition");
                println!("    (two functions into a prop are always equal by funext + prophood of B)");
                println!();
                println!("  If B is a set (h-level 0):");
                println!("    A → B is a set");
                println!("    (paths in A→B = ∀ x, paths in B = propositions)");
                println!();
                println!("  If B is a groupoid (h-level 1):");
                println!("    A → B is a groupoid");
                println!();
                println!("  General rule: h-level(A→B) = h-level(B)");
                println!("  The codomain dominates (the domain can be arbitrary).");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Function Extensionality — happly and funext Sandbox   ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Define finite functions and explore pointwise equality.");
    println!("  See how funext converts pointwise paths to function paths.");
    println!("  Type {} for commands, {} for the theory.\n", cyan("help"), cyan("explain"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}funext{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
