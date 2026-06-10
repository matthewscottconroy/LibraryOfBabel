# Mixed Proof Strategy Challenges

For each theorem below: (1) identify the best proof strategy, (2) write the proof.

## Direct Proof (★)
**1.** If m and n are both odd integers, then m + n is even.
**2.** If a | b, then a | b².
**3.** The product of any integer with an even integer is even.

## Contrapositive (★★)
**4.** If n² is odd, then n is odd.
**5.** If f ∘ g is injective, then g is injective. (Does anything follow about f?)
**6.** For integers, if 3 ∤ n, then 3 ∤ n².

## Contradiction (★★)
**7.** ∜2 is irrational.
**8.** log₂(3) is irrational.
**9.** There is no largest even integer.

## Cases (★★)
**10.** For all integers n, n² + n is even.
**11.** For all integers n, n³ − n is divisible by 6. (Use cases mod 6, or mod 2 and mod 3.)

## Existence and Uniqueness (★★)
**12.** Prove ∃x ∈ ℝ, x² = x + 1. Find the witness explicitly.
**13.** Prove that every real number has a unique additive inverse.

## Mixed Strategy (★★★)
**14.** Prove the well-ordering principle implies mathematical induction and vice versa.
**15.** Prove: if f: A → B and g: B → A satisfy g ∘ f = id_A, then f is injective and g is surjective.

## Lean 4 Challenges (★★★)
**16.** Prove in Lean 4: `∀ n : Int, ∃ m : Int, n < m`
**17.** Prove in Lean 4: `∀ n : Nat, n ≤ n * n`
