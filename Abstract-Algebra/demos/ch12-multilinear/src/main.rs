use common::*;

fn help_string() -> String {
    let mut h = String::new();
    h.push_str("\n  \x1b[1mCommands:\x1b[0m\n");
    h.push_str("    wedge <v1...> / <v2...>       wedge product v1∧v2 in ℝ³\n");
    h.push_str("    cross <v1...> / <v2...>       cross product and its relation to wedge\n");
    h.push_str("    det_as_top <n> <entries>      det(A) as coefficient of e1∧...∧en\n");
    h.push_str("    tensor <v1...> / <v2...>      outer product v1⊗v2 (matrix)\n");
    h.push_str("    sym <v1...> / <v2...>         symmetric product v1⊙v2\n");
    h.push_str("    alt <v1...> / <v2...>         alternating part: v1∧v2\n");
    h.push_str("    metric <g_entries> / <v...>   length of v under metric tensor g\n");
    h.push_str("    demo                          run a showcase of multilinear operations\n");
    h.push_str("    help                          show this help\n");
    h.push_str("    quit                          exit\n");
    h
}

fn parse_vec3(tokens: &[&str]) -> Option<[f64; 3]> {
    if tokens.len() < 3 { return None; }
    let a: Vec<f64> = tokens.iter().filter_map(|s| s.parse().ok()).collect();
    if a.len() < 3 { return None; }
    Some([a[0], a[1], a[2]])
}

fn parse_float_vec(tokens: &[&str]) -> Option<Vec<f64>> {
    let v: Vec<f64> = tokens.iter().filter_map(|s| s.parse().ok()).collect();
    if v.len() == tokens.len() { Some(v) } else { None }
}

fn split_slash<'a>(args: &[&'a str]) -> Option<(Vec<&'a str>, Vec<&'a str>)> {
    args.iter().position(|&s| s == "/")
        .map(|p| (args[..p].to_vec(), args[p+1..].to_vec()))
}

fn dot3(u: &[f64; 3], v: &[f64; 3]) -> f64 {
    u[0]*v[0] + u[1]*v[1] + u[2]*v[2]
}

fn fmt_vec3(label: &str, v: &[f64; 3]) -> String {
    format!("  {} = ({:.4}, {:.4}, {:.4})\n", bold(label), v[0], v[1], v[2])
}

fn wedge_components(v1: &[f64; 3], v2: &[f64; 3]) -> (f64, f64, f64) {
    let w12 = v1[0] * v2[1] - v1[1] * v2[0];
    let w13 = v1[0] * v2[2] - v1[2] * v2[0];
    let w23 = v1[1] * v2[2] - v1[2] * v2[1];
    (w12, w13, w23)
}

fn cross_product(v1: &[f64; 3], v2: &[f64; 3]) -> [f64; 3] {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}

fn cmd_wedge(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Wedge Product v1 ∧ v2 in ℝ³\x1b[0m\n");
    let (t1, t2) = match split_slash(args) {
        Some(p) => p,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: wedge <v1 x y z> / <v2 x y z>")); return out; }
    };
    let v1 = match parse_vec3(&t1) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "v1 must have 3 components.")); return out; } };
    let v2 = match parse_vec3(&t2) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "v2 must have 3 components.")); return out; } };

    out.push_str(&fmt_vec3("v1", &v1));
    out.push_str(&fmt_vec3("v2", &v2));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let (w12, w13, w23) = wedge_components(&v1, &v2);

    out.push_str(&format!("  {}v1 ∧ v2 decomposition:{}\n", bold(""), ""));
    out.push_str(&format!("    (e₁∧e₂) component: {:.4}\n", w12));
    out.push_str(&format!("    (e₁∧e₃) component: {:.4}\n", w13));
    out.push_str(&format!("    (e₂∧e₃) component: {:.4}\n", w23));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  v1 ∧ v2 = ({:.4})·e₁∧e₂  +  ({:.4})·e₁∧e₃  +  ({:.4})·e₂∧e₃\n",
        w12, w13, w23));

    let area = (w12*w12 + w13*w13 + w23*w23).sqrt();
    out.push_str(&format!("  {} {} {}\n", cyan("Area of parallelogram ‖v1∧v2‖"), dim("="), green(&format!("{:.6}", area))));

    let (w12b, w13b, w23b) = wedge_components(&v2, &v1);
    out.push_str("\n");
    out.push_str(&format!("  {}Antisymmetry:{} v2 ∧ v1 = ({:.4})e₁∧e₂ + ({:.4})e₁∧e₃ + ({:.4})e₂∧e₃\n",
        bold(""), "", w12b, w13b, w23b));
    let antisym = (w12 + w12b).abs() < 1e-10 && (w13 + w13b).abs() < 1e-10 && (w23 + w23b).abs() < 1e-10;
    if antisym { out.push_str(&format!("  {} {}\n", green("✓"), "v1∧v2 = −(v2∧v1)  ✓  (antisymmetry confirmed)")); }

    if area < 1e-10 {
        out.push_str(&format!("  {} {}\n", yellow("◆"), "v1 ∧ v2 = 0: vectors are parallel (linearly dependent)."));
    }

    out.push_str(&format!("  {} {}\n", yellow("◆"), "The wedge product is bilinear, antisymmetric, and measures 'signed area'."));
    out
}

fn cmd_cross(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Cross Product and Hodge Dual\x1b[0m\n");
    let (t1, t2) = match split_slash(args) {
        Some(p) => p,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: cross <v1 x y z> / <v2 x y z>")); return out; }
    };
    let v1 = match parse_vec3(&t1) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "v1 must have 3 components.")); return out; } };
    let v2 = match parse_vec3(&t2) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "v2 must have 3 components.")); return out; } };

    out.push_str(&fmt_vec3("v1", &v1));
    out.push_str(&fmt_vec3("v2", &v2));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let (w12, w13, w23) = wedge_components(&v1, &v2);
    let cross = cross_product(&v1, &v2);

    out.push_str(&format!("  {}Wedge product:{} v1∧v2 = {:.4}(e₁∧e₂) + {:.4}(e₁∧e₃) + {:.4}(e₂∧e₃)\n",
        bold(""), "", w12, w13, w23));
    out.push_str("\n");
    out.push_str(&format!("  {}Hodge dual ⋆:{} maps 2-forms to 1-forms in ℝ³:\n", bold(""), ""));
    out.push_str("    ⋆(e₁∧e₂) = e₃,   ⋆(e₁∧e₃) = −e₂,   ⋆(e₂∧e₃) = e₁\n");
    out.push_str("\n");
    let hodge = [w23, -w13, w12];
    out.push_str(&format!("  ⋆(v1∧v2) = {:.4}·e₁ + {:.4}·e₂ + {:.4}·e₃\n",
        hodge[0], hodge[1], hodge[2]));
    out.push_str(&fmt_vec3("  ⋆(v1∧v2)", &hodge));
    out.push_str("\n");
    out.push_str(&fmt_vec3("  v1 × v2 (cross product)", &cross));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let matches = (hodge[0] - cross[0]).abs() < 1e-10
        && (hodge[1] - cross[1]).abs() < 1e-10
        && (hodge[2] - cross[2]).abs() < 1e-10;
    if matches {
        out.push_str(&format!("  {} {}\n", green("✓"), "v1 × v2 = ⋆(v1 ∧ v2)  ✓  Cross product equals Hodge dual of wedge product!"));
    }

    let dot1 = dot3(&v1, &cross);
    let dot2 = dot3(&v2, &cross);
    out.push_str("\n");
    out.push_str(&format!("  {}Perpendicularity checks:{}\n", bold(""), ""));
    out.push_str(&format!("    ⟨v1, v1×v2⟩ = {:.2e}  {}\n", dot1, if dot1.abs() < 1e-10 { green("⊥") } else { red("not ⊥") }));
    out.push_str(&format!("    ⟨v2, v1×v2⟩ = {:.2e}  {}\n", dot2, if dot2.abs() < 1e-10 { green("⊥") } else { red("not ⊥") }));

    out.push_str(&format!("  {} {}\n", yellow("◆"), "The Hodge star ⋆ is an isomorphism Λᵏ(ℝⁿ) → Λⁿ⁻ᵏ(ℝⁿ)."));
    out
}

fn cmd_det_as_top(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Determinant as Top-Degree Form\x1b[0m\n");
    let n = match args.first().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: det_as_top <n> <entries>")); return out; }
    };
    if n == 0 || n > 4 {
        out.push_str(&format!("  {} {}\n", red("✗"), "n must be between 1 and 4."));
        return out;
    }
    let needed = 1 + n * n;
    if args.len() < needed {
        out.push_str(&format!("  {} {}\n", red("✗"), &format!("Need {} entries for {}×{} matrix.", n * n, n, n)));
        return out;
    }
    let entries: Vec<f64> = args[1..needed].iter().filter_map(|s| s.parse().ok()).collect();

    out.push_str(&format!("  {}×{} matrix A:\n", n, n));
    for r in 0..n {
        out.push_str("    │ ");
        for c in 0..n {
            out.push_str(&format!("{:8.4}", entries[r * n + c]));
            if c + 1 < n { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  {}Theory:{} det(A) is the coefficient of e₁∧...∧eₙ in\n", bold(""), ""));
    out.push_str("  the expansion of (Ae₁) ∧ (Ae₂) ∧ ... ∧ (Aeₙ).\n");
    out.push_str("\n");

    out.push_str(&format!("  {}Column vectors (images of basis vectors):{}\n", bold(""), ""));
    for j in 0..n {
        let col: Vec<f64> = (0..n).map(|i| entries[i * n + j]).collect();
        let s: Vec<String> = col.iter().map(|x| format!("{:.4}", x)).collect();
        out.push_str(&format!("    Ae_{} = [{}]\n", j + 1, s.join(", ")));
    }
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    if n == 2 {
        let a = entries[0]; let b = entries[1];
        let c = entries[2]; let d = entries[3];
        out.push_str(&format!("  (Ae₁) ∧ (Ae₂) = ({:.4}e₁ + {:.4}e₂) ∧ ({:.4}e₁ + {:.4}e₂)\n", a, c, b, d));
        out.push_str(&format!("    = ({:.4})(e₁∧e₂)\n", a*d - c*b));
        out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
        let idet = entries[0]*entries[3] - entries[1]*entries[2];
        out.push_str(&format!("  {} {} {}\n", cyan("det(A) (Leibniz formula)"), dim("="), green(&format!("{:.4}", idet))));
        out.push_str(&format!("  {} {}\n", green("✓"), &format!("Coefficient of e₁∧e₂ = {:.4} = det(A)  ✓", a*d - c*b)));
    }

    let int_entries: Vec<i64> = entries.iter().map(|&x| x.round() as i64).collect();
    if int_entries.iter().zip(entries.iter()).all(|(&i, &f)| (i as f64 - f).abs() < 0.5) {
        let mat = Mat::new(n, n, int_entries);
        out.push_str(&format!("  {} {} {}\n", cyan("det(A) (exact integer)"), dim("="), green(&format!("{}", mat.det()))));
    }

    out.push_str(&format!("  {} {}\n", yellow("◆"), "The determinant is the unique alternating n-linear function on n vectors."));
    out
}

fn cmd_tensor(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Tensor Product v1 ⊗ v2\x1b[0m\n");
    let (t1, t2) = match split_slash(args) {
        Some(p) => p,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: tensor <v1 components...> / <v2 components...>")); return out; }
    };
    let v1 = match parse_float_vec(&t1) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse v1.")); return out; } };
    let v2 = match parse_float_vec(&t2) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse v2.")); return out; } };
    let m = v1.len();
    let n = v2.len();

    let s1: Vec<String> = v1.iter().map(|x| format!("{:.3}", x)).collect();
    let s2: Vec<String> = v2.iter().map(|x| format!("{:.3}", x)).collect();
    out.push_str(&format!("  v1 = [{}]  ({}D)\n", s1.join(", "), m));
    out.push_str(&format!("  v2 = [{}]  ({}D)\n", s2.join(", "), n));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  {}v1 ⊗ v2 = outer product ({}×{} matrix):{}\n", bold(""), m, n, ""));
    out.push_str("  (v1⊗v2)_{ij} = v1_i · v2_j\n");
    out.push_str("\n");

    for i in 0..m {
        out.push_str("    │ ");
        for j in 0..n {
            out.push_str(&format!("{:8.4}", v1[i] * v2[j]));
            if j + 1 < n { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }

    out.push_str(&format!("  {} {}\n", yellow("◆"), "The outer product v1⊗v2 is a rank-1 matrix."));
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Every matrix is a sum of rank-1 tensors (SVD decomposition)."));
    out
}

fn cmd_sym(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Symmetric Product v1 ⊙ v2 = v1⊗v2 + v2⊗v1\x1b[0m\n");
    let (t1, t2) = match split_slash(args) {
        Some(p) => p,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: sym <v1 components...> / <v2 components...>")); return out; }
    };
    let v1 = match parse_float_vec(&t1) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse v1.")); return out; } };
    let v2 = match parse_float_vec(&t2) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse v2.")); return out; } };
    if v1.len() != v2.len() { out.push_str(&format!("  {} {}\n", red("✗"), "v1 and v2 must have the same length.")); return out; }
    let n = v1.len();
    let s1: Vec<String> = v1.iter().map(|x| format!("{:.3}", x)).collect();
    let s2: Vec<String> = v2.iter().map(|x| format!("{:.3}", x)).collect();
    out.push_str(&format!("  v1 = [{}]\n", s1.join(", ")));
    out.push_str(&format!("  v2 = [{}]\n", s2.join(", ")));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  {}v1 ⊙ v2 = v1⊗v2 + v2⊗v1  (symmetric under swap):{}\n", bold(""), ""));

    for i in 0..n {
        out.push_str("    │ ");
        for j in 0..n {
            let val = v1[i] * v2[j] + v2[i] * v1[j];
            out.push_str(&format!("{:8.4}", val));
            if j + 1 < n { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }

    let mut sym_ok = true;
    for i in 0..n {
        for j in i+1..n {
            let aij = v1[i]*v2[j] + v2[i]*v1[j];
            let aji = v1[j]*v2[i] + v2[j]*v1[i];
            if (aij - aji).abs() > 1e-10 { sym_ok = false; }
        }
    }
    if sym_ok { out.push_str(&format!("  {} {}\n", green("✓"), "Matrix is symmetric  ✓")); }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Sym²(V) is spanned by symmetric products; it forms a commutative algebra."));
    out
}

fn cmd_alt(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Alternating Part v1 ∧ v2 = v1⊗v2 − v2⊗v1\x1b[0m\n");
    let (t1, t2) = match split_slash(args) {
        Some(p) => p,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: alt <v1 components...> / <v2 components...>")); return out; }
    };
    let v1 = match parse_float_vec(&t1) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse v1.")); return out; } };
    let v2 = match parse_float_vec(&t2) { Some(v) => v, None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse v2.")); return out; } };
    if v1.len() != v2.len() { out.push_str(&format!("  {} {}\n", red("✗"), "v1 and v2 must have the same length.")); return out; }
    let n = v1.len();
    let s1: Vec<String> = v1.iter().map(|x| format!("{:.3}", x)).collect();
    let s2: Vec<String> = v2.iter().map(|x| format!("{:.3}", x)).collect();
    out.push_str(&format!("  v1 = [{}]\n", s1.join(", ")));
    out.push_str(&format!("  v2 = [{}]\n", s2.join(", ")));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  {}v1 ∧ v2 = v1⊗v2 − v2⊗v1  (antisymmetric under swap):{}\n", bold(""), ""));

    for i in 0..n {
        out.push_str("    │ ");
        for j in 0..n {
            let val = v1[i] * v2[j] - v2[i] * v1[j];
            out.push_str(&format!("{:8.4}", val));
            if j + 1 < n { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }

    let mut antisym_ok = true;
    for i in 0..n {
        for j in i+1..n {
            let aij = v1[i]*v2[j] - v2[i]*v1[j];
            let aji = v1[j]*v2[i] - v2[j]*v1[i];
            if (aij + aji).abs() > 1e-10 { antisym_ok = false; }
        }
    }
    if antisym_ok { out.push_str(&format!("  {} {}\n", green("✓"), "Matrix is antisymmetric  ✓")); }
    out.push_str("\n");
    out.push_str(&format!("  {}Decomposition:{} v1⊗v2 = ½(v1⊙v2) + ½(v1∧v2)\n", bold(""), ""));
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Λ²(V) (alternating 2-tensors) equals the span of wedge products."));
    out
}

fn cmd_metric(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Metric Tensor and Vector Length\x1b[0m\n");
    let (g_tokens, v_tokens) = match split_slash(args) {
        Some(p) => p,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: metric <g entries (3×3)> / <v x y z>")); return out; }
    };

    if g_tokens.len() < 9 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Need 9 entries for 3×3 metric tensor g."));
        return out;
    }
    let g_data: Vec<f64> = match parse_float_vec(&g_tokens[..9]) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse g entries.")); return out; }
    };
    let v = match parse_vec3(&v_tokens) {
        Some(x) => x,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "v must have 3 components.")); return out; }
    };

    out.push_str(&format!("  {}Metric tensor g (3×3):{}\n", bold(""), ""));
    for r in 0..3 {
        out.push_str("    │ ");
        for c in 0..3 {
            out.push_str(&format!("{:8.4}", g_data[r * 3 + c]));
            if c < 2 { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }
    out.push_str(&fmt_vec3("  v", &v));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut norm_sq = 0.0f64;
    for i in 0..3 {
        for j in 0..3 {
            norm_sq += v[i] * g_data[i * 3 + j] * v[j];
        }
    }

    out.push_str(&format!("  {} {} {}\n", cyan("‖v‖²_g"), dim("="), green(&format!("{:.6}", norm_sq))));
    if norm_sq >= 0.0 {
        out.push_str(&format!("  {} {} {}\n", cyan("‖v‖_g"), dim("="), green(&format!("{:.6}", norm_sq.sqrt()))));
    } else {
        out.push_str(&format!("  {} {}\n", yellow("◆"), &format!("‖v‖²_g = {:.4} < 0 (pseudo-Riemannian / Minkowski-type metric)", norm_sq)));
    }

    let eucl = v[0]*v[0] + v[1]*v[1] + v[2]*v[2];
    out.push_str(&format!("  {} {} {}\n", cyan("‖v‖_Euclidean"), dim("="), green(&format!("{:.6}", eucl.sqrt()))));
    out.push_str(&format!("  {} {}\n", yellow("◆"), "A Riemannian metric g is a smoothly varying positive-definite inner product."));
    out
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "wedge"       => cmd_wedge(args),
        "cross"       => cmd_cross(args),
        "det_as_top"  => {
            if let Some(n) = args.first().and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "last_n", n);
            }
            cmd_det_as_top(args)
        }
        "tensor"      => cmd_tensor(args),
        "sym"         => cmd_sym(args),
        "alt"         => cmd_alt(args),
        "metric"      => cmd_metric(args),
        "demo" => {
            let mut out = String::new();
            out.push_str("\n  === Demo: Multilinear Algebra and Tensors ===\n\n");
            let v1 = [1.0f64, 0.0, 0.0];
            let v2 = [0.0f64, 1.0, 0.0];
            let (w12, w13, w23) = wedge_components(&v1, &v2);
            out.push_str(&format!("  e1 ∧ e2 = ({:.0}, {:.0}, {:.0}) in basis (e1∧e2, e1∧e3, e2∧e3)\n", w12, w13, w23));
            let cross = cross_product(&v1, &v2);
            out.push_str(&format!("  e1 × e2 = ({:.0}, {:.0}, {:.0})  (= e3)\n", cross[0], cross[1], cross[2]));
            out.push_str("  Tensor e1⊗e2 = [[0,1,0],[0,0,0],[0,0,0]] (rank-1 matrix)\n");
            let _ = state;
            out
        }
        "help" | "h" => help_string(),
        _ => format!("  \x1b[31m✗\x1b[0m Unknown command '{}'. Type 'help'.\n", cmd),
    }
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { print!("{out}"); }
    true
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_ints(&mut s, "v1", &[1, 0, 0]);
    state_set_ints(&mut s, "v2", &[0, 1, 0]);
    s
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    c.title("Ch12: Tensor and Wedge Product Diagram");
    c.subtitle("v1 ⊗ v2 (outer product) and v1 ∧ v2 (antisymmetric part)", 42.0);

    // Draw tensor product grid
    let ox = 60.0; let oy = 80.0;
    let cell = 55.0;
    let v1 = vec!["v1₁", "v1₂", "v1₃"];
    let v2 = vec!["v2₁", "v2₂", "v2₃"];

    // Header row
    c.text_bold(ox + cell, oy, "v2₁", 12.0, colors::BLUE, "middle");
    c.text_bold(ox + 2.0*cell, oy, "v2₂", 12.0, colors::BLUE, "middle");
    c.text_bold(ox + 3.0*cell, oy, "v2₃", 12.0, colors::BLUE, "middle");

    for (i, label) in v1.iter().enumerate() {
        c.text_bold(ox, oy + (i+1) as f64 * cell, label, 12.0, colors::ORANGE, "middle");
        for j in 0..3 {
            let x = ox + (j+1) as f64 * cell;
            let y = oy + (i+1) as f64 * cell;
            let fill = if i == j { colors::ROW_ALT } else { colors::ROW_NORM };
            c.rect(x - 22.0, y - 18.0, 44.0, 36.0, fill, colors::GREY, 0.5);
            c.text(x, y, &format!("{}·{}", label, v2[j]), 10.0, colors::DARK, "middle");
        }
    }

    c.text_bold(ox + 2.0*cell, oy + 5.0*cell, "v1 ⊗ v2  (rank-1 matrix)", 12.0, colors::DARK, "middle");

    // Wedge product = antisym part
    c.text_bold(450.0, 80.0, "Wedge = antisymmetric part:", 13.0, colors::CYAN, "middle");
    c.text(450.0, 105.0, "v1∧v2 = v1⊗v2 − v2⊗v1", 12.0, colors::DARK, "middle");
    c.text(450.0, 125.0, "(skew-symmetric matrix)", 11.0, colors::GREY, "middle");

    // Sym + Alt decomposition
    c.rect(380.0, 145.0, 270.0, 60.0, colors::HEADER_FILL, colors::CYAN, 1.0);
    c.text(515.0, 165.0, "v1⊗v2 = Sym + Alt", 12.0, colors::DARK, "middle");
    c.text(515.0, 185.0, "= ½(v1⊙v2) + ½(v1∧v2)", 12.0, colors::DARK, "middle");

    // Area = |v1 ∧ v2|
    c.text_bold(450.0, 240.0, "‖v1∧v2‖ = area of parallelogram", 12.0, colors::GREEN, "middle");
    c.text(450.0, 260.0, "det(A) = coeff of e1∧...∧en", 11.0, colors::GREY, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node_default("shape", "box");
    g.node_default("style", "filled");
    g.node_default("fillcolor", "lightyellow");
    g.node("tensor", &[("label", "Tensor Product\nV⊗W")]);
    g.node("sym", &[("label", "Sym²(V)\nSymmetric"), ("fillcolor", "lightblue")]);
    g.node("alt", &[("label", "Λ²(V)\nAlternating (Wedge)"), ("fillcolor", "lightgreen")]);
    g.node("cross", &[("label", "Cross Product\n(Hodge dual)")]);
    g.node("det", &[("label", "Determinant\n(top form)")]);
    g.node("metric", &[("label", "Metric Tensor g\n‖v‖²_g = vᵀgv")]);
    g.edge("tensor", "sym", &[("label", "symmetrize")]);
    g.edge("tensor", "alt", &[("label", "antisymmetrize")]);
    g.edge("alt", "cross", &[("label", "Hodge ⋆ in ℝ³")]);
    g.edge("alt", "det", &[("label", "top degree")]);
    g.edge("tensor", "metric", &[("label", "positive definite")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.use_library("arrows,positioning");
    t.raw("  \\tikzset{tens/.style={draw,rectangle,rounded corners,minimum width=2.5cm,minimum height=0.8cm,align=center}}");
    t.node("T", 0.0, 0.0, "$V \\otimes W$", "tens");
    t.node("S", -2.0, -1.8, "$\\mathrm{Sym}^2(V)$", "tens");
    t.node("A", 2.0, -1.8, "$\\Lambda^2(V)$", "tens");
    t.node("D", 2.0, -3.6, "$\\det$ (top form)", "tens");
    t.arrow("T", "S", "sym", "blue,->");
    t.arrow("T", "A", "anti", "red,->");
    t.arrow("A", "D", "top deg", "->");
    t.raw("  \\node[above=0.5cm of T] {$v_1 \\otimes v_2 = \\tfrac{1}{2}(v_1 \\odot v_2) + \\tfrac{1}{2}(v_1 \\wedge v_2)$};");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1, "Ch12: Multilinear Algebra and Tensors");
    a.text_at(2, 2, "──────────────────────────────────────────────");
    a.text_at(2, 4, "Tensor product v1 ⊗ v2  (rank-1 matrix):");
    a.text_at(4, 5, "     v2₁  v2₂  v2₃");
    a.text_at(4, 6, "v1₁ [  *    *    *  ]");
    a.text_at(4, 7, "v1₂ [  *    *    *  ]");
    a.text_at(4, 8, "v1₃ [  *    *    *  ]");
    a.text_at(2, 10, "Decomposition:  v1⊗v2 = Sym + Alt");
    a.text_at(4, 11, "Sym = ½(v1⊗v2 + v2⊗v1)  (symmetric)");
    a.text_at(4, 12, "Alt = ½(v1⊗v2 - v2⊗v1)  (antisymmetric = wedge)");
    a.text_at(2, 14, "Wedge v1∧v2  (area of parallelogram):");
    a.text_at(4, 15, "e1∧e2 component = v1₁v2₂ - v1₂v2₁");
    a.text_at(4, 16, "e1∧e3 component = v1₁v2₃ - v1₃v2₁");
    a.text_at(4, 17, "e2∧e3 component = v1₂v2₃ - v1₃v2₂");
    a.text_at(2, 19, "Cross product = Hodge dual of wedge (in R3)");
    a.text_at(2, 20, "det(A) = coefficient of e1∧e2∧...∧en");
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
                OutputFormat::Svg => {
                    let mut c = SvgCanvas::new(700.0, 500.0);
                    visualize_svg(&mut c, cmd, &args_ref, &state);
                    c.build()
                }
                OutputFormat::Dot => {
                    let mut g = DotGraph::digraph("ch12");
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
            print_banner("Chapter 12", "Multilinear Algebra and Tensors",
                "Wedge products · Exterior algebra · Tensor products · Metrics");
            print_info("Explore the algebra of multilinear maps, wedge products, and tensors.");
            print!("{}", help_string());
            repl("ch12> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
