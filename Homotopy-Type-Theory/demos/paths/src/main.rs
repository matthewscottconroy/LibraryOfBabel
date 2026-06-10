// Path Algebra Sandbox — build path spaces, compose, invert, explore groupoid laws
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

const B: &str = "\x1b[1m";  const R: &str = "\x1b[0m";
const G: &str = "\x1b[32m"; const C: &str = "\x1b[36m";
const Y: &str = "\x1b[33m"; const D: &str = "\x1b[2m";
const RE: &str = "\x1b[31m"; const M: &str = "\x1b[35m";

fn read_line(prompt: &str) -> String {
    print!("{B}{prompt}{R} ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().lock().read_line(&mut s).unwrap();
    s.trim().to_string()
}

// ── Representation ───────────────────────────────────────────────────────────

// A path is an expression tree
#[derive(Clone, Debug)]
enum PathExpr {
    Named(String),               // a named path p
    Refl(String),                // refl_x for point x
    Compose(Box<PathExpr>, Box<PathExpr>),  // p · q
    Inverse(Box<PathExpr>),      // p⁻¹
}

impl PathExpr {
    fn display(&self) -> String {
        match self {
            PathExpr::Named(n) => format!("{C}{n}{R}"),
            PathExpr::Refl(x)  => format!("{D}refl_{R}{Y}{x}{R}"),
            PathExpr::Compose(p, q) => format!("({} · {})", p.display(), q.display()),
            PathExpr::Inverse(p) => format!("{}⁻¹", p.display()),
        }
    }

    // Reduce using groupoid laws:
    //   refl · p = p,  p · refl = p,  (p⁻¹)⁻¹ = p,  p · p⁻¹ = refl,  p⁻¹ · p = refl
    fn normalize(&self, ctx: &Sandbox) -> PathExpr {
        match self {
            PathExpr::Named(n) => PathExpr::Named(n.clone()),
            PathExpr::Refl(x)  => PathExpr::Refl(x.clone()),
            PathExpr::Inverse(inner) => {
                let inner_n = inner.normalize(ctx);
                match inner_n {
                    PathExpr::Inverse(p) => *p,  // (p⁻¹)⁻¹ = p
                    PathExpr::Refl(x)    => PathExpr::Refl(x),  // refl⁻¹ = refl
                    other => PathExpr::Inverse(Box::new(other)),
                }
            }
            PathExpr::Compose(p, q) => {
                let pn = p.normalize(ctx);
                let qn = q.normalize(ctx);
                // refl · q = q
                if let PathExpr::Refl(_) = &pn { return qn; }
                // p · refl = p
                if let PathExpr::Refl(_) = &qn { return pn; }
                // p · p⁻¹ = refl (if p is a named path)
                if let PathExpr::Inverse(ref pi) = qn {
                    if path_expr_eq(&pn, pi) {
                        // find endpoint to make refl_x
                        let x = ctx.path_end(&pn).unwrap_or("?".to_string());
                        return PathExpr::Refl(x);
                    }
                }
                // p⁻¹ · p = refl
                if let PathExpr::Inverse(ref pi) = pn {
                    if path_expr_eq(&qn, pi) {
                        let x = ctx.path_start(&qn).unwrap_or("?".to_string());
                        return PathExpr::Refl(x);
                    }
                }
                PathExpr::Compose(Box::new(pn), Box::new(qn))
            }
        }
    }

    fn endpoints(&self, ctx: &Sandbox) -> Option<(String, String)> {
        match self {
            PathExpr::Named(n) => {
                let (s, e) = ctx.paths.get(n)?;
                Some((s.clone(), e.clone()))
            }
            PathExpr::Refl(x) => Some((x.clone(), x.clone())),
            PathExpr::Compose(p, q) => {
                let (s, _) = p.endpoints(ctx)?;
                let (_, e) = q.endpoints(ctx)?;
                Some((s, e))
            }
            PathExpr::Inverse(p) => {
                let (s, e) = p.endpoints(ctx)?;
                Some((e, s))
            }
        }
    }
}

fn path_expr_eq(a: &PathExpr, b: &PathExpr) -> bool {
    match (a, b) {
        (PathExpr::Named(x), PathExpr::Named(y)) => x == y,
        (PathExpr::Refl(x), PathExpr::Refl(y)) => x == y,
        (PathExpr::Inverse(x), PathExpr::Inverse(y)) => path_expr_eq(x, y),
        (PathExpr::Compose(a1,a2), PathExpr::Compose(b1,b2)) =>
            path_expr_eq(a1,b1) && path_expr_eq(a2,b2),
        _ => false,
    }
}

// ── Sandbox ──────────────────────────────────────────────────────────────────

struct Sandbox {
    points: Vec<String>,
    paths: HashMap<String, (String, String)>,   // name → (start, end)
    path2: HashMap<String, (String, String)>,    // name → (path1, path2) — homotopies
    named: HashMap<String, PathExpr>,           // user-defined compound paths
}

impl Sandbox {
    fn new() -> Self {
        Sandbox {
            points: vec![],
            paths: HashMap::new(),
            path2: HashMap::new(),
            named: HashMap::new(),
        }
    }

    fn path_start(&self, p: &PathExpr) -> Option<String> {
        p.endpoints(self).map(|(s, _)| s)
    }
    fn path_end(&self, p: &PathExpr) -> Option<String> {
        p.endpoints(self).map(|(_, e)| e)
    }

    fn has_point(&self, x: &str) -> bool { self.points.contains(&x.to_string()) }

    fn parse_expr(&self, tokens: &[&str]) -> Option<PathExpr> {
        // Simple parser: handle compose (p q), inverse p^, refl x, named p
        if tokens.is_empty() { return None; }
        if tokens.len() == 1 {
            let t = tokens[0];
            if t.starts_with("refl_") {
                return Some(PathExpr::Refl(t[5..].to_string()));
            }
            if t.ends_with('^') || t.ends_with("^-1") || t.ends_with("⁻¹") {
                let base = t.trim_end_matches("^-1").trim_end_matches('^').trim_end_matches('⁻').trim_end_matches('¹');
                return Some(PathExpr::Inverse(Box::new(PathExpr::Named(base.to_string()))));
            }
            return Some(PathExpr::Named(t.to_string()));
        }
        // Multiple tokens = compose left to right
        let mut exprs: Vec<PathExpr> = vec![];
        for t in tokens {
            exprs.push(self.parse_expr(&[t])?);
        }
        let result = exprs.into_iter().reduce(|a, b| PathExpr::Compose(Box::new(a), Box::new(b)));
        result
    }

    fn handle(&mut self, tokens: &[&str]) {
        match tokens[0] {
            "point" | "pt" => self.cmd_point(tokens),
            "path"         => self.cmd_path(tokens),
            "compose" | "comp" | "·" => self.cmd_compose(tokens),
            "inverse" | "inv"        => self.cmd_inverse(tokens),
            "refl"                   => self.cmd_refl(tokens),
            "eval"                   => self.cmd_eval(tokens),
            "define" | "let"         => self.cmd_define(tokens),
            "check"                  => self.cmd_check(tokens),
            "homotopy" | "2path"     => self.cmd_homotopy(tokens),
            "show"                   => self.cmd_show(),
            "paths"                  => self.cmd_paths(tokens),
            "eckmann" | "ek"         => self.cmd_eckmann(tokens),
            "help" | "h" | "?"       => self.help(),
            _ => println!("{RE}Unknown command. Type 'help'.{R}"),
        }
    }

    fn cmd_point(&mut self, tokens: &[&str]) {
        if tokens.len() < 2 { println!("{RE}Usage: point <name>{R}"); return; }
        for name in &tokens[1..] {
            if self.has_point(name) { println!("{D}Point '{name}' already exists.{R}"); continue; }
            self.points.push(name.to_string());
            println!("{G}Added point {Y}{name}{R}");
        }
    }

    fn cmd_path(&mut self, tokens: &[&str]) {
        // path <name> <start> <end>
        if tokens.len() < 4 { println!("{RE}Usage: path <name> <start> <end>{R}"); return; }
        let (name, start, end) = (tokens[1], tokens[2], tokens[3]);
        if !self.has_point(start) { println!("{RE}Unknown point '{start}'. Add with: point {start}{R}"); return; }
        if !self.has_point(end)   { println!("{RE}Unknown point '{end}'. Add with: point {end}{R}"); return; }
        self.paths.insert(name.to_string(), (start.to_string(), end.to_string()));
        println!("{G}Path {C}{name}{R} : {Y}{start}{R} → {Y}{end}{R}");
    }

    fn cmd_compose(&mut self, tokens: &[&str]) {
        // compose p q  —  or  compose p q = r
        if tokens.len() < 3 { println!("{RE}Usage: compose <p> <q> [= name]{R}"); return; }
        let (p, q) = (tokens[1], tokens[2]);
        let pe = PathExpr::Named(p.to_string());
        let qe = PathExpr::Named(q.to_string());
        // Check endpoint compatibility
        match (self.paths.get(p), self.paths.get(q)) {
            (Some((ps, pe_end)), Some((qs, qe_end))) => {
                if pe_end != qs {
                    println!("{RE}Endpoint mismatch: {p} ends at '{pe_end}' but {q} starts at '{qs}'{R}");
                    return;
                }
                let comp = PathExpr::Compose(Box::new(pe), Box::new(qe));
                let norm = comp.normalize(self);
                println!("{C}{p}{R} · {C}{q}{R} : {Y}{ps}{R} → {Y}{qe_end}{R}");
                println!("  = {}", norm.display());
                // optionally name it
                if let Some(&"=") = tokens.get(3) {
                    if let Some(&name) = tokens.get(4) {
                        self.paths.insert(name.to_string(), (ps.clone(), qe_end.clone()));
                        self.named.insert(name.to_string(), norm);
                        println!("  {G}Saved as '{name}'{R}");
                    }
                }
            }
            _ => println!("{RE}Unknown path(s). Define with: path <name> <start> <end>{R}"),
        }
    }

    fn cmd_inverse(&mut self, tokens: &[&str]) {
        if tokens.len() < 2 { println!("{RE}Usage: inverse <p> [= name]{R}"); return; }
        let p = tokens[1];
        match self.paths.get(p) {
            None => println!("{RE}Unknown path '{p}'{R}"),
            Some((s, e)) => {
                let inv = PathExpr::Inverse(Box::new(PathExpr::Named(p.to_string())));
                let norm = inv.normalize(self);
                let (s, e) = (s.clone(), e.clone());
                println!("{C}{p}{R}⁻¹ : {Y}{e}{R} → {Y}{s}{R}");
                println!("  = {}", norm.display());
                if let Some(&"=") = tokens.get(2) {
                    if let Some(&name) = tokens.get(3) {
                        self.paths.insert(name.to_string(), (e.clone(), s.clone()));
                        self.named.insert(name.to_string(), norm);
                        println!("  {G}Saved as '{name}'{R}");
                    }
                }
            }
        }
    }

    fn cmd_refl(&self, tokens: &[&str]) {
        if tokens.len() < 2 { println!("{RE}Usage: refl <point>{R}"); return; }
        let x = tokens[1];
        if !self.has_point(x) { println!("{RE}Unknown point '{x}'{R}"); return; }
        println!("{D}refl_{R}{Y}{x}{R} : {Y}{x}{R} → {Y}{x}{R}  {D}(identity path at {x}){R}");
    }

    fn cmd_eval(&self, tokens: &[&str]) {
        if tokens.len() < 2 { println!("{RE}Usage: eval <expr...>  e.g.  eval p q^{R}"); return; }
        match self.parse_expr(&tokens[1..]) {
            None => println!("{RE}Cannot parse expression{R}"),
            Some(expr) => {
                let norm = expr.normalize(self);
                let ep = norm.endpoints(self);
                print!("  {}", norm.display());
                if let Some((s, e)) = ep {
                    println!(" : {Y}{s}{R} → {Y}{e}{R}");
                } else {
                    println!(" {D}(endpoints unknown — define paths first){R}");
                }
            }
        }
    }

    fn cmd_define(&mut self, tokens: &[&str]) {
        // let name = p q r...
        if tokens.len() < 4 || tokens[2] != "=" { println!("{RE}Usage: let <name> = <expr...>{R}"); return; }
        let name = tokens[1];
        match self.parse_expr(&tokens[3..]) {
            None => println!("{RE}Cannot parse expression{R}"),
            Some(expr) => {
                let norm = expr.normalize(self);
                let ep = norm.endpoints(self);
                if let Some((s, e)) = ep {
                    self.paths.insert(name.to_string(), (s.clone(), e.clone()));
                    self.named.insert(name.to_string(), norm.clone());
                    println!("{G}Defined {C}{name}{R} : {Y}{s}{R} → {Y}{e}{R} = {}", norm.display());
                } else {
                    println!("{Y}Warning: could not determine endpoints{R}");
                    self.named.insert(name.to_string(), norm);
                }
            }
        }
    }

    fn cmd_check(&self, tokens: &[&str]) {
        if tokens.len() < 2 { self.cmd_check_groupoid(); return; }
        match tokens[1] {
            "groupoid" => self.cmd_check_groupoid(),
            "assoc"    => self.check_assoc(tokens),
            _          => self.cmd_check_groupoid(),
        }
    }

    fn cmd_check_groupoid(&self) {
        println!("{B}Groupoid law check:{R}");
        let paths: Vec<(&str, &str, &str)> = self.paths.iter()
            .map(|(n, (s,e))| (n.as_str(), s.as_str(), e.as_str())).collect();
        println!("  {} Paths defined: {}", tick(true), paths.len());

        // For each path, show refl · p = p
        let mut ok = 0; let mut total = 0;
        for (name, start, end) in &paths {
            let refl_start = PathExpr::Refl(start.to_string());
            let p = PathExpr::Named(name.to_string());
            let comp = PathExpr::Compose(Box::new(refl_start), Box::new(p.clone()));
            let norm = comp.normalize(self);
            let is_named = matches!(&norm, PathExpr::Named(n) if n == name);
            total += 1;
            if is_named { ok += 1; }

            let refl_end = PathExpr::Refl(end.to_string());
            let comp2 = PathExpr::Compose(Box::new(p.clone()), Box::new(refl_end));
            let norm2 = comp2.normalize(self);
            let is_named2 = matches!(&norm2, PathExpr::Named(n) if n == name);
            total += 1;
            if is_named2 { ok += 1; }
        }
        println!("  {} Unit laws: {ok}/{total} hold {D}(refl·p=p and p·refl=p){R}", tick(ok==total));

        // Check p · p⁻¹ = refl
        let mut inv_ok = 0;
        for (name, _, end) in &paths {
            let p = PathExpr::Named(name.to_string());
            let pi = PathExpr::Inverse(Box::new(PathExpr::Named(name.to_string())));
            let comp = PathExpr::Compose(Box::new(p), Box::new(pi));
            let norm = comp.normalize(self);
            let is_refl = matches!(&norm, PathExpr::Refl(x) if x == end);
            if is_refl { inv_ok += 1; }
        }
        println!("  {} Inverse laws: {inv_ok}/{} hold {D}(p·p⁻¹=refl){R}", tick(inv_ok==paths.len()), paths.len());
        println!("{D}Associativity holds by the structure of path composition.{R}");
    }

    fn check_assoc(&self, tokens: &[&str]) {
        if tokens.len() < 5 { println!("{RE}Usage: check assoc <p> <q> <r>{R}"); return; }
        let (p, q, r_name) = (tokens[2], tokens[3], tokens[4]);
        // (p·q)·r vs p·(q·r)
        let pe = PathExpr::Named(p.to_string());
        let qe = PathExpr::Named(q.to_string());
        let re = PathExpr::Named(r_name.to_string());
        let lhs = PathExpr::Compose(
            Box::new(PathExpr::Compose(Box::new(pe.clone()), Box::new(qe.clone()))),
            Box::new(re.clone())
        ).normalize(self);
        let rhs = PathExpr::Compose(
            Box::new(pe),
            Box::new(PathExpr::Compose(Box::new(qe), Box::new(re)))
        ).normalize(self);
        println!("  ({C}{p}{R}·{C}{q}{R})·{C}{r_name}{R} = {}", lhs.display());
        println!("  {C}{p}{R}·({C}{q}{R}·{C}{r_name}{R}) = {}", rhs.display());
        if path_expr_eq(&lhs, &rhs) { println!("  {G}Equal! Associativity holds.{R}"); }
        else { println!("  {Y}Not definitionally equal, but propositionally equal via associator.{R}"); }
    }

    fn cmd_homotopy(&mut self, tokens: &[&str]) {
        // homotopy <name> <p> <q>
        if tokens.len() < 4 { println!("{RE}Usage: homotopy <name> <p> <q>  — declares a 2-path{R}"); return; }
        let (name, p, q) = (tokens[1], tokens[2], tokens[3]);
        match (self.paths.get(p), self.paths.get(q)) {
            (Some((ps, pe)), Some((qs, qe))) => {
                if ps != qs || pe != qe {
                    println!("{RE}Parallel paths must have the same endpoints!{R}");
                    println!("  {C}{p}{R}: {Y}{ps}{R}→{Y}{pe}{R},  {C}{q}{R}: {Y}{qs}{R}→{Y}{qe}{R}");
                    return;
                }
                self.path2.insert(name.to_string(), (p.to_string(), q.to_string()));
                println!("{M}{name}{R} : {C}{p}{R} ⟹ {C}{q}{R}  {D}(2-path / homotopy){R}");
            }
            _ => println!("{RE}Unknown paths. Define with: path <name> <start> <end>{R}"),
        }
    }

    fn cmd_paths(&self, tokens: &[&str]) {
        if tokens.len() >= 3 {
            let (s, e) = (tokens[1], tokens[2]);
            println!("{B}Paths {Y}{s}{R} → {Y}{e}{R}:{R}");
            for (name, (ps, pe)) in &self.paths {
                if ps == s && pe == e { println!("  {C}{name}{R}"); }
            }
        } else {
            self.cmd_show();
        }
    }

    fn cmd_eckmann(&self, tokens: &[&str]) {
        // Demonstrate Eckmann-Hilton: two 2-loops at the same point commute
        println!("{B}Eckmann-Hilton argument:{R}");
        println!("{D}In a double loop space Ω²X, two loops α,β commute: α·β = β·α{R}\n");
        println!("  Consider α,β : refl_x ⟹ refl_x  (2-loops at x)");
        println!("  Two compositions: vertical ·ᵥ and horizontal ·ₕ");
        println!();
        println!("  Horizontal: α ·ₕ β");
        println!("  ┌─────┬─────┐");
        println!("  │  α  │  β  │   = α ·ₕ β");
        println!("  └─────┴─────┘");
        println!();
        println!("  Vertical: α ·ᵥ β");
        println!("  ┌─────┐");
        println!("  │  α  │");
        println!("  ├─────┤   = α ·ᵥ β");
        println!("  │  β  │");
        println!("  └─────┘");
        println!();
        println!("  By interchange law: (α ·ₕ β) = (α ·ᵥ β)");
        println!("  Exchange α←→refl, get: α·β = β·α   {G}(commutativity!){R}");
        println!();
        println!("{D}This is why π₂ groups are abelian — all double loop spaces are.{R}");
        if tokens.len() >= 3 {
            println!("\n  Using your paths {C}{}{R} and {C}{}{R}:", tokens[1], tokens[2]);
            println!("  Both must be 2-paths with same source and target.");
        }
    }

    fn cmd_show(&self) {
        println!("{B}Space:{R}");
        println!("  Points: {}", if self.points.is_empty() { D.to_string() + "none" + R } else {
            self.points.iter().map(|p| format!("{Y}{p}{R}")).collect::<Vec<_>>().join("  ")
        });
        println!("  1-Paths:");
        for (name, (s, e)) in &self.paths {
            let def = self.named.get(name).map(|x| format!(" = {}", x.display())).unwrap_or_default();
            println!("    {C}{name}{R} : {Y}{s}{R} → {Y}{e}{R}{}", def);
        }
        if !self.path2.is_empty() {
            println!("  2-Paths:");
            for (name, (p, q)) in &self.path2 {
                println!("    {M}{name}{R} : {C}{p}{R} ⟹ {C}{q}{R}");
            }
        }
    }

    fn help(&self) {
        println!("{B}Path Algebra Sandbox{R} — build path spaces, explore groupoid laws\n");
        println!("{B}Build the space:{R}");
        println!("  {C}point x y z{R}          add points x, y, z");
        println!("  {C}path p x y{R}            add path p : x → y");
        println!("  {C}refl x{R}                identity path at x");
        println!("  {C}homotopy h p q{R}        declare 2-path h : p ⟹ q (same endpoints)");
        println!();
        println!("{B}Compute with paths:{R}");
        println!("  {C}compose p q{R}           compute p · q (validates endpoint match)");
        println!("  {C}compose p q = r{R}       compose and name the result r");
        println!("  {C}inverse p{R}             compute p⁻¹");
        println!("  {C}eval p q p^{R}           evaluate a path expression (^ = inverse)");
        println!("  {C}let r = p q p^{R}        define r = p · q · p⁻¹");
        println!();
        println!("{B}Verify laws:{R}");
        println!("  {C}check{R}                 verify groupoid laws for all paths");
        println!("  {C}check assoc p q r{R}     check (p·q)·r vs p·(q·r)");
        println!("  {C}eckmann p q{R}           demonstrate Eckmann-Hilton for 2-loops");
        println!();
        println!("{B}Inspect:{R}");
        println!("  {C}show{R}                  display all points and paths");
        println!("  {C}paths x y{R}             list paths from x to y");
        println!();
        println!("{D}Try: point x y z → path p x y → path q y z → compose p q → check{R}");
    }
}

fn tick(b: bool) -> &'static str { if b { "\x1b[32m✓\x1b[0m" } else { "\x1b[31m✗\x1b[0m" } }

fn main() {
    println!("{B}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{R}");
    println!("{B}  Path Algebra Sandbox{R}");
    println!("{D}  Build a space, compose paths, verify groupoid laws{R}");
    println!("{B}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{R}");
    println!("{D}Type 'help' for commands. Try: point x y z → path p x y → path q y z → compose p q{R}\n");

    let mut sb = Sandbox::new();
    loop {
        let line = read_line("paths>");
        if line.is_empty() { continue; }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens[0] {
            "quit" | "q" | "exit" => { println!("{D}Goodbye.{R}"); break; }
            _ => sb.handle(&tokens),
        }
    }
}
