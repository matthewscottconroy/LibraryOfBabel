-- Propositional Tactics in Lean 4
-- Chapter 2, Section 5

-- This file explores the main tactics used for propositional reasoning.

variable (p q r s : Prop)

-- `intro`: introduce hypotheses
-- `exact`: close goal with a term
-- `apply`: apply a function/lemma, leaving premises as subgoals
-- `constructor`: split a conjunction goal into two subgoals
-- `cases`: case-split on a disjunction or other inductive type
-- `assumption`: close goal if it matches a hypothesis
-- `contradiction`: close goal when there is a ⊥ or p and ¬p in context
-- `tauto` / `aesop`: automated propositional tactic

-- ── Demonstration ─────────────────────────────────────────────

theorem and_comm : p ∧ q → q ∧ p := by
  intro ⟨hp, hq⟩
  constructor
  · exact hq
  · exact hp

theorem imp_trans : (p → q) → (q → r) → p → r := by
  intro hpq hqr hp
  apply hqr
  apply hpq
  exact hp

-- `tauto` proves classical tautologies automatically
theorem excluded_middle_demo : p ∨ ¬p := by
  tauto

theorem double_neg : ¬¬p ↔ p := by
  constructor
  · intro hnnp
    by_contra hnp
    exact hnnp hnp
  · intro hp hnnp
    exact hnnp hp

-- ── Structured term-mode proofs ───────────────────────────────

theorem distr : p ∧ (q ∨ r) ↔ (p ∧ q) ∨ (p ∧ r) :=
  ⟨fun ⟨hp, hqr⟩ => hqr.elim (fun hq => Or.inl ⟨hp, hq⟩) (fun hr => Or.inr ⟨hp, hr⟩),
   fun h => h.elim (fun ⟨hp, hq⟩ => ⟨hp, Or.inl hq⟩) (fun ⟨hp, hr⟩ => ⟨hp, Or.inr hr⟩)⟩
