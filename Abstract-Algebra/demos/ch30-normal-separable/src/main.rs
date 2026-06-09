use common::*;

fn show_help() -> String {
    help_string(&[
        ("normal_check <p_coeffs>",          "Check if ℚ[x]/(f) is normal over ℚ"),
        ("separable_check <p_coeffs> <p>",   "Check separability: gcd(f,f')=1?"),
        ("derivative <p_coeffs>",            "Formal derivative of polynomial"),
        ("splitting_field <p_coeffs>",       "Describe splitting field (deg 2 or 3)"),
        ("primitive_element <a> <b>",        "Find c with ℚ(√a,√b)=ℚ(√a+√b)"),
        ("frobenius <p>",                    "Frobenius x↦x^p on ℤ/pℤ[x]/(f)"),
        ("inseparable",                      "Show inseparable polynomial in char p"),
        ("demo",                             "Run a showcase of key results"),
        ("help",                             "Show this help"),
        ("quit",                             "Exit"),
    ])
}

fn parse_coeffs_local(args: &[&str]) -> Option<Vec<i64>> {
    let r: Result<Vec<i64>, _> = args.iter().map(|s| s.parse()).collect();
    match r {
        Ok(v) if !v.is_empty() => Some(v),
        _ => { print_err("Expected integer coefficients (constant first)"); None }
    }
}

fn formal_derivative(coeffs: &[i64]) -> Vec<i64> {
    if coeffs.len() <= 1 { return vec![0]; }
    (1..coeffs.len()).map(|i| i as i64 * coeffs[i]).collect()
}

fn poly_gcd(a: &[i64], b: &[i64]) -> Poly {
    let pa = Poly::new(a.to_vec());
    let pb = Poly::new(b.to_vec());
    Poly::gcd_poly(&pa, &pb)
}

fn cmd_derivative(coeffs: &[i64]) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let df_coeffs = formal_derivative(coeffs);
    let df = Poly::new(df_coeffs.clone());
    out.push_str("=== Formal Derivative ===\n\n");
    out.push_str(&format!("  f(x)  = {}\n", f));
    out.push_str(&format!("  f'(x) = {}\n\n", df));
    out.push_str("Formal derivative is defined algebraically: d/dx(xⁿ) = nxⁿ⁻¹\n");
    out.push_str("Works in any characteristic — no limits needed!\n\n");
    out.push_str("  Term by term:\n");
    for (i, &c) in coeffs.iter().enumerate() {
        if c == 0 { continue; }
        let term = if i == 0 {
            format!("  d/dx({}) = 0", c)
        } else if i == 1 {
            format!("  d/dx({}x) = {}", c, c)
        } else {
            format!("  d/dx({}x^{}) = {}x^{}", c, i, c * i as i64, i - 1)
        };
        out.push_str(&format!("{}\n", term));
    }
    out
}

fn cmd_separable_check(coeffs: &[i64], char_p: i64) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let df_coeffs = formal_derivative(coeffs);
    let df = Poly::new(df_coeffs.clone());
    out.push_str("=== Separability Check ===\n\n");
    out.push_str(&format!("  f(x)  = {}\n", f));
    out.push_str(&format!("  f'(x) = {}\n", df));
    out.push_str(&format!("  characteristic = {}\n\n", char_p));

    if char_p == 0 {
        let g = poly_gcd(coeffs, &df_coeffs);
        out.push_str(&format!("  gcd(f, f') = {}\n\n", g));
        if g.degree().is_none() || g.degree() == Some(0) {
            out.push_str("gcd(f, f') = 1: f is separable over ℚ!\n");
            out.push_str("All roots are distinct — no repeated roots.\n");
        } else {
            out.push_str(&format!("gcd(f, f') = {} ≠ 1: f is INSEPARABLE!\n", g));
            out.push_str("f has repeated roots — it is not separable.\n");
        }
    } else {
        let all_p = coeffs.iter().enumerate()
            .filter(|&(_, &c)| c != 0)
            .all(|(i, _)| i == 0 || (i as i64 % char_p == 0));
        out.push_str(&format!("  Checking if f' ≡ 0 mod {}\n", char_p));
        if df.coeffs.is_empty() || df.coeffs.iter().all(|&c| c % char_p == 0) {
            out.push_str(&format!("f' ≡ 0 mod {}: f is INSEPARABLE in characteristic {}!\n", char_p, char_p));
            out.push_str(&format!("f is a polynomial in x^{} (a p-th power).\n", char_p));
            if all_p {
                out.push_str("Every nonzero term has degree divisible by p — f = g(x^p) for some g.\n");
            }
        } else {
            let g = poly_gcd(coeffs, &df_coeffs);
            if g.degree().is_none() || g.degree() == Some(0) {
                out.push_str(&format!("gcd(f, f') = 1 in char {}: f is separable!\n", char_p));
            } else {
                out.push_str(&format!("gcd(f, f') = {}: f is inseparable.\n", g));
            }
        }
    }
    out
}

fn cmd_normal_check(coeffs: &[i64]) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let deg = match f.degree() {
        None | Some(0) => { return "ERROR: Need polynomial of degree >= 1\n".to_string(); }
        Some(d) => d,
    };
    out.push_str("=== Normal Extension Check ===\n\n");
    out.push_str(&format!("  f(x) = {}\n", f));
    out.push_str(&format!("  deg(f) = {}\n\n", deg));
    out.push_str("A finite extension K/ℚ is normal if it is the splitting field of some polynomial,\n");
    out.push_str("equivalently: every irreducible polynomial over ℚ with a root in K splits in K.\n\n");

    let mut rational_roots = vec![];
    let c0 = coeffs[0].unsigned_abs().max(1);
    for d in divisors(c0) {
        for s in [1i64, -1] {
            if f.eval(s * d as i64) == 0 && !rational_roots.contains(&(s * d as i64)) {
                rational_roots.push(s * d as i64);
            }
        }
    }

    match deg {
        1 => {
            out.push_str("Linear extension: trivially normal (splitting field of a degree-1 poly).\n");
        }
        2 => {
            if coeffs.len() >= 3 {
                let a = coeffs[2]; let b = coeffs[1]; let c = coeffs[0];
                let disc = b*b - 4*a*c;
                out.push_str(&format!("  Discriminant = b²−4ac = {} − 4·{}·{} = {}\n", b*b, a, c, disc));
                if disc < 0 {
                    out.push_str("Quadratic with negative discriminant: splits as conjugate pair over ℚ(i√|Δ|).\n");
                    out.push_str("ℚ[x]/(f) is a normal extension: complex conjugation is the non-trivial automorphism.\n");
                } else {
                    out.push_str("Quadratic: ℚ[x]/(f) ≅ ℚ(√Δ) contains both roots ±√Δ of the quadratic.\n");
                    out.push_str("This is normal: both conjugates of any root are present.\n");
                }
            }
        }
        3 => {
            if rational_roots.is_empty() {
                out.push_str("  Cubic with no rational roots.\n");
                out.push_str("  ℚ[x]/(f) contains one root α, but not necessarily all three roots.\n");
                let disc = if coeffs.len() >= 4 {
                    let a=coeffs[3]; let b=coeffs[2]; let c=coeffs[1]; let d=coeffs[0];
                    Some(18*a*b*c*d - 4*b*b*b*d + b*b*c*c - 4*a*c*c*c - 27*a*a*d*d)
                } else { None };
                if let Some(d) = disc {
                    out.push_str(&format!("  Discriminant = {}\n", d));
                    if d < 0 {
                        out.push_str("Δ < 0: one real root, two complex conjugate roots.\n");
                        out.push_str("ℚ[x]/(f) contains α but NOT its complex conjugate.\n");
                        out.push_str("So ℚ[x]/(f) is NOT normal over ℚ.\n");
                    } else {
                        out.push_str("Δ > 0: three distinct real roots. ℚ[x]/(f) has degree 3 over ℚ.\n");
                        out.push_str("Splitting field has degree 3 or 6.\n");
                        out.push_str("If Galois group is A₃ ≅ ℤ/3ℤ, splitting field = ℚ[x]/(f), which IS normal.\n");
                        out.push_str("If Galois group is S₃, splitting field has degree 6 — NOT normal at degree 3.\n");
                    }
                }
            } else {
                out.push_str(&format!("  Rational roots: {:?}\n", rational_roots));
                if rational_roots.len() == deg {
                    out.push_str("All roots rational: f splits completely over ℚ. Extension is normal.\n");
                } else {
                    out.push_str("Some rational roots found; remaining factor may or may not be normal.\n");
                }
            }
        }
        _ => {
            out.push_str(&format!("Degree-{} polynomial: normality analysis is complex.\n", deg));
            out.push_str("f(x) gives a normal extension iff f splits completely in ℚ[x]/(f).\n");
        }
    }
    out
}

fn cmd_splitting_field(coeffs: &[i64]) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let deg = match f.degree() {
        None | Some(0) => { return "ERROR: Need polynomial of degree >= 1\n".to_string(); }
        Some(d) => d,
    };
    out.push_str("=== Splitting Field Description ===\n\n");
    out.push_str(&format!("  f(x) = {}\n\n", f));
    match deg {
        1 => { out.push_str("Splitting field: ℚ (already splits)\n"); }
        2 => {
            if coeffs.len() >= 3 {
                let a=coeffs[2]; let b=coeffs[1]; let c=coeffs[0];
                let disc = b*b - 4*a*c;
                out.push_str(&format!("  Discriminant Δ = {}\n", disc));
                if disc == 0 {
                    out.push_str("Splitting field: ℚ (repeated root, already in ℚ)\n");
                } else {
                    out.push_str(&format!("Splitting field: ℚ(√({}))\n", disc));
                    out.push_str("Degree over ℚ: 2\n");
                    out.push_str("Roots are (−b ± √Δ) / 2a; both are in ℚ(√Δ).\n");
                }
            }
        }
        3 => {
            let disc = if coeffs.len() >= 4 {
                let a=coeffs[3]; let b=coeffs[2]; let c=coeffs[1]; let d=coeffs[0];
                18*a*b*c*d - 4*b*b*b*d + b*b*c*c - 4*a*c*c*c - 27*a*a*d*d
            } else { 0 };
            out.push_str(&format!("  Discriminant Δ = {}\n", disc));
            let mut rat_roots = vec![];
            let c0 = coeffs[0].unsigned_abs().max(1);
            for d in divisors(c0) {
                for s in [1i64, -1] {
                    if f.eval(s*d as i64)==0 && !rat_roots.contains(&(s*d as i64)) {
                        rat_roots.push(s*d as i64);
                    }
                }
            }
            if rat_roots.len() == 3 {
                out.push_str("Splitting field: ℚ (all roots rational)\n");
            } else if rat_roots.len() == 1 {
                out.push_str("Splitting field: ℚ(α) where α is the irrational root\n");
                out.push_str("Degree: 3\n");
                out.push_str("One rational root factors out; the remaining quadratic factor gives ℚ(α).\n");
            } else {
                if disc > 0 {
                    out.push_str("Splitting field: ℚ(α, √Δ) where Δ is the discriminant\n");
                    out.push_str("Degree: 3 (if Gal ≅ A₃) or 6 (if Gal ≅ S₃)\n");
                } else {
                    out.push_str("Splitting field: ℚ(α, β) where β is complex conjugate\n");
                    out.push_str("Degree: 6  (Gal ≅ S₃)\n");
                }
            }
        }
        _ => {
            out.push_str(&format!("Splitting field of degree-{} polynomial requires root-by-root adjunction.\n", deg));
        }
    }
    out
}

fn cmd_primitive_element(a: i64, b: i64) -> String {
    let mut out = String::new();
    out.push_str("=== Primitive Element Theorem ===\n\n");
    out.push_str("Theorem (Primitive Element): Every finite separable extension has a primitive element.\n");
    out.push_str(&format!("We find c such that ℚ(√{}, √{}) = ℚ(c).\n\n", a, b));
    out.push_str(&format!("  Let α = √{}, β = √{}\n", a, b));
    out.push_str(&format!("  Claim: c = α + β = √{} + √{} is a primitive element.\n\n", a, b));
    let ab = a * b;
    out.push_str(&format!("  c = √{} + √{}\n", a, b));
    out.push_str(&format!("  c² = {} + 2√{} + {} = {} + 2√{}\n", a, ab, b, a+b, ab));
    out.push_str(&format!("  c² − {} = 2√{}\n", a+b, ab));
    out.push_str(&format!("  (c² − {})² = 4·{} = {}\n", a+b, ab, 4*ab));
    let coeff2 = -2*(a+b);
    let coeff0 = (a+b)*(a+b) - 4*ab;
    out.push_str(&format!("  Minimal polynomial of c: x⁴ + {}x² + {}\n\n", coeff2, coeff0));
    out.push_str("  Recovery of α and β from c:\n");
    out.push_str(&format!("  √{} = (c² − {} + {}) / (2c)   [from c·2α formula]\n", a, b, a));
    out.push_str(&format!("  √{} = c − √{}\n\n", b, a));
    out.push_str(&format!("ℚ(√{}, √{}) = ℚ(√{} + √{})  [degree 4 extension]\n\n", a, b, a, b));
    if a == b {
        out.push_str("Special case: a = b, so √a = √b, and ℚ(√a,√b) = ℚ(√a), degree 2.\n");
    }
    if a * b > 0 {
        let sq = (ab as f64).sqrt();
        out.push_str(&format!("  √(ab) = √({}) ≈ {:.4}\n", ab, sq));
    }
    out
}

fn cmd_frobenius(p: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) {
        return format!("ERROR: {} is not prime\n", p);
    }
    out.push_str(&format!("=== Frobenius Endomorphism on 𝔽_p = ℤ/{}ℤ ===\n\n", p));
    out.push_str(&format!("Frobenius: φ: 𝔽_p → 𝔽_p, φ(x) = x^{}\n", p));
    out.push_str("In characteristic p: (x+y)^p = x^p + y^p (Freshman's dream).\n\n");
    out.push_str(&format!("  Action of φ on 𝔽_{}: x ↦ x^{}\n", p, p));
    for x in 0..p {
        let xp = mod_pow(x as i64, p, p as i64);
        let fixed = if xp == x as i64 { "  <- fixed" } else { "" };
        out.push_str(&format!("    {}^{} ≡ {} (mod {}){}\n", x, p, xp, p, fixed));
    }
    out.push_str("\n");
    out.push_str(&format!("Fermat's Little Theorem: x^{} ≡ x (mod {}) for all x.\n", p, p));
    out.push_str("So Frobenius fixes ALL elements of 𝔽_p (it's the identity!).\n\n");
    out.push_str("  Extension fields 𝔽_{p^n}: Frobenius generates Gal(𝔽_{p^n}/𝔽_p).\n");
    out.push_str(&format!("  Gal(𝔽_{{{}^n}}/𝔽_{{{}}}) ≅ ℤ/nℤ, generated by x ↦ x^{}\n\n", p, p, p));
    out.push_str("  Freshman's Dream demonstration in 𝔽_p:\n");
    let a = 2i64; let b = 3i64;
    let lhs = mod_pow((a + b) % p as i64, p, p as i64);
    let rhs = (mod_pow(a, p, p as i64) + mod_pow(b, p, p as i64)) % p as i64;
    out.push_str(&format!("    ({}+{})^{} ≡ {} (mod {})\n", a, b, p, lhs, p));
    out.push_str(&format!("    {}^{} + {}^{} ≡ {} (mod {})\n", a, p, b, p, rhs, p));
    if lhs == rhs {
        out.push_str("(a+b)^p = a^p + b^p confirmed!\n");
    }
    out
}

fn cmd_inseparable() -> String {
    let mut out = String::new();
    out.push_str("=== Inseparable Polynomials in Characteristic p ===\n\n");
    out.push_str("In characteristic p > 0, there exist inseparable polynomials.\n");
    out.push_str("Example: f(x) = x^p − t  in  𝔽_p(t)[x]  (t is transcendental).\n\n");
    out.push_str("  Let p = 2, f(x) = x² − t  in  𝔽_2(t)[x]\n\n");
    out.push_str("  Formal derivative: f'(x) = 2x = 0  (in characteristic 2!)\n");
    out.push_str("  So gcd(f, f') = gcd(x²−t, 0) = x²−t ≠ 1\n\n");
    out.push_str("f is inseparable: gcd(f, f') ≠ 1.\n\n");
    out.push_str("  Why? In char p: f(x) = x^p − t = (x − α)^p  in the algebraic closure.\n");
    out.push_str("  The root α has multiplicity p — it is a repeated root!\n");
    out.push_str("  Even though f is irreducible, it has only one distinct root (with mult. p).\n\n");
    out.push_str("This cannot happen in characteristic 0 (where f' ≠ 0 for non-constant f).\n");
    out.push_str("Characteristic p fields can have inseparable irreducible polynomials — 'purely inseparable' extensions.\n\n");
    out.push_str("  Contrast with x^p − 1 in char p:\n");
    out.push_str("  x^p − 1 = (x−1)^p in char p  [by Freshman's dream]\n");
    out.push_str("  The root 1 has multiplicity p — also inseparable.\n\n");
    out.push_str("  Example: In 𝔽_2[x], f(x) = x² + x + 1 is separable:\n");
    let f_sep = Poly::new(vec![1, 1, 1]);
    let df_sep = Poly::new(formal_derivative(&[1, 1, 1]));
    let g = Poly::gcd_poly(&f_sep, &df_sep);
    out.push_str(&format!("    f(x) = {}  in ℤ[x]\n", f_sep));
    out.push_str(&format!("    f'(x) = {}  in ℤ[x]\n", df_sep));
    out.push_str(&format!("    gcd(f,f') = {}\n", g));
    out.push_str("    Mod 2: f'(x) = 1 (the constant 2x mod 2 = 0 vanishes, but f'=2x+1, mod 2 = 1)\n");
    out.push_str("x²+x+1 is separable in char 2 (its derivative mod 2 is 1, a unit).\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "p", 5);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "normal_check" => {
            match parse_coeffs_local(args) {
                Some(c) => {
                    state_set_str(state, "poly", &args.join(" "));
                    cmd_normal_check(&c)
                }
                None => "Error parsing coefficients\n".to_string(),
            }
        }
        "separable_check" => {
            if args.len() < 2 {
                return "Usage: separable_check <p_coeffs...> <char_p>\n".to_string();
            }
            let char_p = match args.last().unwrap().parse::<i64>() {
                Ok(v) => v,
                Err(_) => return "Last arg must be characteristic (0 for char 0)\n".to_string(),
            };
            let coeffs: Result<Vec<i64>, _> = args[..args.len()-1].iter().map(|s| s.parse()).collect();
            match coeffs {
                Ok(c) if !c.is_empty() => {
                    state_set_str(state, "poly", &args[..args.len()-1].join(" "));
                    state_set_int(state, "p", char_p);
                    cmd_separable_check(&c, char_p)
                }
                _ => "Could not parse polynomial coefficients\n".to_string(),
            }
        }
        "derivative" => {
            match parse_coeffs_local(args) {
                Some(c) => {
                    state_set_str(state, "poly", &args.join(" "));
                    cmd_derivative(&c)
                }
                None => "Error parsing coefficients\n".to_string(),
            }
        }
        "splitting_field" => {
            match parse_coeffs_local(args) {
                Some(c) => {
                    state_set_str(state, "poly", &args.join(" "));
                    cmd_splitting_field(&c)
                }
                None => "Error parsing coefficients\n".to_string(),
            }
        }
        "primitive_element" => {
            let a = match parse_int(args, 0, "a") { Some(v) => v, None => return "Missing arg a\n".to_string() };
            let b = match parse_int(args, 1, "b") { Some(v) => v, None => return "Missing arg b\n".to_string() };
            if a <= 0 || b <= 0 { return "a and b must be positive\n".to_string(); }
            state_set_int(state, "a", a);
            state_set_int(state, "b", b);
            cmd_primitive_element(a, b)
        }
        "frobenius" => {
            let p = match parse_uint(args, 0, "p") { Some(v) => v, None => return "Missing arg p\n".to_string() };
            state_set_int(state, "p", p as i64);
            cmd_frobenius(p)
        }
        "inseparable" => cmd_inseparable(),
        "demo" => {
            let mut out = String::new();
            out.push_str(&cmd_derivative(&[-2, 0, 1]));
            out.push_str("\n");
            out.push_str(&cmd_separable_check(&[-2, 0, 1], 0));
            out.push_str("\n");
            out.push_str(&cmd_normal_check(&[-2, 0, 1]));
            out.push_str("\n");
            out.push_str(&cmd_frobenius(5));
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
    // Normal/separable extension diagram
    c.text_bold(350.0, 35.0, "Normal & Separable Extensions", 16.0, "#222", "middle");
    c.node_circle(350.0, 420.0, "F", "#eef", 28.0, 14.0);
    c.node_circle(200.0, 260.0, "K_sep", "#ffe", 35.0, 12.0);
    c.node_circle(500.0, 260.0, "K_norm", "#ffe", 35.0, 12.0);
    c.node_circle(350.0, 100.0, "K_Galois", "#efe", 40.0, 12.0);
    c.arrow(350.0, 391.0, 225.0, 292.0, "#444", 1.5);
    c.text(255.0, 340.0, "separable", 12.0, "#444", "middle");
    c.arrow(350.0, 391.0, 475.0, 292.0, "#444", 1.5);
    c.text(445.0, 340.0, "normal", 12.0, "#444", "middle");
    c.arrow(218.0, 227.0, 322.0, 133.0, "#444", 1.5);
    c.text(235.0, 165.0, "+normal", 12.0, "#444", "middle");
    c.arrow(482.0, 227.0, 378.0, 133.0, "#444", 1.5);
    c.text(465.0, 165.0, "+sep", 12.0, "#444", "middle");
    c.text(350.0, 200.0, "K is Galois = normal + separable", 13.0, "#222", "middle");
}

fn visualize_dot(g_dot: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g_dot.node("F",       &[("label", "F (base field)"), ("shape", "ellipse")]);
    g_dot.node("Ksep",    &[("label", "Separable ext"), ("shape", "box")]);
    g_dot.node("Knorm",   &[("label", "Normal ext"), ("shape", "box")]);
    g_dot.node("KGalois", &[("label", "Galois ext\\n(normal+sep)"), ("shape", "diamond")]);
    g_dot.edge("F", "Ksep",    &[("label", "sep")]);
    g_dot.edge("F", "Knorm",   &[("label", "norm")]);
    g_dot.edge("Ksep",  "KGalois", &[("label", "+norm")]);
    g_dot.edge("Knorm", "KGalois", &[("label", "+sep")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("F",    0.0, 0.0,  "$F$", "draw,circle");
    t.node("Sep",  -2.0, 2.0, "separable", "draw");
    t.node("Norm",  2.0, 2.0, "normal", "draw");
    t.node("Gal",   0.0, 4.0, "Galois = normal + sep", "draw");
    t.arrow("F", "Sep",  "", "->");
    t.arrow("F", "Norm", "", "->");
    t.arrow("Sep",  "Gal", "+normal", "->");
    t.arrow("Norm", "Gal", "+sep",    "->");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1,  "Normal & Separable Extension Diagram");
    a.text_at(30, 3, "Galois = normal + separable");
    a.text_at(18, 5, "/             \\");
    a.text_at(8, 7,  "Separable ext   Normal ext");
    a.text_at(18, 9, "\\             /");
    a.text_at(28, 11, "F (base field)");
    a.text_at(2, 13, "Key: f separable iff gcd(f,f')=1");
    a.text_at(2, 14, "     f normal iff splits in K/F");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch30"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 30", "Normal and Separable Extensions", "When do extensions behave well?");
            print_note("Explore normality (all conjugates present) and separability (distinct roots).");
            println!("{}", show_help());
            repl("normal> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
