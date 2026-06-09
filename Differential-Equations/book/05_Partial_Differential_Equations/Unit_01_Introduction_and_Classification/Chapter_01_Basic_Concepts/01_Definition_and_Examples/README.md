# Definition and Examples of Partial Differential Equations

The simplest first-order PDE is the transport equation

$$u_t + c u_x = 0,$$

where $u = u(x,t)$ is an unknown function, $c$ is a real constant, and subscripts denote partial derivatives. Despite its apparent simplicity, this equation encodes something profound: the solution is constant along the lines $x - ct = \text{const}$ in the $(x,t)$-plane. If an initial profile $u(x,0) = f(x)$ is given, then $u(x,t) = f(x - ct)$ — the entire profile translates rigidly to the right at speed $c$. Every feature of this example — the role of characteristic curves, the propagation of information, the well-posedness under initial rather than boundary conditions — recurs throughout the theory of PDEs.

## Precise Definition

Let $\Omega \subset \mathbb{R}^n$ be an open set with independent variables $\mathbf{x} = (x_1, \ldots, x_n)$. For a multi-index $\alpha = (\alpha_1, \ldots, \alpha_n)$ with $|\alpha| = \alpha_1 + \cdots + \alpha_n$, write $D^\alpha u = \partial^{|\alpha|} u / \partial x_1^{\alpha_1} \cdots \partial x_n^{\alpha_n}$.

A **partial differential equation of order $k$** is an equation of the form

$$F\bigl(\mathbf{x},\, u(\mathbf{x}),\, Du(\mathbf{x}),\, D^2 u(\mathbf{x}),\, \ldots,\, D^k u(\mathbf{x})\bigr) = 0,$$

where $F$ is a given function on the appropriate jet space and $u \colon \Omega \to \mathbb{R}$ (or $\mathbb{R}^m$ for systems) is the unknown. A **classical solution** is a function $u \in C^k(\Omega)$ satisfying the equation at every point of $\Omega$.

## The Three Canonical Examples

The three most important PDEs in classical mathematical physics are distinguished not just by their form but by the physical phenomena they model and the mathematical structure they possess.

**The heat equation** (or diffusion equation) in one spatial dimension is

$$u_t = k\, u_{xx}, \qquad k > 0.$$

Here $u(x,t)$ represents temperature, $k$ is the thermal diffusivity, and the equation says that the rate of temperature change at a point is proportional to the concavity of the temperature profile. Regions of positive curvature heat up; regions of negative curvature cool down. The equation is parabolic, has a definite arrow of time (it is not invariant under $t \mapsto -t$ when $k > 0$), and smooths initial data instantly.

**The wave equation** in one spatial dimension is

$$u_{tt} = c^2 u_{xx},$$

where $u(x,t)$ is displacement and $c$ is wave speed. Unlike the heat equation, this equation is invariant under time reversal ($t \mapsto -t$), has finite propagation speed $c$, and can sustain discontinuities in the solution's derivatives along characteristic lines $x \pm ct = \text{const}$.

**Laplace's equation** in two dimensions is

$$u_{xx} + u_{yy} = 0,$$

often written $\Delta u = 0$ or $\nabla^2 u = 0$. Solutions are called **harmonic functions**. This equation has no time variable; it describes steady-state phenomena such as electrostatic potentials, incompressible irrotational fluid flow, and steady heat distribution. Solutions are infinitely smooth, in fact real-analytic, on their domain.

## A Gallery of Further Examples

Beyond the three canonical equations, the landscape of PDEs is vast.

The **Poisson equation** $\Delta u = f$ is the nonhomogeneous version of Laplace's equation, arising when sources (charge densities, heat sources) are present.

The **Schrödinger equation** for a free particle in one dimension,

$$i\hbar\, \psi_t = -\frac{\hbar^2}{2m}\,\psi_{xx},$$

is formally like the heat equation with imaginary diffusivity. Solutions do not decay but rather disperse: wave packets spread without damping.

**Burgers' equation**,

$$u_t + u\, u_x = \nu\, u_{xx},$$

couples a nonlinear transport term (resembling the $1$-D Navier-Stokes convection) to diffusion. When $\nu = 0$ (inviscid Burgers), smooth initial data can develop discontinuities (shocks) in finite time — a fundamentally nonlinear phenomenon with no analogue in linear theory.

The **biharmonic equation** $\Delta^2 u = 0$ arises in elasticity (the deflection of a thin elastic plate) and requires fourth-order boundary conditions.

## What Makes These Examples Telling

The heat equation, wave equation, and Laplace's equation are not merely the most historically important PDEs — they are the local models for all second-order linear PDEs. The classification theorem of Chapter 2 asserts that any second-order linear PDE can be reduced, near any point, to one of these three forms via a smooth coordinate change. Understanding these three equations thoroughly is therefore tantamount to understanding the local structure of the entire theory.

Each example also illustrates a different relationship between the equation and its natural auxiliary data. The heat equation wants initial data $u(x,0) = f(x)$ and boundary data on the spatial domain. The wave equation wants both $u(x,0) = f(x)$ and $u_t(x,0) = g(x)$. Laplace's equation wants boundary data all around $\Omega$ and no initial data at all — there is no preferred direction to call "time." Getting these pairings wrong leads immediately to ill-posed problems, as the next sections explain.

## Notation and Conventions

Throughout this module, we use the following standard notation. The Laplacian in $\mathbb{R}^n$ is $\Delta u = \sum_{i=1}^n u_{x_i x_i}$. The gradient is $\nabla u = (u_{x_1}, \ldots, u_{x_n})$. The outward unit normal to $\partial\Omega$ is $\nu$, and the normal derivative is $\partial u / \partial \nu = \nabla u \cdot \nu$. The notation $u_t, u_x, u_{xx}$, etc., is used freely for partial derivatives when the variable names are clear from context.
