-- Temporal safety and recurrence for a two-state transition system
--
-- System: states {0, 1}; transitions 0 → 0, 0 → 1, 1 → 0 (no self-loop at 1).
-- Labeling: p holds exactly at state 1.
--
-- We verify two temporal properties over all valid traces:
--   Safety      G ¬(p ∧ X p)  — p never holds at two consecutive positions
--   Recurrence  G F ¬p        — state 0 is visited infinitely often

import Mathlib.Tactic

/-- An infinite trace through a two-state system. -/
def Trace := ℕ → Fin 2

/-- The transition relation: every step is allowed except 1 → 1. -/
def transition (a b : Fin 2) : Prop := ¬(a = 1 ∧ b = 1)

/-- A trace is valid when every consecutive pair of states is a transition. -/
def validTrace (π : Trace) : Prop := ∀ i, transition (π i) (π (i + 1))

/-- LTL `G φ`: φ holds at every position. -/
def globally (φ : ℕ → Prop) : Prop := ∀ i, φ i

/-- LTL `F φ` evaluated from position `i`: φ holds at some position `j ≥ i`. -/
def eventuallyFrom (φ : ℕ → Prop) (i : ℕ) : Prop := ∃ j, i ≤ j ∧ φ j

/-- Case analysis on the states of a two-state system. -/
theorem fin_two_cases (a : Fin 2) : a = 0 ∨ a = 1 := by
  rcases a with ⟨_ | _ | n, h⟩
  · exact Or.inl rfl
  · exact Or.inr rfl
  · omega

/-- **Safety** `G ¬(p ∧ X p)`: in a valid trace, state 1 is never
    immediately followed by state 1. This is exactly the transition
    constraint, lifted to a temporal property of whole traces. -/
theorem safety (π : Trace) (h : validTrace π) :
    globally (fun i => ¬(π i = 1 ∧ π (i + 1) = 1)) :=
  fun i => h i

/-- In a valid trace, every position is at state 0, or the next one is. -/
theorem zero_soon (π : Trace) (h : validTrace π) (i : ℕ) :
    π i = 0 ∨ π (i + 1) = 0 := by
  rcases fin_two_cases (π i) with h0 | h1
  · exact Or.inl h0
  · rcases fin_two_cases (π (i + 1)) with h0' | h1'
    · exact Or.inr h0'
    · exact absurd ⟨h1, h1'⟩ (h i)

/-- **Recurrence** `G F ¬p`: a valid trace visits state 0 infinitely often.
    From any position, state 0 appears within at most one step. -/
theorem recurrence (π : Trace) (h : validTrace π) :
    globally (eventuallyFrom (fun j => π j = 0)) := by
  intro i
  rcases zero_soon π h i with h0 | h1
  · exact ⟨i, Nat.le_refl i, h0⟩
  · exact ⟨i + 1, Nat.le_succ i, h1⟩
