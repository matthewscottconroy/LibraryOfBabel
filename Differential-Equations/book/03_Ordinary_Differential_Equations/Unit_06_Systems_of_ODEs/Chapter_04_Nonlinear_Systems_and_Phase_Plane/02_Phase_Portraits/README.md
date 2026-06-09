# Phase Portraits

A phase portrait is a representative collection of orbits drawn in the phase plane (or phase space) of an autonomous system, together with arrows indicating the direction of increasing $t$. It is the primary visual tool for understanding the qualitative behavior of a system: where solutions go in the long run, which equilibria attract or repel nearby trajectories, whether periodic orbits exist, and how different initial conditions lead to qualitatively different behaviors. Reading a phase portrait accurately requires understanding both the local structure near equilibria and the global geometry of how orbits connect.

## Constructing a Phase Portrait

The construction of a phase portrait proceeds through several stages, from coarse to fine. Each stage adds information, and in many cases the qualitative picture is already complete after the first few steps.

**Step 1: Find equilibria.** Solve $f(x,y) = 0$ and $g(x,y) = 0$ simultaneously. Each solution is a critical point; mark it in the phase plane.

**Step 2: Draw nullclines.** The $x$-nullcline ($f = 0$) and $y$-nullcline ($g = 0$) divide the plane into regions. In each region, determine the signs of $x'$ and $y'$ by evaluating $f$ and $g$ at a test point. This gives the general direction of motion (right/left for $x$, up/down for $y$) in each region.

**Step 3: Classify equilibria.** For each equilibrium, compute the Jacobian $J$ of $(f,g)$ and find its eigenvalues. The eigenvalues determine the local behavior: node, spiral, center, or saddle (see the classification chapter). Draw typical local orbits: straight-line orbits along eigenvectors for real eigenvalues, spiraling behavior for complex eigenvalues.

**Step 4: Identify special orbits.** Saddle points have stable and unstable manifolds (the stable manifold consists of orbits approaching the saddle as $t \to +\infty$; the unstable manifold consists of orbits leaving it as $t \to -\infty$). These separatrices organize the phase portrait into distinct regions and are often the hardest orbits to draw accurately.

**Step 5: Sketch representative orbits.** Using the nullcline information, the local behavior near each equilibrium, and the direction arrows, sketch a family of representative orbits that capture the global behavior. Include orbits starting in each nullcline-defined region, following the arrows.

## The Predator-Prey Phase Portrait

The Lotka-Volterra predator-prey system $x' = ax - bxy$, $y' = -cy + dxy$ (with $a,b,c,d > 0$; $x$ = prey, $y$ = predator) provides a canonical example. Equilibria are at $(0,0)$ and $(c/d, a/b)$.

Nullclines: $x' = 0$ when $x = 0$ or $y = a/b$ (horizontal lines in the $(x,y)$-plane in the appropriate sense — here the $x$-nullcline is the $y$-axis and the horizontal line $y = a/b$). Similarly, $y' = 0$ when $y = 0$ or $x = c/d$.

The interior equilibrium $(c/d, a/b)$ has Jacobian with eigenvalues $\pm i\sqrt{ac}$ — purely imaginary, indicating a center in the linearization. For the nonlinear system, this case is genuinely a center: there is a conserved quantity $H(x,y) = dx - c\ln x + by - a\ln y$, and orbits are closed curves surrounding the interior equilibrium. The phase portrait consists of concentric closed loops — periodic oscillations where prey and predator populations cycle indefinitely.

The origin $(0,0)$ is a saddle, with the positive $x$- and $y$-axes as the stable and unstable manifolds (respectively). The phase portrait in the first quadrant shows closed orbits surrounding the interior equilibrium, bounded away from the axes for initial conditions with both populations positive.

## The Van der Pol Oscillator

The Van der Pol equation $(1 - x^2)y - x = 0$ with $\mu > 0$ written as the system $x' = y$, $y' = \mu(1-x^2)y - x$ presents a richer phase portrait. The only equilibrium is the origin $(0,0)$. The Jacobian at the origin has eigenvalues with positive real part (the origin is an unstable spiral for small $\mu$). Far from the origin, the damping $\mu(1-x^2)$ is strongly negative (since $1-x^2 < 0$ for $|x|>1$), and solutions are pulled inward.

The combination of outward repulsion near the origin and inward attraction far away forces the existence of a periodic orbit — a limit cycle. All non-equilibrium trajectories spiral toward this limit cycle from inside and outside. The phase portrait shows the unstable origin, the attracting limit cycle, and the spiraling behavior of all other trajectories.

This phase portrait is qualitatively different from the Lotka-Volterra case: the closed orbit is isolated (it is a limit cycle, not part of a family of closed orbits), and it is stable. This distinction cannot be read from the linearization alone; it requires global information.

## Reading Phase Portraits: Key Features

Several features of a phase portrait carry immediate qualitative information. The direction arrows tell whether populations/states grow or shrink. The number and types of equilibria indicate the possible long-term states of the system. Separatrices (stable/unstable manifolds of saddle points) divide the plane into basins of attraction — regions from which trajectories converge to different final states.

Closed orbits represent sustained periodic behavior. Whether a closed orbit is a limit cycle (isolated, attracting or repelling) or part of a center (a family of closed orbits) has fundamentally different implications: limit cycles are structurally stable (they persist under small perturbations of the system), while centers are fragile (a small perturbation may turn a center into a stable or unstable spiral).

Heteroclinic orbits connect different saddle points and form the boundaries between qualitatively different regions. Homoclinic orbits connect a saddle to itself and often serve as the boundary between periodic and non-periodic behavior (as in the pendulum separatrix).

## Global versus Local Information

The classification of equilibria via the Jacobian (linearization) gives local information: what orbits look like near each rest point. Assembling a complete phase portrait requires understanding global structure. The key principle is that orbits fill the phase plane without crossing; they are determined entirely by the vector field, and the qualitative picture is constrained by continuity and the structure of the equilibria.

For linear systems, the global phase portrait is determined entirely by the eigenvalues: all orbits have the same qualitative behavior (all going to the origin, all going away, all spiraling, etc.). For nonlinear systems, different initial conditions can lead to fundamentally different outcomes (attraction to different equilibria, periodic vs. aperiodic behavior), and the phase portrait captures this multiplicity.

The phase portrait is not merely a diagram — it is a complete representation of the system's dynamics, encoding all qualitative information about solution behavior. Skill in constructing and interpreting phase portraits is one of the core competencies in the qualitative theory of differential equations.
