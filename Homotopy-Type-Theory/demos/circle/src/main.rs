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

// ── S¹ as a HIT ───────────────────────────────────────────────────────────────
//
// S¹ has:
//   - base : S¹
//   - loop : base = base
//
// π₁(S¹) = ℤ: loops are classified by winding number.
// We represent paths in S¹ as integer winding numbers (the encode-decode proof).

// ── Path expressions in S¹ ────────────────────────────────────────────────────

#[derive(Clone, Debug)]
enum S1Path {
    Refl,           // refl at base = winding 0
    Loop,           // the generator loop = winding 1
    LoopInv,        // loop⁻¹ = winding -1
    Compose(Box<S1Path>, Box<S1Path>),
    Inverse(Box<S1Path>),
    Power(i64),     // loop^n
    Named(String),
}

impl S1Path {
    fn winding(&self, named: &HashMap<String, S1Path>) -> i64 {
        match self {
            S1Path::Refl => 0,
            S1Path::Loop => 1,
            S1Path::LoopInv => -1,
            S1Path::Power(n) => *n,
            S1Path::Compose(a, b) => a.winding(named) + b.winding(named),
            S1Path::Inverse(p) => -p.winding(named),
            S1Path::Named(s) => named.get(s).map(|p| p.winding(named)).unwrap_or(0),
        }
    }

    fn display(&self) -> String {
        match self {
            S1Path::Refl => "refl".into(),
            S1Path::Loop => "loop".into(),
            S1Path::LoopInv => "loop⁻¹".into(),
            S1Path::Power(n) if *n >= 0 => format!("loop^{n}"),
            S1Path::Power(n) => format!("loop^({n})"),
            S1Path::Compose(a, b) => format!("{} · {}", a.display(), b.display()),
            S1Path::Inverse(p) => format!("({})⁻¹", p.display()),
            S1Path::Named(s) => s.clone(),
        }
    }
}

fn parse_s1_path(s: &str, named: &HashMap<String, S1Path>) -> Option<S1Path> {
    let s = s.trim();
    // Composite: split at first ·  or  .  or  then
    for sep in [" · ", " . ", " then "] {
        if let Some(i) = s.find(sep) {
            let a = parse_s1_path(&s[..i], named)?;
            let b = parse_s1_path(&s[i+sep.len()..], named)?;
            return Some(S1Path::Compose(Box::new(a), Box::new(b)));
        }
    }
    // Inverse suffixes
    if s.ends_with("^-1") || s.ends_with("^{-1}") || s.ends_with("_inv") {
        let base = s.trim_end_matches("^-1").trim_end_matches("^{-1}").trim_end_matches("_inv");
        return Some(S1Path::Inverse(Box::new(parse_s1_path(base, named)?)));
    }
    // loop^n
    if s.starts_with("loop^") {
        let rest = &s[5..].trim_matches(|c| c == '(' || c == ')');
        let n: i64 = rest.parse().ok()?;
        return Some(S1Path::Power(n));
    }
    match s {
        "refl" | "id" => Some(S1Path::Refl),
        "loop" => Some(S1Path::Loop),
        "loop^-1" | "loop_inv" => Some(S1Path::LoopInv),
        name => {
            if named.contains_key(name) { Some(S1Path::Named(name.to_string())) }
            else if let Ok(n) = name.parse::<i64>() { Some(S1Path::Power(n)) }
            else { None }
        }
    }
}

fn wind_display(n: i64) -> String {
    match n {
        0 => format!("{} (contractible, identity in π₁)", green("0")),
        1 => format!("{} (one loop clockwise)", cyan("1")),
        -1 => format!("{} (one loop counterclockwise)", cyan("-1")),
        n if n > 0 => format!("{} (loops clockwise)", cyan(&n.to_string())),
        n => format!("{} (loops counterclockwise)", cyan(&n.to_string())),
    }
}

// ── Circle animation ──────────────────────────────────────────────────────────

fn draw_circle(angle_steps: i64) {
    let n = ((angle_steps % 8 + 8) % 8) as usize;
    let positions = [
        "  ·  ·  *  ·  ·",
        "  ·  ·  ·  *  ·",
        "  ·  ·  ·  ·  *",
        "  ·  ·  ·  *  ·",
        "  ·  ·  *  ·  ·",
        "  ·  *  ·  ·  ·",
        "  *  ·  ·  ·  ·",
        "  ·  *  ·  ·  ·",
    ];
    println!("      ┌─────────────┐");
    println!("      │  S¹         │");
    println!("      │  {}", positions[n]);
    println!("      │  base=●     │");
    println!("      └─────────────┘");
    if angle_steps == 0 { println!("      (at base, winding = 0)"); }
    else { println!("      (winding = {})", angle_steps); }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    named: HashMap<String, S1Path>,
    history: Vec<i64>, // history of winding numbers computed
}

impl Sandbox {
    fn new() -> Self { Sandbox { named: HashMap::new(), history: vec![] } }

    fn print_help() {
        println!("{}", bold("── S¹ Path Language ────────────────────────────────────────────────"));
        println!("  {}               — the basepoint loop (winding +1)", cyan("loop"));
        println!("  {}        — the inverse loop (winding -1)", cyan("loop^-1"));
        println!("  {}          — the trivial path (winding 0)", cyan("refl"));
        println!("  {}          — n-fold iteration of loop", cyan("loop^n"));
        println!("  {}  p q         — compose paths (add windings)", cyan("compose"));
        println!("  {}  p           — reverse path (negate winding)", cyan("invert"));
        println!("  {}  name = ...  — name a path", cyan("let"));
        println!("  {}  p           — compute winding number", cyan("wind"));
        println!("  {}  p           — evaluate and animate", cyan("eval"));
        println!("{}", bold("── Fundamental Group ───────────────────────────────────────────────"));
        println!("  {}       — display the π₁(S¹) = ℤ correspondence", cyan("pi1"));
        println!("  {}  n          — show n as a loop in S¹", cyan("show"));
        println!("  {}  p q        — check if homotopic (same winding)", cyan("homotopic"));
        println!("  {}  p          — check if contractible (winding 0)", cyan("contractible"));
        println!("  {}  p q        — show composition table entry", cyan("mul"));
        println!("{}", bold("── Examples ────────────────────────────────────────────────────────"));
        println!("  eval loop^3 · loop^-1   (= loop^2, winding 2)");
        println!("  homotopic loop^3 loop . loop . loop");
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
            "let" => {
                if let Some((name, expr)) = rest.split_once('=') {
                    let name = name.trim();
                    match parse_s1_path(expr.trim(), &self.named) {
                        Some(p) => {
                            let w = p.winding(&self.named);
                            println!("  {} : {} = winding {}", cyan(name), p.display(), wind_display(w));
                            self.named.insert(name.to_string(), p);
                        }
                        None => println!("  {} Parse error", red("✗")),
                    }
                }
            }
            "wind" => {
                match parse_s1_path(rest, &self.named) {
                    Some(p) => {
                        let w = p.winding(&self.named);
                        println!("  winding({}) = {}", p.display(), wind_display(w));
                        self.history.push(w);
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "eval" => {
                let expr = if rest.is_empty() { cmd } else { rest };
                match parse_s1_path(expr, &self.named) {
                    Some(p) => {
                        let w = p.winding(&self.named);
                        println!("  {} = winding {}", p.display(), wind_display(w));
                        draw_circle(w);
                        self.history.push(w);
                    }
                    None => println!("  {} Parse error in: {expr}", red("✗")),
                }
            }
            "compose" => {
                let ps: Vec<&str> = rest.splitn(2, ' ').collect();
                if ps.len() < 2 { println!("  {} Use: compose p q", red("✗")); return true; }
                let a = parse_s1_path(ps[0], &self.named);
                let b = parse_s1_path(ps[1], &self.named);
                match (a, b) {
                    (Some(p), Some(q)) => {
                        let wp = p.winding(&self.named);
                        let wq = q.winding(&self.named);
                        let w = wp + wq;
                        println!("  ({}) · ({}) = loop^{}", p.display(), q.display(), w);
                        println!("  winding: {} + {} = {}", wp, wq, wind_display(w));
                    }
                    _ => println!("  {} Parse error", red("✗")),
                }
            }
            "invert" => {
                match parse_s1_path(rest, &self.named) {
                    Some(p) => {
                        let w = -p.winding(&self.named);
                        println!("  ({})⁻¹ = loop^{}", p.display(), w);
                        println!("  winding: {}", wind_display(w));
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "homotopic" => {
                let ps: Vec<&str> = rest.splitn(2, ' ').collect();
                if ps.len() < 2 { println!("  {} Use: homotopic p q", red("✗")); return true; }
                match (parse_s1_path(ps[0], &self.named), parse_s1_path(ps[1], &self.named)) {
                    (Some(p), Some(q)) => {
                        let wp = p.winding(&self.named);
                        let wq = q.winding(&self.named);
                        if wp == wq {
                            println!("  {} ∼ {} (both winding {})", p.display(), q.display(), green(&wp.to_string()));
                        } else {
                            println!("  {} ≁ {} (windings {} ≠ {})", p.display(), q.display(), red(&wp.to_string()), red(&wq.to_string()));
                        }
                    }
                    _ => println!("  {} Parse error", red("✗")),
                }
            }
            "contractible" => {
                match parse_s1_path(rest, &self.named) {
                    Some(p) => {
                        let w = p.winding(&self.named);
                        if w == 0 { println!("  {} is {} (winding 0 = contractible to refl)", p.display(), green("contractible")); }
                        else { println!("  {} is {} (winding {} ≠ 0)", p.display(), red("non-contractible"), w); }
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "mul" => {
                let ws: Vec<i64> = rest.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if ws.len() >= 2 {
                    let a = ws[0]; let b = ws[1];
                    println!("  loop^{a} · loop^{b} = loop^{} in π₁(S¹) = ℤ", a + b);
                } else { println!("  {} Use: mul n m", red("✗")); }
            }
            "show" => {
                let n: i64 = rest.parse().unwrap_or(0);
                println!("  loop^{n} in S¹:");
                draw_circle(n);
            }
            "pi1" => {
                println!("{}", bold("  π₁(S¹) ≅ ℤ — the fundamental group of the circle"));
                println!("  ────────────────────────────────────────────────");
                println!("  S¹ is defined as a HIT with:");
                println!("    {}  : S¹", cyan("base"));
                println!("    {}  : base = base   (a non-trivial path!)", cyan("loop"));
                println!();
                println!("  Every loop at base is homotopic to loop^n for a unique n ∈ ℤ");
                println!("  This is the content of π₁(S¹) ≅ ℤ.");
                println!();
                for n in -3i64..=3 {
                    let s = match n { 0 => "(trivial)".into(), 1 => "(generator)".into(),
                        -1 => "(inverse)".into(), _ => String::new() };
                    println!("  loop^{:2}  ↔  {:3}  {}", n, n, dim(&s));
                }
                println!();
                println!("  Group structure: concat paths = add integers");
                println!("  loop^m · loop^n = loop^(m+n)  (addition in ℤ)");
                println!("  (loop^n)⁻¹ = loop^(-n)  (negation in ℤ)");
            }
            "history" => {
                if self.history.is_empty() { println!("  {}", dim("(empty)")); }
                else {
                    println!("  Winding numbers: {:?}", self.history);
                }
            }
            _ => {
                // Try to evaluate bare expression
                match parse_s1_path(trimmed, &self.named) {
                    Some(p) => {
                        let w = p.winding(&self.named);
                        println!("  winding = {}", wind_display(w));
                        self.history.push(w);
                    }
                    None => println!("  {} Unknown command or parse error. Type {} for help.", red("✗"), cyan("help")),
                }
            }
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║     π₁(S¹) ≅ ℤ — The Circle Sandbox                    ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  S¹ is a Higher Inductive Type with one point (base) and one path");
    println!("  (loop : base = base). Explore how loops compose to form ℤ.");
    println!("  Type {} for commands, {} for the full π₁ story.\n", cyan("help"), cyan("pi1"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}S¹{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
