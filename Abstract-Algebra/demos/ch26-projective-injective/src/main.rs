use common::*;

fn help_text() -> String {
    help_string(&[
        ("free_res <n>",         "free resolution of Z/nZ: 0->Z->*n->Z->Z/nZ->0"),
        ("projective_test <n> <k>","is Z/kZ projective over Z/nZ?"),
        ("injective_test <n>",   "is Z/nZ injective as Z-module?"),
        ("flat_test <n>",        "is Z/nZ flat? (iff n is square-free)"),
        ("baer_criterion <n>",   "Baer's criterion: extend map from ideal->Z/nZ"),
        ("splitting <n> <k>",    "does 0->Z/kZ->Z/nkZ->Z/nZ->0 split?"),
        ("pd <n>",               "projective dimension of Z/nZ over Z"),
        ("demo",                 "showcase projective/injective examples"),
        ("help",                 "show this help"),
        ("quit",                 "exit"),
    ])
}

fn cmd_free_res(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let mut out = format!("Free Resolution of Z/{}Z\n", n);
    out.push_str(&format!("  0 -> Z -[x{}]-> Z -[pi]-> Z/{}Z -> 0\n", n, n));
    out.push_str(&format!("  i: Z->Z, i(k) = {}k\n", n));
    out.push_str(&format!("  pi: Z->Z/{}Z, pi(k) = k mod {}\n", n, n));
    out.push_str("  Exactness:\n");
    out.push_str("    i is injective (multiplication by n != 0).\n");
    out.push_str(&format!("    ker(pi) = Im(i) = {}Z.\n", n));
    out.push_str("    pi is surjective.\n");
    out.push_str("  Both Z's are free rank-1: minimal free resolution.\n");
    out.push_str(&format!("  pd_Z(Z/{}Z) = 1.\n", n));
    out.push_str(&format!("  Ext^1_Z(Z/{}Z, M) = M/{}M.\n", n, n));
    out.push_str(&format!("  Tor^1_Z(Z/{}Z, M) = M[{}] = {{x: {}x=0}}.\n", n, n, n));
    out
}

fn cmd_projective_test(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    let k = match parse_uint(args, 1, "k") { Some(x) => x, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    if k == 0 || n % k != 0 {
        return format!("{} does not divide {}: Z/kZ not a natural Z/nZ-module.", k, n);
    }
    let nk = n / k;
    let g = gcd(k as i64, nk as i64).unsigned_abs();
    let mut out = format!("Is Z/{}Z projective over Z/{}Z?\n", k, n);
    out.push_str(&format!("  k={}, n/k={}, gcd(k,n/k)={}\n", k, nk, g));
    if g == 1 {
        out.push_str(&format!("  YES: gcd({},{})=1, sequence splits.\n", k, nk));
        out.push_str(&format!("  Z/{}Z = Z/{}Z + Z/{}Z by CRT.\n", n, k, nk));
    } else {
        out.push_str(&format!("  NO: gcd({},{})={}>1, does not split.\n", k, nk, g));
    }
    out.push_str("  Projective over Z/nZ <=> direct summand of a free module.\n");
    out
}

fn cmd_injective_test(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let mut out = format!("Is Z/{}Z injective as Z-module?\n", n);
    out.push_str("  Over Z: injective <=> divisible (for every m, r*m'=m has solution).\n");
    out.push_str(&format!("  Z/{}Z has exponent {}: NOT divisible for n>1.\n", n, n));
    out.push_str(&format!("  Z/{}Z is NOT injective.\n", n));
    out.push_str("  Injective Z-modules: Q, Q/Z, Prufer p-groups Z[p^inf].\n");
    out
}

fn cmd_flat_test(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let factors = factor(n);
    let square_free = factors.iter().all(|&(_, e)| e == 1);
    let mut out = format!("Is Z/{}Z flat over Z?\n", n);
    out.push_str("  Flat <=> torsion-free for f.g. Z-modules.\n");
    out.push_str(&format!("  Z/{}Z is torsion (not torsion-free).\n", n));
    out.push_str("  Square-free check:\n");
    for &(p, e) in &factors {
        out.push_str(&format!("    {}: exponent {}  {}\n", p, e, if e == 1 { "OK" } else { "FAILS" }));
    }
    if square_free {
        out.push_str(&format!("  Z/{}Z IS FLAT (square-free n <=> Z/nZ semisimple).\n", n));
    } else {
        out.push_str(&format!("  Z/{}Z is NOT flat (n has repeated prime factors).\n", n));
        out.push_str(&format!("  Tor^1_Z(Z/{}Z, Z/{}Z) = Z/{}Z != 0.\n", n, n, n));
    }
    out
}

fn cmd_baer_criterion(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let mut out = format!("Baer's Criterion for Z/{}Z\n", n);
    out.push_str("  M injective <=> every map I->M (I ideal) extends to Z->M.\n");
    for k in 1u64..=6 {
        let g = gcd(k as i64, n as i64).unsigned_abs();
        if g == 1 {
            let t = (0..n).find(|&t| (k * t) % n == 1 % n);
            if let Some(t) = t {
                out.push_str(&format!("  f: ({}) -> Z/{}Z, f({})=1 extends: g(1)={} OK\n", k, n, k, t));
            } else {
                out.push_str(&format!("  f: ({}) -> Z/{}Z, f({})=1 CANNOT extend\n", k, n, k));
            }
        } else {
            out.push_str(&format!("  f: ({}) -> Z/{}Z, f({})=1 CANNOT extend (gcd={}>1)\n", k, n, k, g));
        }
    }
    out.push_str(&format!("  Z/{}Z FAILS Baer's criterion -- NOT injective.\n", n));
    out
}

fn cmd_splitting(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    let k = match parse_uint(args, 1, "k") { Some(x) => x, None => return String::new() };
    if n < 2 || k < 2 { return "n and k must be at least 2".to_string(); }
    let nk = n * k;
    let g = gcd(n as i64, k as i64).unsigned_abs();
    let mut out = format!("Does 0 -> Z/{}Z -> Z/{}Z -> Z/{}Z -> 0 split?\n", k, nk, n);
    out.push_str(&format!("  gcd({},{}) = {}\n", n, k, g));
    if g == 1 {
        out.push_str(&format!("  YES: gcd=1, sequence SPLITS.\n  Z/{}Z = Z/{}Z + Z/{}Z (CRT).\n", nk, n, k));
    } else {
        let l = lcm(n as i64, k as i64).unsigned_abs();
        out.push_str(&format!("  NO: gcd={}>1, does NOT split.\n  Max order in direct sum = lcm = {} < {}.\n", g, l, nk));
    }
    out.push_str("  SES 0->A->B->C->0 splits iff B = A + C.\n");
    out
}

fn cmd_pd(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    if n < 1 { return "n must be at least 1".to_string(); }
    let mut out = format!("Projective Dimension of Z/{}Z over Z\n", n);
    if n == 1 {
        out.push_str("  pd_Z(0) = 0 (zero module, trivially projective).\n");
    } else {
        out.push_str(&format!("  Resolution: 0 -> Z -[x{}]-> Z -[pi]-> Z/{}Z -> 0\n", n, n));
        out.push_str(&format!("  pd_Z(Z/{}Z) = 1\n", n));
        out.push_str(&format!("  Ext^0 = Hom(Z/{}Z, M) = M[{}] = {{x: {}x=0}}\n", n, n, n));
        out.push_str(&format!("  Ext^1 = M/{}M\n", n));
        out.push_str("  Ext^k = 0 for k >= 2.\n");
        out.push_str(&format!("  Tor^0 = Z/{}Z tensor M = M/{}M\n", n, n));
        out.push_str(&format!("  Tor^1 = M[{}]\n", n));
    }
    out.push_str("  gl.dim(Z) = 1 (hereditary ring).\n");
    out
}

fn cmd_demo() -> String {
    let mut out = String::from("Demo: Projective, Injective, Flat\n");
    out.push_str(&cmd_free_res(&["6"]));
    out.push_str(&cmd_flat_test(&["6"]));
    out.push_str(&cmd_splitting(&["2", "3"]));
    out.push_str(&cmd_injective_test(&["6"]));
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "res_n", 6);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "free_res"        => { let r = cmd_free_res(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"res_n",n as i64); } r }
        "projective_test" => { let r = cmd_projective_test(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"res_n",n as i64); } r }
        "injective_test"  => { let r = cmd_injective_test(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"res_n",n as i64); } r }
        "flat_test"       => { let r = cmd_flat_test(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"res_n",n as i64); } r }
        "baer_criterion"  => { let r = cmd_baer_criterion(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"res_n",n as i64); } r }
        "splitting"       => { let r = cmd_splitting(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"res_n",n as i64); } r }
        "pd"              => { let r = cmd_pd(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"res_n",n as i64); } r }
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
    let n = state_get_int(state, "res_n").unwrap_or(6);
    c.title(&format!("Free Resolution of Z/{}Z", n));
    c.subtitle("Projective resolution diagram", 44.0);
    // Draw the sequence: 0 -> Z -[xn]-> Z -[pi]-> Z/nZ -> 0
    let y = 200.0;
    let znz_label = format!("Z/{}Z", n);
    let objects: Vec<(f64, &str)> = vec![
        (80.0,  "0"),
        (220.0, "Z"),
        (380.0, "Z"),
        (540.0, &znz_label),
        (660.0, "0"),
    ];
    for &(x, label) in &objects {
        if label == "0" {
            c.text_bold(x, y, label, 14.0, colors::GREY, "middle");
        } else if label.starts_with('Z') && !label.contains('/') {
            c.rrect(x - 20.0, y - 16.0, 40.0, 32.0, 6.0, colors::LIGHT, colors::CYAN, 1.5);
            c.text_bold(x, y, label, 13.0, colors::DARK, "middle");
        } else {
            c.rrect(x - 30.0, y - 16.0, 60.0, 32.0, 6.0, colors::HEADER_FILL, colors::CYAN, 1.5);
            c.text_bold(x, y, label, 13.0, colors::DARK, "middle");
        }
    }
    let xn_label = format!("x{}", n);
    let arrows: Vec<(f64, f64, &str)> = vec![
        (80.0, 220.0, ""),
        (240.0, 360.0, &xn_label),
        (400.0, 510.0, "pi"),
        (570.0, 660.0, ""),
    ];
    for (x1, x2, label) in arrows {
        c.arrow_labeled(x1 + 5.0, y, x2 - 5.0, y, label, colors::DARK, 1.5);
    }
    // Annotations
    c.text(220.0, y + 50.0, "free rank 1", 10.0, colors::GREY, "middle");
    c.text(380.0, y + 50.0, "free rank 1", 10.0, colors::GREY, "middle");
    c.text(c.width/2.0, y + 90.0,
           &format!("pd_Z(Z/{}Z) = 1    Ext^1(Z/{}Z, M) = M/{}M", n, n, n),
           11.0, colors::DARK, "middle");
    // Square-free / flat info
    let factors = factor(n as u64);
    let sq_free = factors.iter().all(|&(_,e)| e == 1);
    c.text(c.width/2.0, y + 115.0,
           &format!("Flat? {}  (n {} square-free)", if sq_free { "YES" } else { "NO" }, if sq_free { "is" } else { "not" }),
           11.0, if sq_free { colors::GREEN } else { colors::RED }, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "res_n").unwrap_or(6);
    g.graph_attr("label", &format!("Projective resolution of Z/{}Z", n));
    g.graph_attr("rankdir", "LR");
    g.node_default("shape", "box");
    g.node_default("style", "rounded,filled");
    g.node("zero_l", &[("label", "0"), ("fillcolor", "#eeeeee")]);
    g.node("Z1", &[("label", "Z"), ("fillcolor", "#b2ebf2")]);
    g.node("Z2", &[("label", "Z"), ("fillcolor", "#b2ebf2")]);
    g.node("ZnZ", &[("label", &format!("Z/{}Z", n)), ("fillcolor", "#ffd54f")]);
    g.node("zero_r", &[("label", "0"), ("fillcolor", "#eeeeee")]);
    g.edge("zero_l", "Z1", &[]);
    g.edge("Z1", "Z2", &[("label", &format!("x{}", n))]);
    g.edge("Z2", "ZnZ", &[("label", "pi")]);
    g.edge("ZnZ", "zero_r", &[]);
    let factors = factor(n as u64);
    let sq_free = factors.iter().all(|&(_,e)| e == 1);
    g.node("props", &[("label", &format!("pd=1\nFlat: {}\nInjective: No", if sq_free { "Yes" } else { "No" })), ("shape", "note")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "res_n").unwrap_or(6);
    t.raw(&format!("  % Free resolution of Z/{}Z", n));
    t.use_library("arrows.meta");
    let objs: &[(&str, f64, &str)] = &[
        ("zl", -6.0, "0"),
        ("Z1", -3.0, "\\mathbb{Z}"),
        ("Z2",  0.0, "\\mathbb{Z}"),
        ("ZnZ", 3.5, &format!("\\mathbb{{Z}}/{}\\mathbb{{Z}}", n)),
        ("zr",  6.5, "0"),
    ];
    for &(id, x, label) in objs {
        if label == "0" {
            t.node_math(id, x, 0.0, "0", "font=\\small");
        } else {
            t.node_math(id, x, 0.0, label, "draw,rectangle,fill=cyan!20,rounded corners");
        }
    }
    t.arrow("zl", "Z1", "", "");
    t.arrow("Z1", "Z2", &format!("\\times {}", n), "");
    t.arrow("Z2", "ZnZ", "\\pi", "");
    t.arrow("ZnZ", "zr", "", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "res_n").unwrap_or(6);
    a.text_at(2, 0, &format!("Free Resolution of Z/{}Z", n));
    a.hline(2, 1, 50, '-');
    a.text_at(2, 2, &format!("0 -> Z -[x{}]-> Z -[pi]-> Z/{}Z -> 0", n, n));
    a.hline(2, 3, 50, '-');
    a.text_at(2, 4, "Both Z's are free modules (rank 1).");
    a.text_at(2, 5, &format!("pd_Z(Z/{}Z) = 1", n));
    a.text_at(2, 6, &format!("Ext^1(Z/{}Z, M) = M/{}M", n, n));
    a.text_at(2, 7, &format!("Tor^1(Z/{}Z, M) = M[{}] = {{x: {}x=0}}", n, n, n));
    let factors = factor(n as u64);
    let sq_free = factors.iter().all(|&(_,e)| e == 1);
    a.text_at(2, 8, &format!("Flat? {}  Injective? No", if sq_free { "YES (square-free n)" } else { "NO" }));
    a.hline(2, 9, 50, '-');
    a.text_at(2, 10, "gl.dim(Z) = 1 (hereditary)");
    a.text_at(2, 11, "Projective <=> free for f.g. Z-modules");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch26"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 26", "Free, Projective, and Injective Modules",
                         "Free resolutions, Baer's criterion, flatness, splitting sequences");
            println!("{}", help_text());
            repl("ch26> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
