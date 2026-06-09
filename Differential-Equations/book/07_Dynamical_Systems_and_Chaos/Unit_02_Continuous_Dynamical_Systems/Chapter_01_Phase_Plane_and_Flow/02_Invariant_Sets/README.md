# Invariant Sets

The geometry of a dynamical system is organized by its invariant sets—subsets of phase space that are mapped to themselves under the flow. Equilibria are the simplest invariant sets; stable and unstable manifolds are more complex ones. Understanding how these sets intersect and how they organize the surrounding dynamics is the central task of geometric dynamical systems theory.

## Basic Definitions

Let $\phi_t$ be the flow of the vector field $F: U \subset \mathbb{R}^n \to \mathbb{R}^n$.

A set $S \subset U$ is **positively invariant** if $\phi_t(S) \subset S$ for all $t \geq 0$, **negatively invariant** if $\phi_t(S) \subset S$ for all $t \leq 0$, and **invariant** (or bi-invariant) if $\phi_t(S) = S$ for all $t \in \mathbb{R}$.

Every orbit $\{\phi_t(x_0) : t \in I(x_0)\}$ is invariant. Equilibria $\{x^*\}$ are invariant. Closed orbits (limit cycles) are invariant. The entire phase space is trivially invariant.

The **$\omega$-limit set** of a point $x_0$ is

$$\omega(x_0) = \bigcap_{s \geq 0} \overline{\{\phi_t(x_0) : t \geq s\}},$$

the set of accumulation points of the forward orbit. Similarly, the **$\alpha$-limit set** $\alpha(x_0)$ consists of accumulation points of the backward orbit. Both are closed and invariant.

## Stable and Unstable Manifolds

For a hyperbolic equilibrium $x^*$ (where $DF(x^*)$ has no purely imaginary eigenvalues), the stable and unstable manifolds are the most important invariant sets.

**Definition.** The **stable manifold** of $x^*$ is

$$W^s(x^*) = \{x \in U : \lim_{t \to +\infty} \phi_t(x) = x^*\}.$$

The **unstable manifold** is

$$W^u(x^*) = \{x \in U : \lim_{t \to -\infty} \phi_t(x) = x^*\}.$$

**Theorem (Stable Manifold Theorem).** Let $x^*$ be a hyperbolic equilibrium of the $C^k$ vector field $F$. Let $E^s$ (respectively $E^u$) be the stable (respectively unstable) eigenspace of $DF(x^*)$. Then:
1. $W^s(x^*)$ is a $C^k$ immersed submanifold of $\mathbb{R}^n$, tangent to $E^s$ at $x^*$, of dimension $\dim E^s$.
2. $W^u(x^*)$ is a $C^k$ immersed submanifold tangent to $E^u$ at $x^*$, of dimension $\dim E^u$.
3. Both $W^s(x^*)$ and $W^u(x^*)$ are invariant under the flow.

**Proof sketch.** Work in coordinates where $DF(x^*) = \begin{pmatrix} A^s & 0 \\ 0 & A^u \end{pmatrix}$ with $\text{Re}(\text{spec}(A^s)) < 0 < \text{Re}(\text{spec}(A^u))$. Apply the graph transform (or Hadamard-Perron) method: the stable manifold near $x^*$ is the graph of a $C^k$ function $h^s: E^s \to E^u$ with $h^s(0) = 0$, $Dh^s(0) = 0$. This graph is constructed as the fixed point of a contraction on a space of Lipschitz functions. $\square$

## Center Manifolds

When $DF(x^*)$ has eigenvalues with zero real part (the nonhyperbolic case), the stable manifold theorem applies to the stable and unstable directions, but a **center manifold** $W^c(x^*)$ handles the neutral directions.

**Theorem (Center Manifold Theorem).** If $E^c$ is the center eigenspace of $DF(x^*)$, then there exists a $C^k$ invariant manifold $W^c(x^*)$ tangent to $E^c$ at $x^*$. It need not be unique, but the dynamics on any center manifold are $C^k$ conjugate. The center manifold captures all dynamics that are neither attracted to $x^*$ nor repelled from it.

The importance of center manifolds is that bifurcations occur on them: all the interesting dynamics at a non-hyperbolic equilibrium is confined to the (typically low-dimensional) center manifold, reducing the analysis to a lower-dimensional problem.

## Limit Cycles and the Poincaré-Bendixson Theorem

In two dimensions, the $\omega$-limit set of a bounded orbit is fully constrained by the Poincaré-Bendixson theorem:

**Theorem (Poincaré-Bendixson).** If the forward orbit $\{\phi_t(x_0) : t \geq 0\}$ is bounded and its $\omega$-limit set $\omega(x_0)$ contains no equilibria, then $\omega(x_0)$ is a periodic orbit.

**Proof idea.** Take any $p \in \omega(x_0)$. The orbit through $p$ is also in $\omega(x_0)$ (since $\omega(x_0)$ is invariant). Through any point $q$ not on the orbit, draw a transversal $\Sigma$ to the vector field. The orbit through $x_0$ crosses $\Sigma$ at a monotone sequence of points (by the Jordan curve theorem applied to the arc from one crossing to the next), and these crossings must converge to the crossing of $\omega(x_0) \cap \Sigma$. This forces the $\omega$-limit set to be a simple closed curve, i.e., a periodic orbit. $\square$

**Corollary (Bendixson's Criterion).** If $\text{div}\, F$ does not change sign in a simply connected region $D$, then $D$ contains no periodic orbits.

**Proof.** If there were a periodic orbit $\Gamma$ bounding a region $R$, Green's theorem would give $\oint_\Gamma F \cdot ds = \iint_R \text{div}\, F \, dA$. The left side equals $\oint_\Gamma F \cdot ds = 0$ (since $F$ is tangent to $\Gamma$, so $F \cdot \hat{n} = 0$, but more precisely: integrating $F \cdot \hat{n}$ around $\Gamma$... the argument uses the standard form of Green's theorem and the fact that $F$ is tangent to $\Gamma$). If $\text{div}\, F \neq 0$ on $R$, the integral is nonzero, a contradiction. $\square$

## Trapping Regions and the Existence of Limit Cycles

To establish the existence of a limit cycle, one often combines Bendixson's criterion with a trapping region argument:

1. Find a compact region $\mathcal{T}$ with smooth boundary such that the vector field points inward on $\partial \mathcal{T}$ (a **trapping region** or **positively invariant region**).
2. Show that any equilibrium inside $\mathcal{T}$ is repelling (e.g., by checking that $\text{tr}\, DF(x^*) > 0$).
3. By Poincaré-Bendixson, any orbit starting in $\mathcal{T}$ must have its $\omega$-limit set be a periodic orbit.

**Example: Van der Pol oscillator.** The system $\dot{x} = y$, $\dot{y} = \mu(1-x^2)y - x$ (for $\mu > 0$) has a single equilibrium at the origin with $\text{tr}\, DF(0,0) = \mu > 0$ (unstable). One constructs an annular trapping region: the inner boundary is a small circle around the origin (orbits point outward), and the outer boundary is a large circle (orbits point inward, by energy estimates). Poincaré-Bendixson then guarantees the existence of at least one limit cycle in the annulus.

## Invariant Tori and Quasi-Periodic Dynamics

In systems with two or more independent oscillations, trajectories can wind around invariant tori—products of circles $T^2 = S^1 \times S^1$. If the two frequencies $\omega_1, \omega_2$ are rationally related ($\omega_1/\omega_2 \in \mathbb{Q}$), the orbit closes, giving a periodic orbit. If $\omega_1/\omega_2 \notin \mathbb{Q}$, the orbit is dense on the torus—quasi-periodic behavior. Invariant tori appear naturally in near-integrable Hamiltonian systems and are central to the KAM (Kolmogorov-Arnold-Moser) theorem, which describes their persistence under perturbation.
