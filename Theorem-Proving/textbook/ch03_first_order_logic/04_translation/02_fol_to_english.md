# Translating FOL to Natural Language

## Reading Formulas as Sentences

Translating FOL formulas to clear English prose (and back) is an essential skill. The quantifier structure must be preserved precisely — especially the order of quantifiers, which changes meaning.

**Basic patterns**:
- $\forall x\, P(x)$: "Every x is P" / "All x's are P" / "For any x, P(x)"
- $\exists x\, P(x)$: "Some x is P" / "There exists an x such that P(x)"
- $\forall x\, (P(x) \to Q(x))$: "Every P is Q" / "All P's are Q"
- $\exists x\, (P(x) \wedge Q(x))$: "Some P is Q" / "There exists a P that is also Q"

**Note the structural difference**:
- $\forall x\, (P(x) \to Q(x))$: universal with implication — "all P's are Q"
- $\forall x\, (P(x) \wedge Q(x))$: universal with conjunction — "everything is both P and Q" (stronger!)
- $\exists x\, (P(x) \to Q(x))$: existential with implication — often vacuously true
- $\exists x\, (P(x) \wedge Q(x))$: existential with conjunction — "something is both P and Q" (usually intended)

## Nested Quantifiers in Practice

Order matters critically for nested quantifiers:

| Formula | English |
|---------|---------|
| $\forall x\, \exists y\, L(x, y)$ | Everyone loves someone |
| $\exists y\, \forall x\, L(x, y)$ | Someone is loved by everyone (stronger!) |
| $\forall x\, \forall y\, L(x, y)$ | Everyone loves everyone |
| $\exists x\, \exists y\, L(x, y)$ | Someone loves someone |

The difference between $\forall x\, \exists y$ and $\exists y\, \forall x$ is precisely the difference between "for each $x$, there might be a different $y$" vs. "there is one $y$ that works for all $x$."

## Common English Patterns

| English | FOL |
|---------|-----|
| "Only P's are Q" | $\forall x\, (Q(x) \to P(x))$ |
| "No P is Q" | $\neg\exists x\, (P(x) \wedge Q(x))$ or $\forall x\, (P(x) \to \neg Q(x))$ |
| "P is Q except when R" | $\forall x\, (\neg R(x) \to (P(x) \to Q(x)))$ |
| "Exactly one x is P" | $\exists x\, (P(x) \wedge \forall y\, (P(y) \to y = x))$ |

## Exercises
See [problems/ch03_first_order_logic/](../../../problems/ch03_first_order_logic/)
