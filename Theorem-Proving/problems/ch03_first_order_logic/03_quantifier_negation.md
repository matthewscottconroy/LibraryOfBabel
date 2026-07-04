# Quantifier Negation and Equivalences

## Section 1: Quantifier Negation (★)

**1.** Push the negation inward as far as possible (until only literals remain negated):
  a. `¬∀x P(x)`
  b. `¬∃x P(x)`
  c. `¬∀x∃y R(x,y)`
  d. `¬∃x∀y(P(x) → Q(y))`
  e. `¬∀x(P(x) ∨ ¬Q(x))`

## Section 2: Prenex Normal Form (★★)

**2.** Convert each formula to **prenex normal form** (all quantifiers in front):
  a. `∀x P(x) ∧ ∃y Q(y)`
  b. `(∀x P(x)) → (∃y Q(y))`
  c. `¬∀x(P(x) → ∃y R(x,y))`

  Note: converting implication (A→B) to prenex requires care about which variables are
  universally/existentially bound in the body.

## Section 3: Lean 4 Proofs (★★)

**3.** Prove each in Lean 4 (fill in the sorry):

```lean
variable {α : Type*} (P Q : α → Prop)

-- 3a. ∀x(P x ∧ Q x) → (∀x P x) ∧ (∀x Q x)
example : (∀ x, P x ∧ Q x) → (∀ x, P x) ∧ (∀ x, Q x) := by sorry

-- 3b. ∃x(P x ∨ Q x) ↔ (∃x P x) ∨ (∃x Q x)
example : (∃ x, P x ∨ Q x) ↔ (∃ x, P x) ∨ (∃ x, Q x) := by sorry

-- 3c. (∀x P x) → ¬∃x ¬P x
example : (∀ x, P x) → ¬∃ x, ¬P x := by sorry
```

## Section 4: Challenge (★★★)

**4.** Show that the following schema is NOT valid (give a counterexample):
`∃x P(x) ∧ ∃x Q(x) → ∃x(P(x) ∧ Q(x))`

**5.** Show that `∀x∃y R(x,y) → ∃y∀x R(x,y)` is NOT valid.
Then show the converse IS valid: `∃y∀x R(x,y) → ∀x∃y R(x,y)`.
