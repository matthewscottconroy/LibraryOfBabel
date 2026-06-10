-- Temporal Logic Safety Property in Lean 4
-- We model a simple two-state system and verify a safety property

import Mathlib.Tactic

-- A simple Kripke structure: states 0 and 1
-- Transitions: 0 → 0, 0 → 1, 1 → 0 (no self-loop at 1)
-- Labeling: p holds at state 0, ¬p at state 1

-- A trace is an infinite sequence of states
def Trace := ℕ → Fin 2

-- Validity at position i
def holds_at (π : Trace) (i : ℕ) (p : Fin 2 → Prop) : Prop := p (π i)

-- Safety: p holds at every position
def globally (π : Trace) (p : Fin 2 → Prop) : Prop :=
  ∀ i, holds_at π i p

-- Eventually p holds
def eventually (π : Trace) (p : Fin 2 → Prop) : Prop :=
  ∃ i, holds_at π i p

-- The transition relation
def transition : Fin 2 → Fin 2 → Prop
  | ⟨0, _⟩, ⟨0, _⟩ => True   -- 0 → 0
  | ⟨0, _⟩, ⟨1, _⟩ => True   -- 0 → 1
  | ⟨1, _⟩, ⟨0, _⟩ => True   -- 1 → 0
  | ⟨1, _⟩, ⟨1, _⟩ => False  -- no 1 → 1

-- A valid trace respects transitions
def valid_trace (π : Trace) : Prop :=
  ∀ i, transition (π i) (π (i + 1))

-- Safety property: if we start at state 0, we can always return to state 0
-- (This is a liveness property in disguise)
theorem reachability : ∀ (π : Trace), valid_trace π →
    π 0 = ⟨0, by norm_num⟩ →
    ∃ i > 0, π i = ⟨0, by norm_num⟩ := by
  intro π hvalid hstart
  -- From state 0, we can stay at 0 (by the self-loop)
  use 1
  constructor
  · norm_num
  · -- At step 1, π 1 is a successor of π 0 = 0
    -- 0 → 0 or 0 → 1; to show we can reach 0, take the 0→0 path
    -- (We need to strengthen: if π takes the 0→0 transition at step 0...)
    sorry -- Full proof requires fixing the path
