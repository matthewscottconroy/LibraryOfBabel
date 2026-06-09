# Unit 9: Numerical Methods for PDEs

The analytical methods developed in Units 1–8 — separation of variables, Fourier series, Green's functions, characteristics, variational formulations — provide exact or semi-explicit solutions for PDEs on simple domains with special structure. For the vast majority of problems arising in science and engineering — irregular geometries, nonlinear equations, complex initial data, time-dependent problems — numerical methods are essential. This unit develops the finite difference approach to discretizing PDEs, and the stability theory that determines when finite difference schemes produce accurate approximations.

## The Finite Difference Philosophy

Finite difference methods replace continuous derivatives with discrete difference quotients on a grid. The domain $\Omega\times[0,T]$ is covered by a grid of points $(x_j, t_n) = (j\Delta x, n\Delta t)$, and the PDE is approximated at each grid point by a difference equation. The solution $U_j^n \approx u(j\Delta x, n\Delta t)$ satisfies the difference equation exactly, and the error $u(j\Delta x, n\Delta t) - U_j^n$ is controlled by consistency (how well the difference approximates the PDE) and stability (whether errors grow).

The key theorem is the **Lax equivalence theorem**: for a consistent finite difference scheme, stability is equivalent to convergence. This means the two independent checks — consistency (algebraic, from Taylor expansion) and stability (spectral or energy-based) — together guarantee accuracy.

## Structure of This Unit

**Chapter 1: Finite Difference Methods** develops concrete schemes for the heat equation, wave equation, and Laplace equation:

- **Heat equation (1D):** The FTCS (Forward-Time Centered-Space) scheme $\frac{U_j^{n+1}-U_j^n}{\Delta t} = \kappa\frac{U_{j+1}^n-2U_j^n+U_{j-1}^n}{(\Delta x)^2}$ is explicit and simple, but conditionally stable ($\Delta t \leq (\Delta x)^2/(2\kappa)$). The BTCS (Backward-Time Centered-Space) scheme uses the implicit time level and is unconditionally stable, requiring a tridiagonal solve at each step.

- **Crank-Nicolson scheme:** Averages FTCS and BTCS: second-order accurate in both $\Delta x$ and $\Delta t$, unconditionally stable, and requires a tridiagonal solve. The "gold standard" for parabolic equations.

- **Wave equation:** The standard scheme $\frac{U_j^{n+1}-2U_j^n+U_j^{n-1}}{(\Delta t)^2} = c^2\frac{U_{j+1}^n-2U_j^n+U_{j-1}^n}{(\Delta x)^2}$ is explicit and conditionally stable with the CFL condition $c\Delta t/\Delta x \leq 1$.

- **Laplace equation (iterative methods):** The steady-state problem $\Delta u = f$ on a grid is a sparse linear system solved iteratively by Gauss-Seidel, SOR (successive overrelaxation), or multigrid.

**Chapter 2: Stability and Convergence** develops the theory:

- **Von Neumann stability analysis:** Expand the numerical solution in Fourier modes $\xi^n e^{ikj\Delta x}$; the scheme is stable if all Fourier modes satisfy $|\xi(k)| \leq 1$ (or $1+O(\Delta t)$). This gives explicit stability conditions.

- **CFL condition:** The Courant-Friedrichs-Lewy condition $c\Delta t/\Delta x \leq 1$ says that the numerical domain of dependence must contain the physical domain of dependence. For hyperbolic equations, violating CFL is catastrophic.

- **Consistency, stability, and convergence (Lax-Richtmyer theorem):** Consistency ($\Delta t, \Delta x \to 0$: truncation error $\to 0$) + stability ($\Rightarrow$ bounded growth of perturbations) $\Leftrightarrow$ convergence ($\|u - U_h\|_\infty \to 0$).

## Prerequisites

The numerical analysis in this unit requires:
- Undergraduate numerical analysis: Taylor series, floating-point arithmetic, direct solvers for linear systems.
- The PDE theory from Units 3, 4, and 5: heat equation (maximum principle, energy estimates), wave equation (domain of dependence, characteristics), Laplace equation (mean value property, Poisson formula).
- Linear algebra: tridiagonal systems (Thomas algorithm), matrix norms, spectral radius.
- Fourier series (Unit 3): the von Neumann analysis uses discrete Fourier modes.

## Why Stability Is the Hard Part

Consistency is relatively easy to verify: expand the exact solution $u(x_j,t_n)$ in a Taylor series and check that the difference equation approximates the PDE with truncation error $O(\Delta t^p + \Delta x^q)$ for some order $p,q$.

Stability is the deep requirement: it asks whether numerical errors (from rounding, from initial data perturbations) remain bounded as the computation proceeds. An unstable scheme amplifies errors exponentially — even with perfect initial data and exact arithmetic, floating-point rounding errors will cause the computed solution to diverge. Stability is therefore not optional; it is the fundamental constraint on practical numerical methods.

The choice between explicit (simple, but stability-limited) and implicit (requires a linear solve, but unconditionally stable) schemes is the central practical decision in computational PDE. For parabolic equations (heat equation), the stability constraint $\Delta t \lesssim (\Delta x)^2$ of explicit methods is prohibitively restrictive for fine spatial resolution — implicit methods are preferred. For hyperbolic equations (wave equation), explicit methods with the CFL condition are standard.
