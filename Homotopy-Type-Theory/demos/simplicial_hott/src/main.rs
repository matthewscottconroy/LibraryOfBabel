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

// ── Simplicial HoTT ────────────────────────────────────────────────────────────
//
// Simplicial HoTT (Riehl-Shulman 2017): extends HoTT with a directed interval 2
// to talk about (∞,1)-categories internally.
//
// 2 : 𝒰 with endpoints 0, 1 : 2 and no path between them (directed).
//
// Arrow type: A^2 = 2 → A  (directed morphisms in A)
// hom_A(x, y) : 𝒰  (the type of morphisms from x to y)
//
// A type A is a Segal type if composition is unique:
//   Segal(A) := ∀ (f: A^2)(g: A^2)(p: f(1)=g(0)), isContr(Σh, h∘01=f, h∘12=g)

// Simple category representation for interactive exploration
struct Morphism {
    source: String,
    target: String,
    label: String,
}

struct SegalType {
    name: String,
    objects: Vec<String>,
    morphisms: Vec<Morphism>,
}

impl SegalType {
    fn new(name: &str) -> Self {
        SegalType { name: name.into(), objects: Vec::new(), morphisms: Vec::new() }
    }

    fn add_obj(&mut self, o: &str) { self.objects.push(o.into()); }

    fn add_mor(&mut self, src: &str, tgt: &str, lbl: &str) {
        self.morphisms.push(Morphism { source: src.into(), target: tgt.into(), label: lbl.into() });
    }

    fn composable(&self, f: &str, g: &str) -> Option<(&Morphism, &Morphism)> {
        let mf = self.morphisms.iter().find(|m| m.label == f)?;
        let mg = self.morphisms.iter().find(|m| m.label == g)?;
        if mf.target == mg.source { Some((mf, mg)) } else { None }
    }

    fn display(&self) {
        println!("  Segal type «{}»", cyan(&self.name));
        println!("  Objects: {}", self.objects.join(", "));
        let mors: Vec<String> = self.morphisms.iter()
            .map(|m| format!("{}: {}→{}", m.label, m.source, m.target))
            .collect();
        println!("  Morphisms: {}", mors.join(";  "));
    }
}

struct Sandbox {
    types: Vec<SegalType>,
    current: usize,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { types: Vec::new(), current: 0 };
        let mut t = SegalType::new("example");
        t.add_obj("a"); t.add_obj("b"); t.add_obj("c");
        t.add_mor("a", "b", "f");
        t.add_mor("b", "c", "g");
        sb.types.push(t);
        sb
    }

    fn print_help() {
        println!("{}", bold("── Simplicial HoTT Sandbox ─────────────────────────────────────────"));
        println!("  {}  <name>     — create a Segal type", cyan("segal"));
        println!("  {}  <x>        — add an object", cyan("obj"));
        println!("  {}  <x> <y> <f>  — add a morphism f: x→y", cyan("mor"));
        println!("  {}  <f> <g>    — compose morphisms (check Segal condition)", cyan("comp"));
        println!("  {}            — display current type", cyan("show"));
        println!("{}", bold("── Theory ──────────────────────────────────────────────────────────"));
        println!("  {}     — directed interval 2 and arrow types", cyan("interval"));
        println!("  {}      — the Segal condition for types", cyan("segal-cond"));
        println!("  {}   — complete Segal types = (∞,1)-cats", cyan("complete"));
        println!("  {}      — yoneda lemma in simplicial HoTT", cyan("yoneda"));
        println!("  {}       — adjunctions", cyan("adjoint"));
        println!("{}", bold("── Special Types ──────────────────────────────────────────────────"));
        println!("  {}     — representable presheaves", cyan("representable"));
        println!("  {}    — fibrations over Segal types", cyan("fibration"));
        println!("  {}  <n>        — the nerve of an n-category", cyan("nerve"));
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
            "segal" => {
                let name = if rest.is_empty() { "A" } else { rest };
                let t = SegalType::new(name);
                self.current = self.types.len();
                self.types.push(t);
                println!("  Created Segal type «{}» (#{}).", cyan(name), self.current);
            }
            "obj" => {
                if self.types.is_empty() { println!("  Create a type first with {}.", cyan("segal")); return true; }
                let o = if rest.is_empty() { "x" } else { rest };
                self.types[self.current].add_obj(o);
                println!("  Added object {}.", cyan(o));
            }
            "mor" => {
                if self.types.is_empty() { println!("  Create a type first with {}.", cyan("segal")); return true; }
                let args: Vec<&str> = rest.splitn(3, ' ').collect();
                if args.len() < 3 { println!("  Usage: mor <source> <target> <label>"); return true; }
                self.types[self.current].add_mor(args[0], args[1], args[2]);
                println!("  Added morphism {}: {} → {}.", cyan(args[2]), args[0], args[1]);
                println!("  In simplicial HoTT: this is a term in hom_A({}, {})", args[0], args[1]);
            }
            "comp" => {
                if self.types.is_empty() { return true; }
                let args: Vec<&str> = rest.split_whitespace().collect();
                if args.len() < 2 { println!("  Usage: comp <f> <g>"); return true; }
                let t = &self.types[self.current];
                match t.composable(args[0], args[1]) {
                    Some((mf, mg)) => {
                        println!("  {} ∘ {} : {} → {}", args[1], args[0], mf.source, mg.target);
                        println!("  {} Composable! The Segal condition asserts this is unique.", green("✓"));
                        println!("  In simplicial HoTT:");
                        println!("    comp(f,g) : hom_A({},{}) where", mf.source, mg.target);
                        println!("    comp(f,g)(0) = f(0) = {},  comp(f,g)(1) = g(1) = {}", mf.source, mg.target);
                        println!("    comp(f,g)|_{{0,1}} = f,  comp(f,g)|_{{1,2}} = g");
                    }
                    None => {
                        let mf = t.morphisms.iter().find(|m| m.label == args[0]);
                        let mg = t.morphisms.iter().find(|m| m.label == args[1]);
                        match (mf, mg) {
                            (Some(f), Some(g)) => println!("  {} {} ends at {} but {} starts at {}",
                                red("✗"), args[0], f.target, args[1], g.source),
                            _ => println!("  {} Morphism not found.", red("✗")),
                        }
                    }
                }
            }
            "show" => {
                if self.types.is_empty() { println!("  No types yet."); return true; }
                self.types[self.current].display();
            }
            "interval" => {
                println!("{}", bold("── The Directed Interval 2 ─────────────────────────────────────────"));
                println!("  In simplicial HoTT, there is a primitive type {} : 𝒰", cyan("2"));
                println!("  with two points: 0 : 2  and  1 : 2");
                println!();
                println!("  Crucially: there is NO path between 0 and 1 in 2.");
                println!("  {} 2 is NOT a proposition (we can't prove 0=1 or 0≠1)", yellow("Note:"));
                println!("  Instead: 2 has a strict order 0 < 1  (the generating morphism).");
                println!();
                println!("  Arrow type: A^2 = (2 → A)  = the type of morphisms in A");
                println!("  For f : A^2:  f(0) = source,  f(1) = target");
                println!();
                println!("  hom_A(x,y) := {{f : 2→A | f(0)=x ∧ f(1)=y}}");
                println!("  This is the type of directed paths from x to y in A.");
                println!();
                println!("  Simplex types: Δⁿ = {{f : [n]→A | monotone}}");
                println!("    Δ⁰ = A,  Δ¹ = A^2 = Arrow(A),  Δ² = composable pairs");
            }
            "segal-cond" => {
                println!("{}", bold("── The Segal Condition ──────────────────────────────────────────────"));
                println!("  A type A is Segal if:");
                println!("  ∀(f g : 2→A)(p : f(1) = g(0)),  isContr(Σ h:Δ²→A, h|₀₁=f ∧ h|₁₂=g)");
                println!();
                println!("  Intuition: given composable morphisms f:x→y and g:y→z,");
                println!("  there is a unique 2-simplex (triangle) with these as two sides.");
                println!("  The third side is the composite g∘f.");
                println!();
                println!("  In simplicial sets: Segal = all spine inclusions have unique extensions");
                println!("  Equivalently: all inner horn fillers for Λⁿᵢ are unique (0<i<n).");
                println!();
                println!("  Examples of Segal types:");
                println!("    • Any groupoid (every type in Book HoTT is Segal!)");
                println!("    • Any (∞,1)-category (when formalized as a Segal type)");
                println!("    • The universe 𝒰 (with Segal condition from univalence)");
                println!();
                println!("  {} Every type in Book HoTT is Segal (invertible morphisms are unique up to homotopy).", green("Theorem:"));
            }
            "complete" => {
                println!("{}", bold("── Completeness: Segal + Complete = (∞,1)-Category ─────────────────"));
                println!("  A Segal type A is {}:", bold("complete"));
                println!("    the identity-assigning map  A → A^2  x ↦ id_x");
                println!("    is an equivalence into the sub-type of equivalences.");
                println!();
                println!("  Equivalently: every equivalence is a unit (id) in A.");
                println!();
                println!("  Complete Segal = (∞,1)-category.");
                println!("  This matches the Rezk model: complete Segal spaces.");
                println!();
                println!("  In simplicial HoTT:");
                println!("    Riehl-Shulman develop category theory internally in the system.");
                println!("    The Yoneda lemma, adjoints, limits all become theorems.");
                println!();
                println!("  Without completeness: Segal spaces (Barwick, Rezk)");
                println!("  = (∞,1)-categories up to Dwyer-Kan equivalence.");
            }
            "yoneda" => {
                println!("{}", bold("── Yoneda Lemma in Simplicial HoTT ────────────────────────────────"));
                println!("  For a Segal type A and a : A, the presheaf:");
                println!("    A(−,a) : Aᵒᵖ → 𝒰  defined by  x ↦ hom_A(x,a)");
                println!("  is representable.");
                println!();
                println!("  Yoneda Lemma:");
                println!("    Nat(A(−,a), F) ≃ F(a)");
                println!("    (natural transformations from the representable ≃ F at a)");
                println!();
                println!("  Proof in simplicial HoTT:");
                println!("    The map  F(a) → Nat(A(−,a), F)  is  y ↦ (f ↦ F(f)(y))");
                println!("    This is an equivalence by the Fundamental Theorem of ID types.");
                println!("    (Applying FTIT in the simplicial setting.)");
                println!();
                println!("  This is the internal Yoneda lemma for (∞,1)-categories in HoTT.");
            }
            "adjoint" => {
                println!("{}", bold("── Adjunctions in Simplicial HoTT ─────────────────────────────────"));
                println!("  F : A → B  has a right adjoint G : B → A if:");
                println!("    hom_B(F(x), y) ≃ hom_A(x, G(y))  naturally in x,y");
                println!();
                println!("  In simplicial HoTT, this is an equivalence of types:");
                println!("    (2→B, 0↦F(x), 1↦y) ≃ (2→A, 0↦x, 1↦G(y))");
                println!("  parameterized by x:A, y:B.");
                println!();
                println!("  The unit η: id_A → G∘F and counit ε: F∘G → id_B satisfy");
                println!("  triangle identities up to coherent homotopy (all in HoTT).");
                println!();
                println!("  Key theorem: F ⊣ G iff there exist η,ε satisfying triangles.");
                println!("  Proof in simplicial HoTT: Riehl-Shulman §6.");
            }
            "representable" => {
                println!("{}", bold("── Representable Presheaves ─────────────────────────────────────────"));
                println!("  A presheaf F : Aᵒᵖ → 𝒰 is representable if:");
                println!("    ∃ a:A, F ≃ A(−,a)  (naturally)");
                println!();
                println!("  In simplicial HoTT: representables correspond to points of A.");
                println!("  By Yoneda: Aᵒᵖ → PSh(A)  is fully faithful.");
                println!();
                println!("  Every limit in A is a representable presheaf:");
                println!("    lim(D) is the unique a:A such that A(−,a) ≃ lim(A(−,D(−)))");
                println!();
                println!("  This is how limits and colimits are defined in simplicial HoTT.");
            }
            "fibration" => {
                println!("{}", bold("── Fibrations over Segal Types ──────────────────────────────────────"));
                println!("  A map p : E → B over a Segal type B is a:");
                println!();
                println!("  {} — has transport along morphisms, not just paths", bold("Discrete fibration"));
                println!("    ∀f:hom_B(x,y), ∀e:E(x), unique lift of f to E");
                println!("    Corresponds to a covariant functor B → Set/Type");
                println!();
                println!("  {} — has transport and also has inverse", bold("Discrete opfibration"));
                println!("    Contravariant functor Bᵒᵖ → Type");
                println!();
                println!("  {} — complete Segal fibers", bold("Cartesian fibration"));
                println!("    Corresponds to a functor B → Cat_∞");
                println!("    Lurie calls these ∞-categorical fibrations.");
                println!();
                println!("  In simplicial HoTT: all of these are internalized type families.");
                println!("  A type family C : B → 𝒰 = a fibration over B.");
            }
            "nerve" => {
                let n: usize = rest.parse().unwrap_or(2);
                println!("{}", bold(&format!("── Nerve of a {n}-Category ────────────────────────────────────────────")));
                println!("  The nerve N(C) of a 1-category C is a simplicial set:");
                println!("  N(C)_n = {{composable chains c₀→c₁→…→cₙ of n morphisms}}");
                println!();
                println!("  N(C) satisfies the Segal condition:");
                println!("    N(C)_{{m+n}} ≃ N(C)_m ×_{{N(C)_0}} N(C)_n  (Segal maps are equivs)");
                println!();
                match n {
                    1 => println!("  Nerve of a category: the classical construction."),
                    2 => {
                        println!("  Nerve of a 2-category: a simplicial set with extra structure.");
                        println!("  3-simplices = commutative tetrahedra (coherence).");
                    }
                    _ => {
                        println!("  Nerve of a {n}-category: (2n-1)-simplices capture all coherences.");
                        println!("  As n→∞: nerve of an ∞-category = a quasi-category.");
                    }
                }
                println!();
                println!("  In simplicial HoTT: Segal types are the nerve-like structures.");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Simplicial HoTT — Interactive Sandbox                  ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore Segal types — the (∞,1)-categorical extension of HoTT.");
    println!("  Type {} to see the Segal condition, {} for the Yoneda lemma.\n",
        cyan("segal-cond"), cyan("yoneda"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}simp-HoTT{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
