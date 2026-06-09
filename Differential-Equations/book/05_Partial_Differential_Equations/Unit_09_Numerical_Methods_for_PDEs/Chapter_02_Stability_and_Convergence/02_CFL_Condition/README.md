# The CFL Condition

In 1928, Richard Courant, Kurt Friedrichs, and Hans Lewy published a paper establishing that for any explicit finite difference scheme applied to a hyperbolic PDE, the time step $\Delta t$ must satisfy a restriction relative to the spatial step $\Delta x$ — or the scheme will be unstable. This restriction is now known as the **CFL condition**. For the advection equation $u_t + cu_x = 0$: $|c|\Delta t/\Delta x \leq 1$. For the wave equation $u_{tt} = c^2u_{xx}$: $c\Delta t/\Delta x \leq 1$. The CFL condition is not an artifact of the discretization; it is a fundamental constraint from the geometry of characteristics.

## Domain of Dependence: The Core Idea

The **physical domain of dependence** of the point $(x_j, t_n)$ for a PDE is the set of initial data points that can influence the solution at $(x_j, t_n)$. For:

- **Advection equation** $u_t + cu_x = 0$: the characteristic through $(x_j, t_n)$ is $x = x_j - c(t_n - t)$. The domain of dependence is the single point $x_j - ct_n$.

- **Wave equation** $u_{tt} = c^2u_{xx}$: d'Alembert's formula shows the solution at $(x_j, t_n)$ depends on initial data in $[x_j - ct_n, x_j + ct_n]$.

The **numerical domain of dependence** of $(x_j, t_n)$ is the set of initial grid points $\{x_k\}$ that contribute to $U_j^n$ through the computational stencil. For a scheme that updates $U_j^{n+1}$ from $U_{j-p}^n, \ldots, U_{j+q}^n$ (a stencil of width $p+q+1$), the numerical domain of dependence grows by $p+q$ grid points per time step. After $n$ steps: the numerical domain of dependence of $U_j^n$ is $[x_j - p\cdot n\Delta x, x_j + q\cdot n\Delta x]$.

**CFL theorem (1928).** A necessary condition for convergence (as $\Delta x, \Delta t \to 0$ with $\Delta t/\Delta x$ fixed) is that the physical domain of dependence is contained within the numerical domain of dependence.

**Proof idea.** If the physical domain of dependence contains a point $x^*$ not in the numerical domain of dependence, then changing the initial data at $x^*$ affects the exact solution $u(x_j, t_n)$ but cannot affect $U_j^n$ (since $x^*$ is outside the numerical domain of dependence). As $h \to 0$, the numerical solution cannot converge to the exact solution, which depends on the initial data at $x^*$.

## Advection Equation: Upwind and Lax-Wendroff

For $u_t + cu_x = 0$ with $c > 0$, the characteristic speed is $c$ (rightward). The physical domain of dependence of $(x_j, t_n)$ is the single point $x_j - ct_n$.

**First-order upwind scheme.** $U_j^{n+1} = U_j^n - \lambda(U_j^n - U_{j-1}^n)$, where $\lambda = c\Delta t/\Delta x$. The numerical domain of dependence grows leftward by $\Delta x$ per step: after $n$ steps, it extends from $x_j - n\Delta x$ to $x_j$.

**CFL condition.** For the physical domain of dependence (the single point $x_j - ct_n = x_j - cn\Delta t$) to lie within $[x_j - n\Delta x, x_j]$: require $n\Delta x \geq cn\Delta t$, i.e., $\lambda = c\Delta t/\Delta x \leq 1$.

**Von Neumann confirmation.** The amplification factor is $|\xi|^2 = 1 - 2\lambda(1-\lambda)(1-\cos\theta)$. Stability ($|\xi| \leq 1$) requires $\lambda(1-\lambda) \geq 0$, i.e., $0 \leq \lambda \leq 1$ — precisely the CFL condition.

**Lax-Wendroff scheme.** For higher accuracy: $U_j^{n+1} = U_j^n - \frac{\lambda}{2}(U_{j+1}^n - U_{j-1}^n) + \frac{\lambda^2}{2}(U_{j+1}^n - 2U_j^n + U_{j-1}^n)$.

This is second-order in both time and space (LTE $= O(\Delta t^2 + (\Delta x)^2)$). Its amplification factor:

$$\xi = 1 - i\lambda\sin\theta - \lambda^2(1-\cos\theta).$$

Stability analysis: $|\xi|^2 = (1-\lambda^2(1-\cos\theta))^2 + \lambda^2\sin^2\theta = 1 - \lambda^2(1-\lambda^2)(1-\cos\theta)^2$.

For $|\xi|^2 \leq 1$: need $\lambda^2(1-\lambda^2)(1-\cos\theta)^2 \geq 0$, which holds iff $\lambda^2 \leq 1$, i.e., $|\lambda| \leq 1$. Same CFL condition.

**Modified equation.** Expanding $U_j^{n+1}$ in Taylor series and comparing with the advection equation reveals the **modified equation** actually solved:

$$u_t + cu_x = \frac{c(\Delta x)^2}{6}(1-\lambda^2)u_{xxx} + O((\Delta x)^4). \tag{LW modified eq}$$

The leading error term is dispersive ($u_{xxx}$, not dissipative): Lax-Wendroff produces spurious oscillations near discontinuities (Gibbs phenomenon), even though it is stable and second-order accurate for smooth solutions.

Compare with the upwind modified equation:

$$u_t + cu_x = \frac{c\Delta x}{2}(1-\lambda)u_{xx} + O((\Delta x)^2). \tag{upwind modified eq}$$

The leading error term is dissipative ($u_{xx}$, i.e., numerical diffusion): upwind smears discontinuities but does not oscillate. The trade-off between dissipation and dispersion is fundamental to numerical scheme design.

## Wave Equation: CFL Revisited

For $u_{tt} = c^2u_{xx}$, the leapfrog scheme (with Courant number $\lambda = c\Delta t/\Delta x$) has the numerical domain of dependence growing by $\Delta x$ per time step in both directions. After $n$ steps: numerical domain = $[x_j - n\Delta x, x_j + n\Delta x]$.

Physical domain = $[x_j - ct_n, x_j + ct_n] = [x_j - cn\Delta t, x_j + cn\Delta t]$.

Containment: $n\Delta x \geq cn\Delta t$ iff $\lambda \leq 1$.

**At the CFL limit $\lambda = 1$.** The numerical characteristics $x = x_j \pm n\Delta x$ exactly match the physical characteristics $x = x_j \pm cn\Delta t$. The leapfrog scheme propagates information exactly along the physical characteristics — this is why it is dispersion-free at $\lambda = 1$ (as shown in Section 4 of Chapter 1).

**Above the CFL limit $\lambda > 1$.** The physical characteristics leave the numerical cone: information that physically should influence $(x_j, t_n)$ does not reach it in the numerical scheme. The amplification factor has $|\xi| > 1$ for some $\theta$, and the scheme diverges exponentially.

## Multidimensional CFL Conditions

For the 2D wave equation $u_{tt} = c^2(u_{xx}+u_{yy})$ with the explicit scheme:

$$U_{j,k}^{n+1} = 2U_{j,k}^n - U_{j,k}^{n-1} + \lambda_x^2(U_{j+1,k}^n-2U_{j,k}^n+U_{j-1,k}^n) + \lambda_y^2(U_{j,k+1}^n-2U_{j,k}^n+U_{j,k-1}^n),$$

where $\lambda_x = c\Delta t/\Delta x$ and $\lambda_y = c\Delta t/\Delta y$.

The von Neumann amplification factor satisfies $|\xi| = 1$ iff $\alpha_x + \alpha_y \leq 1$ where $\alpha_x = 2\lambda_x^2\sin^2(\theta_x/2)$ and $\alpha_y = 2\lambda_y^2\sin^2(\theta_y/2)$. The worst case is $\theta_x = \theta_y = \pi$: $\alpha_x + \alpha_y = 2(\lambda_x^2 + \lambda_y^2) \leq 2$, giving:

$$\lambda_x^2 + \lambda_y^2 = c^2\Delta t^2\left(\frac{1}{(\Delta x)^2} + \frac{1}{(\Delta y)^2}\right) \leq 1.$$

For a square grid $\Delta x = \Delta y = h$: $c\Delta t \leq h/\sqrt{2}$, i.e., $\lambda = c\Delta t/h \leq 1/\sqrt{2} \approx 0.707$. The 2D CFL condition is **more restrictive** than the 1D condition by a factor of $1/\sqrt{d}$ in $d$ dimensions: $\lambda \leq 1/\sqrt{d}$.

For the $d$-dimensional wave equation on a uniform grid: $c\Delta t/h \leq 1/\sqrt{d}$.

This restriction is significant: in 3D, the time step must be at most $h/(c\sqrt{3})$, which is 42% smaller than the 1D limit. Practical consequence: 3D explicit wave simulation requires $\sim\sqrt{3}$ times as many time steps as the same problem in 1D for the same spatial resolution.

## The CFL Condition for Parabolic Equations

For the heat equation $u_t = \kappa u_{xx}$, the physical domain of dependence is the entire initial line $\mathbb{R}$ (heat propagates instantaneously — the equation is parabolic, with infinite speed of propagation). Thus the CFL argument does not apply: there is no geometric constraint on $\Delta t/\Delta x$.

The FTCS stability condition $r = \kappa\Delta t/(\Delta x)^2 \leq 1/2$ is therefore purely algebraic (a stability condition on the amplification factor) rather than a geometric CFL constraint. As $\Delta x \to 0$ with $r$ fixed: $\Delta t \sim (\Delta x)^2$ — the time step must decrease quadratically with the spatial step, a far more severe restriction than the linear $\Delta t \sim \Delta x$ of hyperbolic CFL conditions.

This is the fundamental contrast between parabolic and hyperbolic explicit schemes: parabolic requires $\Delta t = O((\Delta x)^2)$ (very small time steps for fine grids); hyperbolic requires $\Delta t = O(\Delta x)$ (moderately small time steps). Implicit methods eliminate the parabolic stability restriction entirely and reduce the hyperbolic restriction (though fully implicit schemes for hyperbolic equations introduce excessive numerical dissipation).

## Practical Guidance for Choosing $\Delta t$

For **hyperbolic problems** (wave equation, advection):
- Use $\lambda = c\Delta t/\Delta x = 0.9$ (just below the stability limit) to minimize numerical dispersion.
- The leapfrog scheme at $\lambda = 1$ is exactly dispersion-free but requires $\Delta x$ to be commensurate with $c$.

For **parabolic problems** (heat equation) with explicit schemes:
- Use $r = \kappa\Delta t/(\Delta x)^2 = 0.4$ (a comfortable margin below $r = 1/2$).
- Consider implicit or Crank-Nicolson schemes for fine grids to avoid the $\Delta t = O((\Delta x)^2)$ restriction.

For **mixed problems** (reaction-diffusion, compressible flow): the effective time step restriction is the minimum of all applicable conditions. For $u_t = Du_{xx} + f(u)$: diffusive restriction $\Delta t \leq (\Delta x)^2/(2D)$ and reactive restriction $\Delta t \leq 1/\max|f'(u)|$.
