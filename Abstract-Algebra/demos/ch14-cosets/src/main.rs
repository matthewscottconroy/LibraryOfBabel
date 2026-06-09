use common::*;

fn help_string() -> String {
    let mut h = String::new();
    h.push_str("\n  \x1b[1mCommands:\x1b[0m\n");
    h.push_str("    cosets <n> <gens...>          left cosets of ⟨generators⟩ in ℤ/nℤ\n");
    h.push_str("    lagrange <n> <k>              verify Lagrange: subgroup kℤ/nℤ of ℤ/nℤ\n");
    h.push_str("    normal <n> <subgroup_gens...> check normality of subgroup in D_n\n");
    h.push_str("    quotient <n> <k>              quotient group ℤ/nℤ / kℤ/nℤ with Cayley table\n");
    h.push_str("    index <n> <subgroup_size>     index [G:H] = |G|/|H|\n");
    h.push_str("    left_right <n> <gens...>      compare left vs right cosets of ⟨gens⟩ in ℤ/nℤ\n");
    h.push_str("    rz                            ℝ/ℤ ≅ circle group (fractional parts)\n");
    h.push_str("    demo                          run a showcase of coset operations\n");
    h.push_str("    help                          show this help\n");
    h.push_str("    quit                          exit\n");
    h
}

fn generate_subgroup_zn(n: usize, gens: &[usize]) -> Vec<usize> {
    let mut sub = std::collections::BTreeSet::new();
    sub.insert(0usize);
    let mut stack: Vec<usize> = gens.iter().map(|&g| g % n).collect();
    while let Some(x) = stack.pop() {
        if sub.insert(x) {
            stack.push((x + x) % n);
            for &g in gens {
                stack.push((x + g) % n);
            }
        }
    }
    let mut v: Vec<usize> = sub.into_iter().collect();
    v.sort();
    v
}

fn left_coset_zn(n: usize, subgroup: &[usize], rep: usize) -> Vec<usize> {
    let mut c: Vec<usize> = subgroup.iter().map(|&h| (rep + h) % n).collect();
    c.sort();
    c
}

fn right_coset_zn(n: usize, subgroup: &[usize], rep: usize) -> Vec<usize> {
    let mut c: Vec<usize> = subgroup.iter().map(|&h| (h + rep) % n).collect();
    c.sort();
    c
}

fn coset_vec_to_str(c: &[usize]) -> String {
    format!("{{ {} }}", c.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
}

fn cmd_cosets(args: &[&str]) -> String {
    let mut out = String::new();
    if args.is_empty() {
        out.push_str(&format!("  {} {}\n", red("✗"), "Usage: cosets <n> <gen1> [gen2...]"));
        return out;
    }
    let n = match args[0].parse::<usize>().ok() {
        Some(v) if v >= 2 => v,
        _ => { out.push_str(&format!("  {} {}\n", red("✗"), "n must be ≥ 2.")); return out; }
    };
    if args.len() < 2 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Provide at least one generator."));
        return out;
    }
    let gens: Vec<usize> = args[1..].iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .map(|g| g % n)
        .collect();

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Left Cosets of ⟨{}⟩ in ℤ/{}ℤ\x1b[0m\n",
        gens.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "), n));

    let sub = generate_subgroup_zn(n, &gens);
    out.push_str(&format!("  Subgroup H = ⟨{}⟩ = {}  (order {})\n",
        gens.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "),
        coset_vec_to_str(&sub), sub.len()));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut seen: Vec<Vec<usize>> = Vec::new();
    let mut coset_list: Vec<(usize, Vec<usize>)> = Vec::new();

    for rep in 0..n {
        let coset = left_coset_zn(n, &sub, rep);
        if !seen.contains(&coset) {
            seen.push(coset.clone());
            coset_list.push((rep, coset));
        }
    }

    out.push_str(&format!("  {} left cosets (index [G:H] = {}/{}):\n", coset_list.len(), n, sub.len()));
    for (rep, coset) in &coset_list {
        out.push_str(&format!("    {} + H = {}\n", rep, coset_vec_to_str(coset)));
    }

    let total: usize = coset_list.iter().map(|(_, c)| c.len()).sum();
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    if total == n {
        out.push_str(&format!("  {} {}\n", green("✓"), &format!("Cosets partition ℤ/{}ℤ  ✓", n)));
    }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Lagrange: |G| = |H| · [G:H], so subgroup order always divides |G|."));
    out
}

fn cmd_lagrange(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 2 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Usage: lagrange <n> <k>"));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 2) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "n must be ≥ 2.")); return out; }
    };
    let k = match args[1].parse::<usize>().ok().filter(|&v| v >= 1 && v < n) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "k must be between 1 and n-1.")); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Lagrange's Theorem: {}ℤ/{}ℤ in ℤ/{}ℤ\x1b[0m\n", k, n, n));

    let sub = generate_subgroup_zn(n, &[k]);
    let h_size = sub.len();
    let index = n / h_size;

    out.push_str(&format!("  Group G = ℤ/{}ℤ,  |G| = {}\n", n, n));
    out.push_str(&format!("  Subgroup H = ⟨{}⟩ = {}  |H| = {}\n", k, coset_vec_to_str(&sub), h_size));
    out.push_str(&format!("  Index [G:H] = |G|/|H| = {}/{} = {}\n", n, h_size, index));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  {}Coset partition:{}\n", bold(""), ""));
    let mut seen: Vec<Vec<usize>> = Vec::new();
    let mut coset_idx = 0usize;
    for rep in 0..n {
        let coset = left_coset_zn(n, &sub, rep);
        if !seen.contains(&coset) {
            seen.push(coset.clone());
            out.push_str(&format!("    Coset {}: {} + H = {}\n", coset_idx, rep, coset_vec_to_str(&coset)));
            coset_idx += 1;
        }
    }

    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  {} {}\n", green("✓"), &format!("Lagrange verified: {} = {} × {}  ✓", n, h_size, index)));
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Lagrange: |H| | |G| for any subgroup H of finite group G."));
    out
}

fn dn_elem_to_idx(r: usize, s: usize, n: usize) -> usize {
    s * n + r
}
fn dn_idx_to_elem(idx: usize, n: usize) -> (usize, usize) {
    (idx % n, idx / n)
}
fn dn_mul_elem(a: usize, b: usize, n: usize) -> usize {
    let (ra, sa) = dn_idx_to_elem(a, n);
    let (rb, sb) = dn_idx_to_elem(b, n);
    match (sa, sb) {
        (0, 0) => dn_elem_to_idx((ra + rb) % n, 0, n),
        (0, 1) => dn_elem_to_idx((rb + n - ra) % n, 1, n),
        (1, 0) => dn_elem_to_idx((ra + rb) % n, 1, n),
        (1, 1) => dn_elem_to_idx((rb + n - ra) % n, 0, n),
        _ => unreachable!(),
    }
}

fn dn_label(idx: usize, n: usize) -> String {
    let (r, s) = dn_idx_to_elem(idx, n);
    if s == 0 {
        if r == 0 { "e".to_string() } else { format!("r^{}", r) }
    } else {
        if r == 0 { "s".to_string() } else { format!("sr^{}", r) }
    }
}

fn cmd_normal(args: &[&str]) -> String {
    let mut out = String::new();
    if args.is_empty() {
        out.push_str(&format!("  {} {}\n", red("✗"), "Usage: normal <n> <subgroup_gen1> [gen2...]"));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 2 && v <= 8) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "n must be between 2 and 8.")); return out; }
    };
    if args.len() < 2 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Provide at least one subgroup generator."));
        return out;
    }

    let mut gens: Vec<usize> = Vec::new();
    for &arg in &args[1..] {
        let idx = parse_dn_element(arg, n);
        match idx {
            Some(i) => gens.push(i),
            None => {
                out.push_str(&format!("  {} Unknown D_{} element '{}'.\n", red("✗"), n, arg));
                return out;
            }
        }
    }

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Normality Check in D_{}\x1b[0m\n", n));
    let ord = 2 * n;

    let mut sub = std::collections::BTreeSet::new();
    sub.insert(0usize);
    let mut stack = gens.clone();
    while let Some(x) = stack.pop() {
        if sub.insert(x) {
            for &g in &gens {
                stack.push(dn_mul_elem(x, g, n));
            }
            stack.push(dn_mul_elem(x, x, n));
        }
    }
    let sub: Vec<usize> = {
        let mut v: Vec<usize> = sub.into_iter().collect();
        v.sort();
        v
    };

    let sub_labels: Vec<String> = sub.iter().map(|&i| dn_label(i, n)).collect();
    out.push_str(&format!("  D_{}: order {}\n", n, ord));
    out.push_str(&format!("  H = {{ {} }}  (order {})\n", sub_labels.join(", "), sub.len()));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut is_normal = true;
    let mut witness: Option<(usize, usize, usize)> = None;

    let dn_inverse = |x: usize| -> usize {
        let (r, s) = dn_idx_to_elem(x, n);
        if s == 0 {
            dn_elem_to_idx((n - r) % n, 0, n)
        } else {
            dn_elem_to_idx(r, 1, n)
        }
    };

    out.push_str(&format!("  Checking gHg⁻¹ ⊆ H for all g ∈ D_{}:\n", n));
    for g in 0..ord {
        let g_inv = dn_inverse(g);
        for &h in &sub {
            let conj = dn_mul_elem(g, dn_mul_elem(h, g_inv, n), n);
            if !sub.contains(&conj) {
                is_normal = false;
                if witness.is_none() {
                    witness = Some((g, h, conj));
                }
            }
        }
    }

    if is_normal {
        out.push_str(&format!("  {} {}\n", green("✓"), "H is NORMAL in D_n  ✓"));
    } else {
        out.push_str(&format!("  {} {}\n", red("✗"), "H is NOT normal in D_n."));
        if let Some((g, h, conj)) = witness {
            out.push_str(&format!("  Witness: g = {}, h = {}  →  ghg⁻¹ = {} ∉ H\n",
                dn_label(g, n), dn_label(h, n), dn_label(conj, n)));
        }
    }

    out.push_str(&format!("  {} {}\n", yellow("◆"), "In D_n, the rotation subgroup ⟨r⟩ is always normal (index 2)."));
    out
}

fn parse_dn_element(s: &str, n: usize) -> Option<usize> {
    if s == "e" { return Some(0); }
    if s == "s" { return Some(n); }
    if s.starts_with("sr") {
        let k: usize = s[2..].parse().ok()?;
        if k < n { return Some(n + k); }
        return None;
    }
    if s.starts_with('r') {
        let k: usize = s[1..].parse().ok()?;
        if k < n { return Some(k); }
        return None;
    }
    None
}

fn fmt_cayley(elements: &[String], table: &[Vec<usize>]) -> String {
    let mut out = String::new();
    let n = elements.len();
    let w = elements.iter().map(|s| s.len()).max().unwrap_or(1) + 1;
    out.push_str(&format!("  {:>width$} │", "·", width = w));
    for e in elements { out.push_str(&format!(" {:>width$}", cyan(e), width = w)); }
    out.push('\n');
    out.push_str(&format!("  {}─┼", "─".repeat(w)));
    for _ in 0..n { out.push_str(&"─".repeat(w + 1)); }
    out.push('\n');
    for (i, row) in table.iter().enumerate() {
        out.push_str(&format!("  {:>width$} │", green(&elements[i]), width = w));
        for &j in row { out.push_str(&format!(" {:>width$}", elements[j], width = w)); }
        out.push('\n');
    }
    out
}

fn cmd_quotient(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 2 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Usage: quotient <n> <k>"));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 2) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "n must be ≥ 2.")); return out; }
    };
    let k = match args[1].parse::<usize>().ok().filter(|&v| v >= 1 && v < n) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "k must be between 1 and n-1.")); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Quotient Group ℤ/{}ℤ  /  ⟨{}⟩\x1b[0m\n", n, k));

    let sub = generate_subgroup_zn(n, &[k]);
    let h_size = sub.len();

    if n % h_size != 0 {
        out.push_str(&format!("  {} {}\n", red("✗"), "⟨k⟩ does not have order dividing n."));
        return out;
    }

    let q_size = n / h_size;
    out.push_str(&format!("  H = ⟨{}⟩ = {}  (order {})\n", k, coset_vec_to_str(&sub), h_size));
    out.push_str(&format!("  G/H has order |G|/|H| = {}/{} = {}\n", n, h_size, q_size));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut cosets: Vec<Vec<usize>> = Vec::new();
    let mut seen: Vec<Vec<usize>> = Vec::new();
    let mut reps: Vec<usize> = Vec::new();
    for rep in 0..n {
        let c = left_coset_zn(n, &sub, rep);
        if !seen.contains(&c) {
            seen.push(c.clone());
            cosets.push(c);
            reps.push(rep);
        }
    }

    out.push_str(&format!("  {}Cosets (elements of G/H):{}\n", bold(""), ""));
    for (i, (rep, coset)) in reps.iter().zip(cosets.iter()).enumerate() {
        out.push_str(&format!("    [{}] = {} + H = {}\n", i, rep, coset_vec_to_str(coset)));
    }
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let q = q_size;
    if q > 12 {
        out.push_str(&format!("  {} {}\n", yellow("◆"), "Quotient group too large to display Cayley table."));
        return out;
    }

    let coset_mul = |i: usize, j: usize| -> usize {
        let rep_sum = (reps[i] + reps[j]) % n;
        let c = left_coset_zn(n, &sub, rep_sum);
        cosets.iter().position(|x| *x == c).unwrap_or(0)
    };

    let q_elements: Vec<String> = (0..q).map(|i| format!("[{}]", reps[i])).collect();
    let q_table: Vec<Vec<usize>> = (0..q).map(|i| (0..q).map(|j| coset_mul(i, j)).collect()).collect();

    out.push_str(&format!("  {}Cayley table of G/H:{}\n", bold(""), ""));
    out.push_str(&fmt_cayley(&q_elements, &q_table));

    out.push_str(&format!("  {} {} {}\n", cyan("G/H ≅"), dim("="), green(&format!("ℤ/{}ℤ", q_size))));
    out.push_str(&format!("  {} {}\n", yellow("◆"), "ℤ/nℤ / kℤ ≅ ℤ/(n/gcd(n,k))ℤ."));
    out
}

fn cmd_index(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 2 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Usage: index <n> <subgroup_size>"));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 1) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "n must be ≥ 1.")); return out; }
    };
    let h = match args[1].parse::<usize>().ok().filter(|&v| v >= 1) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "subgroup_size must be ≥ 1.")); return out; }
    };

    out.push_str("\n  \x1b[1m\x1b[33m▸ Index [G : H]\x1b[0m\n");
    if n % h != 0 {
        out.push_str(&format!("  {} |H| = {} does not divide |G| = {}  — violates Lagrange!\n", red("✗"), h, n));
        return out;
    }

    let idx = n / h;
    out.push_str(&format!("  |G| = {},  |H| = {}\n", n, h));
    out.push_str(&format!("  {} {} {}\n", cyan("[G : H]"), dim("="), green(&format!("{}", idx))));
    out.push_str(&format!("  Formula: [G : H] = |G| / |H| = {} / {} = {}\n", n, h, idx));
    out.push_str("\n");
    out.push_str(&format!("  Interpretation: G decomposes into {} disjoint cosets of H.\n", idx));

    if idx == 2 { out.push_str(&format!("  {} {}\n", green("✓"), "Index 2 subgroups are always normal!")); }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Orbit-stabilizer: |G| = |Orb(x)| · |Stab(x)| is a special case."));
    out
}

fn cmd_left_right(args: &[&str]) -> String {
    let mut out = String::new();
    if args.is_empty() {
        out.push_str(&format!("  {} {}\n", red("✗"), "Usage: left_right <n> <gen1> [gen2...]"));
        return out;
    }
    let n = match args[0].parse::<usize>().ok().filter(|&v| v >= 2) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "n must be ≥ 2.")); return out; }
    };
    if args.len() < 2 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Provide at least one generator."));
        return out;
    }
    let gens: Vec<usize> = args[1..].iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .map(|g| g % n)
        .collect();

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Left vs Right Cosets of ⟨{}⟩ in ℤ/{}ℤ\x1b[0m\n",
        gens.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "), n));

    let sub = generate_subgroup_zn(n, &gens);
    out.push_str(&format!("  H = ⟨{}⟩ = {}\n",
        gens.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "),
        coset_vec_to_str(&sub)));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let mut left_distinct: Vec<Vec<usize>> = Vec::new();
    let mut right_distinct: Vec<Vec<usize>> = Vec::new();

    for rep in 0..n {
        let lc = left_coset_zn(n, &sub, rep);
        let rc = right_coset_zn(n, &sub, rep);
        if !left_distinct.contains(&lc) { left_distinct.push(lc); }
        if !right_distinct.contains(&rc) { right_distinct.push(rc); }
    }

    out.push_str(&format!("  {}Left cosets:{}\n", bold(""), ""));
    for c in &left_distinct { out.push_str(&format!("    {}\n", coset_vec_to_str(c))); }
    out.push_str("\n");
    out.push_str(&format!("  {}Right cosets:{}\n", bold(""), ""));
    for c in &right_distinct { out.push_str(&format!("    {}\n", coset_vec_to_str(c))); }
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    if left_distinct == right_distinct {
        out.push_str(&format!("  {} {}\n", green("✓"), "Left cosets = Right cosets  → H is normal in G  ✓"));
    } else {
        out.push_str(&format!("  {} {}\n", red("✗"), "Left cosets ≠ right cosets  → H is NOT normal."));
    }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "A subgroup is normal iff every left coset equals the corresponding right coset."));
    out
}

fn cmd_rz() -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ ℝ/ℤ ≅ Circle Group\x1b[0m\n");
    out.push_str(&format!("  {}The circle group S1{}\n", bold(""), ""));
    out.push_str("  The map θ ↦ e^(2πiθ) : ℝ → S¹ has kernel ℤ.\n");
    out.push_str("  By the First Isomorphism Theorem: ℝ/ℤ ≅ S¹.\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    out.push_str(&format!("  {:>10}  {:>15}  {:>20}\n", bold("x"), bold("[x] in [0,1)"), bold("e^(2πix)")));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    let fracs: Vec<f64> = vec![0.0, 0.25, 0.5, 0.75, 0.1, 0.333, 1.5, -0.3];
    for x in fracs {
        let frac = x.rem_euclid(1.0);
        let theta = 2.0 * std::f64::consts::PI * frac;
        let re = theta.cos();
        let im = theta.sin();
        let sign = if im < 0.0 { "−" } else { "+" };
        out.push_str(&format!("  {:>10.4}  {:>15.4}  {:>20}\n",
            x, frac, format!("{:.4} {}  {:.4}i", re, sign, im.abs())));
    }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "ℝ/ℤ is a compact abelian group, locally isomorphic to ℝ."));
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Characters of ℝ/ℤ are χₙ(x) = e^(2πinx), n ∈ ℤ: Fourier analysis!"));
    out
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "cosets" => {
            if let Some(n) = args.first().and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "group_n", n);
            }
            if let Some(k) = args.get(1).and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "subgroup_gen", k);
            }
            cmd_cosets(args)
        }
        "lagrange" => {
            if let Some(n) = args.first().and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "group_n", n);
            }
            if let Some(k) = args.get(1).and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "subgroup_gen", k);
            }
            cmd_lagrange(args)
        }
        "normal"     => cmd_normal(args),
        "quotient"   => {
            if let Some(n) = args.first().and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "group_n", n);
            }
            if let Some(k) = args.get(1).and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "subgroup_gen", k);
            }
            cmd_quotient(args)
        }
        "index"      => cmd_index(args),
        "left_right" => cmd_left_right(args),
        "rz"         => cmd_rz(),
        "demo" => {
            let mut out = String::new();
            out.push_str("\n  === Demo: Cosets and Quotient Groups ===\n\n");
            let n = 12usize; let k = 4usize;
            let sub = generate_subgroup_zn(n, &[k]);
            out.push_str(&format!("  ℤ/{}ℤ, H = <{}> = {}  (order {})\n", n, k, coset_vec_to_str(&sub), sub.len()));
            let mut seen: Vec<Vec<usize>> = Vec::new();
            let mut coset_idx = 0;
            for rep in 0..n {
                let coset = left_coset_zn(n, &sub, rep);
                if !seen.contains(&coset) {
                    seen.push(coset.clone());
                    out.push_str(&format!("  Coset {}: {} + H = {}\n", coset_idx, rep, coset_vec_to_str(&coset)));
                    coset_idx += 1;
                }
            }
            out.push_str(&format!("  [G:H] = {}/{} = {}\n", n, sub.len(), n/sub.len()));
            let _ = state;
            out
        }
        "help" | "h" => help_string(),
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
    state_set_int(&mut s, "group_n", 12);
    state_set_int(&mut s, "subgroup_gen", 4);
    s
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "group_n").unwrap_or(12) as usize;
    let k = state_get_int(state, "subgroup_gen").unwrap_or(4) as usize;

    c.title(&format!("Ch14: Coset Partition of Z/{}Z by <{}>", n, k));
    c.subtitle("Each row is a distinct coset (Hasse-like partition diagram)", 42.0);

    let sub = generate_subgroup_zn(n, &[k]);
    let mut seen: Vec<Vec<usize>> = Vec::new();
    let mut cosets: Vec<(usize, Vec<usize>)> = Vec::new();
    for rep in 0..n {
        let coset = left_coset_zn(n, &sub, rep);
        if !seen.contains(&coset) {
            seen.push(coset.clone());
            cosets.push((rep, coset));
        }
    }

    let num_cosets = cosets.len();
    let box_w = 80.0;
    let box_h = 35.0;
    let x_start = 40.0;
    let y_start = 65.0;
    let x_gap = (620.0 / num_cosets as f64).min(100.0);

    // G label at top
    c.rrect(280.0, y_start - 50.0, 140.0, 30.0, 5.0, colors::HEADER_FILL, colors::CYAN, 1.5);
    c.text_bold(350.0, y_start - 35.0, &format!("G = Z/{}Z  (order {})", n, n), 12.0, colors::DARK, "middle");

    for (i, (rep, coset)) in cosets.iter().enumerate() {
        let x = x_start + i as f64 * x_gap;
        let y = y_start;
        let fill = if *rep == 0 { colors::HEADER_FILL } else { colors::ROW_ALT };
        c.rrect(x, y, box_w, box_h, 5.0, fill, colors::CYAN, 1.5);
        let elems: Vec<String> = coset.iter().take(4).map(|x| x.to_string()).collect();
        let label = if coset.len() > 4 {
            format!("{}...", elems.join(","))
        } else {
            elems.join(",")
        };
        c.text_bold(x + box_w/2.0, y + 10.0, &format!("{}+H", rep), 11.0, colors::DARK, "middle");
        c.text(x + box_w/2.0, y + 25.0, &format!("{{{}}}", label), 9.0, colors::GREY, "middle");
        // Arrow from G down
        c.arrow(350.0, y_start - 20.0, x + box_w/2.0, y, colors::GREY, 0.8);
    }

    let info_y = y_start + box_h + 30.0;
    c.text(40.0, info_y, &format!("|G|={}, H=<{}> order {}, [G:H]={}", n, k, sub.len(), num_cosets),
           12.0, colors::DARK, "start");
    c.text(40.0, info_y + 18.0, "Lagrange: |G| = |H| * [G:H]  (cosets partition G)",
           11.0, colors::GREY, "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "group_n").unwrap_or(12) as usize;
    let k = state_get_int(state, "subgroup_gen").unwrap_or(4) as usize;
    let sub = generate_subgroup_zn(n, &[k]);
    let mut seen: Vec<Vec<usize>> = Vec::new();
    let mut cosets: Vec<(usize, Vec<usize>)> = Vec::new();
    for rep in 0..n {
        let coset = left_coset_zn(n, &sub, rep);
        if !seen.contains(&coset) {
            seen.push(coset.clone());
            cosets.push((rep, coset));
        }
    }
    g.node_default("shape", "box");
    g.node("G", &[("label", &format!("Z/{}Z", n)), ("style", "filled"), ("fillcolor", "lightblue")]);
    g.node("H", &[("label", &format!("<{}>={:?}", k, sub)), ("style", "filled"), ("fillcolor", "lightgreen")]);
    g.edge("G", "H", &[("label", &format!("subgp order {}", sub.len()))]);
    for (rep, coset) in &cosets {
        let id = format!("C{}", rep);
        let elems: Vec<String> = coset.iter().map(|x| x.to_string()).collect();
        g.node(&id, &[("label", &format!("{}+H={{{}}}", rep, elems.join(",")))]);
        g.edge("G", &id, &[("style", "dashed")]);
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "group_n").unwrap_or(12) as usize;
    let k = state_get_int(state, "subgroup_gen").unwrap_or(4) as usize;
    t.use_library("positioning,arrows");
    let sub = generate_subgroup_zn(n, &[k]);
    let mut seen: Vec<Vec<usize>> = Vec::new();
    let mut cosets: Vec<(usize, Vec<usize>)> = Vec::new();
    for rep in 0..n {
        let coset = left_coset_zn(n, &sub, rep);
        if !seen.contains(&coset) { seen.push(coset.clone()); cosets.push((rep, coset)); }
    }
    t.raw("  \\tikzset{coset/.style={draw,rectangle,minimum width=1.5cm,minimum height=0.6cm}}");
    t.node_math("G", 0.0, 2.0, &format!("G = \\mathbb{{Z}}/{n}\\mathbb{{Z}}"), "draw,ellipse");
    for (i, (rep, _coset)) in cosets.iter().enumerate() {
        let x = -2.0 * (cosets.len() as f64 / 2.0) + i as f64 * 1.8;
        t.node_math(&format!("C{}", i), x, 0.0, &format!("{}+H", rep), "coset");
        t.arrow("G", &format!("C{}", i), "", "->");
    }
    t.node_math("H", 0.0, -2.0, &format!("H = \\langle {} \\rangle", k), "draw,rectangle");
    t.raw(&format!("  \\node[below=2.5cm of G] {{$|G|={}$, $|H|={}$, $[G:H]={}$}};",
        n, sub.len(), cosets.len()));
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "group_n").unwrap_or(12) as usize;
    let k = state_get_int(state, "subgroup_gen").unwrap_or(4) as usize;
    let sub = generate_subgroup_zn(n, &[k]);

    a.text_at(2, 1, &format!("Ch14: Coset Partition  Z/{}Z by <{}>", n, k));
    a.text_at(2, 2, "──────────────────────────────────────────────");
    a.text_at(2, 3, &format!("  H = <{}> = {:?}  (order {})", k, sub, sub.len()));
    a.text_at(2, 4, &format!("  [G:H] = {}/{} = {}", n, sub.len(), n/sub.len()));
    a.text_at(2, 5, "──────────────────────────────────────────────");

    let mut seen: Vec<Vec<usize>> = Vec::new();
    let mut row = 6i32;
    for rep in 0..n {
        let coset = left_coset_zn(n, &sub, rep);
        if !seen.contains(&coset) {
            seen.push(coset.clone());
            let elems: Vec<String> = coset.iter().map(|x| x.to_string()).collect();
            a.text_at(2, row, &format!("  {}+H = {{  {}  }}", rep, elems.join(", ")));
            row += 1;
        }
    }
    a.text_at(2, row + 1, "Lagrange: |G| = |H| * [G:H]  (cosets partition G)");
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
                    let mut g = DotGraph::digraph("ch14");
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
            print_banner("Chapter 14", "Cosets, Normal Subgroups, and Quotient Groups",
                "Lagrange's theorem · Coset partitions · Quotients · Normality");
            print_info("Explore how subgroups partition groups and give rise to quotient groups.");
            print!("{}", help_string());
            repl("ch14> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
