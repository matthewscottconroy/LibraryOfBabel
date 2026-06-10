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

// ── Union-Find for equivalence classes ───────────────────────────────────────

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self { UnionFind { parent: (0..n).collect(), rank: vec![0; n] } }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x { self.parent[x] = self.find(self.parent[x]); }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x); let ry = self.find(y);
        if rx == ry { return; }
        if self.rank[rx] < self.rank[ry] { self.parent[rx] = ry; }
        else if self.rank[rx] > self.rank[ry] { self.parent[ry] = rx; }
        else { self.parent[ry] = rx; self.rank[rx] += 1; }
    }

    fn classes(&mut self, n: usize) -> HashMap<usize, Vec<usize>> {
        let mut classes: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..n { classes.entry(self.find(i)).or_default().push(i); }
        classes
    }
}

// ── Quotient type sandbox ─────────────────────────────────────────────────────

struct QuotientSet {
    name: String,
    elems: Vec<String>,
    relations: Vec<(usize, usize)>, // pairs identified by ~
    uf: UnionFind,
}

impl QuotientSet {
    fn new(name: &str, elems: Vec<String>) -> Self {
        let n = elems.len();
        QuotientSet { name: name.to_string(), elems, relations: vec![], uf: UnionFind::new(n) }
    }

    fn identify(&mut self, a: &str, b: &str) -> Result<(), String> {
        let ia = self.elems.iter().position(|x| x == a).ok_or(format!("{a} not in set"))?;
        let ib = self.elems.iter().position(|x| x == b).ok_or(format!("{b} not in set"))?;
        self.relations.push((ia, ib));
        self.uf.union(ia, ib);
        Ok(())
    }

    fn same_class(&mut self, a: &str, b: &str) -> Result<bool, String> {
        let ia = self.elems.iter().position(|x| x == a).ok_or(format!("{a} not in set"))?;
        let ib = self.elems.iter().position(|x| x == b).ok_or(format!("{b} not in set"))?;
        Ok(self.uf.find(ia) == self.uf.find(ib))
    }

    fn class_of(&mut self, a: &str) -> Result<Vec<String>, String> {
        let ia = self.elems.iter().position(|x| x == a).ok_or(format!("{a} not in set"))?;
        let root = self.uf.find(ia);
        let n = self.elems.len();
        let class: Vec<String> = (0..n).filter(|&i| self.uf.find(i) == root)
            .map(|i| self.elems[i].clone()).collect();
        Ok(class)
    }

    fn classes(&mut self) -> Vec<Vec<String>> {
        let n = self.elems.len();
        let classes = self.uf.classes(n);
        let mut result: Vec<Vec<String>> = classes.values()
            .map(|ids| ids.iter().map(|&i| self.elems[i].clone()).collect())
            .collect();
        result.sort_by_key(|c| c[0].clone());
        result
    }

    fn show(&mut self) {
        let classes = self.classes();
        println!("  {} {}/{}", bold("Quotient:"), self.name, dim("~"));
        println!("  Original: {{{}}}", self.elems.join(", "));
        if self.relations.is_empty() {
            println!("  {} (no identifications yet)", dim("No ~"));
        } else {
            let rels: Vec<String> = self.relations.iter()
                .map(|(i,j)| format!("{} ~ {}", self.elems[*i], self.elems[*j])).collect();
            println!("  Relations: {}", rels.join(", "));
        }
        println!("  Equivalence classes ({} classes):", classes.len());
        for (i, cls) in classes.iter().enumerate() {
            println!("    [{}]₍{}₎ = {{{}}}", cls[0], i, cls.join(", "));
        }
    }
}

// ── Integer construction ℤ = ℕ×ℕ / ~ ─────────────────────────────────────────

fn int_equiv(a: (i64, i64), b: (i64, i64)) -> bool {
    // (a₁,a₂) ~ (b₁,b₂) iff a₁ + b₂ = a₂ + b₁
    a.0 + b.1 == a.1 + b.0
}

fn int_repr(n: i64) -> (i64, i64) {
    if n >= 0 { (n, 0) } else { (0, -n) }
}

fn int_add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) { (a.0 + b.0, a.1 + b.1) }
fn int_mul(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0*b.0 + a.1*b.1, a.0*b.1 + a.1*b.0)
}
fn int_val(p: (i64, i64)) -> i64 { p.0 - p.1 }
fn int_display(p: (i64, i64)) -> String {
    format!("({},{}) [= {}]", p.0, p.1, int_val(p))
}

// ── Rational construction ℚ = ℤ×ℤ₊ / ~ ──────────────────────────────────────

fn rat_equiv(a: (i64, i64), b: (i64, i64)) -> bool {
    a.0 * b.1 == b.0 * a.1
}
fn rat_add(a: (i64,i64), b: (i64,i64)) -> (i64,i64) { (a.0*b.1 + b.0*a.1, a.1*b.1) }
fn rat_mul(a: (i64,i64), b: (i64,i64)) -> (i64,i64) { (a.0*b.0, a.1*b.1) }
fn gcd(mut a: i64, mut b: i64) -> i64 { a = a.abs(); b = b.abs(); while b != 0 { let t = b; b = a % b; a = t; } a }
fn rat_reduce(a: (i64,i64)) -> (i64,i64) {
    if a.1 == 0 { return (0,0); }
    let g = gcd(a.0.abs(), a.1.abs());
    let sign = if a.1 < 0 { -1 } else { 1 };
    (sign * a.0 / g, sign * a.1 / g)
}
fn rat_display(p: (i64,i64)) -> String {
    let r = rat_reduce(p);
    if r.1 == 1 { format!("{}", r.0) } else { format!("{}/{}", r.0, r.1) }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    sets: HashMap<String, QuotientSet>,
    current: String,
    mode: String,
}

impl Sandbox {
    fn new() -> Self {
        Sandbox { sets: HashMap::new(), current: String::new(), mode: "quotient".into() }
    }

    fn print_help() {
        println!("{}", bold("── Quotient Set Builder ────────────────────────────────────────────"));
        println!("  {}  S a b c d          — create set S with elements", cyan("set"));
        println!("  {}  S                  — switch to set S", cyan("use"));
        println!("  {}  a b               — identify a ~ b", cyan("id"));
        println!("  {}  a                 — find class of element", cyan("class"));
        println!("  {}  a b              — check if a ~ b", cyan("same"));
        println!("  {}                    — display quotient set", cyan("show"));
        println!("{}", bold("── Integer Arithmetic ℤ = ℕ×ℕ/~ ───────────────────────────────────"));
        println!("  {}                   — enter integer mode", cyan("ints"));
        println!("  {}  3 0              — show pair representation of 3", cyan("int"));
        println!("  {}  (3,0) (0,2)      — add two integer pairs", cyan("int-add"));
        println!("  {}  (2,0) (0,1)      — multiply integer pairs", cyan("int-mul"));
        println!("  {}  (1,0) (0,1)      — check equivalence", cyan("int-eq"));
        println!("{}", bold("── Rational Arithmetic ℚ = ℤ×ℤ₊/~ ────────────────────────────────"));
        println!("  {}                   — enter rational mode", cyan("rats"));
        println!("  {}  1 3              — display rational 1/3", cyan("rat"));
        println!("  {}  1 2   1 3        — add 1/2 + 1/3", cyan("rat-add"));
        println!("  {}  2 3   3 4        — multiply 2/3 * 3/4", cyan("rat-mul"));
        println!("  {}  2 4   1 2        — check equivalence 2/4 ~ 1/2", cyan("rat-eq"));
        println!("{}", bold("── Preset quotient constructions ───────────────────────────────────"));
        println!("  {}  circle|Z2|Z3|Z6  — load a preset", cyan("preset"));
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
            "set" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: set Name e1 e2 ...", red("✗")); return true; }
                let name = ws[0];
                let elems: Vec<String> = ws[1..].iter().map(|s| s.to_string()).collect();
                let qs = QuotientSet::new(name, elems.clone());
                self.sets.insert(name.to_string(), qs);
                self.current = name.to_string();
                println!("  Created {} = {{{}}}", cyan(name), elems.join(", "));
            }
            "use" | "switch" => {
                if self.sets.contains_key(rest) {
                    self.current = rest.to_string();
                    println!("  Using {}", cyan(rest));
                } else {
                    println!("  {} Set {rest} not found", red("✗"));
                }
            }
            "id" | "identify" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: id a b", red("✗")); return true; }
                if self.current.is_empty() { println!("  {} No current set", red("✗")); return true; }
                let cur = self.current.clone();
                match self.sets.get_mut(&cur).unwrap().identify(ws[0], ws[1]) {
                    Ok(()) => println!("  {} {} ~ {}", green("✓"), cyan(ws[0]), cyan(ws[1])),
                    Err(e) => println!("  {} {}", red("✗"), e),
                }
            }
            "class" => {
                let cur = self.current.clone();
                if cur.is_empty() { println!("  {} No current set", red("✗")); return true; }
                match self.sets.get_mut(&cur).unwrap().class_of(rest) {
                    Ok(cls) => println!("  [{}] = {{{}}}", cyan(rest), cls.join(", ")),
                    Err(e) => println!("  {} {}", red("✗"), e),
                }
            }
            "same" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: same a b", red("✗")); return true; }
                let cur = self.current.clone();
                if cur.is_empty() { println!("  {} No current set", red("✗")); return true; }
                match self.sets.get_mut(&cur).unwrap().same_class(ws[0], ws[1]) {
                    Ok(true) => println!("  {} {} ~ {} in {}", green("✓"), cyan(ws[0]), cyan(ws[1]), cur),
                    Ok(false) => println!("  {} {} ≁ {}", red("✗"), ws[0], ws[1]),
                    Err(e) => println!("  {} {}", red("✗"), e),
                }
            }
            "show" => {
                let cur = self.current.clone();
                if cur.is_empty() { println!("  {} No current set", red("✗")); return true; }
                self.sets.get_mut(&cur).unwrap().show();
            }
            "ints" => { self.mode = "int".into(); println!("  Integer mode. Pairs (a,b) represent a-b. Try: int-add (3,0) (0,2)"); }
            "rats" => { self.mode = "rat".into(); println!("  Rational mode. Pairs (p,q) represent p/q. Try: rat-add 1 2   1 3"); }
            "int" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() >= 2 {
                    let a: i64 = ws[0].parse().unwrap_or(0);
                    let b: i64 = ws[1].parse().unwrap_or(0);
                    println!("  ({a},{b}) ≅ {}", int_val((a,b)));
                } else if let Ok(n) = rest.parse::<i64>() {
                    let p = int_repr(n);
                    println!("  {} = {}", n, int_display(p));
                } else {
                    println!("  {} Use: int n  or  int a b", red("✗"));
                }
            }
            "int-add" => {
                let nums: Vec<i64> = rest.split_whitespace()
                    .filter_map(|s| s.trim_matches(|c| c == '(' || c == ')').parse().ok()).collect();
                if nums.len() >= 4 {
                    let a = (nums[0], nums[1]); let b = (nums[2], nums[3]);
                    let c = int_add(a, b);
                    println!("  {} + {} = {} [= {}]", int_display(a), int_display(b), int_display(c), int_val(c));
                } else { println!("  {} Use: int-add (a1,a2) (b1,b2)", red("✗")); }
            }
            "int-mul" => {
                let nums: Vec<i64> = rest.split_whitespace()
                    .filter_map(|s| s.trim_matches(|c| c == '(' || c == ')').parse().ok()).collect();
                if nums.len() >= 4 {
                    let a = (nums[0], nums[1]); let b = (nums[2], nums[3]);
                    let c = int_mul(a, b);
                    println!("  {} × {} = {} [= {}]", int_display(a), int_display(b), int_display(c), int_val(c));
                } else { println!("  {} Use: int-mul (a1,a2) (b1,b2)", red("✗")); }
            }
            "int-eq" => {
                let nums: Vec<i64> = rest.split_whitespace()
                    .filter_map(|s| s.trim_matches(|c| c == '(' || c == ')').parse().ok()).collect();
                if nums.len() >= 4 {
                    let a = (nums[0], nums[1]); let b = (nums[2], nums[3]);
                    if int_equiv(a, b) { println!("  {} {} ~ {} (same integer {})", green("~"), int_display(a), int_display(b), int_val(a)); }
                    else { println!("  {} {} ≁ {} (different: {} vs {})", red("≁"), int_display(a), int_display(b), int_val(a), int_val(b)); }
                } else { println!("  {} Use: int-eq (a1,a2) (b1,b2)", red("✗")); }
            }
            "rat" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() >= 2 {
                    let p: i64 = ws[0].parse().unwrap_or(0);
                    let q: i64 = ws[1].parse().unwrap_or(1);
                    println!("  ({p},{q}) = {}", rat_display((p,q)));
                } else { println!("  {} Use: rat p q", red("✗")); }
            }
            "rat-add" => {
                let ws: Vec<i64> = rest.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if ws.len() >= 4 {
                    let a = (ws[0],ws[1]); let b = (ws[2],ws[3]);
                    let c = rat_add(a, b);
                    println!("  {}/{} + {}/{} = {}", ws[0],ws[1],ws[2],ws[3], rat_display(c));
                } else { println!("  {} Use: rat-add p1 q1 p2 q2", red("✗")); }
            }
            "rat-mul" => {
                let ws: Vec<i64> = rest.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if ws.len() >= 4 {
                    let a = (ws[0],ws[1]); let b = (ws[2],ws[3]);
                    let c = rat_mul(a, b);
                    println!("  {}/{} × {}/{} = {}", ws[0],ws[1],ws[2],ws[3], rat_display(c));
                } else { println!("  {} Use: rat-mul p1 q1 p2 q2", red("✗")); }
            }
            "rat-eq" => {
                let ws: Vec<i64> = rest.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if ws.len() >= 4 {
                    let a = (ws[0],ws[1]); let b = (ws[2],ws[3]);
                    if rat_equiv(a, b) { println!("  {} {}/{} ~ {}/{} (equal as fractions)", green("~"), ws[0],ws[1],ws[2],ws[3]); }
                    else { println!("  {} {}/{} ≁ {}/{}", red("≁"), ws[0],ws[1],ws[2],ws[3]); }
                } else { println!("  {} Use: rat-eq p1 q1 p2 q2", red("✗")); }
            }
            "preset" => {
                match rest {
                    "circle" => {
                        // S¹ = {0,1,...,n-1} with n~0 is handled by a different approach
                        // Here: circle = {0,1,2,3} / {0~3} to glue endpoints of interval
                        let mut qs = QuotientSet::new("circle", vec!["0","1","2","3","0'"].iter().map(|s|s.to_string()).collect());
                        let _ = qs.identify("0", "0'");
                        self.sets.insert("circle".into(), qs);
                        self.current = "circle".into();
                        println!("  Circle = path with endpoints identified");
                        self.sets.get_mut("circle").unwrap().show();
                    }
                    "Z2" => {
                        let mut qs = QuotientSet::new("Z2", (0i32..6).map(|i| i.to_string()).collect());
                        for i in (0i32..6).step_by(2) { let _ = qs.identify(&i.to_string(), &(i+1).to_string()); }
                        self.sets.insert("Z2".into(), qs);
                        self.current = "Z2".into();
                        println!("  ℤ/2ℤ: pair up 0~1, 2~3, 4~5");
                        self.sets.get_mut("Z2").unwrap().show();
                    }
                    "Z3" => {
                        let mut qs = QuotientSet::new("Z3", (0i32..9).map(|i| i.to_string()).collect());
                        for i in 0i32..9 { let j = (i / 3) * 3; let _ = qs.identify(&i.to_string(), &j.to_string()); }
                        self.sets.insert("Z3".into(), qs);
                        self.current = "Z3".into();
                        println!("  ℤ/3ℤ: group {{0,1,2}}, {{3,4,5}}, {{6,7,8}}");
                        self.sets.get_mut("Z3").unwrap().show();
                    }
                    "Z6" => {
                        println!("  ℤ/6ℤ: built from integers 0..5, showing quotient construction");
                        println!("  {} Try: set Z6 0 1 2 3 4 5 6 7 8 9 10 11", dim("Hint:"));
                        println!("  {}       then: id 0 6 → id 1 7 → id 2 8 → ...", dim("     "));
                    }
                    _ => println!("  {} Presets: circle, Z2, Z3, Z6", red("✗")),
                }
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Quotient Types — ℤ, ℚ, and Identification Sandbox     ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build quotient sets by identifying elements under a relation.");
    println!("  Construct ℤ from ℕ×ℕ and ℚ from integer pairs, step by step.");
    println!("  Type {} for commands, {} for preset constructions.\n", cyan("help"), cyan("preset"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}quot{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
