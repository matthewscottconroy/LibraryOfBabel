# Scope, Binding, and Free Variables

## Overview
A variable is **bound** if it falls within the scope of a quantifier that binds it;
otherwise it is **free**. Free variables are like unresolved references — they need
to be given a value before the formula has a definite truth value.

## Learning Objectives
- Identify free and bound occurrences of variables in a formula
- State what it means for a formula to be a sentence (no free variables)
- Apply α-renaming to avoid variable capture

## Definitions
- **Bound occurrence**: an occurrence of x in ∀x φ or ∃x φ that falls within the scope of
  the leading quantifier
- **Free occurrence**: an occurrence that is not bound
- **Sentence (closed formula)**: a formula with no free variables

## Examples
- ∀x P(x,y) — x is bound; y is free → this is not a sentence
- ∀x∃y P(x,y) — both x and y are bound → this is a sentence
- ∀x(P(x) → Q(x,z)) — x is bound; z is free

## Variable Capture
When substituting t for a free variable x in φ, if t contains a variable y that would
fall into the scope of a ∀y or ∃y in φ, the y in t becomes erroneously bound — **capture**.
Avoid by α-renaming bound variables before substitution.

## Tool Connections
- **Lean 4 / Coq**: de Bruijn indices or naming conventions prevent capture; the elaborator
  handles this automatically, but understanding it helps debug tricky goals
- **Haskell**: lambda calculus capture-avoiding substitution is the same phenomenon

## Exercises
See `problems/ch03_predicate_logic/01_translation_exercises.md`
