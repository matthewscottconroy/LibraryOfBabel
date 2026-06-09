# 1.8 The Stone-Weierstrass Theorem

We close this chapter with a theorem about approximation — about how much you can do with "small" algebras of functions.

The Weierstrass approximation theorem from classical analysis says that every continuous function on a closed interval can be approximated uniformly by polynomials. Stone's generalization asks: what's the minimal structure a collection of functions needs to have in order to be dense? The answer is elegant and widely applicable.

**Theorem 1.8.1 (Stone-Weierstrass).** Let $K$ be a compact Hausdorff space and $\mathcal{A} \subseteq C(K, \mathbb{R})$ a subalgebra that separates points (for $x \neq y$, some $f \in \mathcal{A}$ has $f(x) \neq f(y)$) and contains the constant functions. Then $\mathcal{A}$ is dense in $C(K, \mathbb{R})$ under the uniform norm.

Two conditions: the algebra separates points, and it contains constants. That's all you need. The separating condition is what prevents the algebra from being "blind" to part of the space; constants are needed so you can approximate the values themselves, not just relative comparisons.

**Corollary 1.8.2.** Polynomials are dense in $C([a,b])$. Trigonometric polynomials are dense in $C(\mathbb{T})$ where $\mathbb{T} = \mathbb{R}/\mathbb{Z}$.

The first corollary is Weierstrass's original theorem. The second says that every continuous function on the circle can be approximated by finite Fourier sums — a basic fact in harmonic analysis.

**Application in Dynamics.** Stone-Weierstrass implies that to specify a measure on a compact space, it suffices to specify its integrals against polynomials or trigonometric polynomials. If $\int f\,d\mu = \int f\,d\nu$ for all polynomials $f$, then $\mu = \nu$ as measures. This is the basis for the *moment problem*: determining a measure from its moments. It's also the basis for approximating invariant measures: if you can compute the integrals of polynomials against an approximate invariant measure, Stone-Weierstrass tells you those integrals determine the measure.

This theorem is less central to what follows than, say, Arzelà-Ascoli or the Baire Category Theorem, but it illustrates a recurring theme: density results for function algebras are the key to reducing questions about all continuous functions to questions about a small, tractable subclass.

With this, we have the foundational vocabulary in place. Metric spaces, compactness, continuity, completeness, Banach and Hilbert spaces, genericity. What comes next — measure theory, topology, ODEs, linear algebra — builds directly on this framework. Every major theorem in dynamical systems is ultimately proved by combining these ingredients in some new way.
