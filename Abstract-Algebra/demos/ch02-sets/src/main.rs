use common::*;
use std::collections::HashMap;

// ── Set Theory Explorer ──────────────────────────────────────────────────────
// Sets are finite collections of integers.

fn set_to_str(s: &[i64]) -> String {
    if s.is_empty() { return "∅".to_string(); }
    let elems: Vec<String> = s.iter().map(|x| x.to_string()).collect();
    format!("{{{}}}", elems.join(", "))
}

fn union(a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut result: Vec<i64> = a.to_vec();
    for &x in b { if !result.contains(&x) { result.push(x); } }
    result.sort(); result
}

fn intersection(a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut result: Vec<i64> = a.iter().filter(|x| b.contains(x)).cloned().collect();
    result.sort(); result
}

fn difference(a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut result: Vec<i64> = a.iter().filter(|x| !b.contains(x)).cloned().collect();
    result.sort(); result
}

fn powerset(a: &[i64]) -> Vec<Vec<i64>> {
    let n = a.len();
    if n > 16 { return vec![]; }
    (0u32..(1u32 << n)).map(|mask| {
        a.iter().enumerate()
            .filter(|(i, _)| (mask >> i) & 1 == 1)
            .map(|(_, &x)| x)
            .collect()
    }).collect()
}

fn cartesian_product(a: &[i64], b: &[i64]) -> Vec<(i64, i64)> {
    let mut result = Vec::new();
    for &x in a { for &y in b { result.push((x, y)); } }
    result
}

fn is_reflexive(set: &[i64], rel: &[(i64, i64)]) -> bool {
    set.iter().all(|&x| rel.contains(&(x, x)))
}

fn is_symmetric(rel: &[(i64, i64)]) -> bool {
    rel.iter().all(|&(a, b)| rel.contains(&(b, a)))
}

fn is_transitive(rel: &[(i64, i64)]) -> bool {
    for &(a, b) in rel {
        for &(c, d) in rel {
            if b == c && !rel.contains(&(a, d)) { return false; }
        }
    }
    true
}

fn is_antisymmetric(rel: &[(i64, i64)]) -> bool {
    for &(a, b) in rel {
        if a != b && rel.contains(&(b, a)) { return false; }
    }
    true
}

fn show_help() -> String {
    let mut out = String::new();
    out.push_str("Commands:\n");
    out.push_str("  set <name> <elems>    define a set, e.g. 'set A 1 2 3'\n");
    out.push_str("  show                  list all defined sets\n");
    out.push_str("  union <A> <B>         A ∪ B\n");
    out.push_str("  inter <A> <B>         A ∩ B\n");
    out.push_str("  diff <A> <B>          A \\ B\n");
    out.push_str("  power <A>             powerset of A\n");
    out.push_str("  cart <A> <B>          Cartesian product A × B\n");
    out.push_str("  relation <A>          explore binary relations on A\n");
    out.push_str("  bijection <A> <B>     display a bijection if |A| = |B|\n");
    out.push_str("  demo                  showcase of set operations\n");
    out.push_str("  quit                  exit\n");
    out.push_str("\nElements are space-separated integers.\n");
    out
}

fn get_set_str<'a>(sets: &'a HashMap<String, Vec<i64>>, name: &str) -> Option<&'a Vec<i64>> {
    sets.get(name)
}

// ── State helpers ─────────────────────────────────────────────────────────────

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_ints(&mut s, "set_A", &[1, 2, 3]);
    state_set_ints(&mut s, "set_B", &[2, 3, 4]);
    s
}

fn sets_from_state(state: &StateMap) -> HashMap<String, Vec<i64>> {
    let mut sets = HashMap::new();
    for key in state.keys() {
        if let Some(name) = key.strip_prefix("set_") {
            if let Some(elems) = state_get_ints(state, key) {
                sets.insert(name.to_string(), elems);
            }
        }
    }
    sets
}

// ── run_cmd ───────────────────────────────────────────────────────────────────

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    let mut out = String::new();
    let mut sets = sets_from_state(state);

    match cmd {
        "help" | "h" => {
            out.push_str(&show_help());
        }

        "demo" => {
            let a: Vec<i64> = vec![1, 2, 3];
            let b: Vec<i64> = vec![2, 3, 4];
            out.push_str("=== Set Theory Demo ===\n\n");
            out.push_str(&format!("A = {}\n", set_to_str(&a)));
            out.push_str(&format!("B = {}\n", set_to_str(&b)));
            out.push_str(&format!("A ∪ B = {}\n", set_to_str(&union(&a, &b))));
            out.push_str(&format!("A ∩ B = {}\n", set_to_str(&intersection(&a, &b))));
            out.push_str(&format!("A \\ B = {}\n", set_to_str(&difference(&a, &b))));
            let ps = powerset(&a);
            out.push_str(&format!("P(A) has {} subsets\n", ps.len()));
            out.push_str("Subsets of A:\n");
            for sub in &ps {
                out.push_str(&format!("  {}\n", set_to_str(sub)));
            }
        }

        "set" => {
            if args.is_empty() {
                return "Usage: set <name> <elements...>\n".to_string();
            }
            let name = args[0].to_string();
            let mut elems: Vec<i64> = Vec::new();
            let mut ok = true;
            for &s in &args[1..] {
                match s.parse::<i64>() {
                    Ok(n) => { if !elems.contains(&n) { elems.push(n); } }
                    Err(_) => { out.push_str(&format!("'{}' is not a valid integer.\n", s)); ok = false; }
                }
            }
            if ok {
                elems.sort();
                out.push_str(&format!("Defined {} = {}\n", name, set_to_str(&elems)));
                state_set_ints(state, &format!("set_{}", name), &elems);
            }
        }

        "show" => {
            if sets.is_empty() {
                out.push_str("No sets defined. Use 'set <name> <elements>'.\n");
            } else {
                let mut names: Vec<String> = sets.keys().cloned().collect();
                names.sort();
                for name in &names {
                    let elems = &sets[name];
                    out.push_str(&format!("|{}| = {}  {} = {}\n",
                        name, elems.len(), name, set_to_str(elems)));
                }
            }
        }

        "union" => {
            if args.len() < 2 { return "Usage: union <A> <B>\n".to_string(); }
            match (get_set_str(&sets, args[0]), get_set_str(&sets, args[1])) {
                (Some(a), Some(b)) => {
                    let result = union(a, b);
                    out.push_str(&format!("{} ∪ {} = {}\n", args[0], args[1], set_to_str(&result)));
                    out.push_str(&format!("|result| = {}\n", result.len()));
                }
                _ => out.push_str(&format!("Set '{}' or '{}' not defined.\n", args[0], args[1])),
            }
        }

        "inter" => {
            if args.len() < 2 { return "Usage: inter <A> <B>\n".to_string(); }
            match (get_set_str(&sets, args[0]), get_set_str(&sets, args[1])) {
                (Some(a), Some(b)) => {
                    let result = intersection(a, b);
                    out.push_str(&format!("{} ∩ {} = {}\n", args[0], args[1], set_to_str(&result)));
                    if result.is_empty() { out.push_str("Sets are disjoint.\n"); }
                }
                _ => out.push_str(&format!("Set '{}' or '{}' not defined.\n", args[0], args[1])),
            }
        }

        "diff" => {
            if args.len() < 2 { return "Usage: diff <A> <B>\n".to_string(); }
            match (get_set_str(&sets, args[0]), get_set_str(&sets, args[1])) {
                (Some(a), Some(b)) => {
                    let result = difference(a, b);
                    out.push_str(&format!("{} \\ {} = {}\n", args[0], args[1], set_to_str(&result)));
                    let sym = union(&difference(a, b), &difference(b, a));
                    out.push_str(&format!("Symmetric diff = {}\n", set_to_str(&sym)));
                }
                _ => out.push_str(&format!("Set '{}' or '{}' not defined.\n", args[0], args[1])),
            }
        }

        "power" => {
            if args.is_empty() { return "Usage: power <A>\n".to_string(); }
            match get_set_str(&sets, args[0]) {
                None => out.push_str(&format!("Set '{}' not defined.\n", args[0])),
                Some(a) => {
                    if a.len() > 6 {
                        out.push_str(&format!("|P({})| = 2^{} = {}\n", args[0], a.len(), 1usize << a.len()));
                        return out;
                    }
                    let ps = powerset(a);
                    out.push_str(&format!("Powerset of {} = {}\n", args[0], set_to_str(a)));
                    out.push_str(&format!("|P({})| = 2^{} = {}\n", args[0], a.len(), ps.len()));
                    for (i, subset) in ps.iter().enumerate() {
                        out.push_str(&format!("  {:>3}.  {}\n", i, set_to_str(subset)));
                    }
                }
            }
        }

        "cart" => {
            if args.len() < 2 { return "Usage: cart <A> <B>\n".to_string(); }
            match (get_set_str(&sets, args[0]), get_set_str(&sets, args[1])) {
                (Some(a), Some(b)) => {
                    if a.len() * b.len() > 50 {
                        out.push_str(&format!("|A × B| = {}\n", a.len() * b.len()));
                        return out;
                    }
                    let prod = cartesian_product(a, b);
                    out.push_str(&format!("{} × {} has {} pairs\n", args[0], args[1], prod.len()));
                    let pairs: Vec<String> = prod.iter().map(|(x, y)| format!("({},{})", x, y)).collect();
                    for chunk in pairs.chunks(8) {
                        out.push_str(&format!("  {}\n", chunk.join("  ")));
                    }
                }
                _ => out.push_str(&format!("Set '{}' or '{}' not defined.\n", args[0], args[1])),
            }
        }

        "relation" => {
            if args.is_empty() { return "Usage: relation <A>\n".to_string(); }
            match get_set_str(&sets, args[0]) {
                None => out.push_str(&format!("Set '{}' not defined.\n", args[0])),
                Some(a) => {
                    if a.len() > 4 {
                        out.push_str("Set too large (max 4 elements).\n");
                        return out;
                    }
                    let prod = cartesian_product(a, a);
                    out.push_str(&format!("Binary Relations on {} = {}\n", args[0], set_to_str(a)));
                    out.push_str(&format!("{} × {} has {} pairs → 2^{} possible relations\n",
                        args[0], args[0], prod.len(), prod.len()));

                    let empty_rel: Vec<(i64,i64)> = vec![];
                    let full_rel = prod.clone();
                    let id_rel: Vec<(i64,i64)> = a.iter().map(|&x| (x,x)).collect();
                    let le_rel: Vec<(i64,i64)> = prod.iter().filter(|(x,y)| x <= y).cloned().collect();

                    for (name, rel) in &[("Empty", &empty_rel), ("Full A×A", &full_rel),
                                          ("Identity", &id_rel), ("≤", &le_rel)] {
                        let r = is_reflexive(a, rel);
                        let s = is_symmetric(rel);
                        let t = is_transitive(rel);
                        let as_ = is_antisymmetric(rel);
                        let props: Vec<&str> = [
                            if r { Some("reflexive") } else { None },
                            if s { Some("symmetric") } else { None },
                            if t { Some("transitive") } else { None },
                            if as_ { Some("antisymmetric") } else { None },
                        ].iter().filter_map(|x| *x).collect();
                        out.push_str(&format!("  {}: {}\n", name, props.join(", ")));
                        if r && s && t { out.push_str("    → Equivalence relation!\n"); }
                        if r && as_ && t { out.push_str("    → Partial order!\n"); }
                    }
                }
            }
        }

        "bijection" => {
            if args.len() < 2 { return "Usage: bijection <A> <B>\n".to_string(); }
            match (get_set_str(&sets, args[0]), get_set_str(&sets, args[1])) {
                (Some(a), Some(b)) => {
                    if a.len() != b.len() {
                        out.push_str(&format!("|{}| = {} ≠ {} = |{}| — no bijection.\n",
                            args[0], a.len(), b.len(), args[1]));
                    } else {
                        out.push_str(&format!("Natural bijection {} → {}:\n", args[0], args[1]));
                        for (x, y) in a.iter().zip(b.iter()) {
                            out.push_str(&format!("  {} ↦ {}\n", x, y));
                        }
                        let n_bijs = (1..=a.len()).product::<usize>();
                        out.push_str(&format!("One of {}! = {} possible bijections.\n", a.len(), n_bijs));
                    }
                }
                _ => out.push_str(&format!("Set '{}' or '{}' not defined.\n", args[0], args[1])),
            }
        }

        _ => out.push_str(&format!("Unknown command '{}'. Type 'help'.\n", cmd)),
    }

    // persist sets back to state
    let _ = sets_from_state(state); // already loaded
    out
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { print!("{out}"); }
    true
}

// ── Visualization functions ───────────────────────────────────────────────────

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    c.title("Set Membership Diagram");

    let a: Vec<i64> = state_get_ints(state, "set_A").unwrap_or_else(|| vec![1,2,3]);
    let b: Vec<i64> = state_get_ints(state, "set_B").unwrap_or_else(|| vec![2,3,4]);

    // Draw two overlapping ellipses (Venn)
    let cx_a = 220.0; let cx_b = 380.0; let cy = 250.0;
    c.ellipse(cx_a, cy, 130.0, 90.0, "#42a5f520", colors::BLUE, 2.0);
    c.ellipse(cx_b, cy, 130.0, 90.0, "#66bb6a20", colors::GREEN, 2.0);
    c.text_bold(cx_a - 70.0, cy - 110.0, "A", 16.0, colors::BLUE, "middle");
    c.text_bold(cx_b + 70.0, cy - 110.0, "B", 16.0, colors::GREEN, "middle");

    let inter = intersection(&a, &b);
    let only_a = difference(&a, &b);
    let only_b = difference(&b, &a);

    // Place elements
    let elem_y_positions = [-40.0, 0.0, 40.0, 70.0];
    for (i, &x) in only_a.iter().enumerate() {
        let ey = cy + elem_y_positions.get(i).copied().unwrap_or(i as f64 * 35.0 - 50.0);
        c.circle(cx_a - 50.0, ey, 16.0, colors::HEADER_FILL, colors::BLUE, 1.5);
        c.text(cx_a - 50.0, ey, &x.to_string(), 12.0, colors::DARK, "middle");
    }
    for (i, &x) in inter.iter().enumerate() {
        let ey = cy + elem_y_positions.get(i).copied().unwrap_or(i as f64 * 35.0 - 35.0);
        let ix = (cx_a + cx_b) / 2.0;
        c.circle(ix, ey, 16.0, colors::ROW_ALT, colors::GREY, 1.5);
        c.text(ix, ey, &x.to_string(), 12.0, colors::DARK, "middle");
    }
    for (i, &x) in only_b.iter().enumerate() {
        let ey = cy + elem_y_positions.get(i).copied().unwrap_or(i as f64 * 35.0 - 50.0);
        c.circle(cx_b + 50.0, ey, 16.0, "#c8e6c9", colors::GREEN, 1.5);
        c.text(cx_b + 50.0, ey, &x.to_string(), 12.0, colors::DARK, "middle");
    }

    // Labels
    c.text(350.0, 400.0, &format!("A={} B={}", set_to_str(&a), set_to_str(&b)), 12.0, colors::GREY, "middle");
    c.text(350.0, 420.0, &format!("A∩B={} A∪B={}", set_to_str(&inter), set_to_str(&union(&a,&b))), 12.0, colors::DARK, "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], state: &StateMap) {
    let a: Vec<i64> = state_get_ints(state, "set_A").unwrap_or_else(|| vec![1,2,3]);
    let b: Vec<i64> = state_get_ints(state, "set_B").unwrap_or_else(|| vec![2,3,4]);

    g.node("A", &[("label", &format!("A = {}", set_to_str(&a))),
                   ("shape", "ellipse"), ("fillcolor", "#b2dfdb"), ("style", "filled")]);
    g.node("B", &[("label", &format!("B = {}", set_to_str(&b))),
                   ("shape", "ellipse"), ("fillcolor", "#c8e6c9"), ("style", "filled")]);

    let u = union(&a, &b);
    let i = intersection(&a, &b);
    g.node("union",  &[("label", &format!("A∪B = {}", set_to_str(&u))),  ("shape", "box"), ("style", "filled"), ("fillcolor", "#e0f2f1")]);
    g.node("inter",  &[("label", &format!("A∩B = {}", set_to_str(&i))),  ("shape", "box"), ("style", "filled"), ("fillcolor", "#f1f8e9")]);
    g.edge("A", "union", &[("label", "union")]); g.edge("B", "union", &[]);
    g.edge("A", "inter", &[("label", "inter")]); g.edge("B", "inter", &[]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], state: &StateMap) {
    let a: Vec<i64> = state_get_ints(state, "set_A").unwrap_or_else(|| vec![1,2,3]);
    let b: Vec<i64> = state_get_ints(state, "set_B").unwrap_or_else(|| vec![2,3,4]);
    let u = union(&a, &b);
    let i = intersection(&a, &b);

    t.node("A", -2.0, 0.0, &format!("$A = {}$", set_to_str(&a)), "draw,ellipse,minimum width=3cm");
    t.node("B", 2.0,  0.0, &format!("$B = {}$", set_to_str(&b)), "draw,ellipse,minimum width=3cm");
    t.node("U", -2.0, -2.0, &format!("$A\\cup B = {}$", set_to_str(&u)), "draw");
    t.node("I",  2.0, -2.0, &format!("$A\\cap B = {}$", set_to_str(&i)), "draw");
    t.arrow("A", "U", "union", "-stealth");
    t.arrow("B", "U", "", "");
    t.arrow("A", "I", "inter", "-stealth");
    t.arrow("B", "I", "", "");
}

fn visualize_ascii(a_canvas: &mut AsciiCanvas, _cmd: &str, _args: &[&str], state: &StateMap) {
    let a: Vec<i64> = state_get_ints(state, "set_A").unwrap_or_else(|| vec![1,2,3]);
    let b: Vec<i64> = state_get_ints(state, "set_B").unwrap_or_else(|| vec![2,3,4]);

    a_canvas.text_at(0, 0, "Set Membership Diagram");
    a_canvas.text_at(0, 2, &format!("A = {}", set_to_str(&a)));
    a_canvas.text_at(0, 3, &format!("B = {}", set_to_str(&b)));
    a_canvas.text_at(0, 5, &format!("A union B = {}", set_to_str(&union(&a, &b))));
    a_canvas.text_at(0, 6, &format!("A inter B = {}", set_to_str(&intersection(&a, &b))));
    a_canvas.text_at(0, 7, &format!("A \\ B    = {}", set_to_str(&difference(&a, &b))));
    a_canvas.text_at(0, 8, &format!("B \\ A    = {}", set_to_str(&difference(&b, &a))));

    // ASCII Venn
    a_canvas.text_at(2, 11, "  +--------+--------+");
    a_canvas.text_at(2, 12, "  |   A    | A/\\B  |   B");
    a_canvas.text_at(2, 13, "  |        |       |");
    let only_a = difference(&a, &b);
    let inter = intersection(&a, &b);
    let only_b = difference(&b, &a);
    let oa: Vec<String> = only_a.iter().map(|x| x.to_string()).collect();
    let ib: Vec<String> = inter.iter().map(|x| x.to_string()).collect();
    let ob: Vec<String> = only_b.iter().map(|x| x.to_string()).collect();
    a_canvas.text_at(2, 14, &format!("  | {:6} | {:5} | {}", oa.join(","), ib.join(","), ob.join(",")));
    a_canvas.text_at(2, 15, "  +--------+--------+");
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
                    let mut c = SvgCanvas::new(700.0, 500.0);
                    visualize_svg(&mut c, cmd, &args_ref, &state);
                    c.build()
                }
                OutputFormat::Dot => {
                    let mut g = DotGraph::digraph("ch02");
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
            print_banner("Chapter 2", "Sets, Relations, and Functions",
                "Explore finite sets, relations, and functions interactively");
            print_info("Type 'help' for commands.");
            print_note("Try: set A 1 2 3   then   power A");
            repl("ch2> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
