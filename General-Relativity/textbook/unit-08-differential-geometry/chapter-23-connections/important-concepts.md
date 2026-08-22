# Chapter 23: Important Concepts

---

**Connection**
A rule for differentiating tensor fields on a manifold that (1) satisfies the Leibniz rule, (2) is $C^\infty$-linear in the direction of differentiation, and (3) reduces to partial differentiation on scalars. Specifying a connection is equivalent to specifying the Christoffel symbols $\Gamma^\rho_{\mu\nu}$ in any coordinate chart. A manifold carries infinitely many connections; the Levi-Civita connection is the canonical one for a (pseudo-)Riemannian manifold.

**Covariant Derivative**
$\nabla_\nu V^\rho = \partial_\nu V^\rho + \Gamma^\rho_{\nu\mu}V^\mu$ (for a vector field): the tensor-valued derivative that corrects for the change of basis vectors. Transforms as a tensor of type $(1,1)$ under coordinate changes, unlike $\partial_\nu V^\rho$. The $+\Gamma$ term for contravariant (upper) indices and $-\Gamma$ for covariant (lower) indices follow from the Leibniz rule.

**Levi-Civita Connection**
The unique connection on a (pseudo-)Riemannian manifold that is (1) metric-compatible ($\nabla_\rho g_{\mu\nu} = 0$) and (2) torsion-free ($\Gamma^\rho_{\mu\nu} = \Gamma^\rho_{\nu\mu}$). The uniqueness is the "fundamental theorem of Riemannian geometry." The Christoffel symbols are expressed in terms of first derivatives of the metric.

**Christoffel Symbols**
$\Gamma^\rho_{\mu\nu} = \frac{1}{2}g^{\rho\sigma}(\partial_\mu g_{\nu\sigma} + \partial_\nu g_{\mu\sigma} - \partial_\sigma g_{\mu\nu})$: the components of the Levi-Civita connection in a coordinate basis. Not a tensor (transforms with an inhomogeneous term). Can be set to zero at any single point (normal coordinates), but not globally unless the space is flat. Appear in the geodesic equation, the covariant derivative, and the Riemann tensor.

**Metric Compatibility**
$\nabla_\rho g_{\mu\nu} = 0$: the metric is "covariantly constant." Implies that parallel transport preserves inner products (lengths and angles). The equation of motion for a massive particle preserves $u_\mu u^\mu = -c^2$ along geodesics precisely because $\nabla_\rho g_{\mu\nu} = 0$.

**Torsion**
For a general connection: $T^\rho_{\ \mu\nu} = \Gamma^\rho_{\mu\nu} - \Gamma^\rho_{\nu\mu}$. The torsion tensor measures the "twisting" of the connection. The Levi-Civita connection has zero torsion. Cartan's torsion tensor is related to the antisymmetric part of the connection; it appears in theories of gravity with torsion (Einstein-Cartan theory, where torsion is sourced by fermion spin density).

**Parallel Transport**
Moving a vector along a curve while keeping it "constant" relative to the curved geometry: $DV^\mu/d\lambda = \dot{x}^\nu\nabla_\nu V^\mu = 0$ (the covariant derivative along the curve vanishes). On flat space: identical to keeping the vector constant in direction and magnitude. On a curved manifold: the vector "rotates" relative to the flat embedding. The holonomy (rotation acquired after transport around a closed loop) measures curvature.

**Geodesic**
A curve whose tangent vector is parallel-transported along itself: $D\dot{x}^\mu/d\lambda = \ddot{x}^\mu + \Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$. The "straightest possible path" in a curved manifold. In GR: the worldlines of freely falling particles (massive or massless). Geodesics maximize proper time (principle of maximal aging) and extremize arc length.

**Normal Coordinates**
At any event $p$, coordinates can be chosen such that $g_{\mu\nu}(p) = \eta_{\mu\nu}$ and $\Gamma^\rho_{\mu\nu}(p) = 0$ (but second derivatives of $g_{\mu\nu}$ and hence the Riemann tensor do not vanish). These are "locally inertial" coordinates — the mathematical realization of the equivalence principle. The geodesic equation at $p$ reduces to $d^2x^\mu/d\tau^2 = 0$ in normal coordinates.

**Geodesic Completeness**
A geodesic is complete if it can be extended to all values of its affine parameter. A spacetime is geodesically incomplete if some geodesic cannot be extended past a finite value — the geodesic "hits a singularity." The Penrose-Hawking singularity theorems prove geodesic incompleteness under physically reasonable conditions, without requiring specific forms for the singularity.

**Killing Vectors**
Vector fields $\xi^\mu$ satisfying $\nabla_{(\mu}\xi_{\nu)} = 0$ (the Killing equation). Each Killing vector corresponds to a symmetry of the metric (an isometry) and gives a conserved quantity along geodesics: $p_\mu\xi^\mu =$ const. The Schwarzschild metric has 4 Killing vectors (time translation + 3 rotations); the Kerr metric has 2 (time translation + axial rotation). More symmetry → more conserved quantities → more tractable geodesic equations.
