use common::*;

fn show_help() {
    print_help(&[
        ("mat <n> <entries>",      "set current n×n integer matrix"),
        ("minimal",                "compute minimal polynomial (step-by-step)"),
        ("jordan2",                "Jordan form for current 2×2 matrix"),
        ("nilpotent <n>",          "n×n nilpotent Jordan block and its powers"),
        ("companion <coeffs...>",  "companion matrix for a polynomial"),
        ("rational",               "rational canonical form for current 2×2 matrix"),
        ("demo",                   "run a showcase of canonical forms"),
        ("help",                   "show this help"),
        ("quit",                   "exit"),
    ]);
}

// Multiply two n×n integer matrices stored as flat Vec<i64>
fn mat_mul(a: &[i64], b: &[i64], n: usize) -> Vec<i64> {
    let mut c = vec![0i64; n * n];
    for i in 0..n {
        for k in 0..n {
            if a[i * n + k] == 0 { continue; }
            for j in 0..n {
                c[i * n + j] += a[i * n + k] * b[k * n + j];
            }
        }
    }
    c
}

#[allow(dead_code)]
fn mat_sub(a: &[i64], b: &[i64]) -> Vec<i64> {
    a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()
}

fn mat_is_zero(a: &[i64]) -> bool {
    a.iter().all(|&x| x == 0)
}

fn identity_flat(n: usize) -> Vec<i64> {
    let mut m = vec![0i64; n * n];
    for i in 0..n { m[i * n + i] = 1; }
    m
}

fn scalar_mul_flat(a: &[i64], s: i64) -> Vec<i64> {
    a.iter().map(|&x| x * s).collect()
}

fn print_flat_matrix(data: &[i64], n: usize, label: &str) {
    println!("  {}", bold(label));
    let widths: Vec<usize> = (0..n)
        .map(|c| (0..n).map(|r| format!("{}", data[r * n + c]).len()).max().unwrap_or(1))
        .collect();
    for r in 0..n {
        print!("    │ ");
        for c in 0..n {
            print!("{:>width$}", data[r * n + c], width = widths[c]);
            if c + 1 < n { print!("  "); }
        }
        println!(" │");
    }
}

fn flat_matrix_to_string(data: &[i64], n: usize, label: &str) -> String {
    let mut out = format!("  {}\n", label);
    let widths: Vec<usize> = (0..n)
        .map(|c| (0..n).map(|r| format!("{}", data[r * n + c]).len()).max().unwrap_or(1))
        .collect();
    for r in 0..n {
        out.push_str("    │ ");
        for c in 0..n {
            out.push_str(&format!("{:>width$}", data[r * n + c], width = widths[c]));
            if c + 1 < n { out.push_str("  "); }
        }
        out.push_str(" │\n");
    }
    out
}

// Evaluate a polynomial (coeffs[i] = coeff of x^i) at a matrix A (n×n)
// Returns p(A) as a flat Vec<i64>
fn poly_eval_matrix(coeffs: &[i64], a: &[i64], n: usize) -> Vec<i64> {
    if coeffs.is_empty() {
        return vec![0i64; n * n];
    }
    let mut result = vec![0i64; n * n];
    let mut power = identity_flat(n); // A^0 = I
    for &c in coeffs.iter() {
        let term = scalar_mul_flat(&power, c);
        for i in 0..n * n { result[i] += term[i]; }
        power = mat_mul(&power, a, n);
    }
    result
}

// Char poly of a 2×2 matrix: x^2 - tr*x + det
fn char_poly_2x2(a: &[i64]) -> (i64, i64) {
    let tr = a[0] + a[3];
    let det = a[0] * a[3] - a[1] * a[2];
    (tr, det) // x^2 - tr*x + det
}

fn cmd_minimal(a: &[i64], n: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Minimal Polynomial\x1b[0m\n"));
    out.push_str(&format!("  Testing annihilating polynomials for {}×{} matrix\n", n, n));
    out.push_str(&format!("  {}\n", "\x1b[2m".to_string() + &"─".repeat(60) + "\x1b[0m"));

    if n == 0 {
        out.push_str("  \x1b[31m✗\x1b[0m No matrix set. Use: mat <n> <entries>\n");
        return out;
    }

    out.push_str("  Step 1: Test degree-1 factors (λ - a)I for integer a:\n");
    let mut found = false;
    for a_val in -10i64..=10 {
        let mut p = a.to_vec();
        for i in 0..n { p[i * n + i] -= a_val; }
        if mat_is_zero(&p) {
            out.push_str(&format!("  \x1b[32m✓\x1b[0m  (λ − {}) annihilates A at degree 1\n", a_val));
            out.push_str(&format!("  \x1b[36mMinimal polynomial\x1b[0m \x1b[2m=\x1b[0m \x1b[32mλ − {}\x1b[0m\n", a_val));
            found = true;
            break;
        }
    }
    if found { return out; }
    out.push_str("  No degree-1 annihilator found.\n");

    out.push_str("  Step 2: Test degree-2 polynomials λ^2 + bλ + c:\n");
    let mut found2 = false;
    'outer: for b in -20i64..=20 {
        for c in -20i64..=20 {
            let a2 = mat_mul(a, a, n);
            let ba = scalar_mul_flat(a, b);
            let ci = scalar_mul_flat(&identity_flat(n), c);
            let mut p = a2.clone();
            for i in 0..n * n { p[i] += ba[i] + ci[i]; }
            if mat_is_zero(&p) {
                let poly_str = poly_to_str(&[c, b, 1]);
                out.push_str(&format!("  \x1b[32m✓\x1b[0m  {} annihilates A\n", poly_str));
                out.push_str(&format!("  \x1b[36mMinimal polynomial\x1b[0m \x1b[2m=\x1b[0m \x1b[32m{}\x1b[0m\n", poly_to_str(&[c, b, 1])));
                found2 = true;
                break 'outer;
            }
        }
    }
    if found2 { return out; }
    out.push_str("  No degree-2 annihilator found in range [-20,20].\n");

    out.push_str("  Step 3: Test degree-3 polynomials λ^3 + aλ^2 + bλ + c:\n");
    let mut found3 = false;
    let a2 = mat_mul(a, a, n);
    let a3 = mat_mul(&a2, a, n);
    'outer3: for p2 in -10i64..=10 {
        for p1 in -20i64..=20 {
            for p0 in -20i64..=20 {
                let mut p = a3.clone();
                for i in 0..n * n {
                    p[i] += p2 * a2[i] + p1 * a[i];
                }
                for i in 0..n { p[i * n + i] += p0; }
                if mat_is_zero(&p) {
                    let poly_str = poly_to_str(&[p0, p1, p2, 1]);
                    out.push_str(&format!("  \x1b[32m✓\x1b[0m  {} annihilates A\n", poly_str));
                    out.push_str(&format!("  \x1b[36mMinimal polynomial\x1b[0m \x1b[2m=\x1b[0m \x1b[32m{}\x1b[0m\n", poly_to_str(&[p0, p1, p2, 1])));
                    found3 = true;
                    break 'outer3;
                }
            }
        }
    }
    if !found3 {
        out.push_str("  \x1b[31m✗\x1b[0m Minimal polynomial not found in small-coefficient search.\n");
    }

    out.push_str("  ◆ The minimal polynomial is the monic poly of least degree that annihilates A.\n");
    out.push_str("  ◆ By Cayley-Hamilton, char_poly(A) = 0 always, so min_poly | char_poly.\n");
    out
}

fn poly_to_str(coeffs: &[i64]) -> String {
    if coeffs.is_empty() { return "0".to_string(); }
    let mut parts = Vec::new();
    for (i, &c) in coeffs.iter().enumerate().rev() {
        if c == 0 { continue; }
        let term = match i {
            0 => format!("{}", c),
            1 => if c == 1 { "λ".to_string() } else if c == -1 { "−λ".to_string() }
                 else if c > 0 { format!("{}λ", c) } else { format!("−{}λ", c.abs()) },
            _ => if c == 1 { format!("λ^{}", i) } else if c == -1 { format!("−λ^{}", i) }
                 else if c > 0 { format!("{}λ^{}", c, i) } else { format!("−{}λ^{}", c.abs(), i) },
        };
        parts.push(term);
    }
    if parts.is_empty() { "0".to_string() } else { parts.join(" + ").replace("+ −", "− ") }
}

fn cmd_jordan2(a: &[i64]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Jordan Normal Form (2×2)\x1b[0m\n");
    if a.len() != 4 {
        out.push_str("  \x1b[31m✗\x1b[0m Current matrix is not 2×2. Use: mat 2 <a b c d>\n");
        return out;
    }

    let (tr, det) = char_poly_2x2(a);
    out.push_str(&format!("  Characteristic polynomial: λ² − {}λ + {}\n", tr, det));

    let disc = tr * tr - 4 * det;
    out.push_str(&format!("  Discriminant: {}² − 4·{} = {}\n", tr, det, disc));

    if disc == 0 {
        let lam = tr;
        if tr % 2 == 0 {
            let lam2 = tr / 2;
            out.push_str(&format!("  Repeated eigenvalue λ = {}\n", lam2));
            let mut diff = a.to_vec();
            for i in 0..2 { diff[i * 2 + i] -= lam2; }
            if mat_is_zero(&diff) {
                out.push_str(&format!("  A = {}·I  →  Jordan form is diagonal:\n", lam2));
                out.push_str(&format!("    │ {}  0 │\n    │ 0  {} │\n", lam2, lam2));
                out.push_str("  Minimal polynomial: (λ − λ₀)\n");
            } else {
                out.push_str(&format!("  A ≠ {}·I  →  Jordan block:\n", lam2));
                out.push_str(&format!("    │ {}  1 │\n    │ 0  {} │\n", lam2, lam2));
                out.push_str("  Minimal polynomial: (λ − λ₀)²\n");
            }
        } else {
            out.push_str(&format!("  Repeated eigenvalue λ = {}/2 (non-integer)\n", lam));
            out.push_str("  ◆ This matrix has a fractional repeated eigenvalue; Jordan form is over ℚ.\n");
        }
    } else if disc > 0 {
        let sq = (disc as f64).sqrt() as i64;
        if sq * sq == disc {
            let lam1 = (tr + sq) / 2;
            let lam2 = (tr - sq) / 2;
            if (tr + sq) % 2 == 0 && (tr - sq) % 2 == 0 {
                out.push_str(&format!("  Distinct eigenvalues: λ₁ = {}, λ₂ = {}\n", lam1, lam2));
                out.push_str("  Jordan form (diagonal):\n");
                out.push_str(&format!("    │ {}  0 │\n    │ 0  {} │\n", lam1, lam2));
                out.push_str("  Minimal polynomial: (λ − λ₁)(λ − λ₂)\n");
            } else {
                out.push_str(&format!("  ◆ Eigenvalues are ({} ± {})/2 — not integers.\n", tr, sq));
            }
        } else {
            out.push_str(&format!("  ◆ Discriminant {} is not a perfect square; eigenvalues are irrational.\n", disc));
            let lam_approx = (tr as f64) / 2.0;
            let sq_approx = (disc as f64).sqrt() / 2.0;
            out.push_str(&format!("  λ ≈ {:.4} ± {:.4} (real irrational)\n", lam_approx, sq_approx));
        }
    } else {
        out.push_str(&format!("  ◆ Discriminant {} < 0; complex eigenvalues (no real Jordan form).\n", disc));
        let re = (tr as f64) / 2.0;
        let im = ((-disc) as f64).sqrt() / 2.0;
        out.push_str(&format!("  λ = {:.4} ± {:.4}i\n", re, im));
    }

    out.push_str("  ◆ Jordan form groups eigenvectors/generalized eigenvectors into blocks.\n");
    out
}

fn cmd_nilpotent(n: usize) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  \x1b[1m\x1b[33m▸ Nilpotent Jordan Block N_{}\x1b[0m\n", n));
    if n == 0 || n > 8 {
        out.push_str("  \x1b[31m✗\x1b[0m n must be between 1 and 8\n");
        return out;
    }

    let mut nmat = vec![0i64; n * n];
    for i in 0..n - 1 { nmat[i * n + (i + 1)] = 1; }

    out.push_str(&flat_matrix_to_string(&nmat, n, &format!("N = N_{} (Jordan nilpotent block):", n)));
    out.push_str(&format!("  N^{} = 0  (nilpotency index = {})\n\n", n, n));

    let mut power = nmat.clone();
    for k in 1..=n {
        let label = if k == 1 { "N^1 = N:".to_string() } else { format!("N^{} =", k) };
        out.push_str(&flat_matrix_to_string(&power, n, &label));
        if mat_is_zero(&power) {
            out.push_str(&format!("  \x1b[32m✓\x1b[0m N^{} = 0  ← nilpotency index reached!\n", k));
            break;
        }
        if k < n { power = mat_mul(&power, &nmat, n); }
    }

    out.push_str(&format!("  ◆ The minimal polynomial of N_{} is λ^{}.\n", n, n));
    out.push_str("  ◆ Jordan blocks are the building blocks of all nilpotent matrices.\n");
    out
}

fn cmd_companion(coeffs: &[i64]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Companion Matrix\x1b[0m\n");
    if coeffs.is_empty() {
        out.push_str("  \x1b[31m✗\x1b[0m Usage: companion <a0> <a1> ... <a_{n-1}>  (monic polynomial coefficients)\n");
        return out;
    }

    let n = coeffs.len();
    let mut c = vec![0i64; n * n];

    for i in 0..n { c[i * n + (n - 1)] = -coeffs[i]; }
    for i in 1..n { c[i * n + (i - 1)] = 1; }

    let poly_str: String = {
        let mut s = String::from("x^") + &n.to_string();
        for (i, &c_i) in coeffs.iter().enumerate().rev() {
            if c_i == 0 { continue; }
            let sign = if c_i < 0 { "−" } else { "+" };
            let ac = c_i.abs();
            let term = match i {
                0 => format!("{}", ac),
                1 => if ac == 1 { "x".to_string() } else { format!("{}x", ac) },
                _ => if ac == 1 { format!("x^{}", i) } else { format!("{}x^{}", ac, i) },
            };
            s = format!("{} {} {}", s, sign, term);
        }
        s
    };

    out.push_str(&format!("  Polynomial: \x1b[1m{}\x1b[0m\n", poly_str));
    out.push_str("  (Companion matrix has the polynomial as its characteristic polynomial)\n\n");
    out.push_str(&flat_matrix_to_string(&c, n, "Companion matrix C:"));
    out.push_str("  ◆ Rational canonical form of a cyclic module uses companion matrices.\n");
    out
}

fn cmd_rational(a: &[i64]) -> String {
    let mut out = String::new();
    out.push_str("\n  \x1b[1m\x1b[33m▸ Rational Canonical Form (2×2)\x1b[0m\n");
    if a.len() != 4 {
        out.push_str("  \x1b[31m✗\x1b[0m Current matrix is not 2×2. Use: mat 2 <a b c d>\n");
        return out;
    }

    let (tr, det) = char_poly_2x2(a);
    let char_poly_str = format!("λ² − {}λ + {}", tr, det);
    out.push_str(&format!("  Characteristic polynomial: \x1b[1m{}\x1b[0m\n", char_poly_str));
    out.push_str(&format!("  {}\n", "\x1b[2m".to_string() + &"─".repeat(60) + "\x1b[0m"));

    let is_scalar = a[0] == a[3] && a[1] == 0 && a[2] == 0;
    if is_scalar {
        let lam = a[0];
        out.push_str(&format!("  A = {}·I  →  minimal polynomial = (λ − {})\n", lam, lam));
        out.push_str(&format!("  Invariant factors: [{}, {}]\n", lam, lam));
        out.push_str("\n  Rational canonical form:\n");
        out.push_str(&format!("    │ {}  0 │\n    │ 0  {} │\n", lam, lam));
    } else {
        out.push_str("  A is not scalar  →  minimal polynomial = characteristic polynomial\n");
        out.push_str(&format!("  Invariant factor: [{}]\n\n", char_poly_str));
        out.push_str("  Rational canonical form (companion of char poly):\n");
        out.push_str(&format!("    │  0  {} │\n    │  1  {} │\n", -det, tr));
        out.push_str("  This companion matrix has the same char poly as A.\n");
    }

    out.push_str("  ◆ Rational canonical form works over any field, unlike Jordan form.\n");
    out
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "mat" => {
            let n = match args.first().and_then(|s| s.parse::<usize>().ok()) {
                Some(v) => v,
                None => return "  \x1b[31m✗\x1b[0m Usage: mat <n> <entries>\n".to_string(),
            };
            if args.len() < 1 + n * n {
                return format!("  \x1b[31m✗\x1b[0m Need {} entries for {}×{} matrix\n", n*n, n, n);
            }
            let entries: Vec<i64> = args[1..=n*n].iter()
                .filter_map(|s| s.parse().ok())
                .collect();
            if entries.len() != n * n {
                return "  \x1b[31m✗\x1b[0m Could not parse all matrix entries as integers.\n".to_string();
            }
            state_set_int(state, "mat_n", n as i64);
            state_set_ints(state, "mat_data", &entries);
            let mut out = flat_matrix_to_string(&entries, n, &format!("Current {}×{} matrix:", n, n));
            out.push_str("  ◆ Use 'minimal', 'jordan2', or 'rational' to analyze this matrix.\n");
            out
        }
        "minimal" => {
            let n = state_get_int(state, "mat_n").unwrap_or(0) as usize;
            let data = state_get_ints(state, "mat_data").unwrap_or_default();
            if data.is_empty() {
                "  \x1b[31m✗\x1b[0m No matrix set. Use: mat <n> <entries>\n".to_string()
            } else {
                cmd_minimal(&data, n)
            }
        }
        "jordan2" => {
            let data = state_get_ints(state, "mat_data").unwrap_or_default();
            if data.is_empty() {
                "  \x1b[31m✗\x1b[0m No matrix set. Use: mat 2 <a b c d>\n".to_string()
            } else {
                cmd_jordan2(&data)
            }
        }
        "nilpotent" => {
            match args.first().and_then(|s| s.parse::<usize>().ok()) {
                Some(n) => {
                    state_set_int(state, "nilpotent_n", n as i64);
                    cmd_nilpotent(n)
                }
                None => "  \x1b[31m✗\x1b[0m Usage: nilpotent <n>\n".to_string(),
            }
        }
        "companion" => {
            if args.is_empty() {
                return "  \x1b[31m✗\x1b[0m Usage: companion <a0> <a1> ... <a_{n-1}>\n".to_string();
            }
            let coeffs: Vec<i64> = args.iter().filter_map(|s| s.parse().ok()).collect();
            if coeffs.len() != args.len() {
                return "  \x1b[31m✗\x1b[0m All companion arguments must be integers.\n".to_string();
            }
            cmd_companion(&coeffs)
        }
        "rational" => {
            let data = state_get_ints(state, "mat_data").unwrap_or_default();
            if data.is_empty() {
                "  \x1b[31m✗\x1b[0m No matrix set. Use: mat 2 <a b c d>\n".to_string()
            } else {
                cmd_rational(&data)
            }
        }
        "demo" => {
            let mut out = String::new();
            out.push_str("\n  === Demo: Canonical Forms ===\n\n");
            // Jordan block N_3
            out.push_str(&cmd_nilpotent(3));
            // Companion for x^2 - 3x + 2
            out.push_str(&cmd_companion(&[-2, 3]));
            // Rational form for [[0,-2],[1,3]] (companion of x^2-3x+2)
            let demo_mat = vec![0i64, -2, 1, 3];
            out.push_str(&cmd_rational(&demo_mat));
            out
        }
        "help" | "h" => {
            let mut h = String::new();
            h.push_str("\n  \x1b[1mCommands:\x1b[0m\n");
            h.push_str("    mat <n> <entries>         set current n×n integer matrix\n");
            h.push_str("    minimal                   compute minimal polynomial (step-by-step)\n");
            h.push_str("    jordan2                   Jordan form for current 2×2 matrix\n");
            h.push_str("    nilpotent <n>             n×n nilpotent Jordan block and its powers\n");
            h.push_str("    companion <coeffs...>     companion matrix for a polynomial\n");
            h.push_str("    rational                  rational canonical form for current 2×2 matrix\n");
            h.push_str("    demo                      run a showcase of canonical forms\n");
            h.push_str("    help                      show this help\n");
            h.push_str("    quit                      exit\n");
            h
        }
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
    // Default: 2×2 Jordan block with eigenvalue 2
    state_set_int(&mut s, "mat_n", 2);
    state_set_ints(&mut s, "mat_data", &[2, 1, 0, 2]);
    s
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    c.title("Ch10: Rational Canonical Form Blocks");
    c.subtitle("Companion matrix blocks decomposition", 42.0);

    let n = state_get_int(state, "mat_n").unwrap_or(2) as usize;
    let data = state_get_ints(state, "mat_data").unwrap_or_else(|| vec![2,1,0,2]);

    // Draw the matrix on the left
    if data.len() == n * n && n > 0 {
        let rows: Vec<Vec<i64>> = (0..n).map(|r| (0..n).map(|c_| data[r*n+c_]).collect()).collect();
        c.text_bold(120.0, 80.0, "A =", 14.0, colors::DARK, "middle");
        c.matrix_display(140.0, 65.0, &rows, 40.0);
    }

    // Draw companion block structure
    c.text_bold(350.0, 80.0, "RCF companion block", 13.0, colors::CYAN, "middle");
    if n == 2 && data.len() == 4 {
        let (tr, det) = (data[0] + data[3], data[0]*data[3] - data[1]*data[2]);
        let comp = vec![vec![0i64, -det], vec![1, tr]];
        c.matrix_display(310.0, 65.0, &comp, 45.0);
        c.text(350.0, 160.0, &format!("char poly: λ²−{}λ+{}", tr, det), 12.0, colors::GREY, "middle");
    }

    // Jordan form illustration
    c.text_bold(560.0, 80.0, "Jordan form (schematic)", 12.0, colors::MAGENTA, "middle");
    c.rect(490.0, 95.0, 50.0, 50.0, colors::HEADER_FILL, colors::CYAN, 1.5);
    c.text(515.0, 115.0, "λ", 14.0, colors::DARK, "middle");
    c.text(515.0, 130.0, "1", 11.0, colors::GREY, "middle");
    c.rect(540.0, 145.0, 50.0, 50.0, colors::ROW_ALT, colors::CYAN, 1.5);
    c.text(565.0, 165.0, "λ", 14.0, colors::DARK, "middle");
    c.text(400.0, 230.0, "Block structure captures invariant factors", 11.0, colors::GREY, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "mat_n").unwrap_or(2) as usize;
    let data = state_get_ints(state, "mat_data").unwrap_or_else(|| vec![2,1,0,2]);
    g.node_default("shape", "box");
    g.node_default("style", "filled");
    g.node_default("fillcolor", "lightyellow");
    g.node("A", &[("label", &format!("Matrix A ({}x{})", n, n))]);
    g.node("minpoly", &[("label", "min poly m(λ)")]);
    g.node("charpoly", &[("label", "char poly c(λ)")]);
    g.node("rcf", &[("label", "Rational Canonical Form"), ("fillcolor", "lightblue")]);
    g.node("jordan", &[("label", "Jordan Normal Form"), ("fillcolor", "lightgreen")]);
    g.edge("A", "charpoly", &[("label", "det(λI-A)")]);
    g.edge("charpoly", "minpoly", &[("label", "divides")]);
    g.edge("A", "rcf", &[("label", "invariant factors")]);
    g.edge("A", "jordan", &[("label", "eigenspaces")]);
    if n == 2 && data.len() == 4 {
        let (tr, det) = (data[0]+data[3], data[0]*data[3]-data[1]*data[2]);
        g.node("cpoly", &[("label", &format!("λ²−{}λ+{}", tr, det)), ("fillcolor", "lightyellow")]);
        g.edge("charpoly", "cpoly", &[("label", "=")]);
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "mat_n").unwrap_or(2) as usize;
    let data = state_get_ints(state, "mat_data").unwrap_or_else(|| vec![2,1,0,2]);
    t.use_library("arrows,positioning");
    t.raw("  \\tikzset{block/.style={draw,rectangle,minimum width=2cm,minimum height=0.8cm,align=center}}");
    t.node("A", -3.0, 0.0, "Matrix $A$", "block");
    t.node("rcf", 0.0, 0.0, "RCF", "block");
    t.node("jordan", 0.0, -1.5, "Jordan Form", "block");
    t.arrow("A", "rcf", "invariant factors", "->");
    t.arrow("A", "jordan", "eigenvalues", "->");
    if n == 2 && data.len() == 4 {
        let (tr, det) = (data[0]+data[3], data[0]*data[3]-data[1]*data[2]);
        t.raw(&format!("  \\node[below=0.3cm of A] {{$\\lambda^2 - {}\\lambda + {}$}};", tr, det));
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let n = state_get_int(state, "mat_n").unwrap_or(2) as usize;
    let data = state_get_ints(state, "mat_data").unwrap_or_else(|| vec![2,1,0,2]);

    a.text_at(2, 1, "Ch10: Canonical Forms");
    a.text_at(2, 2, "────────────────────────────────────────────────");

    if n == 2 && data.len() == 4 {
        let (tr, det) = (data[0]+data[3], data[0]*data[3]-data[1]*data[2]);
        a.text_at(2, 4, &format!("Matrix A = [{} {}; {} {}]", data[0], data[1], data[2], data[3]));
        a.text_at(2, 5, &format!("char poly: lambda^2 - {}*lambda + {}", tr, det));
        a.text_at(2, 7, "Companion (RCF block):      Jordan block:");
        a.text_at(2, 8, &format!("[ 0  {} ]                    [ lambda  1  ]", -det));
        a.text_at(2, 9, &format!("[ 1  {} ]                    [   0   lambda]", tr));
        a.text_at(2, 11, "min_poly | char_poly (Cayley-Hamilton)");
    } else {
        a.text_at(2, 4, &format!("Matrix: {}x{}", n, n));
        a.text_at(2, 5, "Use 'mat 2 a b c d' for 2x2 analysis.");
    }
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
                    let mut g = DotGraph::digraph("ch10");
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
            print_banner("Chapter 10", "Canonical Forms",
                "Jordan normal form · Minimal polynomial · Rational canonical form");
            print_info("Explore how matrices decompose into canonical building blocks.");
            show_help();
            repl("ch10> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}

// Verify poly_eval_matrix is used (suppress dead_code for test)
#[allow(dead_code)]
fn _use_poly_eval(coeffs: &[i64], a: &[i64], n: usize) -> Vec<i64> {
    poly_eval_matrix(coeffs, a, n)
}
