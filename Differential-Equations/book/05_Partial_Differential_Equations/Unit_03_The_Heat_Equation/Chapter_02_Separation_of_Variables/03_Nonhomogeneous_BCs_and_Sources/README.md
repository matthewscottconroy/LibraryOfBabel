# Nonhomogeneous Boundary Conditions and Source Terms

The separation of variables and eigenfunction expansion methods as developed in the previous sections assume homogeneous boundary conditions and no source terms. Real problems rarely have this property: the boundary of a heat conductor might be held at a nonzero prescribed temperature, or internal heat sources might be present. This section develops systematic strategies for reducing nonhomogeneous problems to homogeneous ones and for handling source terms via eigenfunction expansions.

## Nonhomogeneous Dirichlet Conditions: Time-Independent Case

Consider the heat equation with fixed nonzero endpoint temperatures:

$$u_t = \kappa u_{xx}, \quad 0 < x < L,\; t > 0,$$
$$u(0,t) = T_1, \quad u(L,t) = T_2, \quad t > 0,$$
$$u(x,0) = f(x), \quad 0 < x < L.$$

**Strategy: subtract the steady state.** As $t\to\infty$, the solution should approach the steady state $u_s(x)$ satisfying $u_s'' = 0$, $u_s(0) = T_1$, $u_s(L) = T_2$. The solution is the linear function $u_s(x) = T_1 + (T_2-T_1)x/L$.

Set $v(x,t) = u(x,t) - u_s(x)$. Then:

$$v_t = \kappa v_{xx}, \quad v(0,t) = 0, \quad v(L,t) = 0, \quad v(x,0) = f(x) - u_s(x).$$

This is a homogeneous problem with initial data $g(x) = f(x) - u_s(x)$. Solve for $v$ by the eigenfunction expansion:

$$v(x,t) = \sum_{n=1}^\infty b_n\sin\!\left(\frac{n\pi x}{L}\right)e^{-\kappa(n\pi/L)^2 t}, \qquad b_n = \frac{2}{L}\int_0^L g(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

The solution is $u(x,t) = u_s(x) + v(x,t)$.

As $t\to\infty$, $v\to 0$ exponentially and $u\to u_s$ — the system equilibrates to the steady state.

## Nonhomogeneous Boundary Conditions: Time-Dependent Case

If the boundary conditions depend on time: $u(0,t) = g_1(t)$, $u(L,t) = g_2(t)$, the steady-state method generalizes by introducing a quasi-static reference function:

$$w(x,t) = g_1(t) + \frac{g_2(t)-g_1(t)}{L}x$$

(a linear interpolation satisfying the boundary conditions at each instant). Then $v = u - w$ satisfies:

$$v_t = \kappa v_{xx} - w_t, \quad v(0,t) = 0, \quad v(L,t) = 0, \quad v(x,0) = f(x) - w(x,0).$$

This is a nonhomogeneous heat equation (with source term $-w_t$) and homogeneous boundary conditions — handled by Duhamel's principle (Chapter 5) or directly by expanding in eigenfunctions.

## Source Terms: Eigenfunction Expansion Method

Consider the nonhomogeneous heat equation:

$$u_t = \kappa u_{xx} + F(x,t), \quad u(0,t) = u(L,t) = 0, \quad u(x,0) = f(x).$$

Expand both $u$ and $F$ in the eigenbasis $\{\sin(n\pi x/L)\}$:

$$u(x,t) = \sum_{n=1}^\infty T_n(t)\sin\!\left(\frac{n\pi x}{L}\right), \quad F(x,t) = \sum_{n=1}^\infty f_n(t)\sin\!\left(\frac{n\pi x}{L}\right),$$

where $f_n(t) = \frac{2}{L}\int_0^L F(x,t)\sin(n\pi x/L)\,dx$.

Substituting into the PDE:

$$\sum_n T_n'(t)\sin\!\left(\frac{n\pi x}{L}\right) = \kappa\sum_n T_n(t)\left(-\frac{n^2\pi^2}{L^2}\right)\sin\!\left(\frac{n\pi x}{L}\right) + \sum_n f_n(t)\sin\!\left(\frac{n\pi x}{L}\right).$$

By orthogonality, each mode satisfies the first-order linear ODE:

$$T_n'(t) + \kappa\frac{n^2\pi^2}{L^2}T_n(t) = f_n(t), \qquad T_n(0) = b_n = \frac{2}{L}\int_0^L f(x)\sin\!\left(\frac{n\pi x}{L}\right)dx.$$

This ODE has the explicit solution by integrating factor:

$$T_n(t) = b_n e^{-\kappa\lambda_n t} + \int_0^t e^{-\kappa\lambda_n(t-s)}f_n(s)\,ds,$$

where $\lambda_n = (n\pi/L)^2$. This is the variation of parameters formula applied to each mode. Summing over $n$ gives the complete solution.

## Physical Interpretation: Modes and Sources

The eigenfunction expansion transforms the PDE into infinitely many decoupled mode equations. Each mode $T_n(t)$ responds independently to its corresponding component $f_n(t)$ of the source. The damping rate $\kappa\lambda_n$ for the $n$-th mode increases with $n^2$ — high-frequency modes are strongly damped and respond sluggishly to oscillating sources. Low-frequency modes (especially the $n=1$ fundamental mode) dominate the response to sources with significant low-frequency content.

## Steady State with Sources

If $F(x,t) = F(x)$ (time-independent source), the steady state $u_s$ satisfies $\kappa u_s'' + F(x) = 0$ with $u_s(0) = u_s(L) = 0$ — a Poisson equation. Setting $v = u - u_s$ reduces to the homogeneous heat equation with modified initial data. The transient $v$ decays to zero and $u \to u_s$ as $t\to\infty$.

## Example: Linearly Ramped Temperature

Consider a rod $[0,\pi]$ with boundary conditions $u(0,t) = 0$, $u(\pi,t) = t$ (linearly increasing endpoint temperature), zero initial condition $u(x,0) = 0$.

The reference function is $w(x,t) = tx/\pi$. Then $v = u - w$ satisfies $v_t = v_{xx} - x/\pi$, $v(0,t) = v(\pi,t) = 0$, $v(x,0) = 0$.

The source in eigenfunction form: $-x/\pi = -\sum_n \frac{2}{\pi}\int_0^\pi (x/\pi)\sin(nx)\,dx \cdot \sin(nx) = \sum_n \frac{2(-1)^{n+1}}{n\pi}\sin(nx)$.

So $f_n(t) = 2(-1)^{n+1}/(n\pi)$ (constant in $t$). The mode ODE $T_n' + n^2 T_n = 2(-1)^{n+1}/(n\pi)$, with $T_n(0) = 0$, has solution:

$$T_n(t) = \frac{2(-1)^{n+1}}{n^3\pi}(1 - e^{-n^2 t}).$$

As $t\to\infty$: $T_n\to 2(-1)^{n+1}/(n^3\pi)$, and $v \to v_s = \sum_n \frac{2(-1)^{n+1}}{n^3\pi}\sin(nx)$ — the Fourier series of $x(\pi-x)/(2\pi)$ (a standard result). The total solution $u = w + v \to tx/\pi + v_s$ — the linearly growing part dominates at large times, as expected.
