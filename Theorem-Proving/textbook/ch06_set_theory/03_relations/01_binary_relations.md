# Binary Relations

## Overview
A **binary relation** R on sets A and B is a subset of A × B. Relations are more general
than functions: an element of A may relate to multiple elements of B, or to none.
Relations formalize comparisons, orderings, connections, and equivalences.

## Learning Objectives
- Define binary relation formally
- Identify the domain, codomain, and graph of a relation
- Give examples of important mathematical relations

## Formal Definition
A binary relation R from A to B is R ⊆ A × B.
We write aRb (or R(a,b)) for (a,b) ∈ R.

## Key Examples
- ≤ on ℝ: {(x,y) | x ≤ y}
- | on ℕ (divisibility): {(a,b) | a divides b}
- ∈ on sets: {(x,A) | x is an element of A}
- = on any set A: {(a,a) | a ∈ A} (the identity/diagonal relation)
- ∅: the empty relation (nothing relates to anything)
- A × B: the total relation (everything relates to everything)

## Operations on Relations
- **Inverse/Converse**: R⁻¹ = {(b,a) | (a,b) ∈ R}
- **Composition**: R ∘ S = {(a,c) | ∃b((a,b) ∈ R ∧ (b,c) ∈ S)}

## Tool Connections
- **Lean 4**: `Rel α β` is `α → β → Prop`; `Relation.Comp` for composition
- **Haskell**: `type Relation a b = a -> b -> Bool` or `Set (a, b)`
- **Python**: `dict` and `set of tuples` for finite relations

## Exercises
See `problems/ch06_set_theory/02_relation_property_proofs.md`
