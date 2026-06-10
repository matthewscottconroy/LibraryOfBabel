# Chapter 13: Exercises

---

## Section 13.1: Topological Spaces

**Exercise 13.1.1.** Verify the topology axioms for:
(a) The cofinite topology on an infinite set X: a set U ⊂ X is open iff U = ∅ or X\U is finite.
(b) The lower limit topology on ℝ: the basis consists of half-open intervals [a, b).
(c) The subspace topology on {0} ∪ {1/n : n ≥ 1} ⊂ ℝ.

**Exercise 13.1.2.** Prove that:
(a) The continuous image of a connected set is connected.
(b) The continuous image of a path-connected set is path-connected.
(c) ℝⁿ is connected for all n ≥ 1.
(d) ℝ \ {0} is not connected (it is the union of two disjoint non-empty open sets).

**Exercise 13.1.3.** Fundamental groups.
(a) Prove π₁(S¹) ≅ ℤ by showing that loops on S¹ are classified by their winding number, and that winding number is a homomorphism.
(b) Argue informally that π₁(ℝ³ \ {z-axis}) ≅ ℤ (a loop either winds around the z-axis n times or it doesn't, and the winding number is a topological invariant).
(c) The torus T² = S¹ × S¹ has π₁(T²) ≅ ℤ × ℤ. Identify generators: the "meridian" (a circle going around the tube) and the "longitude" (a circle going around the hole).

**Exercise 13.1.4** (GR application). A wormhole is a spacetime with a non-simply connected spatial section: there is a "handle" connecting two regions. 
(a) What is the fundamental group π₁(ℝ³ minus a ball, with the boundary sphere identified with another sphere) — this is a "wormhole throat"?
(b) The topological censorship theorem says that if the null energy condition holds, any causal curve threading the wormhole can be deformed (while remaining causal) to not thread it. Interpret this as a statement about the fundamental group of the domain of outer communication.
(c) Exotic matter (negative energy) could in principle support a traversable wormhole. What fundamental group would such a spacetime have?

---

## Section 13.2: Metric Spaces

**Exercise 13.2.1.** For each space and metric, determine whether the space is complete.
(a) (ℚ, |·|) — rational numbers with absolute value metric.
(b) (C([0,1]), d_{sup}) where d_{sup}(f,g) = sup_x |f(x)−g(x)|.
(c) (C([0,1]), d_{L¹}) where d_{L¹}(f,g) = ∫_0^1 |f−g|dx.
(d) ((0,1), |·|) — open interval.
(e) The space ℓ² of square-summable sequences: ℓ² = {(aₙ) : Σ|aₙ|² < ∞} with d(a,b) = (Σ|aₙ−bₙ|²)^{1/2}.

**Exercise 13.2.2.** The Banach fixed-point theorem in a complete metric space.
(a) Show that T(x) = x/2 + 1 on [0,2] is a contraction. Find its fixed point.
(b) Show that T(x) = x + 1 on ℝ has no fixed point (it is not a contraction).
(c) Show that T(x) = cos x on ℝ is not a contraction (|cos' x| = |sin x| can be 1), but has a unique fixed point. What property of [0,1] allows you to prove existence without the contraction condition?

**Exercise 13.2.3.** The Hopf-Rinow theorem for a sphere of radius R:
(a) Verify that the geodesic distance on S^n (⊂ ℝ^{n+1}) is d(x,y) = R arccos(x·y/R²).
(b) Show that S^n is geodesically complete: every geodesic (great circle) can be extended to all parameter values.
(c) Show that S^n is metrically complete (Cauchy sequences converge).
(d) Give an example showing Hopf-Rinow fails for a pseudo-Riemannian manifold: find a spacetime that is geodesically incomplete (contains inextensible geodesics of finite affine parameter) but has no "metric incompleteness" (because the metric is indefinite).

**Exercise 13.2.4.** The Baire category theorem.
(a) Prove: a complete metric space is not a countable union of nowhere dense sets.
(b) Conclude: the Cantor set (a nowhere dense, uncountable compact subset of [0,1]) is not a subset of any countable union of nowhere dense sets — i.e., [0,1] is not a countable union of nowhere dense sets.
(c) Construct a function f: [0,1] → ℝ that is continuous everywhere and differentiable nowhere. (Hint: take a series of zigzag functions that converges uniformly.) This shows that such functions exist without constructing them explicitly — just by Baire category and the fact that the set of continuous functions differentiable at any single point is meager.

---

## Section 13.3: Compactness

**Exercise 13.3.1.** Determine which sets are compact. Justify each answer.
(a) [0,1] × [0,1] ⊂ ℝ² (closed unit square)
(b) {x ∈ ℝⁿ : |x| = 1} (unit sphere S^{n-1})
(c) {f ∈ C([0,1]) : |f(x)| ≤ 1 and |f(x)−f(y)| ≤ |x−y| for all x,y} (equicontinuous family)
(d) The image of a continuous function on a compact domain
(e) The set of n×n matrices with ||A|| ≤ 1

**Exercise 13.3.2.** The Gauss-Bonnet theorem.
(a) For a 2-sphere of radius R: compute K = 1/R² (constant positive curvature), compute the area 4πR², and verify ∫K dA = 4π = 2πχ(S²) where χ(S²) = 2.
(b) For a flat torus T² (K = 0 everywhere): the Gauss-Bonnet integral gives 0 = 2πχ(T²). What is χ(T²)?
(c) For a genus-g surface (g handles): χ = 2 − 2g. Can a genus-2 surface carry a metric with positive curvature everywhere? With zero curvature? With negative curvature?
(d) What does the Gauss-Bonnet theorem imply about the possibility of a "compact flat spacetime"? (A spacetime with zero curvature R_{μνρσ} = 0 everywhere but periodic in some directions — analogous to a flat torus.)

**Exercise 13.3.3** (Trapped surfaces). Consider a 2-sphere S of radius R in Schwarzschild spacetime (r > r_s).
(a) Compute the expansion θ of the outgoing null congruence from S. (The result should depend on r and r_s.)
(b) For r > r_s: is θ positive (expanding) or negative (converging)?
(c) For r < r_s (inside the horizon): show θ < 0 for both ingoing and outgoing null congruences — S is trapped.
(d) Penrose's theorem uses the compactness of S (a closed 2-manifold) and the fact that θ < 0 to derive geodesic incompleteness. Why is compactness of S necessary? (What could go wrong with a non-compact "trapped surface"?)

---

## Section 13.4: Manifolds

**Exercise 13.4.1.** Verify that the 2-sphere S² ⊂ ℝ³ is a smooth manifold using two charts (stereographic projections).
(a) Define the north stereographic projection φ_N: S² \ {N} → ℝ² (where N = (0,0,1)) by projecting from the north pole onto the xy-plane. Compute the formula.
(b) Similarly define φ_S: S² \ {S} → ℝ² (projection from the south pole S = (0,0,−1)).
(c) Compute the transition map φ_S ∘ φ_N^{-1}: ℝ² \ {0} → ℝ² \ {0} and verify it is smooth (in fact, it's a conformal map — which map is it?).

**Exercise 13.4.2.** Tangent vectors and derivations.
(a) At the north pole of S² (in geographic coordinates: latitude θ = π/2, longitude φ undefined), compute the coordinate basis vectors ∂/∂θ and ∂/∂φ.
(b) In the stereographic chart near the north pole, the transition map tells you how to transform these basis vectors. Compute the Jacobian and the transformed basis.
(c) Verify that the vector fields are the same geometric objects expressed in different coordinates (the "same" tangent vector has different components in the two charts, related by the Jacobian of the transition map).

**Exercise 13.4.3.** The pushforward and pullback.
(a) Let f: S² → S² be the antipodal map f(x, y, z) = (−x, −y, −z). Compute the pushforward df: T_p S² → T_{f(p)} S² in stereographic coordinates.
(b) Let g: ℝ → S¹ be g(t) = (cos t, sin t). Compute the pullback g* (the 1-form dθ on S¹) to a 1-form on ℝ.
(c) The gravitational redshift: in Schwarzschild spacetime, a photon emitted at radius r₁ with frequency ω₁ is observed at r₂ with frequency ω₂ = ω₁ √(g_{tt}(r₂)/g_{tt}(r₁)) = ω₁ √((1−r_s/r₂)/(1−r_s/r₁)). Interpret this as a statement about the pullback of the frequency 1-form along the photon's worldline.

---

## Thought Experiments

**Thought Experiment 13.1: The Shape of the Universe**

(a) Observations show the universe is "flat" (the geometry of spatial sections is Euclidean to within measurement precision). Does this mean the universe has the topology of ℝ³, or could it be a flat torus T³ = ℝ³/ℤ³?

(b) A flat torus would have the same local geometry as ℝ³ (both have K = 0 everywhere) but different topology (T³ has π₁ ≅ ℤ³, while ℝ³ is simply connected). What observations could distinguish them?

(c) The "cosmic crystallography" method: if the universe has toroidal topology with periodicity L in some direction, we would see repeated patterns in the CMB sky (identical temperature fluctuations in different directions, at a separation determined by L). Current observations constrain L > 28 billion light-years (larger than the observable universe). What does this mean for the practical distinction between ℝ³ and T³ topologies?

**Thought Experiment 13.2: Compactness and Singularities**

Penrose's singularity theorem uses compactness of trapped surfaces to prove geodesic incompleteness. But the universe is (possibly) non-compact.

(a) Hawking's singularity theorem for cosmology uses a compact Cauchy surface instead of a trapped surface. If the universe is spatially non-compact (ℝ³ topology), does Hawking's theorem apply?

(b) Hawking's theorem was modified to handle non-compact Cauchy surfaces by requiring that the universe be "expanding everywhere" (positive expansion) instead of requiring compactness. This is the condition satisfied by the Big Bang. Explain physically why "expansion everywhere" plays the same role as "trapped surface" in driving geodesic incompleteness.

(c) In de Sitter spacetime (pure positive cosmological constant, no matter), geodesics can be extended to all proper time values — de Sitter is geodesically complete. But de Sitter has horizons. Is this a counterexample to the singularity theorem? What energy condition does de Sitter violate?

---

## Laboratory Exercise

**Lab 13.1: Exploring Topological Properties with Physical Models**

Topology is invariant under continuous deformations. Physical models illustrate this concretely.

**Procedure**: 
1. **Connected vs. disconnected**: Take a rubber band (homeomorphic to S¹) and a pair of rubber bands (two disjoint circles). No continuous deformation turns one into the other — they have different numbers of connected components.

2. **Simply connected vs. not**: A disk is simply connected (any loop can be contracted). An annulus is not. Take a strip of paper forming an annulus and a disk. Try to continuously deform one into the other while remaining in ℝ³ (you cannot). Now drill a hole in the disk — you can deform the resulting annulus into the given annulus.

3. **The Euler characteristic**: Take various polyhedra (cube: V=8, E=12, F=6; tetrahedron: V=4, E=6, F=4; octahedron: V=6, E=12, F=8). Compute V − E + F for each. What do you find? Build a "donut" shape out of play-doh and triangulate it. Compute χ.

**Questions**: (a) What is χ for each polyhedron? (b) Build a surface with χ = 0 (torus). Compute χ = V − E + F for your triangulation. (c) The Gauss-Bonnet theorem relates χ to curvature. For a sphere, ∫K dA = 4π = 2πχ(S²) = 2π·2 = 4π. How is the curvature "distributed" on a cube? (Hint: the cube has zero curvature on the faces, but what happens at the corners?)
