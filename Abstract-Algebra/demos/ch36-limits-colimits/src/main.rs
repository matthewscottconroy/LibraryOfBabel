use common::*;

fn show_help() -> String {
    help_string(&[
        ("product <n> <m>",       "Z/nZ x Z/mZ with universal property"),
        ("coproduct <n> <m>",     "Coproduct in Ab and Grp"),
        ("equalizer <n> <a> <b>", "Equalizer of x->ax and x->bx on Z/nZ"),
        ("pullback <n> <m> <k>",  "Fiber product Z/nZ x_{Z/kZ} Z/mZ"),
        ("inverse_limit <p>",     "Z_p = lim<- Z/p^n for prime p"),
        ("pushout <n> <m> <k>",   "Pushout (amalgamated sum) in Ab"),
        ("padic <p> <depth>",     "p-adic integers: elements of lim<- Z/p^k"),
        ("demo",                  "Run a showcase of key results"),
        ("help",                  "Show this help"),
        ("quit",                  "Exit"),
    ])
}

fn cmd_product(n: i64, m: i64) -> String {
    let mut out = String::new();
    let ord = n * m;
    out.push_str(&format!("=== Product ℤ/{}ℤ × ℤ/{}ℤ ===\n\n", n, m));
    out.push_str(&format!("  Elements: {{(a,b) : a ∈ ℤ/{}ℤ, b ∈ ℤ/{}ℤ}}\n", n, m));
    out.push_str(&format!("Order: {} × {} = {}\n", n, m, ord));
    let g = gcd(n, m);
    let l = lcm(n, m);
    out.push_str(&format!("gcd(n,m) = {}\n", g));
    out.push_str(&format!("lcm(n,m) = {}\n\n", l));
    if g == 1 {
        out.push_str(&format!("gcd(n,m)=1: ℤ/{}ℤ × ℤ/{}ℤ ≅ ℤ/{}ℤ (CRT)\n", n, m, ord));
        out.push_str("By the Chinese Remainder Theorem, since gcd(n,m)=1, the product is cyclic.\n");
    } else {
        out.push_str(&format!("gcd(n,m)={} > 1: ℤ/{}ℤ × ℤ/{}ℤ is not cyclic.\n", g, n, m));
        out.push_str(&format!("Exponent of the group = lcm({},{}) = {}\n", n, m, l));
    }
    out.push_str("\n=== Universal Property of the Product ===\n\n");
    out.push_str("  ℤ/nℤ × ℤ/mℤ comes with projections:\n");
    out.push_str(&format!("    π₁: ℤ/{}ℤ × ℤ/{}ℤ → ℤ/{}ℤ,  (a,b) ↦ a\n", n, m, n));
    out.push_str(&format!("    π₂: ℤ/{}ℤ × ℤ/{}ℤ → ℤ/{}ℤ,  (a,b) ↦ b\n\n", n, m, m));
    out.push_str("  Universal property: for any abelian group A and homomorphisms\n");
    out.push_str(&format!("    f₁: A → ℤ/{}ℤ,  f₂: A → ℤ/{}ℤ\n", n, m));
    out.push_str("  there is a unique φ: A → ℤ/nℤ × ℤ/mℤ with π₁∘φ = f₁ and π₂∘φ = f₂.\n");
    out.push_str("  Explicitly: φ(a) = (f₁(a), f₂(a))\n\n");
    let show_max = (ord).min(12);
    out.push_str("  Elements (showing first 12):\n");
    let mut count = 0;
    'outer: for a in 0..n {
        for b in 0..m {
            out.push_str(&format!("    ({}, {})  order = lcm({}, {}) = {}\n",
                a, b,
                if a == 0 { n } else { n / gcd(a, n) },
                if b == 0 { m } else { m / gcd(b, m) },
                lcm(
                    if a == 0 { n } else { n / gcd(a, n) },
                    if b == 0 { m } else { m / gcd(b, m) }
                )));
            count += 1;
            if count >= show_max { break 'outer; }
        }
    }
    if ord > show_max { out.push_str(&format!("    ... ({} elements total)\n", ord)); }
    out
}

fn cmd_coproduct(n: i64, m: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Coproduct of ℤ/{}ℤ and ℤ/{}ℤ ===\n\n", n, m));
    out.push_str("=== In Ab (abelian groups): Coproduct = Direct Sum = Product ===\n\n");
    out.push_str("  In Ab, coproduct = product for finite groups.\n");
    out.push_str(&format!("  ℤ/{}ℤ ⊕ ℤ/{}ℤ = ℤ/{}ℤ × ℤ/{}ℤ\n", n, m, n, m));
    out.push_str(&format!("  |ℤ/{}ℤ ⊕ ℤ/{}ℤ| = {}\n\n", n, m, n * m));
    out.push_str("  Inclusions (coproduct maps):\n");
    out.push_str(&format!("    ι₁: ℤ/{}ℤ → ℤ/{}ℤ⊕ℤ/{}ℤ,  a ↦ (a, 0)\n", n, n, m));
    out.push_str(&format!("    ι₂: ℤ/{}ℤ → ℤ/{}ℤ⊕ℤ/{}ℤ,  b ↦ (0, b)\n\n", m, n, m));
    out.push_str("  Universal property: for any A with maps f₁: ℤ/nℤ→A, f₂: ℤ/mℤ→A,\n");
    out.push_str("    unique φ: ℤ/nℤ⊕ℤ/mℤ → A with φ∘ι₁=f₁ and φ∘ι₂=f₂.\n");
    out.push_str("    Explicitly: φ(a,b) = f₁(a) + f₂(b)\n\n");
    out.push_str("=== In Grp (non-abelian groups): Coproduct = Free Product ===\n\n");
    out.push_str(&format!("  ℤ/{}ℤ * ℤ/{}ℤ = free product (not direct product!)\n", n, m));
    out.push_str("  Elements: reduced words alternating between generators of each factor.\n");
    out.push_str("  Example: ℤ/2ℤ * ℤ/2ℤ is infinite (= infinite dihedral group D_∞)!\n");
    out.push_str(&format!("  ℤ/{}ℤ * ℤ/{}ℤ has order ∞ for n,m ≥ 2\n\n", n, m));
    out.push_str("In Grp: coproduct ≠ product (for non-trivial groups).\n");
    out.push_str("In Ab: coproduct = product (for finite families; in general, direct sum ≠ direct product).\n");
    out
}

fn cmd_equalizer(n: i64, a: i64, b: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Equalizer of x↦{}x and x↦{}x on ℤ/{}ℤ ===\n\n", a, b, n));
    out.push_str(&format!("  f: ℤ/{}ℤ → ℤ/{}ℤ,  x ↦ {}x mod {}\n", n, n, a, n));
    out.push_str(&format!("  g: ℤ/{}ℤ → ℤ/{}ℤ,  x ↦ {}x mod {}\n\n", n, n, b, n));
    out.push_str("  Equalizer = {x ∈ ℤ/nℤ : f(x) = g(x)} = {x : ax ≡ bx (mod n)}\n");
    out.push_str(&format!("           = {{x : (a-b)x ≡ 0 (mod {})}}\n", n));
    out.push_str(&format!("           = {{x : {}x ≡ 0 (mod {})}}\n\n", a - b, n));
    let diff = (a - b).rem_euclid(n);
    let g = gcd(diff, n);
    out.push_str(&format!("  a - b = {} ≡ {} (mod {})\n", a - b, diff, n));
    out.push_str(&format!("  gcd({}, {}) = {}\n", diff, n, g));
    out.push_str(&format!("  Solutions: x ≡ 0 (mod {}/{}) = x ≡ 0 (mod {})\n\n", n, g, n/g.max(1)));
    let mut eq_elements = vec![];
    for x in 0..n {
        let fx = (a * x).rem_euclid(n);
        let gx = (b * x).rem_euclid(n);
        if fx == gx { eq_elements.push(x); }
    }
    out.push_str(&format!("  Equalizer elements: {{{}}}\n",
        eq_elements.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")));
    out.push_str(&format!("Equalizer order: {}\n\n", eq_elements.len()));
    if eq_elements.len() == n as usize {
        out.push_str("f = g: equalizer is all of ℤ/nℤ.\n");
    } else if eq_elements.len() == 1 {
        out.push_str("Equalizer = {0}: f and g agree only at 0.\n");
    } else {
        out.push_str(&format!("Equalizer ≅ ℤ/{}ℤ  (subgroup of order {})\n", eq_elements.len(), eq_elements.len()));
    }
    out.push_str("\n  Universal property: the equalizer E → ℤ/nℤ is the inclusion,\n");
    out.push_str("    and any map h: A → ℤ/nℤ with f∘h = g∘h factors uniquely through E.\n");
    out
}

fn cmd_pullback(n: i64, m: i64, k: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Pullback: ℤ/{}ℤ ×_{{ℤ/{}ℤ}} ℤ/{}ℤ ===\n\n", n, k, m));
    let g1 = gcd(n, k); let g2 = gcd(m, k);
    out.push_str(&format!("  π_n: ℤ/{}ℤ → ℤ/{}ℤ,  x ↦ x mod {}\n", n, k, k));
    out.push_str(&format!("  π_m: ℤ/{}ℤ → ℤ/{}ℤ,  y ↦ y mod {}\n", m, k, k));
    out.push_str(&format!("  (using the canonical maps ℤ/{}ℤ → ℤ/gcd({},{})ℤ and ℤ/{}ℤ → ℤ/gcd({},{})ℤ)\n\n",
        n, n, k, m, m, k));
    out.push_str("  Pullback P = {(x,y) ∈ ℤ/nℤ × ℤ/mℤ : π_n(x) = π_m(y)}\n\n");
    let mut pb_elements = vec![];
    for x in 0..n {
        for y in 0..m {
            let px = (x * g1).rem_euclid(k) / (n / g1).max(1);
            let py = (y * g2).rem_euclid(k) / (m / g2).max(1);
            let g_all = gcd(gcd(n, m), k);
            if x.rem_euclid(g_all) == y.rem_euclid(g_all) {
                let _ = (px, py);
                pb_elements.push((x, y));
            }
        }
    }
    let show_max = pb_elements.len().min(12);
    out.push_str(&format!("  Pullback elements (gcd({},{},{}) = {}):\n", n, m, k, gcd(gcd(n,m), k)));
    for &(x, y) in &pb_elements[..show_max] {
        out.push_str(&format!("    ({}, {})\n", x, y));
    }
    if pb_elements.len() > show_max {
        out.push_str(&format!("    ... ({} total)\n", pb_elements.len()));
    }
    out.push_str(&format!("Order of pullback: {}\n\n", pb_elements.len()));
    out.push_str("  Universal property: P maps to both ℤ/nℤ and ℤ/mℤ,\n");
    out.push_str("    and any A with maps A→ℤ/nℤ and A→ℤ/mℤ agreeing in ℤ/kℤ\n");
    out.push_str("    factors uniquely through P.\n\n");
    out.push_str("Pullback = fiber product: elements of the product that agree 'downstairs'.\n");
    out
}

fn cmd_inverse_limit(p: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) {
        return format!("ERROR: {} is not prime\n", p);
    }
    out.push_str(&format!("=== p-adic Integers ℤ_p = lim← ℤ/p^n for p={} ===\n\n", p));
    out.push_str("The inverse limit ℤ_p = lim← ℤ/p^n is built from compatible sequences.\n\n");
    out.push_str("  System: ... → ℤ/p³ℤ → ℤ/p²ℤ → ℤ/pℤ  (reduction maps)\n\n");
    out.push_str("  An element of ℤ_p is a sequence (a₁, a₂, a₃, ...) where:\n");
    out.push_str("    aₙ ∈ ℤ/p^nℤ  and  aₙ ≡ aₙ₋₁ (mod p^{n-1})\n\n");
    out.push_str("=== Stages of the Inverse System ===\n\n");
    for n in 1u32..=5 {
        let pn = p.pow(n);
        out.push_str(&format!("  ℤ/{}ℤ = {{0, 1, ..., {}}}  (order = {}^{} = {})\n",
            pn, pn-1, p, n, pn));
    }
    out.push_str("\n=== Example Compatible Sequences ===\n\n");
    out.push_str(&format!("A p-adic integer is written in base p: a = a₀ + a₁p + a₂p² + ..., aᵢ ∈ {{0..{}}}\n\n", p-1));
    let show_examples = 4usize;
    out.push_str("  Some elements of ℤ_p (as compatible sequences):\n");
    for val in 0..show_examples as u64 {
        let seq: Vec<u64> = (1..=5).map(|n| {
            let pn = p.pow(n as u32);
            val % pn
        }).collect();
        out.push_str(&format!("    {} → {:?} [ordinary integer {} viewed in ℤ_p]\n", val, seq, val));
    }
    out.push_str("\n");
    out.push_str(&format!("  −1 in ℤ_{}: the sequence (p−1, p²−1, p³−1, ...) = ({},...)\n", p, p-1));
    for n in 1u32..=4 {
        let pn = p.pow(n);
        out.push_str(&format!("    mod p^{} = {} = {}\n", n, pn-1, pn-1));
    }
    out.push_str(&format!("−1 = ...{r}{r}{r} in base {} (all digits = p-1 = {})\n\n", p, p-1, r = p-1));
    out.push_str(&format!("ℤ_p: an uncountable compact ring (the {}-adic integers)\n", p));
    out.push_str("  ℤ_p contains ℤ as a dense subring.\n");
    out.push_str("  ℤ_p / pⁿℤ_p ≅ ℤ/pⁿℤ for all n.\n");
    out.push_str("ℤ_p is a DVR (discrete valuation ring) with maximal ideal pℤ_p and residue field 𝔽_p.\n");
    out
}

fn cmd_pushout(n: i64, m: i64, k: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("=== Pushout in Ab: ℤ/{}ℤ ⊔_{{ℤ/{}ℤ}} ℤ/{}ℤ ===\n\n", n, k, m));
    out.push_str(&format!("  Maps: f: ℤ/{}ℤ → ℤ/{}ℤ  and  g: ℤ/{}ℤ → ℤ/{}ℤ\n\n", k, n, k, m));
    out.push_str("Pushout in Ab: P = (ℤ/nℤ ⊕ ℤ/mℤ) / {(f(x), −g(x)) : x ∈ ℤ/kℤ}\n\n");
    out.push_str("  The subgroup to quotient by is generated by:\n");
    out.push_str(&format!("    {{(f(x), −g(x)) : x ∈ ℤ/{}ℤ}}\n\n", k));
    let l = lcm(n, m);
    out.push_str("  For the canonical projections (x mod n and x mod m):\n");
    out.push_str(&format!("Pushout ℤ/{}ℤ ⊔_{{ℤ/{}ℤ}} ℤ/{}ℤ ≅ ℤ/{}ℤ\n\n", n, k, m, l));
    out.push_str(&format!("  This is because the pushout colimit 'glues' ℤ/{}ℤ and ℤ/{}ℤ\n", n, m));
    out.push_str(&format!("  along their common quotient ℤ/{}ℤ.\n\n", k));
    out.push_str("In Ab: pushout = tensor product over ℤ in some sense, and also coequalizer.\n");
    out.push_str("Pushout is dual to pullback: reverse all arrows.\n\n");
    out.push_str("  Universal property: pushout P comes with maps:\n");
    out.push_str(&format!("    j₁: ℤ/{}ℤ → P  and  j₂: ℤ/{}ℤ → P\n", n, m));
    out.push_str("    and j₁∘f = j₂∘g (the square commutes).\n");
    out.push_str("  For any A with maps h₁: ℤ/nℤ→A and h₂: ℤ/mℤ→A with h₁∘f = h₂∘g,\n");
    out.push_str("  there is a unique φ: P → A.\n");
    out
}

fn cmd_padic(p: u64, depth: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) {
        return format!("ERROR: {} is not prime\n", p);
    }
    if depth == 0 { return "ERROR: depth must be >= 1\n".to_string(); }
    out.push_str(&format!("=== p-adic Integer Digits (p={}, depth={}) ===\n\n", p, depth));
    out.push_str("A p-adic integer a = a₀ + a₁p + a₂p² + ... with 0 ≤ aᵢ < p.\n");
    out.push_str("Determined by the compatible sequence (a mod p, a mod p², a mod p³, ...).\n\n");
    out.push_str("=== Ordinary Integers as p-adic Integers ===\n\n");
    let show = [0u64, 1, p-1, p, p+1, p*p-1];
    for &val in &show {
        out.push_str(&format!("  {} in ℤ_{}:\n", val, p));
        let mut v = val;
        let mut digits = vec![];
        for _ in 0..depth {
            digits.push(v % p);
            v /= p;
        }
        let digits_str: Vec<String> = digits.iter().map(|d| d.to_string()).collect();
        out.push_str(&format!("    digits (a₀,...,a_{{{}}}): [{}]\n", depth-1, digits_str.join(", ")));
        let seq: Vec<u64> = (1..=depth).map(|n| val % p.pow(n as u32)).collect();
        out.push_str(&format!("    compatible sequence: {:?}\n\n", seq));
    }
    out.push_str("=== Negative Integers as p-adic Integers ===\n\n");
    for &val in &[1i64, 2, p as i64] {
        out.push_str(&format!("  −{} in ℤ_{}:\n", val, p));
        let mut digits = vec![];
        for n in 1..=(depth as u32) {
            let pn = p.pow(n) as i64;
            let rem = ((-val) % pn + pn) % pn;
            if n == 1 {
                digits.push((rem % p as i64) as u64);
            } else {
                let prev_pn = p.pow(n-1) as i64;
                let prev_rem = ((-val) % prev_pn + prev_pn) % prev_pn;
                digits.push(((rem - prev_rem) / prev_pn) as u64);
            }
        }
        let digits_str: Vec<String> = digits.iter().map(|d| d.to_string()).collect();
        out.push_str(&format!("    digits: [{}]  (the sequence eventually becomes all {}s)\n",
            digits_str.join(", "), p-1));
    }
    out.push_str("\n=== p-adic Arithmetic: 1 + (-1) = 0 ===\n\n");
    out.push_str(&format!("  1 in ℤ_{}: digits (1, 0, 0, ...)\n", p));
    out.push_str(&format!("  −1 in ℤ_{}: digits ({}, {}, {}, ...)\n", p, p-1, p-1, p-1));
    out.push_str("  Sum: add digit by digit with carries:\n");
    out.push_str(&format!("    digit 0: 1 + ({}) = {} = 0 (mod {}) with carry 1\n", p-1, p, p));
    out.push_str(&format!("    digit 1: 0 + ({}) + 1 = {} = 0 (mod {}) with carry 1\n", p-1, p, p));
    out.push_str("    ... (all carry through)\n");
    out.push_str("1 + (−1) = 0 in ℤ_p, as expected!\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "n", 6);
    state_set_int(&mut s, "m", 4);
    state_set_int(&mut s, "p", 2);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "product" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let m = match parse_int(args, 1, "m") { Some(v) => v, None => return "Missing arg m\n".to_string() };
            if n <= 0 || m <= 0 { return "n and m must be positive\n".to_string(); }
            state_set_int(state, "n", n);
            state_set_int(state, "m", m);
            cmd_product(n, m)
        }
        "coproduct" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let m = match parse_int(args, 1, "m") { Some(v) => v, None => return "Missing arg m\n".to_string() };
            if n <= 0 || m <= 0 { return "n and m must be positive\n".to_string(); }
            state_set_int(state, "n", n);
            state_set_int(state, "m", m);
            cmd_coproduct(n, m)
        }
        "equalizer" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let a = match parse_int(args, 1, "a") { Some(v) => v, None => return "Missing arg a\n".to_string() };
            let b = match parse_int(args, 2, "b") { Some(v) => v, None => return "Missing arg b\n".to_string() };
            if n <= 0 { return "n must be positive\n".to_string(); }
            state_set_int(state, "n", n);
            state_set_int(state, "a", a);
            state_set_int(state, "b", b);
            cmd_equalizer(n, a, b)
        }
        "pullback" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let m = match parse_int(args, 1, "m") { Some(v) => v, None => return "Missing arg m\n".to_string() };
            let k = match parse_int(args, 2, "k") { Some(v) => v, None => return "Missing arg k\n".to_string() };
            if n <= 0 || m <= 0 || k <= 0 { return "n, m, k must be positive\n".to_string(); }
            state_set_int(state, "n", n);
            state_set_int(state, "m", m);
            state_set_int(state, "k", k);
            cmd_pullback(n, m, k)
        }
        "inverse_limit" => {
            let p = match parse_uint(args, 0, "p") { Some(v) => v, None => return "Missing arg p\n".to_string() };
            state_set_int(state, "p", p as i64);
            cmd_inverse_limit(p)
        }
        "pushout" => {
            let n = match parse_int(args, 0, "n") { Some(v) => v, None => return "Missing arg n\n".to_string() };
            let m = match parse_int(args, 1, "m") { Some(v) => v, None => return "Missing arg m\n".to_string() };
            let k = match parse_int(args, 2, "k") { Some(v) => v, None => return "Missing arg k\n".to_string() };
            if n <= 0 || m <= 0 || k <= 0 { return "n, m, k must be positive\n".to_string(); }
            state_set_int(state, "n", n);
            state_set_int(state, "m", m);
            state_set_int(state, "k", k);
            cmd_pushout(n, m, k)
        }
        "padic" => {
            let p = match parse_uint(args, 0, "p") { Some(v) => v, None => return "Missing arg p\n".to_string() };
            let depth = match parse_uint(args, 1, "depth") { Some(v) => v, None => return "Missing arg depth\n".to_string() };
            state_set_int(state, "p", p as i64);
            state_set_int(state, "depth", depth as i64);
            cmd_padic(p, depth)
        }
        "demo" => {
            let mut out = String::new();
            out.push_str(&cmd_product(6, 4));
            out.push_str("\n");
            out.push_str(&cmd_equalizer(12, 3, 5));
            out.push_str("\n");
            out.push_str(&cmd_inverse_limit(2));
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
    let m = state_get_int(state, "m").unwrap_or(4);
    // Limit/colimit cone diagram
    c.text_bold(350.0, 30.0, "Limit Cone Diagram", 14.0, "#222", "middle");
    // Diagram shape: A -f-> K <-g- B  (the base span)
    c.node_circle(180.0, 300.0, &format!("Z/{}Z", n), "#eef", 30.0, 11.0);
    c.node_circle(520.0, 300.0, &format!("Z/{}Z", m), "#eef", 30.0, 11.0);
    c.node_circle(350.0, 420.0, &format!("Z/{}Z", gcd(n,m)), "#dfd", 32.0, 11.0);
    // Apex = limit
    c.node_circle(350.0, 130.0, "Lim (P)", "#ffd", 35.0, 11.0);
    // Cone arrows from apex to base
    c.arrow(322.0, 158.0, 197.0, 272.0, "#226", 1.8);
    c.text(238.0, 200.0, "\u{03c0}\u{2081}", 12.0, "#226", "middle");
    c.arrow(378.0, 158.0, 503.0, 272.0, "#226", 1.8);
    c.text(462.0, 200.0, "\u{03c0}\u{2082}", 12.0, "#226", "middle");
    // Base diagram arrows
    c.arrow(208.0, 316.0, 317.0, 406.0, "#444", 1.5);
    c.text(240.0, 375.0, "f", 12.0, "#333", "middle");
    c.arrow(492.0, 316.0, 383.0, 406.0, "#444", 1.5);
    c.text(460.0, 375.0, "g", 12.0, "#333", "middle");
    // Annotation
    c.text(350.0, 468.0, &format!("Pullback: Z/{}Z \u{d7}_{{Z/{}Z}} Z/{}Z", n, gcd(n,m), m), 11.0, "#444", "middle");
    c.text(350.0, 485.0, &format!("Colimit (pushout): Z/lcm({},{})Z = Z/{}Z", n, m, lcm(n,m)), 11.0, "#444", "middle");
}

fn visualize_dot(g_dot: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    let m = state_get_int(state, "m").unwrap_or(4);
    let k = gcd(n, m);
    g_dot.node("A",   &[("label", &format!("Z/{}Z", n)), ("shape", "circle")]);
    g_dot.node("B",   &[("label", &format!("Z/{}Z", m)), ("shape", "circle")]);
    g_dot.node("K",   &[("label", &format!("Z/{}Z", k)), ("shape", "circle")]);
    g_dot.node("Lim", &[("label", "Limit P"), ("shape", "diamond")]);
    g_dot.edge("A",   "K",   &[("label", "f")]);
    g_dot.edge("B",   "K",   &[("label", "g")]);
    g_dot.edge("Lim", "A",   &[("label", "pi1")]);
    g_dot.edge("Lim", "B",   &[("label", "pi2")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    let m = state_get_int(state, "m").unwrap_or(4);
    t.node("A",   -3.0, 0.0, &format!("$\\mathbb{{Z}}/{}\\mathbb{{Z}}$", n), "draw");
    t.node("B",    3.0, 0.0, &format!("$\\mathbb{{Z}}/{}\\mathbb{{Z}}$", m), "draw");
    t.node("K",    0.0, -2.0, &format!("$\\mathbb{{Z}}/{}\\mathbb{{Z}}$", gcd(n,m)), "draw");
    t.node("Lim",  0.0,  2.0, "Lim", "draw,diamond");
    t.arrow("A",   "K",   "$f$", "->");
    t.arrow("B",   "K",   "$g$", "->");
    t.arrow("Lim", "A",   "$\\pi_1$", "->");
    t.arrow("Lim", "B",   "$\\pi_2$", "->");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    let m = state_get_int(state, "m").unwrap_or(4);
    let k = gcd(n, m);
    a.text_at(2, 1,  "Limit/Colimit Cone Diagram");
    a.text_at(28, 3, "Limit (P)");
    a.text_at(20, 4, "/           \\");
    a.text_at(12, 5, &format!("Z/{}Z         Z/{}Z", n, m));
    a.text_at(17, 6, "\\           /");
    a.text_at(22, 7, &format!("Z/{}Z (base)", k));
    a.text_at(2, 9,  &format!("Colimit: Z/lcm({},{})Z = Z/{}Z", n, m, lcm(n, m)));
    a.text_at(2, 10, "Pullback = limit of the span diagram.");
    a.text_at(2, 11, "Pushout  = colimit of the cospan diagram.");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch36"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 36", "Limits and Colimits", "Universal constructions: the categorical glue");
            print_note("Explore products, coproducts, equalizers, pullbacks, inverse limits, and more.");
            println!("{}", show_help());
            repl("limits> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
