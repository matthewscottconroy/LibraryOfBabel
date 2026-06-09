use common::*;

fn help_text() -> String {
    help_string(&[
        ("zmod <n>",               "explore Z/nZ: tables, zero divisors, units, nilpotents"),
        ("gaussian <a> <b> <c> <d>", "multiply (a+bi)(c+di), compute norm"),
        ("zsqrt5 <a> <b> <c> <d>", "multiply in Z[sqrt(-5)], show norm"),
        ("factor6",                "6 = 2*3 = (1+sqrt(-5))(1-sqrt(-5)) in Z[sqrt(-5)]"),
        ("units <n>",              "all units of Z/nZ"),
        ("zero_div <n>",           "all zero divisors in Z/nZ"),
        ("nilp <n>",               "all nilpotents in Z/nZ"),
        ("wedderburn",             "Wedderburn: Z/p is a field iff p is prime"),
        ("demo",                   "showcase ring examples"),
        ("help",                   "show this help"),
        ("quit",                   "exit"),
    ])
}

fn cmd_zmod(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let mut out = format!("Ring Z/{}Z\n", n);
    if n > 12 {
        out.push_str("n too large for full table (use n <= 12); showing summary only.\n");
        out.push_str(&cmd_units(args));
        out.push_str(&cmd_zero_div(args));
        out.push_str(&cmd_nilp(args));
        return out;
    }
    let elems: Vec<String> = (0..n).map(|x| x.to_string()).collect();
    let add_table: Vec<Vec<usize>> = (0..n).map(|i| (0..n).map(|j| (i+j)%n).collect()).collect();
    out.push_str("Addition table:\n");
    // Simple text representation
    let header: Vec<String> = std::iter::once("+".to_string()).chain(elems.clone()).collect();
    out.push_str(&format!("  {}\n", header.join(" ")));
    for (i, row) in add_table.iter().enumerate() {
        let row_str: Vec<String> = std::iter::once(elems[i].clone())
            .chain(row.iter().map(|&j| j.to_string())).collect();
        out.push_str(&format!("  {}\n", row_str.join(" ")));
    }
    let mul_table: Vec<Vec<usize>> = (0..n).map(|i| (0..n).map(|j| (i*j)%n).collect()).collect();
    out.push_str("Multiplication table:\n");
    let mheader: Vec<String> = std::iter::once("*".to_string()).chain(elems.clone()).collect();
    out.push_str(&format!("  {}\n", mheader.join(" ")));
    for (i, row) in mul_table.iter().enumerate() {
        let row_str: Vec<String> = std::iter::once(elems[i].clone())
            .chain(row.iter().map(|&j| j.to_string())).collect();
        out.push_str(&format!("  {}\n", row_str.join(" ")));
    }
    out.push_str(&cmd_units(args));
    out.push_str(&cmd_zero_div(args));
    out.push_str(&cmd_nilp(args));
    out.push_str(&format!("Z/{}Z is {} ring.\n", n,
        if is_prime(n as u64) { "a field - every nonzero element is a unit" } else { "not an integral domain (has zero divisors)" }));
    out
}

fn cmd_gaussian(args: &[&str]) -> String {
    let a = match parse_int(args, 0, "a") { Some(x) => x, None => return String::new() };
    let b = match parse_int(args, 1, "b") { Some(x) => x, None => return String::new() };
    let c = match parse_int(args, 2, "c") { Some(x) => x, None => return String::new() };
    let d = match parse_int(args, 3, "d") { Some(x) => x, None => return String::new() };
    let re = a*c - b*d;
    let im = a*d + b*c;
    let n1 = a*a + b*b;
    let n2 = c*c + d*d;
    let np = re*re + im*im;
    let sign_im = if im < 0 { format!("{}i", im) } else { format!("+{}i", im) };
    let mut out = format!("Gaussian Integers Z[i]\n");
    out.push_str(&format!("  ({}{:+}i) * ({}{:+}i) = {}{}\n", a, b, c, d, re, sign_im));
    out.push_str(&format!("  N({}{:+}i) = {}\n", a, b, n1));
    out.push_str(&format!("  N({}{:+}i) = {}\n", c, d, n2));
    out.push_str(&format!("  N(product) = {}\n", np));
    if np == n1 * n2 { out.push_str("  N(ab) = N(a)*N(b) verified.\n"); }
    if np == 1 { out.push_str("  Product is a unit (norm = 1).\n"); }
    if is_prime(np as u64) && np > 1 { out.push_str("  Product is irreducible (norm is prime).\n"); }
    out
}

fn cmd_zsqrt5(args: &[&str]) -> String {
    let a = match parse_int(args, 0, "a") { Some(x) => x, None => return String::new() };
    let b = match parse_int(args, 1, "b") { Some(x) => x, None => return String::new() };
    let c = match parse_int(args, 2, "c") { Some(x) => x, None => return String::new() };
    let d = match parse_int(args, 3, "d") { Some(x) => x, None => return String::new() };
    let re = a*c - 5*b*d;
    let im = a*d + b*c;
    let n1 = a*a + 5*b*b;
    let n2 = c*c + 5*d*d;
    let np = re*re + 5*im*im;
    let fmt = |x: i64, y: i64| -> String {
        if y == 0 { format!("{}", x) }
        else if y < 0 { format!("{} - {}*sqrt(-5)", x, -y) }
        else { format!("{} + {}*sqrt(-5)", x, y) }
    };
    let mut out = format!("Ring Z[sqrt(-5)]\n");
    out.push_str(&format!("  ({}) * ({}) = {}\n", fmt(a,b), fmt(c,d), fmt(re,im)));
    out.push_str(&format!("  N({}) = {}\n", fmt(a,b), n1));
    out.push_str(&format!("  N({}) = {}\n", fmt(c,d), n2));
    out.push_str(&format!("  N(product) = {}\n", np));
    if np == n1*n2 { out.push_str("  Multiplicativity of norm holds.\n"); }
    out.push_str("  Z[sqrt(-5)] is NOT a UFD: unique factorization can fail.\n");
    out
}

fn cmd_factor6() -> String {
    let mut out = String::from("Failure of Unique Factorization in Z[sqrt(-5)]\n");
    out.push_str("  6 = 2 * 3\n");
    out.push_str("  6 = (1+sqrt(-5))(1-sqrt(-5))\n");
    out.push_str("  N(2)=4, N(3)=9, N(1+/-sqrt(-5))=6\n");
    out.push_str("  Each of 2,3,1+sqrt(-5),1-sqrt(-5) is irreducible.\n");
    out.push_str("  These two factorizations are genuinely different!\n");
    out.push_str("  Z[sqrt(-5)] is NOT a UFD. Cl(Z[sqrt(-5)]) = Z/2.\n");
    out
}

fn cmd_units(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let units: Vec<usize> = (1..n).filter(|&x| gcd(x as i64, n as i64) == 1).collect();
    format!("Units of Z/{}Z: {{{}}}  |Units|={}, phi({})={}\n", n,
        units.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
        units.len(), n, euler_totient(n as u64))
}

fn cmd_zero_div(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let zdivs: Vec<(usize,usize)> = (1..n).flat_map(|a|
        (1..n).filter_map(move |b| if (a*b)%n == 0 { Some((a,b)) } else { None })
    ).collect();
    let mut zd_elems: Vec<usize> = zdivs.iter().map(|&(a,_)| a).collect();
    zd_elems.sort(); zd_elems.dedup();
    if zd_elems.is_empty() {
        format!("Zero divisors of Z/{}Z: none (n is prime, integral domain).\n", n)
    } else {
        let pairs: Vec<String> = zdivs.iter().take(5).map(|&(a,b)| format!("{}*{}=0", a,b)).collect();
        format!("Zero divisors of Z/{}Z: {{{}}}  examples: {}\n", n,
            zd_elems.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
            pairs.join(", "))
    }
}

fn cmd_nilp(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let nilps: Vec<usize> = (0..n).filter(|&a| {
        for k in 1..=30u32 {
            if mod_pow(a as i64, k as u64, n as i64) == 0 { return true; }
        }
        false
    }).collect();
    let details: Vec<String> = nilps.iter().filter(|&&a| a != 0).filter_map(|&a| {
        for k in 1..=20u32 {
            if mod_pow(a as i64, k as u64, n as i64) == 0 {
                return Some(format!("{}^{}=0", a, k));
            }
        }
        None
    }).collect();
    format!("Nilpotents of Z/{}Z: {{{}}}  {}\n", n,
        nilps.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
        details.join(", "))
}

fn cmd_wedderburn() -> String {
    let mut out = String::from("Wedderburn: Z/pZ is a field iff p is prime\n");
    let primes = vec![2u64, 3, 5, 7, 11, 13];
    out.push_str("Primes -> fields: ");
    out.push_str(&primes.iter().map(|&p| format!("Z/{}Z", p)).collect::<Vec<_>>().join(", "));
    out.push('\n');
    let composites = vec![4u64, 6, 8, 9, 10, 12];
    out.push_str("Composites -> NOT fields: ");
    out.push_str(&composites.iter().map(|&n| format!("Z/{}Z", n)).collect::<Vec<_>>().join(", "));
    out.push('\n');
    out.push_str("Corollary: Z/nZ is a field iff n is prime.\n");
    out
}

fn cmd_demo() -> String {
    let mut out = String::from("Demo: Ring Examples\n");
    out.push_str(&cmd_units(&["6"]));
    out.push_str(&cmd_zero_div(&["6"]));
    out.push_str(&cmd_nilp(&["8"]));
    out.push_str(&cmd_factor6());
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "last_n", 6);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "zmod"       => { let r = cmd_zmod(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "gaussian"   => cmd_gaussian(args),
        "zsqrt5"     => cmd_zsqrt5(args),
        "factor6"    => cmd_factor6(),
        "units"      => { let r = cmd_units(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "zero_div"   => { let r = cmd_zero_div(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "nilp"       => { let r = cmd_nilp(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "wedderburn" => cmd_wedderburn(),
        "demo"       => cmd_demo(),
        "help" | "h" => help_text(),
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
    let n = state_get_int(state, "last_n").unwrap_or(6) as usize;
    let n_capped = n.min(10);
    // Multiplication table for Z/nZ
    let title = format!("Multiplication table Z/{}Z", n_capped);
    c.title(&title);
    let elems: Vec<String> = (0..n_capped).map(|x| x.to_string()).collect();
    let mul_table: Vec<Vec<usize>> = (0..n_capped).map(|i| (0..n_capped).map(|j| (i*j)%n_capped).collect()).collect();
    let mut cells: Vec<Vec<String>> = vec![{
        let mut h = vec!["*".to_string()];
        h.extend(elems.clone()); h
    }];
    for (i, row) in mul_table.iter().enumerate() {
        let mut r = vec![elems[i].clone()];
        r.extend(row.iter().map(|&j| j.to_string()));
        cells.push(r);
    }
    let cw = 36.0; let ch = 28.0;
    c.table(30.0, 55.0, &cells, cw, ch, colors::HEADER_FILL, colors::ROW_NORM, colors::GREY);
    // Mark units
    let units: Vec<usize> = (1..n_capped).filter(|&x| gcd(x as i64, n_capped as i64) == 1).collect();
    let unit_str: Vec<String> = units.iter().map(|x| x.to_string()).collect();
    c.text(30.0 + (n_capped+1) as f64 * cw / 2.0,
           55.0 + (n_capped+2) as f64 * ch,
           &format!("Units: {{{}}}", unit_str.join(",")),
           11.0, colors::DARK, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "last_n").unwrap_or(6) as usize;
    g.graph_attr("label", &format!("Ring axioms for Z/{}Z", n));
    g.node_default("shape", "box");
    g.node_default("style", "filled");
    g.node_default("fillcolor", "#b2ebf2");
    g.node("ring", &[("label", &format!("Z/{}Z", n)), ("shape", "ellipse"), ("fillcolor", "#ffd54f")]);
    let units: Vec<usize> = (1..n).filter(|&x| gcd(x as i64, n as i64) == 1).collect();
    let zdivs: Vec<usize> = (1..n).filter(|&a| (1..n).any(|b| (a*b)%n == 0)).collect();
    let nilps: Vec<usize> = (0..n).filter(|&a| {
        for k in 1..=20u32 { if mod_pow(a as i64, k as u64, n as i64) == 0 { return true; } }
        false
    }).collect();
    let u_str: Vec<String> = units.iter().map(|x| x.to_string()).collect();
    let z_str: Vec<String> = zdivs.iter().map(|x| x.to_string()).collect();
    let n_str: Vec<String> = nilps.iter().map(|x| x.to_string()).collect();
    g.node("units", &[("label", &format!("Units\n{{{}}}", u_str.join(",")))]);
    g.node("zdivs", &[("label", &format!("Zero Divs\n{{{}}}", z_str.join(",")))]);
    g.node("nilps", &[("label", &format!("Nilpotents\n{{{}}}", n_str.join(",")))]);
    g.edge("ring", "units", &[("label", "subset")]);
    g.edge("ring", "zdivs", &[("label", "subset")]);
    g.edge("nilps", "zdivs", &[("label", "contained in")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "last_n").unwrap_or(6) as usize;
    t.raw(&format!("  % Ring diagram for Z/{}Z", n));
    t.node_math("ring", 0.0, 0.0, &format!("\\mathbb{{Z}}/{}\\mathbb{{Z}}", n),
                "draw,ellipse,fill=yellow!30,minimum width=2.5cm");
    let props = vec![
        ("units", 3.0, 1.0, "Units"),
        ("zdivs", 3.0, -1.0, "Zero Divs"),
        ("nilp", -3.0, 0.0, "Nilpotents"),
    ];
    for &(id, x, y, label) in &props {
        t.node_math(id, x, y, label, "draw,rectangle,fill=cyan!20,rounded corners");
        t.arrow("ring", id, "", "gray");
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "last_n").unwrap_or(6) as usize;
    a.text_at(2, 0, &format!("Z/{}Z ring structure", n));
    a.hline(2, 1, 30, '-');
    let units: Vec<usize> = (1..n).filter(|&x| gcd(x as i64, n as i64) == 1).collect();
    let u_str: Vec<String> = units.iter().map(|x| x.to_string()).collect();
    a.text_at(2, 2, &format!("Units: {{{}}}", u_str.join(",")));
    let zdivs: Vec<usize> = (1..n).filter(|&a_| (1..n).any(|b| (a_*b)%n == 0)).collect();
    let z_str: Vec<String> = zdivs.iter().map(|x| x.to_string()).collect();
    a.text_at(2, 3, &format!("Zero div: {{{}}}", z_str.join(",")));
    let is_field = is_prime(n as u64);
    a.text_at(2, 4, if is_field { "Type: FIELD" } else { "Type: Ring (not field)" });
    // mul table snippet
    if n <= 6 {
        a.text_at(2, 6, "Mult table (mod n):");
        for i in 0..n {
            let row: Vec<String> = (0..n).map(|j| format!("{}", (i*j)%n)).collect();
            a.text_at(2, (7 + i) as i32, &format!("{}: {}", i, row.join(" ")));
        }
    }
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch20"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 20", "Rings and Ring Homomorphisms",
                         "Zero divisors, units, nilpotents, Gaussian integers, Z[sqrt(-5)]");
            println!("{}", help_text());
            repl("ch20> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
