# Adams-Bashforth Methods

The Adams-Bashforth methods are a family of explicit linear multistep methods for solving $y' = f(t,y)$, $y(t_0) = y_0$. They are derived by integrating an interpolating polynomial through previous $f$-values and achieve order $p$ with a $p$-step method using only one new function evaluation per step. For problems where $f$ is expensive to evaluate and where the step size need not change frequently, Adams-Bashforth methods offer excellent efficiency.

## Derivation from Integration

The fundamental observation is that integrating the ODE exactly:

$$y(t_{n+1}) - y(t_n) = \int_{t_n}^{t_{n+1}} f(t, y(t))\,dt.$$

The integral of $f$ over $[t_n, t_{n+1}]$ cannot be computed exactly (that would require knowing $y$), but it can be approximated by interpolating $f$ at previous mesh points $t_n, t_{n-1}, \ldots, t_{n-k+1}$ and integrating the interpolating polynomial exactly.

**Adams-Bashforth 2-step (AB2):** Interpolate $f$ at $t_n$ and $t_{n-1}$ using a linear polynomial $P_1(t)$ passing through $(t_{n-1}, f_{n-1})$ and $(t_n, f_n)$ (where $f_j = f(t_j, y_j)$). Integrate $P_1$ over $[t_n, t_{n+1}]$:

$$\int_{t_n}^{t_{n+1}} P_1(t)\,dt = h\left[\frac{3}{2}f_n - \frac{1}{2}f_{n-1}\right].$$

The AB2 formula is:

$$y_{n+1} = y_n + h\!\left(\frac{3}{2}f_n - \frac{1}{2}f_{n-1}\right).$$

This is second-order: LTE $= \frac{5}{12}h^3 y'''(\xi_n)$.

**Adams-Bashforth 4-step (AB4):** Interpolate $f$ at $t_{n-3}, t_{n-2}, t_{n-1}, t_n$ using a cubic polynomial and integrate over $[t_n, t_{n+1}]$:

$$y_{n+1} = y_n + h\!\left(\frac{55}{24}f_n - \frac{59}{24}f_{n-1} + \frac{37}{24}f_{n-2} - \frac{9}{24}f_{n-3}\right).$$

This is fourth-order: LTE $= \frac{251}{720}h^5 y^{(5)}(\xi_n)$.

## General Adams-Bashforth Formula

The $k$-step Adams-Bashforth method is:

$$y_{n+1} = y_n + h\sum_{j=0}^{k-1}\beta_j^* f_{n-j},$$

where the coefficients $\beta_j^*$ are determined by integrating the $(k-1)$-degree Lagrange interpolating polynomial through the points $(t_{n-j}, f_{n-j})$, $j = 0, 1, \ldots, k-1$. The method is $k$-th order accurate.

The coefficients for the first five Adams-Bashforth methods (normalized by $1/h$) are:

- AB1 (Euler): $[1]$
- AB2: $[3/2, -1/2]$
- AB3: $[23/12, -16/12, 5/12]$
- AB4: $[55/24, -59/24, 37/24, -9/24]$
- AB5: $[1901/720, -2774/720, 2616/720, -1274/720, 251/720]$

Note that Euler's method (AB1) is the trivial one-step Adams-Bashforth method.

## Startup

A $k$-step Adams-Bashforth method requires $k$ starting values $y_0, y_1, \ldots, y_{k-1}$ to initiate the recurrence. Only $y_0 = y(t_0)$ is given by the initial condition. The remaining starting values must be computed by another method. Typically one uses a Runge-Kutta method of matching order (e.g., RK4 to start AB4) to generate $y_1, y_2, y_3$. The startup values should have the same order of accuracy as the multistep method; otherwise the startup error will dominate.

## Stability and Zero-Stability

An explicit multistep method is characterized by its **characteristic polynomials**:

$$\rho(\zeta) = \sum_{j=0}^k \alpha_j \zeta^{k-j}, \qquad \sigma(\zeta) = \sum_{j=0}^k \beta_j \zeta^{k-j}.$$

For Adams-Bashforth $k$-step: $\rho(\zeta) = \zeta^k - \zeta^{k-1}$ (only the leading two terms are nonzero) and $\sigma(\zeta)$ encodes the $\beta_j^*$ coefficients.

**Zero-stability** requires that all roots of $\rho(\zeta)$ lie on or inside the unit disk, with roots on the unit circle being simple. For all Adams-Bashforth methods, $\rho(\zeta) = \zeta^{k-1}(\zeta - 1)$, whose roots are $0$ (with multiplicity $k-1$) and $1$. Since $0$ is inside the unit disk and $1$ is a simple root on the boundary, all Adams-Bashforth methods are zero-stable.

**Absolute stability**: the region of absolute stability for AB methods is generally smaller than for Runge-Kutta methods. For AB4, the step size constraint for the test equation $y' = \lambda y$ (with $\text{Re}(\lambda) < 0$) is $|h\lambda| < 0.3$, much more restrictive than RK4's $|h\lambda| < 2.8$. This makes Adams-Bashforth methods less suitable for problems with fast dynamics (large $|\lambda|$), though for non-stiff problems where step size is limited by accuracy, this is not a constraint.

## Efficiency Advantage

The key efficiency of Adams-Bashforth methods is that each step requires only one new function evaluation (computing $f_{n}$ — all previous $f$-values are already stored). A $k$-th order AB method thus achieves order $k$ with one function evaluation per step (plus the startup cost). In contrast, the $k$-th order Runge-Kutta method requires $k$ function evaluations per step (for $k \leq 4$). For long integrations of non-stiff problems with expensive $f$-evaluations, multistep methods are significantly more efficient.

## Worked Example: AB4 Step

Suppose we have (from previous computation or startup) $f_n = 2.0$, $f_{n-1} = 1.8$, $f_{n-2} = 1.6$, $f_{n-3} = 1.5$, $y_n = 5.0$, $h = 0.1$.

$$y_{n+1} = 5.0 + 0.1\left(\frac{55}{24}(2.0) - \frac{59}{24}(1.8) + \frac{37}{24}(1.6) - \frac{9}{24}(1.5)\right).$$

Computing: $\frac{55 \cdot 2.0 - 59 \cdot 1.8 + 37 \cdot 1.6 - 9 \cdot 1.5}{24} = \frac{110 - 106.2 + 59.2 - 13.5}{24} = \frac{49.5}{24} = 2.0625$.

$y_{n+1} = 5.0 + 0.1(2.0625) = 5.20625.$

The evaluation of $f_{n+1} = f(t_{n+1}, 5.20625)$ is then available for the next step, replacing the oldest stored value $f_{n-3}$.
