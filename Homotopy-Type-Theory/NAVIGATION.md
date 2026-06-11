# How to Use This Book

This repository contains a complete curriculum in Homotopy Type Theory, organized in two parallel structures. Understanding how they relate is the first step to using the material effectively.

---

## The Two-Path Structure

### book/ — Narrative-First Path

The `book/` directory contains 9 units organized as a narrative-first curriculum. Each unit contains multiple chapters, each chapter multiple sections. The prose is expository and conceptually motivated: it builds intuition, explains the "why" before the "what," and uses worked examples to illustrate abstract ideas.

**Use `book/` if:**
- You are new to type theory or homotopy theory
- You want to understand the conceptual motivation behind formal definitions
- You are a philosopher or computer scientist encountering HoTT for the first time
- You want to follow a linear path from foundations to research frontiers

The book/ path follows the curriculum in `curriculum.md` closely: Phase 0 → Phase 8. It is designed for a student who reads the chapters in order.

### chapters/ — Formal-First Path

The `chapters/` directory contains 27 chapters organized by formal content. Each chapter is self-contained: it states definitions precisely, introduces notation systematically, and provides exercises. The chapters are more compressed than the book units but more formally precise.

**Use `chapters/` if:**
- You have a background in logic, type theory, category theory, or algebraic topology
- You want to learn the formal apparatus quickly without extended motivational prose
- You need a reference for a specific concept (e.g., the J rule, the univalence axiom, cubical type theory)
- You are working through formal proofs and need the definitions close at hand

**The key asymmetry:** `book/` gives you the why; `chapters/` gives you the what. Neither is complete without the other. Graduate students will find themselves switching between the two regularly.

---

## Cross-Reference Table

The following table maps each of the 9 book units to the formal chapters that correspond to or extend its content, with a brief description of the connection.

| Book Unit | Book Path | Formal Chapters | Description |
|---|---|---|---|
| Unit 1: Mathematical Foundations | `book/unit-01-mathematical-foundations/` | ch00–ch03 | Logic and proof, set theory, abstract algebra, real analysis. The book unit provides worked narrative examples; the chapters give formal definitions and exercises. |
| Unit 2: Logic and Computation | `book/unit-02-logic-and-computation/` | ch04–ch07 | Proof theory, intuitionistic logic, Curry-Howard correspondence, STLC and System F. The book unit emphasizes the BHK interpretation and the propositions-as-types picture; the chapters formalize the type systems. |
| Unit 3: Dependent Types | `book/unit-03-dependent-types/` | ch08–ch09 | Dependent type theory and Martin-Löf Type Theory (MLTT). The book unit builds intuition for Π-types and Σ-types; the chapters state the full FIEC rules and work through identity type examples. |
| Unit 4: Category Theory | `book/unit-04-category-theory/` | ch10–ch12 | Category theory, categorical logic, higher categories. The book unit emphasizes the Yoneda lemma and adjunctions; the chapters cover fibered categories, contextual categories, and the correspondence between LCCCs and dependent type theory. |
| Unit 5: Topology | `book/unit-05-topology/` | ch13–ch15 | Point-set topology, homotopy theory, simplicial sets. The book unit develops the fundamental group and covering spaces; the chapters formalize the simplicial model that underlies HoTT's semantics. |
| Unit 6: Core HoTT | `book/unit-06-core-hott/` | ch16–ch20 | Identity types, h-levels, univalence, higher inductive types, synthetic homotopy. This is the primary content of HoTT. The book unit gives extended conceptual development; the chapters state the formal rules precisely. |
| Unit 7: Proof Assistants | `book/unit-07-proof-assistants/` | ch21–ch22 | Lean 4 (Mathlib) and Cubical Agda. The book unit provides tutorials and project descriptions; the chapters contain formalization exercises and reference material for both systems. |
| Unit 8: Advanced Foundations | `book/unit-08-advanced-foundations/` | ch23–ch25 | Cubical type theory, simplicial type theory, modal HoTT. The book unit covers CCHM cubical TT and the Glue type; the chapters formalize these and connect to Cartesian cubical TT and cohesive HoTT. |
| Unit 9: Research Frontiers | `book/unit-09-research-frontiers/` | ch26 | Open problems, current research directions, connections to algebraic geometry and physics. The book unit surveys the landscape; ch26 provides entry points to the literature. |

---

## Detailed Chapter Mapping

### Unit 1 ↔ Chapters 00–03

`book/unit-01-mathematical-foundations/ch00-logic-and-proof/` provides the narrative introduction to logic and proof-writing that ch00-logic-and-proof formalizes. Use the book chapter to build intuition; use the formal chapter for the exercises and the precise statement of propositional and predicate logic.

`ch01-set-theory` through `ch03-real-analysis` are prerequisites that provide mathematical maturity. The book versions are more discursive; the formal chapters are closer to textbook chapters in their respective subjects.

**Recommendation:** If you have a mathematics background, read the formal chapters quickly for reference. If you are coming from philosophy or computer science, read the book chapters carefully first.

### Unit 2 ↔ Chapters 04–07

`book/unit-02-logic-and-computation/` covers the material in four formal chapters:
- `ch04-proof-theory`: natural deduction, sequent calculus, normalization
- `ch05-intuitionistic-logic`: BHK interpretation, Kripke semantics, IPC
- `ch06-curry-howard`: the isomorphism in detail; Π-types as ∀, Σ-types as ∃
- `ch07-stlc-system-f`: STLC and System F; strong normalization; parametricity

The demos directory has `demo_bhk.py`, `demo_curry_howard.py`, and `demo_proof_theory.py` that can be run to see these concepts illustrated computationally.

**Recommendation:** Run `demo_curry_howard.py` after reading `ch06-curry-howard`. The visual representation of the proof-program correspondence makes the abstract correspondence concrete.

### Unit 3 ↔ Chapters 08–09

This is the single most important unit-chapter correspondence in the curriculum.

- `ch08-dependent-types`: Π-types, Σ-types, universes, inductive types. This is where the formal machinery of dependent type theory is stated precisely.
- `ch09-mltt`: Martin-Löf Type Theory in full — FIEC rules, the J eliminator, definitional vs. propositional equality, W-types.

The book unit `book/unit-03-dependent-types/` develops the same material with extended motivation. Section `ch08-dependent-types/` in the book gives the crucial intuition for why Π-types are more than function types — they express mathematical statements.

**The critical exercise:** After reading `ch09-mltt`, work through the identity type example end-to-end:
1. Form `Id_A(a, b)` for a type A and terms a, b : A.
2. Construct `refl_a : Id_A(a, a)`.
3. Apply the J eliminator to derive symmetry: from `p : Id_A(a, b)` derive `p⁻¹ : Id_A(b, a)`.
4. Apply J again to derive transitivity.
This exercise is the bridge from Unit 3 to Unit 6.

### Unit 4 ↔ Chapters 10–12

- `ch10-category-theory`: basic category theory through the Yoneda lemma and adjunctions
- `ch11-categorical-logic`: CCCs, LCCCs, toposes, fibered categories, the LCCC–dependent type theory correspondence
- `ch12-higher-categories`: 2-categories, bicategories, groupoids, the homotopy hypothesis, quasi-categories

The `demos/demo_categories.py` and `demos/demo_categorical_logic.py` files illustrate basic categorical constructions. The category theory demos are Python representations of abstract concepts and should be treated as intuition pumps, not formal proofs.

**Why this unit is mandatory for understanding HoTT deeply:** The univalence axiom says `(A = B) ≃ (A ≃ B)`. This is a statement about the ∞-groupoid structure of the universe. Understanding what an ∞-groupoid is (ch12) and what the fibered structure of the universe looks like (ch11) is required to understand why univalence is the right statement.

### Unit 5 ↔ Chapters 13–15

- `ch13-topology`: point-set topology through quotient spaces and compactness
- `ch14-homotopy-theory`: homotopy groups, covering spaces, van Kampen's theorem, Eilenberg-MacLane spaces
- `ch15-simplicial-sets`: the category sSet, Kan complexes, geometric realization, the Quillen model structure

The `demos/demo_simplicial_sets.py` and `demos/demo_covering_spaces.py` files are worth running for intuition.

**Why simplicial sets matter for HoTT:** Voevodsky's original model of HoTT lives in Kan simplicial sets. The identity type `a =_A b` is interpreted as the path space — the Kan complex of 1-simplices from a to b. The univalence axiom corresponds to the fact that, in the model, the canonical map from the universe to the space of equivalences is a Kan fibration with contractible fibers. Reading ch15 before ch18 makes the univalence axiom feel inevitable rather than surprising.

### Unit 6 ↔ Chapters 16–20

This is the core of the book. The correspondence is tight:

- `ch16-identity-types` ↔ `book/unit-06-core-hott/ch16-identity-types/`: The identity type as path space, J rule, groupoid laws, transport, ap, function extensionality.
- `ch17-h-levels` ↔ `book/unit-06-core-hott/ch17-h-levels/`: The h-level hierarchy (contractible → prop → set → groupoid → ...), truncations, and the coherence tower.
- `ch18-univalence` ↔ `book/unit-06-core-hott/ch18-univalence/`: The univalence axiom `(A = B) ≃ (A ≃ B)`, its consequences (funext, propext), and the Glue type construction.
- `ch19-higher-inductive-types` ↔ `book/unit-06-core-hott/ch19-higher-inductive-types/`: HITs (interval, circle, suspension, pushouts, truncations), and the encode-decode method for `π₁(S¹) = ℤ`.
- `ch20-synthetic-homotopy` ↔ `book/unit-06-core-hott/ch20-synthetic-homotopy/`: Freudenthal suspension theorem, Blakers-Massey, the Hopf fibration.

The `demos/` directory has the richest collection for this unit:
- `demo_paths.py`, `demo_groupoid_laws.py`, `demo_transport.py`: identity types
- `demo_hlevels.py`, `demo_truncations.py`: h-levels
- `demo_equiv.py`, `demo_univalence_deep.py`, `demo_funext.py`: univalence
- `demo_circle.py`, `demo_encode_decode.py`, `demo_suspension.py`: HITs
- `demo_hopf.py`, `demo_james.py`: synthetic homotopy

**Recommendation:** For each chapter in Unit 6, read the book version first for motivation, then the formal chapter for definitions, then run the corresponding demo. The tripartite rhythm — motivation, formalization, computation — is the most effective way to internalize this material.

### Unit 7 ↔ Chapters 21–22

- `ch21-lean4`: Lean 4 with Mathlib — tactics, type class inference, the CategoryTheory library, HoTT-adjacent formalizations
- `ch22-cubical-agda`: Agda with `--cubical` — the interval type, path types, hcomp, transp, the Cubical Agda library

The `demos/demo_lean4.py` and `demos/demo_cubical_agda.py` are illustrative; actual proof assistant work requires installing Lean 4 or Agda.

**Critical tools:**
- Lean 4: install from `leanprover.github.io`; Mathlib4 provides the standard library.
- Cubical Agda: install from `agda.readthedocs.io`; run with `--cubical` flag; the Cubical library is at `github.com/agda/cubical`.

### Unit 8 ↔ Chapters 23–25

- `ch23-cubical-type-theory`: CCHM cubical type theory — the interval, de Morgan algebra of cofibrations, composition/transport, the Glue type, univalence as a theorem.
- `ch24-simplicial-type-theory`: Riehl-Shulman simplicial TT — the simplicial interval, extension types, Rezk types, synthetic ∞-category theory.
- `ch25-modal-hott`: Cohesive HoTT — the flat `♭` and sharp `♯` modalities, Real Cohesion (Shulman), differential cohomology synthetically.

The `demos/demo_cubical.py` and `demos/demo_simplicial_hott.py` provide computational illustrations. The `demos/demo_modal_hott.py` covers the modal setting.

### Unit 9 ↔ Chapter 26

- `ch26-research-frontiers`: Open problems in computational HoTT, synthetic homotopy, categorical logic, and connections to algebraic geometry.

The book unit provides narrative context; ch26 provides entry points into the literature. This is the least complete chapter, reflecting the fact that these are open research areas.

---

## Three Reading Tracks

### Track 1: Foundations Track
**Background:** Philosophical logic, proof theory, some mathematics

**Best entry point:** `book/unit-01-mathematical-foundations/` and `book/unit-02-logic-and-computation/`

**Path through the material:**
1. **Months 1–2:** `book/unit-01` (ch00–ch03) — mathematical maturity. Focus on ch00 (logic and proof) and ch02 (algebra). Ch01 (set theory) and ch03 (analysis) can be skimmed if you have the background.
2. **Months 3–4:** `book/unit-02` (ch04–ch07) — proof theory, intuitionistic logic, Curry-Howard. Read ch04 (proof theory) and ch05 (ILC) carefully; ch06 (Curry-Howard) is the conceptual core of this unit for this track.
3. **Months 5–6:** `book/unit-03` (ch08–ch09) — dependent types and MLTT. Ch09 (MLTT) is the most important chapter for this track: the J eliminator is the formal expression of the philosophical claim that all properties of equality derive from reflexivity.
4. **Months 7–8:** `book/unit-06` (ch16–ch18) — core HoTT. Skip unit-05 (topology) on the first pass and come back to it after grasping the main ideas of HoTT. Focus on ch16 (identity types) and ch18 (univalence).
5. **Month 9:** Return to `ch13–ch15` (topology and simplicial sets) with new perspective — now you can see what the classical topology is the model of.
6. **Months 10–12:** `ch19–ch20` (HITs and synthetic homotopy) and `ch21–ch22` (proof assistants).

**Core texts from curriculum.md to prioritize:**
- van Dalen, *Logic and Structure* (intuitionistic logic)
- Hofmann, *Syntax and Semantics of Dependent Types* (MLTT)
- HoTT Book, Chapters 1–4 (core HoTT)

**Philosophical notes:** The identity type (`a =_A b`) is the type-theoretic analog of the classical notion of identity. The J rule is the analog of Leibniz's law (substitution of equals). Univalence (`(A = B) ≃ (A ≃ B)`) is the formal content of the philosophical idea that things that are equivalent in all structural respects are the same. Read ch16 alongside Williamson's *Identity* or Quine's *Word and Object* §24 for philosophical context.

---

### Track 2: Computer Science Track
**Background:** Programming languages, type theory, functional programming

**Best entry point:** `book/unit-03-dependent-types/` and `chapters/ch08-dependent-types`

**Path through the material:**
1. **Month 1:** `ch06-curry-howard` and `ch07-stlc-system-f` — you know these, so skim quickly. Run `demo_curry_howard.py` to get the repo's notation.
2. **Months 2–3:** `ch08-dependent-types` and `ch09-mltt` — the core formal apparatus. Focus on the J eliminator (§2.3 of curriculum.md) and its computation rule.
3. **Months 4–5:** Start Agda or Lean 4 with `ch21-lean4` / `ch22-cubical-agda`. Begin formalizing. The gap between understanding MLTT informally and formalizing it is enormous and epistemically important.
4. **Months 6–7:** `ch16-identity-types` through `ch18-univalence`. These are where type theory diverges most sharply from what you know. Focus on the groupoid laws (ch16), the h-level hierarchy (ch17), and the univalence axiom (ch18).
5. **Month 8:** `ch19-higher-inductive-types` — the encode-decode proof of `π₁(S¹) = ℤ` is the canonical benchmark. Formalize it in Cubical Agda.
6. **Months 9–10:** `ch23-cubical-type-theory` — CCHM cubical TT addresses the canonicity problem that prevents axiomatic HoTT from having a computational interpretation. If you care about the "programs" side of Curry-Howard, this is essential.
7. **Months 11–12:** `ch24-simplicial-type-theory` and `ch26-research-frontiers` — directed type theory (morphisms that are not invertible) and current research.

**Key PL connection:** In HoTT, `transport P p : P a → P b` for `p : a = b` is the type-theoretic analog of coercion or cast in a programming language, but with the crucial difference that the path `p` is a first-class term, not a proof obligation discharged behind the scenes. This has implications for proof-relevant rewriting, quotient types, and the semantics of program equivalence.

**Proof assistant benchmark:** By Month 8, you should be able to formalize the following in Cubical Agda:
```agda
π₁S¹≡ℤ : π₁ S¹ ≡ ℤ
```
This requires: defining S¹ as a HIT, defining the cover `ℤ`, defining the winding number map and its inverse, and proving they are mutual inverses by path induction.

---

### Track 3: Mathematics Track
**Background:** Algebraic topology, graduate-level abstract algebra

**Best entry point:** `book/unit-05-topology/` and `chapters/ch14-homotopy-theory`

**Path through the material:**
1. **Month 1:** `ch13-topology` and `ch14-homotopy-theory` — you know this material; use it to calibrate the notation and see what the synthetic version will reproduce.
2. **Month 2:** `ch15-simplicial-sets` — Kan complexes are the semantic objects; this chapter is essential for understanding why the type theory is the way it is.
3. **Months 3–4:** `ch08-dependent-types` and `ch09-mltt` — the formal apparatus. Focus on the J rule as the type-theoretic analog of the contractibility of the based path fibration.
4. **Months 5–6:** `ch10-category-theory` through `ch12-higher-categories` — you know the first two; focus on ch12 (higher categories) and the homotopy hypothesis: ∞-groupoids ≃ homotopy types.
5. **Months 7–8:** `ch16-identity-types` through `ch20-synthetic-homotopy` — the main payoff. Reading ch16 as a topologist: `refl_a` is the constant path; J is contractibility of the total path space; `transport^P(p)` is parallel transport along a path in a fibration over A.
6. **Month 9:** Formalize `π₁(S¹) = ℤ` (ch19). Compare the HoTT proof to the classical proof using covering spaces (ch14).
7. **Months 10–12:** `ch23-cubical-type-theory` and `ch24-simplicial-type-theory`. The latter is particularly relevant for mathematicians interested in ∞-categories: Riehl-Shulman's simplicial TT gives a synthetic framework for (∞,1)-category theory in which the Yoneda lemma and adjoint functor theorems can be proved formally.

**Key mathematical insight:** The h-level hierarchy in ch17 corresponds exactly to the Postnikov tower. A type at h-level n is an n-truncated homotopy type; the n-truncation `‖A‖ₙ` is the nth Postnikov section. The `isProp A` condition says A has trivial fundamental groupoid; `isSet A` says A is a 0-type (discrete space). Univalence says that `Type` is not a set — it has nontrivial path structure, corresponding to the fact that equivalences of spaces form a non-discrete ∞-groupoid.

**Benchmark:** By Month 9, you should be able to state and sketch the proof (in either HoTT or classical terms) of:
- The Seifert-van Kampen theorem synthetically (ch20)
- The Freudenthal suspension theorem (ch20)
- The Hopf fibration `S¹ → S³ → S²` as a HIT construction (ch20)

---

## Which Path Am I On? A Quick Diagnostic

Answer these questions honestly:

1. Can you state and prove the cut-elimination theorem for propositional sequent calculus? → If yes, you have the background for Track 1 or Track 2; if no, start with `book/unit-01`.

2. Can you implement a type-checker for STLC in Haskell or OCaml? → If yes, Track 2 is natural. Start at `ch08`.

3. Can you compute `π₁(S¹)` using van Kampen's theorem and explain what a universal cover is? → If yes, Track 3 is natural. Start at `ch15`.

4. None of the above? → Start at `book/unit-01-mathematical-foundations/` and follow the book/ path from beginning to end. Read the `curriculum.md` to understand the overall structure. The book/ path is designed for exactly this situation.

---

## A Note on Notation

This book uses the following conventions throughout:
- Π-types (dependent function types): `Π(x : A), B x` or `(x : A) → B x`
- Σ-types (dependent pair types): `Σ(x : A), B x` or `Σ x : A, B x`
- Identity types: `a =_A b` or `Id_A(a, b)` or `a ≡_A b`
- The J eliminator: `J(C, d, b, p) : C(b, p)` with computation `J(C, d, a, refl_a) ≡ d`
- Univalence: `ua : (A ≃ B) → (A = B)` with inverse `idtoeqv : (A = B) → (A ≃ B)`
- Function extensionality: `funext : (f ~ g) → f = g` where `f ~ g := Π(x : A), f x = g x`
- Propositional truncation: `‖A‖` with constructor `|–| : A → ‖A‖` and eliminator requiring a proposition as target
- h-levels: `isContr A`, `isProp A`, `isSet A`, `isGroupoid A`

The `chapters/` directory uses these notations consistently. The `book/` directory sometimes uses informal equivalents in prose — "a path from a to b" instead of `p : a =_A b`, "a type is a set" instead of `isSet A` — before introducing the formal notation.

---

## Quick Navigation by Topic

| Topic | Primary location | Secondary |
|---|---|---|
| J rule (path induction) | ch09-mltt, ch16-identity-types | book/unit-03, book/unit-06/ch16 |
| Univalence axiom | ch18-univalence | book/unit-06/ch18, demo_univalence_deep.py |
| Function extensionality | ch18-univalence §3, ch16 §5 | demo_funext.py |
| H-levels and truncations | ch17-h-levels | book/unit-06/ch17, demo_hlevels.py, demo_truncations.py |
| Higher inductive types | ch19-higher-inductive-types | book/unit-06/ch19, demo_circle.py |
| π₁(S¹) = ℤ | ch19 §encode-decode | demo_encode_decode.py |
| Cubical type theory | ch23-cubical-type-theory | book/unit-08, demo_cubical.py |
| Synthetic ∞-categories | ch24-simplicial-type-theory | demo_simplicial_hott.py |
| Lean 4 formalization | ch21-lean4 | demo_lean4.py |
| Cubical Agda | ch22-cubical-agda | demo_cubical_agda.py |
| Categorical semantics | ch11-categorical-logic | demo_categorical_logic.py |
| Homotopy groups | ch14-homotopy-theory, ch20 | demo_fundamental_theorem.py |
| Hopf fibration | ch20-synthetic-homotopy | demo_hopf.py |
| Open problems | ch26-research-frontiers | demo_research_frontiers.py |
