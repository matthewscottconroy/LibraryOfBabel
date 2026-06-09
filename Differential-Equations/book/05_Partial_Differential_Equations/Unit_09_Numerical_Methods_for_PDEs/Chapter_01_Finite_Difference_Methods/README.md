# Chapter 1: Finite Difference Methods

Finite difference methods approximate the derivatives in a PDE by difference quotients on a grid. The approach is direct and intuitive: replace $\partial u/\partial t$ by $(U^{n+1} - U^n)/\Delta t$, replace $\partial^2 u/\partial x^2$ by $(U_{j+1} - 2U_j + U_{j-1})/(\Delta x)^2$, and solve the resulting algebraic equations. The simplicity of finite differences makes them easy to implement and analyze, and for problems on rectangular domains with smooth solutions, they provide optimal convergence rates with minimal coding effort.

## Grid and Notation

**Space-time grid.** Discretize $[0,L]\times[0,T]$ with grid spacings $\Delta x = L/M$ and $\Delta t = T/N$. Grid points: $x_j = j\Delta x$ for $j = 0,1,\ldots,M$ and $t_n = n\Delta t$ for $n = 0,1,\ldots,N$. Denote the numerical approximation as $U_j^n \approx u(x_j, t_n)$.

**Standard difference operators:**
- **Forward difference in time:** $D_t^+ U_j^n = (U_j^{n+1}-U_j^n)/\Delta t \approx u_t + O(\Delta t)$.
- **Backward difference in time:** $D_t^- U_j^n = (U_j^n-U_j^{n-1})/\Delta t \approx u_t + O(\Delta t)$.
- **Centered second difference in space:** $D_x^2 U_j^n = (U_{j+1}^n-2U_j^n+U_{j-1}^n)/(\Delta x)^2 \approx u_{xx} + O((\Delta x)^2)$.

The centered approximation for $u_{xx}$ is second-order accurate (truncation error $O((\Delta x)^2)$) by Taylor's theorem: $u(x\pm\Delta x) = u \pm u_x\Delta x + \frac{1}{2}u_{xx}(\Delta x)^2 \pm \frac{1}{6}u_{xxx}(\Delta x)^3 + \frac{1}{24}u_{xxxx}(\Delta x)^4 + \cdots$

## Sections

**Section 1: Discretization and Grid** introduces grid generation, difference operators, truncation error analysis (Taylor expansion), and the concept of local truncation error (LTE).

**Section 2: Heat Equation — FTCS and BTCS** analyzes the forward Euler (FTCS, explicit) and backward Euler (BTCS, implicit) schemes for the heat equation $u_t = \kappa u_{xx}$. FTCS stability condition: $r = \kappa\Delta t/(\Delta x)^2 \leq 1/2$. BTCS unconditional stability. Both are first-order in time; second-order in space.

**Section 3: Crank-Nicolson Scheme** presents the average of FTCS and BTCS, which is second-order in both time and space. The Crank-Nicolson scheme requires solving a tridiagonal system at each time step (Thomas algorithm: $O(M)$), but achieves better accuracy per time step than either pure explicit or pure implicit.

**Section 4: Wave Equation Finite Differences** develops the standard leapfrog scheme for $u_{tt} = c^2u_{xx}$, with CFL condition $c\Delta t/\Delta x \leq 1$. The exact dispersion relation of the scheme vs. the continuous equation is analyzed: numerical dispersion (grid waves travel at different speeds than the PDE).

**Section 5: Laplace Equation — Iterative Methods** covers the five-point stencil for $\Delta u = f$ on a rectangle, the resulting sparse linear system, and iterative solvers: Jacobi, Gauss-Seidel, SOR. Convergence rates and the choice of optimal overrelaxation parameter $\omega^*$.

## Key Theme: Stability Determines Success

The central challenge in finite differences is stability. A consistent scheme may still produce a numerically useless solution if the stability condition is violated. The FTCS scheme for the heat equation, applied with $r > 1/2$, produces wildly oscillating solutions — even for perfectly smooth initial data — because the Fourier modes grow geometrically. This instability has no physical basis; it is a purely numerical artifact.

The remedy is either: (i) constrain $\Delta t$ to satisfy the stability condition (explicit scheme, restricted time step); or (ii) use an implicit scheme that is unconditionally stable, at the cost of solving a linear system at each step. For the heat equation on a fine grid ($M$ large), the stability condition $\Delta t \leq (\Delta x)^2/(2\kappa)$ requires $N = T/\Delta t \geq 2\kappa TM^2$ time steps — proportional to $M^2$. An implicit scheme allows $N \sim M$ time steps, reducing the total work from $O(M^3)$ to $O(M^2)$ — often a practical necessity for fine-resolution computations.
