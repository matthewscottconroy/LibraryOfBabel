# Section 13.1: Topological Spaces and Continuous Maps

---

## Section Introduction

**Topology** is the study of properties that are preserved under continuous deformations — stretching and bending, but not tearing or gluing. A topologist famously cannot distinguish a coffee cup from a donut (both have one hole), but can distinguish either from a sphere (no holes). What makes this precise is the concept of a **topological space**: a set with enough structure to define what "continuous" means, without any notion of distance or angle.

For GR, the topological structure of spacetime is not merely a background assumption — it has physical consequences. The topological censorship theorem (Friedman, Schleich, Witt, 1993) implies that, if energy conditions hold, the topology of spatial sections of spacetime cannot change over time and no "topology-changing" process is accessible to physical observers. The existence of closed timelike curves (time travel) is a topological phenomenon. Black hole horizons are topological surfaces. And the global structure of spacetime — whether it is simply connected, whether it has "wormholes," whether it is compact — depends on its topology.

---

## 13.1.1 Topological Spaces

**Definition**: A **topological space** is a set X together with a collection τ of subsets of X (called **open sets**) satisfying:
1. ∅ ∈ τ and X ∈ τ.
2. Arbitrary unions: if U_α ∈ τ for all α, then ∪_α U_α ∈ τ.
3. Finite intersections: if U₁, ..., Uₙ ∈ τ, then U₁ ∩ ... ∩ Uₙ ∈ τ.

The collection τ is called the **topology** on X.

**Intuition**: The open sets encode "proximity" or "nearness" without requiring a specific distance. A point p is "near" a set A if every open set containing p intersects A. The open sets tell you which subsets are "open" — intuitively, which subsets include a full neighborhood around each of their points.

**Examples**:
- **Discrete topology**: τ = 2^X (every subset is open). Every function to/from a discrete space is continuous. This is the "finest" possible topology.
- **Indiscrete topology**: τ = {∅, X} (only the empty set and X are open). This is the "coarsest" possible topology — almost no distinction between points.
- **Standard topology on ℝⁿ**: The open sets are arbitrary unions of open balls B_ε(x) = {y : |y − x| < ε}. This is the topology we have been implicitly using throughout this textbook.
- **Subspace topology**: If (X, τ) is a topological space and A ⊂ X, the **subspace topology** on A is τ_A = {U ∩ A : U ∈ τ}. This makes A into a topological space in its own right.
- **Product topology**: If (X, τ_X) and (Y, τ_Y) are topological spaces, the **product topology** on X × Y has as basis all sets U × V with U ∈ τ_X and V ∈ τ_Y. The product ℝ × ℝ with the product topology is ℝ².

**Closed sets**: A subset A ⊂ X is **closed** if its complement X \ A is open. In ℝ, closed intervals [a, b] are closed sets; open intervals (a, b) are open sets; half-open intervals [a, b) are neither.

**Neighborhoods**: An **open neighborhood** of a point p is any open set U with p ∈ U.

---

## 13.1.2 Continuous Maps

**Definition**: A function f: X → Y between topological spaces is **continuous** if for every open set V ⊂ Y, the preimage f⁻¹(V) = {x ∈ X : f(x) ∈ V} is open in X.

This generalizes the ε-δ definition: in ℝ, f is continuous at p iff for every open interval (f(p)−ε, f(p)+ε), the preimage contains an open interval around p. The topological definition makes this work without reference to distance.

**Equivalent formulations**: f: X → Y is continuous iff:
- For every closed set F ⊂ Y, f⁻¹(F) is closed in X.
- For every p ∈ X and every neighborhood V of f(p), there exists a neighborhood U of p with f(U) ⊂ V.

**Homeomorphism**: A bijection f: X → Y is a **homeomorphism** if both f and f⁻¹ are continuous. Homeomorphic spaces are topologically identical — they have the same "topological structure."

**Examples of homeomorphisms**:
- The real line ℝ is homeomorphic to any open interval (a, b) (via x ↦ arctan(x) stretched to (a, b)).
- The open ball {|x| < 1} in ℝⁿ is homeomorphic to ℝⁿ.
- The sphere S¹ is homeomorphic to ℝ/ℤ (the circle).
- The sphere S² is not homeomorphic to ℝ² (one is compact, the other is not — see Section 13.3).
- A coffee cup is homeomorphic to a torus (both have genus 1 — one handle/hole).

**Topological invariants**: Properties preserved by homeomorphisms. Examples: connectedness, compactness, number of connected components, fundamental group, homology groups. These are the "topological measurements" that distinguish non-homeomorphic spaces.

---

## 13.1.3 Separation Axioms

Topological spaces can have varying degrees of "separability" — the ability to distinguish points using open sets.

**T₀ (Kolmogorov)**: For any two distinct points x ≠ y, there exists an open set containing one but not the other.

**T₁ (Fréchet)**: For any two distinct points, each has an open neighborhood not containing the other. Equivalently, every singleton {x} is closed.

**T₂ (Hausdorff)**: For any two distinct points x ≠ y, there exist disjoint open neighborhoods U ∋ x and V ∋ y.

The Hausdorff condition is the most important. In a Hausdorff space:
- Limits of sequences are unique.
- Compact subsets are closed.
- The graph of a continuous function is closed.

**Smooth manifolds are Hausdorff**: The Hausdorff condition is a physical requirement for spacetime. If two distinct events p and q in spacetime could not be separated by open neighborhoods, we could not distinguish them by any local observation — they would be "the same event" from the viewpoint of any observer. The Hausdorff condition says distinct events are distinguishable.

**The non-Hausdorff line** (counterexample): Take two copies of ℝ and identify all points except 0. The resulting space "ℝ with two origins" has every point distinct, but the two origins cannot be separated. This non-Hausdorff space is excluded from being a manifold by requiring the Hausdorff axiom.

---

## 13.1.4 Connectedness

**Definition**: A topological space X is **connected** if it cannot be written as the disjoint union of two non-empty open sets.

Equivalently: the only subsets of X that are both open and closed (called **clopen**) are ∅ and X.

**Path-connectedness**: X is **path-connected** if for any two points x, y ∈ X, there exists a continuous path γ: [0, 1] → X with γ(0) = x and γ(1) = y.

Path-connected implies connected; the converse is false (the topologist's sine curve is connected but not path-connected).

**Simple connectivity**: X is **simply connected** if it is path-connected and every loop (continuous map S¹ → X) can be contracted to a point (is homotopic to a constant map). Intuitively: simply connected spaces have no "holes" that loops can surround.

**Examples**:
- ℝⁿ is simply connected. Any loop can be contracted to a point.
- ℝ² \ {0} is path-connected but not simply connected: the loop e^{iθ} for θ ∈ [0, 2π] around the origin cannot be contracted. The fundamental group π₁(ℝ² \ {0}) ≅ ℤ (integer winding numbers).
- S¹ is path-connected but not simply connected: π₁(S¹) ≅ ℤ.
- S² is simply connected: every loop on a sphere can be contracted.
- The torus T² has π₁(T²) ≅ ℤ × ℤ: two independent non-contractible loops (around the hole and around the tube).

**GR applications**:
- The exterior of a Schwarzschild black hole (r > r_s) is simply connected in spatial sections. There are no non-contractible loops outside.
- An Einstein-Rosen bridge (wormhole) would create a non-simply connected spatial topology: a loop threading the wormhole cannot be contracted. But the topological censorship theorem shows such loops are not accessible to physical observers if energy conditions hold.
- Whether the spatial sections of the universe are simply connected (ℝ³ topology, standard cosmology) or not (toroidal topology, cosmic crystallographic signatures) is an observational question.

---

## 13.1.5 The Quotient Topology

**Definition**: If (X, τ) is a topological space and ~ is an equivalence relation on X, the **quotient space** X/~ is the set of equivalence classes with the **quotient topology**: a set U ⊂ X/~ is open iff its preimage π⁻¹(U) ⊂ X is open, where π: X → X/~ is the canonical projection.

**Examples**:
- S¹ = [0, 1]/(0 ~ 1): take the interval [0, 1] and identify its endpoints.
- S² = D²/∂D²: take a disk and collapse its boundary to a point.
- The torus T² = [0,1]² / ~ where (x, 0) ~ (x, 1) and (0, y) ~ (1, y): identify opposite edges of a square.
- Minkowski spacetime mod time translation: ℝ^{1,3}/ℤ — a spacetime with periodic time. Such spacetimes would contain closed timelike curves.

**Topological quotients in GR**: Many interesting spacetimes arise as quotients. Anti-de Sitter space (AdS) admits identifications that produce spacetimes with closed timelike curves. The BTZ black hole (Bañados, Teitelboim, Zanelli, 1992) is a quotient of AdS₃. The identification of points creates the horizon. Topology change in spacetime (which would require cutting and gluing — forming a new quotient space) requires either singularities or the violation of energy conditions.

---

## References

- Munkres, J.R. (2000). *Topology*, 2nd ed. Prentice Hall. [The standard undergraduate reference. Chapters 1–2: topological spaces, continuous functions, metric spaces; Chapters 3–4: connectedness and compactness.]
- Hatcher, A. (2002). *Algebraic Topology.* Cambridge University Press. Freely available at https://pi.math.cornell.edu/~hatcher/AT/ATpage.html. [The standard reference for algebraic topology: fundamental group, covering spaces, homology, cohomology.]
- Hawking, S.W. and Ellis, G.F.R. (1973). *The Large Scale Structure of Space-Time.* Cambridge University Press. [Chapter 2: the global structure of spacetime; topology of spacetime; causal structure. The standard reference for GR topology.]
- Friedman, J., Schleich, K., and Witt, D.M. (1993). "Topological censorship." *Physical Review Letters*, 71, 1486–1489. [Proves that the topology of spatial sections is not directly accessible to physical observers — handles are hidden behind horizons if energy conditions hold.]
- Bañados, M., Teitelboim, C., and Zanelli, J. (1992). "Geometry of the 2+1 black hole." *Physical Review Letters*, 69, 1849–1851. [The BTZ black hole: a 3D black hole arising as a quotient of AdS₃ space. An elegant example of topology creating a black hole horizon.]
