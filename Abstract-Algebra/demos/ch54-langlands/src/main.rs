use common::*;

fn show_help() -> String {
    help_string(&[
        ("local_langlands <p> <n>",      "Local Langlands for GL_n(Q_p): correspondence"),
        ("weil_group <p>",               "Weil group W_{Q_p}: Frobenius, inertia, structure"),
        ("l_function <a> <b>",           "L-function of elliptic curve y²=x³+ax+b"),
        ("modularity <a> <b>",           "Modularity theorem for elliptic curve y²=x³+ax+b"),
        ("galois_rep <p> <n>",           "n-dim Galois representation mod p: Frobenius data"),
        ("geometric_langlands",          "Geometric Langlands: D-modules and local systems"),
        ("demo",                         "Run a showcase of all commands"),
        ("help",                         "Show this help"),
        ("quit",                         "Exit"),
    ])
}

// Count points on y² = x³+ax+b over 𝔽_p
fn count_ec_points(a: i64, b: i64, p: u64) -> u64 {
    let p = p as i64;
    let mut count = 1u64; // point at infinity
    for x in 0..p {
        let rhs = ((x*x*x + a*x + b).rem_euclid(p)) as u64;
        if rhs == 0 {
            count += 1;
        } else {
            let exp = ((p-1)/2) as u64;
            let leg = mod_pow(rhs as i64, exp, p) as u64;
            if leg == 1 { count += 2; }
        }
    }
    count
}

fn disc(a: i64, b: i64) -> i64 { -16 * (4*a*a*a + 27*b*b) }

fn ap(a: i64, b: i64, p: u64) -> i64 {
    let np = count_ec_points(a, b, p) as i64;
    p as i64 + 1 - np
}

fn cmd_local_langlands(p: u64, n: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) { return format!("  {} is not prime\n", p); }
    if n < 1 || n > 4 { return "  n must be 1–4\n".to_string(); }
    out.push_str(&format!("\n  ▸ Local Langlands Correspondence for GL_{}(ℚ_p), p={}\n\n", n, p));
    out.push_str("  The local Langlands correspondence (LLC) is a bijection:\n");
    out.push_str("    { irred. smooth reps of GL_n(ℚ_p) } ↔ { n-dim F-semisimple Weil-Deligne reps of W_{ℚ_p} }\n\n");
    out.push_str("  Key Players:\n");
    out.push_str(&format!("    G = GL_{}(ℚ_p): locally compact group, p-adic field ℚ_p\n", n));
    out.push_str("    W_{ℚ_p} = Weil group of ℚ_p (dense in Gal(ℚ̄_p/ℚ_p))\n");
    out.push_str("    WD reps = (ρ: W→GL_n(ℂ), N: nilpotent) with ρ(Frob)·N·ρ(Frob)⁻¹ = p·N\n\n");
    match n {
        1 => {
            out.push_str("  n=1: Local Class Field Theory\n");
            out.push_str("    G = GL_1(ℚ_p) = ℚ_p*\n");
            out.push_str("    Irreducible smooth reps of ℚ_p* = smooth characters χ: ℚ_p* → ℂ*\n");
            out.push_str("    WD reps of dim 1 = characters of W_{ℚ_p}\n");
            out.push_str("    LLC for GL_1: via local Artin map (local class field theory)\n");
            out.push_str("    Art_p: ℚ_p* → W_{ℚ_p}^{ab} (Artin reciprocity map)\n");
            out.push_str("    Character χ of ℚ_p* ↔ χ∘Art_p⁻¹: character of W_{ℚ_p}\n\n");
            out.push_str("    Example characters of ℚ_p*:\n");
            out.push_str("      Unramified: χ(p) = s for s ∈ ℂ*, χ|_{ℤ_p*} = 1\n");
            out.push_str("      Ramified: χ = χ_triv on p^k ℤ_p* for k ≥ 1\n");
            out.push_str("    LLC for n=1 = local class field theory. Proved by local Artin maps.\n");
        }
        2 => {
            out.push_str("  n=2: Local Langlands for GL_2 (Harris-Taylor / Henniart, 2000)\n");
            out.push_str("    Irreducible smooth reps of GL_2(ℚ_p) fall into:\n\n");
            out.push_str("    (1) Supercuspidal: don't appear in parabolic induction.\n");
            out.push_str("        Corresponds to WD reps with irreducible ρ.\n\n");
            out.push_str("    (2) Principal series: Ind_B^G(χ₁⊗χ₂) for χ₁≠χ₂.\n");
            out.push_str("        Corresponds to ρ = χ₁⊕χ₂ (reducible, N=0).\n\n");
            out.push_str("    (3) Special representations: Sp(2) = unique quotient of Ind(|·|^{1/2}⊗|·|^{-1/2}).\n");
            out.push_str("        Corresponds to ρ = unramified character, N ≠ 0 (Steinberg WD rep).\n\n");
            out.push_str("    For p-adic GL_2, the LLC was proved by Kutzko (supercuspidals) and\n");
            out.push_str("    classical methods. The general GL_n case: Harris-Taylor (2001), Henniart (2000).\n");
        }
        _ => {
            out.push_str(&format!("  For GL_{}(ℚ_p):\n", n));
            out.push_str(&format!("    Irreducible smooth reps ↔ {}-dim Weil-Deligne reps of W_{{ℚ_p}}\n\n", n));
            out.push_str("    Proof: Harris-Taylor (2001) via p-adic uniformisation of Shimura varieties.\n");
            out.push_str("    Alternative: Henniart (2000) via numerical LLC + uniqueness.\n");
        }
    }
    out.push_str("\n  Local Factors:\n");
    out.push_str("    L(s,π) = L(s,σ)   (local L-factor)\n");
    out.push_str("    ε(s,π,ψ) = ε(s,σ,ψ)  (epsilon factor)\n");
    out.push_str("    The LLC is characterised by these equalities for all n (uniqueness theorem).\n");
    out
}

fn cmd_weil_group(p: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) { return format!("  {} is not prime\n", p); }
    out.push_str(&format!("\n  ▸ Weil Group W_{{ℚ_p}}, prime p={}\n\n", p));
    out.push_str("  The Weil group W_K of a local field K is a dense subgroup of Gal(K̄/K).\n\n");
    out.push_str("  Structure:\n");
    out.push_str("    Absolute Galois group: G_p = Gal(ℚ̄_p / ℚ_p)\n");
    out.push_str("    Inertia subgroup:      I_p = Gal(ℚ̄_p / ℚ_p^{unr})\n");
    out.push_str("    Wild inertia:          P_p = unique pro-p Sylow of I_p\n\n");
    out.push_str("    Weil group: W_p = ρ⁻¹(ℤ)  where ρ: G_p → Gal(ℚ_p^{unr}/ℚ_p) ≅ Ẑ\n");
    out.push_str("    W_p = elements of G_p that act on ℚ_p^{unr} via an integer power of Frobenius.\n\n");
    out.push_str("  Frobenius Element:\n");
    out.push_str("    The (arithmetic) Frobenius Frob_p ∈ W_p is defined by:\n");
    out.push_str("    Frob_p(x) = x^{1/p} for x ∈ ℚ_p^{unr}  (geometric convention)\n");
    out.push_str("    or Frob_p(x) = x^p  (arithmetic convention)\n");
    out.push_str("    It generates W_p / I_p ≅ ℤ.\n\n");
    out.push_str("  Exact sequence:\n");
    out.push_str("    1 → I_p → W_p → ℤ → 0\n");
    out.push_str("    where the map W_p → ℤ sends Frob_p ↦ 1.\n\n");
    out.push_str("  Weil-Deligne representations:\n");
    out.push_str("    A Weil-Deligne rep of W_p is a pair (ρ, N) where:\n");
    out.push_str("    ρ: W_p → GL_n(ℂ) is a smooth (continuous) representation\n");
    out.push_str("    N ∈ gl_n(ℂ) nilpotent with ρ(Frob_p)·N·ρ(Frob_p)⁻¹ = p·N\n\n");
    out.push_str("    N = 0: pure Weil representations (no monodromy)\n");
    out.push_str("    N ≠ 0: 'ramified at ∞' in the l-adic sense\n\n");
    out.push_str("  Unramified representations:\n");
    out.push_str("    An unramified character: χ: W_p → ℂ* trivial on I_p.\n");
    out.push_str("    Determined by χ(Frob_p) = α ∈ ℂ*.\n");
    out.push_str("    L-factor: L(s, χ) = (1 - α·p^{-s})^{-1}\n\n");
    out.push_str(&format!("  Example: Tate twist ℚ_ℓ(1) for p={}\n", p));
    out.push_str("    The cyclotomic character: χ_cyc: G_p → ℤ_ℓ* ↪ ℂ*\n");
    out.push_str(&format!("    χ_cyc(Frob_p) = {}  (Frobenius acts as multiplication by p on roots of unity)\n", p));
    out.push_str(&format!("    L(s, ℚ_ℓ(1)) = (1 - {}·p^{{-s}})^{{-1}} = (1 - p^{{1-s}})^{{-1}}\n", p));
    out.push_str("  The Weil group and its representations are the local side of the Langlands duality.\n");
    out
}

fn cmd_l_function(a: i64, b: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  ▸ L-function of Elliptic Curve E: y² = x³ + {}x + {}\n", a, b));
    let d = disc(a, b);
    out.push_str(&format!("  Discriminant Δ = -16(4a³+27b²) = {}\n", d));
    if d == 0 { return "  Δ = 0: this is a singular curve (not elliptic). Choose different a,b.\n".to_string(); }
    out.push_str("  L(E,s) = ∏_p L_p(E,p^{-s})^{-1}\n\n");
    out.push_str(&format!("  {:6}  {:10}  {:8}  {:12}  {:18}\n", "p", "#E(𝔽_p)", "a_p", "L_p(T)", "notes"));
    out.push_str(&format!("  {}\n", "-".repeat(65)));
    let bad_primes: Vec<u64> = (2..=50).filter(|&p| is_prime(p) && d % p as i64 == 0).collect();
    for p in [2u64,3,5,7,11,13,17,19,23,29,31,37,41,43,47] {
        if bad_primes.contains(&p) {
            out.push_str(&format!("  {:6}  {:>10}  {:>8}  {:12}  BAD REDUCTION\n", p, "—", "—", "?"));
            continue;
        }
        let np = count_ec_points(a, b, p);
        let ap_val = ap(a, b, p);
        let lp = format!("1{:+}T+{}T²", -ap_val, p);
        let note = if ap_val == 0 { "supersingular" } else { "" };
        out.push_str(&format!("  {:6}  {:10}  {:8}  {:12}  {}\n", p, np, ap_val, lp, note));
    }
    out.push_str("\n  The completed L-function:\n");
    out.push_str("    L*(E,s) = N^{s/2} (2π)^{-s} Γ(s) L(E,s)\n");
    out.push_str("    Satisfies functional equation: L*(E,s) = ±L*(E,2-s)\n\n");
    out.push_str("  Birch and Swinnerton-Dyer Conjecture:\n");
    out.push_str("    ord_{s=1} L(E,s) = rank of E(ℚ)\n");
    out.push_str("    Leading term at s=1 encodes the Tate-Shafarevich group Ш, regulators, etc.\n\n");
    let mut prod = 1.0_f64;
    let mut count = 0;
    for p in (2u64..=200).filter(|&p| is_prime(p) && !bad_primes.contains(&p)).take(20) {
        let np = count_ec_points(a, b, p) as f64;
        prod *= np / p as f64;
        count += 1;
    }
    out.push_str(&format!("  ∏_{{p≤47, good}} #E(𝔽_p)/p ≈ {:.4}  (crude BSD indicator, {} primes)\n\n", prod, count));
    out.push_str("  L(E,s) converges for Re(s)>3/2. Modularity theorem (Wiles et al) proves\n");
    out.push_str("  analytic continuation to ℂ and functional equation: E is modular.\n");
    out
}

fn cmd_modularity(a: i64, b: i64) -> String {
    let mut out = String::new();
    out.push_str(&format!("\n  ▸ Modularity Theorem: E: y² = x³ + {}x + {}\n", a, b));
    let d = disc(a, b);
    out.push_str(&format!("  Discriminant: Δ = {}\n", d));
    if d == 0 { return "  Singular curve. Choose different a,b.\n".to_string(); }
    out.push_str("\n  Modularity Theorem (Wiles-Taylor-Wiles 1995, Breuil-Conrad-Diamond-Taylor 2001):\n");
    out.push_str("  Every elliptic curve E/ℚ is modular: there exists a weight-2 newform f of\n");
    out.push_str("  level N (the conductor of E) such that L(E,s) = L(f,s).\n\n");
    out.push_str("  What this means concretely:\n");
    out.push_str("    f is a holomorphic function f: ℍ → ℂ (ℍ = upper half plane)\n");
    out.push_str("    f is a modular form of weight 2 for Γ₀(N):\n");
    out.push_str("      f(γτ) = (cτ+d)² f(τ)  for γ=[[a,b],[c,d]] ∈ Γ₀(N)\n");
    out.push_str("    f has Fourier expansion: f(τ) = Σ_{n≥1} aₙ q^n,  q = e^{2πiτ}\n");
    out.push_str("    Key: a_p = p+1-#E(𝔽_p) for good primes p.\n\n");
    out.push_str(&format!("  Fourier coefficients a_p matching for E: y²=x³+{}x+{}\n", a, b));
    let bad_primes: Vec<u64> = (2..=100).filter(|&p| is_prime(p) && d % p as i64 == 0).collect();
    out.push_str(&format!("  Bad primes (dividing Δ={}): {:?}\n\n", d, bad_primes));
    out.push_str("  Good primes and their a_p = p+1-#E(𝔽_p):\n");
    for p in [5u64,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61] {
        if bad_primes.contains(&p) { continue; }
        let ap_val = ap(a, b, p);
        out.push_str(&format!("    a_{{{}}} = {}\n", p, ap_val));
    }
    out.push_str("\n  Hecke operators:\n");
    out.push_str("    The modularity means E is an eigenform for all Hecke operators T_p:\n");
    out.push_str("    T_p f = a_p f  for all good primes p.\n\n");
    out.push_str("  Conductor:\n");
    out.push_str("    The conductor N of E encodes the ramification data.\n");
    if !bad_primes.is_empty() {
        out.push_str(&format!("    For our curve, bad primes: {:?}\n", bad_primes));
    }
    out.push_str("\n  Modularity was the key ingredient in Wiles' proof of Fermat's Last Theorem.\n");
    out.push_str("  The 3-5 trick: if a^p+b^p=c^p, the Frey curve E_{a,b,c} is modular but\n");
    out.push_str("  has conductor too small → contradiction with epsilon conjecture (Ribet's theorem).\n");
    out
}

fn cmd_galois_rep(p: u64, n: u64) -> String {
    let mut out = String::new();
    if !is_prime(p) { return format!("  {} is not prime\n", p); }
    if n < 1 || n > 4 { return "  n must be 1–4\n".to_string(); }
    out.push_str(&format!("\n  ▸ {}-dimensional Galois Representation mod p={}\n\n", n, p));
    out.push_str("  A Galois representation is a continuous group homomorphism:\n");
    out.push_str("    ρ: Gal(ℚ̄/ℚ) → GL_n(ℤ_l)  (or GL_n(𝔽_l))\n\n");
    out.push_str("  Unramified representations:\n");
    out.push_str("    For all but finitely many primes q, ρ is unramified at q:\n");
    out.push_str("    ρ(Frob_q) is well-defined up to conjugation.\n");
    out.push_str("    The characteristic polynomial det(1 - ρ(Frob_q)T) is the local L-factor.\n\n");
    out.push_str(&format!("  For n={}, dim representations, key data:\n", n));
    match n {
        1 => {
            out.push_str("  1-dim reps ρ: Gal(ℚ̄/ℚ) → ℤ_l*  = Dirichlet characters (by CFT).\n");
            out.push_str("  Cyclotomic character: χ_l: Gal → ℤ_l*,  χ_l(Frob_q) = q  (for q≠l).\n\n");
            out.push_str(&format!("  Example: mod {} characters\n", p));
            for q in [2u64,3,5,7,11,13,17,19].iter().filter(|&&q| q != p && is_prime(q)) {
                let frob = q % p;
                out.push_str(&format!("    χ_cyc(Frob_{}) = {} (mod {})\n", q, frob, p));
            }
        }
        2 => {
            out.push_str("  2-dim reps come from:\n");
            out.push_str("  (a) Elliptic curves E/ℚ: ρ_{E,l}: Gal → GL_2(ℤ_l) from Tate module T_l(E).\n");
            out.push_str("      Frob_q has char poly T² - a_q·T + q  (for q good prime, q≠l).\n\n");
            out.push_str("  (b) Weight-2 modular forms f = Σ aₙqⁿ:\n");
            out.push_str("      ρ_f: Gal → GL_2(𝒪_λ) (ring of integers of a number field).\n");
            out.push_str("      Tr(ρ_f(Frob_q)) = a_q for q∤N.\n\n");
            out.push_str(&format!("  Example: Frobenius data for a hypothetical 2-dim rep mod {}:\n", p));
            for q in [2u64,3,5,7,11,13,17,19,23].iter().filter(|&&q| is_prime(q) && q != p) {
                let trace = (q % p) as i64;
                let det_val = (q % p) as i64;
                out.push_str(&format!("    Frob_{}: trace = {} (mod {}), det = {} (mod {})\n", q, trace, p, det_val, p));
            }
        }
        _ => {
            out.push_str(&format!("  {}-dim reps come from:\n", n));
            out.push_str(&format!("  (a) Automorphic forms on GL_{}(𝔸_ℚ) via Langlands correspondence.\n", n));
            out.push_str("  (b) Geometric origins: H^k(X_{ℚ̄}, ℚ_l) for algebraic varieties X.\n\n");
            out.push_str(&format!("  Example Frobenius data (schematic, mod {}):\n", p));
            for q in [2u64,3,5,7,11,13].iter().filter(|&&q| is_prime(q) && q != p) {
                let eigenvals: Vec<String> = (0..n).map(|k|
                    format!("{}", (q + k*7) % p)).collect();
                out.push_str(&format!("    Frob_{}: eigenvalues = ({}) (mod {})\n", q, eigenvals.join(","), p));
            }
        }
    }
    out.push_str("\n  Compatibility system:\n");
    out.push_str("    A compatible system of l-adic representations {ρ_l}_l is:\n");
    out.push_str("    for all but finitely many primes q: char poly of ρ_l(Frob_q) ∈ ℤ[T], same for all l.\n\n");
    out.push_str("  Global Langlands: n-dim Galois reps ↔ cuspidal automorphic reps of GL_n(𝔸_ℚ).\n");
    out.push_str("  Proved for GL_1 (class field theory) and GL_2 (modularity, Langlands-Tunnell).\n");
    out.push_str("  Open for GL_n, n≥3 in general; many cases via trace formula (Arthur, etc.).\n");
    out
}

fn cmd_geometric_langlands() -> String {
    let mut out = String::new();
    out.push_str("\n  ▸ Geometric Langlands Program\n\n");
    out.push_str("  The geometric Langlands correspondence (Beilinson-Drinfeld) is an equivalence\n");
    out.push_str("  of derived categories:\n\n");
    out.push_str("  For a smooth projective curve X over ℂ and reductive group G:\n\n");
    out.push_str("    D-modules on Bun_G(X)  ↔  Local systems on X with structure group Ĝ\n");
    out.push_str("      (flat connections)           (representations of π₁(X) in Ĝ(ℂ))\n\n");
    out.push_str("  More precisely, as derived categories:\n");
    out.push_str("    D^b(D-mod(Bun_G))  ≃  D^b(QCoh(LocSys_{Ĝ}(X)))\n\n");
    out.push_str("  Key objects:\n");
    out.push_str("    Bun_G(X) = moduli stack of principal G-bundles on X\n");
    out.push_str("    LocSys_{Ĝ}(X) = moduli stack of Ĝ-local systems on X\n");
    out.push_str("    Ĝ = Langlands dual group (root system transposed)\n\n");
    out.push_str("  Examples of Ĝ:\n");
    out.push_str("    G = GL_n       ↔  Ĝ = GL_n   (self-dual)\n");
    out.push_str("    G = SL_n       ↔  Ĝ = PGL_n\n");
    out.push_str("    G = Sp_{2n}    ↔  Ĝ = SO_{2n+1}\n");
    out.push_str("    G = SO_{2n+1}  ↔  Ĝ = Sp_{2n}\n");
    out.push_str("    G = G_2        ↔  Ĝ = G_2    (self-dual exceptional)\n\n");
    out.push_str("  Hecke eigensheaves:\n");
    out.push_str("    A Hecke eigensheaf for a local system E (Ĝ-local system) is:\n");
    out.push_str("    a D-module F on Bun_G such that Hecke modifications of F give F⊗V_E.\n");
    out.push_str("    Beilinson-Drinfeld: For any E irreducible, ∃ unique Hecke eigensheaf Aut_E.\n\n");
    out.push_str("  Connection to classical Langlands:\n");
    out.push_str("    Classical:  Galois reps of Gal(𝔽_q(X)) ↔ automorphic forms on GL_n(𝔸)\n");
    out.push_str("    Geometric:  Ĝ-local systems on X ↔ D-modules on Bun_G(X)\n");
    out.push_str("    Arithmetic → Geometry: replace Galois group by π₁(X), automorphic forms by D-modules\n\n");
    out.push_str("  Arithmetic version (function fields):\n");
    out.push_str("    Over a finite field 𝔽_q, the geometric Langlands ↔ Langlands over 𝔽_q(X).\n");
    out.push_str("    V. Lafforgue (2018): proved full Langlands for GL_n over function fields.\n\n");
    out.push_str("  Recent Developments: Geometrisation of Langlands (Fargues-Scholze 2021)\n");
    out.push_str("  Fargues-Scholze proved the geometric Langlands conjecture over the Fargues-Fontaine\n");
    out.push_str("  curve — a 'curve' over p-adic fields built from perfectoid geometry.\n");
    out.push_str("    Key idea: replace X over ℂ with the FF curve Y_{F,E} (p-adic analogue).\n");
    out.push_str("    Local Langlands ↔ geometric Langlands over Y_{F,E}.\n\n");
    out.push_str("  The Langlands program connects number theory, geometry, and representation theory.\n");
    out.push_str("  Applications: proof of FLT, Sato-Tate conjecture, functoriality,\n");
    out.push_str("  Shimura varieties, automorphic L-functions, and the Riemann hypothesis for curves.\n");
    out
}

fn default_state() -> StateMap {
    let mut s = state_new();
    state_set_int(&mut s, "ec_a", -1);
    state_set_int(&mut s, "ec_b", 0);
    state_set_int(&mut s, "prime", 5);
    s
}

fn run_cmd(cmd: &str, args: &[&str], state: &mut StateMap) -> String {
    match cmd {
        "help" | "h" => show_help(),
        "local_langlands" => {
            let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64);
            let n = args.get(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(2);
            state_set_int(state, "prime", p as i64);
            state_set_int(state, "ll_n", n as i64);
            cmd_local_langlands(p, n)
        }
        "weil_group" => {
            let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64);
            state_set_int(state, "prime", p as i64);
            cmd_weil_group(p)
        }
        "l_function" => {
            let a = args.get(0).and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(state_get_int(state, "ec_a").unwrap_or(-1));
            let b = args.get(1).and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(state_get_int(state, "ec_b").unwrap_or(0));
            state_set_int(state, "ec_a", a);
            state_set_int(state, "ec_b", b);
            cmd_l_function(a, b)
        }
        "modularity" => {
            let a = args.get(0).and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(state_get_int(state, "ec_a").unwrap_or(-1));
            let b = args.get(1).and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(state_get_int(state, "ec_b").unwrap_or(0));
            state_set_int(state, "ec_a", a);
            state_set_int(state, "ec_b", b);
            cmd_modularity(a, b)
        }
        "galois_rep" => {
            let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64);
            let n = args.get(1).and_then(|s| s.parse::<u64>().ok()).unwrap_or(2);
            state_set_int(state, "prime", p as i64);
            state_set_int(state, "galois_n", n as i64);
            cmd_galois_rep(p, n)
        }
        "geometric_langlands" => cmd_geometric_langlands(),
        "demo" => {
            let mut out = cmd_l_function(-1, 0);
            out.push_str(&cmd_weil_group(5));
            out.push_str(&cmd_geometric_langlands());
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
    // Langlands correspondence: L-functions <-> representations
    c.text_bold(350.0, 30.0, "The Langlands Correspondence", 16.0, "#212121", "middle");
    // Left side: Galois / Arithmetic
    c.rect(30.0, 70.0, 280.0, 360.0, "#e8f4ff", "#3366cc", 2.0);
    c.text_bold(170.0, 95.0, "Arithmetic Side", 14.0, "#3366cc", "middle");
    c.text(170.0, 130.0, "Galois reps", 12.0, "#333333", "middle");
    c.text(170.0, 155.0, "ρ: Gal(Q̄/Q) → GL_n", 11.0, "#333333", "middle");
    c.text(170.0, 195.0, "L-functions L(s,ρ)", 12.0, "#333333", "middle");
    c.text(170.0, 220.0, "Frobenius data", 11.0, "#555555", "middle");
    c.text(170.0, 260.0, "Elliptic curves", 12.0, "#333333", "middle");
    c.text(170.0, 285.0, "y² = x³ + ax + b", 11.0, "#555555", "middle");
    c.text(170.0, 325.0, "Weil group W_{Q_p}", 11.0, "#555555", "middle");
    c.text(170.0, 350.0, "Local Langlands", 11.0, "#333333", "middle");
    // Right side: Automorphic / Geometric
    c.rect(390.0, 70.0, 280.0, 360.0, "#fff4e8", "#cc6633", 2.0);
    c.text_bold(530.0, 95.0, "Automorphic Side", 14.0, "#cc6633", "middle");
    c.text(530.0, 130.0, "Automorphic forms", 12.0, "#333333", "middle");
    c.text(530.0, 155.0, "f: GL_n(A) → C", 11.0, "#333333", "middle");
    c.text(530.0, 195.0, "L-functions L(s,π)", 12.0, "#333333", "middle");
    c.text(530.0, 220.0, "Hecke eigenvalues", 11.0, "#555555", "middle");
    c.text(530.0, 260.0, "Modular forms", 12.0, "#333333", "middle");
    c.text(530.0, 285.0, "weight 2, level N", 11.0, "#555555", "middle");
    c.text(530.0, 325.0, "D-modules on Bun_G", 11.0, "#555555", "middle");
    c.text(530.0, 350.0, "Geometric Langlands", 11.0, "#333333", "middle");
    // Arrow in middle
    c.arrow(310.0, 250.0, 390.0, 250.0, "#666666", 1.5);
    c.arrow(390.0, 270.0, 310.0, 270.0, "#666666", 1.5);
    c.text(350.0, 240.0, "↔", 14.0, "#333333", "middle");
    c.text_bold(350.0, 460.0, "Functoriality principle: L(s,ρ) = L(s,π(ρ))", 13.0, "#212121", "middle");
}

fn visualize_dot(g: &mut DotGraph, _cmd: &str, _args: &[&str], _state: &StateMap) {
    g.node("galois", &[("label", "Galois reps\\nρ: Gal→GL_n"), ("shape", "box")]);
    g.node("automorphic", &[("label", "Automorphic forms\\nGL_n(A_Q)"), ("shape", "box")]);
    g.node("lfunction", &[("label", "L-functions\\nL(s,ρ)=L(s,π)"), ("shape", "diamond")]);
    g.node("local", &[("label", "Local LLC\\nGL_n(Q_p)↔W_p"), ("shape", "ellipse")]);
    g.node("geometric", &[("label", "Geometric Langlands\\nD-mod↔LocSys"), ("shape", "ellipse")]);
    g.edge("galois", "lfunction", &[("label", "Galois L-factor")]);
    g.edge("automorphic", "lfunction", &[("label", "automorphic L-factor")]);
    g.edge("galois", "local", &[("label", "local component")]);
    g.edge("automorphic", "geometric", &[("label", "geometrize")]);
}

fn visualize_tex(t: &mut TikzDoc, _cmd: &str, _args: &[&str], _state: &StateMap) {
    t.node("gal", -3.0, 0.0, "Galois reps", "rectangle,draw");
    t.node("aut", 3.0, 0.0, "Automorphic forms", "rectangle,draw");
    t.node("lf", 0.0, -2.0, "L-functions", "ellipse,draw");
    t.arrow("gal", "aut", "Langlands", "");
    t.arrow("gal", "lf", "L(s,rho)", "");
    t.arrow("aut", "lf", "L(s,pi)", "");
}

fn visualize_ascii(a: &mut AsciiCanvas, _cmd: &str, args: &[&str], state: &StateMap) {
    let p = args.get(0).and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(state_get_int(state, "prime").unwrap_or(5) as u64);
    a.text_at(2, 1, "Langlands Correspondence");
    a.text_at(2, 3, "ARITHMETIC                    AUTOMORPHIC");
    a.text_at(2, 5, "Galois reps           <-->    Automorphic forms");
    a.text_at(2, 7, "rho: Gal(Q-bar/Q)             f on GL_n(A_Q)");
    a.text_at(2, 9, "  -> GL_n(Z_l)");
    a.text_at(2, 11, "L(s,rho) = L(s,pi)  (Functoriality)");
    a.text_at(2, 13, &format!("Local: W_Q_{} <-> GL_n(Q_{})", p, p));
    a.text_at(2, 15, "Geometric: D-mod(Bun_G) <-> LocSys_{G-hat}");
    a.text_at(2, 17, "Elliptic curves: L(E,s) via modularity (Wiles 1995)");
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
                OutputFormat::Dot   => { let mut g = DotGraph::digraph("ch54"); visualize_dot(&mut g, cmd, &args_ref, &state); g.build() }
                OutputFormat::Tex   => { let mut t = TikzDoc::standalone(); visualize_tex(&mut t, cmd, &args_ref, &state); t.build() }
                OutputFormat::Ascii => { let mut a = AsciiCanvas::new(80, 30); visualize_ascii(&mut a, cmd, &args_ref, &state); a.render() }
                _ => run_cmd(cmd, &args_ref, &mut state),
            };
            app.emit(&out);
            if let Some(ref f) = app.save_file { let _ = save_state(f, &state); }
        }
        AppMode::Interactive => {
            print_banner("Chapter 54", "The Langlands Program",
                         "Unifying number theory, geometry, and representation theory");
            repl("ch54> ", &mut |cmd, args| run_cmd_bool(cmd, args, &mut state));
        }
    }
}
