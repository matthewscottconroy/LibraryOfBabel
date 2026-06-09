# Partial Differential Equations

A partial differential equation (PDE) relates an unknown function of several variables to its partial derivatives. Where ordinary differential equations govern systems with a single independent variable — a particle's position in time, a circuit's voltage — PDEs govern fields: the temperature distribution in a solid body, the displacement of a vibrating membrane, the electric potential surrounding a charged conductor. This distinction is not merely technical. It reflects the physical reality that many natural phenomena are intrinsically continuous in both space and time, and their mathematical description requires equations that encode variation in multiple directions simultaneously.

## Why PDEs Are Unavoidable

Consider a thin metal rod of length $L$ with its ends held at fixed temperatures. If you want to know how the temperature $u(x,t)$ evolves from some initial profile, no ODE suffices. The temperature at a point depends on its neighbors — heat flows according to local gradients — and this dependence across space as well as time is exactly what PDEs capture. The governing equation, the heat equation $u_t = k u_{xx}$, encodes Fourier's law of conduction in a single concise statement.

Similarly, the displacement of a plucked guitar string satisfies the wave equation $u_{tt} = c^2 u_{xx}$, and the electrostatic potential in a charge-free region satisfies Laplace's equation $\Delta u = 0$. These three equations — heat, wave, and Laplace — are the canonical examples of parabolic, hyperbolic, and elliptic PDEs respectively, and they organize the entire classical theory.

## Structure of This Module

This module develops the theory of PDEs systematically, beginning with classification and well-posedness and proceeding through the major solution techniques and the physically important equations.

**Unit 1: Introduction and Classification** establishes the foundational vocabulary: what a PDE is, how equations are ordered and classified as linear or nonlinear, and why the distinction between elliptic, parabolic, and hyperbolic types is so consequential. It also addresses the question of what constitutes a well-posed problem — existence, uniqueness, and continuous dependence on data — and surveys the boundary and initial conditions that arise in physical applications.

**Unit 2: Method of Characteristics** develops the most powerful technique for first-order PDEs and illuminates the geometric structure of hyperbolic equations. Characteristic curves carry information through the domain, and understanding them explains both how solutions are constructed and how singularities — shock waves — can form from smooth initial data.

**Unit 3: The Heat Equation** is a deep study of parabolic equations. Separation of variables, Fourier series, and the Fourier transform all appear naturally here. The heat kernel is derived and interpreted; the maximum principle is proved; and Green's functions provide a unified framework that connects to potential theory.

**Unit 4: The Wave Equation** develops the theory of hyperbolic equations, including d'Alembert's formula in one dimension and its extensions to higher dimensions. Normal modes, energy conservation, Huygens' principle, and the profound difference between wave propagation in odd and even spatial dimensions are all explored.

**Unit 5: Laplace's and Poisson's Equations** covers elliptic theory in depth, including harmonic functions, the mean value property, the maximum principle, and the representation of solutions via Green's functions. Both rectangular and polar coordinates are treated, and the Poisson integral formula for the disk is derived.

**Unit 6: Special Functions in PDEs** covers Bessel functions and spherical harmonics systematically, motivated by the appearance of these functions when separation of variables is applied in cylindrical and spherical geometries.

**Unit 7: Nonlinear PDEs** introduces the challenges that arise when linearity is lost — the Fisher equation and traveling waves, Turing pattern formation, the Burgers equation and the Hopf-Cole transformation, and the remarkable soliton solutions of the Korteweg-de Vries equation.

**Unit 8: Variational Methods** develops the calculus of variations, weak formulations of PDEs, Sobolev spaces, and the Lax-Milgram theorem, laying the theoretical foundation for the finite element method.

**Unit 9: Numerical Methods for PDEs** covers finite difference schemes — explicit and implicit — for the heat, wave, and Laplace equations, together with the theory of stability, consistency, and convergence, including von Neumann stability analysis and the CFL condition.

## Prerequisites

This module assumes completion of a course in ordinary differential equations and familiarity with Fourier series and the Fourier transform. Linear algebra (eigenvalues, orthogonal bases) and multivariable calculus (divergence theorem, change of variables) are used freely throughout. A course in complex analysis is helpful but not strictly required until the advanced sections on Green's functions.
