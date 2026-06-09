# Von Neumann Stability Analysis

Every finite difference scheme amplifies or damps the Fourier modes in the numerical solution. A scheme is **stable** if no mode grows without bound over the course of the computation. Von Neumann stability analysis makes this precise: decompose the error into spatial Fourier modes $e^{ij\theta}$, track the complex amplitude $\xi(\theta)$ over one time step, and require $|\xi(\theta)| \leq 1$ for all $\theta \in [-\pi, \pi]$. This analysis is the universal tool for determining stability conditions for linear, constant-coefficient schemes on uniform grids.

## Setup and Notation

Consider a linear finite difference scheme for a PDE on a uniform grid $x_j = j\Delta x$, $t_n = n\Delta t$. The scheme relates $U_j^{n+1}$ (possibly also $U_j^{n-1}$, $U_j^n$, $U_{j\pm 1}^n$, etc.) via linear combinations. Write the scheme symbolically as $\mathcal{L}_h U = 0$.

**Ansatz.** Substitute a single Fourier mode $U_j^n = \xi^n e^{ij\theta}$ into the scheme, where $\xi \in \mathbb{C}$ is to be determined (the **amplification factor** or **growth factor**) and $\theta = k\Delta x \in [-\pi,\pi]$ is the dimensionless wavenumber.

**Stability condition.** The scheme is stable if and only if there exists a constant $C$ independent of $h = \Delta x$, $\Delta t$, $\theta$ such that:

$$|\xi(\theta)| \leq 1 + C\Delta t \quad \text{for all } \theta. \tag{stability}$$

For pure evolution equations with no source terms, the condition simplifies to $|\xi(\theta)| \leq 1$ for all $\theta$. The constant $C$ allowance handles schemes for PDEs with lower-order terms (e.g., damping $u_t + \alpha u = u_{xx}$) where a fixed amount of growth per step is permissible.

**Why Fourier modes?** On a periodic grid ($j \pmod{M}$), the discrete Fourier modes $\{e^{ij\theta_k}\}_{k=0}^{M-1}$ with $\theta_k = 2\pi k/M$ form an orthonormal basis. Any error $e_j^n$ decomposes as a finite Fourier series, and by linearity of the scheme, each mode evolves independently. The global error bound $\|e^n\|_2 \leq \max_\theta|\xi(\theta)|^n\|e^0\|_2$ follows immediately.

## Analysis of FTCS for the Heat Equation

The FTCS scheme for $u_t = \kappa u_{xx}$:

$$U_j^{n+1} = U_j^n + r(U_{j+1}^n - 2U_j^n + U_{j-1}^n), \qquad r = \frac{\kappa\Delta t}{(\Delta x)^2}.$$

Substitute $U_j^n = \xi^n e^{ij\theta}$:

$$\xi e^{ij\theta} = e^{ij\theta} + r\xi^{n-1}\cdot e^{ij\theta}(e^{i\theta} - 2 + e^{-i\theta}).$$

Wait — since $U_j^n = \xi^n e^{ij\theta}$, we have $U_j^{n+1} = \xi^{n+1}e^{ij\theta}$, $U_{j\pm 1}^n = \xi^n e^{i(j\pm 1)\theta}$. Substituting and dividing by $\xi^n e^{ij\theta}$:

$$\xi = 1 + r(e^{i\theta} - 2 + e^{-i\theta}) = 1 + r(2\cos\theta - 2) = 1 - 2r(1 - \cos\theta). \tag{FTCS $\xi$}$$

Using $1 - \cos\theta = 2\sin^2(\theta/2)$:

$$\xi = 1 - 4r\sin^2(\theta/2). \tag{FTCS amplification}$$

**Stability condition.** Since $\xi$ is real, $|\xi| \leq 1$ requires:

$$-1 \leq 1 - 4r\sin^2(\theta/2) \leq 1.$$

The right inequality holds always. The left requires $4r\sin^2(\theta/2) \leq 2$, i.e., $r\sin^2(\theta/2) \leq 1/2$. Since $\max_\theta \sin^2(\theta/2) = 1$ (at $\theta = \pi$, the Nyquist mode $(-1)^j$):

$$\boxed{r \leq \frac{1}{2}.} \tag{FTCS stability}$$

**Geometric picture.** In the $(r, \theta)$ plane: $\xi(\theta, r) = 1 - 4r\sin^2(\theta/2)$ ranges from $\xi = 1$ (at $\theta = 0$, the constant mode) to $\xi = 1 - 4r$ (at $\theta = \pi$). Stability requires $1 - 4r \geq -1$, i.e., $r \leq 1/2$. At $r = 1/2$: the Nyquist mode is multiplied by $-1$ each step (sign alternation, marginal stability). At $r > 1/2$: the Nyquist mode grows as $|1-4r|^n > 1$ — exponential instability.

## Analysis of BTCS

The BTCS scheme substitutes $U_j^{n+1}$ into the spatial operator. Substituting $U_j^n = \xi^n e^{ij\theta}$, $U_j^{n+1} = \xi^{n+1}e^{ij\theta}$, etc., and dividing by $\xi^n e^{ij\theta}$:

$$\xi - 1 = r\xi(e^{i\theta} - 2 + e^{-i\theta}) = -2r\xi(1-\cos\theta).$$

Solving: $\xi(1 + 2r(1-\cos\theta)) = 1$, giving:

$$\xi = \frac{1}{1 + 2r(1-\cos\theta)} = \frac{1}{1 + 4r\sin^2(\theta/2)}. \tag{BTCS amplification}$$

Since $4r\sin^2(\theta/2) \geq 0$: $0 < \xi \leq 1$ for all $\theta$ and all $r > 0$. **BTCS is unconditionally stable.**

## Analysis of Crank-Nicolson

CN averages the spatial operator at $t_n$ and $t_{n+1}$. Substituting:

$$\xi - 1 = \frac{r}{2}(e^{i\theta}-2+e^{-i\theta}) + \frac{r\xi}{2}(e^{i\theta}-2+e^{-i\theta}) = -2r(1-\cos\theta)\cdot\frac{1+\xi}{2}.$$

Solving: $(\xi - 1) = -r(1-\cos\theta)(1+\xi)$, giving:

$$\xi\left(1 + r(1-\cos\theta)\right) = 1 - r(1-\cos\theta),$$

$$\xi = \frac{1 - r(1-\cos\theta)}{1 + r(1-\cos\theta)}. \tag{CN amplification}$$

Since $r(1-\cos\theta) \geq 0$: both numerator and denominator are real. If $r(1-\cos\theta) \leq 1$: $\xi \in [0,1]$. If $r(1-\cos\theta) > 1$: $\xi \in [-1, 0)$. In either case, $|\xi| \leq 1$ for all $r > 0$ and $\theta$. **CN is unconditionally stable.**

**Large $r$ behavior.** As $r \to \infty$: $\xi \to -1$ (for $\theta = \pi$) — the Nyquist mode oscillates in sign but does not decay. CN does not damp high-frequency modes for large $\Delta t$. This contrasts with BTCS: $\xi_{BTCS} \to 0$ for large $r$ — BTCS is more dissipative. For smooth solutions, CN's lack of dissipation is irrelevant; for problems with high-frequency components (e.g., initial data with discontinuities), CN can exhibit persistent oscillations near discontinuities (Gibbs-like phenomenon).

## Leapfrog for the Wave Equation

The leapfrog scheme $U_j^{n+1} - 2U_j^n + U_j^{n-1} = \lambda^2(U_{j+1}^n - 2U_j^n + U_{j-1}^n)$ involves three time levels. The Fourier ansatz $U_j^n = \xi^n e^{ij\theta}$ gives a quadratic in $\xi$:

$$\xi^2 - 2(1-\alpha)\xi + 1 = 0, \qquad \alpha = 2\lambda^2\sin^2(\theta/2), \quad \lambda = c\Delta t/\Delta x.$$

The roots are $\xi_\pm = (1-\alpha) \pm \sqrt{(1-\alpha)^2-1}$.

**Case $|1-\alpha| \leq 1$ (i.e., $0 \leq \alpha \leq 2$):** Discriminant $\leq 0$, so $\xi_\pm = (1-\alpha) \pm i\sqrt{1-(1-\alpha)^2}$. Then $|\xi_\pm|^2 = (1-\alpha)^2 + 1-(1-\alpha)^2 = 1$ — both roots lie on the unit circle (marginal stability, no growth).

**Case $\alpha > 2$:** Discriminant $> 0$, roots real with $\xi_+\xi_- = 1$ (product of roots equals constant term). Since $\xi_+ + \xi_- = 2(1-\alpha) < -2$: the roots satisfy $\xi_+ < -1$ and $|\xi_-| < 1$. The larger root $|\xi_+| > 1$ — unstable.

**Condition $\alpha \leq 2$ for all $\theta$.** Since $\sin^2(\theta/2) \leq 1$: $\alpha \leq 2\lambda^2$. Requiring $2\lambda^2 \leq 2$ gives $\lambda \leq 1$: the **CFL condition**.

**Root condition for multistep schemes.** When the amplification factor satisfies a polynomial $P(\xi) = 0$ of degree $>1$ (multistep in time), the stability condition is the **root condition**: all roots of $P(\xi) = 0$ must satisfy $|\xi| \leq 1$, with any root on $|\xi| = 1$ being simple (no repeated roots on the unit circle). This is the analog of $|\xi| \leq 1$ for one-step schemes.

## Amplification Factor Table and Stability Regions

| Scheme | Amplification factor $\xi(\theta)$ | Stable when |
|---|---|---|
| FTCS | $1 - 4r\sin^2(\theta/2)$ | $r \leq 1/2$ |
| BTCS | $1/(1+4r\sin^2(\theta/2))$ | All $r > 0$ |
| Crank-Nicolson | $(1-2r\sin^2(\theta/2))/(1+2r\sin^2(\theta/2))$ (using $r(1-\cos\theta) = 2r\sin^2$) | All $r > 0$ |
| Leapfrog (wave) | $|\xi_\pm| = 1$ | $\lambda \leq 1$ |
| Upwind (advection) | $1 - \lambda(1-e^{-i\theta})$ | $0 < \lambda \leq 1$ |

For the **first-order upwind scheme** $U_j^{n+1} = U_j^n - \lambda(U_j^n - U_{j-1}^n)$ for $u_t + cu_x = 0$, $c > 0$:

$$\xi = 1 - \lambda(1 - e^{-i\theta}) = (1-\lambda) + \lambda e^{-i\theta}.$$

$|\xi|^2 = (1-\lambda)^2 + 2\lambda(1-\lambda)\cos\theta + \lambda^2 = 1 - 2\lambda(1-\lambda)(1-\cos\theta) \leq 1$ iff $\lambda(1-\lambda) \geq 0$ iff $0 \leq \lambda \leq 1$.

**Interpretation.** Each scheme's stability region in the parameter space (say $r = \kappa\Delta t/(\Delta x)^2$ for parabolic schemes, or $\lambda = c\Delta t/\Delta x$ for hyperbolic) corresponds to the values for which the amplification factor satisfies $|\xi(\theta)| \leq 1 + C\Delta t$ for all $\theta$. Points outside this region lead to exponential growth — the scheme is unstable regardless of how small the initial error.

## Limitations of Von Neumann Analysis

Von Neumann analysis is exact for linear, constant-coefficient schemes on periodic (or infinite) domains. Its validity rests on the completeness of the Fourier basis. Several caveats apply:

1. **Variable coefficients.** For schemes with variable coefficients $a(x)$, the local Fourier analysis (freezing $a(x)$ at each point) gives a necessary but not sufficient condition for stability. The analysis gives a pointwise stability condition that must hold uniformly over all $x$.

2. **Boundary conditions.** Von Neumann analysis ignores boundary effects. For Dirichlet boundaries, the discrete sine transform (rather than the full Fourier transform) is the appropriate basis. The resulting analysis gives the same stability conditions in most cases, but boundary treatments can introduce additional instability modes.

3. **Nonlinear schemes.** For nonlinear schemes, linearization (computing the Jacobian and applying von Neumann analysis to the linearized scheme) is a necessary but not sufficient stability condition.

4. **Multistep schemes.** Schemes using more than two time levels require the root condition for all roots of the characteristic polynomial $P(\xi)$ — not just the dominant root.

Despite these limitations, von Neumann analysis is the indispensable first tool for stability analysis. It provides sharp stability conditions for the prototypical schemes and correctly identifies the mechanism of instability (which Fourier mode goes unstable first and why).
