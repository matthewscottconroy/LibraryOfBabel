use common::*;

// ── Bases and Coordinates Explorer ──────────────────────────────────────────

fn parse_vecs(args: &[&str]) -> Vec<Vec<f64>> {
    let joined = args.join(" ");
    joined.split(';')
        .filter_map(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.is_empty() { return None; }
            let v: Vec<f64> = parts.iter().filter_map(|p| p.parse().ok()).collect();
            if v.len() == parts.len() { Some(v) } else { None }
        })
        .collect()
}

fn vec_to_str(v: &[f64]) -> String {
    let parts: Vec<String> = v.iter().map(|x| format!("{:.4}", x)).collect();
    format!("({})", parts.join(", "))
}

fn solve_linear(a_rows: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let m = a_rows.len();
    if m == 0 { return None; }
    let n = a_rows[0].len();
    if b.len() != m { return None; }

    let mut aug: Vec<Vec<f64>> = a_rows.iter().zip(b.iter())
        .map(|(row, &bi)| { let mut r = row.clone(); r.push(bi); r })
        .collect();

    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();
    for col in 0..n {
        let mut found = None;
        for r in pivot_row..m {
            if aug[r][col].abs() > 1e-9 { found = Some(r); break; }
        }
        let pr = match found { None => continue, Some(r) => r };
        aug.swap(pivot_row, pr);
        let piv = aug[pivot_row][col];
        for c in 0..=n { aug[pivot_row][c] /= piv; }
        for r in 0..m {
            if r == pivot_row { continue; }
            let factor = aug[r][col];
            for c in 0..=n {
                let sub = factor * aug[pivot_row][c];
                aug[r][c] -= sub;
            }
        }
        pivot_cols.push(col);
        pivot_row += 1;
    }

    for r in 0..m {
        let all_zero = (0..n).all(|c| aug[r][c].abs() < 1e-9);
        if all_zero && aug[r][n].abs() > 1e-9 { return None; }
    }

    if pivot_cols.len() != n { return None; }

    let mut x = vec![0.0f64; n];
    for (i, &col) in pivot_cols.iter().enumerate() {
        x[col] = aug[i][n];
    }
    Some(x)
}

fn rref(mat: &[Vec<f64>]) -> (Vec<Vec<f64>>, usize) {
    let mut m = mat.to_vec();
    let rows = m.len();
    if rows == 0 { return (m, 0); }
    let cols = m[0].len();
    let mut pivot_row = 0;
    let mut rank = 0;
    for col in 0..cols {
        let mut found = None;
        for r in pivot_row..rows {
            if m[r][col].abs() > 1e-9 { found = Some(r); break; }
        }
        let pr = match found { None => continue, Some(r) => r };
        m.swap(pivot_row, pr);
        let piv = m[pivot_row][col];
        for c in 0..cols { m[pivot_row][c] /= piv; }
        for r in 0..rows {
            if r == pivot_row { continue; }
            let factor = m[r][col];
            for c in 0..cols {
                let sub = factor * m[pivot_row][c];
                m[r][c] -= sub;
            }
        }
        pivot_row += 1;
        rank += 1;
    }
    (m, rank)
}

fn standard_basis(n: usize) -> Vec<Vec<f64>> {
    (0..n).map(|i| (0..n).map(|j| if i == j { 1.0 } else { 0.0 }).collect()).collect()
}

fn show_help() -> String {
    let mut out = String::new();
    out.push_str("Commands:\n");
    out.push_str("  basis <vecs>            test if vectors form a basis\n");
    out.push_str("  coords <v> / <basis>    express v in basis coordinates\n");
    out.push_str("  dim <vecs>              dimension of span\n");
    out.push_str("  extend <vecs>           extend to a basis of R^n\n");
    out.push_str("  change <B1> / <B2>      change-of-basis matrix\n");
    out.push_str("  standard <n>            standard basis for R^n\n");
    out.push_str("  demo                    showcase of basis/dimension concepts\n");
    out.push_str("  quit                    exit\n");
    out
}

// ── State helpers ─────────────────────────────────────────────────────────────

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "ambient_dim", 3);
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
            out.push_str("=== Bases and Dimension Demo ===\n\n");
            out.push_str("Standard basis for R^3:\n");
            for (i, v) in standard_basis(3).iter().enumerate() {
                out.push_str(&format!("  e{} = {}\n", i+1, vec_to_str(v)));
            }
            out.push('\n');

            out.push_str("Oblique basis B = {(1,1), (1,-1)}:\n");
            let b1 = vec![1.0, 1.0]; let b2 = vec![1.0, -1.0];
            let (_, r) = rref(&[b1.clone(), b2.clone()]);
            out.push_str(&format!("  Rank = {} (forms a basis of R^2)\n\n", r));

            let v = vec![3.0, 4.0_f64];
            let b2_mat: Vec<Vec<f64>> = (0..2).map(|i| vec![b1[i], b2[i]]).collect();
            if let Some(coords) = solve_linear(&b2_mat, &v) {
                out.push_str(&format!("  v = {} = {:.3}*(1,1) + {:.3}*(1,-1)\n\n", vec_to_str(&v), coords[0], coords[1]));
            }

            out.push_str("Change of basis example:\n");
            out.push_str("  B1 = standard, B2 = {(1,1),(1,-1)}\n");
            out.push_str("  P converts B1 coordinates to B2 coordinates.\n");
        }

        "standard" => {
            let n = parse_uint(args, 0, "n").unwrap_or(3) as usize;
            if n > 8 { return "n must be at most 8.\n".to_string(); }
            state_set_int(state, "ambient_dim", n as i64);
            out.push_str(&format!("Standard Basis for R^{}:\n", n));
            for (i, v) in standard_basis(n).iter().enumerate() {
                out.push_str(&format!("  e{} = {}\n", i+1, vec_to_str(v)));
            }
        }

        "dim" => {
            let vecs = parse_vecs(args);
            if vecs.is_empty() { return "Usage: dim <v1> ; <v2> ; ...\n".to_string(); }
            let n = vecs[0].len();
            let (_, r) = rref(&vecs);
            state_set_int(state, "ambient_dim", n as i64);
            out.push_str(&format!("dim(span) = {}\n", r));
            out.push_str(&format!("Vectors given: {}, ambient: R^{}\n", vecs.len(), n));
        }

        "basis" => {
            let vecs = parse_vecs(args);
            if vecs.is_empty() { return "Usage: basis <v1> ; <v2> ; ...\n".to_string(); }
            let n_vecs = vecs.len();
            let n_dim = vecs[0].len();
            let (_, r) = rref(&vecs);
            state_set_int(state, "ambient_dim", n_dim as i64);
            let indep = r == n_vecs;
            let spans = r == n_dim;
            if indep && spans {
                out.push_str(&format!("YES: basis for R^{} ({} independent vectors spanning R^{})\n", n_dim, n_vecs, n_dim));
            } else if indep {
                out.push_str(&format!("NOT a basis: independent but only spans {}-dim subspace of R^{}\n", r, n_dim));
            } else {
                out.push_str(&format!("NOT a basis: {} vectors have rank {} (linearly dependent)\n", n_vecs, r));
            }
        }

        "coords" => {
            let full = args.join(" ");
            let parts: Vec<&str> = full.splitn(2, '/').collect();
            if parts.len() != 2 {
                return "Usage: coords <v> / <basis_vec1> ; <basis_vec2> ; ...\n".to_string();
            }
            let v_str = parts[0].trim();
            let b_str = parts[1].trim();

            let v: Vec<f64> = match v_str.split_whitespace()
                .map(|s| s.parse::<f64>())
                .collect::<Result<Vec<_>,_>>() {
                Ok(v) => v,
                Err(_) => return "Could not parse vector v.\n".to_string(),
            };

            let basis = parse_vecs(&b_str.split_whitespace().collect::<Vec<_>>());
            if basis.is_empty() { return "Could not parse basis vectors.\n".to_string(); }

            let n = v.len();
            let m = n;
            let k = basis.len();
            let a_rows: Vec<Vec<f64>> = (0..m).map(|i| {
                (0..k).map(|j| basis[j].get(i).copied().unwrap_or(0.0)).collect()
            }).collect();

            match solve_linear(&a_rows, &v) {
                None => out.push_str("Cannot express v in this basis.\n"),
                Some(coords) => {
                    out.push_str(&format!("v = {} in standard basis\n", vec_to_str(&v)));
                    for (i, &c) in coords.iter().enumerate() {
                        out.push_str(&format!("  [v]_B coord {}: {:.4}\n", i+1, c));
                    }
                    let reconstructed: Vec<f64> = (0..n).map(|i| {
                        coords.iter().zip(basis.iter()).map(|(&c, b)| c * b[i]).sum()
                    }).collect();
                    out.push_str(&format!("Verification: {:.4?}·B = {}\n", coords, vec_to_str(&reconstructed)));
                }
            }
        }

        "extend" => {
            let vecs = parse_vecs(args);
            if vecs.is_empty() { return "Usage: extend <v1> ; <v2> ; ...\n".to_string(); }
            let n = vecs[0].len();
            let (_, r) = rref(&vecs);
            if r == n {
                out.push_str("Already spans R^n — no extension needed.\n");
                return out;
            }
            let mut extended = vecs.clone();
            for i in 0..n {
                let mut e = vec![0.0f64; n];
                e[i] = 1.0;
                let mut test = extended.clone();
                test.push(e.clone());
                let cur_rank = rref(&extended).1;
                if rref(&test).1 > cur_rank { extended.push(e); }
                if rref(&extended).1 == n { break; }
            }
            out.push_str("Original vectors:\n");
            for (i, v) in vecs.iter().enumerate() {
                out.push_str(&format!("  v{}: {}\n", i+1, vec_to_str(v)));
            }
            out.push_str("Added standard basis vectors:\n");
            for (i, v) in extended[vecs.len()..].iter().enumerate() {
                out.push_str(&format!("  e{}: {}\n", i+1, vec_to_str(v)));
            }
            out.push_str(&format!("Extended to basis of R^{} ({} vectors)\n", n, extended.len()));
        }

        "change" => {
            let full = args.join(" ");
            let parts: Vec<&str> = full.splitn(2, '/').collect();
            if parts.len() != 2 {
                return "Usage: change <B1_vecs> / <B2_vecs>\n".to_string();
            }
            let b1_vecs = parse_vecs(&parts[0].trim().split_whitespace().collect::<Vec<_>>());
            let b2_vecs = parse_vecs(&parts[1].trim().split_whitespace().collect::<Vec<_>>());

            if b1_vecs.is_empty() || b2_vecs.is_empty() {
                return "Could not parse basis vectors.\n".to_string();
            }
            if b1_vecs.len() != b2_vecs.len() {
                return "Both bases must have the same number of vectors.\n".to_string();
            }

            let n = b1_vecs[0].len();
            let k = b1_vecs.len();
            let b2_matrix_rows: Vec<Vec<f64>> = (0..n).map(|i| {
                (0..k).map(|j| b2_vecs[j].get(i).copied().unwrap_or(0.0)).collect()
            }).collect();

            let mut all_ok = true;
            let mut p_cols: Vec<Vec<f64>> = Vec::new();
            for b1v in &b1_vecs {
                match solve_linear(&b2_matrix_rows, b1v) {
                    None => { out.push_str("Could not express a B1 vector in B2 coordinates.\n"); all_ok = false; break; }
                    Some(col) => p_cols.push(col),
                }
            }

            if all_ok {
                out.push_str("Change-of-basis matrix P (B1 coords → B2 coords):\n");
                for i in 0..k {
                    let row: Vec<String> = (0..k).map(|j| format!("{:8.4}", p_cols[j][i])).collect();
                    out.push_str(&format!("  │ {} │\n", row.join("  ")));
                }
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

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "ambient_dim").unwrap_or(3) as usize;
    let n_show = n.min(4);

    c.title(&format!("Standard Basis for R^{}", n));

    let basis = standard_basis(n_show);
    let cell_w = 50.0; let cell_h = 35.0;
    let x0 = (700.0 - n_show as f64 * cell_w) / 2.0;
    let y0 = 70.0;

    // Draw the identity matrix
    let data: Vec<Vec<i64>> = basis.iter().map(|row| row.iter().map(|&x| x as i64).collect()).collect();
    c.matrix_display(x0, y0, &data, cell_w);

    // Labels below
    for (i, _v) in basis.iter().enumerate() {
        let cx = x0 + i as f64 * cell_w + cell_w / 2.0;
        c.text_bold(cx, y0 + n_show as f64 * cell_h + 20.0,
            &format!("e{}", i+1), 12.0, colors::BLUE, "middle");
    }

    // Dimension annotation
    c.text(350.0, y0 + n_show as f64 * cell_h + 50.0,
        &format!("dim(R^{}) = {}", n, n), 14.0, colors::DARK, "middle");

    // Draw span relationship
    let sy = y0 + n_show as f64 * cell_h + 90.0;
    c.rrect(150.0, sy, 400.0, 40.0, 5.0, "#e3f2fd", colors::BLUE, 1.5);
    c.text(350.0, sy + 20.0,
        &format!("span{{e1,...,e{}}} = R^{}", n, n), 13.0, colors::DARK, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "ambient_dim").unwrap_or(3) as usize;

    g.node_default("style", "filled,rounded");
    g.node_default("shape", "box");

    g.node("V", &[("label", &format!("R^{} (dim={})", n, n)), ("fillcolor", "#b2ebf2")]);
    for i in 0..n.min(4) {
        let id = format!("e{}", i+1);
        let mut unit = vec![0i64; n]; unit[i] = 1;
        let label = format!("e{} = {:?}", i+1, unit);
        g.node(&id, &[("label", &label), ("fillcolor", "#e8f5e9")]);
        g.edge(&id, "V", &[("label", "basis vector")]);
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "ambient_dim").unwrap_or(3) as usize;
    t.use_library("arrows.meta");

    let labels = ["e_1","e_2","e_3","e_4"];
    let xs = [0.0, 2.5, 5.0, 7.5];

    t.node("O", -1.0, 0.0, "$O$", "draw,circle,fill=black,minimum size=0.2cm");
    for i in 0..n.min(4) {
        t.node_math(&format!("e{}", i+1), xs[i], 0.0, labels[i], "draw,circle,fill=blue!20,minimum size=1.2cm");
        t.arrow("O", &format!("e{}", i+1), "", "-stealth,blue");
    }
    t.node("V", xs[n.min(4)-1] + 2.5, 0.0, &format!("$R^{}$", n), "draw,fill=cyan!20,minimum size=1.5cm");
    t.raw(&format!("  \\node[above=6mm] at ({:.1},0) {{$\\text{{dim}} = {}$}};", xs[n.min(4).saturating_sub(1)/2], n));
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "ambient_dim").unwrap_or(3) as usize;
    a.text_at(0, 0, &format!("Basis/Dimension Explorer — R^{}", n));
    a.hline(0, 1, 40, '=');

    a.text_at(0, 3, &format!("Standard basis for R^{}:", n));
    for i in 0..n.min(6) {
        let mut v = vec!["0"; n];
        v[i] = "1";
        a.text_at(2, 4 + i as i32, &format!("e{} = ({})", i+1, v.join(",")));
    }

    a.text_at(0, n as i32 + 6, &format!("dim(R^{}) = {}", n, n));
    a.text_at(0, n as i32 + 7, "Any basis has exactly n vectors.");
    a.text_at(0, n as i32 + 9, "Basis = independent + spanning");
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
                    let mut g = DotGraph::digraph("ch05");
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
            print_banner("Chapter 5", "Bases, Dimension, and Coordinates",
                "Explore linear bases, dimension, and coordinate transformations");
            print_info("Type 'help' for commands.");
            print_note("Try: basis 1 0 0 ; 0 1 0 ; 0 0 1");
            repl("ch5> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
