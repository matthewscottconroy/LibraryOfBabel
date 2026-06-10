-- Lean 4 Proof Workflow
-- Chapter 4, Section 5

-- This file demonstrates the standard workflow for proving theorems in Lean 4.
-- Key concepts: goals, hypotheses, tactics, term-mode proofs.

-- ── Step 1: State the theorem ─────────────────────────────────

-- A theorem has: name, optional hypotheses, statement, proof
theorem example1 (p q : Prop) (hp : p) (hpq : p → q) : q :=
  hpq hp                -- term-mode proof: directly apply hpq to hp

-- ── Step 2: Use tactic mode for complex proofs ─────────────────

theorem example2 (p q r : Prop) (h1 : p ∧ q) (h2 : q → r) : p ∧ r := by
  obtain ⟨hp, hq⟩ := h1   -- destructure the conjunction
  exact ⟨hp, h2 hq⟩        -- build the result

-- ── Step 3: Use #check, #eval, #print for exploration ─────────

#check Nat.add_comm        -- prints the type of a lemma
-- #eval 2 + 3             -- evaluates an expression

-- ── Step 4: Understand the goal state ─────────────────────────

-- In VS Code / Emacs, place cursor inside a tactic block to see
-- the current goal state in the infoview panel.

example (p q : Prop) (h : p ∧ q) : q ∧ p := by
  -- Goal: q ∧ p
  -- Context: h : p ∧ q
  constructor
  -- Goal 1: q
  · exact h.2
  -- Goal 2: p
  · exact h.1

-- ── Step 5: sorry and holes ────────────────────────────────────

-- Use `sorry` to leave a proof hole during development.
-- Lean will warn but compile.
theorem exercise_stub (p q : Prop) : p ∨ q → q ∨ p := by
  sorry  -- TODO: fill this in

-- ── Step 6: Import Mathlib for powerful tactics ───────────────

-- import Mathlib.Tactic  -- gives aesop, simp, ring, linarith, omega, etc.
-- These are commented out here; add them to your lakefile and import as needed.
