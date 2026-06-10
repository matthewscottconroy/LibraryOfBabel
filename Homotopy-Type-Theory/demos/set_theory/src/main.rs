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

// ── Set Theory in HoTT ────────────────────────────────────────────────────────
//
// In HoTT, a "set" is a type A with:
//   is_set(A) : Π(x y : A)(p q : x = y), p = q
//   (all identity types are propositions)
//
// HoTT constructs ETCS (Elementary Theory of the Category of Sets)
// and can model ZFC in the cumulative universe.
//
// Aczel's CZF is interpretable in HoTT via W-types + quotients.

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SetElem {
    Atom(String),
    Set(Vec<SetElem>),
}

impl SetElem {
    fn display(&self) -> String {
        match self {
            SetElem::Atom(s) => s.clone(),
            SetElem::Set(elems) => {
                if elems.is_empty() { "∅".into() }
                else {
                    let inner: Vec<String> = elems.iter().map(|e| e.display()).collect();
                    format!("{{{}}}", inner.join(", "))
                }
            }
        }
    }

    fn is_empty_set(&self) -> bool {
        matches!(self, SetElem::Set(v) if v.is_empty())
    }

    fn contains(&self, x: &SetElem) -> bool {
        match self {
            SetElem::Set(v) => v.contains(x),
            _ => false,
        }
    }

    fn size(&self) -> usize {
        match self {
            SetElem::Set(v) => v.len(),
            SetElem::Atom(_) => 1,
        }
    }
}

fn parse_set(s: &str) -> Option<SetElem> {
    let s = s.trim();
    if s == "{}" || s == "empty" || s == "∅" || s == "0" { return Some(SetElem::Set(vec![])); }
    if !s.starts_with('{') {
        return Some(SetElem::Atom(s.into()));
    }
    if !s.ends_with('}') { return None; }
    let inner = &s[1..s.len()-1];
    if inner.trim().is_empty() { return Some(SetElem::Set(vec![])); }
    let mut elems = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    for (i, c) in inner.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => depth -= 1,
            ',' if depth == 0 => {
                if let Some(e) = parse_set(&inner[start..i]) { elems.push(e); }
                start = i + 1;
            }
            _ => {}
        }
    }
    if let Some(e) = parse_set(&inner[start..]) { elems.push(e); }
    elems.sort();
    elems.dedup();
    Some(SetElem::Set(elems))
}

fn union(a: &SetElem, b: &SetElem) -> SetElem {
    match (a, b) {
        (SetElem::Set(va), SetElem::Set(vb)) => {
            let mut v: Vec<SetElem> = va.iter().chain(vb.iter()).cloned().collect();
            v.sort(); v.dedup();
            SetElem::Set(v)
        }
        _ => a.clone(),
    }
}

fn intersection(a: &SetElem, b: &SetElem) -> SetElem {
    match (a, b) {
        (SetElem::Set(va), SetElem::Set(vb)) => {
            let v: Vec<SetElem> = va.iter().filter(|x| vb.contains(x)).cloned().collect();
            SetElem::Set(v)
        }
        _ => SetElem::Set(vec![]),
    }
}

fn difference(a: &SetElem, b: &SetElem) -> SetElem {
    match (a, b) {
        (SetElem::Set(va), SetElem::Set(vb)) => {
            let v: Vec<SetElem> = va.iter().filter(|x| !vb.contains(x)).cloned().collect();
            SetElem::Set(v)
        }
        _ => a.clone(),
    }
}

fn powerset(a: &SetElem) -> SetElem {
    match a {
        SetElem::Set(v) => {
            let n = v.len();
            if n > 4 { return SetElem::Atom(format!("𝒫({}) [too large to enumerate]", a.display())); }
            let mut subs = Vec::new();
            for mask in 0..(1u32 << n) {
                let sub: Vec<SetElem> = v.iter().enumerate()
                    .filter(|(i, _)| mask & (1 << i) != 0)
                    .map(|(_, e)| e.clone())
                    .collect();
                subs.push(SetElem::Set(sub));
            }
            subs.sort();
            SetElem::Set(subs)
        }
        _ => SetElem::Set(vec![]),
    }
}

// Build von Neumann ordinal n
fn ordinal(n: usize) -> SetElem {
    let mut v = Vec::new();
    for i in 0..n { v.push(ordinal(i)); }
    SetElem::Set(v)
}

struct Sandbox {
    named: HashMap<String, SetElem>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { named: HashMap::new() };
        sb.named.insert("empty".into(), SetElem::Set(vec![]));
        sb.named.insert("∅".into(), SetElem::Set(vec![]));
        sb
    }

    fn resolve(&self, s: &str) -> Option<SetElem> {
        if let Some(e) = self.named.get(s) { return Some(e.clone()); }
        parse_set(s)
    }

    fn print_help() {
        println!("{}", bold("── Set Theory in HoTT Sandbox ──────────────────────────────────────"));
        println!("  {}  <name> <set>  — define a set", cyan("let"));
        println!("  {}  <set>       — show a set", cyan("show"));
        println!("  {}  <A> <B>     — compute A ∪ B", cyan("union"));
        println!("  {}  <A> <B>     — compute A ∩ B", cyan("inter"));
        println!("  {}  <A> <B>     — compute A \\ B", cyan("diff"));
        println!("  {}  <A>         — power set 𝒫(A)", cyan("power"));
        println!("  {}  <A> <B>     — check A ⊆ B", cyan("subset"));
        println!("  {}  <n>         — von Neumann ordinal n", cyan("ord"));
        println!("{}", bold("── HoTT Set Theory ─────────────────────────────────────────────────"));
        println!("  {}         — sets = 0-truncated types", cyan("hott-sets"));
        println!("  {}         — axioms of ETCS in HoTT", cyan("etcs"));
        println!("  {}          — ZFC vs HoTT set theory", cyan("zfc"));
        println!("  {}      — cumulative hierarchy in HoTT", cyan("cumulative"));
        println!("  {}      — constructive set theory (CZF)", cyan("czf"));
        println!("  Syntax: {{a,b,c}}, {{}}, atom-name");
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
            "let" => {
                let args: Vec<&str> = rest.splitn(2, ' ').collect();
                if args.len() < 2 { println!("  Usage: let <name> <set>"); return true; }
                match parse_set(args[1]) {
                    Some(s) => {
                        println!("  {} := {}", cyan(args[0]), s.display());
                        self.named.insert(args[0].into(), s);
                    }
                    None => println!("  {} Parse error: {}", red("✗"), args[1]),
                }
            }
            "show" => {
                match self.resolve(rest) {
                    Some(s) => {
                        println!("  {} (|{}| = {})", cyan(&s.display()), s.display(), s.size());
                        if matches!(&s, SetElem::Set(_)) {
                            println!("  h-level: 0 (it's a set in HoTT)");
                        }
                    }
                    None => println!("  {} Unknown: {rest}", red("✗")),
                }
            }
            "union" => {
                let args: Vec<&str> = rest.splitn(2, ' ').collect();
                if args.len() < 2 { println!("  Usage: union <A> <B>"); return true; }
                match (self.resolve(args[0]), self.resolve(args[1])) {
                    (Some(a), Some(b)) => {
                        let r = union(&a, &b);
                        println!("  {} ∪ {} = {}", a.display(), b.display(), cyan(&r.display()));
                    }
                    _ => println!("  {} Could not resolve sets.", red("✗")),
                }
            }
            "inter" => {
                let args: Vec<&str> = rest.splitn(2, ' ').collect();
                if args.len() < 2 { println!("  Usage: inter <A> <B>"); return true; }
                match (self.resolve(args[0]), self.resolve(args[1])) {
                    (Some(a), Some(b)) => {
                        let r = intersection(&a, &b);
                        println!("  {} ∩ {} = {}", a.display(), b.display(), cyan(&r.display()));
                    }
                    _ => println!("  {} Could not resolve sets.", red("✗")),
                }
            }
            "diff" => {
                let args: Vec<&str> = rest.splitn(2, ' ').collect();
                if args.len() < 2 { println!("  Usage: diff <A> <B>"); return true; }
                match (self.resolve(args[0]), self.resolve(args[1])) {
                    (Some(a), Some(b)) => {
                        let r = difference(&a, &b);
                        println!("  {} \\ {} = {}", a.display(), b.display(), cyan(&r.display()));
                    }
                    _ => println!("  {} Could not resolve sets.", red("✗")),
                }
            }
            "power" => {
                match self.resolve(rest) {
                    Some(a) => {
                        let r = powerset(&a);
                        println!("  𝒫({}) = {}", a.display(), cyan(&r.display()));
                        if let SetElem::Set(ref v) = r { println!("  |𝒫({})| = {}", a.display(), v.len()); }
                    }
                    None => println!("  {} Unknown: {rest}", red("✗")),
                }
            }
            "subset" => {
                let args: Vec<&str> = rest.splitn(2, ' ').collect();
                if args.len() < 2 { println!("  Usage: subset <A> <B>"); return true; }
                match (self.resolve(args[0]), self.resolve(args[1])) {
                    (Some(SetElem::Set(va)), Some(ref b)) => {
                        let is_sub = va.iter().all(|x| b.contains(x));
                        if is_sub {
                            println!("  {} ⊆ {}  {}", args[0], args[1], green("✓ Yes"));
                        } else {
                            let not_in: Vec<&SetElem> = va.iter().filter(|x| !b.contains(x)).collect();
                            let strs: Vec<String> = not_in.iter().map(|e| e.display()).collect();
                            println!("  {} ⊄ {}  {} (missing: {})", args[0], args[1], red("✗ No"), strs.join(", "));
                        }
                    }
                    _ => println!("  {} Could not resolve sets.", red("✗")),
                }
            }
            "ord" => {
                let n: usize = rest.parse().unwrap_or(0);
                if n > 5 { println!("  {} n > 5 would be very large. Try n ≤ 5.", yellow("Warning:")); return true; }
                let o = ordinal(n);
                println!("  Von Neumann ordinal {n} = {}", cyan(&o.display()));
                println!("  |{n}| = {n} (it is an n-element set encoding the ordinal)");
                if n > 0 {
                    println!("  {} n = {{0, 1, …, n-1}} in the von Neumann encoding.", dim("Recall:"));
                }
            }
            "hott-sets" => {
                println!("{}", bold("── Sets in HoTT ─────────────────────────────────────────────────────"));
                println!("  A type A is a set (h-level 0) if:");
                println!("    ∀(x y:A)(p q:x=y), p = q");
                println!("  (all paths between any two elements are equal)");
                println!();
                println!("  Equivalently: A is 0-truncated = ‖A‖₀ ≃ A.");
                println!();
                println!("  Examples of sets in HoTT:");
                println!("    ℕ, ℤ, ℚ, Bool, Fin n — all decidable types are sets");
                println!("    A × B if A, B are sets");
                println!("    A → B if B is a set  (function types into sets)");
                println!("    Quotient types (by construction)");
                println!();
                println!("  Not sets (in general):");
                println!("    𝒰 (universe) — not a set, has non-trivial paths (univalence)");
                println!("    S¹, S², … — spheres have non-trivial homotopy");
                println!();
                println!("  The category of sets in HoTT satisfies ETCS.");
            }
            "etcs" => {
                println!("{}", bold("── ETCS in HoTT ─────────────────────────────────────────────────────"));
                println!("  ETCS (Lawvere 1964): axioms for a well-pointed elementary topos.");
                println!();
                println!("  In HoTT, the full subcategory of sets (0-truncated types) satisfies:");
                println!("  1. Finite limits: products A×B, equalizers, terminal 𝟙, initial 𝟘");
                println!("  2. Exponentials: function types A→B");
                println!("  3. Subobject classifier: Prop (propositions = (-1)-types)");
                println!("  4. Natural numbers object: ℕ (with zero, succ, recursion)");
                println!("  5. Axiom of choice: holds (from univalence + propositional resizing?)");
                println!("  6. Well-pointedness: global elements distinguish morphisms");
                println!();
                println!("  HoTT + LEM + propositional resizing is equivalent to ETCS.");
                println!("  Without LEM: intuitionistic ETCS.");
            }
            "zfc" => {
                println!("{}", bold("── ZFC vs HoTT Set Theory ───────────────────────────────────────────"));
                println!("  ZFC is membership-based:  sets are built from ∈");
                println!("  HoTT is structure-based: sets are 0-truncated types");
                println!();
                println!("  Key differences:");
                println!("    ZFC: ∈ is a global binary relation on all sets");
                println!("    HoTT: membership is local — a:A is typed");
                println!();
                println!("    ZFC: sets can contain anything (ill-typed 'paradoxes' prevented by axioms)");
                println!("    HoTT: types are well-formed by construction");
                println!();
                println!("    ZFC: extensionality (sets equal iff same members) is an axiom");
                println!("    HoTT: univalence implies extensionality for sets");
                println!();
                println!("  Interpretation: ZFC can be modeled in HoTT via the cumulative hierarchy");
                println!("  (a W-type encoding of the von Neumann universe).");
                println!();
                println!("  HoTT is strictly stronger in some ways (function extensionality,");
                println!("  univalence) but also more constructive.");
            }
            "cumulative" => {
                println!("{}", bold("── Cumulative Hierarchy in HoTT ─────────────────────────────────────"));
                println!("  Define V : 𝒰 (the cumulative hierarchy) as a quotient inductive type:");
                println!();
                println!("  V : 𝒰");
                println!("  set : (A : 𝒰) → (A → V) → V   [sets indexed by A]");
                println!("  ext : for a:A→V, b:B→V,");
                println!("        (∀x:A, ∃y:B, a(x)=b(y)) → (∀y:B, ∃x:A, a(x)=b(y)) → set(A,a)=set(B,b)");
                println!("        [extensionality as a higher constructor]");
                println!();
                println!("  Membership: x ∈ set(A,a) := ∃i:A, x = a(i)");
                println!("  Subset: x ⊆ y := ∀z, z ∈ x → z ∈ y");
                println!();
                println!("  Theorem (Rijke, Spitters): V in HoTT satisfies IZF (intuitionistic ZF).");
                println!("  With LEM: V satisfies ZFC.");
            }
            "czf" => {
                println!("{}", bold("── Constructive Set Theory (CZF) in HoTT ───────────────────────────"));
                println!("  CZF (Aczel 1978) is set theory without law of excluded middle.");
                println!();
                println!("  CZF axioms interpretable in HoTT:");
                println!("  1. Extensionality: from univalence (for sets)");
                println!("  2. Pairing: {{a,b}} = set(Bool, if_then a else b)");
                println!("  3. Union: ⋃A = set(Σx∈A. |x|, …)");
                println!("  4. Separation: {{x∈A : P(x)}} = set(Σx:A.P(x), …)");
                println!("  5. Replacement: image of a function");
                println!("  6. Infinity: ℕ");
                println!("  7. Regular extension (REA): W-types");
                println!();
                println!("  HoTT is a model of CZF + some extra axioms.");
                println!("  Aczel's interpretation is the canonical connection between");
                println!("  constructive mathematics and type theory.");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Set Theory in HoTT — Interactive Sandbox              ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build sets, explore set operations, and see how HoTT encodes sets.");
    println!("  Sets in HoTT = 0-truncated types with decidable equality.");
    println!("  Try {} or {}\n",
        cyan("let A {a,b,c}"), cyan("hott-sets"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}set{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
