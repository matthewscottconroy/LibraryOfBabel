# Chapter 12: Exercises

---

## Section 12.1: Complex Functions and the Cauchy-Riemann Equations

**Exercise 12.1.1.** For each function, determine where it is holomorphic and compute its derivative there.
(a) f(z) = z³ − 2iz + 1
(b) f(z) = 1/z
(c) f(z) = e^z = e^x cos y + ie^x sin y
(d) f(z) = |z|²
(e) f(z) = sin z = sin(x+iy). Use sin(x+iy) = sin x cosh y + i cos x sinh y.

**Exercise 12.1.2.** Find a holomorphic function f = u + iv where:
(a) u(x, y) = x³ − 3xy² (verify u is harmonic first)
(b) u(x, y) = e^x cos y
(c) v(x, y) = x²y − y³/3

In each case, determine v (or u) using the Cauchy-Riemann equations and write f as a function of z.

**Exercise 12.1.3** (Conformal maps). The map f(z) = z² sends the first quadrant {x > 0, y > 0} to:
(a) Show that the positive real axis maps to the positive real axis, and the positive imaginary axis maps to the negative real axis.
(b) Show that the upper half-plane maps to the full plane minus the negative real axis.
(c) The level curves of Re(f) and Im(f) form orthogonal grids. Draw these grids in the z-plane and the w = f(z) plane for f(z) = z².

**Exercise 12.1.4.** The Joukowski transform f(z) = z + c²/z maps the exterior of the circle |z| = c to the exterior of a segment (with c = 1: the segment [−2, 2]).
(a) Verify that f maps the circle |z| = 1 to the segment [−2, 2].
(b) For a circle of radius r > 1 centered at the origin, f maps it to an ellipse. Find the semi-axes.
(c) The Joukowski transform is used in aerodynamics: it maps the flow around a circle to flow around an airfoil. Why is this valid? (The key: holomorphic maps preserve the Laplace equation, so harmonic velocity potentials map to harmonic velocity potentials.)

---

## Section 12.2: Cauchy's Theorem

**Exercise 12.2.1.** Evaluate the following contour integrals directly.
(a) $\oint_{|z|=1} \frac{1}{z} dz$
(b) $\oint_{|z|=2} \frac{1}{z^2} dz$
(c) $\oint_{|z|=1} e^z dz$
(d) $\oint_{|z|=1} \frac{\cos z}{z} dz$ (use the Cauchy integral formula)
(e) $\oint_{|z|=2} \frac{e^z}{(z-1)^3} dz$

**Exercise 12.2.2.** Prove Liouville's theorem using the Cauchy estimate. If f is entire and |f(z)| ≤ M for all z, show f is constant. [Use the Cauchy integral formula for f'(z₀) with a contour of radius R and let R → ∞.]

**Exercise 12.2.3.** Use Liouville's theorem to prove the fundamental theorem of algebra: every non-constant polynomial has a root. [Assume no root exists and show 1/p(z) would be a bounded entire function.]

**Exercise 12.2.4** (Morera). Use Morera's theorem to prove:
(a) If fₙ are holomorphic on U and fₙ → f uniformly on compact subsets, then f is holomorphic.
(b) The function $F(z) = \int_0^\infty e^{-t} t^{z-1} dt$ (the Gamma function) is holomorphic for Re(z) > 0.

**Exercise 12.2.5.** Analytic continuation. The function $f(z) = \sum_{n=0}^\infty z^n$ converges for |z| < 1 and equals 1/(1−z). Show that:
(a) f cannot be analytically continued across the unit circle at z = 1 (the singularity of 1/(1−z)).
(b) f can be analytically continued across any other boundary point z₀ with |z₀| = 1, z₀ ≠ 1.
(c) The function $g(z) = \sum_{n=0}^\infty z^{n!}$ has the entire unit circle as its natural boundary — it cannot be continued to any point on |z| = 1. (Such a function is said to have the unit circle as its "natural boundary.")

---

## Section 12.3: Residues

**Exercise 12.3.1.** Find the residue of each function at its isolated singularities.
(a) $f(z) = \frac{1}{z^2(z-1)}$ at z = 0 and z = 1.
(b) $f(z) = \frac{e^z}{z^3}$ at z = 0.
(c) $f(z) = \frac{1}{\sin z}$ at z = 0, π, −π.
(d) $f(z) = \frac{z}{(z^2+1)^2}$ at z = ±i.

**Exercise 12.3.2.** Use the residue theorem to evaluate:
(a) $\int_0^{2\pi} \frac{d\theta}{2 + \cos\theta}$
(b) $\int_{-\infty}^\infty \frac{x^2}{(x^2+1)(x^2+4)} dx$
(c) $\int_0^\infty \frac{\ln x}{1+x^2} dx$ [use a keyhole contour]
(d) $\int_0^\infty \frac{\sin x}{x} dx = \pi/2$ [use Im(e^{iz}/z) and an indented semicircle]

**Exercise 12.3.3** (Hawking temperature derivation). Verify the calculation of the Hawking temperature from Section 12.3.4.
(a) Near r = r_s, the Schwarzschild metric in Euclidean signature (τ = it) takes the form $ds^2 \approx (r-r_s)/(r_s) d\tau^2 + r_s/(r-r_s) dr^2$. Introduce $\rho = 2\sqrt{r_s(r-r_s)}$. Show the metric becomes $ds^2 \approx (\rho/(2r_s))^2 d\tau^2 + d\rho^2$.
(b) This is the metric of flat 2D space in polar coordinates (R, θ) with R = ρ and θ = τ/(2r_s). For the space to be non-singular at ρ = 0 (the horizon), θ must be periodic with period 2π. What is the period of τ?
(c) The Euclidean partition function for a scalar field at inverse temperature β has periodic Euclidean time τ with period β. Identify the Hawking temperature T_H.
(d) Express T_H in SI units. For a solar-mass black hole (M ≈ 2×10³⁰ kg), compute T_H numerically.

---

## Section 12.4: Applications

**Exercise 12.4.1.** Kramers-Kronig relations. The susceptibility χ(ω) = 1/(ω₀² − ω² − iγω) describes a damped harmonic oscillator.
(a) Find the poles of χ(ω) in the complex ω-plane. Are they in the upper or lower half-plane?
(b) Compute Re(χ) and Im(χ) for real ω.
(c) Verify the Kramers-Kronig relation: Re(χ(ω)) = (1/π) P.V. ∫ Im(χ(ω'))/(ω'−ω) dω'.
(d) The imaginary part Im(χ) > 0 for ω > 0 represents absorption. What does the Kramers-Kronig relation say about the sign of Re(χ) for frequencies far below and above resonance?

**Exercise 12.4.2** (Penrose diagram via conformal map). Define null coordinates u = t − r, v = t + r in Minkowski spacetime. The Penrose diagram maps (u, v) ↦ (U, V) = (arctan(u), arctan(v)).
(a) What is the range of (U, V)?
(b) Define T = V + U, R = V − U. Show that the Minkowski metric becomes ds² = Ω²(−dT² + dR²) for some conformal factor Ω(T, R).
(c) Draw the Penrose diagram. Label the boundaries: i⁰ (spacelike infinity), i⁺, i⁻ (timelike infinities), ℐ⁺, ℐ⁻ (null infinities).
(d) A worldline at constant r in Minkowski spacetime becomes a vertical line in (t, r). Draw this in the Penrose diagram.

**Exercise 12.4.3.** The Casimir effect via zeta regularization.
(a) Two conducting plates at z = 0 and z = L force the normal modes of the electromagnetic field to be nπ/L for n = 1, 2, 3, ... Compute the formal sum Σ ω_n for ω_n = cnπ/L.
(b) The Riemann zeta function satisfies ζ(−1) = −1/12. Use this to evaluate the regularized vacuum energy E_vac = (ℏ/2) lim_{s→−1} Σ ω_n^{−s}.
(c) The Casimir force per unit area is F/A = −dE_vac/dL. Compute it.
(d) The experimental measurement of the Casimir force (Lamoreaux 1997) confirmed the prediction. For plates separated by L = 1 μm, compute the predicted force per unit area and compare to atmospheric pressure (~10⁵ Pa).

---

## Thought Experiments

**Thought Experiment 12.1: Analytic Continuation and Spacetime**

The Schwarzschild metric in standard coordinates becomes singular at r = r_s (the horizon) because the coordinates break down there, not because the spacetime is singular. The physical metric continues through the horizon — which is shown by analytic continuation (the Kruskal extension).

(a) In what sense is the Kruskal extension an "analytic continuation" of the Schwarzschild metric? (It is not literally the analytic continuation of a complex function, but the idea is analogous.)

(b) The analytic continuation of the Euclidean metric (t → iτ) gives the Hawking temperature. If we could somehow "measure" quantum fields near a horizon, we would observe thermal radiation. But from a distant observer's perspective, the infalling observer never crosses the horizon. Is there a contradiction?

(c) The "firewall" paradox (Almheiri-Marolf-Polchinski-Sully, 2013) proposes that an infalling observer encounters a "firewall" (high-energy radiation) at the horizon, rather than the smooth spacetime predicted by GR. How might complex analysis and analytic continuation help resolve or sharpen this paradox?

**Thought Experiment 12.2: Holomorphic Functions Have No Free Lunch**

A holomorphic function is "infinitely constrained": its values on any curve determine it everywhere (identity theorem). This rigidity is unusual — a real-differentiable function can be modified locally without changing it globally.

(a) Can a holomorphic function vanish on an infinite set without being identically zero? [Yes, e.g., f(z) = sin z vanishes at all integers nπ. What is the difference from the identity theorem?]

(b) A holomorphic function on the unit disk that is continuous on the closed disk is determined by its boundary values (Cauchy integral formula). What does this say about information storage? If you knew the gravitational wave signal on a sphere around a source, would you know the full signal?

(c) In quantum gravity, the holographic principle (t'Hooft, Susskind) proposes that all information in a 3D volume is encoded on its 2D boundary. Is this related to the Cauchy integral formula? Explore the analogy.
