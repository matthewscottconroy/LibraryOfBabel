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

// ── Simplicial complex (finite presentation) ──────────────────────────────────
//
// A simplicial set is determined by:
//   - Xₙ = set of n-simplices (0=vertices, 1=edges, 2=faces, ...)
//   - Face maps: dᵢ : Xₙ → Xₙ₋₁
//   - Degeneracy maps: sᵢ : Xₙ → Xₙ₊₁
//
// We represent a finite simplicial complex by listing non-degenerate simplices.

#[derive(Clone)]
struct SimplicialComplex {
    name: String,
    // simplices[n] = list of n-simplices (each is a sorted set of vertex names)
    simplices: Vec<Vec<Vec<String>>>,
}

impl SimplicialComplex {
    fn new(name: &str) -> Self {
        SimplicialComplex {
            name: name.to_string(),
            simplices: vec![vec![]], // start with dim 0
        }
    }

    fn ensure_dim(&mut self, n: usize) {
        while self.simplices.len() <= n { self.simplices.push(vec![]); }
    }

    fn add_simplex(&mut self, verts: Vec<String>) -> Result<String, String> {
        if verts.is_empty() { return Err("Need at least one vertex".into()); }
        let n = verts.len() - 1; // dimension

        // Check all faces are present (closure condition)
        if n >= 1 {
            for i in 0..verts.len() {
                let face: Vec<String> = verts.iter().enumerate()
                    .filter(|(j, _)| *j != i).map(|(_, v)| v.clone()).collect();
                self.ensure_dim(n - 1);
                if !self.simplices[n-1].contains(&face) {
                    return Err(format!("Face {:?} must be added first", face));
                }
            }
        }

        self.ensure_dim(n);
        let key = verts.clone();
        if self.simplices[n].contains(&key) {
            return Ok(format!("{}-simplex {:?} already exists", n, key));
        }
        self.simplices[n].push(key.clone());
        Ok(format!("Added {}-simplex {:?}", n, key))
    }

    fn add_closure(&mut self, verts: Vec<String>) {
        // Adds all sub-faces automatically (fill_horn-like)
        let n = verts.len() - 1;
        // Add all sub-simplices in order of dimension
        for k in 0..=n {
            // All k+1 subsets of verts
            let subsets = subsets_of_size(&verts, k + 1);
            self.ensure_dim(k);
            for s in subsets {
                if !self.simplices[k].contains(&s) {
                    self.simplices[k].push(s);
                }
            }
        }
    }

    fn faces_of(&self, verts: &[String]) -> Vec<Vec<String>> {
        (0..verts.len()).map(|i| {
            verts.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, v)| v.clone()).collect()
        }).collect()
    }

    fn boundary(&self, dim: usize) -> Vec<(Vec<String>, Vec<Vec<String>>)> {
        if dim == 0 || self.simplices.len() <= dim { return vec![]; }
        self.simplices[dim].iter()
            .map(|s| (s.clone(), self.faces_of(s)))
            .collect()
    }

    fn euler_characteristic(&self) -> i64 {
        self.simplices.iter().enumerate()
            .map(|(n, sn)| if n % 2 == 0 { sn.len() as i64 } else { -(sn.len() as i64) })
            .sum()
    }

    fn betti_naive(&self) -> (i64, i64) {
        // Very rough Betti numbers via Euler for common spaces
        // β₀ = connected components (we count via vertices and edges)
        let v = self.simplices.first().map(|s| s.len()).unwrap_or(0);
        let e = self.simplices.get(1).map(|s| s.len()).unwrap_or(0);
        let f = self.simplices.get(2).map(|s| s.len()).unwrap_or(0);
        let chi = v as i64 - e as i64 + f as i64;
        (1, chi - 1 + 1) // β₀=1 (assume connected), β₁ = E - V + 1 for graphs
    }

    fn is_pure(&self, dim: usize) -> bool {
        self.simplices.get(dim).is_some() &&
        self.simplices.iter().enumerate().skip(dim+1).all(|(_, s)| s.is_empty())
    }

    fn link(&self, v: &str) -> SimplicialComplex {
        let mut result = SimplicialComplex::new(&format!("lk({})", v));
        for (_, sn) in self.simplices.iter().enumerate() {
            for simplex in sn {
                if simplex.contains(&v.to_string()) {
                    let link_simplex: Vec<String> = simplex.iter()
                        .filter(|u| u.as_str() != v).cloned().collect();
                    if !link_simplex.is_empty() {
                        result.ensure_dim(link_simplex.len() - 1);
                        if !result.simplices[link_simplex.len()-1].contains(&link_simplex) {
                            result.simplices[link_simplex.len()-1].push(link_simplex);
                        }
                    }
                }
            }
        }
        result
    }

    fn show(&self) {
        println!("  {} {}", bold("Complex:"), cyan(&self.name));
        for (n, sn) in self.simplices.iter().enumerate() {
            if !sn.is_empty() {
                let label = match n { 0 => "Vertices".into(), 1 => "Edges".into(),
                    2 => "Faces".into(), k => format!("{k}-simplices") };
                let list: Vec<String> = sn.iter().map(|s| format!("{{{}}}", s.join(","))).collect();
                println!("  {}: {}", label, list.join("  "));
            }
        }
        println!("  χ = {}", cyan(&self.euler_characteristic().to_string()));
    }
}

fn subsets_of_size(v: &[String], k: usize) -> Vec<Vec<String>> {
    if k == 0 { return vec![vec![]]; }
    if k > v.len() { return vec![]; }
    let mut result = vec![];
    for i in 0..=(v.len()-k) {
        let rest = subsets_of_size(&v[i+1..], k-1);
        for mut sub in rest {
            sub.insert(0, v[i].clone());
            result.push(sub);
        }
    }
    result
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    complexes: HashMap<String, SimplicialComplex>,
    current: String,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { complexes: HashMap::new(), current: String::new() };
        // Start with a point
        let mut pt = SimplicialComplex::new("point");
        pt.add_closure(vec!["v".into()]);
        sb.complexes.insert("point".into(), pt);
        sb.current = "point".into();
        sb
    }

    fn cur(&self) -> &SimplicialComplex { &self.complexes[&self.current] }
    fn cur_mut(&mut self) -> &mut SimplicialComplex { self.complexes.get_mut(&self.current).unwrap() }

    fn print_help() {
        println!("{}", bold("── Complex Construction ────────────────────────────────────────────"));
        println!("  {}  K                    — create/switch to complex K", cyan("new"));
        println!("  {}  a b c               — add simplex (faces auto-checked)", cyan("add"));
        println!("  {}  a b c               — add simplex + all sub-faces", cyan("fill"));
        println!("  {}                       — show current complex", cyan("show"));
        println!("  {}                       — list all complexes", cyan("list"));
        println!("{}", bold("── Analysis ────────────────────────────────────────────────────────"));
        println!("  {}  n                   — show boundary of n-simplices", cyan("boundary"));
        println!("  {}  v                   — compute link of vertex", cyan("link"));
        println!("  {}               — Euler characteristic", cyan("euler"));
        println!("{}", bold("── Preset spaces ───────────────────────────────────────────────────"));
        println!("  {}  interval|circle|sphere|torus|rp2|disk|cylinder", cyan("preset"));
    }

    fn build_preset(&mut self, name: &str) -> Result<(), String> {
        let mut sc = SimplicialComplex::new(name);
        match name {
            "interval" => {
                sc.add_closure(vec!["0".into(), "1".into()]);
            }
            "circle" => {
                // S¹ as three edges forming a triangle boundary (no face)
                sc.add_closure(vec!["a".into()]); sc.add_closure(vec!["b".into()]); sc.add_closure(vec!["c".into()]);
                sc.ensure_dim(1);
                sc.simplices[1].push(vec!["a".into(),"b".into()]);
                sc.simplices[1].push(vec!["b".into(),"c".into()]);
                sc.simplices[1].push(vec!["a".into(),"c".into()]);
            }
            "disk" => {
                sc.add_closure(vec!["a".into(),"b".into(),"c".into()]);
            }
            "sphere" => {
                // S² = boundary of tetrahedron (4 vertices, 6 edges, 4 faces)
                let verts = ["a","b","c","d"];
                for v in &verts { sc.add_closure(vec![v.to_string()]); }
                // All edges
                for i in 0..4 { for j in i+1..4 { sc.ensure_dim(1); sc.simplices[1].push(vec![verts[i].to_string(),verts[j].to_string()]); } }
                // 4 triangular faces (boundary of tetrahedron, no interior)
                let faces = [["a","b","c"],["a","b","d"],["a","c","d"],["b","c","d"]];
                sc.ensure_dim(2);
                for f in &faces { sc.simplices[2].push(f.iter().map(|s|s.to_string()).collect()); }
            }
            "cylinder" => {
                // S¹ × [0,1] — two triangles forming a square, plus the circles
                sc.add_closure(vec!["a0".into(),"b0".into(),"a1".into()]);
                sc.add_closure(vec!["b0".into(),"b1".into(),"a1".into()]);
            }
            "torus" => {
                println!("  {} Torus requires a quotient construction — showing triangulation outline.", yellow("ℹ"));
                println!("  The standard triangulation uses 7 vertices, 21 edges, 14 triangles.");
                println!("  χ(T²) = 7 - 21 + 14 = 0, matching genus 1.");
                println!("  Use the quotient demo to build T² = [0,1]²/~");
                return Ok(());
            }
            "rp2" => {
                println!("  {} RP² requires 6 vertices, 15 edges, 10 triangles (minimum triangulation).", yellow("ℹ"));
                println!("  χ(RP²) = 6 - 15 + 10 = 1  (non-orientable, β₁ = ℤ/2ℤ).");
                return Ok(());
            }
            _ => return Err(format!("Unknown preset: {name}")),
        }
        self.complexes.insert(name.to_string(), sc);
        self.current = name.to_string();
        println!("  Loaded preset: {}", cyan(name));
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
            "new" | "use" => {
                if rest.is_empty() { println!("  {} Name required", red("✗")); return true; }
                if !self.complexes.contains_key(rest) {
                    let sc = SimplicialComplex::new(rest);
                    self.complexes.insert(rest.to_string(), sc);
                    println!("  Created complex {}", cyan(rest));
                }
                self.current = rest.to_string();
                println!("  Using complex {}", cyan(rest));
            }
            "add" => {
                let verts: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
                if verts.is_empty() { println!("  {} Provide vertex names", red("✗")); return true; }
                match self.cur_mut().add_simplex(verts) {
                    Ok(msg) => println!("  {} {}", green("✓"), msg),
                    Err(e) => println!("  {} {}", red("✗"), e),
                }
            }
            "fill" => {
                let verts: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
                if verts.is_empty() { println!("  {} Provide vertex names", red("✗")); return true; }
                let n = verts.len() - 1;
                self.cur_mut().add_closure(verts.clone());
                println!("  {} Added {}-simplex and all faces: {:?}", green("✓"), n, verts);
            }
            "show" => self.cur().show(),
            "list" => {
                for (n, _) in &self.complexes {
                    let marker = if n == &self.current { green("*") } else { " ".to_string() };
                    println!("  {} {}", marker, cyan(n));
                }
            }
            "boundary" => {
                let dim: usize = rest.parse().unwrap_or(1);
                let bdy = self.cur().boundary(dim);
                if bdy.is_empty() { println!("  No {dim}-simplices"); return true; }
                println!("  {}", bold(&format!("Boundary map ∂{dim}:")));
                for (s, faces) in bdy {
                    let face_str: Vec<String> = faces.iter().map(|f| format!("{{{}}}", f.join(","))).collect();
                    println!("  ∂{{{}}}", s.join(","));
                    for (i, f) in face_str.iter().enumerate() {
                        let sign = if i % 2 == 0 { green("+") } else { red("-") };
                        println!("    {sign} {f}");
                    }
                }
            }
            "link" => {
                if rest.is_empty() { println!("  {} Provide vertex name", red("✗")); return true; }
                let lk = self.cur().link(rest);
                lk.show();
            }
            "euler" => {
                let chi = self.cur().euler_characteristic();
                println!("  χ = {}", cyan(&chi.to_string()));
                let guess = match chi {
                    2 => "sphere (contractible or S²)",
                    1 => "disk or RP²",
                    0 => "torus or Klein bottle or cylinder",
                    n if n < 0 => "higher genus surface",
                    _ => "?",
                };
                println!("  {}", dim(&format!("(typical for: {guess})")));
            }
            "preset" => {
                if rest.is_empty() {
                    println!("  Presets: interval, circle, disk, sphere, cylinder, torus, rp2");
                    return true;
                }
                if let Err(e) = self.build_preset(rest) { println!("  {} {}", red("✗"), e); }
                else { self.cur().show(); }
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Simplicial Sets — Complex Builder Sandbox             ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build simplicial complexes vertex by vertex, edge by edge.");
    println!("  Compute boundary maps, links, and Euler characteristics.");
    println!("  Type {} for commands, {} for preset spaces.\n", cyan("help"), cyan("preset"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}simp{} [{}] > ", CYAN, RESET, sb.current);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
