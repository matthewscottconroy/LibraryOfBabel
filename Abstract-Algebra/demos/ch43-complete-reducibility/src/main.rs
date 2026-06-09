use common::*;

fn show_help() -> String {
    let mut out = String::new();
    let rows = &[
        ("maschke <n> <char_p>",        "Maschke's theorem: char_p ∤ n iff completely reducible"),
        ("group_algebra <n>",           "ℂ[ℤ/nℤ] ≅ ℂⁿ decomposition"),
        ("artin_wedderburn <type> <n>", "Artin-Wedderburn decomposition (Z, S3, S4)"),
        ("dimension_formula <type> <n>","verify Σ(dim Vᵢ)² = |G|"),
        ("regular_decomp <n>",          "regular rep of ℤ/nℤ decomposed"),
        ("idempotent <n>",              "central idempotents of ℂ[ℤ/nℤ]"),
        ("modular_failure <p>",         "Maschke fails for ℤ/pℤ over 𝔽_p"),
        ("demo",                        "showcase of complete reducibility"),
        ("help",                        "show this help"),
        ("quit",                        "exit"),
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

// ── Maschke's theorem ─────────────────────────────────────────────────────────

fn cmd_maschke(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return out };
    let p = match parse_uint(args, 1, "char_p") { Some(x) => x as i64, None => return out };

    s_section(&mut out, &format!("Maschke's Theorem: G = ℤ/{}ℤ over field of char {}", n, p));
    out.push('\n');
    out.push_str("  Theorem (Maschke): The group algebra k[G] is semisimple (every\n");
    out.push_str("  representation is completely reducible) iff char(k) does not divide |G|.\n");
    out.push('\n');

    let divides = p > 0 && n % p == 0;
    let applies = p == 0 || !divides;

    if applies {
        s_ok(&mut out, &format!("char(k) = {} does NOT divide |G| = {} — Maschke APPLIES.", p, n));
        out.push('\n');
        out.push_str("  Averaging construction (Weyl's unitary trick):\n");
        out.push_str("  Given any subrepresentation W ⊆ V, we construct a G-equivariant\n");
        out.push_str("  projection P: V → W by averaging:\n");
        out.push_str("  P = (1/|G|) · Σ_{g ∈ G} g·P₀·g⁻¹\n");
        out.push_str("  where P₀: V → W is any linear projection.\n");
        out.push('\n');
        out.push_str(&format!("  Concrete example for G = ℤ/{}ℤ, any rep V:\n", n));
        out.push_str(&format!("  P = (1/{}) Σ_{{j=0}}^{{{}}} ρ(j) · P₀ · ρ(j)⁻¹\n", n, n-1));
        out.push('\n');
        s_ok(&mut out, "P is G-equivariant and P² = P — a valid complement to W.");
        s_note(&mut out, "The complement W' = ker(P) satisfies V = W ⊕ W' (G-stable).");
    } else {
        s_err(&mut out, &format!("char(k) = {} DIVIDES |G| = {} — Maschke does NOT apply!", p, n));
        out.push('\n');
        out.push_str(&format!("  Counterexample in characteristic p = {}:\n", p));
        out.push_str(&format!("  V = 𝔽_p² with G = ℤ/{}ℤ acting via:\n", p));
        out.push_str("  ρ(1) = [[1, 1], [0, 1]]  (Jordan block, unipotent)\n");
        out.push('\n');
        out.push_str("  The subspace W = span(e₁) = {(a,0) : a ∈ 𝔽_p} is G-stable:\n");
        out.push_str("  ρ(1)·(a,0)^T = (a, 0)^T ✓\n");
        out.push('\n');
        out.push_str("  But V has NO G-stable complement to W:\n");
        out.push_str("  Any line L = span(a,b) with b≠0 would need ρ(1)·(a,b) = (a+b, b) ∈ L.\n");
        out.push_str("  This requires (a+b)/b = a/b, i.e., 1 = 0 in 𝔽_p. Contradiction!\n");
        out.push('\n');
        s_err(&mut out, "V is indecomposable but not irreducible — completely reducibility fails.");
        s_note(&mut out, &format!("In char {}, the group algebra 𝔽_{}[ℤ/{}ℤ] ≅ 𝔽_{}[x]/(x^{}-1) = 𝔽_{}[x]/(x-1)^{}",
            p, p, p, p, p, p, p));
        s_note(&mut out, "This ring is local (only one irrep), not semisimple.");
    }
    out
}

// ── Group algebra decomposition ───────────────────────────────────────────────

fn cmd_group_algebra(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return out };

    s_section(&mut out, &format!("ℂ[ℤ/{}ℤ] — Group Algebra Decomposition", n));
    out.push('\n');
    out.push_str(&format!("  The group algebra ℂ[G] for G = ℤ/{}ℤ has dimension |G| = {}.\n", n, n));
    out.push('\n');
    out.push_str("  By the DFT (Discrete Fourier Transform) / Artin-Wedderburn:\n");
    out.push_str(&format!("  ℂ[ℤ/{}ℤ] ≅ ℂ × ℂ × ... × ℂ  ({} copies)\n", n, n));
    out.push('\n');
    out.push_str(&format!("  More precisely: ℂ[ℤ/{}ℤ] ≅ ⊕_{{k=0}}^{{{}}} ℂ·e_k\n", n, n-1));
    out.push_str(&format!("  where e_k = (1/{}) Σ_j ω^{{-jk}} g^j  (Fourier basis / central idempotents)\n", n));
    out.push('\n');
    out.push_str(&format!("  ω = e^{{2πi/{}}}  (primitive n-th root of unity)\n", n));
    out.push('\n');

    let omega_str = format!("e^{{2πi/{}}}", n);
    out.push_str(&format!("  Isomorphism: ℂ[G] → ℂⁿ,  Σ a_j g^j  ↦  (â₀, â₁, ..., â_{{{}}})\n", n-1));
    out.push_str("  where â_k = Σ_j a_j ω^{jk} = (DFT of a)_k\n");
    out.push('\n');
    out.push_str("  Each factor ℂ = ℂ·e_k corresponds to irrep ρ_k.\n");
    out.push_str("  The projection onto the k-th factor is the Fourier coefficient â_k.\n");
    out.push('\n');
    out.push_str("  Example: g = e₁ (group generator, j=1 basis element)\n");
    out.push_str(&format!("  DFT: ĝ_k = ω^k = {} for k=0,...,{}\n", omega_str, n-1));
    for k in 0..n.min(6) {
        let zeta = 2.0 * PI * k as f64 / n as f64;
        let re = (zeta.cos() * 1000.0).round() / 1000.0;
        let im = (zeta.sin() * 1000.0).round() / 1000.0;
        out.push_str(&format!("    ĝ_{} = {:>7.3} + {:>7.3}i\n", k, re, im));
    }
    if n > 6 { out.push_str("    ...\n"); }
    out.push('\n');
    s_note(&mut out, "The map ℂ[G] → ℂⁿ is the Fourier transform on G.");
    s_note(&mut out, "Convolution in ℂ[G] becomes pointwise multiplication in ℂⁿ.");
    out
}

// ── Artin-Wedderburn ──────────────────────────────────────────────────────────

fn cmd_artin_wedderburn(args: &[&str]) -> String {
    let mut out = String::new();
    let group_type = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Type needed: Z, S3, S4"); return out; } };
    let n = match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out };

    s_section(&mut out, "Artin-Wedderburn Decomposition");
    out.push('\n');
    out.push_str("  Theorem: For a finite group G over ℂ (or any alg. closed field of char 0),\n");
    out.push_str("  ℂ[G] ≅ ⊕_i M_{nᵢ}(ℂ)\n");
    out.push_str("  where the sum is over irreducible representations Vᵢ and nᵢ = dim Vᵢ.\n");
    out.push('\n');

    match group_type {
        "Z" => {
            out.push_str(&format!("  G = ℤ/{}ℤ,  |G| = {}.\n", n, n));
            out.push_str(&format!("  All irreps are 1-dimensional: nᵢ = 1 for all i = 0,...,{}\n", n-1));
            out.push_str(&format!("  ℂ[ℤ/{}ℤ] ≅ ⊕_{{k=0}}^{{{}}} M₁(ℂ) = ℂ^{}\n", n, n-1, n));
            let sum_sq: usize = n;
            s_result(&mut out, "Σ nᵢ²", &format!("{} · 1² = {} = |G| ✓", n, sum_sq));
        },
        "S3" if n == 3 => {
            out.push_str("  G = S₃,  |G| = 6.\n");
            out.push_str("  Irreps: trivial (dim 1), sign (dim 1), standard (dim 2).\n");
            out.push_str("  ℂ[S₃] ≅ M₁(ℂ) ⊕ M₁(ℂ) ⊕ M₂(ℂ)\n");
            out.push_str("  Dimensions: 1 + 1 + 4 = 6 ✓\n");
            s_result(&mut out, "Σ nᵢ²", "1 + 1 + 4 = 6 = |S₃| ✓");
        },
        "S4" if n == 4 => {
            out.push_str("  G = S₄,  |G| = 24.\n");
            out.push_str("  Irreps: 1, 1, 2, 3, 3 (by dimension).\n");
            out.push_str("  ℂ[S₄] ≅ M₁(ℂ) ⊕ M₁(ℂ) ⊕ M₂(ℂ) ⊕ M₃(ℂ) ⊕ M₃(ℂ)\n");
            out.push_str("  Dimensions: 1+1+4+9+9 = 24 ✓\n");
            s_result(&mut out, "Σ nᵢ²", "1 + 1 + 4 + 9 + 9 = 24 = |S₄| ✓");
        },
        _ => {
            s_err(&mut out, "Use: artin_wedderburn Z <n>,  artin_wedderburn S3 3,  artin_wedderburn S4 4");
            return out;
        }
    }

    out.push('\n');
    s_note(&mut out, "Each summand M_{nᵢ}(ℂ) corresponds to the 'matrix algebra' of one irrep.");
    s_note(&mut out, "The primitive central idempotent projecting onto M_{nᵢ}(ℂ) is");
    s_note(&mut out, "  eᵢ = (dim Vᵢ / |G|) · Σ_{g∈G} χᵢ(g⁻¹)·g  in ℂ[G].");
    out
}

// ── Dimension formula ─────────────────────────────────────────────────────────

fn cmd_dimension_formula(args: &[&str]) -> String {
    let mut out = String::new();
    let group_type = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Type needed"); return out; } };
    let n = match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out };

    s_section(&mut out, "Dimension Formula: Σ (dim Vᵢ)² = |G|");
    out.push('\n');

    let (order, dims, name): (usize, Vec<usize>, String) = match group_type {
        "Z" => (n, vec![1; n], format!("ℤ/{}ℤ", n)),
        "S" => match n {
            2 => (2, vec![1, 1], "S₂".to_string()),
            3 => (6, vec![1, 1, 2], "S₃".to_string()),
            4 => (24, vec![1, 1, 2, 3, 3], "S₄".to_string()),
            _ => { s_err(&mut out, "S_n for n=2,3,4 only"); return out; }
        },
        "D" if n >= 3 => {
            let ord = 2 * n;
            let dims = if n % 2 == 1 {
                let mut d = vec![1usize, 1];
                d.extend(vec![2usize; (n-1)/2]);
                d
            } else {
                let mut d = vec![1usize, 1, 1, 1];
                d.extend(vec![2usize; (n-2)/2]);
                d
            };
            (ord, dims, format!("D_{}", n))
        },
        _ => { s_err(&mut out, "Type: Z, S (n=2,3,4), D (n≥3)"); return out; }
    };

    out.push_str(&format!("  G = {},  |G| = {}\n", name, order));
    out.push_str("  Irreducible representations:\n");
    for (i, &d) in dims.iter().enumerate() {
        out.push_str(&format!("    Vᵢ_{}: dim = {}\n", i+1, d));
    }
    out.push('\n');
    let sum_sq: usize = dims.iter().map(|&d| d*d).sum();
    let formula = dims.iter().map(|&d| format!("{}²", d)).collect::<Vec<_>>().join(" + ");
    out.push_str(&format!("  Σ (dim Vᵢ)² = {} = {}\n", formula, sum_sq));
    if sum_sq == order {
        s_ok(&mut out, &format!("{} = |G| ✓", sum_sq));
    } else {
        s_err(&mut out, &format!("{} ≠ |G| = {} — something is wrong", sum_sq, order));
    }
    out.push('\n');
    s_note(&mut out, "This identity is equivalent to the completeness of the character table.");
    s_note(&mut out, "It also follows from: dim ℂ[G] = |G| = Σ (dim Vᵢ)².");
    out
}

// ── Regular decomposition ─────────────────────────────────────────────────────

fn cmd_regular_decomp(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return out };

    s_section(&mut out, &format!("Regular Representation of ℤ/{}ℤ: Decomposition", n));
    out.push('\n');
    out.push_str("  The regular representation R = ℂ[G] (G acting on itself by multiplication).\n");
    out.push_str(&format!("  dim R = |G| = {}.\n", n));
    out.push('\n');
    out.push_str("  By Peter-Weyl for finite groups:\n");
    out.push_str("  R ≅ ⊕_i (dim Vᵢ) · Vᵢ\n");
    out.push_str("  (each irrep Vᵢ appears with multiplicity dim Vᵢ)\n");
    out.push('\n');
    out.push_str(&format!("  For G = ℤ/{}ℤ: all irreps have dim = 1.\n", n));
    out.push_str(&format!("  R ≅ ρ₀ ⊕ ρ₁ ⊕ ... ⊕ ρ_{{{}}}  (each appears with multiplicity 1)\n", n-1));
    out.push('\n');
    out.push_str("  Verification:\n");
    out.push_str(&format!("  dim R = {} = {}\n", n, (0..n).map(|_| 1).sum::<usize>()));
    out.push_str("  Each ρ_k contributes 1 to dim R. ✓\n");
    out.push('\n');
    out.push_str("  The character of R:\n");
    out.push_str(&format!("  χ_R(0) = {} = |G|\n", n));
    for j in 1..n.min(6) {
        out.push_str(&format!("  χ_R({}) = 0  (sum of all n-th roots of unity)\n", j));
    }
    if n > 6 { out.push_str("  ...\n"); }
    out.push('\n');
    s_note(&mut out, "The decomposition of R is essentially the Discrete Fourier Transform on G.");
    s_note(&mut out, "The Fourier basis {e_k} are the projections onto each ρ_k component.");
    out
}

// ── Central idempotents ───────────────────────────────────────────────────────

fn cmd_idempotent(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return out };

    s_section(&mut out, &format!("Central Idempotents of ℂ[ℤ/{}ℤ]", n));
    out.push('\n');
    out.push_str(&format!("  For G = ℤ/{}ℤ and ω = e^{{2πi/{}}}, the central idempotent for ρ_k is:\n", n, n));
    out.push('\n');
    out.push_str(&format!("  e_k = (1/{}) Σ_{{j=0}}^{{{}}} ω^{{-jk}} g^j  ∈ ℂ[G]\n", n, n-1));
    out.push('\n');
    out.push_str("  Properties:\n");
    out.push_str("  • e_k² = e_k   (idempotent)\n");
    out.push_str("  • e_k · e_l = 0  if k ≠ l  (orthogonal)\n");
    out.push_str("  • Σ_k e_k = 1   (completeness)\n");
    out.push_str("  • ρ_m(e_k) = δ_{mk} · 1  (projects onto the k-th eigenspace)\n");
    out.push('\n');
    out.push_str("  Explicit elements (as linear combinations of group basis {g^j}):\n");
    out.push('\n');

    let omega = 2.0 * PI / n as f64;
    for k in 0..n.min(4) {
        out.push_str(&format!("  e_{}: (1/{}) · [", k, n));
        for j in 0..n {
            let angle = -(k as f64) * (j as f64) * omega;
            let re = (angle.cos() * 100.0).round() / 100.0;
            let im = (angle.sin() * 100.0).round() / 100.0;
            if j > 0 { out.push_str(" + "); }
            if im.abs() < 0.01 {
                out.push_str(&format!("{:.2}·g^{}", re, j));
            } else {
                out.push_str(&format!("({:.2}+{:.2}i)·g^{}", re, im, j));
            }
        }
        out.push_str("]\n");
    }
    if n > 4 { out.push_str("  ...\n"); }
    out.push('\n');
    s_note(&mut out, "These are the 'Fourier basis elements' of ℂ[G].");
    s_note(&mut out, "They diagonalize the group algebra and implement the Fourier transform.");
    s_note(&mut out, "In signal processing: e_k corresponds to the k-th frequency component.");
    out
}

// ── Modular failure ───────────────────────────────────────────────────────────

fn cmd_modular_failure(args: &[&str]) -> String {
    let mut out = String::new();
    let p = match parse_uint(args, 0, "p") { Some(x) => x as i64, None => return out };
    if !is_prime(p as u64) { s_err(&mut out, "p must be prime"); return out; }

    s_section(&mut out, &format!("Maschke's Theorem Fails for ℤ/{}ℤ over 𝔽_{}", p, p));
    out.push('\n');
    out.push_str(&format!("  Setting: G = ℤ/{}ℤ,  k = 𝔽_{},  k[G] = 𝔽_{}[ℤ/{}ℤ].\n", p, p, p, p));
    out.push_str(&format!("  char(k) = {} = |G|,  so Maschke fails.\n", p));
    out.push('\n');
    out.push_str("  The group algebra:\n");
    out.push_str(&format!("  𝔽_{}[ℤ/{}ℤ] ≅ 𝔽_{}[x]/(x^{}-1)\n", p, p, p, p));
    out.push_str("  In 𝔽_p: x^p - 1 = (x-1)^p   (Frobenius endomorphism).\n");
    out.push_str(&format!("  So: 𝔽_{}[ℤ/{}ℤ] ≅ 𝔽_{}[x]/(x-1)^{}\n", p, p, p, p));
    out.push('\n');
    out.push_str("  This is a LOCAL ring (unique maximal ideal = (x-1)).\n");
    out.push_str("  A local ring has only ONE simple module: the residue field 𝔽_p.\n");
    out.push('\n');
    out.push_str("  The indecomposable 𝔽_p[G]-modules are:\n");
    for k in 1..=p as usize {
        out.push_str(&format!("    V_{}: 𝔽_p[x]/(x-1)^{}  (dim = {})\n", k, k, k));
    }
    out.push('\n');
    out.push_str("  Explicit indecomposable extension (the Jordan block):\n");
    out.push_str("  V₂ = 𝔽_p² with G-action via ρ(g) = [[1,1],[0,1]]\n");
    out.push_str("  Submodule: W = span(e₁) ≅ V₁ = 𝔽_p  (the trivial module)\n");
    out.push_str("  V₂/W ≅ V₁\n");
    out.push_str("  The short exact sequence 0 → V₁ → V₂ → V₁ → 0 does NOT split.\n");
    out.push('\n');
    s_err(&mut out, "V₂ is NOT a direct sum of simple modules — completely reducibility fails!");
    out.push('\n');
    s_note(&mut out, &format!("The extension class is Ext¹_{{k[G]}}(V₁, V₁) ≅ 𝔽_p ≠ 0."));
    s_note(&mut out, "This is the fundamental difficulty of modular representation theory.");
    s_note(&mut out, "Modular reps of G are studied via block theory, Brauer characters, etc.");
    out
}

fn cmd_demo() -> String {
    let mut out = String::new();
    s_section(&mut out, "Demo: Complete Reducibility Showcase");
    out.push('\n');
    out.push_str("  --- Maschke's theorem for G=ℤ/6ℤ over char 0 ---\n");
    out.push_str(&cmd_maschke(&["6", "0"]));
    out.push('\n');
    out.push_str("  --- Maschke fails: G=ℤ/3ℤ over char 3 ---\n");
    out.push_str(&cmd_maschke(&["3", "3"]));
    out.push('\n');
    out.push_str("  --- Artin-Wedderburn for S₃ ---\n");
    out.push_str(&cmd_artin_wedderburn(&["S3", "3"]));
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
        "maschke"            => cmd_maschke(args),
        "group_algebra"      => cmd_group_algebra(args),
        "artin_wedderburn"   => cmd_artin_wedderburn(args),
        "dimension_formula"  => cmd_dimension_formula(args),
        "regular_decomp"     => cmd_regular_decomp(args),
        "idempotent"         => cmd_idempotent(args),
        "modular_failure"    => cmd_modular_failure(args),
        "demo"               => cmd_demo(),
        "help" | "h"         => show_help(),
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
    // Character table display for S₃
    c.text_bold(350.0, 30.0, "Character Table of S₃", 16.0, "#222", "middle");
    let data = vec![
        ("", "e", "(12)", "(123)"),
        ("χ₁", "1", "1", "1"),
        ("χ₂", "1", "-1", "1"),
        ("χ₃", "2", "0", "-1"),
    ];
    let col_w = 100.0;
    let row_h = 40.0;
    let ox = 100.0;
    let oy = 80.0;
    for (ri, row) in data.iter().enumerate() {
        let y = oy + ri as f64 * row_h;
        let vals = [row.0, row.1, row.2, row.3];
        for (ci, val) in vals.iter().enumerate() {
            let x = ox + ci as f64 * col_w;
            if ri == 0 || ci == 0 {
                c.text_bold(x + 10.0, y + 25.0, val, 13.0, "#222", "start");
            } else {
                c.text(x + 10.0, y + 25.0, val, 13.0, "#333", "start");
            }
            c.rect(x, y, col_w - 2.0, row_h - 2.0, "white", "#aaa", 0.5);
        }
    }
    // Artin-Wedderburn
    c.text(100.0, 280.0, "ℂ[S₃] ≅ M₁(ℂ) ⊕ M₁(ℂ) ⊕ M₂(ℂ)", 12.0, "#333", "start");
    c.text(100.0, 310.0, "dim: 1 + 1 + 4 = 6 = |S₃|", 12.0, "#333", "start");
    c.text(100.0, 360.0, "Maschke: char(k) ∤ |G| ⟹ completely reducible", 12.0, "#333", "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("kG", &[("label", "k[G]"), ("shape", "box")]);
    g.node("V1", &[("label", "V₁"), ("shape", "ellipse")]);
    g.node("V2", &[("label", "V₂"), ("shape", "ellipse")]);
    g.node("Vn", &[("label", "Vₙ"), ("shape", "ellipse")]);
    g.edge("kG", "V1", &[("label", "")]);
    g.edge("kG", "V2", &[("label", "")]);
    g.edge("kG", "Vn", &[("label", "")]);
    g.node("AW", &[("label", "Artin-Wedderburn"), ("shape", "plaintext")]);
    g.edge("V1", "AW", &[("label", "M_{d₁}(ℂ)")]);
    g.edge("V2", "AW", &[("label", "M_{d₂}(ℂ)")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("G", 0.0, 0.0, "$G$", "");
    t.node("kG", 3.0, 0.0, "$k[G]$", "");
    t.node("aw", 6.0, 0.0, "$\\bigoplus_i M_{n_i}(k)$", "");
    t.arrow("G", "kG", "group algebra", "above");
    t.arrow("kG", "aw", "Artin-Wedderburn", "above");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 1, "Complete Reducibility (Maschke's Theorem)");
    a.hline(0, 45, 2, '-');
    a.text_at(0, 4, "char(k) does not divide |G|  ==>  k[G] semisimple");
    a.text_at(0, 5, "Every rep is direct sum of irreducibles");
    a.hline(0, 55, 7, '-');
    a.text_at(0, 9, "Artin-Wedderburn:  k[G] ~= M_{n1}(k) + M_{n2}(k) + ...");
    a.text_at(0, 10, "S3: k[S3] ~= M1(k) + M1(k) + M2(k),  1+1+4=6=|S3|");
    a.text_at(0, 12, "Modular (char p | |G|): fails — indecomp != irreducible");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch43"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 43", "Complete Reducibility", "Maschke's theorem, group algebras, and Artin-Wedderburn");
            print!("{}", show_help());
            print_note("Try: maschke 6 0   or   maschke 3 3   or   group_algebra 4");
            print_note("Or: artin_wedderburn S3 3   or   modular_failure 5");
            repl("ch43> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
