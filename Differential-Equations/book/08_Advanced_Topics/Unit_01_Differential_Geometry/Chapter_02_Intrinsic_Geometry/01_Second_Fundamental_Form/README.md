# The Second Fundamental Form

The second fundamental form encodes the extrinsic geometry of a surface—how the surface bends within the ambient space $\mathbb{R}^3$. While the first fundamental form is intrinsic (measurable from within the surface), the second fundamental form requires knowledge of the embedding. Together, the two fundamental forms determine the surface completely (up to rigid motions), subject to compatibility conditions.

## Normal Curvature

Consider a regular surface $S$ with unit normal field $\hat{N}$, and a point $p \in S$ with tangent vector $\mathbf{v} \in T_pS$ of unit length. Let $\gamma: (-\varepsilon, \varepsilon) \to S$ be any unit-speed curve with $\gamma(0) = p$ and $\gamma'(0) = \mathbf{v}$. The **normal curvature** in the direction $\mathbf{v}$ is:

$$\kappa_n(\mathbf{v}) = \gamma''(0) \cdot \hat{N}(p).$$

This measures the component of the acceleration $\gamma''$ in the normal direction—how much the curve bends toward or away from the surface's interior. It is independent of the choice of $\gamma$ (only depending on $\mathbf{v}$), since $\gamma'' = \kappa_g \mathbf{n} + \kappa_n \hat{N}$, where $\mathbf{n}$ is the principal normal of $\gamma$ and $\kappa_g$ is its geodesic curvature.

## Definition via the Shape Operator

The **shape operator** $\mathcal{W}: T_pS \to T_pS$ (also called the Weingarten map) is defined by:

$$\mathcal{W}(\mathbf{v}) = -D_{\mathbf{v}} \hat{N},$$

where $D_{\mathbf{v}} \hat{N}$ is the directional derivative of $\hat{N}$ in the direction $\mathbf{v}$ (as a function from $S$ to $S^2 \subset \mathbb{R}^3$). Since $\hat{N} \cdot \hat{N} = 1$, differentiating gives $D_{\mathbf{v}}\hat{N} \cdot \hat{N} = 0$, so $D_{\mathbf{v}}\hat{N} \in T_pS$.

The **second fundamental form** is:

$$II(\mathbf{u}, \mathbf{v}) = I(\mathcal{W}(\mathbf{u}), \mathbf{v}) = -D_{\mathbf{u}}\hat{N} \cdot \mathbf{v}.$$

One checks that $II(\mathbf{u}, \mathbf{v}) = \gamma''(0) \cdot \hat{N}$ when $\gamma'(0) = \mathbf{u}$, confirming the connection to normal curvature: $\kappa_n(\mathbf{v}) = II(\mathbf{v}, \mathbf{v})$ for unit $\mathbf{v}$.

**Self-adjointness.** The shape operator is self-adjoint: $II(\mathbf{u}, \mathbf{v}) = II(\mathbf{v}, \mathbf{u})$. Proof: $D_{\mathbf{u}}\hat{N} \cdot \mathbf{v} - D_{\mathbf{v}}\hat{N} \cdot \mathbf{u} = D_{\mathbf{u}}(\hat{N}\cdot\mathbf{v}) - \hat{N} \cdot D_{\mathbf{u}}\mathbf{v} - D_{\mathbf{v}}(\hat{N}\cdot\mathbf{u}) + \hat{N} \cdot D_{\mathbf{v}}\mathbf{u}$. Since $\hat{N} \perp T_pS$ and $D_\mathbf{u}\mathbf{v} - D_\mathbf{v}\mathbf{u} = [\mathbf{u}, \mathbf{v}]$ (Lie bracket), which for $\mathbf{u} = \mathbf{r}_u, \mathbf{v} = \mathbf{r}_v$ equals $\mathbf{r}_{uv} - \mathbf{r}_{vu} = 0$, the expression vanishes. $\square$

## Coordinates

In a local parametrization $\mathbf{r}(u,v)$, the second fundamental form has coefficients:

$$L = \mathbf{r}_{uu} \cdot \hat{N} = -\mathbf{r}_u \cdot \hat{N}_u, \quad M = \mathbf{r}_{uv} \cdot \hat{N} = -\mathbf{r}_u \cdot \hat{N}_v = -\mathbf{r}_v \cdot \hat{N}_u, \quad N = \mathbf{r}_{vv} \cdot \hat{N} = -\mathbf{r}_v \cdot \hat{N}_v.$$

(Here $\hat{N}_u = \partial\hat{N}/\partial u$, etc.) The second equality uses $\mathbf{r}_u \cdot \hat{N} = 0$ differentiated.

The second fundamental form is then:

$$II = L \, du^2 + 2M \, du \, dv + N \, dv^2.$$

Note the potential confusion in notation: $N$ (the second fundamental form coefficient) versus $\hat{N}$ (the unit normal). We use $\hat{N}$ for the normal vector throughout.

## The Weingarten Equations

The matrix of the shape operator $\mathcal{W}$ in the basis $\{\mathbf{r}_u, \mathbf{r}_v\}$ is $g^{-1}h$, where $g = \begin{pmatrix} E & F \\ F & G \end{pmatrix}$ and $h = \begin{pmatrix} L & M \\ M & N \end{pmatrix}$. Specifically:

$$\hat{N}_u = \frac{MF - LG}{EG - F^2} \mathbf{r}_u + \frac{LF - ME}{EG - F^2} \mathbf{r}_v, \quad \hat{N}_v = \frac{NF - MG}{EG - F^2} \mathbf{r}_u + \frac{MF - NE}{EG - F^2} \mathbf{r}_v.$$

These are the **Weingarten equations**, expressing the derivatives of the normal in terms of the tangent frame.

## Principal Curvatures and Directions

Since $\mathcal{W}$ is self-adjoint (symmetric with respect to $I$), the spectral theorem gives two real eigenvalues $\kappa_1 \geq \kappa_2$ (the **principal curvatures**) with $I$-orthogonal eigenvectors (the **principal directions**). The principal curvatures are the maximum and minimum normal curvatures:

$$\kappa_1 = \max_{|\mathbf{v}|=1} \kappa_n(\mathbf{v}), \quad \kappa_2 = \min_{|\mathbf{v}|=1} \kappa_n(\mathbf{v}).$$

The characteristic polynomial of $\mathcal{W}$ gives:

$$\kappa^2 - 2H\kappa + K = 0,$$

where $H = (\kappa_1 + \kappa_2)/2$ is the **mean curvature** and $K = \kappa_1\kappa_2$ is the **Gaussian curvature**:

$$K = \frac{LN - M^2}{EG - F^2}, \quad H = \frac{LG - 2MF + NE}{2(EG - F^2)}.$$

## Examples

**Sphere $S^2_R$.** Every unit tangent direction is a principal direction, and both principal curvatures equal $1/R$. So $K = 1/R^2$ and $H = 1/R$ (with outward normal convention; $H = -1/R$ with inward normal).

**Cylinder.** For the cylinder $\{(x,y,z) : x^2 + y^2 = R^2\}$ with outward normal, one principal curvature is $1/R$ (in the circumferential direction) and the other is $0$ (along the axis). Thus $K = 0$ and $H = 1/(2R)$. Gaussian curvature zero confirms the cylinder is locally isometric to the plane (it can be unrolled).

**Saddle surface $z = xy$.** At the origin, $L = N = 0$ and $M = 1$ (with $E = G = 1$, $F = 0$). So $K = (LN - M^2)/(EG - F^2) = -1 < 0$. The surface has negative Gaussian curvature at the origin, consistent with its saddle shape.

## Minimal Surfaces

A surface is **minimal** if $H = 0$ everywhere, i.e., $\kappa_1 + \kappa_2 = 0$. The name comes from the fact that (at least locally) minimal surfaces are critical points of the area functional: small deformations increase area. Minimal surfaces include the plane ($\kappa_1 = \kappa_2 = 0$), the catenoid $(\kappa_1 = -\kappa_2 \neq 0)$, the helicoid, and the Enneper surface. They arise in soap film problems: a soap film spanning a given boundary curve is a minimal surface (since surface tension minimizes area).

The minimal surface equation for a graph $z = f(x,y)$ is:

$$(1 + f_y^2) f_{xx} - 2f_x f_y f_{xy} + (1 + f_x^2) f_{yy} = 0.$$

This is an elliptic PDE, connecting differential geometry to the PDE theory developed in later units.
