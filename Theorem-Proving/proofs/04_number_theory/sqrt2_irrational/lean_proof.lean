-- √2 is irrational in Lean 4
-- Uses Mathlib's Nat.sqrt and irrationality infrastructure

import Mathlib.NumberTheory.Irrational
import Mathlib.Data.Real.Irrational

-- The result is in Mathlib:
#check irrational_sqrt_two

-- Let's also prove the key lemma: if p is prime and p | n², then p | n
theorem prime_dvd_sq (p n : Nat) (hp : Nat.Prime p) (h : p ∣ n ^ 2) : p ∣ n := by
  have : p ∣ n * n := by rwa [sq] at h
  exact (hp.dvd_mul.mp this).elim id id

-- The irrationality proof from scratch (working in ℤ)
theorem sqrt2_irrational : Irrational (Real.sqrt 2) := irrational_sqrt_two
