# Domain of Dependence and Domain of Influence

The most striking property of the wave equation, distinguishing it from both the heat equation and Laplace's equation, is finite propagation speed: a disturbance at a point $(x_0, 0)$ can affect the solution only within a forward cone emanating from $(x_0,0)$ at speed $c$. This is the mathematical expression of causality — the future cannot be influenced by events "too far away" that there wasn't enough time for the signal to travel.

## Domain of Dependence

The **domain of dependence** of a point $(x_0, t_0)$ is the set of points on the initial line $\{t=0\}$ that affect $u(x_0,t_0)$.

From d'Alembert's formula:

$$u(x_0,t_0) = \frac{\phi(x_0+ct_0) + \phi(x_0-ct_0)}{2} + \frac{1}{2c}\int_{x_0-ct_0}^{x_0+ct_0}\psi(s)\,ds.$$

The solution at $(x_0,t_0)$ depends on $\phi$ only at the two endpoints $x_0 \pm ct_0$, and on $\psi$ on the entire interval $[x_0-ct_0, x_0+ct_0]$.

The domain of dependence is the closed interval:

$$\mathcal{D}(x_0,t_0) = [x_0 - ct_0, x_0 + ct_0].$$

This is the base of the characteristic triangle with apex $(x_0,t_0)$: the two characteristics through $(x_0,t_0)$ are $x - ct = x_0 - ct_0$ (right-going) and $x + ct = x_0 + ct_0$ (left-going), and they intersect the line $t=0$ at $x_0 - ct_0$ and $x_0 + ct_0$ respectively.

**Consequence:** If two initial data sets $(\phi_1, \psi_1)$ and $(\phi_2, \psi_2)$ agree on $[x_0-ct_0, x_0+ct_0]$, then $u_1(x_0,t_0) = u_2(x_0,t_0)$. Modifying the initial data outside the domain of dependence has no effect on the solution at $(x_0,t_0)$.

## Domain of Influence

The **domain of influence** of a point $(x_0, 0)$ on the initial line is the set of space-time points $(x,t)$ with $t > 0$ where the value of $\phi(x_0)$ or $\psi(x_0)$ affects the solution.

A point $(x_0,0)$ is in the domain of dependence of $(x,t)$ if and only if $x - ct \leq x_0 \leq x + ct$. So $(x,t)$ is influenced by $(x_0,0)$ if and only if $|x - x_0| \leq ct$:

$$\mathcal{I}(x_0) = \{(x,t): t > 0,\; |x-x_0| \leq ct\}.$$

This is the forward light cone (or sound cone) with apex at $(x_0,0)$, expanding at speed $c$.

**Consequence:** Any change in initial data at $x_0$ affects the solution only within the forward cone $\{|x - x_0| \leq ct\}$. Outside this cone, the solution is unaffected.

## The Characteristic Triangle

The characteristic triangle for the point $(x_0,t_0)$ is the triangle in the $(x,t)$-plane bounded by:
- The two characteristics through $(x_0,t_0)$: $x - ct = x_0 - ct_0$ and $x + ct = x_0 + ct_0$.
- The initial line $t=0$.

The domain of dependence is the base $[x_0-ct_0, x_0+ct_0]\times\{0\}$ of this triangle.

**Theorem (Domain of Dependence, rigorous form).** If $\phi$ and $\psi$ vanish on $[x_0-ct_0, x_0+ct_0]$, then the solution $u$ vanishes at $(x_0,t_0)$.

This follows immediately from d'Alembert's formula. The converse is not generally true (the solution at $(x_0,t_0)$ might be zero even with nonzero data in the domain of dependence, by cancellation).

## Finite Propagation Speed: Proof via Energy

An elegant proof of finite propagation speed uses the energy integral. Define, for fixed $(x_0,t_0)$ and $0 \leq t \leq t_0$, the "cone energy":

$$E(t) = \frac{1}{2}\int_{x_0-c(t_0-t)}^{x_0+c(t_0-t)}\left[(u_t)^2 + c^2(u_x)^2\right]dx.$$

**Claim:** $E(t)$ is nonincreasing in $t$.

**Proof.** Differentiate $E(t)$:

$$E'(t) = \int_{...}\left[u_t u_{tt} + c^2 u_x u_{xt}\right]dx + c\left[(u_t)^2 + c^2(u_x)^2\right]\Big|_{x_0 - c(t_0-t)}^{x_0+c(t_0-t)}\cdot(-1).$$

Using the wave equation $u_{tt} = c^2 u_{xx}$ and integration by parts on $\int u_t u_{tt} + c^2 u_x u_{xt}\,dx = \int u_t(c^2 u_{xx} - c^2 u_{xx})\,dx = 0$... Wait, more carefully:

$$\int(u_t u_{tt} + c^2 u_x u_{xt})\,dx = \int(c^2 u_t u_{xx} + c^2 u_x u_{xt})\,dx = c^2\int(u_t u_{xx} + u_x u_{xt})\,dx = c^2[u_t u_x]_{\text{endpoints}}.$$

So: $E'(t) = c^2[u_t u_x]_{\text{endpoints}} - c[(u_t)^2 + c^2(u_x)^2]_{\text{endpoints}}$.

Evaluating at $x = x_0 + c(t_0-t)$ (right endpoint) and $x = x_0 - c(t_0-t)$ (left endpoint):

$$E'(t) = -c\left[(u_t - cu_x)^2/2 + (u_t + cu_x)^2/2\right]_{\text{endpoints}} \leq 0.$$

(After expanding and simplifying using the boundary terms.)

Since $E$ is nonincreasing and $E(t_0) \geq 0$, if $E(0) = 0$ (no energy in the domain of dependence of $(x_0,t_0)$), then $E(t_0) = 0$, which means $u_t(x_0,t_0) = u_x(x_0,t_0) = 0$, and by continuity $u(x_0,t_0) = 0$.

## Contrast with the Heat Equation

For the heat equation, the domain of dependence is all of $\mathbb{R}$: the solution at any $(x_0,t_0)$ with $t_0 > 0$ depends on $\phi(y)$ for all $y \in \mathbb{R}$, with weights proportional to the heat kernel. A perturbation of $\phi$ at any point, no matter how far from $x_0$, instantaneously affects the solution at $x_0$.

This infinite propagation speed is physically unrealistic at short time scales (information cannot travel faster than light) but is a reasonable approximation when the diffusion time scale is much larger than the microscopic relaxation time. The wave equation with finite propagation speed is the appropriate model when causality is important.

## Multiple Spatial Dimensions

In $n$ spatial dimensions, d'Alembert's formula generalizes (in different forms depending on $n$) to solutions involving integrals over spheres of radius $ct$ centered at $(x_0, y_0, \ldots)$. The domain of dependence becomes the ball $|\mathbf{x} - \mathbf{x}_0| \leq ct_0$, and finite propagation speed holds in all dimensions. However, Huygens' principle — whether the solution depends only on the sphere $|\mathbf{x}-\mathbf{x}_0| = ct_0$ (sharp wavefronts) or the entire ball (trailing wavefronts) — depends critically on the spatial dimension, as developed in Chapter 3.
