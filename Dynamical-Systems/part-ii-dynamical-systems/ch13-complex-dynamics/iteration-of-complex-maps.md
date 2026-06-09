# 13.1 Iteration of Complex Maps

The setting for this chapter is the Riemann sphere $\hat{\mathbb{C}} = \mathbb{C} \cup \{\infty\}$ and the iteration of rational maps on it. We work over $\mathbb{C}$ rather than $\mathbb{R}$ because complex analysis provides a powerful rigidity tool — holomorphicity — that forces much stronger conclusions than real analysis can. A holomorphic map cannot wiggle freely; it is constrained globally by its local behavior. This is what makes complex dynamics so rich.

**Setup.** Let $f: \hat{\mathbb{C}} \to \hat{\mathbb{C}}$ be a rational map of degree $d \geq 2$ on the Riemann sphere. ("Degree" here means the number of preimages of a generic point, counted with multiplicity.) We study the dynamics of the iteration $f^n = f \circ f \circ \cdots \circ f$.

## The Fatou-Julia Decomposition

The central dichotomy of complex dynamics is between the Fatou set (where orbits are "tame") and the Julia set (where orbits are "wild"). The right way to make this precise is through the concept of normality — the Arzelà-Ascoli condition for families of holomorphic functions.

**Definition 13.1.1.** The *Fatou set* $\mathcal{F}(f) \subseteq \hat{\mathbb{C}}$ is the largest open set on which the family of iterates $\{f^n\}_{n \geq 0}$ is *normal* (every sequence of iterates has a locally uniformly convergent sub-sequence, on any compact subset of $\mathcal{F}(f)$). The *Julia set* is $\mathcal{J}(f) = \hat{\mathbb{C}} \setminus \mathcal{F}(f)$.

**Intuition:** The Fatou set is where nearby orbits behave similarly for all time — where the dynamics is stable and predictable. The Julia set is where the family of iterates fails to be equicontinuous — where nearby orbits can diverge wildly, and the dynamics is chaotic.

## 13.1.1 Basic Properties

These properties are the bedrock on which everything else rests. Prove them once; use them constantly.

**Theorem 13.1.2.**
1. $\mathcal{J}(f)$ is closed, nonempty, and $f(\mathcal{J}(f)) = \mathcal{J}(f)$ (invariant under $f$).
2. $\mathcal{J}(f)$ is perfect (no isolated points) — unless $\mathcal{J}(f) = \hat{\mathbb{C}}$.
3. $f|_{\mathcal{J}(f)}$ is topologically mixing.
4. Repelling periodic orbits are dense in $\mathcal{J}(f)$.
5. $\mathcal{J}(f)$ has empty interior, or $\mathcal{J}(f) = \hat{\mathbb{C}}$.

What this is saying is: the Julia set is the "chaotic core" of the dynamics — a closed, perfect set with dense repelling periodic orbits and mixing dynamics. Points 4 and 3 together make $\mathcal{J}(f)$ Devaney-chaotic in the sense of Chapter 11.

The case $\mathcal{J}(f) = \hat{\mathbb{C}}$ does occur: for example, for Lattès maps (rational maps arising from endomorphisms of elliptic curves), the Julia set is the whole sphere and the Fatou set is empty. These are the "maximally chaotic" rational maps.

## Critical Points and the Fatou-Julia Dichotomy

The critical points of $f$ — the zeros of $f'$ — play a crucial role in determining the global dynamics. For a degree-$d$ rational map, there are $2d-2$ critical points (counted with multiplicity) on $\hat{\mathbb{C}}$ (by the Riemann-Hurwitz formula).

The critical orbits are the "decision points" of the dynamics: where the critical point goes determines what the Julia set looks like.

**Theorem 13.1.3 (Fatou-Julia).** If all critical points of $f$ have bounded orbits — meaning their orbits converge to attracting cycles, or are contained in Siegel disks or Herman rings — then $\mathcal{J}(f)$ is connected. Otherwise, if some critical point has an orbit converging to $\infty$, then $\mathcal{J}(f)$ is a Cantor set.

What this is saying is: the topology of the Julia set (connected vs. Cantor) is entirely determined by the behavior of the critical orbits. For quadratic polynomials, there is a single critical point at $z = 0$, and the dichotomy becomes: $0 \not\to \infty$ iff $\mathcal{J}(f_c)$ is connected iff $c \in \mathcal{M}$ (the Mandelbrot set). The Mandelbrot set is therefore a connectivity locus — the set of parameters where the Julia set is connected.

In the next section, we focus on polynomials, where the structure is richer and more tractable thanks to the special role of $\infty$ as a superattracting fixed point.
