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
        ("exact <n1> <f1> <n2> <f2> <n3>",      "check exactness of 0→ℤ/n1→ℤ/n2→ℤ/n3→0"),
        ("snake <A1> <A2> <A3> <B1> <B2> <B3>", "snake lemma in ℤ/nℤ (connecting homomorphism)"),
        ("kernel_cokernel <n> <m> <a>",          "ker and coker of f:ℤ/nℤ→ℤ/mℤ, x↦ax"),
        ("five_lemma <n>",                       "demonstrate the five lemma with ℤ/nℤ"),
        ("short_exact <n> <k>",                  "display 0→kℤ/nℤ→ℤ/nℤ→ℤ/kℤ→0"),
        ("split <n> <k>",                        "check if 0→ℤ/kℤ→ℤ/nkℤ→ℤ/nℤ→0 splits"),
        ("embedding",                            "Freyd-Mitchell embedding theorem"),
        ("demo",                                 "showcase of abelian category concepts"),
        ("help",                                 "show this help"),
        ("quit",                                 "exit"),
    ];
    let mut out = String::new();
    out.push('\n');
    out.push_str(&format!("  \x1b[1mCommands:\x1b[0m\n"));
    let max = commands.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
    for (cmd, desc) in commands {
        out.push_str(&format!("    {:<width$}  \x1b[2m{}\x1b[0m\n",
            format!("\x1b[36m{}\x1b[0m", cmd), desc, width = max + 2 + 9));
    }
    out.push('\n');
    out
}

// ── Exact sequence check ──────────────────────────────────────────────────────

fn cmd_exact(args: &[&str]) -> String {
    let n1 = match parse_uint(args, 0, "n1") { Some(x) => x as i64, None => return String::new() };
    let f1 = match parse_int(args, 1, "f1") { Some(x) => x, None => return String::new() };
    let n2 = match parse_uint(args, 2, "n2") { Some(x) => x as i64, None => return String::new() };
    let f2 = match parse_int(args, 3, "f2") { Some(x) => x, None => return String::new() };
    let n3 = match parse_uint(args, 4, "n3") { Some(x) => x as i64, None => return String::new() };

    let mut out = String::new();
    s_section(&mut out, "Short Exact Sequence Check");
    out.push_str(&format!("  0 → ℤ/{}ℤ →(×{})→ ℤ/{}ℤ →(×{})→ ℤ/{}ℤ → 0\n", n1, f1, n2, f2, n3));
    s_sep(&mut out);

    let im_f1: Vec<i64> = (0..n1).map(|x| (x * f1).rem_euclid(n2)).collect();
    let im_f1_set: std::collections::BTreeSet<i64> = im_f1.iter().copied().collect();

    let ker_f2: Vec<i64> = (0..n2).filter(|&x| (x * f2).rem_euclid(n3) == 0).collect();
    let ker_f2_set: std::collections::BTreeSet<i64> = ker_f2.iter().copied().collect();

    let exact_mid = im_f1_set == ker_f2_set;

    let ker_f1: Vec<i64> = (0..n1).filter(|&x| (x * f1).rem_euclid(n2) == 0).collect();
    let injective = ker_f1 == vec![0];

    let im_f2: std::collections::BTreeSet<i64> = (0..n2).map(|x| (x * f2).rem_euclid(n3)).collect();
    let surjective = im_f2.len() == n3 as usize;

    s_result(&mut out, "im(f₁)", &format!("{:?}", im_f1_set.iter().collect::<Vec<_>>()));
    s_result(&mut out, "ker(f₂)", &format!("{:?}", ker_f2_set.iter().collect::<Vec<_>>()));

    if injective { s_ok(&mut out, "f₁ is injective (exact at ℤ/n₁ℤ)"); }
    else { s_err(&mut out, &format!("f₁ is NOT injective: ker(f₁) = {:?}", ker_f1)); }

    if exact_mid { s_ok(&mut out, "im(f₁) = ker(f₂) — exact at ℤ/n₂ℤ"); }
    else { s_err(&mut out, "im(f₁) ≠ ker(f₂) — NOT exact at ℤ/n₂ℤ"); }

    if surjective { s_ok(&mut out, "f₂ is surjective (exact at ℤ/n₃ℤ)"); }
    else { s_err(&mut out, "f₂ is NOT surjective"); }

    let all_exact = injective && exact_mid && surjective;
    out.push('\n');
    if all_exact { s_ok(&mut out, "This IS a short exact sequence."); }
    else { s_err(&mut out, "This is NOT a short exact sequence."); }
    s_note(&mut out, "For 0→ℤ/n₁→ℤ/n₂→ℤ/n₃→0 to be exact we need |n₁|·|n₃| = |n₂|.");
    let product = n1 * n3;
    if product == n2 {
        s_info(&mut out, &format!("  Indeed: n₁·n₃ = {}·{} = {} = n₂", n1, n3, product));
    } else {
        s_info(&mut out, &format!("  But: n₁·n₃ = {}·{} = {} ≠ {} = n₂", n1, n3, product, n2));
    }
    out
}

// ── Snake Lemma ───────────────────────────────────────────────────────────────

fn cmd_snake(args: &[&str]) -> String {
    let a1 = match parse_uint(args, 0, "A1") { Some(x) => x as i64, None => return String::new() };
    let a2 = match parse_uint(args, 1, "A2") { Some(x) => x as i64, None => return String::new() };
    let a3 = match parse_uint(args, 2, "A3") { Some(x) => x as i64, None => return String::new() };
    let b1 = match parse_uint(args, 3, "B1") { Some(x) => x as i64, None => return String::new() };
    let b2 = match parse_uint(args, 4, "B2") { Some(x) => x as i64, None => return String::new() };
    let b3 = match parse_uint(args, 5, "B3") { Some(x) => x as i64, None => return String::new() };

    let mut out = String::new();
    s_section(&mut out, "Snake Lemma");
    out.push_str("  Commutative diagram (all modules ℤ/nℤ):\n\n");
    out.push_str(&format!("  0 → ℤ/{}ℤ → ℤ/{}ℤ → ℤ/{}ℤ → 0  (top row, exact)\n", a1, a2, a3));
    out.push_str("        ↓f₁      ↓f₂      ↓f₃\n");
    out.push_str(&format!("  0 → ℤ/{}ℤ → ℤ/{}ℤ → ℤ/{}ℤ → 0  (bottom row, exact)\n", b1, b2, b3));
    s_sep(&mut out);

    out.push_str("  Snake lemma statement:\n");
    out.push_str("  Given a commutative diagram with exact rows, there is a\n");
    out.push_str("  long exact sequence:\n");
    out.push_str("  ker(f₁) → ker(f₂) → ker(f₃) →∂→ coker(f₁) → coker(f₂) → coker(f₃)\n\n");

    let f1 = gcd(a1, b1);
    let f2 = gcd(a2, b2);
    let f3 = gcd(a3, b3);

    s_result(&mut out, "Vertical map f₁ (×f₁ in ℤ/b₁ℤ)", &format!("×{}", f1));
    s_result(&mut out, "Vertical map f₂ (×f₂ in ℤ/b₂ℤ)", &format!("×{}", f2));
    s_result(&mut out, "Vertical map f₃ (×f₃ in ℤ/b₃ℤ)", &format!("×{}", f3));

    let ker1: Vec<i64> = (0..a1).filter(|&x| (f1 * x).rem_euclid(b1) == 0).collect();
    let ker2: Vec<i64> = (0..a2).filter(|&x| (f2 * x).rem_euclid(b2) == 0).collect();
    let ker3: Vec<i64> = (0..a3).filter(|&x| (f3 * x).rem_euclid(b3) == 0).collect();

    let im1: std::collections::BTreeSet<i64> = (0..a1).map(|x| (f1 * x).rem_euclid(b1)).collect();
    let im2: std::collections::BTreeSet<i64> = (0..a2).map(|x| (f2 * x).rem_euclid(b2)).collect();
    let im3: std::collections::BTreeSet<i64> = (0..a3).map(|x| (f3 * x).rem_euclid(b3)).collect();
    let coker1_size = b1 as usize / im1.len();
    let coker2_size = b2 as usize / im2.len();
    let coker3_size = b3 as usize / im3.len();

    out.push('\n');
    s_result(&mut out, "ker(f₁)", &format!("ℤ/{}ℤ  {:?}", ker1.len(), ker1));
    s_result(&mut out, "ker(f₂)", &format!("ℤ/{}ℤ  {:?}", ker2.len(), ker2));
    s_result(&mut out, "ker(f₃)", &format!("ℤ/{}ℤ  {:?}", ker3.len(), ker3));
    s_result(&mut out, "coker(f₁)", &format!("order {}", coker1_size));
    s_result(&mut out, "coker(f₂)", &format!("order {}", coker2_size));
    s_result(&mut out, "coker(f₃)", &format!("order {}", coker3_size));

    out.push('\n');
    s_note(&mut out, "The connecting homomorphism ∂: ker(f₃) → coker(f₁) is constructed");
    s_info(&mut out, "  by lifting an element c ∈ ker(f₃) to b ∈ B₂, applying f₂ to get f₂(b),");
    s_info(&mut out, "  then noting f₂(b) ∈ ker(B₂ → B₃), so it comes from B₁; its class in coker(f₁)");
    s_info(&mut out, "  is ∂(c). This is well-defined and the resulting sequence is exact.");
    out
}

// ── Kernel / Cokernel ─────────────────────────────────────────────────────────

fn cmd_kernel_cokernel(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let m = match parse_uint(args, 1, "m") { Some(x) => x as i64, None => return String::new() };
    let a = match parse_int(args, 2, "a") { Some(x) => x, None => return String::new() };

    let mut out = String::new();
    s_section(&mut out, &format!("f: ℤ/{}ℤ → ℤ/{}ℤ,  x ↦ {}x", n, m, a));

    let ker: Vec<i64> = (0..n).filter(|&x| (a * x).rem_euclid(m) == 0).collect();
    let im: std::collections::BTreeSet<i64> = (0..n).map(|x| (a * x).rem_euclid(m)).collect();
    let im_vec: Vec<i64> = im.iter().copied().collect();

    let mut coker_reps: Vec<i64> = Vec::new();
    let mut seen: std::collections::BTreeSet<i64> = std::collections::BTreeSet::new();
    for y in 0..m {
        if !seen.contains(&y) {
            coker_reps.push(y);
            for &v in &im_vec {
                seen.insert((y + v).rem_euclid(m));
            }
        }
    }

    let ker_order = ker.len();
    let g_ker = if ker_order > 0 { gcd(ker[ker_order.saturating_sub(1)] as i64 + 1, ker_order as i64) } else { 1 };
    let _ = g_ker;

    s_result(&mut out, "ker(f)", &format!("{:?}  (order {})", ker, ker_order));
    s_result(&mut out, "im(f)", &format!("{:?}  (order {})", im_vec, im_vec.len()));
    s_result(&mut out, "coker(f) = ℤ/mℤ/im(f)", &format!("representatives {:?}  (order {})", coker_reps, coker_reps.len()));

    out.push('\n');
    let quotient_order = n / ker_order as i64;
    s_note(&mut out, &format!("First isomorphism theorem: ℤ/{}ℤ / ker(f) ≅ im(f)", n));
    s_info(&mut out, &format!("  |ℤ/{}ℤ| / |ker(f)| = {} / {} = {}", n, n, ker_order, quotient_order));
    s_info(&mut out, &format!("  |im(f)| = {} ✓", im_vec.len()));

    let d = gcd(a.rem_euclid(n), n);
    s_note(&mut out, &format!("ker(f) ≅ ℤ/{}ℤ  (since gcd({} mod {}, {}) = {})", n / d, a, n, n, d));
    let d2 = gcd(a, m);
    s_note(&mut out, &format!("im(f) ≅ ℤ/{}ℤ  (since gcd({}, {}) = {})", m / d2, a, m, d2));
    s_note(&mut out, &format!("coker(f) ≅ ℤ/{}ℤ  (order {})", coker_reps.len(), coker_reps.len()));
    out
}

// ── Five Lemma ────────────────────────────────────────────────────────────────

fn cmd_five_lemma(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as usize, None => return String::new() };
    let mut out = String::new();
    if n < 2 { s_err(&mut out, "n must be at least 2"); return out; }

    s_section(&mut out, "Five Lemma");
    out.push_str("  Consider two exact rows connected by vertical maps:\n\n");
    out.push_str("  A₁ → A₂ → A₃ → A₄ → A₅\n");
    out.push_str("  ↓f₁   ↓f₂   ↓f₃   ↓f₄   ↓f₅\n");
    out.push_str("  B₁ → B₂ → B₃ → B₄ → B₅\n\n");
    out.push_str("  Five Lemma: if f₁,f₂,f₄,f₅ are isomorphisms, then so is f₃.\n");
    s_sep(&mut out);

    let n2 = n * n;
    out.push_str(&format!("  Concrete example with n = {}:\n", n));
    out.push_str(&format!("  0 → ℤ/{}ℤ → ℤ/{}ℤ → ℤ/{}ℤ → 0\n", n, n2, n));
    out.push_str("  ↓id       ↓id       ↓id\n");
    out.push_str(&format!("  0 → ℤ/{}ℤ → ℤ/{}ℤ → ℤ/{}ℤ → 0\n\n", n, n2, n));
    s_ok(&mut out, "All vertical maps are identities, hence isomorphisms.");
    s_note(&mut out, "The five lemma guarantees f₃ is an isomorphism when f₁,f₂,f₄,f₅ are.");
    out.push('\n');

    out.push_str("  Non-trivial example: multiply top row by 2, bottom row unchanged.\n");
    out.push_str(&format!("  Top:    0 → ℤ/{}ℤ →(×{})→ ℤ/{}ℤ →(proj)→ ℤ/{}ℤ → 0\n", n, n, n2, n));
    out.push_str("  Vertical maps: ×2 on middle, identity elsewhere (when applicable).\n\n");
    s_note(&mut out, "The four lemma (half of the five lemma):");
    s_info(&mut out, "  If f₁ surjective, f₂,f₄ isomorphisms ⇒ f₃ surjective.");
    s_info(&mut out, "  If f₅ injective,  f₂,f₄ isomorphisms ⇒ f₃ injective.");
    s_note(&mut out, "Proof uses diagram chasing — the key technique in abelian categories.");
    out
}

// ── Short Exact Sequence Display ──────────────────────────────────────────────

fn cmd_short_exact(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let k = match parse_uint(args, 1, "k") { Some(x) => x as i64, None => return String::new() };
    let mut out = String::new();
    if k == 0 || n % k != 0 { s_err(&mut out, "k must divide n"); return out; }
    let m = n / k;

    s_section(&mut out, &format!("Short Exact Sequence: n={}, k={}", n, k));
    out.push('\n');
    out.push_str(&format!("  0 → ℤ/{}ℤ →ι→ ℤ/{}ℤ →π→ ℤ/{}ℤ → 0\n\n", k, n, m));
    out.push_str("  Here:\n");
    out.push_str(&format!("    ι: ℤ/{}ℤ → ℤ/{}ℤ,  x ↦ {}x   (multiplication by {})\n", k, n, m, m));
    out.push_str(&format!("    π: ℤ/{}ℤ → ℤ/{}ℤ,  x ↦ x mod {}   (reduction mod {})\n", n, m, m, m));
    s_sep(&mut out);

    let im_iota: std::collections::BTreeSet<i64> = (0..k).map(|x| (m * x).rem_euclid(n)).collect();
    let ker_pi: Vec<i64> = (0..n).filter(|&x| x.rem_euclid(m) == 0).collect();
    let ker_pi_set: std::collections::BTreeSet<i64> = ker_pi.iter().copied().collect();

    s_result(&mut out, "im(ι)", &format!("{:?}", im_iota.iter().collect::<Vec<_>>()));
    s_result(&mut out, "ker(π)", &format!("{:?}", ker_pi));
    if im_iota == ker_pi_set {
        s_ok(&mut out, "im(ι) = ker(π): exact at ℤ/nℤ");
    }
    s_ok(&mut out, "ι is injective: ker(ι)={0} ✓");
    s_ok(&mut out, "π is surjective ✓");
    s_note(&mut out, &format!("This encodes: ℤ/{}ℤ / ({}ℤ/{}ℤ) ≅ ℤ/{}ℤ", n, k, n, m));
    s_note(&mut out, &format!("Equivalently: {}·{} = {} = |ℤ/{}ℤ|", k, m, n, n));
    out
}

// ── Split Exact Sequence ──────────────────────────────────────────────────────

fn cmd_split(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let k = match parse_uint(args, 1, "k") { Some(x) => x as i64, None => return String::new() };
    let nk = n * k;

    let mut out = String::new();
    s_section(&mut out, &format!("Split Exact Sequence Check: 0 → ℤ/{}ℤ → ℤ/{}ℤ → ℤ/{}ℤ → 0", k, nk, n));
    out.push('\n');
    out.push_str(&format!("  ι: ℤ/{}ℤ → ℤ/{}ℤ,  x ↦ {}x\n", k, nk, n));
    out.push_str(&format!("  π: ℤ/{}ℤ → ℤ/{}ℤ,  x ↦ x mod {}\n", nk, n, n));
    s_sep(&mut out);

    let d = gcd(n, k);
    let splits = d == 1;

    if splits {
        s_ok(&mut out, &format!("gcd({},{}) = 1 — the sequence SPLITS", n, k));
        out.push('\n');
        out.push_str(&format!("  ℤ/{}ℤ ≅ ℤ/{}ℤ ⊕ ℤ/{}ℤ  (Chinese Remainder Theorem)\n\n", nk, n, k));
        out.push_str(&format!("  Splitting homomorphism s: ℤ/{}ℤ → ℤ/{}ℤ\n", n, nk));
        out.push_str(&format!("    s(x) = {}x mod {}   (since ord({}) = {} in ℤ/{}ℤ)\n\n", k, nk, k, n, nk));
        let ord_k = {
            let mut o = 1i64;
            let mut v = k;
            while v % nk != 0 { v += k; o += 1; if o > nk + 1 { break; } }
            o
        };
        s_result(&mut out, "ord(k) in ℤ/nkℤ", &format!("{}", ord_k));
        if ord_k == n { s_ok(&mut out, &format!("ord({}) = {} = n ✓", k, n)); }
        out.push('\n');
        s_note(&mut out, &format!("The retraction r: ℤ/{}ℤ → ℤ/{}ℤ satisfies r∘ι = id", nk, k));
        let (_, n_inv_k, _) = ext_gcd(n, k);
        let n_inv = n_inv_k.rem_euclid(k);
        out.push_str(&format!("    r(x) = {}x mod {}   (n⁻¹ mod k = {})\n", n_inv, k, n_inv));
    } else {
        s_err(&mut out, &format!("gcd({},{}) = {} ≠ 1 — the sequence does NOT split", n, k, d));
        out.push('\n');
        out.push_str(&format!("  ℤ/{}ℤ is NOT isomorphic to ℤ/{}ℤ ⊕ ℤ/{}ℤ\n", nk, n, k));
        s_note(&mut out, &format!("When gcd(n,k) = {} > 1, ℤ/nkℤ has an element of order lcm(n,k) = {}", d, lcm(n, k)));
        s_note(&mut out, "but ℤ/nℤ ⊕ ℤ/kℤ has exponent lcm(n,k) only if gcd(n,k) = 1");
        s_note(&mut out, "This is a non-split extension — a non-trivial element of Ext¹(ℤ/nℤ, ℤ/kℤ)");
    }
    out
}

// ── Freyd-Mitchell Embedding ──────────────────────────────────────────────────

fn cmd_embedding() -> String {
    let mut out = String::new();
    s_section(&mut out, "Freyd-Mitchell Embedding Theorem");
    out.push('\n');
    out.push_str("  Theorem (Freyd 1964, Mitchell 1965):\n");
    out.push_str("  Every small abelian category 𝒜 admits a full, exact, faithful\n");
    out.push_str("  embedding into R-Mod for some ring R.\n\n");
    s_sep(&mut out);
    out.push_str("  Consequences:\n");
    out.push_str("  • Diagram lemmas (snake, five, horseshoe) can be proved\n");
    out.push_str("    by element-chasing in R-Mod and transferred back to 𝒜.\n");
    out.push_str("  • Abstract abelian categories behave like module categories.\n\n");
    out.push_str("  Concrete instance: the category of finitely generated ℤ-modules\n");
    out.push_str("  embeds into ℤ-Mod (trivially — it IS a subcategory).\n\n");
    out.push_str("  More interesting: take 𝒜 = Rep_fd(G) (finite-dim. representations of G).\n");
    out.push_str("  By Freyd-Mitchell, 𝒜 ↪ ℤ[G]-Mod.\n");
    out.push_str("  Explicitly for G = ℤ/nℤ: ℤ[ℤ/nℤ] ≅ ℤ[x]/(xⁿ−1),\n");
    out.push_str("  and Rep_fd(ℤ/nℤ) ↪ ℤ[x]/(xⁿ−1)-Mod.\n\n");
    for n in [2u64, 3, 4, 6] {
        out.push_str(&format!("  n={}: ℤ[x]/(x^{}−1) ≅ ℤ[x]/({})\n", n, n,
            {
                let factors = factor(n);
                if factors.is_empty() { "x−1".to_string() }
                else {
                    let divs: Vec<u64> = divisors(n);
                    divs.iter().map(|&d| format!("Φ_{}", d)).collect::<Vec<_>>().join("·")
                }
            }));
    }
    out.push('\n');
    s_note(&mut out, "The ring R in the embedding theorem is typically non-commutative.");
    s_note(&mut out, "The embedding need not be unique — it depends on a choice of injective cogenerator.");
    s_note(&mut out, "Grothendieck's AB axioms provide the modern formulation of abelian categories.");
    out
}

fn cmd_demo() -> String {
    let mut out = String::new();
    s_section(&mut out, "Demo: Abelian Category Showcase");
    out.push('\n');
    out.push_str("  --- Exact sequence: 0→ℤ/2→ℤ/6→ℤ/3→0 ---\n");
    out.push_str(&cmd_exact(&["2", "3", "6", "2", "3"]));
    out.push('\n');
    out.push_str("  --- Kernel/cokernel: ℤ/6→ℤ/4, x↦2x ---\n");
    out.push_str(&cmd_kernel_cokernel(&["6", "4", "2"]));
    out.push('\n');
    out.push_str("  --- Split check: 0→ℤ/3→ℤ/12→ℤ/4→0 ---\n");
    out.push_str(&cmd_split(&["4", "3"]));
    out
}

fn default_state() -> StateMap {
    state_new()
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    // Save last-used numeric parameters
    if !args.is_empty() {
        let params: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
        if !params.is_empty() {
            state_set_ints(state, "last_params", &params);
        }
        state_set_str(state, "last_cmd", cmd);
    }
    match cmd {
        "exact"            => cmd_exact(args),
        "snake"            => cmd_snake(args),
        "kernel_cokernel"  => cmd_kernel_cokernel(args),
        "five_lemma"       => cmd_five_lemma(args),
        "short_exact"      => cmd_short_exact(args),
        "split"            => cmd_split(args),
        "embedding"        => cmd_embedding(),
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
    c.text_bold(350.0, 40.0, "Abelian Category: Short Exact Sequence", 16.0, "#222", "middle");
    c.node_circle(80.0, 250.0, "0", "white", 30.0, 13.0);
    c.node_circle(220.0, 250.0, "A", "white", 30.0, 13.0);
    c.node_circle(360.0, 250.0, "B", "white", 30.0, 13.0);
    c.node_circle(500.0, 250.0, "C", "white", 30.0, 13.0);
    c.node_circle(620.0, 250.0, "0", "white", 30.0, 13.0);
    c.arrow(110.0, 250.0, 190.0, 250.0, "#444", 1.5);
    c.arrow(250.0, 250.0, 330.0, 250.0, "#444", 1.5);
    c.arrow(390.0, 250.0, 470.0, 250.0, "#444", 1.5);
    c.arrow(530.0, 250.0, 590.0, 250.0, "#444", 1.5);
    c.text(155.0, 235.0, "f", 13.0, "#333", "middle");
    c.text(295.0, 235.0, "g", 13.0, "#333", "middle");
    c.text(430.0, 235.0, "h", 13.0, "#333", "middle");
    c.text(220.0, 310.0, "ker(g)=im(f)", 12.0, "#555", "middle");
    c.text(360.0, 310.0, "ker(h)=im(g)", 12.0, "#555", "middle");
    c.text(200.0, 380.0, "0 → A → B → C → 0  is exact", 12.0, "#333", "start");
    c.text(200.0, 410.0, "A injects into B, C = B/A", 12.0, "#333", "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("zero1", &[("label", "0"), ("shape", "plaintext")]);
    g.node("A", &[("label", "A"), ("shape", "circle")]);
    g.node("B", &[("label", "B"), ("shape", "circle")]);
    g.node("C", &[("label", "C"), ("shape", "circle")]);
    g.node("zero2", &[("label", "0"), ("shape", "plaintext")]);
    g.edge("zero1", "A", &[("label", "")]);
    g.edge("A", "B", &[("label", "f")]);
    g.edge("B", "C", &[("label", "g")]);
    g.edge("C", "zero2", &[("label", "")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("zero1", 0.0, 0.0, "0", "");
    t.node("A", 2.0, 0.0, "A", "");
    t.node("B", 4.0, 0.0, "B", "");
    t.node("C", 6.0, 0.0, "C", "");
    t.node("zero2", 8.0, 0.0, "0", "");
    t.arrow("zero1", "A", "", "");
    t.arrow("A", "B", "f", "above");
    t.arrow("B", "C", "g", "above");
    t.arrow("C", "zero2", "", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 1, "Abelian Category: Short Exact Sequence");
    a.text_at(0, 3, "0 --f--> A --g--> B --h--> C --> 0");
    a.text_at(0, 5, "ker(g) = im(f)  (exact at A)");
    a.text_at(0, 6, "ker(h) = im(g)  (exact at B)");
    a.text_at(0, 7, "h is surjective (exact at C)");
    a.hline(0, 38, 9, '-');
    a.text_at(0, 10, "Snake Lemma extends this to long exact seq");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch37"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 37", "Abelian Categories", "Exactness, kernels, cokernels, and diagram lemmas");
            print!("{}", show_help());
            print_note("Try: exact 2 1 6 2 3   or   kernel_cokernel 6 4 2   or   split 3 4");
            repl("ch37> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
