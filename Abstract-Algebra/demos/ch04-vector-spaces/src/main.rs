use common::*;

// ── Vector Space Explorer ────────────────────────────────────────────────────

fn parse_vec(s: &str) -> Option<Vec<f64>> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.is_empty() { return None; }
    let v: Vec<f64> = parts.iter().filter_map(|p| p.parse().ok()).collect();
    if v.len() == parts.len() { Some(v) } else { None }
}

fn parse_vecs(args: &[&str]) -> Vec<Vec<f64>> {
    let joined = args.join(" ");
    joined.split(';')
        .filter_map(|s| parse_vec(s.trim()))
        .collect()
}

fn vec_add(a: &[f64], b: &[f64]) -> Option<Vec<f64>> {
    if a.len() != b.len() { return None; }
    Some(a.iter().zip(b.iter()).map(|(x, y)| x + y).collect())
}

fn vec_scale(s: f64, v: &[f64]) -> Vec<f64> {
    v.iter().map(|x| x * s).collect()
}

fn vec_to_str(v: &[f64]) -> String {
    let parts: Vec<String> = v.iter().map(|x| format!("{:.4}", x)).collect();
    format!("({})", parts.join(", "))
}

fn row_reduce_f64(mat: &mut Vec<Vec<f64>>) -> usize {
    let rows = mat.len();
    if rows == 0 { return 0; }
    let cols = mat[0].len();
    let mut pivot_row = 0;
    for col in 0..cols {
        let mut found = None;
        for r in pivot_row..rows {
            if mat[r][col].abs() > 1e-9 { found = Some(r); break; }
        }
        let pr = match found { None => continue, Some(r) => r };
        mat.swap(pivot_row, pr);
        let piv = mat[pivot_row][col];
        for c in 0..cols { mat[pivot_row][c] /= piv; }
        for r in 0..rows {
            if r == pivot_row { continue; }
            let factor = mat[r][col];
            for c in 0..cols {
                let sub = factor * mat[pivot_row][c];
                mat[r][c] -= sub;
            }
        }
        pivot_row += 1;
    }
    pivot_row
}

fn rank_of(vecs: &[Vec<f64>]) -> usize {
    if vecs.is_empty() { return 0; }
    let mut mat: Vec<Vec<f64>> = vecs.to_vec();
    row_reduce_f64(&mut mat)
}

fn are_independent(vecs: &[Vec<f64>]) -> bool {
    rank_of(vecs) == vecs.len()
}

fn find_zero_divisors(n: i64) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    for a in 2..n {
        for b in 2..n {
            if a < b && (a * b) % n == 0 { result.push((a, b)); }
        }
    }
    result
}

fn show_help() -> String {
    let mut out = String::new();
    out.push_str("Commands:\n");
    out.push_str("  add <v1> ; <v2>       vector addition\n");
    out.push_str("  scale <s> <v>         scalar multiplication\n");
    out.push_str("  span <vecs>           rank of the span\n");
    out.push_str("  indep <vecs>          test linear independence\n");
    out.push_str("  subspace <vecs>       check if span is a subspace\n");
    out.push_str("  field <p>             switch to mod p\n");
    out.push_str("  zero_div <n>          find zero divisors in Z/nZ\n");
    out.push_str("  demo                  showcase of vector space concepts\n");
    out.push_str("  quit                  exit\n");
    out.push_str("\nVectors: space-separated numbers. Multiple vectors: use ';'\n");
    out
}

// ── State helpers ─────────────────────────────────────────────────────────────

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "field_p", 0); // 0 = real numbers
    s
}

// ── run_cmd ───────────────────────────────────────────────────────────────────

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    let mut out = String::new();
    let current_field_p: Option<u64> = state_get_int(state, "field_p")
        .filter(|&p| p > 0)
        .map(|p| p as u64);

    match cmd {
        "help" | "h" => {
            out.push_str(&show_help());
        }

        "demo" => {
            out.push_str("=== Vector Space Demo ===\n\n");
            let e1 = vec![1.0, 0.0, 0.0];
            let e2 = vec![0.0, 1.0, 0.0];
            let e3 = vec![0.0, 0.0, 1.0];
            let dep = vec![1.0, 2.0, 3.0];
            let dep2 = vec![2.0, 4.0, 6.0];

            out.push_str("Standard basis {e1,e2,e3} for R^3:\n");
            out.push_str(&format!("  e1 = {}\n  e2 = {}\n  e3 = {}\n", vec_to_str(&e1), vec_to_str(&e2), vec_to_str(&e3)));
            out.push_str(&format!("Independent: {}\n\n", are_independent(&[e1.clone(), e2.clone(), e3.clone()])));

            out.push_str("Linearly dependent example:\n");
            out.push_str(&format!("  v1 = {}\n  v2 = {} = 2*v1\n", vec_to_str(&dep), vec_to_str(&dep2)));
            out.push_str(&format!("Rank: {} (only 1 independent vector)\n\n", rank_of(&[dep, dep2])));

            out.push_str("Zero divisors in Z/6Z:\n");
            for (a, b) in find_zero_divisors(6) {
                out.push_str(&format!("  {} x {} = {} ≡ 0 (mod 6)\n", a, b, a*b));
            }
            out.push_str("Z/6Z is NOT a field (has zero divisors).\n");
        }

        "field" => {
            if let Some(p) = parse_uint(args, 0, "p") {
                if is_prime(p) {
                    state_set_int(state, "field_p", p as i64);
                    out.push_str(&format!("Now working over F_{} = Z/{}Z\n", p, p));
                    out.push_str(&format!("F_{} has {} elements: {{0, 1, ..., {}}}\n", p, p, p-1));
                } else {
                    out.push_str(&format!("{} is not prime. Fields require prime order.\n", p));
                }
            }
        }

        "zero_div" => {
            let n = match parse_int(args, 0, "n") {
                Some(x) if x >= 2 => x,
                _ => return "n must be >= 2\n".to_string(),
            };
            if is_prime(n as u64) {
                out.push_str(&format!("{} is prime: Z/{}Z is a field (no zero divisors).\n", n, n));
            } else {
                let zdivs = find_zero_divisors(n);
                out.push_str(&format!("Z/{}Z zero divisors:\n", n));
                for (a, b) in &zdivs {
                    out.push_str(&format!("  {} x {} = {} ≡ 0 (mod {})\n", a, b, a*b, n));
                }
                if zdivs.is_empty() { out.push_str("None found.\n"); }
            }
        }

        "add" => {
            let vecs = parse_vecs(args);
            if vecs.len() < 2 { return "Usage: add <v1> ; <v2>\n".to_string(); }
            match vec_add(&vecs[0], &vecs[1]) {
                None => out.push_str(&format!("Dimension mismatch: {} ≠ {}\n", vecs[0].len(), vecs[1].len())),
                Some(result) => {
                    state_set_str(state, "last_op", "add");
                    state_set_int(state, "last_dim", vecs[0].len() as i64);
                    out.push_str(&format!("v1 + v2 = {} + {} = {}\n",
                        vec_to_str(&vecs[0]), vec_to_str(&vecs[1]), vec_to_str(&result)));
                    if let Some(p) = current_field_p {
                        let mod_r: Vec<f64> = result.iter().map(|&x| x.rem_euclid(p as f64)).collect();
                        out.push_str(&format!("(mod {}) = {}\n", p, vec_to_str(&mod_r)));
                    }
                }
            }
        }

        "scale" => {
            if args.is_empty() { return "Usage: scale <scalar> <vector...>\n".to_string(); }
            let s: f64 = match args[0].parse() {
                Ok(x) => x,
                Err(_) => return "First argument must be a scalar.\n".to_string(),
            };
            match parse_vec(&args[1..].join(" ")) {
                None => out.push_str("Could not parse vector.\n"),
                Some(v) => {
                    let result = vec_scale(s, &v);
                    out.push_str(&format!("{} * {} = {}\n", s, vec_to_str(&v), vec_to_str(&result)));
                }
            }
        }

        "span" | "rank" => {
            let vecs = parse_vecs(args);
            if vecs.is_empty() { return "Usage: span <v1> ; <v2> ; ...\n".to_string(); }
            let n = vecs[0].len();
            let r = rank_of(&vecs);
            state_set_str(state, "last_op", "span");
            state_set_int(state, "last_dim", n as i64);
            state_set_int(state, "last_rank", r as i64);
            out.push_str(&format!("Vectors: {}, rank: {}, ambient: R^{}\n", vecs.len(), r, n));
            if r == n {
                out.push_str(&format!("These {} vectors span all of R^{}!\n", vecs.len(), n));
            } else {
                out.push_str(&format!("Span is a {}-dimensional subspace of R^{}.\n", r, n));
            }
        }

        "indep" => {
            let vecs = parse_vecs(args);
            if vecs.is_empty() { return "Usage: indep <v1> ; <v2> ; ...\n".to_string(); }
            let r = rank_of(&vecs);
            state_set_str(state, "last_op", "indep");
            state_set_int(state, "last_rank", r as i64);
            state_set_int(state, "last_num_vecs", vecs.len() as i64);
            if are_independent(&vecs) {
                out.push_str(&format!("LINEARLY INDEPENDENT ({} vectors, rank {})\n", vecs.len(), r));
            } else {
                out.push_str(&format!("LINEARLY DEPENDENT ({} vectors, rank {})\n", vecs.len(), r));
                out.push_str(&format!("Only {} of {} are independent.\n", r, vecs.len()));
            }
        }

        "subspace" => {
            let vecs = parse_vecs(args);
            if vecs.is_empty() { return "Usage: subspace <v1> ; <v2> ; ...\n".to_string(); }
            let n = vecs[0].len();
            let r = rank_of(&vecs);
            let consistent = vecs.iter().all(|v| v.len() == n);
            if !consistent {
                out.push_str("Vectors have inconsistent dimensions.\n");
            } else {
                out.push_str("The span IS a subspace (closed under + and scalar mult).\n");
                out.push_str(&format!("Dimension: {}, ambient: R^{}\n", r, n));
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

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    c.title("Vector Space Basis");
    c.subtitle("Standard basis for R^2 and R^3", 42.0);

    // Draw R^2 coordinate axes
    let ox = 200.0; let oy = 280.0;
    c.arrow(ox, oy, ox + 120.0, oy, colors::DARK, 2.0);
    c.arrow(ox, oy, ox, oy - 120.0, colors::DARK, 2.0);
    c.text(ox + 125.0, oy, "x", 13.0, colors::DARK, "start");
    c.text(ox, oy - 125.0, "y", 13.0, colors::DARK, "middle");

    // e1
    c.arrow(ox, oy, ox + 80.0, oy, colors::BLUE, 2.5);
    c.text(ox + 40.0, oy + 16.0, "e1=(1,0)", 11.0, colors::BLUE, "middle");
    // e2
    c.arrow(ox, oy, ox, oy - 80.0, colors::GREEN, 2.5);
    c.text(ox - 45.0, oy - 40.0, "e2=(0,1)", 11.0, colors::GREEN, "middle");

    // A general vector
    c.arrow(ox, oy, ox + 60.0, oy - 70.0, colors::MAGENTA, 2.5);
    c.text(ox + 65.0, oy - 75.0, "v=(3,2)", 11.0, colors::MAGENTA, "start");
    c.dashed_line(ox + 60.0, oy, ox + 60.0, oy - 70.0, colors::GREY, 1.0, "4,3");
    c.dashed_line(ox, oy - 70.0, ox + 60.0, oy - 70.0, colors::GREY, 1.0, "4,3");

    c.text(200.0, 330.0, "v = 3*e1 + 2*e2", 12.0, colors::DARK, "middle");

    // R^3 box hint
    let bx = 450.0; let by = 120.0;
    c.text_bold(bx, by, "Basis for R^3", 14.0, colors::DARK, "middle");
    let basis3 = [("e1", "(1,0,0)"), ("e2", "(0,1,0)"), ("e3", "(0,0,1)")];
    let cols = [colors::BLUE, colors::GREEN, colors::MAGENTA];
    for (i, ((name, coords), col)) in basis3.iter().zip(cols.iter()).enumerate() {
        c.rrect(bx - 60.0, by + 30.0 + i as f64 * 45.0, 120.0, 35.0, 4.0,
            colors::LIGHT, *col, 1.5);
        c.text(bx, by + 50.0 + i as f64 * 45.0,
            &format!("{} = {}", name, coords), 12.0, colors::DARK, "middle");
    }
    c.text(bx, by + 30.0 + 3.0 * 45.0 + 10.0,
        "Any 3 independent vectors form a basis", 11.0, colors::GREY, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node_default("shape", "box");
    g.node_default("style", "filled,rounded");

    g.node("V",  &[("label", "Vector Space V"), ("fillcolor", "#b2ebf2")]);
    g.node("W",  &[("label", "Subspace W"), ("fillcolor", "#c8e6c9")]);
    g.node("sp", &[("label", "span{v1,...,vk}"), ("fillcolor", "#fff9c4")]);
    g.node("0",  &[("label", "{0}"), ("fillcolor", "#fce4ec")]);

    g.edge("W", "V",  &[("label", "subset")]);
    g.edge("sp", "W", &[("label", "spans")]);
    g.edge("0",  "W", &[("label", "trivial subspace")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.use_library("arrows.meta");
    t.node("O",  0.0, 0.0, "O", "draw,circle,minimum size=0.3cm,fill=black");
    t.node("e1", 3.0, 0.0, "$e_1$", "draw,circle,fill=blue!20");
    t.node("e2", 0.0, 3.0, "$e_2$", "draw,circle,fill=green!20");
    t.node("v",  2.0, 1.5, "$v = 2e_1 + 1.5e_2$", "draw,circle,fill=purple!20");
    t.arrow("O", "e1", "1", "-stealth,blue,thick");
    t.arrow("O", "e2", "1", "-stealth,green,thick");
    t.arrow("O", "v",  "", "-stealth,purple,thick");
    t.raw("  \\draw[dashed,gray] (2,0) -- (2,1.5);");
    t.raw("  \\draw[dashed,gray] (0,1.5) -- (2,1.5);");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 0, "Vector Space: Basis Diagram");
    a.text_at(0, 2, "Standard basis for R^3:");
    a.text_at(2, 4, "e1 = (1, 0, 0)");
    a.text_at(2, 5, "e2 = (0, 1, 0)");
    a.text_at(2, 6, "e3 = (0, 0, 1)");

    a.text_at(0, 8, "Any vector v = (a,b,c) = a*e1 + b*e2 + c*e3");

    a.text_at(0, 10, "Independence test (rank = number of vectors):");
    let vecs = vec![vec![1.0,0.0,0.0], vec![0.0,1.0,0.0], vec![0.0,0.0,1.0]];
    a.text_at(2, 11, &format!("rank{{e1,e2,e3}} = {}  (independent)", rank_of(&vecs)));

    let dep = vec![vec![1.0,2.0,3.0], vec![2.0,4.0,6.0]];
    a.text_at(2, 12, &format!("rank{{(1,2,3),(2,4,6)}} = {}  (dependent!)", rank_of(&dep)));

    a.hline(0, 14, 60, '-');
    a.text_at(0, 15, "Subspace axioms: 0 in W, closed under + and scalar mult");
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
                    let mut c = SvgCanvas::new(700.0, 400.0);
                    visualize_svg(&mut c, cmd, &args_ref, &state);
                    c.build()
                }
                OutputFormat::Dot => {
                    let mut g = DotGraph::digraph("ch04");
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
            print_banner("Chapter 4", "Fields and Vector Spaces",
                "Explore vector operations, span, independence, and fields");
            print_info("Type 'help' for commands.");
            print_note("Try: indep 1 0 0 ; 0 1 0 ; 0 0 1");
            repl("ch4> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
