use common::*;

fn show_help() -> String {
    help_string(&[
        ("char_p <p>",              "Characteristic p: what changes from char 0?"),
        ("restricted <p>",         "Restricted Lie algebras in char p, x^[p] operation"),
        ("steinberg <p> <n>",      "Steinberg module dimension p^n for GL_n over 𝔽_p"),
        ("linkage <p> <lambda> <mu>","Check if λ,μ linked (same block) via Jantzen linkage"),
        ("tilting <p> <n>",        "Tilting module T(n): self-dual indecomposable"),
        ("demo",                   "Run a showcase of all commands"),
        ("help",                   "Show this help"),
        ("quit",                   "Exit"),
    ])
}

fn cmd_char_p(p: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) { return format!("  {} is not prime\n", p); }
    out.push_str(&format!("\n  ▸ Representation Theory in Characteristic p = {}\n\n", p));
    out.push_str("  Over a field 𝐤 of characteristic p > 0, representation theory changes radically:\n\n");
    out.push_str("  1. Maschke's Theorem FAILS\n");
    out.push_str("     For group G with p | |G|: the group ring 𝐤[G] is NOT semisimple.\n");
    out.push_str("     Example: G = ℤ/pℤ. The augmentation ideal is a non-split ideal.\n");
    out.push_str(&format!("     For ℤ/pℤ: dim(𝐤[ℤ/pℤ]) = {}. It has only 1 simple module: 𝐤.\n", p));
    out.push_str(&format!("     But 𝐤[ℤ/pℤ] itself has {} irreducible composition factors (all 𝐤 = trivial).\n\n", p));
    out.push_str("  2. Restricted representations for Lie algebras\n");
    out.push_str("     A Lie algebra g over 𝔽_p has a p-map: x ↦ x^[p] (p-th Frobenius).\n");
    out.push_str("     Restricted representations satisfy ρ(x)^p = ρ(x^[p]).\n");
    out.push_str(&format!("     sl(2,𝔽_p): e^[p]=f^[p]=0, h^[p]=h  (since h^p=h mod p for p odd).\n\n"));
    out.push_str(&format!("  3. Baby Verma modules for sl(2) over 𝔽_{}\n", p));
    out.push_str("     For χ=0 (restricted reps), baby Verma Z(λ), dim = p.\n");
    out.push_str(&format!("     Baby Vermas: Z(0), Z(1), ..., Z(p-1), each of dimension {}.\n\n", p));
    out.push_str("  4. Irreducible modules in char p\n");
    out.push_str("     For sl(2,𝔽_p):\n");
    out.push_str("     L(0), L(1), ..., L(p-1): dimensions 1, 2, ..., p.\n");
    for lambda in 0..p.min(8) {
        out.push_str(&format!("       L({}) has dimension {}\n", lambda, lambda+1));
    }
    if p > 8 { out.push_str(&format!("       ... up to L(p-1) of dimension p = {}\n", p)); }
    out.push_str("\n  5. Frobenius kernels and G_rT modules\n");
    out.push_str("     Algebraic groups in char p have Frobenius morphism F: G → G.\n");
    out.push_str("     G_r = ker(F^r) = r-th Frobenius kernel.\n");
    out.push_str("     Representations of G_r ↔ restricted reps of g with p^r-restricted weights.\n");
    out
}

fn cmd_restricted(p: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) { return format!("  {} is not prime\n", p); }
    out.push_str(&format!("\n  ▸ Restricted Lie Algebras in Characteristic p = {}\n\n", p));
    out.push_str("  A restricted Lie algebra (g, [p]) is a Lie algebra over 𝔽_p with\n");
    out.push_str("  a p-map x ↦ x^[p] satisfying:\n");
    out.push_str("    (i)   (λx)^[p] = λ^p · x^[p]\n");
    out.push_str("    (ii)  ad(x^[p]) = ad(x)^p  (as linear maps)\n");
    out.push_str("    (iii) (x+y)^[p] = x^[p] + y^[p] + Σᵢ sᵢ(x,y)  (structure terms)\n\n");
    out.push_str(&format!("  sl(2) as restricted Lie algebra over 𝔽_{}\n", p));
    out.push_str("  Basis: e=[[0,1],[0,0]], f=[[0,0],[1,0]], h=[[1,0],[0,-1]]\n");
    out.push_str("  p-map on matrix Lie algebras: x^[p] = x^p (matrix power)\n\n");
    if p == 2 {
        out.push_str("    e² = [[0,0],[0,0]] = 0  →  e^[2] = 0\n");
    } else {
        out.push_str("    e² = 0 (nilpotent), so e^p = 0  →  e^[p] = 0\n");
    }
    out.push_str("    f^p = 0  →  f^[p] = 0  (similarly nilpotent)\n");
    out.push_str("    h^p: h = diag(1,-1), h^p = diag(1^p,(-1)^p) = diag(1,(-1)^p)\n");
    if p % 2 == 1 {
        out.push_str("    For p odd: (-1)^p = -1, so h^p = diag(1,-1) = h  →  h^[p] = h\n");
    } else {
        out.push_str("    For p=2: (-1)^2 = 1, so h^2 = I, but h ≠ I in sl(2)... h^[2] = 0 mod 2\n");
    }
    out.push_str("\n  Baby Verma Modules Z(λ) for χ=0:\n");
    out.push_str("  Z(λ) = U(sl(2)) / (e, h-λ, f^p) — quotient of universal enveloping algebra\n");
    out.push_str("  dim Z(λ) = p for all λ\n\n");
    out.push_str(&format!("  Weights of Z(λ): λ, λ-2, ..., λ-2(p-1)  mod p\n"));
    for lam in 0..p.min(5) {
        let weights: Vec<i64> = (0..p).map(|k| ((lam as i64 - 2*k as i64).rem_euclid(p as i64))).collect();
        let ws: Vec<String> = weights.iter().map(|w| format!("{}", w)).collect();
        out.push_str(&format!("    Z({}): weights {{ {} }}\n", lam, ws.join(", ")));
    }
    out.push_str("\n  Irreducible modules L(λ) for 0 ≤ λ ≤ p-1:\n");
    for lam in 0..p.min(8) {
        let dim = lam + 1;
        if lam < p - 1 {
            out.push_str(&format!("    L({}) = Z({}) / (socle stuff): dim = {}\n", lam, lam, dim));
        } else {
            out.push_str(&format!("    L(p-1) = Z(p-1): dim = p = {}  (Steinberg module!)\n", p));
        }
    }
    out.push_str(&format!("\n  L(p-1) = Z(p-1) is irreducible of dim p — the Steinberg module for sl(2)!\n"));
    out.push_str("  In char p, L(p-1) plays the role that the trivial module plays in char 0.\n");
    out
}

fn cmd_steinberg(p: u64, n: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) { return format!("  {} is not prime\n", p); }
    if n < 1 || n > 6 { return "  n must be 1–6\n".to_string(); }
    out.push_str(&format!("\n  ▸ Steinberg Module St_n for GL_n over 𝔽_p\n\n"));
    let n_pos_roots = n * (n - 1) / 2;
    let st_dim = p.pow(n_pos_roots as u32);
    out.push_str(&format!("  G = GL_{} over 𝔽_p,  p = {}\n", n, p));
    out.push_str(&format!("  Number of positive roots of A_{{n-1}}: N = n(n-1)/2 = {}·{}/2 = {}\n", n, n-1, n_pos_roots));
    out.push_str(&format!("  dim(Steinberg module St) = p^N = {}^{} = {}\n\n", p, n_pos_roots, st_dim));
    out.push_str("  Properties of the Steinberg Module:\n");
    out.push_str("    (1) Irreducible — it is L((p-1)ρ) where ρ = sum of fundamental weights\n");
    out.push_str("    (2) Projective — it is a projective module for the group algebra 𝔽_p[G(𝔽_p)]\n");
    out.push_str("    (3) Self-dual — St ≅ St* as G-modules\n");
    out.push_str("    (4) The 'largest' p-restricted irrep\n\n");
    out.push_str("  For GL_n explicitly:\n");
    out.push_str("    St = ind_B^G(trivial) = 𝔽_p[G/U]  (induced from unipotent radical)\n");
    out.push_str("    Or: St = H^{top}(flags, 𝔽_p)  (top cohomology of the flag variety)\n\n");
    out.push_str("  Dimensions for various (p,n):\n");
    out.push_str(&format!("  {:6}  {:4}  {:12}  {:20}\n", "p", "n", "#(pos roots)", "dim(St)=p^{#pos}"));
    out.push_str(&format!("  {}\n", "-".repeat(50)));
    for pp in [2u64, 3, 5, 7] {
        for nn in [1u64, 2, 3, 4] {
            let nr = nn*(nn-1)/2;
            let d = pp.pow(nr as u32);
            out.push_str(&format!("  {:6}  {:4}  {:12}  {}\n", pp, nn, nr, d));
        }
    }
    out.push_str("\n  St is the unique irrep of highest weight (p-1)ρ = (p-1)(ω₁+...+ω_{n-1}).\n");
    out.push_str("  Steinberg's tensor product theorem: L(λ) = L(λ₀)⊗L(λ₁)^{[1]}⊗... (Frobenius twists).\n");
    out
}

fn cmd_linkage(p: u64, lambda: i64, mu: i64) -> String {
    let mut out = String::new();
    if !is_prime(p) { return format!("  {} is not prime\n", p); }
    out.push_str(&format!("\n  ▸ Jantzen Linkage: λ={} and μ={} for p={}\n\n", lambda, mu, p));
    out.push_str("  Two weights λ,μ are linked if μ = w·λ for some w in the affine Weyl group W_p.\n");
    out.push_str("  The affine Weyl group W_p acts on weights by: s_{α,np}·λ = s_α(λ+np·α∨·ρ) - ρ\n");
    out.push_str("  (dot action with ρ = sum of fundamental weights = 1 for sl(2))\n\n");
    let p = p as i64;
    let orbit_lambda: Vec<i64> = (-5..=5).flat_map(|n: i64| {
        vec![2*p*n + lambda, 2*p*n + (-lambda - 2)]
    }).collect();
    let orbit_mu: Vec<i64> = (-5..=5).flat_map(|n: i64| {
        vec![2*p*n + mu, 2*p*n + (-mu - 2)]
    }).collect();
    let linked = orbit_lambda.contains(&mu) || orbit_mu.contains(&lambda);
    out.push_str(&format!("  For sl(2), the affine Weyl group W_{} acts on ℤ by:\n", p));
    out.push_str("  Generators: s (reflection) and translations by 2p\n");
    out.push_str("  Dot action orbits (linkage classes) are cosets of 2p:\n\n");
    out.push_str(&format!("  Orbit of {} under W_{} (dot action), selected elements:\n", lambda, p));
    let mut shown: Vec<i64> = orbit_lambda.clone();
    shown.sort(); shown.dedup();
    let shown_str: Vec<String> = shown.iter().filter(|&&x| x >= -20 && x <= 20)
        .map(|x| x.to_string()).collect();
    out.push_str(&format!("    {{ ..., {} , ... }}\n\n", shown_str.join(", ")));
    if linked {
        out.push_str(&format!("  λ={} and μ={} ARE linked (same block).\n", lambda, mu));
        out.push_str("  Being in the same block means: Ext groups between L(λ) and L(μ) can be non-zero.\n");
    } else {
        out.push_str(&format!("  λ={} and μ={} are NOT linked (different blocks).\n", lambda, mu));
        out.push_str("  Different blocks: L(λ) and L(μ) have no non-trivial extensions.\n");
    }
    out.push_str("\n  Block structure for sl(2) mod p:\n");
    out.push_str("  Blocks are indexed by mod-2p residues of λ+1 and their negatives.\n");
    out.push_str("  Steinberg block B_p: contains only L(p-1) (projective, hence simple block).\n\n");
    out.push_str("  Jantzen's sum formula controls the structure of Verma modules mod p.\n");
    out.push_str("  Kazhdan-Lusztig polynomials compute multiplicities [M(λ):L(μ)].\n");
    out
}

fn cmd_tilting(p: u64, n: i64) -> String {
    let mut out = String::new();
    if !is_prime(p) { return format!("  {} is not prime\n", p); }
    out.push_str(&format!("\n  ▸ Tilting Module T({}) for sl(2) in char p={}\n\n", n, p));
    out.push_str("  A tilting module T is:\n");
    out.push_str("    (1) Self-dual: T ≅ T* (or T ≅ ∇(λ) filtration from both sides)\n");
    out.push_str("    (2) Admits a filtration by Weyl modules Δ(μ)\n");
    out.push_str("    (3) Admits a filtration by dual Weyl modules ∇(μ)\n\n");
    let pp = p as i64;
    out.push_str(&format!("  Indecomposable tilting modules T(n) for sl(2), p={}:\n\n", p));
    out.push_str(&format!("  {:4}  {:10}   {}\n", "n", "dim(T(n))", "Structure (Weyl filtration [Δ(μ)])"));
    out.push_str(&format!("  {}\n", "-".repeat(60)));
    for i in 0..=(3*pp).min(20) {
        let (dim, structure) = tilting_sl2(i, pp);
        out.push_str(&format!("  {:4}  {:10}   {}\n", i, dim, structure));
    }
    out.push_str("\n  Key properties of T(n):\n");
    out.push_str("    T(n) for n < p: T(n) = L(n) = Δ(n)  (tilting = Weyl = irreducible in char 0 range)\n");
    out.push_str("    T(p-1) = Steinberg St: projective, simple, dim = p\n");
    out.push_str("    For p ≤ n ≤ 2p-2: T(n) has exactly 2 Weyl factors: Δ(n) and Δ(2p-2-n)\n\n");
    out.push_str("  Tilting modules form a complete set of indecomposable projective-injective objects.\n");
    out.push_str("  Donkin's tilting conjecture (proved): T(2(p^r-1)ρ) = projective cover of L(0).\n");
    out.push_str("  Connection to quantum groups: tilting modules at root of unity ↔ char p modules.\n");
    out
}

fn tilting_sl2(n: i64, p: i64) -> (i64, String) {
    let mut digits = Vec::new();
    let mut tmp = n;
    if tmp == 0 { digits.push(0); }
    while tmp > 0 { digits.push(tmp % p); tmp /= p; }
    let mut dim = 1i64;
    for &a in digits.iter() {
        let factor = if a == p-1 { p } else { a+1 };
        dim *= factor;
    }
    let structure = if n < p {
        if n == p-1 { format!("L({})=St, dim={}", n, n+1) }
        else { format!("L({})=Δ({}), dim={}", n, n, n+1) }
    } else {
        let factors: Vec<String> = digits.iter().enumerate()
            .map(|(k,&a)| if k==0 { format!("Δ({})^{{id}}", a) } else { format!("Δ({})^{{Fr^{}}}", a, k) })
            .collect();
        format!("[{}], dim={}", factors.join("⊗"), dim)
    };
    (dim, structure)
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "prime", 5);
    state_set_int(&mut s, "tilting_n", 7);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "help" | "h" => show_help(),
        "char_p" => {
            let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64);
            state_set_int(state, "prime", p as i64);
            cmd_char_p(p)
        }
        "restricted" => {
            let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64);
            state_set_int(state, "prime", p as i64);
            cmd_restricted(p)
        }
        "steinberg" => {
            let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(state_get_int(state, "prime").unwrap_or(3) as u64);
            let n = args.get(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(3);
            state_set_int(state, "prime", p as i64);
            state_set_int(state, "steinberg_n", n as i64);
            cmd_steinberg(p, n)
        }
        "linkage" => {
            let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64);
            let l = args.get(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(2);
            let m = args.get(2).and_then(|s| s.parse::<i64>().ok()).unwrap_or(8);
            state_set_int(state, "prime", p as i64);
            state_set_int(state, "linkage_lambda", l);
            state_set_int(state, "linkage_mu", m);
            cmd_linkage(p, l, m)
        }
        "tilting" => {
            let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64);
            let n = args.get(1).and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(state_get_int(state, "tilting_n").unwrap_or(7));
            state_set_int(state, "prime", p as i64);
            state_set_int(state, "tilting_n", n);
            cmd_tilting(p, n)
        }
        "demo" => {
            let mut out = cmd_char_p(5);
            out.push_str(&cmd_steinberg(3, 3));
            out.push_str(&cmd_linkage(5, 2, 8));
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

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, args: &[&str], state: &StateMap) {
    // Block diagram for modular representations
    let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64) as i64;
    c.text_bold(350.0, 30.0, &format!("Modular Rep Theory sl(2) mod p={}", p), 16.0, "#212121", "middle");
    let block_w = 80.0_f64;
    let block_h = 60.0_f64;
    let start_x = 50.0_f64;
    let y = 150.0_f64;
    for i in 0..p.min(7) {
        let x = start_x + i as f64 * (block_w + 10.0);
        let (fill, stroke) = if i == p - 1 { ("#ffe0a0", "#cc8800") }
                              else { ("#e0e8ff", "#4444cc") };
        c.rect(x, y, block_w, block_h, fill, stroke, 2.0);
        c.text(x + block_w/2.0, y + 25.0, &format!("L({})", i), 12.0, "#212121", "middle");
        c.text(x + block_w/2.0, y + 45.0, &format!("dim={}", i+1), 10.0, "#555555", "middle");
    }
    // Steinberg label
    let sx = start_x + (p-1) as f64 * (block_w + 10.0);
    c.text(sx + block_w/2.0, y + block_h + 20.0, "Steinberg", 11.0, "#cc8800", "middle");
    // Block decomposition
    c.text_bold(350.0, 280.0, "Block structure (linkage classes)", 14.0, "#212121", "middle");
    for i in 0..p.min(4) {
        let bx = 80.0 + i as f64 * 150.0;
        c.rect(bx, 310.0, 130.0, 100.0, "none", "#888888", 1.0);
        c.text(bx + 65.0, 330.0, &format!("Block B_{}", i+1), 12.0, "#333333", "middle");
        c.text(bx + 65.0, 355.0, &format!("L({}) ↔ L({})", i, 2*p-i-2), 11.0, "#555555", "middle");
    }
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, args: &[&str], state: &StateMap) {
    let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64) as i64;
    for i in 0..p.min(6) {
        let id = format!("L{}", i);
        let lbl = format!("L({})\\ndim={}", i, i+1);
        let shape = if i == p - 1 { "doubleoctagon" } else { "box" };
        g.node(&id, &[("label", &lbl), ("shape", shape)]);
    }
    // Linkage edges
    for i in 0..((p-1)/2).min(3) {
        let id1 = format!("L{}", i);
        let id2 = format!("L{}", 2*p - i - 2);
        if 2*p - i - 2 < p {
            g.edge(&id1, &id2, &[("label", "linked"), ("style", "dashed")]);
        }
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, args: &[&str], state: &StateMap) {
    let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64) as i64;
    for i in 0..p.min(5) {
        let id = format!("L{}", i);
        let lbl = format!("L({})", i);
        let style = if i == p-1 { "rectangle,draw,double" } else { "rectangle,draw" };
        t.node(&id, i as f64 * 2.0, 0.0, &lbl, style);
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, args: &[&str], state: &StateMap) {
    let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64) as i64;
    a.text_at(2, 1, &format!("Modular representations sl(2) over F_p, p={}", p));
    a.text_at(2, 3, "Irreducible modules:");
    for i in 0..p.min(8) {
        let mark = if i == p-1 { " [Steinberg]" } else { "" };
        a.text_at(4, (5 + i) as i32, &format!("L({}) dim={}{}", i, i+1, mark));
    }
    a.text_at(2, (6 + p.min(8)) as i32, &format!("dim(Steinberg) = p = {}", p));
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch51"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 51", "Modular Representation Theory",
                         "Representations over fields of prime characteristic");
            repl("ch51> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
