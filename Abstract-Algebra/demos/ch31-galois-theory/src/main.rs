use common::*;

fn show_help() -> String {
    help_string(&[
        ("galois_group <p_coeffs>",       "Compute Galois group of polynomial (deg <= 4)"),
        ("discriminant <p_coeffs>",       "Discriminant of quadratic or cubic"),
        ("correspondence <n>",            "Galois correspondence for ℚ(ζ_n)/ℚ"),
        ("fixed_field <p_coeffs> <auto>", "Show fixed field for an automorphism"),
        ("sqrt2_sqrt3",                   "Full example: Gal(ℚ(√2,√3)/ℚ)"),
        ("cyclotomic_galois <n>",         "Gal(ℚ(ζ_n)/ℚ) ≅ (ℤ/nℤ)*"),
        ("demo",                          "Run a showcase of key results"),
        ("help",                          "Show this help"),
        ("quit",                          "Exit"),
    ])
}

fn parse_coeffs_local(args: &[&str]) -> Option<Vec<i64>> {
    let r: Result<Vec<i64>, _> = args.iter().map(|s| s.parse()).collect();
    match r {
        Ok(v) if !v.is_empty() => Some(v),
        _ => { print_err("Expected integer coefficients (constant term first)"); None }
    }
}

fn discriminant_quad(coeffs: &[i64]) -> i64 {
    let a = coeffs[2]; let b = coeffs[1]; let c = coeffs[0];
    b*b - 4*a*c
}

fn discriminant_cubic(coeffs: &[i64]) -> i64 {
    let a=coeffs[3]; let b=coeffs[2]; let c=coeffs[1]; let d=coeffs[0];
    18*a*b*c*d - 4*b*b*b*d + b*b*c*c - 4*a*c*c*c - 27*a*a*d*d
}

fn cmd_discriminant(coeffs: &[i64]) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let deg = match f.degree() { None => 0, Some(d) => d };
    out.push_str("=== Polynomial Discriminant ===\n\n");
    out.push_str(&format!("  f(x) = {}\n\n", f));
    match deg {
        2 => {
            let disc = discriminant_quad(coeffs);
            let a=coeffs[2]; let b=coeffs[1]; let c=coeffs[0];
            out.push_str("  For ax²+bx+c: Δ = b²−4ac\n");
            out.push_str(&format!("  Δ = {}²−4·{}·{} = {} − {} = {}\n", b, a, c, b*b, 4*a*c, disc));
            out.push_str(&format!("Discriminant Δ = {}\n\n", disc));
            if disc > 0 { out.push_str("Δ > 0: two distinct real roots.\n"); }
            else if disc == 0 { out.push_str("Δ = 0: double root (repeated).\n"); }
            else { out.push_str("Δ < 0: two complex conjugate roots.\n"); }
        }
        3 => {
            let disc = discriminant_cubic(coeffs);
            let a=coeffs[3]; let b=coeffs[2]; let c=coeffs[1]; let d=coeffs[0];
            out.push_str("  For ax³+bx²+cx+d: Δ = 18abcd − 4b³d + b²c² − 4ac³ − 27a²d²\n");
            out.push_str(&format!("  a={}, b={}, c={}, d={}\n", a, b, c, d));
            out.push_str(&format!("Discriminant Δ = {}\n\n", disc));
            if disc > 0 {
                out.push_str("Δ > 0: three distinct real roots.\n");
                out.push_str("If Δ is a perfect square in ℚ: Gal ≅ A₃ ≅ ℤ/3ℤ.\n");
                out.push_str("If Δ is not a perfect square: Gal ≅ S₃.\n");
                let sq = (disc as f64).sqrt();
                let sq_int = sq.round() as i64;
                if sq_int * sq_int == disc {
                    out.push_str(&format!("√Δ = {} ∈ ℚ: Galois group is A₃!\n", sq_int));
                } else {
                    out.push_str(&format!("√Δ = √{} ∉ ℚ: Galois group is S₃.\n", disc));
                }
            } else if disc == 0 {
                out.push_str("Δ = 0: repeated root.\n");
            } else {
                out.push_str("Δ < 0: one real root, two complex conjugates.\n");
                out.push_str("Galois group is S₃.\n");
            }
        }
        _ => {
            out.push_str(&format!("Discriminant formula implemented for degrees 2 and 3 (got deg={}).\n", deg));
        }
    }
    out
}

fn cmd_galois_group(coeffs: &[i64]) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let deg = match f.degree() { None | Some(0) => { return "ERROR: Need deg >= 1\n".to_string(); } Some(d) => d };
    out.push_str("=== Galois Group Computation ===\n\n");
    out.push_str(&format!("  f(x) = {}\n\n", f));

    let c0 = coeffs[0].unsigned_abs().max(1);
    let mut rat_roots = vec![];
    for d in divisors(c0) {
        for s in [1i64, -1] {
            if f.eval(s*d as i64)==0 && !rat_roots.contains(&(s*d as i64)) {
                rat_roots.push(s*d as i64);
            }
        }
    }

    match deg {
        1 => {
            out.push_str("Galois group: trivial {e}\n");
            out.push_str("Linear polynomial: root is in ℚ, no non-trivial automorphisms.\n");
        }
        2 => {
            let disc = if coeffs.len() >= 3 { discriminant_quad(coeffs) } else { 0 };
            out.push_str(&format!("Discriminant: {}\n", disc));
            if disc == 0 {
                out.push_str("Galois group: {e} (trivial)\n");
                out.push_str("Repeated root: splitting field = ℚ.\n");
            } else {
                let sq = (disc.abs() as f64).sqrt();
                let sq_int = sq.round() as i64;
                if sq_int * sq_int == disc {
                    out.push_str("Galois group: {e} (trivial)\n");
                    out.push_str(&format!("√Δ = {} ∈ ℚ: splitting field = ℚ.\n", sq_int));
                } else {
                    out.push_str("Galois group: ℤ/2ℤ\n");
                    out.push_str("Only automorphism: conjugate the two roots.\n");
                    out.push_str("  σ: √Δ ↦ −√Δ\n");
                }
            }
        }
        3 => {
            if !rat_roots.is_empty() {
                out.push_str(&format!("  Rational roots: {:?}\n", rat_roots));
                if rat_roots.len() == 3 {
                    out.push_str("Galois group: {e} (trivial)\n");
                    out.push_str("All roots are rational: f splits over ℚ.\n");
                } else {
                    out.push_str("f has a rational root but doesn't split; residual quadratic.\n");
                    out.push_str("Galois group: ℤ/2ℤ\n");
                }
            } else {
                let disc = if coeffs.len() >= 4 { discriminant_cubic(coeffs) } else { 0 };
                out.push_str(&format!("Discriminant Δ: {}\n", disc));
                let sq = (disc.abs() as f64).sqrt();
                let sq_int = sq.round() as i64;
                let sq_in_q = sq_int * sq_int == disc && disc >= 0;
                if disc > 0 && sq_in_q {
                    out.push_str("Galois group: A₃ ≅ ℤ/3ℤ  (order 3)\n");
                    out.push_str("Δ > 0 and √Δ ∈ ℚ: three real roots, cyclic Galois group.\n");
                } else {
                    out.push_str("Galois group: S₃  (order 6)\n");
                    if disc < 0 {
                        out.push_str("Δ < 0: one real + two complex roots → Gal ≅ S₃.\n");
                    } else {
                        out.push_str("Δ > 0 but √Δ ∉ ℚ: three real roots but Gal ≅ S₃.\n");
                    }
                }
            }
        }
        4 => {
            out.push_str("Degree 4: Galois group is a subgroup of S₄.\n");
            if !rat_roots.is_empty() {
                out.push_str(&format!("  Rational roots: {:?}\n", rat_roots));
            }
            out.push_str("  Possible Galois groups: S₄(ord 24), A₄(ord 12), D₄(ord 8), V₄(ord 4), ℤ/4ℤ(ord 4).\n");
            out.push_str("Determined by: discriminant, resolvent cubic, and factorization patterns.\n");
        }
        _ => {
            out.push_str(&format!("Degree {} not supported for automatic Galois group computation.\n", deg));
        }
    }
    out
}

fn cmd_correspondence(n: u64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Galois Correspondence for ℚ(ζ_{})/ℚ ===\n\n", n));
    if !is_prime(n) {
        out.push_str(&format!("{} is not prime; using n={} but correspondence is for cyclotomic fields.\n\n", n, n));
    }
    let phi = euler_totient(n);
    out.push_str(&format!("  ζ_{} = e^(2πi/{}), a primitive {}-th root of unity\n", n, n, n));
    out.push_str(&format!("[ℚ(ζ_{n}) : ℚ] = {} = φ({})\n\n", phi, n));
    out.push_str(&format!("  Gal(ℚ(ζ_{})/ℚ) ≅ (ℤ/{}ℤ)*\n", n, n));
    let (units, _) = zn_mul_table(n as usize);
    out.push_str(&format!("  (ℤ/{}ℤ)* = {{{}}}\n", n,
        units.iter().map(|u| u.to_string()).collect::<Vec<_>>().join(", ")));
    out.push_str(&format!("  Order = φ({}) = {}\n\n", n, phi));
    out.push_str("  Each k ∈ (ℤ/nℤ)* corresponds to σ_k: ζ_n ↦ ζ_n^k\n");
    for &k in &units {
        out.push_str(&format!("    σ_{k}: ζ_{n} ↦ ζ_{n}^{k}\n"));
    }
    out.push_str("\n");
    let subgroups = subgroups_zn(phi as usize);
    out.push_str("  Subgroups (by divisors of φ(n)):\n");
    for sg in &subgroups {
        let order = sg.len();
        let index = phi as usize / order;
        out.push_str(&format!("    H of order {} (index {}) → fixed field of degree {} over ℚ\n",
            order, index, index));
    }
    out.push_str("\nThe Galois correspondence: subgroups H ↔ intermediate fields ℚ ⊆ K ⊆ ℚ(ζ_n).\n");
    out.push_str("Larger subgroup ↔ smaller (more fixed) field.\n");
    out
}

fn cmd_fixed_field(coeffs: &[i64], auto_desc: &str) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    out.push_str("=== Fixed Field of an Automorphism ===\n\n");
    out.push_str(&format!("  Base polynomial: f(x) = {}\n", f));
    out.push_str(&format!("  Automorphism: {}\n\n", auto_desc));
    out.push_str("The fixed field consists of all elements left unchanged by the automorphism.\n\n");
    out.push_str("  For ℚ(√a, √b) with automorphisms:\n");
    out.push_str("    id:    fixes everything → fixed field = ℚ(√a, √b)\n");
    out.push_str("    σ(√a→-√a): fixes ℚ(√b) ⊆ ℚ(√a,√b)\n");
    out.push_str("    τ(√b→-√b): fixes ℚ(√a) ⊆ ℚ(√a,√b)\n");
    out.push_str("    στ:    fixes ℚ(√(ab)) ⊆ ℚ(√a,√b)\n\n");
    match auto_desc {
        "id" => {
            out.push_str("Fixed field of id: ℚ(√a, √b)  (the whole extension)\n");
        }
        "sigma" | "s" => {
            out.push_str("Fixed field of σ: ℚ(√b)\n");
            out.push_str("σ negates √a, so any element with non-zero √a coefficient is moved.\n");
        }
        "tau" | "t" => {
            out.push_str("Fixed field of τ: ℚ(√a)\n");
        }
        "sigmatau" | "st" => {
            out.push_str("  The automorphism σ∘τ maps √a↦-√a, √b↦-√b.\n");
            out.push_str("  An element a₀+a₁√2+a₂√3+a₃√6 is fixed iff a₁=a₂=0.\n");
            out.push_str("Fixed field of στ: ℚ(√(ab)) = ℚ(√6) for a=2,b=3\n");
        }
        _ => {
            out.push_str(&format!("Automorphism '{}' not recognized. Use: id, sigma, tau, sigmatau\n", auto_desc));
        }
    }
    out
}

fn cmd_sqrt2_sqrt3() -> String {
    let mut out = String::new();
    out.push_str("=== Full Example: Gal(ℚ(√2, √3)/ℚ) ===\n\n");
    out.push_str("We compute the full Galois group and correspondence for ℚ(√2,√3)/ℚ.\n\n");
    out.push_str("[ℚ(√2,√3) : ℚ] = 4\n");
    out.push_str("Galois group: ℤ/2ℤ × ℤ/2ℤ  (Klein four-group V₄)\n\n");
    out.push_str("=== Four Automorphisms ===\n\n");
    let autos = [
        ("e  (identity)", "√2 ↦ √2,  √3 ↦ √3",  "√6 ↦ √6"),
        ("σ",             "√2 ↦ −√2, √3 ↦ √3",   "√6 ↦ −√6"),
        ("τ",             "√2 ↦ √2,  √3 ↦ −√3",  "√6 ↦ −√6"),
        ("στ",            "√2 ↦ −√2, √3 ↦ −√3",  "√6 ↦ √6"),
    ];
    for (name, action, extra) in &autos {
        out.push_str(&format!("  {:<6} : {}  ({})\n", name, action, extra));
    }
    out.push_str("\n=== Group Structure ===\n\n");
    out.push_str("  σ² = e   τ² = e\n");
    out.push_str("  στ = τσ    (στ)² = e\n");
    out.push_str("  All non-identity elements have order 2 → Klein four-group V₄\n\n");
    out.push_str("=== Galois Correspondence: Subgroups <-> Fixed Fields ===\n\n");
    let rows = [
        ("Gal (full group)",   "{e,σ,τ,στ}", "ℚ",         "4"),
        ("<σ> = {e,σ}",        "{e,σ}",      "ℚ(√3)",     "2"),
        ("<τ> = {e,τ}",        "{e,τ}",      "ℚ(√2)",     "2"),
        ("<στ> = {e,στ}",      "{e,στ}",     "ℚ(√6)",     "2"),
        ("{e} (trivial)",       "{e}",         "ℚ(√2,√3)", "1"),
    ];
    out.push_str(&format!("  {:<22} {:<14} {:<16} {}\n", "Subgroup", "Elements", "Fixed Field", "Index"));
    out.push_str("  -----------------------------------------------------------\n");
    for (sg, elems, fixed, idx) in &rows {
        out.push_str(&format!("  {:<22} {:<14} {:<16} {}\n", sg, elems, fixed, idx));
    }
    out.push_str("\nLarger subgroup ↔ smaller fixed field (anti-order-preserving correspondence).\n");
    out.push_str("The lattice of subgroups is isomorphic to the lattice of intermediate fields (reversed).\n\n");
    out.push_str("=== Verification ===\n\n");
    out.push_str("  Fixed field of <σ>: σ(a+b√2+c√3+d√6) = a−b√2+c√3−d√6\n");
    out.push_str("  Fixed iff b=0 and d=0, i.e. element is a+c√3 ∈ ℚ(√3). ✓\n");
    out.push_str("  Fixed field of <τ>: similarly gives ℚ(√2). ✓\n");
    out.push_str("  Fixed field of <στ>: element a+b√2+c√3+d√6 fixed iff b=c=0 → ℚ(√6). ✓\n");
    out
}

fn cmd_cyclotomic_galois(n: u64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Cyclotomic Field ℚ(ζ_{n}): Gal ≅ (ℤ/{}ℤ)* ===\n\n", n));
    let phi = euler_totient(n);
    out.push_str(&format!("  ζ_{n} = primitive {}-th root of unity = e^(2πi/{})\n", n, n));
    out.push_str(&format!("  Minimal polynomial: Φ_{n}(x) (the {}-th cyclotomic polynomial)\n", n));
    out.push_str(&format!("[ℚ(ζ_{n}) : ℚ] = {} = φ({})\n\n", phi, n));
    let (units, _) = zn_mul_table(n as usize);
    out.push_str(&format!("  (ℤ/{}ℤ)* = {{{}}}  (order {})\n\n",
        n, units.iter().map(|u| u.to_string()).collect::<Vec<_>>().join(", "), phi));
    out.push_str("  Generator σ_k acts by: σ_k(ζ_n) = ζ_n^k\n\n");
    out.push_str("  Orders of elements:\n");
    for &k in &units {
        let ord = mult_order(k as i64, n as i64);
        let gen_mark = if ord == Some(phi) { "  <- generator!" } else { "" };
        out.push_str(&format!("    σ_{}: order = {}{}\n", k,
            ord.map(|o| o.to_string()).unwrap_or("?".to_string()), gen_mark));
    }
    out.push_str("\nThe Galois group is generated by any σ_k with k of maximal order.\n");
    if is_prime(n) {
        out.push_str(&format!("Since {} is prime, (ℤ/{}ℤ)* is cyclic of order {}.\n", n, n, n-1));
    }
    out.push_str("\n");
    out.push_str(&format!("  Cyclotomic polynomial Φ_{}(x):\n", n));
    match n {
        2 => out.push_str("  Φ_2(x) = x + 1\n"),
        3 => out.push_str("  Φ_3(x) = x² + x + 1\n"),
        4 => out.push_str("  Φ_4(x) = x² + 1\n"),
        5 => out.push_str("  Φ_5(x) = x⁴ + x³ + x² + x + 1\n"),
        6 => out.push_str("  Φ_6(x) = x² − x + 1\n"),
        7 => out.push_str("  Φ_7(x) = x⁶ + x⁵ + x⁴ + x³ + x² + x + 1\n"),
        8 => out.push_str("  Φ_8(x) = x⁴ + 1\n"),
        _ => out.push_str(&format!("  Φ_{n}(x): product of (x − ζ_n^k) for gcd(k,n)=1\n")),
    }
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "n", 5);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "galois_group" => {
            match parse_coeffs_local(args) {
                Some(c) => {
                    state_set_str(state, "poly", &args.join(" "));
                    cmd_galois_group(&c)
                }
                None => "Error parsing coefficients\n".to_string(),
            }
        }
        "discriminant" => {
            match parse_coeffs_local(args) {
                Some(c) => {
                    state_set_str(state, "poly", &args.join(" "));
                    cmd_discriminant(&c)
                }
                None => "Error parsing coefficients\n".to_string(),
            }
        }
        "correspondence" => {
            let n = match parse_uint(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            if n < 2 { return "n must be >= 2\n".to_string(); }
            state_set_int(state, "n", n as i64);
            cmd_correspondence(n)
        }
        "fixed_field" => {
            if args.len() < 2 {
                return "Usage: fixed_field <p_coeffs...> <automorphism>\n  automorphism: id | sigma | tau | sigmatau\n".to_string();
            }
            let auto_desc = args[args.len()-1];
            let coeffs: Result<Vec<i64>, _> = args[..args.len()-1].iter().map(|s| s.parse()).collect();
            match coeffs {
                Ok(c) if !c.is_empty() => {
                    state_set_str(state, "poly", &args[..args.len()-1].join(" "));
                    state_set_str(state, "auto", auto_desc);
                    cmd_fixed_field(&c, auto_desc)
                }
                _ => "Could not parse polynomial\n".to_string(),
            }
        }
        "sqrt2_sqrt3" => cmd_sqrt2_sqrt3(),
        "cyclotomic_galois" => {
            let n = match parse_uint(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            if n < 2 { return "n must be >= 2\n".to_string(); }
            state_set_int(state, "n", n as i64);
            cmd_cyclotomic_galois(n)
        }
        "demo" => {
            let mut out = String::new();
            out.push_str(&cmd_sqrt2_sqrt3());
            out.push_str("\n");
            out.push_str(&cmd_galois_group(&[-2, 0, 1]));
            out.push_str("\n");
            out.push_str(&cmd_cyclotomic_galois(5));
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
    let n = state_get_int(state, "n").unwrap_or(5) as u64;
    let phi = euler_totient(n);
    // Galois correspondence: subgroups <-> subfields diagram
    c.text_bold(350.0, 30.0, &format!("Galois Correspondence for Q(zeta_{})/Q", n), 15.0, "#222", "middle");
    // Subgroups side (left)
    c.text_bold(130.0, 70.0, "Subgroups of G", 13.0, "#444", "middle");
    c.node_circle(130.0, 120.0, "G", "#eef", 25.0, 13.0);
    c.node_circle(130.0, 230.0, "H", "#eef", 25.0, 13.0);
    c.node_circle(130.0, 340.0, "{e}", "#eef", 25.0, 12.0);
    c.arrow(130.0, 145.0, 130.0, 202.0, "#444", 1.5);
    c.arrow(130.0, 255.0, 130.0, 312.0, "#444", 1.5);
    // Fields side (right)
    c.text_bold(520.0, 70.0, "Fixed Fields", 13.0, "#444", "middle");
    c.node_circle(520.0, 120.0, "Q", "#efe", 25.0, 13.0);
    c.node_circle(520.0, 230.0, "K^H", "#efe", 28.0, 12.0);
    c.node_circle(520.0, 340.0, &format!("Q(z{})", n), "#efe", 30.0, 11.0);
    c.arrow(520.0, 145.0, 520.0, 202.0, "#444", 1.5);
    c.arrow(520.0, 255.0, 520.0, 312.0, "#444", 1.5);
    // Correspondence labels
    c.text(325.0, 120.0, "<-Fix->", 12.0, "#666", "middle");
    c.text(325.0, 138.0, "-Gal->", 12.0, "#666", "middle");
    c.text(325.0, 230.0, "<-Fix->", 12.0, "#666", "middle");
    c.text(325.0, 248.0, "-Gal->", 12.0, "#666", "middle");
    c.text(325.0, 340.0, "<-Fix->", 12.0, "#666", "middle");
    c.text(325.0, 358.0, "-Gal->", 12.0, "#666", "middle");
    // Info
    c.text(350.0, 455.0, &format!("|G| = phi({}) = {}  (order-reversing)", n, phi), 12.0, "#555", "middle");
}

fn visualize_dot(g_dot: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(5) as u64;
    let phi = euler_totient(n);
    g_dot.node("G",   &[("label", &format!("G=Gal(K/Q), |G|={}", phi)), ("shape", "box")]);
    g_dot.node("H",   &[("label", "H < G (subgroup)"), ("shape", "box")]);
    g_dot.node("e",   &[("label", "{e}"), ("shape", "box")]);
    g_dot.node("Q",   &[("label", "Q (base field)"), ("shape", "ellipse")]);
    g_dot.node("KH",  &[("label", "K^H (fixed field)"), ("shape", "ellipse")]);
    g_dot.node("K",   &[("label", &format!("Q(zeta_{})", n)), ("shape", "ellipse")]);
    g_dot.edge("G", "H",  &[("label", "contains")]);
    g_dot.edge("H", "e",  &[("label", "contains")]);
    g_dot.edge("Q",  "KH", &[("label", "Fix(H)")]);
    g_dot.edge("KH", "K",  &[("label", "Fix({e})")]);
    g_dot.edge("G",  "Q",  &[("label", "Gal(K/Q)", ), ("style", "dashed")]);
    g_dot.edge("H",  "KH", &[("label", "Fix(H)",  ), ("style", "dashed")]);
    g_dot.edge("e",  "K",  &[("label", "Fix({e})" ), ("style", "dashed")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(5) as u64;
    // Left: subgroup lattice; Right: field lattice
    t.node("G",  -3.0, 4.0, "$G$", "draw");
    t.node("H",  -3.0, 2.0, "$H$", "draw");
    t.node("e",  -3.0, 0.0, "$\\{e\\}$", "draw");
    t.node("Q",   3.0, 0.0, "$\\mathbb{Q}$", "draw");
    t.node("KH",  3.0, 2.0, "$K^H$", "draw");
    t.node("K",   3.0, 4.0, &format!("$\\mathbb{{Q}}(\\zeta_{{{}}})$", n), "draw");
    t.arrow("G", "H", "", "-");
    t.arrow("H", "e", "", "-");
    t.arrow("Q", "KH", "", "-");
    t.arrow("KH", "K", "", "-");
    t.arrow("G",  "Q",  "Fix", "<->,dashed");
    t.arrow("H",  "KH", "Fix", "<->,dashed");
    t.arrow("e",  "K",  "Fix", "<->,dashed");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(5) as u64;
    a.text_at(2, 1,  &format!("Galois Correspondence for Q(zeta_{})/Q", n));
    a.text_at(2, 3,  "Subgroups (left)    <-Fix->    Fixed Fields (right)");
    a.text_at(2, 5,  "  G (whole group)   <------>   Q (base field)");
    a.text_at(2, 7,  "     |                               |");
    a.text_at(2, 9,  "  H (subgroup)      <------>   K^H (fixed field)");
    a.text_at(2, 11, "     |                               |");
    a.text_at(2, 13, "  {e} (trivial)     <------>   K=Q(zeta_n)");
    a.text_at(2, 15, "Larger subgroup = smaller fixed field (order-reversing)");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch31"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 31", "Galois Theory", "The symphony of symmetry and roots");
            print_note("Explore Galois groups, discriminants, and the fundamental correspondence.");
            println!("{}", show_help());
            repl("galois> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
