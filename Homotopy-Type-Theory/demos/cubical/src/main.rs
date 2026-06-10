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

// ── Cubical HoTT ──────────────────────────────────────────────────────────────
//
// Key ideas:
// - The interval 𝕀 = {0, 1} with a formal connection ∧, ∨, and complement ~
// - Paths A are functions 𝕀 → A
// - Composition is built from hcomp (homogeneous composition)
// - Univalence is provable (ua : (A ≃ B) → (A =𝒰 B))

// ── Interval expressions ──────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum IExpr {
    Zero,               // 0
    One,                // 1
    Var(String),        // i, j, k, ...
    Meet(Box<IExpr>, Box<IExpr>), // i ∧ j
    Join(Box<IExpr>, Box<IExpr>), // i ∨ j
    Neg(Box<IExpr>),    // ~ i (complement)
}

impl IExpr {
    fn display(&self) -> String {
        match self {
            IExpr::Zero => "0".into(),
            IExpr::One => "1".into(),
            IExpr::Var(s) => s.clone(),
            IExpr::Meet(a, b) => format!("{} ∧ {}", a.display_atom(), b.display_atom()),
            IExpr::Join(a, b) => format!("{} ∨ {}", a.display_atom(), b.display_atom()),
            IExpr::Neg(a) => format!("~{}", a.display_atom()),
        }
    }

    fn display_atom(&self) -> String {
        match self {
            IExpr::Zero | IExpr::One | IExpr::Var(_) | IExpr::Neg(_) => self.display(),
            _ => format!("({})", self.display()),
        }
    }

    fn eval(&self, env: &HashMap<String, bool>) -> Option<bool> {
        match self {
            IExpr::Zero => Some(false),
            IExpr::One => Some(true),
            IExpr::Var(s) => env.get(s).copied(),
            IExpr::Meet(a, b) => Some(a.eval(env)? && b.eval(env)?),
            IExpr::Join(a, b) => Some(a.eval(env)? || b.eval(env)?),
            IExpr::Neg(a) => Some(!a.eval(env)?),
        }
    }

    fn free_vars(&self) -> Vec<String> {
        let mut vs = vec![];
        self.collect_vars(&mut vs);
        vs.sort(); vs.dedup(); vs
    }

    fn collect_vars(&self, out: &mut Vec<String>) {
        match self {
            IExpr::Var(s) => out.push(s.clone()),
            IExpr::Meet(a, b) | IExpr::Join(a, b) => { a.collect_vars(out); b.collect_vars(out); }
            IExpr::Neg(a) => a.collect_vars(out),
            _ => {}
        }
    }
}

fn parse_iexpr(s: &str) -> Option<IExpr> {
    let s = s.trim();
    // Join (lowest precedence)
    if let Some(i) = find_op(s, "|") {
        let a = parse_iexpr(&s[..i])?;
        let b = parse_iexpr(&s[i+1..])?;
        return Some(IExpr::Join(Box::new(a), Box::new(b)));
    }
    // Meet
    if let Some(i) = find_op(s, "&") {
        let a = parse_iexpr(&s[..i])?;
        let b = parse_iexpr(&s[i+1..])?;
        return Some(IExpr::Meet(Box::new(a), Box::new(b)));
    }
    // Neg
    if s.starts_with('~') || s.starts_with('!') {
        let a = parse_iexpr(&s[1..])?;
        return Some(IExpr::Neg(Box::new(a)));
    }
    // Parens
    if s.starts_with('(') && s.ends_with(')') {
        return parse_iexpr(&s[1..s.len()-1]);
    }
    match s {
        "0" | "i0" => Some(IExpr::Zero),
        "1" | "i1" => Some(IExpr::One),
        v if v.chars().all(|c| c.is_alphanumeric() || c == '_') => Some(IExpr::Var(v.to_string())),
        _ => None,
    }
}

fn find_op(s: &str, op: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c { '(' => depth += 1, ')' => depth -= 1, _ => {} }
        if depth == 0 && s[i..].starts_with(op) { return Some(i); }
    }
    None
}

// ── Cubical paths ─────────────────────────────────────────────────────────────

#[derive(Clone)]
struct CubPath {
    name: String,
    param: String,      // the interval variable
    face0: String,      // value at i=0
    face1: String,      // value at i=1
}

impl CubPath {
    fn display(&self) -> String {
        format!("λ{}. [{}..{}]", self.param, self.face0, self.face1)
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    ivars: HashMap<String, bool>,   // interval variable assignments
    cubpaths: HashMap<String, CubPath>,
    points: HashMap<String, String>, // terms/values
}

impl Sandbox {
    fn new() -> Self {
        Sandbox { ivars: HashMap::new(), cubpaths: HashMap::new(), points: HashMap::new() }
    }

    fn print_help() {
        println!("{}", bold("── Interval Calculus ───────────────────────────────────────────────"));
        println!("  {}  i 0|1            — assign value to interval variable", cyan("var"));
        println!("  {}  i ∧ j            — evaluate interval expression", cyan("eval"));
        println!("  {}  i ∧ j            — show truth table", cyan("table"));
        println!("{}", bold("── Cubical Paths ───────────────────────────────────────────────────"));
        println!("  {}  a x y           — define a point", cyan("point"));
        println!("  {}  p i a b         — path from a to b, param i", cyan("cpath"));
        println!("  {}  p               — show path endpoints", cyan("show"));
        println!("  {}  p q             — compose paths", cyan("comp"));
        println!("  {}  p               — inverse path (reverse endpoints)", cyan("inv"));
        println!("{}", bold("── Key Concepts ────────────────────────────────────────────────────"));
        println!("  {}         — explain hcomp and connections", cyan("hcomp"));
        println!("  {}         — explain how ua computes", cyan("ua"));
        println!("  {}         — show interval operations", cyan("interval"));
        println!("  {}         — show main cubical identities", cyan("rules"));
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
            "var" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: var i 0|1", red("✗")); return true; }
                let val = match ws[1] { "0" | "false" => false, "1" | "true" => true,
                    _ => { println!("  {} Use 0 or 1", red("✗")); return true; } };
                self.ivars.insert(ws[0].to_string(), val);
                println!("  {} = {}", cyan(ws[0]), if val { green("1") } else { red("0") });
            }
            "eval" => {
                match parse_iexpr(rest) {
                    Some(e) => {
                        let vars = e.free_vars();
                        let mut env = HashMap::new();
                        for v in &vars {
                            if let Some(&val) = self.ivars.get(v) { env.insert(v.clone(), val); }
                        }
                        match e.eval(&env) {
                            Some(true) => println!("  {} = {}", e.display(), green("1")),
                            Some(false) => println!("  {} = {}", e.display(), red("0")),
                            None => println!("  {} = {} (unassigned vars: {})", e.display(), yellow("?"), vars.join(", ")),
                        }
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "table" => {
                match parse_iexpr(rest) {
                    Some(e) => {
                        let vars = e.free_vars();
                        if vars.len() > 4 { println!("  {} Max 4 vars", red("✗")); return true; }
                        let n = vars.len();
                        print!("  ");
                        for v in &vars { print!("{v:4}"); }
                        println!("  │  {}", e.display());
                        for mask in 0u32..(1 << n) {
                            let mut env = HashMap::new();
                            print!("  ");
                            for (i, v) in vars.iter().enumerate() {
                                let val = (mask >> i) & 1 == 1;
                                env.insert(v.clone(), val);
                                let vs = if val { green("1") } else { red("0") };
                                print!("{:>4}", vs);
                            }
                            let result = match e.eval(&env) {
                                Some(true) => green("1"),
                                Some(false) => red("0"),
                                None => yellow("?"),
                            };
                            println!("  │  {result}");
                        }
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "point" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.is_empty() { println!("  {} Use: point name value", red("✗")); return true; }
                let name = ws[0];
                let value = ws.get(1).copied().unwrap_or(name);
                self.points.insert(name.to_string(), value.to_string());
                println!("  {} : A = {}", cyan(name), value);
            }
            "cpath" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 4 { println!("  {} Use: cpath name param face0 face1", red("✗")); return true; }
                let (pname, param, f0, f1) = (ws[0], ws[1], ws[2], ws[3]);
                println!("  {} = λ{}. [{}..{}]  : {} = {}", cyan(pname), param, f0, f1, f0, f1);
                self.cubpaths.insert(pname.to_string(), CubPath {
                    name: pname.to_string(), param: param.to_string(),
                    face0: f0.to_string(), face1: f1.to_string(),
                });
            }
            "show" => {
                if rest.is_empty() {
                    println!("{}", bold("  Points:")); for (n, v) in &self.points { println!("  {} = {}", cyan(n), v); }
                    println!("{}", bold("  Paths:"));  for (_, p) in &self.cubpaths { println!("  {} = {}", cyan(&p.name), p.display()); }
                    return true;
                }
                if let Some(p) = self.cubpaths.get(rest) {
                    let p = p.clone();
                    println!("  {} = λ{}. [{}..{}]", cyan(&p.name), p.param, p.face0, p.face1);
                    println!("  {}(0) = {}", p.name, cyan(&p.face0));
                    println!("  {}(1) = {}", p.name, cyan(&p.face1));
                    // Evaluate at different points
                    println!("  Midpoint (i = i): {}", dim("(abstract — depends on A)"));
                } else { println!("  {} Path {rest} not found", red("✗")); }
            }
            "comp" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: comp p q", red("✗")); return true; }
                let p = self.cubpaths.get(ws[0]).cloned();
                let q = self.cubpaths.get(ws[1]).cloned();
                match (p, q) {
                    (Some(p), Some(q)) => {
                        if p.face1 != q.face0 {
                            println!("  {} Endpoints don't match: {} ends at {} but {} starts at {}",
                                red("✗"), p.name, p.face1, q.name, q.face0);
                            return true;
                        }
                        let name = format!("{}·{}", p.name, q.name);
                        println!("  {} = λi. hcomp [{}..{}..{}]", cyan(&name), p.face0, q.face0, q.face1);
                        println!("  Endpoints: {} = {} .. {} = {}", p.param, cyan(&p.face0), q.param, cyan(&q.face1));
                        println!("  {}", dim("(Composition via hcomp with i∧j and i∨j connections)"));
                    }
                    _ => println!("  {} Path(s) not found", red("✗")),
                }
            }
            "inv" => {
                if let Some(p) = self.cubpaths.get(rest).cloned() {
                    let inv_name = format!("{}⁻¹", p.name);
                    println!("  {} = λi. {}(~i) : {} = {}", cyan(&inv_name), p.name, p.face1, p.face0);
                    self.cubpaths.insert(inv_name.clone(), CubPath {
                        name: inv_name, param: p.param, face0: p.face1.clone(), face1: p.face0.clone(),
                    });
                } else { println!("  {} Path {rest} not found", red("✗")); }
            }
            "interval" => {
                println!("{}", bold("── The Interval 𝕀 in Cubical HoTT ─────────────────────────────────"));
                println!("  𝕀 = {{0, 1}} with operations:");
                println!("  i ∧ j  (meet/min):   0∧0=0, 0∧1=0, 1∧0=0, 1∧1=1");
                println!("  i ∨ j  (join/max):   0∨0=0, 0∨1=1, 1∨0=1, 1∨1=1");
                println!("  ~i     (complement): ~0=1, ~1=0");
                println!();
                println!("  Connection laws:");
                println!("  i ∧ 0 = 0     i ∨ 0 = i");
                println!("  i ∧ 1 = i     i ∨ 1 = 1");
                println!("  i ∧ ~i = 0    i ∨ ~i = 1");
                println!();
                println!("  A path p : A is a term p : 𝕀 → A");
                println!("  p(0) = start,  p(1) = end");
                println!("  Inverse:   p⁻¹(i) = p(~i)");
                println!("  Refl:      refl_x(i) = x  (constant path)");
            }
            "hcomp" => {
                println!("{}", bold("── hcomp (Homogeneous Composition) ──────────────────────────────────"));
                println!("  hcomp : (i : 𝕀) → Partial (i=1) A → A → A");
                println!();
                println!("  hcomp u a  fills a square whose:");
                println!("    left face   = a      (the 'base')");
                println!("    right face  = u(i=1) (the 'lid')");
                println!();
                println!("  This is how we:");
                println!("  • Compose paths: (p·q)(i) = hcomp with j∈[0,1]");
                println!("  • Prove transp gives homotopies");
                println!("  • Make univalence compute (ua becomes transp)");
                println!();
                println!("  In classical HoTT: paths are defined via J eliminator");
                println!("  In Cubical HoTT:   paths are functions 𝕀 → A, composition computes");
                println!("  This is the {} between the two systems.", bold("key computational difference"));
            }
            "ua" => {
                println!("{}", bold("── ua : (A ≃ B) → (A =𝒰 B) in Cubical HoTT ───────────────────────────"));
                println!("  In classical HoTT: ua is an axiom (has no computational content)");
                println!("  In Cubical HoTT:   ua computes via glue types");
                println!();
                println!("  Glue(φ, T, e, a) is a term in A glued to T via e");
                println!("  where φ is a cofibration formula (like i=0 or i=1)");
                println!();
                println!("  ua(e) : A = B  is built as:");
                println!("    ua(e)(i) = Glue(i=0 ↦ A, i=1 ↦ B) with equivalence e");
                println!();
                println!("  Crucially: transp (ua e) a = e.fwd(a)   ← this COMPUTES!");
                println!("  Classical HoTT: transp (ua e) a is {} in general.", red("stuck"));
                println!();
                println!("  This computational behavior makes Cubical HoTT a proper programming language");
                println!("  for HoTT, where univalence is not just an axiom but an implementable fact.");
            }
            "rules" => {
                println!("{}", bold("── Main Cubical Reduction Rules ────────────────────────────────────"));
                println!("  (λi. t)(0) = t[0/i]                (path application)");
                println!("  (λi. t)(1) = t[1/i]");
                println!("  transp (λi. A) a   computes!        (no stuck terms)");
                println!("  transp (λi. A)(i=0) a = a          (reflexivity)");
                println!("  hcomp(λi. A, u, a)(i=1) = u(1)    (composition filler)");
                println!("  transp (ua e) a = e.fwd(a)          (univalence computes)");
                println!();
                println!("  Try: table i & ~i   (= 0 always — diagonal connection)");
                println!("  Try: table i | ~i   (= 1 always — codiscrete)");
                println!("  Try: cpath p i a b  then  inv p  then  comp p p_inv");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║     Cubical HoTT — Interval and hcomp Sandbox           ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Work with the interval 𝕀, connections (∧, ∨, ~), and cubical paths.");
    println!("  See how paths compute, hcomp fills squares, and ua reduces.");
    println!("  Type {} for commands, {} for the interval, {} for hcomp.\n", cyan("help"), cyan("interval"), cyan("hcomp"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}cubical{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
