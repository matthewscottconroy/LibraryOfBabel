# Atomic Propositions and Propositional Variables

## Overview
The atoms of propositional logic are simple, indivisible propositions. They are represented
by propositional variables (p, q, r, ...) that stand in for any specific statement.
Complex formulas are built from atoms using connectives.

## Learning Objectives
- Identify atomic vs. compound propositions
- Understand the role of variables as schematic placeholders
- Read and write formulas using standard notation

## What Counts as Atomic
An atomic proposition has no internal logical structure that matters for propositional logic.
"Alice is tall," "7 is prime," "it is raining" are all atomic for our purposes.
We abstract away their content and write p, q, r.

## Propositional Variables
Variables allow us to state **logical laws** — patterns true regardless of the content of
the atoms. "p → p" is a tautology no matter what p says.

## Well-Formed Formulas (Syntax)
```
φ ::= p                    (atom, p ∈ {p₀, p₁, p₂, ...})
    | ⊤ | ⊥                (truth, falsity)
    | ¬φ                   (negation)
    | (φ ∧ φ)              (conjunction)
    | (φ ∨ φ)              (disjunction)
    | (φ → φ)              (implication)
    | (φ ↔ φ)              (biconditional)
```
Outermost parentheses are often dropped; precedence: ¬ > ∧ > ∨ > → > ↔.

## Tool Connections
- **Lean 4**: `Prop` is the type of propositions; `p : Prop` declares p as a propositional variable
- **Coq**: `Variable p : Prop` introduces p
- **Python**: `from sympy.logic.boolalg import symbols; p, q = symbols('p q')`
- **Haskell**: `data Formula = Atom String | Not Formula | And Formula Formula | ...`
