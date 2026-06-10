# Proof by Cases

## Overview
When a hypothesis or the domain naturally splits into finitely many exhaustive cases,
prove the conclusion in each case separately. The disjunction elimination rule (∨E)
formalizes this: from φ ∨ ψ, prove χ by proving it under φ and under ψ separately.

## Learning Objectives
- Identify when proof by cases applies
- Ensure cases are exhaustive (cover every possibility)
- Structure case proofs clearly

## Method
To prove Q given that cases C₁, C₂, ..., Cₙ are exhaustive:
1. Show C₁ ∨ C₂ ∨ ... ∨ Cₙ (all cases are covered)
2. For each Cᵢ, assume Cᵢ and prove Q

## Example: |x| ≥ 0 for all real x
**Case 1**: x ≥ 0. Then |x| = x ≥ 0. ✓
**Case 2**: x < 0. Then |x| = -x > 0 ≥ 0. ✓
Since every real number satisfies exactly one case, |x| ≥ 0 for all x. □

## Example: For any integer n, n(n+1) is even
**Case 1**: n is even. Then n = 2k, so n(n+1) = 2k(2k+1) is even. ✓
**Case 2**: n is odd. Then n+1 is even, so n+1 = 2k, and n(n+1) = n·2k is even. ✓
Every integer is either even or odd, so n(n+1) is always even. □

## Law of Excluded Middle
The exhaustiveness of the two cases (φ, ¬φ) follows from the law of excluded middle (LEM).
LEM is classical; in intuitionistic logic you need an explicit decision procedure.

## Tool Connections
- **Lean 4**: `rcases`, `by_cases h : P`, `omega` (for integer arithmetic)
- **Coq**: `destruct (classic P)` (with Classical), or `destruct H` for inductive types
- **Haskell**: pattern matching over data types is exactly proof by cases

## Exercises
See `problems/ch05_proof_strategies/04_mixed_proof_challenges.md`
