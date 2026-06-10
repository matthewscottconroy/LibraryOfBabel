# Section 15.1: The Euler-Lagrange Equations and Hamilton's Principle

---

## Section Introduction

What does it mean for "nature to choose the path of least action"? This is one of the most beautiful ideas in physics — the idea that the trajectory of a mechanical system is not merely described by differential equations but is the answer to an optimization problem.

The **calculus of variations** asks: among all smooth curves joining two fixed points in configuration space, which one minimizes (or more precisely, makes stationary) the action integral? The answer is the curve satisfying the **Euler-Lagrange equations** — a system of second-order ODEs that are equivalent to Newton's laws but are much more flexible.

This variational approach is not merely a reformulation. It reveals the deep structure of mechanics: why Newton's laws have the specific form they do, why symmetries lead to conservation laws, and why the same mathematical framework (with appropriate Lagrangians) describes mechanics, electrodynamics, quantum field theory, and GR.

---

## 15.1.1 The Calculus of Variations

**Setup**: Consider functionals of the form:

$$S[q] = \int_{t_1}^{t_2} L(q(t), \dot{q}(t), t) \, dt$$

where q: [t₁, t₂] → ℝⁿ is a smooth curve and L: ℝⁿ × ℝⁿ × ℝ → ℝ is the **Lagrangian**. The functional S assigns a real number to each curve q — it is a function of functions.

**Variation**: A **variation** δq is a smooth perturbation of q that vanishes at the endpoints: δq(t₁) = δq(t₂) = 0. The varied path is q_ε(t) = q(t) + ε δq(t). The first variation of S is:

$$\delta S = \frac{d}{d\varepsilon}\bigg|_{\varepsilon=0} S[q + \varepsilon\,\delta q] = \int_{t_1}^{t_2} \left[\frac{\partial L}{\partial q^i}\delta q^i + \frac{\partial L}{\partial \dot{q}^i}\delta \dot{q}^i\right] dt$$

Integrating the second term by parts (∂L/∂q̇ⁱ δq̇ⁱ = d/dt(∂L/∂q̇ⁱ δqⁱ) − d/dt(∂L/∂q̇ⁱ)δqⁱ) and using δq(t₁) = δq(t₂) = 0 to discard the boundary terms:

$$\delta S = \int_{t_1}^{t_2} \left[\frac{\partial L}{\partial q^i} - \frac{d}{dt}\frac{\partial L}{\partial \dot{q}^i}\right] \delta q^i \, dt$$

**Hamilton's Principle** (Principle of Stationary Action): The physical trajectory is the one for which δS = 0 for all variations δqⁱ vanishing at the endpoints.

By the fundamental lemma of calculus of variations (if ∫ f(t)η(t)dt = 0 for all smooth η vanishing at endpoints, then f = 0), this gives:

$$\frac{\partial L}{\partial q^i} - \frac{d}{dt}\frac{\partial L}{\partial \dot{q}^i} = 0$$

These are the **Euler-Lagrange equations**.

---

## 15.1.2 The Standard Lagrangian

For a particle of mass m moving in a potential V(q, t), the Lagrangian is:

$$L(q, \dot{q}, t) = T - V = \frac{1}{2}m|\dot{q}|^2 - V(q, t)$$

**Euler-Lagrange equations**:
- ∂L/∂q^i = −∂V/∂q^i
- d/dt(∂L/∂q̇^i) = d/dt(mq̇^i) = mq̈^i

The E-L equations give: mq̈^i = −∂V/∂q^i, which is Newton's second law F^i = −∂V/∂q^i. ✓

The Lagrangian approach reproduces Newton's laws but works in any coordinate system.

**Generalized coordinates and momenta**:
- **Generalized coordinates** q^i: any set of independent coordinates describing the configuration space. They need not be Cartesian.
- **Generalized velocity**: q̇^i
- **Generalized (canonical) momentum**: $p_i = \frac{\partial L}{\partial \dot{q}^i}$. For L = T − V with T = (m/2)|q̇|², p_i = mq̇_i (ordinary momentum in Cartesian coordinates, but more general in curvilinear coordinates).

---

## 15.1.3 Examples

**Harmonic oscillator**: L = (m/2)q̇² − (k/2)q².
- E-L: mq̈ = −kq, i.e., q̈ + ω²q = 0 with ω = √(k/m). ✓

**Simple pendulum**: q = θ (angle from vertical), L = (mℓ²/2)θ̇² − mgℓ(1 − cos θ).
- ∂L/∂θ = −mgℓ sin θ, ∂L/∂θ̇ = mℓ²θ̇.
- E-L: mℓ²θ̈ = −mgℓ sin θ, i.e., θ̈ + (g/ℓ) sin θ = 0. ✓

**Particle in polar coordinates** (r, θ): L = (m/2)(ṙ² + r²θ̇²) − V(r).
- Radial E-L: mṙ̈ − mrθ̇² = −∂V/∂r (centrifugal acceleration term appears automatically).
- Angular E-L: d/dt(mr²θ̇) = 0, so ℓ = mr²θ̇ = const (angular momentum conservation — from the fact that V = V(r) doesn't depend on θ). ✓

The angular momentum conservation follows automatically from the cyclic coordinate θ (L doesn't depend on θ), without invoking the symmetry explicitly. This is a preview of Noether's theorem.

**Geodesics**: The geodesic (shortest path on a manifold) is an extremum of the length functional:

$$S[\gamma] = \int_a^b \sqrt{g_{ij}(\gamma)\dot\gamma^i\dot\gamma^j}\, dt$$

More conveniently, extremize the "energy":

$$S[\gamma] = \int_a^b g_{ij}(\gamma)\dot\gamma^i\dot\gamma^j\, dt$$

The Euler-Lagrange equations for this are exactly the geodesic equation:

$$\ddot{\gamma}^k + \Gamma^k_{ij}\dot\gamma^i\dot\gamma^j = 0$$

where $\Gamma^k_{ij} = \frac{1}{2}g^{kl}(\partial_i g_{jl} + \partial_j g_{il} - \partial_l g_{ij})$ are the Christoffel symbols. The geodesic equation IS the Euler-Lagrange equation for the geodesic Lagrangian.

---

## 15.1.4 Constrained Systems

**Holonomic constraints**: A constraint of the form f(q, t) = 0. A pendulum has the constraint |q| = ℓ. A bead on a wire has the constraint that q lies on the wire.

**Handling constraints**: Two methods:
1. **Eliminate coordinates**: Use the constraint to reduce the number of independent coordinates to the true number of degrees of freedom.
2. **Lagrange multipliers**: Add λ·f(q,t) to the Lagrangian and treat λ as an additional variable. The E-L equations for λ enforce the constraint; the E-L equations for q give the equations of motion.

**Example**: Bead constrained to a circle of radius R in 2D. Constraint: x² + y² = R².
- Cartesian Lagrangian: L = (m/2)(ẋ² + ẏ²) − V(x,y) + λ(x² + y² − R²).
- Better: use θ as coordinate (x = R cos θ, y = R sin θ). Reduced Lagrangian: L = (mR²/2)θ̇² − V(R cos θ, R sin θ). No constraint needed.

**GR connection**: The constraint in GR that the metric has unit normal (for the ADM decomposition) — the lapse function — plays the role of a Lagrange multiplier. The lapse function enforces the Hamiltonian constraint (Section 11.1.3).

---

## 15.1.5 The Action Principle in Field Theory

For a field φ(x^μ) on spacetime (a function of 4 coordinates), the Lagrangian becomes a **Lagrangian density** ℒ(φ, ∂_μφ, x^μ), and the action is:

$$S[\phi] = \int {\cal L}(\phi, \partial_\mu\phi) \, d^4x$$

The Euler-Lagrange equations for this action are the **field equations**:

$$\frac{\partial{\cal L}}{\partial\phi} - \partial_\mu\left(\frac{\partial{\cal L}}{\partial(\partial_\mu\phi)}\right) = 0$$

**Examples**:
- **Klein-Gordon field**: ℒ = −(1/2)(∂_μφ∂^μφ + m²φ²). Field equation: (□ + m²)φ = 0.
- **Electromagnetic field**: ℒ = −(1/4)F_{μν}F^{μν}. Field equation: ∂_μF^{μν} = 0 (vacuum Maxwell equations).
- **Einstein gravity**: ℒ_g = R/(16πG). Field equation: G_{μν} = 8πG T_{μν} (Einstein field equation).

The Einstein-Hilbert action is the field theory action for the metric tensor g_{μν} — the "field" is the metric, and the Euler-Lagrange equations for this action are the Einstein equations. This is the direct application of the variational principle of this section to GR.

---

## References

- Hamilton, W.R. (1834). "On a general method in dynamics." *Philosophical Transactions of the Royal Society*, 124, 247–308. [Hamilton's principle of stationary action in its first general form. One of the great papers in mathematical physics.]
- Euler, L. (1744). *Methodus inveniendi lineas curvas maximi minimive proprietate gaudentes.* Bousquet, Lausanne. [The founding work of the calculus of variations: Euler's equations for extremizing an integral. The Euler-Lagrange equations are here in their original form.]
- Lagrange, J.L. (1788). *Mécanique analytique.* Paris. [The Lagrangian formulation of mechanics: generalized coordinates, the E-L equations, and the treatment of constraints. The foundation of analytical mechanics.]
- Hilbert, D. (1915). "Die Grundlagen der Physik." *Nachrichten von der Gesellschaft der Wissenschaften zu Göttingen*, 395–407. [Hilbert derives the Einstein equations from the Einstein-Hilbert action principle — actually submitting the derivation days before Einstein's own derivation of the field equations.]
