# Quantifiers and Predicates

A predicate is a sentence containing one or more free variables such that substituting specific values for those variables produces a proposition. The sentence "$x^2 > 0$" is a predicate in the variable $x$: substituting $x = 3$ yields the true proposition "$9 > 0$", while substituting $x = 0$ yields the false proposition "$0 > 0$". Predicates are the building blocks of mathematical statements about collections of objects, and quantifiers are the operators that turn predicates into propositions by binding the free variables.

## Universal and Existential Quantifiers

The **universal quantifier** $\forall$ ("for all") binds a variable by asserting that the predicate holds for every element of the domain:
$$\forall x \in S, P(x)$$
is true when $P(x)$ is true for every $x$ in the set $S$, and false when there exists at least one $x \in S$ for which $P(x)$ is false. Such a single counterexample suffices to disprove a universal statement.

The **existential quantifier** $\exists$ ("there exists") asserts that the predicate holds for at least one element:
$$\exists x \in S, P(x)$$
is true when $P(x)$ is true for some $x \in S$, and false when $P(x)$ is false for every $x \in S$.

**Example.** Let $S = \mathbb{R}$ and $P(x)$ be the predicate "$x^2 \geq 0$". Then $\forall x \in \mathbb{R}, x^2 \geq 0$ is true. Now let $Q(x)$ be "$x^2 < 0$". Then $\exists x \in \mathbb{R}, x^2 < 0$ is false.

## Negation of Quantified Statements

The rules for negating quantified statements are the predicate-logic analogs of De Morgan's laws:
$$\neg(\forall x \in S, P(x)) \equiv \exists x \in S, \neg P(x),$$
$$\neg(\exists x \in S, P(x)) \equiv \forall x \in S, \neg P(x).$$

These rules are essential in analysis because many key definitions involve quantifiers, and proofs by contradiction require negating those definitions. Consider the $\varepsilon$-$\delta$ definition of continuity: $f$ is continuous at $a$ if
$$\forall \varepsilon > 0, \exists \delta > 0, \forall x, (|x - a| < \delta \Rightarrow |f(x) - f(a)| < \varepsilon).$$

The negation — "$f$ is discontinuous at $a$" — is:
$$\exists \varepsilon > 0, \forall \delta > 0, \exists x, (|x - a| < \delta \land |f(x) - f(a)| \geq \varepsilon).$$

Getting this negation right requires applying the quantifier negation rules repeatedly, working from the outside in. The final "$\Rightarrow$" becomes "$\land \neg$" by the logical equivalence $\neg(P \Rightarrow Q) \equiv P \land \neg Q$.

## Nested Quantifiers and Order Dependence

When multiple quantifiers appear, their order matters. The statements $\forall x, \exists y, P(x, y)$ and $\exists y, \forall x, P(x, y)$ are logically distinct.

$\forall x, \exists y, P(x, y)$ means: for each $x$ (separately), there is some $y$ (which may depend on $x$) such that $P(x, y)$ holds.

$\exists y, \forall x, P(x, y)$ means: there is a single $y$ that works for all $x$ simultaneously.

The second statement is strictly stronger than the first. This distinction is at the heart of the difference between pointwise and uniform convergence: a sequence of functions $f_n$ converges pointwise to $f$ when
$$\forall x, \forall \varepsilon > 0, \exists N, \forall n > N, |f_n(x) - f(x)| < \varepsilon,$$
while uniform convergence requires
$$\forall \varepsilon > 0, \exists N, \forall x, \forall n > N, |f_n(x) - f(x)| < \varepsilon.$$
The shift of $\exists N$ to the outside means that the same $N$ works for all $x$.

## Proving Quantified Statements

To prove $\forall x \in S, P(x)$, one introduces an arbitrary element $x \in S$ — not a specific one — and shows $P(x)$ holds. The arbitrariness of $x$ is what makes the argument work for all elements.

To prove $\exists x \in S, P(x)$, one exhibits a specific $x$ and verifies $P(x)$.

To disprove $\forall x \in S, P(x)$, one exhibits a counterexample: a specific $x$ with $\neg P(x)$.

To disprove $\exists x \in S, P(x)$, one must show $P(x)$ fails for every $x$ — this is equivalent to proving $\forall x \in S, \neg P(x)$, which is a universal statement and may require a general argument.

**Example.** Prove: for all integers $n \geq 1$, $1 + 2 + \cdots + n = \frac{n(n+1)}{2}$.

This is a universal statement about $\mathbb{N}$ and is proved by induction, a technique formalized in the next section. The predicate is $P(n)$: "$\sum_{k=1}^n k = \frac{n(n+1)}{2}$".

## Bounded Quantifiers and Domain Restrictions

In mathematical writing, quantifiers frequently carry domain restrictions: $\forall x > 0$ is shorthand for $\forall x \in \mathbb{R}, (x > 0 \Rightarrow \ldots)$. Keeping track of these restrictions is important when negating: the negation of $\forall x > 0, P(x)$ is $\exists x > 0, \neg P(x)$, not $\exists x \leq 0, \neg P(x)$.

## Predicates with Multiple Variables

A predicate $P(x, y)$ has two free variables. When one is bound by a quantifier, the result is a predicate in the remaining variable. For example, $\exists y > 0, (x + y = 1)$ is the predicate "$x < 1$" over $\mathbb{R}$: it is true precisely when $x < 1$, since then $y = 1 - x > 0$ is the required witness.

## Common Pitfalls

**Mixing up universal and existential proofs.** To prove a universal statement, you cannot pick a convenient specific $x$; the argument must work for an arbitrary element. Conversely, to prove an existential, guessing the right witness and verifying it is sufficient — a general argument is unnecessary.

**Incorrect negation.** Changing $\forall$ to $\exists$ without also negating the predicate (or vice versa) produces a logically different statement. Both the quantifier and the predicate must change.

**Ignoring quantifier order.** Swapping the order of $\forall$ and $\exists$ changes the meaning of a statement. When writing definitions, quantifier order is not a stylistic choice.

## Connection to Analysis

Every major definition in real analysis is a quantified statement, often with three or four nested quantifiers. The definition of convergence of a sequence, the $\varepsilon$-$\delta$ definition of a limit, the definition of uniform continuity — all are built from $\forall$, $\exists$, and implications. The ability to parse such statements correctly, to negate them accurately, and to construct proofs of them is the single most important skill this chapter develops.
