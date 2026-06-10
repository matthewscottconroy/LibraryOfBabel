-- Propositional proof search in Lean 4
-- The `decide` tactic evaluates decidable propositions by computation.
-- `tauto` handles propositional tautologies automatically.

import Mathlib.Tactic

-- Basic tautologies proved by decision procedure
example (P Q : Prop) [Decidable P] [Decidable Q] : P ∨ ¬P := by decide +kernel
example : True ∨ False := by decide +kernel

-- The `tauto` tactic handles arbitrary propositional tautologies
example (P Q R : Prop) : (P → Q) → (Q → R) → P → R := by tauto
example (P Q : Prop) : ¬(P ∧ Q) ↔ ¬P ∨ ¬Q := by tauto
example (P Q : Prop) : (P → Q) ↔ ¬P ∨ Q := by tauto

-- De Morgan's laws
example (P Q : Prop) : ¬(P ∨ Q) ↔ ¬P ∧ ¬Q := by tauto
example (P Q : Prop) : ¬(P ∧ Q) ↔ ¬P ∨ ¬Q := by tauto

-- Distribution laws
example (P Q R : Prop) : P ∧ (Q ∨ R) ↔ (P ∧ Q) ∨ (P ∧ R) := by tauto
example (P Q R : Prop) : P ∨ (Q ∧ R) ↔ (P ∨ Q) ∧ (P ∨ R) := by tauto

-- Axt/omega can also handle some propositional goals via arithmetic encoding
-- but tauto is the canonical tool

-- Manual proof of a non-trivial tautology
theorem contraposition (P Q : Prop) : (P → Q) → (¬Q → ¬P) := by
  intro h hnq hp
  exact hnq (h hp)

-- Export: propositional satisfiability via native_decide for finite types
-- For Prop over Bool:
#eval (true || false) && (!true || false)  -- evaluates Boolean formulas
