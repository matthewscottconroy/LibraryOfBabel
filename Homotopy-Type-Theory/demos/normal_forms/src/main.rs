// Lambda Calculus Sandbox — define terms, reduce step by step, find normal forms
use std::collections::HashMap;
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

// ── Lambda calculus AST ──────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
enum Term {
    Var(String),
    Lam(String, Box<Term>),
    App(Box<Term>, Box<Term>),
    Nat(u64),
}

impl Term {
    fn display(&self) -> String {
        match self {
            Term::Var(x)    => format!("{C}{x}{R}"),
            Term::Nat(n)    => format!("{Y}{n}{R}"),
            Term::Lam(x, b) => format!("(λ{C}{x}{R}. {})", b.display()),
            Term::App(f, a) => {
                let f_str = match f.as_ref() {
                    Term::Lam(_, _) => format!("({})", f.display()),
                    other => other.display(),
                };
                let a_str = match a.as_ref() {
                    Term::App(_, _) | Term::Lam(_, _) => format!("({})", a.display()),
                    other => other.display(),
                };
                format!("{f_str} {a_str}")
            }
        }
    }

    fn is_normal(&self) -> bool {
        match self {
            Term::Var(_) | Term::Nat(_) => true,
            Term::Lam(_, b) => b.is_normal(),
            Term::App(f, a) => {
                if matches!(f.as_ref(), Term::Lam(_, _)) { return false; }
                f.is_normal() && a.is_normal()
            }
        }
    }

    fn free_vars(&self) -> std::collections::HashSet<String> {
        match self {
            Term::Var(x) => [x.clone()].into(),
            Term::Nat(_) => Default::default(),
            Term::Lam(x, b) => { let mut fv = b.free_vars(); fv.remove(x); fv }
            Term::App(f, a) => { let mut fv = f.free_vars(); fv.extend(a.free_vars()); fv }
        }
    }

    fn subst(&self, var: &str, val: &Term) -> Term {
        match self {
            Term::Var(x) => if x == var { val.clone() } else { Term::Var(x.clone()) },
            Term::Nat(n) => Term::Nat(*n),
            Term::Lam(x, b) => {
                if x == var { return Term::Lam(x.clone(), b.clone()); }
                if val.free_vars().contains(x) {
                    // Alpha-rename: x → x'
                    let x_new = format!("{x}'");
                    let b_renamed = b.subst(x, &Term::Var(x_new.clone()));
                    Term::Lam(x_new, Box::new(b_renamed.subst(var, val)))
                } else {
                    Term::Lam(x.clone(), Box::new(b.subst(var, val)))
                }
            }
            Term::App(f, a) => Term::App(Box::new(f.subst(var, val)), Box::new(a.subst(var, val))),
        }
    }

    fn beta_step(&self) -> Option<Term> {
        match self {
            Term::App(f, a) => {
                if let Term::Lam(x, b) = f.as_ref() {
                    // β-reduce: (λx.b) a → b[a/x]
                    return Some(b.subst(x, a));
                }
                // Try reducing function first
                if let Some(f2) = f.beta_step() {
                    return Some(Term::App(Box::new(f2), a.clone()));
                }
                // Try reducing argument
                if let Some(a2) = a.beta_step() {
                    return Some(Term::App(f.clone(), Box::new(a2)));
                }
                None
            }
            Term::Lam(x, b) => {
                b.beta_step().map(|b2| Term::Lam(x.clone(), Box::new(b2)))
            }
            _ => None,
        }
    }

    fn normalize(&self, max: usize) -> (Term, Vec<Term>) {
        let mut cur = self.clone();
        let mut steps = vec![];
        for _ in 0..max {
            match cur.beta_step() {
                Some(next) => { steps.push(next.clone()); cur = next; }
                None => break,
            }
        }
        (cur, steps)
    }
}

// ── Parser (simple recursive descent) ───────────────────────────────────────

struct Parser<'a> {
    src: &'a [char],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(src: &'a [char]) -> Self { Parser { src, pos: 0 } }

    fn skip_ws(&mut self) { while self.pos < self.src.len() && self.src[self.pos].is_whitespace() { self.pos += 1; } }

    fn peek(&mut self) -> Option<char> { self.skip_ws(); self.src.get(self.pos).copied() }

    fn eat(&mut self, c: char) -> bool {
        self.skip_ws();
        if self.src.get(self.pos) == Some(&c) { self.pos += 1; true } else { false }
    }

    fn parse_name(&mut self) -> Option<String> {
        self.skip_ws();
        if self.pos >= self.src.len() { return None; }
        if !self.src[self.pos].is_alphabetic() && self.src[self.pos] != '_' { return None; }
        let start = self.pos;
        while self.pos < self.src.len() && (self.src[self.pos].is_alphanumeric() || self.src[self.pos] == '_' || self.src[self.pos] == '\'') {
            self.pos += 1;
        }
        Some(self.src[start..self.pos].iter().collect())
    }

    fn parse_number(&mut self) -> Option<u64> {
        self.skip_ws();
        if self.pos >= self.src.len() || !self.src[self.pos].is_ascii_digit() { return None; }
        let start = self.pos;
        while self.pos < self.src.len() && self.src[self.pos].is_ascii_digit() { self.pos += 1; }
        let s: String = self.src[start..self.pos].iter().collect();
        s.parse().ok()
    }

    // atom = var | number | (term) | λx.term
    fn parse_atom(&mut self) -> Option<Term> {
        self.skip_ws();
        if self.pos >= self.src.len() { return None; }
        let c = self.src[self.pos];
        if c == '(' {
            self.pos += 1;
            let t = self.parse_term()?;
            self.eat(')');
            return Some(t);
        }
        if c == 'λ' || c == '\\' {
            self.pos += 1;
            let var = self.parse_name()?;
            self.skip_ws();
            if self.pos < self.src.len() && self.src[self.pos] == '.' { self.pos += 1; }
            let body = self.parse_term()?;
            return Some(Term::Lam(var, Box::new(body)));
        }
        if c.is_ascii_digit() {
            return self.parse_number().map(Term::Nat);
        }
        if c.is_alphabetic() || c == '_' {
            return self.parse_name().map(Term::Var);
        }
        None
    }

    // application = left-associative sequence of atoms
    fn parse_app(&mut self) -> Option<Term> {
        let mut t = self.parse_atom()?;
        loop {
            match self.parse_atom() {
                Some(a) => t = Term::App(Box::new(t), Box::new(a)),
                None => break,
            }
        }
        Some(t)
    }

    fn parse_term(&mut self) -> Option<Term> { self.parse_app() }
}

fn parse(s: &str, defs: &HashMap<String, Term>) -> Result<Term, String> {
    let chars: Vec<char> = s.chars().collect();
    let mut p = Parser::new(&chars);
    let t = p.parse_term().ok_or("Parse error")?;
    Ok(expand_defs(t, defs))
}

fn expand_defs(t: Term, defs: &HashMap<String, Term>) -> Term {
    match t {
        Term::Var(ref x) => defs.get(x).cloned().unwrap_or(t),
        Term::Nat(_) => t,
        Term::Lam(x, b) => Term::Lam(x, Box::new(expand_defs(*b, defs))),
        Term::App(f, a) => Term::App(Box::new(expand_defs(*f, defs)), Box::new(expand_defs(*a, defs))),
    }
}

// Church numerals
fn church(n: u64) -> Term {
    let mut body = Term::Var("x".into());
    for _ in 0..n { body = Term::App(Box::new(Term::Var("f".into())), Box::new(body)); }
    Term::Lam("f".into(), Box::new(Term::Lam("x".into(), Box::new(body))))
}

// ── Sandbox ──────────────────────────────────────────────────────────────────

struct Sandbox {
    defs: HashMap<String, Term>,
}

impl Sandbox {
    fn new() -> Self {
        let mut defs = HashMap::new();
        // Standard prelude
        let prelude = [
            ("I",     r"\x.x"),
            ("K",     r"\x.\y.x"),
            ("S",     r"\f.\g.\x.f x (g x)"),
            ("B",     r"\f.\g.\x.f (g x)"),
            ("C",     r"\f.\x.\y.f y x"),
            ("true",  r"\t.\f.t"),
            ("false", r"\t.\f.f"),
            ("and",   r"\p.\q.p q false"),
            ("or",    r"\p.\q.p true q"),
            ("not",   r"\p.p false true"),
            ("if",    r"\b.\t.\e.b t e"),
            ("zero",  r"\f.\x.x"),
            ("succ",  r"\n.\f.\x.f (n f x)"),
            ("plus",  r"\m.\n.\f.\x.m f (n f x)"),
            ("mul",   r"\m.\n.\f.m (n f)"),
            ("pair",  r"\a.\b.\f.f a b"),
            ("fst",   r"\p.p (\a.\b.a)"),
            ("snd",   r"\p.p (\a.\b.b)"),
        ];
        for (name, expr) in &prelude {
            let chars: Vec<char> = expr.chars().collect();
            if let Some(t) = Parser::new(&chars).parse_term() {
                defs.insert(name.to_string(), t);
            }
        }
        Sandbox { defs }
    }

    fn handle(&mut self, tokens: &[&str]) {
        match tokens[0] {
            "def" | "let"   => self.cmd_def(tokens),
            "eval" | "run"  => self.cmd_eval(tokens),
            "step"          => self.cmd_step(tokens),
            "steps"         => self.cmd_steps(tokens),
            "church"        => self.cmd_church(tokens),
            "show"          => self.cmd_show(tokens),
            "list" | "defs" => self.cmd_list(),
            "type"          => self.cmd_type(tokens),
            "free"          => self.cmd_free(tokens),
            "help" | "h" | "?" => self.help(),
            _ => {
                // Try to eval the whole line as an expression
                let full = tokens.join(" ");
                self.cmd_eval_expr(&full);
            }
        }
    }

    fn cmd_def(&mut self, tokens: &[&str]) {
        // def name = expr...
        if tokens.len() < 3 { println!("{RE}Usage: def <name> = <expr>{R}"); return; }
        let name = tokens[1];
        let rest_start = if tokens.get(2) == Some(&"=") { 3 } else { 2 };
        let expr_str = tokens[rest_start..].join(" ");
        match parse(&expr_str, &self.defs) {
            Ok(t) => {
                println!("{G}Defined {C}{name}{R} = {}", t.display());
                self.defs.insert(name.to_string(), t);
            }
            Err(e) => println!("{RE}Parse error: {e}{R}"),
        }
    }

    fn cmd_eval(&self, tokens: &[&str]) {
        let expr_str = tokens[1..].join(" ");
        self.cmd_eval_expr(&expr_str);
    }

    fn cmd_eval_expr(&self, expr_str: &str) {
        match parse(expr_str, &self.defs) {
            Err(e) => println!("{RE}Parse error: {e}{R}"),
            Ok(t) => {
                let (norm, steps) = t.normalize(200);
                println!("{D}Input:{R} {}", t.display());
                if !steps.is_empty() {
                    println!("{D}→ (reduced in {} step{}){R}", steps.len(), if steps.len()==1 {""} else {"s"});
                }
                println!("{B}Normal form:{R} {}", norm.display());
                if norm.is_normal() { println!("{G}  (fully normalized){R}"); }
                else { println!("{Y}  (max steps reached — may diverge){R}"); }
            }
        }
    }

    fn cmd_step(&self, tokens: &[&str]) {
        let expr_str = tokens[1..].join(" ");
        match parse(&expr_str, &self.defs) {
            Err(e) => println!("{RE}Parse error: {e}{R}"),
            Ok(t) => {
                println!("{D}Before:{R} {}", t.display());
                match t.beta_step() {
                    Some(t2) => println!("{G}→β{R}    {}", t2.display()),
                    None => println!("{G}(already in normal form){R}"),
                }
            }
        }
    }

    fn cmd_steps(&self, tokens: &[&str]) {
        let (max_str, expr_start) = if tokens.get(1).map(|s| s.parse::<usize>().is_ok()).unwrap_or(false) {
            (tokens[1].parse::<usize>().unwrap(), 2)
        } else {
            (20, 1)
        };
        let expr_str = tokens[expr_start..].join(" ");
        match parse(&expr_str, &self.defs) {
            Err(e) => println!("{RE}Parse error: {e}{R}"),
            Ok(t) => {
                println!("{D}Step 0:{R} {}", t.display());
                let (_, steps) = t.normalize(max_str);
                for (i, step) in steps.iter().enumerate() {
                    println!("{G}→β{R} {D}step {}:{R} {}", i+1, step.display());
                }
                if steps.is_empty() { println!("{G}(already in normal form){R}"); }
            }
        }
    }

    fn cmd_church(&mut self, tokens: &[&str]) {
        if tokens.len() < 2 { println!("{RE}Usage: church <n> [name]{R}"); return; }
        let n: u64 = match tokens[1].parse() {
            Ok(n) => n, Err(_) => { println!("{RE}Expected a number{R}"); return; }
        };
        let t = church(n);
        println!("{C}Church({n}){R} = {}", t.display());
        // Optionally bind to a name
        let name = tokens.get(2).copied().unwrap_or_default();
        if !name.is_empty() {
            self.defs.insert(name.to_string(), t);
            println!("{G}Bound to '{name}'{R}");
        }
    }

    fn cmd_show(&self, tokens: &[&str]) {
        if tokens.len() < 2 { self.cmd_list(); return; }
        let name = tokens[1];
        match self.defs.get(name) {
            Some(t) => println!("{C}{name}{R} = {}", t.display()),
            None => println!("{RE}Not defined: {name}{R}"),
        }
    }

    fn cmd_list(&self) {
        println!("{B}Definitions:{R}");
        let mut names: Vec<&str> = self.defs.keys().map(|s| s.as_str()).collect();
        names.sort_unstable();
        for name in names {
            println!("  {C}{name}{R}");
        }
    }

    fn cmd_type(&self, tokens: &[&str]) {
        // Rough type classification
        let expr_str = tokens[1..].join(" ");
        match parse(&expr_str, &self.defs) {
            Err(e) => println!("{RE}Parse error: {e}{R}"),
            Ok(t) => {
                let (norm, _) = t.normalize(100);
                println!("{D}Normalized:{R} {}", norm.display());
                println!("{D}Note: untyped λ-calculus — no type inference.{R}");
                println!("{D}Use 'church n' for natural numbers.{R}");
            }
        }
    }

    fn cmd_free(&self, tokens: &[&str]) {
        let expr_str = tokens[1..].join(" ");
        match parse(&expr_str, &self.defs) {
            Err(e) => println!("{RE}Parse error: {e}{R}"),
            Ok(t) => {
                let fv = t.free_vars();
                if fv.is_empty() { println!("{G}No free variables (closed term){R}"); }
                else {
                    let mut fvs: Vec<&str> = fv.iter().map(|s| s.as_str()).collect();
                    fvs.sort_unstable();
                    println!("{Y}Free variables: {}{R}", fvs.join(", "));
                }
            }
        }
    }

    fn help(&self) {
        println!("{B}Lambda Calculus Sandbox{R} — define terms, reduce, explore normal forms\n");
        println!("{B}Define and evaluate:{R}");
        println!("  {C}def f = \\x.x{R}          define the identity combinator");
        println!("  {C}eval f x{R}               evaluate f applied to x");
        println!("  {C}f x{R}                    (shorthand — just type the expression)");
        println!();
        println!("{B}Reduction:{R}");
        println!("  {C}step f a{R}               one β-reduction step");
        println!("  {C}steps f a{R}              show all reduction steps");
        println!("  {C}steps 5 f a{R}            show up to 5 steps");
        println!();
        println!("{B}Church numerals:{R}");
        println!("  {C}church 3{R}               display church numeral 3");
        println!("  {C}church 3 three{R}         bind church 3 to name 'three'");
        println!("  {C}eval plus (church 2) (church 3){R}   compute 2+3");
        println!("  {C}eval mul (church 3) (church 4){R}    compute 3*4");
        println!();
        println!("{B}Inspect:{R}");
        println!("  {C}show f{R}                 show definition of f");
        println!("  {C}list{R}                   list all definitions");
        println!("  {C}free f x{R}               find free variables");
        println!();
        println!("{B}Preloaded:{R} I K S B C true false and or not if zero succ plus mul pair fst snd");
        println!();
        println!("{D}Try: eval S K K x    — (SKK = I by reduction){R}");
        println!("{D}Or:  eval plus (church 2) (church 3) → church 5{R}");
    }
}

fn main() {
    println!("{B}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{R}");
    println!("{B}  Lambda Calculus — Normal Forms{R}");
    println!("{D}  Define terms, reduce β-redexes, find normal forms{R}");
    println!("{B}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{R}");
    println!("{D}Type 'help'. Preloaded: I K S true false plus mul ...{R}\n");

    let mut sb = Sandbox::new();
    loop {
        let line = read_line("λ>");
        if line.is_empty() { continue; }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens[0] {
            "quit" | "q" | "exit" => { println!("{D}Goodbye.{R}"); break; }
            _ => sb.handle(&tokens),
        }
    }
}
