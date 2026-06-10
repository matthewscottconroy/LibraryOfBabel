# Resolution

## Overview
Resolution (Robinson 1965) is a refutation-complete inference rule for propositional
and first-order logic. It operates on clauses and is the basis of all modern SAT
solvers and the core of Prolog's inference engine.

## Learning Objectives
- State the resolution rule for propositional logic
- Apply resolution refutation to prove theorems
- Explain the role of unification in first-order resolution

## Propositional Resolution
Given clauses C₁ = {l, A₁,...,Aₙ} and C₂ = {¬l, B₁,...,Bₘ}:
```
Resolvent = {A₁,...,Aₙ, B₁,...,Bₘ}
```
The resolved literal l is eliminated. If the resolvent is empty (□), we have a refutation.

## First-Order Resolution
In FOL, we first **unify** complementary literals. The **most general unifier (MGU)**
replaces variables with terms to make two literals identical (one positive, one negative).

Example: From ∀x (P(x) → Q(f(x))) and P(a):
- Clause 1: {¬P(x), Q(f(x))}
- Clause 2: {P(a)}
- Unifier: x ↦ a
- Resolvent: {Q(f(a))}

## Resolution in Python
See `textbook/ch04_proof_systems/03_resolution/resolution.py`

## Exercises
See `problems/ch04_proof_systems/01_natural_deduction_derivations.md`
