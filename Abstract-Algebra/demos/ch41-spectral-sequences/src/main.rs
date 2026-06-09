use common::*;
use std::collections::HashMap;

fn show_help() -> String {
    let mut out = String::new();
    let rows = &[
        ("e2_page <rows> <cols> <entries>",                           "define E₂ page as rows×cols grid of orders"),
        ("differential <sp> <sq> <tp> <tq> <n>",                     "apply differential d_r: E_r^{p,q} → E_r^{p-r,q+r-1}"),
        ("serre_hopf",                                                "Serre SS for Hopf fibration S¹→S³→S²"),
        ("lhs_example <n>",                                          "LHS SS for ℤ/nℤ ◁ ℤ/n²ℤ ↠ ℤ/nℤ"),
        ("collapse <r>",                                              "show E_∞ page when SS collapses at page r"),
        ("filtration <n>",                                           "filtered complex for ℤ/nℤ and its SS"),
        ("convergence",                                              "explain convergence of spectral sequences"),
        ("demo",                                                     "showcase: Hopf fibration and LHS examples"),
        ("help",                                                      "show this help"),
        ("quit",                                                      "exit"),
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
fn s_sep(out: &mut String) {
    out.push_str(&format!("  \x1b[2m{}\x1b[0m\n", "─".repeat(60)));
}

// ── Display helpers ───────────────────────────────────────────────────────────

fn display_e_page(out: &mut String, page: &HashMap<(i32, i32), i64>, r: i32, p_range: (i32, i32), q_range: (i32, i32)) {
    out.push('\n');
    out.push_str(&format!("  E_{} page:\n", r));
    let (p_min, p_max) = p_range;
    let (q_min, q_max) = q_range;
    out.push_str("  q\\p ");
    for p in p_min..=p_max { out.push_str(&format!("  {:>4}", p)); }
    out.push('\n');
    out.push_str("      ");
    for _ in p_min..=p_max { out.push_str("──────"); }
    out.push('\n');
    for q in (q_min..=q_max).rev() {
        out.push_str(&format!("  {:>3} │", q));
        for p in p_min..=p_max {
            let val = page.get(&(p, q)).copied().unwrap_or(0);
            let label = if val == 0 {
                " 0    ".to_string()
            } else if val == 1 {
                " ℤ    ".to_string()
            } else {
                format!(" ℤ/{:<2}  ", val)
            };
            out.push_str(&label);
        }
        out.push('\n');
    }
    out.push('\n');
}

// ── E2 page command ───────────────────────────────────────────────────────────

fn cmd_e2_page(args: &[&str]) -> String {
    let mut out = String::new();
    let rows = match parse_uint(args, 0, "rows") { Some(x) => x as usize, None => return out };
    let cols = match parse_uint(args, 1, "cols") { Some(x) => x as usize, None => return out };
    let needed = rows * cols;
    if args.len() < 2 + needed {
        s_err(&mut out, &format!("Need {} entries (got {})", needed, args.len().saturating_sub(2)));
        return out;
    }
    let entries: Option<Vec<i64>> = args[2..2+needed].iter().map(|s| s.parse().ok()).collect();
    let entries = match entries { Some(e) => e, None => { s_err(&mut out, "Non-integer entry"); return out; } };

    let mut page: HashMap<(i32, i32), i64> = HashMap::new();
    for q in 0..rows {
        for p in 0..cols {
            let val = entries[(rows - 1 - q) * cols + p];
            if val != 0 {
                page.insert((p as i32, q as i32), val);
            }
        }
    }
    s_section(&mut out, "E₂ Page (Bigraded Module)");
    display_e_page(&mut out, &page, 2, (0, cols as i32 - 1), (0, rows as i32 - 1));
    out.push_str("  Entry = 0 means the trivial group.\n");
    out.push_str("  Entry = 1 means ℤ (free).\n");
    out.push_str("  Entry = n means ℤ/nℤ (torsion).\n");
    s_note(&mut out, "Differentials d_r go from (p,q) to (p-r, q+r-1).");
    out
}

// ── Differential ─────────────────────────────────────────────────────────────

fn cmd_differential(args: &[&str]) -> String {
    let mut out = String::new();
    let sp = match parse_int(args, 0, "source_p") { Some(x) => x as i32, None => return out };
    let sq = match parse_int(args, 1, "source_q") { Some(x) => x as i32, None => return out };
    let tp = match parse_int(args, 2, "target_p") { Some(x) => x as i32, None => return out };
    let tq = match parse_int(args, 3, "target_q") { Some(x) => x as i32, None => return out };
    let map_n = match parse_int(args, 4, "map_n") { Some(x) => x, None => return out };

    let r = sp - tp;
    if r <= 0 || tq != sq + r - 1 {
        s_err(&mut out, "Inconsistent (sp,sq) → (tp,tq): must satisfy tp = sp-r, tq = sq+r-1 for some r ≥ 1");
        return out;
    }

    s_section(&mut out, &format!("Differential d_{r}: E_{r}^{{{sp},{sq}}} → E_{r}^{{{tp},{tq}}}",
        r = r, sp = sp, sq = sq, tp = tp, tq = tq));
    out.push('\n');
    out.push_str(&format!("  d_{r}^{{{sp},{sq}}}: E_{r}^{{{sp},{sq}}} → E_{r}^{{{tp},{tq}}}\n",
        r = r, sp = sp, sq = sq, tp = tp, tq = tq));
    out.push_str(&format!("  This differential is multiplication by {}.\n", map_n));
    out.push('\n');
    s_note(&mut out, &format!("On the next page E_{{{}}}:", r + 1));
    out.push_str(&format!("  E_{}^{{{},{}}} = ker(d_{}^{{{},{}}}) / im(d_{} from ({},{}))\n",
        r+1, sp, sq, r, sp, sq, r, sp+r, sq-r+1));
    s_note(&mut out, "After all differentials on page r, take homology to get page r+1.");
    out
}

// ── Serre–Hopf built-in ───────────────────────────────────────────────────────

fn cmd_serre_hopf() -> String {
    let mut out = String::new();
    s_section(&mut out, "Serre Spectral Sequence: Hopf Fibration S¹ → S³ → S²");
    out.push('\n');
    out.push_str("  Fibration: S¹ → S³ → S²\n");
    out.push_str("  Fiber F = S¹,  Base B = S²,  Total space E = S³.\n");
    out.push('\n');
    out.push_str("  E₂ page: E₂^{p,q} = H_p(S²) ⊗ H_q(S¹)\n");
    out.push_str("  (using universal coefficients with trivial local system)\n");
    out.push('\n');

    let mut e2: HashMap<(i32, i32), i64> = HashMap::new();
    e2.insert((0, 0), 1);
    e2.insert((0, 1), 1);
    e2.insert((2, 0), 1);
    e2.insert((2, 1), 1);

    display_e_page(&mut out, &e2, 2, (0, 3), (0, 2));

    out.push_str("  Possible differentials:\n");
    out.push_str("  d₂: E₂^{2,0} → E₂^{0,1}  i.e., ℤ → ℤ\n");
    out.push_str("  (Degree formula: p↓2, q↑1; source at (2,0) → target at (0,1))\n");
    out.push('\n');
    out.push_str("  For the SS to converge to H_*(S³):\n");
    out.push_str("  H_*(S³) = ℤ at degrees 0 and 3 only.\n");
    out.push('\n');
    out.push_str("  The differential d₂: E₂^{2,0} = ℤ → E₂^{0,1} = ℤ must be ×1 (isomorphism).\n");
    out.push_str("  This kills E₂^{2,0} and E₂^{0,1} in the next page.\n");

    let mut e3: HashMap<(i32, i32), i64> = HashMap::new();
    e3.insert((0, 0), 1);
    e3.insert((2, 1), 1);

    display_e_page(&mut out, &e3, 3, (0, 3), (0, 2));

    out.push_str("  E₃ = E_∞  (no more non-trivial differentials in this range).\n");
    out.push('\n');
    out.push_str("  Convergence:\n");
    s_result(&mut out, "H_0(S³)", "ℤ  (from E_∞^{0,0})");
    s_result(&mut out, "H_3(S³)", "ℤ  (from E_∞^{2,1}, p+q = 3)");
    s_result(&mut out, "H_k(S³) for k≠0,3", "0 ✓");
    out.push('\n');
    s_note(&mut out, "The differential d₂ encodes the clutching function of the Hopf bundle.");
    s_note(&mut out, "This computation was first done by Serre in his 1950 thesis.");
    out
}

// ── LHS example ──────────────────────────────────────────────────────────────

fn cmd_lhs_example(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return out };
    if n < 2 { s_err(&mut out, "n must be at least 2"); return out; }
    let n2 = n * n;

    s_section(&mut out, &format!("LHS Spectral Sequence: ℤ/{}ℤ ◁ ℤ/{}ℤ ↠ ℤ/{}ℤ", n, n2, n));
    out.push('\n');
    out.push_str("  Group extension: 1 → N → G → Q → 1\n");
    out.push_str(&format!("  N = ℤ/{}ℤ,  G = ℤ/{}ℤ,  Q = G/N ≅ ℤ/{}ℤ.\n", n, n2, n));
    out.push('\n');
    out.push_str("  LHS (Lyndon-Hochschild-Serre) spectral sequence:\n");
    out.push_str("  E₂^{p,q} = H_p(Q, H_q(N, ℤ)) ⟹ H_{p+q}(G, ℤ)\n");
    out.push('\n');
    out.push_str(&format!("  H_*(N, ℤ) = H_*(ℤ/{}ℤ, ℤ):\n", n));
    out.push_str("    q=0: ℤ   (H_0 = ℤ always)\n");
    out.push_str("    q=1: 0\n");
    out.push_str(&format!("    q=2: ℤ/{}ℤ  (periodic)\n", n));
    out.push_str("    q odd ≥ 1: 0\n");
    out.push('\n');
    out.push_str("  E₂^{p,q} for low degrees:\n");

    let mut e2: HashMap<(i32, i32), i64> = HashMap::new();
    e2.insert((0, 0), 1);
    e2.insert((2, 0), n);
    e2.insert((0, 2), n);
    e2.insert((2, 2), n);
    e2.insert((4, 0), n);
    e2.insert((4, 2), n);

    display_e_page(&mut out, &e2, 2, (0, 4), (0, 4));

    out.push_str(&format!("  Target: H_*(ℤ/{}ℤ, ℤ):\n", n2));
    for k in 0..=4 {
        let h = match k % 2 {
            0 if k == 0 => "ℤ".to_string(),
            0 => format!("ℤ/{}ℤ", n2),
            _ => "0".to_string(),
        };
        out.push_str(&format!("    H_{}(ℤ/{}ℤ, ℤ) = {}\n", k, n2, h));
    }
    out.push('\n');
    out.push_str("  The differential d²: E₂^{2,0} → E₂^{0,1} = 0 is trivially zero.\n");
    out.push_str("  The differential d²: E₂^{2,1} = 0 → ... is zero.\n");
    out.push_str(&format!("  In degree 2: E_∞^{{2,0}} ⊕ E_∞^{{0,2}} must give ℤ/{}ℤ.\n", n2));
    out.push_str("  This requires a non-trivial differential:\n");
    out.push_str("  d³: E₃^{2,1} or a differential reducing E₂^{2,0} and E₂^{0,2}.\n");
    s_note(&mut out, "The LHS SS converges, encoding how H_*(G) is built from H_*(Q) and H_*(N).");
    out
}

// ── Collapse ─────────────────────────────────────────────────────────────────

fn cmd_collapse(args: &[&str]) -> String {
    let mut out = String::new();
    let r = match parse_uint(args, 0, "r") { Some(x) => x as i32, None => return out };

    s_section(&mut out, &format!("Spectral Sequence Collapsing at Page {}", r));
    out.push('\n');
    out.push_str(&format!("  A spectral sequence collapses at page E_{} if:\n", r));
    out.push_str(&format!("  All differentials d_k = 0 for k ≥ {}.\n", r));
    out.push_str(&format!("  Then E_{r} ≅ E_{r_plus_1} ≅ ... ≅ E_∞.\n", r = r, r_plus_1 = r+1));
    out.push('\n');

    if r == 2 {
        out.push_str("  Collapse at E₂ (earliest possible):\n");
        out.push_str("  • Occurs for product fibrations F×B → B.\n");
        out.push_str("  • Leray-Hirsch: if H_*(E) is free H_*(B)-module, SS collapses at E₂.\n");
        out.push_str("  • Künneth formula is the case of a trivial fibration.\n");
        out.push('\n');
        out.push_str("  Example: E₂ page for S¹ × S² (product):\n");
        let mut e2: HashMap<(i32, i32), i64> = HashMap::new();
        e2.insert((0, 0), 1); e2.insert((0, 1), 1);
        e2.insert((2, 0), 1); e2.insert((2, 1), 1);
        display_e_page(&mut out, &e2, 2, (0, 3), (0, 2));
        out.push_str("  All d_r = 0, so E_2 = E_∞.\n");
        out.push_str("  H_*(S¹×S²) = E_∞^{0,0} ⊕ E_∞^{0,1} ⊕ E_∞^{2,0} ⊕ E_∞^{2,1}\n");
        out.push_str("              = ℤ ⊕ ℤ ⊕ ℤ ⊕ ℤ  in degrees 0,1,2,3.\n");
    } else {
        out.push_str(&format!("  Collapse at E_{} means:\n", r));
        out.push_str(&format!("  • Pages 2 through {} may have non-trivial differentials.\n", r-1));
        out.push_str(&format!("  • From page {} onward, all groups are permanent cycles.\n", r));
        out.push('\n');
        out.push_str(&format!("  After collapse, E_{} = E_∞.\n", r));
        out.push_str("  The groups E_∞^{p,q} are the associated graded pieces of H_{p+q}.\n");
    }
    out.push('\n');
    s_note(&mut out, &format!("Detecting collapse: verify d_k = 0 for k = 2, ..., {}", r));
    s_note(&mut out, "One sufficient condition: E_r is concentrated in a single row or column.");
    out
}

// ── Filtration ────────────────────────────────────────────────────────────────

fn cmd_filtration(args: &[&str]) -> String {
    let mut out = String::new();
    let n = match parse_uint(args, 0, "n") { Some(x) => x as i64, None => return out };
    if n < 2 { s_err(&mut out, "n must be at least 2"); return out; }

    s_section(&mut out, &format!("Filtered Complex for ℤ/{}ℤ", n));
    out.push('\n');
    out.push_str(&format!("  Free resolution C• of ℤ/{}ℤ over ℤ:\n", n));
    out.push_str(&format!("  C₁ = ℤ →(×{})→ C₀ = ℤ → ℤ/{}ℤ\n", n, n));
    out.push('\n');
    out.push_str("  Define the canonical filtration:\n");
    out.push_str("  F^0 C• = C•,   F^1 C• = 0\n");
    out.push_str("  (concentrate the filtration in a single step)\n");
    out.push('\n');
    out.push_str("  Associated spectral sequence:\n");
    out.push_str("  E₀^{p,n} = F^p C_n / F^{p+1} C_n\n");
    out.push_str("  E₁^{p,n} = H_n(E₀^{p,•})\n");
    out.push('\n');
    out.push_str("  For our complex:\n");
    out.push_str("  E₁^{0,0} = C₀ = ℤ\n");
    out.push_str("  E₁^{0,1} = C₁ = ℤ\n");
    out.push_str("  E₁^{p,n} = 0 for p ≠ 0\n");
    out.push('\n');
    out.push_str(&format!("  The d₁ differential is the boundary map ×{}:\n", n));
    out.push_str(&format!("  d₁: E₁^{{0,1}} → E₁^{{0,0}}  given by ×{}\n", n));
    out.push('\n');
    out.push_str("  E₂ = H_*(C•):\n");
    out.push_str(&format!("  E₂^{{0,0}} = coker(×{}) = ℤ/{}ℤ  ✓\n", n, n));
    out.push_str(&format!("  E₂^{{0,1}} = ker(×{}) = 0         ✓\n", n));
    out.push('\n');
    s_note(&mut out, "This is the degenerate case of the spectral sequence for a filtered complex.");
    s_note(&mut out, "In general, the spectral sequence is a 'staircase' of successive approximations.");
    out
}

// ── Convergence ───────────────────────────────────────────────────────────────

fn cmd_convergence() -> String {
    let mut out = String::new();
    s_section(&mut out, "Convergence of Spectral Sequences");
    out.push('\n');
    out.push_str("  A spectral sequence {E_r, d_r} converges to H_* if:\n");
    out.push_str("  For each total degree n, there is a filtration\n");
    out.push_str("  0 = F^s H_n ⊆ ... ⊆ F^0 H_n ⊆ ... = H_n\n");
    out.push_str("  such that E_∞^{p,q} ≅ F^p H_{p+q} / F^{p+1} H_{p+q}.\n");
    out.push('\n');
    out.push_str("  The E_∞ page gives the ASSOCIATED GRADED of H_*, not H_* itself.\n");
    out.push('\n');
    s_sep(&mut out);
    out.push('\n');
    out.push_str("  Key points:\n");
    out.push_str("  1. Knowing all E_∞^{p,q} determines H_n only up to extensions.\n");
    out.push_str("  2. A first-quadrant SS: E_r^{p,q} = E_∞^{p,q} once r > max(p, q+1).\n");
    out.push_str("  3. Convergence theorem (Eilenberg-Moore): under boundedness conditions,\n");
    out.push_str("     the filtered SS always converges.\n");
    out.push('\n');
    out.push_str("  Example: Serre SS  E₂^{p,q} = H_p(B; H_q(F)) ⟹ H_{p+q}(E)\n");
    out.push('\n');
    out.push_str("  The filtration on H_n(E) is the 'Serre filtration':\n");
    out.push_str("  F^p H_n(E) = image of the map H_n(π⁻¹(B^{(p)}) → H_n(E))\n");
    out.push_str("  where B^{(p)} = p-skeleton of the base B.\n");
    out.push('\n');
    s_note(&mut out, "Extension problems can be non-trivial:");
    s_note(&mut out, "  E_∞^{1,1} = ℤ/2 and E_∞^{0,2} = ℤ/2 in H_2 could give ℤ/4 or ℤ/2 ⊕ ℤ/2.");
    s_note(&mut out, "This is why spectral sequences are most powerful when combined with other invariants.");
    out
}

fn cmd_demo() -> String {
    let mut out = String::new();
    s_section(&mut out, "Demo: Spectral Sequence Showcase");
    out.push('\n');
    out.push_str("  --- Hopf Fibration Serre SS ---\n");
    out.push_str(&cmd_serre_hopf());
    out.push('\n');
    out.push_str("  --- LHS SS for n=3 ---\n");
    out.push_str(&cmd_lhs_example(&["3"]));
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
        "e2_page"      => cmd_e2_page(args),
        "differential" => cmd_differential(args),
        "serre_hopf"   => cmd_serre_hopf(),
        "lhs_example"  => cmd_lhs_example(args),
        "collapse"     => cmd_collapse(args),
        "filtration"   => cmd_filtration(args),
        "convergence"  => cmd_convergence(),
        "demo"         => cmd_demo(),
        "help" | "h"   => show_help(),
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
    // E₂ page grid for Hopf fibration
    c.text_bold(350.0, 25.0, "E₂ Page: Hopf Fibration S¹→S³→S²", 15.0, "#222", "middle");
    let ox = 100.0;
    let oy = 420.0;
    let cell = 90.0;
    // Grid lines — use c.line for SVG canvas
    for i in 0..4 {
        let x = ox + i as f64 * cell;
        c.line(x, oy - 3.0 * cell, x, oy, "#999", 1.0);
    }
    for j in 0..4 {
        let y = oy - j as f64 * cell;
        c.line(ox, y, ox + 3.0 * cell, y, "#999", 1.0);
    }
    // Axis labels
    c.text(ox - 30.0, oy + 20.0, "p=0", 11.0, "#555", "middle");
    c.text(ox + cell - 30.0, oy + 20.0, "p=1", 11.0, "#555", "middle");
    c.text(ox + 2.0*cell - 30.0, oy + 20.0, "p=2", 11.0, "#555", "middle");
    c.text(ox - 50.0, oy, "q=0", 11.0, "#555", "middle");
    c.text(ox - 50.0, oy - cell, "q=1", 11.0, "#555", "middle");
    c.text(ox - 50.0, oy - 2.0*cell, "q=2", 11.0, "#555", "middle");
    // Entries
    c.text(ox + 30.0, oy - 15.0, "ℤ", 13.0, "#333", "middle");   // (0,0)
    c.text(ox + 30.0, oy - cell - 15.0, "ℤ", 13.0, "#333", "middle");  // (0,1)
    c.text(ox + 2.0*cell + 30.0, oy - 15.0, "ℤ", 13.0, "#333", "middle"); // (2,0)
    c.text(ox + 2.0*cell + 30.0, oy - cell - 15.0, "ℤ", 13.0, "#333", "middle"); // (2,1)
    // d₂ arrow from (2,0) to (0,1)
    c.arrow(ox + 2.0*cell + 45.0, oy - 15.0, ox + 45.0, oy - cell - 15.0, "#e44", 1.5);
    c.text(ox + cell + 10.0, oy - cell / 2.0 + 10.0, "d₂≅", 12.0, "#e44", "start");
    // Legend
    c.text(500.0, 200.0, "d₂ kills E₂^{2,0} and E₂^{0,1}", 12.0, "#333", "start");
    c.text(500.0, 230.0, "E₃ = E_∞", 12.0, "#333", "start");
    c.text(500.0, 260.0, "H*(S³) = ℤ,0,0,ℤ ✓", 12.0, "#333", "start");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    // E2, E3, E_inf pages as nodes
    g.node("E2", &[("label", "E₂ page"), ("shape", "box")]);
    g.node("E3", &[("label", "E₃ page"), ("shape", "box")]);
    g.node("Einf", &[("label", "E_∞ page"), ("shape", "box")]);
    g.edge("E2", "E3", &[("label", "d₂")]);
    g.edge("E3", "Einf", &[("label", "d₃,...")]);
    g.node("H", &[("label", "H_*(Total)"), ("shape", "ellipse")]);
    g.edge("Einf", "H", &[("label", "converges")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    // E2 page grid nodes
    t.node("e00", 0.0, 0.0, "$\\mathbb{Z}$", "");
    t.node("e10", 2.0, 0.0, "$0$", "");
    t.node("e20", 4.0, 0.0, "$\\mathbb{Z}$", "");
    t.node("e01", 0.0, 2.0, "$\\mathbb{Z}$", "");
    t.node("e11", 2.0, 2.0, "$0$", "");
    t.node("e21", 4.0, 2.0, "$\\mathbb{Z}$", "");
    t.arrow("e20", "e01", "$d_2$", "above");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 1, "Spectral Sequence E2 Page (Hopf Fibration)");
    a.hline(0, 45, 2, '-');
    a.text_at(0, 3, " q\\p  |  p=0  |  p=1  |  p=2  ");
    a.hline(0, 32, 4, '-');
    a.text_at(0, 5, " q=2  |   Z   |   0   |   Z   ");
    a.text_at(0, 6, " q=1  |   Z   |   0   |   Z   ");
    a.text_at(0, 7, " q=0  |   Z   |   0   |   Z   ");
    a.hline(0, 32, 8, '-');
    a.text_at(0, 10, "d2: E2^{2,0} --> E2^{0,1}  (iso)");
    a.text_at(0, 11, "E3 = E_inf,  H*(S^3) correct");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch41"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 41", "Spectral Sequences", "Filtering chain complexes to reveal hidden structure");
            print!("{}", show_help());
            print_note("Try: serre_hopf   or   lhs_example 3   or   convergence");
            print_note("Or: e2_page 3 3  0 0 0  1 0 0  0 0 1");
            repl("ch41> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
