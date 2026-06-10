# First-Order Structures

## Overview
A **first-order structure** (or **model**) makes a first-order language concrete:
it provides a domain of individuals and interprets each predicate, function symbol,
and constant. Model theory studies the relationship between theories and their models.

## Learning Objectives
- Define a first-order structure formally
- Evaluate FOL sentences in a given structure
- Define homomorphism and isomorphism of structures

## Formal Definition
A structure M for language L consists of:
- A non-empty set |M| (the **domain** or **universe**)
- For each constant c ∈ L: a domain element cᴹ ∈ |M|
- For each n-ary function symbol f ∈ L: a function fᴹ : |M|ⁿ → |M|
- For each n-ary predicate symbol P ∈ L: a relation Pᴹ ⊆ |M|ⁿ

## Standard Mathematical Structures
- (ℕ, 0, S, +, ×, =): the standard model of Peano arithmetic
- (ℝ, 0, 1, +, ×, ≤): the real ordered field
- (𝒫(U), ∅, ∪, ∩, ᶜ): Boolean algebra of sets

## Truth in a Structure
The truth value of a sentence φ in M (written M ⊨ φ) is defined inductively:
- M ⊨ P(t₁,...,tₙ) iff (t₁ᴹ,...,tₙᴹ) ∈ Pᴹ
- M ⊨ ¬φ iff M ⊭ φ
- M ⊨ φ∧ψ iff M ⊨ φ and M ⊨ ψ
- M ⊨ ∀x φ(x) iff for all a ∈ |M|, M[x↦a] ⊨ φ(x)

## Homomorphisms and Isomorphisms
A **homomorphism** h: M → N preserves the structure (h(fᴹ(ā)) = fᴺ(h(ā)), etc.).
An **isomorphism** is a bijective homomorphism with bijective inverse.
Isomorphic structures satisfy exactly the same first-order sentences.

## Real-World Applications
- Database theory: a database instance is a finite first-order structure
- Abstract algebra: groups, rings, fields are all first-order structures
- Programming language semantics: program states are structures

## Exercises
See `problems/ch09_model_theory/01_structure_construction.md`
