use common::*;

// ── String-accumulating display helpers ───────────────────────────────────────

fn s_section(out: &mut String, title: &str) {
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ {}\x1b[0m\n", title));
}
fn s_result(out: &mut String, label: &str, value: &str) {
    out.push_str(&format!("  \x1b[36m{}\x1b[0m \x1b[2m=\x1b[0m \x1b[32m{}\x1b[0m\n", label, value));
}
fn s_info(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[2m{}\x1b[0m\n", text));
}
fn s_note(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[33m◆\x1b[0m {}\n", text));
}
fn s_err(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[31m✗\x1b[0m {}\n", text));
}
fn s_ok(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[32m✓\x1b[0m {}\n", text));
}
fn s_sep(out: &mut String) {
    out.push_str(&format!("  \x1b[2m{}\x1b[0m\n", "─".repeat(60)));
}

fn show_help() -> String {
    let commands: &[(&str, &str)] = &[
        ("complex <d1_rows> <d1_cols> <entries...> ; <d2_rows> <d2_cols> <entries...>",
                                        "define chain complex by boundary matrices"),
        ("homology <same format>",       "compute homology H_n = ker(∂_n)/im(∂_{n+1})"),
        ("circle",                       "built-in: homology of S¹ (3 verts, 3 edges)"),
        ("sphere",                       "built-in: homology of S² (tetrahedron surface)"),
        ("torus",                        "built-in: homology of T²"),
        ("euler <same format>",          "compute Euler characteristic χ = Σ(-1)^i rank(H_i)"),
        ("chain_homotopy",               "example of chain homotopy inducing same map on homology"),
        ("demo",                         "showcase: circle, sphere, torus homology"),
        ("help",                         "show this help"),
        ("quit",                         "exit"),
    ];
    let mut out = String::new();
    out.push('\n');
    out.push_str("  \x1b[1mCommands:\x1b[0m\n");
    let max = commands.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
    for (cmd, desc) in commands {
        out.push_str(&format!("    {:<width$}  \x1b[2m{}\x1b[0m\n",
            format!("\x1b[36m{}\x1b[0m", cmd), desc, width = max + 2 + 9));
    }
    out.push('\n');
    out
}

// ── Smith normal form helpers ─────────────────────────────────────────────────

fn snf_rank(m: &Mat) -> usize {
    let (d, _) = m.smith_normal_form();
    let min = d.rows.min(d.cols);
    (0..min).filter(|&k| d[(k, k)] != 0).count()
}

fn snf_torsion(m: &Mat) -> Vec<i64> {
    if m.rows == 0 || m.cols == 0 { return vec![]; }
    let (d, _) = m.smith_normal_form();
    let min = d.rows.min(d.cols);
    (0..min).filter_map(|k| {
        let v = d[(k, k)];
        if v > 1 { Some(v) } else { None }
    }).collect()
}

fn format_homology(betti: usize, torsion: &[i64]) -> String {
    let mut parts = Vec::new();
    for _ in 0..betti { parts.push("ℤ".to_string()); }
    for &t in torsion { parts.push(format!("ℤ/{}ℤ", t)); }
    if parts.is_empty() { "0".to_string() } else { parts.join(" ⊕ ") }
}

// ── Parse chain complex from args ─────────────────────────────────────────────

fn parse_matrices(args: &[&str]) -> Option<Vec<Mat>> {
    let mut mats = Vec::new();
    let mut pos = 0;
    while pos < args.len() {
        while pos < args.len() && args[pos] == ";" { pos += 1; }
        if pos >= args.len() { break; }
        let r: usize = match args[pos].parse() { Ok(v) => v, Err(_) => { print_err("Expected rows"); return None; } };
        pos += 1;
        if pos >= args.len() { print_err("Expected cols"); return None; }
        let c: usize = match args[pos].parse() { Ok(v) => v, Err(_) => { print_err("Expected cols"); return None; } };
        pos += 1;
        let need = r * c;
        if pos + need > args.len() { print_err("Not enough matrix entries"); return None; }
        let data: Option<Vec<i64>> = args[pos..pos+need].iter().map(|s| s.parse().ok()).collect();
        let data = match data { Some(d) => d, None => { print_err("Non-integer in matrix"); return None; } };
        mats.push(Mat::new(r, c, data));
        pos += need;
    }
    Some(mats)
}

fn verify_and_display_complex(mats: &[Mat]) -> String {
    let mut out = String::new();
    s_section(&mut out, "Chain Complex Verification (∂∘∂ = 0)");
    for i in 0..mats.len().saturating_sub(1) {
        let d1 = &mats[i];
        let d2 = &mats[i + 1];
        if d1.cols != d2.rows {
            s_err(&mut out, &format!("  ∂_{} ∘ ∂_{}: dimension mismatch ({} cols vs {} rows)", i+1, i+2, d1.cols, d2.rows));
            continue;
        }
        let comp = d1.mul(d2);
        let all_zero = comp.data.iter().all(|&x| x == 0);
        if all_zero {
            s_ok(&mut out, &format!("  ∂_{} ∘ ∂_{} = 0 ✓", i+1, i+2));
        } else {
            s_err(&mut out, &format!("  ∂_{} ∘ ∂_{} ≠ 0 — NOT a valid chain complex!", i+1, i+2));
            out.push_str(&format!("{}", comp));
        }
    }
    out
}

fn cmd_complex(args: &[&str]) -> String {
    let mats = match parse_matrices(args) { Some(m) => m, None => return String::new() };
    if mats.is_empty() { let mut o = String::new(); s_err(&mut o, "No matrices provided"); return o; }
    let mut out = String::new();
    s_section(&mut out, "Chain Complex");
    for (i, m) in mats.iter().enumerate() {
        out.push_str(&format!("  ∂_{}: {}×{}\n", i+1, m.rows, m.cols));
        out.push_str(&format!("{}", m));
    }
    out.push_str(&verify_and_display_complex(&mats));
    out
}

fn cmd_homology_from_mats(mats: &[Mat]) -> String {
    let mut out = String::new();
    out.push_str(&verify_and_display_complex(mats));
    s_section(&mut out, "Homology Groups");

    let n = mats.len();
    let mut euler_chi: i64 = 0;

    for k in 0..=n {
        let d_out = if k > 0 { &mats[k-1] } else {
            &Mat::zero(0, if !mats.is_empty() { mats[0].cols } else { 1 })
        };
        let zero_out = if k == 0 {
            let cols = if !mats.is_empty() { mats[0].cols } else { 1 };
            Mat::zero(0, cols)
        } else {
            Mat::zero(0, 0)
        };
        let boundary_out = if k > 0 { d_out } else { &zero_out };

        let zero_in_mat;
        let boundary_in = if k < n {
            &mats[k]
        } else {
            let rows = mats[n-1].rows;
            zero_in_mat = Mat::zero(rows, 0);
            &zero_in_mat
        };

        let rank_out = if k > 0 { snf_rank(boundary_out) } else { 0 };
        let cols_k = if k > 0 {
            boundary_out.cols
        } else if !mats.is_empty() {
            mats[0].cols
        } else { 1 };
        let ker_dim = cols_k - rank_out;
        let im_dim = snf_rank(boundary_in);
        let betti = ker_dim.saturating_sub(im_dim);

        let torsion = snf_torsion(boundary_in);
        let h_str = format_homology(betti, &torsion);

        s_result(&mut out, &format!("H_{}", k), &h_str);

        let sign: i64 = if k % 2 == 0 { 1 } else { -1 };
        euler_chi += sign * betti as i64;
    }

    out.push('\n');
    s_result(&mut out, "Euler characteristic χ", &euler_chi.to_string());
    out
}

fn cmd_homology(args: &[&str]) -> String {
    let mats = match parse_matrices(args) { Some(m) => m, None => return String::new() };
    if mats.is_empty() { let mut o = String::new(); s_err(&mut o, "No matrices provided"); return o; }
    cmd_homology_from_mats(&mats)
}

fn cmd_euler(args: &[&str]) -> String {
    let mats = match parse_matrices(args) { Some(m) => m, None => return String::new() };
    if mats.is_empty() { let mut o = String::new(); s_err(&mut o, "No matrices provided"); return o; }
    let mut out = verify_and_display_complex(&mats);
    s_section(&mut out, "Euler Characteristic");
    let n = mats.len();
    let mut euler: i64 = 0;
    for k in 0..=n {
        let rank_out = if k > 0 { snf_rank(&mats[k-1]) } else { 0 };
        let cols_k = if k > 0 { mats[k-1].cols } else if !mats.is_empty() { mats[0].cols } else { 1 };
        let ker_dim = cols_k - rank_out;
        let im_dim = if k < n { snf_rank(&mats[k]) } else { 0 };
        let betti = ker_dim.saturating_sub(im_dim) as i64;
        let sign: i64 = if k % 2 == 0 { 1 } else { -1 };
        euler += sign * betti;
        s_info(&mut out, &format!("  rank H_{} = {} (contributes {}{})", k, betti, if sign > 0 { "+" } else { "-" }, betti.abs()));
    }
    s_result(&mut out, "χ = Σ (-1)^i·rank(H_i)", &euler.to_string());
    s_note(&mut out, "χ also equals Σ (-1)^i·dim(C_i) (alternating sum of chain group ranks)");
    out
}

// ── Built-in examples ─────────────────────────────────────────────────────────

fn cmd_circle() -> String {
    let mut out = String::new();
    s_section(&mut out, "Homology of S¹ (Circle)");
    out.push_str("  Triangulation: 3 vertices v₀,v₁,v₂ and 3 edges e₀=[v₀v₁], e₁=[v₁v₂], e₂=[v₀v₂]\n\n");
    out.push_str("  C₁ = ℤ³  (edges)   C₀ = ℤ³  (vertices)\n\n");
    let d1 = Mat::from_rows(vec![
        vec![-1, 0, -1],
        vec![ 1,-1,  0],
        vec![ 0, 1,  1],
    ]);
    out.push_str("  ∂₁ matrix (columns = ∂(eᵢ)):\n");
    out.push_str(&format!("{}", d1));

    let mats = vec![d1];
    out.push_str(&cmd_homology_from_mats(&mats));

    out.push('\n');
    s_note(&mut out, "H₀(S¹) = ℤ  (one connected component)");
    s_note(&mut out, "H₁(S¹) = ℤ  (one independent 1-cycle)");
    s_note(&mut out, "The generator of H₁ is e₀+e₁−e₂, representing the fundamental loop.");
    out
}

fn cmd_sphere() -> String {
    let mut out = String::new();
    s_section(&mut out, "Homology of S² (Sphere — Tetrahedron Surface)");
    out.push_str("  Triangulation: 4 vertices v₀,v₁,v₂,v₃; 6 edges; 4 triangles\n\n");
    let d2 = Mat::from_rows(vec![
        vec![ 1, 1, 0, 0],
        vec![-1, 0, 1, 0],
        vec![ 0,-1,-1, 0],
        vec![ 1, 0, 0, 1],
        vec![ 0, 1, 0,-1],
        vec![ 0, 0, 1, 1],
    ]);

    let d1 = Mat::from_rows(vec![
        vec![-1,-1,-1, 0, 0, 0],
        vec![ 1, 0, 0,-1,-1, 0],
        vec![ 0, 1, 0, 1, 0,-1],
        vec![ 0, 0, 1, 0, 1, 1],
    ]);

    out.push_str("  ∂₂ matrix (rows=edges, cols=triangles):\n");
    out.push_str(&format!("{}", d2));
    out.push_str("  ∂₁ matrix (rows=vertices, cols=edges):\n");
    out.push_str(&format!("{}", d1));

    let mats = vec![d1, d2];
    out.push_str(&cmd_homology_from_mats(&mats));

    out.push('\n');
    s_note(&mut out, "H₀(S²) = ℤ  (connected)");
    s_note(&mut out, "H₁(S²) = 0  (no 1-cycles mod boundary)");
    s_note(&mut out, "H₂(S²) = ℤ  (fundamental class — the whole sphere)");
    out
}

fn cmd_torus() -> String {
    let mut out = String::new();
    s_section(&mut out, "Homology of T² (Torus)");
    out.push_str("  Standard ΔCW structure: 1 vertex, 2 edges (a,b), 1 face (σ)\n");
    out.push_str("  ∂σ = a + b - a - b = 0   (the identification pattern of the torus)\n");
    out.push_str("  ∂a = v - v = 0,  ∂b = v - v = 0\n\n");
    let d2 = Mat::from_rows(vec![vec![0], vec![0]]);
    let d1 = Mat::from_rows(vec![vec![0, 0]]);

    out.push_str("  ∂₂: C₂→C₁  (zero — attaching map of torus)\n");
    out.push_str(&format!("{}", d2));
    out.push_str("  ∂₁: C₁→C₀  (zero — both edges are loops)\n");
    out.push_str(&format!("{}", d1));

    let mats = vec![d1, d2];
    out.push_str(&cmd_homology_from_mats(&mats));

    out.push('\n');
    s_note(&mut out, "H₀(T²) = ℤ   (connected)");
    s_note(&mut out, "H₁(T²) = ℤ²  (two independent loops: a and b)");
    s_note(&mut out, "H₂(T²) = ℤ   (fundamental class)");
    s_note(&mut out, "This matches π₁(T²) = ℤ² (torus has abelian fundamental group)");
    out
}

// ── Chain Homotopy ────────────────────────────────────────────────────────────

fn cmd_chain_homotopy() -> String {
    let mut out = String::new();
    s_section(&mut out, "Chain Homotopy");
    out.push('\n');
    out.push_str("  Two chain maps f,g: C• → D• are chain homotopic if there exist\n");
    out.push_str("  maps hₙ: Cₙ → Dₙ₊₁ such that  f - g = ∂h + h∂\n\n");
    out.push_str("  Key theorem: chain homotopic maps induce the same maps on homology.\n");
    s_sep(&mut out);
    out.push('\n');
    out.push_str("  Concrete example: the chain complex for a contractible space.\n");
    out.push_str("  Let C•: 0 → ℤ →id→ ℤ → 0  (this is exact, hence acyclic)\n\n");
    out.push_str("  The identity map id: C• → C• and the zero map 0: C• → C• are\n");
    out.push_str("  chain homotopic via the homotopy h: ℤ → ℤ given by h₀ = id.\n\n");

    let d1 = Mat::identity(1);
    out.push_str(&format!("  ∂₁ = {:?}\n", d1.data));

    out.push_str("  f - g = I - 0 = I\n");
    out.push_str("  ∂₁∘h₀ = I·I = I\n");
    out.push_str("  h₋₁∘∂₀ = 0 (no C₋₁)\n");
    out.push_str("  So ∂h + h∂ = I = f - g  ✓\n\n");
    s_ok(&mut out, "The identity and zero chain maps are homotopic — they induce same maps on H•.");
    out.push('\n');
    out.push_str("  Both H•(id) = H•(0) because H•(C) = 0 (acyclic complex).\n\n");
    s_note(&mut out, "Chain homotopy is the higher-categorical reason why homology is a functor");
    s_note(&mut out, "from the homotopy category of chain complexes to graded abelian groups.");
    s_note(&mut out, "Equivalences in this homotopy category are quasi-isomorphisms.");
    out
}

fn cmd_demo() -> String {
    let mut out = String::new();
    s_section(&mut out, "Demo: Chain Complex Showcase");
    out.push('\n');
    out.push_str("  --- Circle S¹ ---\n");
    out.push_str(&cmd_circle());
    out.push('\n');
    out.push_str("  --- Sphere S² ---\n");
    out.push_str(&cmd_sphere());
    out.push('\n');
    out.push_str("  --- Torus T² ---\n");
    out.push_str(&cmd_torus());
    out
}

fn default_state() -> StateMap {
    state_new()
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    if !args.is_empty() {
        let params: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
        if !params.is_empty() {
            state_set_ints(state, "last_params", &params);
        }
        state_set_str(state, "last_cmd", cmd);
    }
    match cmd {
        "complex"        => cmd_complex(args),
        "homology"       => cmd_homology(args),
        "circle"         => cmd_circle(),
        "sphere"         => cmd_sphere(),
        "torus"          => cmd_torus(),
        "euler"          => cmd_euler(args),
        "chain_homotopy" => cmd_chain_homotopy(),
        "demo"           => cmd_demo(),
        "help" | "h"     => show_help(),
        _ => {
            let mut out = String::new();
            s_err(&mut out, &format!("Unknown command '{}'. Type 'help'.", cmd));
            out
        }
    }
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { print!("{out}"); }
    true
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    c.text_bold(350.0, 40.0, "Chain Complex", 16.0, "#222", "middle");
    let y = 200.0;
    c.text(60.0, y, "...", 13.0, "#333", "middle");
    c.node_circle(160.0, y, "Cₙ₊₁", "white", 35.0, 11.0);
    c.node_circle(310.0, y, "Cₙ", "white", 35.0, 13.0);
    c.node_circle(460.0, y, "Cₙ₋₁", "white", 35.0, 11.0);
    c.text(580.0, y, "...", 13.0, "#333", "middle");
    c.arrow(80.0, y, 125.0, y, "#444", 1.5);
    c.arrow(195.0, y, 275.0, y, "#444", 1.5);
    c.arrow(345.0, y, 425.0, y, "#444", 1.5);
    c.arrow(495.0, y, 565.0, y, "#444", 1.5);
    c.text(230.0, 180.0, "∂ₙ₊₁", 12.0, "#333", "middle");
    c.text(380.0, 180.0, "∂ₙ", 12.0, "#333", "middle");
    c.text(250.0, 290.0, "∂ₙ ∘ ∂ₙ₊₁ = 0", 12.0, "#333", "start");
    c.text(200.0, 340.0, "Hₙ = ker(∂ₙ) / im(∂ₙ₊₁)", 12.0, "#333", "start");
    c.text(80.0, 420.0, "H*(S¹): ℤ, ℤ", 12.0, "#333", "start");
    c.text(270.0, 420.0, "H*(S²): ℤ, 0, ℤ", 12.0, "#333", "start");
    c.text(470.0, 420.0, "H*(T²): ℤ, ℤ², ℤ", 12.0, "#333", "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("dots1", &[("label", "..."), ("shape", "plaintext")]);
    g.node("Cn1", &[("label", "Cₙ₊₁"), ("shape", "box")]);
    g.node("Cn", &[("label", "Cₙ"), ("shape", "box")]);
    g.node("Cn_1", &[("label", "Cₙ₋₁"), ("shape", "box")]);
    g.node("dots2", &[("label", "..."), ("shape", "plaintext")]);
    g.edge("dots1", "Cn1", &[("label", "")]);
    g.edge("Cn1", "Cn", &[("label", "∂ₙ₊₁")]);
    g.edge("Cn", "Cn_1", &[("label", "∂ₙ")]);
    g.edge("Cn_1", "dots2", &[("label", "")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("cn1", 0.0, 0.0, "$C_{n+1}$", "");
    t.node("cn", 3.0, 0.0, "$C_n$", "");
    t.node("cn_1", 6.0, 0.0, "$C_{n-1}$", "");
    t.arrow("cn1", "cn", "$\\partial_{n+1}$", "above");
    t.arrow("cn", "cn_1", "$\\partial_n$", "above");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 1, "Chain Complex Diagram");
    a.text_at(0, 3, "... --> C_{n+1} --d_{n+1}--> C_n --d_n--> C_{n-1} --> ...");
    a.text_at(0, 5, "Key property: d_n o d_{n+1} = 0");
    a.text_at(0, 7, "H_n = ker(d_n) / im(d_{n+1})");
    a.hline(0, 60, 9, '-');
    a.text_at(0, 11, "S^1: H_0=Z  H_1=Z");
    a.text_at(0, 12, "S^2: H_0=Z  H_1=0  H_2=Z");
    a.text_at(0, 13, "T^2: H_0=Z  H_1=Z^2  H_2=Z");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch38"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 38", "Chain Complexes & Homology", "∂∘∂=0, kernels, images, and topological invariants");
            print!("{}", show_help());
            print_note("Try: circle   or   sphere   or   torus");
            print_note("Or: complex 2 3 -1 1 0 0 -1 1 ; 3 2 1 0 -1 1 0 -1");
            repl("ch38> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
