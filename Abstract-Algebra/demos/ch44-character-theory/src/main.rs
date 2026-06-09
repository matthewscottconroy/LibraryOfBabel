use common::*;

fn show_help() -> String {
    let mut out = String::new();
    let rows = &[
        ("table <type> <n>",              "print character table: Z, S2, S3, S4, D3, D4, V4"),
        ("orthogonality <type> <n>",      "verify row/column orthogonality relations"),
        ("inner_product <chi1> / <chi2> <type> <n>", "inner product of two characters"),
        ("detect_abelian <type> <n>",     "check if G is abelian from characters"),
        ("burnside_paqb <p> <a> <q> <b>", "Burnside p^a*q^b solvability"),
        ("conjugacy_classes <type> <n>",  "list conjugacy classes"),
        ("class_functions <type> <n>",    "irreducible characters as orthonormal basis"),
        ("demo",                          "showcase: character tables and orthogonality"),
        ("help",                          "show this help"),
        ("quit",                          "exit"),
    ];
    let max = rows.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Commands\x1b[0m\n"));
    for (cmd, desc) in rows {
        let width = max + 2 + 9;
        out.push_str(&format!("  \x1b[36m{:<width$}\x1b[0m {}\n", cmd, desc, width = width));
    }
    out.push('\n');
    out
}

// ── Local string-accumulating helpers ────────────────────────────────────────

fn s_section(out: &mut String, title: &str) {
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ {}\x1b[0m\n", title));
}
fn s_result(out: &mut String, label: &str, value: &str) {
    out.push_str(&format!("  \x1b[36m{}\x1b[0m \x1b[2m=\x1b[0m \x1b[32m{}\x1b[0m\n", label, value));
}
fn s_info(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[2m{}\x1b[0m\n", text));
}
fn s_note(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[33m◆\x1b[0m {}\n", text));
}
fn s_err(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[31m✗\x1b[0m {}\n", text));
}
fn s_ok(out: &mut String, text: &str) {
    out.push_str(&format!("  \x1b[32m✓\x1b[0m {}\n", text));
}

// ── Group data structures ─────────────────────────────────────────────────────

struct GroupData {
    name: String,
    order: usize,
    class_names: Vec<String>,
    class_sizes: Vec<usize>,
    char_table: Vec<Vec<(f64, f64)>>,
}

const PI: f64 = std::f64::consts::PI;

fn group_data(kind: &str, n: usize) -> Option<GroupData> {
    match kind {
        "Z" => {
            let class_names: Vec<String> = (0..n).map(|j| j.to_string()).collect();
            let class_sizes = vec![1; n];
            let mut char_table = Vec::new();
            for k in 0..n {
                let row: Vec<(f64, f64)> = (0..n).map(|j| {
                    let angle = 2.0 * PI * (k * j) as f64 / n as f64;
                    ((angle.cos() * 1000.0).round() / 1000.0,
                     (angle.sin() * 1000.0).round() / 1000.0)
                }).collect();
                char_table.push(row);
            }
            Some(GroupData {
                name: format!("ℤ/{}ℤ", n),
                order: n,
                class_names,
                class_sizes,
                char_table,
            })
        },
        "S2" => {
            Some(GroupData {
                name: "S₂ ≅ ℤ/2ℤ".to_string(),
                order: 2,
                class_names: vec!["e".to_string(), "(12)".to_string()],
                class_sizes: vec![1, 1],
                char_table: vec![
                    vec![(1.0,0.0), (1.0,0.0)],
                    vec![(1.0,0.0), (-1.0,0.0)],
                ],
            })
        },
        "S3" => {
            Some(GroupData {
                name: "S₃".to_string(),
                order: 6,
                class_names: vec!["e".to_string(), "(12)".to_string(), "(123)".to_string()],
                class_sizes: vec![1, 3, 2],
                char_table: vec![
                    vec![(1.0,0.0), (1.0,0.0),  (1.0,0.0)],
                    vec![(1.0,0.0), (-1.0,0.0), (1.0,0.0)],
                    vec![(2.0,0.0), (0.0,0.0),  (-1.0,0.0)],
                ],
            })
        },
        "S4" => {
            Some(GroupData {
                name: "S₄".to_string(),
                order: 24,
                class_names: vec![
                    "e".to_string(), "(12)".to_string(), "(12)(34)".to_string(),
                    "(123)".to_string(), "(1234)".to_string()
                ],
                class_sizes: vec![1, 6, 3, 8, 6],
                char_table: vec![
                    vec![(1.0,0.0),(1.0,0.0),(1.0,0.0),(1.0,0.0),(1.0,0.0)],
                    vec![(1.0,0.0),(-1.0,0.0),(1.0,0.0),(1.0,0.0),(-1.0,0.0)],
                    vec![(2.0,0.0),(0.0,0.0),(2.0,0.0),(-1.0,0.0),(0.0,0.0)],
                    vec![(3.0,0.0),(1.0,0.0),(-1.0,0.0),(0.0,0.0),(-1.0,0.0)],
                    vec![(3.0,0.0),(-1.0,0.0),(-1.0,0.0),(0.0,0.0),(1.0,0.0)],
                ],
            })
        },
        "D3" => {
            let mut g = group_data("S3", 3)?;
            g.name = "D₃ ≅ S₃".to_string();
            g.class_names = vec!["e".to_string(), "r".to_string(), "s".to_string()];
            Some(g)
        },
        "D4" => {
            Some(GroupData {
                name: "D₄".to_string(),
                order: 8,
                class_names: vec![
                    "e".to_string(), "r²".to_string(), "r,r³".to_string(),
                    "s,sr²".to_string(), "sr,sr³".to_string()
                ],
                class_sizes: vec![1, 1, 2, 2, 2],
                char_table: vec![
                    vec![(1.0,0.0),(1.0,0.0),(1.0,0.0),(1.0,0.0),(1.0,0.0)],
                    vec![(1.0,0.0),(1.0,0.0),(1.0,0.0),(-1.0,0.0),(-1.0,0.0)],
                    vec![(1.0,0.0),(1.0,0.0),(-1.0,0.0),(1.0,0.0),(-1.0,0.0)],
                    vec![(1.0,0.0),(1.0,0.0),(-1.0,0.0),(-1.0,0.0),(1.0,0.0)],
                    vec![(2.0,0.0),(-2.0,0.0),(0.0,0.0),(0.0,0.0),(0.0,0.0)],
                ],
            })
        },
        "V4" => {
            Some(GroupData {
                name: "V₄ = ℤ/2ℤ × ℤ/2ℤ".to_string(),
                order: 4,
                class_names: vec!["e".to_string(), "a".to_string(), "b".to_string(), "ab".to_string()],
                class_sizes: vec![1, 1, 1, 1],
                char_table: vec![
                    vec![(1.0,0.0),(1.0,0.0),(1.0,0.0),(1.0,0.0)],
                    vec![(1.0,0.0),(1.0,0.0),(-1.0,0.0),(-1.0,0.0)],
                    vec![(1.0,0.0),(-1.0,0.0),(1.0,0.0),(-1.0,0.0)],
                    vec![(1.0,0.0),(-1.0,0.0),(-1.0,0.0),(1.0,0.0)],
                ],
            })
        },
        _ => None,
    }
}

fn fmt_char(c: (f64, f64)) -> String {
    let re = (c.0 * 1000.0).round() / 1000.0;
    let im = (c.1 * 1000.0).round() / 1000.0;
    if im.abs() < 0.001 {
        format!("{:>7.3}", re)
    } else if re.abs() < 0.001 {
        format!("{:>7.3}i", im)
    } else {
        format!("{:>5.2}{:+.2}i", re, im)
    }
}

// ── Table command ─────────────────────────────────────────────────────────────

fn cmd_table(args: &[&str]) -> String {
    let mut out = String::new();
    let kind = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Type needed"); return out; } };
    let n = if matches!(kind, "S2"|"S3"|"S4"|"D3"|"D4"|"V4") { 0 }
    else { match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out } };

    let gd = match group_data(kind, n) {
        Some(g) => g,
        None => { s_err(&mut out, &format!("Unknown group type '{}'. Use Z, S2, S3, S4, D3, D4, V4", kind)); return out; }
    };

    s_section(&mut out, &format!("Character Table of {}", gd.name));
    out.push_str(&format!("  |G| = {},  {} conjugacy classes\n", gd.order, gd.class_names.len()));
    out.push('\n');

    out.push_str(&format!("  {:>10}", "χ \\ class"));
    for name in &gd.class_names { out.push_str(&format!("  {:>10}", name)); }
    out.push('\n');
    out.push_str(&format!("  {:>10}", ""));
    for _ in &gd.class_names { out.push_str(&format!("  {:>10}", "----------")); }
    out.push('\n');

    out.push_str(&format!("  {:>10}", "|class|"));
    for &sz in &gd.class_sizes { out.push_str(&format!("  {:>10}", sz)); }
    out.push('\n');
    out.push_str(&format!("  {:>10}", ""));
    for _ in &gd.class_names { out.push_str(&format!("  {:>10}", "----------")); }
    out.push('\n');

    for (i, row) in gd.char_table.iter().enumerate() {
        out.push_str(&format!("  χ_{:<7}", i+1));
        for &v in row { out.push_str(&format!("  {}", fmt_char(v))); }
        out.push('\n');
    }
    out.push('\n');
    let sum_sq: usize = gd.char_table.iter().map(|r| (r[0].0.round() as usize).pow(2)).sum();
    s_result(&mut out, "Σ (dim)²", &format!("{} = |G| = {} {}", sum_sq, gd.order, if sum_sq == gd.order { "✓" } else { "✗" }));
    out
}

// ── Orthogonality ─────────────────────────────────────────────────────────────

fn cmd_orthogonality(args: &[&str]) -> String {
    let mut out = String::new();
    let kind = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Type needed"); return out; } };
    let n = if matches!(kind, "S2"|"S3"|"S4"|"D3"|"D4"|"V4") { 0 }
    else { match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out } };

    let gd = match group_data(kind, n) {
        Some(g) => g,
        None => { s_err(&mut out, "Unknown group"); return out; }
    };

    s_section(&mut out, &format!("Orthogonality Relations for {}", gd.name));
    out.push('\n');
    out.push_str("  Row orthogonality: (1/|G|) Σ_C |C|·χᵢ(C)·χⱼ(C)̄ = δᵢⱼ\n");
    out.push_str("  Column orthogonality: Σᵢ χᵢ(C)·χᵢ(D)̄ = |G|/|C| · δ_CD\n");
    out.push('\n');

    let g_order = gd.order as f64;
    let n_irreps = gd.char_table.len();
    let n_classes = gd.class_names.len();

    out.push_str("  Row orthogonality check:\n");
    let mut all_ok = true;
    for i in 0..n_irreps {
        for j in i..n_irreps {
            let mut sum = (0.0f64, 0.0f64);
            for c in 0..n_classes {
                let sz = gd.class_sizes[c] as f64;
                let ci = gd.char_table[i][c];
                let cj_bar = (gd.char_table[j][c].0, -gd.char_table[j][c].1);
                sum.0 += sz * (ci.0 * cj_bar.0 - ci.1 * cj_bar.1);
                sum.1 += sz * (ci.0 * cj_bar.1 + ci.1 * cj_bar.0);
            }
            let inner_re = sum.0 / g_order;
            let inner_im = sum.1 / g_order;
            let expected = if i == j { 1.0 } else { 0.0 };
            let ok = (inner_re - expected).abs() < 0.01 && inner_im.abs() < 0.01;
            if !ok { all_ok = false; }
            if i == j {
                s_result(&mut out, &format!("  ⟨χ_{}, χ_{}⟩", i+1, j+1), &format!("{:.3} {}", inner_re, if ok { "✓" } else { "✗" }));
            } else if inner_re.abs() > 0.01 || inner_im.abs() > 0.01 {
                s_err(&mut out, &format!("  ⟨χ_{}, χ_{}⟩ = {:.3}+{:.3}i ≠ 0", i+1, j+1, inner_re, inner_im));
            }
        }
    }
    if all_ok { s_ok(&mut out, "All row orthogonality relations hold ✓"); }
    out.push('\n');

    out.push_str("  Column orthogonality check (first 2 classes):\n");
    for c1 in 0..n_classes.min(3) {
        for c2 in c1..n_classes.min(3) {
            let mut sum = (0.0f64, 0.0f64);
            for i in 0..n_irreps {
                let ci = gd.char_table[i][c1];
                let cj_bar = (gd.char_table[i][c2].0, -gd.char_table[i][c2].1);
                sum.0 += ci.0 * cj_bar.0 - ci.1 * cj_bar.1;
                sum.1 += ci.0 * cj_bar.1 + ci.1 * cj_bar.0;
            }
            let expected = if c1 == c2 { g_order / gd.class_sizes[c1] as f64 } else { 0.0 };
            let ok = (sum.0 - expected).abs() < 0.1;
            s_result(&mut out, &format!("  col({},{}) inner product", c1+1, c2+1),
                &format!("{:.2} (expected {:.0}) {}", sum.0, expected, if ok { "✓" } else { "✗" }));
        }
    }
    out
}

// ── Inner product ─────────────────────────────────────────────────────────────

fn cmd_inner_product(args: &[&str]) -> String {
    let mut out = String::new();
    let sep = args.iter().position(|&s| s == "/");
    let sep = match sep { Some(s) => s, None => { s_err(&mut out, "Use: inner_product <chi1> / <chi2> <type> <n>"); return out; } };

    let chi1: Vec<f64> = args[..sep].iter().filter_map(|s| s.parse().ok()).collect();
    let rest = &args[sep+1..];
    let type_idx = rest.iter().rposition(|s| s.parse::<f64>().is_err());
    let (chi2_args, meta) = if let Some(ti) = type_idx {
        (&rest[..ti], &rest[ti..])
    } else {
        (rest, [].as_slice())
    };
    let chi2: Vec<f64> = chi2_args.iter().filter_map(|s| s.parse().ok()).collect();

    let kind = meta.get(0).copied().unwrap_or("Z");
    let n_val = meta.get(1).and_then(|s| s.parse().ok()).unwrap_or(chi1.len());

    let gd = match group_data(kind, n_val) {
        Some(g) => g,
        None => { s_err(&mut out, "Unknown group"); return out; }
    };

    if chi1.len() != gd.class_names.len() || chi2.len() != gd.class_names.len() {
        s_err(&mut out, &format!("Need {} values for each character", gd.class_names.len()));
        return out;
    }

    s_section(&mut out, "Inner Product of Characters");
    out.push_str(&format!("  χ₁ = {:?}\n", chi1));
    out.push_str(&format!("  χ₂ = {:?}\n", chi2));
    out.push_str(&format!("  ⟨χ₁, χ₂⟩ = (1/{}) Σ_C |C|·χ₁(C)·χ₂(C)̄\n", gd.order));
    out.push('\n');

    let mut inner = 0.0f64;
    for c in 0..gd.class_names.len() {
        inner += gd.class_sizes[c] as f64 * chi1[c] * chi2[c];
    }
    inner /= gd.order as f64;
    let inner_rounded = (inner * 1000.0).round() / 1000.0;
    s_result(&mut out, "⟨χ₁, χ₂⟩", &format!("{}", inner_rounded));

    let is_int = (inner - inner.round()).abs() < 0.01;
    if is_int {
        let i = inner.round() as i64;
        if i == 1 {
            s_note(&mut out, "Inner product = 1: χ₁ and χ₂ are the same irreducible character.");
        } else if i == 0 {
            s_note(&mut out, "Inner product = 0: χ₁ and χ₂ are orthogonal (no common irrep).");
        } else {
            s_note(&mut out, &format!("Inner product = {}: a non-negative integer as expected.", i));
        }
    }

    let inner1 = {
        let mut s = 0.0f64;
        for c in 0..gd.class_names.len() {
            s += gd.class_sizes[c] as f64 * chi1[c] * chi1[c];
        }
        s / gd.order as f64
    };
    s_result(&mut out, "⟨χ₁, χ₁⟩", &format!("{:.3}", inner1));
    if (inner1 - 1.0).abs() < 0.01 {
        s_ok(&mut out, "⟨χ₁, χ₁⟩ = 1: χ₁ is irreducible.");
    } else {
        s_info(&mut out, &format!("⟨χ₁, χ₁⟩ = {:.1}: χ₁ is reducible (would be 1 for irreducible).", inner1));
    }
    out
}

// ── Detect abelian ────────────────────────────────────────────────────────────

fn cmd_detect_abelian(args: &[&str]) -> String {
    let mut out = String::new();
    let kind = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Type needed"); return out; } };
    let n = if matches!(kind, "S2"|"S3"|"S4"|"D3"|"D4"|"V4") { 0 }
    else { match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out } };

    let gd = match group_data(kind, n) {
        Some(g) => g,
        None => { s_err(&mut out, "Unknown group"); return out; }
    };

    s_section(&mut out, &format!("Is {} Abelian? (Character-Theoretic Test)", gd.name));
    out.push('\n');
    out.push_str("  Theorem: G is abelian iff all irreducible characters have degree 1.\n");
    out.push_str("  Equivalently: G is abelian iff #(conjugacy classes) = |G|.\n");
    out.push('\n');

    let dims: Vec<i64> = gd.char_table.iter().map(|r| r[0].0.round() as i64).collect();
    let all_one = dims.iter().all(|&d| d == 1);
    let n_classes = gd.class_names.len();

    out.push_str(&format!("  |G| = {},  #conjugacy classes = {}\n", gd.order, n_classes));
    out.push_str(&format!("  Character degrees: {:?}\n", dims));
    out.push('\n');

    if all_one {
        s_ok(&mut out, &format!("{} is ABELIAN (all irreps are 1-dimensional).", gd.name));
        s_ok(&mut out, &format!("#classes = {} = |G| ✓", n_classes));
    } else {
        let non_one: Vec<&i64> = dims.iter().filter(|&&d| d != 1).collect();
        s_err(&mut out, &format!("{} is NOT abelian.", gd.name));
        s_info(&mut out, &format!("  Non-trivial dimension irreps: {:?}", non_one));
        s_info(&mut out, &format!("  #classes = {} < |G| = {}", n_classes, gd.order));
    }
    out.push('\n');
    s_note(&mut out, "Proof: If G is abelian, every conjugacy class has size 1, so #classes = |G|.");
    s_note(&mut out, "Then Σ dim² = |G| with #classes terms forces all dims = 1.");
    out
}

// ── Burnside p^a q^b ──────────────────────────────────────────────────────────

fn cmd_burnside_paqb(args: &[&str]) -> String {
    let mut out = String::new();
    let p = match parse_uint(args, 0, "p") { Some(x) => x as i64, None => return out };
    let a = match parse_uint(args, 1, "a") { Some(x) => x as i64, None => return out };
    let q = match parse_uint(args, 2, "q") { Some(x) => x as i64, None => return out };
    let b = match parse_uint(args, 3, "b") { Some(x) => x as i64, None => return out };

    let order = mod_pow(p, a as u64, i64::MAX / 2) * mod_pow(q, b as u64, i64::MAX / 2);

    s_section(&mut out, &format!("Burnside's Theorem: |G| = {}^{}·{}^{} = {}", p, a, q, b, order));
    out.push('\n');
    out.push_str("  Theorem (Burnside 1904): Any group of order p^a·q^b (p,q prime) is solvable.\n");
    out.push('\n');
    out.push_str("  The proof uses character theory, specifically:\n");
    out.push_str("  Key Lemma: If χ is a non-trivial irreducible character and |χ(g)| = dim χ,\n");
    out.push_str("  then g is in the center of G (or g is in a small conjugacy class).\n");
    out.push('\n');
    out.push_str("  Sketch of proof:\n");
    out.push_str("  1. Suppose G is simple (we'll derive a contradiction).\n");
    out.push_str("  2. By Sylow theory, G has a p-Sylow subgroup P of order p^a.\n");
    out.push_str("  3. The center Z(P) has order p^k for some k ≥ 1.\n");
    out.push_str("  4. Pick g ∈ Z(P), g ≠ e. The conjugacy class of g has order |G:C_G(g)|.\n");
    out.push_str("  5. Since P ≤ C_G(g), this order divides q^b.\n");
    out.push_str("  6. By character theory, for each irrep χ:\n");
    out.push_str("     either dim χ divides q^b, or χ(g) = 0.\n");
    out.push_str("  7. The orthogonality relations give a contradiction unless G has a normal\n");
    out.push_str("     subgroup. So G is not simple, hence is solvable by induction.\n");
    out.push('\n');
    out.push_str(&format!("  Consequence for |G| = {}:\n", order));
    if a == 0 {
        out.push_str(&format!("  G is a group of prime power order {}^{} = q-group, hence nilpotent (solvable).\n", q, b));
    } else if b == 0 {
        out.push_str(&format!("  G is a group of prime power order {}^{} = p-group, hence nilpotent (solvable).\n", p, a));
    } else {
        out.push_str(&format!("  G has order {}^{}·{}^{} = {}, hence is solvable by Burnside's theorem.\n", p, a, q, b, order));
    }
    out.push('\n');
    s_note(&mut out, "Burnside's proof was simplified using character theory.");
    s_note(&mut out, "A purely group-theoretic proof (without characters) was found by");
    s_note(&mut out, "Goldschmidt (1970) and Matsuyama (1973), much harder.");
    out
}

// ── Conjugacy classes ─────────────────────────────────────────────────────────

fn cmd_conjugacy_classes(args: &[&str]) -> String {
    let mut out = String::new();
    let kind = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Type needed"); return out; } };
    let n = if matches!(kind, "S2"|"S3"|"S4"|"D3"|"D4"|"V4") { 0 }
    else { match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out } };

    let gd = match group_data(kind, n) {
        Some(g) => g,
        None => { s_err(&mut out, "Unknown group"); return out; }
    };

    s_section(&mut out, &format!("Conjugacy Classes of {}", gd.name));
    out.push_str(&format!("  |G| = {},  {} classes.\n", gd.order, gd.class_names.len()));
    out.push('\n');
    for (i, (name, &size)) in gd.class_names.iter().zip(gd.class_sizes.iter()).enumerate() {
        out.push_str(&format!("  Class {}: {}  (size {})\n", i+1, name, size));
    }
    out.push('\n');
    let total: usize = gd.class_sizes.iter().sum();
    s_result(&mut out, "Total elements", &format!("{} = |G| {}", total, if total == gd.order { "✓" } else { "✗" }));
    s_note(&mut out, "Number of conjugacy classes = number of irreducible representations.");
    out
}

// ── Class functions ───────────────────────────────────────────────────────────

fn cmd_class_functions(args: &[&str]) -> String {
    let mut out = String::new();
    let kind = match args.get(0) { Some(&s) => s, None => { s_err(&mut out, "Type needed"); return out; } };
    let n = if matches!(kind, "S2"|"S3"|"S4"|"D3"|"D4"|"V4") { 0 }
    else { match parse_uint(args, 1, "n") { Some(x) => x as usize, None => return out } };

    let gd = match group_data(kind, n) {
        Some(g) => g,
        None => { s_err(&mut out, "Unknown group"); return out; }
    };

    s_section(&mut out, &format!("Class Functions of {} — Orthonormal Basis", gd.name));
    out.push('\n');
    out.push_str("  A class function is f: G → ℂ with f(hgh⁻¹) = f(g) for all g,h ∈ G.\n");
    out.push_str("  Equivalently: f is constant on conjugacy classes.\n");
    out.push('\n');
    out.push_str("  The space of class functions has dimension = #{conjugacy classes}.\n");
    out.push_str(&format!("  For {}: dim = {}\n", gd.name, gd.class_names.len()));
    out.push('\n');
    out.push_str("  Inner product on class functions:\n");
    out.push_str(&format!("  ⟨f, g⟩ = (1/|G|) Σ_{{x∈G}} f(x)·g(x)̄ = (1/{}) Σ_C |C|·f(C)·g(C)̄\n", gd.order));
    out.push('\n');
    out.push_str("  Theorem: The irreducible characters χ₁,...,χ_r form an ORTHONORMAL BASIS\n");
    out.push_str("  for the space of class functions.\n");
    out.push('\n');
    out.push_str("  Verification of orthonormality:\n");
    let n_irreps = gd.char_table.len();
    for i in 0..n_irreps {
        let mut norm_sq = 0.0f64;
        for c in 0..gd.class_names.len() {
            let chi = gd.char_table[i][c];
            norm_sq += gd.class_sizes[c] as f64 * (chi.0*chi.0 + chi.1*chi.1);
        }
        norm_sq /= gd.order as f64;
        s_result(&mut out, &format!("  ‖χ_{}‖²", i+1), &format!("{:.3} {}", norm_sq, if (norm_sq - 1.0).abs() < 0.01 { "✓" } else { "✗" }));
    }
    out.push('\n');
    s_note(&mut out, "Any class function can be expanded: f = Σᵢ ⟨f, χᵢ⟩·χᵢ.");
    s_note(&mut out, "The coefficients ⟨f, χᵢ⟩ give the 'Fourier series' of f on G.");
    out
}

fn cmd_demo() -> String {
    let mut out = String::new();
    s_section(&mut out, "Demo: Character Theory Showcase");
    out.push('\n');
    out.push_str("  --- Character table of S₃ ---\n");
    out.push_str(&cmd_table(&["S3"]));
    out.push('\n');
    out.push_str("  --- Orthogonality for D₄ ---\n");
    out.push_str(&cmd_orthogonality(&["D4"]));
    out.push('\n');
    out.push_str("  --- Burnside: |G| = 2³·3 = 24 ---\n");
    out.push_str(&cmd_burnside_paqb(&["2", "3", "3", "1"]));
    out
}

fn default_state() -> StateMap {
    state_new()
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    if !args.is_empty() {
        let params: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
        if !params.is_empty() {
            state_set_ints(state, "last_params", &params);
        }
        state_set_str(state, "last_cmd", cmd);
    }
    match cmd {
        "table"             => cmd_table(args),
        "orthogonality"     => cmd_orthogonality(args),
        "inner_product"     => cmd_inner_product(args),
        "detect_abelian"    => cmd_detect_abelian(args),
        "burnside_paqb"     => cmd_burnside_paqb(args),
        "conjugacy_classes" => cmd_conjugacy_classes(args),
        "class_functions"   => cmd_class_functions(args),
        "demo"              => cmd_demo(),
        "help" | "h"        => show_help(),
        _ => { let mut out = String::new(); s_err(&mut out, &format!("Unknown command '{}'. Type 'help'.", cmd)); out }
    }
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { print!("{out}"); }
    true
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    // Character table with inner products for S₃
    c.text_bold(350.0, 30.0, "Character Table: S₃  (with orthogonality)", 15.0, "#222", "middle");
    // Table headers
    let cols = ["χ\\C", "e(1)", "(12)(3)", "(123)(2)"];
    let rows_data = [
        ("χ₁", "1", "1", "1"),
        ("χ₂", "1", "-1", "1"),
        ("χ₃", "2", "0", "-1"),
    ];
    let ox = 80.0; let oy = 70.0;
    let cw = 110.0; let rh = 40.0;
    // Headers
    for (ci, &h) in cols.iter().enumerate() {
        c.rect(ox + ci as f64 * cw, oy, cw - 2.0, rh - 2.0, "#dde", "#aaa", 0.5);
        c.text_bold(ox + ci as f64 * cw + 5.0, oy + 25.0, h, 13.0, "#222", "start");
    }
    // Rows
    for (ri, row) in rows_data.iter().enumerate() {
        let y = oy + (ri + 1) as f64 * rh;
        let vals = [row.0, row.1, row.2, row.3];
        for (ci, &v) in vals.iter().enumerate() {
            c.rect(ox + ci as f64 * cw, y, cw - 2.0, rh - 2.0, "white", "#aaa", 0.5);
            if ci == 0 { c.text_bold(ox + 5.0, y + 25.0, v, 13.0, "#222", "start"); }
            else { c.text(ox + ci as f64 * cw + 30.0, y + 25.0, v, 13.0, "#333", "start"); }
        }
    }
    // Orthogonality note
    c.text(80.0, 250.0, "Row orth: ⟨χᵢ, χⱼ⟩ = δᵢⱼ", 12.0, "#333", "start");
    c.text(80.0, 275.0, "⟨χ₃, χ₃⟩ = (4+0+2)/6 = 1 ✓", 12.0, "#333", "start");
    c.text(80.0, 310.0, "Burnside: |G|=p^a*q^b => solvable", 12.0, "#333", "start");
    c.text(80.0, 340.0, "S₃: |S₃|=6=2·3, solvable ✓", 12.0, "#333", "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("G", &[("label", "G"), ("shape", "ellipse")]);
    g.node("chi1", &[("label", "χ₁ (trivial)"), ("shape", "box")]);
    g.node("chi2", &[("label", "χ₂ (sign)"), ("shape", "box")]);
    g.node("chi3", &[("label", "χ₃ (standard)"), ("shape", "box")]);
    g.node("orth", &[("label", "⟨χᵢ,χⱼ⟩=δᵢⱼ"), ("shape", "plaintext")]);
    g.edge("G", "chi1", &[("label", "")]);
    g.edge("G", "chi2", &[("label", "")]);
    g.edge("G", "chi3", &[("label", "")]);
    g.edge("chi1", "orth", &[("label", "")]);
    g.edge("chi2", "orth", &[("label", "")]);
    g.edge("chi3", "orth", &[("label", "")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("chi1", 0.0, 2.0, "$\\chi_1$", "");
    t.node("chi2", 0.0, 0.0, "$\\chi_2$", "");
    t.node("chi3", 0.0, -2.0, "$\\chi_3$", "");
    t.node("inner", 4.0, 0.0, "$\\langle\\chi_i,\\chi_j\\rangle=\\delta_{ij}$", "");
    t.arrow("chi1", "inner", "", "");
    t.arrow("chi2", "inner", "", "");
    t.arrow("chi3", "inner", "", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 1, "Character Theory: S3 Table");
    a.hline(0, 40, 2, '-');
    a.text_at(0, 3, "       | e  | (12) | (123)");
    a.hline(0, 30, 4, '-');
    a.text_at(0, 5, "chi_1  | 1  |  1   |   1");
    a.text_at(0, 6, "chi_2  | 1  | -1   |   1");
    a.text_at(0, 7, "chi_3  | 2  |  0   |  -1");
    a.hline(0, 30, 8, '-');
    a.text_at(0, 10, "Row orthogonality: <chi_i, chi_j> = delta_{ij}");
    a.text_at(0, 11, "Sum dim^2 = 1+1+4 = 6 = |S3|");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch44"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 44", "Character Theory", "Orthogonality, inner products, and Burnside's theorem");
            print!("{}", show_help());
            print_note("Try: table S3   or   table D4   or   orthogonality S4");
            print_note("Or: conjugacy_classes S4   or   detect_abelian V4");
            print_note("Or: burnside_paqb 2 3 3 1   (group of order 2³·3 = 24)");
            repl("ch44> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
