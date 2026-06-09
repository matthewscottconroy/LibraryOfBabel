# 13.5 Quasiconformal Maps

Sullivan's proof of No Wandering Domains — and essentially all modern complex dynamics — relies on a generalization of conformal maps called *quasiconformal maps*. A conformal map preserves angles exactly; a quasiconformal map is allowed to distort angles, but by a bounded amount. This small concession opens up an enormous toolkit.

The key idea is: instead of requiring $\bar\partial f = 0$ (the Cauchy-Riemann equations), we allow $\bar\partial f = \mu \partial f$ where $|\mu| < k < 1$. The coefficient $\mu$ is called the *Beltrami coefficient*, and it measures the infinitesimal distortion of the map at each point.

**Definition 13.5.1.** A homeomorphism $\phi: U \to V$ between open sets in $\mathbb{C}$ is *$K$-quasiconformal* ($K \geq 1$) if it is ACL (absolutely continuous on lines — a weak differentiability condition) and satisfies the *Beltrami inequality* a.e.:
$$|\bar{\partial}\phi| \leq k |\partial\phi| \quad \text{a.e.}, \quad k = \frac{K-1}{K+1} < 1,$$
where $\partial = \frac{1}{2}\!\left(\frac{\partial}{\partial x} - i\frac{\partial}{\partial y}\right)$ and $\bar\partial = \frac{1}{2}\!\left(\frac{\partial}{\partial x} + i\frac{\partial}{\partial y}\right)$ are the complex derivatives.

Conformal maps ($\bar\partial \phi = 0$ everywhere) are $1$-quasiconformal. A $K$-quasiconformal map distorts infinitesimal circles into infinitesimal ellipses with eccentricity at most $K$.

The reason quasiconformal maps are so useful in dynamics is that they compose, and that they can be *constructed* from their Beltrami coefficients via the Measurable Riemann Mapping Theorem. This allows a beautiful method: to construct a conformal conjugacy between two dynamical systems, first construct a quasiconformal one, then show the Beltrami coefficient must vanish.

**Theorem 13.5.2 (Measurable Riemann Mapping Theorem).** Let $\mu: \mathbb{C} \to \mathbb{D}$ be a measurable function with $\|\mu\|_\infty \leq k < 1$ (a *Beltrami coefficient*). Then there exists a unique quasiconformal homeomorphism $\phi: \mathbb{C} \to \mathbb{C}$ (fixing $0, 1, \infty$) solving the *Beltrami equation*:
$$\bar{\partial}\phi = \mu \partial\phi.$$

What this is saying is: you can prescribe the infinitesimal distortion of a homeomorphism — how circles are distorted to ellipses at each point — and the Measurable Riemann Mapping Theorem integrates this data to give an actual homeomorphism. The condition $\|\mu\|_\infty < 1$ ensures the resulting map is quasiconformal (bounded distortion) rather than just a measurable homeomorphism.

This theorem — also called the Ahlfors-Bers theorem — is the analytic core of Sullivan's proof of No Wandering Domains, and it underlies the entire theory of quasiconformal surgery: the technique of cutting along curves and gluing together dynamical systems by quasiconformal maps. It was known in complex analysis long before Sullivan applied it to dynamics, but Sullivan's application transformed both subjects.

**A key application: constructing conjugacies.** If $f$ and $g$ are two rational maps, and we can find a quasiconformal homeomorphism $\phi$ with $\phi \circ f = g \circ \phi$, and if we can show that the Beltrami coefficient of $\phi$ is $f$-invariant and hence must be 0 a.e. (by the ergodicity of the Julia set dynamics), then $\phi$ is actually conformal — a genuine conjugacy. This scheme, applied repeatedly, gives proofs of rigidity theorems: under appropriate hypotheses, quasiconformally equivalent maps are in fact conformally equivalent.

This rigidity philosophy will appear again in Section 13.6, where the Straightening Theorem converts polynomial-like maps to genuine polynomials via a quasiconformal conjugacy.
