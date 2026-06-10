# Existence and Uniqueness Proofs

## Overview
Existence proofs show ∃x P(x); uniqueness proofs show there is *at most* one such x.
Together they establish ∃!x P(x) — unique existence, a common mathematical claim.

## Learning Objectives
- Prove existence by constructing an explicit witness
- Prove uniqueness by assuming two witnesses and showing they are equal
- Combine them into ∃!x P(x) proofs

## Existence by Construction (Constructive Proof)
Exhibit a specific term t and verify P(t). This is the ∃-introduction rule.

**Example**: Prove ∃x∈ℤ(x² = 9). Witness: x = 3. Check: 3² = 9. ✓

## Existence by Non-Constructive Argument
Use contradiction: assume ¬∃x P(x), derive ⊥. You prove existence without exhibiting a witness.
This is non-constructive and not accepted in constructive/intuitionistic mathematics.

**Example**: Proof that there exist irrationals a, b with aᵇ rational.
Consider √2^√2. Either this is rational (take a=b=√2) or irrational — in which case
(√2^√2)^√2 = √2² = 2, rational (take a=√2^√2, b=√2). One case must hold. □
Note: this does not tell us *which* case holds.

## Uniqueness
To prove uniqueness: assume x and y both satisfy P; prove x = y.
This uses the ∀ introduction rule.

**Example**: Prove the additive identity in a group is unique.
Suppose e₁ and e₂ are both identities. Then e₁ = e₁ * e₂ = e₂. □

## Lean 4
```lean
-- Existence: ∃ n : Nat, n * n = 9
example : ∃ n : Nat, n * n = 9 := ⟨3, by norm_num⟩

-- Uniqueness: at most one additive identity
-- (built into Mathlib as group uniqueness lemmas)
```

## Exercises
See `problems/ch05_proof_strategies/02_direct_proof_exercises.md`
