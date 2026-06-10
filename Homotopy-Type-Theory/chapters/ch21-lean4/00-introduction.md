# Chapter 21: Lean 4 and Mathlib — Formal Proof at Scale

## From Theory to Practice

We've now covered the mathematical foundations of HoTT in depth — identity types, h-levels, univalence, higher inductive types, and synthetic homotopy theory. This chapter shifts gears: how do we actually *use* a proof assistant to formalize this mathematics?

Lean 4 is the practical answer for most mathematical formalization today. Its library Mathlib4 is the largest formalized mathematics library in existence, with hundreds of thousands of theorems. If you want to:
- Verify that a theorem from the curriculum actually holds
- Formalize new mathematics at the interface of HoTT and classical math
- Work on automated theorem proving and tactic development
- Contribute to the largest single project in formalized mathematics

...then Lean 4 is the tool.

## Lean 4 vs. Cubical Agda for HoTT

It's worth being direct about the difference:

**Lean 4** is the tool of choice for *classical mathematics at scale*. Its foundations are the Calculus of Constructions with Quotient types and `propext` + `funext` as axioms. It supports Univalence indirectly (via propositional extensionality and function extensionality). But it does NOT natively support general Higher Inductive Types or the full HoTT type theory.

For classical algebra, analysis, topology, number theory, and category theory — Lean 4 / Mathlib is unsurpassed.

**Cubical Agda** is the tool of choice for *HoTT-specific mathematics*. Univalence is a theorem, HITs are first-class, and computations are canonical. But the library is smaller.

For computing homotopy groups, working with HITs, and developing synthetic homotopy theory — Cubical Agda (Chapter 22) is the tool.

This chapter teaches Lean 4 as the foundation for classical mathematical formalization that interfaces with HoTT.

## Chapter Roadmap

**Section 1: Lean 4 Basics** — Core syntax, types, terms, and the type-theoretic foundations.

**Section 2: Tactics and Proofs** — The tactic system, the proof state, core tactics, and automation.

**Section 3: Mathlib** — The library's organization, finding theorems, key areas relevant to HoTT.

**Section 4: Formalization Projects** — Guided examples formalizing mathematical content from the curriculum.
