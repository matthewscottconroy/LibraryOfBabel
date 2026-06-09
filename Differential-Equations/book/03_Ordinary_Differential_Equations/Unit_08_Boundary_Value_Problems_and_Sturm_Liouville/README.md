# Unit 8: Boundary Value Problems and Sturm-Liouville Theory

The preceding units focus primarily on initial value problems, where the state of the system is specified at a single point in time and the ODE determines the future evolution. Boundary value problems (BVPs) are structurally different: conditions are imposed at two or more points (the boundary), and one seeks a solution that satisfies all boundary conditions simultaneously. This change from initial to boundary conditions produces a qualitative shift in the mathematical theory — existence and uniqueness are no longer automatic, eigenvalues play a central role, and the set of solutions has a rich algebraic structure tied to spectral theory.

BVPs arise naturally wherever physical constraints are imposed at spatial boundaries rather than temporal initial states. The deflection of a beam clamped at both ends, the temperature distribution in a rod with fixed endpoint temperatures, the wave functions of a quantum particle in a potential well, the modes of a vibrating string — all are BVPs. The differential equations in these problems are often second-order linear equations with variable coefficients, and their analysis requires the theory of Sturm-Liouville problems.

## Two-Point Boundary Value Problems

A two-point BVP for a second-order linear equation takes the form:

$$y'' + p(x)y' + q(x)y = f(x), \qquad a < x < b,$$

with boundary conditions at both endpoints — for example, $y(a) = \alpha$, $y(b) = \beta$ (Dirichlet conditions), or conditions involving $y'$ (Neumann or Robin conditions). Unlike IVPs, a BVP may have no solution, exactly one solution, or infinitely many solutions. The analysis depends critically on whether the corresponding homogeneous problem has nontrivial solutions.

Green's functions for BVPs provide the analogue of variation of parameters: they express the solution as an integral operator applied to the forcing, with the kernel (the Green's function) encoding the response at one point due to a unit source at another. The shooting method converts BVPs to IVPs by treating the unknown initial data as a parameter to be determined by requiring the boundary condition at the far endpoint to be satisfied.

## Sturm-Liouville Theory

The Sturm-Liouville (SL) theory provides the spectral framework for second-order linear BVPs. A regular Sturm-Liouville problem takes the form:

$$\frac{d}{dx}\left[p(x)y'\right] + [\lambda w(x) - q(x)]y = 0, \qquad a < x < b,$$

with appropriate boundary conditions at $x = a$ and $x = b$. Here $p > 0$, $w > 0$ (the weight function), $q \geq 0$, and $\lambda$ is the eigenvalue parameter. The SL operator $L = -\frac{1}{w}\left[\frac{d}{dx}(p\,d/dx) - q\right]$ is self-adjoint in the weighted $L^2$ space with inner product $\langle f,g\rangle = \int_a^b f(x)g(x)w(x)\,dx$.

The spectral theorem for SL operators is the ODE analogue of the spectral theorem for symmetric matrices: the eigenvalues $\lambda_1 < \lambda_2 < \lambda_3 < \cdots$ are real, simple, and accumulate only at $+\infty$; the corresponding eigenfunctions $\phi_n$ are orthogonal with respect to the weight $w$ and form a complete orthonormal basis for $L^2([a,b], w\,dx)$. Any square-integrable function can be expanded in a generalized Fourier series $\sum c_n \phi_n(x)$ that converges in $L^2$.

## Singular Sturm-Liouville Problems

When $p(a) = 0$ or $p(b) = 0$ (or both), or when the interval is infinite, the SL problem is **singular**. Singular SL problems require modified boundary conditions (limit-point and limit-circle conditions in Weyl's classification) and may have continuous spectrum in addition to discrete eigenvalues. Many of the most important special functions — Legendre polynomials, Bessel functions, Hermite and Laguerre polynomials — arise as eigenfunctions of singular SL problems, which explains their orthogonality and completeness from a unified theoretical perspective.

## Organization

Chapter 1 develops the theory of two-point BVPs directly: existence and uniqueness conditions, Green's functions, and the shooting method. Chapter 2 presents regular Sturm-Liouville theory: the self-adjoint framework, eigenvalue properties, orthogonality, and eigenfunction expansions. Chapter 3 treats singular SL problems, with Legendre and Bessel equations as principal examples. Together these chapters provide the theoretical underpinning for the separation of variables method in partial differential equations and the spectral theory of differential operators.
