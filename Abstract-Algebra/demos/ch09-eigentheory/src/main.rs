use common::*;

// ── Eigentheory Explorer ─────────────────────────────────────────────────────

fn entry(data: &[f64], n: usize, r: usize, c: usize) -> f64 {
    data[r * n + c]
}

fn display_fmat_str(data: &[f64], n: usize) -> String {
    let mut s = String::new();
    for r in 0..n {
        let row: Vec<String> = (0..n).map(|c| format!("{:8.4}", entry(data, n, r, c))).collect();
        s.push_str(&format!("  │ {} │\n", row.join("  ")));
    }
    s
}

// ── Characteristic polynomial ─────────────────────────────────────────────────

fn char_poly_2x2(d: &[f64]) -> [f64; 3] {
    let tr = d[0] + d[3];
    let det = d[0]*d[3] - d[1]*d[2];
    [det, -tr, 1.0]
}

fn char_poly_3x3(d: &[f64]) -> [f64; 4] {
    let a = |r: usize, c: usize| entry(d, 3, r, c);
    let tr = a(0,0) + a(1,1) + a(2,2);
    let det3 = a(0,0)*(a(1,1)*a(2,2)-a(1,2)*a(2,1))
              -a(0,1)*(a(1,0)*a(2,2)-a(1,2)*a(2,0))
              +a(0,2)*(a(1,0)*a(2,1)-a(1,1)*a(2,0));
    let m01 = a(0,0)*a(1,1) - a(0,1)*a(1,0);
    let m02 = a(0,0)*a(2,2) - a(0,2)*a(2,0);
    let m12 = a(1,1)*a(2,2) - a(1,2)*a(2,1);
    let sum_minors = m01 + m02 + m12;
    [-det3, sum_minors, -tr, 1.0]
}

fn poly_display_2(c: &[f64; 3]) -> String {
    let mut s = "λ²".to_string();
    if c[1] < -1e-9 { s.push_str(&format!(" − {:.4}λ", -c[1])); }
    else if c[1] > 1e-9 { s.push_str(&format!(" + {:.4}λ", c[1])); }
    if c[0] < -1e-9 { s.push_str(&format!(" − {:.4}", -c[0])); }
    else if c[0] > 1e-9 { s.push_str(&format!(" + {:.4}", c[0])); }
    s
}

fn poly_display_3(c: &[f64; 4]) -> String {
    let mut s = "λ³".to_string();
    if c[2] < -1e-9 { s.push_str(&format!(" − {:.4}λ²", -c[2])); }
    else if c[2] > 1e-9 { s.push_str(&format!(" + {:.4}λ²", c[2])); }
    if c[1] < -1e-9 { s.push_str(&format!(" − {:.4}λ", -c[1])); }
    else if c[1] > 1e-9 { s.push_str(&format!(" + {:.4}λ", c[1])); }
    if c[0] < -1e-9 { s.push_str(&format!(" − {:.4}", -c[0])); }
    else if c[0] > 1e-9 { s.push_str(&format!(" + {:.4}", c[0])); }
    s
}

fn quadratic_roots(a: f64, b: f64, c: f64) -> Vec<f64> {
    let disc = b*b - 4.0*a*c;
    if disc < -1e-9 { return vec![]; }
    if disc.abs() < 1e-9 { return vec![-b / (2.0*a)]; }
    let sq = disc.sqrt();
    vec![(-b-sq)/(2.0*a), (-b+sq)/(2.0*a)]
}

fn eval_cubic(c: &[f64; 4], x: f64) -> f64 {
    c[0] + c[1]*x + c[2]*x*x + c[3]*x*x*x
}

fn eigenvalues_2x2(d: &[f64]) -> Vec<f64> {
    let c = char_poly_2x2(d);
    quadratic_roots(1.0, c[1], c[0])
}

fn eigenvalues_3x3(d: &[f64]) -> Vec<f64> {
    let c = char_poly_3x3(d);
    let mut roots = Vec::new();
    let bound = (c[0].abs() + c[1].abs() + c[2].abs() + 1.0).ceil() as i64;
    for n_try in -bound..=bound {
        let x = n_try as f64;
        if eval_cubic(&c, x).abs() < 1e-6 {
            if !roots.iter().any(|&r: &f64| (r-x).abs() < 1e-6) { roots.push(x); }
        }
    }
    for n_try in -2*bound..=2*bound {
        let x = n_try as f64 / 2.0;
        if eval_cubic(&c, x).abs() < 1e-6 {
            if !roots.iter().any(|&r: &f64| (r-x).abs() < 1e-6) { roots.push(x); }
        }
    }
    if roots.len() == 1 {
        let r1 = roots[0];
        let a3 = c[3]; let a2 = c[2]+a3*r1; let a1 = c[1]+a2*r1;
        let mut qroots = quadratic_roots(a3, a2, a1);
        for qr in qroots.drain(..) {
            if !roots.iter().any(|&r: &f64| (r-qr).abs() < 1e-6) { roots.push(qr); }
        }
    }
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    roots
}

fn find_eigenvector(d: &[f64], n: usize, lambda: f64) -> Option<Vec<f64>> {
    let mut mat: Vec<Vec<f64>> = (0..n).map(|r| {
        (0..n).map(|c| entry(d, n, r, c) - if r == c { lambda } else { 0.0 }).collect()
    }).collect();

    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();
    for col in 0..n {
        let mut found = None;
        for r in pivot_row..n {
            if mat[r][col].abs() > 1e-8 { found = Some(r); break; }
        }
        let pr = match found { None => continue, Some(r) => r };
        mat.swap(pivot_row, pr);
        let piv = mat[pivot_row][col];
        for c in 0..n { mat[pivot_row][c] /= piv; }
        for r in 0..n {
            if r == pivot_row { continue; }
            let f = mat[r][col];
            for c in 0..n { let sub = f*mat[pivot_row][c]; mat[r][c] -= sub; }
        }
        pivot_cols.push(col);
        pivot_row += 1;
    }

    let free_cols: Vec<usize> = (0..n).filter(|c| !pivot_cols.contains(c)).collect();
    if free_cols.is_empty() { return None; }

    let free = free_cols[0];
    let mut v = vec![0.0f64; n];
    v[free] = 1.0;
    for (pi, &pc) in pivot_cols.iter().enumerate() {
        if pi < mat.len() {
            v[pc] = -mat[pi].get(free).copied().unwrap_or(0.0);
        }
    }

    let norm: f64 = v.iter().map(|x| x*x).sum::<f64>().sqrt();
    if norm > 1e-10 { for x in &mut v { *x /= norm; } }
    Some(v)
}

fn markov_str(n: usize, data: &[f64], steps: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!("Markov Chain ({}x{})\n", n, n));
    out.push_str(&display_fmat_str(data, n));

    let col_sums: Vec<f64> = (0..n).map(|c| (0..n).map(|r| entry(data,n,r,c)).sum()).collect();
    let row_sums: Vec<f64> = (0..n).map(|r| (0..n).map(|c| entry(data,n,r,c)).sum()).collect();
    let is_col = col_sums.iter().all(|&s| (s-1.0).abs() < 1e-6);
    let is_row = row_sums.iter().all(|&s| (s-1.0).abs() < 1e-6);

    if !is_col && !is_row {
        out.push_str("Warning: not a valid stochastic matrix.\n");
    }

    let mut state: Vec<f64> = vec![1.0 / n as f64; n];
    out.push_str(&format!("Start: ({})\n", state.iter().map(|x| format!("{:.4}", x)).collect::<Vec<_>>().join(", ")));
    out.push_str("Iterating:\n");

    let max_display = steps.min(20);
    for step in 0..steps {
        let mut new_state = vec![0.0f64; n];
        if is_col {
            for r in 0..n { for c in 0..n { new_state[r] += entry(data,n,r,c)*state[c]; } }
        } else {
            for r in 0..n { for c in 0..n { new_state[c] += state[r]*entry(data,n,r,c); } }
        }
        let change: f64 = new_state.iter().zip(state.iter()).map(|(a,b)| (a-b).abs()).sum();
        state = new_state;

        if step < max_display || step == steps-1 {
            let s: Vec<String> = state.iter().map(|x| format!("{:.4}", x)).collect();
            out.push_str(&format!("  Step {:>3}: ({})  Δ={:.6}\n", step+1, s.join(", "), change));
        } else if step == max_display {
            out.push_str("  ...\n");
        }
        if change < 1e-8 {
            out.push_str(&format!("\nConverged after {} steps!\n", step+1));
            break;
        }
    }
    let s: Vec<String> = state.iter().map(|x| format!("{:.4}", x)).collect();
    out.push_str(&format!("Steady state: ({})\n", s.join(", ")));
    out.push_str("(= eigenvector of P with eigenvalue 1)\n");
    out
}

fn show_help() -> String {
    let mut out = String::new();
    out.push_str("Commands:\n");
    out.push_str("  mat <n> <entries>     set current n×n matrix (n=2 or 3)\n");
    out.push_str("  show                  display current matrix\n");
    out.push_str("  char_poly             characteristic polynomial det(A-λI)\n");
    out.push_str("  eigenvalues           find eigenvalues\n");
    out.push_str("  eigenvector <lambda>  eigenvector for given λ\n");
    out.push_str("  power <n>             power iteration\n");
    out.push_str("  diag                  check diagonalizability\n");
    out.push_str("  markov <n> <entries>  Markov chain steady state\n");
    out.push_str("  demo                  showcase\n");
    out.push_str("  quit                  exit\n");
    out
}

// ── State helpers ─────────────────────────────────────────────────────────────

fn default_state() -> StateMap {
    let mut s = state_new();
    // Default: 2×2 matrix [[4,1],[2,3]]
    state_set_int(&mut s, "mat_n", 2);
    // Store as flat float array
    let data: Vec<i64> = vec![40, 10, 20, 30]; // scaled by 10
    // Actually store as ints via a hack: use state_set_ints with scaled values
    // We'll use a different key with a flag for f64 via strings
    state_set_str(&mut s, "mat_data", "4 1 2 3");
    s
}

fn get_current_mat(state: &StateMap) -> Option<(usize, Vec<f64>)> {
    let n = state_get_int(state, "mat_n")? as usize;
    let data_str = state_get_str(state, "mat_data")?;
    let data: Vec<f64> = data_str.split_whitespace().filter_map(|s| s.parse().ok()).collect();
    if data.len() != n*n { return None; }
    Some((n, data))
}

fn set_current_mat(state: &mut StateMap, n: usize, data: &[f64]) {
    state_set_int(state, "mat_n", n as i64);
    let data_str = data.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(" ");
    state_set_str(state, "mat_data", &data_str);
}

// ── run_cmd ───────────────────────────────────────────────────────────────────

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    let mut out = String::new();
    match cmd {
        "help" | "h" => {
            out.push_str(&show_help());
        }

        "demo" => {
            out.push_str("=== Eigentheory Demo ===\n\n");
            // 2×2
            let d2 = vec![4.0f64, 1.0, 2.0, 3.0];
            out.push_str("2×2 matrix A = [[4,1],[2,3]]:\n");
            out.push_str(&display_fmat_str(&d2, 2));
            let c2 = char_poly_2x2(&d2);
            out.push_str(&format!("char poly: {}\n", poly_display_2(&c2)));
            let eigs2 = eigenvalues_2x2(&d2);
            for (i, &lam) in eigs2.iter().enumerate() {
                out.push_str(&format!("  λ{} = {:.4}\n", i+1, lam));
                if let Some(v) = find_eigenvector(&d2, 2, lam) {
                    let vs: Vec<String> = v.iter().map(|x| format!("{:.4}", x)).collect();
                    out.push_str(&format!("  eigenvector: ({})\n", vs.join(", ")));
                }
            }
            out.push('\n');

            // 3×3 diagonal
            let d3 = vec![1.0,0.0,0.0, 0.0,2.0,0.0, 0.0,0.0,3.0];
            out.push_str("3×3 diagonal [[1,0,0],[0,2,0],[0,0,3]]:\n");
            let eigs3 = eigenvalues_3x3(&d3);
            out.push_str("Eigenvalues: ");
            out.push_str(&eigs3.iter().map(|l| format!("{:.1}", l)).collect::<Vec<_>>().join(", "));
            out.push('\n');
            out.push_str("Diagonalizable: 3 distinct real eigenvalues.\n\n");

            // Markov
            out.push_str("Markov chain example:\n");
            let markov_data = vec![0.8,0.3, 0.2,0.7];
            out.push_str(&markov_str(2, &markov_data, 30));
        }

        "mat" => {
            let n = match parse_uint(args, 0, "n") {
                Some(n) if n == 2 || n == 3 => n as usize,
                _ => return "n must be 2 or 3.\n".to_string(),
            };
            let expected = n*n;
            if args.len() - 1 != expected {
                return format!("Expected {} entries for {}×{} matrix.\n", expected, n, n);
            }
            let data: Vec<f64> = match args[1..].iter().map(|s| s.parse::<f64>()).collect::<Result<_,_>>() {
                Ok(v) => v, Err(_) => return "Non-numeric entry.\n".to_string(),
            };
            set_current_mat(state, n, &data);
            out.push_str(&format!("Defined {}×{} matrix:\n", n, n));
            out.push_str(&display_fmat_str(&data, n));
        }

        "show" => {
            match get_current_mat(state) {
                None => out.push_str("No matrix. Use 'mat'.\n"),
                Some((n, data)) => {
                    out.push_str(&format!("Current {}×{} matrix:\n", n, n));
                    out.push_str(&display_fmat_str(&data, n));
                }
            }
        }

        "char_poly" => {
            match get_current_mat(state) {
                None => out.push_str("No matrix. Use 'mat'.\n"),
                Some((n, data)) => {
                    out.push_str("Characteristic polynomial det(A - λI):\n\n");
                    if n == 2 {
                        let c = char_poly_2x2(&data);
                        let tr = data[0]+data[3];
                        let det = data[0]*data[3]-data[1]*data[2];
                        out.push_str(&format!("p(λ) = λ² - {:.4}λ + {:.4}\n", tr, det));
                        out.push_str(&format!("     = {}\n", poly_display_2(&c)));
                        out.push_str(&format!("trace(A) = {:.4}\n", tr));
                        out.push_str(&format!("det(A)   = {:.4}\n", det));
                    } else {
                        let c = char_poly_3x3(&data);
                        let tr = data[0]+data[4]+data[8];
                        out.push_str(&format!("p(λ) = {}\n", poly_display_3(&c)));
                        out.push_str(&format!("trace(A) = {:.4}\n", tr));
                    }
                }
            }
        }

        "eigenvalues" => {
            match get_current_mat(state) {
                None => out.push_str("No matrix. Use 'mat'.\n"),
                Some((n, data)) => {
                    let roots = if n == 2 {
                        let c = char_poly_2x2(&data);
                        out.push_str(&format!("p(λ) = {}\n", poly_display_2(&c)));
                        eigenvalues_2x2(&data)
                    } else {
                        let c = char_poly_3x3(&data);
                        out.push_str(&format!("p(λ) = {}\n", poly_display_3(&c)));
                        eigenvalues_3x3(&data)
                    };
                    if roots.is_empty() {
                        out.push_str("No real eigenvalues (all complex).\n");
                    } else {
                        for (i, &lam) in roots.iter().enumerate() {
                            out.push_str(&format!("  λ{} = {:.6}\n", i+1, lam));
                        }
                    }
                }
            }
        }

        "eigenvector" => {
            let lambda: f64 = match args.get(0).and_then(|s| s.parse().ok()) {
                Some(x) => x, None => return "Usage: eigenvector <lambda>\n".to_string(),
            };
            match get_current_mat(state) {
                None => out.push_str("No matrix. Use 'mat'.\n"),
                Some((n, data)) => {
                    match find_eigenvector(&data, n, lambda) {
                        None => out.push_str(&format!("{:.4} is not an eigenvalue.\n", lambda)),
                        Some(v) => {
                            let s: Vec<String> = v.iter().map(|x| format!("{:.4}", x)).collect();
                            out.push_str(&format!("Eigenvector for λ={:.4}: ({})\n", lambda, s.join(", ")));
                            // Verify
                            let av: Vec<f64> = (0..n).map(|r| (0..n).map(|c| entry(&data,n,r,c)*v[c]).sum()).collect();
                            let lv: Vec<f64> = v.iter().map(|&x| x*lambda).collect();
                            let err: f64 = av.iter().zip(lv.iter()).map(|(a,b)| (a-b).abs()).sum();
                            if err < 1e-6 { out.push_str("Verified: Av = λv ✓\n"); }
                        }
                    }
                }
            }
        }

        "power" => {
            let iters = parse_uint(args, 0, "n").unwrap_or(20) as usize;
            match get_current_mat(state) {
                None => out.push_str("No matrix. Use 'mat'.\n"),
                Some((n, data)) => {
                    let fmat = FMat::new(n, n, data.clone());
                    let mut v: Vec<f64> = vec![1.0; n];
                    let mut prev_lambda = 0.0f64;
                    out.push_str(&format!("Power iteration ({} steps):\n", iters));
                    for iter in 0..iters {
                        let mut w = vec![0.0f64; n];
                        for r in 0..n { for c in 0..n { w[r] += fmat[(r,c)]*v[c]; } }
                        let max_abs = w.iter().map(|x| x.abs()).fold(0.0f64, f64::max);
                        if max_abs < 1e-15 { break; }
                        let lambda = w.iter().zip(v.iter())
                            .map(|(a,b)| if b.abs() > 1e-10 { a/b } else { 0.0 })
                            .find(|x| x.abs() > 1e-10).unwrap_or(max_abs);
                        v = w.iter().map(|x| x/max_abs).collect();
                        let change = (lambda-prev_lambda).abs();
                        if iter < 6 || (iter+1)%5 == 0 || iter == iters-1 {
                            let vs: Vec<String> = v.iter().map(|x| format!("{:.4}", x)).collect();
                            out.push_str(&format!("  iter {:>3}: λ≈{:>10.6}  Δλ={:.2e}  v=({})\n",
                                iter+1, lambda, change, vs.join(", ")));
                        }
                        prev_lambda = lambda;
                    }
                    let (dominant_ev, dominant_vec) = fmat.power_iteration(iters);
                    out.push_str(&format!("Dominant eigenvalue: {:.6}\n", dominant_ev));
                    let vs: Vec<String> = dominant_vec.iter().map(|x| format!("{:.4}", x)).collect();
                    out.push_str(&format!("Dominant eigenvector: ({})\n", vs.join(", ")));
                }
            }
        }

        "diag" => {
            match get_current_mat(state) {
                None => out.push_str("No matrix. Use 'mat'.\n"),
                Some((n, data)) => {
                    let roots = if n == 2 { eigenvalues_2x2(&data) } else { eigenvalues_3x3(&data) };
                    out.push_str(&format!("Eigenvalues found: {}\n", roots.len()));
                    for (i, &r) in roots.iter().enumerate() {
                        out.push_str(&format!("  λ{} = {:.6}\n", i+1, r));
                    }
                    if roots.len() == n {
                        let distinct = roots.windows(2).all(|w| (w[1]-w[0]).abs() > 1e-6);
                        if distinct {
                            out.push_str(&format!("DIAGONALIZABLE: {} distinct eigenvalues.\n", n));
                            out.push_str("A = P D P^-1  where D = diag(λ1,...,λn)\n");
                        } else {
                            out.push_str("Repeated eigenvalues: may or may not be diagonalizable.\n");
                        }
                    } else {
                        out.push_str(&format!("Only {} real eigenvalue(s) for {}×{} matrix.\n", roots.len(), n, n));
                        out.push_str("Not diagonalizable over R.\n");
                    }
                }
            }
        }

        "markov" => {
            if args.is_empty() { return "Usage: markov <n> <entries...>\n".to_string(); }
            let n: usize = match args[0].parse::<usize>() {
                Ok(k) if k >= 2 && k <= 6 => k,
                _ => return "n must be 2–6.\n".to_string(),
            };
            if args.len() - 1 != n*n {
                return format!("Expected {} entries.\n", n*n);
            }
            let data: Vec<f64> = match args[1..].iter().map(|s| s.parse::<f64>()).collect::<Result<_,_>>() {
                Ok(v) => v, Err(_) => return "Non-numeric entry.\n".to_string(),
            };
            out.push_str(&markov_str(n, &data, 100));
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
    c.title("Eigenvalue Diagram");

    let (n, data) = get_current_mat(state)
        .unwrap_or_else(|| (2, vec![4.0,1.0,2.0,3.0]));

    // Draw matrix
    let cell = 44.0;
    let mat_x = 60.0; let mat_y = 70.0;
    let data_str = data.iter().map(|x| {
        if x.fract().abs() < 1e-9 && x.abs() < 1e6 { format!("{}", *x as i64) }
        else { format!("{:.2}", x) }
    }).collect::<Vec<_>>();

    // Draw float matrix manually
    for (ri, row) in (0..n).enumerate() {
        for ci in 0..n {
            let x = mat_x + ci as f64 * cell;
            let y = mat_y + ri as f64 * cell;
            c.rect(x, y, cell, cell, colors::LIGHT, colors::GREY, 0.5);
            c.text(x + cell/2.0, y + cell/2.0, &data_str[ri*n+ci], 13.0, colors::DARK, "middle");
        }
    }
    c.text_bold(mat_x + n as f64*cell/2.0, mat_y - 15.0, "A", 14.0, colors::DARK, "middle");

    // Eigenvalues
    let roots = if n == 2 { eigenvalues_2x2(&data) } else { eigenvalues_3x3(&data) };

    let ev_x = 350.0;
    c.text_bold(ev_x, 70.0, "Eigenvalues", 14.0, colors::DARK, "middle");

    for (i, &lam) in roots.iter().enumerate() {
        let y = 100.0 + i as f64 * 80.0;
        let color = [colors::BLUE, colors::GREEN, colors::MAGENTA][i % 3];

        c.circle(ev_x, y, 28.0, colors::LIGHT, color, 2.0);
        c.text_bold(ev_x, y, &format!("λ{}={:.3}", i+1, lam), 11.0, colors::DARK, "middle");

        // Eigenvector
        if let Some(v) = find_eigenvector(&data, n, lam) {
            let vs: Vec<String> = v.iter().map(|x| format!("{:.2}", x)).collect();
            c.text(ev_x + 50.0, y, &format!("v=({})", vs.join(",")), 10.0, colors::GREY, "start");
        }

        // Arrow from matrix
        c.arrow(mat_x + n as f64*cell + 5.0, mat_y + n as f64*cell/2.0,
            ev_x - 32.0, y, colors::GREY, 1.0);
    }

    if roots.is_empty() {
        c.text(ev_x, 150.0, "No real eigenvalues", 13.0, colors::RED, "middle");
    }

    // Status
    let diag = roots.len() == n && roots.windows(2).all(|w| (w[1]-w[0]).abs() > 1e-6);
    let status = if diag { "DIAGONALIZABLE" } else if roots.len() < n { "NOT diagonalizable over R" } else { "Repeated eigenvalues" };
    c.text_bold(350.0, 360.0, status, 13.0,
        if diag { colors::GREEN } else { colors::RED }, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let (n, data) = get_current_mat(state)
        .unwrap_or_else(|| (2, vec![4.0,1.0,2.0,3.0]));

    g.node_default("style", "filled,rounded"); g.node_default("shape", "box");

    g.node("A", &[("label", &format!("A ({}x{})", n, n)), ("fillcolor", "#b2dfdb")]);
    g.node("CP", &[("label", "char poly det(A-λI)"), ("fillcolor", "#fff9c4")]);
    g.edge("A", "CP", &[("label", "compute")]);

    let roots = if n == 2 { eigenvalues_2x2(&data) } else { eigenvalues_3x3(&data) };
    for (i, &lam) in roots.iter().enumerate() {
        let id = format!("ev{}", i+1);
        g.node(&id, &[("label", &format!("λ{} = {:.4}", i+1, lam)), ("fillcolor", "#c8e6c9")]);
        g.edge("CP", &id, &[("label", "root")]);

        if let Some(v) = find_eigenvector(&data, n, lam) {
            let vs: Vec<String> = v.iter().map(|x| format!("{:.3}", x)).collect();
            let vid = format!("ev{}_vec", i+1);
            g.node(&vid, &[("label", &format!("v=({})", vs.join(","))), ("fillcolor", "#e8f5e9")]);
            g.edge(&id, &vid, &[("label", "eigenvector")]);
        }
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let (n, data) = get_current_mat(state)
        .unwrap_or_else(|| (2, vec![4.0,1.0,2.0,3.0]));
    let roots = if n == 2 { eigenvalues_2x2(&data) } else { eigenvalues_3x3(&data) };

    t.node("A", 0.0, 0.0, "$A$", "draw,fill=blue!20,minimum size=1.5cm");
    t.node("CP", 3.0, 0.0, "$\\det(A-\\lambda I)$", "draw,fill=yellow!30");
    t.arrow("A", "CP", "", "-stealth");

    for (i, &lam) in roots.iter().take(3).enumerate() {
        let x = 6.0; let y = (i as f64 - (roots.len()-1) as f64/2.0) * 2.0;
        t.node_math(&format!("ev{}", i+1), x, y, &format!("\\lambda_{}={:.3}", i+1, lam), "draw,circle,fill=green!20");
        t.arrow("CP", &format!("ev{}", i+1), "", "-stealth");
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let (n, data) = get_current_mat(state)
        .unwrap_or_else(|| (2, vec![4.0,1.0,2.0,3.0]));

    a.text_at(0, 0, "Eigenvalue Diagram");
    a.hline(0, 1, 40, '=');
    a.text_at(0, 3, &format!("Matrix A ({}x{}):", n, n));
    for r in 0..n {
        let row: Vec<String> = (0..n).map(|c| format!("{:6.2}", entry(&data,n,r,c))).collect();
        a.text_at(2, 4 + r as i32, &format!("[{}]", row.join(" ")));
    }

    let roots = if n == 2 { eigenvalues_2x2(&data) } else { eigenvalues_3x3(&data) };
    let y0 = n as i32 + 6;
    a.text_at(0, y0, "Eigenvalues:");
    for (i, &lam) in roots.iter().enumerate() {
        a.text_at(2, y0 + 1 + i as i32, &format!("λ{} = {:.6}", i+1, lam));
        if let Some(v) = find_eigenvector(&data, n, lam) {
            let vs: Vec<String> = v.iter().map(|x| format!("{:.3}", x)).collect();
            a.text_at(20, y0 + 1 + i as i32, &format!("  v=({})", vs.join(", ")));
        }
    }
    let diag = roots.len() == n && roots.windows(2).all(|w| (w[1]-w[0]).abs() > 1e-6);
    a.text_at(0, y0 + roots.len() as i32 + 2,
        if diag { "DIAGONALIZABLE" } else { "Not diagonalizable over R" });
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
                    let mut g = DotGraph::digraph("ch09");
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
            print_banner("Chapter 9", "Eigentheory",
                "Eigenvalues, eigenvectors, power iteration, diagonalization");
            print_info("Type 'help' for commands.");
            print_note("Try: mat 2 4 1 3 2   then   char_poly   then   eigenvalues");
            repl("ch9> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
