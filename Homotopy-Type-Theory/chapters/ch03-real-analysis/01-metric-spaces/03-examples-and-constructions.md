# 1.3 Examples and Constructions

## The Zoo of Metric Spaces

Having established the axioms and the basic topology, it's time to populate our zoo. Seeing many examples is essential — each one tests your understanding of the definitions and illuminates a different aspect of the theory. Some will be familiar; others will be surprising.

## Products of Metric Spaces

If $(X, d_X)$ and $(Y, d_Y)$ are metric spaces, we can form their *product* $X \times Y$ in several natural ways.

**The product metric.** The most common choice:
$$d_\infty((x_1, y_1), (x_2, y_2)) = \max(d_X(x_1, x_2), d_Y(y_1, y_2))$$

This is a metric (check the axioms — the triangle inequality follows from the triangle inequalities in $X$ and $Y$). Convergence in this metric is *coordinatewise* convergence: $(x_n, y_n) \to (x, y)$ iff $x_n \to x$ and $y_n \to y$. This is the "right" notion of convergence for pairs.

Other choices like $d_1((x_1, y_1), (x_2, y_2)) = d_X(x_1, x_2) + d_Y(y_1, y_2)$ work equally well and generate the same topology.

## Sequence Spaces

Some of the most important metric spaces in analysis are spaces of sequences.

**$\ell^p$ spaces for $1 \leq p < \infty$.** Let $\ell^p$ be the set of sequences $(a_1, a_2, a_3, \ldots)$ of real numbers with $\sum_{n=1}^\infty |a_n|^p < \infty$. The metric is:
$$d_p(a, b) = \left(\sum_{n=1}^\infty |a_n - b_n|^p\right)^{1/p}$$

For $p = 2$, this is the "$\ell^2$ space" of square-summable sequences. The inner product $\langle a, b \rangle = \sum_n a_n b_n$ makes $\ell^2$ a *Hilbert space* — the prototypical infinite-dimensional analog of Euclidean space. Quantum mechanics is formulated in Hilbert spaces.

**$\ell^\infty$.** The space of bounded sequences, with:
$$d_\infty(a, b) = \sup_{n \geq 1} |a_n - b_n|$$

Convergence here is *uniform* convergence of sequences.

**$c_0$.** The space of sequences converging to zero, also with the sup metric. This is a closed subspace of $\ell^\infty$.

## Function Spaces

Function spaces are metric spaces where the points are functions. They're central in analysis, PDEs, and functional analysis.

**$C([a, b])$:** Continuous real-valued functions on $[a, b]$, with the sup metric:
$$d(f, g) = \sup_{x \in [a, b]} |f(x) - g(x)|$$

Convergence is uniform convergence. This space is complete — the Uniform Limit Theorem says the uniform limit of continuous functions is continuous.

**$C^k([a, b])$:** Functions with $k$ continuous derivatives, with:
$$d(f, g) = \sum_{j=0}^k \sup_{x \in [a,b]} |f^{(j)}(x) - g^{(j)}(x)|$$

Convergence here means the functions and their first $k$ derivatives all converge uniformly.

**$L^p([a, b])$:** Functions with $\int_a^b |f(x)|^p\, dx < \infty$ (integrable to the $p$-th power), with:
$$d_p(f, g) = \left(\int_a^b |f(x) - g(x)|^p\, dx\right)^{1/p}$$

Technically, this requires identifying functions that agree almost everywhere (differing on a set of measure zero). The resulting space is complete — this is the Riesz-Fischer theorem, one of the foundational results of functional analysis.

## Graphs as Metric Spaces

A connected graph $G = (V, E)$ (possibly infinite) becomes a metric space by defining:
$$d(u, v) = \text{length of the shortest path from } u \text{ to } v$$

All four metric axioms are easily verified. The triangle inequality holds because concatenating two paths gives a path.

The Cayley graph of a group (from Chapter 2) becomes a metric space this way, making it possible to study groups geometrically. This is the starting point of *geometric group theory*.

## The Hilbert Cube

The *Hilbert cube* is the set $[0, 1]^{\mathbb{N}}$ — all sequences $(a_1, a_2, a_3, \ldots)$ with $0 \leq a_n \leq 1$ — with metric:
$$d(a, b) = \sum_{n=1}^\infty \frac{|a_n - b_n|}{2^n}$$

The factor $1/2^n$ ensures the series converges. This is a compact metric space (we'll prove this when we discuss compactness), and it's "universal" in the sense that every separable metric space embeds isometrically in the Hilbert cube. This is the Urysohn metrization theorem.

## The Baire Space and Cantor Space

**Cantor space** $\{0, 1\}^{\mathbb{N}}$ is the space of infinite binary sequences, with the metric:
$$d(a, b) = 2^{-n} \text{ where } n \text{ is the first position where } a \text{ and } b \text{ differ}$$

(Define $d(a, a) = 0$.) This is a compact metric space homeomorphic to the Cantor set — the classical middle-thirds fractal in $[0, 1]$. Cantor space is important in descriptive set theory and computability.

**Baire space** $\mathbb{N}^{\mathbb{N}}$ is the space of infinite sequences of natural numbers, with a similar metric. It's a complete metric space (but not compact) and is central in descriptive set theory.

These spaces are where "definable" subsets of complete separable metric spaces — Borel sets, analytic sets, projective sets — live.

## Metrics from Norms

On a vector space $V$, a *norm* $\|\cdot\| : V \to \mathbb{R}_{\geq 0}$ satisfying:
1. $\|v\| = 0 \iff v = 0$
2. $\|\lambda v\| = |\lambda| \|v\|$ for $\lambda \in \mathbb{R}$
3. $\|v + w\| \leq \|v\| + \|w\|$ (the triangle inequality for norms)

induces a metric via $d(v, w) = \|v - w\|$. Such a metric space is called a *normed space*. The three axioms for a metric follow from the norm axioms.

A complete normed space is called a **Banach space**. The spaces $\ell^p$ (for $1 \leq p \leq \infty$) and $C([a,b])$ with the sup norm are all Banach spaces. A Banach space with an inner product whose norm is induced by the inner product is a **Hilbert space**.

## Metrics on Discrete Structures

**Hamming distance** on strings of length $n$ over an alphabet $\Sigma$:
$$d_H(u, v) = |\{i : u_i \neq v_i\}|$$

**Edit distance (Levenshtein distance)** on all strings over $\Sigma$:
$$d_{\text{edit}}(u, v) = \text{minimum number of insertions, deletions, substitutions to convert } u \text{ to } v$$

This is a metric (non-trivial to verify — the triangle inequality requires a careful argument). It's fundamental in computational biology (sequence alignment) and natural language processing.

**Tree metrics.** For a weighted tree, the distance between two leaves is the sum of edge weights on the path between them. Tree metrics are important in phylogenetics and in the study of hyperbolic groups.

## Topological Equivalence: Homeomorphisms

Two metric spaces $(X, d_X)$ and $(Y, d_Y)$ are *homeomorphic* if there is a bijection $f : X \to Y$ such that both $f$ and $f^{-1}$ are continuous. Such an $f$ is a *homeomorphism*.

Homeomorphic spaces have the same topological properties — the same open sets, convergent sequences, connected components, compact subsets. They are "the same space" from a topological point of view, even if they look geometrically different.

**Example:** $(0, 1)$ and $\mathbb{R}$ are homeomorphic, via $f(x) = \tan(\pi(x - 1/2))$.

**Example:** $(0, 1)$ and $[0, 1]$ are *not* homeomorphic. The interval $[0, 1]$ is compact; $(0, 1)$ is not. Homeomorphic spaces must have the same compact subsets.

**Example:** The circle $S^1$ and $[0, 1]$ are not homeomorphic. Removing one point from $[0, 1]$ can disconnect it (remove any interior point); removing one point from $S^1$ leaves a connected space (an arc).

## The Category of Metric Spaces

It's useful to think of metric spaces as forming a *category*:
- **Objects:** metric spaces $(X, d)$
- **Morphisms:** continuous maps (or Lipschitz maps, or isometries, depending on what you want to study)
- **Identity morphism:** the identity function on $X$
- **Composition:** composition of functions

Different choices of morphisms give different categories with different properties. The choice of morphisms determines what "sameness" means. For the coarsest notion (homeomorphism), the morphisms are continuous maps with continuous inverses. For the finest (isometry), the morphisms are distance-preserving bijections.

This categorical perspective will become central in Chapters 9–12, when we study category theory proper.

## Why Examples Matter

Each example in the zoo above illustrates a different phenomenon:
- **Sequence spaces** show that infinite-dimensional spaces behave very differently from finite-dimensional ones.
- **Function spaces** show that "points" can be highly complex objects (entire functions).
- **Discrete structures** show that distance is more general than Euclidean geometry.
- **The Hilbert cube** shows that all separable metric spaces are, in a sense, "the same" (embeddable in one universal space).

The next few sections develop the theory systematically: convergence, completeness, continuity, compactness, and connectedness. Throughout, we'll use these examples as test cases.
