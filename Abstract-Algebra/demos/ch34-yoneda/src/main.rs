use common::*;

fn show_help() -> String {
    help_string(&[
        ("natural <n>",               "Natural transformation: det: GL_n -> (-)* is natural"),
        ("yoneda <n>",                "Yoneda: Nat(Hom(Z/nZ,-), Hom(Z/nZ,-)) = Z/nZ"),
        ("double_dual",               "V = V** (natural) vs V = V* (requires basis)"),
        ("representable <n>",         "Hom(Z/nZ,-) is representable by Z/nZ"),
        ("functor_category <n> <m>",  "Count Nat(Hom(Z/nZ,-), Hom(Z/mZ,-)) = gcd(n,m)"),
        ("demo",                      "Run a showcase of key results"),
        ("help",                      "Show this help"),
        ("quit",                      "Exit"),
    ])
}

fn hom_count(n: i64, m: i64) -> i64 { gcd(n, m) }

fn list_homs_zn_to_zk(n: i64, k: i64) -> Vec<i64> {
    let g = gcd(n, k);
    let step = k / g;
    (0..g).map(|i| (i * step) % k).collect()
}

fn cmd_natural(n: u64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Natural Transformation: det: GL_{}(-) → (-)* is Natural ===\n\n", n));
    out.push_str("A natural transformation η: F → G assigns to each object A a morphism\n");
    out.push_str("η_A: F(A) → G(A), compatibly with all morphisms in the category.\n\n");
    out.push_str(&format!("  F = GL_{}: Ring → Set  (matrices invertible over R)\n", n));
    out.push_str("  G = (-)* = units: Ring → Set  (units of R)\n");
    out.push_str(&format!("  η_R = det: GL_{}(R) → R*  (determinant map)\n\n", n));
    out.push_str("Naturality: for any ring map φ: R → S, the following commutes:\n\n");
    out.push_str(&format!("    GL_{}(R)  -(det)->  R*\n", n));
    out.push_str(&format!("       |                  |\n"));
    out.push_str(&format!("    GL_{}(φ)          phi|_{{(-)*}}\n", n));
    out.push_str("       ↓                  ↓\n");
    out.push_str(&format!("    GL_{}(S)  -(det)->  S*\n\n", n));
    out.push_str("  This commutes because: det(φ(M)) = φ(det(M)) for any ring map φ.\n\n");
    out.push_str("=== Concrete Example: Ring map Z/pZ → Z/qZ ===\n\n");
    let p = 5i64; let q = 5i64;
    out.push_str(&format!("  Using p={}, q={} (same ring for simplicity)\n\n", p, q));
    if n == 2 {
        let mat = Mat::new(2, 2, vec![2, 1, 1, 3]);
        let d = mat.det();
        out.push_str(&format!("  M = [[2,1],[1,3]] over ℤ/{}ℤ\n", p));
        out.push_str(&format!("  det(M) = {} ≡ {} (mod {})\n\n", d, d.rem_euclid(p), p));
        out.push_str(&format!("  Apply φ = id: ℤ/{}ℤ → ℤ/{}ℤ\n", p, q));
        out.push_str(&format!("  φ(det(M)) = {} (mod {})\n", d.rem_euclid(q), q));
        out.push_str(&format!("  det(φ(M)) = det(M mod {}) = {} (mod {})\n", q, d.rem_euclid(q), q));
        out.push_str("Both paths give the same result — naturality square commutes!\n");
    } else {
        out.push_str(&format!("  For n={}: det is a map GL_{}(R) → R*\n", n, n));
        out.push_str("  The naturality condition det(φ(M)) = φ(det(M)) holds for all ring maps φ.\n");
        out.push_str("Naturality of det holds by the ring-homomorphism property of φ.\n");
    }
    out.push_str("\nKey insight: det is 'natural' because it's defined by a universal formula,\n");
    out.push_str("not a choice. Non-natural transformations require arbitrary choices.\n");
    out
}

fn cmd_yoneda(n: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Yoneda Lemma for ℤ/{}ℤ ===\n\n", n));
    out.push_str("Yoneda Lemma: Nat(Hom(A,-), F) ≅ F(A) for any functor F: C → Set.\n\n");
    out.push_str(&format!("  Here: A = ℤ/{}ℤ, F = Hom(ℤ/{}ℤ, -): Ab → Set\n", n, n));
    out.push_str(&format!("  So Nat(Hom(ℤ/{}ℤ,-), Hom(ℤ/{}ℤ,-)) ≅ Hom(ℤ/{}ℤ, ℤ/{}ℤ) ≅ ℤ/{}ℤ\n\n",
        n, n, n, n, gcd(n, n)));
    out.push_str(&format!("  |Hom(ℤ/{}ℤ, ℤ/{}ℤ)| = gcd({},{}) = {}\n\n", n, n, n, n, gcd(n, n)));
    out.push_str("=== Enumerating Natural Transformations Hom(Z/nZ,-) → Hom(Z/nZ,-) ===\n\n");
    out.push_str("  By Yoneda, each natural transformation η corresponds to an element\n");
    out.push_str(&format!("  of Hom(ℤ/{}ℤ, ℤ/{}ℤ), i.e. a homomorphism φ: ℤ/{}ℤ → ℤ/{}ℤ.\n\n", n, n, n, n));
    let homs = list_homs_zn_to_zk(n, n);
    out.push_str(&format!("  All {} homomorphisms ℤ/{}ℤ → ℤ/{}ℤ (sending 1 ↦ ?):\n", homs.len(), n, n));
    for a in &homs {
        out.push_str(&format!("    φ_{}:  1 ↦ {}  (i.e. x ↦ {}x mod {})\n", a, a, a, n));
    }
    out.push_str("\n  Corresponding natural transformations η^a:\n");
    for a in &homs {
        out.push_str(&format!("    η^{}: Hom(ℤ/{}ℤ, M) → Hom(ℤ/{}ℤ, M)\n", a, n, n));
        out.push_str(&format!("         given by η^{}_M(f) = f∘φ_{}\n", a, a));
        out.push_str(&format!("         i.e. precompose with x ↦ {}x\n", a));
    }
    out.push_str("\n");
    out.push_str(&format!("There are exactly {} natural transformations (= |ℤ/{}ℤ|).\n\n", homs.len(), n));
    out.push_str("=== Yoneda Bijection Explicitly ===\n\n");
    out.push_str("  The bijection: η ↦ η_{ℤ/nℤ}(id_{ℤ/nℤ}) ∈ Hom(ℤ/nℤ, ℤ/nℤ)\n");
    out.push_str("  In other words: evaluate η at the identity morphism to get back φ.\n");
    out.push_str("The Yoneda lemma says: a natural transformation is completely determined\n");
    out.push_str("by where it sends the identity morphism. No more information is needed!\n");
    out
}

fn cmd_double_dual() -> String {
    let mut out = String::new();
    out.push_str("=== V = V** (Natural) vs V = V* (Not Natural) ===\n\n");
    out.push_str("For a finite-dimensional vector space V over a field k:\n");
    out.push_str("  V ≅ V*  (both have dim n) — but this isomorphism requires a choice of basis.\n");
    out.push_str("  V ≅ V** — this isomorphism is natural (no choice needed).\n\n");
    out.push_str("=== The Double Dual Isomorphism (Natural) ===\n\n");
    out.push_str("  ev: V → V** is defined by ev(v)(φ) = φ(v)\n");
    out.push_str("  This uses NO basis — it's defined for any v ∈ V and any φ ∈ V*.\n\n");
    out.push_str("  Naturality: for any linear map T: V→W,\n");
    out.push_str("      V -(ev_V)-> V**\n");
    out.push_str("      |               |\n");
    out.push_str("      T            T**\n");
    out.push_str("      ↓               ↓\n");
    out.push_str("      W -(ev_W)-> W**\n\n");
    out.push_str("  T**(φ)(v) = φ(T(v)) = ev_V(T(v))(φ)  — diagram commutes naturally.\n");
    out.push_str("The evaluation map is natural: it commutes with all linear maps T.\n\n");
    out.push_str("=== Concrete Example with Z/nZ ===\n\n");
    let n = 6i64;
    out.push_str(&format!("  M = ℤ/{}ℤ over ℤ\n", n));
    out.push_str(&format!("  M* = Hom(ℤ/{}ℤ, ℤ) = 0  (no non-zero hom ℤ/{}ℤ → ℤ)\n", n, n));
    out.push_str("For torsion modules over ℤ, M* = 0, so the story is different.\n");
    out.push_str("The natural double-dual story works best for free/finitely-generated free modules.\n\n");
    out.push_str("=== The Non-Natural Isomorphism V = V* ===\n\n");
    out.push_str("  Choose a basis e₁,...,eₙ for V. Define dual basis e₁*,...,eₙ* by:\n");
    out.push_str("    eᵢ*(eⱼ) = δᵢⱼ  (Kronecker delta)\n");
    out.push_str("  This gives an isomorphism V→V* by eᵢ↦eᵢ*.\n");
    out.push_str("  But this depends on the choice of basis!\n\n");
    out.push_str("V ≅ V* requires a basis choice — it is NOT a natural isomorphism.\n\n");
    out.push_str("Moral: natural = basis-free = categorically intrinsic.\n");
    out.push_str("Non-natural = requires extra structure (like a basis or inner product).\n");
    out
}

fn cmd_representable(n: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Representable Functor Hom(ℤ/{}ℤ, -) ===\n\n", n));
    out.push_str("A functor F: C → Set is representable if F ≅ Hom(A,-) for some object A.\n\n");
    out.push_str(&format!("  Functor F = Hom(ℤ/{}ℤ, -): Ab → Set\n", n));
    out.push_str(&format!("  F(M) = {{homomorphisms ℤ/{}ℤ → M}} = n-torsion subgroup of M\n\n", n));
    out.push_str(&format!("  Representing object: A = ℤ/{}ℤ\n", n));
    out.push_str(&format!("  Hom(ℤ/{}ℤ, M) ≅ {{m ∈ M : nm = 0}}  (the n-torsion of M)\n\n", n));
    out.push_str("  Values of F on abelian groups:\n");
    for &k in &[2i64, 3, 4, 6, 12] {
        let val = hom_count(n, k);
        out.push_str(&format!("    F(ℤ/{}ℤ) = Hom(ℤ/{}ℤ, ℤ/{}ℤ) = ℤ/{}ℤ  ({} elements)\n",
            k, n, k, val, val));
    }
    out.push_str("\n=== Yoneda: F(A) represents all natural transformations ===\n\n");
    out.push_str(&format!("  Nat(Hom(ℤ/{}ℤ,-), F) ≅ F(ℤ/{}ℤ) = Hom(ℤ/{}ℤ, ℤ/{}ℤ)\n", n, n, n, n));
    let self_homs = hom_count(n, n);
    out.push_str(&format!("  |Nat(Hom(ℤ/{}ℤ,-), F)| = {}\n\n", n, self_homs));
    out.push_str(&format!("The functor Hom(ℤ/{}ℤ,-) is represented by ℤ/{}ℤ.\n", n, n));
    out.push_str("Yoneda: to understand a representable functor, it suffices to study its representing object.\n");
    out
}

fn cmd_functor_category(n: i64, m: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Natural Transformations Hom(ℤ/{}ℤ,-) → Hom(ℤ/{}ℤ,-) ===\n\n", n, m));
    out.push_str("By Yoneda, Nat(Hom(A,-), Hom(B,-)) ≅ Hom(B, A).\n\n");
    out.push_str(&format!("  A = ℤ/{}ℤ, B = ℤ/{}ℤ\n", n, m));
    out.push_str(&format!("  Nat(Hom(ℤ/{}ℤ,-), Hom(ℤ/{}ℤ,-)) ≅ Hom(ℤ/{}ℤ, ℤ/{}ℤ)\n", n, m, m, n));
    let count = hom_count(m, n);
    out.push_str(&format!("Count = gcd({},{}) = {}\n\n", m, n, count));
    out.push_str("  Explicitly, these natural transformations η come from morphisms φ: ℤ/mℤ → ℤ/nℤ:\n");
    let homs = list_homs_zn_to_zk(m, n);
    for (i, a) in homs.iter().enumerate() {
        out.push_str(&format!("    η_{}: precompose with (x ↦ {}x: ℤ/{}ℤ → ℤ/{}ℤ)\n", i, a, m, n));
        let k = lcm(n, m);
        let before: Vec<i64> = list_homs_zn_to_zk(m, k);
        let after: Vec<i64> = list_homs_zn_to_zk(n, k);
        out.push_str(&format!("      On ℤ/{}ℤ: Hom(ℤ/{}ℤ,ℤ/{}ℤ) has {} elements, Hom(ℤ/{}ℤ,ℤ/{}ℤ) has {}\n",
            k, m, k, before.len(), n, k, after.len()));
    }
    out.push_str("\n=== The Functor Category Ab^Ab ===\n\n");
    out.push_str("  The collection of all functors Ab → Set forms a category Fun(Ab, Set).\n");
    out.push_str("  Morphisms in Fun(Ab, Set) are natural transformations.\n");
    out.push_str("  Yoneda embeds Ab → Fun(Ab, Set) fully faithfully.\n");
    out.push_str("Yoneda embedding: A ↦ Hom(A,-) is fully faithful.\n");
    out.push_str("This means: morphisms A → B in Ab correspond bijectively to natural\n");
    out.push_str("transformations Hom(A,-) → Hom(B,-). Category theory in action!\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "n", 6);
    state_set_int(&mut s, "m", 4);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "natural" => {
            let n = match parse_uint(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            if n < 1 { return "n must be >= 1\n".to_string(); }
            state_set_int(state, "n", n as i64);
            cmd_natural(n)
        }
        "yoneda" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            if n <= 0 { return "n must be positive\n".to_string(); }
            state_set_int(state, "n", n);
            cmd_yoneda(n)
        }
        "double_dual" => cmd_double_dual(),
        "representable" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            if n <= 0 { return "n must be positive\n".to_string(); }
            state_set_int(state, "n", n);
            cmd_representable(n)
        }
        "functor_category" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let m = match parse_int(args, 1, "m") { Some(v) => v, None => return "Missing arg m\n".to_string() };
            if n <= 0 || m <= 0 { return "n, m must be positive\n".to_string(); }
            state_set_int(state, "n", n);
            state_set_int(state, "m", m);
            cmd_functor_category(n, m)
        }
        "demo" => {
            let mut out = String::new();
            out.push_str(&cmd_yoneda(6));
            out.push_str("\n");
            out.push_str(&cmd_double_dual());
            out.push_str("\n");
            out.push_str(&cmd_representable(6));
            out
        }
        "help" | "h" => show_help(),
        _ => format!("Unknown command '{}'. Type 'help'.", cmd),
    }
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { println!("{out}"); }
    true
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    // Yoneda embedding diagram
    c.text_bold(350.0, 30.0, "Yoneda Embedding", 14.0, "#222", "middle");
    // Category Ab on left
    c.rect(60.0, 80.0, 200.0, 340.0, "#f9f9ff", "#888", 1.0);
    c.text_bold(160.0, 70.0, "Ab", 13.0, "#333", "middle");
    c.node_circle(160.0, 160.0, "A", "#eef", 25.0, 13.0);
    c.node_circle(160.0, 310.0, "B", "#eef", 25.0, 13.0);
    c.arrow(160.0, 185.0, 160.0, 285.0, "#444", 1.5);
    c.text(175.0, 237.0, "f: A\u{2192}B", 11.0, "#555", "start");
    // Functor category on right
    c.rect(440.0, 80.0, 220.0, 340.0, "#fff9f9", "#888", 1.0);
    c.text_bold(550.0, 70.0, "Fun(Ab,Set)", 13.0, "#333", "middle");
    c.node_circle(550.0, 160.0, &format!("Hom(Z/{}Z,-)", n), "#fee", 40.0, 10.0);
    c.node_circle(550.0, 330.0, "Hom(B,-)", "#fee", 35.0, 11.0);
    c.arrow(550.0, 200.0, 550.0, 293.0, "#444", 1.5);
    c.text(595.0, 247.0, "nat. trans.", 11.0, "#555", "start");
    // Yoneda functor arrows between categories
    c.arrow(265.0, 200.0, 430.0, 200.0, "#226", 1.8);
    c.text(350.0, 185.0, "Y (Yoneda)", 12.0, "#226", "middle");
    c.arrow(265.0, 310.0, 430.0, 310.0, "#226", 1.8);
    // Info
    c.text(350.0, 470.0, &format!("Y(Z/{}Z) = Hom(Z/{}Z,-)  \u{2014} fully faithful embedding", n, n), 11.0, "#444", "middle");
}

fn visualize_dot(g_dot: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    g_dot.node("A",    &[("label", &format!("Z/{}Z", n)), ("shape", "circle")]);
    g_dot.node("B",    &[("label", "B"), ("shape", "circle")]);
    g_dot.node("HomA", &[("label", &format!("Hom(Z/{}Z,-)", n)), ("shape", "box")]);
    g_dot.node("HomB", &[("label", "Hom(B,-)"), ("shape", "box")]);
    g_dot.edge("A",    "B",    &[("label", "f in Ab")]);
    g_dot.edge("HomA", "HomB", &[("label", "nat. trans.")]);
    g_dot.edge("A",    "HomA", &[("label", "Yoneda Y"), ("style", "dashed")]);
    g_dot.edge("B",    "HomB", &[("label", "Yoneda Y"), ("style", "dashed")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    t.node("A",    -3.0, 2.0, &format!("$\\mathbb{{Z}}/{}\\mathbb{{Z}}$", n), "draw");
    t.node("B",    -3.0, 0.0, "$B$", "draw");
    t.node("HomA",  3.0, 2.0, &format!("$\\text{{Hom}}(\\mathbb{{Z}}/{}\\mathbb{{Z}},-)$", n), "draw");
    t.node("HomB",  3.0, 0.0, "$\\text{Hom}(B,-)$", "draw");
    t.arrow("A", "B",       "$f$", "->");
    t.arrow("HomA", "HomB", "$\\eta$", "->");
    t.arrow("A", "HomA", "$Y$", "->,dashed");
    t.arrow("B", "HomB", "$Y$", "->,dashed");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    a.text_at(2, 1,  "Yoneda Embedding: Ab -> Fun(Ab,Set)");
    a.text_at(2, 3,  &format!("  Z/{}Z  ------Y------->  Hom(Z/{}Z,-)", n, n));
    a.text_at(2, 4,  "    |                          |");
    a.text_at(2, 5,  "    f                        nat.trans.");
    a.text_at(2, 6,  "    |                          |");
    a.text_at(2, 7,  "    B  ------Y------->  Hom(B,-)");
    a.text_at(2, 9,  "Yoneda Lemma: Nat(Hom(A,-),F) = F(A)");
    a.text_at(2, 10, "The embedding Y is fully faithful.");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch34"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 34", "Natural Transformations and the Yoneda Lemma", "Every functor is determined by its values on morphisms");
            print_note("Explore naturality, the Yoneda lemma, and representable functors.");
            println!("{}", show_help());
            repl("yoneda> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
