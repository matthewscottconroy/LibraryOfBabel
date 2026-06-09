use common::*;

fn show_help() -> String {
    let mut out = String::new();
    let rows = &[
        ("irreps <type> <n>",            "list irreps: Z=ℤ/nℤ, D=Dihedral, S=Symmetric"),
        ("character <type> <n>",         "character table (traces)"),
        ("schur <n>",                    "Schur's lemma for ℤ/nℤ"),
        ("decompose <type> <n> <vals>",  "decompose a character into irreducibles"),
        ("regular <n>",                  "regular representation of ℤ/nℤ"),
        ("direct_sum <vals> / <vals>",   "direct sum of two characters"),
        ("tensor_rep <n> <m>",           "tensor product ρ_n ⊗ ρ_m of irreps of ℤ/NℤZ (ask N first)"),
        ("demo",                         "showcase of representation theory"),
        ("help",                         "show this help"),
        ("quit",                         "exit"),
    ];
    let max = rows.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Commands\x1b[0m\n"));
    for (cmd, desc) in rows {
        let width = max + 2 + 9;
        out.push_str(&format!("  \x1b[36m{:<width$}\x1b[0m {}\n", cmd, desc, width = width));
    }
    out.push('\n');
    out
}

// ── Local string-accumulating helpers ────────────────────────────────────────

fn s_section(out: &mut String, title: &str) {
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ {}\x1b[0m\n", title));
}
fn s_result(out: &mut String, label: &str, value: &str) {
    out.push_str(&format!("  \x1b[36m{}\x1b[0m \x1b[2m=\x1b[0m \x1b[32m{}\x1b[0m\n", label, value));
}
fn s_note(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[33m◆\x1b[0m {}\n", text));
}
fn s_err(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[31m✗\x1b[0m {}\n", text));
}

// ── Complex number helpers ────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
struct C {
    re: f64,
    im: f64,
}

impl C {
    fn new(re: f64, im: f64) -> Self { C { re, im } }
    fn from_polar(r: f64, theta: f64) -> Self { C { re: r * theta.cos(), im: r * theta.sin() } }
    fn conj(self) -> Self { C { re: self.re, im: -self.im } }
    fn mul(self, other: C) -> Self { C { re: self.re*other.re - self.im*other.im, im: self.re*other.im + self.im*other.re } }
    fn add(self, other: C) -> Self { C { re: self.re + other.re, im: self.im + other.im } }
    fn abs2(self) -> f64 { self.re*self.re + self.im*self.im }
}

fn fmt_c(c: C) -> String {
    let re = (c.re * 1000.0).round() / 1000.0;
    let im = (c.im * 1000.0).round() / 1000.0;
    if im.abs() < 1e-9 {
        format!("{:>7.3}", re)
    } else if re.abs() < 1e-9 {
        format!("{:>7.3}i", im)
    } else if im < 0.0 {
        format!("{:.3}{:.3}i", re, im)
    } else {
        format!("{:.3}+{:.3}i", re, im)
    }
}

const PI: f64 = std::f64::consts::PI;

// ── Irreps of ℤ/nℤ ───────────────────────────────────────────────────────────

fn zn_irrep_char(n: usize, k: usize, j: usize) -> C {
    C::from_polar(1.0, 2.0 * PI * (k * j) as f64 / n as f64)
}

// ── Irreps of D_n (dihedral group of order 2n) ────────────────────────────────

fn dn_char_table(n: usize) -> (Vec<String>, Vec<Vec<f64>>) {
    let mut classes: Vec<String> = vec!["e".to_string()];
    let mut class_sizes: Vec<usize> = vec![1];
    let mut chars: Vec<Vec<f64>> = Vec::new();

    if n % 2 == 1 {
        for j in 1..=(n-1)/2 {
            classes.push(format!("r^{}", j));
            class_sizes.push(2);
        }
        classes.push("s".to_string());
        class_sizes.push(n);

        let num_classes = classes.len();
        let _ = (num_classes, class_sizes);

        let triv: Vec<f64> = vec![1.0; classes.len()];
        chars.push(triv);
        let mut sign = vec![1.0; classes.len()];
        *sign.last_mut().unwrap() = -1.0;
        chars.push(sign);
        for k in 1..=(n-1)/2 {
            let mut row = Vec::new();
            row.push(2.0);
            for j in 1..=(n-1)/2 {
                row.push(2.0 * (2.0 * PI * (j*k) as f64 / n as f64).cos());
            }
            row.push(0.0);
            chars.push(row);
        }
    } else {
        for j in 1..n/2 {
            classes.push(format!("r^{}", j));
            class_sizes.push(2);
        }
        classes.push(format!("r^{}", n/2));
        class_sizes.push(1);
        classes.push("s".to_string());
        class_sizes.push(n/2);
        classes.push("rs".to_string());
        class_sizes.push(n/2);

        let nc = classes.len();
        let t1 = vec![1.0f64; nc]; chars.push(t1.clone());
        let mut t2 = vec![1.0f64; nc];
        t2[1 + (n/2 - 1)] = -1.0;
        let ns = 1 + (n/2 - 1) + 1;
        t2[ns] = 1.0;
        t2[ns+1] = -1.0;
        chars.push(t2);

        let mut t3 = vec![1.0f64; nc];
        t3[1 + (n/2-1)] = 1.0;
        t3[ns] = -1.0;
        t3[ns+1] = 1.0;
        chars.push(t3);

        let mut t4 = vec![1.0f64; nc];
        t4[1 + (n/2-1)] = -1.0;
        t4[ns] = -1.0;
        t4[ns+1] = -1.0;
        chars.push(t4);

        let t1_unused = t1.clone();
        let _ = t1_unused;
        for k in 1..n/2 {
            let mut row = vec![0.0f64; nc];
            row[0] = 2.0;
            for (idx, j) in (1..n/2).enumerate() {
                row[1+idx] = 2.0 * (2.0 * PI * (j*k) as f64 / n as f64).cos();
            }
            row[1 + (n/2-1)] = 2.0 * (PI * k as f64).cos();
            row[ns] = 0.0;
            row[ns+1] = 0.0;
            chars.push(row);
        }
    }

    (classes, chars)
}

// ── Commands ──────────────────────────────────────────────────────────────────

fn cmd_irreps(args: &[&str]) -> String {
    let mut out = String::new();
    let group_type = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Expected type Z, D, or S"); return out; } };
    let n = match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out };

    match group_type {
        "Z" => {
            s_section(&mut out, &format!("Irreducible Representations of ℤ/{}ℤ", n));
            out.push('\n');
            out.push_str(&format!("  ℤ/{}ℤ is abelian, so all irreps are 1-dimensional over ℂ.\n", n));
            out.push_str(&format!("  There are {} irreps: ρ₀, ρ₁, ..., ρ_{}\n", n, n-1));
            out.push('\n');
            for k in 0..n {
                out.push_str(&format!("  ρ_{}: ℤ/{}ℤ → GL₁(ℂ),  ρ_{}(1) = e^{{2πi·{}/{}}} = {}\n",
                    k, n, k, k, n, fmt_c(zn_irrep_char(n, k, 1))));
            }
            s_note(&mut out, "Each ρ_k is a group homomorphism ℤ/nℤ → ℂˣ.");
            s_note(&mut out, &format!("These are the {} characters of the Pontryagin dual ℤ̂/nℤ̂.", n));
        },
        "D" => {
            if n < 3 { s_err(&mut out, "D_n requires n ≥ 3"); return out; }
            if n > 8 { s_err(&mut out, "Use n ≤ 8 for display"); return out; }
            s_section(&mut out, &format!("Irreducible Representations of D_{} (dihedral, order {})", n, 2*n));
            let (classes, chars) = dn_char_table(n);
            let num_irreps = chars.len();
            out.push_str(&format!("  |D_{}| = {},  {} conjugacy classes,  {} irreps.\n", n, 2*n, classes.len(), num_irreps));
            out.push('\n');
            out.push_str("  1-dim irreps: 2 (n odd) or 4 (n even).\n");
            out.push_str("  2-dim irreps: (n-1)/2 (n odd) or (n-2)/2 (n even).\n");
            out.push('\n');
            out.push_str(&format!("  Dimensions: {:?}\n", chars.iter().map(|r| r[0] as i64).collect::<Vec<_>>()));
            s_note(&mut out, &format!("Verify: Σ dim² = {}·{} = {} = |D_{}|",
                chars.iter().map(|r| (r[0] as i64)*(r[0] as i64)).sum::<i64>() / (2*n as i64),
                2*n, chars.iter().map(|r| (r[0] as i64).pow(2)).sum::<i64>(), n));
        },
        "S" => {
            match n {
                2 => {
                    s_section(&mut out, "Irreducible Representations of S₂ ≅ ℤ/2ℤ");
                    out.push_str("  2 irreps: trivial ρ₊ (1,1) and sign ρ₋ (1,-1).\n");
                },
                3 => {
                    s_section(&mut out, "Irreducible Representations of S₃");
                    out.push_str("  |S₃| = 6,  3 conjugacy classes: {e}, {(12),(13),(23)}, {(123),(132)}\n");
                    out.push_str("  3 irreps:\n");
                    out.push_str("  • Trivial: dim 1,   χ = (1, 1, 1)\n");
                    out.push_str("  • Sign:    dim 1,   χ = (1,-1, 1)\n");
                    out.push_str("  • Standard: dim 2,  χ = (2, 0,-1)\n");
                    s_note(&mut out, "1² + 1² + 2² = 1+1+4 = 6 = |S₃| ✓");
                },
                4 => {
                    s_section(&mut out, "Irreducible Representations of S₄");
                    out.push_str("  |S₄| = 24,  5 conjugacy classes.\n");
                    out.push_str("  5 irreps with dimensions: 1, 1, 2, 3, 3.\n");
                    out.push_str("  • Trivial: (1,1,1,1,1)\n");
                    out.push_str("  • Sign:    (1,-1,1,1,-1)\n");
                    out.push_str("  • Standard⊗Sign: (2,0,-1,2,0) — wait, need full table.\n");
                    out.push_str("  Use: character S 4  for the full character table.\n");
                    s_note(&mut out, "1²+1²+2²+3²+3² = 1+1+4+9+9 = 24 = |S₄| ✓");
                },
                _ => s_err(&mut out, "S_n implemented for n = 2, 3, 4"),
            }
        },
        _ => s_err(&mut out, "Type must be Z (cyclic), D (dihedral), or S (symmetric)"),
    }
    out
}

fn cmd_character(args: &[&str]) -> String {
    let mut out = String::new();
    let group_type = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Expected type"); return out; } };
    let n = match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out };

    match group_type {
        "Z" => {
            s_section(&mut out, &format!("Characters of ℤ/{}ℤ", n));
            out.push('\n');
            out.push_str("  Each irrep ρ_k has character χ_k(j) = e^{2πijk/n}.\n");
            out.push('\n');
            out.push_str(&format!("  Character table (conjugacy classes = individual elements 0..{}):\n", n-1));
            out.push_str(&format!("  {:>5}", ""));
            for j in 0..n { out.push_str(&format!("  {:>9}", format!("j={}", j))); }
            out.push('\n');
            out.push_str(&format!("  {:>5}", ""));
            for _ in 0..n { out.push_str(&format!("  {:>9}", "---------")); }
            out.push('\n');
            for k in 0..n {
                out.push_str(&format!("  ρ_{:<3}", k));
                for j in 0..n {
                    out.push_str(&format!("  {}", fmt_c(zn_irrep_char(n, k, j))));
                }
                out.push('\n');
            }
            out.push('\n');
            s_note(&mut out, "Rows: irreps ρ_k.  Columns: group elements j ∈ ℤ/nℤ.");
            s_note(&mut out, "All entries are n-th roots of unity.");
        },
        "D" => {
            if n < 3 || n > 8 { s_err(&mut out, "Use 3 ≤ n ≤ 8"); return out; }
            s_section(&mut out, &format!("Character Table of D_{}", n));
            let (classes, chars) = dn_char_table(n);
            out.push('\n');
            out.push_str(&format!("  {:>10}", ""));
            for c in &classes { out.push_str(&format!("  {:>8}", c)); }
            out.push('\n');
            out.push_str(&format!("  {:>10}", ""));
            for _ in &classes { out.push_str(&format!("  {:>8}", "--------")); }
            out.push('\n');
            for (i, row) in chars.iter().enumerate() {
                out.push_str(&format!("  χ_{:<7}", i+1));
                for &v in row {
                    let vr = (v * 1000.0).round() / 1000.0;
                    out.push_str(&format!("  {:>8.3}", vr));
                }
                out.push('\n');
            }
        },
        _ => s_err(&mut out, "Use: character Z <n>  or  character D <n>"),
    }
    out
}

fn cmd_schur(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return out };

    s_section(&mut out, &format!("Schur's Lemma for ℤ/{}ℤ", n));
    out.push('\n');
    out.push_str("  Schur's Lemma: Any intertwining operator T: V → W between irreducible\n");
    out.push_str("  ℂ-representations of G is either zero or an isomorphism.\n");
    out.push_str("  Moreover, if V = W (same irrep), then T = λ·I for some λ ∈ ℂ.\n");
    out.push('\n');
    out.push_str(&format!("  For G = ℤ/{}ℤ with irreps ρ_k (all 1-dimensional):\n", n));
    out.push('\n');
    for k in 0..n {
        out.push_str(&format!("  Irrep ρ_{k}: T: ℂ → ℂ intertwining ρ_{k} must satisfy\n", k = k));
        out.push_str(&format!("    T(ρ_{k}(1)·v) = ρ_{k}(1)·T(v) for all v ∈ ℂ\n", k = k));
        let zeta = zn_irrep_char(n, k, 1);
        out.push_str(&format!("    T({}·v) = {}·T(v)\n", fmt_c(zeta), fmt_c(zeta)));
        out.push_str("    This is automatic: T is any scalar λ ∈ ℂ. T = λ·I. ✓\n");
        out.push('\n');
    }
    s_note(&mut out, "For 1-dim irreps (all irreps of abelian groups), Schur's lemma is trivial.");
    s_note(&mut out, "For non-abelian groups, Schur's lemma is deeper and more constraining.");
    s_note(&mut out, "It implies: End_G(V) ≅ ℂ for any irrep V over an algebraically closed field.");
    out
}

fn cmd_decompose(args: &[&str]) -> String {
    let mut out = String::new();
    let group_type = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Expected type"); return out; } };
    let n = match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out };

    let chi_vals: Vec<f64> = args[2..].iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    if chi_vals.is_empty() { s_err(&mut out, "Provide character values"); return out; }

    match group_type {
        "Z" => {
            if chi_vals.len() != n {
                s_err(&mut out, &format!("Need {} values for ℤ/{}ℤ", n, n));
                return out;
            }
            s_section(&mut out, &format!("Decompose Character of ℤ/{}ℤ", n));
            out.push_str(&format!("  χ = {:?}\n", chi_vals));
            out.push('\n');
            out.push_str(&format!("  ⟨χ, ρ_k⟩ = (1/{}) Σ_j χ(j)·ρ_k(j)̄\n", n));
            out.push('\n');
            let mut multiplicities = Vec::new();
            for k in 0..n {
                let mut inner: C = C::new(0.0, 0.0);
                for j in 0..n {
                    let chi_j = C::new(chi_vals[j], 0.0);
                    let rho_bar = zn_irrep_char(n, k, j).conj();
                    inner = inner.add(chi_j.mul(rho_bar));
                }
                let m = (inner.re / n as f64).round() as i64;
                multiplicities.push(m);
                if m != 0 {
                    s_result(&mut out, &format!("⟨χ, ρ_{}⟩", k), &format!("{}", m));
                }
            }
            out.push('\n');
            let decomp: Vec<String> = multiplicities.iter().enumerate()
                .filter(|&(_, &m)| m != 0)
                .map(|(k, &m)| if m == 1 { format!("ρ_{}", k) } else { format!("{}·ρ_{}", m, k) })
                .collect();
            s_result(&mut out, "χ", &decomp.join(" ⊕ "));
        },
        _ => s_err(&mut out, "Decomposition implemented for type Z only in this demo"),
    }
    out
}

fn cmd_regular(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return out };

    s_section(&mut out, &format!("Regular Representation of ℤ/{}ℤ", n));
    out.push('\n');
    out.push_str("  The regular representation R is ℂ[G] acting on itself by left multiplication.\n");
    out.push_str(&format!("  For G = ℤ/{}ℤ: basis = {{e_0, e_1, ..., e_{}}}.\n", n, n-1));
    out.push('\n');
    out.push_str("  Character of R:\n");
    out.push_str(&format!("    χ_R(e) = |G| = {}   (trace = {} since I_n has trace n)\n", n, n));
    out.push_str("    χ_R(g) = 0  for g ≠ e  (every non-identity element moves all basis vectors)\n");
    out.push('\n');
    out.push_str(&format!("  χ_R = {:?}\n", {
        let mut v = vec![0.0f64; n];
        v[0] = n as f64;
        v
    }));
    out.push('\n');
    out.push_str("  Decomposition into irreps:\n");
    out.push_str(&format!("  ⟨χ_R, ρ_k⟩ = (1/{}) · {} = 1  for all k = 0,...,{}\n", n, n, n-1));
    out.push('\n');
    out.push_str(&format!("  ℂ[G] ≅ ρ_0 ⊕ ρ_1 ⊕ ... ⊕ ρ_{} = ℂ^{}\n", n-1, n));
    out.push('\n');
    s_note(&mut out, "Each irrep of G appears in the regular representation with multiplicity = its dimension.");
    s_note(&mut out, &format!("For ℤ/{}ℤ: all irreps are 1-dim, each appears exactly once.", n));
    s_note(&mut out, "This is the Peter-Weyl theorem for finite groups.");
    out
}

fn cmd_direct_sum(args: &[&str]) -> String {
    let mut out = String::new();
    let sep = args.iter().position(|&s| s == "/");
    let (chi1_args, chi2_args) = match sep {
        Some(pos) => (&args[..pos], &args[pos+1..]),
        None => { s_err(&mut out, "Separate two characters with '/'"); return out; },
    };

    let chi1: Vec<f64> = chi1_args.iter().filter_map(|s| s.parse().ok()).collect();
    let chi2: Vec<f64> = chi2_args.iter().filter_map(|s| s.parse().ok()).collect();

    if chi1.len() != chi2.len() {
        s_err(&mut out, "Characters must have the same number of values");
        return out;
    }

    s_section(&mut out, "Direct Sum of Characters");
    out.push_str(&format!("  χ₁ = {:?}\n", chi1));
    out.push_str(&format!("  χ₂ = {:?}\n", chi2));
    let direct_sum: Vec<f64> = chi1.iter().zip(chi2.iter()).map(|(a, b)| a + b).collect();
    out.push_str(&format!("  χ₁ ⊕ χ₂ = {:?}\n", direct_sum));
    s_note(&mut out, "(χ₁ ⊕ χ₂)(g) = χ₁(g) + χ₂(g): trace of block diagonal matrix.");
    out
}

fn cmd_tensor_rep(args: &[&str]) -> String {
    let mut out = String::new();
    let group_n = match parse_uint(args, 0, "N") { Some(x) => x as usize, None => return out };
    let k1 = match parse_uint(args, 1, "k1") { Some(x) => x as usize, None => return out };
    let k2 = match parse_uint(args, 2, "k2") { Some(x) => x as usize, None => return out };

    s_section(&mut out, &format!("Tensor Product ρ_{} ⊗ ρ_{} in ℤ/{}ℤ", k1, k2, group_n));
    out.push('\n');
    out.push_str(&format!("  ρ_{} ⊗ ρ_{}: ℤ/{}ℤ → GL₁(ℂ)\n", k1, k2, group_n));
    out.push_str(&format!("  (ρ_{} ⊗ ρ_{})(j) = ρ_{}(j) · ρ_{}(j)  (scalar multiplication)\n", k1, k2, k1, k2));
    out.push('\n');
    out.push_str(&format!("  Character: χ(j) = ρ_{}(j)·ρ_{}(j) = e^{{2πi·{}j/{}}} · e^{{2πi·{}j/{}}} = e^{{2πi·{}j/{}}}\n",
        k1, k2, k1, group_n, k2, group_n, (k1+k2) % group_n, group_n));
    out.push('\n');
    let tensor_k = (k1 + k2) % group_n;
    out.push_str(&format!("  ρ_{} ⊗ ρ_{} ≅ ρ_{}\n", k1, k2, tensor_k));
    out.push('\n');
    s_note(&mut out, "For abelian groups, tensor product of 1-dim irreps is again a 1-dim irrep.");
    s_note(&mut out, &format!("The irreps of ℤ/{}ℤ form a group under tensor product, isomorphic to ℤ/{}ℤ itself.", group_n, group_n));
    out.push('\n');
    out.push_str(&format!("  All tensor products in ℤ/{}ℤ:\n", group_n));
    for i in 0..group_n.min(5) {
        for j in 0..group_n.min(5) {
            out.push_str(&format!("  ρ_{}⊗ρ_{}=ρ_{}  ", i, j, (i+j)%group_n));
        }
        out.push('\n');
    }
    out
}

fn cmd_demo() -> String {
    let mut out = String::new();
    s_section(&mut out, "Demo: Group Representations Showcase");
    out.push('\n');
    out.push_str("  --- Irreps of ℤ/4ℤ ---\n");
    out.push_str(&cmd_irreps(&["Z", "4"]));
    out.push('\n');
    out.push_str("  --- Regular representation of ℤ/3ℤ ---\n");
    out.push_str(&cmd_regular(&["3"]));
    out.push('\n');
    out.push_str("  --- Decompose character of ℤ/4ℤ ---\n");
    out.push_str(&cmd_decompose(&["Z", "4", "4", "0", "0", "0"]));
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
        "irreps"      => cmd_irreps(args),
        "character"   => cmd_character(args),
        "schur"       => cmd_schur(args),
        "decompose"   => cmd_decompose(args),
        "regular"     => cmd_regular(args),
        "direct_sum"  => cmd_direct_sum(args),
        "tensor_rep"  => cmd_tensor_rep(args),
        "demo"        => cmd_demo(),
        "help" | "h"  => show_help(),
        _ => { let mut out = String::new(); s_err(&mut out, &format!("Unknown command '{}'. Type 'help'.", cmd)); out }
    }
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { print!("{out}"); }
    true
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    // Representation matrix diagram for ℤ/4ℤ
    c.text_bold(350.0, 30.0, "Representations of ℤ/4ℤ", 16.0, "#222", "middle");
    let n = 4usize;
    let cx = 350.0_f64;
    let cy = 200.0_f64;
    let r = 120.0_f64;
    // Draw nodes for group elements
    for j in 0..n {
        let angle = 2.0 * PI * j as f64 / n as f64 - PI / 2.0;
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        c.node_circle(x, y, &format!("{}", j), "white", 25.0, 13.0);
    }
    // Representation values around the ring
    for k in 0..n {
        let val = zn_irrep_char(n, k, 1);
        let label = format!("ρ_{}: {}", k, fmt_c(val));
        c.text(10.0, 350.0 + k as f64 * 25.0, &label, 12.0, "#333", "start");
    }
    c.text(cx - 40.0, cy, "ℤ/4ℤ", 13.0, "#333", "middle");
    c.text(500.0, 200.0, "Each ρ_k:", 12.0, "#333", "start");
    c.text(500.0, 225.0, "ρ_k(j) = e^{2πijk/4}", 12.0, "#333", "start");
    c.text(500.0, 250.0, "1-dimensional irreps", 12.0, "#333", "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("G", &[("label", "G = ℤ/nℤ"), ("shape", "ellipse")]);
    g.node("GL", &[("label", "GL_1(ℂ)"), ("shape", "ellipse")]);
    g.node("rho0", &[("label", "ρ₀ (trivial)"), ("shape", "box")]);
    g.node("rho1", &[("label", "ρ₁"), ("shape", "box")]);
    g.node("rhon", &[("label", "ρₙ₋₁"), ("shape", "box")]);
    g.edge("G", "rho0", &[("label", "")]);
    g.edge("G", "rho1", &[("label", "")]);
    g.edge("G", "rhon", &[("label", "")]);
    g.edge("rho0", "GL", &[("label", "")]);
    g.edge("rho1", "GL", &[("label", "")]);
    g.edge("rhon", "GL", &[("label", "")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("G", 0.0, 0.0, "$G$", "");
    t.node("V", 3.0, 1.0, "$V$", "");
    t.node("W", 3.0, -1.0, "$W$", "");
    t.node("GL", 6.0, 0.0, "$\\text{GL}(V)$", "");
    t.arrow("G", "V", "$\\rho$", "above");
    t.arrow("G", "W", "$\\sigma$", "below");
    t.arrow("V", "GL", "", "");
    t.arrow("W", "GL", "", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 1, "Group Representations of Z/nZ");
    a.hline(0, 35, 2, '-');
    a.text_at(0, 4, "rho_k: Z/nZ --> GL_1(C)");
    a.text_at(0, 5, "rho_k(j) = exp(2*pi*i*k*j/n)");
    a.hline(0, 35, 7, '-');
    a.text_at(0, 9,  "n=4 character table:");
    a.text_at(0, 10, "     j=0  j=1   j=2   j=3");
    a.text_at(0, 11, "rho0:  1    1     1     1");
    a.text_at(0, 12, "rho1:  1    i    -1    -i");
    a.text_at(0, 13, "rho2:  1   -1     1    -1");
    a.text_at(0, 14, "rho3:  1   -i    -1     i");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch42"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 42", "Group Representations", "Irreducible representations and their characters");
            print!("{}", show_help());
            print_note("Try: irreps Z 4   or   character Z 6   or   regular 5");
            print_note("Or: schur 3   or   decompose Z 4 4 0 0 0");
            repl("ch42> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
