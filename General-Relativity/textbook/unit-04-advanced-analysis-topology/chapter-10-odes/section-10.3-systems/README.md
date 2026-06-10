# Section 10.3: Systems of ODEs, Phase Portraits, and the Matrix Exponential

---

## Section Introduction

A single second-order ODE is equivalent to a system of two first-order ODEs. This reformulation — writing x'' = f(x, x', t) as the system ẋ = v, v̇ = f(x, v, t) — is not merely a bookkeeping trick. It reveals the geometric structure of solutions: each state (x, v) is a point in **phase space**, and the ODE defines a vector field on that space. Solutions are **integral curves** of this vector field — curves that are everywhere tangent to it.

This geometric picture is essential for GR. The geodesic equation in GR is a second-order ODE for the worldline x^μ(τ). Rewritten as a first-order system (x^μ, u^μ = ẋ^μ), it defines a vector field on the **cotangent bundle** T*M. The study of this vector field — its equilibria, stability, and long-term behavior — is the study of particle motion in curved spacetime.

The **matrix exponential** provides exact solutions when the system is linear. It is the bridge between linear algebra (eigenvalues, eigenvectors) and ODEs (solutions, stability), and it is the analogue of the ordinary exponential e^{at} for matrix-valued problems.

---

## 10.3.1 Systems of First-Order ODEs

A **system** of n first-order ODEs for x = (x¹, ..., xⁿ): (−ε, ε) → ℝⁿ is:

$$\dot{x}(t) = F(t, x(t)), \quad x(t_0) = x_0$$

where F: ℝ × ℝⁿ → ℝⁿ and the dot denotes d/dt.

The Picard-Lindelöf theorem (Section 10.1) extends to systems: if F is continuous and Lipschitz in x (uniformly in t), then the IVP has a unique solution in some interval around t₀.

**Reduction of order**: Every nth-order ODE can be written as a first-order system in ℝⁿ. For y^{(n)} = F(t, y, y', ..., y^{(n−1)}), set x¹ = y, x² = y', ..., xⁿ = y^{(n−1)}. Then:

$$\dot{x}^1 = x^2, \quad \dot{x}^2 = x^3, \quad \ldots, \quad \dot{x}^n = F(t, x^1, \ldots, x^n)$$

This is a first-order system in n unknowns.

**Phase space**: For an autonomous system ẋ = F(x) (F does not depend on t explicitly), the phase space is ℝⁿ (or a more general manifold). The right-hand side F defines a **vector field**: at each point x, the vector F(x) gives the "velocity" of the state. **Solutions** are curves x(t) such that ẋ(t) = F(x(t)) — the curve is always tangent to the vector field.

This geometric picture applies directly to GR:
- The geodesic equation $\ddot{x}^\mu + \Gamma^\mu_{\nu\rho} \dot{x}^\nu \dot{x}^\rho = 0$ defines a vector field on TM (the tangent bundle).
- A geodesic is an integral curve of this vector field.
- The Jacobi equation (geodesic deviation) governs how nearby integral curves diverge — which is exactly the stability analysis of the next section.

---

## 10.3.2 Linear Systems and the Matrix Exponential

For the **linear** system ẋ = Ax (A an n×n constant matrix), the solution is:

$$x(t) = e^{At} x_0$$

where the **matrix exponential** is defined by its Taylor series:

$$e^{At} = I + At + \frac{(At)^2}{2!} + \frac{(At)^3}{3!} + \cdots = \sum_{k=0}^\infty \frac{(At)^k}{k!}$$

**Convergence**: The series converges absolutely for all t (since ||Aᵏtᵏ/k!|| ≤ (||A||·|t|)^k/k!, and the scalar series converges), so e^{At} is well-defined for any matrix A and any t.

**Verification**: d/dt(e^{At}) = A e^{At} (differentiate term by term) and e^{A·0} = I. So x(t) = e^{At} x₀ satisfies ẋ = Ax, x(0) = x₀. ✓

**Computing the matrix exponential**: If A = PDP⁻¹ (diagonalizable), then:

$$e^{At} = P e^{Dt} P^{-1}, \quad e^{Dt} = \text{diag}(e^{\lambda_1 t}, \ldots, e^{\lambda_n t})$$

For A with repeated eigenvalues (Jordan form A = PJP⁻¹): use e^{Jt} and the nilpotent structure.

**Example**: The harmonic oscillator ẍ + ω²x = 0 written as ẋ = v, v̇ = −ω²x:

$$A = \begin{pmatrix} 0 & 1 \\ -\omega^2 & 0 \end{pmatrix}$$

Eigenvalues: λ = ±iω. Eigenvectors: (1, ±iω)ᵀ.

$$e^{At} = \begin{pmatrix} \cos\omega t & \frac{1}{\omega}\sin\omega t \\ -\omega\sin\omega t & \cos\omega t \end{pmatrix}$$

The solution is x(t) = x₀ cos ωt + (v₀/ω) sin ωt. This matches Section 10.2. ✓

---

## 10.3.3 Phase Portraits and Equilibria

For an autonomous system ẋ = F(x), an **equilibrium** (or fixed point) is a point x* where F(x*) = 0. At an equilibrium, ẋ = 0, so x(t) = x* is a constant solution.

**Linearization**: Near an equilibrium x*, let y = x − x* (small displacement). Then:

$$\dot{y} = F(x^* + y) \approx DF(x^*) \cdot y$$

where DF(x*) is the Jacobian of F at x*. The **linearized system** ẏ = Ay (A = DF(x*)) governs the local dynamics.

**Stability classification** (2D): For a linear system ẋ = Ax in 2D, the eigenvalues λ₁, λ₂ of A determine the phase portrait:
- **Stable node** (λ₁ < λ₂ < 0): all trajectories → x* as t → ∞.
- **Unstable node** (0 < λ₁ < λ₂): all trajectories → ∞.
- **Saddle** (λ₁ < 0 < λ₂): trajectories approach along one eigendirection, flee along the other.
- **Stable spiral** (λ = α ± iβ, α < 0): trajectories spiral inward.
- **Unstable spiral** (α > 0): spiral outward.
- **Center** (λ = ±iβ, α = 0): closed orbits (conservative systems).

**Hartman-Grobman theorem**: If all eigenvalues of DF(x*) have non-zero real part (x* is **hyperbolic**), then the nonlinear system near x* is topologically equivalent (homeomorphic) to the linearized system. The phase portrait near a hyperbolic equilibrium looks like its linearization.

**GR application**: The geodesic equation in a Schwarzschild black hole exterior. Setting up the effective potential V_{eff}(r) for circular orbits: circular orbits occur at r satisfying V'_{eff}(r) = 0.
- For r > 3r_s (the innermost stable circular orbit, ISCO), these are **stable** (center-type equilibria in the (r, dr/dτ) phase plane).
- For r_s < r < 3r_s, the circular orbits are **unstable** (saddle points). A slight perturbation will cause the particle to spiral in.
- The ISCO at r = 3r_s (6GM/c² in conventional units) is the boundary: the outermost unstable circular orbit. It is a crucial observational quantity for black holes.

---

## 10.3.4 Stability and Lyapunov Functions

For nonlinear systems, eigenvalues of the linearization detect local stability but not global behavior. **Lyapunov's method** provides a global tool.

**Definition**: A **Lyapunov function** for ẋ = F(x) near an equilibrium x* is a function V: U → ℝ (U open, x* ∈ U) satisfying:
- V(x*) = 0 and V(x) > 0 for x ≠ x* in U (positive definite near x*).
- V̇(x(t)) = ∇V · F(x) ≤ 0 along solutions (V decreases along trajectories).

**Lyapunov's theorem**: If a Lyapunov function exists, x* is **stable**. If V̇ < 0 (strict inequality), x* is **asymptotically stable** (x(t) → x* as t → ∞).

**Intuition**: V is like a "generalized energy" that decreases along trajectories. If V is bounded below (near x*) and always decreasing, trajectories must converge to the minimum — the equilibrium.

**Example**: For ẋ = −x, the function V(x) = x² satisfies V̇ = 2x ẋ = −2x² ≤ 0, with equality only at x* = 0. So x* = 0 is asymptotically stable. This matches the explicit solution x(t) = x₀e^{-t}.

**GR application**: The stability of spacetime solutions is studied using Lyapunov methods adapted to infinite-dimensional PDE systems. The "energy" of a perturbation around a stable solution (like a static star or a Kerr black hole) is the Lyapunov functional; if it is positive definite and decreasing in time, the solution is stable.

---

## 10.3.5 The Geodesic Equation as a Flow

The geodesic equation $\ddot{x}^\mu + \Gamma^\mu_{\nu\rho} \dot{x}^\nu \dot{x}^\rho = 0$ can be written as a first-order system on TM (tangent bundle):

$$\frac{dx^\mu}{d\tau} = u^\mu, \quad \frac{du^\mu}{d\tau} = -\Gamma^\mu_{\nu\rho} u^\nu u^\rho$$

This defines a vector field G on TM, the **geodesic spray**. The integral curves of G, projected down to M, are the geodesics.

**Conservation law**: For a spacetime with a Killing vector field ξ^μ (a vector field satisfying ∇_{(μ}ξ_{ν)} = 0), the quantity ξ_μ u^μ is constant along geodesics:

$$\frac{d}{d\tau}(\xi_\mu u^\mu) = 0$$

This is a **first integral** of the geodesic equation — the ODE for geodesics reduces to a lower-order problem when Killing vectors are present.

- For Schwarzschild: ξ^μ = (∂/∂t)^μ (time translation Killing vector) gives E = −(1 − r_s/r) ṫ (energy per unit mass). The rotational Killing vectors give angular momentum L = r² φ̇.
- These two first integrals reduce the 4D geodesic equation to a 1D first-order ODE for r(τ) — the effective potential equation:

$$\left(\frac{dr}{d\tau}\right)^2 = E^2 - \left(1 - \frac{r_s}{r}\right)\left(c^2 + \frac{L^2}{r^2}\right) \equiv E^2 - V_{\text{eff}}(r)$$

The phase portrait of this equation determines all orbital types: escape, bound orbits, plunging orbits, circular orbits.

---

## References

- Arnold, V.I. (1992). *Ordinary Differential Equations*, 3rd ed. Springer. [Chapters 5–7 on systems of ODEs, phase portraits, and the classification of equilibria. The geometric viewpoint throughout.]
- Hirsch, M.W., Smale, S., and Devaney, R.L. (2013). *Differential Equations, Dynamical Systems, and an Introduction to Chaos*, 3rd ed. Academic Press. [Comprehensive treatment of the qualitative theory of ODEs; phase portraits, Lyapunov stability, and chaos.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation.* W.H. Freeman. [§25.5 on the effective potential for geodesics in Schwarzschild; §33 on circular orbits and ISCO.]
- Lyapunov, A.M. (1892). "The general problem of the stability of motion." Reprinted in *International Journal of Control* 55 (1992), 531–534. [Lyapunov's foundational work on stability theory.]
