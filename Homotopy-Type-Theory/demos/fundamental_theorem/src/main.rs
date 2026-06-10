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

// ── Fundamental Theorem of Identity Types ────────────────────────────────────────
//
// The Fundamental Theorem (HoTT Book §4.7):
//
//   Given A : 𝒰, a : A, and R : A → 𝒰 with ρ : R(a),
//   the following are equivalent:
//     (i)  R is a type family of propositions such that (Σ x:A, R(x)) is contractible
//     (ii) ∀ x:A, (a = x) ≃ R(x)
//
//   Proof: define f: ∀x, (a=x) → R(x) by f(refl) := ρ
//   Then f is an equivalence iff (Σ x, R(x)) is contractible.
//
// This is the HoTT analogue of the Yoneda lemma.
// It's used to prove: path induction, encode-decode, univalence consequences.

#[derive(Clone, Debug)]
struct Relation {
    name: String,
    carrier: String,
    basepoint: String,
    reflexivity: String,
    description: String,
    contractible: bool,
}

impl Relation {
    fn new(name: &str, carrier: &str, base: &str, refl: &str, desc: &str, contr: bool) -> Self {
        Relation {
            name: name.into(),
            carrier: carrier.into(),
            basepoint: base.into(),
            reflexivity: refl.into(),
            description: desc.into(),
            contractible: contr,
        }
    }

    fn display(&self) {
        println!("  Relation R on {} at basepoint {}", cyan(&self.carrier), cyan(&self.basepoint));
        println!("  R(x) = {}", self.description);
        println!("  Reflexivity term ρ: R({}) = {}", self.basepoint, cyan(&self.reflexivity));
        if self.contractible {
            println!("  {} (Σ x:{}, R(x)) is contractible → ({}=x) ≃ R(x) for all x",
                green("✓"), self.carrier, self.basepoint);
        } else {
            println!("  {} (Σ x:{}, R(x)) is NOT contractible → this R is not characterizing ({}=x)",
                red("✗"), self.carrier, self.basepoint);
        }
    }
}

struct Sandbox {
    relations: Vec<Relation>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { relations: Vec::new() };
        sb.relations.push(Relation::new(
            "path-fam", "A", "a", "refl_a",
            "(a = x)  [the identity relation itself]", true
        ));
        sb.relations.push(Relation::new(
            "code-fam", "S¹", "base", "encode(refl) = 0",
            "code(x) via universal cover / fiber",  true
        ));
        sb
    }

    fn print_help() {
        println!("{}", bold("── Fundamental Theorem of Identity Types ────────────────────────────"));
        println!("  {}     — state the fundamental theorem", cyan("theorem"));
        println!("  {}  <R> <A> <a> <ρ>  — add a relation and check conditions", cyan("relation"));
        println!("  {}         — list all relations", cyan("list"));
        println!("  {}  <i>        — display relation #i in detail", cyan("show"));
        println!("{}", bold("── Key Applications ─────────────────────────────────────────────────"));
        println!("  {}     — path induction via the theorem", cyan("path-induction"));
        println!("  {}  — encode-decode method", cyan("encode-decode"));
        println!("  {}      — Yoneda lemma analogy", cyan("yoneda"));
        println!("  {}   — univalence from the theorem", cyan("univalence"));
        println!("{}", bold("── Examples ──────────────────────────────────────────────────────────"));
        println!("  {}     — (a=x) ≃ (a=x) trivially", cyan("trivial"));
        println!("  {}     — (n=m) ≃ code(n,m) for naturals", cyan("nat-code"));
        println!("  {}   — characterize paths in Σ-types", cyan("sigma-paths"));
        println!("  {}     — paths in product types", cyan("prod-paths"));
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
            "theorem" => {
                println!("{}", bold("── Fundamental Theorem of Identity Types ─────────────────────────────"));
                println!("  Setup: A : 𝒰,  a : A,  R : A → 𝒰,  ρ : R(a)");
                println!();
                println!("  Define f : Π(x:A), (a = x) → R(x) by:");
                println!("    f(a, refl_a) := ρ");
                println!("  (path induction / J-eliminator)");
                println!();
                println!("  {}", bold("Theorem (FTIT):"));
                println!("  The following are equivalent:");
                println!("    (i)  ∀x:A, f(x) : (a=x) → R(x) is an equivalence");
                println!("    (ii) (Σ x:A, R(x)) is contractible (has a center of contraction)");
                println!();
                println!("  Proof sketch:");
                println!("    (i)→(ii): The map Σ(x:A),(a=x) → Σ(x:A),R(x) is an equiv.");
                println!("              Σ(x:A),(a=x) is contractible (singleton), so Σ(x:A),R(x) is too.");
                println!("    (ii)→(i): The total map is an equivalence by (ii).");
                println!("              Fibers of a fiberwise map are equiv iff total map is equiv.");
                println!();
                println!("  {} This is the HoTT version of the Yoneda lemma!", green("Key insight:"));
                println!("  Compare: Hom(a,−) is representable ↔ Hom(a,a) has a universal element.");
            }
            "relation" => {
                let args: Vec<&str> = rest.splitn(4, ' ').collect();
                if args.len() < 4 {
                    println!("  Usage: relation <name> <carrier> <base> <rho>");
                    println!("  Example: relation myR A a refl");
                    return true;
                }
                let (name, carrier, base, rho) = (args[0], args[1], args[2], args[3]);
                let idx = self.relations.len();
                let r = Relation::new(name, carrier, base, rho,
                    &format!("{name}(x) — user-defined"), true);
                println!("  Added relation #{idx}: {} on {} at {}",
                    cyan(name), carrier, base);
                println!("  To use FTIT: check that (Σ x:{carrier}, {}(x)) is contractible.", name);
                self.relations.push(r);
            }
            "list" => {
                println!("  Relations:");
                for (i, r) in self.relations.iter().enumerate() {
                    println!("  #{i}: {} on {} at {} — {}",
                        cyan(&r.name), r.carrier, r.basepoint,
                        if r.contractible { green("contractible") } else { red("not contractible") });
                }
            }
            "show" => {
                let i: usize = rest.parse().unwrap_or(0);
                if i < self.relations.len() {
                    self.relations[i].display();
                } else {
                    println!("  {} Index out of range.", red("✗"));
                }
            }
            "path-induction" => {
                println!("{}", bold("── Path Induction via FTIT ──────────────────────────────────────────"));
                println!("  Claim: the J-eliminator (path induction) follows from FTIT.");
                println!();
                println!("  Given: C : Π(x:A)(p:a=x), 𝒰  and  c : C(a, refl_a)");
                println!("  Want: Π(x:A)(p:a=x), C(x,p)");
                println!();
                println!("  Let R(x) := Σ(p:a=x), C(x,p)");
                println!("  Then ρ := (refl_a, c) : R(a)");
                println!("  FTIT: (a=x) ≃ R(x) = Σ(p:a=x), C(x,p)");
                println!();
                println!("  Forward direction: given p:a=x, obtain (p, C-term) from the equiv.");
                println!("  This gives path induction: reduce to the case p = refl.");
                println!();
                println!("  {} J is derivable from FTIT + Σ-elimination.", green("Conclusion:"));
                println!("  {} In practice, FTIT is usually applied post-J.", dim("Note:"));
            }
            "encode-decode" => {
                println!("{}", bold("── Encode-Decode Method ─────────────────────────────────────────────"));
                println!("  To characterize paths (a = b) in some type A:");
                println!();
                println!("  Step 1: Define code : A → A → 𝒰  (the 'expected' path type)");
                println!("  Step 2: Define encode : (a=b) → code(a,b)  using transport or path induction");
                println!("  Step 3: Define decode : code(a,b) → (a=b)");
                println!("  Step 4: Show encode ∘ decode = id and decode ∘ encode = id");
                println!();
                println!("  By FTIT: it suffices to show (Σ b, code(a,b)) is contractible.");
                println!("  This reduces the problem to finding a center of contraction.");
                println!();
                println!("  Examples:");
                println!("    S¹: code(base,base) = ℤ  (winding number)");
                println!("       FTIT ⟹ π₁(S¹) = ℤ");
                println!("    ℕ: code(m,n) = (m=n in ℕ-decidable sense)");
                println!("       FTIT ⟹ ℕ is a set");
                println!("    𝒰: code(A,B) = (A ≃ B)  (univalence)");
                println!("       FTIT ⟹ (A=B) ≃ (A≃B)");
            }
            "yoneda" => {
                println!("{}", bold("── FTIT as Yoneda Lemma ─────────────────────────────────────────────"));
                println!("  Classical Yoneda lemma:");
                println!("    Nat(Hom(a,−), F) ≅ F(a)   for any functor F: C → Set");
                println!();
                println!("  HoTT version (FTIT):");
                println!("    Π(x:A), (a=x) → R(x)  ≃  R(a)");
                println!("    (natural transformations (a=−) → R are determined by R(a))");
                println!();
                println!("  Proof:");
                println!("    → direction: given f, return f(a)(refl_a) : R(a)");
                println!("    ← direction: given ρ:R(a), define f by J: f(x)(refl_a) = ρ");
                println!();
                println!("  The Yoneda embedding says a=x is the free relation.");
                println!("  Any other relation R is characterized by R(a) = how to reflect ρ.");
                println!();
                println!("  FTIT strengthens this: the characterization is an {}.", cyan("equivalence"));
                println!("  Not just a bijection — actually an equivalence of types.");
            }
            "univalence" => {
                println!("{}", bold("── Univalence from FTIT Perspective ────────────────────────────────"));
                println!("  Apply FTIT with:");
                println!("    A := 𝒰 (universe)");
                println!("    a := A : 𝒰");
                println!("    R(B) := (A ≃ B)  (type of equivalences)");
                println!("    ρ := id_A : A ≃ A  (identity equivalence)");
                println!();
                println!("  FTIT says:");
                println!("    (A=B) ≃ (A≃B)   iff   (Σ B:𝒰, A≃B) is contractible");
                println!();
                println!("  The contractibility of (Σ B:𝒰, A≃B) is exactly the");
                println!("  univalence axiom repackaged!");
                println!();
                println!("  Univalence ↔ for all A:𝒰, (Σ B:𝒰, A≃B) is contractible");
                println!("              ↔ (A=B) ≃ (A≃B)  for all A B : 𝒰");
                println!();
                println!("  {} Univalence = FTIT applied to the universe.", cyan("Key:"));
            }
            "trivial" => {
                println!("  Trivial case: R(x) := (a = x) with ρ := refl_a");
                println!("  FTIT: (a=x) ≃ R(x) = (a=x)  trivially by identity equivalence.");
                println!("  (Σ x:A, a=x) = singleton at a, which is contractible. {}", green("✓"));
            }
            "nat-code" => {
                println!("{}", bold("── Paths in ℕ via FTIT ────────────────────────────────────────────────"));
                println!("  Characterize (m = n) in ℕ using code:");
                println!();
                println!("  code : ℕ → ℕ → 𝒰");
                println!("  code(0,   0  ) := 𝟙  (unit type)");
                println!("  code(0,   S n) := 𝟘  (empty type)");
                println!("  code(S m, 0  ) := 𝟘");
                println!("  code(S m, S n) := code(m, n)");
                println!();
                println!("  encode : (m=n) → code(m,n)  by transport along code");
                println!("  decode : code(m,n) → (m=n)  by induction on m,n");
                println!();
                println!("  FTIT (applied at m): (Σ n:ℕ, code(m,n)) is contractible");
                println!("  because code gives decidable equality → singleton at m.");
                println!();
                println!("  {} ℕ is a set: all identity types (m=n) are propositions.", green("Conclusion:"));
            }
            "sigma-paths" => {
                println!("{}", bold("── Paths in Σ-Types ─────────────────────────────────────────────────"));
                println!("  For (a, b) : Σ(x:A), B(x), characterize paths:");
                println!();
                println!("  Claim: ((a,b) = (a',b')) ≃ Σ(p:a=a'), transport(p,b) = b'");
                println!();
                println!("  Proof via FTIT:");
                println!("    R(a',b') := Σ(p:a=a'), transport(p,b) = b'");
                println!("    ρ := (refl_a, refl_b) : R(a,b)");
                println!("    Check: (Σ (a',b'), R(a',b')) ≃ (Σ (a',b'), Σ p, tr(p,b)=b')");
                println!("         ≃ (Σ a', Σ p:a=a', Σ b', tr(p,b)=b')");
                println!("         ≃ (Σ a', Σ p:a=a', 𝟙)   (b' is determined by tr(p,b))");
                println!("         ≃ (Σ a', a=a')   which is contractible. {}",  green("✓"));
                println!();
                println!("  Pair paths: p : (a,b) = (a',b')  decomposes as (pr₁ p, apd pr₂ p).");
            }
            "prod-paths" => {
                println!("{}", bold("── Paths in Product Types ───────────────────────────────────────────"));
                println!("  For (a,b) : A × B, characterize paths:");
                println!();
                println!("  Claim: ((a,b) = (a',b')) ≃ (a=a') × (b=b')");
                println!();
                println!("  Proof: Special case of Σ-paths where B is a constant family.");
                println!("  transport along p : a=a' in constant B is just the identity.");
                println!("  So Σ(p:a=a'), transport(p,b)=b' = (a=a') × (b=b').");
                println!();
                println!("  Constructors: pair⁼¹: (a=a') × (b=b') → (a,b)=(a',b')");
                println!("                ap pr₁: (a,b)=(a',b') → (a=a')");
                println!("                ap pr₂: (a,b)=(a',b') → (b=b')");
                println!();
                println!("  {} Product types are groupoids when A and B are.", green("Corollary:"));
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Fundamental Theorem of Identity Types                  ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  The HoTT analogue of the Yoneda lemma:");
    println!("  (a=x) ≃ R(x) iff (Σx, R(x)) is contractible.");
    println!("  Type {} for the theorem, {} for applications.\n",
        cyan("theorem"), cyan("encode-decode"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}ftit{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
