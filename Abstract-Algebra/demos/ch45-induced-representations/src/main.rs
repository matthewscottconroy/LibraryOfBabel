use common::*;

fn show_help() -> String {
    let mut out = String::new();
    let rows = &[
        ("induce <G_size> <H_size> <chi_H>",            "compute induced character Ind_H^G(χ)"),
        ("restrict <G_chi> / <H_indices>",              "restrict G-character to subgroup H"),
        ("frobenius_reciprocity <Gs> <Hs> <chiH> <psiG>","verify ⟨Ind χ, ψ⟩_G = ⟨χ, Res ψ⟩_H"),
        ("induce_trivial <n> <k>",                      "induce trivial char of kℤ/nℤ to ℤ/nℤ"),
        ("mackey <G> <H> <K>",                          "Mackey's formula for Res∘Ind"),
        ("frobenius_group <n>",                         "Frobenius group Aff(ℤ/nℤ) for n prime"),
        ("demo",                                        "showcase of induced representation examples"),
        ("help",                                        "show this help"),
        ("quit",                                        "exit"),
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
fn s_ok(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[32m✓\x1b[0m {}\n", text));
}

const PI: f64 = std::f64::consts::PI;

// ── Inner product of characters on ℤ/nℤ ──────────────────────────────────────

fn inner_zn(chi1: &[f64], chi2: &[f64], n: usize) -> f64 {
    let sum: f64 = chi1.iter().zip(chi2.iter()).map(|(a, b)| a * b).sum();
    sum / n as f64
}

fn zn_irrep_char_real(n: usize, k: usize, j: usize) -> f64 {
    (2.0 * PI * (k * j) as f64 / n as f64).cos()
}

// ── Induced character (general abelian case) ──────────────────────────────────

fn cmd_induce(args: &[&str]) -> String {
    let mut out = String::new();
    let g_size = match parse_uint(args, 0, "G_size") { Some(x) => x as usize, None => return out };
    let h_size = match parse_uint(args, 1, "H_size") { Some(x) => x as usize, None => return out };

    if g_size == 0 || h_size == 0 || g_size % h_size != 0 {
        s_err(&mut out, "H_size must divide G_size");
        return out;
    }

    let chi_h: Vec<f64> = args[2..].iter().filter_map(|s| s.parse().ok()).collect();
    if chi_h.len() != h_size {
        s_err(&mut out, &format!("Need {} values for the H-character (|H| = {})", h_size, h_size));
        return out;
    }

    s_section(&mut out, "Induced Character: Ind_H^G(χ)");
    out.push_str(&format!("  G = ℤ/{}ℤ,  H = ℤ/{}ℤ (subgroup of index {})\n", g_size, h_size, g_size / h_size));
    out.push_str(&format!("  χ on H: {:?}\n", chi_h));
    out.push('\n');
    out.push_str("  Formula: Ind_H^G(χ)(g) = (1/|H|) Σ_{x∈G, x⁻¹gx∈H} χ(x⁻¹gx)\n");
    out.push('\n');

    let step = g_size / h_size;
    let h_elements: Vec<usize> = (0..g_size).step_by(step).collect();

    out.push_str(&format!("  H = {{{}}} (subgroup of ℤ/{}ℤ)\n",
        h_elements.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "),
        g_size));
    out.push('\n');

    let index = g_size / h_size;
    let mut ind_chi = vec![0.0f64; g_size];

    for (j, &h_elem) in h_elements.iter().enumerate() {
        if j < chi_h.len() {
            ind_chi[h_elem] = index as f64 * chi_h[j];
        }
    }

    out.push_str("  (G is abelian, so x⁻¹gx = g: Ind simplifies to)\n");
    out.push_str("  Ind_H^G(χ)(g) = [G:H]·χ(g) if g ∈ H,  0 if g ∉ H\n");
    out.push('\n');
    out.push_str(&format!("  Induced character on ℤ/{}ℤ:\n", g_size));
    for (j, &v) in ind_chi.iter().enumerate() {
        out.push_str(&format!("    g={}: Ind(χ)(g) = {}\n", j, v));
    }
    out.push('\n');
    s_result(&mut out, "Index [G:H]", &format!("{}", index));
    s_result(&mut out, "dim Ind(χ)", &format!("{} = [G:H]·dim(χ) = {}·{} = {}",
        ind_chi[0], index, chi_h[0], (index as f64) * chi_h[0]));

    out.push('\n');
    out.push_str(&format!("  Decompose into irreps of G = ℤ/{}ℤ:\n", g_size));
    for k in 0..g_size {
        let irrep_chi: Vec<f64> = (0..g_size).map(|j| {
            (2.0 * PI * (k * j) as f64 / g_size as f64).cos()
        }).collect();
        let inner = ind_chi.iter().zip(irrep_chi.iter()).map(|(a, b)| a * b).sum::<f64>() / g_size as f64;
        let m = inner.round() as i64;
        if m != 0 {
            s_result(&mut out, &format!("  ⟨Ind(χ), ρ_{}⟩", k), &format!("{}", m));
        }
    }
    s_note(&mut out, "By Frobenius reciprocity: ⟨Ind_H^G χ, ρ_k⟩_G = ⟨χ, ρ_k|_H⟩_H.");
    out
}

// ── Restrict ──────────────────────────────────────────────────────────────────

fn cmd_restrict(args: &[&str]) -> String {
    let mut out = String::new();
    let sep = args.iter().position(|&s| s == "/");
    let sep = match sep { Some(s) => s, None => { s_err(&mut out, "Use: restrict <G_chi vals> / <H_indices>"); return out; } };

    let g_chi: Vec<f64> = args[..sep].iter().filter_map(|s| s.parse().ok()).collect();
    let h_indices: Vec<usize> = args[sep+1..].iter().filter_map(|s| s.parse().ok()).collect();

    if g_chi.is_empty() || h_indices.is_empty() {
        s_err(&mut out, "Provide G-character values and H element indices");
        return out;
    }

    let n = g_chi.len();
    s_section(&mut out, "Restriction: G-character restricted to subgroup H");
    out.push_str(&format!("  G has {} elements (indices 0..{})\n", n, n-1));
    out.push_str(&format!("  χ on G: {:?}\n", g_chi));
    out.push_str(&format!("  H element indices: {:?}\n", h_indices));
    out.push('\n');

    let h_chi: Vec<f64> = h_indices.iter().filter_map(|&i| {
        if i < n { Some(g_chi[i]) } else {
            None
        }
    }).collect();

    // Report any out-of-range indices
    for &i in &h_indices {
        if i >= n {
            s_err(&mut out, &format!("Index {} out of range (G has {} elements)", i, n));
        }
    }

    out.push_str(&format!("  Res_H^G(χ) = {:?}\n", h_chi));
    out.push('\n');
    out.push_str("  The restriction just takes the values of χ at the H-elements.\n");
    s_note(&mut out, "Res_H^G is an exact functor: Rep(G) → Rep(H).");
    s_note(&mut out, "By Frobenius reciprocity: ⟨Ind_H^G φ, χ⟩_G = ⟨φ, Res_H^G χ⟩_H.");
    out
}

// ── Frobenius reciprocity ─────────────────────────────────────────────────────

fn cmd_frobenius_reciprocity(args: &[&str]) -> String {
    let mut out = String::new();
    let g_size = match parse_uint(args, 0, "G_size") { Some(x) => x as usize, None => return out };
    let h_size = match parse_uint(args, 1, "H_size") { Some(x) => x as usize, None => return out };

    if g_size % h_size != 0 { s_err(&mut out, "H_size must divide G_size"); return out; }

    let remaining = &args[2..];
    if remaining.len() < h_size + g_size {
        s_err(&mut out, &format!("Need {} H-values then {} G-values", h_size, g_size));
        return out;
    }
    let chi_h: Vec<f64> = remaining[..h_size].iter().filter_map(|s| s.parse().ok()).collect();
    let psi_g: Vec<f64> = remaining[h_size..h_size+g_size].iter().filter_map(|s| s.parse().ok()).collect();

    if chi_h.len() != h_size || psi_g.len() != g_size {
        s_err(&mut out, "Could not parse character values");
        return out;
    }

    s_section(&mut out, "Frobenius Reciprocity");
    out.push_str(&format!("  G = ℤ/{}ℤ,  H = ℤ/{}ℤ (subgroup of index {})\n", g_size, h_size, g_size/h_size));
    out.push_str(&format!("  χ_H = {:?}\n", chi_h));
    out.push_str(&format!("  ψ_G = {:?}\n", psi_g));
    out.push('\n');
    out.push_str("  Frobenius Reciprocity: ⟨Ind_H^G χ, ψ⟩_G = ⟨χ, Res_H^G ψ⟩_H\n");
    out.push('\n');

    let step = g_size / h_size;
    let h_elems: Vec<usize> = (0..g_size).step_by(step).collect();
    let index = g_size / h_size;
    let mut ind_chi = vec![0.0f64; g_size];
    for (j, &h_elem) in h_elems.iter().enumerate() {
        if j < chi_h.len() { ind_chi[h_elem] = index as f64 * chi_h[j]; }
    }

    let lhs = ind_chi.iter().zip(psi_g.iter()).map(|(a, b)| a * b).sum::<f64>() / g_size as f64;

    let res_psi: Vec<f64> = h_elems.iter().map(|&i| psi_g[i]).collect();

    let rhs = chi_h.iter().zip(res_psi.iter()).map(|(a, b)| a * b).sum::<f64>() / h_size as f64;

    s_result(&mut out, "⟨Ind_H^G χ, ψ⟩_G", &format!("{:.4}", lhs));
    s_result(&mut out, "⟨χ, Res_H^G ψ⟩_H", &format!("{:.4}", rhs));

    if (lhs - rhs).abs() < 0.001 {
        s_ok(&mut out, "Frobenius reciprocity verified: both sides are equal ✓");
    } else {
        s_err(&mut out, &format!("Mismatch: {:.4} ≠ {:.4}", lhs, rhs));
    }
    out.push('\n');
    s_note(&mut out, "Frobenius reciprocity is a natural isomorphism:");
    s_note(&mut out, "  Hom_G(Ind_H^G V, W) ≅ Hom_H(V, Res_H^G W)");
    s_note(&mut out, "i.e., Ind is LEFT adjoint to Res.");
    out
}

// ── Induce trivial ────────────────────────────────────────────────────────────

fn cmd_induce_trivial(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return out };
    let k = match parse_uint(args, 1, "k") { Some(x) => x as usize, None => return out };

    if k == 0 || n % k != 0 { s_err(&mut out, "k must divide n"); return out; }
    let h_size = n / k;
    let index = k;

    s_section(&mut out, &format!("Ind_H^G(1) where G=ℤ/{}ℤ, H={}ℤ/{}ℤ", n, k, n));
    out.push_str(&format!("  H = {{0, {}, {}, ..., {}}}  (subgroup generated by {})\n", k, 2*k, (h_size-1)*k, k));
    out.push_str(&format!("  |H| = {},  [G:H] = {}\n", h_size, index));
    out.push_str("  χ_H = trivial: χ(h) = 1 for all h ∈ H.\n");
    out.push('\n');

    let h_elems: Vec<usize> = (0..n).step_by(k).collect();
    let mut ind_chi = vec![0.0f64; n];
    for &h in &h_elems { ind_chi[h] = index as f64; }

    out.push_str("  Ind_H^G(1)(g):\n");
    for j in 0..n.min(12) {
        out.push_str(&format!("    g={}: {}\n", j, ind_chi[j]));
    }
    if n > 12 { out.push_str("    ...\n"); }
    out.push('\n');

    out.push_str(&format!("  Decompose into irreps of ℤ/{}ℤ:\n", n));
    let mut decomp = Vec::new();
    for m in 0..n {
        let irrep_chi: Vec<f64> = (0..n).map(|j| (2.0 * PI * (m * j) as f64 / n as f64).cos()).collect();
        let inner = ind_chi.iter().zip(irrep_chi.iter()).map(|(a, b)| a * b).sum::<f64>() / n as f64;
        let mult = inner.round() as i64;
        if mult != 0 {
            decomp.push(format!("ρ_{}", m));
        }
    }
    out.push_str(&format!("  Ind_H^G(1) = {}\n", decomp.join(" ⊕ ")));
    out.push('\n');
    s_note(&mut out, "By Frobenius reciprocity: ⟨Ind 1, ρ_m⟩_G = ⟨1, ρ_m|_H⟩_H.");
    s_note(&mut out, "ρ_m|_H is trivial iff m is a multiple of k (n/h_size = k divides m... depends).");
    s_note(&mut out, "The induced representation contains all irreps ρ_m with ρ_m|_H trivial.");
    out
}

// ── Mackey formula ────────────────────────────────────────────────────────────

fn cmd_mackey(args: &[&str]) -> String {
    let mut out = String::new();
    let g_order = match parse_uint(args, 0, "G_order") { Some(x) => x as usize, None => return out };
    let h_order = match parse_uint(args, 1, "H_order") { Some(x) => x as usize, None => return out };
    let k_order = match parse_uint(args, 2, "K_order") { Some(x) => x as usize, None => return out };

    if g_order % h_order != 0 || g_order % k_order != 0 {
        s_err(&mut out, "Both H and K orders must divide G order");
        return out;
    }

    s_section(&mut out, "Mackey's Formula: Res_K^G ∘ Ind_H^G");
    out.push_str(&format!("  G = ℤ/{}ℤ,  H = ℤ/{}ℤ,  K = ℤ/{}ℤ\n", g_order, h_order, k_order));
    out.push('\n');
    out.push_str("  Mackey's formula (general):\n");
    out.push_str("  Res_K^G(Ind_H^G V) ≅ ⊕_{t ∈ K\\G/H} Ind_{K∩tHt⁻¹}^K(Res_{K∩tHt⁻¹}^{tHt⁻¹} ᵗV)\n");
    out.push('\n');
    out.push_str(&format!("  For G abelian (ℤ/{}ℤ): conjugation is trivial, so tHt⁻¹ = H for all t.\n", g_order));
    out.push_str("  Double cosets K\\G/H = {K+g+H} reduce to standard cosets.\n");
    out.push('\n');

    let h_step = g_order / h_order;
    let k_step = g_order / k_order;

    let inter_step = lcm(h_step as i64, k_step as i64) as usize;
    let inter_order = if inter_step <= g_order { g_order / inter_step } else { 0 };

    out.push_str(&format!("  H = {{0, {}, {}, ...}}  (step {})\n", h_step, 2*h_step, h_step));
    out.push_str(&format!("  K = {{0, {}, {}, ...}}  (step {})\n", k_step, 2*k_step, k_step));
    out.push_str(&format!("  K ∩ H = {{0, {}, ...}}  (step {}, order {})\n", inter_step, inter_step, inter_order));
    out.push('\n');

    let dcoset_size = h_order.min(k_order);
    out.push_str(&format!("  |K\\G/H| = |G| / (|K|·|H| / |K∩H|) = {} / {} = {}\n",
        g_order, dcoset_size, g_order / dcoset_size));
    out.push('\n');
    out.push_str("  Simplified for abelian G:\n");
    out.push_str("  Res_K^G(Ind_H^G χ) ≅ ⊕_{[G:H] cosets} (version of χ for each coset)\n");
    out.push('\n');
    s_note(&mut out, "For abelian groups, the Mackey formula simplifies enormously.");
    s_note(&mut out, "The double coset decomposition becomes trivial (G acts on itself trivially).");
    s_note(&mut out, "For non-abelian G, Mackey's formula is essential for understanding induced reps.");
    out
}

// ── Frobenius group ───────────────────────────────────────────────────────────

fn cmd_frobenius_group(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return out };
    if !is_prime(n as u64) { s_err(&mut out, "n must be prime for Aff(ℤ/nℤ) to be a Frobenius group"); return out; }

    let order = n * (n - 1);

    s_section(&mut out, &format!("Frobenius Group: Aff(ℤ/{}ℤ) = {{x↦ax+b : a≠0}}", n));
    out.push('\n');
    out.push_str(&format!("  Aff(ℤ/{}ℤ) = {{(a,b) : a ∈ (ℤ/{}ℤ)×, b ∈ ℤ/{}ℤ}}\n", n, n, n));
    out.push_str("  Group operation: (a,b)·(c,d) = (ac, ad+b)  (composition of affine maps)\n");
    out.push_str(&format!("  |Aff(ℤ/{}ℤ)| = |ℤ/{}ℤ× | · |ℤ/{}ℤ| = {} · {} = {}\n",
        n, n, n, n-1, n, order));
    out.push('\n');
    out.push_str("  Frobenius decomposition:\n");
    out.push_str(&format!("  • Frobenius kernel K = {{(1,b) : b ∈ ℤ/{}ℤ}} ≅ ℤ/{}ℤ  (normal, order {})\n", n, n, n));
    out.push_str(&format!("  • Frobenius complement H = {{(a,0) : a ∈ (ℤ/{}ℤ)×}} ≅ ℤ/{}ℤ  (order {})\n",
        n, n-1, n-1));
    out.push('\n');
    out.push_str("  Frobenius group structure:\n");
    out.push_str("  1. K is normal in G (easy to verify).\n");
    out.push_str("  2. K ∩ H = {e} (trivial intersection).\n");
    out.push_str("  3. G = KH  (every element is (a,b) = (1,b)·(a,0)).\n");
    out.push_str("  4. Every non-identity element of H has no fixed point in K\\{e}.\n");
    out.push_str("     (The defining property: h·k·h⁻¹ ≠ k for k ≠ e, h ≠ e in H)\n");
    out.push('\n');
    out.push_str("  Frobenius' theorem: The kernel K is the set of elements of G\n");
    out.push_str("  NOT conjugate to any non-identity element of H, plus the identity.\n");
    out.push('\n');
    out.push_str("  Conjugacy classes:\n");
    out.push_str("  • {e}: size 1\n");
    out.push_str(&format!("  • Classes in K\\{{e}}: each has size |H| = {} (H acts freely)\n", n-1));
    let n_k_classes = 1;
    out.push_str(&format!("    Since H = (ℤ/{}ℤ)× acts transitively on ℤ/{}ℤ\\{{0}} (n is prime),\n", n, n));
    out.push_str(&format!("    K\\{{e}} forms a single conjugacy class of size {} in G.\n", n-1));
    out.push_str(&format!("  • Classes from H\\{{e}}: {}\n", n-1));
    out.push_str("    Each non-identity element of H is its own class? No — H is abelian,\n");
    out.push_str("    conjugation within H fixes elements. Conjugation by K acts on H.\n");
    out.push('\n');
    out.push_str(&format!("  Total classes: {} = 1 + 1 + {} = {}\n", n_k_classes + 1 + (n-1) as usize, n-1, 1 + 1 + n-1));
    out.push('\n');
    out.push_str("  Irreducible representations:\n");
    out.push_str(&format!("  • From H (inflated): {} one-dim irreps (via G → G/K ≅ H ≅ ℤ/{}ℤ)\n", n-1, n-1));
    out.push_str(&format!("  • Induced from K (non-trivial chars of K): {} irrep of dimension {}\n", 1, n-1));
    out.push_str(&format!("    (induced from a non-trivial char ψ of K, dim = [G:K] = {})\n", n-1));
    out.push('\n');
    let _sum_sq = (n-1)*(n-1) + (n-1)*1*1;
    let true_sum = (n-1) + (n-1)*(n-1);
    out.push_str(&format!("  Dim check: {} · 1² + 1 · {}² = {} + {} = {} = |G| {}\n",
        n-1, n-1, n-1, (n-1)*(n-1), true_sum, if true_sum == order { "✓" } else { "✗" }));
    out.push('\n');
    s_note(&mut out, "Frobenius groups are the fundamental examples of non-split group extensions.");
    s_note(&mut out, "The kernel K is always a nilpotent normal subgroup (Frobenius' theorem).");
    s_note(&mut out, "Proof of Frobenius' theorem uses character theory in an essential way.");
    out
}

fn cmd_demo() -> String {
    let mut out = String::new();
    s_section(&mut out, "Demo: Induced Representations Showcase");
    out.push('\n');
    out.push_str("  --- Induce trivial char of 2ℤ/6ℤ to ℤ/6ℤ ---\n");
    out.push_str(&cmd_induce_trivial(&["6", "2"]));
    out.push('\n');
    out.push_str("  --- Frobenius group Aff(ℤ/5ℤ) ---\n");
    out.push_str(&cmd_frobenius_group(&["5"]));
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
        "induce"               => cmd_induce(args),
        "restrict"             => cmd_restrict(args),
        "frobenius_reciprocity" => cmd_frobenius_reciprocity(args),
        "induce_trivial"       => cmd_induce_trivial(args),
        "mackey"               => cmd_mackey(args),
        "frobenius_group"      => cmd_frobenius_group(args),
        "demo"                 => cmd_demo(),
        "help" | "h"           => show_help(),
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
    // Coset decomposition diagram for G/H
    c.text_bold(350.0, 30.0, "Induced Representation: Coset Decomposition", 15.0, "#222", "middle");
    let n = 6usize; let k = 2usize;
    let _h_size = n / k;
    let _index = k;
    // Show G = ℤ/6ℤ as circle with cosets highlighted
    let cx = 350.0_f64; let cy = 220.0_f64; let r = 140.0_f64;
    // Full group elements
    for j in 0..n {
        let angle = 2.0 * std::f64::consts::PI * j as f64 / n as f64 - std::f64::consts::PI / 2.0;
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        let in_h = j % k == 0;
        let fill = if in_h { "#cce" } else { "white" };
        c.circle(x, y, 20.0, fill, "#333", 1.5);
        c.text(x - 5.0, y + 5.0, &format!("{}", j), 12.0, "#333", "middle");
        if in_h {
            c.text(x - 5.0, y + 25.0, "H", 10.0, "#66a", "middle");
        }
    }
    c.text(cx - 15.0, cy, "G", 14.0, "#333", "middle");
    // Legend
    c.text(30.0, 400.0, "G = ℤ/6ℤ,  H = {0,2,4} (index 2)", 12.0, "#333", "start");
    c.text(30.0, 425.0, "Ind_H^G(1): value [G:H]=2 on H, 0 off H", 12.0, "#333", "start");
    c.text(30.0, 450.0, "Ind_H^G(1) = ρ₀ ⊕ ρ₂ ⊕ ρ₄ (Frobenius recip.)", 12.0, "#333", "start");
    // Adjunction
    c.text(450.0, 150.0, "Frobenius:", 13.0, "#333", "start");
    c.text(450.0, 175.0, "⟨Ind χ, ψ⟩_G", 12.0, "#333", "start");
    c.text(450.0, 200.0, "  = ⟨χ, Res ψ⟩_H", 12.0, "#333", "start");
    c.text(450.0, 225.0, "Ind ⊣ Res", 13.0, "#333", "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("RepH", &[("label", "Rep(H)"), ("shape", "box")]);
    g.node("RepG", &[("label", "Rep(G)"), ("shape", "box")]);
    g.edge("RepH", "RepG", &[("label", "Ind_H^G")]);
    g.edge("RepG", "RepH", &[("label", "Res_H^G")]);
    g.node("adj", &[("label", "Ind ⊣ Res"), ("shape", "plaintext")]);
    g.node("frob", &[("label", "⟨Ind χ,ψ⟩_G = ⟨χ,Res ψ⟩_H"), ("shape", "plaintext")]);
    g.edge("RepH", "adj", &[("label", "")]);
    g.edge("adj", "frob", &[("label", "")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("reph", 0.0, 0.0, "$\\text{Rep}(H)$", "");
    t.node("repg", 4.0, 0.0, "$\\text{Rep}(G)$", "");
    t.arrow("reph", "repg", "$\\text{Ind}_H^G$", "above");
    t.arrow("repg", "reph", "$\\text{Res}_H^G$", "below");
    t.node("coset1", 0.0, -2.0, "coset $gH$", "");
    t.node("coset2", 2.0, -2.0, "coset $g'H$", "");
    t.node("coset3", 4.0, -2.0, "$\\ldots$", "");
    t.arrow("coset1", "coset2", "", "");
    t.arrow("coset2", "coset3", "", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 1, "Induced Representations: G = Z/6Z, H = {0,2,4}");
    a.hline(0, 50, 2, '-');
    a.text_at(0, 4, "Coset decomposition: G/H = {H, 1+H}");
    a.text_at(0, 5, "H   = {0, 2, 4}");
    a.text_at(0, 6, "1+H = {1, 3, 5}");
    a.hline(0, 35, 8, '-');
    a.text_at(0, 10, "Ind_H^G(trivial): value 2 on H, 0 on 1+H");
    a.text_at(0, 11, "Frobenius reciprocity: Ind -| Res");
    a.text_at(0, 12, "<Ind chi, psi>_G = <chi, Res psi>_H");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch45"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 45", "Induced Representations", "Frobenius reciprocity and induced characters");
            print!("{}", show_help());
            print_note("Try: frobenius_group 5   or   induce_trivial 6 2");
            print_note("Or: induce 6 3 1 1 1   or   mackey 12 4 6");
            repl("ch45> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
