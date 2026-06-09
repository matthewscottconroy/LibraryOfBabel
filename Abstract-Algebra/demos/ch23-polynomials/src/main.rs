use common::*;

fn help_text() -> String {
    help_string(&[
        ("add <f...> / <g...>",     "add polynomials (space-separated, constant first)"),
        ("mul <f...> / <g...>",     "multiply polynomials"),
        ("div <f...> / <g...>",     "divide polynomials; show quotient and remainder"),
        ("gcd <f...> / <g...>",     "polynomial GCD via Euclidean algorithm"),
        ("irreducible <f...> <p>",  "test irreducibility of f mod prime p"),
        ("roots_mod <f...> <p>",    "find all roots of f in Z/pZ"),
        ("adjoin <f...>",           "describe Q[x]/(f): degree, arithmetic"),
        ("factor_zx <f...>",        "factor f over Z via rational root theorem"),
        ("cyclotomic <n>",          "print the n-th cyclotomic polynomial Phi_n(x)"),
        ("demo",                    "showcase polynomial examples"),
        ("help",                    "show this help"),
        ("quit",                    "exit"),
    ])
}

fn split_two(args: &[&str]) -> Option<(Vec<i64>, Vec<i64>)> {
    if let Some(pos) = args.iter().position(|&s| s == "/") {
        let f: Vec<i64> = args[..pos].iter().filter_map(|s| s.parse().ok()).collect();
        let g: Vec<i64> = args[pos+1..].iter().filter_map(|s| s.parse().ok()).collect();
        if f.is_empty() || g.is_empty() { return None; }
        return Some((f, g));
    }
    if args.len() < 2 { return None; }
    for mid in 1..args.len() {
        let f: Vec<i64> = args[..mid].iter().filter_map(|s| s.parse().ok()).collect();
        let g: Vec<i64> = args[mid..].iter().filter_map(|s| s.parse().ok()).collect();
        if !f.is_empty() && !g.is_empty() { return Some((f, g)); }
    }
    None
}

fn cmd_add(args: &[&str]) -> String {
    let (fc, gc) = match split_two(args) {
        Some(x) => x,
        None => return "Usage: add <f coeffs...> / <g coeffs...>".to_string(),
    };
    let f = Poly::new(fc); let g = Poly::new(gc);
    let h = f.add(&g);
    format!("f(x) = {}\ng(x) = {}\nf+g = {}\n", f, g, h)
}

fn cmd_mul(args: &[&str]) -> String {
    let (fc, gc) = match split_two(args) {
        Some(x) => x,
        None => return "Usage: mul <f coeffs...> / <g coeffs...>".to_string(),
    };
    let f = Poly::new(fc); let g = Poly::new(gc);
    let h = f.mul(&g);
    format!("f(x) = {}\ng(x) = {}\nf*g = {}\n", f, g, h)
}

fn cmd_div(args: &[&str]) -> String {
    let (fc, gc) = match split_two(args) {
        Some(x) => x,
        None => return "Usage: div <f coeffs...> / <g coeffs...>".to_string(),
    };
    let f = Poly::new(fc); let g = Poly::new(gc);
    let (q, r) = f.div_rem(&g);
    let mut out = format!("f(x) = {}\ng(x) = {}\nquotient = {}\nremainder = {}\n", f, g, q, r);
    if r.coeffs.is_empty() { out.push_str("g divides f exactly.\n"); }
    out
}

fn cmd_gcd(args: &[&str]) -> String {
    let sep = args.iter().position(|&s| s == "/");
    let (f_args, g_args) = match sep {
        Some(pos) => (&args[..pos], &args[pos+1..]),
        None => return "Usage: gcd <f coeffs> / <g coeffs>".to_string(),
    };
    let fc: Vec<i64> = f_args.iter().filter_map(|s| s.parse().ok()).collect();
    let gc: Vec<i64> = g_args.iter().filter_map(|s| s.parse().ok()).collect();
    if fc.is_empty() || gc.is_empty() { return "Need coefficients on both sides.".to_string(); }
    let mut f = Poly::new(fc); let mut g = Poly::new(gc);
    let mut out = format!("Polynomial GCD\n  f(x) = {}\n  g(x) = {}\n", f, g);
    let mut step = 0;
    while !g.coeffs.is_empty() {
        let (q, r) = f.div_rem(&g);
        out.push_str(&format!("  step {}: ({}) = ({}) * ({}) + ({})\n", step+1, f, q, g, r));
        f = g; g = r; step += 1;
    }
    out.push_str(&format!("  gcd(f,g) = {}\n", f));
    out
}

fn cmd_irreducible(args: &[&str]) -> String {
    if args.len() < 2 { return "Usage: irreducible <f coeffs...> <p>".to_string(); }
    let p = match args.last().and_then(|s| s.parse::<u64>().ok()) {
        Some(x) => x, None => return "Last arg must be prime p.".to_string(),
    };
    if !is_prime(p) { return format!("{} is not prime.", p); }
    let coeffs: Vec<i64> = args[..args.len()-1].iter().filter_map(|s| s.parse().ok()).collect();
    let f = Poly::new(coeffs);
    let deg = match f.degree() { Some(d) => d, None => return "Zero polynomial.".to_string() };
    let roots: Vec<u64> = (0..p).filter(|&x| f.eval_mod(x as i64, p as i64) == 0).collect();
    let mut out = format!("Irreducibility of f(x)={} mod {}\n", f, p);
    if roots.is_empty() { out.push_str("  No roots mod p.\n"); }
    else { out.push_str(&format!("  Roots found: {:?}\n", roots)); }
    if deg <= 3 {
        if roots.is_empty() {
            out.push_str(&format!("  IRREDUCIBLE over Z/{}Z (deg<=3, no roots).\n", p));
        } else {
            out.push_str("  REDUCIBLE mod p (has a root).\n");
        }
    } else {
        let irred = f.is_irreducible_mod(p);
        out.push_str(&format!("  Simplified check: {}\n", if irred { "IRREDUCIBLE" } else { "REDUCIBLE" }));
    }
    out
}

fn cmd_roots_mod(args: &[&str]) -> String {
    if args.len() < 2 { return "Usage: roots_mod <f coeffs...> <p>".to_string(); }
    let p = match args.last().and_then(|s| s.parse::<u64>().ok()) {
        Some(x) => x, None => return "Last arg must be prime p.".to_string(),
    };
    let coeffs: Vec<i64> = args[..args.len()-1].iter().filter_map(|s| s.parse().ok()).collect();
    let f = Poly::new(coeffs);
    let roots: Vec<u64> = (0..p).filter(|&x| f.eval_mod(x as i64, p as i64) == 0).collect();
    let mut out = format!("Roots of f(x)={} in Z/{}Z\n", f, p);
    if roots.is_empty() { out.push_str("  No roots.\n"); }
    else {
        for &r in &roots { out.push_str(&format!("  x={} is a root: f({}) = 0 (mod {})\n", r, r, p)); }
    }
    out.push_str(&format!("  Number of roots: {}\n", roots.len()));
    out
}

fn cmd_adjoin(args: &[&str]) -> String {
    let coeffs: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
    if coeffs.is_empty() { return "Provide coefficients of f (constant first).".to_string(); }
    let f = Poly::new(coeffs);
    let deg = match f.degree() { Some(d) => d, None => return "Zero polynomial.".to_string() };
    if deg < 2 { return "f should have degree >= 2.".to_string(); }
    let basis: Vec<String> = (0..deg).map(|i| match i { 0 => "1".to_string(), 1 => "a".to_string(), _ => format!("a^{}", i) }).collect();
    let mut out = format!("Q[x] / ({})\n  f(x) = {}, deg = {}\n  Q-basis: {{{}}}\n  Degree [Q(a):Q] = {}\n",
        f, f, deg, basis.join(", "), deg);
    let lc = f.leading();
    let lc_inv = if lc == 1 { "".to_string() } else { format!("(1/{})*", lc) };
    let rhs: Vec<String> = f.coeffs[..deg].iter().enumerate()
        .filter(|(_, &c)| c != 0)
        .map(|(i, &c)| {
            let term = match i { 0 => "1".to_string(), 1 => "a".to_string(), _ => format!("a^{}", i) };
            if c == 1 { term } else if c == -1 { format!("-{}", term) } else { format!("{}*{}", c, term) }
        }).collect();
    if rhs.is_empty() { out.push_str(&format!("  a^{} = 0\n", deg)); }
    else { out.push_str(&format!("  a^{} = {}({})\n", deg, lc_inv, rhs.join(" + "))); }
    if deg == 2 {
        let b = f.coeffs.get(1).copied().unwrap_or(0);
        let c_val = f.coeffs.get(0).copied().unwrap_or(0);
        let disc = b*b - 4*lc*c_val;
        out.push_str(&format!("  Discriminant = {}{}\n", disc,
            if disc < 0 { " (complex extension)" } else if disc > 0 { " (real extension)" } else { " (repeated root)" }));
    }
    out
}

fn cmd_factor_zx(args: &[&str]) -> String {
    let coeffs: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
    if coeffs.is_empty() { return "Provide coefficients (constant first).".to_string(); }
    let f = Poly::new(coeffs);
    let deg = match f.degree() { Some(d) => d, None => return "Zero polynomial.".to_string() };
    let leading = f.leading();
    let constant = f.coeffs[0];
    let mut out = format!("Factoring f(x)={} over Z\n  leading={}, constant={}\n", f, leading, constant);
    let mut remaining = f.clone();
    let mut found_factors: Vec<Poly> = Vec::new();
    for x in -20i64..=20 {
        if remaining.coeffs.is_empty() { break; }
        if remaining.eval(x) == 0 {
            out.push_str(&format!("  Root: x = {}\n", x));
            let linear = Poly::new(vec![-x, 1]);
            let (q, r) = remaining.div_rem(&linear);
            if r.coeffs.is_empty() {
                found_factors.push(linear.clone());
                remaining = q;
            }
        }
    }
    if found_factors.is_empty() {
        out.push_str("  No integer roots found.\n");
        if deg <= 3 { out.push_str("  Irreducible over Q (deg<=3, no rational roots).\n"); }
    } else {
        let factor_strs: Vec<String> = found_factors.iter().map(|p| format!("({})", p)).collect();
        let rest = if remaining.coeffs.is_empty() || remaining == Poly::new(vec![1]) {
            String::new()
        } else { format!(" * ({})", remaining) };
        out.push_str(&format!("  f(x) = {}{}\n", factor_strs.join(" * "), rest));
    }
    out
}

fn cyclotomic(n: u64) -> Poly {
    if n == 1 { return Poly::new(vec![-1, 1]); }
    let mut phi: Vec<Poly> = vec![Poly::zero(); (n + 1) as usize];
    phi[1] = Poly::new(vec![-1, 1]);
    for k in 2..=(n as usize) {
        let mut xk_minus_1 = vec![0i64; k + 1];
        xk_minus_1[0] = -1; xk_minus_1[k] = 1;
        let mut num = Poly::new(xk_minus_1);
        let divs = divisors(k as u64);
        for &d in divs.iter().filter(|&&d| d < k as u64) {
            let (q, _r) = num.div_rem(&phi[d as usize]);
            num = q;
        }
        phi[k] = num;
    }
    phi[n as usize].clone()
}

fn cmd_cyclotomic(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    if n == 0 { return "n must be positive".to_string(); }
    if n > 30 { return "n too large (max 30)".to_string(); }
    let phi = cyclotomic(n);
    let prim: Vec<u64> = (1..n).filter(|&k| gcd(k as i64, n as i64) == 1).collect();
    let divs = divisors(n);
    let factor_str: Vec<String> = divs.iter().map(|&d| format!("Phi_{}(x)", d)).collect();
    let mut out = format!("Cyclotomic Polynomial Phi_{}(x)\n  Phi_{} = {}\n  deg = {}\n  phi({}) = {}\n",
        n, n, phi, phi.degree().unwrap_or(0), n, euler_totient(n));
    out.push_str(&format!("  Primitive roots: k in {:?}\n", prim));
    out.push_str(&format!("  x^{} - 1 = {}\n", n, factor_str.join(" * ")));
    if is_prime(n) {
        out.push_str(&format!("  n={} prime: Phi_n irreducible over Q by Eisenstein at p=n.\n", n));
    }
    out
}

fn cmd_demo() -> String {
    let mut out = String::from("Demo: Polynomial Rings\n");
    out.push_str(&cmd_mul(&["-1", "0", "1", "/", "-1", "0", "1"]));
    out.push_str(&cmd_cyclotomic(&["5"]));
    out.push_str(&cmd_factor_zx(&["-6", "1", "1"]));
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_ints(&mut s, "last_poly", &[-1, 0, 1]);
    state_set_int(&mut s, "last_n", 5);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "add"         => { let r = cmd_add(args); let fc: Vec<i64> = args.iter().take_while(|&&s| s != "/").filter_map(|s| s.parse().ok()).collect(); if !fc.is_empty() { state_set_ints(state,"last_poly",&fc); } r }
        "mul"         => { let r = cmd_mul(args); let fc: Vec<i64> = args.iter().take_while(|&&s| s != "/").filter_map(|s| s.parse().ok()).collect(); if !fc.is_empty() { state_set_ints(state,"last_poly",&fc); } r }
        "div"         => { let r = cmd_div(args); let fc: Vec<i64> = args.iter().take_while(|&&s| s != "/").filter_map(|s| s.parse().ok()).collect(); if !fc.is_empty() { state_set_ints(state,"last_poly",&fc); } r }
        "gcd"         => cmd_gcd(args),
        "irreducible" => cmd_irreducible(args),
        "roots_mod"   => cmd_roots_mod(args),
        "adjoin"      => { let r = cmd_adjoin(args);
            let coeffs: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
            if !coeffs.is_empty() { state_set_ints(state, "last_poly", &coeffs); }
            r }
        "factor_zx"   => { let r = cmd_factor_zx(args); let coeffs: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect(); if !coeffs.is_empty() { state_set_ints(state,"last_poly",&coeffs); } r }
        "cyclotomic"  => { let r = cmd_cyclotomic(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "demo"        => cmd_demo(),
        "help" | "h"  => help_text(),
        _ => format!("Unknown command '{}'. Type 'help'.", cmd),
    }
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { println!("{out}"); }
    true
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    // Show roots of last polynomial as dots on real line, and factor tree
    let coeffs = state_get_ints(state, "last_poly").unwrap_or_else(|| vec![-1, 0, 1]);
    let f = Poly::new(coeffs);
    let deg = f.degree().unwrap_or(0);
    c.title(&format!("Polynomial: f(x) = {}", f));
    c.subtitle(&format!("degree = {}", deg), 44.0);
    // Integer roots
    let roots: Vec<i64> = (-10..=10i64).filter(|&x| f.eval(x) == 0).collect();
    let cx_line = c.width / 2.0;
    let y_line = 180.0;
    c.line(50.0, y_line, c.width - 50.0, y_line, colors::GREY, 1.5);
    c.text(cx_line, y_line + 20.0, "integer roots on number line", 11.0, colors::GREY, "middle");
    // Draw tick marks
    for x_val in -8i64..=8 {
        let px = cx_line + x_val as f64 * 40.0;
        c.line(px, y_line - 5.0, px, y_line + 5.0, colors::GREY, 1.0);
        c.text(px, y_line + 14.0, &x_val.to_string(), 9.0, colors::GREY, "middle");
    }
    for &r in &roots {
        if r.abs() <= 8 {
            let px = cx_line + r as f64 * 40.0;
            c.circle(px, y_line, 8.0, colors::RED, colors::DARK, 1.5);
            c.text(px, y_line - 18.0, &format!("x={}", r), 10.0, colors::RED, "middle");
        }
    }
    if roots.is_empty() {
        c.text(cx_line, y_line - 25.0, "No integer roots in [-8,8]", 11.0, colors::GREY, "middle");
    }
    // Cyclotomic Phi_5
    let phi5 = cyclotomic(5);
    c.text(cx_line, y_line + 55.0, &format!("Phi_5(x) = {}", phi5), 11.0, colors::DARK, "middle");
    // Factor visualization
    if !roots.is_empty() {
        let mut remaining = f.clone();
        let mut factors_str: Vec<String> = Vec::new();
        for &r in &roots {
            let linear = Poly::new(vec![-r, 1]);
            let (q, rem) = remaining.div_rem(&linear);
            if rem.coeffs.is_empty() {
                factors_str.push(format!("(x - {})", r));
                remaining = q;
            }
        }
        if !remaining.coeffs.is_empty() && remaining != Poly::new(vec![1]) {
            factors_str.push(format!("({})", remaining));
        }
        c.text(cx_line, y_line + 80.0,
               &format!("f(x) = {}", factors_str.join(" * ")),
               11.0, colors::DARK, "middle");
    }
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let coeffs = state_get_ints(state, "last_poly").unwrap_or_else(|| vec![-1, 0, 1]);
    let f = Poly::new(coeffs);
    g.graph_attr("label", &format!("Polynomial f(x)={}", f));
    g.node_default("shape", "box");
    g.node_default("style", "rounded,filled");
    g.node_default("fillcolor", "#b2ebf2");
    g.node("poly", &[("label", &format!("f(x)={}", f)), ("shape", "ellipse"), ("fillcolor", "#ffd54f")]);
    let roots: Vec<i64> = (-10..=10i64).filter(|&x| f.eval(x) == 0).collect();
    if roots.is_empty() {
        g.node("irred", &[("label", "No integer roots\n(likely irreducible)")]);
        g.edge("poly", "irred", &[]);
    } else {
        for &r in &roots {
            let id = format!("root_{}", r);
            let label = format!("x = {}", r);
            g.node(&id, &[("label", &label), ("fillcolor", "#ef5350"), ("fontcolor", "white")]);
            g.edge("poly", &id, &[("label", "root")]);
        }
    }
    // Cyclotomic
    for n in [3u64, 4, 5] {
        let phi = cyclotomic(n);
        let id = format!("phi{}", n);
        g.node(&id, &[("label", &format!("Phi_{}={}", n, phi))]);
        g.node("cyclo_group", &[("label", "Cyclotomic\nPolynomials"), ("shape", "ellipse")]);
        g.edge("cyclo_group", &id, &[]);
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let coeffs = state_get_ints(state, "last_poly").unwrap_or_else(|| vec![-1, 0, 1]);
    let f = Poly::new(coeffs);
    t.raw(&format!("  % Polynomial f(x)={}", f));
    let escaped = f.to_string().replace("x^", "x^{").replace(" +", "}+").replace(" -", "}-")
        .replace("x^{", "x^{"); // already handled
    t.node_math("poly", 0.0, 0.0, &format!("f(x)={}", escaped), "draw,ellipse,fill=yellow!30,minimum width=3cm");
    let roots: Vec<i64> = (-10..=10i64).filter(|&x| f.eval(x) == 0).collect();
    for (i, &r) in roots.iter().enumerate() {
        let x = (i as f64 - (roots.len() as f64 - 1.0)/2.0) * 2.5;
        t.node_math(&format!("r{}", i), x, -2.5, &format!("x={}", r), "draw,circle,fill=red!30");
        t.arrow("poly", &format!("r{}", i), "root", "gray");
    }
    if roots.is_empty() {
        t.node_math("irred", 0.0, -2.5, "\\text{irreducible}", "draw,rectangle,fill=cyan!20,rounded corners");
        t.arrow("poly", "irred", "", "gray");
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let coeffs = state_get_ints(state, "last_poly").unwrap_or_else(|| vec![-1, 0, 1]);
    let f = Poly::new(coeffs);
    a.text_at(2, 0, &format!("Polynomial: f(x) = {}", f));
    a.hline(2, 1, 50, '-');
    let roots: Vec<i64> = (-10..=10i64).filter(|&x| f.eval(x) == 0).collect();
    if roots.is_empty() {
        a.text_at(2, 2, "No integer roots in [-10,10]");
    } else {
        let roots_str: Vec<String> = roots.iter().map(|x| x.to_string()).collect();
        a.text_at(2, 2, &format!("Integer roots: {}", roots_str.join(", ")));
    }
    a.text_at(2, 3, "Number line: -3 -2 -1  0  1  2  3");
    // simple number line
    a.hline(2, 4, 36, '-');
    for (col, x_val) in (-3i64..=3).enumerate() {
        let px = 2 + col * 5;
        if roots.contains(&x_val) {
            a.text_at(px as i32, 4, "X");
        } else {
            a.text_at(px as i32, 4, "|");
        }
    }
    a.text_at(2, 6, &format!("Phi_5(x) = {}", cyclotomic(5)));
    a.text_at(2, 7, &format!("Phi_6(x) = {}", cyclotomic(6)));
}

fn main() {
    let app = AppArgs::parse();
    let mut state = if let Some(ref f) = app.load_file {
        load_state(f).unwrap_or_else(|_| default_state())
    } else { default_state() };
    match &app.mode {
        AppMode::Run { cmd, args } => {
            let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            let out = match app.format {
                OutputFormat::Svg   => { let mut c = SvgCanvas::new(700.0, 500.0); visualize_svg(&mut c, cmd, &args_ref, &state); c.build() }
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch23"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 23", "Polynomial Rings",
                         "Arithmetic in Z[x] and Q[x], irreducibility, cyclotomic polynomials");
            println!("{}", help_text());
            repl("ch23> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
