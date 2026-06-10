-- Induction Exercises in Lean 4
-- Fill in each `sorry`.

import Mathlib.Tactic
import Mathlib.Data.Nat.Basic

-- ── Exercise 1: Geometric sum ─────────────────────────────────

def geoSum : Nat → Nat
  | 0     => 1
  | n + 1 => geoSum n + 2 ^ (n + 1)

theorem geo_sum_formula (n : Nat) : geoSum n + 1 = 2 ^ (n + 1) := by
  induction n with
  | zero => simp [geoSum]
  | succ k ih => sorry

-- ── Exercise 2: n < 2^n ───────────────────────────────────────

theorem lt_two_pow (n : Nat) : n < 2 ^ n := by
  induction n with
  | zero => simp
  | succ k ih => sorry

-- ── Exercise 3: Sum of first n odds = n² ──────────────────────

-- ∑_{i=0}^{n-1} (2i+1) = n²
theorem sum_odds (n : Nat) : ∑ i in Finset.range n, (2 * i + 1) = n ^ 2 := by
  induction n with
  | zero => simp
  | succ k ih => sorry

-- ── Exercise 4: List append length ───────────────────────────

theorem append_length (l m : List α) :
    (l ++ m).length = l.length + m.length := by
  induction l with
  | nil  => simp
  | cons h t ih => sorry

-- ── Exercise 5: All elements of map satisfy predicate ─────────

theorem map_all (f : α → β) (P : β → Prop) (l : List α)
    (h : ∀ x ∈ l, P (f x)) : ∀ y ∈ l.map f, P y := by
  sorry
