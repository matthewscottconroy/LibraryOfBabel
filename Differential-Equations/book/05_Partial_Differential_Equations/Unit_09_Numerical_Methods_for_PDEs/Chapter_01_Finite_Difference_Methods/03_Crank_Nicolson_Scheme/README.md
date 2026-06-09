# The Crank-Nicolson Scheme

The Crank-Nicolson scheme (Crank and Nicolson, 1947) achieves second-order accuracy in both time and space while remaining unconditionally stable. It is the weighted average of FTCS (explicit, $\theta=0$) and BTCS (implicit, $\theta=1$) with $\theta = 1/2$:

$$\frac{U_j^{n+1}-U_j^n}{\Delta t} = \frac{\kappa}{2}\frac{U_{j+1}^n-2U_j^n+U_{j-1}^n}{(\Delta x)^2} + \frac{\kappa}{2}\frac{U_{j+1}^{n+1}-2U_j^{n+1}+U_{j-1}^{n+1}}{(\Delta x)^2}. \tag{CN}$$

The spatial operator is evaluated at the midpoint $t_{n+1/2} = (t_n+t_{n+1})/2$ — the average of the two time levels. This trapezoidal averaging is the key to second-order temporal accuracy.

## Derivation and Properties

**Rearranging (CN):** Multiply by $\Delta t$ and move unknowns at $n+1$ to the left:

$$-\frac{r}{2}U_{j-1}^{n+1} + (1+r)U_j^{n+1} - \frac{r}{2}U_{j+1}^{n+1} = \frac{r}{2}U_{j-1}^n + (1-r)U_j^n + \frac{r}{2}U_{j+1}^n, \tag{CN system}$$

where $r = \kappa\Delta t/(\Delta x)^2$. This is a **tridiagonal linear system** at each time step:

$$B\mathbf{U}^{n+1} = C\mathbf{U}^n,$$

where $B = \frac{1}{2}(I - r\,T) + \frac{1}{2}I = I - \frac{r}{2}T$ and $C = I + \frac{r}{2}T$ (with $T$ the tridiagonal second-difference matrix).

**Local truncation error.** Expand all quantities at $t_n$ using Taylor series:

$$\frac{U_j^{n+1}-U_j^n}{\Delta t} = u_t\big|_{t_{n+1/2}} + \frac{(\Delta t)^2}{24}u_{ttt} + O(\Delta t^4),$$

$$\frac{1}{2}\left[\frac{U_{j+1}^n-2U_j^n+U_{j-1}^n}{(\Delta x)^2}+\frac{U_{j+1}^{n+1}-2U_j^{n+1}+U_{j-1}^{n+1}}{(\Delta x)^2}\right] = u_{xx}\big|_{t_{n+1/2}} + \frac{(\Delta x)^2}{12}u_{xxxx} + O(\Delta t^2(\Delta x)^2).$$

The difference is $u_t - \kappa u_{xx} = 0$ at $t_{n+1/2}$, and the remaining terms give:

$$\tau_j^n = O(\Delta t^2) + O((\Delta x)^2).$$

**Crank-Nicolson is second-order in both time and space.**

## Stability

The amplification factor for the Fourier mode $e^{ij\theta}$ (von Neumann analysis):

$$\xi = \frac{1 + 2\cdot\frac{r}{2}(\cos\theta-1)}{1 - 2\cdot\frac{r}{2}(\cos\theta-1)} = \frac{1-r(1-\cos\theta)}{1+r(1-\cos\theta)}.$$

Since $r(1-\cos\theta) \geq 0$: $|\xi| = \left|\frac{1-r(1-\cos\theta)}{1+r(1-\cos\theta)}\right| \leq 1$ for all $r > 0$ and all $\theta$.

**Crank-Nicolson is unconditionally stable.** The amplification factor is always $\leq 1$ in absolute value, regardless of the mesh ratio $r$.

**Comparison of amplification factors:**

| Scheme | $\xi(\theta)$ | Stable? |
|---|---|---|
| FTCS | $1 - 2r(1-\cos\theta)$ | $r\leq 1/2$ |
| BTCS | $\frac{1}{1+2r(1-\cos\theta)}$ | All $r > 0$ |
| Crank-Nicolson | $\frac{1-r(1-\cos\theta)}{1+r(1-\cos\theta)}$ | All $r > 0$ |
| Exact | $e^{-\kappa k^2\Delta t}$ ($k=\theta/\Delta x$) | N/A |

For large $r$ (large time step): FTCS is unstable; BTCS and CN are stable. CN has $|\xi| = |1-r\cdot 2\sin^2(\theta/2)|/|1+r\cdot 2\sin^2(\theta/2)|\to 1$ as $r\to\infty$ — high-frequency modes are not damped, only "frozen" ($|\xi|\to 1^-$). BTCS damps them: $\xi_\text{BTCS}\to 0$ as $r\to\infty$. So BTCS is more dissipative than CN for large time steps.

## θ-Scheme

The general one-parameter family:

$$\frac{U_j^{n+1}-U_j^n}{\Delta t} = \kappa\left[\theta\frac{U_{j+1}^{n+1}-2U_j^{n+1}+U_{j-1}^{n+1}}{(\Delta x)^2} + (1-\theta)\frac{U_{j+1}^n-2U_j^n+U_{j-1}^n}{(\Delta x)^2}\right].$$

- $\theta = 0$: FTCS (explicit, 1st-order time, stable iff $r\leq 1/2$).
- $\theta = 1$: BTCS (implicit, 1st-order time, unconditionally stable).
- $\theta = 1/2$: Crank-Nicolson (implicit, 2nd-order time, unconditionally stable).

For $\theta \geq 1/2$: the scheme is unconditionally stable (amplification factor $|\xi| \leq 1$ for all $r$). The accuracy order in time is $O(\Delta t)$ for $\theta \neq 1/2$ and $O(\Delta t^2)$ for $\theta = 1/2$.

## Worked Example: Comparison of Schemes

**Problem.** $u_t = u_{xx}$ on $[0,1]$ with $u(x,0) = \sin(\pi x)$, $u(0,t) = u(1,t) = 0$.

**Exact solution:** $u(x,t) = e^{-\pi^2 t}\sin(\pi x)$.

**Numerical parameters:** $\Delta x = 0.1$, $\Delta t = 0.02$ (so $r = \kappa\Delta t/(\Delta x)^2 = 0.02/0.01 = 2$), $T = 0.1$.

The parameter $r = 2 > 1/2$: FTCS is unstable for this choice.

**BTCS:** At $T = 0.1$ ($N = 5$ steps), amplification for the $\sin(\pi x)$ mode ($\theta = \pi\Delta x = \pi/10$):
$\xi_\text{BTCS} = 1/(1+2r(1-\cos\theta)) = 1/(1+4(1-\cos\pi/10)) = 1/(1+4\cdot 0.0955) = 1/1.382\approx 0.724$.
After 5 steps: $\xi^5\approx 0.20$.
Exact: $e^{-\pi^2\cdot 0.1}\approx 0.373$.
Error at $x=0.5$: $|0.373-0.20| = 0.173$. (Large error due to large $\Delta t$.)

**Crank-Nicolson:** $\xi_\text{CN} = (1-r(1-\cos\theta))/(1+r(1-\cos\theta)) = (1-0.382)/(1+0.382) = 0.618/1.382\approx 0.447$.
After 5 steps: $(0.447)^5\approx 0.018$... Wait: for the dominant $\sin(\pi x)$ mode, $\theta = \pi\Delta x$ is the spatial wavenumber for the $k=1$ eigenmode. Actually $(1-\cos\theta) = 1-\cos(\pi/10)\approx 0.0955$, so $r(1-\cos\theta) = 2\cdot 0.0955 = 0.191$.

$\xi_\text{CN} = (1-0.191)/(1+0.191) = 0.809/1.191 = 0.679$. After 5 steps: $0.679^5\approx 0.143$.

Exact: $e^{-\pi^2\cdot 0.1}\approx 0.373$.

The CN error is still significant because $\Delta t = 0.02$ is large (for reference, with $\Delta t = 0.005$: $r=0.5$, $\xi_\text{CN}\approx 0.905$, $\xi^{20}\approx 0.138$ ... matching exact $0.373$ much better). With a smaller time step, CN is clearly superior to BTCS in accuracy.

## Crank-Nicolson for 2D Heat Equation

For $u_t = \kappa(u_{xx}+u_{yy})$ on a rectangle, the direct Crank-Nicolson generalization requires solving a sparse linear system (not just tridiagonal) at each step. The **ADI (Alternating Direction Implicit)** method of Peaceman and Rachford (1955) splits the 2D implicit step into two 1D tridiagonal solves:

**Step 1 (implicit in $x$, explicit in $y$):**
$$\frac{U^{n+1/2}-U^n}{\Delta t/2} = \kappa\left[\delta_x^2 U^{n+1/2} + \delta_y^2 U^n\right].$$

**Step 2 (explicit in $x$, implicit in $y$):**
$$\frac{U^{n+1}-U^{n+1/2}}{\Delta t/2} = \kappa\left[\delta_x^2 U^{n+1/2} + \delta_y^2 U^{n+1}\right].$$

Each step requires only a tridiagonal solve (in $x$ for step 1, in $y$ for step 2). The combined scheme is second-order accurate and unconditionally stable in 2D (though not all ADI variants extend to higher dimensions stably).
