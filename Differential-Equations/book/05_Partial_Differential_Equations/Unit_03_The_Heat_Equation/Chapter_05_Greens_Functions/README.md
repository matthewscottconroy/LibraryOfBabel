# Chapter 5: Green's Functions for the Heat Equation

Green's functions provide a unified framework for representing solutions of the heat equation in terms of integral operators. Rather than solving a specific initial value problem directly, the Green's function approach finds the response to a point source — a delta function in space and time — and then represents the general solution as an integral (superposition) of point source responses. This approach connects naturally to the fundamental solution (heat kernel) for problems on $\mathbb{R}^n$ and provides an explicit formula for bounded domains that simultaneously handles initial conditions, boundary conditions, and source terms.

## The Green's Function Concept

The Green's function $G(\mathbf{x},t;\mathbf{y},s)$ for the heat equation is the temperature at point $\mathbf{x}$ at time $t$ due to a unit heat source applied at point $\mathbf{y}$ at time $s < t$, with zero initial conditions and homogeneous boundary conditions. It satisfies:

$$G_t - \kappa\Delta_\mathbf{x} G = \delta(\mathbf{x}-\mathbf{y})\delta(t-s), \quad \mathbf{x} \in \Omega,\; t > s,$$
$$G = 0 \text{ on } \partial\Omega, \qquad G(\mathbf{x},t;\mathbf{y},s) = 0 \text{ for } t < s.$$

For $\Omega = \mathbb{R}^n$, there is no spatial boundary, and the Green's function is the heat kernel:

$$G(\mathbf{x},t;\mathbf{y},s) = K(\mathbf{x}-\mathbf{y},t-s) = \frac{1}{(4\pi\kappa(t-s))^{n/2}}\exp\!\left(-\frac{|\mathbf{x}-\mathbf{y}|^2}{4\kappa(t-s)}\right).$$

## Structure of This Chapter

**Section 1: Green's Function for the Heat Equation** develops the theory on bounded domains. The key representation formula expresses the solution of the general initial-boundary value problem as integrals involving $G$:

$$u(\mathbf{x},t) = \int_\Omega G(\mathbf{x},t;\mathbf{y},0)f(\mathbf{y})\,d\mathbf{y} + \int_0^t\int_\Omega G(\mathbf{x},t;\mathbf{y},s)F(\mathbf{y},s)\,d\mathbf{y}\,ds,$$

where $f$ is the initial data and $F$ is the source term. This formula is the heat equation analogue of the variation-of-parameters formula for ODEs.

**Section 2: Duhamel's Principle** provides an alternative derivation of the representation formula using the principle of superposition in time. Duhamel's principle asserts that the solution to the nonhomogeneous problem $u_t - \kappa\Delta u = F$ can be built up by treating each instantaneous source $F(\mathbf{x},s)\,ds$ as creating a "pulse" that then evolves under the homogeneous heat equation for the remaining time $t-s$. The total solution is the integral of these responses over $s$ from $0$ to $t$.

## Central Theme

Green's functions connect the heat equation to the potential theory of Laplace's equation via the relationship between the parabolic Green's function and the elliptic Green's function. In particular, the time integral $\int_0^\infty G(\mathbf{x},t;\mathbf{y},0)\,dt$ is (formally) the Green's function of the Laplacian on $\Omega$ — this is the connection between diffusion and potential theory that underlies many results in probability (the connection between Brownian motion and harmonic functions) and physics (the heat kernel expansion in quantum mechanics and spectral geometry).
