# Chapter 10: Exercises

---

## Section 10.1: First-Order ODEs

**Exercise 10.1.1.** Solve the following IVPs and state the interval of existence.
(a) y' = y², y(0) = 1. Does the solution exist for all t > 0?
(b) y' = −y, y(0) = 3.
(c) y' + (cos x)y = cos x, y(0) = 2.
(d) y' = y^{1/3}, y(0) = 0. Find two solutions. Which hypothesis of Picard-Lindelöf fails?

**Exercise 10.1.2.** Prove: if f is C¹ on ℝ × ℝ, then the solution of y' = f(x, y), y(x₀) = y₀ exists and is unique on some interval (x₀ − h, x₀ + h). [Hint: construct the Picard iterates and use the Banach fixed-point theorem.]

**Exercise 10.1.3.** The first-order ODE for radial infall in Schwarzschild:
$$\frac{dr}{d\tau} = -\sqrt{\frac{r_s}{r}} \cdot c$$
(for a particle falling from rest at infinity, setting E = c).
(a) Separate variables and integrate to find τ(r) — the proper time to fall from radius r to r = 0.
(b) Show that the proper time to fall from r = r_s to r = 0 is finite.
(c) Coordinate time t(r) is given by dt/dr = (dt/dτ)/(dr/dτ). Show that t → ∞ as r → r_s (the crossing takes infinite coordinate time). Interpret.

---

## Section 10.2: Linear Second-Order ODEs

**Exercise 10.2.1.** For each ODE, find the general solution:
(a) y'' − 5y' + 6y = 0
(b) y'' + 4y' + 4y = 0 (repeated root)
(c) y'' + 9y = sin 3x (resonance case)
(d) y'' − y = eˣ (method of undetermined coefficients — also try variation of parameters)

**Exercise 10.2.2.** Verify Abel's theorem. For y'' + P(x)y' + Q(x)y = 0:
(a) Prove W'(y₁, y₂) = −P(x) W(y₁, y₂) directly from the Wronskian definition.
(b) Deduce that W(x) = W(x₀) exp(−∫_{x₀}^x P(t) dt).
(c) Use this to show that y₁ = x and y₂ = x ln x are linearly independent solutions of x²y'' − xy' + y = 0 on (0, ∞).

**Exercise 10.2.3** (Original — Jacobi equation). In a spacetime with constant positive curvature K (a sphere of radius 1/√K), the Jacobi equation for geodesic deviation is:
$$J'' + K J = 0$$
(where ' = d/ds is the derivative along the geodesic).
(a) Solve for J(s) given J(0) = 0 and J'(0) = 1.
(b) Find the first conjugate point (where J = 0 again after s = 0).
(c) What is the physical meaning? (Think about geodesics on a sphere.)
(d) For negative curvature K < 0, solve the Jacobi equation and show geodesics diverge exponentially. Relate to the stability of geodesics in de Sitter space.

**Exercise 10.2.4.** The damped oscillator y'' + 2γy' + ω₀²y = 0:
(a) Find all three cases of solutions (underdamped, critically damped, overdamped).
(b) Sketch the phase portraits (y, y') for each case.
(c) A black hole quasi-normal mode has frequency ω = ω_R − iω_I (complex). Show that this corresponds to the underdamped case with damping rate ω_I. What does the imaginary part of the frequency mean physically?

---

## Section 10.3: Systems of ODEs

**Exercise 10.3.1.** For the system ẋ = Ax where $A = \begin{pmatrix} 1 & 2 \\ 3 & 2 \end{pmatrix}$:
(a) Find the eigenvalues and classify the equilibrium.
(b) Find e^{At}.
(c) Solve the IVP with x(0) = (1, 0)ᵀ.

**Exercise 10.3.2.** Compute the matrix exponential e^{At} for:
(a) $A = \begin{pmatrix} 0 & 1 \\ -\omega^2 & 0 \end{pmatrix}$ (harmonic oscillator)
(b) $A = \begin{pmatrix} \lambda & 1 \\ 0 & \lambda \end{pmatrix}$ (Jordan block)
(c) $A = \begin{pmatrix} 0 & -\omega \\ \omega & 0 \end{pmatrix}$ (rotation)

**Exercise 10.3.3** (Phase portrait — Schwarzschild effective potential). The effective potential for radial geodesics in Schwarzschild is:
$$V_{\text{eff}}(r) = \left(1 - \frac{r_s}{r}\right)\left(1 + \frac{L^2}{r^2}\right)$$
(setting c = 1, choosing units with L as angular momentum per unit mass).
(a) Find the critical points of V_{eff}(r) as a function of r. (Set dV_{eff}/dr = 0.)
(b) Show that for L² > 3r_s², there are two critical points: a local maximum (unstable circular orbit) and local minimum (stable circular orbit).
(c) At L² = 3r_s², the two critical points merge: this is the ISCO. What is its radius?
(d) Sketch V_{eff}(r) for L = 2r_s, indicating all orbit types.

**Exercise 10.3.4.** Find a Lyapunov function for:
(a) ẋ = −x³ (show asymptotic stability of x* = 0)
(b) ẋ = −x + x³ (show instability: find a function that increases along solutions)
(c) The pendulum: ẋ = v, v̇ = −sin x. Show that E = (1/2)v² − cos x is a Lyapunov function for stability but not asymptotic stability of (x, v) = (0, 0).

---

## Section 10.4: Power Series Solutions

**Exercise 10.4.1.** Find two linearly independent power series solutions about x₀ = 0:
(a) y'' − xy = 0 (Airy equation)
(b) (1 − x²)y'' − 2xy' + 6y = 0 (Legendre, ℓ = 2 — verify P₂ is a solution)
(c) xy'' + y' + xy = 0 (Bessel, n = 0 — identify J₀)

**Exercise 10.4.2.** Apply the Frobenius method to find solutions near x = 0 for:
(a) x²y'' + xy' + (x² − 1/4)y = 0 (Bessel with n = 1/2). Show J_{1/2}(x) = √(2/πx) sin x.
(b) 2x²y'' − xy' + (1 + x)y = 0. Find the indicial equation and both Frobenius solutions.

**Exercise 10.4.3** (Gravitational applications). The gravitational potential outside an axisymmetric body is:
$$\Phi(r, \theta) = -\frac{GM}{r}\sum_{\ell=0}^\infty J_\ell \left(\frac{R}{r}\right)^\ell P_\ell(\cos\theta)$$
(a) The J₂ term (quadrupole moment) is the dominant deviation from spherical symmetry for Earth. Given J₂ = 1.083 × 10⁻³, write out the leading correction to the gravitational potential.
(b) Compute the corresponding correction to the orbital frequency for a satellite at radius r. This is the origin of the nodal precession of satellite orbits.
(c) GR adds an additional term −(GMr_s L²)/(r⁴ c²) to V_{eff}. Show this leads to a precessing ellipse (perihelion precession).

---

## Thought Experiments

**Thought Experiment 10.1: The Arrow of Time**

For a linear ODE ẋ = Ax, the solution is x(t) = e^{At}x₀. Replacing t → −t gives x(−t) = e^{−At}x₀.

(a) For which types of equilibria (stable node, unstable node, saddle, center) is the dynamics "reversible" in the sense that the time-reversed solution is also a solution of the same system?

(b) The geodesic equation in GR is time-reversible: if γ(τ) is a geodesic, so is γ(−τ). But physics inside a black hole is not reversible: once across the horizon, you cannot return. Explain the tension — how can a time-symmetric equation produce irreversible behavior?

(c) The Penrose singularity theorem says every geodesic entering a black hole is incomplete (terminates in finite proper time). How does this relate to blow-up in ODEs (Exercise 10.1.1(a))?

**Thought Experiment 10.2: Stability and Prediction**

A chaotic system (like the Lorenz attractor) has sensitive dependence on initial conditions: nearby trajectories diverge exponentially, with Lyapunov exponent λ > 0.

(a) In what sense is a chaotic system "solvable"? The ODE is well-posed (Picard-Lindelöf) — a unique solution exists. But prediction over long times is impossible. Reconcile these.

(b) Gravitational N-body systems (like the solar system) are weakly chaotic with positive Lyapunov exponents. Numerical integrations show the solar system is stable over ~5 billion years, but computations beyond ~100 million years diverge from reality. Does this mean the solar system "will" become unstable?

(c) In GR, the geodesic incompleteness of black hole spacetimes is proven without solving the geodesic equation explicitly — it follows from an energy condition and the Raychaudhuri equation (Unit XV). What does this suggest about the relationship between "existence" and "explicit solution" in physics?

---

## Laboratory Exercises

**Lab 10.1: Measuring the Effective Potential by Analogy**

An analog for the Schwarzschild effective potential can be built mechanically. Consider a ball bearing rolling on a surface shaped like V_{eff}(r).

**Procedure**: Machine or 3D-print a surface whose height is proportional to V_{eff}(r) = (1 − r_s/r)(1 + L²/r²) for chosen values of r_s and L. Roll a ball bearing on this surface.

**Observations**: (a) Find the circular orbit (the minimum of V_{eff}) — the ball circles at that radius. (b) Perturb the orbit slightly — does the ball execute nearly circular orbits (stable minimum) or escape (unstable)? (c) Relate the oscillation frequency of the perturbed orbit to V''_{eff} at the minimum.

**Note**: This analogy is imperfect — the ball's dynamics on the surface include 3D rolling effects — but the qualitative behavior (stable and unstable orbits, escape trajectories) is accurately reproduced.

**Lab 10.2: Observing Resonance**

The resonance of a driven oscillator y'' + 2γy' + ω₀²y = F₀cos(ωt) occurs when ω ≈ ω₀ and γ is small.

**Procedure**: Build a simple pendulum (length ≈ 1 m, so ω₀ ≈ 3.1 rad/s). Drive it with a small periodic push at varying frequencies ω. Measure the amplitude as a function of ω/ω₀.

**Measurements**: (a) Find the resonant frequency. (b) Measure the Q factor: Q = ω₀/(2γ). (c) Gravitational wave detectors are resonant systems: LIGO's arm cavities have very high Q. How does the Q factor affect the bandwidth (range of frequencies to which the detector is sensitive)? (d) Black hole quasi-normal modes have specific Q values. Can you design a resonant detector selective to a specific QNM?
