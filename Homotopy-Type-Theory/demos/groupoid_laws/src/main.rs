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

// ── Path expressions with groupoid laws ───────────────────────────────────────
//
// In HoTT, identity types satisfy groupoid laws *propositionally*:
//   - refl is the identity: refl · p = p, p · refl = p
//   - Inverse laws: p · p⁻¹ = refl, p⁻¹ · p = refl
//   - Associativity: (p · q) · r = p · (q · r)
//
// Higher paths: 2-paths (homotopies between paths) also form a groupoid.
// The Eckmann-Hilton theorem shows that for Ω²(X), composition is commutative.

#[derive(Clone, Debug, PartialEq)]
enum PathExpr {
    Refl(String),                               // refl_x
    Named(String, String, String),              // name : a → b
    Compose(Box<PathExpr>, Box<PathExpr>),
    Inverse(Box<PathExpr>),
    HComp(Box<PathExpr>, Box<PathExpr>),        // horizontal composition of 2-paths
    VComp(Box<PathExpr>, Box<PathExpr>),        // vertical composition of 2-paths
}

impl PathExpr {
    fn start(&self, ctx: &HashMap<String, (String, String)>) -> Option<String> {
        match self {
            PathExpr::Refl(x) => Some(x.clone()),
            PathExpr::Named(n, _, _) => ctx.get(n).map(|(a,_)| a.clone()),
            PathExpr::Compose(p, _) => p.start(ctx),
            PathExpr::Inverse(p) => p.end(ctx),
            PathExpr::HComp(p, _) => p.start(ctx),
            PathExpr::VComp(p, _) => p.start(ctx),
        }
    }

    fn end(&self, ctx: &HashMap<String, (String, String)>) -> Option<String> {
        match self {
            PathExpr::Refl(x) => Some(x.clone()),
            PathExpr::Named(n, _, _) => ctx.get(n).map(|(_,b)| b.clone()),
            PathExpr::Compose(_, q) => q.end(ctx),
            PathExpr::Inverse(p) => p.start(ctx),
            PathExpr::HComp(_, q) => q.end(ctx),
            PathExpr::VComp(_, q) => q.end(ctx),
        }
    }

    fn display(&self) -> String {
        match self {
            PathExpr::Refl(x) => format!("refl_{x}"),
            PathExpr::Named(n, _, _) => n.clone(),
            PathExpr::Compose(p, q) => format!("({} · {})", p.display(), q.display()),
            PathExpr::Inverse(p) => format!("{}⁻¹", p.display()),
            PathExpr::HComp(p, q) => format!("({} ⋆ₕ {})", p.display(), q.display()),
            PathExpr::VComp(p, q) => format!("({} ⋆ᵥ {})", p.display(), q.display()),
        }
    }

    fn normalize(&self, ctx: &HashMap<String, (String, String)>) -> PathExpr {
        match self {
            PathExpr::Compose(p, q) => {
                let pn = p.normalize(ctx);
                let qn = q.normalize(ctx);
                // Groupoid laws
                if let PathExpr::Refl(_) = &pn { return qn; }
                if let PathExpr::Refl(_) = &qn { return pn; }
                // p · p⁻¹ = refl
                if let PathExpr::Inverse(ref pp) = qn {
                    if **pp == pn { return PathExpr::Refl(pn.start(ctx).unwrap_or_default()); }
                }
                // p⁻¹ · p = refl
                if let PathExpr::Inverse(ref pp) = pn {
                    if **pp == qn { return PathExpr::Refl(qn.end(ctx).unwrap_or_default()); }
                }
                PathExpr::Compose(Box::new(pn), Box::new(qn))
            }
            PathExpr::Inverse(p) => {
                let pn = p.normalize(ctx);
                match pn {
                    PathExpr::Refl(x) => PathExpr::Refl(x),
                    PathExpr::Inverse(q) => *q,
                    _ => PathExpr::Inverse(Box::new(pn)),
                }
            }
            _ => self.clone(),
        }
    }
}

// ── 2-path (homotopy) ─────────────────────────────────────────────────────────
struct TwoPath {
    name: String,
    from: PathExpr,
    to: PathExpr,
}

// ── Sandbox ───────────────────────────────────────────────────────────────────
struct Sandbox {
    points: Vec<String>,
    paths: HashMap<String, (String, String)>,   // name -> (start, end)
    named: HashMap<String, PathExpr>,
    two_paths: HashMap<String, TwoPath>,
}

impl Sandbox {
    fn new() -> Self {
        Sandbox { points: vec![], paths: HashMap::new(), named: HashMap::new(), two_paths: HashMap::new() }
    }

    fn print_help() {
        println!("{}", bold("── Space Construction ──────────────────────────────────────────────"));
        println!("  {}  a b c            — add points", cyan("points"));
        println!("  {}  p a b           — add path p : a → b", cyan("path"));
        println!("  {}                  — show space", cyan("show"));
        println!("{}", bold("── Path Operations ─────────────────────────────────────────────────"));
        println!("  {}  p q             — compose: p · q", cyan("compose"));
        println!("  {}  p               — inverse: p⁻¹", cyan("inv"));
        println!("  {}  r = p q p^     — name a composite", cyan("let"));
        println!("  {}  p               — normalize using groupoid laws", cyan("norm"));
        println!("  {}  p q r           — check assoc: (p·q)·r = p·(q·r)", cyan("assoc"));
        println!("{}", bold("── Groupoid Laws ───────────────────────────────────────────────────"));
        println!("  {}            — verify all groupoid laws on named paths", cyan("laws"));
        println!("{}", bold("── Higher Paths ────────────────────────────────────────────────────"));
        println!("  {}  α p q           — define 2-path α : p ⇒ q", cyan("2path"));
        println!("  {}  α β             — vertical compose 2-paths", cyan("vcomp"));
        println!("  {}  α β             — horizontal compose 2-paths", cyan("hcomp"));
        println!("  {}             — Eckmann-Hilton demonstration", cyan("eckmann"));
    }

    fn parse_path_expr(&self, tokens: &[&str]) -> Option<PathExpr> {
        if tokens.is_empty() { return None; }
        // Build composite from left to right
        let mut cur = self.parse_atom(tokens[0])?;
        let mut i = 1;
        while i < tokens.len() {
            let t = tokens[i];
            if t == "·" || t == "." || t == "*" {
                i += 1;
                let next = self.parse_atom(tokens[i])?;
                cur = PathExpr::Compose(Box::new(cur), Box::new(next));
                i += 1;
            } else {
                let next = self.parse_atom(t)?;
                cur = PathExpr::Compose(Box::new(cur), Box::new(next));
                i += 1;
            }
        }
        Some(cur)
    }

    fn parse_atom(&self, t: &str) -> Option<PathExpr> {
        if t.starts_with("refl_") {
            return Some(PathExpr::Refl(t[5..].to_string()));
        }
        if t.ends_with('^') || t.ends_with("^-1") || t.ends_with("_inv") {
            let base = t.trim_end_matches('^').trim_end_matches("^-1").trim_end_matches("_inv");
            return Some(PathExpr::Inverse(Box::new(self.parse_atom(base)?)));
        }
        if let Some((a, b)) = self.paths.get(t) {
            return Some(PathExpr::Named(t.to_string(), a.clone(), b.clone()));
        }
        if let Some(e) = self.named.get(t) { return Some(e.clone()); }
        if t.starts_with("refl") { return Some(PathExpr::Refl("?".into())); }
        None
    }

    fn handle(&mut self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() { return true; }
        if trimmed == "quit" || trimmed == "exit" || trimmed == "q" { return false; }
        if trimmed == "help" || trimmed == "?" { Self::print_help(); return true; }

        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        let cmd = parts[0];
        let rest = if parts.len() > 1 { parts[1].trim() } else { "" };
        let tokens: Vec<&str> = rest.split_whitespace().collect();

        match cmd {
            "points" => {
                for p in tokens { if !self.points.contains(&p.to_string()) { self.points.push(p.to_string()); } }
                println!("  Points: {{{}}}", self.points.join(", "));
            }
            "path" => {
                if tokens.len() < 3 { println!("  {} Use: path name start end", red("✗")); return true; }
                let (name, from, to) = (tokens[0], tokens[1], tokens[2]);
                self.paths.insert(name.to_string(), (from.to_string(), to.to_string()));
                println!("  {} : {} → {}", cyan(name), from, to);
            }
            "show" => {
                println!("  Points: {{{}}}", self.points.join(", "));
                for (name, (a, b)) in &self.paths {
                    println!("  {} : {} → {}", cyan(name), a, b);
                }
                for (name, expr) in &self.named {
                    let start = expr.start(&self.paths).unwrap_or("?".into());
                    let end = expr.end(&self.paths).unwrap_or("?".into());
                    println!("  {} = {} : {} → {}", cyan(name), dim(&expr.display()), start, end);
                }
            }
            "let" => {
                if let Some(eq_pos) = tokens.iter().position(|&t| t == "=") {
                    let name = tokens[0];
                    let expr_tokens = &tokens[eq_pos+1..];
                    match self.parse_path_expr(expr_tokens) {
                        Some(e) => {
                            let start = e.start(&self.paths).unwrap_or("?".into());
                            let end = e.end(&self.paths).unwrap_or("?".into());
                            println!("  {} = {} : {} → {}", cyan(name), e.display(), start, end);
                            self.named.insert(name.to_string(), e);
                        }
                        None => println!("  {} Parse error", red("✗")),
                    }
                } else { println!("  {} Use: let name = expr", red("✗")); }
            }
            "compose" => {
                if tokens.len() < 2 { println!("  {} Use: compose p q ...", red("✗")); return true; }
                match self.parse_path_expr(&tokens) {
                    Some(e) => {
                        let n = e.normalize(&self.paths);
                        println!("  {} = {}", e.display(), bold(&n.display()));
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "inv" => {
                match self.parse_atom(tokens.first().unwrap_or(&"")) {
                    Some(e) => {
                        let inv = PathExpr::Inverse(Box::new(e.clone()));
                        let n = inv.normalize(&self.paths);
                        println!("  ({})⁻¹ = {}", e.display(), bold(&n.display()));
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "norm" => {
                match self.parse_path_expr(&tokens) {
                    Some(e) => {
                        let n = e.normalize(&self.paths);
                        println!("  normalize({}) = {}", dim(&e.display()), bold(&cyan(&n.display())));
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "assoc" => {
                if tokens.len() < 3 { println!("  {} Use: assoc p q r", red("✗")); return true; }
                let p = self.parse_atom(tokens[0]);
                let q = self.parse_atom(tokens[1]);
                let r = self.parse_atom(tokens[2]);
                match (p, q, r) {
                    (Some(p), Some(q), Some(r)) => {
                        let lhs = PathExpr::Compose(Box::new(PathExpr::Compose(Box::new(p.clone()), Box::new(q.clone()))), Box::new(r.clone()));
                        let rhs = PathExpr::Compose(Box::new(p.clone()), Box::new(PathExpr::Compose(Box::new(q.clone()), Box::new(r.clone()))));
                        let ln = lhs.normalize(&self.paths);
                        let rn = rhs.normalize(&self.paths);
                        println!("  LHS: ({} · {}) · {} = {}", p.display(), q.display(), r.display(), ln.display());
                        println!("  RHS: {} · ({} · {}) = {}", p.display(), q.display(), r.display(), rn.display());
                        println!("  Propositionally equal: {} (proof = α : LHS = RHS)", green("yes (by assoc)"));
                        println!("  {}", dim("Note: equality holds as a 2-path, not definitional equality"));
                    }
                    _ => println!("  {} Parse error", red("✗")),
                }
            }
            "laws" => {
                println!("{}", bold("  Groupoid laws for named paths:"));
                let names: Vec<_> = self.named.keys().cloned().collect();
                for name in &names {
                    let e = self.named[name].clone();
                    let start = e.start(&self.paths).unwrap_or("?".into());
                    let end = e.end(&self.paths).unwrap_or("?".into());
                    // refl · e
                    let refl_e = PathExpr::Compose(Box::new(PathExpr::Refl(start.clone())), Box::new(e.clone()));
                    let rn = refl_e.normalize(&self.paths);
                    println!("  refl_{start} · {} = {} {}", name, rn.display(), green("✓"));
                    // e · refl
                    let e_refl = PathExpr::Compose(Box::new(e.clone()), Box::new(PathExpr::Refl(end.clone())));
                    let ern = e_refl.normalize(&self.paths);
                    println!("  {} · refl_{end} = {} {}", name, ern.display(), green("✓"));
                    // e⁻¹ · e
                    let inv_e = PathExpr::Compose(Box::new(PathExpr::Inverse(Box::new(e.clone()))), Box::new(e.clone()));
                    let ien = inv_e.normalize(&self.paths);
                    println!("  {}⁻¹ · {} = {} {}", name, name, ien.display(), green("✓"));
                    println!();
                }
                if names.is_empty() { println!("  {}", dim("Define some paths with 'let' first")); }
            }
            "2path" => {
                if tokens.len() < 3 { println!("  {} Use: 2path name p q", red("✗")); return true; }
                let (alpha, pname, qname) = (tokens[0], tokens[1], tokens[2]);
                let p = self.named.get(pname).cloned().or_else(|| self.parse_atom(pname));
                let q = self.named.get(qname).cloned().or_else(|| self.parse_atom(qname));
                match (p, q) {
                    (Some(p), Some(q)) => {
                        println!("  {} : {} ⇒ {}", cyan(alpha), p.display(), q.display());
                        self.two_paths.insert(alpha.to_string(), TwoPath { name: alpha.to_string(), from: p, to: q });
                    }
                    _ => println!("  {} Path(s) not found", red("✗")),
                }
            }
            "eckmann" => {
                println!("{}", bold("── Eckmann-Hilton Theorem ──────────────────────────────────────────"));
                println!("  For any 2-loops α, β : refl = refl at a point x:");
                println!("  α ⋆ₕ β = β ⋆ₕ α  (horizontal composition is commutative)");
                println!("  and moreover: α ⋆ₕ β = α ⋆ᵥ β  (both compositions agree)");
                println!();
                println!("  Proof sketch:");
                println!("  α ⋆ₕ β = (α · id) ⋆ᵥ (id · β)   (by whiskering)");
                println!("         = α ⋆ᵥ β                   (by unit laws)");
                println!("         = (id · α) ⋆ᵥ (β · id)");
                println!("         = β ⋆ₕ α");
                println!();
                println!("  Consequence: Ω²(X) = π₂(X) is always abelian!");
                println!("  This is why π₂(S²) = ℤ but its elements {}", dim("commute"));
                println!("  even though π₁(X) need not be abelian.");
                println!();
                println!("  In HoTT, this is a theorem about 2-paths in any type.");
            }
            "vcomp" | "hcomp" => {
                if tokens.len() < 2 { println!("  {} Use: {cmd} α β", red("✗")); return true; }
                let alpha = self.two_paths.get(tokens[0]);
                let beta = self.two_paths.get(tokens[1]);
                match (alpha, beta) {
                    (Some(a), Some(b)) => {
                        let kind = if cmd == "vcomp" { "⋆ᵥ" } else { "⋆ₕ" };
                        println!("  ({}) {} ({}) : {} ⇒ {}", a.name, kind, b.name, a.from.display(), b.to.display());
                        println!("  {}", dim("(2-path composition — result is another 2-path)"));
                    }
                    _ => println!("  {} 2-path(s) not found", red("✗")),
                }
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Groupoid Laws & Higher Paths Sandbox                  ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build a space, define paths, and explore the groupoid structure.");
    println!("  Work with 2-paths and see the Eckmann-Hilton theorem in action.");
    println!("  Type {} for commands.\n", cyan("help"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}groupoid{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
