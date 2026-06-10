-- Proof by cases in Lean 4

import Mathlib.Tactic

-- Case analysis on a boolean
theorem bool_cases (b : Bool) : b = true ∨ b = false := by
  cases b
  · right; rfl
  · left; rfl

-- Case analysis on natural numbers (zero vs successor)
theorem nat_zero_or_succ (n : ℕ) : n = 0 ∨ ∃ m, n = m + 1 := by
  cases n with
  | zero => left; rfl
  | succ m => right; exact ⟨m, rfl⟩

-- Case analysis on an Or hypothesis
theorem or_elim_example (P Q R : Prop) (h : P ∨ Q) (hp : P → R) (hq : Q → R) : R := by
  cases h with
  | inl hp' => exact hp hp'
  | inr hq' => exact hq hq'

-- Case split on an integer being non-negative or negative
theorem int_sign (n : ℤ) : 0 ≤ n ∨ n < 0 := by
  omega

-- Trichotomy: a < b, a = b, or a > b
theorem trichotomy_example (a b : ℤ) : a < b ∨ a = b ∨ a > b := by
  omega

-- Case analysis in a proof about absolute value
theorem abs_nonneg' (n : ℤ) : 0 ≤ |n| := abs_nonneg n

-- Using `rcases` for rich pattern matching on cases
theorem rcases_example (P Q R : Prop) (h : (P ∧ Q) ∨ R) : P ∨ R := by
  rcases h with ⟨hp, _⟩ | hr
  · left; exact hp
  · right; exact hr

-- The `omega` tactic handles case splits in linear arithmetic automatically
theorem abs_int (n : ℤ) : n ≥ 0 → |n| = n := by
  intro h
  exact abs_of_nonneg h
