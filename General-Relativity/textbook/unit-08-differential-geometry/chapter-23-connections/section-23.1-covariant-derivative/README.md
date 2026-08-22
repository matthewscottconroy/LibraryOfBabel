# Section 23.1: The Covariant Derivative and Parallel Transport

---

## Section Introduction

The partial derivative $\partial_\mu V^\nu$ of a vector field fails to be a tensor: under the coordinate change $x^\mu \to x'^\mu$, it transforms as:

$$\partial'_\mu V'^\nu = \frac{\partial x^\rho}{\partial x'^\mu}\frac{\partial x'^\nu}{\partial x^\sigma}\partial_\rho V^\sigma + \frac{\partial x^\rho}{\partial x'^\mu}\frac{\partial^2 x'^\nu}{\partial x^\rho\partial x^\sigma}V^\sigma$$

The second term (with second derivatives of the coordinate transformation) spoils the tensor transformation law. It arises because the basis vectors themselves change from point to point.

The **covariant derivative** $\nabla_\mu V^\nu$ is defined to transform as a tensor by adding a correction term (the Christoffel symbol) that exactly cancels the bad term. This section constructs the covariant derivative, discusses its properties, and introduces parallel transport — the geometric operation that defines the connection.

---

## 23.1.1 Motivation: Failure of Partial Derivatives

**In polar coordinates**: Consider a vector field $\mathbf{V} = V^r\hat{\mathbf{r}} + V^\phi\hat{\boldsymbol{\phi}}$ in 2D. The partial derivative $\partial_\phi V^r$ counts the rate of change of the radial component — but it doesn't account for the fact that $\hat{\mathbf{r}}$ itself rotates as $\phi$ changes. The "true" rate of change of $\mathbf{V}$ in the $\phi$-direction includes an extra term from the rotation of the basis vectors.

In polar coordinates: $\partial\hat{\mathbf{r}}/\partial\phi = \hat{\boldsymbol{\phi}}$ and $\partial\hat{\boldsymbol{\phi}}/\partial\phi = -\hat{\mathbf{r}}$. So the derivative of $\mathbf{V}$ in the $\phi$-direction is:

$$\frac{\partial\mathbf{V}}{\partial\phi} = \left(\frac{\partial V^r}{\partial\phi} - V^\phi\right)\hat{\mathbf{r}} + \left(\frac{\partial V^\phi}{\partial\phi} + \frac{V^r}{r}\right)\hat{\boldsymbol{\phi}}$$

(The extra terms $-V^\phi$ and $+V^r/r$ come from the rotation of the basis.) This is precisely the covariant derivative.

---

## 23.1.2 Definition of the Covariant Derivative

A **connection** $\nabla$ on a manifold is a rule for differentiating tensor fields that satisfies:
1. **Linearity**: $\nabla_X(T + S) = \nabla_X T + \nabla_X S$
2. **Leibniz rule**: $\nabla_X(T\otimes S) = (\nabla_X T)\otimes S + T\otimes(\nabla_X S)$
3. **$C^\infty$-linearity in $X$**: $\nabla_{fX+gY}T = f\nabla_X T + g\nabla_Y T$ for functions $f$, $g$

On a coordinate chart, specifying a connection is equivalent to giving the **Christoffel symbols** $\Gamma^\rho_{\mu\nu}$, defined by:

$$\nabla_\mu \mathbf{e}_\nu = \Gamma^\rho_{\mu\nu}\mathbf{e}_\rho$$

(the covariant derivative of the $\nu$-th basis vector in the $\mu$-direction is a linear combination of basis vectors).

**Covariant derivative of a vector field**: $V = V^\mu\mathbf{e}_\mu$ (sum over $\mu$). Then:

$$\nabla_\nu V = \nabla_\nu(V^\mu\mathbf{e}_\mu) = (\partial_\nu V^\mu)\mathbf{e}_\mu + V^\mu\Gamma^\rho_{\nu\mu}\mathbf{e}_\rho = (\partial_\nu V^\rho + \Gamma^\rho_{\nu\mu}V^\mu)\mathbf{e}_\rho$$

So the components of $\nabla_\nu V$ are:

$$(\nabla_\nu V)^\rho \equiv \nabla_\nu V^\rho = \partial_\nu V^\rho + \Gamma^\rho_{\nu\mu}V^\mu$$

**Covariant derivative of a covector** (1-form): By requiring the Leibniz rule to be compatible with $\omega_\mu V^\mu =$ scalar (so $\nabla_\nu(\omega_\mu V^\mu) = \partial_\nu(\omega_\mu V^\mu)$):

$$\nabla_\nu\omega_\mu = \partial_\nu\omega_\mu - \Gamma^\rho_{\nu\mu}\omega_\rho$$

(Note: $+\Gamma$ for contravariant (upper) indices, $-\Gamma$ for covariant (lower) indices.)

**General tensor**: A tensor $T^{\mu\cdots}_{\ \ \nu\cdots}$ of type $(r,s)$:

$$\nabla_\rho T^{\mu_1\cdots\mu_r}_{\ \ \ \ \ \nu_1\cdots\nu_s} = \partial_\rho T^{\mu_1\cdots}_{\ \ \ \ \nu_1\cdots} + \sum_{i=1}^r \Gamma^{\mu_i}_{\rho\sigma}T^{\mu_1\cdots\sigma\cdots}_{\ \ \ \ \nu_1\cdots} - \sum_{j=1}^s \Gamma^\sigma_{\rho\nu_j}T^{\mu_1\cdots}_{\ \ \ \ \nu_1\cdots\sigma\cdots}$$

---

## 23.1.3 The Levi-Civita Connection

Of the infinitely many connections on a manifold, the **Levi-Civita connection** is the unique one satisfying:

1. **Metric compatibility**: $\nabla_\rho g_{\mu\nu} = 0$ (the metric is "covariantly constant" — lengths and angles are preserved under parallel transport)

2. **Torsion-free**: $\Gamma^\rho_{\mu\nu} = \Gamma^\rho_{\nu\mu}$ (symmetric in lower indices; no "twisting")

*Proof of uniqueness*: Write out $\nabla_\rho g_{\mu\nu} = 0$ with the Christoffel symbols. Cyclically permute the indices $(\rho, \mu, \nu)$:

$$\partial_\rho g_{\mu\nu} = \Gamma^\sigma_{\rho\mu}g_{\sigma\nu} + \Gamma^\sigma_{\rho\nu}g_{\sigma\mu}$$
$$\partial_\mu g_{\nu\rho} = \Gamma^\sigma_{\mu\nu}g_{\sigma\rho} + \Gamma^\sigma_{\mu\rho}g_{\sigma\nu}$$
$$\partial_\nu g_{\rho\mu} = \Gamma^\sigma_{\nu\rho}g_{\sigma\mu} + \Gamma^\sigma_{\nu\mu}g_{\sigma\rho}$$

Add the first two and subtract the third. Using torsion-free ($\Gamma^\sigma_{\mu\nu} = \Gamma^\sigma_{\nu\mu}$):

$$\partial_\rho g_{\mu\nu} + \partial_\mu g_{\nu\rho} - \partial_\nu g_{\rho\mu} = 2\Gamma^\sigma_{\rho\mu}g_{\sigma\nu}$$

Solving for $\Gamma$:

$$\boxed{\Gamma^\rho_{\mu\nu} = \frac{1}{2}g^{\rho\sigma}(\partial_\mu g_{\nu\sigma} + \partial_\nu g_{\mu\sigma} - \partial_\sigma g_{\mu\nu})}$$

These are the **Christoffel symbols** (Section 23.2). The uniqueness of the Levi-Civita connection is the "fundamental theorem of Riemannian geometry." □

---

## 23.1.4 Parallel Transport

**Definition**: A vector field $V^\mu$ is **parallel transported** along a curve $x^\mu(\lambda)$ if its covariant derivative along the curve vanishes:

$$\frac{DV^\mu}{d\lambda} \equiv \frac{dx^\nu}{d\lambda}\nabla_\nu V^\mu = \frac{dV^\mu}{d\lambda} + \Gamma^\mu_{\nu\rho}\frac{dx^\nu}{d\lambda}V^\rho = 0$$

This is a system of ODEs (one for each component of $V^\mu$). Given initial data $V^\mu(x_0)$, there is a unique solution along the curve — **the parallel transport** of $V^\mu(x_0)$ along $x(\lambda)$.

**Geometric picture**: Parallel transporting a vector means "carrying it along the curve without rotating or stretching it, relative to the curved geometry." On flat space: parallel transport is just holding the vector constant in direction and magnitude. On a curved surface (e.g., the sphere): the vector rotates relative to the flat embedding, but is "held constant" relative to the intrinsic geometry.

**Metric compatibility**: For the Levi-Civita connection, parallel transport preserves the inner product:

$$\frac{d}{d\lambda}(g_{\mu\nu}V^\mu W^\nu) = \frac{d}{d\lambda}(V\cdot W) = 0$$

if $V^\mu$ and $W^\mu$ are both parallelly transported. (Follows from $\nabla_\rho g_{\mu\nu} = 0$.) Lengths and angles are preserved.

---

## 23.1.5 Geodesics as Autoparallel Curves

A **geodesic** is a curve that parallel transports its own tangent vector:

$$\frac{D}{d\lambda}\frac{dx^\mu}{d\lambda} = \frac{d^2x^\mu}{d\lambda^2} + \Gamma^\mu_{\nu\rho}\frac{dx^\nu}{d\lambda}\frac{dx^\rho}{d\lambda} = 0$$

This is the **geodesic equation** — the GR equation of motion for a freely-falling particle.

**Physical interpretation**: A geodesic is the straightest possible path on a curved manifold — the path along which the velocity vector doesn't change direction (as defined by parallel transport). In flat spacetime, geodesics are straight lines (uniform motion). In curved spacetime, they are the worldlines of freely-falling particles.

**Variational derivation**: Geodesics also extremize the arc length $\int ds = \int\sqrt{-g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu}\,d\lambda$ (for timelike curves) — they are the "longest proper time" paths between events.

**Null geodesics**: Setting $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = 0$ gives null geodesics — the paths of light rays. In the Schwarzschild metric, null geodesics exhibit light bending (one of the classical tests of GR), the Shapiro time delay, and (for $r < 3M$) capture into circular photon orbits.

**Normal coordinates**: At any event $p$, one can choose coordinates in which $g_{\mu\nu}(p) = \eta_{\mu\nu}$ and $\Gamma^\rho_{\mu\nu}(p) = 0$ — the **locally inertial frame** or **Riemann normal coordinates**. In such coordinates, the geodesic equation at $p$ reduces to $d^2x^\mu/d\lambda^2 = 0$ (no acceleration) — this is the mathematical expression of the equivalence principle.

---

## References

- Gauss, C.F. (1827). *Disquisitiones Generales Circa Superficies Curvas.* [Intrinsic geometry of surfaces; the Gaussian curvature; the Gauss-Bonnet theorem. The foundation of differential geometry.]
- Riemann, B. (1854). "Über die Hypothesen, welche der Geometrie zu Grunde liegen." *Abhandlungen der Königlichen Gesellschaft der Wissenschaften zu Göttingen*, 13 (1868). [Riemann's inaugural lecture: the concept of an $n$-dimensional manifold with a metric; the Riemann curvature tensor; the first general framework for differential geometry in any dimension.]
- Christoffel, E.B. (1869). "Über die Transformation der homogenen Differentialausdrücke zweiten Grades." *Journal für die reine und angewandte Mathematik*, 70, 46–70. [Introduces the symbols $\Gamma^\rho_{\mu\nu}$ (now called Christoffel symbols) and the concept of covariant differentiation.]
- Levi-Civita, T. (1917). "Nozione di parallelismo in una varietà qualunque." *Rendiconti del Circolo Matematico di Palermo*, 42, 173–205. [Introduces parallel transport and the Levi-Civita connection. The geometric interpretation of Christoffel's algebraic construction.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [Chapter 13 on covariant derivatives; Chapter 10 on geodesics and the variational principle. The GR physics of parallel transport and geodesics.]
