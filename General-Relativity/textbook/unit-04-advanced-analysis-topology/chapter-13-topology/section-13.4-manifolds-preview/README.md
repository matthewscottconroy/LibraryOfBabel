# Section 13.4: Manifolds: A First Look

---

## Section Introduction

A **smooth manifold** is a topological space that locally looks like ℝⁿ, with smooth transitions between local coordinate systems. This is the mathematical arena of GR: spacetime is a 4-dimensional smooth manifold equipped with a Lorentzian metric.

The definition carefully separates what is intrinsic to the space (its topology, its smooth structure) from what depends on a choice of coordinates. This separation — between geometric objects and their coordinate representations — is the philosophical foundation of GR's general covariance.

---

## 13.4.1 Topological Manifolds

**Definition**: An n-dimensional **topological manifold** M is a Hausdorff, second-countable topological space such that every point p ∈ M has a neighborhood U homeomorphic to an open subset of ℝⁿ.

The homeomorphism φ: U → φ(U) ⊂ ℝⁿ is a **coordinate chart**. The pair (U, φ) assigns "coordinates" to points in U: if p ∈ U, then φ(p) = (x¹(p), ..., xⁿ(p)) ∈ ℝⁿ.

**Atlas**: A collection of charts {(U_α, φ_α)} such that the U_α cover M: ∪_α U_α = M.

**Transition maps**: If two charts (U_α, φ_α) and (U_β, φ_β) overlap (U_α ∩ U_β ≠ ∅), the **transition map** is:

$$\phi_\beta \circ \phi_\alpha^{-1}: \phi_\alpha(U_\alpha \cap U_\beta) \to \phi_\beta(U_\alpha \cap U_\beta)$$

This is a map between open subsets of ℝⁿ — so we can ask whether it is smooth (C∞).

---

## 13.4.2 Smooth Manifolds

**Definition**: A **smooth manifold** is a topological manifold with an atlas in which all transition maps are C∞ (smooth). The **smooth structure** is the equivalence class of compatible atlases.

Two atlases are compatible if their union is also an atlas (all transition maps are smooth). By Zorn's lemma, there is a maximal atlas — a **maximal smooth atlas** or **differentiable structure**.

**Examples**:
- ℝⁿ itself, with one chart (U = ℝⁿ, φ = identity).
- The sphere Sⁿ ⊂ ℝⁿ⁺¹: covered by two charts (stereographic projections from north and south poles). The transition map is smooth.
- Any smooth surface in ℝ³.
- The spacetime manifold M in GR: a 4D smooth manifold with a Lorentzian metric.

**Non-example**: Two copies of ℝ glued at all points except the origin — this is a topological manifold (every point has a neighborhood homeomorphic to ℝ) but it is not Hausdorff (the two "origins" cannot be separated). The Hausdorff condition is a physical requirement: two distinct spacetime events should be topologically distinguishable.

---

## 13.4.3 The Tangent Space

At each point p ∈ M, we want to define the "direction space" — the space of tangent vectors. In ℝⁿ, this is just ℝⁿ itself. On a manifold, it requires more care because the tangent vectors must be defined intrinsically (without reference to an ambient space).

**Definition** (tangent vector via curves): A **tangent vector** at p is an equivalence class of smooth curves γ: (−ε, ε) → M with γ(0) = p, where two curves are equivalent if they have the same velocity in every chart: d/dt(φ ∘ γ)(0) is the same for both.

**Equivalently** (tangent vector as derivation): A tangent vector v at p is a linear map v: C∞(M) → ℝ satisfying the Leibniz rule: v(fg) = v(f)g(p) + f(p)v(g). This is the "directional derivative in direction v."

**The tangent space** T_pM at p is the set of all tangent vectors at p. It is an n-dimensional vector space.

**In coordinates**: If (U, φ = (x¹, ..., xⁿ)) is a chart, the **coordinate basis vectors** at p are:

$$\frac{\partial}{\partial x^i}\bigg|_p \quad (i = 1, \ldots, n)$$

These form a basis for T_pM. Every tangent vector has the form v = vⁱ ∂/∂xⁱ|_p (Einstein summation).

**Transformation of tangent vectors**: Under a coordinate change x' = x'(x), the basis transforms as ∂/∂x'^i = (∂x^j/∂x'^i) ∂/∂x^j. The components transform contravariantly: v'^i = (∂x'^i/∂x^j) v^j — the Jacobian of the coordinate transformation. This is the definition of a contravariant vector.

---

## 13.4.4 Smooth Maps, the Pushforward, and Pullback

A **smooth map** f: M → N between smooth manifolds is a map such that φ_N ∘ f ∘ φ_M⁻¹ is smooth for all charts.

**The pushforward** (differential): df_p: T_pM → T_{f(p)}N is the linear map sending tangent vectors to tangent vectors. In coordinates:

$$df_p\left(\frac{\partial}{\partial x^i}\bigg|_p\right) = \frac{\partial f^j}{\partial x^i}(p) \frac{\partial}{\partial y^j}\bigg|_{f(p)}$$

This is exactly the Jacobian matrix of f at p — the same object as the total derivative of Chapter 7, now interpreted as a map between tangent spaces.

**The pullback**: For a function g: N → ℝ, the **pullback** f*g = g ∘ f is a function on M. For a 1-form ω on N, the pullback f*ω is a 1-form on M: (f*ω)(v) = ω(df_p(v)).

**In GR**: The Levi-Civita connection, the Riemann tensor, the covariant derivative — all are built from these primitives (tangent space, pushforward, smooth maps). The metric gᵤᵥ is a smooth (0,2) tensor field on M: a smooth assignment of an inner product to each T_pM. Everything else follows.

---

## References

- Milnor, J.W. (1956). "On manifolds homeomorphic to the 7-sphere." *Annals of Mathematics*, 64, 399–405. [The discovery of exotic spheres — topologically S⁷ but with different smooth structures than the standard one. Shows smooth structure is more subtle than topology.]
- Munkres, J.R. (2000). *Topology*, 2nd ed. Prentice Hall. [Comprehensive point-set topology reference.]
- Nakahara, M. (2003). *Geometry, Topology and Physics*, 2nd ed. IOP Publishing. [Chapter 5 on manifolds; rigorous and physically motivated.]
- Spivak, M. (1979). *A Comprehensive Introduction to Differential Geometry*, Vol. 1, 2nd ed. Publish or Perish. [The most thorough treatment of manifolds available; Chapter 2 on the definition of a manifold.]
- Wald, R.M. (1984). *General Relativity*. University of Chicago Press. [Appendix A: a concise rigorous treatment of smooth manifolds in the context of GR.]
