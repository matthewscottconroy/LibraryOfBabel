# 4.1 Compact Spaces

## The Intuition

Compactness is one of the most important and most subtle concepts in topology. It captures a precise mathematical sense in which a space is "finite-like," even if it contains infinitely many points.

Here's the intuitive picture. Consider two spaces: the closed interval $[0, 1]$ and the open interval $(0, 1)$. They look similar — both are intervals of length 1. But they behave very differently:
- A continuous function on $[0, 1]$ always attains its maximum value.
- A continuous function on $(0, 1)$ might not: $f(x) = x$ on $(0, 1)$ approaches 1 but never reaches it.

The difference is that $[0, 1]$ is compact and $(0, 1)$ is not. Compactness is what forces "things to happen" — it prevents sequences from escaping to infinity or to the boundary of the space.

## The Open Cover Definition

**Definition.** An *open cover* of a metric space $X$ is a collection $\{U_\alpha\}$ of open sets whose union is $X$: $\bigcup_\alpha U_\alpha = X$.

A *finite subcover* is a finite sub-collection $U_{\alpha_1}, \ldots, U_{\alpha_n}$ that still covers $X$.

**Definition.** A metric space $X$ is *compact* if every open cover of $X$ has a finite subcover.

This definition takes some getting used to. Let's parse it:
- For every way to cover $X$ with open sets...
- ...you can find finitely many of those open sets that still cover $X$.

The content is the phrase "every open cover." A single cover might happen to have a finite subcover by luck. Compactness says it always happens, no matter what cover you start with.

**Example: $[0, 1]$ is compact.** This is the Heine-Borel theorem (in one dimension). Any open cover of $[0, 1]$ has a finite subcover.

**Example: $(0, 1)$ is not compact.** The open cover $\{(1/n, 1) : n \geq 2\}$ covers $(0, 1)$ (every point $x \in (0, 1)$ is in $(1/n, 1)$ for sufficiently large $n$), but no finite sub-collection covers $(0, 1)$: any finite collection $\{(1/n_1, 1), \ldots, (1/n_k, 1)\}$ misses the points in $(0, 1/\max(n_i)]$.

**Example: $\mathbb{R}$ is not compact.** The cover $\{(-n, n) : n \geq 1\}$ has no finite subcover.

## Heine-Borel Theorem

For subsets of $\mathbb{R}^n$, there's a clean characterization of compactness.

**Theorem (Heine-Borel).** A subset $K \subseteq \mathbb{R}^n$ is compact if and only if it is *closed* and *bounded*.

*Bounded* means contained in some ball $B(0, R)$.

This is a theorem, not a definition — it characterizes compactness for $\mathbb{R}^n$ specifically. For general metric spaces, closed and bounded is not equivalent to compact.

*Proof sketch (for $\mathbb{R}$):*
- **Closed and bounded $\Rightarrow$ compact:** By the Bolzano-Weierstrass theorem, every bounded sequence of real numbers has a convergent subsequence. One can show this implies the open cover property.
- **Compact $\Rightarrow$ bounded:** If $K$ were unbounded, $\{(-n, n) : n \geq 1\}$ would be a cover with no finite subcover.
- **Compact $\Rightarrow$ closed:** If $K$ were not closed, some limit point would be outside $K$, and one can construct a cover with no finite subcover.

## Sequential Compactness

For metric spaces, there is an equivalent formulation that's often easier to use.

**Definition.** A metric space is *sequentially compact* if every sequence has a convergent subsequence.

**Theorem.** For metric spaces, compactness $\Leftrightarrow$ sequential compactness.

This equivalence holds for metric spaces but fails for more general topological spaces (which is one reason the open-cover definition is preferred in general topology).

Sequential compactness is very intuitive: you can't "escape" to infinity in a compact space, because every sequence comes back.

**Bolzano-Weierstrass Theorem.** Every bounded sequence of real numbers has a convergent subsequence.

This is the key fact behind sequential compactness of $[a, b]$. The proof uses the *bisection method*: divide the interval in half, one half must contain infinitely many terms, take that half and repeat.

## Properties of Compact Spaces

**Theorem.** A closed subset of a compact space is compact.

*Proof.* Let $F \subseteq X$ be closed, $X$ compact. Given an open cover $\{U_\alpha\}$ of $F$, add $X \setminus F$ (which is open since $F$ is closed) to get an open cover of $X$. By compactness, finitely many of these cover $X$, and hence $F$. Remove $X \setminus F$ if present to get a finite subcover of $F$. $\square$

**Theorem.** The continuous image of a compact space is compact.

*Proof.* Let $f : X \to Y$ be continuous and $X$ compact. Let $\{V_\alpha\}$ be an open cover of $f(X)$. Then $\{f^{-1}(V_\alpha)\}$ is an open cover of $X$ (each $f^{-1}(V_\alpha)$ is open by continuity of $f$). By compactness of $X$, finitely many of these cover $X$: $X = f^{-1}(V_{\alpha_1}) \cup \cdots \cup f^{-1}(V_{\alpha_n})$. Then $f(X) \subseteq V_{\alpha_1} \cup \cdots \cup V_{\alpha_n}$. $\square$

This is one of the most used results in analysis: images of compact sets under continuous maps are compact.

**Corollary (Extreme Value Theorem).** If $f : X \to \mathbb{R}$ is continuous and $X$ is compact, then $f$ attains its maximum and minimum.

*Proof.* $f(X)$ is compact (as a subset of $\mathbb{R}$, hence closed and bounded), so $\sup f(X)$ is in $f(X)$. $\square$

**Theorem.** A continuous bijection from a compact space to a Hausdorff space is a homeomorphism.

*Proof sketch.* We need to show $f^{-1}$ is continuous, i.e., $f$ maps closed sets to closed sets. A closed subset $F$ of a compact space is compact. $f(F)$ is compact, hence closed (in a Hausdorff space, compact sets are closed). $\square$

This is very useful: to show something is a homeomorphism, you only need to verify it's a continuous bijection (the inverse is automatically continuous).

## Uniform Continuity on Compact Spaces

**Theorem.** A continuous function on a compact metric space is uniformly continuous.

*Proof.* Let $f : X \to Y$ be continuous and $X$ compact. Let $\varepsilon > 0$.

For each $x \in X$, by continuity of $f$ at $x$, there exists $\delta_x > 0$ with $f(B(x, \delta_x)) \subseteq B(f(x), \varepsilon/2)$.

The open balls $\{B(x, \delta_x/2) : x \in X\}$ cover $X$. By compactness, finitely many cover $X$: $X \subseteq B(x_1, \delta_1/2) \cup \cdots \cup B(x_n, \delta_n/2)$ where $\delta_i = \delta_{x_i}$.

Let $\delta = \min(\delta_1/2, \ldots, \delta_n/2) > 0$.

Now suppose $d(x, x') < \delta$. Since $X = \bigcup B(x_i, \delta_i/2)$, we have $x \in B(x_i, \delta_i/2)$ for some $i$. Then:
$$d(x', x_i) \leq d(x', x) + d(x, x_i) < \delta + \delta_i/2 \leq \delta_i/2 + \delta_i/2 = \delta_i$$

So both $x$ and $x'$ are in $B(x_i, \delta_i)$, and $f(x), f(x') \in B(f(x_i), \varepsilon/2)$. By the triangle inequality:
$$d(f(x), f(x')) \leq d(f(x), f(x_i)) + d(f(x_i), f(x')) < \varepsilon/2 + \varepsilon/2 = \varepsilon$$

So $f$ is uniformly continuous. $\square$

## Compactness and the Lebesgue Number Lemma

**Definition.** The *Lebesgue number* of an open cover $\{U_\alpha\}$ of a compact metric space $X$ is a number $\lambda > 0$ such that every ball $B(x, \lambda)$ is contained in some $U_\alpha$.

**Lemma (Lebesgue Number Lemma).** Every open cover of a compact metric space has a positive Lebesgue number.

This lemma says: in a compact space, an open cover isn't just "globally finite" — it has a uniform scale below which every ball is covered by a single set.

The Lebesgue number lemma is used in many proofs, for example in showing that continuous maps between compact spaces are uniformly continuous, and in the proof that compact metric spaces are separable.

## Countability and Compactness

A compact metric space $X$ has several countability properties:
1. $X$ is **separable**: it has a countable dense subset.
2. $X$ is **second countable**: its topology has a countable base (a countable collection of open sets from which all open sets can be built by unions).

These follow because a compact metric space can be covered by finitely many balls of radius $1/n$ for each $n$, giving a countable dense set (by taking the centers).

## Compactness in Infinite Dimensions

Compactness is more delicate in infinite-dimensional spaces. A famous result:

**Theorem.** In an infinite-dimensional normed space, the closed unit ball is not compact.

*Proof.* In $\ell^2$, the sequence $(e_n)$ of standard basis vectors (the $n$-th term is 1 in position $n$, 0 elsewhere) satisfies $\|e_m - e_n\|_2 = \sqrt{2}$ for $m \neq n$. This sequence has no convergent subsequence. $\square$

This is one reason functional analysis is harder than finite-dimensional analysis: compactness arguments that work in $\mathbb{R}^n$ don't automatically transfer to function spaces. The substitute is *relative compactness* and the Arzelà-Ascoli theorem (which characterizes compact subsets of $C([a,b])$).

## Compactness in HoTT

The connection between compactness and HoTT is subtle but deep. In homotopy theory, compact spaces play a special role in the construction of CW complexes (spaces built by attaching cells), and CW complexes are the model for the types in HoTT.

More precisely, the *higher inductive types* (HITs) in HoTT are type-theoretic analogs of CW complexes — they're built by attaching "cells" (constructors) at various dimensions. The theory of HITs mirrors the theory of compact CW complexes in classical homotopy theory.

We'll revisit this connection when we study higher inductive types in Chapter 22.
