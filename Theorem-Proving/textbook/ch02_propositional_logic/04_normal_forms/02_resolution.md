# Resolution

## Overview
**Resolution** is an inference rule that operates on clauses in CNF. It is both a proof
system (refutation-complete for propositional logic) and the basis of logic programming
(Prolog). Resolution provides an algorithmic, clause-based approach to theorem proving.

## Learning Objectives
- State the resolution rule
- Apply resolution to derive the empty clause (refutation)
- Understand soundness and completeness of resolution

## The Resolution Rule
Given two clauses:
```
C₁ = (A ∨ l)      C₂ = (B ∨ ¬l)
─────────────────────────────────
         C₁ ∪ C₂ \ {l, ¬l}
       = A ∨ B    (resolvent)
```
where l is the **resolved literal**.

## Refutation Proofs
To prove φ from Γ: add ¬φ to Γ, convert everything to CNF, apply resolution until
the empty clause □ is derived. □ represents ⊥ — a contradiction.
The empty clause derives iff the original set was unsatisfiable (¬φ is inconsistent with Γ).

## Example
Prove q from {p, p→q}:
1. p           (clause: {p})
2. ¬p ∨ q      (clause: {¬p, q})
3. q           (resolve 1, 2 on p)

## Python Implementation
See `textbook/ch02_propositional_logic/05_resolution_in_python/resolution.py`

## Exercises
See `problems/ch02_propositional_logic/02_equivalence_proofs.md`
