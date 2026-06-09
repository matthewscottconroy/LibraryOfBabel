use common::*;

fn help_text() -> String {
    help_string(&[
        ("smith <r> <c> <entries>",      "Smith normal form, step-by-step, read invariant factors"),
        ("classify_module <d...>",        "write module from invariant factors as direct sum"),
        ("rational_form <char_poly...>",  "rational canonical form from characteristic polynomial"),
        ("jordan_form <lambda> <mult> <exp>","Jordan form for eigenvalue, multiplicity, min poly exp"),
        ("compare <d...> / <e...>",       "are two f.g. abelian groups (given by inv. factors) isomorphic?"),
        ("tensor_zn <n> <m>",             "Z/nZ tensor_Z Z/mZ = Z/gcd(n,m)Z"),
        ("demo",                          "showcase structure theorem examples"),
        ("help",                          "show this help"),
        ("quit",                          "exit"),
    ])
}

fn cmd_smith(args: &[&str]) -> String {
    if args.len() < 3 {
        return "Usage: smith <rows> <cols> <space-separated entries>".to_string();
    }
    let rows = match args[0].parse::<usize>() { Ok(r) => r, Err(_) => return "rows must be integer".to_string() };
    let cols = match args[1].parse::<usize>() { Ok(c) => c, Err(_) => return "cols must be integer".to_string() };
    let entries: Vec<i64> = args[2..].iter().filter_map(|s| s.parse().ok()).collect();
    if entries.len() != rows * cols {
        return format!("Expected {} entries, got {}", rows * cols, entries.len());
    }
    let mat = Mat::new(rows, cols, entries);
    let mut out = format!("Input Matrix A:\n{}", mat);
    let (smith, ops) = mat.smith_normal_form();
    out.push_str("Row/Column Operations:\n");
    for (i, op) in ops.iter().enumerate() {
        out.push_str(&format!("  step {:>2}: {}\n", i+1, op));
    }
    out.push_str(&format!("Smith Normal Form D:\n{}", smith));
    let min_dim = rows.min(cols);
    let diag: Vec<i64> = (0..min_dim).map(|k| smith[(k,k)]).collect();
    let inv_factors: Vec<i64> = diag.iter().copied().filter(|&v| v > 1).collect();
    let ones: usize = diag.iter().filter(|&&v| v == 1).count();
    let zeros: usize = diag.iter().filter(|&&v| v == 0).count();
    out.push_str(&format!("Diagonal: {}\n", diag.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")));
    if ones > 0 { out.push_str(&format!("  {} one(s): contribute free Z factors.\n", ones)); }
    if !inv_factors.is_empty() {
        out.push_str(&format!("  Invariant factors (>1): {}\n", inv_factors.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" | ")));
    }
    let mut parts: Vec<String> = Vec::new();
    if zeros > 0 { parts.push(format!("Z^{}", zeros)); }
    for &d in &inv_factors { parts.push(format!("Z/{}", d)); }
    if parts.is_empty() { parts.push("0".to_string()); }
    out.push_str(&format!("Coker(A) = {}\n", parts.join(" + ")));
    if inv_factors.len() >= 2 {
        let ok = inv_factors.windows(2).all(|w| w[1] % w[0] == 0);
        if ok { out.push_str("Divisibility chain d_1 | d_2 | ... verified.\n"); }
    }
    out
}

fn cmd_classify_module(args: &[&str]) -> String {
    if args.is_empty() { return "Provide invariant factors d1 d2 ...".to_string(); }
    let factors: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
    if factors.is_empty() { return "No valid integers.".to_string(); }
    let mut out = format!("Module from Invariant Factors: {}\n",
        factors.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" | "));
    for i in 1..factors.len() {
        if factors[i-1] > 0 && factors[i] > 0 && factors[i] % factors[i-1] != 0 {
            out.push_str(&format!("  Warning: d_{} does not divide d_{}\n", i, i+1));
        }
    }
    let parts: Vec<String> = factors.iter().filter_map(|&d| match d {
        0 => Some("Z".to_string()),
        1 => None,
        _ => Some(format!("Z/{}", d)),
    }).collect();
    let result = if parts.is_empty() { "0".to_string() } else { parts.join(" + ") };
    out.push_str(&format!("M = {}\n", result));
    if factors.iter().all(|&d| d > 0) {
        let order: i64 = factors.iter().filter(|&&d| d > 1).product();
        if order > 0 { out.push_str(&format!("|M| = {}\n", order)); }
    }
    // Primary decomposition
    out.push_str("Primary decomposition:\n");
    let mut primary: Vec<String> = Vec::new();
    for &d in &factors {
        if d <= 1 { continue; }
        for (p, e) in factor(d as u64) { primary.push(format!("Z/{}", p.pow(e))); }
    }
    if primary.is_empty() { out.push_str("  No torsion components.\n"); }
    else { out.push_str(&format!("  {}\n", primary.join(" + "))); }
    out.push_str("Structure Theorem: f.g. module over PID = Z^r + Z/d_1 + ... + Z/d_k, d_i | d_{i+1}.\n");
    out
}

fn companion_matrix(poly: &Poly) -> Option<Mat> {
    let deg = poly.degree()?;
    if deg == 0 { return None; }
    let n = deg;
    let mut c = Mat::zero(n, n);
    for i in 1..n { c[(i, i-1)] = 1; }
    let lc = poly.leading();
    for i in 0..n {
        let coeff = poly.coeffs.get(i).copied().unwrap_or(0);
        c[(i, n-1)] = -coeff / lc;
    }
    Some(c)
}

fn cmd_rational_form(args: &[&str]) -> String {
    let coeffs: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
    if coeffs.is_empty() { return "Provide characteristic polynomial coefficients (constant first).".to_string(); }
    let chi = Poly::new(coeffs);
    let deg = match chi.degree() { Some(d) => d, None => return "Zero polynomial.".to_string() };
    let mut out = format!("Rational Canonical Form\n  char poly X(L) = {}, deg = {}\n", chi, deg);
    out.push_str("  Blocks = companion matrices of invariant factors f_1 | f_2 | ... | f_k.\n");
    out.push_str("  Product f_1 * ... * f_k = X(L).\n");
    out.push_str("Companion Matrix of X(L):\n");
    match companion_matrix(&chi) {
        Some(c) => { out.push_str(&format!("{}", c)); }
        None => { out.push_str("  (undefined for this polynomial)\n"); }
    }
    out.push_str("  Cayley-Hamilton: X(A) = 0 for any matrix A.\n");
    out.push_str("  Rational form defined over any field (no algebraic closure needed).\n");
    out
}

fn cmd_jordan_form(args: &[&str]) -> String {
    let lambda = match parse_int(args, 0, "eigenvalue") { Some(x) => x, None => return String::new() };
    let mult   = match parse_uint(args, 1, "multiplicity") { Some(x) => x as usize, None => return String::new() };
    let exp    = match parse_uint(args, 2, "min_poly_exp") { Some(x) => x as usize, None => return String::new() };
    if exp > mult { return "Min poly exponent cannot exceed algebraic multiplicity.".to_string(); }
    if exp == 0 { return "Exponent must be at least 1.".to_string(); }
    let mut out = format!("Jordan Form for eigenvalue lambda = {}\n", lambda);
    out.push_str(&format!("  Algebraic multiplicity: {}\n  Exponent in min poly: {}\n", mult, exp));
    out.push_str(&format!("Largest Jordan block J_{}({}):\n", exp, lambda));
    for i in 0..exp {
        let mut row = String::from("  | ");
        for j in 0..exp {
            if i == j { row.push_str(&format!("{:>4}", lambda)); }
            else if j == i + 1 { row.push_str("   1"); }
            else { row.push_str("   0"); }
        }
        row.push_str(" |");
        out.push_str(&row);
        out.push('\n');
    }
    out.push_str(&format!("  Block size = {} = exp in min poly.\n", exp));
    if mult > exp { out.push_str(&format!("  Remaining {} units of mult fill smaller blocks.\n", mult - exp)); }
    out.push_str("  Jordan form exists over algebraically closed fields (e.g., C).\n");
    out.push_str("  Over non-closed fields, use rational canonical form.\n");
    out
}

fn cmd_compare(args: &[&str]) -> String {
    let sep = args.iter().position(|&s| s == "/");
    let (d_args, e_args) = match sep {
        Some(pos) => (&args[..pos], &args[pos+1..]),
        None => return "Usage: compare <d1 d2 ...> / <e1 e2 ...>".to_string(),
    };
    let mut ds: Vec<i64> = d_args.iter().filter_map(|s| s.parse().ok()).collect();
    let mut es: Vec<i64> = e_args.iter().filter_map(|s| s.parse().ok()).collect();
    if ds.is_empty() || es.is_empty() { return "Need factors on both sides.".to_string(); }
    ds.retain(|&x| x != 1); ds.sort();
    es.retain(|&x| x != 1); es.sort();
    let d_str = if ds.is_empty() { "0".to_string() } else {
        ds.iter().map(|&d| if d == 0 { "Z".to_string() } else { format!("Z/{}", d) }).collect::<Vec<_>>().join(" + ")
    };
    let e_str = if es.is_empty() { "0".to_string() } else {
        es.iter().map(|&e| if e == 0 { "Z".to_string() } else { format!("Z/{}", e) }).collect::<Vec<_>>().join(" + ")
    };
    let mut out = format!("Comparing f.g. abelian groups:\n  G1 = {}\n  G2 = {}\n", d_str, e_str);
    if ds == es {
        out.push_str("  G1 = G2 -- ISOMORPHIC (same invariant factors).\n");
    } else {
        let ord_d: i64 = ds.iter().filter(|&&x| x > 0).product::<i64>().max(1);
        let ord_e: i64 = es.iter().filter(|&&x| x > 0).product::<i64>().max(1);
        if ord_d != ord_e {
            out.push_str(&format!("  Different orders ({} vs {}): NOT isomorphic.\n", ord_d, ord_e));
        } else {
            out.push_str("  Different invariant factors: NOT isomorphic.\n");
        }
    }
    out.push_str("  Two f.g. abelian groups isomorphic <=> same invariant factors.\n");
    out
}

fn cmd_tensor_zn(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    let m = match parse_uint(args, 1, "m") { Some(x) => x, None => return String::new() };
    if n < 1 || m < 1 { return "n and m must be positive".to_string(); }
    let g = gcd(n as i64, m as i64).unsigned_abs();
    let mut out = format!("Z/{}Z tensor_Z Z/{}Z\n", n, m);
    out.push_str("  From resolution: 0 -> Z -[xn]-> Z -> Z/nZ -> 0\n");
    out.push_str("  Tensor with Z/mZ (right-exact):\n");
    out.push_str(&format!("    Z/{}Z -[x{}]-> Z/{}Z -> Z/{}Z tensor Z/{}Z -> 0\n", m, n, m, n, m));
    out.push_str(&format!("  Image of x{} = n*Z/mZ = gcd(n,m)*Z/mZ = {}*Z/{}Z\n", n, g, m));
    out.push_str(&format!("  Cokernel = Z/{}Z / {}*(Z/{}Z) = Z/{}Z\n", m, n, m, g));
    out.push_str(&format!("  Result: Z/{}Z tensor Z/{}Z = Z/{}Z\n", n, m, g));
    out.push_str(&format!("  gcd({},{}) = {}\n", n, m, g));
    if g <= 12 {
        out.push_str(&format!("  Elements: {} cosets [0],[1],...,[{}]\n", g, g-1));
    }
    out.push_str("  Tensor 'takes the gcd'. Tor^1(Z/nZ, Z/mZ) = Z/gcd(n,m)Z too.\n");
    out
}

fn cmd_demo() -> String {
    let mut out = String::from("Demo: Structure Theorem\n");
    out.push_str(&cmd_smith(&["2", "3", "2", "4", "6", "4", "8"]));
    out.push_str(&cmd_classify_module(&["2", "6"]));
    out.push_str(&cmd_tensor_zn(&["4", "6"]));
    out.push_str(&cmd_compare(&["4", "3", "/", "12"]));
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_ints(&mut s, "inv_factors", &[2, 6]);
    state_set_int(&mut s, "last_n", 6);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "smith"           => { let r = cmd_smith(args); if let (Ok(rows), Ok(cols)) = (args.get(0).unwrap_or(&"").parse::<i64>(), args.get(1).unwrap_or(&"").parse::<i64>()) { state_set_int(state,"last_n",rows * cols); } r }
        "classify_module" => { let r = cmd_classify_module(args);
            let factors: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
            if !factors.is_empty() { state_set_ints(state, "inv_factors", &factors); }
            r }
        "rational_form"   => { let r = cmd_rational_form(args); let coeffs: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect(); if !coeffs.is_empty() { state_set_int(state,"last_n",coeffs.len() as i64); } r }
        "jordan_form"     => { let r = cmd_jordan_form(args); if let Some(lam) = parse_int(args,0,"eigenvalue") { state_set_int(state,"last_n",lam); } r }
        "compare"         => cmd_compare(args),
        "tensor_zn"       => { let r = cmd_tensor_zn(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "demo"            => cmd_demo(),
        "help" | "h"      => help_text(),
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
    let factors = state_get_ints(state, "inv_factors").unwrap_or_else(|| vec![2, 6]);
    c.title("Structure Theorem Decomposition");
    c.subtitle("M = cyclic summands (invariant factor form)", 44.0);
    let cx = c.width / 2.0;
    // Draw M at top
    let parts: Vec<String> = factors.iter().filter_map(|&d| match d {
        0 => Some("Z".to_string()),
        1 => None,
        _ => Some(format!("Z/{}", d)),
    }).collect();
    let m_label = if parts.is_empty() { "0".to_string() } else { parts.join(" + ") };
    c.rrect(cx - 100.0, 55.0, 200.0, 36.0, 8.0, colors::HEADER_FILL, colors::CYAN, 2.0);
    c.text_bold(cx, 73.0, &format!("M = {}", m_label), 13.0, colors::DARK, "middle");
    // Draw summands
    let summands: Vec<(String, i64)> = factors.iter().filter_map(|&d| match d {
        0 => Some(("Z".to_string(), 0)),
        1 => None,
        _ => Some((format!("Z/{}", d), d)),
    }).collect();
    let n_sum = summands.len();
    if n_sum > 0 {
        let spacing = (c.width - 120.0) / n_sum as f64;
        for (i, (label, d)) in summands.iter().enumerate() {
            let x = 60.0 + (i as f64 + 0.5) * spacing;
            let y = 180.0;
            let fill = if *d == 0 { colors::LIGHT } else { colors::ROW_ALT };
            c.rrect(x - 35.0, y - 20.0, 70.0, 40.0, 8.0, fill, colors::CYAN, 1.5);
            c.text_bold(x, y, label, 12.0, colors::DARK, "middle");
            c.arrow(cx, 91.0, x, y - 20.0, colors::GREY, 1.0);
            // Primary decomposition
            if *d > 1 {
                let pf = factor(*d as u64);
                if pf.len() > 1 {
                    let prim: Vec<String> = pf.iter().map(|&(p,e)| format!("Z/{}", p.pow(e))).collect();
                    c.text(x, y + 35.0, &prim.join("+"), 9.0, colors::GREY, "middle");
                }
            }
        }
    }
    // Invariant factor chain
    let chain: Vec<String> = factors.iter().filter(|&&d| d > 1)
        .map(|&d| d.to_string()).collect();
    if chain.len() >= 2 {
        c.text(cx, c.height - 50.0,
               &format!("Divisibility chain: {}", chain.join(" | ")),
               11.0, colors::DARK, "middle");
    }
    c.text(cx, c.height - 30.0,
           "Structure Theorem: M = Z^r + Z/d_1 + ... + Z/d_k, d_i | d_{i+1}",
           10.0, colors::GREY, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let factors = state_get_ints(state, "inv_factors").unwrap_or_else(|| vec![2, 6]);
    let parts: Vec<String> = factors.iter().filter_map(|&d| match d {
        0 => Some("Z".to_string()), 1 => None, _ => Some(format!("Z/{}", d)),
    }).collect();
    g.graph_attr("label", "Structure Theorem Decomposition");
    g.node_default("shape", "box");
    g.node_default("style", "rounded,filled");
    g.node("M", &[("label", &format!("M = {}", parts.join(" + "))), ("fillcolor", "#ffd54f"), ("shape", "ellipse")]);
    for (i, &d) in factors.iter().enumerate() {
        if d <= 1 { continue; }
        let id = format!("s{}", i);
        let label = format!("Z/{}Z", d);
        g.node(&id, &[("label", &label), ("fillcolor", "#b2ebf2")]);
        g.edge("M", &id, &[("label", "summand")]);
        // primary components
        let pf = factor(d as u64);
        for (p, e) in pf {
            let pid = format!("p{}_{}", i, p);
            g.node(&pid, &[("label", &format!("Z/{}Z", p.pow(e))), ("fillcolor", "#e8f5e9")]);
            g.edge(&id, &pid, &[("label", "primary")]);
        }
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let factors = state_get_ints(state, "inv_factors").unwrap_or_else(|| vec![2, 6]);
    t.raw("  % Structure theorem decomposition");
    let parts: Vec<String> = factors.iter().filter_map(|&d| match d {
        0 => Some("\\mathbb{Z}".to_string()),
        1 => None,
        _ => Some(format!("\\mathbb{{Z}}/{}\\mathbb{{Z}}", d)),
    }).collect();
    let m_label = if parts.is_empty() { "0".to_string() } else { parts.join("\\oplus ") };
    t.node_math("M", 0.0, 2.5, &format!("M={}", m_label), "draw,ellipse,fill=yellow!30,minimum width=4cm");
    let summands: Vec<i64> = factors.iter().filter(|&&d| d > 1).copied().collect();
    let n = summands.len();
    for (i, &d) in summands.iter().enumerate() {
        let x = (i as f64 - (n as f64 - 1.0)/2.0) * 3.0;
        let id = format!("s{}", i);
        t.node_math(&id, x, 0.0,
                    &format!("\\mathbb{{Z}}/{}\\mathbb{{Z}}", d), "draw,rectangle,fill=cyan!20,rounded corners");
        t.arrow("M", &id, "", "gray");
        // Primary factors
        let pf = factor(d as u64);
        for (j, &(p, e)) in pf.iter().enumerate() {
            let pid = format!("p{}_{}", i, j);
            let px = x + (j as f64 - (pf.len() as f64 - 1.0)/2.0) * 1.8;
            t.node_math(&pid, px, -2.0,
                        &format!("\\mathbb{{Z}}/{}\\mathbb{{Z}}", p.pow(e)), "draw,circle,fill=green!20,minimum size=0.9cm");
            t.arrow(&id, &pid, "", "gray,dashed");
        }
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let factors = state_get_ints(state, "inv_factors").unwrap_or_else(|| vec![2, 6]);
    let parts: Vec<String> = factors.iter().filter_map(|&d| match d {
        0 => Some("Z".to_string()), 1 => None, _ => Some(format!("Z/{}", d)),
    }).collect();
    a.text_at(2, 0, "Structure Theorem Decomposition");
    a.hline(2, 1, 50, '-');
    a.text_at(2, 2, &format!("M = {}", if parts.is_empty() { "0".to_string() } else { parts.join(" + ") }));
    a.hline(2, 3, 50, '-');
    a.text_at(2, 4, "Summands:");
    for (i, &d) in factors.iter().enumerate() {
        if d <= 1 { continue; }
        let pf = factor(d as u64);
        let prim: Vec<String> = pf.iter().map(|&(p,e)| format!("Z/{}", p.pow(e))).collect();
        a.text_at(2, (5 + i) as i32, &format!("  Z/{}Z = {}", d, prim.join(" + ")));
    }
    let row = 5 + factors.len();
    let chain: Vec<String> = factors.iter().filter(|&&d| d > 1).map(|&d| d.to_string()).collect();
    if chain.len() >= 2 {
        a.text_at(2, row as i32, &format!("Divisibility: {}", chain.join(" | ")));
    }
    a.text_at(2, (row + 1) as i32, "Structure Theorem: unique invariant factors d_1|d_2|...|d_k");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch27"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 27", "Structure Theorem for Modules over PIDs",
                         "Smith normal form, invariant factors, Jordan & rational canonical forms");
            println!("{}", help_text());
            repl("ch27> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
