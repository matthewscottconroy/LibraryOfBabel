# Section 1.4: Common Proof Pitfalls

---

## Section Introduction

Knowing the major proof strategies — direct proof, contradiction, induction — is necessary but not sufficient. The harder skill is recognizing when an argument has gone wrong. Mathematics is littered with "proofs" that look right at first glance but contain hidden errors: an implicit assumption that a set is nonempty, a division by a quantity that might be zero, a conflation of the universal quantifier with the existential, a subtle circular argument where the conclusion was silently assumed in the proof itself.

Some of these errors are elementary but seductive. "Proofs" that $1 = 2$ or that all triangles are isosceles have fooled students for generations — their errors are real but carefully hidden. Learning to find the flaw in a fallacious argument trains exactly the same critical faculty needed to verify a correct one. A reader who can spot the missing hypothesis is a reader who understands what the hypothesis is actually doing.

Other pitfalls are more subtle and arise even in professional mathematics. Circular reasoning is the most insidious: a proof that proves $P$ by assuming $P$ in a disguised form. Insufficient generality — proving a special case but claiming a general result — is another persistent error. And the misuse of quantifiers ("there exists a constant $C$ such that..." where $C$ secretly depends on the variable you're quantifying over) has invalidated many an analysis argument.

This section surveys the most common pitfalls with explicit examples of each. For every fallacious argument, we identify precisely where and why it fails. The goal is to make you a skeptical reader of proofs — including your own.

---

## Subsections

- [1.4.1: Circular Reasoning](1.4.1-circular-reasoning.md)
- [1.4.2: Quantifier Errors](1.4.2-quantifier-errors.md)
- [1.4.3: Division by Zero and Vacuous Cases](1.4.3-division-by-zero.md)
- [1.4.4: Incomplete Induction](1.4.4-incomplete-induction.md)
- [1.4.5: Classic Fallacies and Their Analysis](1.4.5-classic-fallacies.md)
