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

// ── Categorical Logic ─────────────────────────────────────────────────────────
//
// Correspondence between logic and category theory:
//   propositions ↔ objects
//   proofs       ↔ morphisms
//   conjunction  ↔ products
//   disjunction  ↔ coproducts
//   implication  ↔ exponential objects
//   truth        ↔ terminal object
//   falsity      ↔ initial object
//   quantifiers  ↔ adjoints to substitution functors
//
// For HoTT: the ambient category is an (∞,1)-topos

#[derive(Clone, Debug, PartialEq)]
enum Prop {
    Var(String),
    True,
    False,
    And(Box<Prop>, Box<Prop>),
    Or(Box<Prop>, Box<Prop>),
    Implies(Box<Prop>, Box<Prop>),
    Not(Box<Prop>),
    Forall(String, Box<Prop>),
    Exists(String, Box<Prop>),
}

impl Prop {
    fn cat_interp(&self) -> String {
        match self {
            Prop::True => "terminal object 1".into(),
            Prop::False => "initial object 0".into(),
            Prop::Var(x) => format!("subobject {x} ↪ Ω"),
            Prop::And(p, q) => format!("({}) ×_Ω ({})", p.cat_interp(), q.cat_interp()),
            Prop::Or(p, q) => format!("({}) +_Ω ({})", p.cat_interp(), q.cat_interp()),
            Prop::Implies(p, q) => format!("({}) ⟹ ({})", p.cat_interp(), q.cat_interp()),
            Prop::Not(p) => format!("({}) ⟹ 0", p.cat_interp()),
            Prop::Forall(x, p) => format!("Π_{x} ({})", p.cat_interp()),
            Prop::Exists(x, p) => format!("Σ_{x} ({})", p.cat_interp()),
        }
    }

    fn hott_interp(&self) -> String {
        match self {
            Prop::True => "𝟙 (unit type)".into(),
            Prop::False => "𝟘 (empty type)".into(),
            Prop::Var(x) => format!("{x} : 𝒰 (a type / proposition)"),
            Prop::And(p, q) => format!("{} × {}", p.hott_interp(), q.hott_interp()),
            Prop::Or(p, q) => format!("{} + {}", p.hott_interp(), q.hott_interp()),
            Prop::Implies(p, q) => format!("{} → {}", p.hott_interp(), q.hott_interp()),
            Prop::Not(p) => format!("{} → 𝟘", p.hott_interp()),
            Prop::Forall(x, p) => format!("Π ({x}:A), {}", p.hott_interp()),
            Prop::Exists(x, p) => format!("Σ ({x}:A), {}", p.hott_interp()),
        }
    }

    fn parse(s: &str) -> Option<Prop> {
        let s = s.trim();
        if s == "T" || s == "True" || s == "⊤" { return Some(Prop::True); }
        if s == "F" || s == "False" || s == "⊥" { return Some(Prop::False); }
        if s.starts_with("forall ") {
            let rest = &s[7..];
            if let Some(dot) = rest.find('.') {
                let var = rest[..dot].trim().to_string();
                let body = Prop::parse(&rest[dot+1..])?;
                return Some(Prop::Forall(var, Box::new(body)));
            }
        }
        if s.starts_with("exists ") {
            let rest = &s[7..];
            if let Some(dot) = rest.find('.') {
                let var = rest[..dot].trim().to_string();
                let body = Prop::parse(&rest[dot+1..])?;
                return Some(Prop::Exists(var, Box::new(body)));
            }
        }
        if s.contains("->") || s.contains("→") {
            let sep = if s.contains("->") { "->" } else { "→" };
            if let Some(i) = s.rfind(sep) {
                let l = Prop::parse(&s[..i])?;
                let r = Prop::parse(&s[i+sep.len()..])?;
                return Some(Prop::Implies(Box::new(l), Box::new(r)));
            }
        }
        if s.contains("/\\") || s.contains("∧") || s.contains(" and ") {
            let sep = if s.contains("/\\") { "/\\" } else if s.contains("∧") { "∧" } else { " and " };
            if let Some(i) = s.rfind(sep) {
                let l = Prop::parse(&s[..i])?;
                let r = Prop::parse(&s[i+sep.len()..])?;
                return Some(Prop::And(Box::new(l), Box::new(r)));
            }
        }
        if s.contains("\\/") || s.contains("∨") || s.contains(" or ") {
            let sep = if s.contains("\\/") { "\\/" } else if s.contains("∨") { "∨" } else { " or " };
            if let Some(i) = s.rfind(sep) {
                let l = Prop::parse(&s[..i])?;
                let r = Prop::parse(&s[i+sep.len()..])?;
                return Some(Prop::Or(Box::new(l), Box::new(r)));
            }
        }
        if s.starts_with("not ") || s.starts_with("¬") {
            let inner = if s.starts_with("not ") { &s[4..] } else { &s[2..] };
            let p = Prop::parse(inner)?;
            return Some(Prop::Not(Box::new(p)));
        }
        if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Some(Prop::Var(s.to_string()));
        }
        None
    }
}

struct Sandbox {
    named: HashMap<String, Prop>,
    adjoint_history: Vec<String>,
}

impl Sandbox {
    fn new() -> Self {
        Sandbox { named: HashMap::new(), adjoint_history: Vec::new() }
    }

    fn print_help() {
        println!("{}", bold("── Categorical Logic Sandbox ────────────────────────────────────────"));
        println!("  {}  <P>        — parse and show categorical interpretation", cyan("interp"));
        println!("  {}  <name> <P> — name a proposition", cyan("def"));
        println!("  {}  <P>        — show HoTT type-theoretic reading", cyan("hott"));
        println!("{}", bold("── Categorical Structures ───────────────────────────────────────────"));
        println!("  {}      — connectives as limits/colimits", cyan("connectives"));
        println!("  {}      — quantifiers as adjoints", cyan("quantifiers"));
        println!("  {}  <n>        — show logic of rank-n topos", cyan("topos"));
        println!("  {}       — subobject classifier Ω", cyan("omega"));
        println!("{}", bold("── Translations ─────────────────────────────────────────────────────"));
        println!("  {}    — Lindenbaum-Tarski algebra", cyan("lindenbaum"));
        println!("  {}      — Mitchell-Bénabou language", cyan("mitchell"));
        println!("  {}      — internal language of a topos", cyan("internal"));
        println!("  {}      — Lawvere-Tierney topology", cyan("topology"));
        println!("  Syntax: P/\\Q or P and Q, P\\/Q or P or Q, P->Q, not P, forall x. P, exists x. P");
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
            "interp" => {
                match Prop::parse(rest) {
                    Some(p) => {
                        println!("  Proposition: {rest}");
                        println!("  Categorical: {}", cyan(&p.cat_interp()));
                        println!("  HoTT type:   {}", cyan(&p.hott_interp()));
                    }
                    None => println!("  {} Could not parse: {rest}", red("✗")),
                }
            }
            "def" => {
                let args: Vec<&str> = rest.splitn(2, ' ').collect();
                if args.len() < 2 { println!("  Usage: def <name> <prop>"); return true; }
                match Prop::parse(args[1]) {
                    Some(p) => {
                        self.named.insert(args[0].into(), p);
                        println!("  Defined {} = {}", cyan(args[0]), args[1]);
                    }
                    None => println!("  {} Parse error: {}", red("✗"), args[1]),
                }
            }
            "hott" => {
                match Prop::parse(rest) {
                    Some(p) => {
                        println!("  Proposition: {rest}");
                        println!("  HoTT type: {}", cyan(&p.hott_interp()));
                        println!("  In HoTT, propositions are {}-truncated types.", dim("-1"));
                        println!("  The proposition {rest} is (the propositional truncation of) this type.");
                    }
                    None => println!("  {} Could not parse: {rest}", red("✗")),
                }
            }
            "connectives" => {
                println!("{}", bold("── Connectives as Limits and Colimits ───────────────────────────────"));
                println!("  In a topos E, for propositions P, Q : Ω:");
                println!();
                println!("  ⊤  = terminal object morphism: 1 → Ω");
                println!("  ⊥  = initial object morphism: 0 → Ω (via ¬⊤)");
                println!("  P∧Q = pullback of P,Q over Ω (product in Sub(X))");
                println!("  P∨Q = image of P+Q → Ω (coproduct, then image)");
                println!("  P→Q = internal hom [P, Q] in Sub(X)");
                println!("  ¬P  = P → ⊥  = [P, 0]");
                println!();
                println!("  In HoTT (propositions = (-1)-types):");
                println!("  ⊤ = 𝟙, ⊥ = 𝟘, P∧Q = P×Q, P∨Q = ‖P+Q‖₋₁");
                println!("  P→Q = P→Q (function type), ¬P = P→𝟘");
                println!();
                println!("  Note: ∨ requires truncation in HoTT to remain propositional.");
            }
            "quantifiers" => {
                println!("{}", bold("── Quantifiers as Adjoints ──────────────────────────────────────────"));
                println!("  Given a map f: X → Y, substitution gives:");
                println!("    f* : Sub(Y) → Sub(X)  (pulling back a predicate)");
                println!();
                println!("  Left adjoint to f*: ∃_f : Sub(X) → Sub(Y)  (existential)");
                println!("    ∃_f(P)(y) = ∃ x, f(x)=y ∧ P(x)");
                println!("    f* ⊣ ... wait:  ∃_f ⊣ f*");
                println!();
                println!("  Right adjoint to f*: ∀_f : Sub(X) → Sub(Y)  (universal)");
                println!("    ∀_f(P)(y) = ∀ x, f(x)=y → P(x)");
                println!("    f* ⊣ ∀_f");
                println!();
                println!("  Beck-Chevalley: quantifiers commute with substitution along pullbacks.");
                println!();
                println!("  In HoTT: Σ(x:A),P(x) = ∃, Π(x:A),P(x) = ∀");
                println!("  Substitution = weakening in context (add a free variable).");
                println!("  Π and Σ are right/left adjoints to the weakening functor.");
            }
            "topos" | "topos-logic" => {
                let n_str = rest.split_whitespace().next().unwrap_or("1");
                let n: usize = n_str.parse().unwrap_or(1);
                match n {
                    0 => {
                        println!("  0-topos (= ordinary topos):");
                        println!("  • Category with finite limits, power objects, subobject classifier Ω");
                        println!("  • Internal logic: intuitionistic higher-order logic (HOL)");
                        println!("  • Example: Set, presheaf categories, Sh(X)");
                    }
                    1 => {
                        println!("  1-topos (= Grothendieck topos):");
                        println!("  • Sheaves on a site");
                        println!("  • Internal logic: intuitionistic first-order logic + HOL");
                        println!("  • Geometric morphisms = maps between topoi");
                    }
                    _ => {
                        println!("  (∞,1)-topos:");
                        println!("  • Homotopy sheaves on an ∞-site");
                        println!("  • Internal logic: HoTT (homotopy type theory)");
                        println!("  • Univalence corresponds to the universe object classifier");
                        println!("  • Example: ∞-groupoids, ∞-sheaves on a space");
                    }
                }
            }
            "omega" => {
                println!("{}", bold("── Subobject Classifier Ω ──────────────────────────────────────────"));
                println!("  In a topos E, Ω is the object of truth values:");
                println!("  • A morphism X → Ω classifies a subobject of X");
                println!("  • Sub(X) ≅ Hom(X, Ω)");
                println!("  • Ω has a global section ⊤: 1 → Ω (the 'true' element)");
                println!();
                println!("  In Set: Ω = {{true, false}} = 2");
                println!("  In Sh(X): Ω(U) = {{open V ⊆ X : V ⊆ U}}  (opens of X)");
                println!("  In sSet: Ω_n = sieves on [n]  (hereditary subsets of face maps)");
                println!();
                println!("  In HoTT: Ω corresponds to Prop = ‖−‖₋₁ (propositional truncation)");
                println!("  The universe 𝒰 is the object classifier (classifies all types, not just props)");
                println!("  Univalence says the map (A ≃ B) → (A = B) is an equivalence.");
            }
            "lindenbaum" => {
                println!("{}", bold("── Lindenbaum-Tarski Algebra ────────────────────────────────────────"));
                println!("  For a propositional logic L, define:");
                println!("    L/≡ := {{formulas}} / {{P ≡ Q when P⊢Q and Q⊢P}}");
                println!();
                println!("  Classical logic: L/≡ is a Boolean algebra");
                println!("  Intuitionistic logic: L/≡ is a Heyting algebra");
                println!("  Linear logic: L/≡ is a *-autonomous category");
                println!();
                println!("  Heyting algebra operations:");
                println!("    a ∧ b = meet, a ∨ b = join, a → b = Heyting implication");
                println!("    ¬a = a → 0");
                println!("    {} ¬¬a ≠ a in general (excluded middle fails)", yellow("Key:"));
                println!();
                println!("  Categorical logic: every Heyting algebra = subobject lattice Sub(X)");
                println!("  in some Heyting category.");
            }
            "mitchell" => {
                println!("{}", bold("── Mitchell-Bénabou Language ────────────────────────────────────────"));
                println!("  Given a topos E, the M-B language has:");
                println!("    • Types: objects A of E");
                println!("    • Terms: morphisms X → A (in context X)");
                println!("    • Formulas: morphisms X → Ω");
                println!();
                println!("  Type formers:");
                println!("    A × B = product,  A^B = exponential,  Ω = prop type");
                println!("    ΩA = power object P(A) = {{subsets of A}}");
                println!();
                println!("  Quantifiers:");
                println!("    ∃x:A.φ(x) = image of φ: X×A → Ω under ∃_π: Sub(X×A) → Sub(X)");
                println!("    ∀x:A.φ(x) = ∀_π applied to φ");
                println!();
                println!("  This gives a full logic internal to any topos.");
                println!("  HoTT is the Mitchell-Bénabou language for (∞,1)-topoi.");
            }
            "internal" => {
                println!("{}", bold("── Internal Language of a Topos ─────────────────────────────────────"));
                println!("  Every (∞,1)-topos E has an internal language:");
                println!("    • A type theory TT(E) such that");
                println!("    • Models of TT(E) = geometric morphisms into E");
                println!();
                println!("  For the ∞-topos of spaces (∞-groupoids):");
                println!("    TT = Book HoTT + univalence");
                println!();
                println!("  For a cohesive (∞,1)-topos (e.g., smooth ∞-groupoids):");
                println!("    TT = Cohesive HoTT (Shulman, Schreiber)");
                println!("    Extra modalities: ♭ (discrete), ♯ (codiscrete), ʃ (shape)");
                println!();
                println!("  Soundness: HoTT is sound for any (∞,1)-topos.");
                println!("  Completeness: The canonical model is the syntactic category of HoTT.");
            }
            "topology" => {
                println!("{}", bold("── Lawvere-Tierney Topology ─────────────────────────────────────────"));
                println!("  A LT-topology on a topos E is a map j: Ω → Ω such that:");
                println!("    j(⊤) = ⊤");
                println!("    j(j(P)) = j(P)  (idempotent)");
                println!("    j(P∧Q) = j(P) ∧ j(Q)  (left exact)");
                println!();
                println!("  This gives a modality on propositions.");
                println!("  Sheaves for j = {{X : j acts trivially on Sub(X)}}");
                println!();
                println!("  In HoTT: a modality (□, η, μ) corresponds to a LT topology:");
                println!("    □P = j(P) for propositions");
                println!("    The sheaf condition = □-separated types");
                println!();
                println!("  Examples:");
                println!("    Double negation ¬¬: sheaves = Boolean-valued presheaves");
                println!("    Propositional truncation ‖−‖₋₁: sheaves = propositions");
                println!("    n-truncation: sheaves = n-types");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Categorical Logic — Interactive Sandbox                ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore the correspondence between logic and category theory.");
    println!("  Try {} or {}",
        cyan("interp P->Q"), cyan("connectives"));
    println!("  Type 'help' for all commands.\n");

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}cat-logic{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
