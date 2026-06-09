# Poincaré Maps

The Poincaré map, or first return map, is the primary tool for reducing the study of continuous-time dynamical systems to discrete-time problems. By choosing a surface transverse to the flow and recording where orbits return to that surface, one captures the essential features of the periodic orbit structure in a setting where the full theory of iterated maps applies.

## Construction of the Poincaré Map

Let $\dot{x} = F(x)$ be a $C^k$ vector field on $U \subset \mathbb{R}^n$, with flow $\phi_t$. A **Poincaré section** (or Poincaré surface of section) is a codimension-1 $C^k$ submanifold $\Sigma \subset U$ that is everywhere transverse to the vector field: $F(x) \notin T_x\Sigma$ for all $x \in \Sigma$ (where $T_x\Sigma$ is the tangent space to $\Sigma$ at $x$).

For $x_0 \in \Sigma$ in the neighborhood of a periodic orbit $\Gamma$ that crosses $\Sigma$ at a point $p \in \Gamma \cap \Sigma$, the **first return time** $T(x_0) > 0$ is the smallest positive time such that $\phi_{T(x_0)}(x_0) \in \Sigma$. The **Poincaré map** is

$$P: V \subset \Sigma \to \Sigma, \quad P(x_0) = \phi_{T(x_0)}(x_0),$$

defined on some neighborhood $V$ of $p$ in $\Sigma$.

**Theorem.** If $F$ is $C^k$ and $\Sigma$ is a $C^k$ Poincaré section, then the return time $T: V \to \mathbb{R}$ and the Poincaré map $P: V \to \Sigma$ are $C^k$.

The proof uses the implicit function theorem: $T(x_0)$ is implicitly defined by the condition that $\phi_{T(x_0)}(x_0) \in \Sigma$, which is a $C^k$ equation in $(T, x_0)$, and its solution is $C^k$ by the implicit function theorem since $F$ is transverse to $\Sigma$.

## Fixed Points and Periodic Orbits

The periodic orbit $\Gamma$ corresponds to the fixed point $p$ of $P$: $P(p) = p$. More generally, if $\Gamma$ is a periodic orbit crossing $\Sigma$ exactly $k$ times before closing, then $\Gamma$ corresponds to a period-$k$ orbit of $P$, i.e., a fixed point of $P^k$.

The stability of $\Gamma$ as a periodic orbit of the flow is determined by the eigenvalues of $DP(p)$, the derivative of the Poincaré map at the fixed point $p$. These eigenvalues are the **Floquet multipliers** of $\Gamma$ (the nontrivial ones—there is always a trivial multiplier of 1 corresponding to the flow direction, which does not appear in the Poincaré map since $\Sigma$ is transverse to the flow).

**Stability Theorem for Periodic Orbits.** Let $\Gamma$ be a periodic orbit with corresponding fixed point $p$ of the Poincaré map $P$. The orbit $\Gamma$ is:
- **Asymptotically stable** if all eigenvalues of $DP(p)$ have modulus less than 1.
- **Unstable** if any eigenvalue of $DP(p)$ has modulus greater than 1.
- **Non-hyperbolic** if some eigenvalue has modulus exactly 1.

## The Floquet Multipliers via the Fundamental Matrix

The Floquet multipliers can be computed directly from the linearization of the flow along $\Gamma$. Let $T > 0$ be the period of $\Gamma$ and let $Y(t)$ be the fundamental matrix solution of the variational equation

$$\dot{Y} = DF(\phi_t(p)) Y, \quad Y(0) = I.$$

The matrix $M = Y(T)$ is the **monodromy matrix**. Its eigenvalues $\mu_1, \ldots, \mu_n$ are the Floquet multipliers (including the trivial multiplier 1 corresponding to the eigenvector $F(p)$). The eigenvalues of $DP(p)$ are the Floquet multipliers minus the trivial one, i.e., $\mu_1, \ldots, \mu_{n-1}$ after removing the multiplier corresponding to $F(p)$.

The determinant of the monodromy matrix satisfies Liouville's formula:

$$\det M = \exp\left(\int_0^T \text{div}\, F(\phi_t(p)) \, dt\right).$$

This product of all Floquet multipliers is determined solely by the average divergence along $\Gamma$.

## Example: The Lorenz System

For the Lorenz system with $\sigma = 10$, $b = 8/3$, $r = 28$, the system has an unstable periodic orbit near the origin. To find its Floquet multipliers, one can numerically integrate the variational equation along the orbit for one period and compute the eigenvalues of the resulting monodromy matrix. The periodic orbit is unstable (some multipliers outside the unit circle), which is consistent with the existence of the strange attractor.

The Poincaré map of the Lorenz system, taken as the section $z = 27$ (a level surface above the two fixed points), has a return map that is approximately a one-dimensional tent map. This dimensional reduction—from a three-dimensional flow to an approximately one-dimensional map—is the geometric explanation for the chaotic behavior of the Lorenz attractor.

## Bifurcations of Fixed Points of the Poincaré Map

As a parameter $\mu$ is varied, the Poincaré map $P_\mu$ varies smoothly, and its fixed points can undergo bifurcations. The correspondence between flow bifurcations and map bifurcations is:

| Flow bifurcation | Poincaré map bifurcation |
|---|---|
| Saddle-node of periodic orbits | Saddle-node of fixed points |
| Period-doubling of periodic orbit | Period-doubling of fixed point |
| Hopf bifurcation from periodic orbit | Birth of invariant circle |
| Symmetry-breaking of periodic orbit | Pitchfork of fixed point |

In each case, the flow bifurcation reduces to the corresponding map bifurcation, and the entire theory of map bifurcations (developed in Chapter 2 of Unit 1 and in Chapter 3 of this unit) applies.

## The Poincaré Map and Homoclinic Orbits

One of the most important global phenomena detectable via the Poincaré map is the **transverse homoclinic intersection**: when the stable and unstable manifolds of a fixed point of $P$ intersect transversally. The Smale-Birkhoff Homoclinic Theorem states that such an intersection implies the existence of a horseshoe in the Poincaré map, hence chaotic dynamics in the flow.

For the Lorenz system, the unstable manifolds of the origin interact with the fixed points $C^\pm = (\pm\sqrt{b(r-1)}, \pm\sqrt{b(r-1)}, r-1)$ in a complex way that involves homoclinic orbits. These global connections are responsible for the strange attractor structure and were rigorously verified by Tucker (2002) using a computer-assisted proof.
