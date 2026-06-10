-- Mathematical Induction in Lean 4
-- Chapter 7, Section 1

import Mathlib.Tactic
import Mathlib.Data.Nat.Basic

-- ── Sum formula ───────────────────────────────────────────────

-- ∑ᵢ₌₀ⁿ i = n*(n+1)/2
-- We work in ℕ and avoid division; multiply both sides by 2.

def sumTo : Nat → Nat
  | 0     => 0
  | n + 1 => sumTo n + (n + 1)

theorem sumTo_formula (n : Nat) : 2 * sumTo n = n * (n + 1) := by
  induction n with
  | zero => simp [sumTo]
  | succ k ih =>
    simp [sumTo]
    omega    -- linear arithmetic closes the goal after simp

-- ── Powers of 2 ───────────────────────────────────────────────

-- ∑ᵢ₌₀ⁿ 2ⁱ = 2^{n+1} - 1
def geomSum : Nat → Nat
  | 0     => 1
  | n + 1 => geomSum n + 2 ^ (n + 1)

theorem geomSum_formula (n : Nat) : geomSum n = 2 ^ (n + 1) - 1 := by
  induction n with
  | zero => simp [geomSum]
  | succ k ih =>
    simp [geomSum, ih]
    omega

-- ── Divisibility ──────────────────────────────────────────────

-- 3 | n³ - n for all n
theorem three_dvd_cube_minus (n : Nat) : 3 ∣ n ^ 3 - n + 3 * n := by
  induction n with
  | zero => simp
  | succ k ih =>
    ring_nf
    omega

-- ── Strong induction ──────────────────────────────────────────

-- Every nat > 1 has a prime factor (uses strong induction via Nat.rec or Nat.strong_induction_on)
-- (Proved in proofs/ directory; stub here)
theorem has_prime_factor (n : Nat) (hn : 2 ≤ n) : ∃ p, Nat.Prime p ∧ p ∣ n := by
  exact Nat.exists_prime_and_dvd (by omega)

-- ── Exercises ─────────────────────────────────────────────────

-- Exercise 1: ∑ᵢ₌₀ⁿ (2i+1) = (n+1)²
-- Exercise 2: 4 | 5ⁿ - 1 for all n
-- Exercise 3: n < 2ⁿ for all n

example (n : Nat) : n < 2 ^ n := by
  sorry
