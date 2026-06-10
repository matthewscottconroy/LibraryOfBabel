# The Curry-Howard Correspondence

## Overview
The **Curry-Howard correspondence** (independently discovered by Curry ~1934 and Howard 1969)
reveals a deep isomorphism between logic and type theory:
- Propositions correspond to types
- Proofs correspond to programs
- Proof normalization corresponds to program execution

This is not a metaphor — it is a precise mathematical bijection that is the foundation of
every modern proof assistant (Lean, Coq, Agda, Idris).

## Learning Objectives
- State the Curry-Howard correspondence
- Match each logical connective to its type-theoretic counterpart
- Write programs that are simultaneously proofs

## The Dictionary

| Logic | Type Theory |
|-------|-------------|
| Proposition P | Type P |
| Proof of P | Term t : P |
| P ∧ Q | Product type P × Q |
| P ∨ Q | Sum type P + Q |
| P → Q | Function type P → Q |
| ⊥ (False) | Empty type (Void) |
| ⊤ (True) | Unit type () |
| ∀x:A. P(x) | Π-type (dependent product) |
| ∃x:A. P(x) | Σ-type (dependent sum) |

## Key Insight
A proof of P → Q is a *function* that takes a proof of P and returns a proof of Q.
Writing a Haskell function of type `A -> B` is literally constructing a proof of A → B.

## Example
Proof of P ∧ Q → Q ∧ P (conjunction is commutative):
```haskell
-- As a program:
andComm :: (a, b) -> (b, a)
andComm (x, y) = (y, x)
```
```lean
-- As a proof:
theorem and_comm (h : P ∧ Q) : Q ∧ P := ⟨h.2, h.1⟩
```
The function and the proof are the *same thing*.

## Lean 4
See `textbook/ch11_type_theory/02_curry_howard_correspondence/03_curry_howard_in_lean.lean`

## Real-World Applications
- Certified programming: software with machine-checked correctness proofs
- Type-safe APIs: library authors encode invariants in types; users get proof-carrying code
- Dependent type systems: Rust's borrow checker, TypeScript's type narrowing are weak echoes

## Exercises
See `problems/ch11_type_theory/02_curry_howard_exercises.md`
