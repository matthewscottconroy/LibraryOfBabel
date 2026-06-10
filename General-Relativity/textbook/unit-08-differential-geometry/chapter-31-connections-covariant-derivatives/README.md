# Chapter 31: Connections and Covariant Derivatives

---

## Chapter Introduction

On a manifold without additional structure, there is no natural way to compare vectors at different points. If I have a vector at point $p$ and another at point $q$, how do I add them? How do I differentiate a vector field? In Euclidean space, the answer is obvious — just use Cartesian coordinates. But on a curved manifold, the answer is not obvious: vector components in different coordinate patches transform nontrivially.

A **connection** is extra structure on a manifold that specifies how to compare (or "connect") tangent spaces at different points — how to "parallel transport" a vector from $p$ to $q$ along a path. Once we have a connection, we can define the covariant derivative of a tensor field — a generalization of partial differentiation that is tensorial (coordinate-independent).

In GR, the unique torsion-free connection compatible with the metric is the **Levi-Civita connection**, whose Christoffel symbols $\Gamma^\rho_{\mu\nu}$ determine how vectors are parallel-transported along geodesics. The covariant derivative $\nabla_\mu T^{\rho\sigma\cdots}$ replaces ordinary partial derivatives in tensor equations, ensuring the equations are coordinate-independent.

This chapter constructs the Levi-Civita connection from scratch, derives the formula for Christoffel symbols, and develops the covariant derivative's algebraic properties.

---

## Affine Connections

An **affine connection** (or linear connection) $\nabla$ on a manifold $M$ assigns to each pair of smooth vector fields $X, Y$ a new vector field $\nabla_X Y$ (the covariant derivative of $Y$ in the direction $X$), satisfying:

1. **$\mathbb{R}$-linearity in $X$**: $\nabla_{fX+gY}Z = f\nabla_X Z + g\nabla_Y Z$ for $f, g\in C^\infty(M)$
2. **Leibniz rule in $Y$**: $\nabla_X(fY) = (Xf)Y + f\nabla_X Y$
3. **$\mathbb{R}$-linearity in $Y$**: $\nabla_X(Y+Z) = \nabla_X Y + \nabla_X Z$

In local coordinates $\{x^\mu\}$, the connection is determined by its **Christoffel symbols** $\Gamma^\rho_{\mu\nu}$:
$$\nabla_{\partial_\mu}\partial_\nu = \Gamma^\rho_{\mu\nu}\partial_\rho$$

The covariant derivative of a vector field $V = V^\nu\partial_\nu$ in direction $\partial_\mu$:
$$\nabla_\mu V^\rho = \partial_\mu V^\rho + \Gamma^\rho_{\mu\nu}V^\nu$$

---

## Torsion and Metric Compatibility

Two natural conditions restrict the connection:

**Torsion-free**: The torsion tensor $T^\rho_{\ \mu\nu} = \Gamma^\rho_{\mu\nu} - \Gamma^\rho_{\nu\mu}$ vanishes. Equivalently: $\nabla_X Y - \nabla_Y X = [X,Y]$ (covariant derivative commutes with Lie bracket).

**Metric compatibility**: $\nabla_\rho g_{\mu\nu} = 0$. The metric is "constant" under parallel transport.

**Fundamental theorem of Riemannian geometry** (Levi-Civita): For any pseudo-Riemannian metric $g$, there exists a unique torsion-free, metric-compatible connection — the **Levi-Civita connection**.

---

## Christoffel Symbols

The Christoffel symbols of the Levi-Civita connection are determined by $g$:
$$\Gamma^\rho_{\mu\nu} = \frac{1}{2}g^{\rho\sigma}(\partial_\mu g_{\nu\sigma} + \partial_\nu g_{\mu\sigma} - \partial_\sigma g_{\mu\nu})$$

**Derivation**: From metric compatibility $\nabla_\rho g_{\mu\nu} = 0$:
$$\partial_\rho g_{\mu\nu} - \Gamma^\sigma_{\rho\mu}g_{\sigma\nu} - \Gamma^\sigma_{\rho\nu}g_{\mu\sigma} = 0$$

Writing this for three permutations of $(\mu,\nu,\rho)$ and combining (using torsion-free $\Gamma^\rho_{\mu\nu} = \Gamma^\rho_{\nu\mu}$):
$$2\Gamma^\sigma_{\mu\nu}g_{\sigma\rho} = \partial_\mu g_{\nu\rho} + \partial_\nu g_{\mu\rho} - \partial_\rho g_{\mu\nu}$$

Contracting with $g^{\rho\lambda}$ gives the formula above.

**Properties**:
- $\Gamma^\rho_{\mu\nu} = \Gamma^\rho_{\nu\mu}$ (symmetric in lower indices — torsion-free)
- Not a tensor: under $x^\mu\to\tilde{x}^\mu$: $\tilde{\Gamma}^\rho_{\mu\nu} = \frac{\partial\tilde{x}^\rho}{\partial x^\sigma}\frac{\partial x^\alpha}{\partial\tilde{x}^\mu}\frac{\partial x^\beta}{\partial\tilde{x}^\nu}\Gamma^\sigma_{\alpha\beta} + \frac{\partial\tilde{x}^\rho}{\partial x^\sigma}\frac{\partial^2 x^\sigma}{\partial\tilde{x}^\mu\partial\tilde{x}^\nu}$ (inhomogeneous!)
- Can be set to zero at any single point in normal coordinates

---

## Covariant Derivative of General Tensors

The covariant derivative extends to $(r,s)$-tensors by the Leibniz rule:

For a $(1,1)$-tensor $T^\mu_{\ \nu}$:
$$\nabla_\rho T^\mu_{\ \nu} = \partial_\rho T^\mu_{\ \nu} + \Gamma^\mu_{\rho\sigma}T^\sigma_{\ \nu} - \Gamma^\sigma_{\rho\nu}T^\mu_{\ \sigma}$$

(A $+\Gamma$ for each upper index, a $-\Gamma$ for each lower index.)

For a general $(r,s)$-tensor:
$$\nabla_\rho T^{\mu_1\cdots\mu_r}_{\ \ \ \nu_1\cdots\nu_s} = \partial_\rho T^{\mu_1\cdots\mu_r}_{\ \ \ \nu_1\cdots\nu_s} + \sum_{i=1}^r\Gamma^{\mu_i}_{\rho\sigma}T^{\mu_1\cdots\sigma\cdots\mu_r}_{\ \ \ \nu_1\cdots\nu_s} - \sum_{j=1}^s\Gamma^\sigma_{\rho\nu_j}T^{\mu_1\cdots\mu_r}_{\ \ \ \nu_1\cdots\sigma\cdots\nu_s}$$

**Key examples**:
- Scalar: $\nabla_\mu f = \partial_\mu f$ (no connection terms)
- Covector: $\nabla_\mu\alpha_\nu = \partial_\mu\alpha_\nu - \Gamma^\rho_{\mu\nu}\alpha_\rho$
- Metric: $\nabla_\rho g_{\mu\nu} = 0$ (by metric compatibility — all $\Gamma$'s cancel)

---

## Parallel Transport

Given a curve $\gamma: [a,b]\to M$ with tangent $\dot{\gamma}$, a vector field $V$ is **parallel transported** along $\gamma$ if:
$$\frac{DV^\mu}{d\lambda} \equiv \dot{\gamma}^\nu\nabla_\nu V^\mu = 0 \quad\Leftrightarrow\quad \frac{dV^\mu}{d\lambda} + \Gamma^\mu_{\nu\rho}\dot{\gamma}^\nu V^\rho = 0$$

This is a system of linear ODEs — given $V(a)$, it determines $V(b)$ uniquely. The map $P_\gamma: T_{\gamma(a)}M\to T_{\gamma(b)}M$ is the **parallel transport map** — a linear isometry if the connection is metric-compatible.

**Holonomy**: If $\gamma$ is a closed loop, $P_\gamma$ is a linear map from $T_p M$ to itself — an element of $GL(n)$ (or $O(n)$ for a Riemannian connection). The group of all such parallel transports is the **holonomy group**. On a curved manifold, the holonomy of a closed loop measures the curvature enclosed.

**Physical example**: The Foucault pendulum swings in a plane that rotates as Earth rotates beneath it. The pendulum's swing direction is parallel-transported on the sphere (Earth's surface). After one day, the pendulum plane has rotated by an angle equal to the solid angle subtended by the pole — the holonomy of the daily path.

---

## The Second Covariant Derivative and Curvature

The key difference between ordinary partial derivatives and covariant derivatives: they don't commute. The commutator is:
$$[\nabla_\mu, \nabla_\nu]V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma$$

where $R^\rho_{\ \sigma\mu\nu}$ is the **Riemann curvature tensor**:
$$R^\rho_{\ \sigma\mu\nu} = \partial_\mu\Gamma^\rho_{\nu\sigma} - \partial_\nu\Gamma^\rho_{\mu\sigma} + \Gamma^\rho_{\mu\lambda}\Gamma^\lambda_{\nu\sigma} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\mu\sigma}$$

This is the fundamental formula: if you parallel-transport a vector around an infinitesimal loop in the $\mu\nu$-plane, the vector rotates by $R^\rho_{\ \sigma\mu\nu}\delta x^\mu\delta x^\nu$. The Riemann tensor measures the failure of commutativity of covariant derivatives — i.e., it measures curvature.

For covectors: $[\nabla_\mu, \nabla_\nu]\alpha_\rho = -R^\sigma_{\ \rho\mu\nu}\alpha_\sigma$.

For tensors: $[\nabla_\mu, \nabla_\nu]T^\rho_{\ \sigma} = R^\rho_{\ \lambda\mu\nu}T^\lambda_{\ \sigma} - R^\lambda_{\ \sigma\mu\nu}T^\rho_{\ \lambda}$.

---

## Normal Coordinates and the Local Flatness Theorem

A key property of the Levi-Civita connection: at any point $p$, there exist **normal coordinates** centered at $p$ such that $g_{\mu\nu}(p) = \eta_{\mu\nu}$ and $\Gamma^\rho_{\mu\nu}(p) = 0$. In other words, in normal coordinates, the metric is flat to first order at $p$, and covariant derivatives equal partial derivatives at $p$.

**Local flatness theorem**: In normal coordinates, $g_{\mu\nu}(x) = \eta_{\mu\nu} - \frac{1}{3}R_{\mu\alpha\nu\beta}(p)x^\alpha x^\beta + O(x^3)$.

The leading correction to flatness is the Riemann tensor — curvature shows up at second order in the displacement.

**Physical significance**: The equivalence principle in GR says that in a freely falling frame (normal coordinates), the laws of physics look locally like special relativity. Normal coordinates make this precise: $g_{\mu\nu}|_p = \eta_{\mu\nu}$, $\Gamma^\rho_{\mu\nu}|_p = 0$. Curvature — tidal forces — appear at second order $O(x^2)$.

---

## Exercises

**31.1.** *Christoffel symbols for $S^2$.*

The metric on $S^2$ (unit sphere) is $g = d\theta^2 + \sin^2\theta\,d\phi^2$.

(a) Compute all nonzero Christoffel symbols $\Gamma^\rho_{\mu\nu}$ using the formula with metric derivatives.

(b) Write the geodesic equations $\ddot{\gamma}^\rho + \Gamma^\rho_{\mu\nu}\dot{\gamma}^\mu\dot{\gamma}^\nu = 0$. Verify that $\theta = \pi/2$ (equator), $\dot{\phi} = \text{const}$ is a solution (a great circle).

(c) Parallel-transport the vector $V = \partial_\theta$ (pointing south) around the equator $\theta = \pi/2$. Solve the parallel transport ODE $dV^\rho/d\phi + \Gamma^\rho_{\mu\phi}V^\mu = 0$. What is the angle of rotation after one complete circuit?

---

**31.2.** *Covariant derivative vs. partial derivative.*

(a) Show that $\partial_\mu T^\rho_{\ \nu}$ is not a tensor (compute its transformation under $x^\mu\to\tilde{x}^\mu$).

(b) Show that $\nabla_\mu T^\rho_{\ \nu} = \partial_\mu T^\rho_{\ \nu} + \Gamma^\rho_{\mu\sigma}T^\sigma_{\ \nu} - \Gamma^\sigma_{\mu\nu}T^\rho_{\ \sigma}$ is a tensor.

(c) For a scalar $f$: $\nabla_\mu f = \partial_\mu f$. For a 1-form $\alpha_\nu$: verify $\nabla_\mu(\alpha_\nu V^\nu) = (\nabla_\mu\alpha_\nu)V^\nu + \alpha_\nu(\nabla_\mu V^\nu)$ using the Leibniz rule.

---

**31.3.** *The Bianchi identity from parallel transport.*

(a) Show that for any vector $V$: $[\nabla_\mu, \nabla_\nu]V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma$ by direct calculation from the formula for $\nabla_\mu(\nabla_\nu V^\rho)$.

(b) From this, derive the **second Bianchi identity** $\nabla_{[\lambda}R_{\mu\nu]\rho\sigma} = 0$ by applying $[\nabla_{[\lambda}, [\nabla_\mu, \nabla_{\nu]}]]\alpha_\rho = 0$ (Jacobi identity for covariant derivatives) and using the Riemann tensor.

(c) Show that the contracted Bianchi identity $\nabla^\mu R_{\mu\nu} = \frac{1}{2}\nabla_\nu R$ follows, and hence $\nabla^\mu G_{\mu\nu} = 0$.

---

**Thought Experiment T31.1.** *Gauge fields as connections.*

In Yang-Mills gauge theory, the gauge field $A_\mu^a$ (where $a$ labels Lie algebra indices) is a connection on a principal fiber bundle. The covariant derivative acting on matter fields is $D_\mu = \partial_\mu + A_\mu^a T_a$ where $T_a$ are Lie algebra generators. The field strength $F_{\mu\nu}^a = \partial_\mu A_\nu^a - \partial_\nu A_\mu^a + f^{abc}A_\mu^b A_\nu^c$ is the curvature of this connection — exactly analogous to the Riemann tensor.

Write a table comparing:
- GR: Christoffel symbols $\Gamma^\rho_{\mu\nu}$, Riemann tensor $R^\rho_{\ \sigma\mu\nu}$, diffeomorphisms
- EM: $A_\mu$, $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$, gauge transformations $A\to A + d\lambda$
- Yang-Mills: $A_\mu^a$, $F_{\mu\nu}^a$, non-Abelian gauge transformations

What is the mathematical structure common to all three? What is different? Can GR be formulated as a gauge theory?
