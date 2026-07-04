# Mathematical Induction Exercises

## Warm-Up: Identifying the Inductive Structure (★)

**1.** For each formula, identify what is being proved by induction and what the base case is:
  a. "For all n ≥ 1, ∑_{k=1}^{n} k² = n(n+1)(2n+1)/6"
  b. "For all n ≥ 0, 2ⁿ > n"
  c. "For all n ≥ 4, n! > 2ⁿ"

## Basic Induction (★)

**2.** Prove by induction:
  a. ∑_{k=0}^{n} k² = n(n+1)(2n+1)/6
  b. ∑_{k=0}^{n} 2^k = 2^{n+1} - 1
  c. n < 2ⁿ for all n ≥ 0
  d. 3 | (n³ - n) for all n ≥ 0

## Intermediate Induction (★★)

**3.** Prove by strong induction:
  a. Every natural number n ≥ 2 is a product of primes (without uniqueness)
  b. Every amount of postage ≥ 8 cents can be made with 3-cent and 5-cent stamps

**4.** Prove by structural induction on binary trees:
  - A binary tree with n internal nodes has exactly n+1 leaves.
  (Internal node = Node with children; leaf = empty tree or childless node.)

## Lean 4 Induction (★★)

**5.** Prove in Lean 4:
```lean
-- Fill in the sorry
theorem sum_sq (n : Nat) : 6 * ∑ i in Finset.range (n+1), i^2 = n * (n+1) * (2*n+1) := by
  sorry

theorem pow_gt_id (n : Nat) : n < 2^n := by
  sorry
```

## Challenge (★★★)

**6.** Prove the Bernoulli inequality: for x > -1 and n ≥ 1, (1+x)ⁿ ≥ 1 + nx.

**7.** Prove that the Fibonacci sequence Fₙ satisfies Fₙ ≤ (5/3)ⁿ for all n ≥ 1.
(Use strong induction.)

**8.** A chocolate bar has m×n squares. Prove that breaking it into individual squares
requires exactly mn−1 breaks, regardless of the breaking strategy.
