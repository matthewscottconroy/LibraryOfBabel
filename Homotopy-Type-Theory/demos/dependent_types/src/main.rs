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

// ── Types in our mini dependent type theory ───────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum Ty {
    Nat,
    Bool,
    Unit,
    Void,
    Prod(Box<Ty>, Box<Ty>),      // A × B (non-dependent pair)
    Sum(Box<Ty>, Box<Ty>),       // A + B (coproduct)
    Fun(Box<Ty>, Box<Ty>),       // A → B (non-dependent function)
    Vec(usize),                   // Vec n  (length-indexed vector)
    Fin(usize),                   // Fin n  (bounded natural)
    Sigma(String, Box<Ty>, Box<Ty>), // Σ(x:A).B(x) — dependent pair
    Pi(String, Box<Ty>, Box<Ty>),    // Π(x:A).B(x) — dependent function
    Universe(usize),              // 𝒰₀, 𝒰₁, ...
    Named(String),
}

impl Ty {
    fn display(&self) -> String {
        match self {
            Ty::Nat => "ℕ".into(),
            Ty::Bool => "𝟚".into(),
            Ty::Unit => "𝟙".into(),
            Ty::Void => "𝟘".into(),
            Ty::Prod(a, b) => format!("{} × {}", a.display_atom(), b.display_atom()),
            Ty::Sum(a, b) => format!("{} + {}", a.display_atom(), b.display_atom()),
            Ty::Fun(a, b) => format!("{} → {}", a.display_atom(), b.display()),
            Ty::Vec(n) => format!("Vec {n}"),
            Ty::Fin(n) => format!("Fin {n}"),
            Ty::Sigma(x, a, b) => format!("Σ({x}:{}).{}", a.display(), b.display()),
            Ty::Pi(x, a, b) => format!("Π({x}:{}).{}", a.display(), b.display()),
            Ty::Universe(n) => format!("𝒰{n}"),
            Ty::Named(s) => s.clone(),
        }
    }

    fn display_atom(&self) -> String {
        match self {
            Ty::Nat | Ty::Bool | Ty::Unit | Ty::Void | Ty::Universe(_)
            | Ty::Vec(_) | Ty::Fin(_) | Ty::Named(_) => self.display(),
            _ => format!("({})", self.display()),
        }
    }

    fn is_inhabited(&self, ctx: &HashMap<String, Ty>) -> bool {
        match self {
            Ty::Nat | Ty::Bool | Ty::Unit => true,
            Ty::Void => false,
            Ty::Vec(0) => true,  // empty vector
            Ty::Vec(_) => true,  // can always extend
            Ty::Fin(0) => false, // Fin 0 is empty
            Ty::Fin(_) => true,
            Ty::Prod(a, b) => a.is_inhabited(ctx) && b.is_inhabited(ctx),
            Ty::Sum(a, b) => a.is_inhabited(ctx) || b.is_inhabited(ctx),
            Ty::Fun(_, b) => b.is_inhabited(ctx),
            Ty::Sigma(_, a, b) => a.is_inhabited(ctx) && b.is_inhabited(ctx),
            Ty::Pi(_, _, b) => b.is_inhabited(ctx),
            Ty::Universe(_) => true,
            Ty::Named(s) => ctx.get(s).map(|t| t.is_inhabited(ctx)).unwrap_or(false),
        }
    }

    fn elements(&self) -> Option<Vec<String>> {
        match self {
            Ty::Unit => Some(vec!["tt".into()]),
            Ty::Void => Some(vec![]),
            Ty::Bool => Some(vec!["true".into(), "false".into()]),
            Ty::Fin(n) => Some((0..*n).map(|i| format!("fz^{i}fs")).collect()),
            Ty::Nat => None, // infinite
            Ty::Prod(a, b) => {
                let as_ = a.elements()?;
                let bs = b.elements()?;
                let mut pairs = vec![];
                for x in &as_ {
                    for y in &bs {
                        pairs.push(format!("({x}, {y})"));
                    }
                }
                Some(pairs)
            }
            Ty::Sum(a, b) => {
                let mut elems = vec![];
                if let Some(xs) = a.elements() {
                    for x in xs { elems.push(format!("inl({x})")); }
                }
                if let Some(ys) = b.elements() {
                    for y in ys { elems.push(format!("inr({y})")); }
                }
                Some(elems)
            }
            _ => None,
        }
    }

    fn cardinality(&self) -> Option<usize> {
        match self {
            Ty::Unit => Some(1),
            Ty::Void => Some(0),
            Ty::Bool => Some(2),
            Ty::Fin(n) => Some(*n),
            Ty::Nat => None,
            Ty::Prod(a, b) => Some(a.cardinality()? * b.cardinality()?),
            Ty::Sum(a, b) => Some(a.cardinality()? + b.cardinality()?),
            Ty::Fun(a, b) => {
                let na = a.cardinality()?;
                let nb = b.cardinality()?;
                Some(nb.pow(na as u32))
            }
            Ty::Vec(n) => {
                // Vec n over Bool = 2^n (fixed length vectors)
                Some(2usize.pow(*n as u32))
            }
            _ => None,
        }
    }
}

fn parse_ty(s: &str) -> Option<Ty> {
    let s = s.trim();
    // Simple cases
    match s {
        "Nat" | "N" | "nat" => return Some(Ty::Nat),
        "Bool" | "B" | "bool" => return Some(Ty::Bool),
        "Unit" | "1" => return Some(Ty::Unit),
        "Void" | "0" | "Empty" => return Some(Ty::Void),
        _ => {}
    }
    if s.starts_with("Vec ") {
        let n: usize = s[4..].trim().parse().ok()?;
        return Some(Ty::Vec(n));
    }
    if s.starts_with("Fin ") {
        let n: usize = s[4..].trim().parse().ok()?;
        return Some(Ty::Fin(n));
    }
    if s.starts_with("U") {
        if let Ok(n) = s[1..].parse::<usize>() { return Some(Ty::Universe(n)); }
    }
    // Product A * B  or  A x B
    for sep in [" * ", " x ", " × ", " X "] {
        if let Some(i) = s.find(sep) {
            let a = parse_ty(&s[..i])?;
            let b = parse_ty(&s[i+sep.len()..])?;
            return Some(Ty::Prod(Box::new(a), Box::new(b)));
        }
    }
    // Sum A + B
    if let Some(i) = s.find(" + ") {
        let a = parse_ty(&s[..i])?;
        let b = parse_ty(&s[i+3..])?;
        return Some(Ty::Sum(Box::new(a), Box::new(b)));
    }
    // Function A -> B
    if let Some(i) = s.find(" -> ") {
        let a = parse_ty(&s[..i])?;
        let b = parse_ty(&s[i+4..])?;
        return Some(Ty::Fun(Box::new(a), Box::new(b)));
    }
    // Sigma type: Sigma (x:A). B
    if s.starts_with("Sigma") || s.starts_with("exists") {
        let rest = s.trim_start_matches("Sigma").trim_start_matches("exists").trim();
        if rest.starts_with('(') {
            if let Some(colon) = rest.find(':') {
                let x = rest[1..colon].trim().to_string();
                if let Some(close) = rest.find(')') {
                    let a = parse_ty(&rest[colon+1..close])?;
                    let b_str = rest[close+1..].trim().trim_start_matches('.');
                    let b = parse_ty(b_str)?;
                    return Some(Ty::Sigma(x, Box::new(a), Box::new(b)));
                }
            }
        }
    }
    // Pi type: Pi (x:A). B
    if s.starts_with("Pi") || s.starts_with("forall") {
        let rest = s.trim_start_matches("Pi").trim_start_matches("forall").trim();
        if rest.starts_with('(') {
            if let Some(colon) = rest.find(':') {
                let x = rest[1..colon].trim().to_string();
                if let Some(close) = rest.find(')') {
                    let a = parse_ty(&rest[colon+1..close])?;
                    let b_str = rest[close+1..].trim().trim_start_matches('.');
                    let b = parse_ty(b_str)?;
                    return Some(Ty::Pi(x, Box::new(a), Box::new(b)));
                }
            }
        }
    }
    // Named
    if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Some(Ty::Named(s.to_string()));
    }
    None
}

// ── Vec type sandbox ──────────────────────────────────────────────────────────

struct VecSandbox {
    vecs: HashMap<String, (usize, Vec<i64>)>, // name -> (declared len, elements)
}

impl VecSandbox {
    fn new() -> Self { VecSandbox { vecs: HashMap::new() } }

    fn create(&mut self, name: &str, elems: Vec<i64>) {
        let n = elems.len();
        self.vecs.insert(name.to_string(), (n, elems));
        println!("  {} : Vec {n} = {:?}", cyan(name), self.vecs[name].1);
    }

    fn append(&mut self, r: &str, a: &str, b: &str) -> Result<(), String> {
        let (na, va) = self.vecs.get(a).cloned().ok_or(format!("{a} not found"))?;
        let (nb, vb) = self.vecs.get(b).cloned().ok_or(format!("{b} not found"))?;
        let mut res = va.clone();
        res.extend(vb.iter());
        let nr = na + nb;
        println!("  {} : Vec {na} ++ Vec {nb} = Vec {} = {:?}", cyan(r), nr, res);
        self.vecs.insert(r.to_string(), (nr, res));
        Ok(())
    }

    fn head(&self, name: &str) -> Result<(), String> {
        let (n, v) = self.vecs.get(name).ok_or(format!("{name} not found"))?;
        if *n == 0 { return Err("head: Vec 0 is empty — type system prevents this!".into()); }
        println!("  head {} = {}", cyan(name), v[0]);
        Ok(())
    }

    fn tail(&mut self, r: &str, name: &str) -> Result<(), String> {
        let (n, v) = self.vecs.get(name).cloned().ok_or(format!("{name} not found"))?;
        if n == 0 { return Err("tail: Vec 0 is empty — type system prevents this!".into()); }
        let tail = v[1..].to_vec();
        println!("  tail {} = {:?} : Vec {}", cyan(name), tail, n-1);
        self.vecs.insert(r.to_string(), (n-1, tail));
        Ok(())
    }

    fn index(&self, name: &str, i: usize) -> Result<(), String> {
        let (n, v) = self.vecs.get(name).ok_or(format!("{name} not found"))?;
        if i >= *n { return Err(format!("index {i} out of range for Vec {n} — type error!")); }
        println!("  {}[Fin {}] = {}", cyan(name), i, v[i]);
        Ok(())
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    types: HashMap<String, Ty>,
    vecs: VecSandbox,
}

impl Sandbox {
    fn new() -> Self {
        Sandbox { types: HashMap::new(), vecs: VecSandbox::new() }
    }

    fn print_help() {
        println!("{}", bold("── Type Construction ──────────────────────────────────────────────"));
        println!("  {}  T = Nat * Bool         — define a type", cyan("type"));
        println!("  {}  T                      — analyze a type", cyan("info"));
        println!("  {}  A B                    — test if A is isomorphic to B", cyan("iso"));
        println!("{}", bold("── Type Syntax ────────────────────────────────────────────────────"));
        println!("  Nat, Bool, Unit, Void, Vec n, Fin n, U0, U1");
        println!("  A * B  (product)    A + B  (sum/coproduct)    A -> B  (function)");
        println!("  Sigma (x:A). B      Pi (x:A). B");
        println!("{}", bold("── Vec Sandbox (length-indexed vectors) ───────────────────────────"));
        println!("  {}  v 1 2 3              — create Vec 3", cyan("vec"));
        println!("  {}  r a b               — append: Vec n ++ Vec m = Vec (n+m)", cyan("append"));
        println!("  {}  v                   — safe head (fails for Vec 0)", cyan("head"));
        println!("  {}  r v                 — safe tail", cyan("tail"));
        println!("  {}  v 2                 — safe index by Fin bound", cyan("index"));
        println!("{}", bold("── Exploration ────────────────────────────────────────────────────"));
        println!("  {}         — show all named types", cyan("show"));
        println!("  {}         — show type examples", cyan("examples"));
    }

    fn show_type_info(&self, ty: &Ty) {
        println!("  Type: {}", bold(&ty.display()));
        match ty.is_inhabited(&self.types) {
            true => println!("  Inhabited: {}", green("yes")),
            false => println!("  Inhabited: {}", red("no (empty type)")),
        }
        match ty.cardinality() {
            Some(n) => println!("  Cardinality: {}", cyan(&n.to_string())),
            None => println!("  Cardinality: {}", yellow("infinite")),
        }
        if let Some(elems) = ty.elements() {
            if elems.len() <= 16 {
                println!("  Elements: {{{}}}", elems.join(", "));
            } else {
                println!("  Elements: {{{}, ... ({} total)}}", elems[..4].join(", "), elems.len());
            }
        }
        // Curry-Howard correspondence
        let prop = match ty {
            Ty::Prod(_, _) => Some("∧ (conjunction)"),
            Ty::Sum(_, _) => Some("∨ (disjunction)"),
            Ty::Fun(_, _) => Some("→ (implication)"),
            Ty::Unit => Some("⊤ (truth)"),
            Ty::Void => Some("⊥ (falsehood)"),
            Ty::Sigma(_, _, _) => Some("∃ (existential)"),
            Ty::Pi(_, _, _) => Some("∀ (universal)"),
            _ => None,
        };
        if let Some(p) = prop {
            println!("  Propositions-as-types: {}", dim(p));
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
            "type" => {
                if let Some((name, expr)) = rest.split_once('=') {
                    let name = name.trim();
                    match parse_ty(expr.trim()) {
                        Some(ty) => {
                            self.show_type_info(&ty);
                            self.types.insert(name.to_string(), ty);
                        }
                        None => println!("  {} Parse error in: {}", red("✗"), expr),
                    }
                } else {
                    println!("  {} Use: type Name = expression", red("✗"));
                }
            }
            "info" => {
                let ty = if let Some(t) = self.types.get(rest) { t.clone() }
                    else { match parse_ty(rest) { Some(t) => t, None => { println!("  {} Parse error", red("✗")); return true; } } };
                self.show_type_info(&ty);
            }
            "iso" => {
                let ws: Vec<&str> = rest.splitn(2, ' ').collect();
                if ws.len() < 2 { println!("  {} Use: iso A B", red("✗")); return true; }
                let ta = if let Some(t) = self.types.get(ws[0]) { t.clone() }
                    else { match parse_ty(ws[0]) { Some(t) => t, None => { println!("  {} Parse error", red("✗")); return true; } } };
                let tb = if let Some(t) = self.types.get(ws[1]) { t.clone() }
                    else { match parse_ty(ws[1]) { Some(t) => t, None => { println!("  {} Parse error", red("✗")); return true; } } };
                let ca = ta.cardinality();
                let cb = tb.cardinality();
                println!("  {} : cardinality {:?}", ta.display(), ca);
                println!("  {} : cardinality {:?}", tb.display(), cb);
                if ca == cb && ca.is_some() {
                    println!("  {} Same cardinality — likely isomorphic!", green("≅?"));
                } else if ca.is_none() || cb.is_none() {
                    println!("  {} Cannot compare infinite types by cardinality alone.", yellow("?"));
                } else {
                    println!("  {} Different cardinalities — not isomorphic.", red("≇"));
                }
            }
            "show" => {
                if self.types.is_empty() {
                    println!("  {}", dim("(no named types)"));
                } else {
                    for (n, t) in &self.types {
                        let card = t.cardinality().map(|c| format!(" (|·| = {c})")).unwrap_or_default();
                        println!("  {} : {}{}", cyan(n), t.display(), dim(&card));
                    }
                }
            }
            "vec" => {
                let ws: Vec<&str> = rest.splitn(2, ' ').collect();
                if ws.is_empty() { println!("  {} Use: vec name e1 e2 ...", red("✗")); return true; }
                let name = ws[0];
                let elems: Vec<i64> = if ws.len() > 1 {
                    ws[1].split_whitespace().filter_map(|s| s.parse().ok()).collect()
                } else { vec![] };
                self.vecs.create(name, elems);
            }
            "append" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 3 { println!("  {} Use: append result a b", red("✗")); return true; }
                if let Err(e) = self.vecs.append(ws[0], ws[1], ws[2]) {
                    println!("  {} {}", red("✗"), e);
                }
            }
            "head" => {
                if let Err(e) = self.vecs.head(rest) { println!("  {} {}", red("✗"), e); }
            }
            "tail" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: tail result vec", red("✗")); return true; }
                if let Err(e) = self.vecs.tail(ws[0], ws[1]) { println!("  {} {}", red("✗"), e); }
            }
            "index" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: index vec n", red("✗")); return true; }
                let i: usize = match ws[1].parse() { Ok(n) => n, Err(_) => { println!("  {} Not a number", red("✗")); return true; } };
                if let Err(e) = self.vecs.index(ws[0], i) { println!("  {} {}", red("✗"), e); }
            }
            "examples" => {
                println!("{}", bold("── Type isomorphisms (Curry-Howard correspondence) ───────────────"));
                println!("  info Bool                    (2 elements, like a bit)");
                println!("  info Unit + Void             (should be ~Bool)");
                println!("  info Bool * Bool             (4 elements = 2×2)");
                println!("  info Bool -> Bool            (4 elements = 2^2 functions)");
                println!("  iso (A * B) -> C   A -> B -> C   (currying!)");
                println!("{}", bold("── Fin and Vec ────────────────────────────────────────────────────"));
                println!("  info Fin 0                   (empty — no valid index)");
                println!("  info Fin 5                   (5 bounded naturals: 0..4)");
                println!("  vec v 10 20 30               (Vec 3)");
                println!("  head v                       (safe — Vec 3 ≠ Vec 0)");
                println!("  index v 2                    (safe — 2 : Fin 3)");
                println!("  index v 5                    (type error — 5 not in Fin 3)");
                println!("{}", bold("── Dependent types ────────────────────────────────────────────────"));
                println!("  type DB = Sigma (n:Nat). Vec n   (DB with length in type)");
                println!("  type AllPos = Pi (n:Nat). Fin n -> Nat");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║     Dependent Types — Vec/Fin/Σ/Π Sandbox               ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build and analyze dependent types. Explore how Vec n and Fin n");
    println!("  encode length/bounds at the type level. Discover Curry-Howard.");
    println!("  Type {} for commands, {} for examples.\n", cyan("help"), cyan("examples"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}types{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
