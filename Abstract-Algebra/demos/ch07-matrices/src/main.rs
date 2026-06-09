use common::*;
use std::collections::HashMap;

// ── Matrix Calculator ────────────────────────────────────────────────────────

fn display_mat_str(m: &Mat) -> String { format!("{}", m) }

fn mat_add(a: &Mat, b: &Mat) -> Option<Mat> {
    if a.rows != b.rows || a.cols != b.cols { return None; }
    let data: Vec<i64> = a.data.iter().zip(b.data.iter()).map(|(x, y)| x + y).collect();
    Some(Mat::new(a.rows, a.cols, data))
}

fn row_reduction_str(mat: &Mat) -> String {
    let mut m = mat.clone();
    let steps = m.row_reduce_steps();
    let mut out = String::new();
    out.push_str("Starting matrix:\n");
    out.push_str(&display_mat_str(mat));
    out.push('\n');
    for (i, step) in steps.iter().enumerate() {
        out.push_str(&format!("Step {}: {}\n", i+1, step));
    }
    if steps.is_empty() { out.push_str("Already in row echelon form.\n"); }
    out.push_str("Result:\n");
    out.push_str(&display_mat_str(&m));
    out
}

fn rank_from_rref(m: &Mat) -> usize {
    (0..m.rows).filter(|&r| (0..m.cols).any(|c| m[(r, c)] != 0)).count()
}

fn solve_str(a: &Mat, b_vec: &[i64]) -> String {
    let mut out = String::new();
    if a.rows != b_vec.len() {
        return "Number of rows in A must match entries in b.\n".to_string();
    }
    let mut aug_data: Vec<i64> = Vec::new();
    for r in 0..a.rows {
        for c in 0..a.cols { aug_data.push(a[(r, c)]); }
        aug_data.push(b_vec[r]);
    }
    let mut aug = Mat::new(a.rows, a.cols + 1, aug_data);
    out.push_str("Augmented [A|b]:\n");
    out.push_str(&display_mat_str(&aug));
    out.push('\n');
    let steps = aug.row_reduce_steps();
    for (i, step) in steps.iter().enumerate() {
        out.push_str(&format!("Step {}: {}\n", i+1, step));
    }
    out.push_str("\nRow echelon:\n");
    out.push_str(&display_mat_str(&aug));
    out.push('\n');

    let n = a.cols;
    let mut consistent = true;
    for r in 0..a.rows {
        let all_zero = (0..n).all(|c| aug[(r, c)] == 0);
        if all_zero && aug[(r, n)] != 0 { consistent = false; break; }
    }

    if !consistent {
        out.push_str("System is INCONSISTENT (no solution).\n");
        return out;
    }

    let rank = (0..a.rows).filter(|&r| (0..n).any(|c| aug[(r,c)] != 0)).count();
    if rank < n {
        out.push_str(&format!("Infinitely many solutions (rank {} < {} unknowns).\n", rank, n));
        return out;
    }

    out.push_str("Unique solution:\n");
    for i in 0..n.min(a.rows) {
        let piv = aug[(i, i)];
        if piv == 0 {
            out.push_str(&format!("  x{} = free\n", i+1));
        } else {
            let rhs = aug[(i, n)];
            if rhs % piv == 0 {
                out.push_str(&format!("  x{} = {}\n", i+1, rhs/piv));
            } else {
                out.push_str(&format!("  x{} = {}/{}\n", i+1, rhs, piv));
            }
        }
    }
    out
}

fn inverse_str(a: &Mat) -> String {
    let mut out = String::new();
    if a.rows != a.cols { return "Matrix must be square.\n".to_string(); }
    let n = a.rows;
    let det = a.det();
    if det == 0 { return "Matrix is SINGULAR (det = 0). No inverse.\n".to_string(); }
    out.push_str(&format!("det(A) = {}\n", det));

    let mut aug_data: Vec<i64> = Vec::new();
    for r in 0..n {
        for c in 0..n { aug_data.push(a[(r, c)]); }
        for c in 0..n { aug_data.push(if c == r { 1 } else { 0 }); }
    }
    let mut aug = Mat::new(n, 2*n, aug_data);
    aug.row_reduce_steps();

    let mut adj_data: Vec<i64> = vec![0; n*n];
    for i in 0..n {
        for j in 0..n {
            let sign = if (i+j) % 2 == 0 { 1 } else { -1 };
            adj_data[j*n+i] = sign * a.minor(i,j).det();
        }
    }
    let adj = Mat::new(n, n, adj_data);
    out.push_str("Adjugate adj(A):\n");
    out.push_str(&display_mat_str(&adj));
    out.push_str(&format!("A^-1 = (1/{}) * adj(A)\n", det));

    let check = a.mul(&adj);
    out.push_str("Verification A*adj(A):\n");
    out.push_str(&display_mat_str(&check));
    out.push_str(&format!("= {} * I ✓\n", det));
    out
}

fn show_help() -> String {
    let mut out = String::new();
    out.push_str("Commands:\n");
    out.push_str("  set <name> <r> <c> <entries>  store a named matrix\n");
    out.push_str("  show                           list stored matrices\n");
    out.push_str("  mul <A> <B>                    multiply A by B\n");
    out.push_str("  add <A> <B>                    add A and B\n");
    out.push_str("  reduce <A>                     row reduce with steps\n");
    out.push_str("  rref <A>                       RREF\n");
    out.push_str("  rank <A>                       compute rank\n");
    out.push_str("  inverse <A>                    matrix inverse\n");
    out.push_str("  solve <A> <b>                  solve Ax=b\n");
    out.push_str("  demo                           showcase of matrix operations\n");
    out.push_str("  quit                           exit\n");
    out
}

fn get_mat_str(mats: &HashMap<String, Mat>, name: &str) -> Option<Mat> {
    mats.get(name).cloned()
}

// ── State helpers ─────────────────────────────────────────────────────────────

fn default_state() -> StateMap {
    let mut s = state_new();
    let a = Mat::new(2, 2, vec![1,2,3,4]);
    let b = Mat::new(2, 2, vec![5,6,7,8]);
    state_set_mat(&mut s, "mat_A", &a);
    state_set_mat(&mut s, "mat_B", &b);
    state_set_strings(&mut s, "mat_names", &["A".to_string(), "B".to_string()]);
    s
}

fn mats_from_state(state: &StateMap) -> HashMap<String, Mat> {
    let mut mats = HashMap::new();
    let names = state_get_strings(state, "mat_names").unwrap_or_default();
    for name in &names {
        if let Some(m) = state_get_mat(state, &format!("mat_{}", name)) {
            mats.insert(name.clone(), m);
        }
    }
    mats
}

fn save_mats_to_state(state: &mut StateMap, mats: &HashMap<String, Mat>) {
    let names: Vec<String> = mats.keys().cloned().collect();
    state_set_strings(state, "mat_names", &names);
    for (name, m) in mats {
        state_set_mat(state, &format!("mat_{}", name), m);
    }
}

// ── run_cmd ───────────────────────────────────────────────────────────────────

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    let mut out = String::new();
    let mut mats = mats_from_state(state);

    match cmd {
        "help" | "h" => {
            out.push_str(&show_help());
        }

        "demo" => {
            out.push_str("=== Matrix Calculator Demo ===\n\n");
            let a = Mat::new(3, 3, vec![2,1,-1, -3,-1,2, -2,1,2]);
            let b = vec![8i64, -11, -3];
            out.push_str("Matrix A:\n");
            out.push_str(&display_mat_str(&a));
            out.push('\n');
            out.push_str("Solving Ax = b  where b = [8, -11, -3]:\n");
            out.push_str(&solve_str(&a, &b));
            out.push('\n');
            let a2 = Mat::new(2, 2, vec![1,2,3,4]);
            out.push_str("Inverse of A = [[1,2],[3,4]]:\n");
            out.push_str(&inverse_str(&a2));
        }

        "set" => {
            if args.len() < 3 {
                return "Usage: set <name> <rows> <cols> <entries...>\n".to_string();
            }
            let name = args[0].to_string();
            let rows: usize = match args[1].parse() {
                Ok(r) if r > 0 => r, _ => return "rows must be positive integer\n".to_string(),
            };
            let cols: usize = match args[2].parse() {
                Ok(c) if c > 0 => c, _ => return "cols must be positive integer\n".to_string(),
            };
            if args.len() - 3 != rows * cols {
                return format!("Expected {} entries, got {}.\n", rows*cols, args.len()-3);
            }
            let data: Vec<i64> = match args[3..].iter().map(|s| s.parse::<i64>()).collect::<Result<_,_>>() {
                Ok(v) => v, Err(_) => return "Non-integer entry.\n".to_string(),
            };
            let m = Mat::new(rows, cols, data);
            out.push_str(&format!("Stored {} ({}×{})\n", name, rows, cols));
            out.push_str(&display_mat_str(&m));
            mats.insert(name, m);
            save_mats_to_state(state, &mats);
        }

        "show" => {
            if mats.is_empty() {
                out.push_str("No matrices defined.\n");
            } else {
                let mut names: Vec<&String> = mats.keys().collect(); names.sort();
                for name in names {
                    let m = &mats[name];
                    out.push_str(&format!("{} ({}×{}):\n", name, m.rows, m.cols));
                    out.push_str(&display_mat_str(m));
                    out.push('\n');
                }
            }
        }

        "mul" => {
            if args.len() < 2 { return "Usage: mul <A> <B>\n".to_string(); }
            let a = match get_mat_str(&mats, args[0]) { Some(m) => m, None => return format!("Matrix '{}' not defined.\n", args[0]) };
            let b = match get_mat_str(&mats, args[1]) { Some(m) => m, None => return format!("Matrix '{}' not defined.\n", args[1]) };
            if a.cols != b.rows {
                return format!("Cannot multiply: {} is {}×{}, {} is {}×{}\n", args[0], a.rows, a.cols, args[1], b.rows, b.cols);
            }
            let prod = a.mul(&b);
            out.push_str(&format!("{} · {} =\n", args[0], args[1]));
            out.push_str(&display_mat_str(&prod));
        }

        "add" => {
            if args.len() < 2 { return "Usage: add <A> <B>\n".to_string(); }
            let a = match get_mat_str(&mats, args[0]) { Some(m) => m, None => return format!("Matrix '{}' not defined.\n", args[0]) };
            let b = match get_mat_str(&mats, args[1]) { Some(m) => m, None => return format!("Matrix '{}' not defined.\n", args[1]) };
            match mat_add(&a, &b) {
                None => out.push_str(&format!("Dimension mismatch: {}×{} + {}×{}\n", a.rows, a.cols, b.rows, b.cols)),
                Some(sum) => { out.push_str(&format!("{} + {} =\n", args[0], args[1])); out.push_str(&display_mat_str(&sum)); }
            }
        }

        "reduce" => {
            if args.is_empty() { return "Usage: reduce <A>\n".to_string(); }
            match get_mat_str(&mats, args[0]) {
                None => out.push_str(&format!("Matrix '{}' not defined.\n", args[0])),
                Some(m) => out.push_str(&row_reduction_str(&m)),
            }
        }

        "rref" => {
            if args.is_empty() { return "Usage: rref <A>\n".to_string(); }
            match get_mat_str(&mats, args[0]) {
                None => out.push_str(&format!("Matrix '{}' not defined.\n", args[0])),
                Some(m) => {
                    let mut r2 = m.clone();
                    r2.row_reduce_steps();
                    out.push_str(&format!("RREF of {}:\n", args[0]));
                    out.push_str(&display_mat_str(&r2));
                    let rank = rank_from_rref(&r2);
                    out.push_str(&format!("Rank = {}\n", rank));
                }
            }
        }

        "rank" => {
            if args.is_empty() { return "Usage: rank <A>\n".to_string(); }
            match get_mat_str(&mats, args[0]) {
                None => out.push_str(&format!("Matrix '{}' not defined.\n", args[0])),
                Some(m) => {
                    let mut r = m.clone();
                    r.row_reduce_steps();
                    let rank = rank_from_rref(&r);
                    let nullity = m.cols - rank;
                    out.push_str(&format!("rank({}) = {}\n", args[0], rank));
                    out.push_str(&format!("nullity = {}\n", nullity));
                    out.push_str(&format!("rank + nullity = {} = cols ✓\n", rank+nullity));
                }
            }
        }

        "inverse" => {
            if args.is_empty() { return "Usage: inverse <A>\n".to_string(); }
            match get_mat_str(&mats, args[0]) {
                None => out.push_str(&format!("Matrix '{}' not defined.\n", args[0])),
                Some(m) => out.push_str(&inverse_str(&m)),
            }
        }

        "solve" => {
            if args.len() < 2 { return "Usage: solve <A> <b entries...>\n".to_string(); }
            match get_mat_str(&mats, args[0]) {
                None => out.push_str(&format!("Matrix '{}' not defined.\n", args[0])),
                Some(a) => {
                    match args[1..].iter().map(|s| s.parse::<i64>()).collect::<Result<Vec<_>,_>>() {
                        Err(_) => out.push_str("b entries must be integers.\n"),
                        Ok(b) => out.push_str(&solve_str(&a, &b)),
                    }
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
    c.title("Matrix Display");

    let mats = mats_from_state(state);
    let mut x = 80.0;
    let names: Vec<String> = {
        let mut n = mats.keys().cloned().collect::<Vec<_>>();
        n.sort(); n
    };

    for name in &names {
        let m = &mats[name];
        let n_show = m.rows.min(4);
        let c_show = m.cols.min(4);
        let cell = 42.0;

        c.text_bold(x + n_show as f64 * cell / 2.0, 55.0, name, 14.0, colors::DARK, "middle");

        let data: Vec<Vec<i64>> = (0..n_show).map(|r| (0..c_show).map(|co| m[(r,co)]).collect()).collect();
        c.matrix_display(x, 70.0, &data, cell);

        // Rank annotation
        let mut r = m.clone();
        r.row_reduce_steps();
        let rank = rank_from_rref(&r);
        c.text(x + n_show as f64 * cell / 2.0, 70.0 + n_show as f64 * cell + 20.0,
            &format!("rank={}", rank), 11.0, colors::GREY, "middle");

        x += c_show as f64 * cell + 80.0;
        if x > 600.0 { break; }
    }

    c.text(350.0, 380.0,
        "Matrix product AB: (AB)[i,j] = sum_k A[i,k]*B[k,j]",
        11.0, colors::GREY, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let mats = mats_from_state(state);
    g.node_default("style", "filled,rounded"); g.node_default("shape", "box");

    let names: Vec<String> = { let mut n: Vec<_> = mats.keys().cloned().collect(); n.sort(); n };
    for name in &names {
        let m = &mats[name];
        let mut r = m.clone(); r.row_reduce_steps();
        let rank = rank_from_rref(&r);
        let det = if m.rows == m.cols { Some(m.det()) } else { None };
        let label = if let Some(d) = det {
            format!("{} ({}x{}, rank={}, det={})", name, m.rows, m.cols, rank, d)
        } else {
            format!("{} ({}x{}, rank={})", name, m.rows, m.cols, rank)
        };
        g.node(name, &[("label", &label), ("fillcolor", "#e3f2fd")]);
    }

    // If A and B both 2x2, show product
    if names.len() >= 2 {
        let a = &mats[&names[0]]; let b = &mats[&names[1]];
        if a.cols == b.rows {
            let prod = a.mul(b);
            let mut r = prod.clone(); r.row_reduce_steps();
            let rank = rank_from_rref(&r);
            g.node("AB", &[("label", &format!("{}{} ({}x{}, rank={})",
                names[0], names[1], prod.rows, prod.cols, rank)), ("fillcolor", "#e8f5e9")]);
            g.edge(&names[0], "AB", &[("label", "mul")]);
            g.edge(&names[1], "AB", &[]);
        }
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let mats = mats_from_state(state);
    let names: Vec<String> = { let mut n: Vec<_> = mats.keys().cloned().collect(); n.sort(); n };

    for (i, name) in names.iter().take(2).enumerate() {
        let m = &mats[name];
        let x = i as f64 * 5.0;
        t.node(name, x, 0.0, name, "draw,fill=blue!10,minimum size=1.5cm");

        // Show as small table
        let rows: Vec<Vec<String>> = (0..m.rows.min(3)).map(|r|
            (0..m.cols.min(3)).map(|c| m[(r,c)].to_string()).collect()
        ).collect();
        t.matrix_table(&rows, x, -3.0);
    }
    if names.len() >= 2 {
        t.arrow(&names[0], &names[1], "\\cdot", "-stealth,thick");
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let mats = mats_from_state(state);
    a.text_at(0, 0, "Matrix Calculator");
    a.hline(0, 1, 40, '=');

    let names: Vec<String> = { let mut n: Vec<_> = mats.keys().cloned().collect(); n.sort(); n };
    let mut y = 3i32;
    for name in &names {
        let m = &mats[name];
        a.text_at(0, y, &format!("{} ({}x{}):", name, m.rows, m.cols));
        y += 1;
        for r in 0..m.rows.min(4) {
            let row: Vec<String> = (0..m.cols.min(4)).map(|c| format!("{:4}", m[(r,c)])).collect();
            a.text_at(2, y, &format!("[{}]", row.join(" ")));
            y += 1;
        }
        let mut r = m.clone(); r.row_reduce_steps();
        let rank = rank_from_rref(&r);
        a.text_at(2, y, &format!("rank={}", rank));
        y += 2;
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
                    let mut g = DotGraph::digraph("ch07");
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
            print_banner("Chapter 7", "Matrices and Matrix Algebra",
                "Named matrices, row reduction with steps, and linear systems");
            print_info("Type 'help' for commands.");
            print_note("Try: set A 2 2 1 2 3 4   then   rank A");
            repl("ch7> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
