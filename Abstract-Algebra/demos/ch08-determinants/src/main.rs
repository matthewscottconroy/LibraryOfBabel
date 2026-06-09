use common::*;

// ── Determinant Explorer ─────────────────────────────────────────────────────

fn parse_mat(entries: &[&str], rows: usize, cols: usize) -> Option<Mat> {
    if entries.len() != rows * cols {
        return None;
    }
    let data: Vec<i64> = match entries.iter().map(|s| s.parse::<i64>()).collect::<Result<_,_>>() {
        Ok(v) => v, Err(_) => return None,
    };
    Some(Mat::new(rows, cols, data))
}

// ── Cofactor expansion ────────────────────────────────────────────────────────

fn det_with_steps_str(m: &Mat, depth: usize) -> (i64, String) {
    let n = m.rows;
    let indent = "  ".repeat(depth + 1);
    let mut steps = String::new();

    if n == 1 { return (m[(0,0)], String::new()); }

    if n == 2 {
        let d = m[(0,0)] * m[(1,1)] - m[(0,1)] * m[(1,0)];
        if depth == 0 {
            steps.push_str(&format!("{}det = {}*{} - {}*{} = {}\n",
                indent, m[(0,0)], m[(1,1)], m[(0,1)], m[(1,0)], d));
        }
        return (d, steps);
    }

    if depth == 0 {
        steps.push_str(&format!("{}Expanding along row 1:\n", indent));
    }
    let mut total = 0i64;
    for j in 0..n {
        let sign = if j % 2 == 0 { 1i64 } else { -1i64 };
        let entry = m[(0, j)];
        if entry == 0 {
            if depth == 0 {
                steps.push_str(&format!("{}  + ({}) * {} * M[0,{}] = 0  (skipped)\n", indent, sign, entry, j));
            }
            continue;
        }
        let minor = m.minor(0, j);
        let (minor_det, sub_steps) = if n - 1 <= 2 {
            (minor.det(), String::new())
        } else {
            det_with_steps_str(&minor, depth + 1)
        };
        let contribution = sign * entry * minor_det;
        total += contribution;
        if depth == 0 {
            let sign_str = if sign == 1 { "+" } else { "-" };
            steps.push_str(&format!("{}  {} {} * det(M[0,{}]) = {} * {} * {} = {}\n",
                indent, sign_str, entry.abs(), j, sign, entry, minor_det, contribution));
            steps.push_str(&sub_steps);
        }
    }
    (total, steps)
}

fn det_str(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 2 { return "Usage: det <rows> <cols> <entries...>\n".to_string(); }
    let rows: usize = match args[0].parse() { Ok(r) if r > 0 => r, _ => return "rows must be positive\n".to_string() };
    let cols: usize = match args[1].parse() { Ok(c) if c > 0 => c, _ => return "cols must be positive\n".to_string() };
    if rows != cols { return "Determinant only defined for square matrices.\n".to_string(); }
    let m = match parse_mat(&args[2..], rows, cols) { Some(m) => m, None => return format!("Expected {} entries.\n", rows*cols) };

    out.push_str(&format!("{}×{} matrix:\n", rows, cols));
    out.push_str(&format!("{}", m));
    out.push('\n');

    if rows == 1 {
        out.push_str(&format!("det = {}\n", m[(0,0)]));
        return out;
    }
    if rows == 2 {
        let d = m[(0,0)]*m[(1,1)] - m[(0,1)]*m[(1,0)];
        out.push_str(&format!("det = {}*{} - {}*{} = {}\n",
            m[(0,0)], m[(1,1)], m[(0,1)], m[(1,0)], d));
        return out;
    }

    out.push_str("Cofactor expansion along row 1:\n\n");
    let (d, steps) = det_with_steps_str(&m, 0);
    out.push_str(&steps);
    out.push('\n');
    out.push_str(&format!("det = {}\n", d));
    if d == 0 {
        out.push_str("Matrix is SINGULAR (det = 0). Not invertible.\n");
    } else {
        out.push_str("Matrix is INVERTIBLE (det ≠ 0).\n");
    }
    out
}

fn area_str(v1x: f64, v1y: f64, v2x: f64, v2y: f64) -> String {
    let mut out = String::new();
    let det = v1x * v2y - v1y * v2x;
    let area = det.abs();
    out.push_str(&format!("v1 = ({:.3}, {:.3})\n", v1x, v1y));
    out.push_str(&format!("v2 = ({:.3}, {:.3})\n", v2x, v2y));
    out.push_str(&format!("det = {:.3}*{:.3} - {:.3}*{:.3} = {:.6}\n", v1x, v2y, v1y, v2x, det));
    out.push_str(&format!("Area of parallelogram = |det| = {:.6}\n", area));
    out.push_str(&format!("Area of triangle = {:.6}\n", area / 2.0));
    if det > 0.0 { out.push_str("det > 0: positive orientation.\n"); }
    else if det < 0.0 { out.push_str("det < 0: negative orientation.\n"); }
    else { out.push_str("det = 0: vectors are collinear.\n"); }
    out
}

fn vandermonde_str(xs: &[i64]) -> String {
    let n = xs.len();
    let mut out = String::new();
    let data: Vec<i64> = (0..n).flat_map(|i| {
        (0..n).map(move |j| {
            let mut v = 1i64;
            for _ in 0..j { v *= xs[i]; }
            v
        })
    }).collect();
    let v = Mat::new(n, n, data);
    out.push_str(&format!("Vandermonde V({}):\n", xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")));
    out.push_str(&format!("{}", v));
    let det = v.det();
    out.push_str(&format!("\ndet(V) = {}\n", det));

    let mut product = 1i64;
    let mut terms = Vec::new();
    for i in 0..n {
        for j in 0..i {
            let diff = xs[i] - xs[j];
            terms.push(format!("({}-{})={}", xs[i], xs[j], diff));
            product *= diff;
        }
    }
    out.push_str(&format!("Product ∏(xi-xj) = {}\n", terms.join(" * ")));
    out.push_str(&format!("= {}\n", product));
    if det == product { out.push_str("det(V) = ∏(xi-xj) ✓\n"); }
    out
}

fn cramer_str(entries: &[i64], b: &[i64]) -> String {
    let n = b.len();
    let mut out = String::new();
    if n < 2 || n > 3 || entries.len() != n*n {
        return "Provide 2×2 or 3×3 system.\n".to_string();
    }
    let a = Mat::new(n, n, entries.to_vec());
    out.push_str("System Ax = b:\n");
    out.push_str(&format!("{}", a));
    out.push_str(&format!("b = {:?}\n\n", b));
    let det_a = a.det();
    out.push_str(&format!("det(A) = {}\n", det_a));
    if det_a == 0 { return out + "det(A) = 0: Cramer's rule does not apply.\n"; }

    for k in 0..n {
        let mut mod_data = entries.to_vec();
        for i in 0..n { mod_data[i*n+k] = b[i]; }
        let ak = Mat::new(n, n, mod_data);
        let det_ak = ak.det();
        out.push_str(&format!("A_{} (col {} replaced):\n{}", k+1, k+1, ak));
        out.push_str(&format!("det(A_{}) = {}\n", k+1, det_ak));
        if det_ak % det_a == 0 {
            out.push_str(&format!("x{} = {}/{} = {}\n\n", k+1, det_ak, det_a, det_ak/det_a));
        } else {
            out.push_str(&format!("x{} = {}/{}\n\n", k+1, det_ak, det_a));
        }
    }
    out
}

fn properties_str() -> String {
    let mut out = String::new();
    let a = Mat::new(2, 2, vec![1,2,3,4]);
    let b = Mat::new(2, 2, vec![5,6,7,8]);
    let det_a = a.det();
    let det_b = b.det();
    let ab = a.mul(&b);
    let det_ab = ab.det();
    let at = a.transpose();
    let det_at = at.det();

    out.push_str("A = [[1,2],[3,4]]  B = [[5,6],[7,8]]\n\n");
    out.push_str(&format!("det(A) = {}\n", det_a));
    out.push_str(&format!("det(B) = {}\n", det_b));
    out.push_str(&format!("det(AB) = {}\n", det_ab));
    out.push_str(&format!("det(A)*det(B) = {}\n", det_a*det_b));
    if det_ab == det_a*det_b { out.push_str("det(AB) = det(A)*det(B) ✓\n"); }
    out.push_str(&format!("\ndet(Aᵀ) = {}\n", det_at));
    if det_at == det_a { out.push_str("det(Aᵀ) = det(A) ✓\n"); }

    let a2 = Mat::new(2, 2, vec![3,4,1,2]);
    out.push_str(&format!("\nRow-swapped A' det = {} = -det(A) ✓\n", a2.det()));
    out
}

fn show_help() -> String {
    let mut out = String::new();
    out.push_str("Commands:\n");
    out.push_str("  det <rows> <cols> <entries>    compute det with steps\n");
    out.push_str("  area <v1x> <v1y> <v2x> <v2y>  parallelogram area\n");
    out.push_str("  vandermonde <x1> <x2> ...      Vandermonde det\n");
    out.push_str("  cramer <A entries> / <b>        Cramer's rule (2×2 or 3×3)\n");
    out.push_str("  properties                      det(AB)=det(A)det(B), etc.\n");
    out.push_str("  demo                            showcase\n");
    out.push_str("  quit                            exit\n");
    out
}

// ── State helpers ─────────────────────────────────────────────────────────────

fn default_state() -> StateMap {
    let mut s = state_new();
    let m = Mat::new(3, 3, vec![1,2,3, 4,5,6, 7,8,9]);
    state_set_mat(&mut s, "last_mat", &m);
    s
}

// ── run_cmd ───────────────────────────────────────────────────────────────────

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    let mut out = String::new();
    match cmd {
        "help" | "h" => {
            out.push_str(&show_help());
        }

        "demo" => {
            out.push_str("=== Determinants Demo ===\n\n");
            out.push_str("2×2 determinant:\n");
            out.push_str(&det_str(&["2","2","1","2","3","4"]));
            out.push('\n');
            out.push_str("3×3 with cofactor expansion:\n");
            out.push_str(&det_str(&["3","3","2","1","-1","-3","-1","2","-2","1","2"]));
            out.push('\n');
            out.push_str("Properties:\n");
            out.push_str(&properties_str());
            out.push('\n');
            out.push_str("Cramer's rule:\n");
            out.push_str(&cramer_str(&[1,2,3,4], &[5,6]));
        }

        "det" => {
            let s = det_str(args);
            // Try to save the matrix
            if args.len() >= 2 {
                if let (Ok(r), Ok(c)) = (args[0].parse::<usize>(), args[1].parse::<usize>()) {
                    if let Some(m) = parse_mat(&args[2..], r, c) {
                        state_set_mat(state, "last_mat", &m);
                    }
                }
            }
            out.push_str(&s);
        }

        "area" => {
            if args.len() < 4 { return "Usage: area <v1x> <v1y> <v2x> <v2y>\n".to_string(); }
            let nums: Vec<f64> = args.iter().take(4).filter_map(|s| s.parse().ok()).collect();
            if nums.len() < 4 { return "Need 4 numbers.\n".to_string(); }
            out.push_str(&area_str(nums[0], nums[1], nums[2], nums[3]));
        }

        "vandermonde" => {
            if args.is_empty() { return "Usage: vandermonde <x1> <x2> ...\n".to_string(); }
            let xs: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
            if xs.len() < 2 { return "Need at least 2 points.\n".to_string(); }
            if xs.len() > 5 { return "At most 5 points.\n".to_string(); }
            out.push_str(&vandermonde_str(&xs));
        }

        "cramer" => {
            let full = args.join(" ");
            let parts: Vec<&str> = full.splitn(2, '/').collect();
            if parts.len() != 2 { return "Usage: cramer <A entries> / <b entries>\n".to_string(); }
            let a_entries: Vec<i64> = parts[0].split_whitespace().filter_map(|s| s.parse().ok()).collect();
            let b_entries: Vec<i64> = parts[1].split_whitespace().filter_map(|s| s.parse().ok()).collect();
            if b_entries.is_empty() { return "Need b vector.\n".to_string(); }
            out.push_str(&cramer_str(&a_entries, &b_entries));
        }

        "properties" => out.push_str(&properties_str()),

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

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    c.title("Determinant: Cofactor Expansion Tree");

    let m = state_get_mat(state, "last_mat")
        .unwrap_or_else(|| Mat::new(2, 2, vec![1,2,3,4]));

    // Show the matrix
    let n = m.rows.min(3);
    let cell = 42.0;
    let mat_x = 60.0; let mat_y = 60.0;
    let data: Vec<Vec<i64>> = (0..n).map(|r| (0..n).map(|c| m[(r,c)]).collect()).collect();
    c.matrix_display(mat_x, mat_y, &data, cell);

    let det = m.det();
    c.text_bold(mat_x + n as f64 * cell / 2.0, mat_y + n as f64 * cell + 20.0,
        &format!("det = {}", det), 14.0, if det != 0 { colors::GREEN } else { colors::RED }, "middle");

    // Expansion tree (for 3×3)
    if n == 3 {
        let tx = 300.0; let ty = 80.0;
        c.text_bold(tx, ty, "Cofactor expansion along row 1", 12.0, colors::DARK, "middle");

        let ys = [120.0, 200.0, 280.0];
        let signs = ["+", "−", "+"];
        let entries = [m[(0,0)], m[(0,1)], m[(0,2)]];

        for (j, (&y, &entry)) in ys.iter().zip(entries.iter()).enumerate() {
            let sign = signs[j];
            let minor = m.minor(0, j);
            let minor_det = minor.det();
            let contribution = if j % 2 == 0 { 1 } else { -1 } * entry * minor_det;

            c.rrect(tx - 140.0, y - 20.0, 280.0, 40.0, 4.0,
                if contribution == 0 { colors::LIGHT }
                else if contribution > 0 { "#e8f5e9" } else { "#fce4ec" },
                colors::GREY, 0.8);
            c.text(tx, y, &format!("{} {} × det(minor) = {} × {} = {}",
                sign, entry, if j%2==0{1}else{-1}*entry, minor_det, contribution),
                11.0, colors::DARK, "middle");
        }
        c.text_bold(tx, 340.0, &format!("Total = {}", det), 13.0,
            if det != 0 { colors::GREEN } else { colors::RED }, "middle");
    }

    // Geometric interpretation (2×2)
    if n == 2 {
        let ox = 500.0; let oy = 300.0; let sc = 30.0;
        let v1x = m[(0,0)] as f64; let v1y = m[(1,0)] as f64;
        let v2x = m[(0,1)] as f64; let v2y = m[(1,1)] as f64;

        c.arrow(ox, oy, ox + v1x*sc, oy - v1y*sc, colors::BLUE, 2.0);
        c.arrow(ox, oy, ox + v2x*sc, oy - v2y*sc, colors::GREEN, 2.0);

        // Parallelogram
        c.path(&format!("M{:.0},{:.0} L{:.0},{:.0} L{:.0},{:.0} L{:.0},{:.0} Z",
            ox, oy,
            ox+v1x*sc, oy-v1y*sc,
            ox+(v1x+v2x)*sc, oy-(v1y+v2y)*sc,
            ox+v2x*sc, oy-v2y*sc),
            "#2196f320", colors::BLUE, 1.0);
        c.text(ox, oy + 20.0, &format!("Area = |det| = {}", det.abs()), 12.0, colors::DARK, "middle");
    }
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let m = state_get_mat(state, "last_mat")
        .unwrap_or_else(|| Mat::new(2, 2, vec![1,2,3,4]));

    let n = m.rows;
    let det = m.det();

    g.node_default("style", "filled,rounded"); g.node_default("shape", "box");
    g.node("root", &[("label", &format!("det({}x{} matrix) = {}", n, n, det)),
                      ("fillcolor", if det != 0 { "#c8e6c9" } else { "#ffcdd2" })]);

    // Cofactors along row 0
    for j in 0..n.min(4) {
        let entry = m[(0,j)];
        let minor_det = m.minor(0, j).det();
        let sign = if j % 2 == 0 { 1 } else { -1 };
        let contrib = sign * entry * minor_det;
        let id = format!("c{}", j);
        g.node(&id, &[("label", &format!("({}) * {} * {} = {}", sign, entry, minor_det, contrib)),
                       ("fillcolor", "#fff9c4")]);
        g.edge("root", &id, &[("label", &format!("j={}", j))]);
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let m = state_get_mat(state, "last_mat")
        .unwrap_or_else(|| Mat::new(2, 2, vec![1,2,3,4]));
    let n = m.rows;
    let det = m.det();

    // Draw as TikZ matrix node
    t.raw("  \\node at (0,0) {$\\det\\begin{pmatrix}");
    for r in 0..n.min(3) {
        let row: Vec<String> = (0..n.min(3)).map(|c| m[(r,c)].to_string()).collect();
        let ending = if r + 1 < n.min(3) { " \\\\" } else { "" };
        t.raw(&format!("    {}{}", row.join(" & "), ending));
    }
    t.raw(&format!("  \\end{{pmatrix}} = {}$}};", det));
    t.raw(&format!("  \\node[below=8mm] at (0,0) {{{}invertible: $\\det = {}$}};",
        if det != 0 { "" } else { "NOT " }, det));
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let m = state_get_mat(state, "last_mat")
        .unwrap_or_else(|| Mat::new(2, 2, vec![1,2,3,4]));
    let n = m.rows;
    let det = m.det();

    a.text_at(0, 0, "Determinant Expansion");
    a.hline(0, 1, 40, '=');
    a.text_at(0, 3, &format!("Matrix ({}x{}):", n, n));
    for r in 0..n.min(4) {
        let row: Vec<String> = (0..n.min(4)).map(|c| format!("{:4}", m[(r,c)])).collect();
        a.text_at(2, 4 + r as i32, &format!("[{}]", row.join(" ")));
    }

    let det_y = n as i32 + 6;
    a.text_at(0, det_y, &format!("det = {}", det));
    a.text_at(0, det_y + 1,
        if det != 0 { "INVERTIBLE (det != 0)" } else { "SINGULAR (det = 0)" });

    if n <= 3 {
        a.text_at(0, det_y + 3, "Cofactor expansion along row 1:");
        for j in 0..n.min(3) {
            let entry = m[(0,j)];
            let sign = if j % 2 == 0 { 1i64 } else { -1 };
            let minor_det = m.minor(0,j).det();
            a.text_at(2, det_y + 4 + j as i32,
                &format!("j={}: ({}) * {} * {} = {}", j, sign, entry, minor_det, sign*entry*minor_det));
        }
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
                    let mut c = SvgCanvas::new(700.0, 420.0);
                    visualize_svg(&mut c, cmd, &args_ref, &state);
                    c.build()
                }
                OutputFormat::Dot => {
                    let mut g = DotGraph::digraph("ch08");
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
            print_banner("Chapter 8", "Determinants",
                "Cofactor expansion, geometry, Cramer's rule");
            print_info("Type 'help' for commands.");
            print_note("Try: det 3 3 1 2 3 4 5 6 7 8 9");
            repl("ch8> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
