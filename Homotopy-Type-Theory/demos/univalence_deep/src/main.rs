use std::collections::HashMap;
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

// ── Univalence Axiom (deep dive) ──────────────────────────────────────────────
//
// ua    : (A ≃ B) → (A = B)
// idtoeqv : (A = B) → (A ≃ B)
//
// These are inverse: idtoeqv(ua(e)) = e  and  ua(idtoeqv(p)) = p
//
// Consequence: Aut(Bool) = (Bool = Bool) in 𝒰  has exactly 2 elements
// (the two automorphisms: identity and swap).
//
// We demonstrate this on finite types.

// ── Finite type equivalences ──────────────────────────────────────────────────

#[derive(Clone, Debug)]
struct FinEquiv {
    name: String,
    dom: Vec<String>,
    cod: Vec<String>,
    fwd: Vec<usize>,  // fwd[i] = index in cod of f(dom[i])
    bwd: Vec<usize>,  // bwd[j] = index in dom of g(cod[j])
}

impl FinEquiv {
    fn new_id(elems: Vec<String>) -> Self {
        let n = elems.len();
        FinEquiv { name: "id".into(), dom: elems.clone(), cod: elems, fwd: (0..n).collect(), bwd: (0..n).collect() }
    }

    fn new_swap(a: &str, b: &str) -> Self {
        let elems = vec![a.to_string(), b.to_string()];
        FinEquiv { name: "swap".into(), dom: elems.clone(), cod: elems, fwd: vec![1, 0], bwd: vec![1, 0] }
    }

    fn is_valid(&self) -> bool {
        let n = self.dom.len();
        if n != self.cod.len() || n != self.fwd.len() || n != self.bwd.len() { return false; }
        // Check η: bwd[fwd[i]] = i
        for i in 0..n { if self.bwd.get(self.fwd[i]) != Some(&i) { return false; } }
        // Check ε: fwd[bwd[j]] = j
        for j in 0..n { if self.fwd.get(self.bwd[j]) != Some(&j) { return false; } }
        true
    }

    fn apply_fwd(&self, x: &str) -> Option<&str> {
        self.dom.iter().position(|e| e == x).and_then(|i| self.fwd.get(i)).and_then(|&j| self.cod.get(j)).map(|s| s.as_str())
    }

    fn apply_bwd(&self, y: &str) -> Option<&str> {
        self.cod.iter().position(|e| e == y).and_then(|j| self.bwd.get(j)).and_then(|&i| self.dom.get(i)).map(|s| s.as_str())
    }

    fn compose(&self, other: &FinEquiv) -> Option<FinEquiv> {
        if self.cod != other.dom { return None; }
        let fwd: Vec<usize> = self.fwd.iter().map(|&i| *other.fwd.get(i).unwrap_or(&i)).collect();
        let bwd: Vec<usize> = other.bwd.iter().map(|&j| *self.bwd.get(j).unwrap_or(&j)).collect();
        Some(FinEquiv { name: format!("{}∘{}", other.name, self.name), dom: self.dom.clone(), cod: other.cod.clone(), fwd, bwd })
    }

    fn inverse(&self) -> FinEquiv {
        FinEquiv { name: format!("{}⁻¹", self.name), dom: self.cod.clone(), cod: self.dom.clone(), fwd: self.bwd.clone(), bwd: self.fwd.clone() }
    }

    fn display(&self) {
        println!("  {} : {} ≃ {}", bold(&cyan(&self.name)), "{".to_string() + &self.dom.join(",") + "}", "{".to_string() + &self.cod.join(",") + "}");
        for (i, x) in self.dom.iter().enumerate() {
            let y = self.fwd.get(i).and_then(|&j| self.cod.get(j)).cloned().unwrap_or("?".into());
            println!("    {} → {}", x, cyan(&y));
        }
    }

    fn eq_to_equiv(elems: &[String]) -> Vec<FinEquiv> {
        // idtoeqv: paths in 𝒰 correspond to automorphisms
        // For a finite set with n elements, paths = all bijections = Sₙ
        let n = elems.len();
        let mut perms = vec![];
        let mut all_perms: Vec<Vec<usize>> = vec![];
        gen_perms(&(0..n).collect::<Vec<_>>(), &mut vec![], &mut all_perms);
        for (k, perm) in all_perms.iter().enumerate() {
            let fwd = perm.clone();
            let mut bwd = vec![0; n];
            for (i, &j) in fwd.iter().enumerate() { bwd[j] = i; }
            perms.push(FinEquiv { name: format!("σ{k}"), dom: elems.to_vec(), cod: elems.to_vec(), fwd, bwd });
        }
        perms
    }
}

fn gen_perms(remaining: &[usize], current: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if remaining.is_empty() { out.push(current.clone()); return; }
    for i in 0..remaining.len() {
        let mut r2 = remaining.to_vec(); let x = r2.remove(i);
        current.push(x);
        gen_perms(&r2, current, out);
        current.pop();
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    equivs: HashMap<String, FinEquiv>,
    types: HashMap<String, Vec<String>>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { equivs: HashMap::new(), types: HashMap::new() };
        // Pre-load Bool with its two automorphisms
        let bool_elems = vec!["false".into(), "true".into()];
        sb.types.insert("Bool".into(), bool_elems.clone());
        sb.equivs.insert("id_Bool".into(), FinEquiv::new_id(bool_elems.clone()));
        sb.equivs.insert("swap_Bool".into(), FinEquiv::new_swap("false", "true"));
        sb
    }

    fn print_help() {
        println!("{}", bold("── Types and Equivalences ──────────────────────────────────────────"));
        println!("  {}  A a b c         — define a finite type", cyan("type"));
        println!("  {}  e A             — show all automorphisms of A", cyan("aut"));
        println!("  {}  e A B fwd bwd   — define an equivalence", cyan("equiv"));
        println!("  {}  e               — check and display equivalence", cyan("show"));
        println!("  {}  e1 e2           — compose equivalences", cyan("comp"));
        println!("  {}  e               — take inverse", cyan("inv"));
        println!("{}", bold("── Univalence ──────────────────────────────────────────────────────"));
        println!("  {}  e               — apply ua (equiv → path)", cyan("ua"));
        println!("  {}  A               — paths in 𝒰 at A = automorphisms", cyan("paths"));
        println!("  {}  A               — Aut(A) = A's automorphism group", cyan("AutA"));
        println!("{}", bold("── Key Facts ────────────────────────────────────────────────────────"));
        println!("  {}         — Aut(Bool) = ℤ/2ℤ", cyan("aut-bool"));
        println!("  {}         — idtoeqv and ua explained", cyan("idtoeqv"));
        println!("  {}         — transport along ua(e)", cyan("transport"));
        println!("  {}         — full statement of univalence axiom", cyan("axiom"));
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
            "type" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.is_empty() { println!("  {} Use: type Name e1 e2 ...", red("✗")); return true; }
                let name = ws[0];
                let elems: Vec<String> = ws[1..].iter().map(|s| s.to_string()).collect();
                println!("  {} = {{{}}} (|·| = {})", cyan(name), elems.join(", "), elems.len());
                self.types.insert(name.to_string(), elems);
            }
            "aut" => {
                let ty = self.types.get(rest).cloned();
                match ty {
                    Some(elems) => {
                        let auts = FinEquiv::eq_to_equiv(&elems);
                        println!("  {} Aut({}) = all automorphisms ({} total = {}!)", bold("Computing"), cyan(rest), auts.len(), elems.len());
                        for a in &auts {
                            let map: Vec<String> = elems.iter().zip(a.fwd.iter()).map(|(x, &j)| format!("{x}↦{}", elems[j])).collect();
                            println!("  {} : [{}]", cyan(&a.name), map.join(", "));
                        }
                        println!();
                        println!("  By univalence: Aut({rest}) = ({{{}}} = {{{}}} in 𝒰)", elems.join(","), elems.join(","));
                    }
                    None => println!("  {} Type {rest} not found", red("✗")),
                }
            }
            "equiv" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 5 { println!("  {} Use: equiv name A B fwd_idx bwd_idx", red("✗")); return true; }
                let (name, a_name, b_name) = (ws[0], ws[1], ws[2]);
                let ta = self.types.get(a_name).cloned();
                let tb = self.types.get(b_name).cloned();
                match (ta, tb) {
                    (Some(a), Some(b)) => {
                        let fwd: Vec<usize> = ws[3].split(',').filter_map(|s| s.parse().ok()).collect();
                        let bwd: Vec<usize> = ws[4].split(',').filter_map(|s| s.parse().ok()).collect();
                        let e = FinEquiv { name: name.to_string(), dom: a, cod: b, fwd, bwd };
                        if e.is_valid() { println!("  {} Valid equivalence {} : {} ≃ {}", green("✓"), cyan(name), a_name, b_name); }
                        else { println!("  {} Not a valid equivalence (check fwd/bwd indices)", red("✗")); }
                        self.equivs.insert(name.to_string(), e);
                    }
                    _ => println!("  {} Type(s) not found", red("✗")),
                }
            }
            "show" => {
                if let Some(e) = self.equivs.get(rest) {
                    let e = e.clone();
                    e.display();
                    if e.is_valid() { println!("  {} Valid equivalence", green("✓")); }
                    else { println!("  {} Invalid — η or ε fails", red("✗")); }
                } else { println!("  {} Equiv {rest} not found", red("✗")); }
            }
            "comp" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: comp e1 e2", red("✗")); return true; }
                let e1 = self.equivs.get(ws[0]).cloned();
                let e2 = self.equivs.get(ws[1]).cloned();
                match (e1, e2) {
                    (Some(e1), Some(e2)) => match e1.compose(&e2) {
                        Some(c) => { c.display(); self.equivs.insert(c.name.clone(), c); }
                        None => println!("  {} Codomain of {} ≠ domain of {}", red("✗"), ws[0], ws[1]),
                    },
                    _ => println!("  {} Equiv(s) not found", red("✗")),
                }
            }
            "inv" => {
                if let Some(e) = self.equivs.get(rest).cloned() {
                    let inv = e.inverse();
                    inv.display();
                    self.equivs.insert(inv.name.clone(), inv);
                } else { println!("  {} Equiv {rest} not found", red("✗")); }
            }
            "ua" => {
                if let Some(e) = self.equivs.get(rest) {
                    let e = e.clone();
                    println!("  ua({}) : {} = {} in 𝒰", cyan(&e.name), "{".to_string()+&e.dom.join(",")+"}",  "{".to_string()+&e.cod.join(",")+"}" );
                    println!("  A path in the universe between these two types.");
                    println!("  transp(ua({}), a) = {}(a)   [computes!]", e.name, e.name);
                } else { println!("  {} Equiv {rest} not found", red("✗")); }
            }
            "paths" => {
                let ty = self.types.get(rest).cloned();
                match ty {
                    Some(elems) => {
                        let auts = FinEquiv::eq_to_equiv(&elems);
                        println!("  Paths({{{}}}) in 𝒰 ≃ Aut({{{}}}) ≃ S_{}", elems.join(","), elems.join(","), elems.len());
                        println!("  = {} equivalences = {}! elements", auts.len(), elems.len());
                        println!("  Each path is idtoeqv(p) = some automorphism via ua⁻¹");
                    }
                    None => println!("  {} Type {rest} not found", red("✗")),
                }
            }
            "AutA" => {
                let ty = self.types.get(rest).cloned();
                match ty {
                    Some(elems) => {
                        let n = elems.len();
                        let fact: usize = (1..=n).product();
                        println!("  Aut({}) = S_{n}  (symmetric group)", cyan(rest));
                        println!("  |Aut({rest})| = {n}! = {fact}");
                        println!("  By univalence: Aut(A) = (A = A)  in 𝒰");
                        if rest == "Bool" || n == 2 { println!("  Aut(Bool) = ℤ/2ℤ  (only id and swap)"); }
                    }
                    None => println!("  {} Type {rest} not found", red("✗")),
                }
            }
            "aut-bool" => {
                println!("{}", bold("── Aut(Bool) = ℤ/2ℤ ────────────────────────────────────────────────"));
                println!("  Bool has exactly 2 automorphisms:");
                println!("  σ₀ = id   : false↦false, true↦true");
                println!("  σ₁ = swap : false↦true,  true↦false");
                println!();
                println!("  By univalence: Aut(Bool) = (Bool = Bool) in 𝒰₀");
                println!("  So the type (Bool = Bool) has exactly 2 elements!");
                println!();
                println!("  This means: transport along the nontrivial path (ua(swap))");
                println!("  sends 'true' to 'false' and vice versa.");
                println!();
                self.equivs["id_Bool"].clone().display();
                println!();
                self.equivs["swap_Bool"].clone().display();
                println!();
                println!("  comp: swap ∘ swap = id  (the group ℤ/2ℤ)");
            }
            "idtoeqv" => {
                println!("{}", bold("── idtoeqv : (A = B) → (A ≃ B) ────────────────────────────────────"));
                println!("  Given a path p : A = B in the universe 𝒰:");
                println!("  idtoeqv(p) := transport^id(p, _)  =  (transport^id(p), ...)");
                println!();
                println!("  For p = refl: idtoeqv(refl) = id_{{A}}");
                println!("  For p = ua(e): idtoeqv(ua(e)) = e  (by definition of ua)");
                println!();
                println!("  The univalence axiom says: idtoeqv is an EQUIVALENCE");
                println!("  i.e., ua is its quasi-inverse: ua(idtoeqv(p)) = p");
                println!();
                println!("  This is the key: 'isomorphic types are equal' is not just a slogan,");
                println!("  it's the statement that idtoeqv has an inverse.");
            }
            "transport" => {
                println!("{}", bold("── Transport along ua(e) ─────────────────────────────────────────────"));
                println!("  For e : A ≃ B with forward map f : A → B:");
                println!("  transport^P(ua(e), u) = ?");
                println!();
                println!("  Key theorem: transport^id(ua(e), a) = e.fwd(a)");
                println!("  (transport in the identity family = applying the equivalence)");
                println!();
                println!("  Example with Bool and swap:");
                println!("  transport^id(ua(swap), true) = swap(true) = false");
                println!("  transport^id(ua(swap), false) = swap(false) = true");
                println!();
                let swap = self.equivs["swap_Bool"].clone();
                for x in ["true", "false"] {
                    if let Some(y) = swap.apply_fwd(x) { println!("  transport(ua(swap), {x}) = {}", cyan(y)); }
                }
            }
            "axiom" => {
                println!("{}", bold("── Univalence Axiom (Voevodsky, 2006) ───────────────────────────────"));
                println!("  For all types A B : 𝒰:");
                println!("  univalence : isEquiv(idtoeqv_{{A,B}})");
                println!();
                println!("  Equivalently: (A = B) ≃ (A ≃ B)");
                println!("  Equivalently: ua : (A ≃ B) → (A = B)  and  idtoeqv ∘ ua = id,  ua ∘ idtoeqv = id");
                println!();
                println!("  Consequences:");
                println!("  1. Isomorphic structures are equal (structures are defined by properties, not representation)");
                println!("  2. All functions preserve equivalences (no 'evil' functions)");
                println!("  3. Path induction on equalities between types gives equivalences");
                println!("  4. Aut(A) ≃ (A = A)  — automorphisms are paths");
                println!("  5. funext follows from univalence");
                println!();
                println!("  Status:");
                println!("  • Not provable in plain MLTT (requires an axiom)");
                println!("  • Consistent with MLTT (Voevodsky's model in simplicial sets)");
                println!("  • Provable and computable in Cubical HoTT (CCHM, 2017)");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Univalence (Deep) — idtoeqv, ua, Aut(Bool) Sandbox   ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore univalence: the principle that equivalent types are equal.");
    println!("  See how Aut(Bool) = ℤ/2ℤ and how transport along ua computes.");
    println!("  Type {} for the axiom, {} for Aut(Bool), {} for commands.\n", cyan("axiom"), cyan("aut-bool"), cyan("help"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}ua{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
