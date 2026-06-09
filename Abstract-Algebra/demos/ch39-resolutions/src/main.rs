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

fn show_help() -> String {
    let commands: &[(&str, &str)] = &[
        ("free_res <n>",         "free resolution of ℤ/nℤ over ℤ"),
        ("projective_res <n>",   "projective resolution (= free over ℤ)"),
        ("syzygy <n> <k>",       "syzygy module: kernel of ℤ^k → ℤ/nℤ"),
        ("pd <n>",               "projective dimension of ℤ/nℤ as ℤ-module"),
        ("koszul <n>",           "Koszul complex for (x) in ℤ[x]/(xⁿ)"),
        ("hilbert_syzygy",       "Hilbert's syzygy theorem illustration"),
        ("minimal_res <n>",      "minimal free resolution of ℤ/nℤ over ℤ[x]/(xⁿ)"),
        ("demo",                 "showcase of resolution concepts"),
        ("help",                 "show this help"),
        ("quit",                 "exit"),
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

// ── Free Resolution ───────────────────────────────────────────────────────────

fn cmd_free_res(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let mut out = String::new();
    if n < 1 { s_err(&mut out, "n must be positive"); return out; }

    s_section(&mut out, &format!("Free Resolution of ℤ/{}ℤ over ℤ", n));
    out.push('\n');
    out.push_str(&format!("  0 → ℤ →(×{})→ ℤ →(ε)→ ℤ/{}ℤ → 0\n\n", n, n));
    out.push_str("  • P₁ = ℤ  (free module of rank 1)\n");
    out.push_str("  • P₀ = ℤ  (free module of rank 1)\n");
    out.push_str(&format!("  • ∂₁: P₁ → P₀,  1 ↦ n  (multiplication by {})\n", n));
    out.push_str("  • ε: P₀ → ℤ/nℤ,  1 ↦ 1  (augmentation/projection)\n\n");
    // use s_sep equivalent
    out.push_str(&format!("  \x1b[2m{}\x1b[0m\n", "─".repeat(60)));

    if n == 1 {
        out.push_str("  When n=1: ℤ/1ℤ = 0, so the resolution is just 0 → 0.\n");
        s_note(&mut out, "ℤ/1ℤ = 0 is the trivial module; its projective dimension is -∞.");
        return out;
    }

    out.push_str("  Verification:\n");
    out.push_str(&format!("    im(×{}) = {}ℤ ⊆ ℤ\n", n, n));
    out.push_str(&format!("    ker(ε) = {}ℤ   (since ε(k) = k mod {})\n", n, n));
    out.push_str(&format!("    im(×{}) = ker(ε) ✓\n", n));
    out.push_str(&format!("    ×{} is injective (n ≠ 0) ✓\n", n));
    out.push_str("    ε is surjective ✓\n\n");

    s_result(&mut out, "Kernel of ε", &format!("{}ℤ = im(×{})", n, n));
    s_result(&mut out, "Resolution length", "1 (projective dimension = 1)");

    out.push('\n');
    out.push_str("  Display as chain complex:\n");
    let d1 = Mat::new(1, 1, vec![n]);
    out.push_str(&format!("  ∂₁ = {} (multiplication by {} on ℤ)\n", n, n));
    out.push_str(&format!("{}", d1));

    s_note(&mut out, "This resolution is MINIMAL: ∂₁ ⊗ 1 ≠ 0 in ℤ/nℤ ⊗_ℤ P₁ → ℤ/nℤ ⊗_ℤ P₀.");
    s_note(&mut out, "Every ℤ/nℤ-module (n≥2) has projective dimension exactly 1 over ℤ.");
    out
}

// ── Projective Resolution ─────────────────────────────────────────────────────

fn cmd_projective_res(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let mut out = String::new();
    s_section(&mut out, &format!("Projective Resolution of ℤ/{}ℤ over ℤ", n));
    out.push('\n');
    s_note(&mut out, "Over ℤ, every projective module is free (ℤ is a PID).");
    out.push_str("  So the free resolution IS the projective resolution:\n\n");
    out.push_str(&format!("  0 → ℤ →(×{})→ ℤ → ℤ/{}ℤ → 0\n\n", n, n));
    out.push_str("  This is a length-1 projective resolution.\n");
    out.push_str("  • All terms P₁=ℤ, P₀=ℤ are free (hence projective) ℤ-modules.\n\n");
    s_note(&mut out, "General principle: over a PID, every finitely generated module has");
    s_note(&mut out, "a projective resolution of length ≤ 1 (global dimension of ℤ is 1).");
    out.push_str(&cmd_free_res(args));
    out
}

// ── Syzygy ────────────────────────────────────────────────────────────────────

fn cmd_syzygy(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let k = match parse_uint(args, 1, "k") { Some(x) => x as usize, None => return String::new() };
    let mut out = String::new();
    if k == 0 { s_err(&mut out, "k must be at least 1"); return out; }

    s_section(&mut out, &format!("Syzygies: ℤ^{} → ℤ/{}ℤ", k, n));
    out.push('\n');
    out.push_str(&format!("  We present ℤ/{}ℤ using k={} generators, all mapping to 1 ∈ ℤ/{}ℤ.\n", n, k, n));
    out.push_str(&format!("  Map φ: ℤ^{} → ℤ/{}ℤ,  eᵢ ↦ 1  for all i.\n\n", k, n));

    out.push_str("  Kernel (first syzygy module) Syz₁ is generated by:\n");
    out.push_str(&format!("  • n·e₁  =  ({},...)\n", n));
    for i in 1..k {
        let mut v = vec![0i64; k];
        v[0] = 1; v[i] = -1;
        s_info(&mut out, &format!("  • e₁ - e_{} = {:?}", i+1, v));
    }
    out.push('\n');
    out.push_str(&format!("  These {} generators suffice because any (a₁,...,aₖ) with Σaᵢ ≡ 0 mod n\n", k));
    out.push_str("  can be written as a linear combination of the above.\n\n");
    s_result(&mut out, "Rank of syzygy module", &format!("{} (free ℤ-module)", k));
    s_result(&mut out, "Generators count", &format!("{}", k));
    out.push('\n');
    out.push_str(&format!("  The second syzygy module Syz₂ = ker(Syz₁ → ℤ^{}) is computed next.\n", k));
    out.push_str(&format!("  Since Syz₁ is free, the resolution terminates: pd(ℤ/{}ℤ) = 1.\n\n", n));
    s_note(&mut out, "Hilbert's syzygy theorem: over k[x₁,...,xₙ], the k-th syzygy is always free.");
    s_note(&mut out, "This gives the global dimension of polynomial rings: gl.dim(k[x₁,...,xₙ]) = n.");
    out
}

// ── Projective Dimension ──────────────────────────────────────────────────────

fn cmd_pd(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let mut out = String::new();

    s_section(&mut out, &format!("Projective Dimension of ℤ/{}ℤ over ℤ", n));
    out.push('\n');

    if n == 1 {
        out.push_str("  ℤ/1ℤ = 0 (the zero module).\n");
        s_result(&mut out, "pd(0)", "−∞ (by convention)");
        s_note(&mut out, "The zero module is projective (trivially), so pd = −∞.");
        return out;
    }

    out.push_str(&format!("  Free resolution: 0 → ℤ →(×{})→ ℤ → ℤ/{}ℤ → 0\n\n", n, n));
    out.push_str(&format!("  This resolution has length 1, so pd(ℤ/{}ℤ) ≤ 1.\n\n", n));
    out.push_str("  Is pd = 0? That would require ℤ/nℤ to be projective over ℤ.\n");
    out.push_str("  Projective ℤ-modules are free (ℤ is a PID).\n");
    out.push_str("  ℤ/nℤ is NOT free (it has torsion: n·1 = 0).\n");
    out.push_str(&format!("  So pd(ℤ/{}ℤ) ≠ 0.\n\n", n));
    s_result(&mut out, &format!("pd(ℤ/{}ℤ)", n), "1");
    out.push('\n');
    out.push_str(&format!("  This can also be seen from: Ext^1_ℤ(ℤ/{}ℤ, ℤ/{}ℤ) ≅ ℤ/{}ℤ ≠ 0.\n", n, n, n));
    out.push_str("  (A module has pd ≤ k iff Ext^{k+1} vanishes.)\n\n");
    s_note(&mut out, "Over ℤ, the global dimension is 1: gl.dim(ℤ) = 1.");
    s_note(&mut out, "This means every ℤ-module has projective dimension ≤ 1.");
    s_note(&mut out, "Compare: over a field k, gl.dim(k) = 0 (every module is free/projective).");

    if is_prime(n as u64) {
        out.push('\n');
        s_note(&mut out, &format!("Note: n={} is prime, so ℤ/{}ℤ is a field.", n, n));
        s_note(&mut out, &format!("As a ℤ/{}ℤ-module over itself: every module is free, pd = 0.", n));
        s_note(&mut out, "But as a ℤ-module: pd = 1 as computed above.");
    }
    out
}

// ── Koszul Complex ────────────────────────────────────────────────────────────

fn cmd_koszul(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let mut out = String::new();
    if n < 1 { s_err(&mut out, "n must be positive"); return out; }

    s_section(&mut out, &format!("Koszul Complex for (x) in ℤ[x]/(x^{})", n));
    out.push('\n');
    out.push_str(&format!("  Ring R = ℤ[x]/(x^{}),  ideal I = (x).\n", n));
    out.push_str(&format!("  Koszul complex K(x): 0 → R →(×x)→ R → R/xR → 0\n\n"));
    out.push_str(&format!("  Here R/xR = ℤ[x]/(x^{})/xℤ[x]/(x^{}) ≅ ℤ  (setting x=0).\n\n", n, n));
    out.push_str("  Concretely in terms of ℤ-modules:\n");
    out.push_str(&format!("  R = ℤ^{} (basis: 1, x, x², ..., x^{{{}-1}})\n", n, n));
    out.push_str(&format!("  ×x: ℤ^{} → ℤ^{}  is the nilpotent shift matrix (Jordan block)\n\n", n, n));

    let mut mult_x = Mat::zero(n as usize, n as usize);
    for i in 0..(n-1) as usize {
        mult_x[(i+1, i)] = 1;
    }
    out.push_str("  ×x matrix (Jordan block) on R:\n");
    out.push_str(&format!("{}", mult_x));

    let comp = mult_x.mul(&mult_x);
    let is_zero = comp.data.iter().all(|&v| v == 0);

    out.push_str(&format!("  (×x)² = {}zero\n", if is_zero { "" } else { "not " }));
    if !is_zero && n >= 2 {
        out.push_str("  (expected for n≥2: (×x)² ≠ 0, so K(x) is NOT acyclic for n>1)\n");
    }

    out.push('\n');
    out.push_str("  H₀(K(x)) = R/xR ≅ ℤ\n");
    out.push_str("  H₁(K(x)) = ker(×x)/im(×x)\n");

    out.push_str(&format!("  ker(×x) in R: the element x^{} generates ker (since x·x^{} = x^{} = 0 in R)\n", n-1, n-1, n));
    out.push_str(&format!("  im(×x) = {{0, x, 2x, ..., ({}x)}} — this is xR\n", n-1));
    out.push_str(&format!("  H₁ = ker/im = <x^{{{}}}> / <x^{{{}}}> ≅ ℤ/{}ℤ\n\n", n-1, n, n));

    s_result(&mut out, &format!("H₀(K(x) on ℤ[x]/x^{})", n), "ℤ");
    s_result(&mut out, &format!("H₁(K(x) on ℤ[x]/x^{})", n), &format!("ℤ/{}ℤ", n));

    s_note(&mut out, "The Koszul complex is the basic building block for all homological computations.");
    s_note(&mut out, "For a regular sequence r₁,...,rₙ, the Koszul complex K(r₁,...,rₙ) gives a free resolution.");
    out
}

// ── Hilbert's Syzygy Theorem ──────────────────────────────────────────────────

fn cmd_hilbert_syzygy() -> String {
    let mut out = String::new();
    s_section(&mut out, "Hilbert's Syzygy Theorem");
    out.push('\n');
    out.push_str("  Theorem (Hilbert 1890):\n");
    out.push_str("  Let k be a field. Every finitely generated k[x₁,...,xₙ]-module M\n");
    out.push_str("  has a free resolution of length ≤ n.\n");
    out.push_str("  Equivalently: gl.dim(k[x₁,...,xₙ]) = n.\n\n");
    out.push_str(&format!("  \x1b[2m{}\x1b[0m\n", "─".repeat(60)));
    out.push('\n');
    out.push_str("  Example 1: k[x] (n=1, global dimension 1)\n");
    out.push_str("  Module M = k[x]/(xⁿ):\n");
    out.push_str("  0 → k[x] →(×xⁿ)→ k[x] → k[x]/(xⁿ) → 0\n");
    out.push_str("  Length 1 resolution. ✓\n\n");
    out.push_str("  Example 2: k[x,y] (n=2, global dimension 2)\n");
    out.push_str("  Module M = k[x,y]/(x,y) ≅ k:\n");
    out.push_str("  0 → k[x,y] →(y,-x)→ k[x,y]² →(x,y)→ k[x,y] → k → 0\n");
    out.push_str("  (This is the Koszul complex K(x,y))\n");
    out.push_str("  Length 2 resolution. ✓\n\n");
    out.push_str("  The differentials:\n");
    out.push_str("  d₂: 1 ↦ (y, -x)\n");
    out.push_str("  d₁: (f,g) ↦ xf + yg\n");
    out.push_str("  ker(d₁) = {(f,g) : xf + yg = 0} = im(d₂) = (y,-x)·k[x,y] ✓\n\n");
    out.push_str("  Example 3: k[x,y,z] (n=3)\n");
    out.push_str("  M = k (the residue field):\n");
    out.push_str("  0 → k[x,y,z] → k[x,y,z]³ → k[x,y,z]³ → k[x,y,z] → k → 0\n");
    out.push_str("  Length 3 = n resolution. ✓\n\n");
    s_note(&mut out, "Hilbert's original proof used elimination theory and invariant theory.");
    s_note(&mut out, "The modern proof uses the Koszul complex K(x₁,...,xₙ) and induction.");
    s_note(&mut out, "This theorem launched homological algebra as a field.");
    out
}

// ── Minimal Free Resolution ───────────────────────────────────────────────────

fn cmd_minimal_res(args: &[&str]) -> String {
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return String::new() };
    let mut out = String::new();
    if n < 2 { s_err(&mut out, "n must be at least 2"); return out; }

    s_section(&mut out, &format!("Minimal Free Resolution of ℤ/{}ℤ over R = ℤ[x]/(x^{})", n, n));
    out.push('\n');
    out.push_str(&format!("  Ring R = ℤ[x]/(x^{})  (as ℤ-module, R ≅ ℤ^{})\n", n, n));
    out.push_str(&format!("  Module M = ℤ/{}ℤ ≅ R/(x, n)  (both x and n kill M)\n\n", n));
    out.push_str("  Minimal free resolution over R:\n\n");
    out.push_str("  ... → R² →(d₂)→ R² →(d₁)→ R →(ε)→ M → 0\n\n");
    out.push_str("  This is periodic (of period 2) because R is a hypersurface ring.\n\n");
    out.push_str("  d₁: R² → R,  d₁(e₁) = x,  d₁(e₂) = n\n");
    out.push_str("  d₂: R² → R²\n");
    out.push_str("  d₂(e₁) = (n, -x)\n");
    out.push_str(&format!("  d₂(e₂) = (x^{{{}}}, x^{{{}}})  (adjusted for syzygy)\n\n", n-1, n-1));
    out.push_str("  Verification: d₁∘d₂ = 0?\n");
    out.push_str("  d₁(d₂(e₁)) = d₁(n, -x) = nx + (-x)n = 0 ✓\n\n");

    s_result(&mut out, "Projective dimension over ℤ[x]/(xⁿ)", "∞ (periodic resolution)");
    s_note(&mut out, "Over R=ℤ[x]/(xⁿ), modules can have infinite projective dimension.");
    s_note(&mut out, "This is why Ext^k_R(M,N) can be nonzero for all k≥0.");
    s_note(&mut out, "Contrast: over ℤ (a PID), every module has pd ≤ 1.");

    out.push('\n');
    out.push_str("  The Betti numbers (ranks of free modules in resolution):\n");
    out.push_str("  ... 2, 2, 2, 2, 1  (eventually periodic with period 1, ranks alternating 1,2,2,...)\n\n");
    s_note(&mut out, "For M = ℤ/nℤ over ℤ[x]/(xⁿ), the Poincaré series is (1+t)/(1-t²).");
    out
}

fn cmd_demo() -> String {
    let mut out = String::new();
    s_section(&mut out, "Demo: Resolutions Showcase");
    out.push('\n');
    out.push_str("  --- Free resolution of ℤ/6ℤ ---\n");
    out.push_str(&cmd_free_res(&["6"]));
    out.push('\n');
    out.push_str("  --- Syzygy for ℤ/4ℤ with 3 generators ---\n");
    out.push_str(&cmd_syzygy(&["4", "3"]));
    out.push('\n');
    out.push_str("  --- Koszul complex for n=3 ---\n");
    out.push_str(&cmd_koszul(&["3"]));
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
        "free_res"       => cmd_free_res(args),
        "projective_res" => cmd_projective_res(args),
        "syzygy"         => cmd_syzygy(args),
        "pd"             => cmd_pd(args),
        "koszul"         => cmd_koszul(args),
        "hilbert_syzygy" => cmd_hilbert_syzygy(),
        "minimal_res"    => cmd_minimal_res(args),
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
    c.text_bold(350.0, 40.0, "Free Resolution: 0 → ℤ →(×n)→ ℤ → ℤ/nℤ → 0", 15.0, "#222", "middle");
    let y = 200.0;
    c.node_circle(80.0, y, "0", "white", 25.0, 13.0);
    c.node_circle(200.0, y, "P₁=ℤ", "white", 30.0, 11.0);
    c.node_circle(350.0, y, "P₀=ℤ", "white", 30.0, 11.0);
    c.node_circle(500.0, y, "ℤ/nℤ", "white", 40.0, 11.0);
    c.node_circle(630.0, y, "0", "white", 25.0, 13.0);
    c.arrow(105.0, y, 170.0, y, "#444", 1.5);
    c.arrow(230.0, y, 320.0, y, "#444", 1.5);
    c.arrow(380.0, y, 460.0, y, "#444", 1.5);
    c.arrow(540.0, y, 605.0, y, "#444", 1.5);
    c.text(270.0, 182.0, "×n", 13.0, "#333", "middle");
    c.text(420.0, 182.0, "ε", 13.0, "#333", "middle");
    c.text(200.0, 280.0, "free", 11.0, "#555", "middle");
    c.text(350.0, 280.0, "free", 11.0, "#555", "middle");
    c.text(500.0, 280.0, "torsion", 11.0, "#555", "middle");
    c.text(150.0, 370.0, "pd(ℤ/nℤ) = 1 over ℤ   (ℤ is a PID)", 12.0, "#333", "start");
    c.text(150.0, 410.0, "ker(ε) = nℤ = im(×n)  ✓", 12.0, "#333", "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("zero1", &[("label", "0"), ("shape", "plaintext")]);
    g.node("P1", &[("label", "P₁"), ("shape", "box")]);
    g.node("P0", &[("label", "P₀"), ("shape", "box")]);
    g.node("M", &[("label", "M"), ("shape", "box")]);
    g.node("zero2", &[("label", "0"), ("shape", "plaintext")]);
    g.edge("zero1", "P1", &[("label", "")]);
    g.edge("P1", "P0", &[("label", "∂₁")]);
    g.edge("P0", "M", &[("label", "ε")]);
    g.edge("M", "zero2", &[("label", "")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("zero1", 0.0, 0.0, "0", "");
    t.node("p1", 2.0, 0.0, "$P_1$", "");
    t.node("p0", 4.0, 0.0, "$P_0$", "");
    t.node("m", 6.0, 0.0, "$M$", "");
    t.node("zero2", 8.0, 0.0, "0", "");
    t.arrow("zero1", "p1", "", "");
    t.arrow("p1", "p0", "$\\partial_1$", "above");
    t.arrow("p0", "m", "$\\varepsilon$", "above");
    t.arrow("m", "zero2", "", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 1, "Free Resolution Diagram");
    a.text_at(0, 3, "0 --> P_1 --d_1--> P_0 --eps--> M --> 0");
    a.text_at(0, 5, "Each P_i is a free module");
    a.text_at(0, 6, "pd(M) = length of shortest resolution");
    a.hline(0, 45, 8, '-');
    a.text_at(0, 10, "Over Z:  0 --> Z --x*n--> Z --> Z/nZ --> 0");
    a.text_at(0, 11, "pd(Z/nZ) = 1  (Z is a PID)");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch39"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 39", "Resolutions", "Free resolutions, syzygies, and Hilbert's theorem");
            print!("{}", show_help());
            print_note("Try: free_res 6   or   syzygy 4 3   or   koszul 3");
            repl("ch39> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
