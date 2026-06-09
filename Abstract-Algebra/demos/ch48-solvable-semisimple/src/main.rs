use common::*;

fn show_help() -> String {
    help_string(&[
        ("derived <n>",          "Derived series of gl(2): g⁽⁰⁾⊃g⁽¹⁾⊃... up to n steps"),
        ("lower_central <n>",    "Lower central series of gl(2): g₀⊃g₁=[g,g₀]⊃..."),
        ("is_solvable <type>",   "Is gl2, sl2, b (upper-tri), n (strict upper-tri) solvable?"),
        ("levi",                 "Levi decomposition: g = rad ⋊ semisimple"),
        ("engel",                "Engel's theorem: all ad(x) nilpotent ⟹ g nilpotent"),
        ("lie_theorem",          "Lie's theorem: solvable ⟹ common eigenvector over ℂ"),
        ("demo",                 "Run a showcase of all commands"),
        ("help",                 "Show this help"),
        ("quit",                 "Exit"),
    ])
}

// We work with named bases for various subalgebras of gl(2).
// gl(2) basis: E11, E12, E21, E22
// sl(2) basis: e=E12, f=E21, h=E11-E22
// b (upper tri) basis: E11, E12, E22
// n (strict upper tri) basis: E12

fn gl2_bracket(i: usize, j: usize, k: usize, l: usize) -> Vec<(i64, usize)> {
    let idx = |r: usize, c: usize| -> Option<usize> {
        match (r,c) { (0,0)=>Some(0),(0,1)=>Some(1),(1,0)=>Some(2),(1,1)=>Some(3), _=>None }
    };
    let mut result = Vec::new();
    if j == k { if let Some(b) = idx(i,l) { result.push((1i64, b)); } }
    if l == i { if let Some(b) = idx(k,j) { result.push((-1i64, b)); } }
    result
}

fn gl2_commutator_subspace(s1: &[[i64;4]], s2: &[[i64;4]]) -> Vec<[i64;4]> {
    let mut result: Vec<[i64;4]> = Vec::new();
    let basis_pairs = [(0,0),(0,1),(1,0),(1,1)];
    let flat_idx = |r:usize,c:usize| -> usize { r*2+c };
    for v1 in s1 {
        for v2 in s2 {
            let mut res = [0i64;4];
            for &(r1,c1) in &basis_pairs {
                let a = v1[flat_idx(r1,c1)];
                if a == 0 { continue; }
                for &(r2,c2) in &basis_pairs {
                    let b = v2[flat_idx(r2,c2)];
                    if b == 0 { continue; }
                    for (coeff, bidx) in gl2_bracket(r1,c1,r2,c2) {
                        res[bidx] += a * b * coeff;
                    }
                }
            }
            if res != [0,0,0,0] { result.push(res); }
        }
    }
    reduce_span(result)
}

fn reduce_span(mut vecs: Vec<[i64;4]>) -> Vec<[i64;4]> {
    let mut basis: Vec<[i64;4]> = Vec::new();
    for v in vecs.drain(..) {
        if !is_in_span(&v, &basis) { basis.push(v); }
    }
    basis
}

fn is_in_span(v: &[i64;4], basis: &[[i64;4]]) -> bool {
    if basis.is_empty() { return v == &[0,0,0,0]; }
    let mut mat: Vec<Vec<f64>> = basis.iter().map(|b| b.iter().map(|&x| x as f64).collect()).collect();
    let vf: Vec<f64> = v.iter().map(|&x| x as f64).collect();
    let rank_before = rank_f64(&mat);
    mat.push(vf);
    let rank_after = rank_f64(&mat);
    rank_before == rank_after
}

fn rank_f64(mat: &Vec<Vec<f64>>) -> usize {
    if mat.is_empty() || mat[0].is_empty() { return 0; }
    let mut m: Vec<Vec<f64>> = mat.clone();
    let rows = m.len();
    let cols = m[0].len();
    let mut rank = 0;
    let mut pivot_col = 0;
    for r in 0..rows {
        if pivot_col >= cols { break; }
        let mut found = false;
        for rr in r..rows {
            if m[rr][pivot_col].abs() > 1e-9 { m.swap(r, rr); found = true; break; }
        }
        if !found { pivot_col += 1; continue; }
        let piv = m[r][pivot_col];
        for j in 0..cols { m[r][j] /= piv; }
        for rr in 0..rows {
            if rr != r {
                let f = m[rr][pivot_col];
                for j in 0..cols { m[rr][j] -= f * m[r][j]; }
            }
        }
        rank += 1;
        pivot_col += 1;
    }
    rank
}

fn cmd_derived(n: usize) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Derived Series of gl(2)\n");
    out.push_str("  g⁽⁰⁾ = g,  g⁽ᵏ⁺¹⁾ = [g⁽ᵏ⁾, g⁽ᵏ⁾]\n\n");
    let mut current: Vec<[i64;4]> = vec![
        [1,0,0,0], [0,1,0,0], [0,0,1,0], [0,0,0,1]
    ];
    out.push_str(&format!("  g⁽⁰⁾ = gl(2),  dim = {}\n", current.len()));
    out.push_str("    Basis: {E₁₁, E₁₂, E₂₁, E₂₂}\n");
    for k in 1..=n {
        let next = gl2_commutator_subspace(&current, &current);
        let dim = next.len();
        out.push_str(&format!("\n  g⁽{}⁾ = [g⁽{}⁾, g⁽{}⁾],  dim = {}\n", k, k-1, k-1, dim));
        if dim == 0 {
            out.push_str("    = {0}\n");
            out.push_str(&format!("  Derived series terminates at step {}: gl(2) is solvable!\n", k));
            break;
        }
        if dim == 4 { out.push_str("    = gl(2)  (full algebra)\n"); }
        else if dim == 3 { out.push_str("    Basis: {E₁₂, E₂₁, E₁₁-E₂₂} ≅ sl(2)\n"); }
        else if dim == 1 { out.push_str("    Basis: {E₁₂-E₂₁} or scalar — algebra is almost abelian\n"); }
        else {
            let names = ["E₁₁","E₁₂","E₂₁","E₂₂"];
            for v in &next {
                let s: String = v.iter().zip(names.iter())
                    .filter(|(&c,_)| c!=0)
                    .map(|(c,n)| format!("{}·{}", c, n)).collect::<Vec<_>>().join("+");
                out.push_str(&format!("    {}\n", s));
            }
        }
        current = next;
    }
    out.push_str("\n  gl(2)⁽¹⁾ = [gl(2),gl(2)] = sl(2) (traceless matrices, dim=3).\n");
    out.push_str("  gl(2)⁽²⁾ = [sl(2),sl(2)] = sl(2) (sl(2) is perfect: [g,g]=g).\n");
    out.push_str("  gl(2) is NOT solvable: derived series does not reach 0.\n");
    out
}

fn cmd_lower_central(n: usize) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Lower Central Series of gl(2)\n");
    out.push_str("  g₀ = g,  gₖ₊₁ = [g, gₖ]\n\n");
    let full: Vec<[i64;4]> = vec![[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]];
    let mut current = full.clone();
    out.push_str("  g₀ = gl(2), dim = 4\n");
    for k in 1..=n {
        let next = gl2_commutator_subspace(&full, &current);
        out.push_str(&format!("  g{} = [gl(2), g{}], dim = {}\n", k, k-1, next.len()));
        if next.is_empty() {
            out.push_str("  Lower central series terminates.\n");
            break;
        }
        if next.len() == current.len() && !current.is_empty() {
            out.push_str(&format!("  g{} = g{}: lower central series stabilises at dim={}\n", k, k-1, next.len()));
            break;
        }
        current = next;
    }
    out.push_str("\n  Lower central series: g=g₀⊇g₁=[g,g₀]⊇g₂=[g,g₁]⊇...\n");
    out.push_str("  g is nilpotent ⟺ lower central series reaches 0.\n");
    out.push_str("  gl(2)₁ = sl(2), gl(2)₂ = [gl(2), sl(2)] = sl(2): does not reach 0.\n");
    out.push_str("  So gl(2) is not nilpotent (it is only solvable modulo centre).\n");
    out
}

fn cmd_is_solvable(algebra_type: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  ▸ Solvability of {}\n", algebra_type));
    match algebra_type {
        "gl2" => {
            out.push_str("  gl(2) = 4-dimensional: {E₁₁, E₁₂, E₂₁, E₂₂}\n");
            out.push_str("  Derived series: gl(2) ⊃ sl(2) ⊃ sl(2) ⊃ ...\n");
            out.push_str("  gl(2) is NOT solvable: derived series stabilises at sl(2) ≠ 0.\n");
            out.push_str("  However gl(2) has the solvable radical rad(gl(2)) = ℝ·I (the centre).\n");
        }
        "sl2" => {
            out.push_str("  sl(2) = {X ∈ gl(2) | tr(X)=0}, basis {e,f,h}\n");
            out.push_str("  [sl(2), sl(2)] = sl(2)  (sl(2) is perfect)\n");
            out.push_str("  sl(2) is NOT solvable: it is simple (no proper ideals).\n");
            out.push_str("  A simple Lie algebra is semisimple but not solvable (unless dim=1).\n");
        }
        "b" => {
            out.push_str("  b = upper triangular matrices: {E₁₁, E₁₂, E₂₂}, dim=3\n");
            out.push_str("  [b, b] = {strictly upper tri} = span{E₁₂}, dim=1\n");
            out.push_str("  [{E₁₂}, {E₁₂}] = 0\n");
            out.push_str("  b (Borel subalgebra) IS solvable: b ⊃ n ⊃ 0.\n");
            out.push_str("  Borel subalgebras are the maximal solvable subalgebras.\n");
            out.push_str("  Lie's theorem applies: over ℂ, there is a common eigenvector.\n");
        }
        "n" => {
            out.push_str("  n = strictly upper triangular: {E₁₂}, dim=1\n");
            out.push_str("  [n, n] = 0  (abelian)\n");
            out.push_str("  n IS solvable (and nilpotent): it is abelian.\n");
            out.push_str("  Engel's theorem: n acts nilpotently (ad(E₁₂)² = 0 on any 2×2 matrix).\n");
        }
        _ => {
            out.push_str("  Unknown type. Try: gl2, sl2, b, n\n");
        }
    }
    out
}

fn cmd_levi() -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Levi Decomposition: g = rad(g) ⋊ s\n");
    out.push_str("  Every finite-dimensional Lie algebra decomposes as:\n");
    out.push_str("    g = rad(g) ⋊ s\n");
    out.push_str("  where rad(g) is the largest solvable ideal and s is semisimple.\n\n");
    out.push_str("  Example: gl(2) = ℝ·I ⊕ sl(2)\n");
    out.push_str("  rad(gl(2)) = ℝ·I = scalar matrices  (abelian, hence solvable)\n");
    out.push_str("  Levi factor: sl(2)  (simple, hence semisimple)\n");
    out.push_str("  Any M ∈ gl(2) = (tr(M)/2)·I + (M - tr(M)/2·I)\n");
    out.push_str("                   ↑ in rad              ↑ in sl(2)\n\n");
    out.push_str("  Example: b = n ⋊ h (Borel = unipotent ⋊ Cartan)\n");
    out.push_str("  b = upper triangular matrices = {E₁₁, E₁₂, E₂₂}\n");
    out.push_str("  rad(b) = n = span{E₁₂}  (strictly upper tri, nilpotent)\n");
    out.push_str("  Levi factor: h = span{E₁₁-E₂₂}  (Cartan subalgebra, abelian semisimple)\n\n");
    out.push_str("  General Statement (Levi, 1905):\n");
    out.push_str("  Over a field of char 0, every Lie algebra g has a Levi decomposition.\n");
    out.push_str("  The Levi factor s ≅ g/rad(g) is semisimple and unique up to conjugation.\n");
    out.push_str("  This reduces classification to: semisimple algebras + solvable algebras + extensions.\n");
    out
}

fn cmd_engel() -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Engel's Theorem\n");
    out.push_str("  Theorem (Engel, 1894): A Lie algebra g is nilpotent if and only if\n");
    out.push_str("  ad(x): g → g is nilpotent for every x ∈ g.\n\n");
    out.push_str("  Nilpotency of ad(E₁₂) on 2×2 matrices:\n");
    out.push_str("    ad(E₁₂)(E₁₁) = [E₁₂,E₁₁] = -E₁₂\n");
    out.push_str("    ad(E₁₂)(E₁₂) = [E₁₂,E₁₂] = 0\n");
    out.push_str("    ad(E₁₂)(E₂₁) = [E₁₂,E₂₁] = E₁₁-E₂₂\n");
    out.push_str("    ad(E₁₂)(E₂₂) = [E₁₂,E₂₂] = E₁₂\n\n");
    out.push_str("    ad(E₁₂)² applied to any basis element: all reach 0 in ≤2 steps.\n");
    out.push_str("    ad(E₁₂)²(E₂₁) = ad(E₁₂)(E₁₁-E₂₂) = -E₁₂+E₁₂ = 0\n");
    out.push_str("  ad(E₁₂) is nilpotent (ad(E₁₂)² = 0).\n\n");
    out.push_str("  Engel's theorem for n (strictly upper triangular):\n");
    out.push_str("  n = span{E₁₂} is 1-dimensional and abelian, so trivially nilpotent.\n");
    out.push_str("  Engel's theorem is used in the proof of Lie's theorem.\n");
    out
}

fn cmd_lie_theorem() -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Lie's Theorem\n");
    out.push_str("  Theorem (Lie): Let g be a solvable Lie algebra over ℂ, and\n");
    out.push_str("  V a finite-dimensional representation of g. Then there exists\n");
    out.push_str("  v ∈ V, v≠0, that is a simultaneous eigenvector for all x ∈ g.\n\n");
    out.push_str("  Consequence: triangularisability\n");
    out.push_str("  By induction, a solvable g-module has a basis in which all\n");
    out.push_str("  ρ(x) are upper triangular. Equivalently, ρ(g) ⊆ b for some Borel b.\n\n");
    out.push_str("  Example: b acting on ℂ²\n");
    out.push_str("    Common eigenvector: e₁ = (1,0)ᵀ\n");
    out.push_str("    E₁₁·e₁ = 1·e₁  (eigenvalue λ(E₁₁)=1)\n");
    out.push_str("    E₂₂·e₁ = 0·e₁  (eigenvalue λ(E₂₂)=0)\n");
    out.push_str("    E₁₂·e₁ = 0      (eigenvalue 0)\n");
    out.push_str("  e₁ is a common eigenvector for all of b.\n\n");
    out.push_str("  Failure over ℝ: Lie's theorem requires algebraic closure (ℂ).\n");
    out.push_str("  so(2)={[[0,-t],[t,0]]} acts on ℝ² without real eigenvectors.\n");
    out.push_str("  Weyl: every semisimple-algebra rep is completely reducible.\n");
    out.push_str("  Lie: every solvable-algebra rep can be put in upper-triangular form.\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_str(&mut s, "algebra", "b");
    state_set_int(&mut s, "series_steps", 3);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "help" | "h" => show_help(),
        "derived" => {
            let n = args.get(0).and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(state_get_int(state, "series_steps").unwrap_or(3) as usize);
            state_set_int(state, "series_steps", n as i64);
            cmd_derived(n)
        }
        "lower_central" => {
            let n = args.get(0).and_then(|s| s.parse::<usize>().ok())
                .unwrap_or(state_get_int(state, "series_steps").unwrap_or(3) as usize);
            state_set_int(state, "series_steps", n as i64);
            cmd_lower_central(n)
        }
        "is_solvable" => {
            let t: String = args.get(0).map(|s| s.to_string())
                .unwrap_or_else(|| state_get_str(state, "algebra").unwrap_or("b").to_string());
            state_set_str(state, "algebra", &t);
            cmd_is_solvable(&t)
        }
        "levi" => cmd_levi(),
        "engel" => cmd_engel(),
        "lie_theorem" => cmd_lie_theorem(),
        "demo" => {
            let mut out = cmd_is_solvable("b");
            out.push_str(&cmd_derived(3));
            out.push_str(&cmd_levi());
            out
        }
        _ => format!("Unknown command '{}'. Type 'help'.\n", cmd),
    }
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { println!("{}", out); }
    true
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    // Solvable/semisimple decomposition (Levi decomposition)
    c.text_bold(350.0, 40.0, "Levi Decomposition: g = rad(g) ⋊ s", 16.0, "#212121", "middle");
    // Big box for g
    c.rect(50.0, 80.0, 600.0, 350.0, "none", "#888888", 1.0);
    c.text(350.0, 100.0, "g  (arbitrary Lie algebra)", 13.0, "#333333", "middle");
    // Solvable radical
    c.rect(80.0, 120.0, 200.0, 270.0, "#ffe0e0", "#cc4444", 2.0);
    c.text(180.0, 250.0, "rad(g)", 14.0, "#cc4444", "middle");
    c.text(180.0, 275.0, "(solvable radical)", 11.0, "#884444", "middle");
    // Levi factor
    c.rect(320.0, 120.0, 300.0, 270.0, "#e0e8ff", "#4444cc", 2.0);
    c.text(470.0, 250.0, "s  (Levi factor)", 14.0, "#4444cc", "middle");
    c.text(470.0, 275.0, "(semisimple)", 11.0, "#444488", "middle");
    // Arrow: semidirect product
    c.arrow(280.0, 250.0, 320.0, 250.0, "#666666", 1.5);
    c.text(300.0, 240.0, "⋊", 14.0, "#333333", "middle");
    // Derived series
    c.text_bold(350.0, 460.0, "Derived series: gl(2) ⊃ sl(2) ⊃ sl(2) ⊃ ...", 13.0, "#212121", "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("g", &[("label", "g"), ("shape", "ellipse")]);
    g.node("rad", &[("label", "rad(g)\n(solvable)"), ("shape", "box")]);
    g.node("s", &[("label", "s\n(semisimple Levi)"), ("shape", "box")]);
    g.node("zero", &[("label", "0"), ("shape", "point")]);
    g.edge("g", "rad", &[("label", "ideal")]);
    g.edge("g", "s", &[("label", "complement")]);
    g.edge("rad", "zero", &[("label", "derived series")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("g", 0.0, 0.0, "g", "ellipse,draw");
    t.node("rad", -2.0, -2.0, "rad(g)", "rectangle,draw");
    t.node("s", 2.0, -2.0, "s", "rectangle,draw");
    t.arrow("g", "rad", "solvable ideal", "");
    t.arrow("g", "s", "Levi factor", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1, "Levi Decomposition: g = rad(g) x| s");
    a.box_at(2, 3, 30, 10);
    a.text_at(4, 5, "rad(g)");
    a.text_at(4, 7, "solvable radical");
    a.box_at(40, 3, 35, 10);
    a.text_at(42, 5, "s");
    a.text_at(42, 7, "semisimple (Levi)");
    a.text_at(2, 15, "Derived: gl(2) => sl(2) => sl(2) => ...");
    a.text_at(2, 17, "Nilpotent: n => n => 0 (terminates)");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch48"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 48", "Solvable & Semisimple Lie Algebras",
                         "Derived series, Levi decomposition, Engel and Lie");
            repl("ch48> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
