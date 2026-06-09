use common::*;

fn help_text() -> String {
    help_string(&[
        ("zmod <n>",          "Z/nZ as Z-module: submodules, torsion, annihilators"),
        ("torsion <n>",       "torsion elements of Z/nZ (all of them, n*x=0)"),
        ("annihilator <n> <a>","Ann(a) = {r in Z : r*a=0} in Z/nZ"),
        ("submodules <n>",    "all submodules (sub-abelian-groups) of Z/nZ"),
        ("hom <n> <m>",       "Hom_Z(Z/nZ, Z/mZ) = Z/gcd(n,m)Z; show all maps"),
        ("direct_sum <n> <m>","Z/nZ + Z/mZ: elements, submodules, torsion"),
        ("free <rank>",       "free Z-module Z^r: elements, rank"),
        ("simple <n>",        "simple Z-modules with n generators"),
        ("demo",              "showcase module examples"),
        ("help",              "show this help"),
        ("quit",              "exit"),
    ])
}

fn cmd_zmod_module(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let submodules = subgroups_zn(n);
    let mut out = format!("Z/{}Z as Z-Module\n", n);
    out.push_str("  Z acts by: r * [a] = [ra] mod n\n");
    out.push_str("  Submodules:\n");
    for sub in &submodules {
        let gen = sub.iter().min().copied().unwrap_or(0);
        out.push_str(&format!("    <{}> = {{{}}}\n", gen, sub.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    }
    out.push_str(&format!("  Ann(Z/{}Z) = {}Z\n", n, n));
    out.push_str(&format!("  Z/{}Z is entirely torsion (exponent n).\n", n));
    out
}

fn cmd_torsion(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let all: Vec<usize> = (0..n).collect();
    let mut out = format!("Torsion Elements of Z/{}Z: all {} elements\n", n, n);
    out.push_str(&format!("  {{{}}}\n", all.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    out.push_str("  Individual annihilators:\n");
    for a in 0..n.min(8) {
        let ann = n / gcd(a as i64, n as i64).unsigned_abs() as usize;
        if a == 0 { out.push_str("    Ann(0) = Z\n"); }
        else { out.push_str(&format!("    Ann({}) = {}Z  (ord({})={})\n", a, ann, a, ann)); }
    }
    out
}

fn cmd_annihilator(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    let a = match parse_uint(args, 1, "a") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let a_mod = a % n;
    let g = gcd(a_mod as i64, n as i64).unsigned_abs() as usize;
    let ann_gen = n / g.max(1);
    let ann_in_zn: Vec<usize> = (0..n).filter(|&r| (r * a_mod) % n == 0).collect();
    let mut out = format!("Ann({}) in Z/{}Z\n  gcd({},{})={}\n  Ann = {}Z\n  Elements: {{{}}}\n",
        a_mod, n, a_mod, n, g, ann_gen,
        ann_in_zn.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
    for &r in &ann_in_zn[..ann_in_zn.len().min(4)] {
        out.push_str(&format!("  {} * {} = {} = {} mod {}\n", r, a_mod, r*a_mod, (r*a_mod)%n, n));
    }
    out
}

fn cmd_submodules(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    if n < 2 { return "n must be at least 2".to_string(); }
    let divs = divisors(n as u64);
    let mut out = format!("Submodules of Z/{}Z\n", n);
    for &d in &divs {
        let sub: Vec<usize> = (0..n).filter(|&x| x as u64 % d == 0).collect();
        let gen = if d == 1 { 0 } else { (n as u64 / d) as usize };
        out.push_str(&format!("  <{}> = Z/{}Z  size {}  {{{}}}\n",
            gen, n / gen.max(1), sub.len(),
            sub.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    }
    out.push_str("  Containment lattice:\n");
    for &d1 in &divs {
        for &d2 in &divs {
            if d1 < d2 && d2 % d1 == 0 {
                out.push_str(&format!("    <{}> < <{}>\n", n / d2 as usize, n / d1 as usize));
            }
        }
    }
    out.push_str(&format!("  {} submodules total (= number of divisors of n).\n", divs.len()));
    out
}

fn cmd_hom(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    let m = match parse_uint(args, 1, "m") { Some(x) => x as usize, None => return String::new() };
    if n < 1 || m < 1 { return "n and m must be positive".to_string(); }
    let g = gcd(n as i64, m as i64).unsigned_abs() as usize;
    let valid_k: Vec<usize> = (0..m).filter(|&k| (n * k) % m == 0).collect();
    let mut out = format!("Hom_Z(Z/{}Z, Z/{}Z) = Z/{}Z\n", n, m, g);
    out.push_str(&format!("  Valid f(1) values in Z/{}Z: {{{}}}\n", m,
        valid_k.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    for &k in valid_k.iter().take(6) {
        out.push_str(&format!("    phi_{}: [x] -> [{}x mod {}]\n", k, k, m));
    }
    out.push_str(&format!("  |Hom| = {} = gcd({},{}) confirmed.\n", valid_k.len(), n, m));
    out
}

fn cmd_direct_sum(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    let m = match parse_uint(args, 1, "m") { Some(x) => x as usize, None => return String::new() };
    if n < 2 || m < 2 { return "n and m must be at least 2".to_string(); }
    let total = n * m;
    let l = lcm(n as i64, m as i64).unsigned_abs() as usize;
    let g = gcd(n as i64, m as i64).unsigned_abs() as usize;
    let mut out = format!("Z/{}Z + Z/{}Z  (direct sum)\n  Total elements: {}\n", n, m, total);
    out.push_str("  First elements: ");
    let mut count = 0;
    let mut sample = Vec::new();
    'outer: for a in 0..n {
        for b in 0..m {
            sample.push(format!("({},{})", a, b));
            count += 1;
            if count >= 8 { break 'outer; }
        }
    }
    out.push_str(&sample.join(", "));
    if total > 8 { out.push_str(" ..."); }
    out.push('\n');
    out.push_str(&format!("  Ann(M) = lcm({},{}) = {}Z\n", n, m, l));
    if g == 1 {
        out.push_str(&format!("  gcd({},{})=1: M = Z/{}Z (cyclic by CRT).\n", n, m, n*m));
    } else {
        out.push_str(&format!("  gcd({},{})={}>1: M is NOT cyclic.\n", n, m, g));
    }
    out
}

fn cmd_free(args: &[&str]) -> String {
    let rank = match parse_uint(args, 0, "rank") { Some(x) => x as usize, None => return String::new() };
    if rank == 0 { return "rank must be positive".to_string(); }
    if rank > 4 { return "rank too large (max 4)".to_string(); }
    let mut out = format!("Free Z-Module Z^{}\n  Rank: {}, no torsion.\n", rank, rank);
    out.push_str("  Standard basis:\n");
    for i in 0..rank {
        let mut e = vec![0i64; rank];
        e[i] = 1;
        out.push_str(&format!("    e_{} = ({})\n", i+1, e.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    }
    out.push_str("  Z^r: free, projective, torsion-free.\n");
    out
}

fn cmd_simple(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    if n < 1 { return "n must be at least 1".to_string(); }
    let mut out = format!("Simple Z-Modules (n={} generator(s))\n", n);
    out.push_str("  Simple Z-modules = Z/pZ for primes p.\n");
    if n == 1 {
        let primes = primes_up_to(30);
        for &p in &primes[..primes.len().min(8)] {
            out.push_str(&format!("  Z/{}Z: simple (no proper submodules).\n", p));
        }
        out.push_str("\n  Non-simple examples:\n");
        for &c in &[4usize, 6, 8, 9, 12] {
            out.push_str(&format!("  Z/{}Z: {} submodules -- NOT simple.\n", c, divisors(c as u64).len()));
        }
    } else {
        out.push_str("  Multi-generator: quotient is simple iff Z^n/(rels) = Z/pZ.\n");
    }
    out
}

fn cmd_demo() -> String {
    let mut out = String::from("Demo: Modules over Rings\n");
    out.push_str(&cmd_submodules(&["12"]));
    out.push_str(&cmd_hom(&["4", "6"]));
    out.push_str(&cmd_direct_sum(&["2", "3"]));
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "mod_n", 12);
    state_set_int(&mut s, "mod_m", 4);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "zmod"        => { let r = cmd_zmod_module(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"mod_n",n as i64); } r }
        "torsion"     => { let r = cmd_torsion(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"mod_n",n as i64); } r }
        "annihilator" => { let r = cmd_annihilator(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"mod_n",n as i64); } r }
        "submodules"  => { let r = cmd_submodules(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"mod_n",n as i64); } r }
        "hom"         => { let r = cmd_hom(args);
            if let (Some(n), Some(m)) = (parse_uint(args,0,"n"), parse_uint(args,1,"m")) {
                state_set_int(state,"mod_n",n as i64); state_set_int(state,"mod_m",m as i64);
            } r }
        "direct_sum"  => { let r = cmd_direct_sum(args);
            if let (Some(n), Some(m)) = (parse_uint(args,0,"n"), parse_uint(args,1,"m")) {
                state_set_int(state,"mod_n",n as i64); state_set_int(state,"mod_m",m as i64);
            } r }
        "free"        => { let r = cmd_free(args); if let Some(rank) = parse_uint(args,0,"rank") { state_set_int(state,"mod_n",rank as i64); } r }
        "simple"      => { let r = cmd_simple(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"mod_n",n as i64); } r }
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
    let n = state_get_int(state, "mod_n").unwrap_or(12) as usize;
    c.title(&format!("R-Module Z/{}Z with Submodules", n));
    c.subtitle("Submodule lattice (Hasse diagram)", 44.0);
    // Draw M at top
    let cx = c.width / 2.0;
    c.rrect(cx - 40.0, 60.0, 80.0, 32.0, 8.0, colors::HEADER_FILL, colors::CYAN, 1.5);
    c.text_bold(cx, 76.0, &format!("M=Z/{}Z", n), 13.0, colors::DARK, "middle");
    // Submodules = divisors
    let divs = divisors(n as u64);
    let num = divs.len();
    let level_of = |d: u64| -> u32 { factor(d).iter().map(|&(_,e)| e).sum() };
    let max_level = divs.iter().map(|&d| level_of(d)).max().unwrap_or(0);
    let pos: Vec<(f64, f64)> = divs.iter().enumerate().map(|(i, &d)| {
        let lv = level_of(d) as f64;
        let frac = if max_level > 0 { lv / max_level as f64 } else { 0.5 };
        let y = 110.0 + frac * 300.0;
        let x = 60.0 + (i as f64 + 0.5) * (c.width - 120.0) / num as f64;
        (x, y)
    }).collect();
    // Arrow from M to each top-level submodule
    for (i, &d) in divs.iter().enumerate() {
        if d == *divs.last().unwrap_or(&1) {
            c.arrow(cx, 92.0, pos[i].0, pos[i].1 - 20.0, colors::GREY, 1.0);
        }
    }
    for (i, &d1) in divs.iter().enumerate() {
        for (j, &d2) in divs.iter().enumerate() {
            if d2 % d1 == 0 && d2 != d1 {
                let intermediate = divs.iter().any(|&dm| dm != d1 && dm != d2 && d2 % dm == 0 && dm % d1 == 0);
                if !intermediate {
                    c.line(pos[i].0, pos[i].1, pos[j].0, pos[j].1, colors::GREY, 0.8);
                }
            }
        }
    }
    for (i, &d) in divs.iter().enumerate() {
        let (x, y) = pos[i];
        let gen = n as u64 / d;
        let fill = if d == 1 { colors::HEADER_FILL } else { colors::LIGHT };
        c.circle(x, y, 18.0, fill, colors::CYAN, 1.5);
        c.text_bold(x, y - 5.0, &format!("<{}>", gen), 9.0, colors::DARK, "middle");
        c.text(x, y + 6.0, &format!("size {}", d), 8.0, colors::GREY, "middle");
    }
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "mod_n").unwrap_or(12) as usize;
    let m = state_get_int(state, "mod_m").unwrap_or(4) as usize;
    g.graph_attr("label", &format!("Hom_Z(Z/{}Z, Z/{}Z)", n, m));
    g.graph_attr("rankdir", "LR");
    g.node_default("style", "filled");
    g.node("source", &[("label", &format!("Z/{}Z", n)), ("fillcolor", "#ffd54f"), ("shape", "ellipse")]);
    g.node("target", &[("label", &format!("Z/{}Z", m)), ("fillcolor", "#b2ebf2"), ("shape", "ellipse")]);
    let valid_k: Vec<usize> = (0..m).filter(|&k| (n * k) % m == 0).collect();
    for &k in &valid_k {
        let id = format!("phi_{}", k);
        g.node(&id, &[("label", &format!("phi_{}\n1->[{}]", k, k)), ("fillcolor", "#e8f5e9")]);
        g.edge("source", &id, &[]);
        g.edge(&id, "target", &[]);
    }
    let gc = gcd(n as i64, m as i64).unsigned_abs();
    g.node("hom", &[("label", &format!("Hom=Z/{}Z\n({} maps)", gc, valid_k.len())), ("shape", "diamond"), ("fillcolor", "#ffcc80")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "mod_n").unwrap_or(12) as usize;
    t.raw(&format!("  % R-Module Z/{}Z submodule lattice", n));
    t.node_math("M", 0.0, 3.0,
                &format!("M=\\mathbb{{Z}}/{}\\mathbb{{Z}}", n), "draw,ellipse,fill=yellow!30");
    let divs = divisors(n as u64);
    let num = divs.len();
    let level_of = |d: u64| -> u32 { factor(d).iter().map(|&(_,e)| e).sum() };
    let max_level = divs.iter().map(|&d| level_of(d)).max().unwrap_or(1);
    for (i, &d) in divs.iter().enumerate() {
        let gen = n as u64 / d;
        let lv = level_of(d) as f64;
        let y = if max_level > 0 { lv / max_level as f64 * 2.5 } else { 0.0 };
        let x = (i as f64 - (num as f64 - 1.0)/2.0) * 2.0;
        t.node_math(&format!("s{}", d), x, y,
                    &format!("\\langle {}\\rangle", gen), "draw,circle,fill=cyan!20,minimum size=1cm");
        if d == *divs.last().unwrap_or(&1) {
            t.arrow("M", &format!("s{}", d), "", "gray");
        }
    }
    for &d1 in &divs {
        for &d2 in &divs {
            if d2 % d1 == 0 && d2 != d1 {
                let intermediate = divs.iter().any(|&dm| dm != d1 && dm != d2 && d2 % dm == 0 && dm % d1 == 0);
                if !intermediate {
                    t.edge(&format!("s{}", d1), &format!("s{}", d2), "", "gray");
                }
            }
        }
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "mod_n").unwrap_or(12) as usize;
    let m = state_get_int(state, "mod_m").unwrap_or(4) as usize;
    a.text_at(2, 0, &format!("Module Z/{}Z", n));
    a.hline(2, 1, 35, '-');
    a.text_at(2, 2, "Submodules (generators):");
    let divs = divisors(n as u64);
    let subs: Vec<String> = divs.iter().map(|&d| format!("<{}>", n as u64 / d)).collect();
    a.text_at(2, 3, &subs.join(" > "));
    a.hline(2, 4, 35, '-');
    let gc = gcd(n as i64, m as i64).unsigned_abs();
    a.text_at(2, 5, &format!("Hom(Z/{}Z, Z/{}Z) = Z/{}Z", n, m, gc));
    let valid: Vec<usize> = (0..m).filter(|&k| (n*k)%m == 0).collect();
    let valid_str: Vec<String> = valid.iter().map(|k| format!("phi_{}", k)).collect();
    a.text_at(2, 6, &format!("  Maps: {}", valid_str.join(", ")));
    let l = lcm(n as i64, m as i64).unsigned_abs();
    a.text_at(2, 7, &format!("Direct sum Z/{}Z+Z/{}Z: ann=lcm={}Z", n, m, l));
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch25"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 25", "Modules over Rings",
                         "Z-modules, submodules, annihilators, Hom, direct sums");
            println!("{}", help_text());
            repl("ch25> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
