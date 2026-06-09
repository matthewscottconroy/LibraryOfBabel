# Unit 6: Systems of Ordinary Differential Equations

A system of first-order ODEs $\mathbf{x}' = \mathbf{f}(t, \mathbf{x})$ governs the simultaneous evolution of multiple interacting quantities. Such systems arise directly as models (predator-prey dynamics, competing species, multi-compartment pharmacokinetics) and as reformulations of higher-order single equations. They provide the natural framework for studying nonlinear dynamics, phase plane analysis, and stability theory.

## Why Systems?

Every $n$-th order ODE can be written as a system of $n$ first-order equations. The second-order oscillator $y'' + py' + qy = 0$ becomes the two-dimensional system $x_1' = x_2$, $x_2' = -qx_1 - px_2$ by setting $x_1 = y$ and $x_2 = y'$. This reformulation allows geometric analysis (phase portraits) and connects the ODE to linear algebra (eigenvalues, matrix exponentials).

More fundamentally, many physical systems are naturally described by systems rather than single equations: the motion of a coupled oscillator, the interaction of competing populations, the coupled heat and mass transfer in a chemical reactor. The theory of systems is therefore not a mere extension of single-equation methods but an essential framework in its own right.

## Unit Organization

The unit develops in five chapters. Chapter 1 establishes the structural theory of linear systems in matrix form, including the superposition principle, the Wronskian (now a determinant of $n \times n$ matrices), and the fundamental matrix. Chapters 2 and 3 solve homogeneous and nonhomogeneous linear systems, respectively, using eigenvalue methods, the matrix exponential, undetermined coefficients, and variation of parameters.

Chapter 4 turns to nonlinear systems and the phase plane: autonomous systems, phase portraits, linearization near equilibria, classification of equilibria, and Lyapunov stability theory. Chapter 5 addresses limit cycles (closed trajectories attracting nearby orbits), the Poincare-Bendixson theorem (ruling out chaos in the plane), Hopf bifurcation (the birth of limit cycles), and an introduction to chaos in three-dimensional systems.

## Central Themes

The most important conceptual thread of this unit is the geometric picture of solutions as trajectories in the state space (phase space). For linear systems, the geometry is determined by the eigenvalues: saddles, nodes, spirals, and centers are the four types of hyperbolic equilibria, each corresponding to a different pattern of eigenvalues. For nonlinear systems, linearization at equilibria gives local information, while global behavior requires more powerful tools (Lyapunov functions, Poincare-Bendixson).
