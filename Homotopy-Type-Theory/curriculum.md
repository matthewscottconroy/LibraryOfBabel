# Homotopy Type Theory: From Foundations to Research
## A Complete Curriculum for Automated Theorem Proving and Foundational Research

> **Goal:** Gain deep mastery of HoTT, implement proofs in modern proof assistants (Lean 4, Agda, Coq/Rocq), and produce original research at the intersection of homotopy theory, type theory, and the foundations of mathematics and computer science.

> **Philosophy:** Every level builds load-bearing structure. Do not skip phases. The cathedral requires every stone.

---

## How to Use This Document

Each phase has:
- **Core concepts** to understand deeply
- **Primary texts** (read carefully, work all exercises)
- **Secondary texts** (read selectively, reference often)
- **Proof assistant exercises** (formalize as you go — do not defer this)
- **Milestone:** a concrete artifact proving you are ready to advance

Estimated total time for a serious part-time student (15–20 hrs/week): **3–5 years** to Phase 8. A full-time student: **18–30 months**.

---

# PHASE 0 — Mathematical Maturity and Proof Culture
### Duration: 2–4 months (or skip if already fluent)

**What this phase is:** Before type theory, you need fluency in writing rigorous proofs, comfort with abstraction, and an honest survey of where your gaps are.

---

## 0.1 Logic and Proof Writing

**Core concepts:**
- Propositional logic: connectives, truth tables, semantic entailment
- Predicate logic: quantifiers, first-order structures, interpretations
- Proof techniques: direct proof, proof by contradiction, induction (simple, strong, structural, well-founded)
- The difference between syntax and semantics
- What it means for a proof to be *correct* vs. merely *convincing*

**Primary texts:**
- Velleman, *How to Prove It* (3rd ed.) — work every chapter
- Hammack, *Book of Proof* (free online) — supplement for intuition

**Milestone:** Write 20 original, fully rigorous proofs covering all proof styles. Have a mathematician check them.

---

## 0.2 Naive and Axiomatic Set Theory

**Core concepts:**
- Zermelo–Fraenkel set theory with Choice (ZFC): axioms and their motivation
- Ordinals and cardinals
- Well-orderings, transfinite induction
- The cumulative hierarchy V_α
- Why set theory is both the "standard" foundation and why it has problems
- Russell's paradox; why naive comprehension fails
- The axiom of choice: equivalents (Zorn's lemma, well-ordering theorem), independence

**Primary texts:**
- Enderton, *Elements of Set Theory* — thorough, rigorous
- Halmos, *Naive Set Theory* — for intuition and speed

**Secondary texts:**
- Kunen, *Set Theory* (2011) — for the independence results and forcing (come back later)

**Milestone:** Prove the Schröder–Bernstein theorem and the well-ordering of ordinals from scratch. Explain in your own words why the axiom of choice is both essential and controversial.

---

## 0.3 Abstract Algebra

**Core concepts:**
- Groups, subgroups, quotient groups, homomorphisms, isomorphism theorems
- Rings, fields, modules
- Polynomial rings and factorization
- Group actions and the orbit-stabilizer theorem
- Free groups and presentations

**Primary texts:**
- Dummit & Foote, *Abstract Algebra* — the standard reference; work Parts I–III minimum
- Aluffi, *Algebra: Chapter 0* — category-theoretic perspective; highly recommended as complement

**Milestone:** Prove Sylow's theorems. Understand free groups well enough to explain them to someone who has never seen them.

---

## 0.4 Real Analysis

**Core concepts:**
- Metric spaces: convergence, completeness, compactness
- Continuity and uniform continuity
- The real numbers as a complete ordered field
- Sequences and series
- The topology of R^n

**Primary texts:**
- Rudin, *Principles of Mathematical Analysis* (Baby Rudin) — work Chapters 1–7

**Milestone:** Prove the Heine–Borel theorem and the intermediate value theorem from metric space axioms.

---

# PHASE 1 — Classical and Intuitionistic Logic
### Duration: 2–3 months

**What this phase is:** Type theory is built on *constructive* logic. You need to understand why classical logic and constructive logic differ, why this matters computationally, and how to think proof-theoretically.

---

## 1.1 Proof Theory

**Core concepts:**
- Natural deduction (Gentzen-style): introduction and elimination rules for each connective
- Sequent calculus: left/right rules, cut elimination
- Normal forms and normalization
- The subformula property
- Proof terms vs. proof trees

**Primary texts:**
- Troelstra & Schwichtenberg, *Basic Proof Theory* — the canonical reference; Chapters 1–3
- Negri & von Plato, *Structural Proof Theory* — more accessible; read as companion

**Key exercises:**
- Construct natural deduction proofs for: modus ponens, hypothetical syllogism, disjunction elimination
- Show that `¬¬P → P` is *not* derivable in intuitionistic logic
- Prove cut elimination for propositional sequent calculus

---

## 1.2 Intuitionistic Logic and the BHK Interpretation

**Core concepts:**
- The Brouwer–Heyting–Kolmogorov (BHK) interpretation of logical connectives
- Intuitionistic propositional logic (IPC) and its Kripke semantics
- What "proof" means constructively: a proof of `P ∧ Q` is a pair; a proof of `P → Q` is a function
- Classical logic as a special case (double-negation translation)
- The law of excluded middle (LEM): why constructivists reject it, what it costs, what it buys
- Markov's principle, Church's thesis (as logical axioms)
- Heyting algebras

**Primary texts:**
- van Dalen, *Logic and Structure* (5th ed.) — Chapters 5–6
- Troelstra & van Dalen, *Constructivism in Mathematics* Vol. I — Part I

**Key exercises:**
- Translate classical tautologies via the Gödel–Gentzen double-negation translation
- Show that `((P → Q) → P) → P` (Peirce's law) requires LEM
- Model intuitionistic propositional logic in a specific Kripke frame

---

## 1.3 The Curry–Howard Correspondence

**Core concepts:**
- Propositions as types, proofs as programs (the central isomorphism)
- `P ∧ Q` ↔ product type `A × B`
- `P ∨ Q` ↔ sum type `A + B`
- `P → Q` ↔ function type `A → B`
- `⊥` ↔ empty type; `¬P` ↔ `P → ⊥`
- `∀x. P(x)` ↔ dependent product (Π type)
- `∃x. P(x)` ↔ dependent sum (Σ type)
- Normal proofs as normal programs; normalization as proof simplification
- Simply typed lambda calculus (STLC): types, terms, reduction
- Strong normalization of STLC

**Primary texts:**
- Sørensen & Urzyczyn, *Lectures on the Curry-Howard Isomorphism* — the definitive treatment
- Girard, Lafont & Taylor, *Proofs and Types* (free online) — concise and beautiful

**Key exercises:**
- Write a proof of `(A → B) → (B → C) → (A → C)` as a lambda term
- Prove strong normalization for propositional IPC via the method of candidates
- Show the inhabitant of `A → A` is exactly `λx.x`

**Milestone:** Given a proposition in intuitionistic propositional logic, write its proof as a typed lambda term and verify the type.

---

# PHASE 2 — Type Theory
### Duration: 3–5 months

**What this phase is:** The technical heart of the prerequisite structure. You need deep comfort with dependent types before HoTT makes sense.

---

## 2.1 Simply Typed Lambda Calculus and System F

**Core concepts:**
- STLC: syntax, typing rules, reduction rules, Church vs. Curry style
- Progress and preservation (type safety)
- Normalization: weak and strong
- System F: universal quantification over types, polymorphism
- Parametricity (Reynolds): what polymorphic types force functions to do
- System F_ω: type operators, kinds

**Primary texts:**
- Pierce, *Types and Programming Languages* (TAPL) — work Chapters 1–11, 23–25
- Girard, *Proofs and Types* — for System F

**Proof assistant exercises (start Agda or Lean here):**
- Implement STLC in Lean 4 or Agda with a type checker
- Implement System F and prove parametricity for `∀α. α → α`

---

## 2.2 Dependent Type Theory

**Core concepts:**
- Dependent function types (Π types): `(x : A) → B x`
- Dependent pair types (Σ types): `Σ (x : A), B x`
- Universes: `Type₀ : Type₁ : Type₂ : ...`
- Russell vs. Tarski-style universes
- The calculus of constructions (CoC): Barendregt's pure type systems
- Eliminators: how to use a type by specifying what you get from each constructor
- Pattern matching as syntactic sugar for eliminators
- Universes and universe polymorphism
- Cumulativity

**Primary texts:**
- Nordström, Petersson & Smith, *Programming in Martin-Löf's Type Theory* (free online) — the original treatment
- Thompson, *Type Theory and Functional Programming* (free online) — accessible bridge
- Pierce et al., *Software Foundations* Vol. 1 (Coq) — for hands-on practice

**Proof assistant exercises:**
- Formalize all of Phase 0.2 (naive set theory) in Lean 4 or Agda
- Prove properties of vectors (length-indexed lists) using dependent types
- Define the type of well-founded trees and prove properties by structural induction

---

## 2.3 Martin-Löf Type Theory (MLTT)

**Core concepts:**
- Intensional vs. extensional MLTT
- The formation/introduction/elimination/computation (FIEC) rules
- Π types and Σ types in full detail
- The identity type: `Id_A(a, b)` (also written `a =_A b` or `a ≡ b`)
- Path induction: the J eliminator and its computation rule
- The uniqueness of identity proofs (UIP) — and why MLTT does NOT force it
- Inductive types: W-types as a general framework
- Natural numbers as an inductive type; Peano arithmetic in MLTT
- Definitional equality vs. propositional equality
- Normalization and decidability of type checking in MLTT

**Primary texts:**
- Hofmann, *Syntax and Semantics of Dependent Types* (chapter in Pitts & Dybjer eds.) — essential
- Streicher, *Semantics of Type Theory* — model theory
- The original Martin-Löf papers: "An Intuitionistic Theory of Types" (1975), "Intuitionistic Type Theory" (Padova lectures, 1984)

**Critical exercise — the central insight of HoTT:**
- Prove in MLTT that the identity type `Id_Bool(true, false)` is empty
- Now notice: MLTT cannot *prove* that `Id_{Type}(Bool, Bool') = ⊤` even when Bool and Bool' are "the same" — this is the problem univalence solves
- Write out the J eliminator for the identity type and understand what path induction allows

**Milestone:** Formalize all of Peano arithmetic in Agda, including induction. Prove commutativity and associativity of addition from scratch, using only the J eliminator for any equality reasoning.

---

## 2.4 Proof Assistants — Lean 4 (Primary) and Agda (Secondary)

**Why both:** Lean 4 has Mathlib (the largest formalized mathematics library in existence). Agda has cubical mode and HoTT-Agda, which are closer to cutting-edge HoTT research.

### Lean 4

**Core skills:**
- The `theorem`, `def`, `structure`, `class`, `instance` keywords
- Tactic mode: `intro`, `apply`, `exact`, `rw`, `simp`, `ring`, `norm_num`, `omega`, `linarith`
- Term mode proofs
- `#check`, `#print`, `#eval`
- Mathlib: how to find theorems, `exact?`, `apply?`, `rw?`
- Inductive types and recursive functions
- Type class inference
- Metaprogramming basics (macros, tactics-as-programs)

**Primary resources:**
- *Theorem Proving in Lean 4* (official docs, free online) — read cover to cover
- *Mathematics in Lean* (Mathlib docs) — work every chapter
- Avigad et al., *Logic and Proof* — companion

**Projects:**
- Formalize all 20 proofs from Phase 0.1 milestone in Lean 4
- Complete the *Mathematics in Lean* exercises through Chapter 6

### Agda

**Core skills:**
- Syntax: `data`, `record`, `module`, `open`, `import`
- Pattern matching and the with-clause
- Universe levels
- The `--without-K` flag and why it matters for HoTT
- The `--cubical` flag
- Agda standard library and the Agda HoTT library

**Primary resources:**
- *Programming Language Foundations in Agda* (PLFA, free online) — work cover to cover
- Bove & Dybjer, *Dependent Types at Work* (free online)

**Projects:**
- Formalize STLC with progress and preservation in Agda
- Complete all of PLFA Part I and II

---

# PHASE 3 — Category Theory
### Duration: 3–4 months

**What this phase is:** HoTT is deeply categorical. The identity type corresponds to morphisms in a groupoid, univalence corresponds to equivalences in ∞-categories, and higher inductive types correspond to colimits. You cannot understand HoTT deeply without category theory.

---

## 3.1 Basic Category Theory

**Core concepts:**
- Categories: objects, morphisms, composition, identity, associativity
- Functors: structure-preserving maps between categories
- Natural transformations: maps between functors
- The Yoneda lemma: one of the most important theorems in mathematics
- Universal properties: limits (products, equalizers, pullbacks), colimits (coproducts, pushouts, coequalizers)
- Adjunctions: unit, counit, triangular identities; left adjoints preserve colimits
- Monads: definition, Kleisli and Eilenberg–Moore categories
- Representable functors
- The category Set as a motivating example throughout

**Primary texts:**
- Riehl, *Category Theory in Context* (free online) — the best modern introduction
- Mac Lane, *Categories for the Working Mathematician* — the classical reference; read after Riehl

**Secondary texts:**
- Leinster, *Basic Category Theory* (free online) — concise and clear
- Awodey, *Category Theory* — excellent for type theorists

**Proof assistant exercises:**
- Formalize the definition of a category in Lean 4 using Mathlib's `CategoryTheory` library
- Prove the Yoneda lemma in Lean 4
- Formalize adjunctions and prove that left adjoints preserve colimits

**Milestone:** State and prove the Yoneda lemma from scratch on paper. Explain in your own words what the lemma *means* and why it matters.

---

## 3.2 Categorical Logic and Fibrations

**Core concepts:**
- Internal logic of a category
- Cartesian closed categories (CCC) and their correspondence with simply typed lambda calculus
- Locally Cartesian closed categories (LCCC) and dependent type theory
- Toposes: definition and basic properties
- The subobject classifier
- Fibered categories (Grothendieck fibrations): the language for type theory over a base
- Display maps and comprehension categories
- Contextual categories (C-systems) and their correspondence with MLTT

**Primary texts:**
- Jacobs, *Categorical Logic and Type Theory* — the comprehensive reference (free online)
- Johnstone, *Sketches of an Elephant* Vol. I — for topos theory
- Awodey & Warfield, notes on categorical semantics of type theory

**Secondary texts:**
- Pitts, *Categorical Logic* (notes, free online)
- Streicher, *Semantics of Type Theory*

**Milestone:** Explain the correspondence between LCCC structure and dependent type theory. Construct the contextual category of contexts and substitutions for MLTT.

---

## 3.3 Higher Category Theory (Introduction)

**Core concepts:**
- 2-categories: objects, 1-morphisms, 2-morphisms; strict vs. weak
- Bicategories: the weakened notion where associativity holds up to isomorphism
- Groupoids: categories where all morphisms are invertible
- The fundamental groupoid of a topological space
- ∞-groupoids: the homotopy-theoretic generalization
- The homotopy hypothesis (Grothendieck): ∞-groupoids are equivalent to homotopy types
- (∞,1)-categories: ∞-categories where all k-morphisms for k > 1 are invertible
- Quasi-categories (Joyal, Lurie): simplicial sets satisfying the inner horn filling condition
- The nerve of a category

**Primary texts:**
- Leinster, *A Survey of Definitions of n-Category* (arXiv) — for orientation
- Riehl, *A Leisurely Introduction to Simplicial Sets* (free online)
- Bergner, *A Survey of (∞,1)-Categories* (arXiv)
- Lurie, *Higher Topos Theory* (free online) — Chapter 1 as a target for this phase

**Note:** You do not need to master higher category theory before starting HoTT. You need enough to understand the homotopy hypothesis and the categorical semantics. Return here more deeply in Phase 7.

---

# PHASE 4 — Algebraic Topology
### Duration: 3–4 months

**What this phase is:** HoTT is *synthetic* homotopy theory. The "homotopy" in HoTT comes directly from algebraic topology. You need enough classical topology to understand what the synthetic version is capturing.

---

## 4.1 Point-Set Topology

**Core concepts:**
- Topological spaces and continuous maps
- Bases and sub-bases
- Connectedness, path-connectedness
- Compactness (open cover definition, limit point compactness)
- Separation axioms (T0 through T4)
- Quotient spaces and quotient maps
- Product topology, subspace topology
- The real line as the canonical example

**Primary texts:**
- Munkres, *Topology* (2nd ed.) — Chapters 1–3; work all exercises in Chapters 2–3

---

## 4.2 Homotopy Theory

**Core concepts:**
- Homotopy between maps: `H : X × [0,1] → Y`
- Homotopy equivalence vs. homeomorphism
- The fundamental group π₁(X, x₀): definition, group structure, functoriality
- Van Kampen's theorem: computing π₁ of CW complexes
- Covering spaces: the fundamental theorem, universal cover, deck transformations
- Higher homotopy groups πₙ(X, x₀): definition and basic properties
- The long exact sequence of a fibration
- Fibrations and cofibrations (Hurewicz)
- CW complexes: cell attachment, skeleta
- Whitehead's theorem
- Eilenberg–MacLane spaces K(G, n)

**Primary texts:**
- Hatcher, *Algebraic Topology* (free online) — Chapter 1 (fundamental group) in full; Chapter 4 (homotopy theory) Sections 4.1–4.2
- May, *A Concise Course in Algebraic Topology* (free online) — for the fibration/cofibration yoga

**Key exercises:**
- Compute π₁(S¹) using van Kampen's theorem
- Show that π₁(X × Y) ≅ π₁(X) × π₁(Y)
- Prove that covering space morphisms are determined by their value at a single point

**Milestone:** Compute the fundamental group of the torus, the Klein bottle, and RP². Explain how CW complexes allow systematic computation of homotopy groups.

---

## 4.3 Simplicial Homotopy Theory

**Core concepts:**
- Simplicial sets: the functor category `Δᵒᵖ → Set`
- The standard simplex Δ[n]; horns Λᵏ[n]; boundaries ∂Δ[n]
- Kan complexes: the horn-filling condition
- The geometric realization |K| of a simplicial set
- The singular simplicial set Sing(X) of a topological space
- Weak homotopy equivalence between simplicial sets
- The Quillen model structure on simplicial sets
- Homotopy groups of Kan complexes
- Why Kan complexes are the "right" notion of ∞-groupoid

**Primary texts:**
- Friedman, *An Elementary Illustrated Introduction to Simplicial Sets* (arXiv) — start here
- Goerss & Jardine, *Simplicial Homotopy Theory* — the reference
- May, *Simplicial Objects in Algebraic Topology*

**Why this matters for HoTT:** The HoTT Book's model (Voevodsky's original) lives in Kan simplicial sets. Cubical type theory's model lives in a presheaf category on the cube category. Understanding simplicial sets is understanding the semantics.

---

# PHASE 5 — Homotopy Type Theory (Core)
### Duration: 4–6 months

**What this phase is:** The primary subject. This is where everything comes together.

---

## 5.1 The HoTT Book — Foundations

**Read:** Homotopy Type Theory: Univalent Foundations of Mathematics (the HoTT Book, free online at homotopytypetheory.org)

**Part I: Foundations — work every section**

**Core concepts:**

### Type Formers
- Π types (dependent functions)
- Σ types (dependent pairs)
- Coproducts (A + B)
- The empty type (𝟘) and unit type (𝟙)
- The type of booleans (𝟚)
- The natural numbers (ℕ)
- The identity type (a =_A b) — the central object of study

### Identity Types as Path Spaces
- A term `p : a =_A b` is a *path* from `a` to `b` in the *space* A
- Path induction (J) as the fundamental principle
- Why this is the right geometric intuition
- Concatenation of paths: `p ∙ q : a = c` from `p : a = b` and `q : b = c`
- Inversion of paths: `p⁻¹ : b = a`
- The groupoid laws for paths: associativity, unit, inverse — all hold *propositionally* (up to higher paths)
- Transport: if `P : A → Type` and `p : a = b`, then `transport P p : P a → P b`
- The action on paths: `ap f : a = b → f a = f b`

### H-levels (Truncation Levels)
- Contractible types (h-level −2): there exists a center of contraction
- Mere propositions / h-Props (h-level −1): any two elements are equal
- Sets / h-Sets (h-level 0): the identity type is always a mere proposition
- 1-groupoids (h-level 1): the identity types are sets
- n-types: the iterated identity types are eventually contractible
- The hierarchy: contractible → prop → set → groupoid → 2-groupoid → ...
- Truncation: forcing a type to be at most an n-type (the n-truncation ‖A‖ₙ)

### Equivalences
- Quasi-equivalences: `f : A → B` with `g : B → A` and homotopies `f ∘ g ~ id` and `g ∘ f ~ id`
- Why quasi-equivalences form a bad notion (not a proposition)
- Bi-invertible maps (the correct definition)
- Half-adjoint equivalences
- Contractible fibers characterization: `f` is an equivalence iff all fibers `fib_f(b)` are contractible
- The type `A ≃ B` of equivalences between A and B

### Univalence
- The univalence axiom: `(A = B) ≃ (A ≃ B)`
- What this means: paths between types are equivalences
- Consequences: function extensionality (funext), propositional extensionality
- The transport along a path of types: `transport id_Type p : A → B`
- Why univalence is not provable in MLTT (Streicher's groupoid model)
- Univalence as a *theorem* in cubical type theory

**Primary text:** HoTT Book, Chapters 1–4

**Proof assistant exercises (Agda with --without-K, or cubical Agda):**
- Prove the groupoid laws for identity types using path induction only
- Prove function extensionality from univalence
- Show that `isProp A → isProp B → isProp (A × B)`
- Show that `isSet A → isSet (A → B)` iff `isSet B`
- Prove that `ℕ` is a set (h-level 0)

---

## 5.2 Higher Inductive Types (HITs)

**Core concepts:**
- Inductive types as generated by point constructors
- HITs: also generated by *path constructors* (and higher path constructors)
- The interval type `I`: two point constructors `0, 1 : I` and a path constructor `seg : 0 = 1`
- The circle S¹: one point constructor `base : S¹` and one path constructor `loop : base = base`
- Suspension `ΣX`: constructors `N, S : ΣX` and `merid : X → N = S`
- Truncations as HITs: `‖A‖ₙ` the n-truncation
- Pushouts: the HIT encoding of homotopy pushouts
- Colimits as HITs

**Computing with HITs:**
- `π₁(S¹) = ℤ` — the canonical computation in HoTT (requires sophisticated argument)
- The Seifert–van Kampen theorem synthetically
- Freudenthal suspension theorem

**Primary text:** HoTT Book, Chapter 6

**Proof assistant exercises:**
- Define S¹, the torus `T²`, and RP² as HITs in Agda
- Prove `π₁(S¹) = ℤ` (this is a significant project; use the encode-decode method)
- Define the pushout and prove its universal property

**Milestone:** Complete the encode-decode proof of `π₁(S¹) = ℤ` in Agda. This is the canonical benchmark.

---

## 5.3 Synthetic Homotopy Theory

**Core concepts:**
- Fibers and fiber sequences
- The long exact sequence of a fibration in HoTT
- Connectedness: n-connectedness, n-connected maps
- The Blakers–Massey theorem (excision)
- The Freudenthal suspension theorem
- Eilenberg–MacLane spaces as HITs
- Cohomology operations
- The Hopf fibration: `S¹ → S³ → S²` as a construction in HoTT

**Primary texts:**
- HoTT Book, Chapter 8
- Buchholtz, van Doorn & Rijke, *Higher Groups in Homotopy Type Theory* (arXiv)
- Brunerie, *On the Homotopy Groups of Spheres in Homotopy Type Theory* (PhD thesis, arXiv) — a landmark document
- van Doorn, *On the Formalization of Higher Inductive Types and Synthetic Homotopy Theory* (PhD thesis)

**Key result:** Brunerie computed π₄(S³) = ℤ/2ℤ *in* HoTT, with a term whose type was only verified by a computer. This is current research-level work and a concrete goal for Phase 8.

**Proof assistant projects:**
- Formalize the Hopf fibration in Agda
- Prove the Freudenthal suspension theorem
- Formalize the long exact sequence of a fibration

---

## 5.4 Set-Level Mathematics in HoTT

**Core concepts:**
- Sets in HoTT (h-sets): the correct notion of "set" in a univalent foundation
- The category of sets in HoTT: equivalent to a well-pointed boolean elementary topos with NNO
- How classical mathematics embeds into HoTT
- Mere propositions as the correct notion of "truth value"
- The propositional truncation `‖A‖`: "there merely exists"
- Choice and its variants in HoTT: AC, ACω, WISC
- Decidability: `P + ¬P` as a *type*; decidable equality
- The difference between `Σ (n : ℕ), P n` (there exists, with witness) and `‖Σ (n : ℕ), P n‖` (there merely exists)

**Primary text:** HoTT Book, Chapter 3 and 10

**Critical understanding:** In HoTT, the difference between "mere existence" (`‖Σ‖`) and "existence with a chosen witness" (`Σ`) is mathematically and computationally fundamental. This is invisible in classical set theory.

---

# PHASE 6 — Proof Assistants — Advanced
### Duration: 4–6 months (ongoing; deepen throughout later phases)

---

## 6.1 Lean 4 + Mathlib (Research-Level)

**Core skills:**
- The `CategoryTheory` library: categories, functors, natural transformations, limits, adjunctions, monads
- `Topology.Algebra` and homotopy-adjacent material
- Writing your own tactics in Lean 4 (Lean metaprogramming)
- `decide` and reflection tactics
- `norm_cast`, `push_cast` and coercions
- Lean 4 elaboration: how the type checker actually works
- Contributing to Mathlib: style guide, PR process, `leanprover-community` GitHub

**Key projects:**
- Port or contribute a HoTT-adjacent result to Mathlib (even if stated classically)
- Formalize a chapter of a graduate algebra or topology textbook
- Write a custom tactic that automates a repetitive proof pattern in your area

**Resources:**
- Mathlib4 source (GitHub: leanprover-community/mathlib4)
- Lean 4 metaprogramming book (free online)
- Avigad's lecture notes on formalization

---

## 6.2 Agda — HoTT Library and Cubical Mode

**Core skills:**
- The agda/agda-stdlib library
- The HoTT-Agda library (GitHub: HoTT/HoTT-Agda)
- The Cubical Agda library (GitHub: agda/cubical)
- The `--cubical` pragma: interval type, path types, `hcomp`, `transp`
- Universe levels in Agda: `Level`, `lzero`, `lsuc`
- Sized types for termination of coinductive definitions
- Postulates and their risks (`postulate univalence`)
- Reflection in Agda (metaprogramming)

**Key projects:**
- Contribute to the Cubical Agda library
- Formalize a theorem from Chapters 6–8 of the HoTT Book in Cubical Agda
- Implement a decision procedure for some type-theoretic question

---

## 6.3 Coq/Rocq + UniMath

**Core skills:**
- Coq 8.x / Rocq: `Inductive`, `Definition`, `Lemma`, `Theorem`, `Proof`
- Tactic language: `intros`, `apply`, `rewrite`, `exact`, `unfold`, `induction`, `destruct`
- The `ssreflect` library and its tactic style
- UniMath (GitHub: UniMath/UniMath): a library for univalent mathematics in Coq
- UniMath's type-theoretic foundations: `UU` for types, `hProp`, `hSet`, `hhSet`
- The UniMath style: why they avoid proof irrelevance and use `hProp` explicitly

**Key projects:**
- Work through the UniMath school exercises (available on GitHub)
- Formalize a result from the HoTT Book in UniMath

---

# PHASE 7 — Advanced Type Theory
### Duration: 4–6 months

**What this phase is:** The cutting edge. This is where HoTT research is actively happening.

---

## 7.1 Cubical Type Theory

**Core concepts:**

### The Problem with Axiomatic HoTT
- Univalence and HITs as *axioms* break canonicity: there is no algorithm to reduce `ua(f)(x)` to a normal form
- This means we cannot extract computational content from proofs involving univalence
- The "computational interpretation" of HoTT was an open problem until 2015

### Cubical Approach
- The interval `I = [0,1]` as a *primitive type* (not a HIT)
- `I` has endpoints `i0, i1 : I` and a formal dimension variable `i : I`
- Path types `Path A a b = (i : I) → A` with `p i0 = a` and `p i1 = b` (definitionally)
- Composition operation `hcomp`: fills open boxes of paths
- Transport `transp`: transport along a path
- Kan operations: the cubical analogue of Kan filling in simplicial sets

### CCHM Cubical Type Theory (Cohen–Coquand–Huber–Mörtberg)
- The original computational cubical type theory
- De Morgan algebra of cofibrations: `φ ∨ ψ`, `φ ∧ ψ`, `~ φ`
- Partial elements and systems
- Univalence as a *theorem* (not axiom): proved using the Glue type
- The Glue type: an extension of the universe for the univalence proof
- HITs defined constructively: `hcomp` fills their paths

### Cartesian Cubical Type Theory (ABCFHL)
- Angiuli–Brunerie–Coquand–Favonia–Harper–Licata
- Uses a different cube category: Cartesian rather than De Morgan
- Numerically simpler composition and filling
- Closer to the computational content originally envisioned by Licata and Harper

### XTT and Strict Equality
- Sterling–Angiuli–Gratzer: adding a *strict* equality type alongside the path type
- Allows both computational and proof-theoretic equality
- Resolves some problems with abstraction in CCHM

**Primary texts:**
- Cohen, Coquand, Huber & Mörtberg, *Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom* (arXiv 2016) — the CCHM paper; read carefully
- Angiuli et al., *Cartesian Cubical Computational Type Theory* (LICS 2018)
- Huber, *Canonicity for Cubical Type Theory* (LMCS)
- Vezzosi, Mörtberg & Abel, *Cubical Agda: A Dependently Typed Programming Language with Univalence and Higher Inductive Types* (ICFP 2019)

**Proof assistant exercises (Cubical Agda):**
- Prove function extensionality using path types (without postulate)
- Prove univalence from Glue types
- Define the circle S¹ as a HIT and compute `π₁(S¹) = ℤ`
- Prove the Seifert–van Kampen theorem in Cubical Agda

**Milestone:** Prove univalence in Cubical Agda from the Glue type, understanding each step of the construction.

---

## 7.2 Simplicial Type Theory

**Core concepts:**
- Riehl–Shulman, *A Type Theory for Synthetic ∞-Categories* (arXiv 2017)
- Two interval types: the "cubical" interval I and the "simplicial" interval 2
- Extension types: `⟨(t : A | φ) → B t⟩`
- Directed paths and functions (not necessarily invertible)
- The type `Hom_A(a, b)`: morphisms in a synthetic ∞-category
- Rezk types: the synthetic notion of (∞,1)-categories
- Synthetic ∞-groupoids vs. Rezk types: the distinction between HoTT and Simplicial TT
- The Segal condition and Rezk completeness
- The Yoneda lemma in this context

**Primary texts:**
- Riehl & Shulman, *A Type Theory for Synthetic ∞-Categories* (arXiv 1705.07442)
- Gratzer, Weinberger & Buchholtz, recent work on simplicial TT
- Bardomiano-Martínez, formalization in Rzk (the proof assistant for this TT)

**Proof assistant:** Rzk (rzk-lang.org) — the dedicated proof assistant for simplicial/synthetic ∞-category theory

**Key exercises:**
- Install Rzk and formalize the Yoneda lemma in synthetic ∞-category theory
- Prove that Rezk types are closed under (homotopy) limits

---

## 7.3 Modal HoTT and Cohesive Type Theory

**Core concepts:**
- Lawvere's concept of cohesion: a string of adjoint functors `Π ⊣ Δ ⊣ Γ ⊣ coDisc` between a cohesive topos and Set
- Shulman's Real Cohesion: adding a "real-cohesion" modality to HoTT
- The sharp `♯` and flat `♭` modalities
- Differential cohesion: de Rham cohomology synthetically
- Schreiber's Differential cohomology in a cohesive ∞-topos
- Axiomatic synthetic differential geometry in HoTT

**Primary texts:**
- Shulman, *Brouwer's Fixed-Point Theorem in Real-Cohesive Homotopy Type Theory* (arXiv)
- Shulman, *Cohesive Homotopy Type Theory* (slides and notes)
- Licata & Shulman, *Adjoint Logic with a 2-Category of Modes*

**Why this matters:** This connects HoTT to physics (gauge field theory), differential geometry, and algebraic geometry in a synthetic way.

---

## 7.4 Two-Level Type Theory (2LTT)

**Core concepts:**
- A meta-type theory with two levels: an "outer" strict level and an "inner" HoTT level
- Allows reasoning about the model of HoTT without leaving the type-theoretic framework
- Angiuli–Gratzer–Nuyts, *Multimodal Dependent Type Theory* (LICS 2021)
- Applications: proving properties of the univalence axiom, internal parametricity

**Primary texts:**
- Altenkirch et al., *Setoid Type Theory — A Syntactic Translation*
- Capriotti & Kraus, *Univalent Higher Categories via Complete Semi-Segal Types*
- Nuyts et al., recent work on multimodal TT

---

# PHASE 8 — Research Frontiers
### Duration: Ongoing

**What this phase is:** Contributing original work. Choose one or more threads and dig in.

---

## 8.1 Open Problems in HoTT

**Computational:**
- A fully computational model of CCHM cubical type theory in a proof assistant (ongoing in Agda)
- Canonicity and normalization for full CCHM (with all HITs)
- A practical proof assistant for simplicial/directed type theory

**Homotopy-theoretic:**
- Computing homotopy groups of spheres in HoTT (Brunerie's `n` — verified as 2 but the proof term is massive; a cleaner proof is wanted)
- The chromatic tower in synthetic homotopy theory
- Synthetic spectra and stable homotopy theory in HoTT

**Categorical:**
- A complete theory of (∞,1)-categories in Simplicial TT (Rezk completeness, adjunctions, limits)
- Synthetic algebraic K-theory
- A type-theoretic account of ∞-toposes

**Foundational:**
- The consistency strength of HoTT relative to ZFC variants
- The relation between HoTT and ZFC with large cardinal axioms
- A constructive proof of the small object argument

---

## 8.2 Applications to Programming Language Theory

**Research threads:**
- **Parametricity in HoTT:** Internal parametricity (Cavallo–Harper, Nuyts), applications to free theorems
- **Observational type theory (OTT):** Altenkirch–McBride approach to avoiding axioms
- **Quotient types and setoids:** When do you need full univalence vs. just quotient types?
- **Proof-relevant rewriting:** Rewriting systems where the rewrite path matters
- **Homotopical program verification:** Using path types to state and prove equivalences of programs

**Primary texts:**
- Cavallo & Harper, *Internal Parametricity for Cubical Type Theory*
- Altenkirch, Kaposi & Kovács, *Normalisation by Evaluation for Type Theory, in Type Theory*
- Sterling, *Algebraic Type Theory and Universe Hierarchies*

---

## 8.3 Applications to Foundations of Mathematics

**Research threads:**
- **Univalent mathematics:** Rebuilding undergraduate and graduate mathematics on univalent foundations
- **Homotopy-theoretic set theory:** Understanding ZFC inside HoTT (Gylterud, Frumin)
- **The constructive status of classical results:** Which theorems need LEM? Which need choice? Which need univalence?
- **Large cardinal axioms in HoTT:** How do inaccessible cardinals, Mahlo cardinals, etc. look in type theory?

**Primary texts:**
- UniMath project: github.com/UniMath/UniMath
- Gylterud, *From Multisets to Sets in Homotopy Type Theory*
- Escardó's work on searchable types and omniscience principles
- Coquand et al., *Canonicity and Normalisation for Dependent Type Theory*

---

## 8.4 Connections to Algebraic Geometry and Physics

**Research threads:**
- **Condensed mathematics (Clausen–Scholze):** Is there a type-theoretic approach?
- **Motivic homotopy type theory:** A synthetic treatment of A¹-homotopy theory
- **Gauge theory in HoTT:** Schreiber's program, differential cohomology
- **Topological quantum field theory (TQFT) synthetically:** Using ∞-categories in HoTT

**Primary texts:**
- Schreiber & Shulman, *Quantum Gauge Field Theory in Cohesive Homotopy Type Theory*
- Myers, *String Diagrams for Double Categories and Equipments*
- Cherubini & Rijke, *Modal Descent*

---

## 8.5 Building Proof Automation

**Research threads:**
- **Tactics for HoTT:** Automation of path-induction-heavy proofs
- **Decision procedures:** Decidability of equality in specific HITs
- **Reflection and metaprogramming:** Using Lean 4 / Agda reflection to automate HoTT proofs
- **Certified type checkers:** Implementing a type checker for CCHM in Lean 4

**Projects:**
- Implement a tactic that automatically applies path induction and handles the coherence obligations
- Write a decision procedure for equality in free groups (viewed as a HIT)
- Implement a small cubical type checker verified in Lean 4

---

# REFERENCE: The Essential Library

## Canonical Texts (read cover to cover)
1. **HoTT Book** — homotopytypetheory.org (free)
2. **Riehl, *Category Theory in Context*** (free)
3. **Hatcher, *Algebraic Topology*** (free)
4. **PLFA** (Agda, free) — plfa.github.io
5. **Theorem Proving in Lean 4** (free) — leanprover.github.io
6. **Sørensen & Urzyczyn, *Lectures on the Curry-Howard Isomorphism***

## Essential Papers (read and work through)
1. Voevodsky, *An Experimental Library of Formalized Mathematics based on Univalent Foundations* (2015)
2. Cohen–Coquand–Huber–Mörtberg, *Cubical Type Theory* (2016)
3. Riehl–Shulman, *A Type Theory for Synthetic ∞-Categories* (2017)
4. Brunerie, *On the Homotopy Groups of Spheres in HoTT* (PhD thesis, 2016)
5. Shulman, *Univalence for Inverse Diagrams and Homotopy Canonicity* (2015)
6. Licata–Harper, *2-Dimensional Directed Type Theory* (2011)
7. Awodey–Warren, *Homotopy-Theoretic Models of Identity Types* (2009) — the paper that started it all (alongside Voevodsky's notes)

## Key Proof Assistant Libraries
- **Lean 4 / Mathlib4:** github.com/leanprover-community/mathlib4
- **Cubical Agda library:** github.com/agda/cubical
- **HoTT-Agda:** github.com/HoTT/HoTT-Agda
- **UniMath:** github.com/UniMath/UniMath
- **Rzk:** rzk-lang.org

## Key Communities and Conferences
- HoTT/UF workshop (co-located with FSCD and LICS)
- TYPES conference
- LICS (Logic in Computer Science)
- ICFP (functional programming, types)
- Homotopy Type Theory Zulip: hott.zulipchat.com
- nLab: ncatlab.org — wiki for higher structures; an invaluable reference

---

# LEARNING STRATEGY

## The Spiral Method
Visit each topic at increasing depth:
1. **First pass:** Read the primary text, do 30% of exercises, write summary notes
2. **Second pass:** Formalize in a proof assistant
3. **Third pass:** Read primary research papers; find what you do not yet understand
4. **Fourth pass:** Contribute (fix gaps in libraries, formalize new results)

## Formalize Everything
The gap between "understanding a proof" and "being able to formalize it" is enormous and deeply instructive. Do not skip formalization at any phase. When a proof is hard to formalize, that difficulty is mathematical information.

## Keep a Research Journal
One entry per session: what you read, what confused you, what connected to something else. The connections between phases are where the real understanding lives.

## Join the Community Early
Post on the HoTT Zulip (hott.zulipchat.com) from Phase 5 onward. Ask questions. Read discussions. The field is small and welcoming. Attend a HoTT/UF workshop as soon as you have Phase 5 complete.

## Resist Premature Formalization
Do not spend Phases 0–4 trying to formalize everything in a proof assistant. Build mathematical intuition first. Formalization without intuition produces mechanical symbol manipulation.

## The Goal Is Judgment
The final aim is not to memorize this curriculum but to develop mathematical judgment: the ability to look at a new problem in foundations and know which tools apply, what the obstacles are likely to be, and where to look for related work.

---

*"The cathedral is not built in a day. But each stone, set rightly, bears the weight of every stone above it."*
