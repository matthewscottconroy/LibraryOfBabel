# Riemannian Metrics

A smooth manifold carries topological and differential structure, but no intrinsic notion of distance or angle. A Riemannian metric adds this: it is a smoothly varying inner product on each tangent space, allowing one to measure lengths of tangent vectors and hence lengths of curves. With a Riemannian metric, one can define geodesics (shortest paths), curvature, volumes, and the Laplace-Beltrami operator. Riemannian geometry is the framework for general relativity, comparison geometry, geometric analysis, and the study of shapes in higher dimensions.

## Definition

**Definition.** A **Riemannian metric** on a smooth manifold $M$ is a smooth assignment $g: M \to T^{(0,2)}M$ of a positive definite symmetric bilinear form $g_p: T_pM \times T_pM \to \mathbb{R}$ to each point $p \in M$, varying smoothly with $p$.

"Smooth" means: in every local chart $(U, x^1, \ldots, x^n)$, the functions $g_{ij}(x) = g_p\left(\frac{\partial}{\partial x^i}\bigg|_p, \frac{\partial}{\partial x^j}\bigg|_p\right)$ are smooth. The matrix $(g_{ij})$ is symmetric and positive definite at every point.

A manifold equipped with a Riemannian metric is a **Riemannian manifold** $(M, g)$.

**Existence.** Every smooth manifold admits a Riemannian metric. Proof: use a partition of unity $\{\psi_\alpha\}$ subordinate to an atlas $\{(U_\alpha, \phi_\alpha)\}$, and define $g = \sum_\alpha \psi_\alpha (\phi_\alpha^* g_{\text{Eucl}})$, where $g_{\text{Eucl}}$ is the Euclidean metric on $\mathbb{R}^n$. Each term $\psi_\alpha (\phi_\alpha^* g_{\text{Eucl}})$ is a positive semi-definite smooth tensor; the sum is positive definite since the $\psi_\alpha$ form a partition of unity.

## Examples

**Euclidean space.** On $\mathbb{R}^n$, $g = \sum_i dx^i \otimes dx^i$ (i.e., $g_{ij} = \delta_{ij}$). This is the standard inner product.

**Submanifold metric.** If $M \subset \mathbb{R}^N$ is a smooth submanifold, the restriction of the Euclidean inner product to each $T_pM \subset T_p\mathbb{R}^N = \mathbb{R}^N$ is a Riemannian metric on $M$. This is the induced (or pullback) metric. For surfaces in $\mathbb{R}^3$, it is the first fundamental form studied in Chapter 1.

**Sphere $S^n$.** The standard round metric on $S^n$ is the induced metric from the embedding $S^n \hookrightarrow \mathbb{R}^{n+1}$.

**Hyperbolic space $\mathbb{H}^n$.** On the upper half-space $\{(x^1, \ldots, x^n) \in \mathbb{R}^n : x^n > 0\}$, the hyperbolic metric is $g_{ij} = \delta_{ij}/(x^n)^2$. This gives a complete Riemannian manifold of constant sectional curvature $-1$, the model for hyperbolic geometry.

**Lie group metrics.** On a Lie group $G$ with Lie algebra $\mathfrak{g} = T_eG$, a choice of inner product on $\mathfrak{g}$ extends to a left-invariant metric on $G$ by requiring $g_p(u,v) = \langle (dL_{p^{-1}})_p u, (dL_{p^{-1}})_p v \rangle_e$. This makes $G$ a Riemannian manifold on which the left translations are isometries.

## Geodesics

A **geodesic** on $(M, g)$ is a smooth curve $\gamma: I \to M$ satisfying the geodesic equation

$$\nabla_{\dot\gamma} \dot\gamma = 0,$$

where $\nabla$ is the Levi-Civita connection. In local coordinates, this is:

$$\ddot{x}^k + \Gamma^k_{ij} \dot{x}^i \dot{x}^j = 0, \quad k = 1, \ldots, n,$$

with Christoffel symbols $\Gamma^k_{ij} = \frac{1}{2}g^{kl}(\partial_i g_{jl} + \partial_j g_{il} - \partial_l g_{ij})$ (same formula as for surfaces). Geodesics locally minimize length and are the manifold analogue of straight lines.

**Geodesic completeness.** A Riemannian manifold is **complete** if every geodesic can be extended to all of $\mathbb{R}$. By the Hopf-Rinow theorem, completeness is equivalent to the manifold being complete as a metric space (with distance defined as infimum of path lengths). Compact manifolds are always complete.

## The Levi-Civita Connection

The Riemannian metric uniquely determines a **connection** $\nabla$ (covariant derivative) called the **Levi-Civita connection**, characterized by:
1. **Compatibility with the metric:** $\nabla g = 0$ (parallel transport preserves inner products).
2. **Torsion-free:** $\nabla_X Y - \nabla_Y X = [X, Y]$ for all vector fields $X, Y$.

In coordinates: $\nabla_{\partial_i} \partial_j = \sum_k \Gamma^k_{ij} \partial_k$.

## Riemannian Curvature

The **Riemann curvature tensor** measures the failure of covariant derivatives to commute:

$$R(X, Y)Z = \nabla_X \nabla_Y Z - \nabla_Y \nabla_X Z - \nabla_{[X,Y]} Z.$$

In coordinates: $R^l_{kij} = \partial_i \Gamma^l_{jk} - \partial_j \Gamma^l_{ik} + \Gamma^m_{jk}\Gamma^l_{im} - \Gamma^m_{ik}\Gamma^l_{jm}$.

The **sectional curvature** of a 2-plane $\sigma \subset T_pM$ spanned by orthonormal vectors $\{e_1, e_2\}$ is $K(\sigma) = g(R(e_1, e_2)e_2, e_1)$, generalizing Gaussian curvature. The **Ricci tensor** $\text{Ric}(X,Y) = \text{tr}(Z \mapsto R(Z,X)Y)$ and **scalar curvature** $s = \text{tr}_g(\text{Ric})$ are further contractions.

## The Laplace-Beltrami Operator

For a Riemannian manifold $(M, g)$, the **Laplace-Beltrami operator** generalizes the Euclidean Laplacian:

$$\Delta_g f = \text{div}(\text{grad}\, f) = \frac{1}{\sqrt{\det g}} \sum_{i,j} \partial_i\left(\sqrt{\det g} \, g^{ij} \partial_j f\right).$$

This is a second-order elliptic operator on $(M, g)$, and its spectrum (eigenvalues of $-\Delta_g$) encodes geometric information about the manifold—the celebrated question "Can you hear the shape of a drum?" asks whether the spectrum of $-\Delta_g$ determines the geometry of $M$.

The Laplace-Beltrami operator is the central operator in geometric PDE: the heat equation $\partial_t u = \Delta_g u$, the wave equation $\partial_{tt} u = \Delta_g u$, and the Schrödinger equation on a Riemannian manifold all use $\Delta_g$. Its spectral theory (developed in Unit 3) is the foundation for Fourier analysis on curved spaces.
