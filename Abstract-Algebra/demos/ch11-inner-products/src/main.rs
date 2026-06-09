use common::*;

fn help_string() -> String {
    let mut h = String::new();
    h.push_str("\n  \x1b[1mCommands:\x1b[0m\n");
    h.push_str("    inner <v1...> / <v2...>          inner product, length, angle between vectors\n");
    h.push_str("    gram_schmidt <v1> | <v2> | ...   Gram-Schmidt orthogonalization\n");
    h.push_str("    project <v...> / <onto...>       orthogonal projection\n");
    h.push_str("    qr <n> <entries>                 QR decomposition via Gram-Schmidt\n");
    h.push_str("    least_squares <A...> / <b...>    solve Ax≈b via normal equations\n");
    h.push_str("    spectral <n> <entries>           eigenvalues of symmetric matrix\n");
    h.push_str("    demo                             run a showcase\n");
    h.push_str("    help                             show this help\n");
    h.push_str("    quit                             exit\n");
    h
}

fn dot(u: &[f64], v: &[f64]) -> f64 {
    u.iter().zip(v.iter()).map(|(a, b)| a * b).sum()
}

fn norm(v: &[f64]) -> f64 {
    dot(v, v).sqrt()
}

fn normalize(v: &[f64]) -> Vec<f64> {
    let n = norm(v);
    if n < 1e-14 { return vec![0.0; v.len()]; }
    v.iter().map(|x| x / n).collect()
}

fn vec_sub(u: &[f64], v: &[f64]) -> Vec<f64> {
    u.iter().zip(v.iter()).map(|(a, b)| a - b).collect()
}

fn vec_scale(v: &[f64], s: f64) -> Vec<f64> {
    v.iter().map(|x| x * s).collect()
}

fn fmt_vec(label: &str, v: &[f64]) -> String {
    let s: Vec<String> = v.iter().map(|x| format!("{:.4}", x)).collect();
    format!("  {} = [{}]\n", bold(label), s.join(", "))
}

fn parse_float_vec(tokens: &[&str]) -> Option<Vec<f64>> {
    let v: Vec<f64> = tokens.iter()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
    if v.len() != tokens.len() { None } else { Some(v) }
}

/// Split args on "/" delimiter into two groups
fn split_on_slash<'a>(args: &[&'a str]) -> Option<(Vec<&'a str>, Vec<&'a str>)> {
    if let Some(pos) = args.iter().position(|&s| s == "/") {
        Some((args[..pos].to_vec(), args[pos+1..].to_vec()))
    } else {
        None
    }
}

/// Split args on "|" delimiter into multiple groups
fn split_on_pipe<'a>(args: &[&'a str]) -> Vec<Vec<&'a str>> {
    let mut groups: Vec<Vec<&str>> = vec![vec![]];
    for &a in args {
        if a == "|" {
            groups.push(vec![]);
        } else {
            groups.last_mut().unwrap().push(a);
        }
    }
    groups.into_iter().filter(|g| !g.is_empty()).collect()
}

fn cmd_inner(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Inner Product\x1b[0m\n"));
    let (v1_tokens, v2_tokens) = match split_on_slash(args) {
        Some(p) => p,
        None => {
            out.push_str(&format!("  {} {}\n", red("✗"), "Usage: inner <v1 components...> / <v2 components...>"));
            return out;
        }
    };
    let v1 = match parse_float_vec(&v1_tokens) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse v1 as floats.")); return out; }
    };
    let v2 = match parse_float_vec(&v2_tokens) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse v2 as floats.")); return out; }
    };
    if v1.len() != v2.len() {
        out.push_str(&format!("  {} {}\n", red("✗"), "Vectors must have the same dimension."));
        return out;
    }

    out.push_str(&fmt_vec("v1", &v1));
    out.push_str(&fmt_vec("v2", &v2));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let ip = dot(&v1, &v2);
    let len1 = norm(&v1);
    let len2 = norm(&v2);

    out.push_str(&format!("  {} {} {}\n", cyan("⟨v1, v2⟩"), dim("="), green(&format!("{:.6}", ip))));
    out.push_str(&format!("  {} {} {}\n", cyan("‖v1‖"), dim("="), green(&format!("{:.6}", len1))));
    out.push_str(&format!("  {} {} {}\n", cyan("‖v2‖"), dim("="), green(&format!("{:.6}", len2))));

    if len1 > 1e-14 && len2 > 1e-14 {
        let cos_theta = (ip / (len1 * len2)).clamp(-1.0, 1.0);
        let angle_rad = cos_theta.acos();
        let angle_deg = angle_rad.to_degrees();
        out.push_str(&format!("  {} {} {}\n", cyan("cos θ"), dim("="), green(&format!("{:.6}", cos_theta))));
        out.push_str(&format!("  {} {} {}\n", cyan("θ (degrees)"), dim("="), green(&format!("{:.4}°", angle_deg))));

        if ip.abs() < 1e-10 {
            out.push_str(&format!("  {} {}\n", green("✓"), "Vectors are orthogonal!"));
        }
    }

    out.push_str("\n");
    out.push_str(&format!("  {}Cauchy-Schwarz:{} |⟨v1,v2⟩| ≤ ‖v1‖·‖v2‖\n", bold(""), ""));
    out.push_str(&format!("    |{:.6}| ≤ {:.6}·{:.6} = {:.6}  {}\n",
        ip, len1, len2, len1 * len2,
        if ip.abs() <= len1 * len2 + 1e-10 { green("✓") } else { red("✗") }));

    out.push_str(&format!("  {} {}\n", yellow("◆"), "The inner product encodes geometric information: length and angle."));
    out
}

fn cmd_gram_schmidt(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Gram-Schmidt Orthogonalization\x1b[0m\n"));
    let groups = split_on_pipe(args);
    if groups.len() < 2 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Usage: gram_schmidt <v1...> | <v2...> | <v3...> ..."));
        out.push_str(&format!("  {}  Separate vectors with '|'. Example: gram_schmidt 1 1 0 | 1 0 1 | 0 1 1\n", dim("")));
        return out;
    }

    let vectors: Vec<Vec<f64>> = match groups.iter().map(|g| parse_float_vec(g)).collect::<Option<Vec<_>>>() {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse all vector components as floats.")); return out; }
    };

    let d = vectors[0].len();
    if !vectors.iter().all(|v| v.len() == d) {
        out.push_str(&format!("  {} {}\n", red("✗"), "All vectors must have the same dimension."));
        return out;
    }

    out.push_str(&format!("  Input vectors ({}D):\n", d));
    for (i, v) in vectors.iter().enumerate() {
        out.push_str(&fmt_vec(&format!("v{}", i + 1), v));
    }
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut orthonormal: Vec<Vec<f64>> = Vec::new();

    for (i, v) in vectors.iter().enumerate() {
        out.push_str(&format!("  {}Step {}:{} Processing v{}:\n", bold(""), i + 1, "", i + 1));
        let mut u = v.clone();
        for (j, q) in orthonormal.iter().enumerate() {
            let proj_coeff = dot(v, q);
            let proj = vec_scale(q, proj_coeff);
            out.push_str(&format!("    proj_e{} v{} = ⟨v{}, e{}⟩·e{} = {:.4}·e{}\n",
                j + 1, i + 1, i + 1, j + 1, j + 1, proj_coeff, j + 1));
            u = vec_sub(&u, &proj);
        }
        let u_norm = norm(&u);
        if u_norm < 1e-10 {
            out.push_str(&format!("  {} v{} is linearly dependent on previous vectors (skipping).\n", red("✗"), i + 1));
            continue;
        }
        let e = normalize(&u);
        out.push_str(&fmt_vec(&format!("  u{} (orthogonal)", i + 1), &u));
        out.push_str(&fmt_vec(&format!("  e{} (orthonormal, ‖e{}‖={:.4})", i + 1, i + 1, u_norm), &e));
        orthonormal.push(e);
    }

    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  {}Orthonormal basis:{}\n", bold(""), ""));
    for (i, e) in orthonormal.iter().enumerate() {
        out.push_str(&fmt_vec(&format!("  e{}", i + 1), e));
    }

    out.push_str("\n");
    out.push_str(&format!("  {}Orthogonality check:{}\n", bold(""), ""));
    let mut all_ok = true;
    for i in 0..orthonormal.len() {
        for j in i + 1..orthonormal.len() {
            let ip = dot(&orthonormal[i], &orthonormal[j]);
            let ok = ip.abs() < 1e-10;
            if !ok { all_ok = false; }
            out.push_str(&format!("    ⟨e{}, e{}⟩ = {:.2e}  {}\n", i + 1, j + 1, ip,
                if ok { green("⊥") } else { red("not ⊥") }));
        }
    }
    if all_ok { out.push_str(&format!("  {} {}\n", green("✓"), "All pairs orthogonal.")); }

    out.push_str(&format!("  {} {}\n", yellow("◆"), "Gram-Schmidt produces an orthonormal basis for span(v1,...,vk)."));
    out
}

fn cmd_project(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Orthogonal Projection\x1b[0m\n"));
    let (v_tokens, onto_tokens) = match split_on_slash(args) {
        Some(p) => p,
        None => {
            out.push_str(&format!("  {} {}\n", red("✗"), "Usage: project <v...> / <onto_v...>"));
            return out;
        }
    };
    let v = match parse_float_vec(&v_tokens) {
        Some(x) => x,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse v.")); return out; }
    };
    let onto = match parse_float_vec(&onto_tokens) {
        Some(x) => x,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse onto_v.")); return out; }
    };
    if v.len() != onto.len() {
        out.push_str(&format!("  {} {}\n", red("✗"), "v and onto_v must have the same dimension."));
        return out;
    }

    out.push_str(&fmt_vec("v", &v));
    out.push_str(&fmt_vec("onto", &onto));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let onto_norm2 = dot(&onto, &onto);
    if onto_norm2 < 1e-14 {
        out.push_str(&format!("  {} {}\n", red("✗"), "onto_v is the zero vector; projection undefined."));
        return out;
    }

    let coeff = dot(&v, &onto) / onto_norm2;
    let proj = vec_scale(&onto, coeff);
    let perp = vec_sub(&v, &proj);

    out.push_str("  Formula: proj = (⟨v, u⟩ / ⟨u, u⟩) · u\n");
    out.push_str(&format!("           coeff = {:.6} / {:.6} = {:.6}\n", dot(&v, &onto), onto_norm2, coeff));
    out.push_str("\n");
    out.push_str(&fmt_vec("proj_onto(v)", &proj));
    out.push_str(&fmt_vec("v - proj (perpendicular component)", &perp));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let check = dot(&perp, &onto);
    out.push_str(&format!("  Orthogonality check: ⟨perp, onto⟩ = {:.2e}  {}\n",
        check,
        if check.abs() < 1e-10 { green("✓") } else { red("✗") }));

    out.push_str("\n");
    out.push_str(&format!("  {}Decomposition:{} v = proj + perp\n", bold(""), ""));
    out.push_str(&format!("    v = {:.4}·onto + perp\n", coeff));

    out.push_str(&format!("  {} {}\n", yellow("◆"), "Orthogonal projection minimizes ‖v - c·u‖ over all scalars c."));
    out
}

fn cmd_qr(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ QR Decomposition\x1b[0m\n"));
    let n = match args.first().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: qr <n> <entries>")); return out; }
    };
    if n == 0 || n > 6 {
        out.push_str(&format!("  {} {}\n", red("✗"), "n must be between 1 and 6."));
        return out;
    }
    let needed = 1 + n * n;
    if args.len() < needed {
        out.push_str(&format!("  {} {}\n", red("✗"), &format!("Need {} entries for {}×{} matrix.", n*n, n, n)));
        return out;
    }
    let entries: Vec<f64> = args[1..needed].iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    if entries.len() != n * n {
        out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse all entries as floats."));
        return out;
    }

    out.push_str(&format!("  A ({}×{}):\n", n, n));
    for r in 0..n {
        out.push_str("    │ ");
        for c in 0..n {
            out.push_str(&format!("{:8.4}", entries[r * n + c]));
            if c + 1 < n { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let cols: Vec<Vec<f64>> = (0..n)
        .map(|c| (0..n).map(|r| entries[r * n + c]).collect())
        .collect();

    let mut q_cols: Vec<Vec<f64>> = Vec::new();
    let mut r_mat = vec![0.0f64; n * n];

    for j in 0..n {
        let mut u = cols[j].clone();
        for i in 0..q_cols.len() {
            let coeff = dot(&cols[j], &q_cols[i]);
            r_mat[i * n + j] = coeff;
            let sub = vec_scale(&q_cols[i], coeff);
            u = vec_sub(&u, &sub);
        }
        let norm_u = norm(&u);
        r_mat[j * n + j] = norm_u;
        if norm_u > 1e-12 {
            q_cols.push(normalize(&u));
        } else {
            out.push_str(&format!("  {} {}\n", red("✗"), &format!("Column {} is linearly dependent (degenerate QR).", j + 1)));
            q_cols.push(vec![0.0; n]);
        }
    }

    out.push_str(&format!("  {}Q (orthogonal matrix):{}\n", bold(""), ""));
    for r in 0..n {
        out.push_str("    │ ");
        for c in 0..q_cols.len() {
            out.push_str(&format!("{:8.4}", q_cols[c][r]));
            if c + 1 < n { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }

    out.push_str("\n");
    out.push_str(&format!("  {}R (upper triangular):{}\n", bold(""), ""));
    for r in 0..n {
        out.push_str("    │ ");
        for c in 0..n {
            out.push_str(&format!("{:8.4}", r_mat[r * n + c]));
            if c + 1 < n { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }

    out.push_str(&format!("  {} {}\n", yellow("◆"), "Q has orthonormal columns; R is upper triangular with nonneg diagonal."));
    out
}

fn cmd_least_squares(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Least Squares: Ax ≈ b\x1b[0m\n"));
    let (a_tokens, b_tokens) = match split_on_slash(args) {
        Some(p) => p,
        None => {
            out.push_str(&format!("  {} {}\n", red("✗"), "Usage: least_squares <A entries...> / <b entries...>"));
            return out;
        }
    };

    let b: Vec<f64> = match parse_float_vec(&b_tokens) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse b as floats.")); return out; }
    };
    let m = b.len();

    let a_data: Vec<f64> = match parse_float_vec(&a_tokens) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse A entries as floats.")); return out; }
    };

    if a_data.len() % m != 0 {
        out.push_str(&format!("  {} {}\n", red("✗"), &format!("A has {} entries which is not divisible by m={}.", a_data.len(), m)));
        return out;
    }
    let n = a_data.len() / m;

    out.push_str(&format!("  System: A is {}×{}, b is {}×1\n", m, n, m));
    out.push_str("  Normal equations: AᵀA x = Aᵀb\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut ata = vec![0.0f64; n * n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..m {
                ata[i * n + j] += a_data[k * n + i] * a_data[k * n + j];
            }
        }
    }
    let mut atb = vec![0.0f64; n];
    for i in 0..n {
        for k in 0..m {
            atb[i] += a_data[k * n + i] * b[k];
        }
    }

    out.push_str(&format!("  {}AᵀA ={}\n", bold(""), ""));
    for r in 0..n {
        out.push_str("    │ ");
        for c in 0..n {
            out.push_str(&format!("{:8.3}", ata[r * n + c]));
            if c + 1 < n { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }
    out.push_str(&fmt_vec("  Aᵀb", &atb));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    if n > 4 {
        out.push_str(&format!("  {} {}\n", red("✗"), "n > 4 not supported for Gaussian solve in this demo."));
        return out;
    }

    let x = gaussian_solve(&ata, &atb, n);
    match x {
        Some(x_sol) => {
            out.push_str(&fmt_vec("  Least-squares solution x̂", &x_sol));
            let mut resid = b.clone();
            for i in 0..m {
                for j in 0..n {
                    resid[i] -= a_data[i * n + j] * x_sol[j];
                }
            }
            let resid_norm = norm(&resid);
            out.push_str(&format!("  {} {} {}\n", cyan("  ‖Ax̂ − b‖ (residual)"), dim("="), green(&format!("{:.6}", resid_norm))));
        }
        None => {
            out.push_str(&format!("  {} {}\n", red("✗"), "AᵀA is singular; no unique least-squares solution."));
        }
    }

    out.push_str(&format!("  {} {}\n", yellow("◆"), "Least squares minimizes ‖Ax - b‖². The solution satisfies the normal equations."));
    out
}

fn gaussian_solve(a: &[f64], b: &[f64], n: usize) -> Option<Vec<f64>> {
    let mut aug: Vec<f64> = Vec::with_capacity(n * (n + 1));
    for i in 0..n {
        for j in 0..n { aug.push(a[i * n + j]); }
        aug.push(b[i]);
    }
    let nc = n + 1;
    for col in 0..n {
        let pivot = (col..n).max_by(|&i, &j| {
            aug[i * nc + col].abs().partial_cmp(&aug[j * nc + col].abs()).unwrap()
        })?;
        if aug[pivot * nc + col].abs() < 1e-12 { return None; }
        aug.swap(col * nc, pivot * nc);
        for k in 0..nc {
            let tmp = aug[col * nc + k];
            aug[col * nc + k] = aug[pivot * nc + k];
            aug[pivot * nc + k] = tmp;
        }
        let diag = aug[col * nc + col];
        for row in col + 1..n {
            let factor = aug[row * nc + col] / diag;
            for k in col..nc {
                let sub = factor * aug[col * nc + k];
                aug[row * nc + k] -= sub;
            }
        }
    }
    let mut x = vec![0.0f64; n];
    for i in (0..n).rev() {
        let mut s = aug[i * nc + n];
        for j in i + 1..n { s -= aug[i * nc + j] * x[j]; }
        x[i] = s / aug[i * nc + i];
    }
    Some(x)
}

fn cmd_spectral(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Spectral Theorem (Symmetric Matrix)\x1b[0m\n"));
    let n = match args.first().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: spectral <n> <entries>")); return out; }
    };
    if n == 0 || n > 5 {
        out.push_str(&format!("  {} {}\n", red("✗"), "n must be between 1 and 5."));
        return out;
    }
    let needed = 1 + n * n;
    if args.len() < needed {
        out.push_str(&format!("  {} {}\n", red("✗"), &format!("Need {} entries for {}×{} matrix.", n * n, n, n)));
        return out;
    }
    let entries: Vec<f64> = args[1..needed].iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    if entries.len() != n * n {
        out.push_str(&format!("  {} {}\n", red("✗"), "Could not parse all entries as floats."));
        return out;
    }

    let mut is_sym = true;
    for i in 0..n {
        for j in 0..n {
            if (entries[i * n + j] - entries[j * n + i]).abs() > 1e-10 {
                is_sym = false;
            }
        }
    }
    if !is_sym {
        out.push_str(&format!("  {} {}\n", red("✗"), "Matrix is not symmetric. Spectral theorem requires symmetry."));
        return out;
    }

    out.push_str("  Matrix is symmetric ✓\n");
    out.push_str(&format!("  {}Spectral Theorem:{} A = QΛQᵀ for symmetric A,\n", bold(""), ""));
    out.push_str("  where Q is orthogonal and Λ is diagonal.\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let fmat = FMat::new(n, n, entries.clone());
    let (lambda1, v1) = fmat.power_iteration(200);

    out.push_str(&format!("  {} {} {}\n", cyan("  Largest eigenvalue (power iteration)"), dim("="), green(&format!("{:.6}", lambda1))));
    out.push_str(&fmt_vec("  Corresponding eigenvector", &v1));

    if n == 2 {
        let tr = entries[0] + entries[3];
        let det = entries[0] * entries[3] - entries[1] * entries[2];
        let disc = tr * tr - 4.0 * det;
        let lam1 = (tr + disc.sqrt()) / 2.0;
        let lam2 = (tr - disc.sqrt()) / 2.0;
        out.push_str("\n");
        out.push_str(&format!("  {} {} {}\n", cyan("  λ₁"), dim("="), green(&format!("{:.6}", lam1))));
        out.push_str(&format!("  {} {} {}\n", cyan("  λ₂"), dim("="), green(&format!("{:.6}", lam2))));
        out.push_str("\n");
        out.push_str("  Orthogonal diagonalization: A = Q [λ₁  0 ] Qᵀ\n");
        out.push_str("                                    [ 0  λ₂]\n");
        out.push_str(&format!("  {} {}\n", yellow("◆"), &format!("Tr(A) = {} = λ₁+λ₂ = {:.4}+{:.4} = {:.4}",
            tr, lam1, lam2, lam1 + lam2)));
        out.push_str(&format!("  {} {}\n", yellow("◆"), &format!("Det(A) = {:.4} = λ₁·λ₂ = {:.4}·{:.4} = {:.4}",
            det, lam1, lam2, lam1 * lam2)));
    }

    out.push_str(&format!("  {} {}\n", yellow("◆"), "Real symmetric matrices always have real eigenvalues."));
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Eigenvectors for distinct eigenvalues are automatically orthogonal."));
    out
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "inner" => {
            let out = cmd_inner(args);
            // Save last used vectors to state if parseable
            if let Some((v1_tokens, v2_tokens)) = split_on_slash(args) {
                if let Some(v1) = parse_float_vec(&v1_tokens) {
                    let v1i: Vec<i64> = v1.iter().map(|&x| x.round() as i64).collect();
                    state_set_ints(state, "v1", &v1i);
                }
                if let Some(v2) = parse_float_vec(&v2_tokens) {
                    let v2i: Vec<i64> = v2.iter().map(|&x| x.round() as i64).collect();
                    state_set_ints(state, "v2", &v2i);
                }
            }
            out
        }
        "gram_schmidt" => cmd_gram_schmidt(args),
        "project"      => cmd_project(args),
        "qr"           => {
            if let Some(n) = args.first().and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "last_n", n);
            }
            cmd_qr(args)
        }
        "least_squares" => cmd_least_squares(args),
        "spectral"      => {
            if let Some(n) = args.first().and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "last_n", n);
            }
            cmd_spectral(args)
        }
        "demo" => {
            let mut out = String::new();
            out.push_str("\n  === Demo: Inner Product Spaces ===\n\n");
            out.push_str("  [inner] 1 0 0 / 0 1 0:\n");
            let v1 = vec![1.0f64, 0.0, 0.0];
            let v2 = vec![0.0f64, 1.0, 0.0];
            out.push_str(&format!("    <v1,v2> = {:.4}  (orthogonal standard basis vectors)\n", dot(&v1, &v2)));
            out.push_str("  [gram_schmidt] 1 1 0 | 1 0 1 | 0 1 1:\n");
            let vecs: Vec<Vec<f64>> = vec![vec![1.0,1.0,0.0], vec![1.0,0.0,1.0], vec![0.0,1.0,1.0]];
            let e1 = normalize(&vecs[0]);
            out.push_str(&format!("    e1 = [{:.4}, {:.4}, {:.4}]\n", e1[0], e1[1], e1[2]));
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
    // Default demo vectors: orthogonal basis vectors
    state_set_ints(&mut s, "v1", &[1, 0, 0]);
    state_set_ints(&mut s, "v2", &[0, 1, 0]);
    s
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    c.title("Ch11: Gram-Schmidt Orthogonalization");
    c.subtitle("Projecting vectors onto orthogonal complement", 42.0);

    // Draw two input vectors as arrows from origin
    let ox = 120.0; let oy = 380.0;

    // v1 vector
    c.arrow(ox, oy, ox + 160.0, oy - 80.0, colors::BLUE, 2.0);
    c.text_bold(ox + 165.0, oy - 85.0, "v1", 13.0, colors::BLUE, "start");

    // v2 vector
    c.arrow(ox, oy, ox + 80.0, oy - 200.0, colors::ORANGE, 2.0);
    c.text_bold(ox + 85.0, oy - 205.0, "v2", 13.0, colors::ORANGE, "start");

    // e1 = normalize(v1)
    c.arrow(ox, oy, ox + 120.0, oy - 60.0, colors::GREEN, 2.5);
    c.text_bold(ox + 125.0, oy - 65.0, "e1", 13.0, colors::GREEN, "start");

    // projection of v2 onto e1
    c.dashed_line(ox + 80.0, oy - 200.0, ox + 90.0, oy - 45.0, colors::GREY, 1.0, "4,4");
    c.text(ox + 60.0, oy - 120.0, "proj", 11.0, colors::GREY, "end");

    // e2 = v2 - proj
    c.arrow(ox + 90.0, oy - 45.0, ox + 80.0, oy - 200.0, colors::MAGENTA, 2.5);
    c.text_bold(ox + 95.0, oy - 130.0, "u2", 13.0, colors::MAGENTA, "start");

    // Right angle mark
    c.rect(ox + 87.0, oy - 52.0, 8.0, 8.0, colors::NONE, colors::GREY, 1.0);

    // Labels and notes
    c.text(350.0, 120.0, "Gram-Schmidt:", 14.0, colors::DARK, "start");
    c.text(350.0, 145.0, "e1 = v1 / |v1|", 12.0, colors::GREEN, "start");
    c.text(350.0, 165.0, "u2 = v2 - <v2,e1>*e1", 12.0, colors::MAGENTA, "start");
    c.text(350.0, 185.0, "e2 = u2 / |u2|", 12.0, colors::CYAN, "start");
    c.text(350.0, 215.0, "<e1,e2> = 0  (orthogonal)", 12.0, colors::GREY, "start");

    // Cauchy-Schwarz
    c.rect(330.0, 250.0, 300.0, 50.0, colors::ROW_ALT, colors::GREEN, 1.0);
    c.text(480.0, 275.0, "|<v1,v2>| <= |v1| * |v2|", 12.0, colors::DARK, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node_default("shape", "ellipse");
    g.node_default("style", "filled");
    g.node_default("fillcolor", "lightcyan");
    g.node("V", &[("label", "V (inner product space)")]);
    g.node("ON", &[("label", "Orthonormal Basis\n{e1,...,ek}"), ("fillcolor", "lightgreen")]);
    g.node("proj", &[("label", "Projection\nonto subspace")]);
    g.node("qr", &[("label", "QR Decomposition\nA = QR")]);
    g.node("ls", &[("label", "Least Squares\nmin|Ax-b|")]);
    g.edge("V", "ON", &[("label", "Gram-Schmidt")]);
    g.edge("ON", "proj", &[("label", "proj_W(v)")]);
    g.edge("ON", "qr", &[("label", "columns")]);
    g.edge("qr", "ls", &[("label", "normal eqs")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.use_library("arrows,positioning");
    t.raw("  \\tikzset{vec/.style={->,>=stealth,thick}}");
    t.raw("  % Gram-Schmidt diagram");
    t.node("O", 0.0, 0.0, "$O$", "circle,draw,minimum size=4pt,inner sep=0pt");
    t.node("V1", 2.5, 1.0, "$\\mathbf{v}_1$", "above right");
    t.node("V2", 1.2, 2.8, "$\\mathbf{v}_2$", "above left");
    t.node("E1", 2.0, 0.8, "$\\mathbf{e}_1$", "below right");
    t.node("U2", 0.5, 2.0, "$\\mathbf{u}_2$", "left");
    t.raw("  \\draw[vec,blue] (O) -- (V1);");
    t.raw("  \\draw[vec,orange] (O) -- (V2);");
    t.raw("  \\draw[vec,green!70!black,very thick] (O) -- (E1);");
    t.raw("  \\draw[vec,purple,very thick] (O) -- (U2);");
    t.raw("  \\node[below=1.5cm of O] {$\\langle \\mathbf{e}_1,\\mathbf{e}_2\\rangle = 0$};");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1, "Ch11: Gram-Schmidt Orthogonalization");
    a.text_at(2, 2, "──────────────────────────────────────────────");
    a.text_at(2, 4, "Input vectors v1, v2:");
    a.text_at(4, 5, "v1 = [1.0, 1.0, 0.0]");
    a.text_at(4, 6, "v2 = [1.0, 0.0, 1.0]");
    a.text_at(2, 8, "Step 1: e1 = v1 / |v1|");
    a.text_at(4, 9, "e1 = [0.7071, 0.7071, 0.0000]");
    a.text_at(2, 11, "Step 2: u2 = v2 - <v2,e1>*e1");
    a.text_at(4, 12, "<v2,e1> = 0.7071");
    a.text_at(4, 13, "u2 = v2 - 0.7071*e1 = [0.5, -0.5, 1.0]");
    a.text_at(2, 15, "Step 3: e2 = u2 / |u2|");
    a.text_at(4, 16, "e2 = [0.4082, -0.4082, 0.8165]");
    a.text_at(2, 18, "Orthogonality: <e1,e2> ≈ 0.000  OK");
    a.text_at(2, 20, "QR: A = Q * R  (Q orthogonal, R upper triangular)");
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
                    let mut g = DotGraph::digraph("ch11");
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
            print_banner("Chapter 11", "Inner Product Spaces",
                "Dot products · Gram-Schmidt · QR · Least squares · Spectral theorem");
            print_info("Explore geometry through inner products and orthogonality.");
            print!("{}", help_string());
            repl("ch11> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
