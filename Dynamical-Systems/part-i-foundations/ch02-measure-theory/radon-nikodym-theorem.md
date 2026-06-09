# 2.5 Radon-Nikodym Theorem

This theorem answers a fundamental question: when can one measure be expressed as an integral against another? The answer shapes everything from Lyapunov exponents to entropy theory.

**Definition 2.5.1.** A measure $\nu$ is *absolutely continuous* with respect to $\mu$ (written $\nu \ll \mu$) if $\mu(A) = 0$ implies $\nu(A) = 0$. $\nu$ is *singular* with respect to $\mu$ (written $\nu \perp \mu$) if there exists $A$ with $\mu(A) = 0$ and $\nu(A^c) = 0$.

Absolute continuity means: wherever $\mu$ sees no mass, $\nu$ sees no mass either. Singularity means: $\mu$ and $\nu$ are "supported on disjoint sets" (up to null sets). These are the two extreme cases; every measure can be decomposed into these components.

**Theorem 2.5.2 (Radon-Nikodym).** Let $\mu, \nu$ be $\sigma$-finite measures on $(\Omega, \mathcal{F})$ with $\nu \ll \mu$. Then there exists a unique (a.e.) nonneg measurable function $h = d\nu/d\mu$ such that
$$\nu(A) = \int_A h\,d\mu \quad \text{for all } A \in \mathcal{F}.$$

The function $h = d\nu/d\mu$ is the *Radon-Nikodym derivative* or *density* of $\nu$ with respect to $\mu$.

What this is really saying: if $\nu$ is absolutely continuous with respect to $\mu$, then $\nu$ is just $\mu$ "reweighted" by a density function $h$. The measure of any set under $\nu$ is obtained by integrating the density $h$ against $\mu$ over that set. This generalizes the relationship between a probability distribution and its probability density function from calculus — that's the case where $\mu$ is Lebesgue measure.

The $\sigma$-finite hypothesis is needed to avoid pathological cases where the theorem fails.

**Theorem 2.5.3 (Lebesgue Decomposition).** Every $\sigma$-finite measure $\nu$ can be uniquely written as $\nu = \nu_{\text{ac}} + \nu_{\text{sing}}$ where $\nu_{\text{ac}} \ll \mu$ and $\nu_{\text{sing}} \perp \mu$.

Every measure decomposes into a part that's absolutely continuous with respect to $\mu$ (which has a Radon-Nikodym derivative) and a part that's singular to $\mu$ (which lives on a $\mu$-null set). This is the analog of the unique decomposition of a function into absolutely continuous and singular parts.

**Application in Dynamics.** The Radon-Nikodym derivative appears throughout dynamics:
- The Jacobian $|\det Df|$ is the Radon-Nikodym derivative $d(f_*\lambda)/d\lambda$ — it measures how the map $f$ expands or contracts Lebesgue measure.
- The construction of SRB measures (Sinai-Ruelle-Bowen measures, the "physical" invariant measures of hyperbolic systems) involves finding measures whose conditional distributions on unstable manifolds are absolutely continuous with respect to Lebesgue measure — i.e., finding the Radon-Nikodym derivative on each unstable leaf.
- In entropy theory, the information function $I(A|\mathcal{G})$ is defined via Radon-Nikodym derivatives of conditional measures.

The Radon-Nikodym theorem is also the existence theorem for conditional expectation, as we'll see in Section 2.6.
