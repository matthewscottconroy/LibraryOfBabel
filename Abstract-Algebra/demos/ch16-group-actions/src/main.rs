use common::*;

fn show_help() -> String {
    let mut h = String::new();
    h.push_str("\n  \x1b[1mCommands:\x1b[0m\n");
    h.push_str("    orbit <n> <g> <x>          orbit of x under ⟨g⟩ acting on ℤ/nℤ by addition\n");
    h.push_str("    stabilizer <n> <g> <x>     stabilizer of x under g-action on ℤ/nℤ\n");
    h.push_str("    orbit_stab <n> <g>          verify orbit-stabilizer theorem for all x in ℤ/nℤ\n");
    h.push_str("    burnside <n> <k>            Burnside: colorings of n-cycle with k colors up to rotation\n");
    h.push_str("    class_eq <type> <n>         class equation for group type (Z or D)\n");
    h.push_str("    cayley_action <n>           Cayley's theorem: embed ℤ/nℤ in S_n\n");
    h.push_str("    necklace <n> <k>            count distinct necklaces: n beads, k colors\n");
    h.push_str("    demo                        demonstrate group actions on Z/6Z\n");
    h.push_str("    help                        show this help\n");
    h.push_str("    quit                        exit\n");
    h
}

fn cmd_orbit(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 3 {
        out.push_str(&format!("  {} Usage: orbit <n> <g> <x>\n", red("✗")));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 1) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be >= 1.\n", red("✗"))); return out; }
    };
    let g = match args[1].parse::<usize>().ok() {
        Some(v) => v % n,
        None => { out.push_str(&format!("  {} g must be a non-negative integer.\n", red("✗"))); return out; }
    };
    let x0 = match args[2].parse::<usize>().ok().filter(|&v| v < n) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} x must be in 0..{}.\n", red("✗"), n-1)); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Orbit of {} under <{}> acting on Z/{}Z\x1b[0m\n", x0, g, n));
    out.push_str(&format!("  Action: g*x = (x + g) mod {}  (translation by g)\n", n));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut orbit = Vec::new();
    let mut cur = x0;
    let mut step = 0usize;
    loop {
        orbit.push(cur);
        step += 1;
        cur = (cur + g) % n;
        if cur == x0 { break; }
        if step > n { break; }
    }

    out.push_str(&format!("  Computing orbit of {}:\n", x0));
    for (i, &pt) in orbit.iter().enumerate() {
        out.push_str(&format!("    {}^{} * {} = {}\n", g, i, x0, pt));
    }

    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    let orb_str: Vec<String> = orbit.iter().map(|x| x.to_string()).collect();
    out.push_str(&format!("  Orb({}) = {{ {} }}  (size {})\n", x0, orb_str.join(", "), orbit.len()));

    let g_order = additive_order(g, n);
    out.push_str(&format!("  {} {} {}\n", cyan("ord(g)"), dim("="), green(&format!("{}", g_order))));
    out.push_str(&format!("  {} {} {}\n", cyan("|Orb(x)|"), dim("="), green(&format!("{}", orbit.len()))));

    if orbit.len() == g_order {
        out.push_str(&format!("  {} Orbit size equals ord(g) (as expected for free action).\n", green("✓")));
    }
    out.push_str(&format!("  {} The orbit of x is the set of all points reachable from x by the action.\n", yellow("◆")));
    out.push_str(&format!("  {} Orbits partition the set being acted upon.\n", yellow("◆")));
    out
}

fn cmd_stabilizer(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 3 {
        out.push_str(&format!("  {} Usage: stabilizer <n> <g> <x>\n", red("✗")));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 1) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be >= 1.\n", red("✗"))); return out; }
    };
    let g = match args[1].parse::<usize>().ok() {
        Some(v) => v % n,
        None => { out.push_str(&format!("  {} g must be non-negative.\n", red("✗"))); return out; }
    };
    let x = match args[2].parse::<usize>().ok().filter(|&v| v < n) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} x must be in 0..{}.\n", red("✗"), n-1)); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Stabilizer of {} under g={} action on Z/{}Z\x1b[0m\n", x, g, n));
    out.push_str(&format!("  Stab(x) = {{ k in <g> : k + x = x (mod {}) }}\n", n));
    out.push_str(&format!("          = {{ k in <g> : k = 0 (mod {}) }}\n", n));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut g_sub = Vec::new();
    let mut cur = 0usize;
    loop {
        g_sub.push(cur);
        cur = (cur + g) % n;
        if cur == 0 { break; }
        if g_sub.len() > n { break; }
    }
    g_sub.sort();

    let stab: Vec<usize> = g_sub.iter().filter(|&&k| {
        (x + k) % n == x
    }).cloned().collect();

    let g_str: Vec<String> = g_sub.iter().map(|v| v.to_string()).collect();
    let stab_str: Vec<String> = stab.iter().map(|v| v.to_string()).collect();
    out.push_str(&format!("  <g> = {{ {} }}\n", g_str.join(", ")));
    out.push_str(&format!("  Stab({}) = {{ {} }}  (order {})\n", x, stab_str.join(", "), stab.len()));

    let orb_size = additive_order(g, n);
    let stab_size = stab.len();
    let group_size = g_sub.len();

    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str("  Orbit-Stabilizer: |Orb| * |Stab| = |G|\n");
    out.push_str(&format!("    {} * {} = {}  {}\n", orb_size, stab_size, group_size,
        if orb_size * stab_size == group_size { green("OK") } else { red("FAIL") }));

    out.push_str(&format!("  {} For translation actions, stabilizers are trivial (k*x = x => k = 0).\n", yellow("◆")));
    out.push_str(&format!("  {} Stabilizers are always subgroups of the acting group.\n", yellow("◆")));
    out
}

fn cmd_orbit_stab(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 2 {
        out.push_str(&format!("  {} Usage: orbit_stab <n> <g>\n", red("✗")));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 1) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be >= 1.\n", red("✗"))); return out; }
    };
    let g = match args[1].parse::<usize>().ok() {
        Some(v) => v % n,
        None => { out.push_str(&format!("  {} g must be non-negative.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Orbit-Stabilizer Theorem: g={} on Z/{}Z\x1b[0m\n", g, n));
    out.push_str("  Theorem: |Orb(x)| * |Stab(x)| = |<g>|  for all x\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let g_order = additive_order(g, n);
    out.push_str(&format!("  |<g>| = {}\n\n", g_order));
    out.push_str(&format!("  {:>6}  {:>10}  {:>10}  {:>10}  {:>8}\n",
        bold("x"), bold("|Orb(x)|"), bold("|Stab(x)|"), bold("Product"), bold("Check")));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    for x in 0..n {
        let mut orbit = Vec::new();
        let mut cur = x;
        loop {
            orbit.push(cur);
            cur = (cur + g) % n;
            if cur == x { break; }
            if orbit.len() > n { break; }
        }
        let stab_size = 1usize;
        let prod = orbit.len() * stab_size;
        let ok = prod == g_order;
        out.push_str(&format!("  {:>6}  {:>10}  {:>10}  {:>10}  {:>8}\n",
            x, orbit.len(), stab_size, prod,
            if ok { green("OK") } else { red("FAIL") }));
    }

    out.push_str(&format!("  {} For translation on Z/nZ, every orbit has size ord(g) and trivial stabilizer.\n", yellow("◆")));
    out.push_str(&format!("  {} The orbit-stabilizer theorem is the key to counting via group actions.\n", yellow("◆")));
    out
}

fn cmd_burnside(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 2 {
        out.push_str(&format!("  {} Usage: burnside <n> <k>\n", red("✗")));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 1 && v <= 20) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be between 1 and 20.\n", red("✗"))); return out; }
    };
    let k = match args[1].parse::<usize>().ok().filter(|&v| v >= 1 && v <= 10) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} k must be between 1 and 10.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Burnside's Lemma: Colorings of {}-cycle with {} colors\x1b[0m\n", n, k));
    out.push_str(&format!("  Group: Z/{}Z acting on colorings of {} positions by rotation.\n", n, n));
    out.push_str("  Burnside: |orbits| = (1/|G|) * Sum_g |Fix(g)|\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str("  Fix(rotation by d) = k^gcd(d,n)  (period must divide gcd)\n\n");
    out.push_str(&format!("  {:>8}  {:>12}  {:>12}\n",
        bold("d (rot)"), bold("gcd(d,n)"), bold("|Fix(r^d)|")));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut total_fix = 0u64;
    for d in 0..n {
        let g = gcd(d as i64, n as i64) as u32;
        let fix = (k as u64).pow(g);
        total_fix += fix;
        out.push_str(&format!("  {:>8}  {:>12}  {:>12}\n", d, g, fix));
    }

    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  Sum of |Fix(g)| = {}\n", total_fix));
    out.push_str(&format!("  |Z/{}Z| = {}\n", n, n));
    let result = total_fix / n as u64;
    out.push_str(&format!("  Distinct colorings = {} / {} = {}\n", total_fix, n, result));
    out.push_str(&format!("  {} {} {}\n",
        cyan(&format!("Distinct {}-colorings of {}-cycle", k, n)),
        dim("="),
        green(&format!("{}", result))));
    out.push_str(&format!("  {} Burnside's lemma counts orbits: X/G = (1/|G|) * Sum |X^g|.\n", yellow("◆")));
    out.push_str(&format!("  {} This counts combinatorial objects up to symmetry.\n", yellow("◆")));
    out
}

fn cmd_class_eq(args: &[&str]) -> String {
    let mut out = String::new();
    let gtype = match args.first() {
        Some(&s) => s,
        None => { out.push_str(&format!("  {} Usage: class_eq <Z|D> <n>\n", red("✗"))); return out; }
    };
    let n = match args.get(1).and_then(|s| s.parse::<usize>().ok()).filter(|&v| v >= 2) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be >= 2.\n", red("✗"))); return out; }
    };

    match gtype {
        "Z" => {
            out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Class Equation for Z/{}Z\x1b[0m\n", n));
            out.push_str(&format!("  Z/{}Z is abelian: each element is its own conjugacy class.\n", n));
            out.push_str("  Z(G) = G, so the class equation is: |G| = |Z(G)|\n");
            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
            out.push_str("  Conjugacy classes:\n");
            for x in 0..n {
                out.push_str(&format!("    [{}] = {{{}}}  (size 1)\n", x, x));
            }
            out.push_str(&format!("  {} {} {}\n", cyan("|G|"), dim("="), green(&format!("{}", n))));
            out.push_str(&format!("  {} {} {}\n", cyan("|Z(G)|"), dim("="), green(&format!("{}", n))));
            out.push_str(&format!("  {} All elements central; class equation: |G| = |Z(G)| + 0.\n", green("✓")));
        }
        "D" => {
            out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Class Equation for D_{}\x1b[0m\n", n));
            let ord = 2 * n;
            out.push_str(&format!("  D_{}: order {}\n", n, ord));
            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

            let mut class_sizes = Vec::new();

            out.push_str("  [e] = {e}  (size 1)\n");
            class_sizes.push(1usize);

            if n % 2 == 0 {
                out.push_str(&format!("  [r^{}] = {{r^{}}}  (size 1)\n", n/2, n/2));
                class_sizes.push(1);
            }
            let mut k = 1usize;
            let lim = if n % 2 == 0 { n/2 } else { (n-1)/2 };
            while k <= lim {
                if n % 2 == 0 && k == n/2 { k += 1; continue; }
                out.push_str(&format!("  [r^{}] = {{r^{}, r^{}}}  (size 2)\n", k, k, n - k));
                class_sizes.push(2);
                k += 1;
            }

            if n % 2 == 0 {
                let half = n / 2;
                out.push_str(&format!("  even reflections: {{s, sr^2, ...}}  (size {})\n", half));
                out.push_str(&format!("  odd reflections:  {{sr, sr^3, ...}}  (size {})\n", half));
                class_sizes.push(half);
                class_sizes.push(half);
            } else {
                out.push_str(&format!("  reflections: {{s, sr, sr^2, ...}}  (size {})\n", n));
                class_sizes.push(n);
            }

            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
            let total: usize = class_sizes.iter().sum();
            let sum_str: Vec<String> = class_sizes.iter().map(|x| x.to_string()).collect();
            out.push_str(&format!("  Class equation: |D_{}| = {}\n", n, sum_str.join(" + ")));
            out.push_str(&format!("    {} = {}  {}\n", ord, total,
                if total == ord { green("OK") } else { red("FAIL") }));

            let center_size = if n % 2 == 0 { 2 } else { 1 };
            out.push_str(&format!("  {} {} {}\n", cyan("|Z(D_n)|"), dim("="), green(&format!("{}", center_size))));
            out.push_str(&format!("  {} Center of D_n: {{e}} if n odd; {{e, r^{{n/2}}}} if n even.\n", yellow("◆")));
        }
        _ => { out.push_str(&format!("  {} Type must be Z or D.\n", red("✗"))); return out; }
    }
    out.push_str(&format!("  {} Class equation: |G| = |Z(G)| + Sum_{{non-central classes}} [G:C_G(g)].\n", yellow("◆")));
    out
}

fn cmd_cayley_action(args: &[&str]) -> String {
    let mut out = String::new();
    let n_str = args.first().copied().unwrap_or("4");
    let n = match n_str.parse::<usize>().ok().filter(|&v| v >= 2 && v <= 6) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be between 2 and 6.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Cayley's Theorem: Z/{}Z embeds in S_{}\x1b[0m\n", n, n));
    out.push_str("  Cayley's Theorem: Every group G embeds into S_{|G|}.\n");
    out.push_str("  The embedding: g |--> sigma_g,  where sigma_g(x) = g + x  (left mult).\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  Permutation representation of Z/{}Z:\n", n));
    out.push_str(&format!("  {:>6}  {:>40}\n", bold("g"), bold("sigma_g (permutation)")));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    for g in 0..n {
        let perm: Vec<usize> = (0..n).map(|x| (g + x) % n).collect();
        let perm_str = format!("[{}]",
            perm.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
        out.push_str(&format!("  {:>6}  {:>40}\n", g, perm_str));
    }

    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str("  Verify homomorphism property: sigma_{g+h} = sigma_g o sigma_h?\n");
    let g = 1usize; let h = 2usize;
    let gh = (g + h) % n;
    let sigma_g: Vec<usize> = (0..n).map(|x| (g + x) % n).collect();
    let sigma_h: Vec<usize> = (0..n).map(|x| (h + x) % n).collect();
    let comp: Vec<usize> = (0..n).map(|x| sigma_g[sigma_h[x]]).collect();
    let sigma_gh: Vec<usize> = (0..n).map(|x| (gh + x) % n).collect();

    out.push_str(&format!("    sigma_{} o sigma_{} = [{}]\n",
        g, h, comp.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")));
    out.push_str(&format!("    sigma_{{{}+{}}} = sigma_{} = [{}]\n",
        g, h, gh, sigma_gh.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")));
    if comp == sigma_gh {
        out.push_str(&format!("  {} Homomorphism property confirmed!\n", green("✓")));
    }

    out.push_str(&format!("  {} Cayley's theorem shows every abstract group is a permutation group.\n", yellow("◆")));
    out.push_str(&format!("  {} Z/{}Z embeds as a cyclic subgroup of S_{} generated by sigma_1.\n", yellow("◆"), n, n));
    out
}

fn cmd_necklace(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 2 {
        out.push_str(&format!("  {} Usage: necklace <n> <k>\n", red("✗")));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 1 && v <= 20) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be between 1 and 20.\n", red("✗"))); return out; }
    };
    let k = match args[1].parse::<usize>().ok().filter(|&v| v >= 1 && v <= 10) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} k must be between 1 and 10.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Necklace Counting: {} beads, {} colors\x1b[0m\n", n, k));
    out.push_str("  Necklaces: equivalent under rotation AND reflection.\n");
    out.push_str(&format!("  Acting group: dihedral D_n of order {}.\n", 2 * n));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str("  Burnside: |necklaces| = (1/2n) * Sum_g |Fix(g)|\n\n");

    let mut rotation_sum = 0u64;
    out.push_str("  Rotation contributions (Fix(r^d) = k^gcd(d,n)):\n");
    for d in 0..n {
        let g = gcd(d as i64, n as i64) as u32;
        let fix = (k as u64).pow(g);
        rotation_sum += fix;
        out.push_str(&format!("    d={}: k^gcd({},{}) = {}^{} = {}\n", d, d, n, k, g, fix));
    }

    out.push_str("\n  Reflection contributions:\n");
    let mut reflection_sum = 0u64;

    if n % 2 == 0 {
        let fix1 = (k as u64).pow(n as u32 / 2 + 1);
        let count1 = n / 2;
        let fix2 = (k as u64).pow(n as u32 / 2);
        let count2 = n / 2;
        out.push_str(&format!("    {} through bead-pairs: k^(n/2+1) = {}^{} = {}\n", count1, k, n/2+1, fix1));
        out.push_str(&format!("    {} through edge midpoints: k^(n/2) = {}^{} = {}\n", count2, k, n/2, fix2));
        reflection_sum = count1 as u64 * fix1 + count2 as u64 * fix2;
    } else {
        let fix = (k as u64).pow((n + 1) as u32 / 2);
        out.push_str(&format!("    {} reflections: k^((n+1)/2) = {}^{} = {}\n", n, k, (n+1)/2, fix));
        reflection_sum = n as u64 * fix;
    }

    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    let total = rotation_sum + reflection_sum;
    let group_order = 2 * n as u64;
    let result = total / group_order;

    out.push_str(&format!("  Rotation sum = {}\n", rotation_sum));
    out.push_str(&format!("  Reflection sum = {}\n", reflection_sum));
    out.push_str(&format!("  Total = {}\n", total));
    out.push_str(&format!("  {} {} {}\n",
        cyan(&format!("Distinct {}-color {}-bead necklaces", k, n)),
        dim("="),
        green(&format!("{} / {} = {}", total, group_order, result))));
    out.push_str(&format!("  {} Without reflections: divide only by n (directed necklaces).\n", yellow("◆")));
    out.push_str(&format!("  {} With reflections: dihedral action gives unoriented necklaces.\n", yellow("◆")));
    out
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "orbit" => {
            if let (Some(&n_s), Some(&g_s)) = (args.first(), args.get(1)) {
                if let (Ok(n), Ok(g)) = (n_s.parse::<i64>(), g_s.parse::<i64>()) {
                    state_set_int(state, "group_n", n);
                    state_set_int(state, "gen", g);
                }
            }
            cmd_orbit(args)
        }
        "stabilizer" => {
            if let (Some(&n_s), Some(&g_s)) = (args.first(), args.get(1)) {
                if let (Ok(n), Ok(g)) = (n_s.parse::<i64>(), g_s.parse::<i64>()) {
                    state_set_int(state, "group_n", n);
                    state_set_int(state, "gen", g);
                }
            }
            cmd_stabilizer(args)
        }
        "orbit_stab" => {
            if let (Some(&n_s), Some(&g_s)) = (args.first(), args.get(1)) {
                if let (Ok(n), Ok(g)) = (n_s.parse::<i64>(), g_s.parse::<i64>()) {
                    state_set_int(state, "group_n", n);
                    state_set_int(state, "gen", g);
                }
            }
            cmd_orbit_stab(args)
        }
        "burnside" => {
            if let Some(&n_s) = args.first() {
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "group_n", n);
                }
            }
            cmd_burnside(args)
        }
        "class_eq" => {
            if let Some(&n_s) = args.get(1) {
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "group_n", n);
                }
            }
            cmd_class_eq(args)
        }
        "cayley_action" => {
            if let Some(&n_s) = args.first() {
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "group_n", n);
                }
            }
            cmd_cayley_action(args)
        }
        "necklace" => {
            if let Some(&n_s) = args.first() {
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "group_n", n);
                }
            }
            cmd_necklace(args)
        }
        "demo" => {
            let n: usize = state_get_int(state, "group_n").unwrap_or(6) as usize;
            let g: usize = state_get_int(state, "gen").unwrap_or(2) as usize % n.max(1);
            let mut out = String::new();
            out.push_str(&format!("=== Group Actions Demo: Z/{}Z, generator {} ===\n", n, g));

            let g_order = additive_order(g, n);
            out.push_str(&format!("  ord({}) = {}  in Z/{}Z\n", g, g_order, n));

            let mut visited = vec![false; n];
            let mut orbit_idx = 0;
            for start in 0..n {
                if visited[start] { continue; }
                let mut orbit = Vec::new();
                let mut cur = start;
                loop {
                    orbit.push(cur);
                    visited[cur] = true;
                    cur = (cur + g) % n;
                    if cur == start { break; }
                    if orbit.len() > n { break; }
                }
                let orb_str: Vec<String> = orbit.iter().map(|x| x.to_string()).collect();
                out.push_str(&format!("  Orbit {}: {{ {} }}\n", orbit_idx, orb_str.join(", ")));
                orbit_idx += 1;
            }

            let k = 2usize;
            let mut total_fix = 0u64;
            for d in 0..n {
                let gc = gcd(d as i64, n as i64) as u32;
                total_fix += (k as u64).pow(gc);
            }
            let colorings = total_fix / n as u64;
            out.push_str(&format!("\n  Burnside: {}-colorings of {}-cycle = {}\n", k, n, colorings));
            out.push_str("  Orbit-Stabilizer: |Orb(x)| * |Stab(x)| = |<g>|\n");
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
    state_set_int(&mut s, "group_n", 6);
    state_set_int(&mut s, "gen", 2);
    s
}

fn visualize_svg(canvas: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n: usize = state_get_int(state, "group_n").unwrap_or(6) as usize;
    let g: usize = state_get_int(state, "gen").unwrap_or(2) as usize % n.max(1);

    let cx = 350.0f64;
    let cy = 260.0f64;
    let r = 180.0f64;
    let n_f = n as f64;

    let g_ord = additive_order(g, n);
    let n_orbits = n / g_ord.max(1);

    let orbit_colors = ["#4a90d9", "#e87040", "#50b86c", "#9b59b6", "#e67e22", "#1abc9c"];

    let mut orbit_of = vec![0usize; n];
    {
        let mut visited = vec![false; n];
        let mut oi = 0;
        for start in 0..n {
            if visited[start] { continue; }
            let mut cur = start;
            loop {
                orbit_of[cur] = oi;
                visited[cur] = true;
                cur = (cur + g) % n;
                if cur == start { break; }
                if orbit_of[cur] == oi && visited[cur] { break; }
            }
            oi += 1;
        }
    }

    let mut pts: Vec<(f64, f64)> = Vec::new();
    for i in 0..n {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / n_f - std::f64::consts::PI / 2.0;
        let x = cx + r * angle.cos();
        let y = cy + r * angle.sin();
        pts.push((x, y));
    }

    for i in 0..n {
        let j = (i + g) % n;
        let (x1, y1) = pts[i];
        let (x2, y2) = pts[j];
        let dx = x2 - x1; let dy = y2 - y1;
        let len = (dx*dx + dy*dy).sqrt().max(1.0);
        let sx = x1 + dx * 16.0 / len;
        let sy = y1 + dy * 16.0 / len;
        let ex = x2 - dx * 16.0 / len;
        let ey = y2 - dy * 16.0 / len;
        canvas.arrow(sx, sy, ex, ey, "#aaaaaa", 1.2);
    }

    for i in 0..n {
        let (x, y) = pts[i];
        let color = orbit_colors[orbit_of[i] % orbit_colors.len()];
        canvas.circle(x, y, 14.0, color, "#fff", 1.5);
        canvas.text(x, y + 4.0, &format!("{}", i), 12.0, "white", "middle");
    }

    canvas.text_bold(cx, 28.0, &format!("Z/{}Z under +{}", n, g), 18.0, "#222", "middle");
    canvas.text(cx, 52.0, &format!("ord({}) = {}  |  {} orbit(s)", g, g_ord, n_orbits), 13.0, "#555", "middle");

    let mut seen_orbits: Vec<usize> = Vec::new();
    for i in 0..n {
        if !seen_orbits.contains(&orbit_of[i]) { seen_orbits.push(orbit_of[i]); }
    }
    for (li, oi) in seen_orbits.iter().enumerate() {
        let lx = 30.0 + li as f64 * 110.0;
        let ly = 490.0f64;
        let color = orbit_colors[oi % orbit_colors.len()];
        canvas.circle(lx, ly, 10.0, color, "#fff", 1.0);
        canvas.text(lx + 18.0, ly + 4.0, &format!("Orbit {}", oi), 12.0, "#333", "start");
    }
}

fn visualize_dot(graph: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n: usize = state_get_int(state, "group_n").unwrap_or(6) as usize;
    let g: usize = state_get_int(state, "gen").unwrap_or(2) as usize % n.max(1);

    for i in 0..n {
        graph.node(&format!("x{}", i), &[
            ("label", &format!("{}", i)),
            ("shape", "circle"),
        ]);
    }

    for i in 0..n {
        let j = (i + g) % n;
        graph.edge(&format!("x{}", i), &format!("x{}", j), &[
            ("label", &format!("+{}", g)),
            ("color", "#4a90d9"),
        ]);
    }
}

fn visualize_tex(doc: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n: usize = state_get_int(state, "group_n").unwrap_or(6) as usize;
    let g: usize = state_get_int(state, "gen").unwrap_or(2) as usize % n.max(1);

    for i in 0..n {
        doc.node(&format!("x{}", i), i as f64 * 1.5, 0.0,
            &format!("${}$", i), "draw,circle");
    }

    for i in 0..n {
        let j = (i + g) % n;
        doc.arrow(&format!("x{}", i), &format!("x{}", j), "", "bend left=30");
    }
}

fn visualize_ascii(canvas: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n: usize = state_get_int(state, "group_n").unwrap_or(6) as usize;
    let g: usize = state_get_int(state, "gen").unwrap_or(2) as usize % n.max(1);

    canvas.text_at(0, 0, &format!("Z/{}Z under action +{}:", n, g));

    let g_ord = additive_order(g, n);
    canvas.text_at(0, 1, &format!("  ord({}) = {}  ->  {} orbit(s)", g, g_ord, n / g_ord.max(1)));

    let mut visited = vec![false; n];
    let mut row = 3;
    let mut orbit_idx = 0;
    for start in 0..n {
        if visited[start] { continue; }
        let mut orbit = Vec::new();
        let mut cur = start;
        loop {
            orbit.push(cur);
            visited[cur] = true;
            cur = (cur + g) % n;
            if cur == start { break; }
            if orbit.len() > n { break; }
        }
        let orb_str: Vec<String> = orbit.iter().map(|x| x.to_string()).collect();
        canvas.text_at(0, row, &format!("  Orbit {}: {{ {} }}", orbit_idx, orb_str.join(" -> ")));
        row += 1;
        orbit_idx += 1;
    }

    row += 1;
    let k = 2usize;
    let mut total_fix = 0u64;
    for d in 0..n {
        let gc = gcd(d as i64, n as i64) as u32;
        total_fix += (k as u64).pow(gc);
    }
    let colorings = total_fix / n.max(1) as u64;
    canvas.text_at(0, row, &format!("  Burnside (2 colors): {} colorings", colorings));
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
                    let mut c = SvgCanvas::new(700.0, 520.0);
                    visualize_svg(&mut c, cmd, &args_ref, &state);
                    c.build()
                }
                OutputFormat::Dot => {
                    let mut gr = DotGraph::digraph("ch16");
                    visualize_dot(&mut gr, cmd, &args_ref, &state);
                    gr.build()
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
            print_banner(
                "Chapter 16",
                "Group Actions",
                "Orbits · Stabilizers · Burnside's lemma · Cayley's theorem",
            );
            print_info("Explore how groups act on sets and count structures up to symmetry.");
            print!("{}", show_help());
            repl("ch16> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
