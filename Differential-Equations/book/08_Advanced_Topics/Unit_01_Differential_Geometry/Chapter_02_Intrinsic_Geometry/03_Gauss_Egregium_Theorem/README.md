# The Gauss Theorema Egregium

Gauss called it the "remarkable theorem," and the name has stuck for nearly two centuries. The Theorema Egregium states that the Gaussian curvature $K$ of a surface in $\mathbb{R}^3$ is an **intrinsic** invariant: it can be computed entirely from the first fundamental form and its derivatives, without reference to the second fundamental form or to how the surface is embedded. A consequence is that Gaussian curvature is preserved by isometries—maps that preserve distances. You cannot flatten a sphere without distortion; Gauss gave the precise mathematical content of this fact.

## The Statement

**Theorem (Gauss, Theorema Egregium, 1828).** Let $S$ be a regular surface in $\mathbb{R}^3$ with first fundamental form coefficients $E, F, G$ in some local parametrization. Then the Gaussian curvature $K = \kappa_1\kappa_2$ can be expressed as a function of $E, F, G$ and their first and second partial derivatives alone.

The theorem is remarkable because $K$ is defined via the second fundamental form $h = \begin{pmatrix} L & M \\ M & N \end{pmatrix}$ as $K = (LN - M^2)/(EG - F^2)$—an extrinsic formula. That this extrinsic quantity equals an intrinsic one is deeply non-obvious.

## The Gauss Equation

The key identity underlying the theorem is the **Gauss equation** (or **equation of Gauss**):

$$R_{1212} = LN - M^2,$$

where $R_{1212}$ is a component of the **Riemann curvature tensor**, computed entirely from the Christoffel symbols (and hence from $E, F, G$):

$$R_{ijk}^l = \partial_i \Gamma^l_{jk} - \partial_j \Gamma^l_{ik} + \Gamma^m_{jk}\Gamma^l_{im} - \Gamma^m_{ik}\Gamma^l_{jm}.$$

The Gaussian curvature then equals $K = R_{1212}/(EG - F^2)$, which is purely intrinsic.

## Explicit Formula for Orthogonal Coordinates

When $F = 0$ (orthogonal coordinates), the formula simplifies substantially. With $E = e^{2\alpha}$, $G = e^{2\beta}$ (or any positive $E, G$), the Gaussian curvature is:

$$K = -\frac{1}{2\sqrt{EG}} \left[\frac{\partial}{\partial u}\left(\frac{\partial_u G}{\sqrt{EG}}\right) + \frac{\partial}{\partial v}\left(\frac{\partial_v E}{\sqrt{EG}}\right)\right].$$

For the sphere with $E = R^2$, $F = 0$, $G = R^2\sin^2\theta$:

$$\frac{\partial_\theta G}{\sqrt{EG}} = \frac{2R^2\sin\theta\cos\theta}{\sqrt{R^2 \cdot R^2\sin^2\theta}} = \frac{2R^2\sin\theta\cos\theta}{R^2\sin\theta} = 2\cos\theta.$$

$$\frac{\partial}{\partial\theta}(2\cos\theta) = -2\sin\theta, \quad \frac{\partial_\varphi E}{\sqrt{EG}} = 0.$$

$$K = -\frac{1}{2R^2\sin\theta}\left(-2\sin\theta + 0\right) = \frac{1}{R^2}. \checkmark$$

## Proof Sketch via Structural Equations

The cleanest proof uses the **Gauss structural equations** (compatibility conditions for the Frenet equations of a surface). Differentiating the Gauss equations:

$$\mathbf{r}_{uu} = \Gamma^1_{11}\mathbf{r}_u + \Gamma^2_{11}\mathbf{r}_v + L\hat{N}$$
$$\mathbf{r}_{uv} = \Gamma^1_{12}\mathbf{r}_u + \Gamma^2_{12}\mathbf{r}_v + M\hat{N}$$
$$\mathbf{r}_{vv} = \Gamma^1_{22}\mathbf{r}_u + \Gamma^2_{22}\mathbf{r}_v + N\hat{N}$$

and the Weingarten equations, cross-differentiating and using $\mathbf{r}_{uuv} = \mathbf{r}_{uvu}$ (equality of mixed partials applied to $\mathbf{r}_u$), one obtains an identity expressing $LN - M^2$ in terms of $\Gamma^k_{ij}$ and their derivatives—precisely the Gauss equation. Since $\Gamma^k_{ij}$ depend only on $E, F, G$, the theorem follows.

## Consequences

**Isometries preserve Gaussian curvature.** If $\phi: S_1 \to S_2$ is an isometry (preserves the first fundamental form), then $K_{S_1}(p) = K_{S_2}(\phi(p))$ for all $p$. This gives the immediate conclusion:

**Corollary.** A sphere is not locally isometric to a plane. The sphere has $K = 1/R^2 > 0$ everywhere; the plane has $K = 0$.

This is the mathematical theorem underlying the fact that any flat map of the earth must distort some distances or angles. Every map projection preserves some geometric feature (angles for conformal projections, areas for equal-area projections, geodesics for gnomonic projections) but cannot preserve all simultaneously.

**Corollary.** A surface with $K = 0$ everywhere is locally isometric to the plane. Such surfaces—called **flat surfaces** or **developable surfaces**—are precisely the surfaces that can be unrolled without stretching. They include cones, cylinders, and tangent developables.

## The Codazzi-Mainardi Equations

The integrability conditions for the structural equations yield, in addition to the Gauss equation, the **Codazzi-Mainardi equations**:

$$M_u - L_v = L\Gamma^2_{12} - M(\Gamma^2_{11} - \Gamma^1_{12}) - N\Gamma^1_{11},$$
$$N_u - M_v = L\Gamma^2_{22} - M(\Gamma^2_{12} - \Gamma^1_{22}) - N\Gamma^1_{12}.$$

These involve both the first and second fundamental forms. The Gauss equation and Codazzi-Mainardi equations together form the complete set of compatibility conditions for the existence of a surface with prescribed first and second fundamental forms—the **Bonnet theorem**:

**Theorem (Bonnet).** Given smooth functions $E, F, G$ (with $EG - F^2 > 0$) and $L, M, N$ on an open set $U \subset \mathbb{R}^2$ satisfying the Gauss equation and the Codazzi-Mainardi equations, there exists a regular surface $S$ with these first and second fundamental forms, unique up to rigid motion.

## Geometric Curvature and Geodesics

The Theorema Egregium also implies that **geodesic curvature** is intrinsic. A curve $\gamma$ on $S$ has geodesic curvature $\kappa_g$ measuring how much it bends within the surface (as opposed to normal curvature, which measures bending toward the ambient space). Since $\kappa_g$ depends only on the curve and the metric (first fundamental form), it is preserved by isometries. Geodesics ($\kappa_g = 0$) are intrinsically defined as the straightest curves, and the Theorema Egregium guarantees that isometries map geodesics to geodesics.

## Higher Dimensions: The Riemann Curvature Tensor

The Theorema Egregium extends to higher dimensions: in Riemannian geometry on an $n$-dimensional manifold, the **Riemann curvature tensor** $R$ is an intrinsic object computed from the metric tensor $g_{ij}$ and its derivatives. For surfaces ($n = 2$), $R$ has essentially one independent component, which is the Gaussian curvature. For $n \geq 3$, $R$ captures richer curvature information (sectional curvatures, Ricci curvature, scalar curvature) that is the basis of Riemannian geometry and Einstein's general relativity.
