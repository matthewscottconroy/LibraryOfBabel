-- First-Order Logic in Lean 4
-- Chapter 3, Section 6

-- In Lean 4, FOL is expressed via:
-- - Types for domains
-- - Functions α → Prop for predicates
-- - Terms of type α for individuals
-- The logic is higher-order (HOL), but we can restrict to FOL patterns.

variable {α : Type*}
variable (P Q : α → Prop)
variable (R : α → α → Prop)

-- ── Quantifier rules ──────────────────────────────────────────

-- Universal introduction: prove ∀x P(x) by proving P(x) for arbitrary x
theorem forall_intro (h : ∀ x, P x) : ∀ x, P x := h

-- Universal elimination (instantiation)
theorem forall_elim (h : ∀ x, P x) (a : α) : P a := h a

-- Existential introduction (by witness)
theorem exists_intro (a : α) (h : P a) : ∃ x, P x := ⟨a, h⟩

-- Existential elimination
theorem exists_elim (h : ∃ x, P x) (k : ∀ x, P x → Q x) (b : α) : ∃ x, Q x := by
  obtain ⟨a, ha⟩ := h
  exact ⟨a, k a ha⟩

-- ── Quantifier equivalences ───────────────────────────────────

-- Quantifier negation: ¬∀x P(x) ↔ ∃x ¬P(x)
theorem neg_forall : (¬ ∀ x, P x) ↔ (∃ x, ¬ P x) := by
  constructor
  · intro h
    by_contra hall
    apply h
    intro x
    by_contra hnpx
    apply hall
    exact ⟨x, hnpx⟩
  · intro ⟨x, hnpx⟩ hall
    exact hnpx (hall x)

-- ¬∃x P(x) ↔ ∀x ¬P(x)
theorem neg_exists : (¬ ∃ x, P x) ↔ (∀ x, ¬ P x) := by
  constructor
  · intro h x hpx; exact h ⟨x, hpx⟩
  · intro h ⟨x, hpx⟩; exact h x hpx

-- ── Exercises ─────────────────────────────────────────────────

-- Exercise 1: ∀x(P(x) → Q(x)) → ∀x P(x) → ∀x Q(x)
theorem forall_imp_dist : (∀ x, P x → Q x) → (∀ x, P x) → ∀ x, Q x := by
  sorry

-- Exercise 2: ∃x(P(x) ∧ Q(x)) → ∃x P(x)
theorem exists_and_proj : (∃ x, P x ∧ Q x) → ∃ x, P x := by
  sorry

-- Exercise 3: (∀x P(x)) ∧ (∀x Q(x)) ↔ ∀x(P(x) ∧ Q(x))
theorem forall_and_split : (∀ x, P x) ∧ (∀ x, Q x) ↔ ∀ x, P x ∧ Q x := by
  sorry
