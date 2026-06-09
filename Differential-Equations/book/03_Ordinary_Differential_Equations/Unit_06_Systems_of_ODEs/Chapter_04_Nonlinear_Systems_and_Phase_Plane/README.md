# Chapter 4: Nonlinear Systems and the Phase Plane

The theory of linear systems provides exact solution formulas and a complete classification of behavior. Nonlinear systems resist such exact treatment: superposition fails, and closed-form solutions exist only for exceptional cases. Yet nonlinear systems are ubiquitous in science and engineering — the pendulum, predator-prey dynamics, electrical oscillators, chemical kinetics, and neural firing models are all intrinsically nonlinear. The tools of qualitative analysis allow one to understand the long-term behavior of solutions without solving the equations explicitly.

The central object is the phase plane (or, in $n$ dimensions, the phase space). A two-dimensional autonomous system $x' = f(x,y)$, $y' = g(x,y)$ defines a vector field: at each point $(x,y)$, the vector $(f,g)$ indicates the instantaneous velocity of the solution trajectory passing through that point. Solution curves in the phase plane — called orbits or trajectories — follow this vector field, never crossing (by uniqueness) and giving a complete geometric picture of all possible behaviors.

## Equilibria and Linearization

The equilibria (rest points, critical points) of the system are points $(x^*, y^*)$ where $f(x^*,y^*) = 0$ and $g(x^*,y^*) = 0$. Near an equilibrium, the nonlinear system can be approximated by a linear system obtained by expanding $f$ and $g$ in a Taylor series. If $\mathbf{u} = (x - x^*, y - y^*)$ denotes a small displacement, the linearized system is $\mathbf{u}' = J\mathbf{u}$, where $J$ is the Jacobian matrix of $(f,g)$ evaluated at the equilibrium. The eigenvalues of $J$ determine the local behavior: stable/unstable nodes, saddles, spirals, and centers. The Hartman-Grobman theorem guarantees that the qualitative behavior of the nonlinear system near a hyperbolic equilibrium (one with no purely imaginary eigenvalues) is topologically equivalent to that of the linear approximation.

## Global Analysis: Lyapunov Methods and Limit Sets

Linearization is inherently local. To understand global behavior — whether all solutions tend to an equilibrium, whether periodic orbits exist, whether solutions can escape to infinity — different tools are needed. Lyapunov's method constructs an energy-like function $V(x,y)$ that decreases along trajectories, providing global stability information without solving the system. The Poincaré-Bendixson theorem characterizes the possible limit sets in the plane, ruling out chaos and constraining what trajectories can do in the long run.

## Coverage

This chapter develops the theory systematically. Autonomous systems and the geometry of the phase plane are introduced first, establishing the concepts of orbits, isoclines, and limit sets. Phase portraits synthesize local information from all equilibria into a global picture. Linearization provides the local classification at each equilibrium. The classification of equilibria in two dimensions — node, spiral, center, saddle, and degenerate cases — is developed in full, including the trace-determinant diagram that organizes all cases. Finally, Lyapunov's direct method provides a global stability tool applicable to systems where linearization is inconclusive or where one needs more than local information.
