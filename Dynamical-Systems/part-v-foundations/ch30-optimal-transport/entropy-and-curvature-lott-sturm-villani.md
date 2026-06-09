# 30.5 Entropy and Curvature — Lott-Sturm-Villani

Here is one of the most beautiful surprises in 21st-century mathematics. Ricci curvature — a concept from Riemannian geometry, measuring how much parallel geodesics converge or diverge — turns out to be equivalent to a convexity property of entropy in Wasserstein space. This equivalence allows you to define Ricci curvature bounds for spaces that aren't smooth manifolds at all: graphs, metric spaces, fractals.

The key actors are John Lott and Cédric Villani (working jointly) and Karl-Theodor Sturm (working independently), all publishing in 2006.

**Definition 30.5.1 (Displacement Convexity).** A functional $\mathcal{F}: \mathcal{P}_2(X) \to \mathbb{R} \cup \{+\infty\}$ is *$K$-displacement convex* if along any $W_2$-geodesic $(\mu_t)$:
$$\mathcal{F}(\mu_t) \leq (1-t)\mathcal{F}(\mu_0) + t\mathcal{F}(\mu_1) - \frac{K}{2}t(1-t)W_2(\mu_0,\mu_1)^2.$$

This is convexity with a quantitative modulus. Ordinary convexity says $\mathcal{F}$ is below the chord. $K$-displacement convexity says $\mathcal{F}$ is below the chord by a specific amount, controlled by the $W_2$ distance and the parameter $K$.

**Theorem 30.5.2 (Lott-Sturm-Villani, 2006).** A smooth Riemannian manifold $(M, g)$ has Ricci curvature $\geq K$ iff the Boltzmann entropy $S(\mu) = \int \rho\log\rho\,dx$ is $K$-displacement convex in $(\mathcal{P}_2(M), W_2)$.

Let's unpack this. On a positively curved manifold (like a sphere), geodesics converge. This means mass transported along geodesics gets "compressed," which increases its density. Higher density means higher entropy. The curvature bound $\text{Ric} \geq K$ is precisely the quantitative version: entropy gains at least $K/2$ curvature-worth of convexity along displacement interpolations.

On negatively curved spaces (like hyperbolic space), geodesics diverge, mass spreads out, and entropy is more concave. Lower Ricci curvature $=$ less displacement-convexity of entropy.

**Corollary 30.5.3.** The *synthetic Ricci curvature bound* $\text{Ric} \geq K$ for metric measure spaces is defined via displacement convexity of entropy in Wasserstein space. This allows defining Ricci curvature for non-smooth spaces (graphs, fractals, etc.).

The corollary is the payoff. The LSV theorem is an equivalence for smooth manifolds, but the right-hand side — displacement convexity of entropy — makes sense for any metric measure space (a metric space with a reference probability measure). So we take it as the *definition* of Ricci curvature for non-smooth spaces.

This is the synthetic approach to geometry: define a concept by its properties (here, the curvature's effect on entropy convexity) rather than by its construction (here, the tensor formula for Ricci curvature). Synthetic Ricci curvature bounds are stable under Gromov-Hausdorff limits — they survive when the smooth manifold is deformed into a singular space. Classical Ricci curvature doesn't survive such deformations.

The LSV theory connects to some of the deepest questions in geometric analysis: the relationship between spectral gaps and curvature (the Poincaré inequality follows from displacement convexity of entropy), the stability of Ricci flow limits, and the structure of spaces satisfying the Einstein equations with positive cosmological constant.
