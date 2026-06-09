# Unit 5: Laplace's and Poisson's Equations

Laplace's equation $\Delta u = 0$ and its nonhomogeneous counterpart $\Delta u = f$ (Poisson's equation) are the paradigmatic elliptic PDEs. They describe equilibrium configurations: the steady-state temperature distribution in a body, the electric potential in a charge-free region, the velocity potential of an irrotational fluid flow, the gravitational potential in empty space. Because there is no time variable, solutions represent the state a system would reach if left undisturbed indefinitely, and the theory concerns itself with the geometry of these equilibrium states rather than their temporal evolution.

## Physical Origins

Laplace's equation emerges whenever a conserved quantity — charge, heat, mass flux — is in equilibrium. If the temperature satisfies $u_t = \kappa\Delta u$ and reaches a steady state $u_t = 0$, then $\Delta u = 0$. If the electric potential $V$ satisfies $\Delta V = -\rho/\varepsilon_0$ and there are no charges ($\rho = 0$), then $\Delta V = 0$. If the velocity field of an incompressible irrotational fluid is $\mathbf{v} = \nabla\phi$, then incompressibility $\nabla\cdot\mathbf{v}=0$ gives $\Delta\phi = 0$.

## Unit Overview

**Unit 5** develops the theory of Laplace's and Poisson's equations systematically across five chapters.

**Chapter 1: Derivation and Properties** establishes the key properties of harmonic functions (solutions of Laplace's equation): smoothness, the mean value property, and the maximum principle. These properties are more than technical tools — they are the geometric heart of elliptic theory.

**Chapter 2: Rectangular Coordinates** solves the Dirichlet problem on a rectangle using separation of variables and double Fourier series, and treats Neumann and mixed boundary conditions.

**Chapter 3: Polar Coordinates** solves the Dirichlet problem on a disk, derives the celebrated Poisson integral formula, and treats annular domains.

**Chapter 4: Green's Functions for Laplace** develops the fundamental solution, the Green's function for Laplace's equation, and the method of images — a technique for constructing Green's functions for simple domains by exploiting geometric symmetry.

**Chapter 5: 3D Laplace in Spherical Coordinates** separates variables in spherical coordinates, leading to spherical harmonics and the connection between Legendre polynomials and potential theory.

## The Maximum Principle as Organizing Theme

The maximum principle — a harmonic function cannot attain an interior maximum or minimum — is the organizing theorem of elliptic PDE theory. It implies uniqueness for the Dirichlet problem, comparison principles, and a priori estimates that are foundational for the entire theory of elliptic equations. Every result in this unit connects back to the maximum principle in one form or another.
