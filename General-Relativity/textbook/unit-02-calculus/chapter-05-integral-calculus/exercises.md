# Chapter 5 Exercises: Integral Calculus

---

## Section 5.1: The Riemann Integral

**Exercise 5.1.1** *(computing Riemann sums)*  
Let f(x) = x² on [0, 1]. Compute:
- (a) The left Riemann sum Lₙ with n equal subintervals.
- (b) The right Riemann sum Rₙ.
- (c) Show that both Lₙ and Rₙ approach 1/3 as n → ∞.
- (d) Show that |Rₙ − Lₙ| = 1/n for all n. Interpret this geometrically.

**Exercise 5.1.2** *(the Dirichlet function)*  
Let D(x) = 1 if x ∈ ℚ, D(x) = 0 if x ∉ ℚ.
- (a) Prove that D is not Riemann integrable on [0, 1] by showing U(D, P) = 1 and L(D, P) = 0 for every partition P.
- (b) The function D is Lebesgue integrable with ∫₀¹ D dx = 0. What does this tell you about the "size" of ℚ ∩ [0, 1] in the sense of measure theory?
- (c) Modify D to make it Riemann integrable: define E(x) = 1 if x = p/q in lowest terms with q ≤ N, and E(x) = 0 otherwise. Show that E is Riemann integrable for every N (it has finitely many discontinuities).

**Exercise 5.1.3** *(integrability criterion)*  
A function f is Riemann integrable iff for every ε > 0, there exists a partition P such that U(f, P) − L(f, P) < ε. Use this to prove:
- (a) f(x) = sin(1/x) (extended to f(0) = 0) is Riemann integrable on [0, 1]. (Hint: handle the interval [0, δ] separately for small δ.)
- (b) Every function with finitely many jump discontinuities is Riemann integrable.

---

## Section 5.2: The Fundamental Theorem

**Exercise 5.2.1** *(FTC in both directions)*  
Let F(x) = ∫₀ˣ t sin t dt.
- (a) Compute F'(x) directly from FTC Part I.
- (b) Compute F(π) − F(0) = ∫₀^π t sin t dt using FTC Part II after finding an antiderivative by integration by parts.
- (c) Verify that your answers are consistent.

**Exercise 5.2.2** *(a surprising derivative)*  
Define G(x) = ∫₁^{x²} sin(t²) dt. Compute G'(x). (Apply the chain rule to the FTC — the upper limit is not x but x².) What is G'(1)?

**Exercise 5.2.3** *(conservation laws via FTC)*  
A fluid flows along the x-axis with density ρ(x, t) and velocity field v(x, t). The **continuity equation** (conservation of mass) is:

$$\frac{\partial \rho}{\partial t} + \frac{\partial (\rho v)}{\partial x} = 0$$

- (a) Show that this implies d/dt ∫ₐᵇ ρ(x, t) dx = [ρv]ₐᵇ = ρ(a,t)v(a,t) − ρ(b,t)v(b,t). (Differentiate under the integral sign, then apply the FTC.)
- (b) Interpret this physically: the rate of change of total mass in [a, b] equals the flux in at x = a minus the flux out at x = b.
- (c) This is a 1D version of the divergence theorem. State the generalization you expect in 3D.

---

## Section 5.3: Techniques of Integration

**Exercise 5.3.1** *(technique selection)*  
Evaluate each integral. State which technique you use and why.
- (a) ∫ x² ln x dx
- (b) ∫ arctan x dx
- (c) ∫ dx / (x² − 4)
- (d) ∫ x / √(4 − x²) dx
- (e) ∫ sin⁴ x dx (use the identity sin²x = (1 − cos 2x)/2)
- (f) ∫ eˣ cos x dx

**Exercise 5.3.2** *(building the error function)*  
The **error function** is erf(x) = (2/√π) ∫₀ˣ e^{-t²} dt. It cannot be expressed in terms of elementary functions, but:
- (a) Compute erf'(x) using FTC Part I.
- (b) Use integration by parts to show ∫ x² e^{-x²} dx = −x e^{-x²}/2 + (√π/4) erf(x) + C.
- (c) Give the first four non-zero terms of the Maclaurin series for erf(x). (Use the series for e^{-t²} and integrate term by term.)
- (d) Estimate erf(0.5) using your series. Compare with the true value erf(0.5) ≈ 0.5205.

**Exercise 5.3.3** *(Wallis' product — a remarkable identity)*  
Define $I_n = \int_0^{\pi/2} \sin^n x \, dx$.
- (a) Use integration by parts to derive the reduction formula: $I_n = \frac{n-1}{n} I_{n-2}$.
- (b) Compute I₀ = π/2, I₁ = 1, and use the reduction to compute I₂, I₃, I₄, I₅.
- (c) Show that $\frac{I_{2n}}{I_{2n+1}} \to 1$ as n → ∞. (Use the squeeze: I_{2n+1} ≤ I_{2n} ≤ I_{2n-1} = (2n)/(2n-1) I_{2n+1}.)
- (d) Conclude **Wallis' product**: $\frac{\pi}{2} = \frac{2}{1} \cdot \frac{2}{3} \cdot \frac{4}{3} \cdot \frac{4}{5} \cdot \frac{6}{5} \cdot \frac{6}{7} \cdots$

This is a beautiful identity connecting π to a product of rationals. It was discovered by John Wallis in 1655, before the formal development of calculus.

---

## Section 5.4: Improper Integrals

**Exercise 5.4.1** *(convergence classification)*  
For each integral, determine whether it converges or diverges. If it converges, find its value.
- (a) $\int_0^1 \frac{1}{\sqrt{x}} dx$
- (b) $\int_1^\infty \frac{1}{x(1+x^2)} dx$
- (c) $\int_0^\infty x e^{-x^2} dx$
- (d) $\int_{-\infty}^\infty \frac{1}{1+x^4} dx$ (Hint: partial fractions over ℝ, or use the Residue Theorem from Chapter 12 for elegance.)
- (e) $\int_0^\infty \frac{\sin x}{x} dx$ (This converges conditionally. The value is π/2 — prove this using complex analysis in Chapter 12.)

**Exercise 5.4.2** *(Gamma function properties)*  
Using only the definition $\Gamma(s) = \int_0^\infty t^{s-1} e^{-t} dt$:
- (a) Prove Γ(s+1) = s Γ(s) by integration by parts.
- (b) Prove Γ(n) = (n−1)! for positive integers n by induction.
- (c) Show that Γ(1/2) = √π. (Let t = u², convert to the Gaussian integral ∫_{-∞}^∞ e^{-u²} du = √π.)
- (d) Compute Γ(3/2) and Γ(5/2).
- (e) Use the Gamma function to evaluate $\int_0^\infty t^3 e^{-2t} dt$ (by substitution u = 2t).

---

## Thought Experiments

**Thought Experiment 5.1** *(what does area mean?)*  
Before defining the Riemann integral, what does "the area under the graph of f" mean? For a polygon, area has a clear geometric definition. For a curve, we must define area as a limit. The Riemann integral is one definition. Could we have defined area differently and gotten a different answer? What property should any reasonable definition of area satisfy, and which of these properties uniquely determines the integral?

Now think about this: in GR, the "area" of a surface and the "volume" of a spacetime region are integrals of the form ∫√(det g) dⁿx, where g is the induced metric. The √(det g) factor is the generalization of "the width of an infinitesimal rectangle adapted to the geometry." How does the conceptual structure of the Riemann integral generalize to this case?

**Thought Experiment 5.2** *(the FTC and time-reversal)*  
The FTC says that ∫ₐᵇ F'(x) dx = F(b) − F(a) — the integral of the rate of change equals the net change. This seems obvious. But it is deep.

Consider: if we reverse time (b and a swap), the integral changes sign: ∫ᵦᵃ = −∫ₐᵇ. Yet physical processes are often time-asymmetric — ice melts, entropy increases. How does the FTC, which is time-symmetric, describe time-asymmetric processes? (Hint: the asymmetry is in the initial conditions and the form of F', not in the FTC itself.)

**Thought Experiment 5.3** *(Stokes' theorem as FTC)*  
The FTC says: the integral of the derivative of f over [a, b] equals the boundary values of f. The "boundary" of [a, b] is the two points {a, b}.

Stokes' theorem in 3D says: ∫∫_S (∇ × F) · dA = ∮_C F · dl. The integral of the "derivative" (curl) of F over a surface S equals the integral of F over the boundary ∂S = C.

The generalized Stokes' theorem says: ∫_M dω = ∫_{∂M} ω, for a differential form ω on a manifold M. This is FTC in full generality.

In GR, the conservation law ∇_μ T^{μν} = 0 (covariant divergence of the stress-energy tensor is zero) is intimately connected to this. Explain in your own words how a conservation law can be thought of as a "Stokes' theorem statement."

---

## Laboratory Projects

**Lab 5.1** *(numerical integration accuracy)*  
Choose f(x) = sin(πx) on [0, 1] (whose exact integral is 2/π).

- Implement the left sum, right sum, trapezoid rule, midpoint rule, and Simpson's rule in code.
- For each method, compute the approximation for n = 2, 4, 8, 16, 32, 64, 128 subintervals.
- Plot the error vs n on a log-log scale. The slope should give the order of convergence.
- Expected: left/right O(h), midpoint/trapezoid O(h²), Simpson O(h⁴). Verify.
- Does the convergence rate change for a less smooth function? Repeat with f(x) = |x − 0.5| on [0, 1].

**Lab 5.2** *(the Gaussian integral by simulation)*  
The Gaussian integral ∫_{-∞}^∞ e^{-x²} dx = √π can be "proved" by a probabilistic simulation:

- Generate N random points (x, y) uniformly in [−R, R] × [0, 1].
- Count how many satisfy y < e^{-x²}.
- This fraction estimates ∫_{-R}^R e^{-x²} dx / (2R).
- For R = 3 (where e^{-9} ≈ 0.0001, so the tail contributes negligibly), estimate √π.
- How large must N be to get 2 decimal places of accuracy? 3 decimal places?
- This is Monte Carlo integration — the standard method for high-dimensional integrals in quantum field theory, where the dimension is the number of field values being integrated (potentially infinite).

**Lab 5.3** *(arc length and geodesics)*  
The **arc length** of a curve y = f(x) from x = a to x = b is $L = \int_a^b \sqrt{1 + [f'(x)]^2} \, dx$.

- Compute the arc length of y = x² from x = 0 to x = 1 numerically (the exact answer involves an arctanh).
- Compute the arc length of y = sin x from x = 0 to x = π.
- Now consider the surface of revolution generated by rotating y = f(x) around the x-axis. Its area is $A = 2\pi \int_a^b f(x) \sqrt{1 + [f'(x)]^2} \, dx$. Compute the surface area of the unit sphere (f(x) = √(1−x²), a = −1, b = 1). You should get 4π.
- The arc-length formula is the starting point for the GR line element ds² = gᵤᵥ dxᵘ dx^ν. In flat spacetime, ds² = −c²dt² + dx² + dy² + dz², and the spacetime arc length of a path gives proper time. The analogy between Euclidean arc length and Lorentzian proper time is direct.
