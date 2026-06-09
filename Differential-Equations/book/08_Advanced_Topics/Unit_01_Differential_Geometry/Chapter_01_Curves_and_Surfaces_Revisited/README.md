# Chapter 1: Curves and Surfaces Revisited

The geometry of curves and surfaces in Euclidean space is the historical origin of differential geometry and remains its most computationally explicit domain. While later chapters develop abstract and intrinsic methods, the concrete setting of $\mathbb{R}^3$ provides essential intuition and the first appearances of fundamental ideas: curvature, the fundamental theorem, and the relationship between local differential data and global geometric shape.

## Regular Curves

A **parameterized curve** is a smooth map $\gamma: I \to \mathbb{R}^3$ on an interval $I \subset \mathbb{R}$. It is called **regular** if $\gamma'(t) \neq 0$ for all $t \in I$. The regularity condition ensures that the curve has a well-defined tangent direction everywhere. The **arc length** from $a \in I$ is $s(t) = \int_a^t |\gamma'(u)| \, du$, and parameterizing by arc length gives $|\gamma'(s)| = 1$.

The **curvature** of a unit-speed curve $\gamma$ is $\kappa(s) = |\gamma''(s)|$. It measures how fast the curve bends. A straight line has $\kappa = 0$; a circle of radius $r$ has constant curvature $\kappa = 1/r$.

## The Frenet-Serret Frame

For a unit-speed curve with $\kappa > 0$ everywhere, one defines:

- **Unit tangent:** $T(s) = \gamma'(s)$
- **Principal normal:** $N(s) = T'(s)/|T'(s)| = T'(s)/\kappa(s)$
- **Binormal:** $B(s) = T(s) \times N(s)$

These three orthonormal vectors form the **Frenet frame** at each point. Differentiating and using $|T| = |N| = |B| = 1$ and pairwise orthogonality, one obtains the **Frenet-Serret equations**:

$$T' = \kappa N, \quad N' = -\kappa T + \tau B, \quad B' = -\tau N,$$

where $\tau = -B' \cdot N$ is the **torsion**. The curvature $\kappa \geq 0$ measures bending; the torsion $\tau$ (which may be of either sign) measures twisting out of the osculating plane.

**Fundamental Theorem of Curves.** Given smooth functions $\kappa: I \to (0, \infty)$ and $\tau: I \to \mathbb{R}$, there exists a regular curve $\gamma: I \to \mathbb{R}^3$, unique up to rigid motion of $\mathbb{R}^3$, with curvature $\kappa$ and torsion $\tau$.

The proof is an existence and uniqueness theorem for the Frenet ODE system above, initialized by choosing $T(0), N(0), B(0)$ as any positively oriented orthonormal frame.

## Regular Surfaces

A **regular surface** is a set $S \subset \mathbb{R}^3$ such that every point $p \in S$ has a neighborhood $V$ in $S$ that is the image of a smooth injective map $\mathbf{r}: U \to \mathbb{R}^3$ from an open set $U \subset \mathbb{R}^2$, with $d\mathbf{r}$ injective at every point. The map $\mathbf{r}$ is called a **local parametrization** or coordinate chart.

At each point $p = \mathbf{r}(u_0, v_0)$, the **tangent plane** $T_pS$ is spanned by $\mathbf{r}_u$ and $\mathbf{r}_v$ (the partial derivatives of $\mathbf{r}$). The **unit normal** is $N = (\mathbf{r}_u \times \mathbf{r}_v)/|\mathbf{r}_u \times \mathbf{r}_v|$.

## The First Fundamental Form

The **first fundamental form** at $p$ is the restriction of the Euclidean inner product to $T_pS$:

$$I(u, v) = u \cdot v, \quad u, v \in T_pS.$$

In coordinates, if $\mathbf{r}(u, v)$ is a local parametrization, then for a tangent vector $d\mathbf{r} = \mathbf{r}_u \, du + \mathbf{r}_v \, dv$:

$$I = ds^2 = E \, du^2 + 2F \, du \, dv + G \, dv^2,$$

where $E = \mathbf{r}_u \cdot \mathbf{r}_u$, $F = \mathbf{r}_u \cdot \mathbf{r}_v$, $G = \mathbf{r}_v \cdot \mathbf{r}_v$ are the coefficients of the first fundamental form.

The first fundamental form determines:
- **Lengths** of curves on $S$: $L(\gamma) = \int_a^b \sqrt{E\dot{u}^2 + 2F\dot{u}\dot{v} + G\dot{v}^2} \, dt$.
- **Angles** between intersecting curves.
- **Areas**: $A(R) = \iint_U \sqrt{EG - F^2} \, du \, dv$.

All of these are intrinsic: they depend only on $I$, not on how $S$ is embedded in $\mathbb{R}^3$.

## Key Results and the Second Fundamental Form

The second fundamental form $II = L \, du^2 + 2M \, du \, dv + N \, dv^2$ (with $L = \mathbf{r}_{uu} \cdot \hat{N}$, $M = \mathbf{r}_{uv} \cdot \hat{N}$, $N = \mathbf{r}_{vv} \cdot \hat{N}$) measures how $S$ bends within $\mathbb{R}^3$. While the first fundamental form is intrinsic, the second fundamental form is extrinsic—it depends on the embedding. The relationship between the two is the subject of Chapter 2.

This chapter concludes by establishing the fundamental theorem of surfaces (the analogue of the Frenet theorem for curves): a surface is determined up to rigid motion by its first and second fundamental forms, subject to the Gauss and Codazzi-Mainardi integrability conditions.
