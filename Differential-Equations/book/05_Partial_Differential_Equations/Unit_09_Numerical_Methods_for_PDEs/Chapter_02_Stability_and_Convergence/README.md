# Chapter 2: Stability and Convergence

The practical success of a finite difference scheme depends on three interlocking properties: **consistency** (the discrete equations approximate the PDE), **stability** (errors do not amplify), and **convergence** (the numerical solution approaches the exact solution as the grid is refined). The central theorem of numerical analysis for linear PDE problems — the Lax equivalence theorem — asserts that for a consistent scheme, stability is both necessary and sufficient for convergence. This chapter develops the precise definitions, the main analytical tool (von Neumann stability analysis), and the complete proof of the Lax theorem.

## The Three Properties

**Consistency** is a local property: the scheme approximates the differential operator to some order. If the local truncation error (LTE) is $\tau_h = O(h^p)$ as $h \to 0$, the scheme is consistent of order $p$.

**Stability** is a global property: errors present at any time level do not grow unboundedly as the computation proceeds. For linear schemes, stability is equivalent to a uniform bound on powers of the amplification matrix.

**Convergence** is the desired outcome: the numerical solution $U_j^n$ satisfies $|U_j^n - u(x_j, t_n)| \to 0$ as $\Delta x, \Delta t \to 0$.

The **Lax equivalence theorem** (Lax and Richtmyer, 1956) states: for a well-posed linear initial value problem and a consistent finite difference scheme, stability is necessary and sufficient for convergence.

## Von Neumann Analysis

The primary tool for determining stability is **von Neumann (Fourier) stability analysis**. The method decomposes the numerical error into Fourier modes $e^{ij\theta}$ (for spatial wavenumber $\theta = k\Delta x$) and tracks the amplitude of each mode over one time step via the **amplification factor** $\xi(\theta)$. For the scheme to be stable, all modes must remain bounded:

$$|\xi(\theta)| \leq 1 + C\Delta t \quad \text{for all } \theta,$$

(the factor $C\Delta t$ allowance accommodates source terms; for pure evolution equations, $|\xi| \leq 1$ is required).

Section 1 develops this analysis systematically: deriving $\xi(\theta)$ for FTCS, BTCS, Crank-Nicolson, and the leapfrog scheme; interpreting the stability conditions geometrically in the $(\Delta t, \Delta x)$ parameter space; and handling schemes with multiple time levels (where a characteristic polynomial in $\xi$ must satisfy the root condition).

## CFL Condition and Characteristic Analysis

For hyperbolic equations, stability is governed not by a mesh ratio but by a characteristic speed condition. The **CFL (Courant-Friedrichs-Lewy) condition** — $c\Delta t/\Delta x \leq 1$ for the wave equation — has a geometric interpretation: the numerical domain of dependence must contain the physical domain of dependence. Section 2 develops this idea in depth.

The CFL condition is both a stability condition (the leapfrog scheme is stable if and only if the CFL number $\lambda = c\Delta t/\Delta x \leq 1$) and a fundamental constraint from the theory of characteristics: no explicit scheme can be stable if the numerical domain of dependence is strictly smaller than the physical one. This is the content of the original CFL paper (1928), which established that the stability condition is a consequence of the mathematical structure of the PDE, not merely a numerical artifact.

Section 2 also discusses the **modified equation** — the PDE actually solved by the discrete scheme to leading order — which reveals how numerical dissipation and dispersion arise from the truncation error terms.

## Lax Equivalence Theorem

Section 3 presents the complete proof of the Lax equivalence theorem and its implications. The proof uses the following strategy: express the numerical error as a sum of contributions from the initial truncation error, the accumulated local truncation errors, and the initial data errors; bound each contribution using stability (the key step); and conclude convergence of order $\min(p, q)$ where $p$ is the LTE order in time and $q$ in space.

The theorem applies to **linear** problems. For nonlinear problems, the situation is more subtle: the scheme may be stable in a linearized sense but exhibit nonlinear instability. Section 3 surveys the extensions to nonlinear problems: invariant region principles, energy methods, and the role of entropy conditions for conservation laws.

## Section Overview

**Section 1: Von Neumann Stability Analysis** — systematic derivation of the amplification factor for all schemes of Chapter 1; stability regions in parameter space; root condition for multistep schemes; worked examples.

**Section 2: CFL Condition** — geometric interpretation via domains of dependence; modified equation analysis; numerical dissipation and dispersion; stability for first-order upwind and second-order Lax-Wendroff schemes.

**Section 3: Consistency, Stability, and Convergence** — complete proof of the Lax equivalence theorem; error estimation; convergence rates; extensions to nonlinear problems and stability for conservation laws.

## Practical Significance

Understanding stability and convergence is not merely theoretical. In practice:

- **Stability violations are catastrophic.** A scheme violating the stability condition produces errors that grow exponentially, overwhelming the physical solution within a few time steps. No amount of grid refinement helps — the scheme diverges faster as $h \to 0$.

- **Stability does not imply accuracy.** An unconditionally stable scheme (BTCS, Crank-Nicolson) can still be inaccurate if $\Delta t$ is too large. Stability bounds the growth of errors; accuracy bounds their initial size.

- **The CFL number is a physical invariant.** For wave propagation, the CFL number $\lambda = c\Delta t/\Delta x$ must be at most 1. Choosing $\lambda$ close to 1 (e.g., $\lambda = 0.9$) minimizes numerical dispersion; choosing $\lambda \ll 1$ wastes computational effort.

The material of this chapter forms the analytical foundation for all serious work with time-dependent PDEs on structured grids.
