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

// ── W-type trees ──────────────────────────────────────────────────────────────
//
// W(A, B) = sup(a, f) where a : A, f : B(a) → W(A,B)
// The W-type is the initial algebra of the polynomial functor X ↦ Σ(a:A). (B(a) → X)
//
// We work with labelled trees where:
//   - Each node carries a label (the 'a : A')
//   - The arity of a node is determined by its label (the 'B(a)')

#[derive(Clone, Debug)]
struct WTree {
    label: String,
    arity: usize,       // how many children this constructor expects
    children: Vec<WTree>,
}

impl WTree {
    fn display(&self, indent: usize) -> String {
        let prefix = " ".repeat(indent * 2);
        if self.children.is_empty() {
            format!("{prefix}{}()", cyan(&self.label))
        } else {
            let ch: Vec<String> = self.children.iter()
                .map(|c| format!("\n{}", c.display(indent + 1))).collect();
            format!("{prefix}{}({})", cyan(&self.label), ch.concat())
        }
    }

    fn size(&self) -> usize {
        1 + self.children.iter().map(|c| c.size()).sum::<usize>()
    }

    fn depth(&self) -> usize {
        if self.children.is_empty() { 1 }
        else { 1 + self.children.iter().map(|c| c.depth()).max().unwrap_or(0) }
    }

    fn leaves(&self) -> usize {
        if self.children.is_empty() { 1 }
        else { self.children.iter().map(|c| c.leaves()).sum() }
    }
}

// ── Signature (the 'A' and 'B' in W(A,B)) ────────────────────────────────────

struct Signature {
    name: String,
    constructors: HashMap<String, usize>, // constructor name -> arity
}

impl Signature {
    fn new(name: &str) -> Self { Signature { name: name.to_string(), constructors: HashMap::new() } }

    fn add_constructor(&mut self, name: &str, arity: usize) {
        self.constructors.insert(name.to_string(), arity);
    }

    fn show(&self) {
        println!("  {} {}", bold("W-type:"), cyan(&self.name));
        println!("  Constructors:");
        let mut ctors: Vec<_> = self.constructors.iter().collect();
        ctors.sort_by_key(|(n,_)| (*n).clone());
        for (name, arity) in &ctors {
            let arrow = if **arity == 0 {
                format!(": {}", self.name)
            } else {
                format!(": {} → {}", (0..**arity).map(|_| self.name.clone()).collect::<Vec<_>>().join(" → "), self.name)
            };
            println!("    {} {}", cyan(name), dim(&arrow));
        }
    }
}

// ── Parse simple W-tree expressions ──────────────────────────────────────────
// Grammar: tree = label | label '(' tree (',' tree)* ')'

fn parse_tree(s: &str, sig: &Signature) -> Result<WTree, String> {
    let s = s.trim();
    let pos = s.find('(');
    match pos {
        None => {
            // leaf: just a label
            let label = s.trim().to_string();
            let arity = sig.constructors.get(&label).copied()
                .ok_or_else(|| format!("Unknown constructor: {label}"))?;
            if arity != 0 { return Err(format!("{label} expects {arity} children")); }
            Ok(WTree { label, arity: 0, children: vec![] })
        }
        Some(i) => {
            let label = s[..i].trim().to_string();
            let arity = sig.constructors.get(&label).copied()
                .ok_or_else(|| format!("Unknown constructor: {label}"))?;
            if !s.ends_with(')') { return Err("Missing closing paren".into()); }
            let inner = &s[i+1..s.len()-1];
            let children = split_args(inner)?;
            if children.len() != arity {
                return Err(format!("{label} expects {arity} children, got {}", children.len()));
            }
            let mut ch = vec![];
            for c in &children { ch.push(parse_tree(c, sig)?); }
            Ok(WTree { label, arity, children: ch })
        }
    }
}

fn split_args(s: &str) -> Result<Vec<String>, String> {
    let mut args = vec![];
    let mut depth = 0;
    let mut cur = String::new();
    for ch in s.chars() {
        match ch {
            '(' => { depth += 1; cur.push(ch); }
            ')' => { depth -= 1; cur.push(ch); }
            ',' if depth == 0 => { args.push(cur.trim().to_string()); cur.clear(); }
            _ => cur.push(ch),
        }
    }
    if !cur.trim().is_empty() { args.push(cur.trim().to_string()); }
    Ok(args)
}

// ── Recursive operations on trees ────────────────────────────────────────────

fn tree_fold(tree: &WTree, leaf_val: &dyn Fn(&str) -> i64, node_val: &dyn Fn(&str, Vec<i64>) -> i64) -> i64 {
    if tree.children.is_empty() {
        leaf_val(&tree.label)
    } else {
        let child_vals: Vec<i64> = tree.children.iter().map(|c| tree_fold(c, leaf_val, node_val)).collect();
        node_val(&tree.label, child_vals)
    }
}

// Compute 'value' of a natural number tree (zero/succ encoding)
fn nat_of(tree: &WTree) -> Option<u64> {
    match tree.label.as_str() {
        "zero" if tree.children.is_empty() => Some(0),
        "succ" if tree.children.len() == 1 => nat_of(&tree.children[0]).map(|n| n + 1),
        _ => None,
    }
}

// Build natural number tree
fn nat_tree(n: u64) -> WTree {
    if n == 0 { WTree { label: "zero".into(), arity: 0, children: vec![] } }
    else { WTree { label: "succ".into(), arity: 1, children: vec![nat_tree(n - 1)] } }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    sigs: HashMap<String, Signature>,
    trees: HashMap<String, (String, WTree)>, // name -> (sig name, tree)
    current_sig: String,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { sigs: HashMap::new(), trees: HashMap::new(), current_sig: String::new() };
        // Pre-load: natural numbers as W-type
        let mut nat = Signature::new("ℕ");
        nat.add_constructor("zero", 0);
        nat.add_constructor("succ", 1);
        sb.sigs.insert("nat".into(), nat);
        // Pre-load: binary trees
        let mut btree = Signature::new("BTree");
        btree.add_constructor("leaf", 0);
        btree.add_constructor("node", 2);
        sb.sigs.insert("btree".into(), btree);
        // Pre-load: lists (like ℕ-indexed)
        let mut list = Signature::new("List");
        list.add_constructor("nil", 0);
        list.add_constructor("cons", 1);  // simplified: cons has 1 sub-list child
        sb.sigs.insert("list".into(), list);
        sb.current_sig = "nat".into();
        sb
    }

    fn cur_sig(&self) -> &Signature { &self.sigs[&self.current_sig] }

    fn print_help() {
        println!("{}", bold("── W-type Signatures ───────────────────────────────────────────────"));
        println!("  {}  Name                — create a new W-type", cyan("wtype"));
        println!("  {}  ctor n             — add constructor of arity n", cyan("ctor"));
        println!("  {}  Name               — switch to W-type", cyan("use"));
        println!("  {}                      — show current signature", cyan("sig"));
        println!("{}", bold("── Building Trees ──────────────────────────────────────────────────"));
        println!("  {}  t expr             — build and name a tree", cyan("build"));
        println!("  {}  succ(succ(zero)) — build a tree inline", cyan("tree"));
        println!("  {}  t                  — display a tree", cyan("show"));
        println!("  {}                      — list all named trees", cyan("trees"));
        println!("{}", bold("── Analysis ────────────────────────────────────────────────────────"));
        println!("  {}  t                  — size, depth, leaves", cyan("info"));
        println!("  {}  t                  — fold with addition (nat)", cyan("sum"));
        println!("  {}  n                  — build the W-nat for number n", cyan("nat"));
        println!("  {}         — show preloaded W-types", cyan("presets"));
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
            "wtype" => {
                if rest.is_empty() { println!("  {} Use: wtype Name", red("✗")); return true; }
                let sig = Signature::new(rest);
                self.sigs.insert(rest.to_string(), sig);
                self.current_sig = rest.to_string();
                println!("  Created W-type {}", cyan(rest));
            }
            "ctor" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: ctor name arity", red("✗")); return true; }
                let arity: usize = ws[1].parse().unwrap_or(0);
                let name = ws[0];
                let sig_name = self.current_sig.clone();
                if let Some(sig) = self.sigs.get_mut(&sig_name) {
                    sig.add_constructor(name, arity);
                    let arr = if arity == 0 { format!(": {}", sig.name) }
                        else { format!("{} → {}", std::iter::repeat(sig.name.clone()).take(arity).collect::<Vec<_>>().join(" → "), sig.name) };
                    println!("  Added {} {}", cyan(name), dim(&arr));
                } else { println!("  {} No current W-type", red("✗")); }
            }
            "use" | "switch" => {
                if self.sigs.contains_key(rest) {
                    self.current_sig = rest.to_string();
                    println!("  Using W-type {}", cyan(rest));
                    self.cur_sig().show();
                } else { println!("  {} W-type {rest} not found", red("✗")); }
            }
            "sig" => self.cur_sig().show(),
            "build" => {
                let ws: Vec<&str> = rest.splitn(2, ' ').collect();
                if ws.len() < 2 { println!("  {} Use: build name expr", red("✗")); return true; }
                let name = ws[0];
                let expr = ws[1];
                let sig_name = self.current_sig.clone();
                match parse_tree(expr, self.sigs.get(&sig_name).unwrap()) {
                    Ok(tree) => {
                        println!("{}", tree.display(2));
                        self.trees.insert(name.to_string(), (sig_name, tree));
                        println!("  {} {}", green("✓"), cyan(name));
                    }
                    Err(e) => println!("  {} {}", red("✗"), e),
                }
            }
            "tree" => {
                let sig_name = self.current_sig.clone();
                match parse_tree(rest, self.sigs.get(&sig_name).unwrap()) {
                    Ok(tree) => println!("{}", tree.display(2)),
                    Err(e) => println!("  {} {}", red("✗"), e),
                }
            }
            "show" => {
                if let Some((_, tree)) = self.trees.get(rest) {
                    let tree = tree.clone();
                    println!("{}", tree.display(2));
                    if let Some(n) = nat_of(&tree) { println!("  = {} (as ℕ)", cyan(&n.to_string())); }
                } else { println!("  {} Tree {rest} not found", red("✗")); }
            }
            "trees" => {
                if self.trees.is_empty() { println!("  {}", dim("(no trees)")); return true; }
                for (name, (sig, tree)) in &self.trees {
                    print!("  {} [{}] size={} depth={}", cyan(name), dim(sig), tree.size(), tree.depth());
                    if let Some(n) = nat_of(tree) { print!(" = {n}"); }
                    println!();
                }
            }
            "info" => {
                if let Some((_, tree)) = self.trees.get(rest) {
                    let tree = tree.clone();
                    println!("  Size (nodes):  {}", cyan(&tree.size().to_string()));
                    println!("  Depth:         {}", cyan(&tree.depth().to_string()));
                    println!("  Leaves:        {}", cyan(&tree.leaves().to_string()));
                    if let Some(n) = nat_of(&tree) { println!("  ℕ-value:       {}", cyan(&n.to_string())); }
                } else { println!("  {} Tree {rest} not found", red("✗")); }
            }
            "sum" => {
                if let Some((_, tree)) = self.trees.get(rest) {
                    let tree = tree.clone();
                    let s: i64 = tree_fold(&tree, &|_| 1i64, &|_, children: Vec<i64>| children.iter().sum::<i64>() + 1);
                    println!("  fold (+) {} = {}", cyan(rest), cyan(&s.to_string()));
                } else { println!("  {} Tree {rest} not found", red("✗")); }
            }
            "nat" => {
                let n: u64 = rest.parse().unwrap_or(0);
                if n > 10 { println!("  {} Max 10 for readability", yellow("ℹ")); return true; }
                // Switch to nat sig temporarily
                let old = self.current_sig.clone();
                self.current_sig = "nat".into();
                let tree = nat_tree(n);
                println!("  ℕ-encoding of {}:", n);
                println!("{}", tree.display(2));
                println!("  Size: {}", tree.size());
                self.current_sig = old;
            }
            "presets" => {
                println!("{}", bold("── Preloaded W-types ───────────────────────────────────────────────"));
                println!("  {}  ℕ = W({{zero,succ}}, arity)  — natural numbers", cyan("nat"));
                println!("    zero : ℕ              (no children)");
                println!("    succ : ℕ → ℕ          (one child)");
                println!("    Build: tree succ(succ(succ(zero)))   → 3");
                println!("    Or:    nat 5  → builds succ^5(zero)");
                println!();
                println!("  {}  BTree = W({{leaf,node}}, arity)  — binary trees", cyan("btree"));
                println!("    leaf : BTree           (no children)");
                println!("    node : BTree→BTree→BTree");
                println!("    Build: use btree → tree node(node(leaf,leaf),leaf)");
                println!();
                println!("  {}  List = W({{nil,cons}}, arity)  — linked lists (spine)", cyan("list"));
                println!("    nil  : List            (no children)");
                println!("    cons : List → List     (one sub-list)");
                println!();
                println!("{}", bold("── Key W-type insight ──────────────────────────────────────────────"));
                println!("  W(A,B) is the {}: it has elements of type A as", bold("initial algebra"));
                println!("  node-labels, and B(a) determines the arity (number of children)");
                println!("  of a node labeled a. This single construction encodes ℕ, List,");
                println!("  BTree, ordinals, and every inductive type.");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║       W-Types — The Universal Inductive Type Sandbox     ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Define W-type signatures (constructors + arities) and build trees.");
    println!("  See how ℕ, BTree, List, and ordinals are all special cases of W(A,B).");
    println!("  Type {} for commands, {} to see preloaded types.\n", cyan("help"), cyan("presets"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    println!("  Current: {} (nat). Try: {} or {}", cyan("ℕ"), cyan("nat 5"), cyan("tree succ(succ(zero))"));
    println!();
    loop {
        print!("  {}W[{}]{} > ", CYAN, sb.current_sig, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
