use std::io::{self, BufRead, Write};

const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn bold(s: &str) -> String { format!("{BOLD}{s}{RESET}") }
fn cyan(s: &str) -> String { format!("{CYAN}{s}{RESET}") }
fn green(s: &str) -> String { format!("{GREEN}{s}{RESET}") }
fn yellow(s: &str) -> String { format!("{YELLOW}{s}{RESET}") }
fn red(s: &str) -> String { format!("{RED}{s}{RESET}") }
fn dim(s: &str) -> String { format!("{DIM}{s}{RESET}") }

// ── Eilenberg-MacLane Spaces ────────────────────────────────────────────────────
//
// K(G, n) is the unique (up to homotopy) space with:
//   πₙ(K(G,n)) = G
//   πₖ(K(G,n)) = 0  for k ≠ n
//
// Represented cohomology: Hⁿ(X; G) ≃ [X, K(G,n)]
//
// Key examples:
//   K(ℤ, 1) = S¹ = BZ (classifying space of ℤ)
//   K(ℤ, 2) = CP∞ = BS¹ (infinite complex projective space)
//   K(ℤ/2ℤ, 1) = RP∞ = BZ/2Z
//   K(ℤ/nℤ, 1) = lens space L∞(n)

#[derive(Clone, Debug)]
struct EMSpace {
    group: String,
    n: usize,
    description: String,
    cohomology_ring: String,
}

impl EMSpace {
    fn new(group: &str, n: usize) -> Self {
        let (description, cohomology_ring) = em_known(group, n);
        EMSpace {
            group: group.into(),
            n,
            description: description.into(),
            cohomology_ring: cohomology_ring.into(),
        }
    }

    fn display(&self) {
        println!("  K({}, {})", cyan(&self.group), self.n);
        println!("  Description: {}", self.description);
        println!("  π{}(K({},{})) = {}", self.n, self.group, self.n, cyan(&self.group));
        println!("  πₖ(K({},{})) = 0  for k ≠ {}", self.group, self.n, self.n);
        println!("  H*(K({},{}); ℤ) = {}", self.group, self.n, self.cohomology_ring);
    }
}

fn em_known(group: &str, n: usize) -> (&'static str, &'static str) {
    match (group, n) {
        ("Z" | "ℤ", 1) => ("S¹ (circle, classifying space of ℤ)", "ℤ[x] with |x|=2 restricted: H*(S¹;ℤ)=ℤ in deg 0,1"),
        ("Z" | "ℤ", 2) => ("CP∞ (infinite complex projective space = BS¹)", "ℤ[x] polynomial ring, |x|=2"),
        ("Z" | "ℤ", 3) => ("K(ℤ,3) — no simple geometric description", "Complex: H*(K(ℤ,3);ℤ/2) = ℤ/2[Sq²x, ...]"),
        ("Z2" | "ℤ/2ℤ", 1) => ("RP∞ (infinite real projective space = BO(1))", "ℤ/2[x] polynomial ring, |x|=1"),
        ("Z2" | "ℤ/2ℤ", 2) => ("K(ℤ/2,2)", "ℤ/2[x,Sq¹x,...] via Steenrod operations"),
        ("Z3" | "ℤ/3ℤ", 1) => ("L∞(3) — infinite lens space", "ℤ/3[x]⊗E[y], |x|=2, |y|=1 (periodic)"),
        ("Zn", 1) => ("L∞(n) — infinite lens space BZ/nZ", "H*(BZ/nZ;ℤ)=ℤ[x]/(nx)⊗E[y] (roughly)"),
        _ => ("K(G,n) — abstract Eilenberg-MacLane space", "Computable via spectral sequences"),
    }
}

fn cohomology(space: &str, n: usize, coeff: &str) -> String {
    match (space, n, coeff) {
        ("S1" | "S¹", 0, _) => "ℤ".into(),
        ("S1" | "S¹", 1, _) => "ℤ".into(),
        ("S1" | "S¹", _, _) => "0".into(),
        ("CP1" | "S2" | "CP¹" | "S²", 0, _) => "ℤ".into(),
        ("CP1" | "S2" | "CP¹" | "S²", 2, _) => "ℤ".into(),
        ("CP1" | "S2" | "CP¹" | "S²", _, _) => "0".into(),
        ("RP2" | "RP²", 0, "Z") => "ℤ".into(),
        ("RP2" | "RP²", 1, "Z") => "0".into(),
        ("RP2" | "RP²", 2, "Z") => "ℤ/2ℤ".into(),
        ("RP2" | "RP²", _, "Z") => "0".into(),
        ("RP2" | "RP²", 0, "Z2") => "ℤ/2ℤ".into(),
        ("RP2" | "RP²", 1, "Z2") => "ℤ/2ℤ".into(),
        ("RP2" | "RP²", 2, "Z2") => "ℤ/2ℤ".into(),
        ("T2" | "T" | "torus", 0, _) => "ℤ".into(),
        ("T2" | "T" | "torus", 1, _) => "ℤ⊕ℤ".into(),
        ("T2" | "T" | "torus", 2, _) => "ℤ".into(),
        ("T2" | "T" | "torus", _, _) => "0".into(),
        _ => format!("H^{n}({space}; {coeff}) — use universal coefficient theorem"),
    }
}

struct Sandbox {
    spaces: Vec<EMSpace>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { spaces: Vec::new() };
        sb.spaces.push(EMSpace::new("ℤ", 1));
        sb.spaces.push(EMSpace::new("ℤ", 2));
        sb.spaces.push(EMSpace::new("ℤ/2ℤ", 1));
        sb
    }

    fn print_help() {
        println!("{}", bold("── Eilenberg-MacLane Spaces ─────────────────────────────────────────"));
        println!("  {}  <G> <n>      — show K(G,n)", cyan("km"));
        println!("  {}         — list K(G,n) spaces built so far", cyan("list"));
        println!("  {}  <X> <n> <G>  — compute Hⁿ(X; G) via K(G,n)", cyan("cohom"));
        println!("{}", bold("── Theory ────────────────────────────────────────────────────────────"));
        println!("  {}    — represented cohomology theorem", cyan("represented"));
        println!("  {}  <G> <n>      — loop/suspension of K(G,n)", cyan("loop"));
        println!("  {}     — postnikov towers via K(G,n)", cyan("postnikov"));
        println!("  {}  <G>          — classifying space BG = K(G,1)", cyan("classifying"));
        println!("{}", bold("── Computations ─────────────────────────────────────────────────────"));
        println!("  {}   — Steenrod operations and K(ℤ/2,n)", cyan("steenrod"));
        println!("  {}  <G>          — group cohomology via K(G,1)", cyan("group-cohom"));
        println!("  {}     — cup product structure", cyan("cup"));
        println!("  G can be: Z, Z2, Z3, Zn");
    }

    fn handle(&mut self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() { return true; }
        if trimmed == "quit" || trimmed == "exit" || trimmed == "q" { return false; }
        if trimmed == "help" || trimmed == "?" { Self::print_help(); return true; }

        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        let cmd = parts[0];
        let rest = if parts.len() > 1 { parts[1].trim() } else { "" };

        match cmd {
            "km" | "K" => {
                let args: Vec<&str> = rest.split_whitespace().collect();
                if args.len() < 2 {
                    println!("  Usage: km <G> <n>   e.g. km Z 2  or  km Z2 1");
                    return true;
                }
                let group = args[0];
                let n: usize = args[1].parse().unwrap_or(1);
                let sp = EMSpace::new(group, n);
                sp.display();
                self.spaces.push(sp);
            }
            "list" => {
                println!("  Eilenberg-MacLane spaces:");
                for (i, sp) in self.spaces.iter().enumerate() {
                    println!("  #{i}: K({},{}) — {}", sp.group, sp.n, dim(&sp.description));
                }
            }
            "cohom" => {
                let args: Vec<&str> = rest.split_whitespace().collect();
                if args.len() < 3 {
                    println!("  Usage: cohom <X> <n> <G>  e.g. cohom S1 1 Z");
                    return true;
                }
                let (space, n_str, g) = (args[0], args[1], args[2]);
                let n: usize = n_str.parse().unwrap_or(0);
                let val = cohomology(space, n, g);
                println!("  H^{n}({space}; {g}) = {}", cyan(&val));
                println!("  Via represented cohomology: [X, K({g},{n})]");
            }
            "represented" => {
                println!("{}", bold("── Represented Cohomology ────────────────────────────────────────────"));
                println!("  Brown representability theorem:");
                println!("    Hⁿ(X; G) ≃ [X, K(G,n)]  (homotopy classes of maps)");
                println!();
                println!("  This means:");
                println!("  • Cohomology classes = maps into Eilenberg-MacLane spaces");
                println!("  • Cohomology operations = natural transformations between them");
                println!("  • Cup product = composition with the pairing K(G,n)×K(G,n)→K(G,2n)");
                println!();
                println!("  In HoTT:");
                println!("    Hⁿ(X; G) := ‖ X → K(G,n) ‖₀  (0-truncation = set of maps)");
                println!("    For abelian G and n≥1, K(G,n) is an abelian group in HoTT.");
                println!();
                println!("  Examples:");
                println!("    H¹(X; ℤ) ≃ [X, K(ℤ,1)] = [X, S¹] = Hom(π₁X, ℤ)");
                println!("    H²(X; ℤ) ≃ [X, K(ℤ,2)] = [X, CP∞] = line bundles on X");
                println!("    H¹(X; ℤ/2) ≃ [X, RP∞] = real line bundles on X");
            }
            "loop" => {
                let args: Vec<&str> = rest.split_whitespace().collect();
                let group = if args.is_empty() { "G" } else { args[0] };
                let n: usize = if args.len() > 1 { args[1].parse().unwrap_or(2) } else { 2 };
                println!("  Loop and suspension of Eilenberg-MacLane spaces:");
                println!("    ΩK({group},{n}) ≃ K({group},{}) for n≥1", n.saturating_sub(1));
                println!("    ΣK({group},{n}) — not generally an EM space");
                println!();
                println!("  The sequence … K(G,0) ← K(G,1) ← K(G,2) ← …");
                println!("  forms an Ω-spectrum: the Eilenberg-MacLane spectrum HG.");
                println!();
                println!("  Concrete: ΩS¹ = K(ℤ,0) = ℤ (discrete)");
                println!("            ΩCP∞ = S¹ = K(ℤ,1)");
                println!("            Ω(K(ℤ/2,2)) = K(ℤ/2,1) = RP∞");
            }
            "postnikov" => {
                println!("{}", bold("── Postnikov Towers ──────────────────────────────────────────────────"));
                println!("  Every space X has a Postnikov tower:");
                println!("    X → … → X[3] → X[2] → X[1] → *");
                println!();
                println!("  X[n] = n-th Postnikov section: πₖ(X[n]) = πₖ(X) for k≤n, else 0");
                println!();
                println!("  Each layer is a principal fibration with fiber K(πₙ(X), n):");
                println!("    K(πₙ(X), n) → X[n] → X[n-1]");
                println!("  classified by the k-invariant kⁿ ∈ Hⁿ⁺¹(X[n-1]; πₙ(X)).");
                println!();
                println!("  For a K(G,n) space: only one non-trivial layer.");
                println!();
                println!("  Example: S² has Postnikov tower:");
                println!("    K(ℤ,2) = CP∞ → S²[2] = S²");
                println!("    S²[3] is K(ℤ,2)-fibration over S²[2], classified by k³ ∈ H⁴(S²;ℤ) = ℤ");
                println!("    This k-invariant is the generator, giving S²[3].");
            }
            "classifying" => {
                let g = if rest.is_empty() { "G" } else { rest };
                println!("  Classifying space BG = K(G,1) for {g}:", );
                println!("  • πₖ(BG) = 0 for k≠1,  π₁(BG) = G");
                println!();
                match g {
                    "Z" | "ℤ" => {
                        println!("  B(ℤ) = K(ℤ,1) = S¹");
                        println!("  Principal ℤ-bundles over X ↔ [X, S¹] = H¹(X;ℤ)");
                    }
                    "Z2" | "ℤ/2ℤ" => {
                        println!("  B(ℤ/2) = K(ℤ/2,1) = RP∞");
                        println!("  Real line bundles over X ↔ [X, RP∞] = H¹(X;ℤ/2)");
                    }
                    "S1" | "U1" => {
                        println!("  B(S¹) = B(U(1)) = K(ℤ,2) = CP∞");
                        println!("  Complex line bundles over X ↔ [X, CP∞] = H²(X;ℤ) = Pic(X)");
                    }
                    _ => {
                        println!("  BG = K(G,1) classifies principal G-bundles.");
                        println!("  Principal G-bundles over X ↔ [X, BG] = H¹(X; G)  (for abelian G)");
                    }
                }
            }
            "steenrod" => {
                println!("{}", bold("── Steenrod Operations and K(ℤ/2, n) ────────────────────────────────"));
                println!("  Steenrod squares: Sqⁱ: Hⁿ(X;ℤ/2) → Hⁿ⁺ⁱ(X;ℤ/2)");
                println!("  These are cohomology operations, i.e., natural transformations");
                println!("  Hⁿ(-;ℤ/2) → Hⁿ⁺ⁱ(-;ℤ/2).");
                println!();
                println!("  By representability: cohomology operations correspond to");
                println!("  elements of H*(K(ℤ/2,n); ℤ/2) (as n varies).");
                println!();
                println!("  Serre computation:");
                println!("    H*(K(ℤ/2,1); ℤ/2) = H*(RP∞; ℤ/2) = ℤ/2[x], |x|=1");
                println!("    H*(K(ℤ/2,2); ℤ/2) = ℤ/2[x, Sq¹x, Sq²Sq¹x, ...], |x|=2");
                println!();
                println!("  Cartan seminar computed all H*(K(ℤ/2,n); ℤ/2) using Sq operations.");
                println!("  This is foundational for the Adams spectral sequence.");
            }
            "group-cohom" => {
                let g = if rest.is_empty() { "G" } else { rest };
                println!("  Group cohomology via K(G,1) = BG:");
                println!("  H^n(G; M) = Hⁿ(BG; M) = Hⁿ(K(G,1); M)");
                println!();
                match g {
                    "Z" | "ℤ" => {
                        println!("  H^n(ℤ; M) = Hⁿ(S¹; M):");
                        println!("    H⁰ = Mᴳ (fixed points)");
                        println!("    H¹ = M_G (coinvariants = M/(gm-m))");
                        println!("    H^n = 0 for n≥2  (ℤ has cohomological dimension 1)");
                    }
                    "Z2" | "ℤ/2ℤ" => {
                        println!("  H^n(ℤ/2; M) = Hⁿ(RP∞; M):");
                        println!("    H⁰ = Mᴳ (fixed points)");
                        println!("    H^{{2k+1}} = ker(N)/im(D)  for k≥0");
                        println!("    H^{{2k}} = ker(D)/im(N)  for k≥1");
                        println!("  where N = norm map, D = difference map (for M = ℤ with trivial action):");
                        println!("    H^{{2k}}(ℤ/2;ℤ) = ℤ/2,  H^{{2k+1}}(ℤ/2;ℤ) = ℤ/2  for k≥1");
                    }
                    _ => {
                        println!("  H^n({g}; M) = Hⁿ(K({g},1); M) = group cohomology");
                        println!("  Computed via the bar construction or explicit resolutions.");
                    }
                }
            }
            "cup" => {
                println!("{}", bold("── Cup Product via Eilenberg-MacLane Spaces ─────────────────────────"));
                println!("  The cup product:");
                println!("    ∪: Hᵐ(X;G) × Hⁿ(X;G) → Hᵐ⁺ⁿ(X;G)");
                println!("  corresponds to:");
                println!("    [f]: X→K(G,m),  [g]: X→K(G,n)");
                println!("    [f∪g]: X →^Δ X×X →^{{f×g}} K(G,m)×K(G,n) →^μ K(G,m+n)");
                println!();
                println!("  where μ: K(G,m)×K(G,n) → K(G,m+n) is the multiplication map");
                println!("  for the H-space structure on K(G,*).");
                println!();
                println!("  This makes H*(X;G) into a graded ring.");
                println!();
                println!("  In HoTT: H*(X;G) is a graded ring with cup product");
                println!("  defined by the group structure on K(G,n).");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Eilenberg-MacLane Spaces — Interactive Sandbox         ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  K(G,n): the space with πₙ=G and all other homotopy groups trivial.");
    println!("  Cohomology as maps: Hⁿ(X;G) ≃ [X, K(G,n)].");
    println!("  Type {} or {} to start exploring.\n",
        cyan("km Z 2"), cyan("represented"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}K(G,n){} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
