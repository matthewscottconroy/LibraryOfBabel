# Chapter 8: Exercises

---

## Section 8.1: The Gradient, Divergence, and Curl

**Exercise 8.1.1.** Let f(x, y, z) = x²y + yz³.
(a) Compute ∇f.
(b) Find the directional derivative of f at the point (1, 1, 1) in the direction of v = (1, 2, −1)/√6.
(c) At what point on the surface f = 3 closest to the origin does the gradient point directly away from the origin?

**Exercise 8.1.2.** Prove the product rule for the gradient: ∇(fg) = f∇g + g∇f. Use this to prove the product rule for divergence: ∇·(fF) = f∇·F + (∇f)·F.

**Exercise 8.1.3.** For the electric field of a point charge, F(r) = r/|r|³:
(a) Compute ∇·F for r ≠ 0. You should get 0.
(b) Why doesn't this contradict Gauss's law ∮ F·dA = 4πk_eq? What is the correct statement involving the delta function?
(c) Verify by direct calculation that ∮_{|r|=R} F·dA = 4π for any R > 0.

**Exercise 8.1.4.** For a vector field F = (y, z, x), compute:
(a) ∇·F (divergence)
(b) ∇×F (curl)
(c) ∇(∇·F) (gradient of divergence)
(d) ∇·(∇×F) (verify it's zero)
(e) ∇×(∇×F) and verify it equals ∇(∇·F) − ∇²F.

**Exercise 8.1.5** (Harder). The vector Laplacian in spherical coordinates is ∇²F = ∇(∇·F) − ∇×(∇×F). For a radial field F = f(r)r̂:
(a) Compute ∇·F and ∇×F.
(b) Use these to compute ∇²F without converting to Cartesian coordinates.
(c) What ODE must f(r) satisfy for F to be harmonic (∇²F = 0) for r > 0?

**Exercise 8.1.6** (Physical). In a gravitational field, the metric perturbation in the Newtonian limit satisfies ∇²Φ = 4πGρ. Given a uniform sphere of density ρ₀ and radius R:
(a) Find Φ outside the sphere (the exterior solution).
(b) Find Φ inside the sphere.
(c) Verify continuity of Φ and ∇Φ at r = R.
(d) Interpret the result: in what sense does the exterior field "know" only the total mass?

---

## Section 8.2: Line Integrals and Surface Integrals

**Exercise 8.2.1.** Compute the line integral ∫_C F·dr where F = (y, x, z) and C is:
(a) The straight line from (0,0,0) to (1,1,1).
(b) The helix r(t) = (cos t, sin t, t) for t ∈ [0, 2π].
(c) Show that F is conservative and find its potential function. Then verify your answers to (a) and (b) using the potential.

**Exercise 8.2.2** (Original). A particle moves along a path in the Schwarzschild spacetime. The worldline in coordinates (t, r) (suppressing angles) satisfies dr/dt = −√(r_s/r) (for an infalling particle). The "work" done by the effective potential along the worldline is W = ∫_C V_eff dr.
(a) Why is this integral path-dependent in general?
(b) For a freely falling particle (geodesic motion), the effective potential relates to the energy as E² = (dr/dτ)² + (1 − r_s/r). Write the geodesic condition as a statement about ∫ along the worldline.
(c) What does it mean physically that the geodesic in spacetime is the path of stationary proper time?

**Exercise 8.2.3.** Compute the surface integral ∬_S F·dA where F = (x, y, z) and S is:
(a) The sphere x² + y² + z² = R².
(b) The upper hemisphere z ≥ 0 with upward normal.
(c) The cylinder x² + y² = R², 0 ≤ z ≤ h, with outward normal.

**Exercise 8.2.4.** The surface of a gravitational wave detector is a sphere of radius R. The gravitational wave flux (energy per unit area per unit time) has the form:
$$F = \frac{P}{4\pi r^2} f(\theta, \phi)$$
where P is the total power and f is the angular distribution (normalized so that ∬ f sin θ dθ dφ = 4π). Show that the total power intercepted by the sphere is P, regardless of R, demonstrating conservation of energy for gravitational radiation.

**Exercise 8.2.5.** Proper time as a line integral. In Minkowski spacetime with coordinates (t, x, y, z) and metric ds² = −c²dt² + dx² + dy² + dz², the proper time along a worldline γ is:
$$\tau = \int_\gamma \frac{1}{c}\sqrt{-\eta_{\mu\nu}\frac{dx^\mu}{d\lambda}\frac{dx^\nu}{d\lambda}} d\lambda$$
(a) For a particle at rest (x = y = z = const), show τ = t (the proper time equals coordinate time).
(b) For a particle moving with constant velocity v, show τ = t/γ where γ = 1/√(1−v²/c²).
(c) Two twins separate at age 0. Twin A stays at rest; twin B travels at 0.8c for 5 years (proper time), then returns. Compute each twin's age when they reunite using the line integral formula. This is the twin paradox.

---

## Section 8.3: Integral Theorems

**Exercise 8.3.1.** Use Green's theorem to compute ∮_C (x²y dx − xy² dy) where C is the boundary of the region 0 ≤ x ≤ 1, 0 ≤ y ≤ x².

**Exercise 8.3.2.** The divergence theorem in GR. The conservation law for the stress-energy tensor is ∇_μ T^{μν} = 0 (covariant divergence). In a coordinate system with T^{0ν} representing the energy-momentum flux:
(a) Explain why ∂_μ (√(-g) T^{μν}) ≠ 0 in general, even when ∇_μ T^{μν} = 0.
(b) Under what special condition (on the coordinate system or on the metric) does ∂_μ T^{μν} = 0? What does this mean physically?
(c) Why can't we simply apply the divergence theorem to define total conserved energy in GR, as we can in flat spacetime?

**Exercise 8.3.3.** Verify Stokes' theorem for F = (y, −x, z²) and the surface S: the upper hemisphere x² + y² + z² = 1, z ≥ 0, with boundary C: the unit circle in the z = 0 plane.

**Exercise 8.3.4** (Thought experiment). The generalized Stokes' theorem ∫_M dω = ∫_{∂M} ω says: the integral of the exterior derivative over a manifold equals the integral of the form over its boundary.
(a) State the fundamental theorem of calculus, the divergence theorem, and Green's theorem as special cases of this.
(b) What is the boundary of a closed surface (like a sphere)? What does the generalized Stokes' theorem say in this case?
(c) In GR, the "boundary" of spacetime can be taken at spatial infinity or at a black hole horizon. What physical conservation law does the generalized Stokes' theorem give in each case?

**Exercise 8.3.5.** Use the divergence theorem to prove that if ∇²f = 0 in a domain D (f is harmonic) and f = 0 on ∂D, then f = 0 everywhere in D.

---

## Section 8.4: Differential Forms

**Exercise 8.4.1.** Let ω = x dy ∧ dz + y dz ∧ dx + z dx ∧ dy on ℝ³.
(a) Compute dω.
(b) Evaluate ∫_S ω where S is the unit sphere, oriented outward.
(c) Verify Stokes' theorem: compare ∫_S ω with ∫_{∂S} (something) — but wait, what is ∂S? What does this tell you?

**Exercise 8.4.2.** The electromagnetic 2-form in spacetime is F = (1/2) F_{μν} dx^μ ∧ dx^ν.
(a) Write out F explicitly in terms of E and B fields using F_{0i} = E_i and F_{ij} = ε_{ijk} B_k.
(b) Compute dF using the exterior derivative.
(c) Show that dF = 0 is equivalent to the homogeneous Maxwell equations ∇·B = 0 and ∂B/∂t + ∇×E = 0.
(d) The inhomogeneous equations are d★F = ★J where J is the 4-current 1-form. Write this out in components and identify it as ∇·E = ρ/ε₀ and ∇×B − ∂E/∂t = μ₀J.

**Exercise 8.4.3.** de Rham cohomology.
(a) Show that any closed k-form on ℝⁿ is exact (this is the Poincaré lemma). [Hint: construct an explicit homotopy operator.]
(b) Show that the form ω = (x dy − y dx)/(x² + y²) on ℝ² \ {0} is closed but not exact. [Hint: compute ∮_{|r|=1} ω.]
(c) What does (b) say about the topology of ℝ² \ {0}? Compute H¹(ℝ² \ {0}).

**Exercise 8.4.4** (Original). The Aharonov-Bohm effect. Consider a region ℝ³ \ (z-axis) (space with the z-axis removed) and a magnetic vector potential A such that B = ∇×A = 0 (no magnetic field outside a solenoid along the z-axis). However, the line integral ∮_C A·dr ≠ 0 for a loop encircling the z-axis.
(a) Formulate this using differential forms: what does "A is closed but not exact" mean here?
(b) The Aharonov-Bohm phase is Φ = e/ℏ ∮_C A·dr. Show this is a topological invariant: it depends only on the homotopy class of C (how many times C winds around the z-axis).
(c) Explain what H¹(ℝ³ \ {z-axis}) measures physically.

**Exercise 8.4.5.** The Hodge dual on Minkowski spacetime. Define the Hodge star ★ on ℝ^{1,3} with metric η = diag(−1,+1,+1,+1).
(a) Compute ★(dt∧dx), ★(dx∧dy), ★(dt∧dx∧dy).
(b) Show that ★★ = −1 on 2-forms in Minkowski spacetime.
(c) The Maxwell action is S = −(1/4) ∫ F ∧ ★F. Write this out in components and show it equals ∫ (1/2)(E² − c²B²) d⁴x (up to constants).

---

## Thought Experiments

**Thought Experiment 8.1: The Unreachable Interior**

You are a physicist at the event horizon of a Schwarzschild black hole. The boundary ∂M of the "exterior region" (r > r_s) includes both spatial infinity and the horizon. By the generalized Stokes' theorem, a conservation law (say, conservation of electric charge) in the exterior region gives:

$$\frac{d}{dt}Q_{\text{exterior}} = -\text{flux through horizon} - \text{flux through infinity}$$

(a) If a charged particle falls into the black hole, the flux through the horizon is non-zero. Does the exterior charge change?  
(b) Can an external observer determine whether the infalling charge has crossed the horizon? What observable detects it?  
(c) The "no-hair theorem" says the exterior gravitational field depends only on mass, charge, and angular momentum. Is this consistent with charge conservation as formulated above?

**Thought Experiment 8.2: Gauss's Law in Curved Space**

In flat space, Gauss's law ∮_S E·dA = Q_{enc}/ε₀ lets you compute the field of a symmetric charge distribution without solving a PDE. In GR, the divergence theorem involves √(-g) and the curved metric.

(a) For a point mass in Schwarzschild spacetime, the "gravitational field" ∇Φ ≈ −GM/r² r̂ holds approximately in the weak-field limit. Does the "total mass enclosed" argument still work?  
(b) The ADM mass is defined as a surface integral at infinity: M = (1/16πG) ∮_{S_∞} (∂_j h_{ij} − ∂_i h_{jj}) dAⁱ where h_{ij} is the deviation of the spatial metric from flat. This is exactly an application of the divergence theorem. What is the "source" whose integral gives the ADM mass?  
(c) Can you define a "local" gravitational energy density in GR? What obstruction does the equivalence principle provide?

---

## Laboratory Exercises

**Lab 8.1: Visualizing Vector Fields and Their Curl**

A two-dimensional vector field can be visualized by plotting arrows at grid points. Experimentally:
- Hang iron filings over a surface and hold a bar magnet below. The filings align with B, visualizing the field lines (integral curves of the vector field).
- Use a small paddlewheel in a flowing fluid to detect curl: if the paddlewheel rotates, the flow has non-zero curl at that point.

**Procedure**: Set up a laminar flow of water in a channel (a trough with gently flowing water). Place small floating objects at various points to visualize streamlines. Drop a small, neutrally buoyant object that can rotate (a small disk with a pointer) to detect local rotation (curl). Compare regions near walls (high shear, high curl) versus the center (lower curl).

**Questions**: (a) Where is the curl largest? (b) The divergence of an incompressible fluid flow is zero — can you verify this by observing that streamlines don't converge or diverge in the bulk? (c) For a circular vortex, compute the expected curl analytically and compare to observation.

**Lab 8.2: Faraday's Law as Stokes' Theorem**

Faraday's law is ∮_C E·dl = −d/dt ∫_S B·dA — precisely Stokes' theorem applied to the relation dF = 0.

**Procedure**: Construct a simple induction coil: a circular loop of wire (C) enclosing an area S, connected to an oscilloscope. Pass a bar magnet through the loop at various speeds.

**Measurement**: Record the induced EMF (∮ E·dl) versus time. Simultaneously track the position of the magnet (and hence the flux ∫ B·dA through the loop, calculable from the dipole field). Verify that the EMF equals −dΦ/dt.

**Conceptual extension**: This is differential forms in action. The form F (electromagnetic 2-form) satisfies dF = 0. Stokes' theorem applied to dF gives the relation between line integral and surface integral that is Faraday's law. The same mathematical structure governs the curvature 2-form in GR.
