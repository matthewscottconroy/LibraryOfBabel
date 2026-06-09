use common::*;

// ── Propositional Logic Explorer ────────────────────────────────────────────
// Recursive-descent parser for formulas over variables p,q,r,s,t
// Connectives: & (and), | (or), ! (not), -> (implies), <-> (iff)
// Precedence (lowest to highest): <-> -> | & !

#[derive(Clone, Debug)]
enum Expr {
    Var(char),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Implies(Box<Expr>, Box<Expr>),
    Iff(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self, env: &[(char, bool)]) -> bool {
        match self {
            Expr::Var(c) => env.iter().find(|(v, _)| v == c).map(|(_, b)| *b).unwrap_or(false),
            Expr::Not(e) => !e.eval(env),
            Expr::And(a, b) => a.eval(env) && b.eval(env),
            Expr::Or(a, b) => a.eval(env) || b.eval(env),
            Expr::Implies(a, b) => !a.eval(env) || b.eval(env),
            Expr::Iff(a, b) => a.eval(env) == b.eval(env),
        }
    }

    fn vars(&self) -> Vec<char> {
        let mut v = Vec::new();
        self.collect_vars(&mut v);
        v.sort();
        v.dedup();
        v
    }

    fn collect_vars(&self, out: &mut Vec<char>) {
        match self {
            Expr::Var(c) => out.push(*c),
            Expr::Not(e) => e.collect_vars(out),
            Expr::And(a, b) | Expr::Or(a, b) | Expr::Implies(a, b) | Expr::Iff(a, b) => {
                a.collect_vars(out);
                b.collect_vars(out);
            }
        }
    }

    fn display(&self) -> String {
        match self {
            Expr::Var(c) => c.to_string(),
            Expr::Not(e) => format!("!{}", e.display_atom()),
            Expr::And(a, b) => format!("{} & {}", a.display_and(), b.display_and()),
            Expr::Or(a, b) => format!("{} | {}", a.display_or(), b.display_or()),
            Expr::Implies(a, b) => format!("{} -> {}", a.display_or(), b.display()),
            Expr::Iff(a, b) => format!("{} <-> {}", a.display_implies(), b.display_implies()),
        }
    }

    fn display_atom(&self) -> String {
        match self {
            Expr::Var(_) | Expr::Not(_) => self.display(),
            _ => format!("({})", self.display()),
        }
    }
    fn display_and(&self) -> String {
        match self {
            Expr::Var(_) | Expr::Not(_) | Expr::And(_, _) => self.display(),
            _ => format!("({})", self.display()),
        }
    }
    fn display_or(&self) -> String {
        match self {
            Expr::Var(_) | Expr::Not(_) | Expr::And(_, _) | Expr::Or(_, _) => self.display(),
            _ => format!("({})", self.display()),
        }
    }
    fn display_implies(&self) -> String {
        match self {
            Expr::Iff(_, _) => format!("({})", self.display()),
            _ => self.display(),
        }
    }
}

// ── Parser ───────────────────────────────────────────────────────────────────

struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(s: &str) -> Self {
        Parser { input: s.chars().collect(), pos: 0 }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos] == ' ' {
            self.pos += 1;
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.skip_ws();
        if self.pos < self.input.len() {
            let c = self.input[self.pos];
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.skip_ws();
        self.input.get(self.pos).copied()
    }

    fn starts_with(&mut self, pat: &str) -> bool {
        self.skip_ws();
        let chars: Vec<char> = pat.chars().collect();
        if self.pos + chars.len() > self.input.len() { return false; }
        self.input[self.pos..self.pos + chars.len()] == chars[..]
    }

    fn consume(&mut self, pat: &str) -> bool {
        if self.starts_with(pat) {
            self.pos += pat.len();
            true
        } else {
            false
        }
    }

    fn parse_iff(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_implies()?;
        while self.starts_with("<->") {
            self.consume("<->");
            let right = self.parse_implies()?;
            left = Expr::Iff(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_implies(&mut self) -> Result<Expr, String> {
        let left = self.parse_or()?;
        if self.starts_with("->") {
            self.consume("->");
            let right = self.parse_implies()?;
            Ok(Expr::Implies(Box::new(left), Box::new(right)))
        } else {
            Ok(left)
        }
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.peek_char() == Some('|') {
            self.next_char();
            let right = self.parse_and()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_not()?;
        while self.peek_char() == Some('&') {
            self.next_char();
            let right = self.parse_not()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_not(&mut self) -> Result<Expr, String> {
        if self.peek_char() == Some('!') {
            self.next_char();
            let e = self.parse_not()?;
            Ok(Expr::Not(Box::new(e)))
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        match self.peek_char() {
            Some('(') => {
                self.next_char();
                let e = self.parse_iff()?;
                if self.peek_char() != Some(')') {
                    return Err("Expected closing ')'".to_string());
                }
                self.next_char();
                Ok(e)
            }
            Some(c) if "pqrst".contains(c) => {
                self.next_char();
                Ok(Expr::Var(c))
            }
            Some(c) => Err(format!("Unexpected character '{}'", c)),
            None => Err("Unexpected end of formula".to_string()),
        }
    }

    fn parse(&mut self) -> Result<Expr, String> {
        let e = self.parse_iff()?;
        self.skip_ws();
        if self.pos < self.input.len() {
            Err(format!("Unexpected input at position {}: '{}'", self.pos,
                &self.input[self.pos..].iter().collect::<String>()))
        } else {
            Ok(e)
        }
    }
}

fn parse_formula(s: &str) -> Result<Expr, String> {
    Parser::new(s).parse()
}

// ── Truth table utilities ─────────────────────────────────────────────────────

fn all_assignments(vars: &[char]) -> Vec<Vec<(char, bool)>> {
    let n = vars.len();
    (0..(1u32 << n)).map(|mask| {
        vars.iter().enumerate().map(|(i, &c)| (c, (mask >> (n - 1 - i)) & 1 == 1)).collect()
    }).collect()
}

fn truth_table_str(expr: &Expr) -> String {
    let vars = expr.vars();
    let assignments = all_assignments(&vars);
    let mut out = String::new();

    // header
    out.push_str("  ");
    for v in &vars { out.push_str(&format!(" {}  ", v)); }
    out.push_str("│  result\n");
    out.push_str("  ");
    for _ in &vars { out.push_str("────"); }
    out.push_str("┼────────\n");

    let mut all_true = true;
    let mut all_false = true;

    for env in &assignments {
        let result = expr.eval(env);
        if result { all_false = false; } else { all_true = false; }
        out.push_str("  ");
        for (_, v) in env {
            let s = if *v { "T" } else { "F" };
            out.push_str(&format!("  {}  ", s));
        }
        out.push_str("│    ");
        out.push_str(if result { "T\n" } else { "F\n" });
    }

    out.push('\n');
    if all_true {
        out.push_str("  TAUTOLOGY: true under every assignment.\n");
    } else if all_false {
        out.push_str("  CONTRADICTION: false under every assignment.\n");
    } else {
        out.push_str("  CONTINGENT: true for some, false for others.\n");
    }
    out
}

fn print_truth_table(expr: &Expr) {
    let vars = expr.vars();
    let assignments = all_assignments(&vars);

    print!("  ");
    for v in &vars { print!(" {}  ", cyan(&v.to_string())); }
    print!("│  {}", bold("result"));
    println!();
    print!("  ");
    for _ in &vars { print!("────"); }
    print!("┼────────");
    println!();

    let mut all_true = true;
    let mut all_false = true;

    for env in &assignments {
        let result = expr.eval(env);
        if result { all_false = false; } else { all_true = false; }
        print!("  ");
        for (_, v) in env {
            let s = if *v { green("T") } else { red("F") };
            print!("  {}  ", s);
        }
        print!("│    ");
        if result { println!("{}", green("T")); } else { println!("{}", red("F")); }
    }

    println!();
    if all_true {
        print_ok(&format!("This formula is a {}", bold("TAUTOLOGY")));
        print_info("It is true under every assignment.");
    } else if all_false {
        print_ok(&format!("This formula is a {}", bold("CONTRADICTION")));
        print_info("It is false under every assignment.");
    } else {
        print_info("This formula is CONTINGENT (true for some, false for others).");
    }
}

fn is_tautology(expr: &Expr) -> bool {
    let vars = expr.vars();
    all_assignments(&vars).iter().all(|env| expr.eval(env))
}

fn find_satisfying(expr: &Expr) -> Option<Vec<(char, bool)>> {
    let vars = expr.vars();
    all_assignments(&vars).into_iter().find(|env| expr.eval(env))
}

fn are_equivalent(e1: &Expr, e2: &Expr) -> bool {
    let mut vars: Vec<char> = e1.vars().into_iter().chain(e2.vars()).collect();
    vars.sort();
    vars.dedup();
    all_assignments(&vars).iter().all(|env| e1.eval(env) == e2.eval(env))
}

fn extract_implication(expr: &Expr) -> Option<(Expr, Expr)> {
    if let Expr::Implies(p, q) = expr {
        Some((*p.clone(), *q.clone()))
    } else {
        None
    }
}

fn show_help() -> String {
    let mut out = String::new();
    out.push_str("Commands:\n");
    out.push_str("  truth <formula>         print full truth table\n");
    out.push_str("  equiv <f1> / <f2>       check logical equivalence (separate with /)\n");
    out.push_str("  tautology <formula>     check if formula is always true\n");
    out.push_str("  satisfy <formula>       find a satisfying assignment\n");
    out.push_str("  contrapositive <f>      show contrapositive of P -> Q\n");
    out.push_str("  demo                    showcase of key tautologies and equivalences\n");
    out.push_str("  quit                    exit the program\n");
    out.push_str("\nVariables: p, q, r, s, t\n");
    out.push_str("Connectives: ! (not)  & (and)  | (or)  -> (implies)  <-> (iff)\n");
    out.push_str("Precedence: <-> lowest, then ->, then |, then &, then ! highest\n");
    out
}

// ── State helpers ─────────────────────────────────────────────────────────────

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_str(&mut s, "last_formula", "p -> q");
    s
}

// ── run_cmd: returns String output ───────────────────────────────────────────

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    let mut out = String::new();
    match cmd {
        "help" | "h" => {
            out.push_str(&show_help());
        }

        "demo" => {
            out.push_str("=== Logic Demo ===\n\n");
            let demos = [
                ("!p | p",                              "Tautology (law of excluded middle)"),
                ("p & !p",                              "Contradiction"),
                ("(p -> q) & (q -> r) -> (p -> r)",    "Hypothetical syllogism"),
                ("!(p & q) <-> (!p | !q)",              "De Morgan's law"),
                ("p -> q",                              "Implication"),
                ("!q -> !p",                            "Contrapositive of p->q"),
            ];
            for (formula, label) in &demos {
                out.push_str(&format!("--- {} ---\n", label));
                if let Ok(expr) = parse_formula(formula) {
                    out.push_str(&format!("  {}\n", expr.display()));
                    out.push_str(&truth_table_str(&expr));
                }
                out.push('\n');
            }
        }

        "truth" => {
            let formula = args.join(" ");
            if formula.is_empty() {
                return "Usage: truth <formula>\n".to_string();
            }
            match parse_formula(&formula) {
                Err(e) => out.push_str(&format!("Parse error: {}\n", e)),
                Ok(expr) => {
                    state_set_str(state, "last_formula", &formula);
                    out.push_str(&format!("Truth table for: {}\n", expr.display()));
                    out.push_str(&truth_table_str(&expr));
                }
            }
        }

        "tautology" => {
            let formula = args.join(" ");
            if formula.is_empty() {
                return "Usage: tautology <formula>\n".to_string();
            }
            match parse_formula(&formula) {
                Err(e) => out.push_str(&format!("Parse error: {}\n", e)),
                Ok(expr) => {
                    state_set_str(state, "last_formula", &formula);
                    let t = is_tautology(&expr);
                    if t {
                        out.push_str(&format!("{} IS a tautology.\n", expr.display()));
                    } else {
                        out.push_str(&format!("{} is NOT a tautology.\n", expr.display()));
                        if let Some(env) = find_satisfying(&Expr::Not(Box::new(expr.clone()))) {
                            let assignment: String = env.iter()
                                .map(|(c, v)| format!("{}={}", c, if *v { "T" } else { "F" }))
                                .collect::<Vec<_>>().join(", ");
                            out.push_str(&format!("Counterexample: {}\n", assignment));
                        }
                    }
                }
            }
        }

        "satisfy" => {
            let formula = args.join(" ");
            if formula.is_empty() {
                return "Usage: satisfy <formula>\n".to_string();
            }
            match parse_formula(&formula) {
                Err(e) => out.push_str(&format!("Parse error: {}\n", e)),
                Ok(expr) => {
                    match find_satisfying(&expr) {
                        None => out.push_str(&format!("{} is unsatisfiable.\n", expr.display())),
                        Some(env) => {
                            let assignment: String = env.iter()
                                .map(|(c, v)| format!("{}={}", c, if *v { "T" } else { "F" }))
                                .collect::<Vec<_>>().join(", ");
                            out.push_str(&format!("Satisfying assignment: {}\n", assignment));
                        }
                    }
                }
            }
        }

        "equiv" => {
            let full = args.join(" ");
            let parts: Vec<&str> = full.splitn(2, '/').collect();
            if parts.len() != 2 {
                return "Usage: equiv <formula1> / <formula2>\n".to_string();
            }
            let f1 = parts[0].trim();
            let f2 = parts[1].trim();
            match (parse_formula(f1), parse_formula(f2)) {
                (Ok(e1), Ok(e2)) => {
                    let eq = are_equivalent(&e1, &e2);
                    out.push_str(&format!("F1: {}\nF2: {}\n", e1.display(), e2.display()));
                    if eq {
                        out.push_str("LOGICALLY EQUIVALENT.\n");
                    } else {
                        out.push_str("NOT logically equivalent.\n");
                        let mut vars: Vec<char> = e1.vars().into_iter().chain(e2.vars()).collect();
                        vars.sort(); vars.dedup();
                        for env in all_assignments(&vars) {
                            let v1 = e1.eval(&env);
                            let v2 = e2.eval(&env);
                            if v1 != v2 {
                                let assignment: String = env.iter()
                                    .map(|(c, v)| format!("{}={}", c, if *v { "T" } else { "F" }))
                                    .collect::<Vec<_>>().join(", ");
                                out.push_str(&format!("Distinguishing: {}  F1={} F2={}\n",
                                    assignment,
                                    if v1 { "T" } else { "F" },
                                    if v2 { "T" } else { "F" }));
                                break;
                            }
                        }
                    }
                }
                (Err(e), _) => out.push_str(&format!("Parse error in F1: {}\n", e)),
                (_, Err(e)) => out.push_str(&format!("Parse error in F2: {}\n", e)),
            }
        }

        "contrapositive" => {
            let formula = args.join(" ");
            if formula.is_empty() {
                return "Usage: contrapositive <P -> Q>\n".to_string();
            }
            match parse_formula(&formula) {
                Err(e) => out.push_str(&format!("Parse error: {}\n", e)),
                Ok(expr) => {
                    match extract_implication(&expr) {
                        None => out.push_str("Formula must be of the form P -> Q\n"),
                        Some((p, q)) => {
                            let contra = Expr::Implies(
                                Box::new(Expr::Not(Box::new(q.clone()))),
                                Box::new(Expr::Not(Box::new(p.clone()))),
                            );
                            out.push_str(&format!("Original:      {} -> {}\n", p.display(), q.display()));
                            out.push_str(&format!("Contrapositive: !{} -> !{}\n",
                                q.display_atom(), p.display_atom()));
                            let eq = are_equivalent(&expr, &contra);
                            if eq {
                                out.push_str("Equivalent: contrapositive always equals original.\n");
                            }
                        }
                    }
                }
            }
        }

        _ => out.push_str(&format!("Unknown command '{}'. Type 'help'.\n", cmd)),
    }
    out
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { print!("{out}"); }
    true
}

// ── Visualization functions ───────────────────────────────────────────────────

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, args: &[&str], state: &StateMap) {
    let formula_str = if !args.is_empty() {
        args.join(" ")
    } else {
        state_get_str(state, "last_formula").unwrap_or("p -> q").to_string()
    };

    c.title("Truth Table");
    c.subtitle(&format!("Formula: {}", formula_str), 42.0);

    let expr = match parse_formula(&formula_str) {
        Ok(e) => e,
        Err(_) => { c.text(350.0, 250.0, "Parse error", 16.0, colors::RED, "middle"); return; }
    };

    let vars = expr.vars();
    let assignments = all_assignments(&vars);
    let ncols = vars.len() + 1;
    let nrows = assignments.len() + 1;
    let cell_w = 60.0;
    let cell_h = 28.0;
    let x0 = (700.0 - ncols as f64 * cell_w) / 2.0;
    let y0 = 60.0;

    let mut cells: Vec<Vec<String>> = Vec::new();
    let mut header: Vec<String> = vars.iter().map(|v| v.to_string()).collect();
    header.push("result".to_string());
    cells.push(header);

    for env in &assignments {
        let result = expr.eval(env);
        let mut row: Vec<String> = env.iter().map(|(_, v)| if *v { "T" } else { "F" }.to_string()).collect();
        row.push(if result { "T" } else { "F" }.to_string());
        cells.push(row);
    }

    // Color result cells
    let (tw, _th) = c.table(x0, y0, &cells, cell_w, cell_h,
        colors::HEADER_FILL, colors::ROW_NORM, colors::GREY);

    // Overlay colors on result column
    for (ri, env) in assignments.iter().enumerate() {
        let result = expr.eval(env);
        let fill = if result { colors::GREEN } else { colors::RED };
        let x = x0 + (ncols - 1) as f64 * cell_w;
        let y = y0 + (ri + 1) as f64 * cell_h;
        c.rect(x, y, cell_w, cell_h, fill, colors::GREY, 0.5);
        c.text_bold(x + cell_w/2.0, y + cell_h/2.0,
            if result { "T" } else { "F" }, 13.0, colors::WHITE, "middle");
    }

    let _ = tw;

    // Status at bottom
    let all_true = assignments.iter().all(|env| expr.eval(env));
    let all_false = assignments.iter().all(|env| !expr.eval(env));
    let status = if all_true { "TAUTOLOGY" } else if all_false { "CONTRADICTION" } else { "CONTINGENT" };
    let color = if all_true { colors::GREEN } else if all_false { colors::RED } else { colors::BLUE };
    let bot = y0 + nrows as f64 * cell_h + 20.0;
    c.text_bold(350.0, bot, status, 15.0, color, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, args: &[&str], state: &StateMap) {
    let formula_str = if !args.is_empty() {
        args.join(" ")
    } else {
        state_get_str(state, "last_formula").unwrap_or("p -> q").to_string()
    };

    g.node_default("shape", "box");
    g.node_default("style", "rounded,filled");
    g.node_default("fillcolor", "#e0f7fa");

    let expr = match parse_formula(&formula_str) {
        Ok(e) => e,
        Err(_) => {
            g.node("error", &[("label", "Parse error")]);
            return;
        }
    };

    let vars = expr.vars();
    let assignments = all_assignments(&vars);

    // Build nodes for each assignment
    for (i, env) in assignments.iter().enumerate() {
        let result = expr.eval(env);
        let label: String = env.iter()
            .map(|(c, v)| format!("{}={}", c, if *v { "T" } else { "F" }))
            .collect::<Vec<_>>().join(", ");
        let color = if result { "#a5d6a7" } else { "#ef9a9a" };
        g.node(&format!("a{}", i), &[
            ("label", &label),
            ("fillcolor", color),
        ]);
    }
    g.node("formula", &[("label", &formula_str), ("shape", "ellipse"), ("fillcolor", "#b2ebf2")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, args: &[&str], state: &StateMap) {
    let formula_str = if !args.is_empty() {
        args.join(" ")
    } else {
        state_get_str(state, "last_formula").unwrap_or("p -> q").to_string()
    };

    let expr = match parse_formula(&formula_str) {
        Ok(e) => e,
        Err(_) => {
            t.raw("% Parse error");
            return;
        }
    };

    let vars = expr.vars();
    let assignments = all_assignments(&vars);
    let ncols = vars.len() + 1;

    // Build a tabular in TikZ
    let col_spec = "c".repeat(ncols);
    let header: String = vars.iter().map(|v| format!("${}$", v))
        .chain(std::iter::once("Result".to_string()))
        .collect::<Vec<_>>().join(" & ");

    t.raw(&format!("\\node at (0,0) {{\\begin{{tabular}}{{|{}|}}", col_spec));
    t.raw("\\hline");
    t.raw(&format!("{} \\\\", header));
    t.raw("\\hline");
    for env in &assignments {
        let result = expr.eval(env);
        let row: String = env.iter()
            .map(|(_, v)| if *v { "T" } else { "F" }.to_string())
            .chain(std::iter::once(if result { "T" } else { "F" }.to_string()))
            .collect::<Vec<_>>().join(" & ");
        t.raw(&format!("{} \\\\", row));
    }
    t.raw("\\hline");
    t.raw("\\end{tabular}};");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, args: &[&str], state: &StateMap) {
    let formula_str = if !args.is_empty() {
        args.join(" ")
    } else {
        state_get_str(state, "last_formula").unwrap_or("p -> q").to_string()
    };

    let expr = match parse_formula(&formula_str) {
        Ok(e) => e,
        Err(_) => { a.text_at(0, 0, "Parse error"); return; }
    };

    let vars = expr.vars();
    let assignments = all_assignments(&vars);
    let col_w = 4usize;

    // Title
    a.text_at(1, 0, &format!("Truth Table: {}", formula_str));

    // Header
    let mut x = 1i32;
    for v in &vars {
        a.text_at(x, 2, &v.to_string());
        x += col_w as i32;
    }
    a.text_at(x, 2, "res");

    // Separator
    a.hline(1, 3, (vars.len() * col_w + 6) as i32, '-');

    for (ri, env) in assignments.iter().enumerate() {
        let result = expr.eval(env);
        let mut xi = 1i32;
        for (_, v) in env {
            a.text_at(xi, 4 + ri as i32, if *v { "T" } else { "F" });
            xi += col_w as i32;
        }
        a.text_at(xi + 1, 4 + ri as i32, if result { "T" } else { "F" });
    }
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    let app = AppArgs::parse();
    let mut state = if let Some(ref f) = app.load_file {
        load_state(f).unwrap_or_else(|_| default_state())
    } else { default_state() };

    match &app.mode {
        AppMode::Run { cmd, args } => {
            let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            let out = match app.format {
                OutputFormat::Svg => {
                    let mut c = SvgCanvas::new(700.0, 500.0);
                    visualize_svg(&mut c, cmd, &args_ref, &state);
                    c.build()
                }
                OutputFormat::Dot => {
                    let mut g = DotGraph::digraph("ch01");
                    visualize_dot(&mut g, cmd, &args_ref, &state);
                    g.build()
                }
                OutputFormat::Tex => {
                    let mut t = TikzDoc::standalone();
                    visualize_tex(&mut t, cmd, &args_ref, &state);
                    t.build()
                }
                OutputFormat::Ascii => {
                    let mut a = AsciiCanvas::new(80, 30);
                    visualize_ascii(&mut a, cmd, &args_ref, &state);
                    a.render()
                }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 1", "Logic and the Art of Proof",
                "Explore propositional logic interactively");
            print_info("Type 'help' for commands.");
            print_note("Try: truth !p | p   (a tautology)");
            print_note("Try: truth p & !p   (a contradiction)");
            repl("ch1> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
