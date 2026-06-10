# Chapter 15: Lagrangian and Hamiltonian Mechanics

---

## Chapter Introduction

There are two fundamentally different ways to describe the motion of a mechanical system. Newton's approach starts with forces: identify every force on each particle and solve F = ma. This is direct but can be cumbersome — especially for constrained systems (a pendulum, a bead on a wire, a planet orbiting a star).

Lagrange's approach asks a different question: among all possible paths from configuration A at time t₁ to configuration B at time t₂, which one does nature choose? The answer is the path of **stationary action** — the path for which the integral of the Lagrangian (kinetic minus potential energy) is stationary under small variations. This is **Hamilton's principle** (the principle of stationary action), and the resulting equations of motion are the **Euler-Lagrange equations**.

This variational approach has three enormous advantages:

1. **Generalized coordinates**: You can choose whatever coordinates are convenient — polar, spherical, generalized — and the Lagrange equations take the same form in all of them. Constraints are automatically handled.

2. **Noether's theorem**: Every continuous symmetry of the Lagrangian corresponds to a conserved quantity. Time translation symmetry → energy conservation. Spatial translation symmetry → momentum conservation. Rotational symmetry → angular momentum conservation. This theorem — proven by Emmy Noether in 1915 — is one of the most profound results in theoretical physics.

3. **The action principle generalizes to field theory and GR**: The Einstein-Hilbert action S = ∫ (R/16πG + L_matter) √(-g) d⁴x, varied with respect to the metric, gives the Einstein field equations. GR is the theory of gravity that follows from this action principle.

This chapter develops the Lagrangian and Hamiltonian formulations, proves Noether's theorem, and establishes the variational foundation that will be used throughout the rest of the textbook.

---

## Chapter Contents

- **Section 15.1**: The Euler-Lagrange Equations
  - Calculus of variations; the action functional
  - Euler-Lagrange equations; derivation from Hamilton's principle
  - Generalized coordinates; constrained systems
  - Examples: harmonic oscillator, pendulum, particle in central force field

- **Section 15.2**: Noether's Theorem
  - Statement and proof of Noether's theorem
  - Symmetries and conservation laws: energy, momentum, angular momentum
  - Gauge symmetry and current conservation
  - Noether's theorem in field theory

- **Section 15.3**: Hamiltonian Mechanics and Phase Space
  - Legendre transform; the Hamiltonian H = p·q̇ − L
  - Hamilton's equations ṗ = −∂H/∂q, q̇ = ∂H/∂p
  - Phase space and phase portrait
  - Poisson brackets; canonical transformations
  - Liouville's theorem; integrability

---

## The Einstein-Hilbert Action

The Lagrangian formulation of GR begins with the action:

$$S[g] = \frac{1}{16\pi G} \int_M R \sqrt{-g} \, d^4x + S_{\rm matter}$$

where R = g^{μν} R_{μν} is the Ricci scalar, √(−g) d⁴x is the invariant volume element, and G is Newton's constant. Varying S with respect to g^{μν} and setting δS = 0 gives:

$$G_{\mu\nu} \equiv R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R = 8\pi G T_{\mu\nu}$$

This is the **Einstein field equation** — the generalization of Poisson's equation ∇²Φ = 4πGρ. The left side is the Einstein tensor (curvature), the right side is the stress-energy tensor (matter and energy content).

This derivation — from an action principle — is exactly the Lagrangian approach of this chapter applied to a field theory on a curved background. Everything the reader learns here (Euler-Lagrange equations, Noether's theorem, variational principles) is directly used in Unit IX.
