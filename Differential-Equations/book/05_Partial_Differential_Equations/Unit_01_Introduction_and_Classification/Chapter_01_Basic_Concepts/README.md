# Chapter 1: Basic Concepts of Partial Differential Equations

The study of partial differential equations begins with the simplest possible question: what is a PDE? The answer requires care, because PDEs are far more varied than ODEs, and the vocabulary used to describe them — order, linearity, homogeneity, well-posedness — carries precise meanings that shape everything that follows. This chapter establishes that vocabulary and introduces the conceptual framework that organizes the entire subject.

## Defining a PDE

A partial differential equation is a relation among an unknown function $u$ of $n \geq 2$ independent variables $x_1, \ldots, x_n$ and the partial derivatives of $u$ up to some finite order. Formally, a PDE of order $k$ is an equation of the form

$$F\!\left(x_1,\ldots,x_n,\, u,\, \frac{\partial u}{\partial x_1},\ldots, \frac{\partial^k u}{\partial x_n^k}\right) = 0,$$

where $F$ is a given function and $u$ is the unknown. The domain of $u$ is typically an open subset $\Omega \subset \mathbb{R}^n$, sometimes with boundary $\partial\Omega$ where auxiliary conditions are imposed.

The three sections of this chapter develop the core ideas:

## Section 1: Definition and Examples

The opening section grounds the abstract definition in concrete examples. The heat equation $u_t = k \nabla^2 u$, the wave equation $u_{tt} = c^2 \nabla^2 u$, and Laplace's equation $\nabla^2 u = 0$ are introduced alongside first-order transport equations, Burgers' equation, and the Schrödinger equation. These examples illustrate the range of phenomena PDEs describe and motivate the classification theory developed in Chapter 2. The notion of a classical solution — a function with all required partial derivatives existing and satisfying the equation pointwise — is defined, alongside a preliminary discussion of when classical solutions might fail to exist and why weaker notions of solution are sometimes necessary.

## Section 2: Order and Linearity

A PDE is linear if it is linear in the unknown function and all its partial derivatives, with coefficients that may depend on the independent variables but not on $u$ itself. It is semilinear if the highest-order terms are linear. It is quasilinear if the highest-order terms are linear in the highest-order derivatives (though the coefficients may depend on lower-order derivatives of $u$). It is fully nonlinear if nonlinearity appears in the highest-order derivatives. The distinction matters enormously: linear PDEs admit superposition principles, satisfy explicit representation formulas, and have a complete spectral theory. Nonlinear PDEs require entirely different methods and can exhibit phenomena — shocks, solitons, blow-up — with no linear counterpart.

A linear PDE is homogeneous if every term involves $u$ or its derivatives; a forcing term $f(x,t)$ on the right-hand side makes the equation nonhomogeneous (or inhomogeneous). The superposition principle for linear homogeneous equations states that any linear combination of solutions is again a solution, which is the engine behind separation of variables, Fourier series, and eigenfunction expansions.

## Section 3: Well-Posedness

Hadamard's concept of well-posedness captures what it means for a PDE problem to be mathematically sound and physically meaningful. A problem is well-posed if:

1. A solution exists.
2. The solution is unique.
3. The solution depends continuously on the data (initial conditions, boundary conditions, source terms).

The third condition is the subtlest and most physically important. If small errors in measured data can produce arbitrarily large errors in the solution, then no amount of mathematical sophistication makes the problem computationally or physically tractable. Well-posedness is not a property of the PDE alone but of the PDE together with its auxiliary conditions: changing the boundary conditions can convert a well-posed problem into an ill-posed one.

Classic examples of ill-posed problems include the backward heat equation (solving $u_t = u_{xx}$ backward in time, which amplifies high-frequency errors exponentially) and the Cauchy problem for Laplace's equation (Hadamard's example: data $u(x,0) = 0$, $u_y(x,0) = n^{-1}\sin(nx)$ produces solution $u(x,y) = n^{-2}\sinh(ny)\sin(nx)$ which grows without bound as $n \to \infty$, even though the Cauchy data approaches zero).

This chapter thus sets the stage by insisting that mathematical well-posedness is not an optional refinement but a prerequisite for meaningful analysis.
