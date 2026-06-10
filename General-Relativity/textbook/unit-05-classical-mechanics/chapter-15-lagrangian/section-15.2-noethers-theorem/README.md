# Section 15.2: Noether's Theorem

---

## Section Introduction

In 1915, Emmy Noether proved one of the most beautiful theorems in physics: **every continuous symmetry of the action corresponds to a conserved quantity**. Time translation invariance gives energy conservation. Spatial translation invariance gives momentum conservation. Rotational invariance gives angular momentum conservation. Gauge invariance in electrodynamics gives electric charge conservation.

This theorem transforms our understanding of conservation laws: they are not independent assumptions but *consequences* of symmetry. And conversely, if we observe a conservation law, we know there is a corresponding symmetry.

In GR, the Einstein equations G_{μν} = 8πG T_{μν} are derived from the Einstein-Hilbert action, which is invariant under diffeomorphisms (smooth coordinate changes). Noether's theorem applied to this diffeomorphism invariance gives the contracted Bianchi identity ∇_μ G^{μν} = 0, which implies ∇_μ T^{μν} = 0 — local conservation of energy-momentum.

---

## 15.2.1 Statement of Noether's Theorem

**Setup**: Consider the action $S[q] = \int L(q, \dot q, t) dt$, and a one-parameter family of transformations:

$$q^i \to q^i + \varepsilon K^i(q, \dot q, t) + O(\varepsilon^2)$$

(a smooth deformation of the trajectory depending on a parameter ε, with ε = 0 giving the identity).

**Definition**: This is a **symmetry of the Lagrangian** if:

$$L(q + \varepsilon K, \dot q + \varepsilon \dot K, t) = L(q, \dot q, t) + \varepsilon \frac{dF}{dt}$$

for some function F (the Lagrangian changes by a total time derivative — which doesn't affect the equations of motion).

**Noether's Theorem**: If the one-parameter family above is a symmetry of the Lagrangian, then the quantity:

$$J = \frac{\partial L}{\partial \dot{q}^i} K^i - F$$

is conserved along any solution of the Euler-Lagrange equations: $\dot{J} = 0$.

*Proof*:

$$\dot{J} = \frac{d}{dt}\left(\frac{\partial L}{\partial \dot{q}^i}\right) K^i + \frac{\partial L}{\partial \dot{q}^i}\dot{K}^i - \dot{F}$$

By the assumption that the transformation is a symmetry:

$$\varepsilon\dot{J} = \frac{d}{d\varepsilon}\bigg|_{\varepsilon=0}L(q+\varepsilon K, \dot{q}+\varepsilon\dot{K}) - \varepsilon\dot{F} = \frac{\partial L}{\partial q^i}K^i + \frac{\partial L}{\partial \dot{q}^i}\dot{K}^i - \dot{F}$$

On a solution of the E-L equations, $\partial L/\partial q^i = d/dt(\partial L/\partial \dot{q}^i)$. So:

$$\varepsilon\dot{J} = \frac{d}{dt}\left(\frac{\partial L}{\partial \dot{q}^i}\right)K^i + \frac{\partial L}{\partial \dot{q}^i}\dot{K}^i - \dot{F} = \frac{d}{dt}\left(\frac{\partial L}{\partial \dot{q}^i}K^i\right) - \dot{F} = \varepsilon\dot{J} - \varepsilon\dot{J} = 0$$ □

---

## 15.2.2 The Standard Conservation Laws

**Time translation symmetry → Energy conservation**:

The symmetry is $t \to t + \varepsilon$, $q^i(t) \to q^i(t + \varepsilon) \approx q^i(t) + \varepsilon\dot{q}^i(t)$, so $K^i = \dot{q}^i$.

For a time-independent Lagrangian (∂L/∂t = 0), the Lagrangian changes by $dL/dt \cdot \varepsilon$ under this transformation — it changes by a total derivative with F = L. The Noether charge is:

$$J = \frac{\partial L}{\partial \dot{q}^i}\dot{q}^i - L \equiv H$$

This is the **Hamiltonian** (Legendre transform of L). Energy H is conserved when L has no explicit time dependence.

**Spatial translation symmetry → Momentum conservation**:

For a system in ℝ³ with $L = T - V(q)$, the symmetry $q^i \to q^i + \varepsilon e^i$ (translation in direction $e$) with K^i = e^i and F = 0. The Noether charge is:

$$J = \frac{\partial L}{\partial \dot{q}^i}e^i = p_i e^i = \mathbf{p} \cdot \mathbf{e}$$

If V(q) = V(|q|) is spherically symmetric, then L is invariant under all translations in 3D... wait: L is translationally invariant only if V is constant. More precisely: the **total** momentum of a system with no external forces is conserved (the center-of-mass translational symmetry of the total Lagrangian).

**Rotational symmetry → Angular momentum conservation**:

Rotation by ε about the z-axis: q → q + ε(−q_y, q_x, 0). The Noether charge is:

$$J = p_x(-q_y) + p_y(q_x) = q_x p_y - q_y p_x = L_z$$

The z-component of angular momentum. If L is invariant under all rotations (as for a central force V = V(|q|)), all three components of L = q × p are conserved.

**Table of symmetries and conservation laws**:

| Symmetry | Conserved quantity |
|----------|-------------------|
| Time translation | Energy H |
| Spatial translation | Linear momentum p |
| Rotation | Angular momentum L = q × p |
| Boost (Galilean) | Center of mass position |
| Phase rotation (ψ → e^{iα}ψ) | Particle number N |
| Gauge symmetry (A → A + ∇χ) | Electric charge Q |
| Diffeomorphism invariance | ∇_μ T^{μν} = 0 (energy-momentum conservation) |

---

## 15.2.3 Noether's Theorem for Field Theories

For a field theory with action $S[\phi] = \int {\cal L}(\phi, \partial_\mu\phi)\, d^4x$, a symmetry is a transformation $\phi \to \phi + \varepsilon J$ (and possibly $x^\mu \to x^\mu + \varepsilon\xi^\mu$) that leaves the Lagrangian density invariant up to a total divergence.

**Noether current**: The Noether current $J^\mu$ is:

$$J^\mu = \frac{\partial {\cal L}}{\partial(\partial_\mu\phi)}J - T^{\mu\nu}\xi_\nu$$

where $T^{\mu\nu}$ is the canonical stress-energy tensor.

The conservation law is $\partial_\mu J^\mu = 0$ (by the E-L equations), which by the divergence theorem gives:

$$Q = \int_\Sigma J^0 \, d^3x = \text{const}$$

(conserved charge Q is the integral of the time-component of J over a spatial slice).

**Examples in field theory**:
- U(1) phase rotation φ → e^{iα}φ for a complex scalar field: gives the electric current $J^\mu$ and charge conservation.
- Spacetime translation ξ^μ = a^μ (constant): gives the stress-energy tensor $T^{\mu\nu}$ and energy-momentum conservation.
- Lorentz transformations: give the conserved angular momentum tensor and boost generator.

---

## 15.2.4 Noether's Theorem and GR

**Diffeomorphism invariance of the Einstein-Hilbert action**: The action $S[g] = \int R\sqrt{-g}\, d^4x$ is invariant under diffeomorphisms (arbitrary smooth coordinate changes). This is Noether's theorem applied to a gauge symmetry.

The Noether identity corresponding to diffeomorphism invariance is not a conservation law of the form $\partial_\mu J^\mu = 0$ (because diffeomorphisms are a gauge symmetry, not a global symmetry). Instead, it gives an identity between the equations of motion: the **Bianchi identity**:

$$\nabla_\mu G^{\mu\nu} = 0$$

where G^{μν} is the Einstein tensor. This identity holds off-shell (for any metric, not just solutions of the field equations). By the field equations G^{μν} = 8πG T^{μν}, this implies:

$$\nabla_\mu T^{\mu\nu} = 0$$

This is the **covariant conservation of energy-momentum** — a consequence of the diffeomorphism invariance of the gravitational action, via Noether's theorem.

[Noether, E. (1918). "Invariante Variationsprobleme." *Nachrichten von der Gesellschaft der Wissenschaften zu Göttingen*, 235–257. The original paper proving both theorems: the one for global symmetries (giving conserved currents) and the one for gauge symmetries (giving identities between the equations of motion). Both theorems are in this one 12-page paper.]

---

## 15.2.5 The Energy Problem in GR

Noether's theorem guarantees conserved energy for a system with time-translation symmetry. But GR's spacetime is dynamical — the metric evolves with time, breaking exact time-translation symmetry. There is no general conserved energy for GR.

However:
- For **stationary spacetimes** (those with a timelike Killing vector ξ^μ = (∂/∂t)^μ): the energy $E = -g_{μν} u^μ ξ^ν$ is conserved along geodesics. (Noether's theorem for this specific time-translation symmetry.)
- For **asymptotically flat spacetimes**: the ADM energy (measured at infinity) is conserved. It equals the total mass-energy content of the spacetime, including gravitational binding energy.
- For **cosmological spacetimes** (FLRW): there is no timelike Killing vector in general, and energy is not conserved. The expansion of the universe does work on photons (cosmological redshift) — this energy is not "lost" but goes into the kinetic energy of the universe's expansion.

The energy problem in GR is one of the most conceptually subtle aspects of the theory. The Noether framework reveals why: conservation of energy requires a symmetry, and dynamical spacetime may lack that symmetry globally.

---

## References

- Noether, E. (1918). "Invariante Variationsprobleme." *Nachrichten von der Gesellschaft der Wissenschaften zu Göttingen, Math.-Phys. Klasse*, 235–257. Translated by M.A. Tavel, *Transport Theory and Statistical Physics*, 1 (1971), 183–207. [The original paper: both Noether theorems (global symmetries and gauge symmetries) in 12 pages. One of the most important papers in theoretical physics.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [§21.3 on the action principle and Noether's theorem in GR; §15.2 on the Bianchi identity as the gauge Noether identity.]
- Wald, R.M. (1993). "Black hole entropy is Noether charge." *Physical Review D*, 48, R3427–R3431. [A beautiful application of Noether's theorem: the entropy of a black hole in any diff-invariant gravity theory is the Noether charge associated with the diffeomorphism that generates the horizon. For Einstein gravity, this gives S = A/(4G), the Bekenstein-Hawking entropy.]
