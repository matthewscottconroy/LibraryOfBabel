# The First Fundamental Form

The first fundamental form is the inner product on tangent spaces of a surface, inherited from the ambient Euclidean space. It is the foundational object of intrinsic surface geometry: all lengths, angles, and areas are computed from it, and two surfaces are locally isometric if and only if they have the same first fundamental form (in appropriate coordinates). This section treats the first fundamental form as a Riemannian metric in coordinates, develops the calculus of lengths and areas, and introduces geodesics as the intrinsic analogue of straight lines.

## The Metric Tensor in Coordinates

Let $S$ be a regular surface with local parametrization $\mathbf{r}: U \to S$, $U \subset \mathbb{R}^2$. At each point $\mathbf{r}(u,v)$, the tangent plane is spanned by $\mathbf{r}_u$ and $\mathbf{r}_v$. The first fundamental form is the positive definite quadratic form on the tangent plane given by the dot product:

$$I(a\mathbf{r}_u + b\mathbf{r}_v, c\mathbf{r}_u + d\mathbf{r}_v) = (a\mathbf{r}_u + b\mathbf{r}_v) \cdot (c\mathbf{r}_u + d\mathbf{r}_v) = Eac + F(ad+bc) + Gbd,$$

where $E = \mathbf{r}_u \cdot \mathbf{r}_u$, $F = \mathbf{r}_u \cdot \mathbf{r}_v$, $G = \mathbf{r}_v \cdot \mathbf{r}_v$. These are smooth functions on $U$, and the matrix $g = \begin{pmatrix} E & F \\ F & G \end{pmatrix}$ is the **metric tensor** (or **Gram matrix**) of the parametrization. Positive definiteness: $EG - F^2 = |\mathbf{r}_u \times \mathbf{r}_v|^2 > 0$.

The differential element of arc length is:

$$ds^2 = E \, du^2 + 2F \, du \, dv + G \, dv^2 = g_{ij} \, dx^i \, dx^j$$

(using Einstein summation with $x^1 = u$, $x^2 = v$, $g_{11} = E$, $g_{12} = g_{21} = F$, $g_{22} = G$).

## Length of Curves

For a curve $\alpha: [a,b] \to S$, $\alpha(t) = \mathbf{r}(u(t), v(t))$, the arc length is:

$$L(\alpha) = \int_a^b |\alpha'(t)| \, dt = \int_a^b \sqrt{g_{ij} \dot{x}^i \dot{x}^j} \, dt = \int_a^b \sqrt{E\dot{u}^2 + 2F\dot{u}\dot{v} + G\dot{v}^2} \, dt.$$

**Example on the sphere.** Take the sphere $S^2_R$ with $E = R^2$, $F = 0$, $G = R^2\sin^2\theta$. The equator $\theta = \pi/2$, $\varphi \in [0, 2\pi]$ has arc length $\int_0^{2\pi} \sqrt{0 + 0 + R^2 \cdot 1} \, d\varphi = 2\pi R$. A meridian $\varphi = \text{const}$, $\theta \in [0, \pi]$ has length $\int_0^\pi R \, d\theta = \pi R$.

## Geodesics

A **geodesic** on a surface $S$ is a curve $\gamma(s)$ (parameterized by arc length) that locally minimizes length. Geodesics are the intrinsic analogue of straight lines: on the sphere, they are great circles; on a cylinder, they are helices; on the plane, they are straight lines.

The **geodesic equations** are the Euler-Lagrange equations for the length functional $L(\alpha) = \int \sqrt{g_{ij}\dot{x}^i\dot{x}^j} \, dt$. In terms of arc length parameterization, they are:

$$\ddot{x}^k + \Gamma^k_{ij} \dot{x}^i \dot{x}^j = 0, \quad k = 1, 2,$$

where $\Gamma^k_{ij}$ are the **Christoffel symbols**:

$$\Gamma^k_{ij} = \frac{1}{2} g^{kl} \left(\frac{\partial g_{il}}{\partial x^j} + \frac{\partial g_{jl}}{\partial x^i} - \frac{\partial g_{ij}}{\partial x^l}\right).$$

Here $g^{kl}$ is the inverse of the metric tensor $g_{ij}$.

**Derivation for a surface.** On a surface, $k, i, j \in \{1, 2\}$. Compute $\Gamma^k_{ij}$ using $g = \begin{pmatrix} E & F \\ F & G \end{pmatrix}$ and $g^{-1} = \frac{1}{EG-F^2}\begin{pmatrix} G & -F \\ -F & E \end{pmatrix}$.

For the sphere with $E = R^2$, $F = 0$, $G = R^2\sin^2\theta$:

$$\Gamma^1_{22} = -\sin\theta\cos\theta, \quad \Gamma^2_{12} = \Gamma^2_{21} = \cot\theta, \quad \text{all others zero}.$$

The geodesic equations become $\ddot{\theta} - \sin\theta\cos\theta \, \dot{\varphi}^2 = 0$ and $\ddot{\varphi} + 2\cot\theta \, \dot{\theta}\dot{\varphi} = 0$. One verifies that great circles (e.g., $\varphi = 0$, $\theta = s/R$) satisfy these equations.

## Area and the Area Element

The **area element** of $S$ in the parametrization $\mathbf{r}(u,v)$ is:

$$dA = |\mathbf{r}_u \times \mathbf{r}_v| \, du \, dv = \sqrt{EG - F^2} \, du \, dv = \sqrt{\det g} \, du \, dv.$$

For a region $\mathcal{R} = \mathbf{r}(D)$:

$$\text{Area}(\mathcal{R}) = \iint_D \sqrt{EG - F^2} \, du \, dv.$$

**Examples:**
- Sphere $S^2_R$: $dA = R^2\sin\theta \, d\theta \, d\varphi$, so $\text{Area} = R^2 \int_0^\pi\sin\theta \, d\theta \int_0^{2\pi} d\varphi = 4\pi R^2$.
- Torus: $dA = r(R + r\cos\theta) \, d\theta \, d\varphi$, so $\text{Area} = r \int_0^{2\pi}(R + r\cos\theta) \, d\theta \int_0^{2\pi} d\varphi = 4\pi^2 Rr$.

## Conformal Coordinates and Isothermal Parameters

A coordinate system is **conformal** (or **isothermal**) if $F = 0$ and $E = G = \lambda^2$ for some positive function $\lambda$. In conformal coordinates, angles are measured exactly as in the Euclidean plane, and the metric takes the form $ds^2 = \lambda^2(du^2 + dv^2)$.

**Theorem (Gauss).** Every regular surface admits conformal local coordinates.

This theorem, a consequence of the uniformization theorem for Riemann surfaces, is fundamental in complex analysis and minimal surface theory. Conformal coordinates simplify many computations: the Laplace-Beltrami operator becomes $\Delta_{LB} f = \frac{1}{\lambda^2}(\partial^2_u + \partial^2_v) f$.

## The Metric as Intrinsic Data

The first fundamental form is intrinsic: it can be observed and measured by a two-dimensional being living on the surface, without reference to the ambient $\mathbb{R}^3$. Two surfaces are **locally isometric** if there is a local diffeomorphism between them preserving the first fundamental form. Such surfaces are geometrically indistinguishable by measurements made within the surface.

The fundamental question of intrinsic geometry—pursued in Chapter 2—is: what properties of a surface can be computed from the first fundamental form alone? Gaussian curvature (as Gauss proved) is one such property, despite appearing at first to require the second fundamental form. This is the content of the Theorema Egregium.
