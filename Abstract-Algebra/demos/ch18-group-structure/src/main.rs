use common::*;

fn show_help() -> String {
    let mut h = String::new();
    h.push_str("\n  \x1b[1mCommands:\x1b[0m\n");
    h.push_str("    derived <type> <n>       commutator series G >= G' >= G'' >= ... for Z or D group\n");
    h.push_str("    composition <type> <n>   composition series and composition factors\n");
    h.push_str("    solvable <n>             check if S_n is solvable (yes for n<=4, no for n>=5)\n");
    h.push_str("    simple <n>               check if A_n is simple for n >= 5\n");
    h.push_str("    free_word <n>            enumerate reduced words of F_2 up to length n\n");
    h.push_str("    jordan_holder            Jordan-Holder theorem for S_4: two series, same factors\n");
    h.push_str("    nilpotent <p> <a>        p-groups are nilpotent: lower central series for Z/p^aZ\n");
    h.push_str("    demo                     demonstrate derived/composition series for current group\n");
    h.push_str("    help                     show this help\n");
    h.push_str("    quit                     exit\n");
    h
}

// ─── Group infrastructure ─────────────────────────────────────────────────────

/// Subgroup closure: given a set of elements in Z/nZ (additive), close under operation
fn subgroup_closure_zn(n: usize, generators: &[usize]) -> Vec<usize> {
    let mut sub = std::collections::BTreeSet::new();
    sub.insert(0usize);
    let mut stack: Vec<usize> = generators.to_vec();
    while let Some(x) = stack.pop() {
        if sub.insert(x % n) {
            for &g in generators {
                let prod = (x + g) % n;
                if !sub.contains(&prod) { stack.push(prod); }
            }
            let inv = (n - x % n) % n;
            if !sub.contains(&inv) { stack.push(inv); }
        }
    }
    let mut v: Vec<usize> = sub.into_iter().collect();
    v.sort();
    v
}

/// [D_n, D_n] = <r^2>
fn dn_commutator_subgroup(n: usize) -> Vec<usize> {
    let mut gens = Vec::new();
    if n > 0 {
        gens.push(2 % n);
    }
    subgroup_closure_zn(n, &gens)
}

fn derived_series_zn(n: usize) -> Vec<(String, Vec<usize>)> {
    vec![
        (format!("G = Z/{}Z", n), (0..n).collect()),
        ("G' = {0}".to_string(), vec![0]),
    ]
}

fn derived_series_dn(n: usize) -> Vec<(String, Vec<usize>)> {
    let mut series: Vec<(String, Vec<usize>)> = Vec::new();
    series.push((format!("D_{}", n), (0..n).collect()));
    let d1 = dn_commutator_subgroup(n);
    series.push(("D_n' = <r^2>".to_string(), d1.clone()));
    series.push(("D_n'' = {e}".to_string(), vec![0]));
    series
}

fn cmd_derived(args: &[&str]) -> String {
    let mut out = String::new();
    let gtype = match args.first() {
        Some(&s) => s,
        None => { out.push_str(&format!("  {} Usage: derived <Z|D> <n>\n", red("✗"))); return out; }
    };
    let n = match args.get(1).and_then(|s| s.parse::<usize>().ok()).filter(|&v| v >= 2) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be >= 2.\n", red("✗"))); return out; }
    };

    match gtype {
        "Z" => {
            out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Derived Series of Z/{}Z\x1b[0m\n", n));
            out.push_str(&format!("  Z/{}Z is abelian: [G,G] = {{e}} immediately.\n", n));
            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
            let series = derived_series_zn(n);
            for (i, (label, sub)) in series.iter().enumerate() {
                let sub_str: Vec<String> = sub.iter().map(|x| x.to_string()).collect();
                out.push_str(&format!("  G_{} = {}  =  {{{}}}  (order {})\n",
                    i, label, sub_str.join(", "), sub.len()));
            }
            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
            out.push_str(&format!("  {} G' = {{e}}: Z/nZ is abelian, hence solvable.\n", green("✓")));
            out.push_str(&format!("  {} A group is solvable iff its derived series reaches {{e}}.\n", yellow("◆")));
        }
        "D" => {
            out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Derived Series of D_{}\x1b[0m\n", n));
            out.push_str(&format!("  D_{} = dihedral group of order {}\n", n, 2*n));
            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
            let series = derived_series_dn(n);
            for (i, (label, sub)) in series.iter().enumerate() {
                let sub_str: Vec<String> = sub.iter().map(|x| x.to_string()).collect();
                out.push_str(&format!("  G_{} = {}  =  {{{}}}  (order {})\n",
                    i, label, sub_str.join(", "), sub.len()));
            }
            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
            out.push_str(&format!("  {} D_{} is solvable: derived series terminates at {{e}}.\n", green("✓"), n));
            out.push_str(&format!("  {} D_n' = <r^2>, and since <r^2> is abelian (cyclic), D_n'' = {{e}}.\n", yellow("◆")));
        }
        _ => { out.push_str(&format!("  {} Type must be Z or D.\n", red("✗"))); return out; }
    }
    out.push_str(&format!("  {} Derived series: G^(0) = G, G^(i+1) = [G^(i), G^(i)].\n", yellow("◆")));
    out.push_str(&format!("  {} G is solvable iff G^(k) = {{e}} for some k.\n", yellow("◆")));
    out
}

fn cmd_composition(args: &[&str]) -> String {
    let mut out = String::new();
    let gtype = match args.first() {
        Some(&s) => s,
        None => { out.push_str(&format!("  {} Usage: composition <Z|D|S> <n>\n", red("✗"))); return out; }
    };
    let n = match args.get(1).and_then(|s| s.parse::<usize>().ok()).filter(|&v| v >= 1) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be >= 1.\n", red("✗"))); return out; }
    };

    match gtype {
        "Z" => {
            out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Composition Series of Z/{}Z\x1b[0m\n", n));
            out.push_str(&format!("  Z/{}Z has order {}.\n", n, n));
            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

            let facts = factor(n as u64);
            out.push_str(&format!("  {} = {}\n", n,
                facts.iter().map(|(p, a)| if *a == 1 { format!("{}", p) } else { format!("{}^{}", p, a) })
                     .collect::<Vec<_>>().join(" * ")));
            out.push('\n');

            let mut chain: Vec<(usize, String)> = Vec::new();
            chain.push((n, format!("Z/{}Z", n)));

            let mut current_order = n;
            let primes_in_order: Vec<u64> = facts.iter()
                .flat_map(|&(p, a)| std::iter::repeat(p).take(a as usize))
                .collect();

            for p in &primes_in_order {
                current_order /= *p as usize;
                chain.push((current_order, format!("Z/{}Z", current_order)));
            }

            out.push_str("  Composition series:\n");
            for (i, (ord, label)) in chain.iter().enumerate() {
                if i + 1 < chain.len() {
                    let next_ord = chain[i + 1].0;
                    let factor_ord = ord / next_ord;
                    out.push_str(&format!("    {} >= {}  [factor: Z/{}Z]\n", label, chain[i+1].1, factor_ord));
                } else {
                    out.push_str(&format!("    {} >= {{e}}\n", label));
                }
            }

            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
            out.push_str("  Composition factors:\n");
            let primes_display: Vec<String> = facts.iter()
                .flat_map(|&(p, a)| std::iter::repeat(p as usize).take(a as usize))
                .map(|p| format!("Z/{}Z", p))
                .collect();
            for f in &primes_display {
                out.push_str(&format!("    {}\n", f));
            }
            out.push_str(&format!("  {} All composition factors are simple (Z/pZ for prime p).\n", yellow("◆")));
            out.push_str(&format!("  {} Jordan-Holder: any two composition series give the same factors.\n", yellow("◆")));
        }
        "D" => {
            out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Composition Series of D_{}\x1b[0m\n", n));
            if n < 2 { out.push_str(&format!("  {} n must be >= 2 for D_n.\n", red("✗"))); return out; }
            out.push_str(&format!("  D_{}: order {}\n", n, 2*n));
            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

            out.push_str(&format!("  Chain: D_{} >= <r> >= {{e}}\n", n));
            out.push_str(&format!("  Factors: D_{}/{}<r> = D_n/<r> ≅ Z/2Z,  <r>/{{e}} ≅ Z/{}Z\n", n, "", n));

            if is_prime(n as u64) {
                out.push_str(&format!("  {} Since n is prime, <r> ≅ Z/nZ is simple, so this is a composition series.\n", yellow("◆")));
            } else {
                out.push_str(&format!("  {} Since n is composite, <r> is not simple; need a refinement.\n", yellow("◆")));
                out.push_str(&format!("  Refined: D_{} >= <r> >= <r^p> >= ... >= {{e}}  for each prime p | n\n", n));
            }
        }
        "S" => {
            out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Composition Series of S_{}\x1b[0m\n", n));
            out.push_str(&format!("  S_{}: order {}! = {}\n", n, n, (1..=n).product::<usize>()));
            out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

            match n {
                1 | 2 => {
                    out.push_str("  S_1 = {e}  or  S_2 = Z/2Z  (trivial series)\n");
                }
                3 => {
                    out.push_str("  S_3 >= A_3 >= {e}\n");
                    out.push_str("  Factors: S_3/A_3 ≅ Z/2Z,  A_3 ≅ Z/3Z  (simple)\n");
                    out.push_str(&format!("  {} S_3 is solvable.\n", green("✓")));
                }
                4 => {
                    out.push_str("  S_4 >= A_4 >= V_4 >= Z/2Z >= {e}\n");
                    out.push_str("  Factors: S_4/A_4 ≅ Z/2Z\n");
                    out.push_str("           A_4/V_4 ≅ Z/3Z\n");
                    out.push_str("           V_4/Z/2Z ≅ Z/2Z\n");
                    out.push_str("           Z/2Z/{e} ≅ Z/2Z\n");
                    out.push_str(&format!("  {} S_4 is solvable (all factors are Z/pZ).\n", green("✓")));
                }
                n if n >= 5 => {
                    out.push_str(&format!("  S_{} >= A_{} >= {{e}}\n", n, n));
                    out.push_str(&format!("  Factors: S_{}/A_{} ≅ Z/2Z\n", n, n));
                    out.push_str(&format!("           A_{} is simple (no normal subgroups)\n", n));
                    out.push_str(&format!("  {} S_{} is NOT solvable (A_{} is a non-abelian simple factor).\n", red("✗"), n, n));
                }
                _ => {}
            }
        }
        _ => { out.push_str(&format!("  {} Type must be Z, D, or S.\n", red("✗"))); }
    }
    out
}

fn cmd_solvable(args: &[&str]) -> String {
    let mut out = String::new();
    let n_str = args.first().copied().unwrap_or("4");
    let n = match n_str.parse::<usize>().ok().filter(|&v| v >= 1 && v <= 8) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be between 1 and 8.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Is S_{} Solvable?\x1b[0m\n", n));
    out.push_str(&format!("  S_{} has order {}! = {}\n", n, n, (1..=n).product::<usize>()));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    match n {
        1 => {
            out.push_str(&format!("  {} S_1 = {{e}} is trivially solvable.\n", green("✓")));
        }
        2 => {
            out.push_str("  S_2 = Z/2Z: derived series S_2 >= {e}.\n");
            out.push_str(&format!("  {} S_2 is solvable (abelian).\n", green("✓")));
        }
        3 => {
            out.push_str("  Derived series:\n");
            out.push_str("    S_3^(0) = S_3  (order 6)\n");
            out.push_str("    S_3^(1) = [S_3, S_3] = A_3 = Z/3Z  (order 3)\n");
            out.push_str("    S_3^(2) = [A_3, A_3] = {e}  (A_3 is abelian)\n");
            out.push_str(&format!("  {} S_3 is solvable.\n", green("✓")));
        }
        4 => {
            out.push_str("  Derived series:\n");
            out.push_str("    S_4^(0) = S_4  (order 24)\n");
            out.push_str("    S_4^(1) = A_4  (order 12, even permutations)\n");
            out.push_str("    S_4^(2) = [A_4,A_4] = V_4  (Klein four-group, order 4)\n");
            out.push_str("    S_4^(3) = [V_4,V_4] = {e}  (V_4 is abelian)\n");
            out.push_str(&format!("  {} S_4 is solvable.\n", green("✓")));
            out.push_str(&format!("  {} V_4 = {{e,(12)(34),(13)(24),(14)(23)}} is the key normal subgroup.\n", yellow("◆")));
        }
        n if n >= 5 => {
            out.push_str("  Derived series:\n");
            out.push_str(&format!("    S_{}^(0) = S_{}  (order {})\n", n, n, (1..=n).product::<usize>()));
            out.push_str(&format!("    S_{}^(1) = A_{}  (order {})\n", n, n, (1..=n).product::<usize>() / 2));
            out.push_str(&format!("    S_{}^(2) = A_{}  (A_n is PERFECT: [A_n,A_n] = A_n)\n", n, n));
            out.push_str("    Series does not reach {e}!\n");
            out.push_str(&format!("  {} S_{} is NOT solvable  (n >= 5).\n", red("✗"), n));
            out.push_str(&format!("  {} A_{} is a non-abelian simple group for n >= 5.\n", yellow("◆"), n));
            out.push_str(&format!("  {} This is related to the insolubility of polynomials of degree >= 5 by radicals.\n", yellow("◆")));
        }
        _ => {}
    }

    out.push_str(&format!("  {} Galois proved: degree-n polynomial solvable by radicals iff Gal(f) is solvable.\n", yellow("◆")));
    out
}

fn cmd_simple(args: &[&str]) -> String {
    let mut out = String::new();
    let n_str = args.first().copied().unwrap_or("5");
    let n = match n_str.parse::<usize>().ok().filter(|&v| v >= 2 && v <= 8) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be between 2 and 8.\n", red("✗"))); return out; }
    };

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Is A_{} Simple?\x1b[0m\n", n));
    let an_order = (1..=n).product::<usize>() / 2;
    out.push_str(&format!("  A_{} has order {}!/2 = {}\n", n, n, an_order));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    match n {
        2 => {
            out.push_str("  A_2 = {e}  (trivial group).\n");
            out.push_str(&format!("  {} Trivial group: simple by convention.\n", yellow("◆")));
        }
        3 => {
            out.push_str("  A_3 = Z/3Z  (cyclic of order 3, prime).\n");
            out.push_str(&format!("  {} A_3 is simple (cyclic of prime order).\n", green("✓")));
        }
        4 => {
            out.push_str("  A_4 has order 12 and contains V_4 = {e,(12)(34),(13)(24),(14)(23)}\n");
            out.push_str("  V_4 is a normal subgroup of A_4 (not just of S_4).\n");
            out.push_str(&format!("  {} A_4 is NOT simple (V_4 is a proper normal subgroup).\n", red("✗")));
            out.push_str(&format!("  {} This is why S_4 is solvable but S_5 is not.\n", yellow("◆")));
        }
        5 => {
            out.push_str("  A_5 has order 60.\n");
            out.push_str(&format!("  {}Proof of simplicity (outline):{}\n", bold(""), ""));
            out.push_str("  1. Count elements by cycle type:\n");
            out.push_str("     - Identity: 1\n");
            out.push_str("     - 3-cycles (abc): 20\n");
            out.push_str("     - 5-cycles (abcde): 24\n");
            out.push_str("     - Products of 2-cycles (ab)(cd): 15\n");
            out.push_str("  2. Any normal subgroup N must be a union of conjugacy classes.\n");
            out.push_str("  3. |N| must divide 60 = |A_5|.\n");
            out.push_str("  4. Check: no subset of {1,20,24,15} sums to a proper divisor of 60\n");
            out.push_str("     (except 1 and 60).\n");
            out.push_str(&format!("  {} A_5 is simple!\n", green("✓")));
            out.push_str("\n  Divisors of 60: 1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60\n");
            out.push_str("  Class sizes: 1, 20, 24, 15 -- no proper sub-sum gives a divisor of 60.\n");
        }
        n if n >= 6 => {
            out.push_str(&format!("  For n >= 5: A_n is simple.\n"));
            out.push_str("  Proof: A_5 is simple (base case).\n");
            out.push_str("  Induction: A_n is generated by 3-cycles.\n");
            out.push_str("  Any normal N <= A_n with N != {e} must contain all 3-cycles,\n");
            out.push_str("  hence N = A_n.\n");
            out.push_str(&format!("  {} A_{} is simple  (by induction from A_5).\n", green("✓"), n));
        }
        _ => {}
    }

    out.push_str(&format!("  {} Simple groups are the 'atoms' of group theory.\n", yellow("◆")));
    out.push_str(&format!("  {} Classification of finite simple groups: 26 sporadic + families.\n", yellow("◆")));
    out
}

fn cmd_free_word(args: &[&str]) -> String {
    let mut out = String::new();
    let n_str = args.first().copied().unwrap_or("3");
    let max_len = match n_str.parse::<usize>().ok().filter(|&v| v <= 5) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} n must be between 0 and 5.\n", red("✗"))); return out; }
    };

    out.push_str("\n  \x1b[1m\x1b[33m▸ Free Group F_2: Reduced Words\x1b[0m\n");
    out.push_str("  F_2 = <a, b> = free group on generators a, b.\n");
    out.push_str("  Elements are reduced words in {a, a^-1, b, b^-1}.\n");
    out.push_str("  Reduced: no adjacent pair like a*a^-1 or b^-1*b.\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    let gen_names = ["a", "a\u{207b}\u{00b9}", "b", "b\u{207b}\u{00b9}"];
    let inverse_of = |g: usize| -> usize { g ^ 1 };

    let mut words_by_len: Vec<Vec<Vec<usize>>> = Vec::new();
    words_by_len.push(vec![vec![]]); // length 0: identity

    for _ in 1..=max_len {
        let prev = words_by_len.last().unwrap().clone();
        let mut new_words = Vec::new();
        for word in &prev {
            for g in 0..4 {
                if let Some(&last) = word.last() {
                    if last == inverse_of(g) { continue; }
                }
                let mut w = word.clone();
                w.push(g);
                new_words.push(w);
            }
        }
        words_by_len.push(new_words);
    }

    for (len, words) in words_by_len.iter().enumerate() {
        out.push_str(&format!("  {}Length {}:{} {} words\n", bold(""), len, "", words.len()));
        let display_words: Vec<String> = words.iter().take(8).map(|w| {
            if w.is_empty() { "e".to_string() }
            else { w.iter().map(|&g| gen_names[g]).collect::<Vec<_>>().join("") }
        }).collect();
        let etc = if words.len() > 8 { format!("  ... (+{})", words.len() - 8) } else { String::new() };
        out.push_str(&format!("    {}{}\n", display_words.join(",  "), etc));
    }

    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    let total: usize = words_by_len.iter().map(|v| v.len()).sum();
    out.push_str(&format!("  Total reduced words of length <= {}: {}\n\n", max_len, total));

    out.push_str(&format!("  {}Growth formula:{} |words of length n| = 4 * 3^(n-1)  for n >= 1\n", bold(""), ""));
    out.push_str("  (Each generator has 4 choices; each subsequent has 3 to stay reduced.)\n");
    if max_len >= 1 {
        for k in 1..=max_len {
            let formula = 4 * (3usize).pow((k - 1) as u32);
            let actual = words_by_len[k].len();
            out.push_str(&format!("    n={}: 4*3^{} = {}  (actual: {})  {}\n",
                k, k-1, formula, actual,
                if formula == actual { green("OK") } else { red("mismatch") }));
        }
    }

    out.push_str(&format!("  {} F_2 has exponential growth: it is NOT amenable and contains F_n for all n.\n", yellow("◆")));
    out.push_str(&format!("  {} The Cayley graph of F_2 is an infinite 4-regular tree.\n", yellow("◆")));
    out
}

fn cmd_jordan_holder() -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Jordan-Holder Theorem for S_4\x1b[0m\n");
    out.push_str(&format!("  {}Theorem:{} Any two composition series of G have the same\n", bold(""), ""));
    out.push_str("  length and the same composition factors (up to permutation).\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str("  S_4 has order 24 = 2^3 * 3.\n\n");
    out.push_str(&format!("  {}Series 1:{}\n", bold(""), ""));
    out.push_str("    S_4  >=  A_4  >=  V_4  >=  <(12)(34)>  >=  {e}\n");
    out.push_str("    Order:  24       12        4              2\n");
    out.push_str("    Factors:\n");
    out.push_str("      S_4 / A_4             ≅  Z/2Z\n");
    out.push_str("      A_4 / V_4             ≅  Z/3Z\n");
    out.push_str("      V_4 / <(12)(34)>      ≅  Z/2Z\n");
    out.push_str("      <(12)(34)> / {e}      ≅  Z/2Z\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  {}Series 2:{}\n", bold(""), ""));
    out.push_str("    S_4  >=  A_4  >=  V_4  >=  <(13)(24)>  >=  {e}\n");
    out.push_str("    Order:  24       12        4              2\n");
    out.push_str("    Factors:\n");
    out.push_str("      S_4 / A_4             ≅  Z/2Z\n");
    out.push_str("      A_4 / V_4             ≅  Z/3Z\n");
    out.push_str("      V_4 / <(13)(24)>      ≅  Z/2Z\n");
    out.push_str("      <(13)(24)> / {e}      ≅  Z/2Z\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  {}Series 3 (different path):{}\n", bold(""), ""));
    out.push_str("    S_4  >=  <(12),(34),(1234)>  >=  V_4  >=  <(12)(34)>  >=  {e}\n");
    out.push_str("    Order:  24                       8          4              2\n");
    out.push_str("    Factors:\n");
    out.push_str("      S_4 / D_4                ≅  Z/3Z  (index 3 subgroup)\n");
    out.push_str("      D_4 / V_4                ≅  Z/2Z\n");
    out.push_str("      V_4 / Z/2Z               ≅  Z/2Z\n");
    out.push_str("      Z/2Z / {e}               ≅  Z/2Z\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  {}Comparison:{}\n", bold(""), ""));
    out.push_str("  Series 1 factors: {Z/2Z, Z/3Z, Z/2Z, Z/2Z}\n");
    out.push_str("  Series 2 factors: {Z/2Z, Z/3Z, Z/2Z, Z/2Z}\n");
    out.push_str("  Series 3 factors: {Z/3Z, Z/2Z, Z/2Z, Z/2Z}\n");
    out.push_str(&format!("  {} Same multiset of factors in all three series!\n", green("✓")));

    out.push_str(&format!("  {} Jordan-Holder: composition factors are well-defined invariants of the group.\n", yellow("◆")));
    out.push_str(&format!("  {} The factors of S_4 are: {{Z/2Z, Z/2Z, Z/2Z, Z/3Z}} (three 2's and one 3).\n", yellow("◆")));
    out
}

fn cmd_nilpotent(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 2 {
        out.push_str(&format!("  {} Usage: nilpotent <p> <a>\n", red("✗")));
        return out;
    }
    let p = match args[0].parse::<usize>().ok().filter(|&v| is_prime(v as u64)) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} p must be prime.\n", red("✗"))); return out; }
    };
    let a = match args[1].parse::<usize>().ok().filter(|&v| v >= 1 && v <= 5) {
        Some(v) => v,
        None => { out.push_str(&format!("  {} a must be between 1 and 5.\n", red("✗"))); return out; }
    };
    let n = p.pow(a as u32);

    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Nilpotency of Z/p^aZ = Z/{}Z\x1b[0m\n", n));
    out.push_str(&format!("  {}Theorem:{} Every p-group is nilpotent.\n", bold(""), ""));
    out.push_str("  A group G is nilpotent of class c if the lower central series\n");
    out.push_str("  G = G_0 >= G_1 >= G_2 >= ... >= G_c = {e}  terminates.\n");
    out.push_str("  where G_{i+1} = [G, G_i].\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  G = Z/{}Z  (abelian, p-group with p={})\n", n, p));
    out.push_str("  Lower central series:\n");
    out.push_str(&format!("    G_0 = Z/{}Z  (order {})\n", n, n));
    out.push_str("    G_1 = [G, G_0] = [G, G] = {0}  (abelian group)\n");
    out.push_str(&format!("  Series: Z/{}Z  >=  {{0}}\n", n));
    out.push_str("  Nilpotency class: 1  (since G_1 = {e})\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  {}Why p-groups are nilpotent (general proof):{}\n", bold(""), ""));
    out.push_str("  Induction on a = log_p(|G|):\n");
    out.push_str("  Base: a=1, G = Z/pZ is abelian, nilpotent of class 1.\n");
    out.push_str("  Step: G has non-trivial center Z(G) (p-group theorem).\n");
    out.push_str("        G/Z(G) is a p-group of smaller order, hence nilpotent.\n");
    out.push_str("        Nilpotency lifts from G/Z(G) to G.\n");
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));

    out.push_str(&format!("  {}Central series (another view):{}\n", bold(""), ""));
    out.push_str("  Upper central series: {e} = Z_0 <= Z_1 <= Z_2 <= ... = G\n");
    out.push_str("  where Z_{i+1}/Z_i = Z(G/Z_i).\n\n");
    out.push_str(&format!("  For Z/{}Z (abelian):\n", n));
    out.push_str("    Z_0 = {0}\n");
    out.push_str("    Z_1 = Z(G/Z_0) = G  (G is abelian, Z(G) = G)\n");
    out.push_str("  Terminates in 1 step.\n");

    out.push_str(&format!("  {} Nilpotent groups = finite direct products of p-groups (for finite groups).\n", yellow("◆")));
    out.push_str(&format!("  {} Every abelian group is nilpotent of class <= 1.\n", yellow("◆")));
    out
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "derived" => {
            if let (Some(&t), Some(&n_s)) = (args.first(), args.get(1)) {
                state_set_str(state, "group_type", t);
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "group_n", n);
                }
            }
            cmd_derived(args)
        }
        "composition" => {
            if let (Some(&t), Some(&n_s)) = (args.first(), args.get(1)) {
                state_set_str(state, "group_type", t);
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "group_n", n);
                }
            }
            cmd_composition(args)
        }
        "solvable" => {
            if let Some(&n_s) = args.first() {
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "group_n", n);
                }
            }
            cmd_solvable(args)
        }
        "simple" => {
            if let Some(&n_s) = args.first() {
                if let Ok(n) = n_s.parse::<i64>() {
                    state_set_int(state, "group_n", n);
                }
            }
            cmd_simple(args)
        }
        "free_word" => cmd_free_word(args),
        "jordan_holder" => cmd_jordan_holder(),
        "nilpotent" => {
            if let (Some(&p_s), Some(&a_s)) = (args.first(), args.get(1)) {
                if let (Ok(p), Ok(a)) = (p_s.parse::<i64>(), a_s.parse::<i64>()) {
                    state_set_int(state, "group_n", p.pow(a as u32));
                }
            }
            cmd_nilpotent(args)
        }
        "demo" => {
            let gtype = state_get_str(state, "group_type").unwrap_or("Z").to_string();
            let n: usize = state_get_int(state, "group_n").unwrap_or(6) as usize;
            let mut out = String::new();
            out.push_str(&format!("=== Group Structure Demo: {} n={} ===\n", gtype, n));

            if gtype == "Z" {
                out.push_str(&format!("  G = Z/{}Z is abelian\n", n));
                out.push_str("  Derived series: G >= {0}\n");
                out.push_str("  G' = {0}: solvable, nilpotent of class 1\n");
                let facts = factor(n as u64);
                out.push_str("  Composition factors: ");
                let cf: Vec<String> = facts.iter()
                    .flat_map(|&(p, a)| std::iter::repeat(p).take(a as usize))
                    .map(|p| format!("Z/{}Z", p))
                    .collect();
                out.push_str(&cf.join(", "));
                out.push('\n');
            } else {
                out.push_str(&format!("  G = D_{} (order {})\n", n, 2*n));
                out.push_str("  Derived series: D_n >= <r^2> >= {e}\n");
                out.push_str("  D_n is solvable (but non-abelian for n >= 3)\n");
            }
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
    state_set_str(&mut s, "group_type", "Z");
    state_set_int(&mut s, "group_n", 12);
    s
}

fn visualize_svg(canvas: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let gtype = state_get_str(state, "group_type").unwrap_or("Z").to_string();
    let n: usize = state_get_int(state, "group_n").unwrap_or(12) as usize;

    canvas.text_bold(350.0, 30.0,
        &format!("Derived Series: {} n={}", gtype, n), 18.0, "#222", "middle");

    let series: Vec<(String, String, String)> = if gtype == "Z" {
        vec![
            (format!("G = Z/{}Z", n), format!("order {}", n), "#4a90d9".to_string()),
            ("G' = {0}".to_string(), "order 1".to_string(), "#e87040".to_string()),
        ]
    } else {
        let d1_size = {
            let g2 = 2 % n.max(1);
            if g2 == 0 { 1 } else { n / gcd(g2 as i64, n as i64) as usize }
        };
        vec![
            (format!("D_{}", n), format!("order {}", 2*n), "#4a90d9".to_string()),
            ("D_n' = <r^2>".to_string(), format!("order {}", d1_size), "#e87040".to_string()),
            ("{e}".to_string(), "order 1".to_string(), "#50b86c".to_string()),
        ]
    };

    let box_w = 200.0f64;
    let box_h = 56.0f64;
    let cx = 350.0f64;
    let start_y = 70.0f64;
    let gap = 90.0f64;

    for (i, (label, ord_str, color)) in series.iter().enumerate() {
        let y = start_y + i as f64 * gap;
        canvas.rect(cx - box_w/2.0, y, box_w, box_h, color, "#fff", 1.0);
        canvas.text_bold(cx, y + 22.0, label, 14.0, "white", "middle");
        canvas.text(cx, y + 40.0, ord_str, 12.0, "white", "middle");

        if i + 1 < series.len() {
            canvas.arrow(cx, y + box_h, cx, y + gap, "#555", 1.5);
            let next_ord: usize = if i == 0 && gtype == "Z" { n } else { 1 };
            if next_ord > 1 {
                canvas.text(cx + 20.0, y + box_h + gap/2.0 - 6.0,
                    &format!("factor: {}", next_ord), 11.0, "#555", "start");
            }
        }
    }

    let last_y = start_y + (series.len() as f64 - 1.0) * gap + box_h + 20.0;
    canvas.text(cx, last_y + 20.0,
        &format!("{} is SOLVABLE (derived series reaches {{e}})",
            if gtype == "Z" { format!("Z/{}Z", n) } else { format!("D_{}", n) }),
        13.0, "#27ae60", "middle");

    let facts = factor(n as u64);
    let cf: Vec<String> = facts.iter()
        .flat_map(|&(p, a)| std::iter::repeat(p).take(a as usize))
        .map(|p| format!("Z/{}Z", p))
        .collect();
    canvas.text(cx, last_y + 44.0,
        &format!("Composition factors: {}", cf.join(", ")), 12.0, "#555", "middle");
}

fn visualize_dot(graph: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let gtype = state_get_str(state, "group_type").unwrap_or("Z").to_string();
    let n: usize = state_get_int(state, "group_n").unwrap_or(12) as usize;

    let series: Vec<(&str, String)> = if gtype == "Z" {
        vec![
            ("G", format!("G = Z/{}Z\\n|G| = {}", n, n)),
            ("Gp", format!("G' = {{0}}\\n|G'| = 1")),
        ]
    } else {
        vec![
            ("G", format!("D_{}\\n|G| = {}", n, 2*n)),
            ("Gp", format!("D_n' = <r^2>")),
            ("Gpp", "{e}".to_string()),
        ]
    };

    let colors = ["#4a90d9", "#e87040", "#50b86c"];

    for (i, (id, label)) in series.iter().enumerate() {
        graph.node(id, &[
            ("label", label),
            ("shape", "box"),
            ("style", "filled"),
            ("fillcolor", colors[i % colors.len()]),
            ("fontcolor", "white"),
        ]);
        if i + 1 < series.len() {
            graph.edge(id, series[i+1].0, &[
                ("label", "contains"),
                ("color", "#aaaaaa"),
            ]);
        }
    }
}

fn visualize_tex(doc: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let gtype = state_get_str(state, "group_type").unwrap_or("Z").to_string();
    let n: usize = state_get_int(state, "group_n").unwrap_or(12) as usize;

    let series: Vec<(String, f64, f64)> = if gtype == "Z" {
        vec![
            (format!("$G = \\mathbb{{Z}}/{}\\mathbb{{Z}}$", n), 0.0, 4.0),
            ("$G' = \\{0\\}$".to_string(), 0.0, 2.0),
            ("$\\{e\\}$".to_string(), 0.0, 0.0),
        ]
    } else {
        vec![
            (format!("$D_{{{}}}$", n), 0.0, 4.0),
            ("$\\langle r^2 \\rangle$".to_string(), 0.0, 2.0),
            ("$\\{e\\}$".to_string(), 0.0, 0.0),
        ]
    };

    let ids: Vec<String> = (0..series.len()).map(|i| format!("G{}", i)).collect();
    for (i, (label, x, y)) in series.iter().enumerate() {
        doc.node(&ids[i], *x, *y, label, "draw,rectangle");
        if i + 1 < series.len() {
            doc.arrow(&ids[i], &ids[i+1], "", "-stealth");
        }
    }
}

fn visualize_ascii(canvas: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let gtype = state_get_str(state, "group_type").unwrap_or("Z").to_string();
    let n: usize = state_get_int(state, "group_n").unwrap_or(12) as usize;

    canvas.text_at(0, 0, &format!("Derived Series for {} n={}:", gtype, n));

    if gtype == "Z" {
        canvas.text_at(0, 2, &format!("  G_0 = Z/{}Z  (order {})", n, n));
        canvas.text_at(0, 3, "       |");
        canvas.text_at(0, 4, "       v  [G,G] = {0} (abelian)");
        canvas.text_at(0, 5, "  G_1 = {0}  (order 1)");
        canvas.text_at(0, 7, "  SOLVABLE: series terminates");
    } else {
        canvas.text_at(0, 2, &format!("  G_0 = D_{}  (order {})", n, 2*n));
        canvas.text_at(0, 3, "       |");
        canvas.text_at(0, 4, "       v  [D_n,D_n] = <r^2>");
        canvas.text_at(0, 5, "  G_1 = <r^2>  (cyclic)");
        canvas.text_at(0, 6, "       |");
        canvas.text_at(0, 7, "       v  [<r^2>,<r^2>] = {e}");
        canvas.text_at(0, 8, "  G_2 = {e}");
        canvas.text_at(0, 10, "  SOLVABLE: D_n is always solvable");
    }

    let facts = factor(n as u64);
    let cf: Vec<String> = facts.iter()
        .flat_map(|&(p, a)| std::iter::repeat(p).take(a as usize))
        .map(|p| format!("Z/{}Z", p))
        .collect();
    canvas.text_at(0, 12, &format!("  Composition factors: {}", cf.join(", ")));
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
                    let mut c = SvgCanvas::new(700.0, 480.0);
                    visualize_svg(&mut c, cmd, &args_ref, &state);
                    c.build()
                }
                OutputFormat::Dot => {
                    let mut gr = DotGraph::digraph("ch18");
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
                "Chapter 18",
                "Structure of Groups",
                "Derived series · Composition series · Solvability · Nilpotency",
            );
            print_info("Explore how groups decompose into simple and abelian pieces.");
            print!("{}", show_help());
            repl("ch18> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
