use common::*;

fn help_string() -> String {
    let mut h = String::new();
    h.push_str("\n  \x1b[1mCommands:\x1b[0m\n");
    h.push_str("    cayley <type> <n>       Cayley table: Z=ℤ/nℤ, U=(ℤ/nℤ)*, D=dihedral, S=symmetric, V=Klein\n");
    h.push_str("    order <g> <type> <n>   order of element g in group\n");
    h.push_str("    subgroups <type> <n>   find all subgroups\n");
    h.push_str("    is_abelian <type> <n>  check if group is abelian\n");
    h.push_str("    elements <type> <n>    list all elements and their orders\n");
    h.push_str("    cyclic_gen <type> <n>  find generators (elements whose order = |G|)\n");
    h.push_str("    center <type> <n>      compute the center Z(G)\n");
    h.push_str("    demo                   run a showcase of group operations\n");
    h.push_str("    help                   show this help\n");
    h.push_str("    quit                   exit\n");
    h
}

// ─── Group representations ────────────────────────────────────────────────────

struct Group {
    elements: Vec<String>,
    table: Vec<Vec<usize>>,
}

impl Group {
    fn order(&self) -> usize { self.elements.len() }

    fn identity_idx(&self) -> usize {
        for i in 0..self.order() {
            if (0..self.order()).all(|j| self.table[i][j] == j) {
                return i;
            }
        }
        0
    }

    fn mul(&self, i: usize, j: usize) -> usize { self.table[i][j] }

    fn inverse(&self, i: usize) -> usize {
        let e = self.identity_idx();
        (0..self.order()).find(|&j| self.mul(i, j) == e).unwrap_or(0)
    }

    fn element_order(&self, i: usize) -> usize {
        let e = self.identity_idx();
        let mut cur = i;
        for k in 1..=self.order() {
            if cur == e { return k; }
            cur = self.mul(cur, i);
        }
        self.order()
    }

    fn is_abelian(&self) -> bool {
        let n = self.order();
        for i in 0..n {
            for j in 0..n {
                if self.table[i][j] != self.table[j][i] { return false; }
            }
        }
        true
    }

    fn center(&self) -> Vec<usize> {
        let n = self.order();
        (0..n).filter(|&g| {
            (0..n).all(|h| self.mul(g, h) == self.mul(h, g))
        }).collect()
    }

    fn is_subgroup(&self, subset: &[usize]) -> bool {
        let e = self.identity_idx();
        if !subset.contains(&e) { return false; }
        for &a in subset {
            let inv_a = self.inverse(a);
            if !subset.contains(&inv_a) { return false; }
            for &b in subset {
                if !subset.contains(&self.mul(a, b)) { return false; }
            }
        }
        true
    }

    fn subgroups(&self) -> Vec<Vec<usize>> {
        let n = self.order();
        let mut result = Vec::new();
        for size in 1..=n {
            if n % size != 0 { continue; }
            for_each_subset(n, size, &mut |s| {
                let subset: Vec<usize> = s.to_vec();
                if self.is_subgroup(&subset) {
                    if !result.contains(&subset) {
                        result.push(subset);
                    }
                }
            });
        }
        result
    }

    fn generators(&self) -> Vec<usize> {
        let ord = self.order();
        (0..ord).filter(|&g| self.element_order(g) == ord).collect()
    }
}

fn for_each_subset(n: usize, size: usize, f: &mut impl FnMut(&[usize])) {
    let mut combo = (0..size).collect::<Vec<_>>();
    loop {
        f(&combo);
        let mut i = size;
        loop {
            if i == 0 { return; }
            i -= 1;
            if combo[i] < n - size + i { break; }
        }
        combo[i] += 1;
        for j in i + 1..size {
            combo[j] = combo[j - 1] + 1;
        }
    }
}

// ─── Group constructors ───────────────────────────────────────────────────────

fn build_zn(n: usize) -> Group {
    let elements: Vec<String> = (0..n).map(|i| i.to_string()).collect();
    let table: Vec<Vec<usize>> = (0..n).map(|i| (0..n).map(|j| (i + j) % n).collect()).collect();
    Group { elements, table }
}

fn build_un(n: usize) -> Group {
    let units: Vec<usize> = (1..n).filter(|&x| gcd(x as i64, n as i64) == 1).collect();
    let m = units.len();
    let table: Vec<Vec<usize>> = (0..m).map(|i|
        (0..m).map(|j| {
            let prod = (units[i] * units[j]) % n;
            units.iter().position(|&u| u == prod).unwrap()
        }).collect()
    ).collect();
    let elements = units.iter().map(|x| x.to_string()).collect();
    Group { elements, table }
}

fn build_dn(n: usize) -> Group {
    let ord = 2 * n;
    let elements: Vec<String> = {
        let mut v: Vec<String> = (0..n).map(|k| if k == 0 { "e".to_string() } else { format!("r{}", k) }).collect();
        v.extend((0..n).map(|k| if k == 0 { "s".to_string() } else { format!("sr{}", k) }));
        v
    };
    let mut table = vec![vec![0usize; ord]; ord];
    for i in 0..ord {
        for j in 0..ord {
            table[i][j] = dn_mul(i, j, n);
        }
    }
    Group { elements, table }
}

fn dn_mul(i: usize, j: usize, n: usize) -> usize {
    let i_is_refl = i >= n;
    let j_is_refl = j >= n;
    let ia = if i_is_refl { i - n } else { i };
    let ja = if j_is_refl { j - n } else { j };
    match (i_is_refl, j_is_refl) {
        (false, false) => (ia + ja) % n,
        (false, true)  => n + (ja + n - ia) % n,
        (true,  false) => n + (ia + ja) % n,
        (true,  true)  => (ja + n - ia) % n,
    }
}

fn build_s3() -> Group {
    let perms: Vec<Vec<usize>> = vec![
        vec![0,1,2], // e
        vec![0,2,1], // (12)
        vec![1,0,2], // (01)
        vec![1,2,0], // (012)
        vec![2,0,1], // (021)
        vec![2,1,0], // (02)
    ];
    let labels = vec!["e","(12)","(01)","(012)","(021)","(02)"];
    let elements: Vec<String> = labels.iter().map(|s| s.to_string()).collect();
    let n = perms.len();
    let mut table = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            let comp: Vec<usize> = (0..3).map(|k| perms[i][perms[j][k]]).collect();
            let idx = perms.iter().position(|p| *p == comp).unwrap();
            table[i][j] = idx;
        }
    }
    Group { elements, table }
}

fn build_s4() -> Group {
    let mut perms: Vec<Vec<usize>> = Vec::new();
    let items = [0usize, 1, 2, 3];
    for_each_permutation(&items, &mut |p| perms.push(p.to_vec()));

    let elements: Vec<String> = perms.iter().map(|p| {
        format!("[{}]", p.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""))
    }).collect();
    let n = perms.len();
    let mut table = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            let comp: Vec<usize> = (0..4).map(|k| perms[i][perms[j][k]]).collect();
            let idx = perms.iter().position(|p| *p == comp).unwrap();
            table[i][j] = idx;
        }
    }
    Group { elements, table }
}

fn for_each_permutation(items: &[usize], f: &mut impl FnMut(&[usize])) {
    let mut perm = items.to_vec();
    perm.sort();
    loop {
        f(&perm);
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

fn build_v4() -> Group {
    let elements = vec!["e".to_string(), "a".to_string(), "b".to_string(), "c".to_string()];
    let table = vec![
        vec![0, 1, 2, 3],
        vec![1, 0, 3, 2],
        vec![2, 3, 0, 1],
        vec![3, 2, 1, 0],
    ];
    Group { elements, table }
}

fn parse_group(gtype: &str, n: usize) -> Option<Group> {
    match gtype {
        "Z" => Some(build_zn(n)),
        "U" => Some(build_un(n)),
        "D" => {
            if n < 2 { return None; }
            if n > 6 { return None; }
            Some(build_dn(n))
        }
        "S" => match n {
            3 => Some(build_s3()),
            4 => Some(build_s4()),
            _ => None,
        }
        "V" => Some(build_v4()),
        _ => None,
    }
}

fn group_label(gtype: &str, n: usize) -> String {
    match gtype {
        "Z" => format!("ℤ/{}ℤ", n),
        "U" => format!("(ℤ/{}ℤ)*", n),
        "D" => format!("D_{} (dihedral, order {})", n, 2*n),
        "S" => format!("S_{} (symmetric, order {})", n, (1..=n).product::<usize>()),
        "V" => "Klein four-group V₄".to_string(),
        _ => gtype.to_string(),
    }
}

// String-returning Cayley table formatter
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

// ─── Commands ─────────────────────────────────────────────────────────────────

fn cmd_cayley(args: &[&str]) -> String {
    let mut out = String::new();
    let gtype = match args.first() {
        Some(&s) => s,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: cayley <type> <n>")); return out; }
    };
    let n = if gtype == "V" { 4 } else {
        match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: cayley <type> <n>")); return out; }
        }
    };
    let g = match parse_group(gtype, n) {
        Some(g) => g,
        None => {
            out.push_str(&format!("  {} Unknown group type '{}' or invalid n. Use: Z, U, D, S, V\n", red("✗"), gtype));
            return out;
        }
    };
    if g.order() > 24 {
        out.push_str(&format!("  {} Group has {} elements — too large to display Cayley table.\n", red("✗"), g.order()));
        out.push_str(&format!("  {} {}\n", yellow("◆"), "Use 'elements' to see orders, or 'subgroups' for structure."));
        return out;
    }
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Cayley Table for {}\x1b[0m\n", group_label(gtype, n)));
    out.push_str(&fmt_cayley(&g.elements, &g.table));
    out.push_str(&format!("  {} {}\n", yellow("◆"), &format!("Group order: |G| = {}", g.order())));
    if g.is_abelian() {
        out.push_str(&format!("  {} {}\n", green("✓"), "Group is abelian (table is symmetric)."));
    } else {
        out.push_str(&format!("  {} {}\n", yellow("◆"), "Group is non-abelian (table is not symmetric)."));
    }
    out
}

fn cmd_order(args: &[&str]) -> String {
    let mut out = String::new();
    if args.len() < 3 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Usage: order <element> <type> <n>"));
        return out;
    }
    let g_str = args[0];
    let gtype = args[1];
    let n = match args[2].parse::<usize>().ok() {
        Some(v) => v,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "n must be a positive integer.")); return out; }
    };
    let g = match parse_group(gtype, n) {
        Some(g) => g,
        None => { out.push_str(&format!("  {} Unknown group or invalid n.\n", red("✗"))); return out; }
    };
    let idx = match g.elements.iter().position(|e| e == g_str) {
        Some(i) => i,
        None => {
            out.push_str(&format!("  {} Element '{}' not found in {}.\n", red("✗"), g_str, group_label(gtype, n)));
            out.push_str(&format!("  {}  Available elements:\n", dim("")));
            out.push_str(&format!("  {}  {}\n", dim(""), g.elements.join(", ")));
            return out;
        }
    };
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Order of {} in {}\x1b[0m\n", g_str, group_label(gtype, n)));
    let ord = g.element_order(idx);
    let e = g.identity_idx();
    out.push_str(&format!("  Computing powers of {}:\n", bold(g_str)));
    let mut cur = idx;
    for k in 1..=ord {
        out.push_str(&format!("    {}^{} = {}{}\n", g_str, k, g.elements[cur],
            if cur == e { "  ← identity" } else { "" }));
        if cur == e {
            out.push_str(&format!("  {} {} {}\n", cyan(&format!("ord({})", g_str)), dim("="), green(&format!("{}", k))));
            break;
        }
        cur = g.mul(cur, idx);
    }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "By Lagrange's theorem, ord(g) divides |G|."));
    out
}

fn cmd_subgroups(args: &[&str]) -> String {
    let mut out = String::new();
    let gtype = match args.first() {
        Some(&s) => s,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: subgroups <type> <n>")); return out; }
    };
    let n = if gtype == "V" { 4 } else {
        match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: subgroups <type> <n>")); return out; }
        }
    };
    let g = match parse_group(gtype, n) {
        Some(g) => g,
        None => { out.push_str(&format!("  {} Unknown group or invalid n.\n", red("✗"))); return out; }
    };
    if g.order() > 24 {
        out.push_str(&format!("  {} {}\n", red("✗"), "Subgroup search limited to groups of order ≤ 24."));
        return out;
    }
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Subgroups of {}\x1b[0m\n", group_label(gtype, n)));
    let subs = g.subgroups();
    out.push_str(&format!("  Found {} subgroup(s):\n", subs.len()));
    for (i, sub) in subs.iter().enumerate() {
        let elems: Vec<&str> = sub.iter().map(|&k| g.elements[k].as_str()).collect();
        out.push_str(&format!("  H_{}: {{ {} }}  (order {})\n", i + 1, elems.join(", "), sub.len()));
    }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "By Lagrange's theorem, each subgroup order divides |G|."));
    out
}

fn cmd_is_abelian(args: &[&str]) -> String {
    let mut out = String::new();
    let gtype = match args.first() {
        Some(&s) => s,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: is_abelian <type> <n>")); return out; }
    };
    let n = if gtype == "V" { 4 } else {
        match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: is_abelian <type> <n>")); return out; }
        }
    };
    let g = match parse_group(gtype, n) {
        Some(g) => g,
        None => { out.push_str(&format!("  {} Unknown group or invalid n.\n", red("✗"))); return out; }
    };
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Is {} abelian?\x1b[0m\n", group_label(gtype, n)));
    if g.is_abelian() {
        out.push_str(&format!("  {} {}\n", green("✓"), &format!("{} is abelian: ab = ba for all a,b.", group_label(gtype, n))));
    } else {
        let ord = g.order();
        'outer: for i in 0..ord {
            for j in 0..ord {
                if g.mul(i, j) != g.mul(j, i) {
                    out.push_str(&format!("  Counterexample: {} · {} = {} but {} · {} = {}\n",
                        g.elements[i], g.elements[j], g.elements[g.mul(i,j)],
                        g.elements[j], g.elements[i], g.elements[g.mul(j,i)]));
                    break 'outer;
                }
            }
        }
        out.push_str(&format!("  {} {}\n", red("✗"), &format!("{} is non-abelian.", group_label(gtype, n))));
    }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Abelian groups have symmetric Cayley tables."));
    out
}

fn cmd_elements(args: &[&str]) -> String {
    let mut out = String::new();
    let gtype = match args.first() {
        Some(&s) => s,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: elements <type> <n>")); return out; }
    };
    let n = if gtype == "V" { 4 } else {
        match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: elements <type> <n>")); return out; }
        }
    };
    let g = match parse_group(gtype, n) {
        Some(g) => g,
        None => { out.push_str(&format!("  {} Unknown group or invalid n.\n", red("✗"))); return out; }
    };
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Elements of {} (|G| = {})\x1b[0m\n", group_label(gtype, n), g.order()));
    out.push_str(&format!("  {:>12}  {:>8}\n", bold("Element"), bold("Order")));
    out.push_str(&format!("  {}\n", dim(&"─".repeat(60))));
    for i in 0..g.order() {
        let ord = g.element_order(i);
        out.push_str(&format!("  {:>12}  {:>8}\n", g.elements[i], ord));
    }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Element order divides group order (Lagrange)."));
    out
}

fn cmd_cyclic_gen(args: &[&str]) -> String {
    let mut out = String::new();
    let gtype = match args.first() {
        Some(&s) => s,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: cyclic_gen <type> <n>")); return out; }
    };
    let n = if gtype == "V" { 4 } else {
        match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: cyclic_gen <type> <n>")); return out; }
        }
    };
    let g = match parse_group(gtype, n) {
        Some(g) => g,
        None => { out.push_str(&format!("  {} Unknown group or invalid n.\n", red("✗"))); return out; }
    };
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Generators of {}\x1b[0m\n", group_label(gtype, n)));
    let gens = g.generators();
    if gens.is_empty() {
        out.push_str(&format!("  {} {}\n", yellow("◆"), "No single generator exists — group is not cyclic."));
    } else {
        let gen_strs: Vec<&str> = gens.iter().map(|&i| g.elements[i].as_str()).collect();
        out.push_str(&format!("  Generators (elements of order |G| = {}):\n", g.order()));
        out.push_str(&format!("    {}\n", gen_strs.join(", ")));
        out.push_str(&format!("  {} {}\n", green("✓"), &format!("Group is cyclic with {} generator(s).", gens.len())));
    }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "A group is cyclic iff it has an element of order equal to |G|."));
    out
}

fn cmd_center(args: &[&str]) -> String {
    let mut out = String::new();
    let gtype = match args.first() {
        Some(&s) => s,
        None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: center <type> <n>")); return out; }
    };
    let n = if gtype == "V" { 4 } else {
        match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => { out.push_str(&format!("  {} {}\n", red("✗"), "Usage: center <type> <n>")); return out; }
        }
    };
    let g = match parse_group(gtype, n) {
        Some(g) => g,
        None => { out.push_str(&format!("  {} Unknown group or invalid n.\n", red("✗"))); return out; }
    };
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Center Z(G) of {}\x1b[0m\n", group_label(gtype, n)));
    let center = g.center();
    let center_strs: Vec<&str> = center.iter().map(|&i| g.elements[i].as_str()).collect();
    out.push_str(&format!("  Z(G) = {{ {} }}  (order {})\n", center_strs.join(", "), center.len()));
    if center.len() == g.order() {
        out.push_str(&format!("  {} {}\n", green("✓"), "Z(G) = G: group is abelian."));
    } else if center.len() == 1 {
        out.push_str(&format!("  {} {}\n", yellow("◆"), "Z(G) = {e}: group has trivial center."));
    } else {
        out.push_str(&format!("  {} {}\n", yellow("◆"), &format!("[G : Z(G)] = {}", g.order() / center.len())));
    }
    out.push_str(&format!("  {} {}\n", yellow("◆"), "Z(G) = {g ∈ G : gx = xg for all x ∈ G} is always a normal subgroup."));
    out
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    // Helper to parse group type and n and save to state
    let save_group_params = |args: &[&str], state: &mut StateMap, is_v: bool| {
        let gtype = args.first().copied().unwrap_or("Z");
        state_set_str(state, "group_type", gtype);
        if !is_v {
            if let Some(n) = args.get(1).and_then(|s| s.parse::<i64>().ok()) {
                state_set_int(state, "group_n", n);
            }
        }
    };

    match cmd {
        "cayley" => {
            let is_v = args.first().map(|&s| s == "V").unwrap_or(false);
            save_group_params(args, state, is_v);
            cmd_cayley(args)
        }
        "order" => {
            // args: <element> <type> <n>
            if args.len() >= 3 {
                state_set_str(state, "group_type", args[1]);
                if let Ok(n) = args[2].parse::<i64>() {
                    state_set_int(state, "group_n", n);
                }
            }
            cmd_order(args)
        }
        "subgroups" => {
            let is_v = args.first().map(|&s| s == "V").unwrap_or(false);
            save_group_params(args, state, is_v);
            cmd_subgroups(args)
        }
        "is_abelian" => {
            let is_v = args.first().map(|&s| s == "V").unwrap_or(false);
            save_group_params(args, state, is_v);
            cmd_is_abelian(args)
        }
        "elements" => {
            let is_v = args.first().map(|&s| s == "V").unwrap_or(false);
            save_group_params(args, state, is_v);
            cmd_elements(args)
        }
        "cyclic_gen" => {
            let is_v = args.first().map(|&s| s == "V").unwrap_or(false);
            save_group_params(args, state, is_v);
            cmd_cyclic_gen(args)
        }
        "center" => {
            let is_v = args.first().map(|&s| s == "V").unwrap_or(false);
            save_group_params(args, state, is_v);
            cmd_center(args)
        }
        "demo" => {
            let mut out = String::new();
            out.push_str("\n  === Demo: Groups and Subgroups ===\n\n");
            let z6 = build_zn(6);
            out.push_str(&format!("  Z/6Z: order {}, abelian={}\n", z6.order(), z6.is_abelian()));
            out.push_str("  Element orders in Z/6Z:\n");
            for i in 0..z6.order() {
                out.push_str(&format!("    ord({}) = {}\n", z6.elements[i], z6.element_order(i)));
            }
            let s3 = build_s3();
            out.push_str(&format!("\n  S_3: order {}, abelian={}\n", s3.order(), s3.is_abelian()));
            let center = s3.center();
            let c_strs: Vec<&str> = center.iter().map(|&i| s3.elements[i].as_str()).collect();
            out.push_str(&format!("  Z(S_3) = {{ {} }}\n", c_strs.join(", ")));
            let _ = state;
            out
        }
        "help" | "h" => help_string(),
        _ => format!("  \x1b[31m✗\x1b[0m Unknown command '{}'. Type 'help' for commands.\n", cmd),
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
    state_set_int(&mut s, "group_n", 6);
    s
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let gtype = state_get_str(state, "group_type").unwrap_or("Z");
    let n = state_get_int(state, "group_n").unwrap_or(6) as usize;
    let label = group_label(gtype, n);

    c.title(&format!("Ch13: Cayley Table — {}", label));
    c.subtitle("Group multiplication table (colored by row)", 42.0);

    // Build the group and render its Cayley table as SVG
    let g = match gtype {
        "Z" => build_zn(n),
        "D" if n >= 2 && n <= 6 => build_dn(n),
        "V" => build_v4(),
        "S" if n == 3 => build_s3(),
        _ => build_zn(n.min(8)),
    };

    if g.order() > 12 {
        c.text(350.0, 200.0, &format!("Group |G|={} — too large for SVG Cayley table", g.order()),
               14.0, colors::GREY, "middle");
        c.text(350.0, 230.0, "Use 'cayley' command in REPL for text output.", 12.0, colors::GREY, "middle");
        return;
    }

    let ord = g.order();
    let cw = (600.0 / (ord + 1) as f64).min(45.0);
    let ch = cw;
    let x0 = 40.0; let y0 = 60.0;

    // Header row
    c.rect(x0, y0, cw, ch, colors::HEADER_FILL, colors::GREY, 0.5);
    c.text_bold(x0 + cw/2.0, y0 + ch/2.0, "·", 12.0, colors::DARK, "middle");
    for j in 0..ord {
        let x = x0 + (j+1) as f64 * cw;
        c.rect(x, y0, cw, ch, colors::HEADER_FILL, colors::GREY, 0.5);
        c.text_bold(x + cw/2.0, y0 + ch/2.0, &g.elements[j], 11.0, colors::DARK, "middle");
    }

    // Table rows
    let palette = [colors::ROW_NORM, colors::ROW_ALT];
    for i in 0..ord {
        let y = y0 + (i+1) as f64 * ch;
        c.rect(x0, y, cw, ch, colors::HEADER_FILL, colors::GREY, 0.5);
        c.text_bold(x0 + cw/2.0, y + ch/2.0, &g.elements[i], 11.0, colors::DARK, "middle");
        for j in 0..ord {
            let x = x0 + (j+1) as f64 * cw;
            let prod = g.mul(i, j);
            let fill = palette[prod % 2];
            c.rect(x, y, cw, ch, fill, colors::GREY, 0.5);
            c.text(x + cw/2.0, y + ch/2.0, &g.elements[prod], 10.0, colors::DARK, "middle");
        }
    }

    let info_y = y0 + (ord + 2) as f64 * ch;
    c.text(x0, info_y, &format!("|G| = {}   abelian: {}", ord, g.is_abelian()),
           12.0, colors::GREY, "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let gtype = state_get_str(state, "group_type").unwrap_or("Z");
    let n = state_get_int(state, "group_n").unwrap_or(6) as usize;
    g.node_default("shape", "circle");
    g.node_default("style", "filled");
    g.node_default("fillcolor", "lightyellow");
    let grp = match gtype {
        "Z" => build_zn(n),
        "V" => build_v4(),
        "S" if n == 3 => build_s3(),
        _ => build_zn(n.min(6)),
    };
    let subs = if grp.order() <= 12 { grp.subgroups() } else { vec![] };
    g.node("G", &[("label", &format!("G={}", group_label(gtype, n))), ("fillcolor", "lightblue")]);
    for (i, sub) in subs.iter().enumerate() {
        let elems: Vec<&str> = sub.iter().map(|&k| grp.elements[k].as_str()).collect();
        let lbl = format!("H{}={{{}}}", i+1, elems.join(","));
        g.node(&format!("H{}", i), &[("label", &lbl)]);
        g.edge("G", &format!("H{}", i), &[("label", &format!("|{}|", sub.len()))]);
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let gtype = state_get_str(state, "group_type").unwrap_or("Z");
    let n = state_get_int(state, "group_n").unwrap_or(6) as usize;
    t.use_library("matrix,positioning");
    let grp = match gtype {
        "Z" => build_zn(n.min(6)),
        "V" => build_v4(),
        _ => build_zn(n.min(6)),
    };
    let ord = grp.order().min(6);
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut header = vec!["$\\cdot$".to_string()];
    for j in 0..ord { header.push(format!("${}$", grp.elements[j])); }
    rows.push(header);
    for i in 0..ord {
        let mut row = vec![format!("${}$", grp.elements[i])];
        for j in 0..ord {
            row.push(format!("${}$", grp.elements[grp.mul(i,j)]));
        }
        rows.push(row);
    }
    t.matrix_table(&rows, 0.0, 0.0);
    t.raw(&format!("  \\node[below=1cm of mat] {{$|G| = {}$, abelian: {}}};",
        grp.order(), grp.is_abelian()));
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let gtype = state_get_str(state, "group_type").unwrap_or("Z");
    let n = state_get_int(state, "group_n").unwrap_or(6) as usize;

    a.text_at(2, 1, &format!("Ch13: Cayley Table for {}", group_label(gtype, n)));
    a.text_at(2, 2, "──────────────────────────────────────────────");

    let g = match gtype {
        "Z" => build_zn(n.min(8)),
        "V" => build_v4(),
        _ => build_zn(n.min(8)),
    };

    let ord = g.order().min(8);
    // Header
    let mut header = String::from("  · | ");
    for j in 0..ord { header.push_str(&format!("{:2} ", g.elements[j])); }
    a.text_at(2, 4, &header);
    a.hline(2, 5, (4 + 3 * ord) as i32, '-');
    for i in 0..ord.min(10) {
        let mut row = format!("{:3} | ", g.elements[i]);
        for j in 0..ord {
            row.push_str(&format!("{:2} ", g.elements[g.mul(i, j)]));
        }
        a.text_at(2, (6 + i) as i32, &row);
    }
    a.text_at(2, (7 + ord) as i32, &format!("|G| = {}  abelian: {}", g.order(), g.is_abelian()));
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
                    let mut g = DotGraph::digraph("ch13");
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
            print_banner("Chapter 13", "Groups and Subgroups",
                "Cayley tables · Orders · Subgroups · Dihedral & Symmetric groups");
            print_info("Explore concrete groups: Z/nZ, U(n), dihedral D_n, symmetric S_n, Klein V4.");
            print!("{}", help_string());
            repl("ch13> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
