# Section 13.3: Compactness and Its Consequences

---

## Section Introduction

**Compactness** is the topological property that corresponds, intuitively, to being "finite" or "bounded" in a topological sense. A compact space has no "escape routes" — sequences cannot wander off to infinity, continuous functions achieve their extrema, and open covers can always be reduced to finite covers. Every theorem you have ever proven about a closed interval [a, b] that felt like it needed "boundedness" probably used compactness.

In GR, compactness plays a decisive role. Compact spatial sections (a closed universe) have different topology from non-compact ones (open or flat universe). Compact Cauchy surfaces are related to the "global hyperbolicity" of spacetime (the property that makes the initial value problem well-posed). The Penrose singularity theorem uses compactness of trapped surfaces. And conformal compactification — which produces the Penrose diagrams of Section 12.4 — literally compactifies spacetime to study its global properties.

---

## 13.3.1 Compact Spaces

**Definition (open cover)**: A collection {U_α} of open sets is an **open cover** of X if X = ∪_α U_α.

**Definition (compactness)**: A topological space X is **compact** if every open cover of X has a **finite subcover**: a finite subcollection {U_{α₁}, ..., U_{α_n}} that still covers X.

**Intuition**: No matter how you try to cover X with open sets, you never need infinitely many of them. The compact space has "no infinitely fine structure" — finite information suffices to cover it.

**Examples**:
- [a, b] ⊂ ℝ is compact (the Heine-Borel theorem below).
- (0, 1) ⊂ ℝ is not compact: the cover {(1/n, 1) : n ≥ 1} has no finite subcover.
- ℝ is not compact: {(−n, n) : n ≥ 1} has no finite subcover.
- S^n (n-sphere) is compact.
- The torus T² = S¹ × S¹ is compact (product of compact spaces is compact — Tychonoff's theorem).
- Any closed bounded subset of ℝⁿ is compact (Heine-Borel theorem).

**Theorem** (Heine-Borel, 1872): A subset K ⊂ ℝⁿ is compact if and only if it is **closed and bounded**.

*Proof sketch*: (Closed + bounded → compact) Any open cover has a finite subcover, by a nested bisection argument. (Compact → closed) Any convergent sequence in K has its limit in K (by openness of complements). (Compact → bounded) The cover by unit balls has a finite subcover, bounding K. □

**Compact metric spaces**: In a metric space, compactness is equivalent to:
- **Sequential compactness**: every sequence has a convergent subsequence (Bolzano-Weierstrass for metric spaces).
- **Complete + totally bounded**: X is complete and for every ε > 0, X can be covered by finitely many ε-balls.

---

## 13.3.2 Consequences of Compactness

**Theorem** (Extreme Value Theorem, general version): If f: X → ℝ is continuous and X is compact, then f attains its maximum and minimum on X.

*Proof*: f(X) is a compact subset of ℝ, hence closed and bounded. Therefore sup f(X) ∈ f(X) and inf f(X) ∈ f(X). □

**Uniform continuity**: A continuous function on a compact metric space is uniformly continuous.

*Proof*: For any ε, cover X by open balls B_{δ(x)/2}(x) where δ(x) is the ε-response at x. Take a finite subcover; let δ = min{δ(xᵢ)/2}. Then d(x, y) < δ implies d(f(x), f(y)) < ε. □

**Continuity and compactness**: The continuous image of a compact set is compact.

**Compactness and limits**: In a compact Hausdorff space, every sequence (or net) has a cluster point. This is why compactness is so useful in analysis: you can always extract convergent subsequences.

**Arzela-Ascoli theorem**: A subset F ⊂ C([a,b]) is compact (in the sup norm) iff it is:
- **Uniformly bounded**: sup_{f ∈ F} sup_x |f(x)| < ∞.
- **Equicontinuous**: for every ε, there exists δ such that |x − y| < δ implies |f(x) − f(y)| < ε for all f ∈ F simultaneously.

This theorem is used to extract convergent subsequences of functions — a key tool in PDE theory and GR (e.g., proving existence of solutions by taking limits of approximate solutions).

---

## 13.3.3 Compact Surfaces and the Classification Theorem

A **compact surface** is a compact 2-dimensional manifold without boundary. These are completely classified:

**Theorem** (Classification of compact surfaces): Every compact connected orientable surface is homeomorphic to exactly one of:
- The sphere S² (genus 0, Euler characteristic χ = 2),
- The torus T² (genus 1, χ = 0),
- The connected sum of g tori #g T² (genus g, χ = 2 − 2g, g ≥ 2).

The **Euler characteristic** χ = V − E + F (vertices minus edges plus faces in any triangulation) is a topological invariant — it doesn't depend on the triangulation, only on the topology.

The **Gauss-Bonnet theorem**: For a compact oriented Riemannian surface Σ:

$$\int_\Sigma K \, dA = 2\pi \chi(\Sigma)$$

where K is the Gaussian curvature and χ is the Euler characteristic. This theorem connects a global topological quantity (χ, an integer) to a geometric quantity (∫K dA, a real number computed from the curvature).

**Generalization**: The Chern-Gauss-Bonnet theorem for a compact oriented Riemannian 2n-manifold M:

$$\int_M \text{Pfaff}(\Omega) = (2\pi)^n \chi(M)$$

where Pfaff(Ω) is a polynomial in the curvature 2-form. For n = 2 (a 4-manifold, like spacetime compactified): ∫ (K_{abcd}K^{abcd} − 4R_{ab}R^{ab} + R²) dV = 32π²χ.

---

## 13.3.4 Compactness in Spacetime: Cauchy Surfaces

In GR, the initial value problem for the Einstein equations is well-posed when the spacetime is **globally hyperbolic** — a condition with a topological flavor.

**Definition**: A spacetime (M, g) is **globally hyperbolic** if it satisfies:
1. Strong causality (no almost-closed causal curves): for every event p, there exist arbitrarily small neighborhoods U of p such that no causal curve leaves and re-enters U.
2. For every pair of events p, q, the set J⁺(p) ∩ J⁻(q) (the set of events in the causal future of p and causal past of q) is compact.

Condition 2 is the compactness condition: the "causal diamond" between any two events is compact. This prevents causal curves from "threading" holes in spacetime.

**Theorem** (Geroch, 1970): A spacetime is globally hyperbolic iff it admits a **Cauchy surface** — a spacelike hypersurface Σ such that every inextensible causal curve meets Σ exactly once.

A Cauchy surface is the "initial time slice" for the initial value problem. The Einstein equations, given initial data on Σ, determine the spacetime in the full domain of dependence D(Σ) = D⁺(Σ) ∪ D⁻(Σ).

**Topological consequence**: If (M, g) is globally hyperbolic, then M is homeomorphic to ℝ × Σ for a Cauchy surface Σ. The topology of spacetime is determined by the topology of its Cauchy surface.

**Compact vs. non-compact Cauchy surfaces**:
- If Σ is compact: "closed universe" — finite spatial volume. Cosmological models with compact spatial sections (FLRW with positive curvature, or toroidal spatial topology) have compact Cauchy surfaces.
- If Σ is non-compact: "open" or "flat" universe — infinite spatial extent. The standard ΛCDM model has non-compact Cauchy surfaces (ℝ³ topology).
- The observational question of which is realized in our universe remains open: current observations are consistent with both.

---

## 13.3.5 Trapped Surfaces and Penrose's Singularity Theorem

The most important application of compactness in GR is Penrose's singularity theorem (1965).

**Definition**: A **trapped surface** is a compact spacelike 2-surface S such that both families of outgoing null geodesics from S are converging — that is, the expansion θ < 0 for both the outgoing and ingoing null normals.

Normally, "outgoing" light from a 2-surface expands (θ > 0 for outgoing, θ < 0 for ingoing). A trapped surface has both families converging — gravity is so strong that even light is being pulled inward. This occurs inside the Schwarzschild horizon at r = r_s.

**Penrose Singularity Theorem (1965)**: Let (M, g) be a globally hyperbolic spacetime satisfying the null energy condition (R_{μν}k^μk^ν ≥ 0 for all null k^μ). If M contains a trapped surface S, then M is geodesically incomplete.

*Key ideas of proof*:
1. By the null energy condition and the Raychaudhuri equation, the expansion θ of a null congruence decreases at a rate dθ/dλ ≤ −θ²/2. If θ < 0 initially, θ → −∞ in finite affine parameter (the congruence focuses).
2. If the spacetime were complete (all null geodesics extendable to all λ > 0), then all null geodesics from S would reach a focal point in finite affine parameter.
3. After the focal point, the null geodesics from S are no longer in the boundary of J⁺(S) — points "beyond" the focal point can be reached by longer paths through the interior.
4. The boundary of J⁺(S) — the "edge of the future" of S — is compact (S was compact and the compactness of J⁺(p) ∩ J⁻(q) propagates). But the argument shows ∂J⁺(S) would have to be empty (if all null geodesics leave it) — contradicting compactness.
5. Therefore, the spacetime is geodesically incomplete.

**Significance**: The theorem uses no explicit solution — it applies to any spacetime satisfying the stated conditions. It shows that singularities are not artifacts of spherical symmetry (like the r = 0 singularity of Schwarzschild) but generic features of GR in the presence of sufficient matter concentration. Penrose shared the 2020 Nobel Prize in Physics partly for this result.

[Penrose, R. (1965). "Gravitational collapse and space-time singularities." *Physical Review Letters*, 14, 57–59.]

---

## References

- Munkres, J.R. (2000). *Topology*, 2nd ed. Prentice Hall. [Chapters 3–5: compactness, product spaces, the Tychonoff theorem.]
- Wald, R.M. (1984). *General Relativity.* University of Chicago Press. [Chapter 8: the singularity theorems; Sections 8.1–8.2 on the Raychaudhuri equation and causality; Section 8.3 on the Penrose singularity theorem. The standard GR reference for this material.]
- Hawking, S.W. and Ellis, G.F.R. (1973). *The Large Scale Structure of Space-Time.* Cambridge University Press. [Chapters 8–10: the singularity theorems, energy conditions, trapped surfaces. The comprehensive reference for GR global structure.]
- Penrose, R. (1965). "Gravitational collapse and space-time singularities." *Physical Review Letters*, 14, 57–59. [The original singularity theorem paper — 3 pages that changed GR. Every step uses the compactness of trapped surfaces.]
- Geroch, R. (1970). "Domain of dependence." *Journal of Mathematical Physics*, 11, 437–449. [Proves that global hyperbolicity is equivalent to the existence of a Cauchy surface, and establishes the topological structure ℝ × Σ of globally hyperbolic spacetimes.]
- Gauss, C.F. (1827). *Disquisitiones generales circa superficies curvas.* [The original Gauss-Bonnet theorem for surfaces; the beginning of the study of intrinsic curvature and its connection to topology.]
