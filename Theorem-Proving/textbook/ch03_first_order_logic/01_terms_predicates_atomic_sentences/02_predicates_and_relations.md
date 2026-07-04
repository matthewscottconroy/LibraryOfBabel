# Predicates, Relations, and Atomic Sentences

## Overview
Predicates represent properties (unary) and relations (binary, ternary, ...).
Atomic sentences combine predicates with terms to make the simplest true/false claims in FOL.

## Learning Objectives
- Distinguish predicates, terms, and atomic sentences
- Write atomic sentences in Tarski's World notation
- Understand the arity of predicates

## Terms
A **term** denotes an individual:
- Constants: a, b, 0, "Alice"
- Variables: x, y, z
- Function applications: f(t₁,...,tₙ) where each tᵢ is a term

## Atomic Sentences
An **atomic sentence** is P(t₁,...,tₙ) where P is an n-ary predicate and each tᵢ is a term
with no free variables (a *closed* term or *ground* term).

Examples in Tarski's World:
- `Cube(a)` — block a is a cube
- `LeftOf(a, b)` — a is to the left of b
- `SameSize(a, b)` — a and b are the same size
- `a = b` — identity: a and b name the same block

## Identity
The identity predicate `=` is special: `t₁ = t₂` is true iff t₁ and t₂ denote the same
individual. It is always present in standard FOL (FOL with equality).

## Tool Connections
- **Lean 4**: a unary predicate is `p : α → Prop`; binary relation is `r : α → α → Prop`
- **Coq**: `Inductive` types define predicates; `=` is built in as an inductive type
- **Python**: predicates are Python functions returning bool; relations are sets of tuples

## Exercises
See `problems/ch03_first_order_logic/01_translation_exercises.md`
