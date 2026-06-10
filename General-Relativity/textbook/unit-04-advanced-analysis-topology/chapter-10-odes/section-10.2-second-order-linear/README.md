# Section 10.2: Linear Second-Order ODEs

---

## Section Introduction

Second-order linear ODEs arise everywhere in physics: Newton's second law (F = ma), the harmonic oscillator, the Schrödinger equation (as a 1D eigenvalue problem), the wave equation in spherical harmonics, and the geodesic equation in GR. The theory of these equations is pure linear algebra applied to function spaces.

The key insight: the set of all solutions to a homogeneous linear ODE is a **vector space** of dimension equal to the order of the equation. A basis of this space is called a **fundamental system of solutions**. The general solution is a linear combination of fundamental solutions.

---

## 10.2.1 The General Theory

Consider the second-order linear ODE:

$$y'' + P(x)y' + Q(x)y = R(x)$$

**Homogeneous case** (R = 0): y'' + P(x)y' + Q(x)y = 0.

**Theorem** (Existence and Uniqueness for Linear ODEs): If P, Q, R are continuous on (a, b), then for any x₀ ∈ (a, b) and y₀, y₀' ∈ ℝ, the IVP has a **unique** solution on the entire interval (a, b) (not just locally).

The global existence (unlike the nonlinear case) follows because the linear structure prevents blow-up: a linear combination of solutions is a solution, so solutions cannot blow up without destroying linearity.

**The solution space**: The set of all solutions to the homogeneous equation is a 2-dimensional vector space. Two solutions y₁, y₂ form a **fundamental system** if they are linearly independent. The general solution is y = c₁y₁ + c₂y₂.

**The Wronskian**: The **Wronskian** of two solutions y₁, y₂ is:

$$W(y_1, y_2)(x) = \begin{vmatrix} y_1 & y_2 \\ y_1' & y_2' \end{vmatrix} = y_1 y_2' - y_2 y_1'$$

**Abel's theorem**: W'(x) = −P(x) W(x), so $W(x) = W(x_0) \exp\left(-\int_{x_0}^x P(t) dt\right)$.

**Consequence**: W is either identically 0 (y₁, y₂ are linearly dependent) or never 0 on (a, b). Linear independence is an all-or-nothing property for solutions.

---

## 10.2.2 Constant Coefficient Equations

For y'' + py' + qy = 0 (p, q constant), try y = eˡˣ. The **characteristic equation** is λ² + pλ + q = 0.

**Cases**:
- **Two distinct real roots** λ₁ ≠ λ₂: y = c₁e^{λ₁x} + c₂e^{λ₂x}.
- **Repeated root** λ₁ = λ₂ = λ: y = (c₁ + c₂x)eˡˣ.
- **Complex conjugate roots** λ = α ± iβ: y = e^{αx}(c₁ cos βx + c₂ sin βx).

**The harmonic oscillator**: y'' + ω²y = 0. Roots ±iω. Solution: y = A cos ωt + B sin ωt — simple harmonic motion with angular frequency ω.

**Damped oscillator**: y'' + 2γy' + ω₀²y = 0. Underdamped (γ < ω₀): y = Ae^{−γt} cos(ω₁t + φ), where ω₁ = √(ω₀² − γ²). Critical (γ = ω₀): y = (c₁ + c₂t)e^{−γt}. Overdamped (γ > ω₀): exponential decay.

**Connection to GR**: Quasi-normal modes of black holes satisfy a damped oscillator equation with complex frequency ω = ω_R − iω_I. The real part ω_R is the oscillation frequency; the imaginary part ω_I is the damping rate. These are the "ringing frequencies" of a black hole after a perturbation, analogous to the normal modes of a bell. They are characteristic of the black hole's mass and spin, not the perturbation.

---

## 10.2.3 Variation of Parameters

For the inhomogeneous equation y'' + P(x)y' + Q(x)y = R(x), given a fundamental system {y₁, y₂} for the homogeneous equation, the particular solution is:

$$y_p(x) = -y_1(x) \int \frac{y_2(x) R(x)}{W(x)} dx + y_2(x) \int \frac{y_1(x) R(x)}{W(x)} dx$$

*Derivation*: Try y_p = u₁(x)y₁ + u₂(x)y₂. Impose u₁'y₁ + u₂'y₂ = 0 (one equation). Substituting into the ODE gives u₁'y₁' + u₂'y₂' = R (second equation). Solving the 2×2 system by Cramer's rule gives the formulas above.

**Green's function**: The formula can be written as $y_p(x) = \int G(x, t) R(t) dt$, where G is the **Green's function** of the differential operator. This is the template for all Green's function methods in PDEs and quantum field theory.

---

## 10.2.4 The Jacobi Equation

In GR, the **Jacobi equation** (equation of geodesic deviation) governs how nearby geodesics diverge:

$$\frac{D^2 J^\mu}{d\tau^2} + R^\mu_{\ \nu\rho\sigma} u^\nu J^\rho u^\sigma = 0$$

where J^μ is the **Jacobi field** (the deviation vector between nearby geodesics), u^ν = dx^ν/dτ is the 4-velocity, and R^μ_{νρσ} is the Riemann curvature tensor.

In a locally flat region (R = 0): D²J/dτ² = 0 — geodesics separate linearly (no focusing or defocusing).

With curvature: if R_{0i0j} < 0 (focusing — positive curvature in the time-space plane), geodesics converge. If R_{0i0j} > 0, they diverge.

The Jacobi equation is a **linear second-order ODE** for J — analogous to the harmonic oscillator with R playing the role of the spring constant. Positive curvature is like a negative spring constant — unstable. This analogy is made precise in the Raychaudhuri equation (Chapter 51).

**Significance**: The Jacobi equation determines:
- Whether geodesics have conjugate points (where J = 0 despite J'≠ 0 initially) — the GR analogue of focusing.
- Whether a geodesic is locally length-minimizing (no conjugate points in the interior) — crucial for the singularity theorems.

---

## References

- Arnold, V.I. (1992). *Ordinary Differential Equations*, 3rd ed. Springer. [Chapters 3–4.]
- Boyce, W.E. and DiPrima, R.C. (2017). *Elementary Differential Equations*, 11th ed. Wiley. [The standard undergraduate ODE text; comprehensive and clear.]
- Misner, C.W., Thorne, K.S., and Wheeler, J.A. (1973). *Gravitation*. W.H. Freeman. [§25.2 on geodesic deviation and the Jacobi equation; Box 25.1 for the analogy with harmonic oscillators.]
