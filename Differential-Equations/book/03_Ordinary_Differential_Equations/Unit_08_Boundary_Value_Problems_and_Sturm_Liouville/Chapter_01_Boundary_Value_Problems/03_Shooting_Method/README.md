# The Shooting Method

The shooting method transforms a two-point boundary value problem into an initial value problem by treating the missing initial data as a free parameter and choosing it to satisfy the boundary condition at the far endpoint. The name comes from the analogy of adjusting the angle of a cannon to hit a target: one "shoots" the trajectory from the initial point and adjusts the angle (the missing initial data) until the trajectory reaches the target (the far boundary condition).

## Linear Shooting

For the linear BVP:

$$y'' + p(x)y' + q(x)y = f(x), \qquad y(a) = \alpha, \quad y(b) = \beta,$$

the unknown initial data is $s = y'(a)$. One solves two auxiliary IVPs:

**Problem 1 (particular solution):** $u'' + pu' + qu = f$, $u(a) = \alpha$, $u'(a) = 0$.

**Problem 2 (homogeneous solution):** $v'' + pv' + qv = 0$, $v(a) = 0$, $v'(a) = 1$.

The general solution is $y = u + sv$ for any constant $s$. Applying the boundary condition at $b$:

$$y(b) = u(b) + sv(b) = \beta \implies s = \frac{\beta - u(b)}{v(b)},$$

provided $v(b) \neq 0$. If $v(b) = 0$, the BVP is at an eigenvalue and either has no solution or infinitely many.

This method is exact for linear BVPs: the two auxiliary IVPs are solved numerically (e.g., with RK4), and the formula for $s$ is applied to get the unique solution, also computed numerically. The procedure requires two forward IVP solves and one algebraic operation.

## Nonlinear Shooting

For a nonlinear BVP $y'' = F(x, y, y')$, $y(a) = \alpha$, $y(b) = \beta$, define the **shooting function**:

$$\phi(s) = y(b; s) - \beta,$$

where $y(x; s)$ is the solution to the IVP $y'' = F(x,y,y')$, $y(a) = \alpha$, $y'(a) = s$. The shooting method seeks a root $s^*$ of $\phi(s) = 0$: for this choice of initial slope, the solution trajectory hits the target $y(b) = \beta$.

**Algorithm:**
1. Choose an initial guess $s_0$.
2. Solve the IVP to $x = b$ with $y'(a) = s_0$; compute $\phi(s_0) = y(b; s_0) - \beta$.
3. Update $s$ using a root-finding method (bisection, secant, or Newton's method).
4. Repeat until $|\phi(s_k)| < \text{tolerance}$.

**Newton's method for the shooting function:**

$$s_{k+1} = s_k - \frac{\phi(s_k)}{\phi'(s_k)}.$$

The derivative $\phi'(s) = \partial y(b;s)/\partial s$ is computed by solving the **variational equation**: differentiate the ODE with respect to $s$ to get $z = \partial y/\partial s$:

$$z'' = \frac{\partial F}{\partial y}z + \frac{\partial F}{\partial y'}z', \qquad z(a) = 0, \quad z'(a) = 1.$$

This is a linear ODE for $z$ that can be solved simultaneously with the original ODE, providing $\phi'(s) = z(b)$ exactly. Newton's method with the variational equation converges quadratically when started near the solution — typically 4–6 iterations are sufficient.

## Worked Example: Linear Shooting

Solve $y'' = y$, $y(0) = 1$, $y(1) = e^{-1} \approx 0.3679$.

The general solution is $y = c_1 e^x + c_2 e^{-x}$. Exact solution meeting the BCs: $y(0) = c_1 + c_2 = 1$, $y(1) = c_1 e + c_2/e = 1/e$. From the second: $c_1 e^2 + c_2 = 1$, so $c_1(e^2 - 1) = 0$: $c_1 = 0$, $c_2 = 1$. Exact: $y = e^{-x}$.

Linear shooting:

**IVP 1:** $u'' = u$, $u(0) = 1$, $u'(0) = 0$. Solution: $u = \cosh x$. $u(1) = \cosh 1 \approx 1.5431$.

**IVP 2:** $v'' = v$, $v(0) = 0$, $v'(0) = 1$. Solution: $v = \sinh x$. $v(1) = \sinh 1 \approx 1.1752$.

$s = (\beta - u(1))/v(1) = (1/e - \cosh 1)/\sinh 1$. Compute: $(0.3679 - 1.5431)/1.1752 = -1.1752/1.1752 = -1$.

Solution: $y = \cosh x - \sinh x = e^{-x}$. Confirmed.

## Stability and Sensitivity

The shooting method can be numerically unstable for certain BVPs. If the ODE has exponentially growing components, the shooting trajectory may diverge wildly as $x$ increases, making the residual $\phi(s)$ very sensitive to $s$ and the numerical computation unreliable.

**Example.** The BVP $y'' - 100y = 0$, $y(0) = 1$, $y(1) = e^{-10}$. The general solution is $y = c_1 e^{10x} + c_2 e^{-10x}$. The exact solution is $y = e^{-10x}$ (with $c_1 = 0$). For the shooting method, any initial slope $s$ near $y'(0) = -10$ that is slightly off will introduce a component of $e^{10x}$, which grows by a factor of $e^{10} \approx 22,026$ to $x = 1$. The shooting function $\phi(s)$ is extremely steep near the root: small errors in $s$ give huge errors in $y(1)$.

**Remedies:** Parallel shooting (also called multiple shooting) divides $[a,b]$ into subintervals, applies shooting on each, and matches values at interior nodes — reducing the interval length and hence the amplification. For very stiff BVPs, finite difference or collocation methods are more stable than simple shooting.

## Multiple Shooting

Multiple shooting divides $[a,b]$ into $M$ subintervals $[x_0, x_1], \ldots, [x_{M-1}, x_M]$ with $x_0 = a$, $x_M = b$. On each subinterval, one solves an IVP with unknown initial values $(y(x_m), y'(x_m))$ for $m = 1, \ldots, M-1$ at the interior nodes. The conditions are:

- The solution on $[x_m, x_{m+1}]$ starts at $(y_m, y_m')$ and reaches $x_{m+1}$.
- Continuity: the endpoint of the solution on $[x_{m-1}, x_m]$ must equal the start of the solution on $[x_m, x_{m+1}]$.
- Boundary conditions at $x_0 = a$ and $x_M = b$.

This gives a large system of nonlinear equations for the unknown interior values, solved by Newton's method. The Jacobian of this system is banded, making the linear algebra efficient. Multiple shooting is the standard method for difficult nonlinear BVPs and is implemented in production codes such as MATLAB's `bvp4c` and `bvp5c`.
