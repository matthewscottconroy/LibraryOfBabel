use common::*;

fn show_help() -> String {
    help_string(&[
        ("so2 <theta_deg>",                   "Rotation matrix in SO(2), verify det=1 and Rᵀ=R⁻¹"),
        ("su2 <a_re> <a_im> <b_re> <b_im>",   "SU(2) matrix from (a,b), verify unitarity"),
        ("gl2 <a> <b> <c> <d>",               "GL(2,ℝ) element: determinant, Lie algebra log"),
        ("exp_mat <a> <b> <c> <d>",           "Matrix exponential of 2×2 via power series"),
        ("one_param <a> <b> <c> <d> <t>",     "One-parameter subgroup exp(tX) for tangent X"),
        ("cover",                              "SU(2) → SO(3) double cover explained"),
        ("demo",                               "Run a showcase of all commands"),
        ("help",                               "Show this help"),
        ("quit",                               "Exit"),
    ])
}

// 2×2 float matrix helpers
fn fmat2(a: f64, b: f64, c: f64, d: f64) -> [[f64; 2]; 2] {
    [[a, b], [c, d]]
}
fn fmat_mul(m: &[[f64; 2]; 2], n: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    let mut r = [[0f64; 2]; 2];
    for i in 0..2 { for j in 0..2 { for k in 0..2 { r[i][j] += m[i][k] * n[k][j]; } } }
    r
}
fn fmat_add(m: &[[f64; 2]; 2], n: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    let mut r = [[0f64; 2]; 2];
    for i in 0..2 { for j in 0..2 { r[i][j] = m[i][j] + n[i][j]; } }
    r
}
fn fmat_scale(m: &[[f64; 2]; 2], s: f64) -> [[f64; 2]; 2] {
    let mut r = [[0f64; 2]; 2];
    for i in 0..2 { for j in 0..2 { r[i][j] = m[i][j] * s; } }
    r
}
fn fmat_det(m: &[[f64; 2]; 2]) -> f64 { m[0][0]*m[1][1] - m[0][1]*m[1][0] }
fn fmat_trace(m: &[[f64; 2]; 2]) -> f64 { m[0][0] + m[1][1] }
fn fmat_transpose(m: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    [[m[0][0], m[1][0]], [m[0][1], m[1][1]]]
}
fn fmat_inv(m: &[[f64; 2]; 2]) -> Option<[[f64; 2]; 2]> {
    let d = fmat_det(m);
    if d.abs() < 1e-12 { return None; }
    Some(fmat2(m[1][1]/d, -m[0][1]/d, -m[1][0]/d, m[0][0]/d))
}
fn fmat_identity() -> [[f64; 2]; 2] { [[1.0, 0.0], [0.0, 1.0]] }
fn fmt_fmat2(label: &str, m: &[[f64; 2]; 2]) -> String {
    let mut s = format!("  {}:\n", label);
    for i in 0..2 {
        s.push_str(&format!("    [ {:8.4}  {:8.4} ]\n", m[i][0], m[i][1]));
    }
    s
}

const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_RESET: &str = "\x1b[0m";

// Matrix exponential via truncated power series: exp(M) = I + M + M²/2! + ... (20 terms)
fn mat_exp(m: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    let mut result = fmat_identity();
    let mut term = fmat_identity();
    for k in 1..=20usize {
        term = fmat_mul(&term, m);
        term = fmat_scale(&term, 1.0 / k as f64);
        result = fmat_add(&result, &term);
    }
    result
}

// Matrix logarithm of M near identity: log(M) = (M-I) - (M-I)²/2 + (M-I)³/3 - ...
fn mat_log_near_id(m: &[[f64; 2]; 2]) -> [[f64; 2]; 2] {
    let id = fmat_identity();
    let a = fmat_add(m, &fmat_scale(&id, -1.0));
    let mut result = [[0f64; 2]; 2];
    let mut power = fmat_identity();
    for k in 1..=30usize {
        power = fmat_mul(&power, &a);
        let sign = if k % 2 == 0 { -1.0 } else { 1.0 };
        let term = fmat_scale(&power, sign / k as f64);
        result = fmat_add(&result, &term);
    }
    result
}

fn cmd_so2(theta_deg: f64) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  ▸ SO(2) — Special Orthogonal Group in 2D\n"));
    let theta = theta_deg * std::f64::consts::PI / 180.0;
    let r = fmat2(theta.cos(), -theta.sin(), theta.sin(), theta.cos());
    out.push_str(&format!("  θ = {}°  ({:.6} rad)\n", theta_deg, theta));
    out.push_str(&fmt_fmat2("R(θ)", &r));
    let det = fmat_det(&r);
    out.push_str(&format!("  det(R) = {:.6}\n", det));
    out.push_str("  det = 1 confirms R ∈ SL(2,ℝ). Orthogonality: Rᵀ = R⁻¹.\n");
    let rt = fmat_transpose(&r);
    let rinv = fmat_inv(&r).unwrap();
    out.push_str(&fmt_fmat2("Rᵀ", &rt));
    out.push_str(&fmt_fmat2("R⁻¹", &rinv));
    let diff_max = (0..2).flat_map(|i| (0..2).map(move |j| (rt[i][j] - rinv[i][j]).abs())).fold(0f64, f64::max);
    out.push_str(&format!("  max |Rᵀ - R⁻¹| = {:.2e}\n", diff_max));
    out.push_str("  SO(2) ≅ S¹ (the circle group). Lie algebra so(2) = span{[[0,-1],[1,0]]}.\n");
    out
}

fn cmd_su2(are: f64, aim: f64, bre: f64, bim: f64) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ SU(2) — Special Unitary Group\n");
    let norm_sq = are*are + aim*aim + bre*bre + bim*bim;
    out.push_str(&format!("  a = {:.4} + {:.4}i,  b = {:.4} + {:.4}i\n", are, aim, bre, bim));
    out.push_str(&format!("  |a|² + |b|² = {:.6}\n", norm_sq));
    if (norm_sq - 1.0).abs() > 0.01 {
        out.push_str(&format!("  Not a valid SU(2) element: |a|²+|b|² = {:.4} ≠ 1. Normalising...\n", norm_sq));
    }
    let scale = norm_sq.sqrt();
    let (a_re, a_im, b_re, b_im) = (are/scale, aim/scale, bre/scale, bim/scale);
    out.push_str(&format!("  Normalised SU(2) element (complex 2×2):\n"));
    out.push_str(&format!("    ┌                                              ┐\n"));
    out.push_str(&format!("    │  ({:+.4},{:+.4}i)  ({:+.4},{:+.4}i)  │\n", a_re, a_im, -b_re, b_im));
    out.push_str(&format!("    │  ({:+.4},{:+.4}i)  ({:+.4},{:+.4}i)  │\n", b_re, b_im, a_re, -a_im));
    out.push_str(&format!("    └                                              ┘\n"));
    let det_re = a_re*a_re + a_im*a_im + b_re*b_re + b_im*b_im;
    out.push_str(&format!("  det (should be 1) = {:.6}\n", det_re));
    out.push_str("  SU(2) = {M ∈ M₂(ℂ) | M†M = I, det M = 1}. As a manifold it is S³.\n");
    out.push_str("  SU(2) double-covers SO(3): ker = {±I}.\n");
    let w = a_re; let x = a_im; let y = b_re; let z = b_im;
    out.push_str(&format!("  Unit quaternion q = ({:.4}) + ({:.4})i + ({:.4})j + ({:.4})k\n", w, x, y, z));
    let r00 = 1.0 - 2.0*(y*y+z*z); let r01 = 2.0*(x*y-z*w); let r02 = 2.0*(x*z+y*w);
    let r10 = 2.0*(x*y+z*w); let r11 = 1.0 - 2.0*(x*x+z*z); let r12 = 2.0*(y*z-x*w);
    let r20 = 2.0*(x*z-y*w); let r21 = 2.0*(y*z+x*w); let r22 = 1.0 - 2.0*(x*x+y*y);
    out.push_str("  SO(3) matrix R:\n");
    out.push_str(&format!("    [ {:7.4}  {:7.4}  {:7.4} ]\n", r00, r01, r02));
    out.push_str(&format!("    [ {:7.4}  {:7.4}  {:7.4} ]\n", r10, r11, r12));
    out.push_str(&format!("    [ {:7.4}  {:7.4}  {:7.4} ]\n", r20, r21, r22));
    let det3 = r00*(r11*r22-r12*r21) - r01*(r10*r22-r12*r20) + r02*(r10*r21-r11*r20);
    out.push_str(&format!("  det(R₃) = {:.6}\n", det3));
    out
}

fn cmd_gl2(a: f64, b: f64, c: f64, d: f64) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ GL(2,ℝ) — General Linear Group\n");
    let m = fmat2(a, b, c, d);
    out.push_str(&fmt_fmat2("M", &m));
    let det = fmat_det(&m);
    out.push_str(&format!("  det(M) = {:.6}\n", det));
    if det.abs() < 1e-10 {
        out.push_str("  det = 0: this matrix is NOT in GL(2,ℝ).\n");
        return out;
    }
    out.push_str("  det ≠ 0: M ∈ GL(2,ℝ).\n");
    out.push_str("  GL(2,ℝ) = invertible 2×2 real matrices. Lie algebra gl(2) = M₂(ℝ).\n");
    let tr = fmat_trace(&m);
    let disc = tr*tr - 4.0*det;
    out.push_str(&format!("  Characteristic polynomial: λ² - ({:.4})λ + ({:.4})\n", tr, det));
    if disc >= 0.0 {
        let l1 = (tr + disc.sqrt()) / 2.0;
        let l2 = (tr - disc.sqrt()) / 2.0;
        out.push_str(&format!("  eigenvalues (real) = {:.4}, {:.4}\n", l1, l2));
    } else {
        let re = tr / 2.0;
        let im = (-disc).sqrt() / 2.0;
        out.push_str(&format!("  eigenvalues (complex) = {:.4} ± {:.4}i\n", re, im));
    }
    let id = fmat_identity();
    let diff = fmat_add(&m, &fmat_scale(&id, -1.0));
    let norm: f64 = diff.iter().flat_map(|r| r.iter()).map(|x| x*x).sum::<f64>().sqrt();
    if norm < 0.9 {
        let log_m = mat_log_near_id(&m);
        out.push_str(&fmt_fmat2("log(M) [Lie algebra element]", &log_m));
        let exp_log = mat_exp(&log_m);
        out.push_str(&fmt_fmat2("exp(log(M)) [should recover M]", &exp_log));
        out.push_str("  For M near I: exp: gl(2)→GL(2,ℝ) is a local diffeomorphism.\n");
    } else {
        out.push_str("  M is not near I — log series not reliable.\n");
    }
    out
}

fn cmd_exp_mat(a: f64, b: f64, c: f64, d: f64) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Matrix Exponential exp(X) via Power Series\n");
    let x = fmat2(a, b, c, d);
    out.push_str(&fmt_fmat2("X (Lie algebra element)", &x));
    let ex = mat_exp(&x);
    out.push_str(&fmt_fmat2("exp(X)", &ex));
    let det = fmat_det(&ex);
    let tr = fmat_trace(&x);
    out.push_str(&format!("  det(exp(X)) = {:.6}\n", det));
    out.push_str(&format!("  exp(tr(X)) = {:.6}\n", tr.exp()));
    out.push_str("  Jacobi's formula: det(exp(X)) = exp(tr(X)).\n");
    out.push_str("  exp maps gl(2) → GL(2,ℝ) and so(2) → SO(2), sl(2) → SL(2,ℝ), etc.\n");
    out
}

fn cmd_one_param(a: f64, b: f64, c: f64, d: f64, t: f64) -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ One-Parameter Subgroup exp(tX)\n");
    let x = fmat2(a, b, c, d);
    out.push_str(&fmt_fmat2("X (tangent vector at identity)", &x));
    out.push_str(&format!("  t = {:.4}\n", t));
    let tx = fmat_scale(&x, t);
    let etx = mat_exp(&tx);
    out.push_str(&fmt_fmat2("exp(tX)", &etx));
    out.push_str("  γ(t) = exp(tX) is a smooth homomorphism ℝ → G with γ'(0) = X.\n");
    out.push_str("  Every one-parameter subgroup arises this way — the exponential map.\n");
    let s = t / 2.0;
    let esx = mat_exp(&fmat_scale(&x, s));
    let esx2 = fmat_mul(&esx, &esx);
    let diff_max = (0..2).flat_map(|i| (0..2).map(move |j| (etx[i][j] - esx2[i][j]).abs())).fold(0f64, f64::max);
    out.push_str(&format!("  max |exp(tX) - exp(t/2·X)²| = {:.2e}\n", diff_max));
    out.push_str("  exp((s+t)X) = exp(sX)·exp(tX): the subgroup law holds.\n");
    out
}

fn cmd_cover() -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ SU(2) → SO(3): The Double Cover\n");
    out.push_str("  The covering map π: SU(2) → SO(3) is a degree-2 Lie group homomorphism.\n");
    out.push_str("  ker(π) = {I, -I} ≅ ℤ/2ℤ, so π is 2-to-1.\n\n");
    out.push_str("  Construction via conjugation action:\n");
    out.push_str("  Identify ℝ³ with traceless Hermitian matrices: su(2)* ≅ ℝ³ via\n");
    out.push_str("    (x,y,z) ↦ [ z,    x-iy ]\n");
    out.push_str("               [ x+iy, -z   ]\n\n");
    out.push_str("  For U ∈ SU(2), define ρ(U): H ↦ UHU†.\n");
    out.push_str("  ρ(U) ∈ SO(3), and ρ(U) = ρ(-U), giving the 2-to-1 map.\n\n");
    out.push_str("  Example: U = exp(iθ/2 · σ₃):\n");
    for theta_deg in [0, 45, 90, 180, 360] {
        out.push_str(&format!("  θ={:3}°: U=exp(iθ/2·σ₃) → rotation by {:3}° in SO(3)\n",
            theta_deg, theta_deg));
    }
    out.push_str("\n  At θ=360°, U=-I (not +I!), but ρ(-I)=ρ(I)=Id_{SO(3)}.\n");
    out.push_str("  Spinors: 360° rotation sends spinor to its negative. 720° returns it.\n");
    out.push_str("\n  π₁(SO(3)) = ℤ/2ℤ  (non-trivial — the 'belt trick')\n");
    out.push_str("  π₁(SU(2)) = 0     (SU(2) ≅ S³ is simply connected — the universal cover)\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_str(&mut s, "group", "SO2");
    state_set_float(&mut s, "theta", 45.0);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "help" | "h" => show_help(),
        "so2" => {
            if let Some(th) = args.get(0).and_then(|s| s.parse::<f64>().ok()) {
                state_set_float(state, "theta", th);
                state_set_str(state, "group", "SO2");
                cmd_so2(th)
            } else {
                let th = state_get_float(state, "theta").unwrap_or(45.0);
                cmd_so2(th)
            }
        }
        "su2" => {
            let ar = args.get(0).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.6);
            let ai = args.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.8);
            let br = args.get(2).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            let bi = args.get(3).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            state_set_str(state, "group", "SU2");
            state_set_float(state, "su2_ar", ar);
            state_set_float(state, "su2_ai", ai);
            state_set_float(state, "su2_br", br);
            state_set_float(state, "su2_bi", bi);
            cmd_su2(ar, ai, br, bi)
        }
        "gl2" => {
            let a = args.get(0).and_then(|s| s.parse::<f64>().ok()).unwrap_or(1.1);
            let b = args.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.1);
            let c = args.get(2).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.1);
            let d = args.get(3).and_then(|s| s.parse::<f64>().ok()).unwrap_or(1.1);
            state_set_str(state, "group", "GL2");
            state_set_float(state, "gl2_a", a);
            state_set_float(state, "gl2_b", b);
            state_set_float(state, "gl2_c", c);
            state_set_float(state, "gl2_d", d);
            cmd_gl2(a, b, c, d)
        }
        "exp_mat" => {
            let a = args.get(0).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            let b = args.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(-1.0);
            let c = args.get(2).and_then(|s| s.parse::<f64>().ok()).unwrap_or(1.0);
            let d = args.get(3).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            state_set_float(state, "mat_a", a);
            state_set_float(state, "mat_b", b);
            state_set_float(state, "mat_c", c);
            state_set_float(state, "mat_d", d);
            cmd_exp_mat(a, b, c, d)
        }
        "one_param" => {
            let a = args.get(0).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            let b = args.get(1).and_then(|s| s.parse::<f64>().ok()).unwrap_or(-1.0);
            let c = args.get(2).and_then(|s| s.parse::<f64>().ok()).unwrap_or(1.0);
            let d = args.get(3).and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
            let t = args.get(4).and_then(|s| s.parse::<f64>().ok()).unwrap_or(1.0);
            state_set_float(state, "mat_a", a);
            state_set_float(state, "mat_b", b);
            state_set_float(state, "mat_c", c);
            state_set_float(state, "mat_d", d);
            state_set_float(state, "one_param_t", t);
            cmd_one_param(a, b, c, d, t)
        }
        "cover" => cmd_cover(),
        "demo" => {
            let mut out = String::new();
            out.push_str(&cmd_so2(45.0));
            out.push_str(&cmd_su2(0.6, 0.8, 0.0, 0.0));
            out.push_str(&cmd_exp_mat(0.0, -1.0, 1.0, 0.0));
            out.push_str(&cmd_cover());
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
    // Lie group structure: manifold + group diagram
    c.text_bold(350.0, 40.0, "Lie Group Structure", 16.0, "#212121", "middle");
    // Circle representing SO(2) = S¹
    c.circle(200.0, 250.0, 120.0, "none", "#4488ff", 2.0);
    c.text(200.0, 250.0, "SO(2) ≅ S¹", 13.0, "#4488ff", "middle");
    // S³ sphere representing SU(2)
    c.circle(480.0, 250.0, 100.0, "none", "#ff8844", 2.0);
    c.text(480.0, 250.0, "SU(2) ≅ S³", 13.0, "#ff8844", "middle");
    // Arrow: double cover
    c.arrow(310.0, 250.0, 370.0, 250.0, "#666666", 1.5);
    c.text(340.0, 240.0, "2:1", 11.0, "#666666", "middle");
    // Label axes
    c.text(200.0, 390.0, "rotation by θ", 11.0, "#555555", "middle");
    c.text(480.0, 370.0, "unit quaternions", 11.0, "#555555", "middle");
    // exp map
    c.text_bold(340.0, 450.0, "exp: g → G", 14.0, "#212121", "middle");
    c.text(340.0, 475.0, "so(2) → SO(2),  su(2) → SU(2)", 11.0, "#555555", "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("GL2", &[("label", "GL(2,ℝ)"), ("shape", "ellipse")]);
    g.node("SL2", &[("label", "SL(2,ℝ)"), ("shape", "ellipse")]);
    g.node("SO2", &[("label", "SO(2)"), ("shape", "ellipse")]);
    g.node("SU2", &[("label", "SU(2)"), ("shape", "ellipse")]);
    g.node("SO3", &[("label", "SO(3)"), ("shape", "ellipse")]);
    g.edge("GL2", "SL2", &[("label", "det=1")]);
    g.edge("SL2", "SO2", &[("label", "orthogonal")]);
    g.edge("SU2", "SO3", &[("label", "2:1 cover")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("G", 0.0, 0.0, "GL(2,ℝ)", "rectangle,draw");
    t.node("S", 3.0, 0.0, "SL(2,ℝ)", "rectangle,draw");
    t.node("O", 6.0, 0.0, "SO(2)", "circle,draw");
    t.arrow("G", "S", "det=1", "");
    t.arrow("S", "O", "orth.", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, _args: &[&str], _state: &StateMap) {
    a.text_at(2, 1, "Lie Group Hierarchy");
    a.text_at(2, 3, "GL(2,R)");
    a.text_at(4, 5, "|  det=1");
    a.text_at(2, 7, "SL(2,R)");
    a.text_at(4, 9, "|  orthogonal");
    a.text_at(2, 11, "SO(2) = S^1");
    a.text_at(25, 3, "SU(2) = S^3");
    a.text_at(25, 5, "  |  2:1");
    a.text_at(25, 7, "SO(3) = RP^3");
    a.text_at(2, 14, "exp: Lie algebra ---> Lie group");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch46"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 46", "Lie Groups", "Smooth groups: symmetry meets calculus");
            repl("ch46> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
