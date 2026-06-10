# Chapter 10: Important Concepts

---

**Initial Value Problem (IVP)**: The problem of finding a function y(x) satisfying a given ODE together with an initial condition y(x₀) = y₀. The Picard-Lindelöf theorem guarantees existence and uniqueness under a Lipschitz condition.

**Picard-Lindelöf Theorem**: If f is continuous and Lipschitz in y on a rectangle around (x₀, y₀), then the IVP y' = f(x, y), y(x₀) = y₀ has a unique solution on some interval containing x₀. The proof uses the Banach fixed-point theorem applied to the Picard iteration T(φ)(x) = y₀ + ∫ f(t, φ(t)) dt.

**Banach Fixed-Point Theorem**: A contraction mapping T: X → X on a complete metric space has a unique fixed point, approached by the iteration xₙ₊₁ = T(xₙ). The foundational tool for existence-uniqueness results throughout analysis, ODE theory, and PDE theory.

**Lipschitz Condition**: A function f(x, y) is Lipschitz in y if |f(x, y₁) − f(x, y₂)| ≤ K|y₁ − y₂|. This is the condition ensuring uniqueness in the Picard-Lindelöf theorem. It can fail for functions like f = y^{1/3} at y = 0, where the derivative blows up.

**Blow-Up (Finite-Time Blow-Up)**: A solution y(t) of an ODE that diverges (|y(t)| → ∞) as t approaches a finite time T < ∞. Example: y' = y², y(0) = 1 blows up at t = 1. In GR, blow-up in the geodesic equation corresponds to a spacetime singularity — an event where tidal forces become infinite and the geodesic cannot be extended.

**Solution Space (for Linear ODEs)**: The set of all solutions to a homogeneous linear ODE y^{(n)} + ... = 0 is a vector space of dimension n over ℝ. A basis for this space is a **fundamental system** of solutions. This is the link between linear algebra (vector spaces) and ODE theory.

**Wronskian**: W(y₁, y₂)(x) = y₁y₂' − y₂y₁'. Vanishes identically iff the solutions are linearly dependent; is never zero iff they are linearly independent (Abel's theorem). The Wronskian also appears in the formula for variation of parameters (the particular solution of an inhomogeneous ODE).

**Abel's Theorem**: For y'' + P(x)y' + Q(x)y = 0, the Wronskian satisfies W'(x) = −P(x)W(x), giving W(x) = W(x₀)exp(−∫P dt). In GR, the analogue is the Raychaudhuri equation, which governs the evolution of the expansion scalar of a congruence of geodesics.

**Fundamental Matrix**: The matrix Φ(t) whose columns are n linearly independent solutions of the linear system ẋ = A(t)x. The general solution is x(t) = Φ(t)c for some constant vector c.

**Matrix Exponential**: e^{At} = Σ (At)^k/k! — the solution of ẋ = Ax, x(0) = x₀ is x(t) = e^{At}x₀. Computable via diagonalization: if A = PDP⁻¹, then e^{At} = Pe^{Dt}P⁻¹. The matrix exponential is also the flow of the linear vector field F(x) = Ax.

**Phase Space**: For a system of ODEs ẋ = F(x) with x ∈ ℝⁿ, the phase space is ℝⁿ (or a manifold). The right-hand side F defines a **vector field** on phase space. Solutions are **integral curves** — curves everywhere tangent to F. The global structure of the set of all solutions is the **phase portrait**.

**Equilibrium (Fixed Point)**: A point x* where F(x*) = 0. At an equilibrium, the system "rests" (ẋ = 0, so x(t) = x* is a constant solution). All long-term behavior of trajectories is organized around the equilibria and their stable/unstable manifolds.

**Linearization**: Near an equilibrium x*, the nonlinear system ẋ = F(x) is approximated by ẏ = DF(x*)y where y = x − x*. The linearized system captures the local stability when DF(x*) has no purely imaginary eigenvalues (hyperbolic equilibrium).

**Stable/Unstable/Center Equilibrium**: Determined by eigenvalues of DF(x*). All negative real parts → stable (all trajectories approach x*). All positive → unstable. Mixed → saddle. Purely imaginary → center (need nonlinear analysis). In GR: stable circular orbits in Schwarzschild have stable equilibria in the effective potential; the ISCO is the degenerate case.

**Lyapunov Stability**: An equilibrium x* is Lyapunov stable if solutions starting near x* remain near x* for all future time. Asymptotically stable if they also converge to x*. Lyapunov's direct method: find a positive definite function V with V̇ ≤ 0 to prove stability without solving the ODE.

**Hartman-Grobman Theorem**: Near a hyperbolic equilibrium, the nonlinear phase portrait is topologically equivalent to the linearized one. This justifies classifying equilibria by the eigenvalues of the linearization.

**Killing Vector and First Integrals**: A Killing vector field ξ^μ on a spacetime satisfies ∇_(μ)ξ_(ν) = 0. For any geodesic with 4-velocity u^μ, the quantity ξ_μ u^μ is constant along the geodesic. Each Killing vector provides a conserved quantity (energy for time translation, angular momentum for rotational symmetry), reducing the geodesic equation.

**Geodesic Spray**: The vector field G on the tangent bundle TM defined by the geodesic equation, rewritten as a first-order system: d(x^μ)/dτ = u^μ, d(u^μ)/dτ = −Γ^μ_{νρ} u^ν u^ρ. Geodesics are integral curves of this vector field on TM.

**Effective Potential**: A function V_{eff}(r) such that the energy conservation law for radial motion takes the form (dr/dτ)² = E² − V_{eff}(r). The turning points of the motion (where dr/dτ = 0) are the roots of E² = V_{eff}(r). Local minima of V_{eff} correspond to stable circular orbits; local maxima to unstable ones.

**Regular Singular Point**: A singular point x₀ of an ODE where the singularity is "mild" — (x−x₀)P(x) and (x−x₀)²Q(x) remain analytic. Frobenius series solutions exist at regular singular points. The Regge-Wheeler equation for black hole perturbations has regular singular points at the horizon and irregular singular point at infinity.

**Indicial Equation**: The algebraic equation r(r−1) + p₀r + q₀ = 0 arising in the Frobenius method, whose roots r₁, r₂ determine the leading power behavior of the two solutions near the singular point.

**Legendre Polynomials**: The solutions P_ℓ(cos θ) of the Legendre equation arising from separation of variables in spherical coordinates. Orthogonal polynomials of degree ℓ, satisfying ∫_{-1}^1 P_ℓ P_{ℓ'} du = 2/(2ℓ+1) δ_{ℓℓ'}. They provide the angular basis for gravitational multipole expansions.

**Spherical Harmonics Y^ℓ_m(θ,φ)**: Complete orthonormal basis for L²(S²). Every angular function on the sphere expands in Y^ℓ_m. The gravitational wave polarization tensor, the black hole quasi-normal modes, and the CMB temperature fluctuations are all expanded in spherical harmonics.
