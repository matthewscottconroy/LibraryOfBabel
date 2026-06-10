# Tautologies, Contradictions, and Contingencies

## Overview
Every propositional formula falls into one of three categories based on its truth table.
These categories — tautology, contradiction, and contingency — are fundamental.

## Learning Objectives
- Define tautology, contradiction, and contingency
- Identify which category a given formula falls into
- Connect tautologies to logical validity

## Definitions

**Tautology**: true under every valuation. Written ⊨ φ.
Examples: p ∨ ¬p (law of excluded middle), p → p, (p → q) → (¬q → ¬p)

**Contradiction (Unsatisfiable)**: false under every valuation.
Examples: p ∧ ¬p, (p → q) ∧ p ∧ ¬q

**Contingency**: true under some valuations, false under others.
Examples: p, p → q, p ∧ q

## Connection to Validity
A formula φ is a tautology iff the argument with no premises and conclusion φ is valid.
Γ ⊨ φ iff (∧Γ) → φ is a tautology (where ∧Γ is the conjunction of all sentences in Γ).

## The Satisfiability Problem (SAT)
Deciding whether a propositional formula is satisfiable is the canonical NP-complete problem.
Truth tables check this in O(2ⁿ); modern SAT solvers do dramatically better in practice.
SAT solving is the engine behind much of formal verification (see ch10, ch13).

## Tool Connections
- **Python / Z3**: `z3.solve(formula)` checks satisfiability; UNSAT ≡ tautology of negation
- **Lean 4**: `decide` tactic decides propositional tautologies over `Bool`
- **Coq**: `tauto` tactic proves intuitionistic propositional tautologies

## Exercises
See `problems/ch02_propositional_logic/03_tautology_identification.md`
