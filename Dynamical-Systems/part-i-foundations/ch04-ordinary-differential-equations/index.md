# Chapter 4 — Ordinary Differential Equations

> *A flow is a group homomorphism from time into diffeomorphisms. An ODE is the infinitesimal generator of this group. Mastering ODEs means learning to think in flows.*

---

The shift from "solutions to equations" to "flows on spaces" is one of the conceptual leaps that separates the dynamical systems perspective from classical ODE theory. In classical analysis, you solve an ODE and write down a formula. In dynamical systems, you study the *flow* — the family of maps $\Phi_t: M \to M$ that the ODE generates — and you ask about its global properties: fixed points, periodic orbits, stable and unstable manifolds, chaos.

This chapter builds the bridge between the two perspectives. We start with existence and uniqueness (Picard-Lindelöf, which is just the Banach Fixed Point Theorem applied to a function space), move to the global picture of flows, classify linear systems through their spectral data, and then study the local geometry near hyperbolic equilibria through the Hartman-Grobman theorem and the stable manifold theorem. The chapter closes with gradient systems, Hamiltonian systems, and Poincaré maps — the main structural types you'll encounter.

**Prerequisites:** Chapters 1 (metric spaces, Banach spaces, Contraction Mapping Theorem), 3 (smooth manifolds, tangent bundles).

**What this chapter builds:**

Existence and uniqueness of solutions (Picard-Lindelöf, Carathéodory); the global perspective of the *flow* as a family of diffeomorphisms; linear systems and their spectral classification; the Hartman-Grobman theorem (linearization at hyperbolic equilibria); stable and unstable manifolds; the center manifold theorem; phase portraits and qualitative analysis; and gradient and Hamiltonian systems.

**Sections:**

- [4.1 Existence and Uniqueness](existence-and-uniqueness.md)
- [4.2 The Flow as a Family of Diffeomorphisms](the-flow.md)
- [4.3 Linear Systems](linear-systems.md)
- [4.4 Linearization and the Hartman-Grobman Theorem](linearization-and-hartman-grobman.md)
- [4.5 The Center Manifold Theorem](center-manifold-theorem.md)
- [4.6 Phase Portraits and Qualitative Analysis](phase-portraits.md)
- [4.7 Gradient and Hamiltonian Systems](gradient-and-hamiltonian.md)
- [Exercises](exercises.md)
- [Notes](notes.md)
