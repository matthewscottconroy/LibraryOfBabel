-- Propositional Logic in Lean 4
-- Chapter 2, Section 5

-- Lean 4 uses dependent type theory. Propositions are types;
-- proofs are terms. For propositional logic, we work in Prop.

-- Variables introduce propositional assumptions
variable (p q r : Prop)

-- ── Basic proofs ──────────────────────────────────────────────

-- Identity: p → p
theorem id_proof : p → p := fun h => h

-- Modus Ponens: p, p → q ⊢ q
theorem modus_ponens (hp : p) (hpq : p → q) : q := hpq hp

-- Modus Tollens: p → q, ¬q ⊢ ¬p
theorem modus_tollens (hpq : p → q) (hnq : ¬q) : ¬p :=
  fun hp => hnq (hpq hp)

-- Hypothetical Syllogism: p→q, q→r ⊢ p→r
theorem hyp_syl (hpq : p → q) (hqr : q → r) : p → r :=
  fun hp => hqr (hpq hp)

-- ── Conjunction ───────────────────────────────────────────────

theorem and_intro (hp : p) (hq : q) : p ∧ q := ⟨hp, hq⟩

theorem and_elim_left (h : p ∧ q) : p := h.1

theorem and_elim_right (h : p ∧ q) : q := h.2

-- ── Disjunction ───────────────────────────────────────────────

theorem or_intro_left (hp : p) : p ∨ q := Or.inl hp

theorem or_intro_right (hq : q) : p ∨ q := Or.inr hq

-- Disjunction elimination (proof by cases)
theorem or_elim (h : p ∨ q) (hpr : p → r) (hqr : q → r) : r :=
  h.elim hpr hqr

-- ── De Morgan ─────────────────────────────────────────────────

-- ¬(p ∧ q) ↔ (¬p ∨ ¬q)  [requires classical logic for →]
theorem de_morgan_and : ¬(p ∧ q) ↔ (¬p ∨ ¬q) := by
  constructor
  · intro h
    by_cases hp : p
    · exact Or.inr (fun hq => h ⟨hp, hq⟩)
    · exact Or.inl hp
  · intro h ⟨hp, hq⟩
    cases h with
    | inl hnp => exact hnp hp
    | inr hnq => exact hnq hq

-- ── Tactic-style proofs ───────────────────────────────────────

-- Same as id_proof but using tactic mode
example : p → p := by
  intro h
  exact h

-- Contrapositive
theorem contrapositive : (p → q) → (¬q → ¬p) := by
  intro hpq hnq hp
  exact hnq (hpq hp)

-- ── Exercise stubs (fill in the proofs) ───────────────────────

-- Exercise 1: Double negation introduction
theorem dne_intro : p → ¬¬p := by
  sorry

-- Exercise 2: p ∧ (p → q) → q
theorem and_mp : p ∧ (p → q) → q := by
  sorry

-- Exercise 3: (p ∨ q) → (q ∨ p)
theorem or_comm : p ∨ q → q ∨ p := by
  sorry
