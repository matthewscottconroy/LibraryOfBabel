# Heat Equation: FTCS and BTCS Schemes

The heat equation $u_t = \kappa u_{xx}$ on $[0,L]\times[0,T]$ with Dirichlet boundary conditions $u(0,t) = u(L,t) = 0$ and initial condition $u(x,0) = f(x)$ is the prototype for numerical methods for parabolic PDEs. Two fundamental schemes — FTCS (forward in time, centered in space) and BTCS (backward in time, centered in space) — illustrate the dichotomy between explicit (simple but conditionally stable) and implicit (requires a linear solve but unconditionally stable) methods.

## FTCS (Forward Euler, Explicit)

**Scheme.** At interior grid points $j = 1,\ldots,M-1$ and time levels $n = 0,1,\ldots,N-1$:

$$\frac{U_j^{n+1}-U_j^n}{\Delta t} = \kappa\frac{U_{j+1}^n-2U_j^n+U_{j-1}^n}{(\Delta x)^2}. \tag{FTCS}$$

Solving for $U_j^{n+1}$:

$$U_j^{n+1} = rU_{j+1}^n + (1-2r)U_j^n + rU_{j-1}^n, \qquad r = \frac{\kappa\Delta t}{(\Delta x)^2}.$$

**Explicit:** $U^{n+1}$ is computed directly from $U^n$ with no linear system to solve.

**Local truncation error:** By Taylor expansion:

$$\tau_j^n = \frac{u_j^{n+1}-u_j^n}{\Delta t} - \kappa\frac{u_{j+1}^n-2u_j^n+u_{j-1}^n}{(\Delta x)^2} = \frac{\Delta t}{2}u_{tt} - \frac{\kappa(\Delta x)^2}{12}u_{xxxx} + O(\Delta t^2 + (\Delta x)^4).$$

Since $u_t = \kappa u_{xx}$: $u_{tt} = \kappa u_{xxt} = \kappa^2 u_{xxxx}$. So $\tau_j^n = \frac{\kappa\Delta t}{2}u_{xxxx} - \frac{\kappa(\Delta x)^2}{12}u_{xxxx} + \cdots = \kappa u_{xxxx}\left[\frac{\Delta t}{2} - \frac{(\Delta x)^2}{12}\right] + \cdots$

The LTE is $O(\Delta t + (\Delta x)^2)$: **first-order in time, second-order in space**.

**Stability condition (from von Neumann analysis, Chapter 2):**

$$r = \frac{\kappa\Delta t}{(\Delta x)^2} \leq \frac{1}{2}. \tag{Stability}$$

For $r > 1/2$: the Fourier mode $e^{ij\theta}$ is amplified by factor $|1-2r(1-\cos\theta)| > 1$ for $\theta \approx \pi$ (high-frequency modes), causing oscillations that grow without bound.

**Example.** Take $\kappa = 1$, $\Delta x = 0.1$ ($M=10$). Stability requires $\Delta t \leq 0.005$. With $\Delta t = 0.006 > 0.005$: $r = 0.6 > 0.5$. The coefficient of $U_j^n$ in FTCS is $1-2r = -0.2 < 0$, and oscillations develop within a few time steps, swamping the physical solution.

**Implementation.** In matrix form, FTCS is $\mathbf{U}^{n+1} = A\mathbf{U}^n$ where $A = I + r\,T$ with $T$ the tridiagonal matrix of $(-2,1,1)$. The matrix $A$ must have spectral radius $\leq 1$ for stability.

## BTCS (Backward Euler, Implicit)

**Scheme.** Use the implicit time level for the spatial difference:

$$\frac{U_j^{n+1}-U_j^n}{\Delta t} = \kappa\frac{U_{j+1}^{n+1}-2U_j^{n+1}+U_{j-1}^{n+1}}{(\Delta x)^2}. \tag{BTCS}$$

Rearranging:

$$-rU_{j-1}^{n+1} + (1+2r)U_j^{n+1} - rU_{j+1}^{n+1} = U_j^n. \tag{Linear system}$$

At each time step, this is a **tridiagonal linear system** $B\mathbf{U}^{n+1} = \mathbf{U}^n$ with $B = I - r\,T$ (diagonally dominant tridiagonal matrix). Solved efficiently by the **Thomas algorithm** in $O(M)$ operations.

**Unconditional stability.** The amplification factor for Fourier mode $e^{ij\theta}$ is:

$$\xi = \frac{1}{1+2r(1-\cos\theta)}.$$

Since $2r(1-\cos\theta) \geq 0$: $0 < \xi \leq 1$ for all $\theta$ and all $r > 0$. The BTCS scheme is **unconditionally stable** — any time step $\Delta t > 0$ gives a stable scheme.

**Local truncation error:** Same analysis as FTCS but with the spatial term evaluated at $t_{n+1}$:

$$\tau_j^n = -\frac{\Delta t}{2}u_{tt} - \frac{\kappa(\Delta x)^2}{12}u_{xxxx} + O(\Delta t^2 + (\Delta x)^4) = O(\Delta t + (\Delta x)^2).$$

Also **first-order in time, second-order in space**.

**Thomas algorithm.** For the tridiagonal system $a_jU_{j-1} + b_jU_j + c_jU_{j+1} = d_j$:
1. **Forward sweep:** $\tilde b_j = b_j - a_j c_{j-1}/\tilde b_{j-1}$, $\tilde d_j = d_j - a_j\tilde d_{j-1}/\tilde b_{j-1}$.
2. **Back substitution:** $U_{M-1} = \tilde d_{M-1}/\tilde b_{M-1}$; $U_j = (\tilde d_j - c_j U_{j+1})/\tilde b_j$.

For BTCS: $a_j = -r$, $b_j = 1+2r$, $c_j = -r$. The diagonal dominance ($b_j > |a_j| + |c_j|$) guarantees numerical stability of the Thomas algorithm.

## Comparison: FTCS vs. BTCS

| | FTCS | BTCS |
|---|---|---|
| Time differencing | Forward Euler ($O(\Delta t)$) | Backward Euler ($O(\Delta t)$) |
| Spatial differencing | Centered ($O((\Delta x)^2)$) | Centered ($O((\Delta x)^2)$) |
| Accuracy | $O(\Delta t + (\Delta x)^2)$ | $O(\Delta t + (\Delta x)^2)$ |
| Stability | $r \leq 1/2$ (conditional) | All $r > 0$ (unconditional) |
| Cost per step | $O(M)$ (explicit) | $O(M)$ (Thomas algorithm) |
| Maximum principle | Preserved for $r \leq 1/2$ | Preserved for all $r$ |

Both schemes are first-order in time. The key difference is stability: BTCS can use larger time steps, but requires solving a linear system.

## Maximum Principle for FTCS

**Discrete maximum principle.** For $0 < r \leq 1/2$: if $0 \leq U_j^n \leq M$ for all $j$ and the boundary data $U_0^n = U_M^n = 0$, then $0 \leq U_j^{n+1} \leq M$.

**Proof.** $U_j^{n+1} = rU_{j+1}^n + (1-2r)U_j^n + rU_{j-1}^n$ is a convex combination (all coefficients $r, 1-2r, r \geq 0$ when $r \leq 1/2$) of the neighboring values. So $U_j^{n+1}\in[\min_{j'}U_{j'}^n, \max_{j'}U_{j'}^n]$. $\square$

For $r > 1/2$: the coefficient $(1-2r) < 0$ and the convex combination property fails. Negative values appear even for non-negative initial data — the instability manifests as a violation of the maximum principle.

## Worked Example: Gaussian Initial Data

**Setup.** $\kappa = 1$, $L = 1$, $u(x,0) = \sin(\pi x)$, $M = 10$ ($\Delta x = 0.1$), $T = 0.1$.

**Exact solution:** $u(x,t) = e^{-\pi^2 t}\sin(\pi x)$.

**FTCS with $r = 0.5$ ($\Delta t = 0.005$, $N = 20$):** The amplification factor for mode $\theta = \pi\Delta x = \pi/10$ is $1 - 2\cdot 0.5(1-\cos\pi/10) \approx 1 - (1-0.975) = 0.975$. After $N = 20$ steps: $(0.975)^{20} \approx 0.60$. Exact: $e^{-\pi^2\cdot 0.1}\approx e^{-0.987}\approx 0.373$.

Wait — the mode $\theta = \pi j\Delta x$ for the first eigenmode is $\theta = \pi\Delta x$ only for wavenumber $k=1$. The amplification for FTCS is $\xi = 1-2r(1-\cos\theta)$ for each $\theta = k\pi\Delta x$. For the exact solution mode $\theta_1 = \pi\Delta x = \pi/10$: $\xi_1 = 1-2(0.5)(1-\cos\pi/10) = 1-(1-\cos\pi/10) = \cos^2(\pi/20)$ (using $1-\cos\theta = 2\sin^2(\theta/2)$: $\xi_1 = 1-4(0.5)\sin^2(\pi/20) = 1-2\sin^2(\pi/20) = \cos(\pi/10)$).

After $N = 20$ steps: $\xi_1^{20} = \cos^{20}(\pi/10) \approx (0.951)^{20}\approx 0.36$. Exact factor: $e^{-\pi^2\cdot 0.1}\approx 0.373$. Agreement to within a few percent — the scheme is accurate for the stable case.

**BTCS with $r = 2$ ($\Delta t = 0.02$, $N = 5$):** Amplification: $\xi_1 = 1/(1+2\cdot 2(1-\cos\pi/10)) = 1/(1+4\cdot 0.0955) = 1/1.382\approx 0.724$. After 5 steps: $0.724^5\approx 0.20$. Exact: $e^{-\pi^2\cdot 0.1}\approx 0.373$. Error about 46% — much larger than FTCS because the larger time step sacrifices accuracy (but gains stability). This illustrates that unconditional stability does not mean unconditional accuracy: $\Delta t$ must still be small enough for temporal accuracy.
