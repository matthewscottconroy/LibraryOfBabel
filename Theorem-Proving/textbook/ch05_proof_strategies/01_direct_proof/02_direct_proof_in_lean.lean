-- Direct Proof in Lean 4
-- Chapter 5, Section 1

import Mathlib.Tactic
import Mathlib.Data.Int.Basic

-- ── Even and odd ──────────────────────────────────────────────

def isEven (n : Int) : Prop := ∃ k, n = 2 * k
def isOdd  (n : Int) : Prop := ∃ k, n = 2 * k + 1

-- Direct proof: if n is even, n² is even
theorem even_sq (n : Int) (h : isEven n) : isEven (n * n) := by
  obtain ⟨k, hk⟩ := h
  exact ⟨2 * k * k, by rw [hk]; ring⟩

-- Direct proof: sum of two even numbers is even
theorem even_add (m n : Int) (hm : isEven m) (hn : isEven n) : isEven (m + n) := by
  obtain ⟨j, hj⟩ := hm
  obtain ⟨k, hk⟩ := hn
  exact ⟨j + k, by rw [hj, hk]; ring⟩

-- Direct proof: divisibility is transitive
theorem dvd_trans' (a b c : Int) (hab : a ∣ b) (hbc : b ∣ c) : a ∣ c :=
  dvd_trans hab hbc    -- Mathlib already has this; here for illustration

-- Direct proof (manual): a | b → b | c → a | c
theorem dvd_trans_manual (a b c : Int)
    (hab : ∃ m, b = a * m) (hbc : ∃ n, c = b * n) : ∃ p, c = a * p := by
  obtain ⟨m, hm⟩ := hab
  obtain ⟨n, hn⟩ := hbc
  exact ⟨m * n, by rw [hn, hm]; ring⟩

-- ── Exercises ─────────────────────────────────────────────────

-- Exercise 1: if n is odd, n² is odd
theorem odd_sq (n : Int) (h : isOdd n) : isOdd (n * n) := by
  sorry

-- Exercise 2: sum of two odd numbers is even
theorem odd_add_odd (m n : Int) (hm : isOdd m) (hn : isOdd n) : isEven (m + n) := by
  sorry
