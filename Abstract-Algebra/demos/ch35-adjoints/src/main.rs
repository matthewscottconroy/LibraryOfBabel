use common::*;

fn show_help() -> String {
    help_string(&[
        ("free_forget <n> <m>",    "Free-forgetful adjunction for S={1..n} and G=Z/mZ"),
        ("tensor_hom <n> <m> <k>","Tensor-Hom adjunction: Hom(Z/nZ*Z/mZ,Z/kZ) = Hom(Z/nZ,Hom(Z/mZ,Z/kZ))"),
        ("unit_counit <n> <m>",   "Unit and counit of the free-forgetful adjunction"),
        ("preserve_limits",       "Right adjoints preserve limits: U(G*H) = U(G)*U(H)"),
        ("galois_adjunction <n>", "Galois correspondence as an adjunction"),
        ("examples",              "Classic adjunctions"),
        ("demo",                  "Run a showcase of key results"),
        ("help",                  "Show this help"),
        ("quit",                  "Exit"),
    ])
}

fn hom_free_to_zn(gen_count: u64, m: u64) -> u64 {
    m.pow(gen_count as u32)
}

fn set_maps(s_size: u64, m: u64) -> u64 {
    m.pow(s_size as u32)
}

fn cmd_free_forget(n: u64, m: u64) -> String {
    let mut out = String::new();
    out.push_str("=== Free-Forgetful Adjunction ===\n\n");
    out.push_str("Adjunction: F⊣U where F: Set→Ab (free abelian) and U: Ab→Set (forgetful).\n");
    out.push_str("Hom_Ab(F(S), G) ≅ Hom_Set(S, U(G))\n\n");
    out.push_str(&format!("  S = {{1, 2, ..., {}}} (a set of {} elements)\n", n, n));
    out.push_str(&format!("  G = ℤ/{}ℤ (the group)\n\n", m));
    let lhs = hom_free_to_zn(n, m);
    out.push_str("  LHS: Hom_Ab(F(S), G) = Hom_Ab(ℤⁿ, ℤ/mℤ)\n");
    out.push_str(&format!("    F(S) = ℤ^{} (free abelian group on {} generators)\n", n, n));
    out.push_str(&format!("    Each generator can map to any of {} elements of ℤ/{}ℤ\n", m, m));
    out.push_str(&format!("    So |Hom_Ab(ℤ^{}, ℤ/{}ℤ)| = {}^{} = {}\n\n", n, m, m, n, lhs));
    let rhs = set_maps(n, m);
    out.push_str("  RHS: Hom_Set(S, U(G)) = {functions {1..n} → ℤ/mℤ as a set}\n");
    out.push_str(&format!("    |U(ℤ/{}ℤ)| = {} (as a set)\n", m, m));
    out.push_str(&format!("    Number of set functions = {}^{} = {}\n\n", m, n, rhs));
    if lhs == rhs {
        out.push_str(&format!("Both sides = {} — adjunction verified!\n\n", lhs));
    } else {
        out.push_str(&format!("Mismatch: LHS={}, RHS={}\n\n", lhs, rhs));
    }
    out.push_str("  Explicit bijection:\n");
    out.push_str("    A group homomorphism φ: ℤⁿ → ℤ/mℤ is given by n integers (a₁,...,aₙ) with\n");
    out.push_str("    φ(e_i) = aᵢ, where e_i is the i-th standard basis vector.\n");
    out.push_str("    This corresponds to the set function f: {1..n} → ℤ/mℤ sending i ↦ aᵢ.\n\n");
    if n <= 2 && m <= 4 {
        out.push_str("  Explicit list of homomorphisms:\n");
        let mut idx = 0;
        for a1 in 0..m {
            if n == 1 {
                out.push_str(&format!("    φ_{}: e₁ ↦ {}\n", idx, a1));
                idx += 1;
            } else {
                for a2 in 0..m {
                    out.push_str(&format!("    φ_{}: e₁↦{}, e₂↦{}\n", idx, a1, a2));
                    idx += 1;
                }
            }
        }
        out.push_str("\n");
    }
    out.push_str("The adjunction shows: 'giving a group hom from a free group' is the same\n");
    out.push_str("as 'choosing where the generators go' — a set-level choice.\n");
    out
}

fn cmd_tensor_hom(n: i64, m: i64, k: i64) -> String {
    let mut out = String::new();
    out.push_str("=== Tensor-Hom Adjunction ===\n\n");
    out.push_str("Hom(A⊗B, C) ≅ Hom(A, Hom(B, C))  (Tensor-Hom adjunction)\n\n");
    out.push_str(&format!("  A = ℤ/{}ℤ,  B = ℤ/{}ℤ,  C = ℤ/{}ℤ\n\n", n, m, k));
    let g = gcd(n, m);
    let lhs_count = gcd(g, k);
    out.push_str("  LHS: Hom(ℤ/nℤ ⊗ ℤ/mℤ, ℤ/kℤ)\n");
    out.push_str(&format!("     = Hom(ℤ/gcd({},{})ℤ, ℤ/{}ℤ)  [since ℤ/{}⊗ℤ/{} ≅ ℤ/gcd({},{})ℤ]\n",
        n, m, k, n, m, n, m));
    out.push_str(&format!("     = gcd(gcd({},{}), {}) = gcd({},{}) = {}\n\n", n, m, k, g, k, lhs_count));
    let inner = gcd(m, k);
    let rhs_count = gcd(n, inner);
    out.push_str("  RHS: Hom(ℤ/nℤ, Hom(ℤ/mℤ, ℤ/kℤ))\n");
    out.push_str(&format!("     = Hom(ℤ/{}ℤ, ℤ/gcd({},{})ℤ)  [inner Hom = ℤ/gcd({},{})ℤ]\n",
        n, m, k, m, k));
    out.push_str(&format!("     = gcd({}, gcd({},{})) = gcd({},{}) = {}\n\n", n, m, k, n, inner, rhs_count));
    if lhs_count == rhs_count {
        out.push_str(&format!("Both sides = {} — Tensor-Hom adjunction verified!\n\n", lhs_count));
    } else {
        out.push_str(&format!("Mismatch! LHS={}, RHS={} (check input).\n\n", lhs_count, rhs_count));
    }
    out.push_str("  The natural bijection explicitly:\n");
    out.push_str("    φ: ℤ/nℤ⊗ℤ/mℤ → ℤ/kℤ  corresponds to  ψ: ℤ/nℤ → Hom(ℤ/mℤ,ℤ/kℤ)\n");
    out.push_str("    via  ψ(a)(b) = φ(a⊗b)  (currying)\n");
    out.push_str("    and  φ(a⊗b) = ψ(a)(b)  (uncurrying)\n\n");
    out.push_str("This is the categorical version of currying from computer science!\n");
    out
}

fn cmd_unit_counit(n: u64, m: u64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Unit and Counit of F⊣U for ℤ/{}ℤ and S of size {} ===\n\n", m, n));
    out.push_str("An adjunction F⊣U comes with:\n");
    out.push_str("  Unit:    η: Id_C → U∘F  (embed objects into the free construction)\n");
    out.push_str("  Counit:  ε: F∘U → Id_D  (quotient the free object back down)\n\n");
    out.push_str("=== Unit η: S → U(F(S)) ===\n\n");
    out.push_str(&format!("  S = {{1, 2, ..., {}}} (a set)\n", n));
    out.push_str(&format!("  F(S) = ℤ^{} (free abelian group)\n", n));
    out.push_str(&format!("  U(F(S)) = ℤ^{} viewed as a set\n\n", n));
    out.push_str("  η_S: S → U(F(S)) = inclusion of generators\n");
    for i in 1..=n {
        let mut basis = vec![0i64; n as usize];
        basis[(i-1) as usize] = 1;
        out.push_str(&format!("    η({}) = e_{} = ({}) ∈ ℤ^{}\n",
            i, i,
            basis.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
            n));
    }
    out.push_str("\n");
    out.push_str(&format!("=== Counit ε: F(U(G)) → G for G = ℤ/{}ℤ ===\n\n", m));
    out.push_str(&format!("  G = ℤ/{}ℤ\n", m));
    out.push_str(&format!("  U(G) = {{0, 1, ..., {}}} (the underlying set)\n", m-1));
    out.push_str(&format!("  F(U(G)) = ℤ^{} (free abelian on all elements of G)\n\n", m));
    out.push_str("  ε_G: F(U(G)) → G\n");
    out.push_str(&format!("    ε sends basis vector e_{{g}} ↦ g ∈ ℤ/{}ℤ\n", m));
    out.push_str("    (i.e., the generator labelled by g maps to g itself)\n\n");
    out.push_str("  Triangle identities:\n");
    out.push_str("    ε_{F(S)} ∘ F(η_S) = id_{F(S)}\n");
    out.push_str("    U(ε_G) ∘ η_{U(G)} = id_{U(G)}\n\n");
    out.push_str("The unit η shows: every set embeds into the free group on it.\n");
    out.push_str("The counit ε shows: the free group on a group maps back to that group.\n");
    out
}

fn cmd_preserve_limits() -> String {
    let mut out = String::new();
    out.push_str("=== Right Adjoints Preserve Limits ===\n\n");
    out.push_str("Theorem: Right adjoints preserve limits.\n");
    out.push_str("Equivalently, left adjoints preserve colimits.\n\n");
    out.push_str("  Demonstration: U: Ab → Set is right adjoint to F: Set → Ab.\n");
    out.push_str("  Therefore U preserves limits, in particular: products.\n\n");
    out.push_str("=== U preserves products: U(G × H) = U(G) × U(H) ===\n\n");
    for &(n, m) in &[(2u64, 3u64), (3, 4), (4, 6)] {
        let g_order = n;
        let h_order = m;
        let prod_order = n * m;
        out.push_str(&format!("  G = ℤ/{}ℤ (order {}), H = ℤ/{}ℤ (order {})\n", n, g_order, m, h_order));
        out.push_str(&format!("    |U(G×H)| = {} × {} = {}\n", n, m, prod_order));
        out.push_str(&format!("    |U(G)×U(H)| = {} × {} = {}\n", n, m, prod_order));
        out.push_str(&format!("    Sizes match: U(ℤ/{}×ℤ/{}) = U(ℤ/{}) × U(ℤ/{})\n\n", n, m, n, m));
    }
    out.push_str("=== Explicit Product for ℤ/2ℤ × ℤ/3ℤ ===\n\n");
    out.push_str("  G = ℤ/2ℤ = {0,1}, H = ℤ/3ℤ = {0,1,2}\n");
    out.push_str("  G × H = {(g,h) : g∈G, h∈H}:\n");
    for g in 0..2u64 {
        for h in 0..3u64 {
            out.push_str(&format!("    ({}, {})\n", g, h));
        }
    }
    out.push_str("  |G×H| = 6 = |G|·|H|  ✓\n\n");
    out.push_str("U preserves ALL limits: products, equalizers, pullbacks, etc.\n");
    out.push_str("F (free abelian) preserves colimits: coproducts (disjoint union of generators), etc.\n");
    out
}

fn cmd_galois_adjunction(n: u64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Galois Correspondence as an Adjunction (ℚ(ζ_{})) ===\n\n", n));
    out.push_str("The Galois correspondence is an adjunction between:\n");
    out.push_str("  - Subgroups of G = Gal(K/F)  (ordered by inclusion)\n");
    out.push_str("  - Intermediate fields F ⊆ E ⊆ K  (ordered by inclusion)\n\n");
    let phi = euler_totient(n);
    let (units, _) = zn_mul_table(n as usize);
    out.push_str(&format!("  K = ℚ(ζ_{}),  F = ℚ\n", n));
    out.push_str(&format!("  G = Gal(K/ℚ) ≅ (ℤ/{}ℤ)* = {{{}}}\n",
        n, units.iter().map(|u| u.to_string()).collect::<Vec<_>>().join(", ")));
    out.push_str(&format!("  |G| = {} = φ({})\n\n", phi, n));
    out.push_str("=== The Two Functors ===\n\n");
    out.push_str("  Fix: Subgroups(G)^op → IntermediateFields(K/F)\n");
    out.push_str("       H ↦ K^H = {elements of K fixed by all σ ∈ H}\n\n");
    out.push_str("  Gal: IntermediateFields(K/F)^op → Subgroups(G)\n");
    out.push_str("       E ↦ Gal(K/E) = {σ ∈ G : σ fixes E pointwise}\n\n");
    out.push_str("=== The Adjunction ===\n\n");
    out.push_str("  H ≤ Gal(K/Fix(H))    [unit: H ↦ Gal(K/K^H)]\n");
    out.push_str("  Fix(Gal(K/E)) = E     [counit: Fix(Gal(K/E)) = E  for Galois extensions]\n\n");
    out.push_str("The pair (Fix, Gal) form an adjunction (actually a Galois connection = adjunction on posets).\n\n");
    let subs = subgroups_zn(phi as usize);
    out.push_str("=== Subgroup-Field Correspondence ===\n\n");
    out.push_str("  Subgroups of G (by index) ↔ Intermediate fields (by degree over F):\n");
    for sg in &subs {
        let h_order = sg.len();
        let field_degree = phi as usize / h_order;
        out.push_str(&format!("    H of order {} ↔ Fixed field of degree {} over ℚ\n",
            h_order, field_degree));
    }
    out.push_str("\nGalois correspondence: order-reversing bijection (larger subgroup = smaller fixed field).\n");
    out
}

fn cmd_examples() -> String {
    let mut out = String::new();
    out.push_str("=== Classic Adjunctions ===\n\n");
    out.push_str(&format!("  {:<28} {:<16} {:<18} {}\n", "Adjunction", "Left", "Right", "Categories"));
    out.push_str("  ---------------------------------------------------------------------------\n");
    let rows = [
        ("Free ⊣ Forgetful",        "F: Set→Grp",     "U: Grp→Set",     "Grp / Set"),
        ("Free abelian ⊣ Forget",   "F: Set→Ab",      "U: Ab→Set",      "Ab / Set"),
        ("Tensor ⊣ Hom",            "-⊗M: Mod→Mod",   "Hom(M,-)",       "R-Mod"),
        ("Extension ⊣ Restriction", "-⊗_R S",         "Res: S-Mod→R-Mod","rings R→S"),
        ("Σ ⊣ Ω",                   "Σ: Top*→Top*",   "Ω: Top*→Top*",   "Top*"),
        ("∃ ⊣ f* ⊣ ∀",              "∃ (image)",      "f* (pullback)",   "logic/Set"),
        ("π₀ ⊣ disc",               "π₀: Top→Set",    "disc: Set→Top",   "discrete top."),
        ("Colim ⊣ Const ⊣ Lim",     "colim",          "lim",             "diagram cats"),
    ];
    for (adj, left, right, cats) in &rows {
        out.push_str(&format!("  {:<28} {:<16} {:<18} {}\n", adj, left, right, cats));
    }
    out.push_str("\nA pair F⊣G is an adjunction if Hom(F(A),B) ≅ Hom(A,G(B)) naturally in A and B.\n");
    out.push_str("Equivalently, there exist unit η: Id→G∘F and counit ε: F∘G→Id satisfying triangle identities.\n\n");
    out.push_str("=== Adjunctions in Logic ===\n\n");
    out.push_str("  Substitution f*: Pred(Y) → Pred(X)  for a map f: X → Y\n");
    out.push_str("  This has adjoints:\n");
    out.push_str("    ∃_f ⊣ f* ⊣ ∀_f\n");
    out.push_str("    ∃_f: existential quantification along f\n");
    out.push_str("    ∀_f: universal quantification along f\n\n");
    out.push_str("This captures: '∃x.P(x,y)' corresponds to the image under projection,\n");
    out.push_str("and '∀x.P(x,y)' corresponds to sections over all fibers.\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "n", 3);
    state_set_int(&mut s, "m", 6);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "free_forget" => {
            let n = match parse_uint(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let m = match parse_uint(args, 1, "m") { Some(v) => v, None => return "Missing arg m\n".to_string() };
            if n == 0 || m == 0 { return "n and m must be positive\n".to_string(); }
            state_set_int(state, "n", n as i64);
            state_set_int(state, "m", m as i64);
            cmd_free_forget(n, m)
        }
        "tensor_hom" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let m = match parse_int(args, 1, "m") { Some(v) => v, None => return "Missing arg m\n".to_string() };
            let k = match parse_int(args, 2, "k") { Some(v) => v, None => return "Missing arg k\n".to_string() };
            if n <= 0 || m <= 0 || k <= 0 { return "n, m, k must be positive\n".to_string(); }
            cmd_tensor_hom(n, m, k)
        }
        "unit_counit" => {
            let n = match parse_uint(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let m = match parse_uint(args, 1, "m") { Some(v) => v, None => return "Missing arg m\n".to_string() };
            if n == 0 || m == 0 { return "n and m must be positive\n".to_string(); }
            state_set_int(state, "n", n as i64);
            state_set_int(state, "m", m as i64);
            cmd_unit_counit(n, m)
        }
        "preserve_limits" => {
            state_set_str(state, "last_cmd", "preserve_limits");
            cmd_preserve_limits()
        }
        "galois_adjunction" => {
            let n = match parse_uint(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            if n < 2 { return "n must be >= 2\n".to_string(); }
            state_set_int(state, "n", n as i64);
            cmd_galois_adjunction(n)
        }
        "examples" => {
            state_set_str(state, "last_cmd", "examples");
            cmd_examples()
        }
        "demo" => {
            let mut out = String::new();
            out.push_str(&cmd_examples());
            out.push_str("\n");
            out.push_str(&cmd_tensor_hom(6, 4, 12));
            out.push_str("\n");
            out.push_str(&cmd_galois_adjunction(5));
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

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    // Adjoint pair (L ⊣ R) diagram
    c.text_bold(350.0, 30.0, "Adjoint Pair: L \u{22a3} R", 14.0, "#222", "middle");
    // Two categories C (left) and D (right)
    c.rect(50.0, 80.0, 220.0, 350.0, "#f9f9ff", "#888", 1.0);
    c.text_bold(160.0, 72.0, "C", 13.0, "#333", "middle");
    c.rect(430.0, 80.0, 220.0, 350.0, "#fff9f9", "#888", 1.0);
    c.text_bold(540.0, 72.0, "D", 13.0, "#333", "middle");
    // Objects
    c.node_circle(160.0, 200.0, "A", "#eef", 25.0, 13.0);
    c.node_circle(160.0, 330.0, "A'", "#eef", 25.0, 12.0);
    c.node_circle(540.0, 200.0, "B", "#fee", 25.0, 13.0);
    c.node_circle(540.0, 330.0, "B'", "#fee", 25.0, 12.0);
    // Morphism arrows within categories
    c.arrow(160.0, 225.0, 160.0, 304.0, "#444", 1.5);
    c.text(175.0, 265.0, "f", 12.0, "#333", "start");
    c.arrow(540.0, 225.0, 540.0, 304.0, "#444", 1.5);
    c.text(555.0, 265.0, "g", 12.0, "#333", "start");
    // Adjoint functor arrows between categories
    c.arrow(280.0, 170.0, 420.0, 170.0, "#226", 2.0);
    c.text(350.0, 155.0, "L (left adjoint)", 12.0, "#226", "middle");
    c.arrow(420.0, 230.0, 280.0, 230.0, "#622", 2.0);
    c.text(350.0, 248.0, "R (right adjoint)", 12.0, "#622", "middle");
    // Adjunction iso box
    c.rect(145.0, 443.0, 410.0, 30.0, "#fffbe6", "#888", 1.0);
    c.text(350.0, 462.0, "Hom_D(L(A), B) \u{2245} Hom_C(A, R(B))   naturally in A,B", 11.0, "#333", "middle");
}

fn visualize_dot(g_dot: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g_dot.node("C", &[("label", "Category C"), ("shape", "box")]);
    g_dot.node("D", &[("label", "Category D"), ("shape", "box")]);
    g_dot.edge("C", "D", &[("label", "L (left adj)")]);
    g_dot.edge("D", "C", &[("label", "R (right adj)")]);
    g_dot.node("iso", &[("label", "Hom(L(A),B) = Hom(A,R(B))"), ("shape", "note")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("C", -3.0, 0.0, "$\\mathcal{C}$", "draw");
    t.node("D",  3.0, 0.0, "$\\mathcal{D}$", "draw");
    t.arrow("C", "D", "$L$", "->,bend left=20");
    t.arrow("D", "C", "$R$", "->,bend left=20");
    t.node("adj", 0.0, -2.0, "$L \\dashv R$", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1,  "Adjoint Pair L -| R");
    a.text_at(5, 3,  "+-------+         +-------+");
    a.text_at(5, 4,  "|       |  --L-->  |       |");
    a.text_at(5, 5,  "|   C   |          |   D   |");
    a.text_at(5, 6,  "|       |  <--R--  |       |");
    a.text_at(5, 7,  "+-------+         +-------+");
    a.text_at(2, 9,  "Hom_D(L(A), B) = Hom_C(A, R(B))  [naturally]");
    a.text_at(2, 11, "Unit:   eta: id_C -> R*L");
    a.text_at(2, 12, "Counit: eps: L*R -> id_D");
    a.text_at(2, 13, "Triangle identities: eps_L * L(eta) = id_L");
    a.text_at(2, 14, "                     R(eps) * eta_R = id_R");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch35"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 35", "Adjoint Functors", "The most ubiquitous structure in mathematics");
            print_note("Explore free-forgetful, tensor-hom, and Galois adjunctions.");
            println!("{}", show_help());
            repl("adjoint> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
