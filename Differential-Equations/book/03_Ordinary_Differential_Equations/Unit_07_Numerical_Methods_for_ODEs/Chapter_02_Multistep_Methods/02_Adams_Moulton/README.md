# Adams-Moulton Methods

The Adams-Moulton methods are the implicit counterpart to the Adams-Bashforth family. Where Adams-Bashforth interpolates $f$ at past mesh points $t_{n-k+1}, \ldots, t_n$ and integrates over $[t_n, t_{n+1}]$, Adams-Moulton includes the future point $t_{n+1}$ in the interpolation. The resulting formula involves $f_{n+1} = f(t_{n+1}, y_{n+1})$ on the right-hand side, making $y_{n+1}$ appear implicitly.

The reward for this added complexity is significantly improved stability (larger regions of absolute stability) and better accuracy for the same number of steps: a $k$-step Adams-Moulton method achieves order $k+1$, one order higher than the $k$-step Adams-Bashforth method. This is why Adams-Moulton methods are typically paired with Adams-Bashforth predictors in predictor-corrector schemes.

## Derivation

The same integration approach as Adams-Bashforth: integrate the ODE over $[t_n, t_{n+1}]$ and approximate the integrand by an interpolating polynomial, but now through $k+1$ points including the endpoint $t_{n+1}$:

**Adams-Moulton 1-step (backward Euler):** Using only $t_{n+1}$:

$$y_{n+1} = y_n + hf_{n+1}.$$

**Adams-Moulton 2-step (trapezoidal rule / Crank-Nicolson):** Interpolating at $t_n$ and $t_{n+1}$ (linear):

$$y_{n+1} = y_n + \frac{h}{2}(f_n + f_{n+1}).$$

This is second-order (LTE $= -\frac{1}{12}h^3y'''$) and is commonly called the **Crank-Nicolson method** in the context of PDEs.

**Adams-Moulton 3-step (the standard AM4):** Interpolating at $t_{n-1}, t_n, t_{n+1}$ (quadratic):

$$y_{n+1} = y_n + h\left(\frac{5}{12}f_{n+1} + \frac{8}{12}f_n - \frac{1}{12}f_{n-1}\right).$$

This is third-order.

**Adams-Moulton 4-step (AM4, the standard pairing for AB4):** Interpolating at $t_{n-2}, t_{n-1}, t_n, t_{n+1}$ (cubic):

$$y_{n+1} = y_n + h\left(\frac{9}{24}f_{n+1} + \frac{19}{24}f_n - \frac{5}{24}f_{n-1} + \frac{1}{24}f_{n-2}\right).$$

This is fourth-order. The AM4 formula, while a 3-step method (using $f_{n-2}, f_{n-1}, f_n$ in addition to the implicit $f_{n+1}$), achieves the same fourth-order accuracy as AB4 but with better stability — a $k$-step Adams-Moulton achieves order $k+1$.

## Stability

The stability superiority of Adams-Moulton over Adams-Bashforth is substantial. For the test equation $y' = \lambda y$, the region of absolute stability of AM4 includes a much larger region of the left half-plane than AB4. The AM2 (trapezoidal rule) is A-stable — its stability region is the entire left half-plane — making it suitable for mildly stiff problems. However, for highly stiff problems even A-stable Adams-Moulton methods may require small step sizes, and BDF methods are preferred.

The characteristic polynomial analysis applies: the Adams-Moulton $k$-step method has $\rho(\zeta) = \zeta^k - \zeta^{k-1}$ (same as Adams-Bashforth), so zero-stability holds for all $k$. The $\sigma(\zeta)$ polynomial now has a nonzero leading coefficient (the $\beta_{-1}$ weight for $f_{n+1}$), making the method implicit.

## Solving the Implicit Equation

At each step, one must solve $y_{n+1} = y_n + h(\beta_{-1}f(t_{n+1}, y_{n+1}) + \text{known terms})$ for $y_{n+1}$. For linear $f$, this is straightforward. For nonlinear $f$, Newton's method is typically used:

$$y_{n+1}^{(k+1)} = y_{n+1}^{(k)} - \frac{y_{n+1}^{(k)} - y_n - h\beta_{-1}f(t_{n+1}, y_{n+1}^{(k)}) - R_n}{1 - h\beta_{-1}f_y(t_{n+1}, y_{n+1}^{(k)})},$$

where $R_n$ is the sum of known terms. Newton's method typically converges in 2–3 iterations when initialized with a good predictor. The function $f_y$ (the partial derivative with respect to $y$) must also be evaluated or estimated, adding overhead.

Alternatively, **functional iteration** (fixed-point iteration) can be used when $h|f_y| \ll 1$:

$$y_{n+1}^{(k+1)} = y_n + h\beta_{-1}f(t_{n+1}, y_{n+1}^{(k)}) + R_n.$$

This converges when $h|\beta_{-1}||f_y| < 1$, which may require small step sizes for stiff problems. For stiff equations, Newton's method (or a modified version with a frozen Jacobian) is preferred.

## The Adams-Moulton Corrector

In practice, Adams-Moulton methods are almost always used as **correctors** in a predictor-corrector pair (see next section). The predictor (an Adams-Bashforth method) provides an initial guess $y_{n+1}^{(0)}$, which is close enough to the solution that functional iteration converges in one step — the PECE (predict, evaluate, correct, evaluate) strategy. This avoids Newton's method entirely and makes the scheme computationally efficient.

One step of correction (the corrector applied once to the predictor's output) is often sufficient for good accuracy. Additional correction iterations improve accuracy modestly but add function evaluations; in practice, one corrector iteration is the standard approach.

## Dahlquist's Order Barrier

An important theoretical result constrains how good an implicit linear multistep method can be: **Dahlquist's second barrier theorem** states that a zero-stable, $k$-step implicit linear multistep method has order at most $k+1$ if $k$ is odd and $k+2$ if $k$ is even. The Adams-Moulton $k$-step method achieves order $k+1$, which meets this barrier for odd $k$.

More importantly, **A-stable linear multistep methods have order at most 2** (Dahlquist's first barrier). The AM2 (trapezoidal rule) achieves this maximum: it is the only explicit or implicit Adams method that is A-stable. For higher-order A-stable multistep methods, one must look outside the Adams family — the BDF methods sacrifice A-stability but maintain stability for a wider class of stiff problems.
