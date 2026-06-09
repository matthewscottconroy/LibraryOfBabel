use common::*;

// ── Cardinality Explorer ─────────────────────────────────────────────────────

fn cantor_pair(k1: u64, k2: u64) -> u64 {
    (k1 + k2) * (k1 + k2 + 1) / 2 + k2
}

fn cantor_unpair(n: u64) -> (u64, u64) {
    let w = (((8 * n + 1) as f64).sqrt() as u64 - 1) / 2;
    let t = (w * w + w) / 2;
    let k2 = n - t;
    let k1 = w - k2;
    (k1, k2)
}

fn nat_to_int(n: u64) -> i64 {
    if n == 0 { 0 }
    else if n % 2 == 1 { ((n + 1) / 2) as i64 }
    else { -((n / 2) as i64) }
}

fn cantor_diagonal_str(n: usize) -> String {
    let mut seqs: Vec<Vec<u8>> = Vec::new();
    for i in 0..n {
        let seq: Vec<u8> = (0..n).map(|j| {
            let v = (i.wrapping_mul(17).wrapping_add(j.wrapping_mul(31))
                      .wrapping_add(i.wrapping_mul(j).wrapping_mul(7))) % 2;
            v as u8
        }).collect();
        seqs.push(seq);
    }

    let mut out = String::new();
    out.push_str(&format!("Cantor Diagonal Argument (n={})\n\n", n));
    out.push_str(&format!("Suppose {} binary sequences enumerate [0,1]:\n\n", n));

    for (i, seq) in seqs.iter().enumerate() {
        out.push_str(&format!("  s{}: ", i));
        for (j, &b) in seq.iter().enumerate() {
            if j == i { out.push_str(&format!("[{}]", b)); }
            else { out.push_str(&format!(" {} ", b)); }
        }
        out.push('\n');
    }
    out.push('\n');

    let diagonal: Vec<u8> = (0..n).map(|i| seqs[i][i]).collect();
    let anti: Vec<u8> = diagonal.iter().map(|&b| 1 - b).collect();

    out.push_str("Diagonal:      ");
    for &b in &diagonal { out.push_str(&format!(" {} ", b)); }
    out.push('\n');
    out.push_str("Anti-diagonal: ");
    for &b in &anti { out.push_str(&format!(" {} ", b)); }
    out.push('\n');
    out.push('\n');
    out.push_str("The anti-diagonal differs from every sequence → list is incomplete!\n");
    out.push_str("Conclusion: |R| > aleph_0 (uncountable)\n");
    out
}

fn hilbert_hotel_str(n: u64) -> String {
    let mut out = String::new();
    out.push_str("Hilbert's Hotel\n\n");
    out.push_str("Infinitely many rooms, all occupied.\n");
    out.push_str(&format!("{} new guests arrive.\n\n", n));
    out.push_str(&format!("Solution: move each guest from room k to room k + {}\n\n", n));
    for k in 1..=(n + 5).min(10) {
        if k <= n {
            out.push_str(&format!("  Room {:>2}: [New Guest {}]\n", k, k));
        } else {
            out.push_str(&format!("  Room {:>2}: Guest {} (was room {})\n", k, k - n, k - n));
        }
    }
    out.push_str("  ...\n\n");
    out.push_str(&format!("Bijection: old_guest(k) → room(k+{})\n", n));
    out.push_str("Infinite sets can biject with proper subsets of themselves.\n");
    out
}

fn show_help() -> String {
    let mut out = String::new();
    out.push_str("Commands:\n");
    out.push_str("  diagonal <n>          Cantor diagonal argument with n sequences\n");
    out.push_str("  hotel <n>             Hilbert Hotel: accommodate n new guests\n");
    out.push_str("  pairing <a> <b>       Cantor pairing function: N×N → N\n");
    out.push_str("  unpairing <n>         Inverse Cantor pairing\n");
    out.push_str("  compare <n> <m>       Bijection between {0..n-1} and Z/mZ\n");
    out.push_str("  naturals_to_z         Bijection N → Z\n");
    out.push_str("  sizes                 aleph_0 vs |R|\n");
    out.push_str("  demo                  showcase of cardinality concepts\n");
    out.push_str("  quit                  exit\n");
    out
}

// ── State helpers ─────────────────────────────────────────────────────────────

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "diagonal_n", 6);
    state_set_int(&mut s, "hotel_guests", 3);
    s
}

// ── run_cmd ───────────────────────────────────────────────────────────────────

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    let mut out = String::new();
    match cmd {
        "help" | "h" => {
            out.push_str(&show_help());
        }

        "demo" => {
            out.push_str("=== Cardinality Demo ===\n\n");
            out.push_str(&cantor_diagonal_str(5));
            out.push('\n');
            out.push_str(&hilbert_hotel_str(3));
            out.push('\n');
            out.push_str("Cantor pairing grid (a,b → N):\n");
            out.push_str("      0    1    2    3    4\n");
            for a in 0..5u64 {
                out.push_str(&format!("  {}:", a));
                for b in 0..5u64 { out.push_str(&format!("  {:>4}", cantor_pair(a, b))); }
                out.push('\n');
            }
        }

        "diagonal" => {
            let n = parse_uint(args, 0, "n").unwrap_or(6) as usize;
            if n < 2 || n > 16 { return "n must be 2–16.\n".to_string(); }
            state_set_int(state, "diagonal_n", n as i64);
            out.push_str(&cantor_diagonal_str(n));
        }

        "hotel" => {
            let n = parse_uint(args, 0, "n").unwrap_or(3);
            state_set_int(state, "hotel_guests", n as i64);
            out.push_str(&hilbert_hotel_str(n));
        }

        "pairing" => {
            let a = parse_uint(args, 0, "a").unwrap_or(0);
            let b = parse_uint(args, 1, "b").unwrap_or(0);
            let p = cantor_pair(a, b);
            out.push_str(&format!("pi({}, {}) = {}\n", a, b, p));
            out.push_str("Formula: pi(k1,k2) = (k1+k2)(k1+k2+1)/2 + k2\n\n");
            out.push_str("Pairing grid:\n");
            out.push_str("      0    1    2    3    4\n");
            for a2 in 0..5u64 {
                out.push_str(&format!("  {}:", a2));
                for b2 in 0..5u64 {
                    let v = cantor_pair(a2, b2);
                    if a2 == a && b2 == b {
                        out.push_str(&format!(" [{:>3}]", v));
                    } else {
                        out.push_str(&format!("  {:>4}", v));
                    }
                }
                out.push('\n');
            }
        }

        "unpairing" => {
            let n = parse_uint(args, 0, "n").unwrap_or(0);
            let (k1, k2) = cantor_unpair(n);
            out.push_str(&format!("pi^-1({}) = ({}, {})\n", n, k1, k2));
            out.push_str(&format!("Verification: pi({}, {}) = {}\n", k1, k2, cantor_pair(k1, k2)));
        }

        "compare" => {
            let n = parse_uint(args, 0, "n").unwrap_or(5) as usize;
            let m = parse_uint(args, 1, "m").unwrap_or(5) as usize;
            if n != m {
                out.push_str(&format!("|{{0..{}}}| = {} ≠ {} = |Z/{}Z| — no bijection.\n", n-1, n, m, m));
            } else {
                out.push_str(&format!("Bijection {{0..{}}} ↔ Z/{}Z:\n", n-1, m));
                for k in 0..n.min(12) {
                    out.push_str(&format!("  {} ↦ [{}]\n", k, k % m));
                }
                if n > 12 { out.push_str("  ...\n"); }
            }
        }

        "naturals_to_z" => {
            out.push_str("Bijection N → Z:\n");
            out.push_str("f(n) = n/2 if n even, -(n+1)/2 if n odd\n\n");
            out.push_str("  n  : ");
            for n in 0..=12u64 { out.push_str(&format!(" {:>4}", n)); }
            out.push('\n');
            out.push_str("  f(n): ");
            for n in 0..=12u64 { out.push_str(&format!(" {:>4}", nat_to_int(n))); }
            out.push('\n');
            out.push_str("\n|Z| = aleph_0 = |N|\n");
        }

        "sizes" => {
            out.push_str("Infinite Cardinality Hierarchy\n\n");
            out.push_str("  aleph_0 = |N| = |Z| = |Q|  (countably infinite)\n");
            out.push_str("  2^aleph_0 = |R| = |C|       (uncountable continuum)\n");
            out.push_str("  2^|R| = |P(R)|              (even larger)\n\n");
            out.push_str("Cantor's theorem: |P(X)| > |X| for any set X.\n");
            out.push_str("Continuum hypothesis: 2^aleph_0 = aleph_1 (independent of ZFC).\n");
        }

        _ => out.push_str(&format!("Unknown command '{}'. Type 'help'.\n", cmd)),
    }
    out
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { print!("{out}"); }
    true
}

// ── Visualization functions ───────────────────────────────────────────────────

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    c.title("Cantor Pairing Function");
    c.subtitle("pi(a,b) = (a+b)(a+b+1)/2 + b", 42.0);

    let n = 6usize;
    let cell_w = 55.0; let cell_h = 30.0;
    let x0 = 50.0; let y0 = 60.0;

    // Header row
    let mut cells: Vec<Vec<String>> = Vec::new();
    let mut header = vec!["a\\b".to_string()];
    for b in 0..n { header.push(b.to_string()); }
    cells.push(header);

    for a in 0..n {
        let mut row = vec![a.to_string()];
        for b in 0..n {
            row.push(cantor_pair(a as u64, b as u64).to_string());
        }
        cells.push(row);
    }

    c.table(x0, y0, &cells, cell_w, cell_h, colors::HEADER_FILL, colors::ROW_NORM, colors::GREY);

    // Diagonal highlight
    for i in 0..n {
        let x = x0 + (i + 1) as f64 * cell_w;
        let y = y0 + (i + 1) as f64 * cell_h;
        c.rect(x, y, cell_w, cell_h, "#ffd54f40", colors::ORANGE, 1.5);
    }

    c.text(350.0, 260.0, "Diagonal cells: consecutive values", 12.0, colors::GREY, "middle");
    c.text(350.0, 280.0, "NxN is countable: same size as N", 13.0, colors::DARK, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node_default("shape", "box");
    g.node_default("style", "filled,rounded");

    g.node("finite",  &[("label", "Finite sets\n|A| = n"), ("fillcolor", "#e8f5e9")]);
    g.node("aleph0",  &[("label", "aleph_0 = |N| = |Z| = |Q|"), ("fillcolor", "#b2ebf2")]);
    g.node("cont",    &[("label", "2^aleph_0 = |R|"), ("fillcolor", "#b3e5fc")]);
    g.node("power_r", &[("label", "2^|R| = |P(R)|"), ("fillcolor", "#e1bee7")]);

    g.edge("finite", "aleph0", &[("label", "<")]);
    g.edge("aleph0", "cont",   &[("label", "<  (Cantor diagonal)")]);
    g.edge("cont",   "power_r",&[("label", "<  (Cantor theorem)")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("nat",   0.0, 0.0,  "N", "draw,circle,minimum size=1.5cm");
    t.node("int",   3.0, 0.0,  "Z", "draw,circle,minimum size=1.5cm");
    t.node("rat",   6.0, 0.0,  "Q", "draw,circle,minimum size=1.5cm");
    t.node("real",  3.0, -3.0, "R", "draw,circle,minimum size=2cm,fill=gray!20");
    t.arrow("nat", "int", "f", "-stealth,blue");
    t.arrow("int", "rat", "g", "-stealth,blue");
    t.arrow("rat", "real", "?", "-stealth,red,dashed");
    t.raw("  \\node[above=2mm of nat] {$\\aleph_0$};");
    t.raw("  \\node[above=2mm of real] {$2^{\\aleph_0}$};");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(0, 0, "Cardinality Comparison");
    a.text_at(0, 2, "Finite sets");
    a.arrow_right(14, 2, 6);
    a.text_at(21, 2, "aleph_0 = |N|=|Z|=|Q|");
    a.arrow_right(43, 2, 6);
    a.text_at(50, 2, "2^aleph_0 = |R|");

    a.text_at(0, 4, "Cantor Pairing  pi(a,b) = (a+b)(a+b+1)/2 + b:");
    a.text_at(0, 6, "      b=0  b=1  b=2  b=3  b=4");
    for a_val in 0..5usize {
        let mut line = format!("  a={}:", a_val);
        for b_val in 0..5u64 {
            line.push_str(&format!("  {:>4}", cantor_pair(a_val as u64, b_val)));
        }
        a.text_at(0, 7 + a_val as i32, &line);
    }
    a.text_at(0, 13, "Diagonal: 0, 2, 5, 9, 14, ... (anti-diagonals)");
    a.text_at(0, 14, "|N x N| = aleph_0  (same size as N!)");
}

// ── main ──────────────────────────────────────────────────────────────────────

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
                    let mut c = SvgCanvas::new(700.0, 350.0);
                    visualize_svg(&mut c, cmd, &args_ref, &state);
                    c.build()
                }
                OutputFormat::Dot => {
                    let mut g = DotGraph::digraph("ch03");
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
            print_banner("Chapter 3", "Cardinality and the Axiom of Choice",
                "Explore infinite sets, Cantor's theorem, and beyond");
            print_info("Type 'help' for commands.");
            print_note("Try: diagonal 6");
            print_note("Try: hotel 3");
            repl("ch3> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
