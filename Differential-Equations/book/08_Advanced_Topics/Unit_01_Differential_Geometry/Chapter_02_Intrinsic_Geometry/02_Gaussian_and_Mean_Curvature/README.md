# Gaussian and Mean Curvature

The two principal curvatures $\kappa_1$ and $\kappa_2$ together encode all the curvature information of a surface at a point. Their symmetric combinations—the Gaussian curvature $K = \kappa_1\kappa_2$ and the mean curvature $H = (\kappa_1 + \kappa_2)/2$—are the two most important scalar invariants. Each has a different geometric character: Gaussian curvature is intrinsic (determined by the first fundamental form alone, as Gauss proved), while mean curvature is extrinsic (dependent on the embedding). Understanding both, and distinguishing their geometric roles, is the central task of this section.

## Geometric Interpretation of Gaussian Curvature

**Sign of $K$.** The Gaussian curvature $K = \kappa_1\kappa_2$ can be positive, negative, or zero:

- $K > 0$ (**elliptic point**): both principal curvatures have the same sign. The surface bends the same way in all directions. Near this point, the surface lies entirely on one side of its tangent plane. Example: a sphere, an ellipsoid, the top of a hill.

- $K < 0$ (**hyperbolic point**): principal curvatures have opposite signs. The surface curves up in one principal direction and down in the other, giving a saddle shape. Near this point, the surface passes through its tangent plane. Example: a saddle, a hyperbolic paraboloid $z = x^2 - y^2$, or any point on a pseudosphere.

- $K = 0$ (**parabolic point**): at least one principal curvature is zero. The surface is locally flat in one direction. Examples: points on a cylinder, cone, or any developable surface.

**The Gauss map.** Define the Gauss map $\nu: S \to S^2$ by $\nu(p) = \hat{N}(p)$ (the unit normal). The shape operator is (up to sign) the derivative of the Gauss map. The Gaussian curvature $K(p)$ equals the Jacobian determinant of $\nu$ at $p$ (with appropriate sign). Geometrically, $|K(p)|$ measures how much the unit normal rotates per unit area of surface: a highly curved surface maps a small patch to a large region on $S^2$.

**Total Gaussian curvature.** For a region $R \subset S$, $\iint_R K \, dA = \text{signed area of } \nu(R)$ on $S^2$. For the full sphere, $\iint K \, dA = 4\pi$. The Gauss-Bonnet theorem generalizes this: for any compact surface, the total curvature equals $2\pi\chi(S)$.

## Computation of Gaussian Curvature

In coordinates, the Gaussian curvature is:

$$K = \frac{LN - M^2}{EG - F^2}.$$

This formula appears to require the second fundamental form. But the Theorema Egregium (Section 3) gives $K$ in terms of $E, F, G$ and their derivatives—a highly nontrivial fact.

**Brioschi's formula** (1852) gives $K$ purely in terms of $E, F, G$:

$$K = \frac{\begin{vmatrix} -\frac{1}{2}E_{vv} + F_{uv} - \frac{1}{2}G_{uu} & \frac{1}{2}E_u & F_u - \frac{1}{2}E_v \\ F_v - \frac{1}{2}G_u & E & F \\ \frac{1}{2}G_v & F & G \end{vmatrix} - \begin{vmatrix} 0 & \frac{1}{2}E_v & \frac{1}{2}G_u \\ \frac{1}{2}E_v & E & F \\ \frac{1}{2}G_u & F & G \end{vmatrix}}{(EG - F^2)^2}.$$

While unwieldy in general, this formula simplifies considerably when $F = 0$ (orthogonal coordinates):

$$K = -\frac{1}{2\sqrt{EG}} \left[\frac{\partial}{\partial u}\left(\frac{G_u}{\sqrt{EG}}\right) + \frac{\partial}{\partial v}\left(\frac{E_v}{\sqrt{EG}}\right)\right].$$

## Examples

**Sphere of radius $R$:** $K = 1/R^2$ everywhere. Total curvature $= 4\pi R^2 \cdot 1/R^2 = 4\pi = 2\pi\chi(S^2) = 2\pi \cdot 2$. Consistent with Gauss-Bonnet.

**Torus:** At the outer equator ($\theta = 0$, $\kappa_1 = 1/r$, $\kappa_2 = \cos\theta/(R + r\cos\theta)|_{\theta=0} = 1/(R+r) > 0$), $K > 0$. At the inner equator ($\theta = \pi$, $\kappa_1 = 1/r$, $\kappa_2 = -1/(R-r) < 0$), $K < 0$. The total curvature integrates to $\iint K \, dA = 2\pi\chi(T^2) = 0$, since the positive and negative contributions cancel.

**Saddle surface $z = axy$:** At the origin with the usual orientation, $L = N = 0$, $M = a/\sqrt{1+(ax)^2+(ay)^2}|_0 = a$, $E = G = 1$, $F = 0$. So $K = (0 - a^2)/(1-0) = -a^2 < 0$.

## Mean Curvature

The **mean curvature** $H = (\kappa_1 + \kappa_2)/2$ has a direct variational meaning: $H(p) = 0$ if and only if the area is stationary under all normal perturbations of $S$ supported near $p$. This is why minimal surfaces ($H = 0$ everywhere) are area-critical.

The first variation formula for area is:

$$\frac{d}{dt}\bigg|_{t=0} A(S_t) = -2\int_S H \phi \, dA,$$

where $S_t$ is the family of surfaces obtained by moving $S$ by $t\phi \hat{N}$ (a normal perturbation with scalar function $\phi$). Setting this to zero for all $\phi$ gives $H = 0$.

**Mean curvature flow** is the evolution $\dot{x} = H\hat{N}$: each point moves in the normal direction at a rate equal to the mean curvature. This is the gradient flow of the area functional and smooths out surface irregularities. It is the geometric analogue of the heat equation, and like the heat equation, it can drive solutions to extinction (the surface shrinks and disappears) in finite time.

## Curvature of Level Sets

For a surface given as a level set $F(x,y,z) = c$ with $|\nabla F| > 0$, the unit normal is $\hat{N} = \nabla F / |\nabla F|$. The mean curvature is:

$$H = -\frac{1}{2}\text{div}\left(\frac{\nabla F}{|\nabla F|}\right) = -\frac{|\nabla F|^2 \Delta F - \sum_{i,j} F_{x_i} F_{x_j} F_{x_ix_j}}{2|\nabla F|^3}.$$

For a sphere $F = x^2 + y^2 + z^2 - R^2$: $|\nabla F| = 2R$, $\Delta F = 6$, and the mean curvature formula gives $H = 1/R$, confirming our computation.

## Applications

**Capillarity.** The Young-Laplace equation $\Delta P = 2H\gamma$ (where $\Delta P$ is the pressure difference across a soap film and $\gamma$ is surface tension) shows that surfaces of constant mean curvature are equilibrium shapes for pressurized soap bubbles.

**Computer graphics and image processing.** Mean curvature flow is used for surface smoothing and denoising. Surfaces are evolved toward smaller area while preserving shape features; the flow is discretized for computational meshes.

**General relativity.** The constraint equations for the initial data of the Einstein field equations involve the mean curvature of spacelike hypersurfaces in the spacetime manifold.
