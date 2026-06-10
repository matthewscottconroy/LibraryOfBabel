# Continuity

## What Continuity Means

A function is continuous if it "doesn't jump" — if small changes in the input produce small changes in the output. This intuition is exactly right. The ε-δ definition formalizes it: for any size of output change you demand (ε), there is a size of input change (δ) that guarantees it.

We defined continuous functions in the metric space section. Here we develop the theory: properties of continuous functions, the key theorems (Intermediate Value Theorem, Extreme Value Theorem), uniform continuity, and the connection to homotopy.

## Operations on Continuous Functions

**Theorem.** Compositions, sums, products, and quotients of continuous functions are continuous (where defined).

*Proof of composition.* If f: X → Y and g: Y → Z are continuous and x₀ ∈ X, we want: for every ε > 0, find δ > 0 such that d_X(x, x₀) < δ implies d_Z(g(f(x)), g(f(x₀))) < ε.

Since g is continuous at f(x₀), find η > 0 such that d_Y(y, f(x₀)) < η implies d_Z(g(y), g(f(x₀))) < ε.

Since f is continuous at x₀, find δ > 0 such that d_X(x, x₀) < δ implies d_Y(f(x), f(x₀)) < η.

Combining: d_X(x, x₀) < δ ⟹ d_Y(f(x), f(x₀)) < η ⟹ d_Z(g(f(x)), g(f(x₀))) < ε. □

**Equivalent characterizations of continuity:**
- f is continuous iff for every sequence xₙ → x₀ in X, f(xₙ) → f(x₀) in Y.
- f is continuous iff for every closed set F ⊆ Y, f⁻¹(F) is closed in X.
- f is continuous iff for every x₀ and every open neighborhood V of f(x₀), there is an open neighborhood U of x₀ with f(U) ⊆ V.

The sequential characterization (xₙ → x₀ implies f(xₙ) → f(x₀)) is often easiest to use in practice.

## Intermediate Value Theorem

**Theorem (IVT).** Let f: [a, b] → ℝ be continuous, and suppose f(a) < c < f(b) (or f(b) < c < f(a)). Then there exists x ∈ (a, b) with f(x) = c.

*Proof.* Assume f(a) < c < f(b). Let S = {x ∈ [a,b] | f(x) ≤ c}. S is non-empty (a ∈ S) and bounded above (by b). Let x* = sup S. We claim f(x*) = c.

f(x*) ≤ c: For each n, there exists xₙ ∈ S with x* - 1/n < xₙ ≤ x*. So xₙ → x*, and by continuity f(xₙ) → f(x*). Since f(xₙ) ≤ c for all n, we get f(x*) ≤ c.

f(x*) ≥ c: For each n, x* + 1/n > sup S, so x* + 1/n ∉ S (when x* + 1/n ≤ b), meaning f(x* + 1/n) > c. Taking n → ∞: f(x*) ≥ c.

Both together: f(x*) = c. □

**Consequences:**
- Every continuous function from [0,1] to [0,1] has a fixed point (Brouwer fixed point theorem in dimension 1).
- Every polynomial of odd degree has a real root.
- Between any two distinct values of a continuous function on a connected domain, all intermediate values are achieved.

The IVT is a theorem about *connected* spaces: it fails for disconnected domains. (A continuous function on {0} ∪ {1} ⊆ ℝ — two isolated points — can jump.) The connectedness of [a,b] is the essential ingredient.

## Uniform Continuity

**Definition.** f: X → Y is *uniformly continuous* if for every ε > 0, there exists δ > 0 such that d_X(x, x') < δ implies d_Y(f(x), f(x')) < ε for *all* x, x' ∈ X simultaneously.

The difference from ordinary continuity: δ depends only on ε, not on the specific point x. Uniform continuity is a global property; ordinary continuity is local.

**Examples:**
- f(x) = x² is continuous on ℝ but not uniformly continuous: near x = 0, small changes produce small changes in f(x), but near x = 100, the same change in x produces a change of about 200 times that magnitude.
- f(x) = x² is uniformly continuous on any bounded interval [a, b] — the derivative is bounded there.
- f(x) = sin(x) is uniformly continuous on ℝ (the derivative is bounded: |sin'(x)| = |cos(x)| ≤ 1).

**Theorem (Heine-Cantor).** If f: X → Y is continuous and X is compact, then f is uniformly continuous.

This theorem is one of the reasons compactness matters: on compact domains, continuity automatically upgrades to uniform continuity, which has much stronger consequences for approximation and integration.

## The Extreme Value Theorem

**Theorem (EVT).** If f: X → ℝ is continuous and X is compact, then f achieves its maximum and minimum: there exist x*, x** ∈ X with f(x*) ≤ f(x) ≤ f(x**) for all x ∈ X.

*Proof sketch.* By Heine-Cantor, f is uniformly continuous. The image f(X) is a compact subset of ℝ (continuous images of compact sets are compact — proved in the compactness section). Compact subsets of ℝ are closed and bounded (Heine-Borel). A non-empty closed bounded subset of ℝ has a supremum that is attained (since the set is closed and the supremum is a limit point). □

The EVT is one of the fundamental tools of optimization: to find the maximum of a continuous function on a compact set, you need only look for critical points (where the derivative is zero) and boundary values. The maximum exists and is achieved — this is not trivial without compactness.

**Warning.** The EVT fails without compactness:
- f(x) = x on ℝ has no maximum.
- f(x) = x on (0, 1) approaches 1 but never achieves it.
- f(x) = 1/x on (0, 1] is unbounded.

## Homeomorphisms and Topological Invariants

A *homeomorphism* is a bijective continuous function with a continuous inverse. Homeomorphisms preserve all topological properties: connectedness, compactness, the fundamental group, homology groups.

**Topological invariants** are properties preserved by homeomorphism:
- Compactness: [0,1] is compact, ℝ is not. They are not homeomorphic.
- Connectedness: [0,1] is connected, {0} ∪ {1} is not.
- Dimension: ℝ and ℝ² are not homeomorphic (proved using dimension theory or homotopy groups).
- Fundamental group: S¹ and ℝ are not homeomorphic because π₁(S¹) = ℤ ≠ {e} = π₁(ℝ).

The fundamental group is the most powerful topological invariant from Chapter 2. Computing it from a topological description of a space — and using it to distinguish spaces — is the central tool of algebraic topology.

## Continuity and Paths

The connection to homotopy theory: a *path* is a continuous function γ: [0,1] → X. The definition requires nothing more than continuity. Every topological property of paths — homotopy between paths, composition of paths, the fundamental group — is built on the notion of continuity.

A *homotopy* between paths γ₀ and γ₁ from x to y is a continuous function H: [0,1] × [0,1] → X with H(t, 0) = γ₀(t), H(t, 1) = γ₁(t), H(0, s) = x, H(1, s) = y. The key word is *continuous*: the deformation must be a continuous function.

Why does continuity matter for homotopy? Because topological invariants are preserved under continuous deformation. If γ₀ and γ₁ are homotopic, they "look the same" topologically — they explore the same topological region of X. A discontinuous "deformation" could jump over a hole and would not preserve topological information.

In HoTT, the continuous paths in a topological space correspond to the elements of the identity type: a path γ: [0,1] → X from x to y corresponds to a term p: x =_X y. The homotopy H: [0,1]² → X corresponds to a term in the identity type of the identity type — a 2-path between paths. The hierarchy of continuity is the hierarchy of higher identity types.

Continuity is the glue of mathematics. It connects discrete choices (induction, recursion) to continuous structures (paths, deformations, flows). Understanding it deeply — through ε-δ definitions, topological characterizations, and its role in defining paths and homotopies — is the foundation for the topological intuition that HoTT formalizes.
