# Common FOL Translation Patterns

## A Pattern Library

Translating between English and FOL is a skill built through practice with recurring patterns. Here is a library of the most common.

**Universal affirmative** ("All A's are B"):
$$\forall x\, (A(x) \to B(x))$$

**Particular affirmative** ("Some A's are B"):
$$\exists x\, (A(x) \wedge B(x))$$

**Universal negative** ("No A is B"):
$$\forall x\, (A(x) \to \neg B(x)) \quad \equiv \quad \neg\exists x\, (A(x) \wedge B(x))$$

**Particular negative** ("Some A is not B"):
$$\exists x\, (A(x) \wedge \neg B(x))$$

These are Aristotle's four categorical propositions (A, I, E, O), now given set-theoretic content by FOL.

## Relations and Multi-Place Predicates

Many interesting statements involve relations $R(x, y)$:

| English | FOL |
|---------|-----|
| "$x$ is between $a$ and $b$" | $a < x \wedge x < b$ |
| "There is something bigger than $x$" | $\exists y\, (y > x)$ |
| "$f$ is one-to-one" | $\forall x\, \forall y\, (f(x) = f(y) \to x = y)$ |
| "$R$ is an equivalence relation" | Reflexive $\wedge$ Symmetric $\wedge$ Transitive (each in FOL) |
| "Every integer has a prime factor" | $\forall n\, \exists p\, (\text{Prime}(p) \wedge p \mid n)$ |

## Mathematical Statements as FOL

Classic mathematical definitions translated precisely:

**Continuity** (informal "no breaks"): For each $\varepsilon > 0$ there exists $\delta > 0$ such that if $|x - a| < \delta$ then $|f(x) - f(a)| < \varepsilon$:
$$\forall \varepsilon > 0\, \exists \delta > 0\, \forall x\, (|x - a| < \delta \to |f(x) - f(a)| < \varepsilon)$$

**Density** of $\mathbb{Q}$ in $\mathbb{R}$: Between any two reals there is a rational:
$$\forall x\, \forall y\, (x < y \to \exists q \in \mathbb{Q},\; x < q \wedge q < y)$$

**Injectivity**: $\forall x\, \forall y\, (f(x) = f(y) \to x = y)$

**Surjectivity**: $\forall y\, \exists x\, (f(x) = y)$

## Exercises
See [problems/ch03_first_order_logic/](../../../problems/ch03_first_order_logic/)
