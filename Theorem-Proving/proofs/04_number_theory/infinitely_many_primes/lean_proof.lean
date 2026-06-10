-- Infinitely Many Primes in Lean 4
-- Uses Mathlib's Nat.Prime infrastructure

import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Data.Nat.Prime.Infinite

-- The theorem is in Mathlib as:
#check Nat.infinite_setOf_prime

-- Here we prove it from more basic principles to illustrate the technique.

-- Key lemma: n! + 1 has a prime factor > n
lemma factorial_succ_has_large_prime (n : Nat) :
    ∃ p, Nat.Prime p ∧ p ∣ n.factorial + 1 ∧ p > n := by
  -- n! + 1 > 1, so it has a prime factor p
  have h1 : n.factorial + 1 > 1 := by omega
  obtain ⟨p, hp, hdvd⟩ := Nat.exists_prime_and_dvd (by omega)
  use p, hp, hdvd
  -- p ∤ n! (since all primes ≤ n divide n! but not n!+1)
  by_contra hle
  push_neg at hle
  have hle' : p ≤ n := by omega
  have : p ∣ n.factorial := Nat.dvd_factorial.mpr ⟨hp.pos, hle'⟩
  have : p ∣ 1 := by omega
  exact Nat.Prime.one_lt hp |>.ne' (Nat.eq_one_of_dvd_one this)

-- The main theorem
theorem infinitely_many_primes : ∀ n, ∃ p, Nat.Prime p ∧ p > n := by
  intro n
  obtain ⟨p, hp, _, hgt⟩ := factorial_succ_has_large_prime n
  exact ⟨p, hp, hgt⟩
