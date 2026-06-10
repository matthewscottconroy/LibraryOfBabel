# Exercises: Real Analysis

## Section 1: Metric Spaces

**1.1.** (Routine) Verify that each of the following is a metric on the indicated set:
(a) d(x, y) = |x - y| on ℝ.  
(b) d(x, y) = max_{i} |xᵢ - yᵢ| on ℝⁿ.  
(c) d(x, y) = 0 if x = y, 1 if x ≠ y (discrete metric), on any set X.  
(d) d(f, g) = sup_{t∈[0,1]} |f(t) - g(t)| on C([0,1]).  

For each, verify all three axioms explicitly.

**1.2.** (Routine) Describe the open balls B(x, r) in each metric:
(a) ℝ with |x - y|.  
(b) ℝ² with the Euclidean metric.  
(c) ℝ² with the taxicab (L¹) metric.  
(d) ℝ² with the max (L∞) metric.  
Draw the unit balls (r = 1, x = origin) for (b), (c), (d).

**1.3.** (Standard) Prove: in any metric space, the intersection of two open sets is open. The union of any collection of open sets is open.

**1.4.** (Standard) Show that the open balls B(x, r) are indeed open sets. Show that closed balls {y | d(x,y) ≤ r} are closed.

**1.5.** (Standard) Give an example of a metric space where there is a sequence with no convergent subsequence (showing that not all metric spaces are sequentially compact).

**1.6.** (Proof) Prove: a function f: X → Y between metric spaces is continuous iff f⁻¹(U) is open in X for every open U in Y.

**1.7.** (Proof-level) The *product metric* on X × Y can be defined as d((x₁,y₁), (x₂,y₂)) = max(d_X(x₁,x₂), d_Y(y₁,y₂)). Prove:
(a) This is a metric on X × Y.
(b) The projection maps π₁: X × Y → X and π₂: X × Y → Y are continuous.
(c) A map f: Z → X × Y is continuous iff π₁ ∘ f and π₂ ∘ f are both continuous.

## Section 2: Convergence and Completeness

**2.1.** (Routine) In ℝ, find the limits of:
(a) aₙ = n/(n+1)  
(b) bₙ = (2n² + 1)/(n² + 3)  
(c) cₙ = (−1)ⁿ/n  
(d) dₙ = n sin(1/n)  

Prove each limit using the ε-N definition.

**2.2.** (Routine) Show that aₙ = 1/√n is Cauchy (without explicitly finding its limit, though you may use the limit to check).

**2.3.** (Standard) Prove that the sequence fₙ(x) = xⁿ on [0,1] does not converge in C([0,1]) with the sup-norm metric, even though it converges pointwise. What is the pointwise limit? Why does it fail to be in C([0,1])?

**2.4.** (Standard) Prove the Banach Fixed Point Theorem: if (X, d) is complete and f: X → X satisfies d(f(x), f(y)) ≤ c·d(x,y) for some c < 1, then f has a unique fixed point. Give an application: use the theorem to prove that the differential equation y' = y, y(0) = 1, has a unique solution on any bounded interval [0, T].

**2.5.** (Proof) Prove: the space C([0,1]) with the sup-norm is complete. (Sketch the argument: given a Cauchy sequence (fₙ) in C([0,1]), show (a) fₙ(x) is Cauchy in ℝ for each x, so converges to some f(x); (b) the convergence is uniform; (c) f is continuous.)

**2.6.** (Proof) Let X be a metric space and X̂ its completion. Prove the universal property: for any complete metric space Y and any uniformly continuous function f: X → Y, there is a unique uniformly continuous extension f̂: X̂ → Y with f̂ ∘ i = f (where i: X → X̂ is the embedding).

## Section 3: Continuity

**3.1.** (Routine) Use the ε-δ definition to prove:
(a) f(x) = 2x + 3 is continuous at every point of ℝ.  
(b) f(x) = x² is continuous at x = 2.  
(c) f(x) = 1/x is continuous at every x ≠ 0.  

**3.2.** (Standard) Prove: if f, g: X → ℝ are continuous, then max(f, g) and min(f, g) are continuous.

**3.3.** (Standard) Prove the sequential characterization of continuity: f: X → Y is continuous at x₀ iff for every sequence xₙ → x₀ in X, f(xₙ) → f(x₀) in Y.

**3.4.** (Proof) Prove the Intermediate Value Theorem in full, using the completeness of ℝ (as in the text). Identify precisely where completeness is used.

**3.5.** (Proof) Define a *homeomorphism* precisely. Prove that (0, 1) and ℝ are homeomorphic. Prove that [0, 1] and (0, 1) are *not* homeomorphic. (Hint for the latter: what property does [0,1] have that (0,1) lacks?)

**3.6.** (Proof-level) The *Borsuk-Ulam theorem* (in dimension 1): for any continuous f: S¹ → ℝ, there exists x ∈ S¹ with f(x) = f(-x). (Here S¹ is the unit circle and -x is the antipodal point.) Prove this using the IVT.

## Section 4: Compactness

**4.1.** (Routine) Determine which of the following subsets of ℝ are compact. Justify each answer using Heine-Borel.
(a) [0, 1]  
(b) (0, 1]  
(c) {1/n | n ≥ 1}  
(d) {1/n | n ≥ 1} ∪ {0}  
(e) ℝ  

**4.2.** (Standard) Prove the Extreme Value Theorem: a continuous function f: X → ℝ on a compact metric space X achieves its maximum and minimum. (Use the fact that continuous images of compact sets are compact and compact subsets of ℝ are closed and bounded.)

**4.3.** (Standard) Use the Bolzano-Weierstrass theorem to prove: every bounded sequence in ℝ has a convergent subsequence.

**4.4.** (Proof) Prove the Heine-Cantor theorem: a continuous function f: X → Y on a compact metric space X is uniformly continuous. (The proof uses the open cover definition of compactness.)

**4.5.** (Proof) A metric space is *sequentially compact* if every sequence has a convergent subsequence. Prove: compact metric spaces are sequentially compact. (Hint: an infinite sequence with infinitely many distinct values has an accumulation point in a compact space.)

**4.6.** (Proof-level) The *Lebesgue number lemma*: if {Uα} is an open cover of a compact metric space X, there exists δ > 0 (the Lebesgue number) such that every open ball B(x, δ) is contained in some Uα. Prove this. Then use it to give an alternative proof of the Heine-Cantor theorem.

## Section 5: Connectedness and Paths

**5.1.** (Standard) Prove: ℝⁿ is path-connected for any n ≥ 1. Prove: Sⁿ = {x ∈ ℝⁿ⁺¹ | |x| = 1} is path-connected for n ≥ 1.

**5.2.** (Standard) Prove: connected subsets of ℝ are intervals (possibly unbounded or degenerate). (The proof should show that a non-interval subset of ℝ is disconnected.)

**5.3.** (Proof) Prove: path concatenation is a continuous function. That is, if γ: [0,1] → X is a path from x to y and δ: [0,1] → X is a path from y to z, the concatenation (γ ∗ δ): [0,1] → X is continuous.

**5.4.** (Proof) Prove that homotopy (with fixed endpoints) is an equivalence relation on paths. That is:
(a) Every path is homotopic to itself.  
(b) If γ ≃ δ, then δ ≃ γ.  
(c) If γ ≃ δ and δ ≃ ε, then γ ≃ ε.  

**5.5.** (Proof) Prove that the fundamental group π₁(X, x₀) is indeed a group: verify that the multiplication [γ][δ] = [γ∗δ] is well-defined, associative, has an identity, and has inverses. For associativity, explicitly construct the homotopy between (γ ∗ δ) ∗ ε and γ ∗ (δ ∗ ε).

**5.6.** (Proof-level) A *simply connected* space is path-connected with trivial fundamental group. Prove: ℝⁿ is simply connected for all n ≥ 1. Prove: S¹ is not simply connected (exhibit a loop that is not null-homotopic; argue that the winding number obstructs null-homotopy).
