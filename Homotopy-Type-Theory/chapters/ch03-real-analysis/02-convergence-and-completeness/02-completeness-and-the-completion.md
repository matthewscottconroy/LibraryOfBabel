# 2.2 Completeness and the Completion Theorem

## The Gap in the Rationals

Consider the sequence of rational numbers:
$$1, 1.4, 1.41, 1.414, 1.4142, 1.41421, \ldots$$

These are the decimal approximations of $\sqrt{2}$. They form a Cauchy sequence in $\mathbb{Q}$: the terms get arbitrarily close to each other. But they have no limit in $\mathbb{Q}$, because $\sqrt{2} \notin \mathbb{Q}$.

The rationals have *gaps*. Sequences that "should" converge (that satisfy the Cauchy condition) sometimes fail to converge because the limit would be irrational. The real numbers were invented, in part, to fill these gaps.

**Definition.** A metric space $(X, d)$ is *complete* if every Cauchy sequence in $X$ converges to a limit in $X$.

Completeness is one of the most important properties a metric space can have. It means: the metric space is "closed under limits" — if a sequence is trying to converge (by being Cauchy), it actually does converge, and the limit is in the space.

## Examples of Complete and Incomplete Spaces

**Complete spaces:**
- $\mathbb{R}$ with the standard metric (this is the defining property of the real numbers)
- $\mathbb{R}^n$ with any of the standard metrics ($d_1, d_2, d_\infty$)
- Every closed subspace of a complete metric space
- $C([a, b])$ with the sup metric (the Uniform Limit Theorem)
- $\ell^p$ spaces for $1 \leq p \leq \infty$
- Any compact metric space

**Incomplete spaces:**
- $\mathbb{Q}$ with the standard metric (as above)
- Any open interval $(a, b)$ in $\mathbb{R}$ (the sequence $a + 1/n$ is Cauchy but has limit $a \notin (a, b)$)
- $\mathbb{Q}^n$ with the Euclidean metric

## Completeness of $\mathbb{R}$

The completeness of $\mathbb{R}$ is not something we can prove from scratch here — it depends on how $\mathbb{R}$ is constructed. But we can state the key result and connect it to the constructions discussed in Chapter 1.

**Theorem (Completeness of $\mathbb{R}$).** Every Cauchy sequence of real numbers converges.

If $\mathbb{R}$ is constructed via Dedekind cuts: completeness follows because every bounded, non-empty set of rationals has a least upper bound (the Dedekind cut *is* the least upper bound).

If $\mathbb{R}$ is constructed via Cauchy sequences: completeness is essentially built in by construction — the real numbers are defined as equivalence classes of Cauchy sequences of rationals.

This is one of the satisfying features of the Cauchy sequence construction: it makes the completeness of $\mathbb{R}$ almost tautological. The price is that you have to work hard to show the construction gives a field with the right properties.

## Key Theorems About Complete Spaces

**Theorem (Nested closed balls).** A metric space $X$ is complete if and only if: for any sequence of closed balls $\overline{B}(x_n, r_n)$ with $r_n \to 0$ and each ball contained in the previous one, there is exactly one point in their intersection.

This is the metric analog of the nested interval theorem for $\mathbb{R}$: $[a_1, b_1] \supseteq [a_2, b_2] \supseteq \ldots$ with $b_n - a_n \to 0$ implies $\bigcap [a_n, b_n]$ is a single point.

**Theorem (Baire Category Theorem).** If $X$ is a complete metric space, then $X$ is *not* a countable union of nowhere-dense sets.

A set $A$ is *nowhere dense* if its closure has empty interior — it's "thin." The Baire category theorem says complete spaces are "big" in a precise sense: they can't be covered by countably many thin sets.

*Applications of the Baire Category Theorem:*
1. There exist continuous functions $[0, 1] \to \mathbb{R}$ that are nowhere differentiable. (The set of continuous functions with a derivative somewhere is meager in $C([0,1])$.)
2. There is no function $f : \mathbb{R} \to \mathbb{R}$ that is continuous exactly at the rational points. (The set of discontinuity points of any function is an $F_\sigma$ set — a countable union of closed sets — and the rationals cannot form a $G_\delta$ set.)
3. The Uniform Boundedness Principle in functional analysis (a consequence for Banach spaces).

The Baire Category Theorem is a powerful tool that shows "generic" objects in complete metric spaces have complicated properties.

## The Completion Theorem

If a metric space is not complete, can we "add in the missing limits" to make it complete? Yes — this is the *completion* construction.

**Theorem (Completion).** For any metric space $(X, d)$, there exists a complete metric space $(\hat{X}, \hat{d})$ and an isometric embedding $\iota : X \to \hat{X}$ such that:
1. $\iota(X)$ is dense in $\hat{X}$.
2. $\hat{X}$ is unique in the following sense: if $(\hat{X}', \hat{d}')$ is another complete metric space with an isometric embedding $\iota' : X \to \hat{X}'$ satisfying (1), then there is a unique isometry $\phi : \hat{X} \to \hat{X}'$ with $\phi \circ \iota = \iota'$.

$(\hat{X}, \hat{d})$ is the *completion* of $(X, d)$.

The uniqueness clause (2) is a *universal property* — the completion is the unique (up to isometry) complete metric space into which $X$ embeds densely. This is the same pattern we'll see repeatedly: an object is characterized by a universal property rather than by a specific construction.

**Construction.** The completion is constructed just like $\mathbb{R}$ from $\mathbb{Q}$, but for arbitrary metric spaces:
- Take all Cauchy sequences in $X$.
- Define equivalence: $(x_n) \sim (y_n)$ iff $d(x_n, y_n) \to 0$.
- Let $\hat{X}$ = equivalence classes of Cauchy sequences.
- Define $\hat{d}([(x_n)], [(y_n)]) = \lim_{n \to \infty} d(x_n, y_n)$ (the limit exists because $(d(x_n, y_n))$ is Cauchy in $\mathbb{R}$, which is complete).
- Embed $X \hookrightarrow \hat{X}$ by $x \mapsto [(x, x, x, \ldots)]$ (the constant sequence at $x$).

Verifying this works — that $\hat{d}$ is well-defined, is a metric, makes $\hat{X}$ complete, and makes $\iota(X)$ dense — requires careful checking but no deep ideas.

**The real numbers are the completion of $\mathbb{Q}$.** Under the Cauchy sequence construction, $\mathbb{R}$ is exactly the completion of $\mathbb{Q}$. The irrational numbers are the "added" limits of Cauchy sequences of rationals that don't converge in $\mathbb{Q}$.

## Completeness of Function Spaces

One of the most important results in analysis is that $C([a, b])$ is complete.

**Theorem (Uniform Limit Theorem).** If $(f_n)$ is a sequence of continuous functions on $[a, b]$ that converges uniformly to $f$ (i.e., $\sup_x |f_n(x) - f(x)| \to 0$), then $f$ is continuous.

*Proof.* We want to show $f$ is continuous at any $x_0 \in [a, b]$. Let $\varepsilon > 0$.

Since $f_n \to f$ uniformly, there exists $N$ with $\sup_x |f_N(x) - f(x)| < \varepsilon/3$. Since $f_N$ is continuous at $x_0$, there exists $\delta > 0$ with $|f_N(x) - f_N(x_0)| < \varepsilon/3$ whenever $|x - x_0| < \delta$.

For $|x - x_0| < \delta$:
$$|f(x) - f(x_0)| \leq |f(x) - f_N(x)| + |f_N(x) - f_N(x_0)| + |f_N(x_0) - f(x_0)|$$
$$< \varepsilon/3 + \varepsilon/3 + \varepsilon/3 = \varepsilon$$

So $f$ is continuous. $\square$

The $\varepsilon/3$ split here is a standard analysis trick: when you need to estimate a difference using a middle term, split $\varepsilon$ into thirds.

**Corollary.** $C([a, b])$ with the sup metric is complete.

*Proof.* Let $(f_n)$ be Cauchy in $C([a, b])$. For each fixed $x$, $(f_n(x))$ is Cauchy in $\mathbb{R}$ (since $|f_m(x) - f_n(x)| \leq d_\infty(f_m, f_n)$). Since $\mathbb{R}$ is complete, $f_n(x)$ converges to some $f(x)$. This defines $f : [a, b] \to \mathbb{R}$.

The convergence $f_n \to f$ is uniform (exercise: verify this from the Cauchy condition). By the Uniform Limit Theorem, $f$ is continuous. $\square$

## Fixed Points in Complete Spaces

Complete metric spaces are the natural setting for the **Banach Fixed Point Theorem** (also called the Contraction Mapping Theorem), one of the most useful results in analysis.

**Definition.** A function $f : X \to X$ is a *contraction* if there exists $k \in [0, 1)$ with:
$$d(f(x), f(y)) \leq k \cdot d(x, y) \quad \text{for all } x, y \in X$$

A contraction shrinks distances by a factor of at most $k$.

**Theorem (Banach Fixed Point Theorem).** If $(X, d)$ is a complete metric space and $f : X \to X$ is a contraction, then $f$ has a unique fixed point $x^*$ (with $f(x^*) = x^*$), and for any starting point $x_0$, the iteration $x_n = f(x_{n-1})$ converges to $x^*$.

*Proof.* Let $x_0 \in X$ and $x_{n+1} = f(x_n)$. We show $(x_n)$ is Cauchy.

$d(x_{n+1}, x_n) = d(f(x_n), f(x_{n-1})) \leq k \cdot d(x_n, x_{n-1}) \leq \cdots \leq k^n d(x_1, x_0)$.

For $m > n$:
$$d(x_m, x_n) \leq \sum_{j=n}^{m-1} d(x_{j+1}, x_j) \leq \sum_{j=n}^{m-1} k^j d(x_1, x_0) \leq \frac{k^n}{1-k} d(x_1, x_0) \to 0$$

So $(x_n)$ is Cauchy, hence converges to some $x^*$. Since $f$ is continuous (as a contraction, it's Lipschitz), $f(x^*) = f(\lim x_n) = \lim f(x_n) = \lim x_{n+1} = x^*$.

Uniqueness: if $f(x^*) = x^*$ and $f(y^*) = y^*$, then $d(x^*, y^*) = d(f(x^*), f(y^*)) \leq k \cdot d(x^*, y^*)$, so $(1-k)d(x^*, y^*) \leq 0$, giving $d(x^*, y^*) = 0$, so $x^* = y^*$. $\square$

**Applications:**
- Picard's theorem on existence and uniqueness of solutions to ODEs
- Newton's method for finding roots (in the right setting, it's a contraction)
- Iterative algorithms in numerical analysis and machine learning

The Banach Fixed Point Theorem is a wonderful example of how completeness is not just a "nice" property but an essential ingredient in constructive existence proofs.

## Completeness vs. Other Notions

Completeness is a metric property, not a topological one. Two homeomorphic spaces can differ in completeness:
- $(0, 1)$ and $\mathbb{R}$ are homeomorphic but $(0, 1)$ is incomplete while $\mathbb{R}$ is complete.

However, if a metric space is completely metrizable (homeomorphic to a complete metric space), this is a topological property: *Polish spaces* are separable completely metrizable spaces. Polish spaces form a very well-behaved class of topological spaces where descriptive set theory flourishes.

The next section turns to the key bridge between metric spaces and point-set topology: continuity.
