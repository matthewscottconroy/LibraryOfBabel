use common::*;

fn show_help() -> String {
    help_string(&[
        ("category <name>",              "Describe a category (Set/Grp/Vect/Top/Pos)"),
        ("functor <name>",               "Describe a functor"),
        ("compose_functor",              "Show functor composition"),
        ("commute <f1> <f2> <g1> <g2>", "Check commutativity of a square of Z/nZ maps"),
        ("isomorphism <n> <m>",          "Automorphisms of Z/nZ (n=m required)"),
        ("opposite",                     "Explain opposite category"),
        ("examples",                     "Table of categories and their morphisms"),
        ("demo",                         "Run a showcase of key results"),
        ("help",                         "Show this help"),
        ("quit",                         "Exit"),
    ])
}

fn cmd_category(name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Category: {} ===\n\n", name.to_uppercase()));
    match name.to_lowercase().as_str() {
        "set" => {
            out.push_str("Objects:    sets\n");
            out.push_str("Morphisms:  functions between sets\n");
            out.push_str("Composition: function composition  (g∘f)(x) = g(f(x))\n");
            out.push_str("Identity:   id_A: A→A, x↦x\n\n");
            out.push_str("  Category axioms:\n");
            out.push_str("    Associativity: h∘(g∘f) = (h∘g)∘f for all composable f,g,h\n");
            out.push_str("    Unit: f∘id_A = f = id_B∘f for f: A→B\n\n");
            out.push_str("Set is the archetypal category. Most concrete structures live over Set.\n");
        }
        "grp" => {
            out.push_str("Objects:    groups (G, ·, e)\n");
            out.push_str("Morphisms:  group homomorphisms φ: G→H (φ(xy)=φ(x)φ(y))\n");
            out.push_str("Composition: composition of homomorphisms\n");
            out.push_str("Identity:   id_G: G→G, g↦g\n\n");
            out.push_str("  Special morphisms:\n");
            out.push_str("    Monomorphism = injective homomorphism\n");
            out.push_str("    Epimorphism  = surjective homomorphism (in Grp)\n");
            out.push_str("    Isomorphism  = bijective homomorphism\n\n");
            out.push_str("Example: ℤ→ℤ/nℤ by x↦x mod n is an epimorphism but not split.\n");
        }
        "vect" => {
            out.push_str("Objects:    vector spaces over a field k\n");
            out.push_str("Morphisms:  linear maps T: V→W\n");
            out.push_str("Composition: composition of linear maps\n");
            out.push_str("Identity:   id_V: V→V\n\n");
            out.push_str("  Vect_k is an abelian category.\n");
            out.push_str("  Every morphism has a kernel and cokernel.\n");
            out.push_str("  Every short exact sequence 0→V→W→U→0 splits (Vect is semisimple).\n\n");
            out.push_str("If k is a field, every subspace has a complement.\n");
        }
        "top" => {
            out.push_str("Objects:    topological spaces (X, τ)\n");
            out.push_str("Morphisms:  continuous maps f: X→Y\n");
            out.push_str("Composition: composition of continuous maps\n");
            out.push_str("Identity:   id_X: X→X (trivially continuous)\n\n");
            out.push_str("  Key functors out of Top:\n");
            out.push_str("    π₁: Top* → Grp  (fundamental group)\n");
            out.push_str("    H_n: Top → Ab   (homology groups)\n");
            out.push_str("    H^n: Top^op → Ab (cohomology — contravariant)\n\n");
            out.push_str("Homeomorphism = isomorphism in Top.\n");
        }
        "pos" => {
            out.push_str("Objects:    partially ordered sets (P, <=)\n");
            out.push_str("Morphisms:  order-preserving maps: a<=b → f(a)<=f(b)\n");
            out.push_str("Composition: composition of order-preserving maps\n");
            out.push_str("Identity:   id_P: P→P\n\n");
            out.push_str("  Special feature: hom-sets have at most 1 element.\n");
            out.push_str("    Hom(a,b) = {*} if a<=b, else empty\n");
            out.push_str("  A poset is a category: objects=elements, morphisms='<='.\n\n");
            out.push_str("Limits in Pos = meets (infima). Colimits = joins (suprema).\n");
            out.push_str("Adjunctions between posets = Galois connections.\n");
        }
        _ => {
            out.push_str(&format!("Unknown category '{}'. Try: Set, Grp, Vect, Top, Pos\n", name));
        }
    }
    out
}

fn cmd_functor(name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Functor: {} ===\n\n", name));
    match name.to_lowercase().as_str() {
        "forget_grp" | "forgetful" => {
            out.push_str("Name:        Forgetful functor U: Grp → Set\n");
            out.push_str("On objects:  U(G) = underlying set of G\n");
            out.push_str("On morphisms: U(φ) = φ as a function of sets\n\n");
            out.push_str("  Properties:\n");
            out.push_str("    Faithful: U(φ)=U(ψ) implies φ=ψ\n");
            out.push_str("    Not full: not every set function is a group homomorphism\n\n");
            out.push_str("U has a left adjoint: the free group functor F: Set → Grp.\n");
        }
        "free_grp" | "free" => {
            out.push_str("Name:        Free group functor F: Set → Grp\n");
            out.push_str("On objects:  F(S) = free group on generators S\n");
            out.push_str("On morphisms: F(f) = unique homo extending f on generators\n\n");
            out.push_str("  Universal property: for any group G and f: S→G,\n");
            out.push_str("    there is a unique φ: F(S)→G with φ∘ι = f.\n\n");
            out.push_str("  Examples:\n");
            out.push_str("    F(∅) = {e} (trivial group)\n");
            out.push_str("    F({a}) = ℤ (infinite cyclic)\n");
            out.push_str("    F({a,b}) = free group on 2 generators\n\n");
            out.push_str("F⊣U: Hom_Grp(F(S),G) ≅ Hom_Set(S,U(G)).\n");
        }
        "fundamental_group" => {
            out.push_str("Name:        Fundamental group π₁: Top* → Grp\n");
            out.push_str("On objects:  π₁(X,x₀) = loops at x₀ up to homotopy\n");
            out.push_str("On morphisms: f*: π₁(X,x₀) → π₁(Y,f(x₀))\n\n");
            out.push_str("  π₁(S¹) = ℤ, π₁(T²) = ℤ×ℤ, π₁(RP²) = ℤ/2ℤ\n");
        }
        "homology" => {
            out.push_str("Name:        Singular homology H_n: Top → Ab\n");
            out.push_str("On objects:  H_n(X) = n-th homology group\n");
            out.push_str("On morphisms: f_*: H_n(X) → H_n(Y)\n\n");
            out.push_str("Homotopy equivalent spaces have isomorphic homology.\n");
        }
        "dual_vect" | "dual" => {
            out.push_str("Name:        Dual functor (-)* : Vect^op → Vect\n");
            out.push_str("On objects:  V* = Hom(V,k)\n");
            out.push_str("On morphisms: T*: W*→V* by T*(φ) = φ∘T (contravariant)\n\n");
            out.push_str("V → V** is natural. V → V* requires a basis choice (not natural).\n");
        }
        _ => {
            out.push_str(&format!("Unknown functor '{}'. Try: forget_grp, free_grp, fundamental_group, homology, dual_vect\n", name));
        }
    }
    out
}

fn cmd_compose_functor() -> String {
    let mut out = String::new();
    out.push_str("=== Functor Composition: Grp →(U)→ Set →(F)→ Grp ===\n\n");
    out.push_str("We can compose functors, just like we compose functions.\n\n");
    out.push_str("  U: Grp → Set  (forgetful)\n");
    out.push_str("  F: Set → Grp  (free group)\n");
    out.push_str("  F∘U: Grp → Grp  (composition)\n\n");
    out.push_str("  On a group G: (F∘U)(G) = F(U(G)) = free group on elements of G\n\n");
    out.push_str("  Example: G = ℤ/3ℤ = {0,1,2}\n");
    out.push_str("    U(ℤ/3ℤ) = {0,1,2}  (as a set, forgetting group structure)\n");
    out.push_str("    F({0,1,2}) = free group on 3 generators — infinite, non-abelian!\n\n");
    out.push_str("F∘U != identity functor on Grp.\n\n");
    out.push_str("  Unit of the adjunction η: id_Grp → F∘U:\n");
    out.push_str("    η_G: G → F(U(G))  sends g ∈ G to itself as a generator.\n");
    out.push_str("    The group homomorphism from F(U(G)) → G (via the adjunction) is the quotient.\n\n");
    out.push_str("The pair (F, U) is an adjunction: F⊣U.\n");
    out.push_str("η and ε (unit and counit) measure how far F∘U and U∘F are from identities.\n");
    out
}

fn cmd_commute(f1_desc: &str, f2_desc: &str, g1_desc: &str, g2_desc: &str) -> String {
    let mut out = String::new();
    out.push_str("=== Checking Commutativity of a Square ===\n\n");
    out.push_str("  Format: each map is 'a:n' meaning x↦ax on ℤ/nℤ\n\n");
    fn parse_map(s: &str) -> Option<(i64, i64)> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 2 {
            let a = parts[0].parse::<i64>().ok()?;
            let n = parts[1].parse::<i64>().ok()?;
            Some((a, n))
        } else { None }
    }
    let f1 = match parse_map(f1_desc) {
        Some(v) => v,
        None => return "Bad f1 format. Use a:n e.g. 2:6\n".to_string(),
    };
    let f2 = match parse_map(f2_desc) { Some(v) => v, None => return "Bad f2\n".to_string() };
    let g1 = match parse_map(g1_desc) { Some(v) => v, None => return "Bad g1\n".to_string() };
    let g2 = match parse_map(g2_desc) { Some(v) => v, None => return "Bad g2\n".to_string() };

    out.push_str("  Square:  A →(f1)→ B →(f2)→ D\n");
    out.push_str("           |                   |\n");
    out.push_str("          (g1)               (f2∘f1)\n");
    out.push_str("           ↓                   ↓\n");
    out.push_str("           C ──────(g2)──────→ D\n\n");
    out.push_str(&format!("  f1: x ↦ {}x  on ℤ/{}ℤ\n", f1.0, f1.1));
    out.push_str(&format!("  f2: x ↦ {}x  on ℤ/{}ℤ\n", f2.0, f2.1));
    out.push_str(&format!("  g1: x ↦ {}x  on ℤ/{}ℤ\n", g1.0, g1.1));
    out.push_str(&format!("  g2: x ↦ {}x  on ℤ/{}ℤ\n\n", g2.0, g2.1));
    let n = f1.1;
    let mut all_commute = true;
    out.push_str("  Checking f2∘f1 = g2∘g1 on all elements of ℤ/nℤ:\n");
    for x in 0..n {
        let lhs = (f2.0 * ((f1.0 * x).rem_euclid(f2.1))).rem_euclid(f2.1);
        let rhs = (g2.0 * ((g1.0 * x).rem_euclid(g2.1))).rem_euclid(g2.1);
        let ok = lhs == rhs;
        if !ok { all_commute = false; }
        let sym = if ok { "OK" } else { "FAIL" };
        out.push_str(&format!("    x={}: f2(f1({}))={}, g2(g1({}))={}  [{}]\n", x, x, lhs, x, rhs, sym));
    }
    out.push_str("\n");
    if all_commute {
        out.push_str("The square commutes: f2∘f1 = g2∘g1.\n");
    } else {
        out.push_str("The square does NOT commute.\n");
    }
    out
}

fn cmd_isomorphism(n: i64, m: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Isomorphisms ℤ/{}ℤ → ℤ/{}ℤ ===\n\n", n, m));
    out.push_str("  A homomorphism φ: ℤ/nℤ → ℤ/mℤ is x ↦ ax.\n");
    out.push_str("  It is an isomorphism iff n=m and gcd(a,n)=1.\n\n");
    if n != m {
        out.push_str(&format!("n={} != m={}: no isomorphism exists (groups of different orders).\n", n, m));
        return out;
    }
    let (units, _) = zn_mul_table(n as usize);
    out.push_str(&format!("  Automorphisms of ℤ/{}ℤ (x ↦ ax, gcd(a,{})=1):\n", n, n));
    for &a in &units {
        let ord = mult_order(a as i64, n)
            .map(|o| o.to_string())
            .unwrap_or("?".to_string());
        out.push_str(&format!("    x ↦ {}x  (order of this automorphism: {})\n", a, ord));
    }
    out.push_str("\n");
    out.push_str(&format!("Aut(ℤ/{}ℤ) = (ℤ/{}ℤ)* = {} automorphisms\n", n, n, units.len()));
    out.push_str(&format!("= phi({}) = {}\n", n, euler_totient(n as u64)));
    out
}

fn cmd_opposite() -> String {
    let mut out = String::new();
    out.push_str("=== Opposite Category C^op ===\n\n");
    out.push_str("Given a category C, its opposite C^op reverses all arrows.\n\n");
    out.push_str("  Definition:\n");
    out.push_str("    Ob(C^op)    = Ob(C)\n");
    out.push_str("    Hom^op(A,B) = Hom(B,A)\n");
    out.push_str("    f^op ∘^op g^op = (g ∘ f)^op  [composition reverses]\n\n");
    out.push_str("  Examples:\n");
    out.push_str("    Set^op: morphisms are 'reverse functions' (formally)\n");
    out.push_str("    Vect^op: related to dual spaces\n");
    out.push_str("    Grp^op: isomorphic to Grp (via the inverse map g↦g⁻¹)\n\n");
    out.push_str("  Why it matters:\n");
    out.push_str("  1. Duality: every theorem has a dual in C^op.\n");
    out.push_str("     Product ↔ Coproduct; Limit ↔ Colimit; Mono ↔ Epi.\n");
    out.push_str("  2. Contravariant functors F: C → D = covariant F: C^op → D.\n");
    out.push_str("  3. Yoneda uses Hom(-,A): C^op → Set.\n\n");
    out.push_str("  Concrete example in Ab:\n");
    let h1 = gcd(2, 6);
    let h2 = gcd(6, 2);
    out.push_str(&format!("  |Hom(ℤ/2ℤ, ℤ/6ℤ)| = gcd(2,6) = {}\n", h1));
    out.push_str(&format!("  |Hom^op(ℤ/2ℤ,ℤ/6ℤ)| = |Hom(ℤ/6ℤ,ℤ/2ℤ)| = gcd(6,2) = {}\n", h2));
    out.push_str("Both equal 2 in this case, but C^op is generally different from C.\n");
    out
}

fn cmd_examples() -> String {
    let mut out = String::new();
    out.push_str("=== Examples of Categories ===\n\n");
    out.push_str(&format!("  {:<12} {:<22} {:<24} {}\n", "Category", "Objects", "Morphisms", "Notes"));
    out.push_str("  --------------------------------------------------------------------------\n");
    let rows = [
        ("Set",     "sets",              "functions",                "archetypal"),
        ("Grp",     "groups",            "homomorphisms",            "has free objects"),
        ("Ab",      "abelian groups",    "group homomorphisms",      "abelian category"),
        ("Ring",    "rings",             "ring homomorphisms",       "has tensor product"),
        ("R-Mod",   "R-modules",         "R-linear maps",            "Ext, Tor live here"),
        ("Vect_k",  "k-vector spaces",   "linear maps",              "every obj is free"),
        ("Top",     "top. spaces",       "continuous maps",          "homotopy theory"),
        ("Man",     "manifolds",         "smooth maps",              "differential geom."),
        ("Pos",     "posets",            "order-preserving maps",    "<= gives morphisms"),
        ("Cat",     "small categories",  "functors",                 "categories of cats"),
        ("1",       "{*}",               "{id_*}",                   "terminal category"),
        ("0",       "empty",             "none",                     "initial category"),
    ];
    for (cat, obj, mor, note) in &rows {
        out.push_str(&format!("  {:<12} {:<22} {:<24} {}\n", cat, obj, mor, note));
    }
    out.push_str("\nA category = objects + morphisms + composition, satisfying associativity + unit.\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_str(&mut s, "cat", "grp");
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "category" => {
            if args.is_empty() { return "Usage: category <name>\n".to_string(); }
            state_set_str(state, "cat", args[0]);
            cmd_category(args[0])
        }
        "functor" => {
            if args.is_empty() { return "Usage: functor <name>\n".to_string(); }
            state_set_str(state, "functor", args[0]);
            cmd_functor(args[0])
        }
        "compose_functor" => cmd_compose_functor(),
        "commute" => {
            if args.len() < 4 {
                return "Usage: commute <f1> <f2> <g1> <g2>  where each is a:n\n".to_string();
            }
            state_set_str(state, "f1", args[0]);
            state_set_str(state, "f2", args[1]);
            state_set_str(state, "g1", args[2]);
            state_set_str(state, "g2", args[3]);
            cmd_commute(args[0], args[1], args[2], args[3])
        }
        "isomorphism" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let m = match parse_int(args, 1, "m") { Some(v) => v, None => return "Missing arg m\n".to_string() };
            if n <= 0 || m <= 0 { return "n and m must be positive\n".to_string(); }
            state_set_int(state, "n", n);
            state_set_int(state, "m", m);
            cmd_isomorphism(n, m)
        }
        "opposite" => cmd_opposite(),
        "examples" => cmd_examples(),
        "demo" => {
            let mut out = String::new();
            out.push_str(&cmd_category("grp"));
            out.push_str("\n");
            out.push_str(&cmd_functor("forgetful"));
            out.push_str("\n");
            out.push_str(&cmd_compose_functor());
            out.push_str("\n");
            out.push_str(&cmd_examples());
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
    // Commutative diagram: A -f-> B, A -g-> C, B -h-> D, C -k-> D
    c.text_bold(350.0, 30.0, "Commutative Diagram in a Category", 14.0, "#222", "middle");
    c.node_circle(180.0, 150.0, "A", "#eef", 25.0, 14.0);
    c.node_circle(520.0, 150.0, "B", "#eef", 25.0, 14.0);
    c.node_circle(180.0, 380.0, "C", "#eef", 25.0, 14.0);
    c.node_circle(520.0, 380.0, "D", "#eef", 25.0, 14.0);
    c.arrow(205.0, 150.0, 490.0, 150.0, "#444", 1.5);
    c.text(350.0, 133.0, "f", 13.0, "#222", "middle");
    c.arrow(180.0, 175.0, 180.0, 350.0, "#444", 1.5);
    c.text(163.0, 265.0, "g", 13.0, "#222", "end");
    c.arrow(520.0, 175.0, 520.0, 350.0, "#444", 1.5);
    c.text(535.0, 265.0, "h", 13.0, "#222", "start");
    c.arrow(205.0, 380.0, 490.0, 380.0, "#444", 1.5);
    c.text(350.0, 363.0, "k", 13.0, "#222", "middle");
    c.text(350.0, 265.0, "h \u{2218} f = k \u{2218} g", 12.0, "#444", "middle");
    c.text(350.0, 285.0, "(square commutes)", 11.0, "#666", "middle");
    c.rect(260.0, 250.0, 200.0, 55.0, "none", "#888", 1.0);
    // Functor info
    c.text(50.0, 430.0, "F: C \u{2192} D", 12.0, "#444", "start");
    c.text(50.0, 448.0, "preserves composition", 11.0, "#666", "start");
    c.text(50.0, 466.0, "and identities", 11.0, "#666", "start");
}

fn visualize_dot(g_dot: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g_dot.node("A", &[("label", "A"), ("shape", "circle")]);
    g_dot.node("B", &[("label", "B"), ("shape", "circle")]);
    g_dot.node("C", &[("label", "C"), ("shape", "circle")]);
    g_dot.node("D", &[("label", "D"), ("shape", "circle")]);
    g_dot.edge("A", "B", &[("label", "f")]);
    g_dot.edge("A", "C", &[("label", "g")]);
    g_dot.edge("B", "D", &[("label", "h")]);
    g_dot.edge("C", "D", &[("label", "k")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("A", 0.0, 2.0, "$A$", "draw,circle");
    t.node("B", 3.0, 2.0, "$B$", "draw,circle");
    t.node("C", 0.0, 0.0, "$C$", "draw,circle");
    t.node("D", 3.0, 0.0, "$D$", "draw,circle");
    t.arrow("A", "B", "$f$", "->");
    t.arrow("A", "C", "$g$", "->");
    t.arrow("B", "D", "$h$", "->");
    t.arrow("C", "D", "$k$", "->");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1,  "Commutative Diagram");
    a.text_at(10, 3, "A -----f-----> B");
    a.text_at(10, 4, "|              |");
    a.text_at(10, 5, "g              h");
    a.text_at(10, 6, "|              |");
    a.text_at(10, 7, "C -----k-----> D");
    a.text_at(2, 9,  "Commutes: h*f = k*g");
    a.text_at(2, 11, "Functor F: objects->objects, morphisms->morphisms");
    a.text_at(2, 12, "  F(g*f) = F(g)*F(f)  and  F(id_A) = id_{F(A)}");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch33"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 33", "Categories and Functors", "The language of modern mathematics");
            print_note("Explore categories, functors, and the maps between mathematical structures.");
            println!("{}", show_help());
            repl("cat> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
