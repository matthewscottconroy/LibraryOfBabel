# Chapter 1: Phase Plane and Flow

The phase plane is the coordinate system in which both the position and the velocity (or more generally, all state variables) of a dynamical system are represented simultaneously. For an autonomous two-dimensional system $\dot{x} = f(x,y)$, $\dot{y} = g(x,y)$, every point $(x,y)$ in the plane has an associated velocity vector $(f(x,y), g(x,y))$. The totality of solution curves threading through these velocity vectors is the phase portrait, and reading a phase portrait is among the most powerful techniques in applied mathematics.

## The Flow of a Vector Field

Given a smooth vector field $F: U \subset \mathbb{R}^n \to \mathbb{R}^n$, the associated ODE $\dot{x} = F(x)$ has, by the Picard-Lindelöf theorem, a unique solution $\phi(t, x_0)$ for each initial condition $x_0 \in U$ and for times in some maximal interval $(-T_-(x_0), T_+(x_0))$. The map $\phi_t: x_0 \mapsto \phi(t, x_0)$ is the **flow** of $F$ at time $t$.

The flow satisfies three fundamental properties:
- $\phi_0 = \text{id}$ (the identity map)
- $\phi_{t+s} = \phi_t \circ \phi_s$ (the group law)
- $\frac{d}{dt}\phi_t(x_0) = F(\phi_t(x_0))$ (satisfies the ODE)

The group law expresses the determinism of the system: evolving for time $s$ and then for time $t$ is the same as evolving for time $t + s$ directly.

## Equilibria and Periodic Orbits

The fundamental objects organizing the phase portrait are equilibria and periodic orbits. An **equilibrium** (fixed point of the flow) is a point $x^*$ with $F(x^*) = 0$, meaning $\phi_t(x^*) = x^*$ for all $t$. A **periodic orbit** is a non-constant solution satisfying $\phi_{T}(x_0) = x_0$ for some minimal period $T > 0$; it appears as a closed curve in phase space.

Between these are the heteroclinic and homoclinic orbits: solution curves that connect distinct equilibria (heteroclinic) or leave and return to the same equilibrium (homoclinic). These separatrices divide the phase plane into invariant regions.

## The Poincaré-Bendixson Theorem

In two dimensions, the possible long-term behaviors are severely constrained by topology.

**Theorem (Poincaré-Bendixson).** Let $F: U \subset \mathbb{R}^2 \to \mathbb{R}^2$ be $C^1$, and let $x_0 \in U$ be such that the forward orbit $\{\phi_t(x_0) : t \geq 0\}$ is bounded and contained in $U$. Then the $\omega$-limit set $\omega(x_0)$ is one of:
1. A single equilibrium point.
2. A periodic orbit.
3. A union of equilibria and heteroclinic or homoclinic orbits connecting them.

The theorem rules out chaos in the plane: bounded non-periodic orbits must approach a fixed point or limit cycle. The proof uses the Jordan curve theorem: any simple closed curve in $\mathbb{R}^2$ divides the plane into two regions, and this topological constraint prevents orbits from escaping to new parts of phase space once trapped in a bounded region.

## Stability of Equilibria: The Linearization Theorem

Near an equilibrium $x^*$, the flow is approximated by the linearized flow $\dot{y} = DF(x^*) y$, where $y = x - x^*$ and $DF(x^*)$ is the Jacobian matrix. The eigenvalues of $DF(x^*)$ determine the local behavior:

- All eigenvalues have negative real part: $x^*$ is a **stable node** or **stable spiral** (asymptotically stable).
- All eigenvalues have positive real part: $x^*$ is an **unstable node** or **unstable spiral** (unstable).
- Eigenvalues with both positive and negative real parts: $x^*$ is a **saddle** (unstable, but with stable and unstable manifolds).
- Pure imaginary eigenvalues: the linearization gives a center; nonlinear terms determine whether it is stable or unstable in the full system.

**Theorem (Hartman-Grobman).** If $DF(x^*)$ has no eigenvalue with zero real part (the hyperbolic case), then the flow near $x^*$ is topologically conjugate to the linearized flow near 0. The conjugacy is a homeomorphism, not necessarily a diffeomorphism.

## Invariant Sets

An **invariant set** is a set $S \subset U$ with $\phi_t(S) = S$ for all $t$. Equilibria, periodic orbits, and their stable and unstable manifolds are all invariant sets. The **stable manifold** of a saddle point $x^*$ is the set of all initial conditions whose forward orbit converges to $x^*$:

$$W^s(x^*) = \{x \in U : \lim_{t \to \infty} \phi_t(x) = x^*\}.$$

The **unstable manifold** $W^u(x^*)$ is defined similarly with $t \to -\infty$. For a saddle with $n$ negative and $m$ positive eigenvalues, $W^s(x^*)$ is an $n$-dimensional manifold and $W^u(x^*)$ is $m$-dimensional, both tangent to the corresponding eigenspaces at $x^*$.

## Key Theorems in This Chapter

The chapter develops the following results:
- **Section 1** (Flow of Differential Equations): the flow is $C^k$ if $F$ is $C^k$; the variational equation $\dot{Y} = DF(\phi_t(x_0)) Y$ governs how the flow responds to perturbations of initial conditions.
- **Section 2** (Invariant Sets): stable and unstable manifold theorem for hyperbolic equilibria; the center manifold theorem for nonhyperbolic equilibria; invariant foliations and their role in decomposing phase space.

Together, these tools allow a complete qualitative analysis of two-dimensional autonomous systems and provide the geometric vocabulary needed for higher-dimensional analysis in Chapter 2.
