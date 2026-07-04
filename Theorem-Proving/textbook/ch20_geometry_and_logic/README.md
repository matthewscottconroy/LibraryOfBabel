# Chapter 20: Geometry and Logic

Geometry as a logical laboratory — the discipline where the axiomatic method was born, where the first genuine independence proof was found, and where, uniquely among the great mathematical theories, *completeness and decidability actually hold*.

## Overview

For over two thousand years Euclid's *Elements* was rigor made visible. Yet its proofs lean on the diagram — circles are read off as intersecting, points as lying between others — and the nineteenth century discovered that these appeals conceal genuine **logical gaps**: missing axioms of continuity, order, and congruence. Repairing them forced a new conception of what an axiom is — not a self-evident truth about space but an **uninterpreted sentence that implicitly defines its primitives**, correct only if every step survives in *every* model.

Three episodes make geometry the sharpest case study in metalogic. First, the **parallel postulate**: two millennia of failed attempts to prove it from the others collapsed when Bolyai, Lobachevsky, and Gauss built consistent geometries where it fails, and Beltrami, Klein, and Poincaré supplied Euclidean *models* of them — the first proof that an axiom is **independent** of the rest, a template later reused for the Axiom of Choice and the Continuum Hypothesis. Second, **Hilbert's *Grundlagen der Geometrie*** (1899) gave the first fully rigorous axiomatization — five groups (incidence, order, congruence, parallels, continuity) — and settled consistency, independence, and categoricity by explicitly constructing models, inventing modern axiomatics in the process. Third — the chapter's logical payoff — **Tarski** showed that the first-order theory of elementary Euclidean geometry is **consistent, complete, and decidable**, because it is bi-interpretable with the theory of **real-closed fields**, which admits quantifier elimination. This is the flat contrast to arithmetic, which by Gödel's theorems is incomplete and undecidable; the difference is exactly the integers, for geometry is decidable *because it cannot count*. The chapter closes in the proof assistants — Tarski's system formalized in **GeoCoq**, analytic geometry over $\mathbb{R}$ in Lean's Mathlib — completing the loop from Euclid's informal system to a machine-checked, machine-*decidable* one.

## Why It Matters

Geometry is where the concept of a formal system, an unintended model, and an independence proof were first forced into the open. It is also the great counterexample to the mood of Gödel's theorems: incompleteness is not the fate of every rich theory but the special mark of theories that can interpret arithmetic. Tarski's completeness-and-decidability result — with its coordinatization engine linking synthetic geometry to real-closed fields — is one of the cleanest demonstrations in logic that *how much a theory can express* determines *whether it can be tamed*. And it is not idle: cylindrical algebraic decomposition and SMT solvers turn that decidability into working software for robotics, CAD, and verified geometry.

## Chapter Roadmap

1. [Euclid's Elements as a Formal System](01_euclid/01_euclid_axioms.md) — the postulates, Proposition I.1 as a derivation, the continuity and order gaps, diagrams as implicit axioms, material vs. formal axiomatics.
2. [Non-Euclidean Geometry and the Parallel Postulate](02_non_euclidean/01_parallel_postulate.md) — equivalents of the postulate, Saccheri, hyperbolic geometry and its models (Klein, Poincaré), the angle-defect theorem, and the first independence proof.
3. [Hilbert's *Grundlagen*](03_hilbert/01_hilbert_axioms.md) — the five axiom groups, the pons asinorum without superposition, consistency and independence by models, segment arithmetic, second-order categoricity.
4. [Tarski's Elementary Geometry: Complete and Decidable](04_tarski/01_tarski_geometry.md) — the two-primitive first-order system, real-closed fields and quantifier elimination, the completeness and decidability theorems, and why geometry escapes Gödel.
5. [Geometry in Proof Assistants](05_lean/01_geometry_lean.md) — GeoCoq's formalization of Tarski's axioms, the Tarski–Hilbert equivalence, synthetic vs. analytic geometry in Lean's Mathlib, mechanized decision procedures.

## Prerequisites

- [Chapter 3: First-Order Logic](../ch03_first_order_logic/) — every axiom system here is a first-order (or, for Hilbert's continuity, second-order) theory.
- [Chapter 9: Model Theory](../ch09_model_theory/) — models, categoricity, and quantifier elimination are the working tools of Sections 3–4.
- [Chapter 10: Computability and Incompleteness](../ch10_computability_and_incompleteness/) — the essential foil: Tarski's decidability mirrors Gödel's incompleteness.
- Helpful: [Chapter 19: Abstract Algebra](../ch19_abstract_algebra/) (real-closed fields) and [Chapter 6: Set Theory](../ch06_set_theory/) (the AC/CH independence analogy).
