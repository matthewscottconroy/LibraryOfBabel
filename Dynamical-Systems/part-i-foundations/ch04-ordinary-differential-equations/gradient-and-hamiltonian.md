# 4.7 Gradient and Hamiltonian Systems

Two special classes of ODEs deserve particular attention because of their ubiquity in physics and their distinctive dynamical properties. Gradient systems always converge; Hamiltonian systems always preserve energy and volume.

## 4.7.1 Gradient Systems

**Definition 4.7.1.** A *gradient system* on a Riemannian manifold $(M, g)$ is $\dot{x} = -\text{grad}_g(V)(x)$ for a smooth *potential function* $V: M \to \mathbb{R}$.

The vector field always points in the direction of steepest descent of $V$. The key properties:

**Properties:**
- $V$ decreases along orbits: $\frac{d}{dt} V(\Phi_t(x)) = -\|\text{grad}(V)\|^2 \leq 0$
- Omega-limit sets consist of equilibria: $\omega(p) \subseteq \{q : \text{grad}(V)(q) = 0\}$ (by LaSalle's invariance principle)
- Gradient systems have no periodic orbits (since $V$ is strictly decreasing along non-constant orbits)

The first property says $V$ is a *Lyapunov function* — it decreases along trajectories. The third property is why gradient systems are "boring" from a dynamical point of view: there's no recurrence. Every orbit converges to an equilibrium (a critical point of $V$), and the interesting question is which equilibrium — that's the problem of the basin of attraction.

Gradient systems are the dynamical model for optimization: the ODE is a continuous-time gradient descent, and the equilibria are (local) minima, maxima, and saddles of $V$. This connection to optimization is exploited in machine learning (gradient flow training of neural networks) and in the study of Morse theory (the topology of $M$ is reflected in the gradient dynamics).

## 4.7.2 Hamiltonian Systems

Hamiltonian systems are the polar opposite of gradient systems: they preserve everything.

**Definition 4.7.2.** A *Hamiltonian system* on a symplectic manifold $(M, \omega)$ is determined by a smooth function $H: M \to \mathbb{R}$ (the Hamiltonian). The vector field $X_H$ satisfies $\omega(X_H, \cdot) = dH$.

In local Darboux coordinates $(q_1, \ldots, q_n, p_1, \ldots, p_n)$ with $\omega = \sum_i dq_i \wedge dp_i$:
$$\dot{q}_i = \frac{\partial H}{\partial p_i}, \quad \dot{p}_i = -\frac{\partial H}{\partial q_i}.$$

These are Hamilton's equations, the foundation of classical mechanics. The $q_i$ are generalized positions, the $p_i$ are generalized momenta, and $H$ is the total energy.

**Conservation Laws:**
- $H$ is conserved: $\frac{d}{dt} H = 0$ (energy conservation — orbits stay on energy level sets)
- *Liouville's theorem*: the flow preserves phase space volume (the Liouville measure $\omega^n$)

Energy conservation is a consequence of the antisymmetry of the symplectic form: $\frac{d}{dt} H(x(t)) = dH \cdot \dot{x} = dH \cdot X_H = \omega(X_H, X_H) = 0$.

Liouville's theorem is more subtle. The symplectic form $\omega$ is closed ($d\omega = 0$), so its $n$-th power $\omega^n$ is also closed — it's the volume form preserved by the flow. This is a consequence of Stokes' theorem applied to the flow.

The dynamical consequences of these conservation laws are profound. Hamiltonian systems cannot have attractors in the usual sense — if $H$ is preserved, orbits can't converge to a set of lower $H$-value. If volume is preserved, orbits can't asymptotically contract. This is why Hamiltonian systems exhibit recurrence (Poincaré's recurrence theorem), quasi-periodic behavior (KAM theory), and can only exhibit "conservative chaos" — chaos that preserves volume.

The contrast between gradient systems (always converge, no recurrence) and Hamiltonian systems (never converge, always recurrent) is a fundamental structural dichotomy in dynamics. Most interesting systems are neither — but understanding the two extremes clarifies the picture.
