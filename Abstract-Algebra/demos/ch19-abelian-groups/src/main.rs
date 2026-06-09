use common::*;

fn help_text() -> String {
    help_string(&[
        ("classify <n>",              "all abelian groups of order n"),
        ("smith <r> <c> <entries>",   "Smith normal form of integer matrix"),
        ("invariant_factors <d...>",  "write group from invariant factors"),
        ("primary <d...>",            "convert invariant factors to primary decomp"),
        ("is_cyclic <n>",             "check if Z/nZ is cyclic (it always is)"),
        ("homology <entries>",        "H_1 of simplicial 1-complex (boundary matrix row-by-row)"),
        ("all_to_30",                 "list all abelian groups of orders 1-30"),
        ("demo",                      "showcase: classify several orders"),
        ("help",                      "show this help"),
        ("quit",                      "exit"),
    ])
}

// Partitions of a positive integer n
fn partitions(n: u64) -> Vec<Vec<u64>> {
    if n == 0 { return vec![vec![]]; }
    let mut result = Vec::new();
    fn helper(n: u64, max: u64, current: &mut Vec<u64>, result: &mut Vec<Vec<u64>>) {
        if n == 0 { result.push(current.clone()); return; }
        for k in (1..=n.min(max)).rev() {
            current.push(k);
            helper(n - k, k, current, result);
            current.pop();
        }
    }
    helper(n, n, &mut vec![], &mut result);
    result
}

// Abelian groups of order n: for each prime power p^e, take partitions of e
fn abelian_groups_of_order(n: u64) -> Vec<String> {
    let factors = factor(n);
    if factors.is_empty() { return vec!["trivial group {0}".to_string()]; }

    let components: Vec<Vec<Vec<u64>>> = factors.iter().map(|&(p, e)| {
        partitions(e as u64).into_iter().map(|part| {
            part.iter().map(|&k| {
                let mut pk = 1u64;
                for _ in 0..k { pk *= p; }
                pk
            }).collect::<Vec<u64>>()
        }).collect()
    }).collect();

    let mut groups: Vec<Vec<u64>> = vec![vec![]];
    for comp in &components {
        let mut new_groups = Vec::new();
        for g in &groups {
            for part in comp {
                let mut ng = g.clone();
                ng.extend_from_slice(part);
                new_groups.push(ng);
            }
        }
        groups = new_groups;
    }

    groups.iter().map(|g| {
        if g.is_empty() || (g.len() == 1 && g[0] == 1) {
            return "{ 0 } (trivial)".to_string();
        }
        let parts: Vec<String> = g.iter().filter(|&&x| x > 1).map(|&x| format!("Z/{}", x)).collect();
        if parts.is_empty() { "{ 0 }".to_string() }
        else { parts.join(" + ") }
    }).collect()
}

fn cmd_classify(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    if n == 0 { return "n must be positive".to_string(); }
    let mut out = String::new();
    out.push_str(&format!("Abelian groups of order {}:\n", n));
    let factors = factor(n);
    out.push_str(&format!("  {} = {}\n", n,
        factors.iter().map(|&(p,e)| if e==1 { format!("{}",p) } else { format!("{}^{}",p,e) })
               .collect::<Vec<_>>().join(" * ")));
    let groups = abelian_groups_of_order(n);
    for (i, g) in groups.iter().enumerate() {
        out.push_str(&format!("  {}. {}\n", i + 1, g));
    }
    out.push_str(&format!("There are {} abelian group(s) of order {} up to isomorphism.\n", groups.len(), n));
    out
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
    let mut out = format!("Input Matrix:\n{}", mat);
    let (smith, ops) = mat.smith_normal_form();
    out.push_str("Operations:\n");
    for op in &ops { out.push_str(&format!("  {}\n", op)); }
    out.push_str(&format!("Smith Normal Form:\n{}", smith));
    let min_dim = rows.min(cols);
    let inv: Vec<i64> = (0..min_dim).map(|k| smith[(k,k)]).filter(|&v| v != 0).collect();
    if inv.is_empty() {
        out.push_str("All diagonal entries are 0 (zero matrix).\n");
    } else {
        out.push_str(&format!("Invariant factors: {}\n", inv.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" | ")));
        let chain_ok = inv.windows(2).all(|w| w[1] % w[0] == 0);
        if chain_ok { out.push_str("Divisibility chain d_1 | d_2 | ... | d_k holds.\n"); }
    }
    out.push_str("Module Z^m / Im(A) = (direct sum Z/d_i) + Z^(free part).\n");
    out
}

fn cmd_invariant_factors(args: &[&str]) -> String {
    if args.is_empty() { return "Provide invariant factors d1 d2 ...".to_string(); }
    let factors: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
    if factors.is_empty() { return "No valid integers given.".to_string(); }
    let mut out = format!("Invariant Factors: {}\n", factors.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" | "));
    for i in 1..factors.len() {
        if factors[i] % factors[i-1] != 0 {
            out.push_str(&format!("Warning: d_{} = {} does not divide d_{} = {}!\n", i, factors[i-1], i+1, factors[i]));
        }
    }
    let parts: Vec<String> = factors.iter().map(|&d| {
        if d == 0 { "Z".to_string() } else { format!("Z/{}", d) }
    }).collect();
    out.push_str(&format!("G = {}\n", parts.join(" + ")));
    let order: i64 = factors.iter().filter(|&&d| d != 0).product();
    if factors.iter().all(|&d| d != 0) {
        out.push_str(&format!("|G| = {}\n", order));
    }
    out
}

fn cmd_primary(args: &[&str]) -> String {
    if args.is_empty() { return "Provide invariant factors d1 d2 ...".to_string(); }
    let factors: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
    if factors.is_empty() { return "No valid integers given.".to_string(); }
    let mut out = format!("Primary Decomposition from invariant factors: {}\n",
        factors.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" | "));
    let mut all_primary: Vec<String> = Vec::new();
    for &d in &factors {
        if d == 0 { all_primary.push("Z".to_string()); continue; }
        let pf = factor(d as u64);
        for (p, e) in &pf {
            let pk = p.pow(*e);
            all_primary.push(format!("Z/{}", pk));
        }
    }
    out.push_str(&format!("Primary decomp: {}\n", all_primary.join(" + ")));
    out
}

fn cmd_is_cyclic(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x, None => return String::new() };
    if n == 0 { return "n must be positive".to_string(); }
    let mut out = format!("Is Z/{}Z cyclic?\n", n);
    out.push_str(&format!("Z/{}Z is always cyclic - generated by 1.\n", n));
    let factors = factor(n);
    if factors.len() > 1 {
        let pairwise_coprime = factors.windows(2).all(|w| w[0].0 != w[1].0);
        if pairwise_coprime {
            let decomp: Vec<String> = factors.iter().map(|&(p, e)| format!("Z/{}", p.pow(e))).collect();
            out.push_str(&format!("By CRT: Z/{n} = {}\n", decomp.join(" + ")));
        }
    }
    out
}

fn cmd_homology(args: &[&str]) -> String {
    if args.is_empty() {
        return "Provide the boundary matrix as rows of integers.\nExample: homology 1 -1 0  0 1 -1  1 0 -1".to_string();
    }
    let entries: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
    let len = entries.len();
    let mut rows = 1usize;
    let mut cols = len;
    for r in 2..=len {
        if len % r == 0 { rows = r; cols = len / r; break; }
    }
    if rows * cols != len {
        return "Could not determine matrix dimensions.".to_string();
    }
    let mat = Mat::new(rows, cols, entries);
    let mut out = format!("H_1 of Simplicial 1-Complex ({}x{} boundary matrix):\n", rows, cols);
    out.push_str(&format!("{}", mat));
    let (smith, _) = mat.smith_normal_form();
    out.push_str(&format!("Smith Normal Form:\n{}", smith));
    let min_dim = rows.min(cols);
    let diag: Vec<i64> = (0..min_dim).map(|k| smith[(k,k)]).collect();
    let rank = diag.iter().filter(|&&v| v != 0).count();
    let torsion: Vec<i64> = diag.iter().filter(|&&v| v != 0 && v != 1 && v != -1).copied().collect();
    let free_rank = cols.saturating_sub(rank);
    let h1_free = rows.saturating_sub(rank);
    if torsion.is_empty() {
        out.push_str(&format!("H_1 = {}\n", if h1_free == 0 { "0".to_string() } else { format!("Z^{}", h1_free) }));
    } else {
        let tors_str: Vec<String> = torsion.iter().map(|&t| format!("Z/{}", t)).collect();
        let parts = if h1_free > 0 {
            format!("{} + Z^{}", tors_str.join(" + "), h1_free)
        } else { tors_str.join(" + ") };
        out.push_str(&format!("H_1 = {}\n", parts));
    }
    out.push_str(&format!("H_0 (connected components) = Z^{}\n", free_rank.max(1)));
    out
}

fn cmd_all_to_30() -> String {
    let mut out = String::from("All Abelian Groups of Orders 1-30:\n");
    for n in 1u64..=30 {
        let groups = abelian_groups_of_order(n);
        out.push_str(&format!("  n={:>2}: ({}) {}\n", n, groups.len(), groups.join(", ")));
    }
    out
}

fn cmd_demo() -> String {
    let mut out = String::from("Demo: Abelian Groups\n");
    for &n in &[1u64, 4, 6, 8, 12, 16, 24, 36] {
        let groups = abelian_groups_of_order(n);
        out.push_str(&format!("  |G|={:>2}: {}\n", n, groups.join("  |  ")));
    }
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "last_n", 12);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "classify"          => { let r = cmd_classify(args); if !r.is_empty() { if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } } r }
        "smith"             => { if let Some(r) = parse_uint(args,0,"r") { state_set_int(state,"last_n",r as i64); } cmd_smith(args) }
        "invariant_factors" => cmd_invariant_factors(args),
        "primary"           => cmd_primary(args),
        "is_cyclic"         => { let r = cmd_is_cyclic(args); if let Some(n) = parse_uint(args,0,"n") { state_set_int(state,"last_n",n as i64); } r }
        "homology"          => cmd_homology(args),
        "all_to_30"         => cmd_all_to_30(),
        "demo"              => cmd_demo(),
        "help" | "h"        => help_text(),
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
    let n = state_get_int(state, "last_n").unwrap_or(12) as u64;
    let groups = abelian_groups_of_order(n);
    c.title(&format!("Abelian groups of order {}", n));
    c.subtitle(&format!("{} group(s) up to isomorphism", groups.len()), 44.0);
    let cx = c.width / 2.0;
    let start_y = 80.0;
    let spacing = 60.0;
    for (i, g) in groups.iter().enumerate() {
        let y = start_y + i as f64 * spacing;
        c.rrect(cx - 160.0, y - 18.0, 320.0, 36.0, 8.0,
                colors::HEADER_FILL, colors::CYAN, 1.5);
        c.text_bold(cx, y, g, 13.0, colors::DARK, "middle");
        c.text(cx - 180.0, y, &format!("{}.", i+1), 11.0, colors::GREY, "middle");
    }
    // factor display
    let factors = factor(n);
    let fstr: Vec<String> = factors.iter().map(|&(p,e)| if e==1 { format!("{}",p) } else { format!("{}^{}",p,e) }).collect();
    c.text(cx, start_y + groups.len() as f64 * spacing + 20.0,
           &format!("{} = {}", n, fstr.join(" * ")), 12.0, colors::GREY, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "last_n").unwrap_or(12) as u64;
    let groups = abelian_groups_of_order(n);
    g.graph_attr("label", &format!("Abelian groups of order {}", n));
    g.node_default("shape", "box");
    g.node_default("style", "rounded,filled");
    g.node_default("fillcolor", "#b2ebf2");
    g.node("root", &[("label", &format!("|G|={}", n)), ("shape", "ellipse"), ("fillcolor", "#ffd54f")]);
    for (i, grp) in groups.iter().enumerate() {
        let id = format!("g{}", i);
        g.node(&id, &[("label", grp.as_str())]);
        g.edge("root", &id, &[]);
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "last_n").unwrap_or(12) as u64;
    let groups = abelian_groups_of_order(n);
    t.raw(&format!("  % Abelian groups of order {}", n));
    t.node_math("root", 0.0, 0.0, &format!("|G|={}", n), "draw,ellipse,fill=yellow!30");
    for (i, grp) in groups.iter().enumerate() {
        let x = (i as f64 - (groups.len() as f64 - 1.0)/2.0) * 3.5;
        let clean = grp.replace('+', "\\oplus ").replace('Z', "\\mathbb{Z}");
        t.node_math(&format!("g{}", i), x, -2.0,
                    &clean, "draw,rounded corners,fill=cyan!20");
        t.arrow("root", &format!("g{}", i), "", "gray");
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "last_n").unwrap_or(12) as u64;
    let groups = abelian_groups_of_order(n);
    a.text_at(2, 0, &format!("Abelian groups of order {}", n));
    a.hline(2, 1, 40, '-');
    for (i, grp) in groups.iter().take(a.height - 3).enumerate() {
        a.text_at(2, (i + 2) as i32, &format!("{}. {}", i+1, grp));
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch19"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 19", "Finitely Generated Abelian Groups",
                         "Structure theorem, Smith normal form, primary decomposition");
            println!("{}", help_text());
            repl("ch19> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
