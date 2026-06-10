# Gödel's Completeness Theorem

## Overview
Gödel's completeness theorem (1929) states that first-order logic is **complete**:
every valid sentence has a formal proof. This fundamental result connects semantic truth
(validity in all models) with syntactic derivability (existence of a proof).

## Learning Objectives
- State the completeness and soundness theorems for FOL
- Understand the significance of the result
- Distinguish completeness from Gödel's *incompleteness* theorems

## The Theorems

**Soundness**: If Γ ⊢ φ (there is a formal proof of φ from Γ), then Γ ⊨ φ.
*Proof*: by induction on proof length; each rule preserves truth.

**Completeness** (Gödel 1929): If Γ ⊨ φ, then Γ ⊢ φ.
*Proof*: Construct a model from the syntax itself (the Henkin construction).
If no proof exists, extend Γ to a maximal consistent set, add "witnesses" (Henkin constants)
for existential sentences, and read off a model. This model satisfies Γ but not ¬φ.

**Corollary**: Γ ⊨ φ iff Γ ⊢ φ (for FOL with a complete proof system like natural deduction).

## The Compactness Theorem
Follows from completeness: if every finite subset of Γ is satisfiable, then Γ is satisfiable.
*Proof*: if Γ were unsatisfiable, Γ ⊨ ⊥, so Γ ⊢ ⊥. But proofs are finite, so some finite
subset of Γ derives ⊥, contradicting finite satisfiability.

## Significance
- The proof system is not missing any valid inferences
- There is no "semantic gap" between what is true and what is provable in FOL
- Contrast: second-order logic is *incomplete* — Gödel's proof does not apply there

## Warning: Not the Same as Incompleteness
Gödel's *incompleteness theorems* (1931) say that any sufficiently strong consistent
axiomatic theory (like PA) has true sentences that are not provable *from its axioms*.
This does *not* contradict completeness: every valid FOL sentence is provable in FOL.

## Exercises
See `problems/ch09_model_theory/03_completeness_applications.md`
