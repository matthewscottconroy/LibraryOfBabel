-- Propositional Logic: Partial Lean Proofs
-- Fill in each `sorry` with a valid proof.

variable (p q r : Prop)

-- ── Level 1: Basic ────────────────────────────────────────────

-- 1. Identity
theorem ex1 : p → p := by sorry

-- 2. Conjunction commutativity
theorem ex2 : p ∧ q → q ∧ p := by sorry

-- 3. Disjunction commutativity
theorem ex3 : p ∨ q → q ∨ p := by sorry

-- ── Level 2: Intermediate ─────────────────────────────────────

-- 4. Hypothetical syllogism
theorem ex4 : (p → q) → (q → r) → p → r := by sorry

-- 5. Constructive dilemma
theorem ex5 : (p → r) → (q → r) → p ∨ q → r := by sorry

-- 6. Distribution of conjunction over disjunction
theorem ex6 : p ∧ (q ∨ r) ↔ (p ∧ q) ∨ (p ∧ r) := by sorry

-- ── Level 3: Harder ───────────────────────────────────────────

-- 7. De Morgan (conjunction) — requires classical logic
theorem ex7 : ¬(p ∧ q) ↔ ¬p ∨ ¬q := by sorry

-- 8. Export-import (no sorry needed — try term mode)
theorem ex8 : (p → q → r) ↔ (p ∧ q → r) := by sorry

-- 9. Double negation (classical)
theorem ex9 : ¬¬p ↔ p := by sorry

-- ── Level 4: Challenge ────────────────────────────────────────

-- 10. Peirce's law (requires classical logic or by_contra)
theorem ex10 : ((p → q) → p) → p := by sorry

-- 11. The independence of the conditional
-- Show: (p → q) ∨ (q → p) is a tautology
theorem ex11 : (p → q) ∨ (q → p) := by sorry
