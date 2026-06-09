# 10.5 Global Bifurcations

Local bifurcations occur near a single equilibrium and can be analyzed by the local behavior of the vector field. Global bifurcations are different: they involve the collision of large-scale geometric objects — stable and unstable manifolds — and they can produce dynamics of surprising complexity.

---

## 10.5.1 Homoclinic Bifurcations

**Definition 10.5.1.** A *homoclinic bifurcation* occurs when the stable and unstable manifolds of an equilibrium (or periodic orbit) become tangent or coincide.

Recall from Chapter 9 that a transverse homoclinic intersection implies a horseshoe and positive topological entropy. A homoclinic bifurcation is when this intersection is about to form (or about to break): the stable and unstable manifolds are tangent, on the verge of intersecting transversally.

The most striking result in global bifurcation theory is Shilnikov's theorem, which shows that in three dimensions, a homoclinic orbit to a saddle-focus can force infinitely many periodic orbits in any neighborhood — a form of chaos arising from a single global orbit.

**Theorem 10.5.2 (Shilnikov's Theorem).** Let $p$ be a saddle-focus equilibrium of a 3D ODE with eigenvalues $-\rho \pm i\omega$ ($\rho, \omega > 0$, stable) and $\lambda > 0$ (unstable). If a homoclinic orbit $\gamma$ connects $p$ to itself and $\rho < \lambda$ (the *Shilnikov condition*), then in any neighborhood of $\gamma$, there are infinitely many periodic orbits — a *Shilnikov chaos*.

**Interpretation:** The Shilnikov condition $\rho < \lambda$ means the unstable eigenvalue is stronger than the real part of the stable eigenvalues. As the orbit spirals back toward the equilibrium, the instability forces it to pass through the neighborhood of $p$ in a new location each time, creating a horseshoe.

Let's think through the mechanism. The orbit leaves $p$ along the one-dimensional unstable manifold (expanding). As it returns to $p$, it spirals in along the stable manifold (contracting with oscillation). The spiral is governed by $e^{(-\rho \pm i\omega)t}$: it approaches $p$ at rate $e^{-\rho t}$ while rotating at frequency $\omega$. But the "return time" to a cross-section near $p$ is determined by the unstable direction, which expands at rate $e^{\lambda t}$.

The Shilnikov condition $\rho < \lambda$ ensures that the oscillatory spiraling "wins" over the straight-line approach: each return pass through the neighborhood of $p$ occurs at a different rotation angle. This creates a horseshoe structure — the return map near the homoclinic orbit stretches and folds, creating infinitely many periodic orbits.

---

## 10.5.2 Heteroclinic Cycles

**Definition 10.5.3.** A *heteroclinic cycle* is a collection of equilibria $p_1, \ldots, p_k$ and orbits $\gamma_i \in W^u(p_i) \cap W^s(p_{i+1})$ (indices mod $k$).

Heteroclinic cycles can be attracting or repelling and are structurally unstable in general (they can be broken by perturbations). But in systems with symmetry, they can be robust.

A heteroclinic cycle is a "circuit" of equilibria connected by orbits. Each orbit in the cycle asymptotes from one equilibrium and to the next. If you follow a trajectory near the cycle, it spends longer and longer times near each equilibrium in turn, before switching to the next. The switching times grow exponentially. This produces slow switching dynamics — common in models of neural competition, predator-prey systems with cycling, and fluid turbulence.

In systems with symmetry, heteroclinic cycles can be robust: the symmetry forces the relevant stable and unstable manifolds to intersect. These robust heteroclinic cycles are a source of complex, intermittent dynamics in symmetric systems — systems where the trajectory repeatedly visits the neighborhood of each equilibrium, with exponentially growing sojourn times.
