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

// ── Lean 4 and Mathlib ────────────────────────────────────────────────────────
//
// Lean 4: a dependently-typed programming language and theorem prover.
// Uses CIC (Calculus of Inductive Constructions) with universe polymorphism.
//
// HoTT support in Lean 4:
//   • mathlib4 has some HoTT concepts (Groupoid, etc.)
//   • Lean4-HoTT library: a partial port of HoTT in Lean 4
//   • No native cubical support (uses axioms instead)
//
// Key Lean 4 HoTT axioms:
//   funext : (∀ x, f x = g x) → f = g
//   propext : (P ↔ Q) → P = Q
//   Quotient (built-in)

// Small proof state
#[derive(Clone, Debug)]
struct Goal {
    context: Vec<(String, String)>,
    target: String,
}

impl Goal {
    fn new(target: &str) -> Self {
        Goal { context: Vec::new(), target: target.into() }
    }

    fn add_hyp(&mut self, name: &str, ty: &str) {
        self.context.push((name.into(), ty.into()));
    }

    fn display(&self) {
        println!("  {}", dim("Context:"));
        for (n, t) in &self.context {
            println!("    {} : {}", cyan(n), t);
        }
        println!("  {}", dim("─────────────────────────"));
        println!("  ⊢ {}", yellow(&self.target));
    }
}

struct Sandbox {
    goals: Vec<Goal>,
    current_goal: usize,
    history: Vec<String>,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { goals: Vec::new(), current_goal: 0, history: Vec::new() };
        let mut g = Goal::new("True");
        g.add_hyp("A", "Type");
        g.add_hyp("a", "A");
        sb.goals.push(g);
        sb
    }

    fn print_help() {
        println!("{}", bold("── Lean 4 HoTT Sandbox ─────────────────────────────────────────────"));
        println!("  {}  <stmt>     — show Lean 4 syntax for a statement", cyan("lean"));
        println!("  {}  <tactic>   — explain a tactic", cyan("tactic"));
        println!("  {}  <target>   — create a new proof goal", cyan("goal"));
        println!("  {}  <n> <T>    — add a hypothesis to current goal", cyan("hyp"));
        println!("  {}             — show current proof state", cyan("state"));
        println!("{}", bold("── Language Features ────────────────────────────────────────────────"));
        println!("  {}     — type universe system (Sort, Type, Prop)", cyan("universes"));
        println!("  {}     — inductive types in Lean 4", cyan("inductive"));
        println!("  {}     — structure and class system", cyan("structure"));
        println!("  {}    — type classes vs explicit Π-types", cyan("typeclass"));
        println!("{}", bold("── HoTT in Lean 4 ──────────────────────────────────────────────────"));
        println!("  {}       — univalence status in Lean 4", cyan("univalence"));
        println!("  {}        — propext and funext", cyan("axioms"));
        println!("  {}        — Quotient types (built-in)", cyan("quotient"));
        println!("  {}      — mathlib4 HoTT content", cyan("mathlib"));
        println!("  {}    — comparison with Agda and Coq", cyan("comparison"));
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
            "lean" => {
                let stmt = rest;
                match stmt {
                    "funext" | "fun-ext" => {
                        println!("  Lean 4 / Mathlib:");
                        println!("  {}", cyan("theorem funext {f g : α → β} (h : ∀ x, f x = g x) : f = g :="));
                        println!("  {}",  cyan("  funext h  -- built-in axiom"));
                        println!();
                        println!("  Or equivalently using the funext tactic:");
                        println!("  {}",  cyan("  ext x"));
                        println!("  {}",  cyan("  exact h x"));
                    }
                    "propext" => {
                        println!("  {}",  cyan("theorem propext {P Q : Prop} (h : P ↔ Q) : P = Q :="));
                        println!("  {}",  cyan("  propext h  -- built-in axiom in Lean 4"));
                    }
                    "ua" | "univalence" => {
                        println!("  Univalence is NOT built-in to Lean 4.");
                        println!("  In HoTT library for Lean 4:");
                        println!("  {}",  cyan("axiom univalence : (A ≃ B) → (A = B)"));
                        println!("  Adding this axiom makes propext derivable for universe-level Prop.");
                        println!("  {} Lean 4 standard library uses propext, not full univalence.", yellow("Note:"));
                    }
                    "equiv" => {
                        println!("  {}",  cyan("structure Equiv (α β : Sort*) where"));
                        println!("  {}",  cyan("  toFun : α → β"));
                        println!("  {}",  cyan("  invFun : β → α"));
                        println!("  {}",  cyan("  leftInv : ∀ a, invFun (toFun a) = a"));
                        println!("  {}",  cyan("  rightInv : ∀ b, toFun (invFun b) = b"));
                        println!();
                        println!("  Mathlib notation: α ≃ β");
                    }
                    "Eq" | "path" | "=" => {
                        println!("  {}",  cyan("inductive Eq : α → α → Prop where"));
                        println!("  {}",  cyan("  | refl (a : α) : Eq a a"));
                        println!();
                        println!("  Note: Eq lives in Prop (proof-irrelevant in Lean 4!)");
                        println!("  This means all proofs of a = b are equal: subsingleton.");
                        println!("  {} This is K/UIP, NOT HoTT-compatible by default.", yellow("Warning:"));
                        println!("  HoTT libraries avoid K and use --without-K flag.");
                    }
                    _ => {
                        println!("  Available: funext, propext, ua, equiv, Eq/path");
                    }
                }
            }
            "tactic" => {
                match rest {
                    "exact" => println!("  exact t : close goal with term t (t must have the goal's type)"),
                    "apply" => println!("  apply f : apply function f, creating sub-goals for its arguments"),
                    "intro" => println!("  intro x : introduce a hypothesis x (for ∀ or →)"),
                    "rw" | "rewrite" => println!("  rw [h] : rewrite using h : a = b (replace a with b)"),
                    "simp" => println!("  simp : simplify using simp lemmas (may close goal)"),
                    "ring" => println!("  ring : prove equalities in commutative (semi)rings"),
                    "omega" => println!("  omega : linear arithmetic over ℤ/ℕ"),
                    "constructor" => println!("  constructor : split a conjunction or provide structure fields"),
                    "cases" => println!("  cases h : case split on inductive type h"),
                    "induction" => println!("  induction n : induct on n (with induction hypothesis)"),
                    "ext" => println!("  ext : apply extensionality (funext, setext, etc.)"),
                    "decide" => println!("  decide : decide a decidable proposition by computation"),
                    "native_decide" => println!("  native_decide : decide using native code evaluation (fast)"),
                    "aesop" => println!("  aesop : automatic proof search (type-class based)"),
                    _ => println!("  Tactics: exact, apply, intro, rw, simp, ring, omega, cases, induction, ext, decide, aesop"),
                }
            }
            "goal" => {
                let target = if rest.is_empty() { "A = A" } else { rest };
                let g = Goal::new(target);
                self.current_goal = self.goals.len();
                self.goals.push(g);
                println!("  Created goal #{}: ⊢ {}", self.current_goal, cyan(target));
            }
            "hyp" => {
                let args: Vec<&str> = rest.splitn(2, ' ').collect();
                if args.len() < 2 { println!("  Usage: hyp <name> <type>"); return true; }
                if !self.goals.is_empty() {
                    self.goals[self.current_goal].add_hyp(args[0], args[1]);
                    println!("  Added {} : {}", cyan(args[0]), args[1]);
                }
            }
            "state" => {
                if self.goals.is_empty() { println!("  No goals."); return true; }
                println!("  Goal #{} of {}:", self.current_goal + 1, self.goals.len());
                self.goals[self.current_goal].display();
            }
            "universes" => {
                println!("{}", bold("── Universe System in Lean 4 ───────────────────────────────────────"));
                println!("  Lean 4 has a predicative hierarchy:");
                println!("    Prop   : Sort 0  (proof-irrelevant propositions)");
                println!("    Type 0 : Sort 1  (small types = sets)");
                println!("    Type 1 : Sort 2  (types of types)");
                println!("    Type n : Sort (n+1)");
                println!();
                println!("  Sort* = universe-polymorphic: works for any Sort level.");
                println!();
                println!("  Key: Prop is impredicative (∀ p:Prop, P p : Prop for any P).");
                println!("  This is inconsistent with HoTT without propositional resizing.");
                println!();
                println!("  For HoTT: use --without-K and avoid Prop/K-axiom.");
                println!("  The HoTT library uses Type (not Prop) for propositions.");
            }
            "inductive" => {
                println!("{}", bold("── Inductive Types in Lean 4 ───────────────────────────────────────"));
                println!("  {}",  cyan("inductive Nat : Type where"));
                println!("  {}",  cyan("  | zero : Nat"));
                println!("  {}",  cyan("  | succ : Nat → Nat"));
                println!();
                println!("  {}",  cyan("inductive List (α : Type u) : Type u where"));
                println!("  {}",  cyan("  | nil  : List α"));
                println!("  {}",  cyan("  | cons : α → List α → List α"));
                println!();
                println!("  HITs are NOT natively supported in Lean 4.");
                println!("  They can be postulated (axioms) but don't compute.");
                println!("  Use Quotient for quotient types (built-in).");
            }
            "structure" => {
                println!("{}", bold("── Structures and Classes in Lean 4 ────────────────────────────────"));
                println!("  {}",  cyan("structure Group (α : Type*) where"));
                println!("  {}",  cyan("  mul : α → α → α"));
                println!("  {}",  cyan("  one : α"));
                println!("  {}",  cyan("  inv : α → α"));
                println!("  {}",  cyan("  mul_assoc : ∀ a b c, mul (mul a b) c = mul a (mul b c)"));
                println!("  -- etc.");
                println!();
                println!("  Type classes extend structures with instance search:");
                println!("  {}",  cyan("class Group (α : Type*) extends Mul α, One α, Inv α where"));
                println!("  {}",  cyan("  mul_assoc : ∀ a b c : α, a * b * c = a * (b * c)"));
                println!("  -- etc.");
            }
            "typeclass" => {
                println!("{}", bold("── Type Classes vs Explicit Π ────────────────────────────────────────"));
                println!("  Type class (implicit, inferred):");
                println!("  {}",  cyan("def double [Add α] [OfNat α 2] (x : α) := x + x"));
                println!();
                println!("  Explicit Π-type (must pass manually):");
                println!("  {}",  cyan("def double' (α : Type) (add : α → α → α) (x : α) := add x x"));
                println!();
                println!("  In HoTT: equivalences and other structure are often given as");
                println!("  explicit arguments (to avoid typeclass coherence issues).");
                println!();
                println!("  Mathlib uses type classes extensively: Ring, Group, Module, etc.");
                println!("  HoTT libraries tend to use explicit Σ-types for mathematical structure.");
            }
            "univalence" => {
                println!("{}", bold("── Univalence in Lean 4 ────────────────────────────────────────────"));
                println!("  Lean 4 standard library: NO univalence.");
                println!("  Lean 4 standard library axioms:");
                println!("    {} propext : (P ↔ Q) → P = Q", cyan("✓"));
                println!("    {} Classical.em : P ∨ ¬P (law of excluded middle)", cyan("✓"));
                println!("    {} funext : (∀x, f x = g x) → f = g", cyan("✓"));
                println!("    {} Quot.sound : r a b → Quot.mk a = Quot.mk b", cyan("✓"));
                println!();
                println!("  No univalence: (A ≃ B) → (A = B) is NOT provable in std Lean 4.");
                println!();
                println!("  For HoTT in Lean 4: add as an axiom.");
                println!("  The lean4-HoTT library (by various contributors) does this.");
                println!("  {} With univalence + propext: full Book HoTT is available.", green("Then:"));
            }
            "axioms" => {
                println!("{}", bold("── Lean 4 Standard Axioms ──────────────────────────────────────────"));
                println!("  Core (always present):");
                println!("    propext  : Prop extensionality");
                println!("    funext   : function extensionality");
                println!("    Quot     : quotient types");
                println!();
                println!("  Classical (in import Mathlib.Logic.Classical):");
                println!("    Classical.em  : P ∨ ¬P for any Prop P");
                println!("    Classical.choice : (∃ x, P x) → {{x // P x}}  (epsilon operator)");
                println!();
                println!("  HoTT-incompatible axioms to avoid:");
                println!("    Classical.em   — implies LEM (anti-constructive)");
                println!("    Classical.choice — non-constructive choice");
                println!("    K              — definitional proof irrelevance (breaks HoTT)");
                println!();
                println!("  Use --without-K in Lean 4 to disable the K eliminator.");
            }
            "quotient" => {
                println!("{}", bold("── Quotient Types in Lean 4 ─────────────────────────────────────────"));
                println!("  Lean 4 has built-in quotient types:");
                println!("  {}",  cyan("Quotient : {α : Sort u} → Setoid α → Sort u"));
                println!();
                println!("  Interface:");
                println!("  {}",  cyan("Quotient.mk : α → Quotient s"));
                println!("  {}",  cyan("Quotient.sound : s.r a b → Quotient.mk a = Quotient.mk b"));
                println!("  {}",  cyan("Quotient.lift : (f : α → β) → (∀ a b, s.r a b → f a = f b) → Quotient s → β"));
                println!();
                println!("  Used in mathlib for: ℤ (pairs of ℕ), ℚ (pairs of ℤ),");
                println!("  finsets, multisets, homotopy groups, etc.");
                println!();
                println!("  This is the same as HoTT's set-quotient when working with sets.");
                println!("  For HITs with path constructors: must postulate.");
            }
            "mathlib" => {
                println!("{}", bold("── Mathlib4 HoTT Content ───────────────────────────────────────────"));
                println!("  Mathlib4 is the main mathematical library for Lean 4.");
                println!("  It is primarily classical (uses LEM and choice freely).");
                println!();
                println!("  HoTT-relevant content in mathlib4:");
                println!("    • Equiv (type equivalences): α ≃ β");
                println!("    • Function.bijective, surjective, injective");
                println!("    • Groupoid class (category where all morphisms are isos)");
                println!("    • Quotient and Setoid infrastructure");
                println!("    • Truncation via subtypes: {{x // P x}}");
                println!("    • Basic homotopy theory in Topology.*");
                println!();
                println!("  Not in mathlib (or partial):");
                println!("    • HITs (S¹, suspension, pushouts as types)");
                println!("    • Univalence");
                println!("    • Synthetic homotopy theory");
                println!();
                println!("  The lean4-HoTT project aims to fill these gaps.");
            }
            "comparison" => {
                println!("{}", bold("── Lean 4 vs Agda vs Coq for HoTT ──────────────────────────────────"));
                println!("  Feature           Lean 4          Agda (cubical)   Coq");
                println!("  ──────────────────────────────────────────────────────────");
                println!("  Univalence        axiom           computational    axiom (HoTT lib)");
                println!("  HITs              axiom           built-in         axiom");
                println!("  Prop/impredicative yes            no               yes (Prop)");
                println!("  --without-K       yes             built-in         --");
                println!("  Large library     Mathlib (huge)  stdlib (medium)  Mathcomp (large)");
                println!("  Automation        aesop, decide   limited          omega, ring");
                println!("  HoTT library      lean4-HoTT      HoTT-Agda        HoTT-Coq");
                println!();
                println!("  For HoTT: Agda (cubical) is currently the most advanced.");
                println!("  For general math: Lean 4 + Mathlib is the best-supported ecosystem.");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Lean 4 and HoTT — Interactive Sandbox                  ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore HoTT concepts as they appear in Lean 4.");
    println!("  See what's built-in, what's axiomatic, and how it compares to Agda.");
    println!("  Type {} or {}.\n",
        cyan("univalence"), cyan("comparison"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}lean4{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
