-- Number theory in Lean 4 with Mathlib
import Mathlib.Data.Nat.GCD.Basic
import Mathlib.Data.Nat.Prime.Basic
import Mathlib.Data.Int.ModCast
import Mathlib.Tactic

-- ==========================================
-- GCD and Euclidean Algorithm
-- ==========================================

-- Mathlib's Nat.gcd is computable
#eval Nat.gcd 48 36  -- 12
#eval Nat.gcd 17 5   -- 1 (coprime)

-- Bézout's identity: gcd(a,b) = ua + vb for some integers u, v
-- In Mathlib: Nat.Coprime.eq_one_of_pos' and Int.gcd_eq_gcd_ab
example (a b : ℤ) : ∃ u v : ℤ, u * a + v * b = Int.gcd a b := by
  exact ⟨Int.gcdA a b, Int.gcdB a b, (Int.gcd_eq_gcd_ab a b).symm⟩

-- ==========================================
-- Prime Numbers
-- ==========================================

-- Primality is decidable
#eval Nat.Prime 17   -- should evaluate to True

-- Every natural number > 1 has a prime factor
theorem has_prime_factor : ∀ n : ℕ, 1 < n → ∃ p, p.Prime ∧ p ∣ n :=
  Nat.exists_prime_and_dvd

-- The fundamental theorem of arithmetic: unique factorization
-- In Mathlib via Nat.factors
#eval Nat.factors 360  -- [2, 2, 2, 3, 3, 5]

theorem factors_prod (n : ℕ) (hn : 0 < n) : (Nat.factors n).prod = n :=
  Nat.factors_prod hn

-- ==========================================
-- Modular Arithmetic
-- ==========================================

-- Fermat's little theorem: a^p ≡ a (mod p) for prime p
-- In Mathlib: ZMod.pow_card
example (p : ℕ) [Fact p.Prime] (a : ZMod p) : a ^ p = a :=
  ZMod.pow_card a

-- Chinese Remainder Theorem
-- If gcd(m, n) = 1, then ℤ/(mn) ≅ ℤ/m × ℤ/n
-- In Mathlib: ZMod.chineseRemainder
example : ZMod (3 * 5) ≃+* ZMod 3 × ZMod 5 :=
  ZMod.chineseRemainder (by norm_num) 3 5

-- ==========================================
-- Divisibility
-- ==========================================

-- Euclid's lemma: if p | ab and p is prime, then p | a or p | b
theorem euclid_lemma {p a b : ℕ} (hp : p.Prime) (h : p ∣ a * b) :
    p ∣ a ∨ p ∣ b :=
  (hp.dvd_mul.mp h)

-- Infinitely many primes (Euclid's theorem)
theorem infinite_primes : ∀ n : ℕ, ∃ p > n, p.Prime :=
  Nat.exists_infinite_primes

-- ==========================================
-- Extended Euclidean Algorithm
-- ==========================================

-- Compute gcd and Bézout coefficients
def extGcd : ℤ → ℤ → ℤ × ℤ × ℤ
  | 0, b => (b, 0, 1)
  | a, b =>
    let (g, s, t) := extGcd (b % a) a
    (g, t - (b / a) * s, s)

-- Verify: for gcd(a,b) = g, we have sa + tb = g
-- (proof by induction on a, following the algorithm)
