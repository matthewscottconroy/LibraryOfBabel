# Individuals, Domains, and the Language of FOL

## Overview
First-order logic (FOL) extends propositional logic by talking about *individuals* —
specific objects in a domain — and their properties and relations. This gives us the
expressive power to capture the structure of mathematical theories.

## Learning Objectives
- Identify the components of first-order language (constants, variables, predicates, functions)
- Specify a first-order language for a given domain
- Understand what a domain of discourse is

## Components of a First-Order Language L
- **Constants**: names for specific individuals (a, b, c, Alice, 0, π)
- **Variables**: range over domain elements (x, y, z, ...)
- **Predicate symbols**: express properties and relations (P(x), Loves(x,y), <)
  Each predicate has a fixed **arity** (number of arguments)
- **Function symbols**: map individuals to individuals (f(x), +, successor)
  Each function has a fixed arity
- **Logical symbols**: ∀, ∃, ¬, ∧, ∨, →, ↔, =

## Tarski's World Language
The blocks-world language of *Language, Proof and Logic* has:
- Constants: a, b, c, d, e, f (names for blocks)
- Predicates: Cube(x), Tet(x), Dodec(x), Small(x), Medium(x), Large(x),
              LeftOf(x,y), RightOf(x,y), FrontOf(x,y), BackOf(x,y),
              SameSize(x,y), SameShape(x,y), SameRow(x,y), SameCol(x,y),
              Adjoins(x,y), Between(x,y,z)
- No function symbols

## Tool Connections
- **Tarski's World**: the world gives a domain (the blocks on the grid); predicates are
  interpreted by the spatial relationships between blocks
- **Lean 4**: `variable (α : Type)` introduces a domain; predicates are `α → Prop`
- **Coq**: `Variable A : Type.` introduces a domain

## Exercises
See `problems/ch03_first_order_logic/01_translation_exercises.md`
