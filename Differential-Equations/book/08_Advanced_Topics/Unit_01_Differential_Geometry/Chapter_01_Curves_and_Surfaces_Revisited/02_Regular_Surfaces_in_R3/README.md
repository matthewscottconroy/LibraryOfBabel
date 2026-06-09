# Regular Surfaces in $\mathbb{R}^3$

A surface in $\mathbb{R}^3$ is a two-dimensional geometric object—a sphere, a torus, a saddle—embedded in three-dimensional space. Making this notion precise requires care: the surface must be smooth (no corners or self-intersections), and the notion of "two-dimensional" must be given a rigorous meaning. The definition of a regular surface provides this precision, and the first fundamental form captures the intrinsic geometry that remains after forgetting how the surface sits in ambient space.

## Definition of Regular Surface

**Definition.** A subset $S \subset \mathbb{R}^3$ is a **regular surface** if, for each $p \in S$, there exists an open set $V \subset \mathbb{R}^3$ with $p \in V$ and a smooth map $\mathbf{r}: U \to V \cap S$ from an open set $U \subset \mathbb{R}^2$ such that:

1. $\mathbf{r}$ is a homeomorphism from $U$ onto $V \cap S$.
2. The differential $d\mathbf{r}(q): \mathbb{R}^2 \to \mathbb{R}^3$ is injective for every $q \in U$.

The map $\mathbf{r}$ is called a **local parametrization** (or coordinate chart) of $S$. Condition 2 (regularity) means the Jacobian matrix $\begin{pmatrix} \mathbf{r}_u & \mathbf{r}_v \end{pmatrix}$ has rank 2 everywhere, i.e., $\mathbf{r}_u \times \mathbf{r}_v \neq 0$.

Equivalently, $S$ is a regular surface if it is a smooth 2-dimensional submanifold of $\mathbb{R}^3$.

## Examples

**Sphere $S^2_R$.** The sphere of radius $R$ can be covered by two sterographic projection charts, or more practically by the parametrization $\mathbf{r}(\theta, \varphi) = (R\sin\theta\cos\varphi, R\sin\theta\sin\varphi, R\cos\theta)$ for $\theta \in (0,\pi)$, $\varphi \in (0, 2\pi)$ (missing two meridians and the poles).

**Torus.** With major radius $R$ and minor radius $r < R$: $\mathbf{r}(\theta, \varphi) = ((R + r\cos\theta)\cos\varphi, (R + r\cos\theta)\sin\varphi, r\sin\theta)$.

**Graph surface.** If $f: U \to \mathbb{R}$ is smooth, the graph $S = \{(x,y,f(x,y)) : (x,y) \in U\}$ is a regular surface with parametrization $\mathbf{r}(x,y) = (x, y, f(x,y))$.

**Level surface.** If $F: \mathbb{R}^3 \to \mathbb{R}$ is smooth and $\nabla F(p) \neq 0$ for all $p \in F^{-1}(c)$, then $S = F^{-1}(c)$ is a regular surface (by the implicit function theorem).

## The Tangent Plane

At a point $p = \mathbf{r}(u_0, v_0) \in S$, the **tangent plane** $T_pS$ is the two-dimensional subspace of $\mathbb{R}^3$ spanned by $\mathbf{r}_u(u_0, v_0)$ and $\mathbf{r}_v(u_0, v_0)$. The tangent plane is well-defined (independent of the choice of parametrization), since any other parametrization $\tilde{\mathbf{r}}$ is related to $\mathbf{r}$ by a diffeomorphism, and the chain rule shows the tangent planes agree.

The **unit normal** at $p$ is

$$\hat{N}(p) = \frac{\mathbf{r}_u \times \mathbf{r}_v}{|\mathbf{r}_u \times \mathbf{r}_v|} \bigg|_{(u_0,v_0)}.$$

A surface is **orientable** if a globally consistent choice of unit normal $\hat{N}: S \to S^2$ exists. The sphere, torus, and graph surfaces are orientable; the Möbius strip is not.

## The First Fundamental Form

The **first fundamental form** on $S$ is the inner product on each tangent space $T_pS$ inherited from $\mathbb{R}^3$: for tangent vectors $\mathbf{u}, \mathbf{v} \in T_pS$,

$$I(\mathbf{u}, \mathbf{v}) = \mathbf{u} \cdot \mathbf{v}.$$

In the coordinate basis $\{\mathbf{r}_u, \mathbf{r}_v\}$, writing a tangent vector as $\mathbf{w} = a\mathbf{r}_u + b\mathbf{r}_v$, the first fundamental form is the quadratic form

$$I = E \, du^2 + 2F \, du \, dv + G \, dv^2,$$

with coefficients:

$$E = \mathbf{r}_u \cdot \mathbf{r}_u, \quad F = \mathbf{r}_u \cdot \mathbf{r}_v, \quad G = \mathbf{r}_v \cdot \mathbf{r}_v.$$

The matrix $\begin{pmatrix} E & F \\ F & G \end{pmatrix}$ is positive definite (since $EG - F^2 = |\mathbf{r}_u \times \mathbf{r}_v|^2 > 0$ by the regularity condition).

## Lengths, Angles, and Areas from the First Fundamental Form

The first fundamental form encodes all intrinsic metric information:

**Length of a curve.** For a curve $\alpha(t) = \mathbf{r}(u(t), v(t))$ on $S$:

$$L(\alpha) = \int_a^b |\alpha'(t)| \, dt = \int_a^b \sqrt{E\dot{u}^2 + 2F\dot{u}\dot{v} + G\dot{v}^2} \, dt.$$

**Angle between curves.** The angle $\theta$ between two curves meeting at $p$ is determined by $\cos\theta = I(\mathbf{u}, \mathbf{v})/(|\mathbf{u}||\mathbf{v}|)$ where $\mathbf{u}, \mathbf{v}$ are their tangent vectors at $p$.

**Area.** For a region $\mathcal{R} = \mathbf{r}(U)$:

$$A(\mathcal{R}) = \iint_U |\mathbf{r}_u \times \mathbf{r}_v| \, du \, dv = \iint_U \sqrt{EG - F^2} \, du \, dv.$$

## Examples: First Fundamental Forms

**Sphere of radius $R$** (using $\mathbf{r}(\theta, \varphi) = R(\sin\theta\cos\varphi, \sin\theta\sin\varphi, \cos\theta)$):

$$E = R^2, \quad F = 0, \quad G = R^2\sin^2\theta.$$

$$ds^2 = R^2 \, d\theta^2 + R^2\sin^2\theta \, d\varphi^2.$$

The area element is $\sqrt{EG - F^2} \, d\theta \, d\varphi = R^2 \sin\theta \, d\theta \, d\varphi$, giving $A(S^2_R) = 4\pi R^2$.

**Torus** (with radii $R, r$):

$$E = r^2, \quad F = 0, \quad G = (R + r\cos\theta)^2.$$

$$ds^2 = r^2 \, d\theta^2 + (R + r\cos\theta)^2 \, d\varphi^2.$$

## Isometries and Conformal Maps

A **local isometry** between surfaces $S_1$ and $S_2$ is a diffeomorphism $\phi: S_1 \to S_2$ that preserves the first fundamental form: $I_{S_1}(\mathbf{u}, \mathbf{v}) = I_{S_2}(d\phi(\mathbf{u}), d\phi(\mathbf{v}))$. Isometries preserve lengths and areas. A **conformal map** preserves angles (but not necessarily lengths): $I_{S_2}(d\phi(\mathbf{u}), d\phi(\mathbf{v})) = \lambda^2 I_{S_1}(\mathbf{u}, \mathbf{v})$ for some scalar function $\lambda > 0$.

A cylinder can be unrolled flat (it is locally isometric to a plane), since $ds^2 = d\theta^2 + dz^2 = dx^2 + dy^2$ in the appropriate coordinates. A sphere cannot be mapped isometrically to the plane—this is the mathematical reason no flat map of the earth can be both length-preserving and angle-preserving, which Gauss's Theorema Egregium will make rigorous in Chapter 2.

## Change of Coordinates

If $\tilde{\mathbf{r}}(\tilde{u}, \tilde{v}) = \mathbf{r}(u(\tilde{u}, \tilde{v}), v(\tilde{u}, \tilde{v}))$ is another parametrization of the same patch, the coefficients of the first fundamental form transform as:

$$\tilde{E} = E u_{\tilde{u}}^2 + 2F u_{\tilde{u}} v_{\tilde{u}} + G v_{\tilde{u}}^2, \quad \text{etc.}$$

In matrix form, if $J = \begin{pmatrix} u_{\tilde{u}} & u_{\tilde{v}} \\ v_{\tilde{u}} & v_{\tilde{v}} \end{pmatrix}$ is the Jacobian of the coordinate change, then $\tilde{g} = J^T g J$ where $g = \begin{pmatrix} E & F \\ F & G \end{pmatrix}$. This transformation law identifies $(E,F,G)$ as the components of a **Riemannian metric tensor**—the beginning of the abstract theory developed in Chapter 3.
