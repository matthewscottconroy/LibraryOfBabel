# Chapter 21: Lean 4 and Mathlib — Formal Proof at Scale

## Introduction

Lean 4 is a proof assistant and programming language developed by Leonardo de Moura and the Lean FRO team. Its mathematical library, Mathlib4, is the largest single library of formalized mathematics in existence, containing hundreds of thousands of theorems spanning algebra, analysis, topology, number theory, category theory, and more.

For our purposes, Lean 4 serves two roles:
1. **Practice ground for formalization:** Lean 4's expressive type system (based on the Calculus of Inductive Constructions) allows all the mathematics in Phases 0–4 to be formalized rigorously.
2. **Research tool:** Lean 4's metaprogramming system allows you to build custom tactics, decision procedures, and automation — the infrastructure for mathematical automation research.

This chapter is a working guide to Lean 4, organized around the mathematical content of this curriculum.

---

## 1. The Lean 4 System

### 1.1 Installation and Setup

```bash
# Install elan (Lean version manager)
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh

# Create a new project with Mathlib
lake new my_hott_project math
cd my_hott_project
lake update
lake build
```

VS Code extension: install `lean4` from the marketplace. The extension gives real-time type-checking, `#check` feedback, and goal display.

### 1.2 File Structure

A Lean 4 file consists of:
- `import` statements (importing libraries)
- `section`/`namespace` blocks (organizing code)
- `def`, `theorem`, `lemma`, `example` declarations
- `#check`, `#eval`, `#print` commands (queries)

```lean
import Mathlib.Algebra.Group.Basic
import Mathlib.Topology.Basic

open Nat

#check Nat.add_comm  -- Check a theorem's type
#eval Nat.gcd 12 8   -- Evaluate an expression
```

---

## 2. Core Syntax

### 2.1 Terms and Types

```lean
-- Variables and their types
def myNat : Nat := 42
def myProp : Prop := 1 + 1 = 2
def myFun : Nat → Nat := fun n => n + 1

-- Dependent function type (Π type)
def idFun : (A : Type) → A → A := fun A a => a

-- Dependent pair type (Σ type)
def evenPair : Σ n : Nat, n % 2 = 0 := ⟨4, rfl⟩

-- Anonymous constructor ⟨a, b⟩ for structures and Σ types
```

### 2.2 Propositions and Proofs

In Lean 4, `Prop` is the universe of propositions. Every `theorem` and `lemma` produces a term of a type in `Prop`.

```lean
-- A theorem is just a declaration whose type is a Prop
theorem add_comm_simple (m n : Nat) : m + n = n + m :=
  Nat.add_comm m n  -- Term-mode proof: cite existing theorem

-- Or prove it yourself:
theorem my_add_comm (m n : Nat) : m + n = n + m := by
  induction m with
  | zero => simp
  | succ k ih => simp [Nat.succ_add, ih]
```

### 2.3 Inductive Types

```lean
-- Custom inductive type
inductive MyList (α : Type) : Type where
  | nil  : MyList α
  | cons : α → MyList α → MyList α

-- Pattern matching
def myLength : MyList α → Nat
  | .nil       => 0
  | .cons _ xs => 1 + myLength xs

-- Proof by pattern matching (term mode)
theorem nil_length : myLength (MyList.nil (α := Nat)) = 0 := rfl
```

---

## 3. Tactic Mode

### 3.1 The Proof State

In tactic mode, the proof state shows:
- Context (hypotheses): `h : P`, `x : A`, etc.
- Goal: `⊢ Q` (what remains to be proved)

```lean
theorem example_tactic (P Q : Prop) (hp : P) (hq : Q) : P ∧ Q := by
  -- Goal: ⊢ P ∧ Q
  constructor
  -- Goal 1: ⊢ P
  · exact hp
  -- Goal 2: ⊢ Q
  · exact hq
```

### 3.2 Core Tactics

**Navigation and structure:**
```lean
intro h        -- Introduce a hypothesis h : P from goal ⊢ P → Q
apply f        -- If f : P → Q and goal is ⊢ Q, reduces to ⊢ P
exact e        -- Close goal with term e (must match exactly)
constructor    -- Split conjunction or use first constructor of inductive
left; right    -- Choose left or right branch of disjunction
obtain ⟨a,b⟩ := h  -- Destruct h into components
```

**Rewriting:**
```lean
rw [h]         -- Rewrite goal using h : a = b (replaces a with b)
rw [← h]       -- Rewrite right-to-left (replaces b with a)
simp           -- Simplification using simp lemmas
simp [h, f]    -- Simplification using h and f as additional lemmas
ring           -- Prove ring equalities (ring axioms)
omega          -- Linear arithmetic over integers/naturals
norm_num       -- Numerical normalization
```

**Induction and case analysis:**
```lean
induction n with        -- Induction on natural number n
| zero => ...           -- Base case
| succ k ih => ...      -- Inductive case with IH ih

cases h with            -- Case analysis on h : A ∨ B or inductive type
| inl hp => ...         -- Left case
| inr hq => ...         -- Right case

rcases h with ⟨a, ha⟩  -- Recursive case analysis (destructs nested patterns)
```

**Automation:**
```lean
tauto          -- Propositional tautologies
decide         -- Decidable propositions (computes the answer)
aesop          -- General-purpose automation
positivity     -- Positivity of expressions
```

### 3.3 Searching for Lemmas

```lean
exact?     -- Find a lemma that closes the goal
apply?     -- Find a lemma that applies to the goal
rw?        -- Find a lemma that rewrites the goal
simp?      -- Find the minimal simp set needed
```

---

## 4. Working with Mathlib

### 4.1 The Organization of Mathlib

Mathlib is organized into directories matching mathematical areas:
- `Mathlib.Algebra`: Groups, rings, fields, modules
- `Mathlib.Topology`: Topological spaces, metric spaces
- `Mathlib.Analysis`: Real and complex analysis, measure theory
- `Mathlib.CategoryTheory`: Categories, functors, natural transformations
- `Mathlib.Order`: Lattices, partial orders
- `Mathlib.Logic`: Classical logic, decidability
- `Mathlib.Data`: Specific data types (Nat, Int, List, Fin, ...)

### 4.2 Finding Theorems

**By name:** Lean 4 uses a systematic naming convention:
- `Nat.add_comm`: commutativity of addition for `Nat`
- `List.length_append`: length of list concatenation
- `Group.mul_inv_cancel`: right inverse in a group

**By symbol search:** Use `#check @Nat.add_comm` to see the type; use the VS Code extension to hover over names.

**By automation:** Use `exact?`, `apply?`, or `#check?` to search.

**Online:** Mathlib docs at `leanprover-community.github.io/mathlib4_docs/`

### 4.3 The CategoryTheory Library

```lean
import Mathlib.CategoryTheory.Category.Basic
import Mathlib.CategoryTheory.Functor.Basic
import Mathlib.CategoryTheory.NatTrans

open CategoryTheory

-- A category is a typeclass
variable {C : Type*} [Category C]

-- Morphisms: f : X ⟶ Y (special arrow)
variable {X Y Z : C}

-- The Yoneda lemma
#check yonedaEquiv  -- yonedaEquiv : (yoneda.obj X ⟹ F) ≃ F.obj X

-- Functors
#check Functor.comp  -- Composition of functors

-- Natural transformations
#check NatTrans.comp -- Vertical composition
```

---

## 5. Formalizing Mathematics: A Guided Example

### 5.1 Groups in Lean 4

```lean
-- Groups are already in Mathlib as a typeclass
#check @Group   -- Group : Type u → Prop

-- Using a group
variable {G : Type*} [Group G]

-- The isomorphism theorems
#check QuotientGroup.quotientKerEquivRange
-- : (G ⧸ f.ker) ≃* f.range   (First Isomorphism Theorem)

-- Free groups
#check FreeGroup  -- FreeGroup : Type u → Type u
#check FreeGroup.of  -- Generator inclusion
```

### 5.2 Formalizing Phase 0 Results

```lean
-- Theorem: every group of prime order is cyclic
theorem prime_order_cyclic (G : Type*) [Group G] [Fintype G]
    (p : ℕ) (hp : Nat.Prime p) (h : Fintype.card G = p) :
    IsCyclic G := by
  exact isCyclic_of_prime_card hp h
-- (This is already in Mathlib!)

-- Schröder-Bernstein theorem
#check Set.schroeder_bernstein
```

---

## 6. Tactic Writing: Lean 4 Metaprogramming

Lean 4's metaprogramming system allows you to write custom tactics in Lean itself.

### 6.1 The Macro System

```lean
-- A simple custom tactic using macros
macro "myTactic" : tactic => `(tactic| simp; ring)

-- Use it
example (n : Nat) : n + 0 = n := by myTactic
```

### 6.2 Writing a Custom Tactic

```lean
import Lean.Elab.Tactic

open Lean.Elab.Tactic

-- A tactic that tries `rfl` then `simp`
elab "try_rfl_then_simp" : tactic => do
  let goal ← getMainGoal
  try
    closeMainGoal (← mkAppM `rfl #[])
  catch _ =>
    evalTactic (← `(tactic| simp))
```

---

## 7. Projects

The following projects build toward research-level formalization in Lean 4:

**Project 1 (Beginner, 2-4 weeks):** Formalize all theorems from Chapter 2 (Abstract Algebra) not already in Mathlib. Focus on free groups and group presentations.

**Project 2 (Intermediate, 1-2 months):** Add a new algebraic structure to Mathlib. For example: ordered groups with a specific property, or a new class of rings.

**Project 3 (Advanced, 3-6 months):** Formalize a chapter of an advanced textbook in Mathlib. Possible targets:
- Hatcher's Algebraic Topology, Chapter 0 or 1 (fundamental group)
- Serre's Local Fields (for algebraic number theory)
- Atiyah-MacDonald, Chapter 1 (commutative rings)

**Project 4 (Research, 6+ months):** Write a new tactic or decision procedure for Lean 4. Possible targets:
- A tactic for automating group theory calculations (word problem for specific groups)
- A normalization procedure for Σ/Π types
- Integration with an external solver (SAT, SMT)

---

## 8. Contributing to Mathlib

### 8.1 The Contribution Process

1. Find a theorem that Mathlib is missing (use `exact?` — if it can't find it, it might not exist)
2. Open an issue or check existing issues on GitHub
3. Write the theorem and proof, following Mathlib's style guide
4. Submit a pull request and address reviewer feedback

### 8.2 Style Guide Highlights

- Name theorems following the convention: `Namespace.adjective_noun_property`
- Use `simp` lemmas with `@[simp]` attribute for standard simplifications
- Write docstrings for all public declarations
- Use the `gcongr` tactic for congruence goals
- Prefer `rintro`, `obtain`, `rcases` over manual destructions

---

## Exercises

**21.1.** In Lean 4, prove without using Mathlib's `Nat.add_comm`:
  - `Nat.add_zero : ∀ n, n + 0 = n`
  - `Nat.succ_add : ∀ m n, succ m + n = succ (m + n)`
  - `Nat.add_comm : ∀ m n, m + n = n + m` (using the above)

**21.2.** Formalize the Schröder-Bernstein theorem from scratch in Lean 4 (without using the Mathlib version).

**21.3.** In Lean 4, define a type class `MyGroup` with the group axioms, and prove the following from the axioms alone (no Mathlib):
  - The identity is unique
  - Inverses are unique
  - $(ab)^{-1} = b^{-1}a^{-1}$

**21.4.** Using Mathlib's `CategoryTheory` library:
  - State and check the Yoneda lemma (`yonedaEquiv`)
  - Prove that any two terminal objects in a category are isomorphic

**21.5.** Write a custom Lean 4 tactic `group_simp` that applies the group axioms to simplify group expressions.

**21.6 (Research).** Find three theorems in the curriculum that are NOT yet in Mathlib. Submit PRs adding them. (Check the Mathlib issue tracker for "good first issue" labels.)
