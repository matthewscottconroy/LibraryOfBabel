# Chapter 1: Boundary Value Problems

A boundary value problem (BVP) for a second-order ODE requires the solution to satisfy conditions at two (or more) distinct points, rather than specifying the full initial state at a single point. This structural change has profound consequences: existence and uniqueness no longer follow automatically from the Picard-Lindelöf theorem, eigenvalues appear naturally, and the solution structure is tied to the spectral properties of the associated differential operator.

The simplest class is the **two-point BVP** on an interval $[a,b]$: find $y(x)$ satisfying $y'' + p(x)y' + q(x)y = f(x)$ together with boundary conditions at $x = a$ and $x = b$. Dirichlet conditions specify $y(a)$ and $y(b)$; Neumann conditions specify $y'(a)$ and $y'(b)$; Robin (mixed) conditions involve linear combinations of $y$ and $y'$ at each endpoint.

For the homogeneous problem ($f = 0$) with homogeneous boundary conditions, the question of existence and uniqueness reduces to: does the homogeneous equation have a nontrivial solution satisfying the boundary conditions? If not, the nonhomogeneous problem has a unique solution; if so (an eigenvalue is present), the nonhomogeneous problem has either no solution or infinitely many solutions (the Fredholm alternative).

**Green's functions** provide the solution formula for nonhomogeneous BVPs with homogeneous boundary conditions: $y(x) = \int_a^b G(x,\xi)f(\xi)\,d\xi$, where the Green's function $G(x,\xi)$ is the response at $x$ to a unit point source at $\xi$. The Green's function encodes all information about the BVP and is the integral-operator analogue of the matrix inverse.

The **shooting method** is the primary numerical approach: guess the missing initial data (e.g., $y'(a) = s$), solve the resulting IVP, and adjust $s$ until the boundary condition at $x = b$ is satisfied. For linear equations, this can be done exactly with two complementary solutions; for nonlinear equations, root-finding (bisection, Newton's method) is applied to the residual function.

This chapter develops all three topics — the existence/uniqueness theory, Green's functions, and the shooting method — providing both the theoretical framework and practical computational tools for two-point BVPs.
