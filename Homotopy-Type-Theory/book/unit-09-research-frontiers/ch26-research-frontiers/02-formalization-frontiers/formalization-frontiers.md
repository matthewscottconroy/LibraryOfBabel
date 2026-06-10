# 02 — Formalization Frontiers

## What It Means to Formalize

When mathematicians formalize a proof, they are not merely translating it from one notation to another. Formalization forces you to be explicit about things that mathematical prose permits you to leave vague: the exact types of variables, the precise form of induction, the specific lemma being applied, the universe level of each definition. These elisions are not flaws in mathematical writing — they are what makes it readable. But when a machine is checking the proof, every elision must be resolved.

The result of this resolution is almost always surprising. Proofs that seemed clear become tangled. Definitions that seemed straightforward reveal hidden dependencies. And occasionally, the process of making everything explicit exposes a gap — a step that was taken for granted but requires a non-trivial argument. Formalization is research: it discovers things about mathematics that informal mathematics does not see.

This is why the frontier of formalization is not just "has theorem X been machine-checked?" It is "what does formalizing X reveal about X?" The Brunerie proof revealed the Brunerie number. The formalization of the Hopf fibration in Cubical Agda revealed a clean decomposition of the fibration condition using pushout squares. The encode-decode proof of π₁(S¹) = ℤ revealed that the winding number is a fundamental concept, not just a convenient metaphor.

What follows is a survey of the active formalization projects and their current frontiers.

---

## The Cubical Agda Library

The primary repository: github.com/agda/cubical

The Cubical Agda library is the main HoTT formalization infrastructure. It is organized into modules covering the core type theory, standard mathematical objects, and increasingly sophisticated homotopy theory. Here is the current state as of 2025–2026.

**What is fully formalized:**
- The circle S¹ as a HIT, with π₁(S¹) = ℤ (encode-decode proof)
- The Hopf fibration η : S³ → S² (using the join construction)
- The Freudenthal suspension theorem (with the correct connectivity bounds)
- The Blakers-Massey theorem (Anel-Biedermann-Finster-Joyal, in the ∞-topos version)
- Pushouts and their universal properties
- Truncations and their recursion principles
- The Brunerie number computation: π₄(S³) = ℤ/2ℤ (Ljungström-Mörtberg 2023)
- Eilenberg-MacLane spaces K(G, n) for abelian groups G (up to n = 2 practically)
- Group cohomology via K(G, 1)
- The Seifert-van Kampen theorem

**Current gaps:**
- π_n(Sⁿ) = ℤ for all n: the base case n = 1 is done; the inductive step using Freudenthal requires a clean setup of the induction that has not been written
- π₅(S⁴): no formalization exists; this is the next open case
- Blakers-Massey sharpness: the bound is proved; examples achieving it are not formalized
- Long exact sequence of a fibration in full generality: partial
- Stable homotopy groups and spectra: definitions exist; systematic theory is absent
- Algebraic K-theory: essentially absent
- The cobordism hypothesis: not yet approachable

**How to find gaps.** Browse github.com/agda/cubical/issues with labels "wanted theorem," "enhancement," and "good first issue." The maintainers (primarily Anders Mörtberg, Evan Cavallo, and Axel Ljungström) actively maintain this list. It is the most accurate real-time map of what is needed.

**How to contribute to the Cubical Agda library.**
```
Step 1: Clone the repository.
  git clone https://github.com/agda/cubical
  cd cubical
  agda --cubical Cubical.Everything

Step 2: Find a theorem you understand well enough to attempt.
  Browse the issues. Read the surrounding library files. Identify what
  infrastructure exists and what is missing.

Step 3: State the target as an Agda type.
  Before writing any proof, write the type of the theorem you are proving.
  Check that it compiles (as a postulate or with a hole). This forces you
  to understand the exact statement.

Step 4: Fill in the proof.
  Use holes (?) liberally. Write the structure of the proof first, then fill
  in pieces. The Agda interaction mode will tell you the type of each hole
  and the context, which is invaluable for navigation.

Step 5: Submit a pull request.
  Follow the conventions in CONVENTIONS.md in the repository. The maintainers
  are welcoming to well-prepared contributions.
```

**Library conventions to know:**
- Files live in the appropriate subdirectory: `Cubical/Homotopy/` for homotopy theory, `Cubical/HITs/` for specific HITs, `Cubical/Algebra/` for group theory
- Naming follows mathematical convention: `ΩS¹≃ℤ` for the main theorem about π₁(S¹)
- Universe polymorphism is used throughout: pay attention to universe levels
- The library uses cubical primitives directly: `hcomp`, `transp`, `i0`, `i1`; understanding these is essential for any non-trivial contribution

---

## The Rzk Library and Simplicial Type Theory

The Rzk proof assistant implements simplicial type theory and is the vehicle for machine-verifying the Riehl-Shulman program. The main library is sHoTT (Synthetic Homotopy in Rzk's Type Theory):

Repository: github.com/rzk-lang/sHoTT

**What is formalized in sHoTT (as of 2025):**
- Segal types and their basic properties (composition, associativity up to equivalence)
- Rezk types and the Rezk completion
- The Yoneda lemma: the presheaf functor y : A → (A → U) is fully faithful (Kudasov-Riehl-Weinberger, CPP 2024)
- Adjoint functors between Segal types: unit, counit, triangle identities
- (Co)Cartesian fibrations and the Grothendieck construction (partial)

**Current gaps in sHoTT:**
- Limits and colimits in Segal types
- The Grothendieck construction (correspondence between left fibrations and functors into U): stated but not fully proved
- Stable ∞-categories and the stable Yoneda lemma
- Presheaf ∞-toposes
- Directed univalence (the central open problem; not formalized because not proved)

**How to contribute to sHoTT.** The Rzk syntax is designed to read like the Riehl-Shulman papers: the main contribution pathway is taking a theorem from the papers and translating it into Rzk. The paper "A Synthetic Theory of ∞-Categories" (arXiv:1705.07442) contains theorems with proofs; translating these proofs into Rzk syntax is the primary work. The Rzk documentation at rzk-lang.github.io explains the syntax in detail.

The community for sHoTT is smaller and more active than the Cubical Agda community — contributions are high-impact precisely because the library is young. Nikolai Kudasov (the primary Rzk developer), Jonathan Weinberger, and Emily Riehl are the main contacts.

---

## The UniMath Library

Repository: github.com/UniMath/UniMath

UniMath is the formalization library in Coq that Voevodsky initiated and that his collaborators have developed since his death. It is notable for its foundational purity: it is built on Book HoTT (not cubical), so all its theorems are proved from the univalent foundations axioms without cubical computation rules.

**What UniMath contains:**
- Foundations of HoTT: h-levels, equivalences, univalence (as an axiom), function extensionality
- Category theory: functors, natural transformations, adjunctions, limits, colimits
- Bicategories and displayed categories
- Algebra: groups, rings, modules
- Set theory: well-founded relations, ordinals
- Rezk completion (categorical construction)

**UniMath's distinctive feature.** Because UniMath uses Book HoTT (axiomatic univalence), its proofs are independent of any particular computational interpretation. This means UniMath proofs are valid in any model of Book HoTT — in the classical set-theoretic model, in the Kan simplicial set model, in the ∞-topos model. This is a feature from a foundations perspective, even though it means UniMath cannot do the kind of explicit computation that Cubical Agda can.

**Current focus.** UniMath's current development is primarily in category theory and algebra. The main active contributors are Benedikt Ahrens, Peter LeFanu Lumsdaine, and Dan Grayson (one of the original Voevodsky collaborators). A reader interested in machine-verifying category-theoretic results may find UniMath more natural than Cubical Agda.

---

## Lean 4 and Mathlib: The HoTT Question

Lean 4 and its mathematical library Mathlib are the dominant infrastructure for machine-verified mathematics in classical foundations (essentially ZFC-compatible dependent type theory with proof irrelevance). Mathlib covers enormous swaths of undergraduate and graduate mathematics: number theory, algebraic geometry, analysis, topology.

**What Mathlib has that is HoTT-adjacent:**
- The fundamental group of a topological space (via covering space theory)
- Homology and cohomology for simplicial complexes (classical)
- Fiber bundles and the Hopf fibration (as a classical fiber bundle)
- Free groups and van Kampen (classical topological version)

**What Mathlib cannot do directly:**
- Higher inductive types: Lean 4 identifies all proofs of the same proposition (proof irrelevance), which is incompatible with HITs
- Synthetic homotopy theory: without HITs, you cannot give the synthetic proof of π₁(S¹) = ℤ
- Univalence: Lean 4 is not univalent; function extensionality is an axiom but propositional extensionality (for propositions, not types) is what's available

**Can you do HoTT in Lean 4?** Partially, and with effort. Some researchers have built small HoTT libraries for Lean 4 by postulating the HoTT axioms (univalence, HITs) explicitly. This is not standard Lean and requires care with universe levels and with the interaction between HoTT axioms and Lean's built-in logic. But it is possible in principle, and a fully-featured Lean 4 HoTT library — parallel to but separate from Mathlib — is a realistic medium-term project.

**What you can contribute to Mathlib that touches HoTT ideas:**
- Classical algebraic topology results that are naturally HoTT-inspired but provable classically: van Kampen, long exact sequences, covering spaces, the Hurewicz theorem
- Formal group laws and their connection to stable homotopy (via number theory in Mathlib)
- The Hopf fibration and its role in Hopf invariant computations (classical)

The Mathlib contribution process is well-documented (docs.lean-lang.org/mathlib4). Start with the Lean Zulip (leanprover.zulipchat.com) to find other contributors working on topology.

---

## Formalizing the Brunerie Proof: What It Would Take

The Ljungström-Mörtberg (2023) proof is machine-verifiable in Cubical Agda. Is there a version that a person could verify by hand — that is, a proof where the conceptual structure is transparent enough that each step can be checked mentally?

The obstacle is not any single step but the size and complexity of the composite computation. A fully "human-verifiable" proof would require:

1. A clean synthetic definition of the Hopf invariant Hopf : π_{2n-1}(Sⁿ) → ℤ (for n = 2, this gives the Hopf fibration's invariant)
2. A clean proof that the Hopf invariant of the Brunerie composite is ±2
3. The EHP sequence as a long exact sequence, formalized in a way that the connecting maps are explicit
4. A computation that the relevant connecting map carries the generator of π₄(S³) to 2 times the generator of π₃(S²)

Steps 1–3 are within reach using current Cubical Agda infrastructure. Step 4 is the hard part: it requires computing with elements of homotopy groups, not just proving that the groups are abstractly isomorphic. The computation involves the James-Hopf invariant at the second level, which is defined by an attaching map that reduces, in principle, to a specific path in a specific type — but the path is very long.

**A realistic intermediate goal:** Formalize steps 1–3 in Cubical Agda and leave step 4 as a `sorry` with a precise statement of what needs to be computed. This would clarify exactly what the remaining computational obstacle is.

---

## The Synthetic Hopf Fibration: A Worked Example of Frontier Formalization

The Hopf fibration η : S³ → S² is formalized in Cubical Agda in `Cubical/HITs/Hopf.agda`. The formalization uses the join construction: S³ = S¹ * S¹ and the Hopf map η : S¹ * S¹ → S² is defined using the two copies of S¹. The fibers η⁻¹(x) ≅ S¹ for each x : S² follow from the join structure.

This formalization is both complete and instructive. Studying it closely — understanding why each step is written the way it is, what library infrastructure is used, where the cubical interval appears — is one of the best preparations for contributing to the library. It demonstrates the gap between "understanding the Hopf fibration" and "writing code that Cubical Agda accepts": several intermediate results about joins and path spaces are needed that would not appear in a classical write-up.

**What studying this file teaches:**
- How to define a fiber bundle using HIT technology
- How to use the join HIT (`_*_`) and its elimination principle
- How path-over-path reasoning works in Cubical Agda (the `PathP` type)
- How to prove fibers are equivalent using cubical transport

**What the file does not contain:** A proof that the total space is S³ (that the join construction actually gives S³, not just some type). This is stated but the proof uses the associativity of joins, which requires further library infrastructure. This gap — proving the join construction gives the right spaces — is an open formalization problem.
