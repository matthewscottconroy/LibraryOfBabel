# Section 13.2: Metric Spaces and Completeness

---

## Section Introduction

A **metric space** is a set with a notion of distance — a function d(x, y) measuring how far apart two points are. This is the setting for most of analysis, and it generalizes both ℝⁿ (Euclidean distance) and the spaces of functions that arise in PDE theory (L² spaces, Sobolev spaces).

Metric spaces are topological spaces: the open sets are unions of open balls B_ε(x) = {y : d(x,y) < ε}. But metric spaces have more structure than general topological spaces — the distance function allows us to define Cauchy sequences and ask whether they converge. This is **completeness**, the property we have been implicitly using throughout this text (in the Banach fixed-point theorem, in the construction of the reals from rationals, in the completeness of L² spaces).

For GR, the analogue of a metric space is a pseudo-Riemannian manifold — a manifold where the "distance function" is replaced by the metric tensor g_{μν}, which is allowed to be indefinite (it has Lorentzian signature). The spacetime metric is not a genuine metric in the sense of this section (since d(x,y) could be zero for x ≠ y along a null geodesic), but the mathematical analysis of metric spaces provides the conceptual foundation.

---

## 13.2.1 Metric Spaces

**Definition**: A **metric space** is a pair (X, d) where X is a set and d: X × X → ℝ satisfies:
1. **Positivity**: d(x, y) ≥ 0, and d(x, y) = 0 iff x = y.
2. **Symmetry**: d(x, y) = d(y, x).
3. **Triangle inequality**: d(x, z) ≤ d(x, y) + d(y, z).

**Examples**:
- ℝⁿ with Euclidean distance d(x, y) = |x − y| = √(Σ(xⁱ − yⁱ)²).
- ℝⁿ with the taxicab metric d₁(x, y) = Σ|xⁱ − yⁱ|.
- Any normed vector space (V, ||·||): d(x, y) = ||x − y||.
- C([a, b]) with the sup metric: d(f, g) = sup_{x∈[a,b]}|f(x) − g(x)|. This is the metric in which uniform convergence is convergence in C([a, b]).
- C([a, b]) with the L² metric: d(f, g) = (∫_a^b |f−g|²)^{1/2}. This is the L² metric on the space of square-integrable functions.
- Discrete metric: d(x, y) = 0 if x = y, d(x, y) = 1 if x ≠ y. Every subset is open.
- The **Hausdorff metric** on compact subsets of a metric space: d_H(A, B) = inf{ε : A ⊂ B_ε and B ⊂ A_ε}. The space of fractal sets with the Hausdorff metric is a natural setting for iterated function systems.

**Convergence**: A sequence (xₙ) in (X, d) **converges** to x if d(xₙ, x) → 0 as n → ∞.

**Continuity**: f: (X, d_X) → (Y, d_Y) is continuous at p iff for every ε > 0 there exists δ > 0 such that d_X(x, p) < δ implies d_Y(f(x), f(p)) < ε. This is the ε-δ definition, generalized to metric spaces.

---

## 13.2.2 Cauchy Sequences and Completeness

**Definition**: A sequence (xₙ) in (X, d) is a **Cauchy sequence** if for every ε > 0, there exists N such that d(xₙ, xₘ) < ε for all n, m > N.

Every convergent sequence is Cauchy (by the triangle inequality). The converse — every Cauchy sequence converges — is the definition of completeness.

**Definition**: A metric space is **complete** if every Cauchy sequence converges to a point in X.

**Examples**:
- ℝ with the standard metric is complete (this is the construction of the reals: ℝ = completion of ℚ).
- ℚ with the standard metric is NOT complete: the sequence 1, 1.4, 1.41, 1.414, ... is Cauchy but converges to √2 ∉ ℚ.
- C([a, b]) with the sup metric is complete (the Cauchy sequence criterion is equivalent to uniform convergence, and uniform limits of continuous functions are continuous).
- C([a, b]) with the L² metric is NOT complete: one can construct a Cauchy sequence of continuous functions converging in L² to a discontinuous function. The completion of C([a, b]) in the L² metric is L²([a, b]) — the space of square-integrable functions.

**Completion**: Every metric space (X, d) has a unique completion X̄ — a complete metric space containing X as a dense subset, such that X̄ is the "smallest" complete space containing X. The completion is constructed by adding the limits of all Cauchy sequences.
- ℝ = completion of ℚ.
- L²([a,b]) = completion of C([a,b]) in the L² metric.
- The **Sobolev space** H^k(Ω) = completion of C∞(Ω) in the W^{k,2} norm.

---

## 13.2.3 Banach Spaces and Hilbert Spaces

A **normed space** is a vector space V with a norm ||·|| satisfying: ||v|| ≥ 0 (= 0 iff v = 0), ||αv|| = |α| ||v||, ||v + w|| ≤ ||v|| + ||w||. The norm induces a metric d(v, w) = ||v − w||.

**Banach space**: A complete normed space. Examples:
- ℝⁿ with the Euclidean norm.
- C([a,b]) with the sup norm.
- Lᵖ(Ω) with the Lᵖ norm ||f||_p = (∫|f|ᵖ)^{1/p}.
- Sobolev spaces W^{k,p}(Ω).

**Hilbert space**: A Banach space whose norm comes from an inner product: ||v||² = ⟨v, v⟩. Examples:
- ℝⁿ with the dot product.
- L²(Ω) with ⟨f, g⟩ = ∫ f(x) g(x) dx.
- The space of quantum states in quantum mechanics.

Hilbert spaces are the natural infinite-dimensional generalization of Euclidean space. The spectral theorem for self-adjoint operators on Hilbert spaces is the infinite-dimensional analogue of the spectral theorem for symmetric matrices — it gives a complete orthonormal basis of eigenfunctions, generalizing the normal mode expansion of the previous chapter.

**GR connections**:
- The space of solutions to the linearized Einstein equations (gravitational wave modes) forms a Hilbert space, with inner product given by the symplectic structure on the phase space.
- The Hilbert space of quantum states in QFT in curved spacetime (needed for Hawking radiation) requires careful construction, because different observers may define different vacua (the "vacuum ambiguity" problem).
- The ADM phase space for GR — the space of solutions to the constraint equations — is an infinite-dimensional manifold modeled on Sobolev spaces.

---

## 13.2.4 The Baire Category Theorem

**Definition**: A set E in a topological space X is:
- **Nowhere dense** if its closure Ē has empty interior: int(Ē) = ∅. Intuitively, E does not "fill up" any open set.
- **Meager** (first category) if it is a countable union of nowhere dense sets.
- **Nonmeager** (second category) if it is not meager.

**Baire Category Theorem**: A complete metric space is nonmeager in itself. Equivalently: the intersection of countably many dense open sets is dense.

*Proof sketch*: Suppose X = ∪ Eₙ where each Eₙ is nowhere dense. Construct nested closed balls B₁ ⊃ B₂ ⊃ ... where Bₙ ∩ Eₙ = ∅ and radius → 0. By completeness, the intersection ∩Bₙ is non-empty. But this point is not in any Eₙ, contradicting X = ∪Eₙ. □

**Applications**:
1. **ℝ is uncountable**: If ℝ were countable, it would be a countable union of singletons {x}, each nowhere dense. But Baire says ℝ is nonmeager — contradiction.
2. **Uniform boundedness principle** (Banach-Steinhaus): If a family of bounded linear operators T_α: X → Y (X a Banach space) satisfies sup_α ||T_α(x)|| < ∞ for each x, then sup_α ||T_α|| < ∞. This is used constantly in functional analysis.
3. **Existence of nowhere-differentiable continuous functions**: The set of continuous functions on [0,1] that are differentiable at even one point is meager in C([0,1]) — so "almost all" continuous functions (in the Baire category sense) are nowhere differentiable. The Weierstrass function of Chapter 4 is the rule, not the exception.
4. **Open mapping theorem**: A surjective continuous linear map between Banach spaces is an open map. This is used to prove the inverse function theorem in infinite dimensions.

The Baire category theorem is not about probability (measure) but about "topological size": meager sets are "topologically small" even if they have full measure.

---

## 13.2.5 Geodesic Distance and Completeness in Riemannian Geometry

For a Riemannian manifold (M, g), the **geodesic distance** between two points p and q is:

$$d(p, q) = \inf_\gamma \int_a^b \sqrt{g_{\gamma(t)}(\dot\gamma(t), \dot\gamma(t))} \, dt$$

where the infimum is over all smooth paths from p to q. This is a genuine metric on M (positive, symmetric, triangle inequality). The topology induced by this metric coincides with the manifold topology.

**Hopf-Rinow theorem**: For a connected Riemannian manifold (M, g):
1. **Geodesic completeness** (every geodesic can be extended to all parameter values) is equivalent to
2. **Metric completeness** (M is complete as a metric space, i.e., Cauchy sequences converge) is equivalent to
3. **Closed and bounded subsets of M are compact**.

Moreover, if M is complete, any two points can be connected by a minimizing geodesic.

**GR relevance**: Spacetime in GR is not Riemannian but pseudo-Riemannian (Lorentzian signature). The Hopf-Rinow theorem fails for Lorentzian manifolds: geodesic completeness and metric completeness are different, and there may be no minimizing geodesic between two points. More importantly, geodesic **incompleteness** in GR — the inability to extend all geodesics to arbitrarily large parameter values — is the definition of a **singularity**. The Penrose-Hawking singularity theorems prove that certain spacetimes are geodesically incomplete under physically reasonable conditions.

---

## References

- Munkres, J.R. (2000). *Topology*, 2nd ed. Prentice Hall. [Chapter 2.7 on complete metric spaces; Chapter 4 on the Baire category theorem.]
- Rudin, W. (1976). *Principles of Mathematical Analysis*, 3rd ed. McGraw-Hill. [Chapter 2: metric spaces; Chapter 3: Cauchy sequences; Chapter 5: differentiation in metric spaces.]
- Kreyszig, E. (1978). *Introductory Functional Analysis with Applications.* Wiley. [Chapters 1–2: metric spaces and Banach spaces; accessible introduction to functional analysis for physicists.]
- Hopf, H. and Rinow, W. (1931). "Über den Begriff der vollständigen differentialgeometrischen Fläche." *Commentarii Mathematici Helvetici*, 3, 209–225. [The Hopf-Rinow theorem: geodesic completeness ↔ metric completeness for Riemannian manifolds.]
- Hawking, S.W. and Penrose, R. (1970). "The singularities of gravitational collapse and cosmology." *Proceedings of the Royal Society A*, 314, 529–548. [Uses geodesic incompleteness as the definition of a spacetime singularity and proves the most general singularity theorem.]
