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

// ── Type families over a base ─────────────────────────────────────────────────
//
// Transport: given p : a = b and u : P(a), we get transport(p, u) : P(b).
//
// We model this with:
//   - A base type (discrete, finite)
//   - A type family P : Base → Type
//   - Elements of P(x) for various x
//   - Paths (equalities) in the base
//   - Transport along paths

// ── Value kinds in type families ──────────────────────────────────────────────

enum FamilyKind {
    Constant(String),               // P(x) = T always
    Identity,                       // P(x) = x  (the identity family over ℕ/Fin)
    Pred(Box<dyn Fn(usize) -> String + Send + Sync>), // computed
    Succ,                           // P(n) = n+1
    Double,                         // P(n) = 2n
    Neg,                            // P(n) = -n (mod)
    Power(usize),                   // P(n) = n^k
}

impl Clone for FamilyKind {
    fn clone(&self) -> Self {
        match self {
            FamilyKind::Constant(s) => FamilyKind::Constant(s.clone()),
            FamilyKind::Identity => FamilyKind::Identity,
            FamilyKind::Succ => FamilyKind::Succ,
            FamilyKind::Double => FamilyKind::Double,
            FamilyKind::Neg => FamilyKind::Neg,
            FamilyKind::Power(k) => FamilyKind::Power(*k),
            FamilyKind::Pred(_) => FamilyKind::Constant("custom".into()),
        }
    }
}

impl std::fmt::Debug for FamilyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl FamilyKind {
    fn apply(&self, x: usize) -> String {
        match self {
            FamilyKind::Constant(s) => s.clone(),
            FamilyKind::Identity => x.to_string(),
            FamilyKind::Succ => (x + 1).to_string(),
            FamilyKind::Double => (2 * x).to_string(),
            FamilyKind::Neg => format!("{}", -(x as i64)),
            FamilyKind::Power(k) => x.pow(*k as u32).to_string(),
            FamilyKind::Pred(_) => "?".into(),
        }
    }

    fn name(&self) -> String {
        match self {
            FamilyKind::Constant(s) => format!("const({s})"),
            FamilyKind::Identity => "id".into(),
            FamilyKind::Succ => "succ".into(),
            FamilyKind::Double => "double".into(),
            FamilyKind::Neg => "neg".into(),
            FamilyKind::Power(k) => format!("pow{k}"),
            FamilyKind::Pred(_) => "custom".into(),
        }
    }
}

// ── Transport sandbox ─────────────────────────────────────────────────────────

struct Family {
    name: String,
    kind: FamilyKind,
}

struct BasePoint {
    name: String,
    value: usize,
}

struct Elem {
    name: String,
    base: String,
    value: String,
    family: String,
}

struct Path {
    name: String,
    from: String,
    to: String,
}

struct Sandbox {
    points: HashMap<String, usize>,  // name -> value
    families: HashMap<String, FamilyKind>,
    elems: HashMap<String, Elem>,
    paths: HashMap<String, Path>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox {
            points: HashMap::new(),
            families: HashMap::new(),
            elems: HashMap::new(),
            paths: HashMap::new(),
        };
        // Pre-load some base points
        for i in 0usize..6 { sb.points.insert(i.to_string(), i); }
        // Pre-load families
        sb.families.insert("id".into(), FamilyKind::Identity);
        sb.families.insert("succ".into(), FamilyKind::Succ);
        sb.families.insert("double".into(), FamilyKind::Double);
        sb.families.insert("square".into(), FamilyKind::Power(2));
        sb.families.insert("cube".into(), FamilyKind::Power(3));
        sb.families.insert("neg".into(), FamilyKind::Neg);
        sb
    }

    fn get_point(&self, s: &str) -> Option<usize> {
        self.points.get(s).copied().or_else(|| s.parse().ok())
    }

    fn print_help() {
        println!("{}", bold("── Base Type ───────────────────────────────────────────────────────"));
        println!("  {}  p 3              — add a named base point (value 3)", cyan("point"));
        println!("  {}                  — show all base points", cyan("points"));
        println!("{}", bold("── Type Families ───────────────────────────────────────────────────"));
        println!("  {}  F id|succ|double|square|neg — load a preset family", cyan("family"));
        println!("  {}  F const K         — constant family P(x)=K", cyan("family"));
        println!("  {}  F x               — show P(x) for all points", cyan("show-fam"));
        println!("{}", bold("── Elements ────────────────────────────────────────────────────────"));
        println!("  {}  e F p            — create element e : P(p)", cyan("elem"));
        println!("  {}  e               — show element", cyan("show-elem"));
        println!("{}", bold("── Paths and Transport ─────────────────────────────────────────────"));
        println!("  {}  p a b           — create path a = b", cyan("path"));
        println!("  {}  e p             — transport e along path p", cyan("transport"));
        println!("  {}  e p1 p2         — transport along composed path", cyan("transport*"));
        println!("{}", bold("── Exploration ─────────────────────────────────────────────────────"));
        println!("  {}         — show all families, points, elems, paths", cyan("show"));
        println!("  {}         — explain transport in HoTT", cyan("explain"));
        println!("  {}         — interactive transport examples", cyan("examples"));
    }

    fn do_transport(&self, elem_name: &str, path_name: &str) -> Result<(), String> {
        let elem = self.elems.get(elem_name).ok_or_else(|| format!("Element {elem_name} not found"))?;
        let path = self.paths.get(path_name).ok_or_else(|| format!("Path {path_name} not found"))?;

        // Check that elem is at the start of the path
        if elem.base != path.from {
            return Err(format!("Element {} : P({}) but path starts at {}", elem_name, elem.base, path.from));
        }

        let from_val = self.get_point(&path.from).ok_or_else(|| format!("Point {} not found", path.from))?;
        let to_val   = self.get_point(&path.to).ok_or_else(|| format!("Point {} not found", path.to))?;
        let fam = self.families.get(&elem.family).ok_or_else(|| format!("Family {} not found", elem.family))?;

        println!("  {}", bold("Transport:"));
        println!("  Given:   {} : {}({}={}) = P({})", cyan(elem_name), cyan(&elem.family), from_val, from_val, from_val);
        println!("  Path:    {} : {} = {}", cyan(path_name), path.from, path.to);
        println!("  Family:  {}(x) = x mapped by {}", cyan(&elem.family), fam.name());

        let old_val = &elem.value;
        let new_val = fam.apply(to_val);

        println!("  Before:  {} : P({}) = {}", old_val, path.from, fam.apply(from_val));
        println!("  After:   {} : P({}) = {}", green(&new_val), path.to, green(&new_val));
        println!("  {} transport({}, {}) = {}", green("→"), path_name, elem_name, bold(&new_val));

        Ok(())
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
            "point" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.is_empty() { println!("  {} Use: point name [value]", red("✗")); return true; }
                let name = ws[0];
                let val: usize = ws.get(1).and_then(|s| s.parse().ok()).unwrap_or_else(|| {
                    name.parse().unwrap_or(self.points.len())
                });
                self.points.insert(name.to_string(), val);
                println!("  {} = {}", cyan(name), val);
            }
            "points" => {
                let mut pts: Vec<_> = self.points.iter().collect();
                pts.sort_by_key(|(n, _)| (*n).clone());
                for (name, val) in pts { println!("  {} = {}", cyan(name), val); }
            }
            "family" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.is_empty() { println!("  {} Use: family Name kind", red("✗")); return true; }
                let name = ws[0];
                let kind_str = ws.get(1).copied().unwrap_or("id");
                let kind = match kind_str {
                    "id" | "identity" => FamilyKind::Identity,
                    "succ" | "successor" => FamilyKind::Succ,
                    "double" => FamilyKind::Double,
                    "square" => FamilyKind::Power(2),
                    "cube" => FamilyKind::Power(3),
                    "neg" => FamilyKind::Neg,
                    "const" => {
                        let k = ws.get(2).copied().unwrap_or("K").to_string();
                        FamilyKind::Constant(k)
                    }
                    _ => { println!("  {} Unknown family kind: {kind_str}", red("✗")); return true; }
                };
                println!("  {} : {} → Type = {}", cyan(name), "Base", kind.name());
                self.families.insert(name.to_string(), kind);
            }
            "show-fam" => {
                let fam = self.families.get(rest).ok_or_else(|| format!("Family {rest} not found"));
                match fam {
                    Ok(fam) => {
                        let fam = fam.clone();
                        println!("  {}(x):", cyan(rest));
                        let mut pts: Vec<_> = self.points.iter().collect();
                        pts.sort_by_key(|(_, v)| **v);
                        for (name, &val) in &pts {
                            println!("  {}({}) = {}", rest, cyan(name), cyan(&fam.apply(val)));
                        }
                    }
                    Err(e) => println!("  {} {}", red("✗"), e),
                }
            }
            "elem" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 3 { println!("  {} Use: elem name Family point", red("✗")); return true; }
                let (ename, fname, pname) = (ws[0], ws[1], ws[2]);
                let fam = self.families.get(fname).cloned();
                let pt = self.get_point(pname);
                match (fam, pt) {
                    (Some(f), Some(val)) => {
                        let value = f.apply(val);
                        println!("  {} : {}({}) = {}", cyan(ename), fname, pname, cyan(&value));
                        self.elems.insert(ename.to_string(), Elem {
                            name: ename.to_string(), base: pname.to_string(),
                            value, family: fname.to_string(),
                        });
                    }
                    (None, _) => println!("  {} Family {fname} not found", red("✗")),
                    (_, None) => println!("  {} Point {pname} not found", red("✗")),
                }
            }
            "show-elem" => {
                if let Some(e) = self.elems.get(rest) {
                    println!("  {} : {}({}) = {}", cyan(&e.name), e.family, e.base, cyan(&e.value));
                } else { println!("  {} Elem {rest} not found", red("✗")); }
            }
            "path" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 3 { println!("  {} Use: path name from to", red("✗")); return true; }
                let (pname, from, to) = (ws[0], ws[1], ws[2]);
                if self.get_point(from).is_none() { println!("  {} Point {from} not found", red("✗")); return true; }
                if self.get_point(to).is_none() { println!("  {} Point {to} not found", red("✗")); return true; }
                println!("  {} : {} = {}", cyan(pname), from, to);
                self.paths.insert(pname.to_string(), Path { name: pname.to_string(), from: from.to_string(), to: to.to_string() });
            }
            "transport" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: transport elem path", red("✗")); return true; }
                if let Err(e) = self.do_transport(ws[0], ws[1]) { println!("  {} {}", red("✗"), e); }
            }
            "transport*" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 3 { println!("  {} Use: transport* elem path1 path2 ...", red("✗")); return true; }
                let elem_name = ws[0];
                let paths = &ws[1..];
                // Validate chain
                let mut cur_base = self.elems.get(elem_name).map(|e| e.base.clone())
                    .ok_or_else(|| format!("Elem {elem_name} not found"));
                match cur_base {
                    Err(e) => { println!("  {} {}", red("✗"), e); return true; }
                    Ok(ref mut base) => {
                        println!("  {} Transporting {} along {} paths:", bold("→"), cyan(elem_name), paths.len());
                        for &pname in paths {
                            if let Some(path) = self.paths.get(pname) {
                                if path.from != *base {
                                    println!("  {} Path {pname} starts at {} but element is at {}", red("✗"), path.from, base);
                                    return true;
                                }
                                let fam_name = self.elems.get(elem_name).unwrap().family.clone();
                                let to_val = self.get_point(&path.to).unwrap_or(0);
                                let fam = self.families.get(&fam_name).cloned();
                                if let Some(f) = fam {
                                    let new_val = f.apply(to_val);
                                    println!("  transport({pname}, ·) : P({}) → P({}) = {}", base, path.to, cyan(&new_val));
                                    *base = path.to.clone();
                                }
                            } else { println!("  {} Path {pname} not found", red("✗")); return true; }
                        }
                    }
                }
            }
            "show" => {
                println!("{}", bold("  Base points:"));
                let mut pts: Vec<_> = self.points.iter().collect();
                pts.sort_by_key(|(_, v)| **v);
                for (n, v) in pts { print!("  {} = {}    ", cyan(n), v); } println!();
                println!("{}", bold("  Families:"));
                for (n, f) in &self.families { print!("  {}({})    ", cyan(n), f.name()); } println!();
                println!("{}", bold("  Elements:"));
                for (_, e) in &self.elems { println!("  {} : {}({}) = {}", cyan(&e.name), e.family, e.base, e.value); }
                println!("{}", bold("  Paths:"));
                for (_, p) in &self.paths { println!("  {} : {} = {}", cyan(&p.name), p.from, p.to); }
            }
            "explain" => {
                println!("{}", bold("── Transport in HoTT ───────────────────────────────────────────────"));
                println!("  Given:");
                println!("    P : A → Type    (a type family over A)");
                println!("    p : a = b       (a path in A)");
                println!("    u : P(a)        (an element in the fiber over a)");
                println!("  Transport gives:");
                println!("    transport(p, u) : P(b)");
                println!();
                println!("  Key properties:");
                println!("  • transport(refl, u) = u");
                println!("  • transport(p · q, u) = transport(q, transport(p, u))");
                println!("  • transport(p⁻¹, transport(p, u)) = u");
                println!();
                println!("  Geometrically: transport slides u along the path p");
                println!("  from the fiber over a to the fiber over b.");
                println!();
                println!("  In vector bundles: parallel transport along a curve.");
                println!("  In type theory:    substitution of witnesses along equalities.");
            }
            "examples" => {
                println!("{}", bold("── Try this sequence ───────────────────────────────────────────────"));
                println!("  point a 2          → define a = 2");
                println!("  point b 5          → define b = 5");
                println!("  family P square    → P(x) = x²");
                println!("  elem u P a         → u : P(2) = 4");
                println!("  path p a b         → p : 2 = 5");
                println!("  transport u p      → transport(p, u) : P(5) = 25");
                println!();
                println!("  Then try the constant family:");
                println!("  family Q const Hello");
                println!("  elem v Q a         → v : Q(2) = Hello");
                println!("  transport v p      → transport(p, v) : Q(5) = Hello  (constant!)");
                println!();
                println!("  The point: transport along p : a=b moves value from P(a) to P(b).");
                println!("  A constant family doesn't change; the identity family changes value.");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║       Transport — Sliding Values Along Paths Sandbox     ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Define type families P : Base → Type, create elements, and");
    println!("  transport them along paths using the J eliminator principle.");
    println!("  Type {} for commands, {} for a guided example.\n", cyan("help"), cyan("examples"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}transport{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
