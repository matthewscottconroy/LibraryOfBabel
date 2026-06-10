// Group Laboratory — build groups, check axioms, discover structure
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Write};

const B: &str = "\x1b[1m";  const R: &str = "\x1b[0m";
const G: &str = "\x1b[32m"; const C: &str = "\x1b[36m";
const Y: &str = "\x1b[33m"; const D: &str = "\x1b[2m";
const RE: &str = "\x1b[31m";

fn read_line(prompt: &str) -> String {
    print!("{B}{prompt}{R} ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().lock().read_line(&mut s).unwrap();
    s.trim().to_string()
}

// ── Group state ──────────────────────────────────────────────────────────────

struct Group {
    elems: Vec<String>,
    table: Vec<Vec<Option<usize>>>,  // table[a][b] = a*b
}

impl Group {
    fn new(elems: Vec<String>) -> Self {
        let n = elems.len();
        Group { table: vec![vec![None; n]; n], elems }
    }

    fn idx(&self, name: &str) -> Option<usize> {
        self.elems.iter().position(|e| e == name)
    }

    fn mul(&self, a: usize, b: usize) -> Option<usize> {
        self.table[a][b]
    }

    fn mul_name(&self, a: &str, b: &str) -> Option<String> {
        let ia = self.idx(a)?; let ib = self.idx(b)?;
        let ic = self.mul(ia, ib)?;
        Some(self.elems[ic].clone())
    }

    fn check_closed(&self) -> bool {
        for row in &self.table {
            for &cell in row {
                if cell.is_none() { return false; }
            }
        }
        true
    }

    fn check_assoc(&self) -> Option<(String, String, String)> {
        let n = self.elems.len();
        for a in 0..n {
            for b in 0..n {
                for c in 0..n {
                    let ab = self.mul(a, b)?;
                    let bc = self.mul(b, c)?;
                    let lhs = self.mul(ab, c)?;
                    let rhs = self.mul(a, bc)?;
                    if lhs != rhs {
                        return Some((
                            self.elems[a].clone(),
                            self.elems[b].clone(),
                            self.elems[c].clone(),
                        ));
                    }
                }
            }
        }
        None
    }

    fn find_identity(&self) -> Option<usize> {
        let n = self.elems.len();
        'outer: for e in 0..n {
            for a in 0..n {
                if self.mul(e, a) != Some(a) { continue 'outer; }
                if self.mul(a, e) != Some(a) { continue 'outer; }
            }
            return Some(e);
        }
        None
    }

    fn check_inverses(&self, id: usize) -> Option<(String, String)> {
        let n = self.elems.len();
        for a in 0..n {
            let has_inv = (0..n).any(|b| {
                self.mul(a, b) == Some(id) && self.mul(b, a) == Some(id)
            });
            if !has_inv {
                return Some((self.elems[a].clone(), "?".into()));
            }
        }
        None
    }

    fn element_order(&self, a: usize) -> Option<usize> {
        let id = self.find_identity()?;
        let mut cur = a;
        for k in 1..=self.elems.len() {
            if cur == id { return Some(k); }
            cur = self.mul(cur, a)?;
        }
        None
    }

    fn subgroups(&self) -> Vec<Vec<usize>> {
        let n = self.elems.len();
        let id = match self.find_identity() { Some(i) => i, None => return vec![] };
        let mut result = vec![];
        // Check all subsets containing identity
        for mask in 0u64..(1u64 << n) {
            if mask & (1 << id) == 0 { continue; }
            let subset: Vec<usize> = (0..n).filter(|&i| mask & (1 << i) != 0).collect();
            if self.is_subgroup(&subset, id) {
                result.push(subset);
            }
        }
        result
    }

    fn is_subgroup(&self, subset: &[usize], id: usize) -> bool {
        let set: HashSet<usize> = subset.iter().copied().collect();
        if !set.contains(&id) { return false; }
        // Closed under multiplication
        for &a in subset {
            for &b in subset {
                match self.mul(a, b) {
                    Some(c) if set.contains(&c) => {}
                    _ => return false,
                }
            }
        }
        // Closed under inverse (follows from finiteness + closure, but check)
        true
    }

    fn center(&self) -> Vec<usize> {
        let n = self.elems.len();
        (0..n).filter(|&a| {
            (0..n).all(|b| self.mul(a, b) == self.mul(b, a))
        }).collect()
    }

    fn is_abelian(&self) -> bool {
        let n = self.elems.len();
        for a in 0..n {
            for b in 0..n {
                if self.mul(a, b) != self.mul(b, a) { return false; }
            }
        }
        true
    }

    fn left_coset(&self, subset: &[usize], a: usize) -> Option<Vec<usize>> {
        let mut coset = vec![];
        for &h in subset {
            coset.push(self.mul(a, h)?);
        }
        coset.sort_unstable();
        Some(coset)
    }
}

// ── Preset groups ────────────────────────────────────────────────────────────

fn make_zn(n: usize) -> Group {
    let elems: Vec<String> = (0..n).map(|i| i.to_string()).collect();
    let mut g = Group::new(elems);
    for a in 0..n {
        for b in 0..n {
            g.table[a][b] = Some((a + b) % n);
        }
    }
    g
}

fn make_s3() -> Group {
    // S3: permutations of {1,2,3}  e, (12), (13), (23), (123), (132)
    let elems = vec!["e","r","r2","s","sr","sr2"].iter().map(|s| s.to_string()).collect();
    // D3 presentation: r³=e, s²=e, sr=r²s
    // Elements: e=0,r=1,r2=2,s=3,sr=4,sr2=5
    // Multiplication table for D3
    let raw = [
        [0,1,2,3,4,5],
        [1,2,0,4,5,3],
        [2,0,1,5,3,4],
        [3,5,4,0,2,1],
        [4,3,5,1,0,2],
        [5,4,3,2,1,0],
    ];
    let mut g = Group::new(elems);
    for a in 0..6 {
        for b in 0..6 {
            g.table[a][b] = Some(raw[a][b]);
        }
    }
    g
}

fn make_v4() -> Group {
    let elems = vec!["e","a","b","c"].iter().map(|s| s.to_string()).collect();
    let raw = [[0,1,2,3],[1,0,3,2],[2,3,0,1],[3,2,1,0]];
    let mut g = Group::new(elems);
    for a in 0..4 {
        for b in 0..4 { g.table[a][b] = Some(raw[a][b]); }
    }
    g
}

// ── Sandbox ──────────────────────────────────────────────────────────────────

struct Sandbox {
    group: Option<Group>,
    named_subgroups: HashMap<String, Vec<usize>>,
}

impl Sandbox {
    fn new() -> Self { Sandbox { group: None, named_subgroups: HashMap::new() } }

    fn require_group(&self) -> Option<&Group> {
        match &self.group {
            Some(g) => Some(g),
            None => { println!("{RE}No group loaded. Use: new zn 6  |  new s3  |  new v4  |  new <a> <b> ...{R}"); None }
        }
    }

    fn handle(&mut self, tokens: &[&str]) {
        match tokens[0] {
            "new" => self.cmd_new(tokens),
            "set" => self.cmd_set(tokens),
            "mul" | "*" => self.cmd_mul(tokens),
            "check" => self.cmd_check(),
            "table" | "show" => self.cmd_table(),
            "order" => self.cmd_order(tokens),
            "elements" | "elems" => self.cmd_elements(),
            "subgroups" => self.cmd_subgroups(),
            "center" | "Z" => self.cmd_center(),
            "abelian" => self.cmd_abelian(),
            "coset" => self.cmd_coset(tokens),
            "name" => self.cmd_name(tokens),
            "conjugate" => self.cmd_conjugate(tokens),
            "commutator" => self.cmd_commutator(tokens),
            "help" | "h" | "?" => self.help(),
            _ => println!("{RE}Unknown command. Type 'help' for commands.{R}"),
        }
    }

    fn cmd_new(&mut self, tokens: &[&str]) {
        if tokens.len() < 2 { println!("{RE}Usage: new <zn N | s3 | v4 | elem1 elem2 ...>{R}"); return; }
        match tokens[1] {
            "zn" => {
                let n: usize = tokens.get(2).and_then(|s| s.parse().ok()).unwrap_or(4);
                self.group = Some(make_zn(n));
                self.named_subgroups.clear();
                println!("{G}Created ℤ/{n}ℤ with elements 0..{}{R}", n-1);
            }
            "s3" | "d3" => {
                self.group = Some(make_s3());
                self.named_subgroups.clear();
                println!("{G}Created S₃ (= D₃) with elements e,r,r²,s,sr,sr²{R}");
            }
            "v4" => {
                self.group = Some(make_v4());
                self.named_subgroups.clear();
                println!("{G}Created V₄ (Klein four-group) with elements e,a,b,c{R}");
            }
            _ => {
                // Custom: elements are the remaining tokens
                let elems: Vec<String> = tokens[1..].iter().map(|s| s.to_string()).collect();
                println!("{G}Created group with {} elements: {:?}{R}", elems.len(), elems);
                println!("{D}Use 'set a b = c' to define the multiplication table.{R}");
                self.group = Some(Group::new(elems));
                self.named_subgroups.clear();
            }
        }
    }

    fn cmd_set(&mut self, tokens: &[&str]) {
        // set a b = c
        if tokens.len() < 4 { println!("{RE}Usage: set <a> <b> = <c>  or  set <a> <b> <c>{R}"); return; }
        let g = match &mut self.group { Some(g) => g, None => { println!("{RE}No group loaded.{R}"); return; } };
        let a_name = tokens[1];
        let b_name = tokens[2];
        let c_name = if tokens[3] == "=" { tokens.get(4).copied().unwrap_or("") } else { tokens[3] };
        let ia = match g.idx(a_name) { Some(i) => i, None => { println!("{RE}Unknown element: {a_name}{R}"); return; } };
        let ib = match g.idx(b_name) { Some(i) => i, None => { println!("{RE}Unknown element: {b_name}{R}"); return; } };
        let ic = match g.idx(c_name) { Some(i) => i, None => { println!("{RE}Unknown element: {c_name}{R}"); return; } };
        g.table[ia][ib] = Some(ic);
        println!("{C}{a_name} · {b_name} = {c_name}{R}");
    }

    fn cmd_mul(&self, tokens: &[&str]) {
        let g = match self.require_group() { Some(g) => g, None => return };
        if tokens.len() < 3 { println!("{RE}Usage: mul <a> <b>{R}"); return; }
        match g.mul_name(tokens[1], tokens[2]) {
            Some(c) => println!("{C}{} · {} = {B}{}{R}", tokens[1], tokens[2], c),
            None => println!("{RE}Result not defined yet. Use 'set {} {} = <result>'{R}", tokens[1], tokens[2]),
        }
    }

    fn cmd_check(&self) {
        let g = match self.require_group() { Some(g) => g, None => return };
        println!("{B}Checking group axioms:{R}");
        // Closed
        let closed = g.check_closed();
        println!("  {} Closure:      {}", tick(closed), if closed { "all products defined" } else { "some products undefined!" });
        if !closed { println!("{D}  (Use 'set a b = c' to fill in the table){R}"); return; }
        // Associativity
        match g.check_assoc() {
            None => println!("  {} Associativity: {G}holds for all triples{R}", tick(true)),
            Some((a, b, c)) => println!("  {} Associativity: {RE}FAILS at ({a},{b},{c}): (a·b)·c ≠ a·(b·c){R}", tick(false)),
        }
        // Identity
        match g.find_identity() {
            Some(id) => println!("  {} Identity:     {G}found e = '{}'{R}", tick(true), g.elems[id]),
            None => println!("  {} Identity:     {RE}no identity element found!{R}", tick(false)),
        }
        // Inverses
        if let Some(id) = g.find_identity() {
            match g.check_inverses(id) {
                None => println!("  {} Inverses:     {G}every element has an inverse{R}", tick(true)),
                Some((a, _)) => println!("  {} Inverses:     {RE}'{a}' has no inverse!{R}", tick(false)),
            }
        }
        // Abelian check (bonus)
        if closed {
            if g.is_abelian() { println!("  {D}(Group is abelian — all elements commute){R}"); }
        }
    }

    fn cmd_table(&self) {
        let g = match self.require_group() { Some(g) => g, None => return };
        let n = g.elems.len();
        let w = g.elems.iter().map(|e| e.len()).max().unwrap_or(1);
        print!("{B}{:>w$}{R} │", "·");
        for e in &g.elems { print!(" {C}{e:>w$}{R}"); }
        println!();
        println!("{D}{}{R}", "─".repeat((w + 1) * (n + 1) + n));
        for (i, a) in g.elems.iter().enumerate() {
            print!("{B}{a:>w$}{R} │");
            for j in 0..n {
                match g.mul(i, j) {
                    Some(k) => print!(" {G}{:>w$}{R}", g.elems[k]),
                    None    => print!(" {D}{:>w$}{R}", "?"),
                }
            }
            println!();
        }
    }

    fn cmd_order(&self, tokens: &[&str]) {
        let g = match self.require_group() { Some(g) => g, None => return };
        if tokens.len() < 2 {
            // Print order of every element
            println!("{B}Element orders:{R}");
            for (i, e) in g.elems.iter().enumerate() {
                match g.element_order(i) {
                    Some(k) => println!("  {C}{e}{R}: order {Y}{k}{R}"),
                    None    => println!("  {C}{e}{R}: {D}(table incomplete){R}"),
                }
            }
            return;
        }
        let a = tokens[1];
        let ia = match g.idx(a) { Some(i) => i, None => { println!("{RE}Unknown: {a}{R}"); return; } };
        match g.element_order(ia) {
            Some(k) => println!("ord({C}{a}{R}) = {Y}{k}{R}"),
            None => println!("{RE}Table incomplete or element has infinite order{R}"),
        }
    }

    fn cmd_elements(&self) {
        let g = match self.require_group() { Some(g) => g, None => return };
        println!("{B}Elements:{R} {}", g.elems.iter().enumerate()
            .map(|(i,e)| format!("{C}{e}{R}[{D}{i}{R}]")).collect::<Vec<_>>().join("  "));
        println!("{B}Order of group:|G|{R} = {Y}{}{R}", g.elems.len());
    }

    fn cmd_subgroups(&self) {
        let g = match self.require_group() { Some(g) => g, None => return };
        if !g.check_closed() { println!("{RE}Complete the multiplication table first ('check' shows what's missing){R}"); return; }
        if g.elems.len() > 12 { println!("{RE}Group too large for exhaustive subgroup search (n>12){R}"); return; }
        let subs = g.subgroups();
        println!("{B}Subgroups of G (order {}):{R} {} found", g.elems.len(), subs.len());
        for sub in &subs {
            let names: Vec<&str> = sub.iter().map(|&i| g.elems[i].as_str()).collect();
            println!("  {C}{{{}}}{R}  (order {})", names.join(", "), sub.len());
        }
        println!("{D}(Lagrange: each order divides |G| = {}){R}", g.elems.len());
    }

    fn cmd_center(&self) {
        let g = match self.require_group() { Some(g) => g, None => return };
        let z = g.center();
        let names: Vec<&str> = z.iter().map(|&i| g.elems[i].as_str()).collect();
        println!("{B}Center Z(G):{R} {{{C}{}{R}}}", names.join(", "));
        if z.len() == g.elems.len() { println!("  {G}Z(G) = G — group is abelian{R}"); }
    }

    fn cmd_abelian(&self) {
        let g = match self.require_group() { Some(g) => g, None => return };
        if g.is_abelian() { println!("{G}Abelian: all elements commute.{R}"); }
        else {
            println!("{Y}Not abelian. Non-commuting pairs:{R}");
            let n = g.elems.len();
            let mut shown = 0;
            for a in 0..n { for b in 0..n {
                if g.mul(a,b) != g.mul(b,a) && a < b && shown < 5 {
                    let ab = g.mul(a,b).map(|i| g.elems[i].as_str()).unwrap_or("?");
                    let ba = g.mul(b,a).map(|i| g.elems[i].as_str()).unwrap_or("?");
                    println!("  {C}{} · {} = {}{R}  but  {C}{} · {} = {}{R}",
                        g.elems[a], g.elems[b], ab, g.elems[b], g.elems[a], ba);
                    shown += 1;
                }
            }}
        }
    }

    fn cmd_coset(&self, tokens: &[&str]) {
        // coset <subgroup-name-or-elements> <element>
        let g = match self.require_group() { Some(g) => g, None => return };
        if tokens.len() < 3 { println!("{RE}Usage: coset <H-name> <a>   or   name a subgroup first{R}"); return; }
        let (h_name, a_name) = (tokens[1], tokens[tokens.len()-1]);
        let h = match self.named_subgroups.get(h_name) {
            Some(h) => h.clone(),
            None => { println!("{RE}Unknown subgroup '{h_name}'. Name one with: name <H> <e1> <e2> ...{R}"); return; }
        };
        let ia = match g.idx(a_name) { Some(i) => i, None => { println!("{RE}Unknown element: {a_name}{R}"); return; } };
        match g.left_coset(&h, ia) {
            Some(coset) => {
                let names: Vec<&str> = coset.iter().map(|&i| g.elems[i].as_str()).collect();
                println!("{C}{a_name}{R}·{C}{h_name}{R} = {{{G}{}{R}}}", names.join(", "));
            }
            None => println!("{RE}Table incomplete{R}"),
        }
    }

    fn cmd_name(&mut self, tokens: &[&str]) {
        // name <H-name> <e1> <e2> ...
        let g = match self.require_group() { Some(g) => g, None => return };
        if tokens.len() < 3 { println!("{RE}Usage: name <label> <elem1> <elem2> ...{R}"); return; }
        let label = tokens[1].to_string();
        let mut indices = vec![];
        for &e in &tokens[2..] {
            match g.idx(e) {
                Some(i) => indices.push(i),
                None => { println!("{RE}Unknown element: {e}{R}"); return; }
            }
        }
        println!("{G}Named subgroup '{label}' = {{{}}}{R}", tokens[2..].join(", "));
        self.named_subgroups.insert(label, indices);
    }

    fn cmd_conjugate(&self, tokens: &[&str]) {
        let g = match self.require_group() { Some(g) => g, None => return };
        if tokens.len() < 3 { println!("{RE}Usage: conjugate <g> <a>  — computes a·g·a⁻¹{R}"); return; }
        let (gn, an) = (tokens[1], tokens[2]);
        let ig = match g.idx(gn) { Some(i) => i, None => { println!("{RE}Unknown: {gn}{R}"); return; } };
        let ia = match g.idx(an) { Some(i) => i, None => { println!("{RE}Unknown: {an}{R}"); return; } };
        let id = match g.find_identity() { Some(i) => i, None => { println!("{RE}No identity found{R}"); return; } };
        // find a⁻¹
        let n = g.elems.len();
        let a_inv = (0..n).find(|&b| g.mul(ia,b) == Some(id));
        match a_inv {
            None => println!("{RE}{an} has no inverse (table incomplete?){R}"),
            Some(ai) => {
                let ag = g.mul(ia, ig).and_then(|t| g.mul(t, ai));
                match ag {
                    Some(r) => println!("{C}{an}·{gn}·{an}⁻¹ = {B}{}{R}", g.elems[r]),
                    None => println!("{RE}Table incomplete{R}"),
                }
            }
        }
    }

    fn cmd_commutator(&self, tokens: &[&str]) {
        let g = match self.require_group() { Some(g) => g, None => return };
        if tokens.len() < 3 { println!("{RE}Usage: commutator <a> <b>  — computes [a,b] = a⁻¹·b⁻¹·a·b{R}"); return; }
        let (an, bn) = (tokens[1], tokens[2]);
        let ia = match g.idx(an) { Some(i) => i, None => { println!("{RE}Unknown: {an}{R}"); return; } };
        let ib = match g.idx(bn) { Some(i) => i, None => { println!("{RE}Unknown: {bn}{R}"); return; } };
        let id = match g.find_identity() { Some(i) => i, None => { println!("{RE}No identity{R}"); return; } };
        let n = g.elems.len();
        let a_inv = (0..n).find(|&x| g.mul(ia,x) == Some(id));
        let b_inv = (0..n).find(|&x| g.mul(ib,x) == Some(id));
        match (a_inv, b_inv) {
            (Some(ai), Some(bi)) => {
                let r = g.mul(ai, bi).and_then(|t| g.mul(t, ia)).and_then(|t| g.mul(t, ib));
                match r {
                    Some(k) => {
                        let is_id = k == id;
                        println!("[{C}{an}{R},{C}{bn}{R}] = {B}{}{R}{}",
                            g.elems[k], if is_id { format!("  {D}(a and b commute){R}") } else { String::new() });
                    }
                    None => println!("{RE}Table incomplete{R}"),
                }
            }
            _ => println!("{RE}Inverses not found (table incomplete?){R}"),
        }
    }

    fn help(&self) {
        println!("{B}Group Laboratory{R} — build and explore groups\n");
        println!("{B}Load a group:{R}");
        println!("  {C}new zn 6{R}         create ℤ/6ℤ (integers mod 6)");
        println!("  {C}new s3{R}           create S₃ (symmetric group on 3 elements)");
        println!("  {C}new v4{R}           create V₄ (Klein four-group)");
        println!("  {C}new e a b c{R}      custom group with given elements");
        println!();
        println!("{B}Build the multiplication table:{R}");
        println!("  {C}set a b c{R}        set a·b = c");
        println!("  {C}mul a b{R}          compute a·b");
        println!();
        println!("{B}Check and explore:{R}");
        println!("  {C}check{R}            verify all group axioms");
        println!("  {C}table{R}            show the Cayley table");
        println!("  {C}elements{R}         list elements and group order");
        println!("  {C}order <a>{R}        order of element a");
        println!("  {C}order{R}            orders of all elements");
        println!("  {C}abelian{R}          check if group is abelian");
        println!("  {C}center{R}           compute the center Z(G)");
        println!("  {C}subgroups{R}        enumerate all subgroups (for |G|≤12)");
        println!("  {C}conjugate g a{R}    compute a·g·a⁻¹");
        println!("  {C}commutator a b{R}   compute [a,b] = a⁻¹b⁻¹ab");
        println!();
        println!("{B}Subgroup cosets:{R}");
        println!("  {C}name H e a b{R}     name a subset H = {{e,a,b}}");
        println!("  {C}coset H a{R}        compute left coset a·H");
        println!();
        println!("{D}Try: new zn 6 → check → table → subgroups{R}");
        println!("{D}Or:  new e a b → set e e e → set e a a → ... → check{R}");
    }
}

fn tick(b: bool) -> &'static str { if b { "\x1b[32m✓\x1b[0m" } else { "\x1b[31m✗\x1b[0m" } }

fn main() {
    println!("{B}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{R}");
    println!("{B}  Group Laboratory{R}");
    println!("{D}  Build groups, check axioms, discover structure{R}");
    println!("{B}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{R}");
    println!("{D}Type 'help' for commands. Try: new zn 6{R}\n");

    let mut sb = Sandbox::new();
    loop {
        let line = read_line("group>");
        if line.is_empty() { continue; }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens[0] {
            "quit" | "q" | "exit" => { println!("{D}Goodbye.{R}"); break; }
            _ => sb.handle(&tokens),
        }
    }
}
