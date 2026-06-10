# Section 11.1: Classification of PDEs and Well-Posedness

---

## Section Introduction

A **partial differential equation** involves an unknown function of several variables and its partial derivatives. Unlike ODEs, where the solution is a function of one variable and existence-uniqueness theory is well-developed, PDEs come in fundamentally different types that require different analysis and admit qualitatively different solutions.

The three fundamental types — **elliptic**, **parabolic**, and **hyperbolic** — correspond roughly to three types of physical problems: equilibrium (elliptic), diffusion (parabolic), and wave propagation (hyperbolic). In GR, the wave character of the field equations — the fact that spacetime curvature propagates at speed c — is precisely the statement that the Einstein equations are **hyperbolic** in appropriate gauges.

The concept of **well-posedness** (Hadamard, 1902) asks: does a PDE problem have a unique solution that depends continuously on the initial data? Well-posedness is not a mathematical nicety — it is a physical requirement. An ill-posed problem would mean that infinitesimally different physical situations lead to arbitrarily different outcomes, making prediction impossible.

---

## 11.1.1 What is a Partial Differential Equation?

A **PDE** of order k for an unknown function u: Ω ⊂ ℝⁿ → ℝ is a relation:

$$F(x, u, \partial u, \partial^2 u, \ldots, \partial^k u) = 0$$

involving the independent variables x = (x¹, ..., xⁿ), the function value u(x), and its partial derivatives up to order k.

**Examples** (each with its physical origin):
- **Laplace's equation**: ∂²u/∂x² + ∂²u/∂y² = 0, or in n dimensions: ∇²u = 0. Equilibrium temperature, electrostatic potential in vacuum, Newtonian gravitational potential outside sources.
- **Poisson's equation**: ∇²u = f(x). Gravitational potential with sources: ∇²Φ = 4πGρ.
- **Heat equation**: ∂u/∂t = κ ∇²u. Temperature distribution in a conducting medium; also appears in quantum mechanics as the Schrödinger equation in imaginary time.
- **Wave equation**: ∂²u/∂t² = c² ∇²u. Propagating waves: sound, light, gravitational waves.
- **Schrödinger equation**: iℏ ∂ψ/∂t = −ℏ²/(2m) ∇²ψ + Vψ. Quantum mechanical amplitude.
- **Einstein equations**: G_{μν} = 8πG/c⁴ T_{μν}. The field equations of GR — a system of ten coupled, nonlinear PDEs.

**Linear vs. nonlinear**: A PDE is linear if F is linear in u and its derivatives. Linear PDEs have the superposition principle: sums of solutions are solutions. The Einstein equations are nonlinear: gravity gravitates.

**Order**: The order of a PDE is the highest derivative that appears. The wave equation is second order; the Euler-Bernoulli beam equation is fourth order.

---

## 11.1.2 The Classification of Second-Order Linear PDEs

For a second-order linear PDE in two variables:

$$A\frac{\partial^2 u}{\partial x^2} + 2B\frac{\partial^2 u}{\partial x \partial y} + C\frac{\partial^2 u}{\partial y^2} + \text{lower order terms} = 0$$

the **discriminant** B² − AC determines the type:
- **Elliptic**: B² − AC < 0. Example: Laplace's equation (A = C = 1, B = 0: B² − AC = −1 < 0).
- **Parabolic**: B² − AC = 0. Example: Heat equation (one direction is time, A = 1, B = C = 0: B² − AC = 0).
- **Hyperbolic**: B² − AC > 0. Example: Wave equation (A = 1, C = −1, B = 0: B² − AC = 0 + 1 = 1 > 0).

The type is invariant under smooth coordinate changes — it is a geometric property.

**Characteristics**: For a hyperbolic PDE, the **characteristic curves** are the directions along which the equation degenerates — the "propagation directions." For the wave equation u_{tt} = c²u_{xx}, the characteristics are the lines x ± ct = const — the light cone directions. Disturbances propagate along characteristics.

**In GR**: The linearized Einstein equations (in appropriate gauge) are hyperbolic PDEs. The characteristics are null geodesics — the light cone of each event. Information propagates at most at speed c. The **causal structure** of spacetime — which events can influence which others — is encoded in the characteristic structure of the Einstein equations.

---

## 11.1.3 Well-Posedness (Hadamard)

**Definition** (Hadamard, 1902): A PDE problem (equation + initial/boundary conditions) is **well-posed** if:
1. **Existence**: A solution exists.
2. **Uniqueness**: The solution is unique.
3. **Continuous dependence**: The solution depends continuously on the initial data (small perturbations to the data cause small perturbations to the solution).

Well-posedness is the minimal requirement for a physically meaningful problem. Without existence, the model predicts no physical state. Without uniqueness, the model cannot predict which state evolves from a given initial configuration. Without continuous dependence, the model predicts wildly different outcomes for barely distinguishable initial conditions — making it useless for physics.

**Natural problems by type**:
- **Elliptic** (Laplace, Poisson): Well-posed as a **boundary value problem** (BVP) — specify u on the boundary ∂Ω, solve for u in the interior Ω. The Dirichlet problem: given f: ∂Ω → ℝ, find u: Ω → ℝ with ∇²u = 0 in Ω and u|_{∂Ω} = f.
- **Parabolic** (heat): Well-posed as an **initial value problem** (IVP) in one time direction — give u(x, 0) = u₀(x), get u(x, t) for t > 0. The heat equation has only one natural time direction (forward).
- **Hyperbolic** (wave): Well-posed as an **IVP** with data on a spacelike hypersurface: give u(x, 0) = u₀(x) and ∂_t u(x, 0) = u₁(x), get u(x, t) for t > 0 (or t < 0 — both directions in time are equally natural).

**Ill-posed examples**:
- The Laplace equation with Cauchy data on a curve (instead of boundary data) is ill-posed: the Hadamard example (1902) shows that small, high-frequency oscillations in the data produce solutions that blow up instantaneously.
- Backward heat equation (evolving backward in time) is ill-posed: high-frequency modes grow exponentially.

**GR and well-posedness**: The initial value formulation of GR (Arnowitt-Deser-Misner, 1959; Choquet-Bruhat, 1952) establishes that the Einstein equations, in harmonic gauge, form a well-posed hyperbolic IVP. Given initial data (a Riemannian metric and second fundamental form on a spacelike hypersurface satisfying the constraint equations), there exists a unique maximal development — a spacetime that extends until a singularity is reached. This is the mathematical basis for numerical relativity.

[Choquet-Bruhat, Y. (1952). "Théorème d'existence pour certains systèmes d'équations aux dérivées partielles non linéaires." *Acta Mathematica* 88, 141–225.]

---

## 11.1.4 Initial and Boundary Conditions

The type of PDE determines which additional data makes the problem well-posed.

**For the wave equation** u_{tt} = c²∇²u on ℝ³ × [0,∞):
- **Cauchy data**: specify u(x, 0) and u_t(x, 0) on the initial surface t = 0.
- The solution is determined in the **domain of dependence**: u(x₀, t₀) depends only on Cauchy data in the ball |x − x₀| ≤ ct₀ (by Huygens' principle in 3D).
- The solution is determined in the **domain of influence**: the Cauchy data at x₀ influences the solution only for |x − x₀| ≤ c|t| (signals travel at most at speed c).

**For the heat equation** u_t = κ∇²u on ℝⁿ × [0,∞):
- Specify u(x, 0) = u₀(x) (initial temperature distribution).
- The heat equation is **smoothing**: even discontinuous initial data produces smooth solutions for t > 0. (This is in stark contrast to the wave equation, which propagates discontinuities.)
- The solution for all t > 0 is determined by the initial data; no boundary conditions at infinity are needed if u₀ decays sufficiently.

**For Laplace's equation** ∇²u = 0 on a bounded domain Ω:
- **Dirichlet problem**: specify u on ∂Ω.
- **Neumann problem**: specify ∂u/∂n (normal derivative) on ∂Ω.
- **Mixed**: Dirichlet on part of ∂Ω, Neumann on the rest.
- Exactly one of each type is appropriate; overdetermining (specifying both u and ∂u/∂n — Cauchy data for an elliptic equation) is ill-posed.

**GR**: The choice of initial data for the Einstein equations must satisfy the **constraint equations** — a system of four PDEs relating the initial metric and extrinsic curvature. These are elliptic equations (analogues of the Gauss and Codazzi equations). The time evolution is hyperbolic. This decomposition — elliptic constraints + hyperbolic evolution — is the ADM formalism (Unit XIII).

---

## References

- Hadamard, J. (1902). "Sur les problèmes aux dérivées partielles et leur signification physique." *Princeton University Bulletin*, 49–52. [Introduces well-posedness; the Hadamard example of an ill-posed Cauchy problem for the Laplace equation.]
- Evans, L.C. (2010). *Partial Differential Equations*, 2nd ed. American Mathematical Society. [The standard graduate PDE reference. Chapter 1: four important PDEs; Chapter 2: Laplace equation; Chapters 2–4: classification and well-posedness. Rigorous and complete.]
- Choquet-Bruhat, Y. (1952). "Théorème d'existence pour certains systèmes d'équations aux dérivées partielles non linéaires." *Acta Mathematica* 88, 141–225. [The foundational paper establishing well-posedness of the Einstein equations as an IVP — the first existence theorem for GR.]
- Arnowitt, R., Deser, S., and Misner, C.W. (1962). "The dynamics of general relativity." In L. Witten, ed., *Gravitation: An Introduction to Current Research.* Wiley. Reprinted in *General Relativity and Gravitation* 40 (2008), 1997–2027. [The ADM formalism: 3+1 decomposition of the Einstein equations as an IVP.]
