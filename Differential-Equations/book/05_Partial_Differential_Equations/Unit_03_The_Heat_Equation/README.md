# Unit 3: The Heat Equation

Of all partial differential equations, the heat equation

$$u_t = k\,\Delta u$$

is arguably the most extensively studied. Introduced by Fourier in his Théorie analytique de la chaleur (1822) as a model of thermal conduction, it has since become the canonical example of a parabolic PDE and the prototype for the entire theory of diffusion. Its solutions exhibit instantaneous smoothing, irreversibility, a maximum principle, and a rich spectral structure — properties that reappear throughout analysis and probability.

## The Physical Context

Heat conduction in a solid body $\Omega \subset \mathbb{R}^n$ is governed by Fourier's law (the heat flux $\mathbf{q} = -k\nabla u$ is proportional to the negative temperature gradient) and conservation of energy ($\rho c_p u_t = -\nabla\cdot\mathbf{q} + Q$). Combining these gives the heat equation $u_t = \kappa\Delta u + Q/(\rho c_p)$, where $\kappa = k/(\rho c_p)$ is the thermal diffusivity. The same equation governs:

- Diffusion of chemical species in a medium (Fick's second law).
- Probability density of Brownian motion (the Fokker-Planck equation reduces to the heat equation for a free particle).
- The Black-Scholes equation in mathematical finance (after a change of variables).
- The imaginary-time Schrödinger equation in quantum mechanics.

## Unit Overview

This unit develops the theory of the heat equation through five chapters.

**Chapter 1: Derivation and Physical Interpretation** derives the heat equation from Fourier's law and conservation of energy, discusses the diffusion interpretation, and establishes the dimensionless form. The thermal diffusivity $\kappa$ determines the time scale for heat to equilibrate over a length scale $L$: the equilibration time is $\sim L^2/\kappa$.

**Chapter 2: Separation of Variables** develops the solution of the heat equation on bounded domains via eigenfunction expansions. The spatial eigenfunctions (determined by the geometry of $\Omega$ and the boundary conditions) are the modes of the system; each mode decays exponentially in time at a rate proportional to the corresponding eigenvalue. High-frequency spatial oscillations decay faster than low-frequency ones, which is the mathematical expression of the smoothing property.

**Chapter 3: Fourier Transform Method** treats the heat equation on all of $\mathbb{R}^n$ (or a half-space), where the Fourier transform converts the PDE into an ODE in time. The fundamental solution (heat kernel) $K(\mathbf{x},t) = (4\pi\kappa t)^{-n/2}e^{-|\mathbf{x}|^2/(4\kappa t)}$ encodes all the essential structure of the equation. The maximum principle is proved and its consequences explored.

**Chapter 4: Other Geometries** extends the eigenfunction expansion method to cylindrical and spherical domains, where separation of variables leads to Bessel's equation and the associated Legendre equation respectively.

**Chapter 5: Green's Functions** develops the Green's function approach for the heat equation, connecting it to the fundamental solution and providing a unified framework for both the initial value problem and the nonhomogeneous problem via Duhamel's principle.

## Central Themes

Two themes run through this entire unit. First, the heat equation is a model of information loss: the smoothing property means that fine details of the initial temperature distribution are progressively erased over time, and running the equation backward is ill-posed. Second, the spectral structure of the Laplacian (its eigenvalues and eigenfunctions on the domain $\Omega$) completely determines the long-time behavior of solutions: the slowest-decaying mode (the fundamental mode) dominates at large times, and its decay rate $e^{-\kappa\lambda_1 t}$ is determined by the first eigenvalue $\lambda_1$ of $-\Delta$ on $\Omega$.
