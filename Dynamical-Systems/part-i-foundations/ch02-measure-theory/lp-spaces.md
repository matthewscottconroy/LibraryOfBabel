# 2.4 $L^p$ Spaces

The $L^p$ spaces are the natural function spaces of analysis and dynamics. They package the Lebesgue integral into a family of Banach spaces indexed by a parameter $p$, each one capturing a different notion of "size" of a function.

**Definition 2.4.1.** For $1 \leq p < \infty$, the space $L^p(\Omega, \mathcal{F}, \mu)$ consists of (equivalence classes of $\mu$-a.e.-equal) measurable functions $f$ with $\int |f|^p\,d\mu < \infty$, normed by
$$\|f\|_p = \left(\int |f|^p\,d\mu\right)^{1/p}.$$
For $p = \infty$: $L^\infty(\mu)$ consists of essentially bounded functions with $\|f\|_\infty = \text{ess sup}|f|$.

The key word in the definition is "equivalence classes." Two functions that agree $\mu$-almost everywhere are identified — they are the same element of $L^p$. This identification is necessary for the $L^p$ norm to be a genuine norm (otherwise $\|f\|_p = 0$ wouldn't imply $f = 0$, just $f = 0$ a.e.).

The fundamental inequality relating different $L^p$ norms is due to Hölder:

**Theorem 2.4.2 (Hölder's Inequality).** If $1/p + 1/q = 1$ (conjugate exponents), then for $f \in L^p$ and $g \in L^q$:
$$\int |fg|\,d\mu \leq \|f\|_p \|g\|_q.$$

*(proof sketch)* Young's inequality: $ab \leq a^p/p + b^q/q$ for $a, b \geq 0$. Apply to $a = |f|/\|f\|_p$ and $b = |g|/\|g\|_q$.

Hölder's inequality generalizes the Cauchy-Schwarz inequality (the case $p = q = 2$). It's used to bound products: if you know a function is in $L^p$ and another is in $L^q$ with $1/p + 1/q = 1$, their product is integrable. This is the measure-theoretic version of the fact that a bounded function times an integrable function is integrable.

**Theorem 2.4.3 (Minkowski's Inequality).** $\|f + g\|_p \leq \|f\|_p + \|g\|_p$.

This is the triangle inequality for the $L^p$ norm — it's what makes $\|\cdot\|_p$ a genuine norm. For $p \geq 1$, Minkowski's inequality follows from Hölder's.

**Theorem 2.4.4 (Riesz-Fischer).** For $1 \leq p \leq \infty$, $L^p(\mu)$ is a Banach space.

This is the completeness theorem for $L^p$. The proof goes: given a Cauchy sequence in $L^p$, extract a subsequence with fast convergence, show it converges pointwise a.e. using the monotone convergence theorem, then verify the limit is in $L^p$ and the whole sequence converges to it. The dominated convergence theorem handles the interchange of limit and integral.

The duality structure of $L^p$ spaces is one of the beautiful features of the theory:

**Theorem 2.4.5 (Duality).** For $1 \leq p < \infty$, the dual of $L^p(\mu)$ is $L^q(\mu)$ where $1/p + 1/q = 1$: every bounded linear functional on $L^p$ has the form $f \mapsto \int fg\,d\mu$ for a unique $g \in L^q$.

What this says: the "dual" of $L^p$ (the space of bounded linear functionals on it) is exactly $L^q$. Bounded linear functionals are given by integration against a function in the dual space. This is the rigorous version of the physicist's notation $\langle \phi | \psi \rangle$.

**Special Case:** $L^2(\mu)$ is a Hilbert space with inner product $\langle f, g \rangle = \int fg\,d\mu$. When $p = q = 2$, the duality pairing becomes an inner product, and $L^2$ is self-dual. This is the natural function space for spectral analysis of dynamical systems — the Koopman operator acts unitarily on $L^2(\mu)$ when $\mu$ is an invariant measure.

The family $L^1, L^2, L^\infty$ covers the three most important cases: $L^1$ is the space of integrable functions (the most permissive), $L^2$ is the Hilbert space where spectral analysis happens, and $L^\infty$ is the space of essentially bounded functions (the most restrictive). Each one appears in dynamical systems theory in its own right.
