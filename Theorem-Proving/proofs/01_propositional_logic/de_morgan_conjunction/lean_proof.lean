-- De Morgan's Law (Conjunction) in Lean 4

theorem de_morgan_and (P Q : Prop) : ¬(P ∧ Q) ↔ (¬P ∨ ¬Q) := by
  constructor
  · intro h
    by_cases hp : P
    · right; intro hq; exact h ⟨hp, hq⟩
    · left; exact hp
  · intro h ⟨hp, hq⟩
    rcases h with hnp | hnq
    · exact hnp hp
    · exact hnq hq

-- Also available as Mathlib lemma:
-- #check not_and_or
