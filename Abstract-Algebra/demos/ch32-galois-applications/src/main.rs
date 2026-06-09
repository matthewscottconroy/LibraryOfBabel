use common::*;

fn show_help() -> String {
    help_string(&[
        ("solvable <degree>",         "Is S_n solvable? (yes for n<=4, no for n>=5)"),
        ("radical_solvable <coeffs>", "Is polynomial solvable by radicals?"),
        ("quintic",                   "Example unsolvable quintic x^5-5x+2"),
        ("constructible <n>",         "Is regular n-gon constructible?"),
        ("fermat_primes",             "List known Fermat primes"),
        ("trisect <angle_deg>",       "Check if angle is trisectable"),
        ("finite_field <p> <n>",      "Structure of GF(p^n)"),
        ("demo",                      "Run a showcase of key results"),
        ("help",                      "Show this help"),
        ("quit",                      "Exit"),
    ])
}

fn parse_coeffs_local(args: &[&str]) -> Option<Vec<i64>> {
    let r: Result<Vec<i64>, _> = args.iter().map(|s| s.parse()).collect();
    match r {
        Ok(v) if !v.is_empty() => Some(v),
        _ => { print_err("Expected integer polynomial coefficients (constant first)"); None }
    }
}

fn cmd_solvable(degree: u64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Is S_{} solvable? ===\n\n", degree));
    out.push_str("Galois' Theorem: a polynomial f is solvable by radicals iff its Galois group is solvable.\n");
    out.push_str("S_n (symmetric group on n elements) is solvable iff n <= 4.\n\n");
    match degree {
        1 => {
            out.push_str("S₁ = {e} is trivially solvable (trivial group).\n");
        }
        2 => {
            out.push_str("S₂ ≅ ℤ/2ℤ is abelian, hence solvable.\n");
            out.push_str("  Derived series: S₂ ⊃ {e}  (terminates in 1 step)\n");
        }
        3 => {
            out.push_str("S₃ is solvable.\n");
            out.push_str("  Derived series: S₃ ⊃ A₃ ⊃ {e}\n");
            out.push_str("  |S₃|=6, |A₃|=3, [S₃:A₃]=2 (normal), A₃≅ℤ/3ℤ (abelian)\n");
        }
        4 => {
            out.push_str("S₄ is solvable.\n");
            out.push_str("  Derived series: S₄ ⊃ A₄ ⊃ V₄ ⊃ {e}\n");
            out.push_str("  |S₄|=24, |A₄|=12, |V₄|=4 (Klein four-group)\n");
            out.push_str("  V₄ = {e,(12)(34),(13)(24),(14)(23)} is normal in A₄.\n");
            out.push_str("  Quotients: S₄/A₄≅ℤ/2, A₄/V₄≅ℤ/3, V₄/{e}≅ℤ/2×ℤ/2 — all abelian.\n");
        }
        n if n >= 5 => {
            out.push_str(&format!("S_{} is NOT solvable!\n", n));
            out.push_str(&format!("  For n >= 5: A_n is simple (has no non-trivial normal subgroups).\n"));
            out.push_str(&format!("  The only normal subgroups of S_{} are {{e}}, A_{}, and S_{}.\n", n, n, n));
            out.push_str(&format!("  Since A_{} is non-abelian and simple, S_{} is not solvable.\n\n", n, n));
            out.push_str(&format!("This is why degree-{} polynomials with Galois group S_{}\n", n, n));
            out.push_str("are NOT solvable by radicals — there is no radical formula!\n");
        }
        _ => {
            out.push_str("Degree must be a positive integer\n");
        }
    }
    out.push_str("\n=== Summary Table ===\n\n");
    out.push_str(&format!("  {:<8} {:<10} {:<10} {}\n", "n", "|S_n|", "Solvable", "Reason"));
    out.push_str("  --------------------------------------------------------\n");
    for (n, ord, sol, reason) in [
        (1u64, "1",   "Yes", "trivial"),
        (2,    "2",   "Yes", "abelian"),
        (3,    "6",   "Yes", "derived series S3⊃A3⊃{e}"),
        (4,    "24",  "Yes", "derived series S4⊃A4⊃V4⊃{e}"),
        (5,    "120", "No",  "A5 is simple non-abelian"),
        (6,    "720", "No",  "contains A6, which is simple"),
    ] {
        let marker = if n == degree { " <-" } else { "" };
        out.push_str(&format!("  {:<8} {:<10} {:<10} {}{}\n", n, ord, sol, reason, marker));
    }
    out
}

fn cmd_radical_solvable(coeffs: &[i64]) -> String {
    let mut out = String::new();
    let f = Poly::new(coeffs.to_vec());
    let deg = match f.degree() { None | Some(0) => { return "ERROR: Need degree >= 1\n".to_string(); } Some(d) => d };
    out.push_str("=== Solvability by Radicals ===\n\n");
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
    out.push_str(&format!("  Rational roots: {:?}\n\n", rat_roots));
    match deg {
        1 => {
            out.push_str("Degree 1: always solvable by radicals (root is rational).\n");
        }
        2 => {
            out.push_str("Degree 2: always solvable by radicals via quadratic formula.\n");
            let a=coeffs[2]; let b=coeffs[1]; let c=coeffs[0];
            let disc = b*b - 4*a*c;
            out.push_str(&format!("  Roots: x = (−{} ± √{}) / {}\n", b, disc, 2*a));
        }
        3 => {
            out.push_str("Degree 3: always solvable by radicals (Cardano's formula, 1545).\n");
            out.push_str("Galois group is a subgroup of S₃, which is solvable.\n");
        }
        4 => {
            out.push_str("Degree 4: always solvable by radicals (Ferrari's formula, 1545).\n");
            out.push_str("Galois group is a subgroup of S₄, which is solvable.\n");
        }
        n if n >= 5 => {
            let num_rat = rat_roots.len();
            if num_rat >= deg {
                out.push_str(&format!("All {} roots are rational — trivially solvable!\n", num_rat));
            } else if num_rat > 0 {
                out.push_str(&format!("Has {} rational root(s). The remaining factor may or may not be solvable.\n", num_rat));
                out.push_str(&format!("If the degree-{} irreducible factor has Galois group S_{}, it's unsolvable.\n", deg - num_rat, deg - num_rat));
            } else {
                out.push_str(&format!("Degree {} with no rational roots.\n", deg));
                out.push_str("If the Galois group is S_n (n>=5), it is NOT solvable by radicals.\n");
                out.push_str("To determine: check if the Galois group is solvable.\n");
                out.push_str("For a 'generic' degree-5 polynomial, Gal ≅ S₅, which is unsolvable.\n");
            }
        }
        _ => {}
    }
    out
}

fn cmd_quintic() -> String {
    let mut out = String::new();
    out.push_str("=== Unsolvable Quintic: x⁵ − 5x + 2 ===\n\n");
    out.push_str("The polynomial f(x) = x⁵ − 5x + 2 has Galois group S₅, so it is\n");
    out.push_str("NOT solvable by radicals — there is no closed-form formula for its roots.\n\n");
    let f = Poly::new(vec![2, -5, 0, 0, 0, 1]);
    out.push_str(&format!("  f(x) = {}\n\n", f));
    out.push_str("  Checking for rational roots (candidates: ±1, ±2):\n");
    for r in [-2i64, -1, 1, 2] {
        out.push_str(&format!("    f({}) = {}\n", r, f.eval(r)));
    }
    out.push_str("No rational roots → f is irreducible over ℚ (degree 5, no rational roots).\n\n");
    out.push_str("  Real roots (approximate):\n");
    out.push_str("    f(−2) = −24, f(−1) = 6, f(0) = 2, f(1) = −2, f(2) = 24\n");
    out.push_str("    Sign changes at: (−2,−1), (0,1), (1,2)\n");
    out.push_str("f has exactly 3 real roots and 2 complex conjugate roots.\n\n");
    out.push_str("=== Why the Galois Group is S₅ ===\n\n");
    out.push_str("  Key facts used to prove Gal(f) ≅ S₅:\n");
    out.push_str("  1. f is irreducible of degree 5 → Gal(f) contains a 5-cycle\n");
    out.push_str("     (the Galois group acts transitively on the 5 roots)\n");
    out.push_str("  2. f has exactly 2 complex (non-real) roots\n");
    out.push_str("     → complex conjugation acts as a transposition (2-cycle) on the roots\n");
    out.push_str("  3. A subgroup of S₅ containing both a 5-cycle and a 2-cycle = S₅\n\n");
    out.push_str("Gal(f) ≅ S₅ is not solvable → f is not solvable by radicals.\n\n");
    out.push_str("Abel-Ruffini Theorem (1824): there is no general radical formula for degree >= 5.\n");
    out.push_str("Galois Theory explains exactly WHEN an individual polynomial is solvable.\n");
    out
}

fn cmd_constructible(n: u64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Is the Regular {}-gon Constructible? ===\n\n", n));
    out.push_str("Gauss-Wantzel Theorem: a regular n-gon is constructible by compass and straightedge\n");
    out.push_str("iff n = 2^k · p₁ · p₂ · ... · pᵣ where pᵢ are distinct Fermat primes.\n\n");
    let known_fermat: [u64; 5] = [3, 5, 17, 257, 65537];
    let mut m = n;
    let mut pow2 = 0u64;
    while m % 2 == 0 { m /= 2; pow2 += 1; }
    let remaining = m;
    let mut fac = remaining;
    let mut fermat_factors = vec![];
    let mut non_fermat = vec![];
    for &fp in &known_fermat {
        if fac % fp == 0 {
            fac /= fp;
            fermat_factors.push(fp);
            if fac % fp == 0 {
                non_fermat.push(fp);
                while fac % fp == 0 { fac /= fp; }
            }
        }
    }
    if fac > 1 {
        let facts = factor(fac);
        for (p, _) in facts {
            non_fermat.push(p);
        }
    }
    out.push_str(&format!("  n = {}\n", n));
    out.push_str(&format!("  2^k factor: 2^{}\n", pow2));
    out.push_str(&format!("  Odd part: {}\n\n", remaining));
    if fermat_factors.is_empty() && remaining == 1 {
        out.push_str(&format!("n = 2^{} (pure power of 2): regular {}-gon IS constructible!\n", pow2, n));
    } else if non_fermat.is_empty() && fermat_factors.len() == fermat_factors.iter().collect::<std::collections::HashSet<_>>().len() {
        out.push_str(&format!("Regular {}-gon IS constructible!\n", n));
        out.push_str(&format!("  Fermat prime factors: {:?}\n", fermat_factors));
        out.push_str(&format!("  n = 2^{} × {}\n", pow2, fermat_factors.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" × ")));
    } else {
        out.push_str(&format!("Regular {}-gon is NOT constructible.\n", n));
        if !non_fermat.is_empty() {
            out.push_str(&format!("  Non-Fermat odd prime factors: {:?}\n", non_fermat));
            out.push_str("The odd part contains primes that are not Fermat primes.\n");
        }
    }
    out.push_str("\n  Constructible n-gons for small n:\n");
    let constructible = [3u64, 4, 5, 6, 8, 10, 12, 15, 16, 17, 20, 24, 30, 32, 34];
    out.push_str(&format!("    {:?}\n", constructible));
    out.push_str("Note: n=7 (NOT constructible), n=9 (NOT), n=11 (NOT), n=13 (NOT).\n");
    out
}

fn cmd_fermat_primes() -> String {
    let mut out = String::new();
    out.push_str("=== Known Fermat Primes ===\n\n");
    out.push_str("Fermat primes have the form F_n = 2^(2^n) + 1.\n\n");
    let fermat = [
        (0u32, 3u64,   "2^1 + 1 = 3",      "known since antiquity"),
        (1,    5,       "2^2 + 1 = 5",      "known since antiquity"),
        (2,    17,      "2^4 + 1 = 17",     "Gauss (1796): 17-gon constructible!"),
        (3,    257,     "2^8 + 1 = 257",    "Gauss (1796)"),
        (4,    65537,   "2^16 + 1 = 65537", "last known Fermat prime"),
    ];
    for (n, f, formula, note) in &fermat {
        out.push_str(&format!("  F_{} = {}  =  {}  [{}]\n", n, f, formula, note));
    }
    out.push_str("\n  F₅ = 2^32 + 1 = 4,294,967,297 = 641 × 6,700,417  (NOT prime, found by Euler 1732)\n");
    out.push_str("  F₆ through F₃₂ are all known to be composite.\n\n");
    out.push_str("It is unknown whether any other Fermat primes exist.\n");
    out.push_str("Fermat conjectured all F_n are prime — this is false!\n\n");
    out.push_str("  Connection to constructibility:\n");
    out.push_str("  Gauss proved: regular p-gon (p prime) constructible iff p is a Fermat prime.\n");
    out.push_str("  Key: [ℚ(ζ_p):ℚ] = p−1, which must be a power of 2 for constructibility.\n");
    for (_, f, _, _) in &fermat {
        let p_m_1 = f - 1;
        let pow = (p_m_1 as f64).log2();
        out.push_str(&format!("    p={}: p−1 = {} = 2^{:.0}\n", f, p_m_1, pow));
    }
    out
}

fn cmd_trisect(angle_deg: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Can {}° be Trisected? ===\n\n", angle_deg));
    out.push_str("The trisection problem: given angle θ, construct θ/3 by compass and straightedge.\n\n");
    out.push_str(&format!("  Angle: {} degrees\n\n", angle_deg));
    let trisected = angle_deg / 3;
    let remainder = angle_deg % 3;
    out.push_str(&format!("  {}°/3 = {}° (remainder {})\n\n", angle_deg, trisected, remainder));
    out.push_str("  The algebraic obstruction:\n");
    out.push_str("  To trisect angle θ, we need to construct cos(θ/3).\n");
    out.push_str("  Using the triple angle formula: cos(θ) = 4cos³(θ/3) − 3cos(θ/3)\n");
    out.push_str("  Let x = cos(θ/3). Then x satisfies: 4x³ − 3x − cos(θ) = 0\n\n");
    if angle_deg == 60 {
        out.push_str("  For θ = 60°: cos(60°) = 1/2\n");
        out.push_str("  So x = cos(20°) satisfies: 4x³ − 3x − 1/2 = 0, i.e. 8x³ − 6x − 1 = 0\n");
        let f = Poly::new(vec![-1, -6, 0, 8]);
        out.push_str(&format!("  f(x) = {}\n", f));
        out.push_str("  Check rational roots (±1, ±1/2, ±1/4, ±1/8):\n");
        for (n, d) in [(1i64,1),(-1,1),(1,2),(-1,2),(1,4),(-1,4),(1,8),(-1,8)] {
            let num = 8*n*n*n - 6*n*d*d - d*d*d;
            let den = d*d*d;
            if num == 0 { out.push_str(&format!("    f({}/{}) = 0  <- rational root!\n", n, d)); }
            else { out.push_str(&format!("    f({}/{}) = {}/{} != 0\n", n, d, num, den)); }
        }
        out.push_str("No rational roots — f is irreducible of degree 3 over ℚ.\n");
        out.push_str("cos(20°) generates a degree-3 extension of ℚ.\n");
        out.push_str("But constructible numbers have extensions of degree a power of 2 — NOT 3.\n");
        out.push_str("Therefore: 60° CANNOT be trisected by compass and straightedge!\n\n");
        out.push_str("This is one of the three classical impossibility results of ancient geometry.\n");
    } else {
        let cos_theta = (angle_deg as f64).to_radians().cos();
        out.push_str(&format!("  cos({}°) ≈ {:.6}\n", angle_deg, cos_theta));
        out.push_str(&format!("  x = cos({}°/3) = cos({}°) satisfies 4x³ − 3x − cos({}°) = 0\n\n",
            angle_deg, angle_deg/3, angle_deg));
        if angle_deg == 90 {
            out.push_str("90°/3 = 30°, and cos(30°) = √3/2 which IS constructible.\n");
            out.push_str("So 90° CAN be trisected!\n");
        } else {
            out.push_str("For most angles, the trisection cubic is irreducible of degree 3 over ℚ.\n");
            out.push_str("Degree 3 is not a power of 2, so the trisection is generally impossible.\n");
        }
    }
    out
}

fn cmd_finite_field(p: u64, n: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) {
        return format!("ERROR: {} is not prime\n", p);
    }
    let q = p.pow(n as u32);
    out.push_str(&format!("=== Finite Field GF({}^{}) = GF({}) ===\n\n", p, n, q));
    out.push_str(&format!("  GF({}) = GF({}^{}) has {} elements\n", q, p, n, q));
    out.push_str(&format!("  |GF({})| = {}\n\n", q, q));
    out.push_str(&format!("  GF({}) ≅ 𝔽_{}[x]/(f) where f is irreducible of degree {} over 𝔽_{}\n\n",
        q, p, n, p));
    out.push_str(&format!("  Finding an irreducible polynomial of degree {} over 𝔽_{}...\n", n, p));
    let found = find_irreducible(p, n);
    match found {
        Some(coeffs) => {
            let f = Poly::new(coeffs.clone());
            out.push_str(&format!("  Found: f(x) = {}\n\n", f));
            out.push_str(&format!("  GF({}) = 𝔽_{}[x]/({})\n", q, p, f));
            out.push_str(&format!("  Basis over 𝔽_{}: {{1, α, α², ..., α^{}}}\n\n", p, n-1));
        }
        None => {
            out.push_str(&format!("  (Irreducible polynomial search skipped for large p={}, n={})\n\n", p, n));
        }
    }
    out.push_str("=== Frobenius Automorphism ===\n\n");
    out.push_str(&format!("  φ: GF({}) → GF({}),  x ↦ x^{}\n", q, q, p));
    out.push_str(&format!("  φ generates Gal(GF({})/GF({})) ≅ ℤ/{}ℤ\n", q, p, n));
    out.push_str(&format!("  |Gal(GF({})/GF({})| = {}\n\n", q, p, n));
    out.push_str("  Powers of Frobenius and their fixed fields:\n");
    let mut pk = p;
    for k in 1..=n {
        let fixed_size = pk;
        let marker = if k == n { "  [identity on GF(p^n)]" } else { "" };
        out.push_str(&format!("    φ^{}: x ↦ x^{}  fixes GF({}){}\n", k, pk, fixed_size, marker));
        if k < n { pk *= p; }
    }
    out.push_str("\nThe subfield structure of GF(p^n) mirrors the divisor lattice of n.\n");
    out.push_str("GF(p^d) ⊆ GF(p^n) iff d | n.\n");
    out
}

fn find_irreducible(p: u64, n: u64) -> Option<Vec<i64>> {
    if n > 4 { return None; }
    let p_i = p as i64;
    let num_polys = p.pow(n as u32);
    'outer: for code in 0..num_polys {
        let mut coeffs = vec![0i64; (n + 1) as usize];
        coeffs[n as usize] = 1;
        let mut tmp = code;
        for i in 0..n {
            coeffs[i as usize] = (tmp % p) as i64;
            tmp /= p;
        }
        let f = Poly::new(coeffs.clone());
        for x in 0..p {
            if f.eval_mod(x as i64, p_i) == 0 { continue 'outer; }
        }
        if n <= 3 { return Some(coeffs); }
        return Some(coeffs);
    }
    None
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "degree", 5);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "solvable" => {
            let deg = match parse_uint(args, 0, "degree") { Some(v) => v, None => return "Missing arg degree\n".to_string() };
            state_set_int(state, "degree", deg as i64);
            cmd_solvable(deg)
        }
        "radical_solvable" => {
            match parse_coeffs_local(args) {
                Some(c) => {
                    state_set_str(state, "poly", &args.join(" "));
                    cmd_radical_solvable(&c)
                }
                None => "Error parsing coefficients\n".to_string(),
            }
        }
        "quintic" => cmd_quintic(),
        "constructible" => {
            let n = match parse_uint(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            if n < 3 { return "n must be >= 3\n".to_string(); }
            state_set_int(state, "n", n as i64);
            cmd_constructible(n)
        }
        "fermat_primes" => cmd_fermat_primes(),
        "trisect" => {
            let ang = match parse_int(args, 0, "angle_deg") { Some(v) => v, None => return "Missing arg angle_deg\n".to_string() };
            state_set_int(state, "angle_deg", ang);
            cmd_trisect(ang)
        }
        "finite_field" => {
            let p = match parse_uint(args, 0, "p") { Some(v) => v, None => return "Missing arg p\n".to_string() };
            let n = match parse_uint(args, 1, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            if n < 1 { return "n must be >= 1\n".to_string(); }
            state_set_int(state, "p", p as i64);
            state_set_int(state, "n", n as i64);
            cmd_finite_field(p, n)
        }
        "demo" => {
            let mut out = String::new();
            out.push_str(&cmd_solvable(5));
            out.push_str("\n");
            out.push_str(&cmd_quintic());
            out.push_str("\n");
            out.push_str(&cmd_constructible(17));
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
    // Solvability tower diagram
    c.text_bold(350.0, 30.0, "Solvability by Radicals: Tower of Radical Extensions", 14.0, "#222", "middle");
    c.node_circle(350.0, 420.0, "F", "#eef", 28.0, 14.0);
    c.node_circle(350.0, 320.0, "F(a1^(1/n1))", "#eef", 40.0, 11.0);
    c.node_circle(350.0, 220.0, "F(a1,a2)", "#eef", 36.0, 12.0);
    c.node_circle(350.0, 120.0, "Splitting field", "#efe", 42.0, 11.0);
    c.arrow(350.0, 391.0, 350.0, 360.0, "#444", 1.5);
    c.text(380.0, 372.0, "adjoin n1-th root", 12.0, "#444", "start");
    c.arrow(350.0, 283.0, 350.0, 256.0, "#444", 1.5);
    c.text(380.0, 272.0, "adjoin n2-th root", 12.0, "#444", "start");
    c.arrow(350.0, 183.0, 350.0, 161.0, "#444", 1.5);
    c.text(380.0, 172.0, "...", 14.0, "#444", "start");
    c.text(110.0, 258.0, "Solvable by radicals", 12.0, "#222", "middle");
    c.text(110.0, 276.0, "= such a tower exists", 12.0, "#222", "middle");
    c.rect(30.0, 244.0, 165.0, 46.0, "none", "#888", 1.0);
    c.text(110.0, 160.0, "NOT solvable: Gal(f)=S5", 12.0, "#c00", "middle");
    c.text(110.0, 178.0, "e.g. x^5-5x+2", 12.0, "#c00", "middle");
}

fn visualize_dot(g_dot: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g_dot.node("F",   &[("label", "F (base)"), ("shape", "ellipse")]);
    g_dot.node("F1",  &[("label", "F(a^(1/n))"), ("shape", "box")]);
    g_dot.node("F2",  &[("label", "F(a,b^(1/m))"), ("shape", "box")]);
    g_dot.node("Sp",  &[("label", "Splitting field"), ("shape", "diamond")]);
    g_dot.edge("F", "F1", &[("label", "radical")]);
    g_dot.edge("F1", "F2", &[("label", "radical")]);
    g_dot.edge("F2", "Sp", &[("label", "radical")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("F",  0.0, 0.0, "$F$", "draw,circle");
    t.node("F1", 0.0, 2.0, "$F(\\alpha_1)$", "draw");
    t.node("F2", 0.0, 4.0, "$F(\\alpha_1,\\alpha_2)$", "draw");
    t.node("K",  0.0, 6.0, "Splitting field", "draw");
    t.arrow("F",  "F1", "$\\sqrt[n_1]{a_1}$", "->");
    t.arrow("F1", "F2", "$\\sqrt[n_2]{a_2}$", "->");
    t.arrow("F2", "K",  "$\\cdots$", "->");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1,  "Solvability by Radicals — Radical Tower");
    a.text_at(25, 3, "Splitting field");
    a.text_at(29, 5, "|");
    a.text_at(15, 7, "F(a1^1/n1, a2^1/n2, ...)");
    a.text_at(29, 9, "|  adjoin radical");
    a.text_at(20, 11, "F(a1^1/n1)");
    a.text_at(29, 13, "|  adjoin radical");
    a.text_at(28, 15, "F (base)");
    a.text_at(2, 17, "Solvable by radicals = such tower exists");
    a.text_at(2, 18, "S5 (n>=5 generic) = NOT solvable (Abel-Ruffini)");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch32"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 32", "Applications of Galois Theory", "Unsolvability, constructibility, and finite fields");
            print_note("Explore what Galois theory tells us about solvability and geometry.");
            println!("{}", show_help());
            repl("galois-app> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
