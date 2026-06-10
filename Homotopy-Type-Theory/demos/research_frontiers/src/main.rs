use std::io::{self, BufRead, Write};
use std::collections::HashMap;

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

// ── Research Frontiers in HoTT ────────────────────────────────────────────────
//
// Active research areas as of 2024–2025:
//
//  1. Two-level type theory (2LTT) — strict + homotopy types together
//  2. Directed type theory — (∞,1)-categories internally
//  3. Differential cohomology and physics applications
//  4. Formal verification of advanced mathematics (Liquid Tensor, etc.)
//  5. Computational implementations (Cubical, XTT, RedTT)
//  6. Higher algebra in HoTT (∞-groups, spectra, E∞-rings)
//  7. Synthetic algebraic geometry

struct ResearchArea {
    name: String,
    status: &'static str,
    key_workers: Vec<String>,
    description: String,
    key_results: Vec<String>,
    open_problems: Vec<String>,
}

impl ResearchArea {
    fn new(name: &str, status: &'static str) -> Self {
        ResearchArea {
            name: name.into(),
            status,
            key_workers: Vec::new(),
            description: String::new(),
            key_results: Vec::new(),
            open_problems: Vec::new(),
        }
    }

    fn display_summary(&self) {
        let status_str = match self.status {
            "active" => green("active"),
            "emerging" => yellow("emerging"),
            "mature" => cyan("mature"),
            _ => dim(self.status),
        };
        println!("  {} [{}]  {}", cyan(&self.name), status_str, self.description);
    }

    fn display_full(&self) {
        println!("{}", bold(&format!("── {} ────────────────────────────────────────────────────────", self.name)));
        println!("  Status: {} | Key people: {}", self.status, self.key_workers.join(", "));
        println!("  {}", self.description);
        if !self.key_results.is_empty() {
            println!("  {} Results:", green("✓"));
            for r in &self.key_results { println!("    • {r}"); }
        }
        if !self.open_problems.is_empty() {
            println!("  {} Open problems:", yellow("?"));
            for p in &self.open_problems { println!("    • {p}"); }
        }
    }
}

fn build_areas() -> HashMap<String, ResearchArea> {
    let mut m = HashMap::new();

    let mut a = ResearchArea::new("Two-Level Type Theory (2LTT)", "active");
    a.key_workers = vec!["Annenkov".into(), "Capriotti".into(), "Kraus".into(), "Sattler".into()];
    a.description = "Combines strict equality with homotopy equality for better foundation".into();
    a.key_results = vec![
        "2LTT can interpret both MLTT and HoTT simultaneously".into(),
        "Strict coercions allow defining homotopy-invariant operations".into(),
        "Enables synthetic (∞,1)-category theory without directed interval".into(),
    ];
    a.open_problems = vec![
        "Find canonical 2LTT with good computational properties".into(),
        "Relate to XTT and multimodal type theory".into(),
    ];
    m.insert("2ltt".into(), a);

    let mut a = ResearchArea::new("Directed Type Theory", "emerging");
    a.key_workers = vec!["Riehl".into(), "Shulman".into(), "Weaver".into(), "Licata".into()];
    a.description = "Type theory with directed morphisms for (∞,1)-categories internally".into();
    a.key_results = vec![
        "Simplicial HoTT (Riehl-Shulman 2017): Segal types, Yoneda, adjoints".into(),
        "Yoneda lemma proved synthetically in simplicial HoTT".into(),
        "Cartesian fibrations characterized type-theoretically".into(),
    ];
    a.open_problems = vec![
        "Combine cubical (univalent) and simplicial (directed) in one theory".into(),
        "Formalize in a proof assistant without ad-hoc additions".into(),
        "Complete the Grothendieck construction internally".into(),
    ];
    m.insert("directed".into(), a);

    let mut a = ResearchArea::new("Cohesive HoTT and Differential Cohomology", "active");
    a.key_workers = vec!["Schreiber".into(), "Shulman".into(), "Myers".into()];
    a.description = "Synthetic differential geometry and physics formalized in HoTT".into();
    a.key_results = vec![
        "Cohesive HoTT: ʃ⊣♭⊣♯ axioms for smooth structure".into(),
        "De Rham theorem proved synthetically".into(),
        "Prequantum field theory formalized in cohesive HoTT".into(),
        "Higher gauge theory (Chern-Weil) formalized".into(),
    ];
    a.open_problems = vec![
        "Quantization (path integrals) in synthetic HoTT".into(),
        "M-theory formalization (Schreiber's program)".into(),
    ];
    m.insert("cohesive".into(), a);

    let mut a = ResearchArea::new("Formal Verification of Advanced Math", "active");
    a.key_workers = vec!["Scholze".into(), "Clausen".into(), "Commelin".into(), "Buzzard".into()];
    a.description = "Machine-verified proofs of cutting-edge theorems".into();
    a.key_results = vec![
        "Liquid Tensor Experiment: condensed mathematics in Lean 4".into(),
        "Perfectoid spaces in Lean 3 (Buzzard et al.)".into(),
        "Fermat's Last Theorem: modular forms in Lean 4 (in progress)".into(),
        "Brunerie number β=2 in Cubical Agda (2022)".into(),
    ];
    a.open_problems = vec![
        "Fully formalize the Langlands program".into(),
        "Automated discovery of new mathematics".into(),
    ];
    m.insert("verification".into(), a);

    let mut a = ResearchArea::new("Higher Algebra and Spectra", "active");
    a.key_workers = vec!["Anel".into(), "Joyal".into(), "Barwick".into(), "Gepner".into()];
    a.description = "∞-groups, spectra, E∞-rings, and higher K-theory in HoTT".into();
    a.key_results = vec![
        "Delooping machinery for A∞ and E∞-spaces in HoTT".into(),
        "Spectrum objects defined as sequential spectra in HoTT".into(),
        "Algebraic K-theory of type theory formalized".into(),
        "∞-Topos of ∞-groups classified by BG = K(G,1) for ∞-groups G".into(),
    ];
    a.open_problems = vec![
        "E∞-ring spectra in HoTT without strict commutativity issues".into(),
        "Chromatic homotopy theory (v_n periodicity) in HoTT".into(),
    ];
    m.insert("higher-algebra".into(), a);

    let mut a = ResearchArea::new("XTT and Multimodal Type Theory", "emerging");
    a.key_workers = vec!["Sterling".into(), "Harper".into(), "Birkedal".into(), "Gratzer".into()];
    a.description = "Extension type theory: combines HoTT with strict operations".into();
    a.key_results = vec![
        "XTT: extension types give a cartesian cubical model".into(),
        "Multimodal type theory (MTT): unifies modal type theories".into(),
        "Parametricity from multimodality: free theorems for free".into(),
    ];
    a.open_problems = vec![
        "Canonicity for XTT/MTT (do all closed terms normalize?)".into(),
        "Extend MTT to ∞-categorical settings".into(),
    ];
    m.insert("xtt".into(), a);

    let mut a = ResearchArea::new("Synthetic Algebraic Geometry (SAG)", "emerging");
    a.key_workers = vec!["Anel".into(), "Blechschmidt".into(), "Cherubini".into()];
    a.description = "Algebraic geometry done internally in an ∞-topos of sheaves".into();
    a.key_results = vec![
        "Internal Zariski topos: schemes as synthetic objects".into(),
        "Quasi-coherent sheaves = modules over the generic ring object".into(),
        "Naïve commutative algebra works internally".into(),
    ];
    a.open_problems = vec![
        "Prove Serre's GAGA theorem synthetically".into(),
        "Coherent duality in the synthetic setting".into(),
    ];
    m.insert("sag".into(), a);

    let mut a = ResearchArea::new("Univalent Foundations of Mathematics", "mature");
    a.key_workers = vec!["Voevodsky".into(), "Awodey".into(), "Warren".into(), "van den Berg".into()];
    a.description = "Using HoTT as a new foundation for all of mathematics".into();
    a.key_results = vec![
        "UniMath library: basic mathematics formalized in Coq with univalence".into(),
        "HoTT Book (2013): collective effort on foundations".into(),
        "π₁(S¹) = ℤ, Blakers-Massey, Freudenthal all proved in HoTT".into(),
        "Whitehead's theorem proved in HoTT".into(),
    ];
    a.open_problems = vec![
        "Propositional resizing: is it consistent with univalence?".into(),
        "The right notion of 'set' for foundations (ETCS vs ZFC in HoTT)".into(),
    ];
    m.insert("foundations".into(), a);

    m
}

struct Sandbox {
    areas: HashMap<String, ResearchArea>,
    viewed: Vec<String>,
}

impl Sandbox {
    fn new() -> Self {
        Sandbox { areas: build_areas(), viewed: Vec::new() }
    }

    fn print_help() {
        println!("{}", bold("── Research Frontiers in HoTT ───────────────────────────────────────"));
        println!("  {}         — list all research areas", cyan("list"));
        println!("  {}  <area>     — show details for an area", cyan("show"));
        println!("  {}   — compare competing approaches", cyan("landscape"));
        println!("{}", bold("── Specific Areas ────────────────────────────────────────────────────"));
        println!("  {}         — Two-Level Type Theory", cyan("2ltt"));
        println!("  {}      — directed type theory (simplicial HoTT)", cyan("directed"));
        println!("  {}      — cohesive HoTT and physics", cyan("cohesive"));
        println!("  {} — formal verification (Liquid Tensor etc.)", cyan("verification"));
        println!("  {}  — ∞-groups, spectra in HoTT", cyan("higher-algebra"));
        println!("  {}          — XTT and multimodal type theory", cyan("xtt"));
        println!("  {}          — synthetic algebraic geometry", cyan("sag"));
        println!("  {}   — univalent foundations program", cyan("foundations"));
        println!("{}", bold("── Big Questions ─────────────────────────────────────────────────────"));
        println!("  {}     — key open problems", cyan("open"));
        println!("  {}     — future directions", cyan("future"));
        println!("  {}    — entry points for researchers", cyan("entry"));
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
            "list" => {
                println!("  Research Areas in HoTT:");
                let mut names: Vec<&String> = self.areas.keys().collect();
                names.sort();
                for k in &names {
                    let a = &self.areas[*k];
                    a.display_summary();
                }
            }
            "show" | "2ltt" | "directed" | "cohesive" | "verification"
            | "higher-algebra" | "xtt" | "sag" | "foundations" => {
                let key = if cmd == "show" { rest } else { cmd };
                match self.areas.get(key) {
                    Some(a) => {
                        a.display_full();
                        self.viewed.push(key.into());
                    }
                    None => println!("  {} Unknown area: {key}. Type {} for list.", red("✗"), cyan("list")),
                }
            }
            "landscape" => {
                println!("{}", bold("── The Research Landscape ───────────────────────────────────────────"));
                println!("  Three main tensions in HoTT research:");
                println!();
                println!("  1. {} vs {}", cyan("Classical"), cyan("Constructive"));
                println!("     Classical (Lean 4 + Mathlib): LEM, choice, large library");
                println!("     Constructive (Cubical Agda): computable, no axioms, HIT-native");
                println!();
                println!("  2. {} vs {}", cyan("Undirected (Book HoTT)"), cyan("Directed (Simp. HoTT)"));
                println!("     Book HoTT: ∞-groupoids, works in cubical");
                println!("     Simplicial HoTT: (∞,1)-categories, directed morphisms");
                println!();
                println!("  3. {} vs {}", cyan("Foundations"), cyan("Applications"));
                println!("     Foundations: get the type theory right first");
                println!("     Applications: use HoTT as a tool for math/CS");
                println!();
                println!("  Current consensus: use Cubical Agda for HoTT research,");
                println!("  Lean 4 for classical math formalization.");
            }
            "open" => {
                println!("{}", bold("── Key Open Problems in HoTT ────────────────────────────────────────"));
                println!("  1. {} Canonicity and normalization for cubical type theory with HITs",
                    yellow("Computational:"));
                println!("     (Closed under computation? All terms reduce to canonical form?)");
                println!();
                println!("  2. {} Single type theory for both univalent + directed",
                    yellow("Foundational:"));
                println!("     (Simplicial HoTT + cubical = ?)");
                println!();
                println!("  3. {} Is propositional resizing consistent with UA?",
                    yellow("Logical:"));
                println!("     (Can every Prop be in 𝒰₀?)");
                println!();
                println!("  4. {} Develop algebraic topology synthetically to state-of-art",
                    yellow("Mathematical:"));
                println!("     (Chromatic homotopy, L-theory, etc.)");
                println!();
                println!("  5. {} Quantum field theory in cohesive HoTT",
                    yellow("Physics:"));
                println!("     (Path integrals, quantization, anomalies)");
                println!();
                println!("  6. {} Efficient proof automation for HoTT",
                    yellow("Practical:"));
                println!("     (Tactics, decision procedures for homotopy-level reasoning)");
            }
            "future" => {
                println!("{}", bold("── Future Directions ────────────────────────────────────────────────"));
                println!("  Near term (2025–2030):");
                println!("    • Complete Brunerie's program: all of synthetic homotopy theory");
                println!("    • Directed HoTT: full Yoneda, limits, adjoint functor theorem");
                println!("    • Liquid mathematics: condensed sets, nuclear modules in HoTT");
                println!("    • AI-assisted HoTT: LLMs as tactic suggesters");
                println!();
                println!("  Medium term (2030–2040):");
                println!("    • A single proof assistant for all of mathematics");
                println!("    • Formalizing the Langlands correspondence");
                println!("    • Cohesive HoTT as foundation for mathematical physics");
                println!("    • Automated generation of new theorems from HoTT");
                println!();
                println!("  Long term:");
                println!("    • HoTT as the 'assembly language' of mathematics");
                println!("    • Quantum computation type theory (already emerging)");
                println!("    • Verified AI systems (types as specifications)");
            }
            "entry" => {
                println!("{}", bold("── Entry Points for Researchers ─────────────────────────────────────"));
                println!("  If you're coming from:");
                println!();
                println!("  {}:", bold("Algebraic topology"));
                println!("    → Start with HoTT Book Ch 2–8, then Brunerie's thesis");
                println!("    → Learn Cubical Agda, formalize π₁(S¹)");
                println!();
                println!("  {}:", bold("Category theory"));
                println!("    → Simplicial HoTT (Riehl-Shulman)");
                println!("    → Categorical logic and internal languages");
                println!();
                println!("  {}:", bold("Logic / proof theory"));
                println!("    → HoTT as an extension of MLTT");
                println!("    → 2LTT, multimodal type theory (MTT)");
                println!();
                println!("  {}:", bold("Programming languages / CS"));
                println!("    → Lean 4 + Mathlib for automated reasoning");
                println!("    → Cubical Agda for computational HoTT");
                println!("    → Parametricity and relational type theory");
                println!();
                println!("  {}:", bold("Physics / geometry"));
                println!("    → Cohesive HoTT (Schreiber-Shulman)");
                println!("    → Synthetic differential geometry in HoTT");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Research Frontiers in HoTT — Interactive Sandbox       ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore active research frontiers in Homotopy Type Theory.");
    println!("  From directed type theory to synthetic algebraic geometry.");
    println!("  Type {} for an overview, {} for open problems.\n",
        cyan("list"), cyan("open"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}frontiers{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
