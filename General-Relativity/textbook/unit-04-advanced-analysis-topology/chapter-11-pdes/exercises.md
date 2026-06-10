# Chapter 11: Exercises

---

## Section 11.1: Classification and Well-Posedness

**Exercise 11.1.1.** Classify each PDE as elliptic, parabolic, or hyperbolic.
(a) u_{xx} + 4u_{xy} + 4u_{yy} = 0
(b) u_{xx} − 2u_{xy} + u_{yy} = 0
(c) u_{xx} + u_{yy} + u_{zz} = u_t
(d) u_{tt} = c²(u_{xx} + u_{yy} + u_{zz})
(e) u_{tt} − u_{xx} + u = 0 (Klein-Gordon equation)
(f) iu_t = −u_{xx}/2 + V(x)u (Schrödinger, is it hyperbolic/parabolic/elliptic?)

**Exercise 11.1.2.** The Hadamard example of ill-posedness. Consider the Cauchy problem for Laplace's equation on the strip {y > 0}:
$$u_{xx} + u_{yy} = 0, \quad u(x, 0) = 0, \quad u_y(x, 0) = \epsilon \sin(nx)/n$$
(a) Verify that u_n(x, y) = (ε/n²) sin(nx) sinh(ny) is a solution.
(b) As n → ∞, the initial data u_y(x, 0) = ε sin(nx)/n → 0 uniformly. What happens to the solution u_n(x, y) for any fixed y > 0?
(c) Explain why this shows that Cauchy data for the Laplace equation gives an ill-posed problem: the solution does not depend continuously on the data.

**Exercise 11.1.3.** The ADM constraint equations. In GR, initial data (Σ, h, K) — a Riemannian 3-metric h on a hypersurface Σ and an extrinsic curvature K — must satisfy:
$$R - |K|^2 + (\text{tr} K)^2 = 16\pi G \rho \quad \text{(Hamiltonian constraint)}$$
$$D_j K^j_{\ i} - D_i (\text{tr} K) = 8\pi G J_i \quad \text{(momentum constraint)}$$
(a) Count the degrees of freedom: how many free functions are in (h, K)? How many constraint equations are there? How many "freely specifiable" functions remain?
(b) These constraints are elliptic equations for the "conformal factor" and "extrinsic curvature trace" given the freely specifiable data. Explain why elliptic equations are the natural type for constraints (as opposed to hyperbolic equations, which would be the evolution equations).

---

## Section 11.2: The Wave Equation

**Exercise 11.2.1.** Using d'Alembert's formula, solve the wave equation u_{tt} = c²u_{xx} with:
(a) u(x, 0) = sin x, u_t(x, 0) = 0.
(b) u(x, 0) = 0, u_t(x, 0) = cos x.
(c) u(x, 0) = e^{-x²}, u_t(x, 0) = 0. Describe the solution qualitatively for large t.

**Exercise 11.2.2.** Energy for the wave equation. For u_{tt} = c²u_{xx} on ℝ with smooth initial data:
(a) Define E(t) = (1/2)∫(u_t² + c²u_x²)dx. Show dE/dt = 0 (energy is conserved).
(b) Use energy conservation to prove uniqueness: if u and v are two solutions with the same initial data, then u = v.
(c) Show that the support of u(·, t) is contained in the "light cone" of the support of the initial data.

**Exercise 11.2.3** (Original — gravitational wave dispersion constraint). LIGO observed the gravitational wave signal GW150914 arriving simultaneously at two detectors separated by 3002 km, with a time delay of 6.9 ms — consistent with travel at c. The signal had frequency content from 35 to 250 Hz.
(a) If gravitons had a small mass m_g, their dispersion relation would be ω² = c²k² + (m_g c²/ℏ)². Show that the group velocity v_g = dω/dk would then be less than c.
(b) Show that a massive graviton would cause higher-frequency components to travel faster than lower-frequency ones. Is this consistent with the LIGO observation?
(c) Using the travel distance D ≈ 1.3 Gpc and the maximum measurable frequency difference, estimate the constraint on m_g.

**Exercise 11.2.4.** Huygens' principle in 2D vs. 3D.
(a) The solution in 3D involves an integral only over the sphere |y−x| = ct. The solution in 2D involves an integral over the disk |y−x| ≤ ct. Why is this?
(b) If you call your friend on the phone (3D sound), do you hear the beginning and end of each word clearly? What about ripples on a pond (effectively 2D)?
(c) In 3+1 dimensions, a "light pulse" (a flash at t = 0) produces a sharp spherical wavefront that passes and leaves silence. What would communication be like in 1+1 or 2+1 dimensional spacetimes?

---

## Section 11.3: The Heat Equation

**Exercise 11.3.1.** Show that the heat kernel K(x, t) = (4πκt)^{-n/2} exp(−|x|²/4κt) satisfies:
(a) ∂_t K = κ∇²K for t > 0.
(b) ∫_{ℝⁿ} K(x, t) dx = 1 for all t > 0.
(c) K(x, t) → δ(x) as t → 0⁺ in the distributional sense. [Hint: test against a smooth function φ and use the substitution y = x/√(κt).]

**Exercise 11.3.2.** The maximum principle for the heat equation.
(a) State and prove the maximum principle: the maximum of a solution u of u_t = κu_{xx} on [0, L] × [0, T] is attained on the parabolic boundary.
(b) Use the maximum principle to prove uniqueness for the heat equation on [0, L] × [0, T] with Dirichlet boundary conditions.
(c) Use the maximum principle to prove the comparison principle: if u₀(x) ≤ v₀(x) and the boundary conditions for u are ≤ those for v, then u(x, t) ≤ v(x, t) for all t > 0.

**Exercise 11.3.3** (Original — Ricci flow analogy). Hamilton's Ricci flow equation ∂g_{ij}/∂t = −2R_{ij} is the "heat equation for the metric."
(a) For a metric on the 2-sphere g = r(t)² (dθ² + sin²θ dφ²) (a round sphere of radius r(t)), compute R_{ij} and write the Ricci flow equation as an ODE for r(t).
(b) Solve the ODE. What happens to the sphere? At what time T does the solution break down?
(c) For a flat metric (R_{ij} = 0), Ricci flow doesn't change the metric. What is the analogue for the heat equation?
(d) Perelman showed that Ricci flow with surgery "uniformizes" 3-manifolds. By analogy with the heat equation, explain what "uniformizing" means and why it requires surgery for some manifolds.

---

## Section 11.4: Green's Functions

**Exercise 11.4.1.** Verify that G(x) = 1/(4π|x|) satisfies −∇²G = δ³(x) by:
(a) Computing −∇²G for |x| > 0 and showing it is zero.
(b) Integrating −∇²G over a ball B_ε and using the divergence theorem to show the integral equals 1.

**Exercise 11.4.2.** Using the Green's function for the half-space ℝ³_+ (the "method of images"), solve:
−∇²u = 0 in z > 0, with u(x, y, 0) = f(x, y) (Dirichlet data on the plane z = 0).
(a) Write the Poisson kernel (the normal derivative of G on the boundary).
(b) Write the solution as a convolution.
(c) Verify that your solution approaches f(x, y) as z → 0⁺.

**Exercise 11.4.3.** The retarded Green's function for gravitational waves.
(a) The linearized Einstein equations in harmonic gauge give $\Box \bar{h}_{\mu\nu} = -16\pi G T_{\mu\nu}$. Using the retarded Green's function, write the solution for $\bar{h}_{ij}$ due to a quadrupolar source T_{ij}(t, x') that is localized (source size R ≪ distance r = |x|).
(b) In the far field, expand in 1/r. Show that $\bar{h}_{ij} \sim (1/r) F_{ij}(t - r/c)$ — a spherical gravitational wave.
(c) Identify the "quadrupole moment" $I_{ij}(t) = \int x'_i x'_j T_{00}(t, x') d^3x'$ and write $\bar{h}_{ij}$ in terms of $\ddot{I}_{ij}$.

---

## Thought Experiments

**Thought Experiment 11.1: The Information Paradox**

The wave equation is time-reversible: if u(x, t) is a solution, so is u(x, −t). The heat equation is not: the smoothing property means information is lost.

(a) The Klein-Gordon equation $\Box\phi = m^2\phi$ is hyperbolic and time-reversible. Information about the source persists indefinitely in the wave field. Does this mean gravitational waves "remember" their source?

(b) Hawking showed that black holes emit thermal (featureless) radiation — apparently erasing information about what fell in. If the evolution equations of GR are time-reversible (hyperbolic), how can information be lost?

(c) Does the answer depend on whether we treat the problem classically (GR only) or quantum-mechanically? What is the "information paradox" and why does it remain unsolved?

**Thought Experiment 11.2: Why Spacetime Has 3+1 Dimensions**

The wave equation behaves differently in different dimensions:
- In 1+1 D: sharp wavefronts. Huygens' principle holds.
- In 2+1 D: tails (no sharp Huygens). Waves linger.
- In 3+1 D: sharp wavefronts again. Huygens' principle holds.
- In 4+1 D: tails again. Etc.

(a) Huygens' principle holds in d+1 spacetime dimensions exactly when d is odd. For even-dimensional space, waves leave tails. Verify for d = 1 and d = 3.

(b) Barrow and Tipler (1986) argued that Huygens' principle holding (odd spatial dimensions) is required for coherent signal transmission and therefore for the development of complex structures. Specifically, they argued that even one extra spatial dimension would make nerve signals "smeared" — impossible to process. Evaluate this anthropic argument.

(c) The strength of gravity (and other forces) falls as 1/r^{d-1} in d spatial dimensions. For d = 3, this gives the inverse square law. For d = 3, circular orbits exist. For d > 3, no stable circular orbits exist. What does this suggest about the "anthropic" explanation of 3+1 dimensions?
