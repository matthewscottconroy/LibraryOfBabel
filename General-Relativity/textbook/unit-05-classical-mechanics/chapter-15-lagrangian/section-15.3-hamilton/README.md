# Section 15.3: Hamiltonian Mechanics and Phase Space

---

## Section Introduction

The Lagrangian formulation works in **configuration space**: the space of positions q. The Hamiltonian formulation works in **phase space**: the space of positions q and momenta p. This doubling of the number of variables — from n to 2n — makes the equations first-order (rather than second-order), reveals elegant symmetry between position and momentum, and provides the natural framework for statistical mechanics, quantum mechanics, and the canonical approach to GR.

Phase space is a **symplectic manifold**: a manifold with a non-degenerate closed 2-form ω = dp_i ∧ dq^i. The Hamiltonian flow (Hamilton's equations) preserves this 2-form — it is a symplectomorphism. This geometric viewpoint connects classical mechanics to differential geometry and provides the framework for the ADM formulation of GR (where the phase space is the space of initial data — Riemannian 3-metrics and their conjugate momenta).

---

## 15.3.1 The Legendre Transform and the Hamiltonian

**Generalized momenta**: $p_i = \frac{\partial L}{\partial \dot{q}^i}$.

**The Legendre transform**: The Hamiltonian is defined by:

$$H(q, p, t) = p_i \dot{q}^i - L(q, \dot{q}, t)$$

where q̇ must be expressed in terms of (q, p) using the relation p_i = ∂L/∂q̇^i (assuming this can be inverted — which requires that L is strictly convex in q̇).

**Equivalence**: The Euler-Lagrange equations in terms of (q, L) are equivalent to Hamilton's equations in terms of (q, p, H):

$$\dot{q}^i = \frac{\partial H}{\partial p_i}, \qquad \dot{p}_i = -\frac{\partial H}{\partial q^i}$$

*Proof*: 
$$dH = \dot{q}^i dp_i + p_i d\dot{q}^i - \frac{\partial L}{\partial q^i}dq^i - \frac{\partial L}{\partial \dot{q}^i}d\dot{q}^i = \dot{q}^i dp_i - \frac{\partial L}{\partial q^i}dq^i$$

(the $p_i d\dot{q}^i - \frac{\partial L}{\partial \dot{q}^i}d\dot{q}^i = 0$ since $p_i = \frac{\partial L}{\partial \dot{q}^i}$). Reading off:

$$\frac{\partial H}{\partial p_i} = \dot{q}^i, \qquad \frac{\partial H}{\partial q^i} = -\frac{\partial L}{\partial q^i} = -\dot{p}_i$$

(using the E-L equations $\dot p_i = \partial L/\partial q^i$ in the last step). □

**Example**: For L = (m/2)q̇² − V(q):
- p = mq̇, so q̇ = p/m.
- H = p·(p/m) − (m/2)(p/m)² + V(q) = p²/(2m) + V(q) = T + V.

The Hamiltonian equals total energy (kinetic + potential) when L = T − V with T quadratic in q̇.

---

## 15.3.2 Poisson Brackets and the Algebra of Observables

**Definition**: The **Poisson bracket** of two functions f, g on phase space is:

$$\{f, g\} = \frac{\partial f}{\partial q^i}\frac{\partial g}{\partial p_i} - \frac{\partial f}{\partial p_i}\frac{\partial g}{\partial q^i}$$

**Properties**:
- Anti-symmetry: {f, g} = −{g, f}
- Linearity: {f + g, h} = {f, h} + {g, h}
- Leibniz rule: {fg, h} = f{g, h} + {f, h}g
- Jacobi identity: {{f, g}, h} + {{g, h}, f} + {{h, f}, g} = 0

**Fundamental brackets**: {q^i, p_j} = δ^i_j, {q^i, q^j} = 0, {p_i, p_j} = 0.

**Equations of motion**: For any observable f(q, p, t):

$$\dot{f} = \{f, H\} + \frac{\partial f}{\partial t}$$

In particular: q̇^i = {q^i, H} = ∂H/∂p_i and ṗ_i = {p_i, H} = −∂H/∂q^i. ✓

**Conservation**: f is conserved iff {f, H} = 0 and ∂f/∂t = 0. The Poisson bracket with H generates time evolution.

**Connection to quantum mechanics**: In canonical quantization, the Poisson bracket is replaced by the commutator: {f, g} → (1/iℏ)[f̂, ĝ]. The fundamental brackets become [q̂^i, p̂_j] = iℏ δ^i_j — the **canonical commutation relations** of quantum mechanics.

---

## 15.3.3 Symplectic Structure and Canonical Transformations

The fundamental Poisson brackets can be encoded in the **symplectic 2-form**:

$$\omega = dp_i \wedge dq^i$$

(a differential form on phase space). The Poisson bracket is expressed in terms of ω: {f, g} = ω(X_f, X_g) where X_f is the Hamiltonian vector field of f, defined by ω(X_f, ·) = df.

**Liouville's theorem** (for Hamiltonian systems): The symplectic 2-form ω is preserved by the Hamiltonian flow: £_{X_H}ω = 0. Equivalently, the volume form on phase space (ω^n = ω ∧ ω ∧ ... ∧ ω, n times) is preserved — the Hamiltonian flow is **volume-preserving**.

*Proof*: $\frac{d}{dt}\int_V \omega^n = \int_V £_{X_H}\omega^n = 0$ since $£_{X_H}\omega = 0$. □

**Physical significance**: An ensemble of particles (with a distribution in phase space) evolves with constant phase-space density. This is Liouville's theorem in statistical mechanics: the phase-space distribution function is conserved along trajectories. It is the basis of statistical mechanics and thermodynamics.

**Canonical transformations**: A transformation (q, p) → (Q, P) is canonical if it preserves the symplectic form: ω = dP_i ∧ dQ^i (same as dp_i ∧ dq^i). Canonical transformations preserve Hamilton's equations and the Poisson brackets.

**GR connection**: The phase space of GR is the space of solutions to the constraint equations (a Riemannian 3-metric h_{ij} and conjugate momentum π^{ij}). The symplectic structure Ω = ∫ δπ^{ij} ∧ δh_{ij} d³x is preserved by time evolution. The Hamiltonian and momentum constraints generate diffeomorphisms — they are the "gauge transformations" of GR's Hamiltonian formulation.

---

## 15.3.4 Integrable Systems and Action-Angle Variables

A Hamiltonian system with n degrees of freedom is **completely integrable** if it has n independent conserved quantities in involution ({F_i, F_j} = 0 for all i, j). By the Arnol'd-Liouville theorem, the motion is confined to n-dimensional tori in phase space ("invariant tori"), and can be described by **action-angle variables** (J_i, θ_i) where:
- J_i are the "actions" (adiabatic invariants): J_i = (1/2π) ∮ p_i dq^i (integral of p over one period)
- θ_i are the "angles": conjugate to J_i, increasing uniformly θ̇_i = ω_i = ∂H/∂J_i
- The motion is: θ_i(t) = ω_i t + θ_i(0), J_i = const

**Examples of integrable systems**:
- The Kepler problem (two bodies under gravity): has 3 constants of motion (E, L², L_z). Fully integrable. The Keplerian orbits are ellipses — the closed invariant tori.
- The harmonic oscillator: integrable. Action J = E/ω.
- The Schwarzschild geodesics: integrable (E, L², L_z, + the constraint g_{μν}u^μu^ν = −1). The orbit is a precessing ellipse.

**Perturbation theory (KAM theorem)**: When a small perturbation ε is added to an integrable system, most invariant tori survive (slightly deformed), but some break up into chaos. The Kolmogorov-Arnol'd-Moser (KAM) theorem quantifies which tori survive (those with "sufficiently irrational" frequencies). This is the basis for understanding the long-term stability of planetary orbits.

**GR application**: The Schwarzschild geodesic problem is integrable (it has exactly the right number of conserved quantities). But the Kerr geodesic problem (for a rotating black hole) has a surprising extra constant: the Carter constant Q = L_θ² + ... (found by Carter in 1968). This fourth constant makes Kerr geodesics integrable — they can be solved by quadrature, enabling efficient computation of gravitational wave templates.

[Carter, B. (1968). "Global structure of the Kerr family of gravitational fields." *Physical Review*, 174, 1559–1571. The paper introducing the Carter constant and proving the separability of geodesics in Kerr spacetime.]

---

## References

- Hamilton, W.R. (1834–1835). "On a general method in dynamics." *Philosophical Transactions*, 124, 247–308; 125, 95–144. [Hamilton's introduction of the Hamiltonian function and canonical equations — one of the great papers in mathematical physics.]
- Arnol'd, V.I. (1989). *Mathematical Methods of Classical Mechanics*, 2nd ed. Springer. [The most geometrically sophisticated treatment: phase space, symplectic geometry, Poisson brackets, integrable systems, KAM theory. Chapter 9 on the Liouville theorem; Appendix on symplectic geometry.]
- Carter, B. (1968). "Global structure of the Kerr family of gravitational fields." *Physical Review*, 174, 1559–1571. [Introduces the Carter constant, enabling exact integration of geodesics in Kerr spacetime. The "hidden symmetry" encoded in the Carter constant is related to the existence of a Killing tensor — a rank-2 symmetric tensor satisfying ∇_{(μ}K_{νρ)} = 0.]
- Arnowitt, R., Deser, S., and Misner, C.W. (1962). "The dynamics of general relativity." In L. Witten, ed., *Gravitation.* [The Hamiltonian formulation of GR: phase space, constraints, and symplectic structure. The ADM formalism is the Hamiltonian mechanics of GR.]
