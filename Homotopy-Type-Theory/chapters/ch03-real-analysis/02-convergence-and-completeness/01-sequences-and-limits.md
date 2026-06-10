# 2.1 Sequences and Limits

## What Is Convergence?

Here is the central question of analysis: what does it mean for a sequence of points to "approach" a limit?

In calculus, you learned: $\lim_{n \to \infty} a_n = L$ if, for any $\varepsilon > 0$, there exists $N$ such that $|a_n - L| < \varepsilon$ for all $n \geq N$. Translated: eventually, all terms of the sequence are within any prescribed distance $\varepsilon$ of $L$.

In a metric space, this becomes:

**Definition.** Let $(X, d)$ be a metric space. A sequence $(x_n)_{n \geq 1}$ in $X$ *converges* to a point $x \in X$ if:
$$\forall \varepsilon > 0,\, \exists N \in \mathbb{N},\, \forall n \geq N:\, d(x_n, x) < \varepsilon$$

We write $x_n \to x$ or $\lim_{n \to \infty} x_n = x$.

The point $x$ is the *limit* of the sequence.

## Uniqueness of Limits

A basic sanity check: can a sequence have two different limits?

**Theorem.** Limits are unique. If $x_n \to x$ and $x_n \to y$, then $x = y$.

*Proof.* Suppose $x_n \to x$ and $x_n \to y$. Let $\varepsilon > 0$. By convergence:
- There exists $N_1$ with $d(x_n, x) < \varepsilon/2$ for all $n \geq N_1$.
- There exists $N_2$ with $d(x_n, y) < \varepsilon/2$ for all $n \geq N_2$.

Let $N = \max(N_1, N_2)$. For $n \geq N$, by the triangle inequality:
$$d(x, y) \leq d(x, x_n) + d(x_n, y) < \varepsilon/2 + \varepsilon/2 = \varepsilon$$

Since $d(x, y) < \varepsilon$ for all $\varepsilon > 0$ and $d(x, y) \geq 0$, we must have $d(x, y) = 0$, hence $x = y$. $\square$

The $\varepsilon/2$ split is a standard technique: to show $d(x, y) < \varepsilon$, find an intermediate point $x_n$ and use the triangle inequality.

## Sequences and Closed Sets

Closed sets have a natural characterization via sequences.

**Theorem.** A set $F \subseteq X$ is closed if and only if: for every sequence $(x_n)$ in $F$ that converges in $X$, the limit is also in $F$.

*Proof.* ($\Rightarrow$) Suppose $F$ is closed and $x_n \to x$ with all $x_n \in F$. We want $x \in F$. Suppose for contradiction $x \notin F$. Then $x \in X \setminus F$, which is open. So there exists $r > 0$ with $B(x, r) \subseteq X \setminus F$. But since $x_n \to x$, eventually $d(x_n, x) < r$, meaning $x_n \in B(x, r) \subseteq X \setminus F$ — contradicting $x_n \in F$.

($\Leftarrow$) Suppose every convergent sequence in $F$ has its limit in $F$. We show $F$ is closed by showing $X \setminus F$ is open. Take $x \notin F$. We need an open ball around $x$ that stays in $X \setminus F$. 

If no such ball existed, then for each $n$, the ball $B(x, 1/n)$ would contain a point $x_n \in F$. This sequence converges to $x$ (since $d(x_n, x) < 1/n \to 0$). By hypothesis, the limit $x$ would be in $F$ — contradiction. So such a ball exists, and $X \setminus F$ is open. $\square$

This is often used as the *definition* of a closed set in topology: closed = closed under limits of sequences (at least in metric spaces; for general topological spaces, one needs the language of nets or filters).

## Examples of Convergence

**In $\mathbb{R}$:** The sequence $1/n \to 0$. The sequence $(-1)^n$ does not converge.

**In $\mathbb{R}^2$:** The sequence $(1/n, 2/n) \to (0, 0)$, convergence is coordinatewise.

**In $C([0, 1])$ with the sup metric:** The sequence $f_n(x) = x^n$ does not converge uniformly to a continuous function (the "limit" $f(x) = \lim_n x^n$ is $0$ for $x \in [0, 1)$ and $1$ for $x = 1$, which is discontinuous). So $(f_n)$ is not convergent in $C([0, 1])$ with the sup metric.

**In the discrete metric:** A sequence converges iff it is eventually constant: $x_n \to x$ iff there exists $N$ with $x_n = x$ for all $n \geq N$. (Since $d(x_n, x) < 1$ forces $x_n = x$ in the discrete metric.)

## Subsequences

**Definition.** A *subsequence* of $(x_n)$ is a sequence $(x_{n_k})_{k \geq 1}$ where $n_1 < n_2 < n_3 < \cdots$ is a strictly increasing sequence of natural numbers.

A subsequence picks out infinitely many terms of the original sequence in order.

**Proposition.** If $x_n \to x$, then every subsequence $x_{n_k} \to x$.

*Proof.* Given $\varepsilon > 0$, find $N$ with $d(x_n, x) < \varepsilon$ for $n \geq N$. Since $n_k \geq k$ (as $n_k$ is strictly increasing), for $k \geq N$ we have $n_k \geq k \geq N$, so $d(x_{n_k}, x) < \varepsilon$. $\square$

The converse can fail: a sequence can have convergent subsequences without converging itself. The sequence $(-1)^n$ has the subsequences $1, 1, 1, \ldots$ and $-1, -1, -1, \ldots$, converging to different limits, so $(-1)^n$ diverges.

## Cauchy Sequences

We've been defining convergence relative to a limit point. But sometimes we want to say a sequence is "trying to converge" even before we know what the limit is. This is the Cauchy condition.

**Definition.** A sequence $(x_n)$ in $(X, d)$ is a *Cauchy sequence* if:
$$\forall \varepsilon > 0,\, \exists N \in \mathbb{N},\, \forall m, n \geq N:\, d(x_m, x_n) < \varepsilon$$

Cauchy means: the terms eventually become arbitrarily close to each other.

**Theorem.** Every convergent sequence is Cauchy.

*Proof.* Suppose $x_n \to x$. Given $\varepsilon > 0$, find $N$ with $d(x_n, x) < \varepsilon/2$ for $n \geq N$. Then for $m, n \geq N$:
$$d(x_m, x_n) \leq d(x_m, x) + d(x, x_n) < \varepsilon/2 + \varepsilon/2 = \varepsilon$$

So $(x_n)$ is Cauchy. $\square$

The converse is not always true: a Cauchy sequence in a metric space need not converge. The standard example is $\mathbb{Q}$: the sequence $3, 3.1, 3.14, 3.141, 3.1415, \ldots$ (decimal approximations of $\pi$) is Cauchy in $\mathbb{Q}$ but has no limit in $\mathbb{Q}$ (since $\pi \notin \mathbb{Q}$).

This leads directly to the definition of completeness.

## Convergence and Open Sets: The Topological Viewpoint

We defined convergence using the metric. But we can also characterize convergence in terms of open sets — a purely topological notion.

**Proposition.** $x_n \to x$ if and only if for every open set $U$ containing $x$, all but finitely many $x_n$ lie in $U$.

*Proof.* ($\Rightarrow$) If $x_n \to x$ and $U$ is open with $x \in U$, then there's an $\varepsilon > 0$ with $B(x, \varepsilon) \subseteq U$. Find $N$ with $d(x_n, x) < \varepsilon$ for $n \geq N$. Then $x_n \in B(x, \varepsilon) \subseteq U$ for $n \geq N$.

($\Leftarrow$) Given $\varepsilon > 0$, the open ball $B(x, \varepsilon)$ is an open set containing $x$. By hypothesis, all but finitely many $x_n$ are in $B(x, \varepsilon)$, meaning eventually $d(x_n, x) < \varepsilon$. $\square$

This topological characterization of convergence is important: it shows that convergence is determined entirely by the topology (the collection of open sets), not by the metric itself. If two metrics generate the same topology, they have the same convergent sequences.

## Continuity via Sequences

Here's a useful bridge between the sequence world and the function world.

**Theorem.** A function $f : X \to Y$ between metric spaces is continuous at $x \in X$ if and only if: for every sequence $x_n \to x$ in $X$, we have $f(x_n) \to f(x)$ in $Y$.

We'll prove this properly in the continuity section, but the idea is clean: continuity means "preserves convergence."

## Density and Separability

**Definition.** A subset $A \subseteq X$ is *dense* in $X$ if $\overline{A} = X$, equivalently, every non-empty open set meets $A$, equivalently, every point of $X$ is a limit of a sequence in $A$.

$\mathbb{Q}$ is dense in $\mathbb{R}$. The rationals approximate every real number arbitrarily well.

**Definition.** A metric space is *separable* if it contains a countable dense subset.

$\mathbb{R}$ is separable (with $\mathbb{Q}$ dense). $\mathbb{R}^n$ is separable ($\mathbb{Q}^n$ is dense). $C([0, 1])$ with the sup metric is separable (polynomials with rational coefficients are dense, by the Weierstrass Approximation Theorem).

Separability is a "smallness" condition on a metric space. It means the space is well-approximable by a countable collection of points. Separable spaces are well-behaved in many respects: the topology is determined by countable information, and many constructions that require uncountable choices can be made countable.

Non-separable spaces exist: $\ell^\infty(\mathbb{N})$ is not separable (it contains an uncountable set of elements that are all at distance 1 from each other).

## The Interplay Between Algebraic and Analytic Structure

In many of the most important examples, the metric space has additional algebraic structure:
- $(\mathbb{R}, +, \cdot)$ is a field, and the metric is compatible with the field operations.
- $(C([0, 1]), +, \cdot)$ is a ring (under pointwise operations), and convergence is compatible.
- Hilbert spaces have both linear structure and an inner product.

When algebraic and analytic structure are compatible, you get Banach algebras, Lie groups, topological groups, and other rich structures. The interplay between topology and algebra is a recurring theme in this curriculum, culminating in the algebraic topology and HoTT chapters.

The next section takes up the key question: when do Cauchy sequences converge?
