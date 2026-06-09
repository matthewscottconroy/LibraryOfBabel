# 30.2 Brenier's Theorem and Geometry

Kantorovich's formulation made the existence and duality theory for optimal transport clean and tractable. But it didn't say much about the *geometry* of the optimal transport map itself. What does $T^*$ look like? Is it smooth? Does it have a special structure?

For the squared-distance cost — the most natural choice in Euclidean space — Yann Brenier answered this question completely in 1991. The answer revealed an unexpected connection between optimal transport and convex analysis.

**Theorem 30.2.1 (Brenier, 1991).** For $\mu, \nu$ probability measures on $\mathbb{R}^n$ with $\mu$ absolutely continuous w.r.t. Lebesgue, the optimal transport map for cost $c(x,y) = |x-y|^2$ is unique and equals the gradient of a convex function:
$$T^* = \nabla\phi$$
for some convex $\phi: \mathbb{R}^n \to \mathbb{R}$ (the *Brenier potential*). Moreover, $T^*_\# \mu = \nu$ and $T^*$ is the unique map with this property that is a gradient of a convex function.

This is a remarkable geometric statement. Any optimal transport map (for squared-distance cost) is a "conservative" map — it's the gradient of a potential function. Compare to classical mechanics: the force field is the gradient of the potential energy, and the flow it generates is conservative (energy-preserving). Optimal transport is the analogue for probability measures: the "force" moving mass from $\mu$ to $\nu$ is conservative.

The equation $T^* = \nabla\phi$ with the pushforward constraint $T^*_\# \mu = \nu$ becomes the Monge-Ampère equation:
$$\det(D^2\phi(x)) = \frac{\mu(x)}{\nu(\nabla\phi(x))}$$
when $\mu$ and $\nu$ have densities. This is a fully nonlinear PDE, and proving regularity for it was a major achievement of 20th-century analysis.

**Theorem 30.2.2 (Polar Factorization).** Every diffeomorphism $u: \mathbb{R}^n \to \mathbb{R}^n$ (pushing Lebesgue to Lebesgue) factors as:
$$u = \nabla\phi \circ s$$
where $\nabla\phi$ is an optimal transport map and $s$ is a measure-preserving map. This is the *polar factorization* — the analogue of polar decomposition for maps.

In linear algebra, polar decomposition says every matrix $A$ factors as $A = U\Sigma$ where $U$ is orthogonal and $\Sigma$ is positive semidefinite. For maps of $\mathbb{R}^n$, the analogous statement replaces "orthogonal matrix" with "volume-preserving map" and "positive semidefinite matrix" with "gradient of a convex function." The factorization is the Brenier polar factorization.

**Remark 30.2.3.** The Brenier polar factorization shows that optimal transport provides a canonical decomposition of any diffeomorphism into a "gradient part" (conservative, carrying mass efficiently) and a "volume-preserving part" (conservative, rearranging mass without transport cost). This decomposition is the dynamically natural one: the gradient part is the "thermodynamic" component (going downhill in free energy), and the volume-preserving part is the "dynamical" component (symplectic, preserving the natural volume).

This factorization plays an important role in the mathematical foundations of hydrodynamics. Arnold's theorem (Chapter 20) says that ideal fluid flow is geodesic motion on the group of volume-preserving diffeomorphisms. The Brenier polar factorization decomposes any fluid flow into its conservative and volume-preserving components — a decomposition that shows up in the numerical analysis of fluid equations.
