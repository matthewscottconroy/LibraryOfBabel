use common::*;

fn help_text() -> String {
    help_string(&[
        ("ideal <n> <k>",         "ideal kZ in Z; elements mod n; Z/nZ/(k mod n)"),
        ("quotient <n> <k>",      "build Z/nZ/(k) as ring, show multiplication table"),
        ("is_prime_ideal <n> <k>","check if (k) is prime ideal in Z/nZ"),
        ("is_maximal <n> <k>",    "check if (k) is maximal ideal in Z/nZ"),
        ("adjoin_root <coeffs>",  "R[x]/(f): describe extension, show arithmetic of coset x"),
        ("radical <n>",           "nilradical of Z/nZ = intersection of all prime ideals"),
        ("contain <n> <k1> <k2>", "does ideal (k1) contain (k2) in Z/nZ?"),
        ("demo",                  "showcase ideal examples"),
        ("help",                  "show this help"),
        ("quit",                  "exit"),
    ])
}

fn ideal_generator(n: usize, k: usize) -> usize {
    gcd(k as i64, n as i64).unsigned_abs() as usize % n
}

fn ideal_elements(n: usize, k: usize) -> Vec<usize> {
    let g = ideal_generator(n, k);
    if g == 0 { return vec![0]; }
    (0..n).filter(|&x| x % g == 0).collect()
}

fn cmd_ideal(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    let k = match parse_uint(args, 1, "k") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let g = ideal_generator(n, k);
    let elems = ideal_elements(n, k);
    let mut out = format!("Ideal ({}) in Z/{}Z = ({})  [gcd({},{})={}]\n", k, n, g, k, n, g);
    out.push_str(&format!("  Elements: {{{}}}\n", elems.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    out.push_str(&format!("  |ideal| = {}\n", elems.len()));
    out.push_str(&format!("  Quotient Z/{}Z / ({}) = Z/{}Z\n", n, k, g));
    out
}

fn cmd_quotient(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    let k = match parse_uint(args, 1, "k") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let g = ideal_generator(n, k);
    if g == 0 { return "k = 0: whole ring, trivial quotient.".to_string(); }
    let q = g;
    let mut out = format!("Quotient ring Z/{}Z / ({}) = Z/{}Z\n", n, k, q);
    if q > 12 {
        out.push_str("Quotient ring too large for full table.\n");
        return out;
    }
    let elems: Vec<String> = (0..q).map(|x| x.to_string()).collect();
    let mul_table: Vec<Vec<usize>> = (0..q).map(|i| (0..q).map(|j| (i*j)%q).collect()).collect();
    out.push_str("Multiplication table:\n");
    let header: Vec<String> = std::iter::once("*".to_string()).chain(elems.clone()).collect();
    out.push_str(&format!("  {}\n", header.join(" ")));
    for (i, row) in mul_table.iter().enumerate() {
        let row_str: Vec<String> = std::iter::once(elems[i].clone())
            .chain(row.iter().map(|&j| j.to_string())).collect();
        out.push_str(&format!("  {}\n", row_str.join(" ")));
    }
    out
}

fn cmd_is_prime_ideal(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    let k = match parse_uint(args, 1, "k") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let g = ideal_generator(n, k);
    let q = g;
    let prime = is_prime(q as u64);
    let mut out = format!("Is ({}) a prime ideal in Z/{}Z?\n", k, n);
    out.push_str(&format!("  Quotient Z/{}Z / ({}) = Z/{}Z\n", n, k, q));
    if prime {
        out.push_str(&format!("  YES: quotient Z/{}Z is integral domain ({} prime).\n", q, q));
    } else if q == 1 {
        out.push_str("  Degenerate: ideal = whole ring.\n");
    } else {
        out.push_str(&format!("  NO: quotient Z/{}Z has zero divisors ({} not prime).\n", q, q));
    }
    out
}

fn cmd_is_maximal(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    let k = match parse_uint(args, 1, "k") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let g = ideal_generator(n, k);
    let q = g;
    let prime = is_prime(q as u64);
    let mut out = format!("Is ({}) maximal in Z/{}Z?\n", k, n);
    out.push_str(&format!("  Quotient Z/{}Z / ({}) = Z/{}Z\n", n, k, q));
    if prime {
        out.push_str(&format!("  YES: quotient Z/{}Z is a field ({} prime).\n", q, q));
    } else if q == 1 {
        out.push_str("  Degenerate: whole ring.\n");
    } else {
        out.push_str(&format!("  NO: quotient Z/{}Z not a field ({} composite).\n", q, q));
    }
    out
}

fn cmd_adjoin_root(args: &[&str]) -> String {
    if args.is_empty() {
        return "Usage: adjoin_root <coeffs...>  (constant term first)\nExample: adjoin_root 2 0 1  means x^2+2".to_string();
    }
    let coeffs: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
    let f = Poly::new(coeffs);
    let deg = match f.degree() { Some(d) => d, None => return "Zero polynomial.".to_string() };
    let mut out = format!("Q[x] / ({}) -- Adjoining a Root\n", f);
    out.push_str(&format!("  f(x) = {}, deg = {}\n", f, deg));
    let irred_mod2 = f.is_irreducible_mod(2);
    let irred_mod3 = f.is_irreducible_mod(3);
    out.push_str(&format!("  Irreducible mod 2? {}  mod 3? {}\n",
        if irred_mod2 { "Yes" } else { "No" },
        if irred_mod3 { "Yes" } else { "No" }));
    let basis: Vec<String> = (0..deg).map(|i| if i == 0 { "1".to_string() } else if i == 1 { "a".to_string() } else { format!("a^{}", i) }).collect();
    out.push_str(&format!("  Basis: {{{}}}\n", basis.join(", ")));
    out.push_str(&format!("  Degree [Q(a):Q] = {}\n", deg));
    if deg == 2 {
        let b = f.coeffs.get(1).copied().unwrap_or(0);
        let c_val = f.coeffs.get(0).copied().unwrap_or(0);
        let lc = f.leading();
        let disc = b*b - 4*lc*c_val;
        out.push_str(&format!("  Discriminant = {}\n", disc));
        if disc < 0 { out.push_str("  disc < 0: complex quadratic field extension.\n"); }
        else if disc > 0 { out.push_str("  disc > 0: real quadratic field.\n"); }
    }
    out.push_str("  Q[x]/(f) is a field iff f is irreducible over Q.\n");
    out
}

fn cmd_radical(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let nilpotents: Vec<usize> = (0..n).filter(|&a| {
        for k in 1..=20u32 {
            if mod_pow(a as i64, k as u64, n as i64) == 0 { return true; }
        }
        false
    }).collect();
    let mut out = format!("Nilradical of Z/{}Z\n", n);
    out.push_str(&format!("  Nilradical = {{{}}} (nilpotent elements)\n",
        nilpotents.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    let prime_ideals: Vec<u64> = factor(n as u64).iter().map(|&(p,_)| p).collect();
    out.push_str("  Prime ideals correspond to prime divisors of n:\n");
    for &p in &prime_ideals {
        out.push_str(&format!("    ({}) [quotient = Z/{}Z]\n", p, p));
    }
    let intersection: Vec<usize> = (0..n).filter(|&x| {
        prime_ideals.iter().all(|&p| x as u64 % p == 0)
    }).collect();
    out.push_str(&format!("  Intersection of prime ideals = {{{}}}\n",
        intersection.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    if nilpotents == intersection { out.push_str("  Nilradical = intersection verified!\n"); }
    out
}

fn cmd_contain(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    let k1 = match parse_uint(args, 1, "k1") { Some(x) => x as usize, None => return String::new() };
    let k2 = match parse_uint(args, 2, "k2") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let g1 = ideal_generator(n, k1);
    let g2 = ideal_generator(n, k2);
    let elems1 = ideal_elements(n, k1);
    let elems2 = ideal_elements(n, k2);
    let mut out = format!("Ideal containment in Z/{}Z\n", n);
    out.push_str(&format!("  ({}) = ({}) = {{{}}}\n", k1, g1, elems1.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    out.push_str(&format!("  ({}) = ({}) = {{{}}}\n", k2, g2, elems2.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    let contains_12 = g1 > 0 && g2 % g1 == 0;
    let contains_21 = g2 > 0 && g1 % g2 == 0;
    out.push_str(&format!("  ({}) contains ({})? {}\n", k1, k2, if contains_12 { "YES" } else { "NO" }));
    out.push_str(&format!("  ({}) contains ({})? {}\n", k2, k1, if contains_21 { "YES" } else { "NO" }));
    if contains_12 && contains_21 { out.push_str("  The ideals are equal.\n"); }
    out
}

fn cmd_demo() -> String {
    let mut out = String::from("Demo: Ideals in Z/12Z\n");
    out.push_str(&cmd_is_prime_ideal(&["12", "3"]));
    out.push_str(&cmd_is_maximal(&["12", "3"]));
    out.push_str(&cmd_radical(&["12"]));
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "last_n", 12);
    state_set_int(&mut s, "last_k", 3);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "ideal"           => { let r = cmd_ideal(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "quotient"        => { let r = cmd_quotient(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } if let Some(k) = parse_uint(args,1,"k") { state_set_int(state,"last_k",k as i64); } r }
        "is_prime_ideal"  => { let r = cmd_is_prime_ideal(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "is_maximal"      => { let r = cmd_is_maximal(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "adjoin_root"     => cmd_adjoin_root(args),
        "radical"         => { let r = cmd_radical(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "contain"         => cmd_contain(args),
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
    let n = state_get_int(state, "last_n").unwrap_or(12) as usize;
    c.title(&format!("Ideal lattice of Z/{}Z (Hasse diagram)", n));
    // Ideals correspond to divisors of n; arranged by size
    let divs = divisors(n as u64);
    let num = divs.len();
    let cx = c.width / 2.0;
    let total_h = c.height - 80.0;
    // position by divisor index (level = number of prime factors counted with multiplicity)
    let level_of = |d: u64| -> u32 { factor(d).iter().map(|&(_,e)| e).sum() };
    let max_level = divs.iter().map(|&d| level_of(d)).max().unwrap_or(0);
    let pos: Vec<(f64, f64)> = divs.iter().enumerate().map(|(i, &d)| {
        let lv = level_of(d) as f64;
        let frac = if max_level > 0 { lv / max_level as f64 } else { 0.0 };
        let y = 80.0 + frac * total_h;
        let x = cx + (i as f64 - (num as f64 - 1.0)/2.0) * 90.0;
        (x, y)
    }).collect();
    // Draw edges: d1 -> d2 if d1 | d2 and no intermediate
    for (i, &d1) in divs.iter().enumerate() {
        for (j, &d2) in divs.iter().enumerate() {
            if d2 % d1 == 0 && d2 != d1 {
                // check no intermediate divisor
                let intermediate = divs.iter().any(|&dm| dm != d1 && dm != d2 && d2 % dm == 0 && dm % d1 == 0);
                if !intermediate {
                    c.line(pos[i].0, pos[i].1, pos[j].0, pos[j].1, colors::GREY, 1.0);
                }
            }
        }
    }
    // Draw nodes
    for (i, &d) in divs.iter().enumerate() {
        let (x, y) = pos[i];
        let gen = n as u64 / d; // generator of this ideal
        let fill = if is_prime(d) { colors::HEADER_FILL } else { colors::LIGHT };
        c.circle(x, y, 20.0, fill, colors::CYAN, 1.5);
        c.text_bold(x, y - 5.0, &format!("({})", gen), 10.0, colors::DARK, "middle");
        c.text(x, y + 7.0, &format!("size {}", d), 9.0, colors::GREY, "middle");
    }
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "last_n").unwrap_or(12) as usize;
    g.graph_attr("label", &format!("Ideal Hasse diagram Z/{}Z", n));
    g.graph_attr("rankdir", "BT");
    g.node_default("shape", "ellipse");
    g.node_default("style", "filled");
    g.node_default("fillcolor", "#b2ebf2");
    let divs = divisors(n as u64);
    for &d in &divs {
        let gen = n as u64 / d;
        let id = format!("d{}", d);
        let fill = if is_prime(d) { "#ffd54f" } else { "#b2ebf2" };
        g.node(&id, &[("label", &format!("({}) size={}", gen, d)), ("fillcolor", fill)]);
    }
    for &d1 in &divs {
        for &d2 in &divs {
            if d2 % d1 == 0 && d2 != d1 {
                let intermediate = divs.iter().any(|&dm| dm != d1 && dm != d2 && d2 % dm == 0 && dm % d1 == 0);
                if !intermediate {
                    g.edge(&format!("d{}", d1), &format!("d{}", d2), &[]);
                }
            }
        }
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "last_n").unwrap_or(12) as usize;
    t.raw(&format!("  % Ideal lattice for Z/{}Z", n));
    let divs = divisors(n as u64);
    let num = divs.len();
    for (i, &d) in divs.iter().enumerate() {
        let gen = n as u64 / d;
        let x = (i as f64 - (num as f64 - 1.0)/2.0) * 2.5;
        let level: u32 = factor(d).iter().map(|&(_,e)| e).sum();
        let y = level as f64 * 2.0;
        t.node_math(&format!("d{}", d), x, y,
                    &format!("({})", gen), "draw,circle,fill=cyan!20");
    }
    for &d1 in &divs {
        for &d2 in &divs {
            if d2 % d1 == 0 && d2 != d1 {
                let intermediate = divs.iter().any(|&dm| dm != d1 && dm != d2 && d2 % dm == 0 && dm % d1 == 0);
                if !intermediate {
                    t.edge(&format!("d{}", d1), &format!("d{}", d2), "", "gray");
                }
            }
        }
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "last_n").unwrap_or(12) as usize;
    a.text_at(2, 0, &format!("Ideal lattice of Z/{}Z", n));
    a.hline(2, 1, 40, '-');
    let divs = divisors(n as u64);
    a.text_at(2, 2, "Ideals (by generator, ascending):");
    let mut chain: Vec<String> = Vec::new();
    for &d in &divs {
        let gen = n as u64 / d;
        chain.push(format!("({})", gen));
    }
    a.text_at(2, 3, &chain.join(" < "));
    let prime_ideals: Vec<u64> = factor(n as u64).iter().map(|&(p,_)| p).collect();
    a.text_at(2, 5, "Prime ideals:");
    for (i, &p) in prime_ideals.iter().enumerate() {
        a.text_at(2, (6 + i) as i32, &format!("  ({}) -- quotient Z/{}Z is field", p, p));
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch21"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 21", "Ideals and Quotient Rings",
                         "Prime ideals, maximal ideals, nilradical, adjoining roots");
            println!("{}", help_text());
            repl("ch21> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
