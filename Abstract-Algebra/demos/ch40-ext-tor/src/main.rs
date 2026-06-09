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

fn show_help() -> String {
    let commands: &[(&str, &str)] = &[
        ("ext <n> <m>",              "Ext¹_ℤ(ℤ/nℤ, ℤ/mℤ) ≅ ℤ/gcd(n,m)ℤ"),
        ("tor <n> <m>",              "Tor₁^ℤ(ℤ/nℤ, ℤ/mℤ) ≅ ℤ/gcd(n,m)ℤ"),
        ("ext_n <n> <m> <k>",        "Extᵏ_ℤ(ℤ/nℤ, ℤ/mℤ) for higher k"),
        ("extensions <n> <m>",       "list all extensions 0→ℤ/nℤ→E→ℤ/mℤ→0"),
        ("group_cohomology <n>",     "H^k(ℤ/nℤ, ℤ) for k=0,1,2,3"),
        ("long_exact <n> <m> <k>",   "long exact Ext sequence from 0→ℤ/nℤ→ℤ/mnℤ→ℤ/mℤ→0"),
        ("flat_test <n>",            "Tor test for flatness of ℤ/nℤ"),
        ("demo",                     "showcase of Ext/Tor computations"),
        ("help",                     "show this help"),
        ("quit",                     "exit"),
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

// ── Ext¹ ──────────────────────────────────────────────────────────────────────

fn cmd_ext(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let m = match parse_uint(args, 1, "m") { Some(x) => x as i64, None => return String::new() };
    let d = gcd(n, m);

    let mut out = String::new();
    s_section(&mut out, &format!("Ext¹_ℤ(ℤ/{}ℤ, ℤ/{}ℤ)", n, m));
    out.push('\n');
    out.push_str(&format!("  Step 1: Free resolution of ℤ/{}ℤ over ℤ:\n", n));
    out.push_str(&format!("    0 → ℤ →(×{})→ ℤ →(ε)→ ℤ/{}ℤ → 0\n\n", n, n));
    out.push_str(&format!("  Step 2: Apply Hom_ℤ(−, ℤ/{}ℤ) to the resolution (drop ε):\n", m));
    out.push_str(&format!("    0 → Hom(ℤ, ℤ/{}ℤ) →(−∘×{})→ Hom(ℤ, ℤ/{}ℤ) → 0\n\n", m, n, m));
    out.push_str(&format!("  Identify: Hom(ℤ, ℤ/{}ℤ) ≅ ℤ/{}ℤ\n", m, m));
    out.push_str(&format!("  The map (−∘×{}) becomes ×{}: ℤ/{}ℤ → ℤ/{}ℤ\n\n", n, n, m, m));
    out.push_str("  Step 3: Compute cohomology of:\n");
    out.push_str(&format!("    0 → ℤ/{}ℤ →(×{})→ ℤ/{}ℤ → 0\n\n", m, n, m));
    out.push_str(&format!("  Ext⁰ = H⁰ = ker(×{}) = {{x ∈ ℤ/{}ℤ : {}x ≡ 0 mod {}}}\n", n, m, n, m));
    let ker0: Vec<i64> = (0..m).filter(|&x| (n * x).rem_euclid(m) == 0).collect();
    s_result(&mut out, "Ext⁰ = Hom(ℤ/nℤ, ℤ/mℤ)", &format!("ℤ/{}ℤ  (= ker(×{}), size {})", d, n, ker0.len()));

    out.push('\n');
    out.push_str(&format!("  Ext¹ = H¹ = coker(×{}) = ℤ/{}ℤ / im(×{})\n", n, m, n));
    let im_n: std::collections::BTreeSet<i64> = (0..m).map(|x| (n * x).rem_euclid(m)).collect();
    let coker_size = m as usize / im_n.len();
    s_result(&mut out, &format!("Ext¹_ℤ(ℤ/{}ℤ, ℤ/{}ℤ)", n, m),
        &format!("ℤ/{}ℤ  (= ℤ/gcd({},{})ℤ)", coker_size, n, m));
    out.push('\n');
    out.push_str(&format!("  Formula: Ext¹_ℤ(ℤ/nℤ, ℤ/mℤ) ≅ ℤ/gcd(n,m)ℤ = ℤ/{}ℤ\n", d));
    s_note(&mut out, &format!("gcd({}, {}) = {}", n, m, d));
    s_note(&mut out, "This formula is exact for all n,m ≥ 1 over ℤ.");
    out
}

// ── Tor₁ ─────────────────────────────────────────────────────────────────────

fn cmd_tor(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let m = match parse_uint(args, 1, "m") { Some(x) => x as i64, None => return String::new() };
    let d = gcd(n, m);

    let mut out = String::new();
    s_section(&mut out, &format!("Tor₁^ℤ(ℤ/{}ℤ, ℤ/{}ℤ)", n, m));
    out.push('\n');
    out.push_str(&format!("  Step 1: Free resolution of ℤ/{}ℤ:\n", n));
    out.push_str(&format!("    0 → ℤ →(×{})→ ℤ → ℤ/{}ℤ → 0\n\n", n, n));
    out.push_str(&format!("  Step 2: Tensor with ℤ/{}ℤ (apply − ⊗_ℤ ℤ/{}ℤ):\n", m, m));
    out.push_str(&format!("    ℤ ⊗ ℤ/{}ℤ → ℤ ⊗ ℤ/{}ℤ\n", m, m));
    out.push_str(&format!("    = ℤ/{}ℤ →(×{})→ ℤ/{}ℤ\n\n", m, n, m));
    out.push_str(&format!("  The map ×{}: ℤ/{}ℤ → ℤ/{}ℤ,  x ↦ {}x mod {}\n\n", n, m, m, n, m));

    let ker: Vec<i64> = (0..m).filter(|&x| (n * x).rem_euclid(m) == 0).collect();
    let im: std::collections::BTreeSet<i64> = (0..m).map(|x| (n * x).rem_euclid(m)).collect();

    out.push_str(&format!("  Tor₀ = H₀ = coker(×{}) = ℤ/{}ℤ / {{multiples of {} in ℤ/{}ℤ}}\n", n, m, n, m));
    let coker_size = m as usize / im.len();
    s_result(&mut out, "ℤ/nℤ ⊗_ℤ ℤ/mℤ", &format!("ℤ/{}ℤ  (= ℤ/gcd({},{})ℤ)", coker_size, n, m));

    out.push('\n');
    out.push_str(&format!("  Tor₁ = H₁ = ker(×{}) in ℤ/{}ℤ\n", n, m));
    s_result(&mut out, &format!("Tor₁^ℤ(ℤ/{}ℤ, ℤ/{}ℤ)", n, m),
        &format!("ℤ/{}ℤ  (ker has {} elements)", d, ker.len()));

    out.push('\n');
    out.push_str(&format!("  Formula: Tor₁^ℤ(ℤ/nℤ, ℤ/mℤ) ≅ ℤ/gcd(n,m)ℤ = ℤ/{}ℤ\n", d));
    s_note(&mut out, "Note: Tor₁ ≅ Ext¹ for ℤ-modules of this type (both equal ℤ/gcd(n,m)ℤ).");
    s_note(&mut out, "This is a special coincidence for ℤ — not true in general.");
    s_note(&mut out, "Tor measures the failure of the tensor product to be exact (flatness obstruction).");
    out
}

// ── Higher Ext ────────────────────────────────────────────────────────────────

fn cmd_ext_n(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let m = match parse_uint(args, 1, "m") { Some(x) => x as i64, None => return String::new() };
    let k = match parse_uint(args, 2, "k") { Some(x) => x as i64, None => return String::new() };
    let d = gcd(n, m);

    let mut out = String::new();
    s_section(&mut out, &format!("Ext^{}_ℤ(ℤ/{}ℤ, ℤ/{}ℤ)", k, n, m));
    out.push('\n');
    out.push_str("  The ℤ-module ℤ/nℤ has projective dimension 1.\n");
    out.push_str("  So Ext^k = 0 for k ≥ 2 (the resolution has only 2 terms).\n\n");

    match k {
        0 => {
            let ker: Vec<i64> = (0..m).filter(|&x| (n * x).rem_euclid(m) == 0).collect();
            s_result(&mut out, "Ext⁰ = Hom(ℤ/nℤ, ℤ/mℤ)", &format!("ℤ/{}ℤ  (size {})", d, ker.len()));
            s_info(&mut out, "  A homomorphism f: ℤ/nℤ → ℤ/mℤ is determined by f(1) ∈ ℤ/mℤ with n·f(1) = 0.");
            s_info(&mut out, &format!("  Solutions: f(1) ∈ {{x : {}x ≡ 0 mod {}}} = {:?}", n, m, ker));
        },
        1 => {
            s_result(&mut out, "Ext¹_ℤ(ℤ/nℤ, ℤ/mℤ)", &format!("ℤ/{}ℤ  (= ℤ/gcd({},{})ℤ)", d, n, m));
            s_note(&mut out, "This classifies extensions 0 → ℤ/mℤ → E → ℤ/nℤ → 0.");
        },
        _ => {
            s_result(&mut out, &format!("Ext^{}_ℤ(ℤ/nℤ, ℤ/mℤ)", k), "0");
            s_info(&mut out, "  Reason: ℤ/nℤ has projective dimension 1, so all higher Ext vanish.");
            s_info(&mut out, "  The free resolution 0 → ℤ → ℤ → ℤ/nℤ → 0 has length 1.");
        },
    }
    out.push('\n');
    out.push_str(&format!("  Summary of Ext^k_ℤ(ℤ/{}ℤ, ℤ/{}ℤ):\n", n, m));
    out.push_str(&format!("    k=0: ℤ/{}ℤ  (Hom group)\n", d));
    out.push_str(&format!("    k=1: ℤ/{}ℤ  (extension group)\n", d));
    out.push_str("    k≥2: 0\n");
    out
}

// ── Extensions ───────────────────────────────────────────────────────────────

fn cmd_extensions(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let m = match parse_uint(args, 1, "m") { Some(x) => x as i64, None => return String::new() };
    let d = gcd(n, m);

    let mut out = String::new();
    s_section(&mut out, &format!("Extensions: 0 → ℤ/{}ℤ → E → ℤ/{}ℤ → 0", n, m));
    out.push('\n');
    out.push_str(&format!("  Classified by Ext¹_ℤ(ℤ/{}ℤ, ℤ/{}ℤ) ≅ ℤ/{}ℤ.\n", m, n, d));
    out.push_str(&format!("  So there are {} isomorphism classes.\n\n", d));

    let nm = n * m;

    out.push_str(&format!("  The middle group E (order {}) can be:\n", n * m));

    for k in 0..d {
        let (g, c) = if k == 0 {
            (lcm(n, m), n * m / lcm(n, m))
        } else {
            (n * m / gcd(n * m / d * k, n * m), 1)
        };
        let _ = (g, c);
        if k == 0 {
            out.push_str(&format!("  k=0 (split): E ≅ ℤ/{}ℤ ⊕ ℤ/{}ℤ  (direct sum)\n", lcm(n, m) / d, d * d));
            out.push_str(&format!("    Actually: E₀ ≅ ℤ/{}ℤ ⊕ ℤ/{}ℤ  (direct product, split)\n", n, m));
        } else {
            out.push_str(&format!("  k={}: E_k = ℤ/{}ℤ  [extension class k·δ where δ generates Ext¹]\n", k, nm));
        }
    }

    out.push('\n');
    s_note(&mut out, &format!("When d = gcd({},{}) = {}, there are {} distinct extension classes.", n, m, d, d));
    s_note(&mut out, "The split extension (k=0) corresponds to the direct sum ℤ/nℤ ⊕ ℤ/mℤ.");
    if d > 1 {
        s_note(&mut out, &format!("The non-split extensions give ℤ/{}ℤ as the middle group (cyclic).", nm));
    }
    out
}

// ── Group Cohomology ──────────────────────────────────────────────────────────

fn cmd_group_cohomology(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let mut out = String::new();
    if n < 1 { s_err(&mut out, "n must be positive"); return out; }

    s_section(&mut out, &format!("Group Cohomology H^k(ℤ/{}ℤ, ℤ)", n));
    out.push('\n');
    out.push_str(&format!("  G = ℤ/{}ℤ,  coefficients in ℤ (trivial G-action).\n\n", n));
    out.push_str("  Standard periodic resolution of ℤ over ℤ[G]:\n");
    out.push_str("  ... → ℤ[G] →(N)→ ℤ[G] →(t-1)→ ℤ[G] →(N)→ ℤ[G] →(ε)→ ℤ → 0\n");
    out.push_str(&format!("  where t = generator of G, N = 1+t+t²+...+t^{{{}-1}} (norm element)\n\n", n));

    for k in 0..=4 {
        let h = match k {
            0 => format!("ℤ  (fixed points ℤ^G = ℤ, since G acts trivially)"),
            1 => format!("0  (Der(G,ℤ)/Inn = 0; trivial derivations on abelian G)"),
            2 => format!("ℤ/{}ℤ  (extensions of G by ℤ, = Ext¹_{{ℤ[G]}}(ℤ,ℤ))", n),
            3 => format!("0"),
            4 => format!("ℤ/{}ℤ  (periodic: same as H²)", n),
            _ => format!("..."),
        };
        s_result(&mut out, &format!("H^{}(ℤ/{}ℤ, ℤ)", k, n), &h);
    }

    out.push('\n');
    out.push_str("  Pattern: H^k(ℤ/nℤ, ℤ) is periodic of period 2:\n");
    out.push_str("    k=0:         ℤ\n");
    out.push_str("    k odd ≥ 1:  0\n");
    out.push_str(&format!("    k even ≥ 2: ℤ/{}ℤ\n\n", n));
    out.push_str("  This periodicity follows from the periodic resolution of ℤ over ℤ[ℤ/nℤ].\n");
    s_note(&mut out, &format!("H²(G,ℤ) ≅ Hom(G, ℚ/ℤ) ≅ G for finite abelian G (Pontryagin dual)."));
    s_note(&mut out, &format!("For G = ℤ/{}ℤ: Hom(ℤ/{}ℤ, ℚ/ℤ) ≅ ℤ/{}ℤ ✓", n, n, n));
    out
}

// ── Long Exact Ext Sequence ───────────────────────────────────────────────────

fn cmd_long_exact(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let m = match parse_uint(args, 1, "m") { Some(x) => x as i64, None => return String::new() };
    let k = match parse_uint(args, 2, "k") { Some(x) => x as i64, None => return String::new() };
    let mn = m * n;
    let d_nk = gcd(n, k);
    let d_mnk = gcd(mn, k);
    let d_mk = gcd(m, k);

    let mut out = String::new();
    s_section(&mut out, &format!("Long Exact Ext Sequence: n={}, m={}, k={}", n, m, k));
    out.push('\n');
    out.push_str("  Short exact sequence:\n");
    out.push_str(&format!("  0 → ℤ/{}ℤ →(×{})→ ℤ/{}ℤ →(mod {})→ ℤ/{}ℤ → 0\n\n", n, m, mn, n, m));
    out.push_str(&format!("  Apply Ext_ℤ(−, ℤ/{}ℤ) to get long exact sequence:\n\n", k));
    out.push_str("  0\n");
    out.push_str(&format!("  → Hom(ℤ/{}ℤ, ℤ/{}ℤ)\n", m, k));
    out.push_str(&format!("  → Hom(ℤ/{}ℤ, ℤ/{}ℤ)\n", mn, k));
    out.push_str(&format!("  → Hom(ℤ/{}ℤ, ℤ/{}ℤ)\n", n, k));
    out.push_str("  →∂→\n");
    out.push_str(&format!("  Ext¹(ℤ/{}ℤ, ℤ/{}ℤ)\n", m, k));
    out.push_str(&format!("  → Ext¹(ℤ/{}ℤ, ℤ/{}ℤ)\n", mn, k));
    out.push_str(&format!("  → Ext¹(ℤ/{}ℤ, ℤ/{}ℤ)\n", n, k));
    out.push_str("  → 0\n\n");
    out.push_str("  Computing each term:\n");
    s_result(&mut out, &format!("Hom(ℤ/{}ℤ, ℤ/{}ℤ)", m, k), &format!("ℤ/{}ℤ", d_mk));
    s_result(&mut out, &format!("Hom(ℤ/{}ℤ, ℤ/{}ℤ)", mn, k), &format!("ℤ/{}ℤ", d_mnk));
    s_result(&mut out, &format!("Hom(ℤ/{}ℤ, ℤ/{}ℤ)", n, k), &format!("ℤ/{}ℤ", d_nk));
    s_result(&mut out, &format!("Ext¹(ℤ/{}ℤ, ℤ/{}ℤ)", m, k), &format!("ℤ/{}ℤ", d_mk));
    s_result(&mut out, &format!("Ext¹(ℤ/{}ℤ, ℤ/{}ℤ)", mn, k), &format!("ℤ/{}ℤ", d_mnk));
    s_result(&mut out, &format!("Ext¹(ℤ/{}ℤ, ℤ/{}ℤ)", n, k), &format!("ℤ/{}ℤ", d_nk));
    out.push('\n');
    s_note(&mut out, "The connecting homomorphism ∂: Hom(ℤ/nℤ, ℤ/kℤ) → Ext¹(ℤ/mℤ, ℤ/kℤ)");
    s_info(&mut out, "  sends a homomorphism f: ℤ/nℤ → ℤ/kℤ to the extension class");
    s_info(&mut out, "  obtained by pulling back the extension along f.");
    out
}

// ── Flatness Test ─────────────────────────────────────────────────────────────

fn cmd_flat_test(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let mut out = String::new();
    if n < 2 { s_err(&mut out, "n must be at least 2"); return out; }

    s_section(&mut out, &format!("Flatness Test for ℤ/{}ℤ", n));
    out.push('\n');
    out.push_str("  A ℤ-module M is flat iff Tor₁^ℤ(M, N) = 0 for all N.\n\n");

    let factors = factor(n as u64);
    out.push_str(&format!("  Testing M = ℤ/{}ℤ:\n", n));
    out.push_str(&format!("  We compute Tor₁^ℤ(ℤ/{}ℤ, N) for specific N:\n\n", n));

    for (p, _) in &factors {
        let p = *p as i64;
        let d = gcd(n, p);
        out.push_str(&format!("  N = ℤ/{}ℤ: Tor₁(ℤ/{}ℤ, ℤ/{}ℤ) = ℤ/gcd({},{})ℤ = ℤ/{}ℤ\n", p, n, p, n, p, d));
        if d > 1 {
            s_err(&mut out, &format!("  Tor₁ ≠ 0 (= ℤ/{}ℤ) — ℤ/{}ℤ is NOT flat!", d, n));
        }
    }

    out.push('\n');
    s_result(&mut out, &format!("Is ℤ/{}ℤ flat over ℤ?", n), "NO");
    out.push('\n');
    out.push_str(&format!("  Reason: Tor₁^ℤ(ℤ/{}ℤ, ℤ/nℤ) = ℤ/{}ℤ ≠ 0.\n\n", n, n));
    out.push_str("  Alternative proof via exactness:\n");
    out.push_str(&format!("  Tensor  0 → ℤ →(×{})→ ℤ  with ℤ/{}ℤ:\n", n, n));
    out.push_str(&format!("  Get:    ℤ/{}ℤ →(×{})→ ℤ/{}ℤ\n", n, n, n));
    out.push_str(&format!("  But ×{}: ℤ/{}ℤ → ℤ/{}ℤ sends everything to 0!\n", n, n, n));
    out.push_str(&format!("  The map is the zero map, but the original ×{} was injective.\n", n));
    out.push_str(&format!("  So tensoring destroyed injectivity — ℤ/{}ℤ is not flat.\n\n", n));
    s_note(&mut out, "Flat ℤ-modules are exactly the torsion-free ℤ-modules.");
    s_note(&mut out, "ℤ, ℚ, and any free ℤ-module are flat. ℤ/nℤ has torsion, hence is not flat.");
    out
}

fn cmd_demo() -> String {
    let mut out = String::new();
    s_section(&mut out, "Demo: Ext and Tor Showcase");
    out.push('\n');
    out.push_str("  --- Ext¹(ℤ/6ℤ, ℤ/4ℤ) ---\n");
    out.push_str(&cmd_ext(&["6", "4"]));
    out.push('\n');
    out.push_str("  --- Tor₁(ℤ/6ℤ, ℤ/4ℤ) ---\n");
    out.push_str(&cmd_tor(&["6", "4"]));
    out.push('\n');
    out.push_str("  --- Group cohomology H*(ℤ/5ℤ, ℤ) ---\n");
    out.push_str(&cmd_group_cohomology(&["5"]));
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
        "ext"              => cmd_ext(args),
        "tor"              => cmd_tor(args),
        "ext_n"            => cmd_ext_n(args),
        "extensions"       => cmd_extensions(args),
        "group_cohomology" => cmd_group_cohomology(args),
        "long_exact"       => cmd_long_exact(args),
        "flat_test"        => cmd_flat_test(args),
        "demo"             => cmd_demo(),
        "help" | "h"       => show_help(),
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
    c.text_bold(350.0, 30.0, "Long Exact Ext/Tor Sequence", 16.0, "#222", "middle");
    let y1 = 120.0;
    let y2 = 220.0;
    let y3 = 320.0;
    c.text(20.0, y1, "0 →", 13.0, "#333", "start");
    c.node_circle(120.0, y1, "ℤ/nℤ", "white", 28.0, 11.0);
    c.text(155.0, y1, "→", 13.0, "#333", "middle");
    c.node_circle(220.0, y1, "ℤ/mnℤ", "white", 32.0, 10.0);
    c.text(260.0, y1, "→", 13.0, "#333", "middle");
    c.node_circle(330.0, y1, "ℤ/mℤ", "white", 28.0, 11.0);
    c.text(365.0, y1, "→  0", 13.0, "#333", "start");
    c.text(20.0, y2, "0 → Hom(ℤ/mℤ,k) → Hom(ℤ/mnℤ,k) → Hom(ℤ/nℤ,k)", 12.0, "#333", "start");
    c.arrow(600.0, y2, 600.0, y3 - 20.0, "#444", 1.5);
    c.text(610.0, (y2 + y3) / 2.0, "∂", 13.0, "#333", "start");
    c.text(20.0, y3, "  Ext¹(ℤ/mℤ,k) → Ext¹(ℤ/mnℤ,k) → Ext¹(ℤ/nℤ,k) → 0", 12.0, "#333", "start");
    c.text(150.0, 420.0, "Ext¹_ℤ(ℤ/nℤ, ℤ/mℤ) ≅ ℤ/gcd(n,m)ℤ", 12.0, "#333", "start");
    c.text(150.0, 460.0, "Tor₁_ℤ(ℤ/nℤ, ℤ/mℤ) ≅ ℤ/gcd(n,m)ℤ", 12.0, "#333", "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("ext0", &[("label", "Ext⁰"), ("shape", "box")]);
    g.node("ext1", &[("label", "Ext¹"), ("shape", "box")]);
    g.node("tor0", &[("label", "Tor₀"), ("shape", "box")]);
    g.node("tor1", &[("label", "Tor₁"), ("shape", "box")]);
    g.edge("ext0", "ext1", &[("label", "derived")]);
    g.edge("tor0", "tor1", &[("label", "derived")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("hom", 0.0, 0.0, "$\\text{Hom}$", "");
    t.node("ext1", 3.0, 0.0, "$\\text{Ext}^1$", "");
    t.node("ext2", 6.0, 0.0, "$\\text{Ext}^2=0$", "");
    t.node("tor0", 0.0, -2.0, "$\\text{Tor}_0$", "");
    t.node("tor1", 3.0, -2.0, "$\\text{Tor}_1$", "");
    t.arrow("hom", "ext1", "$\\partial$", "above");
    t.arrow("ext1", "ext2", "", "above");
    t.arrow("tor0", "tor1", "$\\partial$", "above");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 1, "Ext/Tor Long Exact Sequence");
    a.hline(0, 40, 2, '-');
    a.text_at(0, 4, "Short exact:  0 -> A -> B -> C -> 0");
    a.text_at(0, 6, "Long exact Ext:");
    a.text_at(0, 7, "0 -> Hom(C,k) -> Hom(B,k) -> Hom(A,k) -d-> Ext1(C,k) -> ...");
    a.hline(0, 60, 9, '-');
    a.text_at(0, 11, "Ext1_Z(Z/nZ, Z/mZ) = Z/gcd(n,m)Z");
    a.text_at(0, 12, "Tor1_Z(Z/nZ, Z/mZ) = Z/gcd(n,m)Z");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch40"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 40", "Ext and Tor", "Derived functors: measuring failure of exactness");
            print!("{}", show_help());
            print_note("Try: ext 6 4   or   tor 6 4   or   extensions 2 4");
            print_note("Or: group_cohomology 5   or   long_exact 2 3 6");
            repl("ch40> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
