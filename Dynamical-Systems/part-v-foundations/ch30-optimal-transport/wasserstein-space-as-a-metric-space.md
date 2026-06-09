# 30.3 Wasserstein Space as a Metric Space

The Wasserstein distances $W_p$ are not just useful quantities — they define a genuine metric on the space of probability measures, and that metric space has rich geometric structure. Understanding this structure is the key to understanding why so many evolution equations in mathematical physics take their natural form in Wasserstein space.

**Theorem 30.3.1 (Wasserstein Space).** The space $(\mathcal{P}_p(X), W_p)$ of probability measures on a complete separable metric space $X$ with finite $p$-th moments is a complete separable metric space. Convergence in $W_p$ is equivalent to weak convergence plus convergence of $p$-th moments.

Completeness and separability (the Polish space property) make Wasserstein space a well-behaved analytic object. The characterization of $W_p$-convergence is important: it's strictly stronger than weak convergence (which just cares about integrals of continuous bounded functions), but the extra condition — that $p$-th moments converge — is natural and checkable.

For $W_1$, the metric is equivalent to the weak$^*$ topology on the space of distributions. For $W_2$, the geometry is richer, and it's the $W_2$ case where the Riemannian structure and the connection to gradient flows appear.

**Theorem 30.3.2 (Geodesics in Wasserstein Space).** For $\mu, \nu \in \mathcal{P}_2(\mathbb{R}^n)$ with $\mu$ absolutely continuous, the unique $W_2$-geodesic is:
$$\mu_t = ((1-t)\text{id} + t\nabla\phi)_\# \mu, \quad t \in [0,1],$$
where $\nabla\phi$ is the Brenier potential. The geodesic is "straight line" in the sense that mass moves along straight paths.

In plain terms: to travel optimally from $\mu$ to $\nu$ in Wasserstein space, each grain of mass traces a straight line in $\mathbb{R}^n$, from its starting point $x$ to its destination $\nabla\phi(x)$. At time $t$, the mass that started at $x$ is at $(1-t)x + t\nabla\phi(x)$. The geodesic is a linear interpolation between identity and the transport map.

This is exactly the "displacement interpolation" introduced by McCann in 1997. It gives a natural way to interpolate between probability measures — not by mixing them (which would be a straight line in the convex set of measures) but by moving the mass along geodesics. Displacement interpolation is the "metric" interpolation.

**Theorem 30.3.3 (Otto's Riemannian Structure).** The Wasserstein space $(\mathcal{P}_2(\mathbb{R}^n), W_2)$ has a (formal) Riemannian structure: the tangent space at $\mu$ is $\{s: \int s\,d\mu = 0\}$ (zero-mean functions), with inner product:
$$\langle s_1, s_2 \rangle_\mu = \int \nabla\phi_1 \cdot \nabla\phi_2\,d\mu$$
where $\nabla\phi_i$ are the "velocity fields" solving continuity equations. The Riemannian metric gives $W_2$.

Felix Otto introduced this Riemannian structure in 2001, showing that the formal differential geometry of Wasserstein space makes many of the identities of PDE theory into geometric truisms. The word "formal" here means the tangent space is infinite-dimensional and the Riemannian structure requires care to make rigorous; Otto's insight was that working formally in this geometry gives correct results, and the rigorous framework was developed by Ambrosio, Gigli, and Savaré.

The key consequence: PDEs that are "gradient flows" in some informal sense (the solution decreases some functional as fast as possible) become literally gradient flows in the Wasserstein Riemannian geometry. This is the subject of the next section.
