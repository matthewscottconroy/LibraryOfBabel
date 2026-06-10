# De Morgan (Conjunction): Paper Proof

## Theorem
⊢ ¬(P ∧ Q) ↔ (¬P ∨ ¬Q)

## Proof (→ direction): ¬(P ∧ Q) → (¬P ∨ ¬Q)
Assume ¬(P ∧ Q). By the law of excluded middle, either P or ¬P.

**Case 1**: ¬P. Then ¬P ∨ ¬Q holds by ∨I₁.

**Case 2**: P. We show ¬Q. Assume Q. Then P ∧ Q holds (by ∧I). But we assumed ¬(P ∧ Q) — contradiction.
  So ¬Q, and ¬P ∨ ¬Q holds by ∨I₂.

In both cases, ¬P ∨ ¬Q. □

## Proof (← direction): (¬P ∨ ¬Q) → ¬(P ∧ Q)
Assume ¬P ∨ ¬Q. Assume for contradiction that P ∧ Q.
Then P and Q both hold (by ∧E).

**Case 1**: ¬P. P ∧ ¬P — contradiction.
**Case 2**: ¬Q. Q ∧ ¬Q — contradiction.

In both cases, contradiction. So ¬(P ∧ Q). □
