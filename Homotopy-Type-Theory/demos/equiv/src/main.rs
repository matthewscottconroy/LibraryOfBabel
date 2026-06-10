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

// ── Finite type equivalences ──────────────────────────────────────────────────
//
// An equivalence f : A ≃ B consists of:
//   - f : A → B
//   - g : B → A  (quasi-inverse)
//   - η : ∀ a, g(f(a)) = a
//   - ε : ∀ b, f(g(b)) = b
//
// For finite types, we represent elements as integers and functions as tables.

#[derive(Clone)]
struct FinType {
    name: String,
    size: usize,
    labels: Vec<String>,
}

impl FinType {
    fn new(name: &str, labels: Vec<String>) -> Self {
        FinType { name: name.to_string(), size: labels.len(), labels }
    }

    fn display_elem(&self, i: usize) -> &str {
        self.labels.get(i).map(|s| s.as_str()).unwrap_or("?")
    }
}

#[derive(Clone)]
struct Equiv {
    name: String,
    dom: String,
    cod: String,
    // f and g as index maps
    fwd: Vec<usize>, // f: dom -> cod
    bwd: Vec<usize>, // g: cod -> dom
}

impl Equiv {
    fn check(&self, dom: &FinType, cod: &FinType) -> Vec<String> {
        let mut errors = vec![];
        if dom.size != self.fwd.len() {
            errors.push(format!("f has wrong domain size: {} vs {}", self.fwd.len(), dom.size));
        }
        if cod.size != self.bwd.len() {
            errors.push(format!("g has wrong codomain size: {} vs {}", self.bwd.len(), cod.size));
        }
        // Check η: g(f(a)) = a for all a
        for i in 0..dom.size {
            if i < self.fwd.len() {
                let fi = self.fwd[i];
                if fi < self.bwd.len() {
                    let gfi = self.bwd[fi];
                    if gfi != i { errors.push(format!("η fails at {}: g(f({})) = {} ≠ {}", dom.display_elem(i), dom.display_elem(i), dom.display_elem(gfi), dom.display_elem(i))); }
                }
            }
        }
        // Check ε: f(g(b)) = b for all b
        for j in 0..cod.size {
            if j < self.bwd.len() {
                let gj = self.bwd[j];
                if gj < self.fwd.len() {
                    let fgj = self.fwd[gj];
                    if fgj != j { errors.push(format!("ε fails at {}: f(g({})) = {} ≠ {}", cod.display_elem(j), cod.display_elem(j), cod.display_elem(fgj), cod.display_elem(j))); }
                }
            }
        }
        // Check sizes match
        if dom.size != cod.size { errors.push(format!("Sizes differ: |{}|={} ≠ |{}|={}", dom.name, dom.size, cod.name, cod.size)); }
        errors
    }

    fn show(&self, dom: &FinType, cod: &FinType) {
        println!("  {} : {} ≃ {}", bold(&self.name), cyan(&self.dom), cyan(&self.cod));
        println!("  {}", bold("Forward (f):"));
        for (i, &j) in self.fwd.iter().enumerate() {
            let di = dom.display_elem(i);
            let cj = if j < cod.size { cod.display_elem(j) } else { "?" };
            println!("    {} ↦ {}", cyan(di), cyan(cj));
        }
        println!("  {}", bold("Backward (g):"));
        for (j, &i) in self.bwd.iter().enumerate() {
            let cj = cod.display_elem(j);
            let di = if i < dom.size { dom.display_elem(i) } else { "?" };
            println!("    {} ↦ {}", cyan(cj), cyan(di));
        }
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    types: HashMap<String, FinType>,
    equivs: HashMap<String, Equiv>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { types: HashMap::new(), equivs: HashMap::new() };
        // Pre-load some types
        sb.types.insert("Bool".into(), FinType::new("Bool", vec!["false".into(), "true".into()]));
        sb.types.insert("Unit+Unit".into(), FinType::new("Unit+Unit", vec!["inl(tt)".into(), "inr(tt)".into()]));
        sb.types.insert("Fin2".into(), FinType::new("Fin2", vec!["0".into(), "1".into()]));
        sb.types.insert("Fin3".into(), FinType::new("Fin3", vec!["0".into(), "1".into(), "2".into()]));
        sb.types.insert("Fin4".into(), FinType::new("Fin4", vec!["0".into(), "1".into(), "2".into(), "3".into()]));
        sb
    }

    fn print_help() {
        println!("{}", bold("── Type Definition ─────────────────────────────────────────────────"));
        println!("  {}  A a0 a1 a2      — define a finite type", cyan("type"));
        println!("  {}                  — list all types", cyan("types"));
        println!("{}", bold("── Equivalences ────────────────────────────────────────────────────"));
        println!("  {}  e A B f g       — define equiv: f=fwd indices, g=bwd indices", cyan("equiv"));
        println!("  {}  e               — verify and display an equivalence", cyan("check"));
        println!("  {}  e               — show equivalence table", cyan("show"));
        println!("  {}  e1 e2           — compose two equivalences", cyan("comp"));
        println!("  {}  e               — compute the inverse equivalence", cyan("inv"));
        println!("{}", bold("── Univalence Consequences ─────────────────────────────────────────"));
        println!("  {}  A B             — compute automorphisms of A = A ≃ A", cyan("aut"));
        println!("  {}  A B             — check if same size (necessary for equiv)", cyan("same-size"));
        println!("{}", bold("── Presets ─────────────────────────────────────────────────────────"));
        println!("  {}                  — show interesting equivalences", cyan("examples"));
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
                if ws.is_empty() { println!("  {} Use: type Name e0 e1 ...", red("✗")); return true; }
                let name = ws[0];
                let labels: Vec<String> = ws[1..].iter().map(|s| s.to_string()).collect();
                if labels.is_empty() { println!("  {} Need at least one element", red("✗")); return true; }
                println!("  {} = {{{}}} (size {})", cyan(name), labels.join(", "), labels.len());
                self.types.insert(name.to_string(), FinType::new(name, labels));
            }
            "types" => {
                for (name, ty) in &self.types {
                    println!("  {} = {{{}}} (|·| = {})", cyan(name), ty.labels.join(", "), ty.size);
                }
            }
            "equiv" => {
                // equiv name A B fwd_indices bwd_indices
                // Example: equiv e Bool Fin2 0,1 0,1
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 5 { println!("  {} Use: equiv name A B fwd_idxs bwd_idxs", red("✗")); return true; }
                let (name, dom_name, cod_name) = (ws[0], ws[1], ws[2]);
                if !self.types.contains_key(dom_name) { println!("  {} Type {dom_name} not found", red("✗")); return true; }
                if !self.types.contains_key(cod_name) { println!("  {} Type {cod_name} not found", red("✗")); return true; }
                let parse_idx = |s: &str| -> Vec<usize> { s.split(',').filter_map(|x| x.parse().ok()).collect() };
                let fwd = parse_idx(ws[3]);
                let bwd = parse_idx(ws[4]);
                let e = Equiv { name: name.to_string(), dom: dom_name.to_string(), cod: cod_name.to_string(), fwd, bwd };
                println!("  Defined {}", cyan(name));
                self.equivs.insert(name.to_string(), e);
            }
            "check" => {
                if let Some(e) = self.equivs.get(rest) {
                    let e = e.clone();
                    let dom = self.types.get(&e.dom).cloned();
                    let cod = self.types.get(&e.cod).cloned();
                    match (dom, cod) {
                        (Some(d), Some(c)) => {
                            let errors = e.check(&d, &c);
                            if errors.is_empty() {
                                println!("  {} {} : {} ≃ {} is a valid equivalence!", green("✓"), cyan(&e.name), cyan(&e.dom), cyan(&e.cod));
                            } else {
                                println!("  {} Not a valid equivalence:", red("✗"));
                                for err in errors { println!("    {}", err); }
                            }
                        }
                        _ => println!("  {} Types not found", red("✗")),
                    }
                } else { println!("  {} Equiv {rest} not found", red("✗")); }
            }
            "show" => {
                if let Some(e) = self.equivs.get(rest) {
                    let e = e.clone();
                    let dom = self.types.get(&e.dom).cloned();
                    let cod = self.types.get(&e.cod).cloned();
                    if let (Some(d), Some(c)) = (dom, cod) { e.show(&d, &c); }
                } else { println!("  {} Equiv {rest} not found", red("✗")); }
            }
            "inv" => {
                if let Some(e) = self.equivs.get(rest) {
                    let e = e.clone();
                    let inv_name = format!("{}_inv", e.name);
                    let inv = Equiv { name: inv_name.clone(), dom: e.cod.clone(), cod: e.dom.clone(), fwd: e.bwd.clone(), bwd: e.fwd.clone() };
                    println!("  {} : {} ≃ {}", cyan(&inv_name), cyan(&inv.dom), cyan(&inv.cod));
                    self.equivs.insert(inv_name, inv);
                } else { println!("  {} Equiv {rest} not found", red("✗")); }
            }
            "comp" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: comp e1 e2", red("✗")); return true; }
                let e1 = self.equivs.get(ws[0]).cloned();
                let e2 = self.equivs.get(ws[1]).cloned();
                match (e1, e2) {
                    (Some(e1), Some(e2)) => {
                        if e1.cod != e2.dom {
                            println!("  {} Codomain of {} ({}) ≠ domain of {} ({})", red("✗"), ws[0], e1.cod, ws[1], e2.dom);
                            return true;
                        }
                        // Compose: (e2 ∘ e1)(a) = e2(e1(a))
                        let fwd: Vec<usize> = e1.fwd.iter().map(|&i| *e2.fwd.get(i).unwrap_or(&i)).collect();
                        let bwd: Vec<usize> = e2.bwd.iter().map(|&j| *e1.bwd.get(j).unwrap_or(&j)).collect();
                        let comp_name = format!("{}∘{}", ws[1], ws[0]);
                        let comp = Equiv { name: comp_name.clone(), dom: e1.dom.clone(), cod: e2.cod.clone(), fwd, bwd };
                        println!("  {} : {} ≃ {}", cyan(&comp_name), cyan(&comp.dom), cyan(&comp.cod));
                        self.equivs.insert(comp_name, comp);
                    }
                    _ => println!("  {} One or both equivs not found", red("✗")),
                }
            }
            "aut" => {
                // Compute all automorphisms A ≃ A (= all bijections)
                if !self.types.contains_key(rest) { println!("  {} Type {rest} not found", red("✗")); return true; }
                let ty = self.types[rest].clone();
                let n = ty.size;
                println!("  {} Aut({}) = all bijections {} → {}", bold("Computing"), cyan(rest), rest, rest);
                println!("  |Aut({rest})| = {}! = {}", n, factorial(n));
                if n <= 4 {
                    let mut perms = vec![];
                    gen_perms((0..n).collect(), &mut vec![], &mut perms);
                    for (k, p) in perms.iter().enumerate() {
                        let map: Vec<String> = p.iter().enumerate().map(|(i,&j)| format!("{}↦{}", ty.display_elem(i), ty.display_elem(j))).collect();
                        println!("  σ{k}: [{}]", map.join(", "));
                    }
                } else {
                    println!("  (too many to display — {} elements)", n);
                }
                println!("  {}", dim("By univalence: Aut(Bool) = ℤ/2ℤ (two automorphisms: id and swap)"));
            }
            "same-size" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: same-size A B", red("✗")); return true; }
                let ta = self.types.get(ws[0]);
                let tb = self.types.get(ws[1]);
                match (ta, tb) {
                    (Some(a), Some(b)) => {
                        if a.size == b.size {
                            println!("  {} |{}| = |{}| = {} — equivalence possible!", green("✓"), ws[0], ws[1], a.size);
                        } else {
                            println!("  {} |{}|={} ≠ |{}|={} — no equivalence exists.", red("✗"), ws[0], a.size, ws[1], b.size);
                        }
                    }
                    _ => println!("  {} Type(s) not found", red("✗")),
                }
            }
            "examples" => {
                println!("{}", bold("── Pre-built equivalences to explore ──────────────────────────────"));
                println!("  Bool ≃ Fin2:");
                println!("    equiv e Bool Fin2 0,1 0,1");
                println!("    check e");
                println!();
                println!("  Bool ≃ Unit+Unit:");
                println!("    equiv e Bool Unit+Unit 0,1 0,1");
                println!("    check e");
                println!();
                println!("  Automorphisms of Bool (= ℤ/2ℤ by univalence):");
                println!("    aut Bool");
                println!();
                println!("  Build and compose equivs:");
                println!("    type A x y  →  type B 0 1  →  type C a b");
                println!("    equiv e1 A B 0,1 0,1  →  equiv e2 B C 0,1 0,1");
                println!("    comp e1 e2  →  check e1∘e2");
                println!();
                println!("{}", bold("── Univalence axiom ────────────────────────────────────────────────"));
                println!("  ua : (A ≃ B) → (A = B)   [in universe 𝒰]");
                println!("  This means isomorphic types are literally equal in HoTT.");
                println!("  Automorphisms of Bool = paths Bool = Bool in universe,");
                println!("  and there are exactly 2 = |ℤ/2ℤ|.");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn factorial(n: usize) -> usize { if n == 0 { 1 } else { n * factorial(n-1) } }

fn gen_perms(remaining: Vec<usize>, current: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if remaining.is_empty() { out.push(current.clone()); return; }
    for i in 0..remaining.len() {
        let mut r2 = remaining.clone(); let x = r2.remove(i);
        current.push(x);
        gen_perms(r2, current, out);
        current.pop();
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║     Univalence — Type Equivalence Sandbox               ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Define finite types and build equivalences between them.");
    println!("  Verify that (f, g, η, ε) constitute a valid equivalence.");
    println!("  Explore Aut(A), composition, and univalence consequences.");
    println!("  Type {} for commands, {} for guided examples.\n", cyan("help"), cyan("examples"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}equiv{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
