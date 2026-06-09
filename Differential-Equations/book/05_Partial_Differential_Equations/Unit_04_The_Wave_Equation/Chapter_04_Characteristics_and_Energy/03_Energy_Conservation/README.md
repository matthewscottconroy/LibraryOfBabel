# Energy Conservation for the Wave Equation

The conservation of energy is the quantitative heart of the well-posedness theory for the wave equation. Unlike the heat equation (which dissipates energy) or elliptic equations (which have no time evolution), the wave equation is a conservative system: the total mechanical energy (kinetic plus potential) is exactly preserved for all time. This conservation law implies uniqueness, continuous dependence on data, and the stability of numerical schemes.

## Definition of Energy

For the wave equation $u_{tt} = c^2\Delta u$ on a domain $\Omega$ with Dirichlet boundary conditions, the energy is:

$$E(t) = \frac{1}{2}\int_\Omega\left[(u_t)^2 + c^2|\nabla u|^2\right]d\mathbf{x}.$$

The two terms are the **kinetic energy density** $(u_t)^2/2$ (kinetic energy of the string/membrane per unit mass times mass density, simplified) and the **potential energy density** $c^2|\nabla u|^2/2 = (T/\rho)|\nabla u|^2/2$ (elastic potential energy stored in the stretched string).

## Proof of Energy Conservation

Multiply the wave equation by $u_t$:

$$u_t u_{tt} = c^2 u_t\Delta u.$$

The left side is $\frac{1}{2}\frac{\partial}{\partial t}(u_t)^2$. For the right side, use the identity $u_t\Delta u = \nabla\cdot(u_t\nabla u) - \nabla u_t\cdot\nabla u$:

$$c^2 u_t\Delta u = c^2\nabla\cdot(u_t\nabla u) - c^2\nabla u_t\cdot\nabla u = c^2\nabla\cdot(u_t\nabla u) - \frac{c^2}{2}\frac{\partial}{\partial t}|\nabla u|^2.$$

So the equation becomes:

$$\frac{\partial}{\partial t}\left[\frac{(u_t)^2 + c^2|\nabla u|^2}{2}\right] = c^2\nabla\cdot(u_t\nabla u).$$

Integrating over $\Omega$:

$$\frac{dE}{dt} = c^2\int_\Omega\nabla\cdot(u_t\nabla u)\,d\mathbf{x} = c^2\oint_{\partial\Omega}u_t\frac{\partial u}{\partial\nu}\,dS.$$

For Dirichlet conditions $u|_{\partial\Omega}=0$: $u_t|_{\partial\Omega}=0$ as well, so the boundary integral vanishes and $dE/dt = 0$.

For Neumann conditions $\partial u/\partial\nu|_{\partial\Omega}=0$: the boundary integral also vanishes.

**Theorem (Energy Conservation).** For the wave equation on a bounded domain $\Omega$ with homogeneous Dirichlet or Neumann boundary conditions, the energy $E(t)$ is constant:

$$E(t) = E(0) = \frac{1}{2}\int_\Omega\left[\psi^2 + c^2|\nabla\phi|^2\right]d\mathbf{x}$$

where $\phi = u(\cdot,0)$ and $\psi = u_t(\cdot,0)$ are the initial data.

## Uniqueness from Energy Conservation

Suppose $u_1$ and $u_2$ both satisfy the wave equation with the same initial and boundary data. Let $w = u_1 - u_2$. Then $w$ satisfies the wave equation with zero initial data ($w(\mathbf{x},0) = 0$, $w_t(\mathbf{x},0) = 0$) and homogeneous boundary conditions.

By energy conservation: $E(t) = E(0) = 0$ for all $t \geq 0$.

Since $E(t) = \frac{1}{2}\int[(w_t)^2 + c^2|\nabla w|^2]\,d\mathbf{x} = 0$, we conclude $w_t = 0$ and $|\nabla w| = 0$ everywhere, so $w$ is constant in space and time. The initial condition $w(\mathbf{x},0) = 0$ gives $w \equiv 0$.

## Energy Inequalities: Continuous Dependence

If the initial data $(\phi_1,\psi_1)$ and $(\phi_2,\psi_2)$ differ by $\varepsilon$ in the energy norm:

$$\frac{1}{2}\int\left[(\psi_1-\psi_2)^2 + c^2|\nabla(\phi_1-\phi_2)|^2\right]d\mathbf{x} \leq \varepsilon^2,$$

then $E_w(t) = \varepsilon^2$ for all $t$, so the difference $w = u_1 - u_2$ satisfies $\int(w_t^2 + c^2|\nabla w|^2) \leq 2\varepsilon^2$ for all $t$.

This does not immediately bound $\|w\|_{L^2}$, but by the Poincaré inequality ($\|\nabla w\|_{L^2} \geq C\|w\|_{L^2}$ for Dirichlet conditions), we get $\|w(\cdot,t)\|_{L^2} \leq C'\varepsilon$. The solution is Lipschitz continuous in the energy norm.

## Energy for the 1D Wave Equation

For $u_{tt} = c^2 u_{xx}$ on $\mathbb{R}$:

$$E(t) = \frac{1}{2}\int_{-\infty}^\infty\left[(u_t)^2 + c^2(u_x)^2\right]dx.$$

Using d'Alembert's formula $u = f(x+ct) + g(x-ct)$:
$u_t = cf' - cg'$, $u_x = f' + g'$.

Then $(u_t)^2 + c^2(u_x)^2 = 2c^2[(f')^2 + (g')^2]$, and

$$E = c^2\int_{-\infty}^\infty[(f')^2 + (g')^2]\,dx,$$

which is manifestly time-independent (since $\int(f'(x+ct))^2\,dx = \int(f')^2\,dx$ after substitution, independent of $t$). This confirms energy conservation directly from d'Alembert's formula.

## Local Energy and Finite Propagation Speed

Define the **local energy** in a ball $B_R(\mathbf{x}_0)$:

$$E_{B_R}(t) = \frac{1}{2}\int_{B_R(\mathbf{x}_0)}\left[(u_t)^2 + c^2|\nabla u|^2\right]d\mathbf{x}.$$

If the initial data is supported in $B_R(\mathbf{x}_0)$ (all energy initially concentrated in this ball), then by finite propagation speed, all energy remains within the ball $B_{R+ct}(\mathbf{x}_0)$ for time $t$. More precisely, by the cone energy argument (from Section 3.3), the local energy in $B_R(\mathbf{x}_0)$ is nonincreasing in time (energy can only leave, not enter, if there's no source outside).

This local energy decay property is the quantitative expression of finite propagation speed and is used extensively in the study of scattering theory and long-time behavior of wave equations.

## Comparison: Heat vs. Wave Energy

| Property | Heat equation | Wave equation |
|----------|--------------|---------------|
| Energy $E(t)$ | Decreasing ($E' \leq 0$) | Constant ($E' = 0$) |
| $\|u(\cdot,t)\|_{L^2}$ | Decreasing | Can grow (by Poincaré inequality) |
| Long-time behavior | Decay to zero | Perpetual oscillation |
| Uniqueness proof | $E \to 0$ directly | $E = E(0) = 0$ implies $w \equiv 0$ |
