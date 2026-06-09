use common::*;

// ── Linear Maps Explorer ─────────────────────────────────────────────────────
// T: R^n → R^m represented by an m×n matrix.

fn parse_fmat(rows: usize, cols: usize, entries: &[&str]) -> Option<Vec<Vec<f64>>> {
    if entries.len() != rows * cols {
        return None;
    }
    let data: Vec<f64> = match entries.iter().map(|s| s.parse::<f64>()).collect::<Result<_,_>>() {
        Ok(v) => v,
        Err(_) => return None,
    };
    Some((0..rows).map(|r| data[r*cols..(r+1)*cols].to_vec()).collect())
}

fn display_fmat_str(m: &[Vec<f64>]) -> String {
    let mut out = String::new();
    for row in m {
        let s: Vec<String> = row.iter().map(|x| format!("{:8.4}", x)).collect();
        out.push_str(&format!("  │ {} │\n", s.join("  ")));
    }
    out
}

fn rref_pivots(mat: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<usize>) {
    let mut m = mat.to_vec();
    let rows = m.len();
    if rows == 0 { return (m, vec![]); }
    let cols = m[0].len();
    let mut pivot_row = 0;
    let mut pivot_cols = Vec::new();
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
            let f = m[r][col];
            for c in 0..cols { let sub = f * m[pivot_row][c]; m[r][c] -= sub; }
        }
        pivot_cols.push(col);
        pivot_row += 1;
    }
    (m, pivot_cols)
}

fn rank_of_mat(m: &[Vec<f64>]) -> usize { rref_pivots(m).1.len() }

fn kernel(mat: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows = mat.len();
    if rows == 0 { return vec![]; }
    let n = mat[0].len();
    let (rref, pivot_cols) = rref_pivots(mat);
    let rank = pivot_cols.len();
    let nullity = n - rank;
    if nullity == 0 { return vec![]; }

    let free_cols: Vec<usize> = (0..n).filter(|c| !pivot_cols.contains(c)).collect();
    let mut basis = Vec::new();
    for &free in &free_cols {
        let mut v = vec![0.0f64; n];
        v[free] = 1.0;
        for (pi, &pc) in pivot_cols.iter().enumerate() {
            if pi < rref.len() {
                v[pc] = -rref[pi].get(free).copied().unwrap_or(0.0);
            }
        }
        basis.push(v);
    }
    basis
}

fn image_basis(mat: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if mat.is_empty() { return vec![]; }
    let (_, pivot_cols) = rref_pivots(mat);
    let rows = mat.len();
    pivot_cols.iter().map(|&c| (0..rows).map(|r| mat[r][c]).collect()).collect()
}

fn mat_mul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    if a.is_empty() || b.is_empty() { return None; }
    let (m, k) = (a.len(), a[0].len());
    if k != b.len() { return None; }
    let n = b[0].len();
    Some((0..m).map(|i| (0..n).map(|j| (0..k).map(|l| a[i][l] * b[l][j]).sum()).collect()).collect())
}

fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if m.is_empty() { return vec![]; }
    let rows = m.len(); let cols = m[0].len();
    (0..cols).map(|j| (0..rows).map(|i| m[i][j]).collect()).collect()
}

fn vec_to_str(v: &[f64]) -> String {
    let parts: Vec<String> = v.iter().map(|x| format!("{:.4}", x)).collect();
    format!("({})", parts.join(", "))
}

fn show_help() -> String {
    let mut out = String::new();
    out.push_str("Commands:\n");
    out.push_str("  map <rows> <cols> <entries>              define current map T\n");
    out.push_str("  show                                     display T\n");
    out.push_str("  kernel                                   ker(T) = null space\n");
    out.push_str("  image                                    im(T) = column space\n");
    out.push_str("  ranknullity                              rank-nullity theorem\n");
    out.push_str("  compose <rA> <cA> <A> / <rB> <cB> <B>  A∘B\n");
    out.push_str("  dual                                     transpose Tᵀ\n");
    out.push_str("  demo                                     showcase of linear map concepts\n");
    out.push_str("  quit                                     exit\n");
    out
}

// ── State helpers ─────────────────────────────────────────────────────────────

fn default_state() -> StateMap {
    let mut s = state_new();
    // Default map: T: R^3 → R^2 given by [[1,0,2],[0,1,-1]]
    state_set_ints(&mut s, "map_rows", &[2]);
    state_set_ints(&mut s, "map_cols", &[3]);
    let m = Mat::new(2, 3, vec![1,0,2, 0,1,-1]);
    state_set_mat(&mut s, "current_map", &m);
    s
}

fn get_current_map(state: &StateMap) -> Option<(usize, usize, Vec<Vec<f64>>)> {
    let rows_v = state_get_ints(state, "map_rows")?;
    let cols_v = state_get_ints(state, "map_cols")?;
    let mat_i = state_get_mat(state, "current_map")?;
    let rows = rows_v[0] as usize;
    let cols = cols_v[0] as usize;
    let mat: Vec<Vec<f64>> = (0..rows).map(|r| (0..cols).map(|c| mat_i[(r,c)] as f64).collect()).collect();
    Some((rows, cols, mat))
}

fn set_current_map(state: &mut StateMap, rows: usize, cols: usize, mat: &[Vec<f64>]) {
    state_set_ints(state, "map_rows", &[rows as i64]);
    state_set_ints(state, "map_cols", &[cols as i64]);
    let data: Vec<i64> = mat.iter().flat_map(|r| r.iter().map(|&x| x.round() as i64)).collect();
    let m = Mat::new(rows, cols, data);
    state_set_mat(state, "current_map", &m);
}

// ── run_cmd ───────────────────────────────────────────────────────────────────

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    let mut out = String::new();
    match cmd {
        "help" | "h" => {
            out.push_str(&show_help());
        }

        "demo" => {
            out.push_str("=== Linear Maps Demo ===\n\n");
            // T: R^3 → R^2
            let rows = 2; let cols = 3;
            let mat = vec![vec![1.0,0.0,2.0], vec![0.0,1.0,-1.0]];
            out.push_str(&format!("T: R^{} → R^{}\n", cols, rows));
            out.push_str(&display_fmat_str(&mat));
            let rank = rank_of_mat(&mat);
            let nullity = cols - rank;
            out.push_str(&format!("rank(T) = {}, nullity(T) = {}, dim(domain) = {}\n", rank, nullity, cols));
            out.push_str(&format!("rank + nullity = {} + {} = {} ✓\n\n", rank, nullity, rank+nullity));

            let ker = kernel(&mat);
            out.push_str("Kernel basis:\n");
            for (i, v) in ker.iter().enumerate() { out.push_str(&format!("  k{}: {}\n", i+1, vec_to_str(v))); }
            out.push('\n');
            let im = image_basis(&mat);
            out.push_str("Image basis:\n");
            for (i, v) in im.iter().enumerate() { out.push_str(&format!("  i{}: {}\n", i+1, vec_to_str(v))); }
        }

        "map" => {
            if args.len() < 2 { return "Usage: map <rows> <cols> <entries...>\n".to_string(); }
            let rows: usize = match args[0].parse::<usize>() {
                Ok(r) if r > 0 => r,
                _ => return "rows must be positive integer\n".to_string(),
            };
            let cols: usize = match args[1].parse::<usize>() {
                Ok(c) if c > 0 => c,
                _ => return "cols must be positive integer\n".to_string(),
            };
            match parse_fmat(rows, cols, &args[2..]) {
                None => out.push_str(&format!("Expected {} entries for {}×{} matrix.\n", rows*cols, rows, cols)),
                Some(mat) => {
                    out.push_str(&format!("Defined T: R^{} → R^{}\n", cols, rows));
                    out.push_str(&display_fmat_str(&mat));
                    set_current_map(state, rows, cols, &mat);
                }
            }
        }

        "show" => {
            match get_current_map(state) {
                None => out.push_str("No map defined. Use 'map'.\n"),
                Some((rows, cols, mat)) => {
                    out.push_str(&format!("T: R^{} → R^{}\n", cols, rows));
                    out.push_str(&display_fmat_str(&mat));
                    out.push_str(&format!("rank = {}\n", rank_of_mat(&mat)));
                }
            }
        }

        "kernel" => {
            match get_current_map(state) {
                None => out.push_str("No map defined. Use 'map'.\n"),
                Some((_rows, _cols, mat)) => {
                    let ker = kernel(&mat);
                    out.push_str(&format!("dim(ker T) = {}\n", ker.len()));
                    if ker.is_empty() {
                        out.push_str("ker(T) = {0}  (T is injective)\n");
                    } else {
                        out.push_str("Basis for ker(T):\n");
                        for (i, v) in ker.iter().enumerate() {
                            out.push_str(&format!("  k{}: {}\n", i+1, vec_to_str(v)));
                        }
                    }
                }
            }
        }

        "image" => {
            match get_current_map(state) {
                None => out.push_str("No map defined. Use 'map'.\n"),
                Some((_rows, _cols, mat)) => {
                    let im = image_basis(&mat);
                    out.push_str(&format!("dim(im T) = rank = {}\n", im.len()));
                    if im.is_empty() {
                        out.push_str("T is the zero map; im(T) = {0}.\n");
                    } else {
                        out.push_str("Basis for im(T):\n");
                        for (i, v) in im.iter().enumerate() {
                            out.push_str(&format!("  i{}: {}\n", i+1, vec_to_str(v)));
                        }
                    }
                }
            }
        }

        "ranknullity" => {
            match get_current_map(state) {
                None => out.push_str("No map defined. Use 'map'.\n"),
                Some((rows, cols, mat)) => {
                    let rank = rank_of_mat(&mat);
                    let nullity = cols - rank;
                    out.push_str("Rank-Nullity Theorem:\n");
                    out.push_str(&format!("  dim(domain) = {}\n  rank(T) = {}\n  nullity(T) = {}\n",
                        cols, rank, nullity));
                    out.push_str(&format!("  rank + nullity = {} + {} = {} = n ✓\n", rank, nullity, rank+nullity));
                    if nullity == 0 { out.push_str("  T is injective.\n"); }
                    if rank == rows { out.push_str("  T is surjective.\n"); }
                    if rank == rows && nullity == 0 { out.push_str("  T is an isomorphism!\n"); }
                }
            }
        }

        "dual" => {
            match get_current_map(state) {
                None => out.push_str("No map defined. Use 'map'.\n"),
                Some((rows, cols, mat)) => {
                    let t = transpose(&mat);
                    out.push_str(&format!("Dual (Transpose): R^{} → R^{}\n", rows, cols));
                    out.push_str(&display_fmat_str(&t));
                    out.push_str("rank(T) = rank(Tᵀ)\n");
                }
            }
        }

        "compose" => {
            let full = args.join(" ");
            let parts: Vec<&str> = full.splitn(2, '/').collect();
            if parts.len() != 2 {
                return "Usage: compose <rA> <cA> <A entries> / <rB> <cB> <B entries>\n".to_string();
            }
            let parse_mat_spec = |s: &str| -> Option<(usize, usize, Vec<Vec<f64>>)> {
                let tokens: Vec<&str> = s.split_whitespace().collect();
                if tokens.len() < 2 { return None; }
                let r: usize = tokens[0].parse().ok()?;
                let c: usize = tokens[1].parse().ok()?;
                let mat = parse_fmat(r, c, &tokens[2..])?;
                Some((r, c, mat))
            };
            match (parse_mat_spec(parts[0].trim()), parse_mat_spec(parts[1].trim())) {
                (Some((ra, ca, a)), Some((rb, cb, b))) => {
                    if ca != rb {
                        out.push_str(&format!("Cannot compose: A is {}×{}, B is {}×{}\n", ra, ca, rb, cb));
                    } else {
                        match mat_mul(&a, &b) {
                            None => out.push_str("Composition failed.\n"),
                            Some(prod) => {
                                out.push_str(&format!("A∘B: R^{} → R^{}\n", cb, ra));
                                out.push_str(&display_fmat_str(&prod));
                                let _ = cb;
                            }
                        }
                    }
                }
                _ => out.push_str("Could not parse matrix specs.\n"),
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
    c.title("Linear Map Diagram");

    let (rows, cols, mat) = get_current_map(state)
        .unwrap_or_else(|| (2, 3, vec![vec![1.0,0.0,2.0], vec![0.0,1.0,-1.0]]));

    // Domain box (left)
    let dom_x = 80.0; let dom_cy = 200.0;
    c.rrect(dom_x - 60.0, dom_cy - 80.0, 120.0, 160.0, 8.0, "#e3f2fd", colors::BLUE, 2.0);
    c.text_bold(dom_x, dom_cy - 95.0, &format!("R^{}", cols), 14.0, colors::BLUE, "middle");

    let rank = rank_of_mat(&mat);
    let ker = kernel(&mat);
    let ker_dim = ker.len();

    // Show domain basis vectors
    for i in 0..cols.min(4) {
        let y = dom_cy - 50.0 + i as f64 * 35.0;
        c.circle(dom_x, y, 12.0, colors::LIGHT, colors::BLUE, 1.5);
        c.text(dom_x, y, &format!("e{}", i+1), 10.0, colors::DARK, "middle");
    }

    // Codomain box (right)
    let cod_x = 550.0;
    c.rrect(cod_x - 60.0, dom_cy - 80.0, 120.0, 160.0, 8.0, "#e8f5e9", colors::GREEN, 2.0);
    c.text_bold(cod_x, dom_cy - 95.0, &format!("R^{}", rows), 14.0, colors::GREEN, "middle");

    for i in 0..rows.min(4) {
        let y = dom_cy - 50.0 + i as f64 * 35.0;
        c.circle(cod_x, y, 12.0, colors::LIGHT, colors::GREEN, 1.5);
        c.text(cod_x, y, &format!("f{}", i+1), 10.0, colors::DARK, "middle");
    }

    // Arrows T
    c.arrow_labeled(dom_x + 65.0, dom_cy, cod_x - 65.0, dom_cy,
        "T", colors::MAGENTA, 2.5);

    // Im(T) box
    let im_h = rank as f64 * 30.0 + 10.0;
    let im_y = dom_cy - im_h / 2.0;
    c.rrect(cod_x - 55.0, im_y, 110.0, im_h, 4.0, "#c8e6c9", colors::GREEN, 2.0);
    c.text(cod_x, im_y - 12.0, &format!("im(T), dim={}", rank), 11.0, colors::GREEN, "middle");

    // Kernel annotation
    if ker_dim > 0 {
        c.rrect(dom_x - 55.0, dom_cy + 60.0, 110.0, 30.0, 4.0, "#fce4ec", colors::RED, 1.5);
        c.text(dom_x, dom_cy + 75.0, &format!("ker(T), dim={}", ker_dim), 11.0, colors::RED, "middle");
    }

    // Rank-nullity
    c.text(320.0, 370.0,
        &format!("rank({}) + nullity({}) = {} = dim(R^{})", rank, ker_dim, rank+ker_dim, cols),
        12.0, colors::DARK, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let (rows, cols, mat) = get_current_map(state)
        .unwrap_or_else(|| (2, 3, vec![vec![1.0,0.0,2.0], vec![0.0,1.0,-1.0]]));

    let rank = rank_of_mat(&mat);
    let ker_dim = cols - rank;

    g.node_default("style", "filled,rounded"); g.node_default("shape", "box");

    g.node("Dom", &[("label", &format!("R^{} (domain)", cols)), ("fillcolor", "#b2dfdb")]);
    g.node("Cod", &[("label", &format!("R^{} (codomain)", rows)), ("fillcolor", "#c8e6c9")]);
    g.node("Im",  &[("label", &format!("im(T), dim={}", rank)), ("fillcolor", "#a5d6a7")]);
    g.node("Ker", &[("label", &format!("ker(T), dim={}", ker_dim)), ("fillcolor", "#ffcdd2")]);

    g.edge("Dom", "Cod", &[("label", "T")]);
    g.edge("Dom", "Ker", &[("label", "null space")]);
    g.edge("T",   "Im",  &[]);
    g.edge("Im",  "Cod", &[("label", "subset")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let (rows, cols, mat) = get_current_map(state)
        .unwrap_or_else(|| (2, 3, vec![vec![1.0,0.0,2.0], vec![0.0,1.0,-1.0]]));

    let rank = rank_of_mat(&mat);
    let ker_dim = cols - rank;

    t.node("Dom",  0.0, 0.0, &format!("$\\mathbb{{R}}^{}$", cols), "draw,circle,fill=blue!20,minimum size=2cm");
    t.node("Cod",  6.0, 0.0, &format!("$\\mathbb{{R}}^{}$", rows), "draw,circle,fill=green!20,minimum size=2cm");
    t.node("Ker",  0.0, -3.0, &format!("$\\ker(T)$, $\\dim={}$", ker_dim), "draw,fill=red!10");
    t.node("Im",   6.0, -3.0, &format!("$\\operatorname{{im}}(T)$, $\\dim={}$", rank), "draw,fill=green!10");

    t.arrow("Dom", "Cod", "T", "-stealth,thick");
    t.arrow("Dom", "Ker", "", "-stealth,dashed,red");
    t.arrow("Im",  "Cod", "", "right hook-stealth");
    t.raw(&format!("  \\node[above=3mm] at (3,0) {{rank + nullity = ${}+{}={}$}};",
        rank, ker_dim, rank+ker_dim));
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let (rows, cols, mat) = get_current_map(state)
        .unwrap_or_else(|| (2, 3, vec![vec![1.0,0.0,2.0], vec![0.0,1.0,-1.0]]));
    let rank = rank_of_mat(&mat);
    let ker_dim = cols - rank;

    a.text_at(0, 0, &format!("Linear Map T: R^{} -> R^{}", cols, rows));
    a.hline(0, 1, 40, '=');

    // ASCII diagram
    a.box_at(0, 3, 12, 6);
    a.text_at(3, 6, &format!("R^{}", cols));
    a.arrow_right(12, 6, 10);
    a.text_at(14, 5, "T");
    a.box_at(22, 3, 12, 6);
    a.text_at(25, 6, &format!("R^{}", rows));

    a.text_at(0, 11, &format!("rank(T)    = {}", rank));
    a.text_at(0, 12, &format!("nullity(T) = {}", ker_dim));
    a.text_at(0, 13, &format!("rank + nullity = {} + {} = {} = dim(domain)",
        rank, ker_dim, rank+ker_dim));

    a.text_at(0, 15, "Matrix:");
    for (ri, row) in mat.iter().enumerate() {
        let s: Vec<String> = row.iter().map(|x| format!("{:6.2}", x)).collect();
        a.text_at(2, 16 + ri as i32, &format!("[ {} ]", s.join("  ")));
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
                    let mut g = DotGraph::digraph("ch06");
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
            print_banner("Chapter 6", "Linear Maps",
                "Kernels, images, rank-nullity, and linear transformations");
            print_info("Type 'help' for commands.");
            print_note("Try: map 2 3 1 0 2 0 1 -1   then   kernel");
            repl("ch6> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
