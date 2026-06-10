# Modus Ponens: Paper Proof

## Theorem
P, P → Q ⊢ Q

## Natural Deduction Proof Tree
```
   P        P → Q
   ────────────────  (→E)
            Q
```

## Fitch-Style Proof
```
1. P          (premise)
2. P → Q      (premise)
3. Q          (→E, 1, 2)
```

## Semantic Validity
| P | Q | P→Q | P ∧ (P→Q) | Q |
|---|---|-----|-----------|---|
| T | T |  T  |     T     | T |
| T | F |  F  |     F     | F |
| F | T |  T  |     F     | T |
| F | F |  T  |     F     | F |

In every row where both P and P→Q are true, Q is true. ✓

## Discussion
Modus Ponens is so basic that it is an axiom (or primitive rule) in virtually every
proof system. In sequent calculus it follows from the Cut rule applied to the axiom
P ⊢ P and the left-implication rule.
