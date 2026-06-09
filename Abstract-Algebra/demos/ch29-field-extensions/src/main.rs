use common::*;

fn show_help() -> String {
    help_string(&[
        ("minimal_poly <coeffs>",     "Degree of extension from minimal polynomial"),
        ("degree <d1 d2 ...>",        "Tower law: total degree from list of degrees"),
        ("algebraic <p_coeffs> <v>",  "Check if v is a root of f (algebraic over ℚ)"),
        ("adjoin <p_coeffs>",         "Describe ℚ[x]/(f): basis and arithmetic"),
        ("splitting <p_coeffs>",      "Factor a cubic/quartic over its splitting field"),
        ("tower_example",             "Tower ℚ ⊂ ℚ(√2) ⊂ ℚ(√2,√3)"),
        ("transcendental",            "Contrast π, e (transcendental) vs √2 (algebraic)"),
        ("demo",                      "Run a showcase of key results"),
        ("help",                      "Show this help"),
        ("quit",                      "Exit"),
    ])
}

fn parse_coeffs_local(args: &[&str]) -> Option<Vec<i64>> {
    let coeffs: Result<Vec<i64>, _> = args.iter().map(|s| s.parse::<i64>()).collect();
    match coeffs {
        Ok(v) if !v.is_empty() => Some(v),
        _ => { print_err("Expected integer coefficients (lowest degree first)"); None }
    }
}

fn poly_display(coeffs: &[i64]) -> String {
    Poly::new(coeffs.to_vec()).to_string()
}

fn poly_mul_mod(a: &[i64], b: &[i64], f: &[i64]) -> Vec<i64> {
    let pa = Poly::new(a.to_vec());
    let pb = Poly::new(b.to_vec());
    let pf = Poly::new(f.to_vec());
    let prod = pa.mul(&pb);
    let (_, rem) = prod.div_rem(&pf);
    let mut result = rem.coeffs.clone();
    let n = f.len() - 1;
    while result.len() < n { result.push(0); }
    result.truncate(n);
    result
}

fn poly_add_mod(a: &[i64], b: &[i64], f: &[i64]) -> Vec<i64> {
    let pa = Poly::new(a.to_vec());
    let pb = Poly::new(b.to_vec());
    let sum = pa.add(&pb);
    let pf = Poly::new(f.to_vec());
    let (_, rem) = sum.div_rem(&pf);
    let mut result = rem.coeffs.clone();
    let n = f.len() - 1;
    while result.len() < n { result.push(0); }
    result.truncate(n);
    result
}

fn cmd_minimal_poly(coeffs: &[i64]) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let deg = match f.degree() {
        None => { return "ERROR: Polynomial is zero\n".to_string(); }
        Some(0) => { return "ERROR: Polynomial is a constant — no extension\n".to_string(); }
        Some(d) => d,
    };
    out.push_str("=== Minimal Polynomial and Degree of Extension ===\n\n");
    out.push_str(&format!("  f(x) = {}\n", f));
    out.push_str(&format!("  deg(f) = {}\n\n", deg));
    out.push_str(&format!("If f is the minimal polynomial of α over ℚ,\n"));
    out.push_str(&format!("then [ℚ(α):ℚ] = deg(f) = {}.\n\n", deg));
    out.push_str(&format!("[ℚ(α):ℚ] = {}\n\n", deg));
    out.push_str("  A basis for ℚ(α) over ℚ is:\n");
    for i in 0..deg {
        let basis_elem = if i == 0 { "1".to_string() }
            else if i == 1 { "α".to_string() }
            else { format!("α^{}", i) };
        out.push_str(&format!("    e_{} = {}\n", i, basis_elem));
    }
    out.push_str("\n  Every element of ℚ(α) is uniquely written as:\n");
    let basis_str: Vec<String> = (0..deg).map(|i| {
        if i == 0 { "a₀".to_string() }
        else if i == 1 { "a₁α".to_string() }
        else { format!("a_{}·α^{}", i, i) }
    }).collect();
    out.push_str(&format!("    {}\n\n", basis_str.join(" + ")));
    let mut rational_roots = vec![];
    if let Some(lead) = coeffs.last() {
        if let Some(c0) = coeffs.first() {
            for d in divisors(c0.unsigned_abs().max(1)) {
                for s in [1i64, -1] {
                    for dd in divisors(lead.unsigned_abs().max(1)) {
                        let r_num = s * d as i64;
                        let r_den = dd as i64;
                        if r_den == 1 {
                            let v = f.eval(r_num);
                            if v == 0 { rational_roots.push(r_num); }
                        }
                    }
                }
            }
        }
    }
    rational_roots.dedup();
    if rational_roots.is_empty() {
        out.push_str("No rational roots found — f is likely irreducible over ℚ.\n");
        out.push_str("If f is irreducible, it is indeed the minimal polynomial of its root.\n");
    } else {
        out.push_str(&format!("Rational roots found: {:?}\n", rational_roots));
        out.push_str("f has rational roots, so it factors over ℚ and is NOT irreducible.\n");
        out.push_str("The minimal polynomial of a root would have smaller degree.\n");
    }
    out
}

fn cmd_degree(degrees: &[u64]) -> String {
    let mut out = String::new();
    if degrees.is_empty() {
        return "Provide a list of degrees separated by spaces\n".to_string();
    }
    out.push_str("=== Tower Law for Field Extensions ===\n\n");
    out.push_str("Tower Law: [F_n : F_0] = [F_n : F_{n-1}] · ... · [F_1 : F_0]\n\n");
    out.push_str("  Extension tower:\n");
    let total: u64 = degrees.iter().product();
    for (i, &d) in degrees.iter().enumerate() {
        out.push_str(&format!("    [F_{} : F_{}] = {}\n", i+1, i, d));
    }
    out.push_str("\n");
    out.push_str(&format!("[F_{} : F_0] = {}\n", degrees.len(), total));
    out.push_str(&format!("  (Product: {} = {})\n\n",
        degrees.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(" × "), total));
    out.push_str("The total degree is the product of the step degrees.\n");
    if total.is_power_of_two() {
        out.push_str("Total degree is a power of 2 — elements are constructible by compass and straightedge!\n");
    }
    out
}

fn cmd_algebraic(coeffs: &[i64], value: i64) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    out.push_str("=== Algebraic Element Check ===\n\n");
    out.push_str(&format!("  f(x) = {}\n", f));
    out.push_str(&format!("  value = {}\n\n", value));
    let fv = f.eval(value);
    out.push_str(&format!("f({}) = {}\n\n", value, fv));
    if fv == 0 {
        out.push_str(&format!("{} is a root of f — it is algebraic over ℚ!\n", value));
        out.push_str(&format!("α = {} satisfies f(α) = 0, so α is algebraic of degree ≤ {}.\n",
            value, f.degree().unwrap_or(0)));
    } else {
        out.push_str(&format!("{} is NOT a root of f(x) = {}.\n", value, f));
        out.push_str("To be algebraic over ℚ, the value must satisfy some polynomial with rational coefficients.\n");
    }
    out.push_str("\n  Nearby evaluations:\n");
    for x in (value-3)..=(value+3) {
        let fx = f.eval(x);
        let marker = if fx == 0 { "  <- root!" } else { "" };
        out.push_str(&format!("    f({}) = {}{}\n", x, fx, marker));
    }
    out
}

fn cmd_adjoin(coeffs: &[i64]) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let deg = match f.degree() {
        None | Some(0) => { return "ERROR: Need a polynomial of degree >= 1\n".to_string(); }
        Some(d) => d,
    };
    out.push_str("=== ℚ[x]/(f): Quotient Ring Arithmetic ===\n\n");
    out.push_str(&format!("  f(x) = {}\n", f));
    out.push_str(&format!("  deg(f) = {}\n\n", deg));
    out.push_str(&format!("ℚ[x]/(f) has ℚ-basis: {{1, α, α², ..., α^{}}} where f(α)=0.\n\n", deg-1));
    let mut alpha_deg = vec![0i64; deg + 1];
    alpha_deg[deg] = 1;
    let (_, rem_alpha) = Poly::new(alpha_deg.clone()).div_rem(&f);
    out.push_str(&format!("  Key relation: α^{} = {}\n\n", deg, rem_alpha));
    if deg >= 2 {
        let a = vec![1i64, 1];
        let b = vec![1i64, -1];
        let prod = poly_mul_mod(&a, &b, coeffs);
        out.push_str("  Example multiplication in ℚ[x]/(f):\n");
        out.push_str(&format!("    (1+α)(1−α) = {}\n\n", poly_display(&prod)));
        let a2: Vec<i64> = (0..deg).map(|i| if i == deg-1 { 1 } else { 0 }).collect();
        let b2: Vec<i64> = vec![0, 1];
        let prod2 = poly_mul_mod(&a2, &b2, coeffs);
        out.push_str(&format!("    α^{} · α = α^{} ≡ {}\n\n", deg-1, deg, poly_display(&prod2)));
    }
    if deg >= 2 {
        let a = vec![1i64, 2, 1];
        let b = vec![3i64, -1];
        let mut av = a.clone(); av.truncate(deg);
        let mut bv = b.clone(); bv.resize(deg, 0);
        let sum = poly_add_mod(&av, &bv, coeffs);
        out.push_str("  Example addition:\n");
        out.push_str(&format!("    ({}) + ({}) = {}\n", poly_display(&av), poly_display(&bv), poly_display(&sum)));
    }
    out.push_str("\nIf f is irreducible, ℚ[x]/(f) is a field extension of ℚ of degree deg(f).\n");
    out
}

fn cmd_splitting(coeffs: &[i64]) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let deg = match f.degree() {
        None => { return "ERROR: Zero polynomial\n".to_string(); }
        Some(d) => d,
    };
    out.push_str("=== Splitting Field of a Polynomial ===\n\n");
    out.push_str(&format!("  f(x) = {}\n\n", f));
    let mut rational_roots = vec![];
    let c0 = coeffs[0].unsigned_abs().max(1);
    let clead = coeffs[deg].unsigned_abs().max(1);
    for d in divisors(c0) {
        for s in [1i64, -1] {
            for dd in divisors(clead) {
                if dd == 1 {
                    let r = s * d as i64;
                    if f.eval(r) == 0 && !rational_roots.contains(&r) {
                        rational_roots.push(r);
                    }
                }
            }
        }
    }
    if rational_roots.is_empty() {
        out.push_str("  No rational roots found.\n");
        match deg {
            2 => {
                if coeffs.len() >= 3 {
                    let a = coeffs[2]; let b = coeffs[1]; let c = coeffs[0];
                    let disc = b*b - 4*a*c;
                    out.push_str(&format!("  Discriminant: b²−4ac = {}²−4·{}·{} = {}\n", b, a, c, disc));
                    if disc < 0 {
                        out.push_str("Splitting field: ℚ(i√|Δ|) — imaginary quadratic\n");
                    } else {
                        let sq = (disc as f64).sqrt();
                        out.push_str(&format!("Splitting field: ℚ(√{})\n", disc));
                        out.push_str(&format!("Roots involve √{} ≈ {:.4}\n", disc, sq));
                    }
                }
            }
            3 => {
                out.push_str("Cubic with no rational roots: Galois group is S₃ or A₃.\n");
                out.push_str("Splitting field has degree 3 or 6 over ℚ.\n");
                if coeffs.len() >= 4 {
                    let a = coeffs[3]; let b = coeffs[2]; let c = coeffs[1]; let d = coeffs[0];
                    let disc = 18*a*b*c*d - 4*b*b*b*d + b*b*c*c - 4*a*c*c*c - 27*a*a*d*d;
                    out.push_str(&format!("Discriminant Δ = {}\n", disc));
                    if disc > 0 {
                        out.push_str("Δ > 0: 3 real roots, Galois group is A₃ ≅ ℤ/3ℤ, splitting field degree 3.\n");
                    } else {
                        out.push_str("Δ < 0: 1 real root, 2 complex roots, Galois group is S₃, splitting field degree 6.\n");
                    }
                }
            }
            _ => {
                out.push_str(&format!("For degree-{} polynomial, splitting field analysis is complex.\n", deg));
                out.push_str("Splitting field is ℚ(α₁,...,αₙ) where αᵢ are all roots.\n");
            }
        }
    } else {
        out.push_str(&format!("  Rational roots: {:?}\n", rational_roots));
        for &r in &rational_roots {
            out.push_str(&format!("    f({}) = {}\n", r, f.eval(r)));
        }
        out.push_str("These roots are already in ℚ.\n");
        if rational_roots.len() == deg {
            out.push_str("f splits completely over ℚ! Splitting field = ℚ.\n");
        } else {
            out.push_str("f splits partially over ℚ; remaining factor needs algebraic extension.\n");
        }
    }
    out
}

fn cmd_tower_example() -> String {
    let mut out = String::new();
    out.push_str("=== Tower Example: ℚ ⊂ ℚ(√2) ⊂ ℚ(√2, √3) ===\n\n");
    out.push_str("We build a tower of field extensions step by step.\n\n");
    out.push_str("  Step 1: ℚ ⊂ ℚ(√2)\n");
    out.push_str("    √2 is a root of f(x) = x² − 2\n");
    out.push_str("    f is irreducible over ℚ (no rational roots)\n");
    out.push_str("    [ℚ(√2) : ℚ] = 2\n");
    out.push_str("    Basis: {1, √2}\n");
    out.push_str("    Elements: a + b√2 for a,b ∈ ℚ\n\n");
    out.push_str("  Step 2: ℚ(√2) ⊂ ℚ(√2, √3)\n");
    out.push_str("    √3 is a root of g(x) = x² − 3\n");
    out.push_str("    g is irreducible over ℚ(√2) (√3 ∉ ℚ(√2))\n");
    out.push_str("    [ℚ(√2,√3) : ℚ(√2)] = 2\n");
    out.push_str("    Basis over ℚ(√2): {1, √3}\n\n");
    out.push_str("  Tower Law:\n");
    out.push_str("    [ℚ(√2,√3) : ℚ] = 2 × 2 = 4\n\n");
    out.push_str("  Basis for ℚ(√2,√3) over ℚ: {1, √2, √3, √6}\n");
    out.push_str("  Elements: a + b√2 + c√3 + d√6 for a,b,c,d ∈ ℚ\n\n");
    out.push_str("  Galois group: Gal(ℚ(√2,√3)/ℚ) ≅ ℤ/2ℤ × ℤ/2ℤ (the Klein four-group)\n");
    out.push_str("  Four automorphisms:\n");
    out.push_str("    id:    √2 ↦ √2,  √3 ↦ √3\n");
    out.push_str("    σ:     √2 ↦ −√2, √3 ↦ √3\n");
    out.push_str("    τ:     √2 ↦ √2,  √3 ↦ −√3\n");
    out.push_str("    στ:    √2 ↦ −√2, √3 ↦ −√3\n\n");
    out.push_str("Each subgroup of ℤ/2×ℤ/2 corresponds to a fixed field.\n");
    out.push_str("This is the Galois correspondence in action!\n");
    out
}

fn cmd_transcendental() -> String {
    let mut out = String::new();
    out.push_str("=== Transcendental vs Algebraic Numbers ===\n\n");
    out.push_str("A number α is algebraic over ℚ if it satisfies a non-zero polynomial f ∈ ℚ[x].\n");
    out.push_str("Otherwise α is transcendental.\n\n");
    out.push_str("  ALGEBRAIC NUMBERS:\n\n");
    out.push_str("  √2: root of x² − 2 = 0\n");
    out.push_str("    Minimal polynomial: x² − 2\n");
    out.push_str("    [ℚ(√2) : ℚ] = 2\n\n");
    out.push_str("  ∛5: root of x³ − 5 = 0\n");
    out.push_str("    Minimal polynomial: x³ − 5 (irreducible by Eisenstein at p=5)\n");
    out.push_str("    [ℚ(∛5) : ℚ] = 3\n\n");
    out.push_str("  ζ₅ = e^(2πi/5): root of x⁵−1 = 0, or better Φ₅(x) = x⁴+x³+x²+x+1\n");
    out.push_str("    [ℚ(ζ₅) : ℚ] = 4\n\n");
    out.push_str("---\n");
    out.push_str("  TRANSCENDENTAL NUMBERS:\n\n");
    out.push_str("  π ≈ 3.14159...\n");
    out.push_str("    Lindemann-Weierstrass Theorem (1882): e^α is transcendental for any\n");
    out.push_str("    nonzero algebraic α. Taking α = iπ gives e^(iπ) = −1 (algebraic),\n");
    out.push_str("    so iπ, hence π, must be transcendental.\n");
    out.push_str("    Consequence: squaring the circle is impossible!\n\n");
    out.push_str("  e ≈ 2.71828...\n");
    out.push_str("    Hermite (1873): e is transcendental.\n");
    out.push_str("    Also follows from Lindemann-Weierstrass (take α=1).\n\n");
    out.push_str("Algebraic numbers are countable; transcendental numbers are uncountable.\n");
    out.push_str("In fact, 'most' real numbers are transcendental!\n\n");
    let f2 = Poly::new(vec![-2, 0, 1]);
    out.push_str(&format!("  Verification: f(x) = {}\n", f2));
    out.push_str(&format!("  f(1) = {}   f(2) = {}\n", f2.eval(1), f2.eval(2)));
    out.push_str("  Since f changes sign between 1 and 2, √2 ∈ (1,2) is a root.\n");
    out.push_str("√2 is algebraic (we exhibited its minimal polynomial).\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_str(&mut s, "poly", "-2 0 1");
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "minimal_poly" => {
            match parse_coeffs_local(args) {
                Some(coeffs) => {
                    state_set_str(state, "poly", &args.join(" "));
                    cmd_minimal_poly(&coeffs)
                }
                None => "Error parsing coefficients\n".to_string(),
            }
        }
        "degree" => {
            let degrees: Result<Vec<u64>, _> = args.iter().map(|s| s.parse::<u64>()).collect();
            match degrees {
                Ok(v) if !v.is_empty() => {
                    state_set_str(state, "degrees", &args.join(" "));
                    cmd_degree(&v)
                }
                _ => "Provide positive integer degrees, e.g.: degree 2 3 2\n".to_string(),
            }
        }
        "algebraic" => {
            if args.len() < 2 {
                return "Usage: algebraic <p_coeffs...> <value>  (last arg is the value)\n".to_string();
            }
            let value = match args.last().unwrap().parse::<i64>() {
                Ok(v) => v,
                Err(_) => return "Last argument must be the integer value to test\n".to_string(),
            };
            let coeffs: Result<Vec<i64>, _> = args[..args.len()-1].iter().map(|s| s.parse::<i64>()).collect();
            match coeffs {
                Ok(c) if !c.is_empty() => {
                    state_set_str(state, "poly", &args[..args.len()-1].join(" "));
                    state_set_int(state, "value", value);
                    cmd_algebraic(&c, value)
                }
                _ => "Could not parse polynomial coefficients\n".to_string(),
            }
        }
        "adjoin" => {
            match parse_coeffs_local(args) {
                Some(coeffs) => {
                    state_set_str(state, "poly", &args.join(" "));
                    cmd_adjoin(&coeffs)
                }
                None => "Error parsing coefficients\n".to_string(),
            }
        }
        "splitting" => {
            match parse_coeffs_local(args) {
                Some(coeffs) => {
                    state_set_str(state, "poly", &args.join(" "));
                    cmd_splitting(&coeffs)
                }
                None => "Error parsing coefficients\n".to_string(),
            }
        }
        "tower_example" => cmd_tower_example(),
        "transcendental" => cmd_transcendental(),
        "demo" => {
            let mut out = String::new();
            out.push_str(&cmd_tower_example());
            out.push_str("\n");
            out.push_str(&cmd_minimal_poly(&[-2, 0, 1]));
            out.push_str("\n");
            out.push_str(&cmd_degree(&[2, 2]));
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
    // Field extension tower diagram: F ⊆ K ⊆ L
    c.text_bold(350.0, 35.0, "Field Extension Tower", 16.0, "#222", "middle");
    // Q at bottom, Q(√2) in middle, Q(√2,√3) at top
    c.node_circle(350.0, 420.0, "Q", "#eef", 28.0, 14.0);
    c.node_circle(350.0, 260.0, "Q(sqrt2)", "#eef", 35.0, 12.0);
    c.node_circle(350.0, 100.0, "Q(sqrt2,sqrt3)", "#eef", 42.0, 11.0);
    // Arrows bottom-to-top
    c.arrow(350.0, 391.0, 350.0, 296.0, "#444", 1.5);
    c.text(370.0, 340.0, "[K:F]=2", 13.0, "#444", "start");
    c.arrow(350.0, 224.0, 350.0, 143.0, "#444", 1.5);
    c.text(370.0, 180.0, "[L:K]=2", 13.0, "#444", "start");
    // Total degree
    c.text(150.0, 260.0, "[L:F] = 2x2 = 4", 13.0, "#222", "middle");
    c.rect(80.0, 248.0, 145.0, 26.0, "none", "#888", 1.0);
    // Basis info
    c.text(500.0, 100.0, "Basis: {1,sqrt2,sqrt3,sqrt6}", 12.0, "#555", "start");
    c.text(500.0, 260.0, "Basis: {1,sqrt2}", 12.0, "#555", "start");
    c.text(500.0, 420.0, "Basis: {1}", 12.0, "#555", "start");
}

fn visualize_dot(g_dot: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g_dot.node("Q",    &[("label", "ℚ"), ("shape", "ellipse")]);
    g_dot.node("Qr2",  &[("label", "ℚ(√2)"), ("shape", "ellipse")]);
    g_dot.node("Qr2r3",&[("label", "ℚ(√2,√3)"), ("shape", "ellipse")]);
    g_dot.edge("Q", "Qr2",   &[("label", "[K:F]=2")]);
    g_dot.edge("Qr2", "Qr2r3",&[("label", "[L:K]=2")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("Q",   0.0, 0.0,  "$\\mathbb{Q}$", "draw,circle");
    t.node("K",   0.0, 2.0,  "$\\mathbb{Q}(\\sqrt{2})$", "draw");
    t.node("L",   0.0, 4.0,  "$\\mathbb{Q}(\\sqrt{2},\\sqrt{3})$", "draw");
    t.arrow("Q", "K", "$[K:F]=2$", "->");
    t.arrow("K", "L", "$[L:K]=2$", "->");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1, "Field Extension Tower");
    a.text_at(10, 3,  "Q(sqrt2, sqrt3)  deg 4 over Q");
    a.text_at(18, 5, "|  [L:K]=2");
    a.text_at(10, 7,  "Q(sqrt2)          deg 2 over Q");
    a.text_at(18, 9, "|  [K:F]=2");
    a.text_at(10, 11, "Q                 base field");
    a.text_at(2, 13, "Tower law: [L:F] = [L:K]*[K:F] = 2*2 = 4");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch29"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 29", "Field Extensions", "Growing fields by adjoining roots");
            print_note("Explore algebraic extensions, minimal polynomials, and field towers.");
            println!("{}", show_help());
            repl("fields> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
