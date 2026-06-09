# Unit 2: Continuous Dynamical Systems

A continuous dynamical system is an autonomous ordinary differential equation $\dot{x} = F(x)$, where $x \in \mathbb{R}^n$ and $F: U \to \mathbb{R}^n$ is a smooth vector field on an open set $U$. The solution starting at $x_0$ at time $t = 0$ is denoted $\phi_t(x_0)$, and the map $\phi_t: U \to U$ is called the **flow** at time $t$. Unlike the discrete setting, where time advances in integer steps, here time is a continuous parameter, and the solution curves $t \mapsto \phi_t(x_0)$ fill the phase space without crossing.

## The Geometry of Phase Space

The central insight of continuous dynamical systems is that the totality of all solution curves—the **phase portrait**—reveals the qualitative behavior of the system far more clearly than any individual solution. Equilibria appear as isolated points. Periodic orbits appear as closed curves. Separatrices (trajectories approaching or departing from saddle points) divide the phase space into regions with qualitatively different behavior. The task is to classify and understand this geometric structure.

In two dimensions (the phase plane), the Poincaré-Bendixson theorem severely constrains possible behavior: bounded orbits must approach an equilibrium or a limit cycle. In three or more dimensions, the constraints loosen, and chaotic orbits—bounded but not approaching any periodic set—become possible.

## Unit Structure

**Chapter 1: Phase Plane and Flow** develops the theory for two-dimensional autonomous systems. The flow is defined precisely and related to the existence-uniqueness theorem. Invariant sets—including equilibria, limit cycles, and stable/unstable manifolds—are classified. The Poincaré-Bendixson theorem is proved and used to establish the existence of limit cycles.

**Chapter 2: Higher-Dimensional Systems** moves to three and more dimensions, where qualitatively new phenomena emerge. The Poincaré map reduces questions about periodic orbits to fixed point problems for discrete maps, connecting the continuous and discrete theories. The Lorenz system—three coupled ODEs derived from a truncation of the Navier-Stokes equations—provides the canonical example of chaotic behavior in a continuous flow. Lyapunov exponents quantify the rate of divergence of nearby orbits and serve as the primary diagnostic for chaos.

**Chapter 3: Bifurcation Theory** classifies the qualitative changes that occur when parameters of a vector field are varied. Saddle-node, transcritical, pitchfork, and Hopf bifurcations are the local bifurcations. Global bifurcations—homoclinic orbits, heteroclinic connections, saddle-node bifurcations of limit cycles—produce more complex changes in the phase portrait and can create or destroy strange attractors.

## Central Themes

The transition from order to chaos in continuous systems follows a more intricate path than in one-dimensional maps. The Ruelle-Takens-Newhouse scenario, an alternative to period doubling, posits that quasi-periodic flow on a torus (two or more incommensurate frequencies) can break down into chaos after a small perturbation. The Lorenz system illustrates yet another scenario, where chaos appears suddenly via a subcritical Hopf bifurcation and a global homoclinic bifurcation.

Throughout, the connection between continuous and discrete systems—via the Poincaré map and the time-$T$ map—unifies the theory. The existence of a strange attractor in a flow is typically established by showing that the Poincaré map has a strange attractor, reducing the problem to the discrete theory of Chapter 2 in Unit 1.
