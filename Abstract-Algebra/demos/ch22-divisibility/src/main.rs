use common::*;

fn help_text() -> String {
    help_string(&[
        ("euclidean <a> <b>",           "Euclidean algorithm with all steps"),
        ("gcd_poly <f...> / <g...>",    "Euclidean algorithm on polynomials"),
        ("bezout <a> <b>",              "Bezout coefficients: as + bt = gcd(a,b)"),
        ("ufd_check <ring>",            "check UFD: ring = gaussian or zsqrt5"),
        ("norm_gaussian <a> <b>",       "norm of a+bi, unit/irreducible check"),
        ("split_prime <p>",             "how prime p splits in Z[i]"),
        ("eisenstein <coeffs> <p>",     "apply Eisenstein's criterion with prime p"),
        ("hierarchy",                   "domain hierarchy: ED < PID < UFD < Domain"),
        ("demo",                        "showcase divisibility examples"),
        ("help",                        "show this help"),
        ("quit",                        "exit"),
    ])
}

fn cmd_euclidean(args: &[&str]) -> String {
    let a = match parse_int(args, 0, "a") { Some(x) => x, None => return String::new() };
    let b = match parse_int(args, 1, "b") { Some(x) => x, None => return String::new() };
    let mut out = format!("Euclidean Algorithm: gcd({}, {})\n", a, b);
    let mut x = a.abs();
    let mut y = b.abs();
    let mut step = 0;
    while y != 0 {
        let q = x / y;
        let r = x % y;
        out.push_str(&format!("  step {}: {} = {} * {} + {}\n", step+1, x, q, y, r));
        x = y; y = r; step += 1;
    }
    out.push_str(&format!("  gcd = {}\n", x));
    out
}

fn cmd_gcd_poly(args: &[&str]) -> String {
    let sep = args.iter().position(|&s| s == "/");
    let (f_args, g_args) = match sep {
        Some(pos) => (&args[..pos], &args[pos+1..]),
        None => return "Usage: gcd_poly <f coeffs> / <g coeffs>".to_string(),
    };
    let f_coeffs: Vec<i64> = f_args.iter().filter_map(|s| s.parse().ok()).collect();
    let g_coeffs: Vec<i64> = g_args.iter().filter_map(|s| s.parse().ok()).collect();
    if f_coeffs.is_empty() || g_coeffs.is_empty() {
        return "Need non-empty coefficient lists on both sides of /".to_string();
    }
    let mut f = Poly::new(f_coeffs);
    let mut g = Poly::new(g_coeffs);
    let mut out = format!("Polynomial GCD\n  f(x) = {}\n  g(x) = {}\n", f, g);
    let mut step = 0;
    while !g.coeffs.is_empty() {
        let (q, r) = f.div_rem(&g);
        out.push_str(&format!("  step {}: ({}) = ({}) * ({}) + ({})\n", step+1, f, q, g, r));
        f = g; g = r; step += 1;
    }
    out.push_str(&format!("  gcd = {}\n", f));
    out
}

fn cmd_bezout(args: &[&str]) -> String {
    let a = match parse_int(args, 0, "a") { Some(x) => x, None => return String::new() };
    let b = match parse_int(args, 1, "b") { Some(x) => x, None => return String::new() };
    let (g, s, t) = ext_gcd(a, b);
    let mut out = format!("Bezout: gcd({},{}) = {}\n", a, b, g);
    out.push_str(&format!("  s = {}, t = {}\n", s, t));
    out.push_str(&format!("  Verification: {}*{} + {}*{} = {}\n", a, s, b, t, a*s + b*t));
    if a*s + b*t == g { out.push_str("  Bezout identity as + bt = gcd verified!\n"); }
    // Show steps
    let mut x = a.abs(); let mut y = b.abs();
    let mut steps: Vec<(i64,i64,i64,i64)> = Vec::new();
    while y != 0 {
        let q = x / y; let r = x % y;
        steps.push((x, q, y, r));
        x = y; y = r;
    }
    out.push_str("  Back-substitution steps:\n");
    for (i, &(xi, q, yi, r)) in steps.iter().enumerate() {
        out.push_str(&format!("    {}: {} = {}*{} + {}\n", i+1, xi, q, yi, r));
    }
    out
}

fn cmd_ufd_check(args: &[&str]) -> String {
    let ring = args.first().copied().unwrap_or("gaussian");
    match ring {
        "gaussian" => {
            "Z[i] IS a UFD (Euclidean domain).\n  N(a+bi) = a^2+b^2, units: {1,-1,i,-i}.\n  Example: 5 = (2+i)(2-i), N(2+i)=5 prime.\n  PID => UFD. Z[i] has Euclidean algorithm.\n".to_string()
        },
        "zsqrt5" => {
            "Z[sqrt(-5)] is NOT a UFD.\n  6 = 2*3 = (1+sqrt(-5))(1-sqrt(-5))\n  Both factorizations use irreducibles.\n  Ideal class group Cl(Z[sqrt(-5)]) = Z/2 (nontrivial).\n  UFD <=> every irreducible is prime; fails here.\n".to_string()
        },
        _ => "Unknown ring. Use 'gaussian' or 'zsqrt5'.".to_string(),
    }
}

fn cmd_norm_gaussian(args: &[&str]) -> String {
    let a = match parse_int(args, 0, "a") { Some(x) => x, None => return String::new() };
    let b = match parse_int(args, 1, "b") { Some(x) => x, None => return String::new() };
    let n = a*a + b*b;
    let mut out = format!("N({}{:+}i) = {}^2 + {}^2 = {}\n", a, b, a, b, n);
    if n == 0 { out.push_str("  Zero element.\n"); }
    else if n == 1 { out.push_str("  Unit: N = 1.\n"); }
    else if is_prime(n as u64) { out.push_str(&format!("  Irreducible in Z[i]: N={} is prime.\n", n)); }
    else { out.push_str(&format!("  Composite norm {}; element may factor further.\n", n)); }
    out
}

fn cmd_split_prime(args: &[&str]) -> String {
    let p = match parse_uint(args, 0, "p") { Some(x) => x, None => return String::new() };
    if !is_prime(p) { return format!("{} is not prime.", p); }
    let mut out = format!("How {} splits in Z[i]:\n", p);
    if p == 2 {
        out.push_str("  2 ramifies: (2) = (1+i)^2  N(1+i)=2\n");
    } else if p % 4 == 1 {
        out.push_str(&format!("  {} = 1 (mod 4): SPLITS in Z[i]\n", p));
        let mut found = None;
        'outer: for a in 1..=(p as i64) {
            for b in 0..=a {
                if a*a + b*b == p as i64 { found = Some((a,b)); break 'outer; }
            }
        }
        if let Some((a,b)) = found {
            out.push_str(&format!("  {} = ({}{:+}i)({}{:+}i)  N={}*{}={}\n",
                p, a, b, a, -b, a*a+b*b, a*a+b*b, p));
        }
    } else {
        out.push_str(&format!("  {} = 3 (mod 4): INERT in Z[i] (remains prime)\n", p));
    }
    out.push_str("  p=2 ramifies; p=1(4) splits; p=3(4) inert.\n");
    out
}

fn cmd_eisenstein(args: &[&str]) -> String {
    if args.len() < 2 {
        return "Usage: eisenstein <coeffs...> <p>  (last arg is prime p)".to_string();
    }
    let p = match args.last().and_then(|s| s.parse::<u64>().ok()) {
        Some(x) => x, None => return "Last argument must be prime p.".to_string(),
    };
    let coeffs: Vec<i64> = args[..args.len()-1].iter().filter_map(|s| s.parse().ok()).collect();
    if coeffs.is_empty() { return "No polynomial coefficients given.".to_string(); }
    let f = Poly::new(coeffs.clone());
    let deg = match f.degree() { Some(d) => d, None => return "Zero polynomial.".to_string() };
    if !is_prime(p) { return format!("{} is not prime.", p); }
    let mut out = format!("Eisenstein criterion for f(x)={} at p={}\n", f, p);
    let lc = coeffs[deg];
    let cond1 = lc % p as i64 != 0;
    out.push_str(&format!("  (1) p does not divide a_n: {} does not divide {}? {}\n",
        p, lc, if cond1 { "YES" } else { "NO -- FAILS" }));
    let mut cond2 = true;
    for i in 0..deg {
        let c = coeffs[i];
        if c % p as i64 != 0 {
            out.push_str(&format!("  (2) p divides a_{}: {} divides {}? NO -- FAILS\n", i, p, c));
            cond2 = false; break;
        } else {
            out.push_str(&format!("  (2) p divides a_{}: {} divides {}? YES\n", i, p, c));
        }
    }
    let c0 = coeffs[0];
    let cond3 = c0 % (p*p) as i64 != 0;
    out.push_str(&format!("  (3) p^2 does not divide a_0: {}^2={} does not divide {}? {}\n",
        p, p*p, c0, if cond3 { "YES" } else { "NO -- FAILS" }));
    if cond1 && cond2 && cond3 {
        out.push_str(&format!("  ALL conditions met! f(x)={} is IRREDUCIBLE over Q.\n", f));
    } else {
        out.push_str("  Eisenstein does not apply for this p.\n");
    }
    out
}

fn cmd_hierarchy() -> String {
    let mut out = String::from("Domain Hierarchy: ED < PID < UFD < Integral Domain < Commutative Ring\n\n");
    let rows = [
        ("Ring",          "ED",  "PID", "UFD", "Domain"),
        ("Z",             "YES", "YES", "YES", "YES"),
        ("Z[i]",          "YES", "YES", "YES", "YES"),
        ("k[x] (field k)","YES", "YES", "YES", "YES"),
        ("Z[x]",          "NO",  "NO",  "YES", "YES"),
        ("Z[sqrt(-5)]",   "NO",  "NO",  "NO",  "YES"),
        ("Z/pZ (prime)",  "YES", "YES", "YES", "YES (field)"),
        ("Z/nZ (comp.)",  "NO",  "NO",  "NO",  "NO"),
    ];
    out.push_str(&format!("  {:<22} {:<6} {:<6} {:<6} {}\n", "Ring", "ED", "PID", "UFD", "Domain"));
    out.push_str(&format!("  {}\n", "-".repeat(55)));
    for &(ring, ed, pid, ufd, dom) in &rows[1..] {
        out.push_str(&format!("  {:<22} {:<6} {:<6} {:<6} {}\n", ring, ed, pid, ufd, dom));
    }
    out
}

fn cmd_demo() -> String {
    let mut out = String::from("Demo: Divisibility\n");
    out.push_str(&cmd_euclidean(&["48", "18"]));
    out.push_str(&cmd_bezout(&["35", "15"]));
    out.push_str(&cmd_split_prime(&["5"]));
    out.push_str(&cmd_ufd_check(&["zsqrt5"]));
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "last_a", 48);
    state_set_int(&mut s, "last_b", 18);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "euclidean"     => { let r = cmd_euclidean(args);
            if let (Some(a), Some(b)) = (parse_int(args,0,"a"), parse_int(args,1,"b")) {
                state_set_int(state,"last_a",a); state_set_int(state,"last_b",b);
            } r }
        "gcd_poly"      => cmd_gcd_poly(args),
        "bezout"        => { let r = cmd_bezout(args);
            if let (Some(a), Some(b)) = (parse_int(args,0,"a"), parse_int(args,1,"b")) {
                state_set_int(state,"last_a",a); state_set_int(state,"last_b",b);
            } r }
        "ufd_check"     => cmd_ufd_check(args),
        "norm_gaussian" => cmd_norm_gaussian(args),
        "split_prime"   => { let r = cmd_split_prime(args); if let Some(p) = parse_uint(args,0,"p") { state_set_int(state,"last_a",p as i64); } r }
        "eisenstein"    => cmd_eisenstein(args),
        "hierarchy"     => cmd_hierarchy(),
        "demo"          => cmd_demo(),
        "help" | "h"    => help_text(),
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
    let a = state_get_int(state, "last_a").unwrap_or(48) as u64;
    let b = state_get_int(state, "last_b").unwrap_or(18) as u64;
    let g = gcd(a as i64, b as i64).unsigned_abs();
    c.title(&format!("Divisibility Hasse: divisors of gcd({},{})={}", a, b, g));
    c.subtitle("Hasse diagram of divisors", 44.0);
    let divs = divisors(g);
    let num = divs.len();
    let cx = c.width / 2.0;
    let level_of = |d: u64| -> u32 { factor(d).iter().map(|&(_,e)| e).sum() };
    let max_level = divs.iter().map(|&d| level_of(d)).max().unwrap_or(0);
    let pos: Vec<(f64, f64)> = divs.iter().enumerate().map(|(i, &d)| {
        let lv = level_of(d) as f64;
        let frac = if max_level > 0 { lv / max_level as f64 } else { 0.5 };
        let y = 80.0 + frac * 350.0;
        let x = cx + (i as f64 - (num as f64 - 1.0)/2.0) * 80.0;
        (x, y)
    }).collect();
    for (i, &d1) in divs.iter().enumerate() {
        for (j, &d2) in divs.iter().enumerate() {
            if d2 % d1 == 0 && d2 != d1 {
                let intermediate = divs.iter().any(|&dm| dm != d1 && dm != d2 && d2 % dm == 0 && dm % d1 == 0);
                if !intermediate {
                    c.line(pos[i].0, pos[i].1, pos[j].0, pos[j].1, colors::GREY, 1.0);
                }
            }
        }
    }
    for (i, &d) in divs.iter().enumerate() {
        let (x, y) = pos[i];
        let fill = if is_prime(d) { colors::HEADER_FILL } else { colors::LIGHT };
        c.circle(x, y, 20.0, fill, colors::CYAN, 1.5);
        c.text_bold(x, y, &d.to_string(), 12.0, colors::DARK, "middle");
    }
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let a = state_get_int(state, "last_a").unwrap_or(48) as u64;
    let b = state_get_int(state, "last_b").unwrap_or(18) as u64;
    let gc = gcd(a as i64, b as i64).unsigned_abs();
    g.graph_attr("label", &format!("Divisors of gcd({},{})={}", a, b, gc));
    g.graph_attr("rankdir", "BT");
    g.node_default("shape", "circle");
    g.node_default("style", "filled");
    g.node_default("fillcolor", "#b2ebf2");
    let divs = divisors(gc);
    for &d in &divs {
        let fill = if is_prime(d) { "#ffd54f" } else { "#b2ebf2" };
        g.node(&d.to_string(), &[("fillcolor", fill)]);
    }
    for &d1 in &divs {
        for &d2 in &divs {
            if d2 % d1 == 0 && d2 != d1 {
                let intermediate = divs.iter().any(|&dm| dm != d1 && dm != d2 && d2 % dm == 0 && dm % d1 == 0);
                if !intermediate {
                    g.edge(&d1.to_string(), &d2.to_string(), &[]);
                }
            }
        }
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let a = state_get_int(state, "last_a").unwrap_or(48) as u64;
    let b = state_get_int(state, "last_b").unwrap_or(18) as u64;
    let gc = gcd(a as i64, b as i64).unsigned_abs();
    t.raw(&format!("  % Divisibility Hasse for gcd({},{})={}", a, b, gc));
    let divs = divisors(gc);
    let num = divs.len();
    let level_of = |d: u64| -> u32 { factor(d).iter().map(|&(_,e)| e).sum() };
    let max_level = divs.iter().map(|&d| level_of(d)).max().unwrap_or(0);
    for (i, &d) in divs.iter().enumerate() {
        let lv = level_of(d) as f64;
        let y = if max_level > 0 { lv / max_level as f64 * 4.0 } else { 0.0 };
        let x = (i as f64 - (num as f64 - 1.0)/2.0) * 1.8;
        t.node_math(&d.to_string(), x, y, &d.to_string(), "draw,circle,fill=cyan!20");
    }
    for &d1 in &divs {
        for &d2 in &divs {
            if d2 % d1 == 0 && d2 != d1 {
                let intermediate = divs.iter().any(|&dm| dm != d1 && dm != d2 && d2 % dm == 0 && dm % d1 == 0);
                if !intermediate {
                    t.edge(&d1.to_string(), &d2.to_string(), "", "gray");
                }
            }
        }
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let av = state_get_int(state, "last_a").unwrap_or(48);
    let b = state_get_int(state, "last_b").unwrap_or(18);
    let g = gcd(av, b).unsigned_abs();
    a.text_at(2, 0, &format!("Euclidean: gcd({},{})={}", av, b, g));
    a.hline(2, 1, 40, '-');
    // Show steps
    let mut x = av.abs(); let mut y = b.abs();
    let mut row = 2i32;
    while y != 0 {
        let q = x / y; let r = x % y;
        a.text_at(2, row, &format!("{} = {}*{} + {}", x, q, y, r));
        x = y; y = r; row += 1;
    }
    a.text_at(2, row, &format!("gcd = {}", x));
    a.hline(2, row+1, 25, '-');
    // Hierarchy
    a.text_at(2, row+2, "Hierarchy: ED < PID < UFD < Domain");
    a.text_at(2, row+3, "  Z, Z[i], k[x] are all EDs");
    a.text_at(2, row+4, "  Z[sqrt(-5)]: Domain but not UFD");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch22"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 22", "Divisibility and the Domain Hierarchy",
                         "ED < PID < UFD < Domain, Euclidean algorithm, Eisenstein");
            println!("{}", help_text());
            repl("ch22> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
