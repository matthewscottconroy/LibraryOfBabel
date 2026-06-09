use common::*;

fn show_help() -> String {
    help_string(&[
        ("roots <type>",            "All roots of A1, A2, B2, G2"),
        ("simple <type>",           "Simple roots for A1, A2, B2"),
        ("cartan_matrix <type>",    "Cartan matrix <αᵢ,αⱼ∨> for A1, A2, B2"),
        ("weyl_group <type>",       "Weyl group generators and elements for A2"),
        ("reflection <a b> / <c d>","Apply reflection s_α(β) = β - <β,α∨>α"),
        ("dynkin <type>",           "Dynkin diagram description for A1,A2,B2,G2"),
        ("demo",                    "Run a showcase of all commands"),
        ("help",                    "Show this help"),
        ("quit",                    "Exit"),
    ])
}

// Root system data. Roots stored as 2D float vectors.
fn dot(a: &[f64;2], b: &[f64;2]) -> f64 { a[0]*b[0] + a[1]*b[1] }
fn inner(a: &[f64;2], b: &[f64;2]) -> f64 { dot(a,b) }
// Cartan pairing: <β, α∨> = 2<β,α>/<α,α>
fn cartan(beta: &[f64;2], alpha: &[f64;2]) -> f64 {
    2.0 * inner(beta, alpha) / inner(alpha, alpha)
}
// Reflection: s_α(β) = β - <β,α∨>α
fn reflect(beta: &[f64;2], alpha: &[f64;2]) -> [f64;2] {
    let c = cartan(beta, alpha);
    [beta[0] - c*alpha[0], beta[1] - c*alpha[1]]
}
fn norm(v: &[f64;2]) -> f64 { inner(v,v).sqrt() }

// Returns all roots of the given type as float 2-vectors.
fn get_roots(rtype: &str) -> Option<Vec<[f64;2]>> {
    match rtype {
        "A1" => Some(vec![[1.0,0.0],[-1.0,0.0]]),
        "A2" => {
            let s3 = (3f64).sqrt() / 2.0;
            Some(vec![
                [1.0, 0.0], [-1.0, 0.0],
                [-0.5, s3], [0.5, -s3],
                [0.5, s3], [-0.5, -s3],
            ])
        }
        "B2" => {
            Some(vec![
                [1.0,0.0],[-1.0,0.0],[0.0,1.0],[0.0,-1.0],
                [1.0,1.0],[-1.0,-1.0],[1.0,-1.0],[-1.0,1.0],
            ])
        }
        "G2" => {
            let s3 = (3f64).sqrt();
            Some(vec![
                [1.0, 0.0], [-1.0, 0.0],
                [0.5, s3/2.0], [-0.5, -s3/2.0],
                [-0.5, s3/2.0], [0.5, -s3/2.0],
                [1.5, s3/2.0], [-1.5, -s3/2.0],
                [1.5, -s3/2.0], [-1.5, s3/2.0],
                [0.0, s3], [0.0, -s3],
            ])
        }
        _ => None,
    }
}

fn get_simple_roots(rtype: &str) -> Option<Vec<[f64;2]>> {
    match rtype {
        "A1" => Some(vec![[1.0, 0.0]]),
        "A2" => {
            let s3 = (3f64).sqrt() / 2.0;
            Some(vec![[1.0, 0.0], [-0.5, s3]])
        }
        "B2" => Some(vec![[1.0, -1.0], [0.0, 1.0]]),
        _ => None,
    }
}

fn fmt_root(r: &[f64;2]) -> String {
    let fmt_coord = |x: f64| -> String {
        if (x - x.round()).abs() < 1e-9 { format!("{:+.0}", x) }
        else { format!("{:+.4}", x) }
    };
    format!("({}, {})", fmt_coord(r[0]), fmt_coord(r[1]))
}

fn cmd_roots(rtype: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  ▸ Root System {}\n", rtype));
    let roots = match get_roots(rtype) {
        Some(r) => r,
        None => return "  Unknown type. Try: A1, A2, B2, G2\n".to_string(),
    };
    out.push_str(&format!("  Number of roots: {}\n\n", roots.len()));
    out.push_str(&format!("  {:20}  {:8}  {:8}\n", "Root", "Length", "|<β,α∨>|"));
    out.push_str(&format!("  {}\n", "-".repeat(50)));
    let ref_root = &roots[0];
    for r in &roots {
        let length = norm(r);
        let c = cartan(r, ref_root).abs();
        let dir = if r[0] > 0.0 || (r[0] == 0.0 && r[1] > 0.0) { "+" } else { "-" };
        out.push_str(&format!("  {:20}  {:8.4}  {:8.4}  {}\n", fmt_root(r), length, c, dir));
    }
    out.push_str("\n");
    let lengths: std::collections::BTreeSet<String> = roots.iter()
        .map(|r| format!("{:.4}", norm(r))).collect();
    if lengths.len() > 1 {
        out.push_str(&format!("  Root lengths: {}\n", lengths.into_iter().collect::<Vec<_>>().join(", ")));
        out.push_str(&format!("  {}: multiple root lengths — not simply-laced.\n", rtype));
    } else {
        out.push_str(&format!("  {}: all roots have equal length — simply-laced (ADE type).\n", rtype));
    }
    out.push_str("  Axioms: Φ ⊂ V finite, spans V, Φ∩ℝα={±α}, s_α(Φ)=Φ, <β,α∨>∈ℤ.\n");
    out
}

fn cmd_simple(rtype: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  ▸ Simple Roots of {}\n", rtype));
    let simple = match get_simple_roots(rtype) {
        Some(r) => r,
        None => return "  Simple roots known for: A1, A2, B2\n".to_string(),
    };
    let roots = get_roots(rtype).unwrap();
    out.push_str("  A simple root system Δ ⊂ Φ is a basis with all roots being\n");
    out.push_str("  non-negative or non-positive integer linear combinations of Δ.\n\n");
    for (i, s) in simple.iter().enumerate() {
        out.push_str(&format!("  α_{} = {}\n", i+1, fmt_root(s)));
    }
    out.push_str("\n  Expressing all positive roots as sums of simple roots:\n");
    for r in &roots {
        if r[0] > 1e-9 || (r[0].abs() < 1e-9 && r[1] > 1e-9) {
            let mut expressed = false;
            if simple.len() == 2 {
                let s1 = &simple[0]; let s2 = &simple[1];
                let det = s1[0]*s2[1] - s1[1]*s2[0];
                if det.abs() > 1e-9 {
                    let c1 = (r[0]*s2[1] - r[1]*s2[0]) / det;
                    let c2 = (s1[0]*r[1] - s1[1]*r[0]) / det;
                    if c1 > -1e-9 && c2 > -1e-9 {
                        let c1r = c1.round() as i32; let c2r = c2.round() as i32;
                        out.push_str(&format!("  {} = {}·α₁ + {}·α₂\n", fmt_root(r), c1r, c2r));
                        expressed = true;
                    }
                }
            } else if simple.len() == 1 {
                out.push_str(&format!("  {} = 1·α₁\n", fmt_root(r)));
                expressed = true;
            }
            if !expressed { out.push_str(&format!("  {}\n", fmt_root(r))); }
        }
    }
    out.push_str("  Every root is an integer linear combination of simple roots.\n");
    out.push_str("  Positive roots have all non-negative coefficients.\n");
    out
}

fn cmd_cartan_matrix(rtype: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  ▸ Cartan Matrix of {}\n", rtype));
    let simple = match get_simple_roots(rtype) {
        Some(r) => r,
        None => return "  Cartan matrix known for: A1, A2, B2\n".to_string(),
    };
    let n = simple.len();
    out.push_str("  Cartan matrix Aᵢⱼ = <αᵢ, αⱼ∨> = 2<αᵢ,αⱼ>/<αⱼ,αⱼ>\n\n");
    let mut mat = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            mat[i][j] = cartan(&simple[i], &simple[j]).round() as i32;
        }
    }
    out.push_str("  A = ");
    for (i, row) in mat.iter().enumerate() {
        if i > 0 { out.push_str("      "); }
        out.push_str("[");
        for (j, &v) in row.iter().enumerate() {
            if j > 0 { out.push_str("  "); }
            out.push_str(&format!("{:2}", v));
        }
        out.push_str(" ]\n");
    }
    out.push_str("\n  Cartan matrix encodes the root system completely (up to isomorphism).\n");
    out.push_str("  Diagonal entries are always 2. Off-diagonal entries ∈ {0,-1,-2,-3}.\n");
    match rtype {
        "A1" => out.push_str("  A1: [2]. Rank 1. Corresponds to sl(2).\n"),
        "A2" => {
            out.push_str("  A2: [[2,-1],[-1,2]]. Rank 2. Corresponds to sl(3).\n");
            out.push_str("  Both roots have equal length: A2 is simply-laced (ADE).\n");
        }
        "B2" => {
            out.push_str("  B2: [[2,-1],[-2,2]]. One short, one long root (ratio √2).\n");
            out.push_str("  B2 corresponds to so(5) ≅ sp(4). Not simply-laced.\n");
        }
        _ => {}
    }
    out
}

fn cmd_weyl_group(rtype: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  ▸ Weyl Group of {}\n", rtype));
    let simple = match get_simple_roots(rtype) {
        Some(r) => r,
        None => return "  Weyl group shown for: A1, A2, B2\n".to_string(),
    };
    match rtype {
        "A1" => {
            out.push_str("  W(A1) = {1, s₁} ≅ ℤ/2ℤ, order 2.\n");
            out.push_str("  s₁: α₁ ↦ -α₁  (reflection through 0)\n");
        }
        "A2" => {
            out.push_str("  W(A2) ≅ S₃ (symmetric group on 3 letters), order 6.\n");
            out.push_str("  Generators: s₁, s₂ (reflections through α₁, α₂ hyperplanes).\n");
            out.push_str("  Relations: s₁²=s₂²=1, (s₁s₂)³=1  (braid relation for A2).\n\n");
            out.push_str("  Elements of W(A2) and their action on simple roots:\n");
            let s1 = &simple[0]; let s2 = &simple[1];
            let elements: Vec<(&str, Vec<[f64;2]>)> = vec![
                ("1", simple.clone()),
                ("s₁", vec![reflect(s1,s1), reflect(s2,s1)]),
                ("s₂", vec![reflect(s1,s2), reflect(s2,s2)]),
                ("s₁s₂", {let a=reflect(s1,s2); let b=reflect(s2,s2); vec![reflect(&a,s1),reflect(&b,s1)]}),
                ("s₂s₁", {let a=reflect(s1,s1); let b=reflect(s2,s1); vec![reflect(&a,s2),reflect(&b,s2)]}),
                ("s₁s₂s₁", {
                    let a=reflect(s1,s2); let b=reflect(s2,s2);
                    let c=reflect(&a,s1); let d=reflect(&b,s1);
                    vec![reflect(&c,s2), reflect(&d,s2)]
                }),
            ];
            for (name, imgs) in &elements {
                out.push_str(&format!("  {}: α₁↦{}, α₂↦{}\n", name, fmt_root(&imgs[0]), fmt_root(&imgs[1])));
            }
            out.push_str("  W(A2) ≅ S₃ permutes the 3 roots {±α₁, ±α₂, ±(α₁+α₂)} in pairs.\n");
        }
        "B2" => {
            out.push_str("  W(B2) ≅ Dih₄ (dihedral group of order 8), order 8.\n");
            out.push_str("  Generated by s₁,s₂: s₁²=s₂²=1, (s₁s₂)⁴=1.\n");
        }
        _ => return "  Weyl group shown for: A1, A2, B2\n".to_string(),
    }
    out
}

fn cmd_reflection(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Reflection s_α(β) = β - <β,α∨>·α\n");
    let seps: Vec<usize> = args.iter().enumerate().filter(|(_,&s)|s=="/").map(|(i,_)|i).collect();
    if seps.is_empty() { return "  Usage: reflection <a b> / <c d>  (α = (a,b), β = (c,d))\n".to_string(); }
    let parse2 = |off: usize| -> Option<[f64;2]> {
        let v: Vec<f64> = args.iter().skip(off).take(2).filter_map(|s|s.parse().ok()).collect();
        if v.len()<2 { None } else { Some([v[0],v[1]]) }
    };
    let alpha = match parse2(0) { Some(v)=>v, None=>return "  Bad α\n".to_string() };
    let beta  = match parse2(seps[0]+1) { Some(v)=>v, None=>return "  Bad β\n".to_string() };
    out.push_str(&format!("  α = {}  (root)\n", fmt_root(&alpha)));
    out.push_str(&format!("  β = {}  (vector to reflect)\n", fmt_root(&beta)));
    let c = cartan(&beta, &alpha);
    out.push_str(&format!("  <β,α∨> = 2<β,α>/<α,α> = {:.4}\n", c));
    let result = reflect(&beta, &alpha);
    out.push_str(&format!("  s_α(β) = β - {:.4}·α = {}\n", c, fmt_root(&result)));
    if (c - c.round()).abs() < 1e-9 {
        out.push_str(&format!("  <β,α∨> = {} ∈ ℤ  (integrality condition satisfied)\n", c.round() as i32));
    } else {
        out.push_str(&format!("  <β,α∨> = {:.4} is not an integer — not roots of the same system\n", c));
    }
    out.push_str("  s_α is an orthogonal reflection: s_α(α)=-α, s_α fixes the hyperplane ⊥α.\n");
    out
}

fn cmd_dynkin(rtype: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  ▸ Dynkin Diagram of {}\n", rtype));
    match rtype {
        "A1" => {
            out.push_str("  ○\n  α₁\n\n");
            out.push_str("  Single node. Rank 1. Corresponds to sl(2) and SU(2).\n");
        }
        "A2" => {
            out.push_str("  ○───○\n  α₁  α₂\n\n");
            out.push_str("  Two nodes connected by a single edge (both roots have equal length).\n");
            out.push_str("  Single edge ↔ <α₁,α₂∨>=<α₂,α₁∨>=-1 (angle 120°).\n");
            out.push_str("  Corresponds to sl(3) and SU(3).\n");
        }
        "B2" => {
            out.push_str("  ○═══>○\n  α₁   α₂\n\n");
            out.push_str("  Two nodes connected by a double edge with arrow pointing to SHORT root.\n");
            out.push_str("  Double edge ↔ <α₁,α₂∨>=-1, <α₂,α₁∨>=-2  (roots of different lengths).\n");
            out.push_str("  Arrow points from long → short root.\n");
            out.push_str("  Corresponds to so(5) and Sp(4).\n");
        }
        "G2" => {
            out.push_str("  ○≡≡≡>○\n  α₁    α₂\n\n");
            out.push_str("  Triple edge with arrow: <α₁,α₂∨>=-1, <α₂,α₁∨>=-3.\n");
            out.push_str("  G2 is an exceptional Lie algebra, dim=14, rank=2.\n");
            out.push_str("  Long:short root length ratio = √3.\n");
        }
        _ => return "  Dynkin diagrams for: A1, A2, B2, G2\n".to_string(),
    }
    out.push_str("\n  Dynkin diagrams classify all simple complex Lie algebras:\n");
    out.push_str("    ADE (simply-laced): Aₙ,Dₙ,E₆,E₇,E₈  — single edges only\n");
    out.push_str("    BCF (non-laced):    Bₙ,Cₙ,F₄          — double edges\n");
    out.push_str("    Exceptional:        G₂                  — triple edge\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_str(&mut s, "root_system", "A2");
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "help" | "h" => show_help(),
        "roots" => {
            let t: String = args.get(0).map(|s| s.to_string())
                .unwrap_or_else(|| state_get_str(state, "root_system").unwrap_or("A2").to_string());
            state_set_str(state, "root_system", &t);
            cmd_roots(&t)
        }
        "simple" => {
            let t: String = args.get(0).map(|s| s.to_string())
                .unwrap_or_else(|| state_get_str(state, "root_system").unwrap_or("A2").to_string());
            state_set_str(state, "root_system", &t);
            cmd_simple(&t)
        }
        "cartan_matrix" => {
            let t: String = args.get(0).map(|s| s.to_string())
                .unwrap_or_else(|| state_get_str(state, "root_system").unwrap_or("A2").to_string());
            state_set_str(state, "root_system", &t);
            cmd_cartan_matrix(&t)
        }
        "weyl_group" => {
            let t: String = args.get(0).map(|s| s.to_string())
                .unwrap_or_else(|| state_get_str(state, "root_system").unwrap_or("A2").to_string());
            state_set_str(state, "root_system", &t);
            cmd_weyl_group(&t)
        }
        "reflection" => {
            state_set_str(state, "last_cmd", "reflection");
            cmd_reflection(args)
        }
        "dynkin" => {
            let t: String = args.get(0).map(|s| s.to_string())
                .unwrap_or_else(|| state_get_str(state, "root_system").unwrap_or("A2").to_string());
            state_set_str(state, "root_system", &t);
            cmd_dynkin(&t)
        }
        "demo" => {
            let mut out = cmd_roots("A2");
            out.push_str(&cmd_cartan_matrix("B2"));
            out.push_str(&cmd_weyl_group("A2"));
            out.push_str(&cmd_dynkin("G2"));
            out
        }
        _ => format!("Unknown command '{}'. Type 'help'.\n", cmd),
    }
}

fn run_cmd_bool(cmd: &str, args: &[&str], state: &mut StateMap) -> bool {
    if cmd == "quit" || cmd == "exit" || cmd == "q" { return false; }
    let out = run_cmd(cmd, args, state);
    if !out.is_empty() { println!("{}", out); }
    true
}

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, args: &[&str], state: &StateMap) {
    // Root system diagram — plot roots as arrows from origin
    let rtype = args.get(0).copied()
        .unwrap_or_else(|| state_get_str(state, "root_system").unwrap_or("A2"));
    c.text_bold(350.0, 30.0, &format!("Root System {}", rtype), 16.0, "#212121", "middle");
    let cx = 350.0_f64;
    let cy = 260.0_f64;
    let scale = 120.0_f64;
    // Draw axes
    c.arrow(cx - scale*1.5, cy, cx + scale*1.5, cy, "#aaaaaa", 1.5);
    c.arrow(cx, cy + scale*1.5, cx, cy - scale*1.5, "#aaaaaa", 1.5);
    // Draw roots
    if let Some(roots) = get_roots(rtype) {
        for r in &roots {
            let x2 = cx + r[0] * scale;
            let y2 = cy - r[1] * scale;
            let color = if r[0] > 0.0 || (r[0] == 0.0 && r[1] > 0.0) {
                "#4488ff"
            } else {
                "#ff4444"
            };
            c.arrow(cx, cy, x2, y2, color, 2.0);
            c.text(x2 + 8.0, y2, &fmt_root(r), 11.0, "#333333", "start");
        }
    }
    c.text(350.0, 480.0, "Blue = positive roots, Red = negative roots", 11.0, "#666666", "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, args: &[&str], state: &StateMap) {
    let rtype = args.get(0).copied()
        .unwrap_or_else(|| state_get_str(state, "root_system").unwrap_or("A2"));
    g.node("origin", &[("label", "0"), ("shape", "point")]);
    if let Some(roots) = get_roots(rtype) {
        for (i, r) in roots.iter().enumerate() {
            let id = format!("r{}", i);
            let lbl = fmt_root(r);
            g.node(&id, &[("label", &lbl), ("shape", "circle")]);
            g.edge("origin", &id, &[]);
        }
    }
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, args: &[&str], state: &StateMap) {
    let rtype = args.get(0).copied()
        .unwrap_or_else(|| state_get_str(state, "root_system").unwrap_or("A2"));
    if let Some(simple) = get_simple_roots(rtype) {
        for (i, s) in simple.iter().enumerate() {
            let id = format!("a{}", i+1);
            let lbl = format!("α_{}", i+1);
            t.node(&id, s[0] * 2.0, s[1] * 2.0, &lbl, "circle,draw");
        }
        if simple.len() >= 2 {
            t.arrow("a1", "a2", "", "");
        }
    }
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, args: &[&str], state: &StateMap) {
    let rtype = args.get(0).copied()
        .unwrap_or_else(|| state_get_str(state, "root_system").unwrap_or("A2"));
    a.text_at(2, 1, &format!("Root System {} — Dynkin Diagram", rtype));
    match rtype {
        "A1" => { a.text_at(10, 5, "O"); a.text_at(10, 7, "a1"); }
        "A2" => {
            a.text_at(10, 5, "O---O");
            a.text_at(10, 7, "a1  a2");
            a.text_at(2, 10, "Simply laced: all roots equal length");
        }
        "B2" => {
            a.text_at(10, 5, "O==>O");
            a.text_at(10, 7, "a1  a2");
            a.text_at(2, 10, "Double edge: two root lengths");
        }
        "G2" => {
            a.text_at(10, 5, "O===>O");
            a.text_at(10, 7, "a1   a2");
            a.text_at(2, 10, "Triple edge: exceptional, dim=14");
        }
        _ => { a.text_at(2, 5, "Unknown root system type"); }
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
                OutputFormat::Svg   => { let mut c = SvgCanvas::new(700.0, 500.0); visualize_svg(&mut c, cmd, &args_ref, &state); c.build() }
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch49"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 49", "Root Systems",
                         "The combinatorial skeleton of semisimple Lie algebras");
            repl("ch49> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
