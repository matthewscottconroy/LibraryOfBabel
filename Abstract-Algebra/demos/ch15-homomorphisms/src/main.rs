use common::*;

fn show_help() -> String {
    let mut h = String::new();
    h.push_str("\n  \x1b[1mCommands:\x1b[0m\n");
    h.push_str("    homomorphism <n> <m> <a>    map x↦ax: ℤ/nℤ → ℤ/mℤ; verify homomorphism\n");
    h.push_str("    kernel <n> <m> <a>          kernel of x↦ax mod m from ℤ/nℤ\n");
    h.push_str("    image <n> <m> <a>           image of x↦ax mod m from ℤ/nℤ\n");
    h.push_str("    first_iso <n> <a>           first isomorphism theorem for x↦ax on ℤ/nℤ\n");
    h.push_str("    sign <n>                    sign homomorphism Sₙ → {±1}; kernel = Aₙ\n");
    h.push_str("    semidirect <n>              D_n as Z/nZ ⋊ Z/2Z semidirect product\n");
    h.push_str("    automorphisms <n>           Aut(ℤ/nℤ) ≅ (ℤ/nℤ)* (list automorphisms)\n");
    h.push_str("    demo                        run a showcase of homomorphisms\n");
    h.push_str("    help                        show this help\n");
    h.push_str("    quit                        exit\n");
    h
}

fn map_fn(x: i64, a: i64, m: i64) -> i64 {
    (a * x).rem_euclid(m)
}

fn cmd_homomorphism(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 3 {
        out.push_str(&format!("  {} Usage: homomorphism <n> <m> <a>\n", red("✗")));
        return out;
    }
    let n = match args[0].parse::<i64>().ok().filter(|&v| v >= 2) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be ≥ 2.\n", red("✗"))); return out; }
    };
    let m = match args[1].parse::<i64>().ok().filter(|&v| v >= 1) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} m must be ≥ 1.\n", red("✗"))); return out; }
    };
    let a = match args[2].parse::<i64>().ok() {
        Some(v) => v,
        None => { out.push_str(&format!("  {} a must be an integer.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Homomorphism φ: ℤ/{}ℤ → ℤ/{}ℤ,  φ(x) = {}x mod {}\x1b[0m\n", n, m, a, m));
    out.push_str(&format!("  {}Mapping table:{}\n", bold(""), ""));
    out.push_str(&format!("  {:>6}  {:>12}\n", bold("x"), bold(&format!("{}x mod {}", a, m))));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    for x in 0..n {
        let fx = map_fn(x, a, m);
        out.push_str(&format!("  {:>6}  {:>12}\n", x, fx));
    }
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut is_homo = true;
    let mut counter: Option<(i64, i64)> = None;
    'outer: for x in 0..n {
        for y in 0..n {
            let lhs = map_fn((x + y) % n, a, m);
            let rhs = (map_fn(x, a, m) + map_fn(y, a, m)).rem_euclid(m);
            if lhs != rhs {
                is_homo = false;
                counter = Some((x, y));
                break 'outer;
            }
        }
    }

    out.push_str(&format!("  {}Homomorphism check:{} φ(x+y) = φ(x)+φ(y)?\n", bold(""), ""));
    if is_homo {
        out.push_str(&format!("  {} φ IS a homomorphism ℤ/{}ℤ → ℤ/{}ℤ.\n", green("✓"), n, m));
    } else if let Some((x, y)) = counter {
        out.push_str(&format!("  {} φ is NOT a homomorphism.\n", red("✗")));
        out.push_str(&format!("  Counterexample: x={}, y={}\n", x, y));
    }

    out.push_str(&format!("  {} φ(x) = ax is always well-defined; it is a homo iff φ(n) = an ≡ 0 (mod m).\n", yellow("◆")));
    out
}

fn cmd_kernel(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 3 {
        out.push_str(&format!("  {} Usage: kernel <n> <m> <a>\n", red("✗")));
        return out;
    }
    let n = match args[0].parse::<i64>().ok().filter(|&v| v >= 2) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be ≥ 2.\n", red("✗"))); return out; }
    };
    let m = match args[1].parse::<i64>().ok().filter(|&v| v >= 1) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} m must be ≥ 1.\n", red("✗"))); return out; }
    };
    let a = match args[2].parse::<i64>().ok() {
        Some(v) => v,
        None => { out.push_str(&format!("  {} a must be an integer.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Kernel of φ(x) = {}x: ℤ/{}ℤ → ℤ/{}ℤ\x1b[0m\n", a, n, m));

    let ker: Vec<i64> = (0..n).filter(|&x| map_fn(x, a, m) == 0).collect();
    let ker_str: Vec<String> = ker.iter().map(|x| x.to_string()).collect();
    out.push_str(&format!("  ker(φ) = {{ {} }}  (order {})\n", ker_str.join(", "), ker.len()));

    if !ker.is_empty() {
        out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
        for &x in &ker {
            out.push_str(&format!("    {}·{} = {} ≡ {} (mod {})\n", a, x, a * x, (a * x).rem_euclid(m), m));
        }
    }

    if n % ker.len() as i64 == 0 {
        out.push_str(&format!("  {} {} {}\n", cyan("ker order"), dim("="), green(&format!("{}", ker.len()))));
        out.push_str(&format!("  {} {} {}\n", cyan("[G : ker(φ)]"), dim("="), green(&format!("{}", n / ker.len() as i64))));
        out.push_str(&format!("  {} The kernel is a normal subgroup (since ℤ/nℤ is abelian).\n", yellow("◆")));
        out.push_str(&format!("  {} By the first isomorphism theorem: ℤ/nℤ / ker ≅ im(φ).\n", yellow("◆")));
    }
    out
}

fn cmd_image(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 3 {
        out.push_str(&format!("  {} Usage: image <n> <m> <a>\n", red("✗")));
        return out;
    }
    let n = match args[0].parse::<i64>().ok().filter(|&v| v >= 2) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be ≥ 2.\n", red("✗"))); return out; }
    };
    let m = match args[1].parse::<i64>().ok().filter(|&v| v >= 1) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} m must be ≥ 1.\n", red("✗"))); return out; }
    };
    let a = match args[2].parse::<i64>().ok() {
        Some(v) => v,
        None => { out.push_str(&format!("  {} a must be an integer.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Image of φ(x) = {}x: ℤ/{}ℤ → ℤ/{}ℤ\x1b[0m\n", a, n, m));

    let mut im: Vec<i64> = (0..n).map(|x| map_fn(x, a, m)).collect();
    im.sort(); im.dedup();
    let im_str: Vec<String> = im.iter().map(|x| x.to_string()).collect();

    out.push_str(&format!("  im(φ) = {{ {} }}  (order {})\n", im_str.join(", "), im.len()));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let g = gcd(a.abs(), m);
    let gen = g.rem_euclid(m);
    out.push_str(&format!("  im(φ) = ⟨{}⟩ in ℤ/{}ℤ = ⟨gcd({},{})⟩ = ⟨{}⟩\n", a.rem_euclid(m), m, a.abs(), m, gen));
    out.push_str(&format!("  {} {} {}\n", cyan("gcd(a, m)"), dim("="), green(&format!("{}", g))));
    out.push_str(&format!("  {} {} {}\n", cyan("|im(φ)|"), dim("="), green(&format!("{}", m / g))));
    out.push_str(&format!("  {} The image of x↦ax mod m is the subgroup ⟨gcd(a,m)⟩ in ℤ/mℤ.\n", yellow("◆")));
    out
}

fn cmd_first_iso(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 2 {
        out.push_str(&format!("  {} Usage: first_iso <n> <a>\n", red("✗")));
        return out;
    }
    let n = match args[0].parse::<i64>().ok().filter(|&v| v >= 2) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be ≥ 2.\n", red("✗"))); return out; }
    };
    let a = match args[1].parse::<i64>().ok() {
        Some(v) => v,
        None => { out.push_str(&format!("  {} a must be an integer.\n", red("✗"))); return out; }
    };

    let m = n;
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ First Isomorphism Theorem: φ(x) = {}x on ℤ/{}ℤ\x1b[0m\n", a, n));
    out.push_str(&format!("  φ: ℤ/{}ℤ → ℤ/{}ℤ,  φ(x) = {}x mod {}\n", n, n, a, n));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let ker: Vec<i64> = (0..n).filter(|&x| map_fn(x, a, m) == 0).collect();
    let mut im: Vec<i64> = (0..n).map(|x| map_fn(x, a, m)).collect();
    im.sort(); im.dedup();

    let ker_str: Vec<String> = ker.iter().map(|x| x.to_string()).collect();
    let im_str: Vec<String> = im.iter().map(|x| x.to_string()).collect();

    out.push_str(&format!("  ker(φ) = {{ {} }}  order {}\n", ker_str.join(", "), ker.len()));
    out.push_str(&format!("  im(φ)  = {{ {} }}  order {}\n", im_str.join(", "), im.len()));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  {}First Isomorphism Theorem:{} ℤ/{}ℤ / ker(φ) ≅ im(φ)\n", bold(""), "", n));

    let k_size = ker.len() as i64;
    let q_size = n / k_size;

    out.push_str(&format!("  Quotient G/ker has order |G|/|ker| = {}/{} = {}\n", n, k_size, q_size));
    out.push_str(&format!("  im(φ) has order {}\n", im.len()));

    if q_size == im.len() as i64 {
        out.push_str(&format!("  {} Orders match: |G/ker| = |im| = {}  ✓\n", green("✓"), q_size));
    }

    out.push_str(&format!("\n  {}Explicit isomorphism [x] ↦ φ(x) = {}x:{}\n", bold(""), a, ""));

    let mut seen_cosets: Vec<Vec<i64>> = Vec::new();
    let mut reps: Vec<i64> = Vec::new();
    for x in 0..n {
        let c: Vec<i64> = {
            let mut v: Vec<i64> = ker.iter().map(|&h| (x + h).rem_euclid(n)).collect();
            v.sort();
            v
        };
        if !seen_cosets.contains(&c) {
            seen_cosets.push(c);
            reps.push(x);
        }
    }

    for rep in &reps {
        let img = map_fn(*rep, a, m);
        out.push_str(&format!("    [{}] + ker  ↦  φ({}) = {}\n", rep, rep, img));
    }

    let mut img_vals: Vec<i64> = reps.iter().map(|&x| map_fn(x, a, m)).collect();
    img_vals.sort(); img_vals.dedup();
    if img_vals.len() == reps.len() {
        out.push_str(&format!("  {} Map [x] ↦ φ(x) is injective  ✓\n", green("✓")));
    }

    out.push_str(&format!("  {} First Isomorphism Theorem: for φ: G → H, G/ker(φ) ≅ im(φ).\n", yellow("◆")));
    out
}

fn cmd_sign(args: &[&str]) -> String {
    let mut out = String::new();
    let n_str = args.first().copied().unwrap_or("3");
    let n = match n_str.parse::<usize>().ok().filter(|&v| v >= 2 && v <= 5) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be between 2 and 5.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Sign Homomorphism sgn: S_{} → {{+1, −1}}\x1b[0m\n", n));
    out.push_str(&format!("  {}Definition:{} sgn(σ) = (−1)^(number of inversions)\n", bold(""), ""));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut perms: Vec<Vec<usize>> = Vec::new();
    let base: Vec<usize> = (0..n).collect();
    enumerate_perms(&base, &mut perms);

    let sign_of = |perm: &[usize]| -> i64 {
        let mut inv = 0i64;
        for i in 0..n {
            for j in i + 1..n {
                if perm[i] > perm[j] { inv += 1; }
            }
        }
        if inv % 2 == 0 { 1 } else { -1 }
    };

    out.push_str(&format!("  {:>20}  {:>10}  {:>6}\n", bold("Permutation"), bold("Inversions"), bold("Sign")));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut even_count = 0usize;
    let mut odd_count = 0usize;

    for perm in &perms {
        let inv_count: usize = (0..n).flat_map(|i| (i+1..n).filter(move |&j| perm[i] > perm[j])).count();
        let s = sign_of(perm);
        let perm_str = format!("[{}]", perm.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""));
        out.push_str(&format!("  {:>20}  {:>10}  {:>6}\n",
            perm_str, inv_count, if s == 1 { "+1" } else { "−1" }));
        if s == 1 { even_count += 1; } else { odd_count += 1; }
    }

    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  Even permutations (ker = A_{}): {} elements\n", n, even_count));
    out.push_str(&format!("  Odd permutations: {} elements\n", odd_count));

    out.push_str(&format!("  {} ker(sgn) = A_{} (alternating group) has order {}! / 2 = {}.\n",
        yellow("◆"), n, n, (1..=n).product::<usize>() / 2));
    out
}

fn enumerate_perms(items: &[usize], out: &mut Vec<Vec<usize>>) {
    let mut perm = items.to_vec();
    perm.sort();
    loop {
        out.push(perm.clone());
        let n = perm.len();
        let mut i = n;
        loop {
            if i <= 1 { return; }
            i -= 1;
            if perm[i - 1] < perm[i] { break; }
        }
        let mut j = n - 1;
        while perm[j] <= perm[i - 1] { j -= 1; }
        perm.swap(i - 1, j);
        perm[i..].reverse();
    }
}

fn cmd_semidirect(args: &[&str]) -> String {
    let mut out = String::new();
    let n_str = args.first().copied().unwrap_or("3");
    let n = match n_str.parse::<usize>().ok().filter(|&v| v >= 2 && v <= 6) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be between 2 and 6.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ D_{} as ℤ/{}ℤ ⋊ ℤ/2ℤ (Semidirect Product)\x1b[0m\n", n, n));
    out.push_str(&format!("  {}Recall:{} D_n presentation: ⟨r, s | rⁿ=e, s²=e, srs⁻¹=r⁻¹⟩\n", bold(""), ""));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  Normal subgroup N = ⟨r⟩ ≅ ℤ/{}ℤ  (rotations)\n", n));
    out.push_str("  Complement K = ⟨s⟩ ≅ ℤ/2ℤ  (a reflection)\n");
    out.push_str("  Action: φ(1)(r^k) = r^(-k)\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let label = |r: usize, s: usize| -> String {
        if s == 0 {
            if r == 0 { "e".to_string() } else { format!("r^{}", r) }
        } else {
            if r == 0 { "s".to_string() } else { format!("sr^{}", r) }
        }
    };

    out.push_str(&format!("  {}Multiplication table sample:{}\n", bold(""), ""));
    let examples = vec![(0,0,1,0),(0,0,0,1),(1,0,1,0),(0,1,0,1),(1,1,1,0),(0,1,1,0)];
    for (ra, sa, rb, sb) in examples {
        let new_b = if sa == 1 { (n - rb) % n } else { rb };
        let rc = (ra + new_b) % n;
        let sc = (sa + sb) % 2;
        out.push_str(&format!("    {} · {} = {}\n", label(ra, sa), label(rb, sb), label(rc, sc)));
    }
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str("  1. N = ⟨r⟩ is normal in D_n (index 2).\n");
    out.push_str("  2. K = ⟨s⟩ is a complement: N ∩ K = {e}, NK = D_n.\n");
    out.push_str("  3. K acts on N by conjugation: s·r^k·s⁻¹ = r^(−k).\n");
    out.push_str(&format!("  {} D_{} is the simplest non-trivial semidirect product.\n", yellow("◆"), n));
    out
}

fn cmd_automorphisms(args: &[&str]) -> String {
    let mut out = String::new();
    let n_str = args.first().copied().unwrap_or("8");
    let n = match n_str.parse::<usize>().ok().filter(|&v| v >= 2 && v <= 30) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be between 2 and 30.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Aut(ℤ/{}ℤ) ≅ (ℤ/{}ℤ)*\x1b[0m\n", n, n));
    out.push_str(&format!("  {}Theorem:{} Aut(ℤ/nℤ) ≅ (ℤ/nℤ)* via a ↦ φ_a where φ_a(x) = ax.\n", bold(""), ""));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let units: Vec<usize> = (1..n).filter(|&a| gcd(a as i64, n as i64) == 1).collect();
    out.push_str(&format!("  |Aut(ℤ/{}ℤ)| = ϕ({}) = {}\n", n, n, units.len()));
    out.push('\n');
    out.push_str(&format!("  {}Automorphisms (φ_a : x ↦ ax):{}\n", bold(""), ""));
    out.push_str(&format!("  {:>6}  {:>8}  {:>20}\n", bold("a"), bold("gcd(a,n)"), bold("φ_a(0..4...)")));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    for &a in &units {
        let values: Vec<String> = (0..n.min(5)).map(|x| format!("{}", (a * x) % n)).collect();
        out.push_str(&format!("  {:>6}  {:>8}  {:>20}\n",
            a, gcd(a as i64, n as i64),
            format!("{}...", values.join(","))));
    }

    out.push_str(&format!("  {} |(ℤ/{}ℤ)*| = ϕ({}) = {} (Euler's totient function).\n", yellow("◆"), n, n, units.len()));
    out.push_str(&format!("  {} For prime p: Aut(ℤ/pℤ) ≅ ℤ/(p-1)ℤ.\n", yellow("◆")));
    out
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "homomorphism" => {
            if let (Some(&n_s), Some(&m_s), Some(&a_s)) = (args.first(), args.get(1), args.get(2)) {
                if let (Ok(n), Ok(m), Ok(a)) = (n_s.parse::<i64>(), m_s.parse::<i64>(), a_s.parse::<i64>()) {
                    state_set_int(state, "n", n);
                    state_set_int(state, "m", m);
                    state_set_int(state, "a", a);
                }
            }
            cmd_homomorphism(args)
        }
        "kernel" => {
            if let (Some(&n_s), Some(&m_s), Some(&a_s)) = (args.first(), args.get(1), args.get(2)) {
                if let (Ok(n), Ok(m), Ok(a)) = (n_s.parse::<i64>(), m_s.parse::<i64>(), a_s.parse::<i64>()) {
                    state_set_int(state, "n", n);
                    state_set_int(state, "m", m);
                    state_set_int(state, "a", a);
                }
            }
            cmd_kernel(args)
        }
        "image" => {
            if let (Some(&n_s), Some(&m_s), Some(&a_s)) = (args.first(), args.get(1), args.get(2)) {
                if let (Ok(n), Ok(m), Ok(a)) = (n_s.parse::<i64>(), m_s.parse::<i64>(), a_s.parse::<i64>()) {
                    state_set_int(state, "n", n);
                    state_set_int(state, "m", m);
                    state_set_int(state, "a", a);
                }
            }
            cmd_image(args)
        }
        "first_iso" => {
            if let (Some(&n_s), Some(&a_s)) = (args.first(), args.get(1)) {
                if let (Ok(n), Ok(a)) = (n_s.parse::<i64>(), a_s.parse::<i64>()) {
                    state_set_int(state, "n", n);
                    state_set_int(state, "a", a);
                }
            }
            cmd_first_iso(args)
        }
        "sign" => {
            if let Some(&n_s) = args.first() {
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "n", n);
                }
            }
            cmd_sign(args)
        }
        "semidirect" => {
            if let Some(&n_s) = args.first() {
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "n", n);
                }
            }
            cmd_semidirect(args)
        }
        "automorphisms" => {
            if let Some(&n_s) = args.first() {
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "n", n);
                }
            }
            cmd_automorphisms(args)
        }
        "demo" => {
            let mut out = String::new();
            out.push_str("\n  === Demo: Homomorphisms ===\n\n");
            let n = 6i64; let m = 3i64; let a = 2i64;
            out.push_str(&format!("  φ: Z/{}Z → Z/{}Z,  φ(x) = {}x mod {}\n", n, m, a, m));
            let ker: Vec<i64> = (0..n).filter(|&x| map_fn(x, a, m) == 0).collect();
            let mut im: Vec<i64> = (0..n).map(|x| map_fn(x, a, m)).collect();
            im.sort(); im.dedup();
            out.push_str(&format!("  ker(φ) = {:?}  (order {})\n", ker, ker.len()));
            out.push_str(&format!("  im(φ) = {:?}  (order {})\n", im, im.len()));
            out.push_str(&format!("  G/ker ≅ im:  {}/{} = {}  ✓\n", n, ker.len(), im.len() as i64));
            out
        }
        "help" | "h" => show_help(),
        _ => format!("  \x1b[31m✗\x1b[0m Unknown command '{}'. Type 'help'.\n", cmd),
    }
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { print!("{out}"); }
    true
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "n", 6);
    state_set_int(&mut s, "m", 3);
    state_set_int(&mut s, "a", 2);
    s
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    let m = state_get_int(state, "m").unwrap_or(3);
    let a = state_get_int(state, "a").unwrap_or(2);

    c.title(&format!("Ch15: Homomorphism φ(x)={}x: Z/{}Z → Z/{}Z", a, n, m));
    c.subtitle("Kernel highlighted; image shown in codomain", 42.0);

    let ker: Vec<i64> = (0..n).filter(|&x| map_fn(x, a, m) == 0).collect();
    let mut im: Vec<i64> = (0..n).map(|x| map_fn(x, a, m)).collect();
    im.sort(); im.dedup();

    let lx = 100.0; let rx = 500.0;
    let cy_start = 90.0; let spacing = 45.0;

    c.text_bold(lx, 65.0, &format!("Z/{}Z (domain)", n), 13.0, colors::DARK, "middle");
    c.text_bold(rx, 65.0, &format!("Z/{}Z (codomain)", m), 13.0, colors::DARK, "middle");

    for x in 0..n {
        let cy = cy_start + x as f64 * spacing;
        let in_ker = ker.contains(&x);
        let fill = if in_ker { colors::RED } else { colors::LIGHT };
        let stroke = if in_ker { colors::RED } else { colors::CYAN };
        c.circle(lx, cy, 18.0, fill, stroke, 1.5);
        c.text(lx, cy, &x.to_string(), 12.0, if in_ker { colors::LIGHT } else { colors::DARK }, "middle");
    }

    for y in 0..m {
        let cy = cy_start + (n as f64 / 2.0 - m as f64 / 2.0 + y as f64) * spacing;
        let in_im = im.contains(&y);
        let fill = if in_im { colors::GREEN } else { colors::LIGHT };
        let stroke = if in_im { colors::GREEN } else { colors::GREY };
        c.circle(rx, cy, 18.0, fill, stroke, 1.5);
        c.text(rx, cy, &y.to_string(), 12.0, if in_im { colors::LIGHT } else { colors::DARK }, "middle");
    }

    for x in 0..n {
        let cy_src = cy_start + x as f64 * spacing;
        let fx = map_fn(x, a, m);
        let cy_dst = cy_start + (n as f64 / 2.0 - m as f64 / 2.0 + fx as f64) * spacing;
        let stroke = if ker.contains(&x) { colors::RED } else { colors::BLUE };
        c.arrow(lx + 18.0, cy_src, rx - 18.0, cy_dst, stroke, 1.0);
    }

    let ly = cy_start + n as f64 * spacing + 20.0;
    c.circle(lx - 30.0, ly, 8.0, colors::RED, colors::RED, 1.0);
    c.text(lx - 18.0, ly, "ker(φ)", 11.0, colors::RED, "start");
    c.circle(rx - 30.0, ly, 8.0, colors::GREEN, colors::GREEN, 1.0);
    c.text(rx - 18.0, ly, "im(φ)", 11.0, colors::GREEN, "start");
    c.text(300.0, ly + 20.0, "First Iso: G/ker ≅ im", 12.0, colors::DARK, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    let m = state_get_int(state, "m").unwrap_or(3);
    let a = state_get_int(state, "a").unwrap_or(2);
    let ker: Vec<i64> = (0..n).filter(|&x| map_fn(x, a, m) == 0).collect();
    let mut im: Vec<i64> = (0..n).map(|x| map_fn(x, a, m)).collect();
    im.sort(); im.dedup();

    g.node_default("shape", "circle");
    g.node("G", &[("label", &format!("Z/{}Z", n)), ("style", "filled"), ("fillcolor", "lightblue")]);
    g.node("H", &[("label", &format!("Z/{}Z", m)), ("style", "filled"), ("fillcolor", "lightgreen")]);
    let ker_str: Vec<String> = ker.iter().map(|x| x.to_string()).collect();
    let im_str: Vec<String> = im.iter().map(|x| x.to_string()).collect();
    g.node("K", &[("label", &format!("ker={{{}}}", ker_str.join(","))), ("fillcolor", "lightyellow")]);
    g.node("I", &[("label", &format!("im={{{}}}", im_str.join(","))), ("fillcolor", "lightyellow")]);
    g.node("Q", &[("label", &format!("G/ker (iso to im)"))]);
    g.edge("G", "H", &[("label", &format!("phi(x)={}x", a))]);
    g.edge("G", "K", &[("label", "kernel")]);
    g.edge("H", "I", &[("label", "image")]);
    g.edge("Q", "I", &[("label", "1st iso thm"), ("style", "dashed")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    let m = state_get_int(state, "m").unwrap_or(3);
    let a = state_get_int(state, "a").unwrap_or(2);
    t.use_library("arrows,positioning");
    t.raw("  \\tikzset{grp/.style={draw,ellipse,minimum width=1.8cm,minimum height=0.8cm}}");
    t.node_math("G", -3.0, 0.0, &format!("G=\\mathbb{{Z}}/{n}\\mathbb{{Z}}"), "grp");
    t.node_math("H", 3.0, 0.0, &format!("H=\\mathbb{{Z}}/{m}\\mathbb{{Z}}"), "grp");
    t.node_math("K", -3.0, -1.8, "\\ker(\\varphi)", "grp");
    t.node_math("I", 3.0, -1.8, "\\mathrm{im}(\\varphi)", "grp");
    t.node_math("Q", 0.0, -1.8, "G/\\ker", "grp");
    t.arrow("G", "H", &format!("\\varphi(x)={}x", a), "->");
    t.arrow("G", "K", "\\ker", "->");
    t.arrow("H", "I", "\\mathrm{im}", "->");
    t.arrow("Q", "I", "\\cong", "dashed,->");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "n").unwrap_or(6);
    let m = state_get_int(state, "m").unwrap_or(3);
    let av = state_get_int(state, "a").unwrap_or(2);

    let ker: Vec<i64> = (0..n).filter(|&x| map_fn(x, av, m) == 0).collect();
    let mut im: Vec<i64> = (0..n).map(|x| map_fn(x, av, m)).collect();
    im.sort(); im.dedup();

    a.text_at(2, 1, &format!("Ch15: Homomorphism phi(x)={}x: Z/{}Z -> Z/{}Z", av, n, m));
    a.text_at(2, 2, "──────────────────────────────────────────────");
    a.text_at(2, 3, &format!("  ker(phi) = {:?}", ker));
    a.text_at(2, 4, &format!("  im(phi)  = {:?}", im));
    a.text_at(2, 5, "──────────────────────────────────────────────");
    a.text_at(2, 6, "  x  phi(x)  in_ker?");
    a.text_at(2, 7, "  ─────────────────");
    for x in 0..n.min(10) {
        let fx = map_fn(x, av, m);
        let in_ker = ker.contains(&x);
        a.text_at(2, 8 + x as i32, &format!("  {}  {}      {}", x, fx, if in_ker { "<ker" } else { "" }));
    }
    let row = 9 + n as i32;
    a.text_at(2, row, "──────────────────────────────────────────────");
    a.text_at(2, row + 1, "First Isomorphism Theorem: G/ker(phi) ~= im(phi)");
    let im_str2: Vec<String> = im.iter().map(|x| x.to_string()).collect();
    a.text_at(2, row + 2, &format!("  |G/ker| = {}/{} = {}  =  |im| = {}",
        n, ker.len(), n as usize / ker.len(), im_str2.len()));
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
                OutputFormat::Svg => {
                    let mut c = SvgCanvas::new(700.0, 500.0);
                    visualize_svg(&mut c, cmd, &args_ref, &state);
                    c.build()
                }
                OutputFormat::Dot => {
                    let mut g = DotGraph::digraph("ch15");
                    visualize_dot(&mut g, cmd, &args_ref, &state);
                    g.build()
                }
                OutputFormat::Tex => {
                    let mut t = TikzDoc::standalone();
                    visualize_tex(&mut t, cmd, &args_ref, &state);
                    t.build()
                }
                OutputFormat::Ascii => {
                    let mut a = AsciiCanvas::new(80, 30);
                    visualize_ascii(&mut a, cmd, &args_ref, &state);
                    a.render()
                }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 15", "Homomorphisms and the Isomorphism Theorems",
                "Kernels · Images · First Isomorphism Theorem · Automorphisms");
            print_info("Explore structure-preserving maps between groups.");
            print!("{}", show_help());
            repl("ch15> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
