# Well-Posedness of PDE Problems

The concept of well-posedness, introduced by Jacques Hadamard around 1902, is one of the deepest organizing ideas in the theory of PDEs. It asks not merely whether an equation has a solution, but whether the problem — the equation together with its auxiliary conditions — is mathematically coherent. A problem that has a solution but not a unique one, or whose solution is catastrophically sensitive to small perturbations in data, is mathematically pathological and physically useless.

## Hadamard's Three Conditions

A PDE problem is **well-posed** (in the sense of Hadamard) if:

1. **Existence.** For every choice of data in the specified class, a solution exists.
2. **Uniqueness.** The solution is unique: two solutions satisfying the same data must be identical.
3. **Continuous dependence.** The solution depends continuously on the data. If the data are perturbed by a small amount (in an appropriate norm), the solution changes by a correspondingly small amount.

If any of these three conditions fails, the problem is **ill-posed**. Well-posedness is always relative to a choice of function spaces — the class of admissible data and the space in which solutions are sought. A problem may be well-posed in one function space and ill-posed in another.

## Why Each Condition Matters

**Existence** is the prerequisite for everything else. Without it, the equation has no solution and any attempt to compute or approximate one is doomed. Proving existence for nonlinear PDEs is often the deepest and hardest part of the theory.

**Uniqueness** ensures that the equation has a definite prediction. In physics, a deterministic system described by a PDE should have exactly one future state given its present state. Uniqueness also makes the comparison of analytical and numerical solutions meaningful.

**Continuous dependence** is the most practically urgent condition. Physical data is always measured with error; initial conditions are approximations. If small errors in data lead to large errors in solutions, then the mathematical model — however perfectly specified — cannot make useful predictions. Continuous dependence is also essential for numerical computation: a stable numerical scheme is one that controls the growth of rounding errors, which is essentially continuous dependence at the discrete level.

## The Backward Heat Equation: A Canonical Ill-Posed Problem

The heat equation $u_t = k u_{xx}$ is well-posed forward in time with initial data $u(x,0) = f(x)$. Now consider running it backward: given $u(x,T) = g(x)$, find $u(x,t)$ for $t < T$.

This backward problem fails condition 3 spectacularly. To see why, consider the Fourier modes. A forward heat solution with frequency $n$ behaves as $e^{-kn^2 t}$, decaying exponentially as $t$ increases. Running backward in time, this same mode grows as $e^{kn^2(T-t)}$ — exponentially in $n$. A perturbation of the terminal data of size $\epsilon\sin(nx)$ produces an error in the backward solution of size $\epsilon e^{kn^2 T}$, which grows without bound as $n \to \infty$. No matter how smooth the perturbation is, the effect on the backward solution is catastrophically large.

This is not a deficiency of the method — it is a genuine mathematical fact about the problem. The backward heat equation is ill-posed, and any numerical scheme that attempts to solve it directly will be overwhelmed by exponentially growing errors.

## Hadamard's Example for Laplace's Equation

Hadamard gave a famous example showing that the Cauchy problem for Laplace's equation is ill-posed. Consider

$$u_{xx} + u_{yy} = 0, \qquad u(x,0) = 0, \qquad u_y(x,0) = \phi(x).$$

For the data $\phi_n(x) = n^{-1}\sin(nx)$ (which is small in $C^0$ norm), the exact solution is

$$u_n(x,y) = \frac{\sinh(ny)}{n^2}\sin(nx).$$

As $n \to \infty$, $\phi_n \to 0$ uniformly, but for any fixed $y \neq 0$,

$$u_n(x,y) \sim \frac{e^{ny}}{2n^2}\sin(nx) \to \infty.$$

The Cauchy data is becoming small, but the solution is growing without bound. This is a failure of condition 3.

The correct formulation for Laplace's equation is a boundary value problem — data specified on all of $\partial\Omega$ — not a Cauchy problem. The lesson is that the type of auxiliary data must match the type of equation.

## Well-Posedness for the Three Canonical Equations

The following table summarizes the appropriate auxiliary conditions for each canonical type:

| Equation | Type | Appropriate Problem |
|----------|------|---------------------|
| Heat equation $u_t = k\Delta u$ | Parabolic | Initial value problem (specify $u$ at $t=0$); boundary conditions on spatial domain |
| Wave equation $u_{tt} = c^2 \Delta u$ | Hyperbolic | Initial value problem (specify $u$ and $u_t$ at $t=0$); boundary conditions on spatial domain |
| Laplace's equation $\Delta u = 0$ | Elliptic | Boundary value problem (specify $u$ or $\partial u/\partial\nu$ on all of $\partial\Omega$) |

These pairings are not arbitrary conventions — they follow from the mathematical structure of each equation and are verified by well-posedness theorems. Swapping them (e.g., giving Cauchy data for Laplace) leads to ill-posedness.

## Energy Methods and Uniqueness

One of the most elegant ways to prove uniqueness is the energy method, which is applicable to both the heat equation and the wave equation.

For the heat equation on a bounded domain $\Omega$ with Dirichlet boundary conditions, suppose $u_1$ and $u_2$ are two solutions with the same initial and boundary data. Then $w = u_1 - u_2$ satisfies $w_t = k\Delta w$ with $w = 0$ on $\partial\Omega$ and $w(\mathbf{x},0) = 0$. Define the energy

$$E(t) = \frac{1}{2}\int_\Omega w^2\,d\mathbf{x}.$$

Differentiating and using the PDE:

$$\frac{dE}{dt} = \int_\Omega w\, w_t\,d\mathbf{x} = k\int_\Omega w\,\Delta w\,d\mathbf{x} = -k\int_\Omega |\nabla w|^2\,d\mathbf{x} \leq 0,$$

where integration by parts and the boundary condition $w|_{\partial\Omega}=0$ were used. Since $E(t) \geq 0$ for all $t$ and $E(0) = 0$ and $E$ is nonincreasing, we conclude $E(t) = 0$ for all $t \geq 0$, hence $w \equiv 0$ and $u_1 = u_2$.

This argument is paradigmatic: construct a nonnegative "energy" for the difference of two solutions, show it is nonincreasing (or strictly decreasing), and use the initial condition to conclude it is zero. Variants of this argument prove uniqueness for wave equations, elliptic equations, and many nonlinear problems.

## Continuous Dependence and Stability

For the heat equation, continuous dependence follows from the same energy inequality: if two solutions have initial data differing by $\delta$ in $L^2(\Omega)$ (with the same boundary data), then their difference $w$ satisfies

$$E(t) \leq E(0) = \frac{\delta^2}{2},$$

so $\|u_1(\cdot,t) - u_2(\cdot,t)\|_{L^2} \leq \delta$ for all $t > 0$. The solution map is a contraction in $L^2$ — small initial errors can only shrink over time (the smoothing property of the heat equation). This is well-posedness in the most satisfying sense.

## Connection to Numerical Methods

Well-posedness is not merely an analytical concept — it is the foundation of numerical analysis for PDEs. A numerical scheme approximating a well-posed problem can be expected, under mild consistency conditions, to converge to the true solution as the mesh is refined (this is the content of the Lax equivalence theorem). A scheme approximating an ill-posed problem will exhibit exponential growth of rounding errors and produce nonsense, regardless of how carefully it is implemented.

Understanding well-posedness is therefore not a theoretical luxury but a practical necessity for anyone who wants to compute reliable solutions to PDEs.
