# Chapter 13: Important Concepts

---

**Topological Space**: A set X with a collection τ of "open sets" satisfying: ∅ and X are open; arbitrary unions of open sets are open; finite intersections of open sets are open. The open sets encode "proximity" without requiring a distance function.

**Open Set**: A set U in a topological space that contains a full neighborhood of each of its points. In ℝⁿ, a set is open iff every point has a surrounding open ball contained in the set. Open sets are the primitive data of a topological space.

**Continuous Map**: f: X → Y is continuous iff the preimage of every open set in Y is open in X. Generalizes the ε-δ definition to topological spaces without distance.

**Homeomorphism**: A bijection f: X → Y such that both f and f⁻¹ are continuous. Homeomorphic spaces have the same topological structure. A coffee cup is homeomorphic to a torus (both have genus 1).

**Topological Invariant**: A property preserved by homeomorphisms: connectedness, compactness, fundamental group, homology groups, Euler characteristic. Used to distinguish non-homeomorphic spaces.

**Hausdorff Space (T₂)**: A topological space where any two distinct points have disjoint open neighborhoods. In a Hausdorff space, limits of sequences are unique and compact subsets are closed. Smooth manifolds are required to be Hausdorff — distinct spacetime events must be topologically distinguishable.

**Connected Space**: Cannot be written as the disjoint union of two non-empty open sets. Equivalent condition: the only clopen (simultaneously open and closed) subsets are ∅ and X. The connected components partition X into maximal connected subsets.

**Path-Connected**: A space where any two points can be connected by a continuous path. Path-connected implies connected; the converse fails.

**Simply Connected**: Path-connected space where every loop can be contracted to a point. π₁(X) = {1} (trivial fundamental group). ℝⁿ is simply connected; S¹ and ℝ² \ {0} are not. Simply connected spacetimes have no "topological holes" that causal curves can thread.

**Fundamental Group π₁(X, x₀)**: The group of homotopy classes of loops based at x₀ under concatenation. Measures the "hole structure" of X. π₁(S¹) ≅ ℤ (winding number); π₁(T²) ≅ ℤ × ℤ; π₁(S²) = {1} (trivially simply connected).

**Compact Space**: Every open cover has a finite subcover. Compact = "topologically finite." In ℝⁿ: compact iff closed and bounded (Heine-Borel). Compact spaces satisfy the extreme value theorem; continuous images of compact sets are compact; compact subsets of Hausdorff spaces are closed.

**Heine-Borel Theorem**: In ℝⁿ, a set is compact iff it is closed and bounded. The characterization of compactness that makes the intermediate value theorem, extreme value theorem, and uniform continuity provable for functions on [a, b].

**Sequential Compactness**: Every sequence has a convergent subsequence. Equivalent to compactness in metric spaces (Bolzano-Weierstrass generalized). The tool for extracting convergent subsequences from bounded sequences of functions in function spaces.

**Arzela-Ascoli Theorem**: A family F ⊂ C([a,b]) is compact iff it is uniformly bounded and equicontinuous. The fundamental tool for proving existence of solutions (by showing approximate solutions have convergent subsequences).

**Metric Space**: A set X with a distance function d: X×X → [0,∞) satisfying d(x,y) ≥ 0 (= 0 iff x=y), symmetry, and the triangle inequality. Every metric space is a topological space; open balls are open sets.

**Complete Metric Space**: A metric space where every Cauchy sequence converges. ℝ, C([0,1]) with sup norm, Hilbert spaces. The setting for the Banach fixed-point theorem, which underlies the Picard-Lindelöf theorem, the IFT, and existence theorems throughout analysis.

**Banach Space / Hilbert Space**: A complete normed space (Banach) or a Banach space whose norm comes from an inner product (Hilbert). Hilbert spaces are the natural infinite-dimensional generalization of Euclidean space — the setting for quantum mechanics and functional analysis.

**Baire Category Theorem**: A complete metric space cannot be a countable union of nowhere dense sets (meager sets). Consequence: "most" continuous functions are nowhere differentiable; bounded operators satisfying pointwise bounds are uniformly bounded.

**Topological Manifold**: A Hausdorff, second-countable topological space where every point has a neighborhood homeomorphic to an open subset of ℝⁿ. The dimension n is a topological invariant. Spacetime is a 4-dimensional topological manifold.

**Smooth Manifold**: A topological manifold with a smooth atlas — charts whose transition maps are C∞. The smooth structure enables differentiation on the manifold. The tangent space, smooth functions, and differential forms are defined using the smooth structure.

**Smooth Atlas**: A collection of charts {(U_α, φ_α)} covering M, such that all transition maps φ_β ∘ φ_α⁻¹ are smooth. Two atlases are compatible if their union is an atlas. The maximal atlas (differentiable structure) is the equivalence class of compatible atlases.

**Tangent Space T_pM**: The n-dimensional vector space of "directions" at a point p ∈ M. Defined intrinsically either as equivalence classes of curves (same velocity → same tangent vector) or as derivations (linear maps C∞(M) → ℝ satisfying the Leibniz rule). The coordinate basis vectors ∂/∂xⁱ form a basis.

**Pushforward df_p**: The linear map T_pM → T_{f(p)}N induced by a smooth map f: M → N. In coordinates, it is the Jacobian matrix of f. This is the differential-geometric generalization of the derivative as a linear map from Chapter 4.

**Euler Characteristic χ(M)**: A topological invariant of compact manifolds. For a compact surface: χ = 2 − 2g where g is the genus (number of handles). Connected by the Gauss-Bonnet theorem to the total curvature: ∫_Σ K dA = 2πχ(Σ).

**Gauss-Bonnet Theorem**: ∫_M K dA = 2πχ(M) for a compact oriented Riemannian surface. The integral of the Gaussian curvature over a surface equals 2π times the Euler characteristic. The profound connection between a local geometric quantity (curvature) and a global topological invariant (Euler characteristic).

**Global Hyperbolicity**: A spacetime property equivalent to the existence of a Cauchy surface (a spacelike hypersurface met exactly once by every inextensible causal curve). Globally hyperbolic spacetimes have topology ℝ × Σ for a Cauchy surface Σ. Required for well-posedness of the GR initial value problem.

**Trapped Surface**: A compact spacelike 2-surface S in spacetime where both families of null geodesics from S are converging (θ < 0 for both ingoing and outgoing directions). Inside a Schwarzschild black hole for r < r_s. The starting point for Penrose's singularity theorem.

**Penrose Singularity Theorem (1965)**: If a globally hyperbolic spacetime satisfies the null energy condition and contains a trapped surface, then it contains a geodesic that cannot be extended to arbitrarily large affine parameter. Spacetime is geodesically incomplete — it has a singularity. Uses compactness of trapped surfaces and the Raychaudhuri focusing equation.

**Geodesic Incompleteness**: The existence of a geodesic that cannot be extended to all values of its affine parameter. This is the definition of a spacetime singularity in GR — a point or region where geodesics "run out." The Penrose and Hawking singularity theorems prove geodesic incompleteness under physically reasonable conditions.

**Topological Censorship Theorem**: If the null energy condition holds, any causal curve in the exterior region that "detects" the topology of the interior must pass through a trapped surface (black hole). Physical observers outside cannot directly observe non-trivial topology (wormholes, handles) — such topological features are hidden behind horizons.
