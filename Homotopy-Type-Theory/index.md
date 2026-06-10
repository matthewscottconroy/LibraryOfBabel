# Homotopy Type Theory: A Textbook in 27 Chapters

## Master Index and Navigation Guide

This textbook covers the full path from mathematical foundations to research-level homotopy type theory. It accompanies the curriculum document (`curriculum.md`) and is organized into eight phases, each building on the previous.

Every chapter contains: definitions, theorems with proofs, worked examples, Agda or Lean code, and exercises (including at least one research-level problem).

---

## Phase 0 — Mathematical Foundations

These chapters develop the prerequisite mathematical maturity for type theory. They can be read quickly by mathematicians or computer scientists with relevant background.

| Chapter | Title | Key Topics |
|---------|-------|------------|
| [Ch. 00](chapters/ch00-logic-and-proof.md) | Logic and Proof | Propositional and predicate logic, proof techniques, induction |
| [Ch. 01](chapters/ch01-set-theory.md) | Set Theory | ZFC axioms, ordinals, cardinals, axiom of choice |
| [Ch. 02](chapters/ch02-abstract-algebra.md) | Abstract Algebra | Groups, isomorphism theorems, free groups, group presentations |
| [Ch. 03](chapters/ch03-real-analysis.md) | Real Analysis | Metric spaces, completeness, compactness, connectedness |

**Milestone:** Prove 20 rigorous mathematical theorems across logic, set theory, and algebra.

---

## Phase 1 — Logic and Computation

The theoretical foundations of type theory: proof theory, constructivism, and the Curry-Howard correspondence.

| Chapter | Title | Key Topics |
|---------|-------|------------|
| [Ch. 04](chapters/ch04-proof-theory.md) | Proof Theory | Natural deduction, normalization, sequent calculus, cut elimination |
| [Ch. 05](chapters/ch05-intuitionistic-logic.md) | Intuitionistic Logic | BHK interpretation, IPC, Kripke semantics, constructivism |
| [Ch. 06](chapters/ch06-curry-howard.md) | The Curry-Howard Correspondence | Propositions as types, proofs as programs, the central dictionary |
| [Ch. 07](chapters/ch07-stlc-system-f.md) | Simply Typed Lambda Calculus and System F | STLC, strong normalization, System F, parametricity |

**Milestone:** Prove strong normalization for STLC by logical relations. Prove a parametricity theorem.

---

## Phase 2 — Dependent Types and Martin-Löf Type Theory

The type-theoretic core: dependent types, universes, and MLTT.

| Chapter | Title | Key Topics |
|---------|-------|------------|
| [Ch. 08](chapters/ch08-dependent-types.md) | Dependent Type Theory | Π types, Σ types, universes, inductive types |
| [Ch. 09](chapters/ch09-mltt.md) | Martin-Löf Type Theory | Four judgments, identity type, J eliminator, transport, UIP failure |

**Milestone:** Formalize the group axioms and prove uniqueness of identity/inverses in Lean 4. Prove J implies path induction.

---

## Phase 3 — Category Theory and Categorical Logic

The categorical perspective on logic and type theory.

| Chapter | Title | Key Topics |
|---------|-------|------------|
| [Ch. 10](chapters/ch10-category-theory.md) | Category Theory | Categories, functors, natural transformations, Yoneda, adjunctions |
| [Ch. 11](chapters/ch11-categorical-logic.md) | Categorical Logic | CCCs, LCCCs, fibrations, toposes, simplicial set model |
| [Ch. 12](chapters/ch12-higher-categories.md) | Higher Category Theory | Bicategories, groupoids, ∞-groupoids, quasi-categories, homotopy hypothesis |

**Milestone:** Prove the Yoneda lemma. Understand the Awodey-Warren theorem on groupoid models.

---

## Phase 4 — Topology and Homotopy Theory

The geometric content that HoTT captures.

| Chapter | Title | Key Topics |
|---------|-------|------------|
| [Ch. 13](chapters/ch13-topology.md) | Point-Set Topology | Topological spaces, continuity, separation axioms, compactness, CW complexes |
| [Ch. 14](chapters/ch14-homotopy-theory.md) | Algebraic Topology | Fundamental group, covering spaces, van Kampen, higher homotopy groups, fibrations |
| [Ch. 15](chapters/ch15-simplicial-sets.md) | Simplicial Sets | Simplex category, Kan complexes, model structure, Voevodsky's model |

**Milestone:** Prove π₁(S¹) = ℤ using the universal covering space. Understand the Kan-Quillen model structure.

---

## Phase 5 — Core Homotopy Type Theory

The mathematical heart of the subject.

| Chapter | Title | Key Topics |
|---------|-------|------------|
| [Ch. 16](chapters/ch16-identity-types.md) | Identity Types as Paths | Homotopy interpretation, J eliminator, groupoid laws, transport, higher paths |
| [Ch. 17](chapters/ch17-h-levels.md) | H-Levels and Truncations | Contractibility, h-props, h-sets, n-types, propositional truncation |
| [Ch. 18](chapters/ch18-univalence.md) | Equivalences and Univalence | Bi-invertible maps, univalence axiom, funext, structure invariance |
| [Ch. 19](chapters/ch19-higher-inductive-types.md) | Higher Inductive Types | Interval, circle, suspension, pushouts, truncations as HITs |
| [Ch. 20](chapters/ch20-synthetic-homotopy.md) | Synthetic Homotopy Theory | Encode-decode, π₁(S¹)=ℤ, van Kampen, Freudenthal, Hopf fibration, Brunerie |

**Milestone:** Prove π₁(S¹) = ℤ using the encode-decode method. Understand the Hopf fibration.

---

## Phase 6 — Proof Assistants

Practical formalization in modern proof assistants.

| Chapter | Title | Key Topics |
|---------|-------|------------|
| [Ch. 21](chapters/ch21-lean4.md) | Lean 4 and Mathlib | Tactic mode, Mathlib organization, CategoryTheory, metaprogramming |
| [Ch. 22](chapters/ch22-cubical-agda.md) | Cubical Agda | Interval type, path types as functions, Glue/univalence, the Cubical library |

**Milestone:** Formalize one HoTT theorem not in Mathlib. Contribute a pull request to the Cubical Agda library.

---

## Phase 7 — Advanced Foundations

The research frontier in type-theoretic foundations.

| Chapter | Title | Key Topics |
|---------|-------|------------|
| [Ch. 23](chapters/ch23-cubical-type-theory.md) | Cubical Type Theory | CCHM, interval, composition, Glue type, univalence as theorem, canonicity |
| [Ch. 24](chapters/ch24-simplicial-type-theory.md) | Simplicial Type Theory | Two intervals, Segal types, Rezk types, synthetic Yoneda, Rzk proof assistant |
| [Ch. 25](chapters/ch25-modal-hott.md) | Modal Homotopy Type Theory | Cohesion, flat/sharp modalities, de Rham cohomology, gauge theory |

**Milestone:** Prove funext from the cubical path definition in Cubical Agda. Read and summarize one of the three foundational papers (CCHM, Riehl-Shulman, or Shulman-Schreiber).

---

## Phase 8 — Research

The path to original contributions.

| Chapter | Title | Key Topics |
|---------|-------|------------|
| [Ch. 26](chapters/ch26-research-frontiers.md) | Research Frontiers | Open problems, formalization gaps, how to contribute, the longer view |

**Milestone:** Identify one open problem. Read two papers on it. Write a 5-page research proposal.

---

## Appendices

### Quick Reference: Key Theorems and Definitions

**Chapter 9 (MLTT):**
- J eliminator: given $C : \Pi_{x,y:A} (x = y) \to \mathsf{Type}$ and $d : \Pi_{x:A} C(x,x,\mathsf{refl})$, we have $J(C,d,p) : C(a,b,p)$ for any $p : a = b$.
- Transport: $\mathsf{tr}^B(p, b) : B(y)$ for $p : x = y$ and $b : B(x)$.

**Chapter 17 (H-Levels):**
- Contractible: $\mathsf{isContr}(A) :\equiv \Sigma_{a:A} \Pi_{x:A} a = x$
- H-prop: $\mathsf{isProp}(A) :\equiv \Pi_{x,y:A} x = y$
- H-set: $\mathsf{isSet}(A) :\equiv \Pi_{x,y:A} \mathsf{isProp}(x = y)$

**Chapter 18 (Univalence):**
- Univalence: $\mathsf{idToEquiv} : (A = B) \to (A \simeq B)$ is an equivalence.
- Consequence: $\mathsf{ua} : (A \simeq B) \to (A = B)$ with $\mathsf{transport}(\mathsf{ua}(e), a) = e(a)$.

**Chapter 19 (HITs):**
- Circle: $S^1 = \{\mathsf{base} : S^1,\ \mathsf{loop} : \mathsf{base} = \mathsf{base}\}$
- Pushout: $A \sqcup_C B = \{\mathsf{inl} : A \to A \sqcup_C B,\ \mathsf{inr} : B \to A \sqcup_C B,\ \mathsf{glue} : \mathsf{inl} \circ f \sim \mathsf{inr} \circ g\}$

### Reading Order for Different Backgrounds

**For mathematicians (algebraic topology background):**
Start at Ch. 06, skim Ch. 07-09, read Ch. 10-12 quickly, begin carefully at Ch. 16.

**For computer scientists (type theory background):**
Skim Ch. 00-03, read Ch. 04-09 carefully, skim Ch. 10-12, begin carefully at Ch. 13.

**For logicians:**
Read Ch. 00-06 carefully, skim Ch. 07-09, read Ch. 10-11 carefully, then Ch. 16 onward.

**For proof assistant users (Lean/Coq experience):**
Skim Ch. 00-09, read Ch. 10-20 carefully, begin formalization at Ch. 21.

### Key Proof Assistants and Libraries

| Tool | For | Library | Chapter |
|------|-----|---------|---------|
| Lean 4 | Classical mathematics, scale | Mathlib4 | Ch. 21 |
| Cubical Agda | HoTT, computational content | agda/cubical | Ch. 22 |
| Rzk | Simplicial/directed type theory | sHoTT | Ch. 24 |
| Coq/Rocq | HoTT, UniMath | UniMath, HoTT-Coq | — |

### Community Resources

- **HoTT Zulip:** hott.zulipchat.com (primary community)
- **HoTT Book:** homotopytypetheory.org/book (free PDF)
- **Cubical Agda Library:** github.com/agda/cubical
- **Mathlib4:** github.com/leanprover-community/mathlib4
- **Rzk:** rzk-lang.github.io
- **nLab:** ncatlab.org (comprehensive reference for categorical/HoTT concepts)
- **HoTTEST seminars:** homotopytype.theory/seminars

---

## A Note on the Curriculum

This textbook is designed to be read in order. Each chapter assumes the previous ones. The exercises range from routine verification (working through definitions) to research problems (open questions at the frontier).

The research problems are marked explicitly. They are not harder versions of the routine problems — they are genuinely open questions where the right answer is not known. This is intentional: the point of a research-level curriculum is to develop the capacity to work at the frontier, not just to reproduce settled results.

The cathedral is built from the foundation up. Every stone matters.
