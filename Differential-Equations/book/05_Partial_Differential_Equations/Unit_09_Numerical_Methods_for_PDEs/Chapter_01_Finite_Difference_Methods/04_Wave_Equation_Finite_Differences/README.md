# Wave Equation: Finite Differences

The wave equation $u_{tt} = c^2 u_{xx}$ presents different challenges than the heat equation. It is hyperbolic rather than parabolic, so solutions propagate at finite speed $c$ rather than spreading instantly. The natural numerical scheme — centered differences in both time and space — is explicit and conditionally stable with the **CFL (Courant-Friedrichs-Lewy) condition** $c\Delta t/\Delta x \leq 1$. This condition has a beautiful physical interpretation: the numerical domain of dependence must include the physical domain of dependence.

## The Standard Scheme (Leapfrog)

For $u_{tt} = c^2 u_{xx}$ on $[0,L]\times[0,T]$ with boundary conditions $u(0,t) = u(L,t) = 0$ and initial conditions $u(x,0) = f(x)$, $u_t(x,0) = g(x)$:

$$\frac{U_j^{n+1}-2U_j^n+U_j^{n-1}}{(\Delta t)^2} = c^2\frac{U_{j+1}^n-2U_j^n+U_{j-1}^n}{(\Delta x)^2}. \tag{Wave FD}$$

Solving for $U_j^{n+1}$:

$$U_j^{n+1} = \lambda^2 U_{j+1}^n + 2(1-\lambda^2)U_j^n + \lambda^2 U_{j-1}^n - U_j^{n-1}, \qquad \lambda = \frac{c\Delta t}{\Delta x}. \tag{Explicit update}$$

The parameter $\lambda$ is the **Courant number** (or CFL number). This is the **leapfrog scheme**: it uses three time levels ($n-1$, $n$, $n+1$) and is explicit.

**Local truncation error.** By Taylor expansion:

$$\tau_j^n = \frac{\Delta t^2}{12}u_{tttt} - c^2\frac{(\Delta x)^2}{12}u_{xxxx} + O(\Delta t^4 + (\Delta x)^4) = O(\Delta t^2 + (\Delta x)^2).$$

The scheme is **second-order in both time and space**.

## CFL Stability Condition

**Theorem.** The leapfrog scheme (Wave FD) is stable if and only if $\lambda = c\Delta t/\Delta x \leq 1$.

**Proof (von Neumann).** Seek solutions of the form $U_j^n = \xi^n e^{ij\theta}$. Substituting into (Wave FD):

$$\xi^2 e^{ij\theta} - 2\xi e^{ij\theta} + e^{ij\theta} = \lambda^2\xi(e^{i(j+1)\theta} - 2e^{ij\theta} + e^{i(j-1)\theta}).$$

Dividing by $\xi e^{ij\theta}$: $\xi - 2 + \xi^{-1} = \lambda^2(e^{i\theta}-2+e^{-i\theta}) = -4\lambda^2\sin^2(\theta/2)$.

Setting $\alpha = 2\lambda^2\sin^2(\theta/2)$: $\xi + \xi^{-1} = 2 - 2\alpha = 2(1-\alpha)$.

The quadratic $\xi^2 - 2(1-\alpha)\xi + 1 = 0$ has solutions $\xi = (1-\alpha) \pm \sqrt{(1-\alpha)^2-1}$.

- If $|1-\alpha| \leq 1$ (i.e., $0 \leq \alpha \leq 2$): the discriminant is $\leq 0$, so $\xi = (1-\alpha)\pm i\sqrt{1-(1-\alpha)^2}$. Then $|\xi|^2 = (1-\alpha)^2 + 1-(1-\alpha)^2 = 1$ — the Fourier mode is propagated without growth ($|\xi| = 1$).

- If $\alpha > 2$ (i.e., $\lambda^2\sin^2(\theta/2) > 1$): the discriminant $>0$ and $\xi_{1,2}$ are real with $\xi_1\xi_2 = 1$. Since $\xi_1+\xi_2 = 2(1-\alpha) < -2$, both roots have absolute value $\neq 1$, and the larger root $|\xi_{\max}| > 1$ — unstable.

The condition $\alpha = 2\lambda^2\sin^2(\theta/2) \leq 2$ for all $\theta$ requires $\lambda^2\leq 1$, i.e., $\lambda \leq 1$. $\square$

**CFL condition: $c\Delta t \leq \Delta x$.**

## Physical Interpretation: Domain of Dependence

The exact solution $u(x_j,t_n)$ depends on $u_0$ in the interval $[x_j - ct_n, x_j + ct_n]$ (domain of dependence from d'Alembert's formula). The numerical scheme (Wave FD) computes $U_j^n$ from $U_j^{n-1}$, $U_{j\pm 1}^{n-1}$, $U_j^{n-2}$ — effectively from the triangle of points reachable by stepping back in time. The **numerical domain of dependence** grows by $\pm\Delta x$ per time step $\Delta t$, so after $n$ steps it covers $[x_j - n\Delta x, x_j + n\Delta x] = [x_j - (c\Delta t/\Delta x)\cdot ct_n/c, \ldots]$.

**CFL condition in terms of domains:** The scheme is stable iff the numerical domain of dependence contains the physical domain of dependence:

$$[x_j - n\Delta x, x_j + n\Delta x] \supseteq [x_j - ct_n, x_j + ct_n]$$
$$\iff n\Delta x \geq ct_n = cn\Delta t \iff \Delta x \geq c\Delta t \iff \lambda \leq 1.$$

**Violation of CFL.** If $\lambda > 1$: the numerical domain of dependence is too narrow — information at the physical boundaries of the domain of dependence ($x = x_j \pm ct_n$) does not reach the grid point $(x_j, t_n)$ in time. The scheme "misses" the physical characteristics and produces an incorrect (and unstable) solution.

## Exact Dispersion Relation

The continuous wave equation has dispersion relation $\omega = ck$ (all waves travel at speed $c$, no dispersion). The leapfrog scheme has numerical dispersion: substituting $U_j^n = e^{i(k x_j - \omega t_n)}$ into (Wave FD):

$$\frac{-4\sin^2(\omega\Delta t/2)}{(\Delta t)^2} = c^2\frac{-4\sin^2(k\Delta x/2)}{(\Delta x)^2}.$$

Numerical dispersion relation: $\frac{\sin(\omega\Delta t/2)}{\Delta t/2} = c\frac{\sin(k\Delta x/2)}{\Delta x/2}$.

For small $k\Delta x$ and $\omega\Delta t$ (long waves): $\omega \approx ck$ — the numerical phase velocity agrees with $c$. For large $k\Delta x \approx \pi$ (short waves / Nyquist): $\sin(k\Delta x/2) = \sin(\pi/2) = 1$, but $\sin(\omega\Delta t/2) = (\Delta t/\Delta x)\cdot c$. So $\omega\Delta t/2 = \arcsin(\lambda) \neq \pi/2$ unless $\lambda = 1$. Short waves travel at a different speed than long waves — numerical dispersion causes wave packets to spread over time.

**At the CFL limit $\lambda = 1$:** The numerical dispersion relation becomes $\sin(\omega\Delta t/2) = \sin(k\Delta x/2)$, giving $\omega = k\cdot(\Delta x/\Delta t) = kc$ — the scheme is dispersion-free! At the CFL limit, the leapfrog scheme for the wave equation is exact (no numerical dispersion) for any wavelength. This remarkable property explains why the CFL-limit leapfrog scheme is the gold standard for wave propagation.

## Initial Conditions

To start the leapfrog scheme (which requires two initial time levels $U^0$ and $U^{-1}$):

- **$U_j^0 = f(x_j)$** (from $u(x,0) = f(x)$).
- **$U_j^{-1}$:** From $u_t(x,0) = g(x)$, use the Taylor expansion: $U_j^{-1} = U_j^0 - \Delta t\, g(x_j) + \frac{(\Delta t)^2}{2}u_{tt}(x_j,0)$. Using $u_{tt} = c^2 u_{xx}$ at $t=0$: $U_j^{-1} = f(x_j) - \Delta t\, g(x_j) + \frac{c^2(\Delta t)^2}{2}\frac{f(x_{j+1})-2f(x_j)+f(x_{j-1})}{(\Delta x)^2}$.

This initialization is second-order accurate in $\Delta t$.

## Worked Example: Plucked String

**Setup.** $c = 1$, $L = 1$, $f(x) = \sin(\pi x)$, $g(x) = 0$. $\Delta x = 0.1$, $\lambda = 1$ ($\Delta t = \Delta x/c = 0.1$). $T = 1$.

**Exact solution:** $u(x,t) = \cos(\pi t)\sin(\pi x)$ (standing wave, returns to initial condition at $t=1$).

**At $\lambda = 1$:** The numerical scheme is exact for sinusoidal initial data (zero dispersion). After $N = 10$ time steps ($T = 1$): the numerical solution should equal $U_j^{10} = \cos(\pi)\sin(\pi x_j) = -\sin(\pi x_j)$... wait, at $T = 1$: $u(x,1) = \cos(\pi)\sin(\pi x) = -\sin(\pi x)$. But the scheme is exact, so $U_j^{10} = -f(x_j) = -\sin(\pi x_j)$ — the string is at its extreme downward position after half a period.

At $T = 2$: $u(x,2) = \cos(2\pi)\sin(\pi x) = \sin(\pi x) = f(x)$ — the string returns to initial position. The numerical scheme reproduces this exactly, confirming zero numerical dispersion at $\lambda = 1$.

## Energy Conservation

The discrete energy for the leapfrog scheme:

$$E^n = \frac{1}{2}\sum_j\left[\frac{(U_j^{n+1}-U_j^n)^2}{(\Delta t)^2} + c^2\frac{(U_{j+1}^n-U_j^n)^2}{(\Delta x)^2}\right](\Delta x)(\Delta t)$$

(kinetic plus potential energy, discretized). For $\lambda \leq 1$: $E^n$ is conserved exactly (up to machine precision) — the leapfrog scheme for the wave equation is energy-preserving, consistent with the PDE's energy conservation.

For $\lambda > 1$: the discrete energy grows exponentially — numerical instability manifests as creation of energy, physically nonsensical.
