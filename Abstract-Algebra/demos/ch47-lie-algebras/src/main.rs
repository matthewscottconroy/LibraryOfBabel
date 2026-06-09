use common::*;

fn show_help() -> String {
    help_string(&[
        ("bracket <a b c d> / <e f g h>",   "Lie bracket [X,Y]=XY-YX for 2×2 matrices"),
        ("sl2",                              "Generators e,f,h of sl(2) and bracket relations"),
        ("jacobi <a b c d> / <e f g h> / <i j k l>", "Verify Jacobi identity"),
        ("adjoint <a> <b> <c>",             "Adjoint rep of sl(2): ad(ae+bf+ch) as 3×3 matrix"),
        ("killing <a1 b1 c1> / <a2 b2 c2>", "Killing form B(X,Y) = Tr(ad(X)∘ad(Y))"),
        ("cartan_criterion",                "Killing form non-degeneracy → sl(2) semisimple"),
        ("demo",                            "Run a showcase of all commands"),
        ("help",                            "Show this help"),
        ("quit",                            "Exit"),
    ])
}

type M2 = [[f64; 2]; 2];
type M3 = [[f64; 3]; 3];

fn m2_mul(a: &M2, b: &M2) -> M2 {
    let mut r = [[0f64;2];2];
    for i in 0..2 { for j in 0..2 { for k in 0..2 { r[i][j] += a[i][k]*b[k][j]; } } }
    r
}
fn m2_sub(a: &M2, b: &M2) -> M2 {
    [[a[0][0]-b[0][0], a[0][1]-b[0][1]], [a[1][0]-b[1][0], a[1][1]-b[1][1]]]
}
fn m2_add(a: &M2, b: &M2) -> M2 {
    [[a[0][0]+b[0][0], a[0][1]+b[0][1]], [a[1][0]+b[1][0], a[1][1]+b[1][1]]]
}
fn m2_scale(a: &M2, s: f64) -> M2 {
    [[a[0][0]*s, a[0][1]*s], [a[1][0]*s, a[1][1]*s]]
}
fn bracket(x: &M2, y: &M2) -> M2 { m2_sub(&m2_mul(x, y), &m2_mul(y, x)) }
fn m2_norm(m: &M2) -> f64 {
    m.iter().flat_map(|r| r.iter()).map(|x| x*x).sum::<f64>().sqrt()
}

fn fmt_m2(label: &str, m: &M2) -> String {
    let mut s = format!("  {}:\n", label);
    s.push_str(&format!("    [ {:8.4}  {:8.4} ]\n", m[0][0], m[0][1]));
    s.push_str(&format!("    [ {:8.4}  {:8.4} ]\n", m[1][0], m[1][1]));
    s
}
fn fmt_m3(label: &str, m: &M3) -> String {
    let mut s = format!("  {}:\n", label);
    for row in m { s.push_str(&format!("    [ {:7.4}  {:7.4}  {:7.4} ]\n", row[0], row[1], row[2])); }
    s
}

fn m3_mul(a: &M3, b: &M3) -> M3 {
    let mut r = [[0f64;3];3];
    for i in 0..3 { for j in 0..3 { for k in 0..3 { r[i][j] += a[i][k]*b[k][j]; } } }
    r
}
fn m3_trace(m: &M3) -> f64 { m[0][0]+m[1][1]+m[2][2] }

fn gen_e() -> M2 { [[0.0,1.0],[0.0,0.0]] }
fn gen_f() -> M2 { [[0.0,0.0],[1.0,0.0]] }
fn gen_h() -> M2 { [[1.0,0.0],[0.0,-1.0]] }

fn parse_m2_at(args: &[&str], offset: usize) -> Option<M2> {
    let v: Vec<f64> = args.iter().skip(offset).take(4)
        .filter_map(|s| s.parse().ok()).collect();
    if v.len() < 4 { return None; }
    Some([[v[0],v[1]],[v[2],v[3]]])
}

fn find_slashes(args: &[&str]) -> Vec<usize> {
    args.iter().enumerate().filter(|(_,&s)| s=="/").map(|(i,_)| i).collect()
}

// Adjoint rep of sl(2) in basis {e, f, h}
fn adjoint_sl2(a: f64, b: f64, c: f64) -> M3 {
    [
        [ 2.0*c,  0.0,   -2.0*a ],
        [ 0.0,   -2.0*c,  2.0*b ],
        [-b,     -a,      0.0   ],
    ]
}

fn cmd_bracket(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Lie Bracket [X,Y] = XY - YX\n");
    let seps = find_slashes(args);
    if seps.is_empty() {
        return "Usage: bracket <a b c d> / <e f g h>\n".to_string();
    }
    let x = match parse_m2_at(args, 0) { Some(m)=>m, None=>return "Bad matrix X\n".to_string() };
    let y = match parse_m2_at(args, seps[0]+1) { Some(m)=>m, None=>return "Bad matrix Y\n".to_string() };
    out.push_str(&fmt_m2("X", &x));
    out.push_str(&fmt_m2("Y", &y));
    let comm = bracket(&x, &y);
    out.push_str(&fmt_m2("[X,Y] = XY - YX", &comm));
    if m2_norm(&comm) < 1e-9 {
        out.push_str("  X and Y commute: [X,Y] = 0.\n");
    }
    let anti = m2_add(&comm, &bracket(&y, &x));
    out.push_str(&format!("  antisymmetry check |[X,Y]+[Y,X]| = {:.2e}\n", m2_norm(&anti)));
    out.push_str("  The Lie bracket [X,Y]=XY-YX satisfies bilinearity, antisymmetry, and Jacobi.\n");
    out
}

fn cmd_sl2() -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ sl(2,ℝ) — Lie Algebra of SL(2,ℝ)\n");
    let e = gen_e(); let f = gen_f(); let h = gen_h();
    out.push_str(&fmt_m2("e = [[0,1],[0,0]]", &e));
    out.push_str(&fmt_m2("f = [[0,0],[1,0]]", &f));
    out.push_str(&fmt_m2("h = [[1,0],[0,-1]]", &h));
    out.push_str("\n  Structure constants:\n");
    let he = bracket(&h, &e);
    let hf = bracket(&h, &f);
    let ef = bracket(&e, &f);
    out.push_str(&fmt_m2("[h,e]", &he));
    out.push_str(&format!("  [h,e] = 2e  (error: {:.2e})\n", m2_norm(&m2_sub(&he, &m2_scale(&e,2.0)))));
    out.push_str(&fmt_m2("[h,f]", &hf));
    out.push_str(&format!("  [h,f] = -2f  (error: {:.2e})\n", m2_norm(&m2_sub(&hf, &m2_scale(&f,-2.0)))));
    out.push_str(&fmt_m2("[e,f]", &ef));
    out.push_str(&format!("  [e,f] = h  (error: {:.2e})\n", m2_norm(&m2_sub(&ef, &h))));
    out.push_str("\n  sl(2) is the unique 3-dim simple Lie algebra over ℝ.\n");
    out.push_str("  h is the Cartan subalgebra (maximal abelian, semisimple element).\n");
    out.push_str("  [h,e]=2e, [h,f]=-2f: e and f are root vectors for roots α=2 and -α=-2.\n");
    out
}

fn cmd_jacobi(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Jacobi Identity: [X,[Y,Z]] + [Y,[Z,X]] + [Z,[X,Y]] = 0\n");
    let seps = find_slashes(args);
    if seps.len() < 2 {
        return "Usage: jacobi <a b c d> / <e f g h> / <i j k l>\n".to_string();
    }
    let x = match parse_m2_at(args, 0) { Some(m)=>m, None=>return "Bad X\n".to_string() };
    let y = match parse_m2_at(args, seps[0]+1) { Some(m)=>m, None=>return "Bad Y\n".to_string() };
    let z = match parse_m2_at(args, seps[1]+1) { Some(m)=>m, None=>return "Bad Z\n".to_string() };
    out.push_str(&fmt_m2("X", &x));
    out.push_str(&fmt_m2("Y", &y));
    out.push_str(&fmt_m2("Z", &z));
    let t1 = bracket(&x, &bracket(&y, &z));
    let t2 = bracket(&y, &bracket(&z, &x));
    let t3 = bracket(&z, &bracket(&x, &y));
    let sum = m2_add(&m2_add(&t1, &t2), &t3);
    out.push_str(&fmt_m2("[X,[Y,Z]]", &t1));
    out.push_str(&fmt_m2("[Y,[Z,X]]", &t2));
    out.push_str(&fmt_m2("[Z,[X,Y]]", &t3));
    out.push_str(&fmt_m2("Sum", &sum));
    let err = m2_norm(&sum);
    if err < 1e-9 {
        out.push_str(&format!("  Jacobi identity holds! (error = {:.2e})\n", err));
    } else {
        out.push_str(&format!("  Jacobi error = {:.2e}\n", err));
    }
    out.push_str("  Jacobi identity encodes that ad: g → gl(g) is a Lie algebra homomorphism.\n");
    out
}

fn cmd_adjoint(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Adjoint Representation ad: sl(2) → gl(3)\n");
    let v: Vec<f64> = args.iter().take(3).filter_map(|s|s.parse().ok()).collect();
    if v.len() < 3 { return "Usage: adjoint <a> <b> <c>  for X=a·e+b·f+c·h\n".to_string(); }
    let (a,b,c) = (v[0],v[1],v[2]);
    out.push_str(&format!("  X = {}·e + {}·f + {}·h  in sl(2)\n", a, b, c));
    let ad = adjoint_sl2(a, b, c);
    out.push_str(&fmt_m3("ad(X) [columns: ad(X)e, ad(X)f, ad(X)h]", &ad));
    let tr = ad[0][0]+ad[1][1]+ad[2][2];
    out.push_str(&format!("  tr(ad(X)) = {:.4}\n", tr));
    out.push_str("  tr(ad(X))=0 for all X ∈ sl(2): consequence of semisimplicity ([g,g]=g).\n");
    // Check ad is a homomorphism on generators
    let ad_h = adjoint_sl2(0.0,0.0,1.0);
    let ad_e = adjoint_sl2(1.0,0.0,0.0);
    let lhs = adjoint_sl2(0.0,0.0,2.0);
    let rhs_comm: M3 = {
        let he = m3_mul(&ad_h, &ad_e);
        let eh = m3_mul(&ad_e, &ad_h);
        let mut r = [[0f64;3];3];
        for i in 0..3 { for j in 0..3 { r[i][j] = he[i][j]-eh[i][j]; } }
        r
    };
    let diff: f64 = (0..3).flat_map(|i|(0..3).map(move |j| (lhs[i][j]-rhs_comm[i][j]).abs())).fold(0f64,f64::max);
    out.push_str(&format!("  ad([h,e]) = [ad(h),ad(e)]  (max error: {:.2e})\n", diff));
    out
}

fn cmd_killing(args: &[&str]) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Killing Form B(X,Y) = Tr(ad(X) ∘ ad(Y))\n");
    let seps = find_slashes(args);
    if seps.is_empty() { return "Usage: killing <a1 b1 c1> / <a2 b2 c2>\n".to_string(); }
    let parse3 = |offset: usize| -> Option<(f64,f64,f64)> {
        let v: Vec<f64> = args.iter().skip(offset).take(3).filter_map(|s|s.parse().ok()).collect();
        if v.len()<3 { None } else { Some((v[0],v[1],v[2])) }
    };
    let (a1,b1,c1) = match parse3(0) { Some(v)=>v, None=>return "Bad first triple\n".to_string() };
    let (a2,b2,c2) = match parse3(seps[0]+1) { Some(v)=>v, None=>return "Bad second triple\n".to_string() };
    out.push_str(&format!("  X = {}·e + {}·f + {}·h\n", a1, b1, c1));
    out.push_str(&format!("  Y = {}·e + {}·f + {}·h\n", a2, b2, c2));
    let adx = adjoint_sl2(a1, b1, c1);
    let ady = adjoint_sl2(a2, b2, c2);
    let prod = m3_mul(&adx, &ady);
    out.push_str(&fmt_m3("ad(X)", &adx));
    out.push_str(&fmt_m3("ad(Y)", &ady));
    out.push_str(&fmt_m3("ad(X)∘ad(Y)", &prod));
    let kf = m3_trace(&prod);
    out.push_str(&format!("  B(X,Y) = {:.4}\n", kf));
    let formula = 8.0*c1*c2 + 4.0*(b1*a2 + a1*b2);
    out.push_str(&format!("  Formula: 8c₁c₂ + 4(b₁a₂+a₁b₂) = {:.4}\n", formula));
    out.push_str("\n  Full Killing form matrix in basis {e,f,h}:\n");
    out.push_str("       e    f    h\n");
    out.push_str("  e  [ 0    4    0 ]\n");
    out.push_str("  f  [ 4    0    0 ]\n");
    out.push_str("  h  [ 0    0    8 ]\n");
    out.push_str("  det(B) = -128  (≠0 ⟹ semisimple)\n");
    out.push_str("  Cartan's criterion: g semisimple ⟺ Killing form non-degenerate.\n");
    out
}

fn cmd_cartan_criterion() -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Cartan's Criterion: sl(2) is Semisimple\n");
    out.push_str("  Killing form matrix in basis {e,f,h}:\n");
    out.push_str("    B = [  0  4  0 ]\n");
    out.push_str("        [  4  0  0 ]\n");
    out.push_str("        [  0  0  8 ]\n");
    out.push_str("  det(B) = -128  (non-zero)\n");
    out.push_str("  Non-degenerate Killing form ⟹ sl(2) is semisimple.\n\n");
    out.push_str("  sl(2) is in fact simple:\n");
    out.push_str("  Any ideal I ⊆ sl(2) is ad(h)-stable, so I is spanned by root vectors.\n");
    out.push_str("  If e ∈ I: [f,e]=-h ∈ I, then [h,e]=2e ∈ I ⟹ I=sl(2).\n");
    out.push_str("  Every non-zero ideal is all of sl(2): sl(2) is simple.\n\n");
    out.push_str("  Over ℂ, simple Lie algebras: Aₙ=sl(n+1), Bₙ=so(2n+1), Cₙ=sp(2n), Dₙ=so(2n),\n");
    out.push_str("  and exceptionals G₂,F₄,E₆,E₇,E₈. sl(2)≅A₁ is the simplest.\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_str(&mut s, "algebra", "sl2");
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "help" | "h" => show_help(),
        "bracket" => {
            state_set_str(state, "last_cmd", "bracket");
            cmd_bracket(args)
        }
        "sl2" => {
            state_set_str(state, "algebra", "sl2");
            cmd_sl2()
        }
        "jacobi" => {
            state_set_str(state, "last_cmd", "jacobi");
            cmd_jacobi(args)
        }
        "adjoint" => {
            if let Some(&a) = args.get(0) { state_set_str(state, "adjoint_a", a); }
            if let Some(&b) = args.get(1) { state_set_str(state, "adjoint_b", b); }
            if let Some(&c) = args.get(2) { state_set_str(state, "adjoint_c", c); }
            cmd_adjoint(args)
        }
        "killing" => {
            state_set_str(state, "last_cmd", "killing");
            cmd_killing(args)
        }
        "cartan_criterion" => cmd_cartan_criterion(),
        "demo" => {
            let mut out = cmd_sl2();
            out.push_str(&cmd_bracket(&["0","1","0","0","/","0","0","1","0"]));
            out.push_str(&cmd_adjoint(&["1","0","0"]));
            out.push_str(&cmd_cartan_criterion());
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

fn visualize_svg(c: &mut SvgCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    // Lie bracket diagram: [X,Y] = XY - YX
    c.text_bold(350.0, 40.0, "Lie Algebra sl(2) Bracket Structure", 16.0, "#212121", "middle");
    // Three generators
    c.node_circle(200.0, 150.0, "e", "#b2ebf2", 24.0, 13.0);
    c.node_circle(350.0, 300.0, "h", "#ffd54f", 24.0, 13.0);
    c.node_circle(500.0, 150.0, "f", "#ffccbc", 24.0, 13.0);
    // Bracket arrows with labels
    c.arrow(200.0, 150.0, 350.0, 300.0, "#666666", 1.5);
    c.text(255.0, 240.0, "[h,e]=2e", 11.0, "#444444", "middle");
    c.arrow(500.0, 150.0, 350.0, 300.0, "#666666", 1.5);
    c.text(455.0, 240.0, "[h,f]=-2f", 11.0, "#444444", "middle");
    c.arrow(200.0, 150.0, 500.0, 150.0, "#666666", 1.5);
    c.text(350.0, 130.0, "[e,f]=h", 11.0, "#444444", "middle");
    // Killing form
    c.text_bold(350.0, 400.0, "Killing Form B(X,Y) = Tr(ad(X)∘ad(Y))", 13.0, "#212121", "middle");
    c.text(350.0, 430.0, "B non-degenerate ⟹ sl(2) semisimple", 11.0, "#555555", "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("e", &[("label", "e (raise)"), ("shape", "circle")]);
    g.node("f", &[("label", "f (lower)"), ("shape", "circle")]);
    g.node("h", &[("label", "h (Cartan)"), ("shape", "circle")]);
    g.edge("h", "e", &[("label", "[h,e]=2e")]);
    g.edge("h", "f", &[("label", "[h,f]=-2f")]);
    g.edge("e", "f", &[("label", "[e,f]=h")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("e", 0.0, 2.0, "e", "circle,draw");
    t.node("h", 2.0, 0.0, "h", "circle,draw");
    t.node("f", 4.0, 2.0, "f", "circle,draw");
    t.arrow("h", "e", "[h,e]=2e", "");
    t.arrow("h", "f", "[h,f]=-2f", "");
    t.arrow("e", "f", "[e,f]=h", "bend left");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1, "sl(2) Lie algebra: generators and brackets");
    a.text_at(10, 5, "e");
    a.text_at(35, 5, "f");
    a.text_at(22, 12, "h");
    a.text_at(12, 8, "[e,f] = h");
    a.text_at(2, 15, "[h,e] = 2e    [h,f] = -2f");
    a.text_at(2, 18, "Killing form: B(e,f)=4, B(h,h)=8 => semisimple");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch47"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 47", "Lie Algebras", "Linearising symmetry at the identity");
            repl("ch47> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
