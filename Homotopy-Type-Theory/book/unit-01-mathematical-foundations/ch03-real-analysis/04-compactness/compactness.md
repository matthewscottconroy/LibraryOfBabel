# Compactness

## The Most Useful Concept in Analysis

If you had to choose one analytic concept to keep — one property of spaces that makes theorems provable and computations tractable — it would be compactness. Compact spaces behave like finite sets in a remarkable number of ways. Continuous functions on compact spaces achieve their maxima. Sequences in compact spaces have convergent subsequences. Covers of compact spaces have finite subcovers. Every one of these "finite-like" properties is the content of an important theorem, and they hold precisely because of compactness.

**Definition.** A metric space X is *compact* if every open cover has a finite subcover. That is: if {Uα}_{α ∈ I} is a collection of open sets with X = ⋃_α Uα, then there exists a finite subcollection Uα₁, ..., Uαₙ with X = Uα₁ ∪ ... ∪ Uαₙ.

This definition is topological — it is stated entirely in terms of open sets, not metrics. It extends to all topological spaces, not just metric spaces.

## Why Finite Subcovers?

The definition sounds technical. Let us see why it captures "finite-like" behavior.

Suppose X is compact and f: X → ℝ is continuous. We want to show f is bounded. Consider the open sets Uₙ = f⁻¹((-n, n)) for n = 1, 2, 3, .... These cover X (every x has a finite value f(x), so x ∈ Uₙ for sufficiently large n). Since X is compact, a finite subcover suffices: X = Uₙ₁ ∪ ... ∪ Uₙₖ for some n₁, ..., nₖ. But then X = f⁻¹((-n, n)) for n = max(n₁, ..., nₖ). So f maps X into (-n, n), meaning f is bounded.

The argument uses the finite subcover to extract a single bound that works globally. This is the pattern: start with an infinite family of local facts, use compactness to reduce to a finite family, combine the finite family into a global conclusion.

## Heine-Borel Theorem

**Theorem (Heine-Borel).** A subset K ⊆ ℝⁿ is compact if and only if K is closed and bounded.

*Proof (the key direction: closed and bounded → compact).*

First, prove [a, b] is compact (in ℝ). Suppose {Uα} covers [a, b]. Let S = {x ∈ [a, b] | [a, x] has a finite subcover}. S is non-empty (a ∈ S, since a is in some Uα). Let c = sup S.

We claim c ∈ S and c = b. Since c ∈ [a, b], there is some Uα₀ containing c, and Uα₀ is open, so some interval (c - δ, c + δ) ⊆ Uα₀. By definition of sup, there exists x ∈ S with x > c - δ. Then [a, x] has a finite cover F, and [a, c] ⊆ [a, x] ∪ (c - δ, c + δ) ⊆ (finite cover F) ∪ {Uα₀}. So c ∈ S.

If c < b, then similarly c + δ/2 ∈ S (extend the cover by Uα₀), contradicting sup. So c = b. □

For ℝⁿ: a closed bounded subset of ℝⁿ is a closed subset of a closed cube [a, b]ⁿ. Closed subsets of compact sets are compact. Finite products of compact sets are compact (proved below). So [a, b]ⁿ is compact, and K ⊆ [a, b]ⁿ closed gives K compact.

## Sequential Compactness

**Definition.** A metric space X is *sequentially compact* if every sequence has a convergent subsequence.

**Theorem.** For metric spaces, compactness and sequential compactness are equivalent.

*Proof sketch.*

(Compact → Sequentially Compact) Let (xₙ) be a sequence in compact X. If the sequence has infinitely many distinct values, we can extract a subsequence converging to some limit — the compactness gives us the limit. (Details use the fact that compact metric spaces are separable.)

(Sequentially Compact → Compact) Suppose X is not compact. Then there is an open cover with no finite subcover. Use sequential compactness to derive a contradiction by constructing a sequence with no convergent subsequence. □

For metric spaces, the equivalence means we can use whichever definition is more convenient. Sequences are often easier to work with concretely.

**Bolzano-Weierstrass Theorem.** Every bounded sequence in ℝⁿ has a convergent subsequence.

This is the sequential compactness of closed bounded subsets of ℝⁿ. It is one of the most useful results in analysis, underlying the proof of many existence theorems.

## Products and Subsets

**Theorem.** Closed subsets of compact spaces are compact.

*Proof.* Let K be compact and F ⊆ K closed. Let {Uα} be an open cover of F. Then {Uα} ∪ {K \ F} (adding the complement of F, which is open in K) covers K. Take a finite subcover of K: say Uα₁, ..., Uαₙ, K \ F. Remove K \ F — the remaining Uα₁, ..., Uαₙ cover F. □

**Theorem.** Continuous images of compact sets are compact.

*Proof.* Let f: X → Y be continuous and X compact. Let {Vα} cover f(X). Then {f⁻¹(Vα)} covers X. Take a finite subcover f⁻¹(Vα₁), ..., f⁻¹(Vαₙ). Then Vα₁, ..., Vαₙ cover f(X). □

**Tychonoff's Theorem.** The product of any family of compact spaces is compact.

For finite products, this is straightforward. For infinite products (even uncountably infinite), it requires the Axiom of Choice and is equivalent to AC. The Tychonoff theorem is one of the cornerstones of functional analysis and topology.

## Compactness and Analysis

**Extreme Value Theorem.** Continuous f: X → ℝ on compact X achieves its maximum and minimum.

*Proof.* f(X) is compact (continuous image of compact). Compact subsets of ℝ are closed and bounded (Heine-Borel). Bounded non-empty sets have suprema; the closed condition ensures the supremum is achieved. □

**Heine-Cantor Theorem.** Continuous f: X → Y on compact X is uniformly continuous.

*Proof.* Given ε > 0, for each x ∈ X find δ(x) > 0 with d_X(x', x) < δ(x) implying d_Y(f(x'), f(x)) < ε/2. The balls B(x, δ(x)/2) cover X. Take a finite subcover with centers x₁, ..., xₙ and let δ = min(δ(x₁)/2, ..., δ(xₙ)/2). If d_X(x, x') < δ, then x is in some B(xᵢ, δ(xᵢ)/2), and d_X(x', xᵢ) ≤ d_X(x', x) + d_X(x, xᵢ) < δ + δ(xᵢ)/2 ≤ δ(xᵢ). So d_Y(f(x), f(xᵢ)) < ε/2 and d_Y(f(x'), f(xᵢ)) < ε/2, giving d_Y(f(x), f(x')) < ε. □

**Arzelà-Ascoli Theorem.** A family F of functions f: [a,b] → ℝ is precompact (has compact closure) in C([a,b]) if and only if F is *uniformly bounded* (|f(x)| ≤ M for all f ∈ F and x ∈ [a,b]) and *equicontinuous* (for every ε > 0, there exists δ > 0 such that |x-y| < δ implies |f(x)-f(y)| < ε for *all* f ∈ F simultaneously).

Arzelà-Ascoli is fundamental in the theory of differential equations: to prove the existence of a solution, one often constructs a sequence of approximate solutions and uses Arzelà-Ascoli to extract a convergent subsequence.

## Compactness in HoTT

Compactness has a type-theoretic analogue, though it is more subtle. In HoTT, the appropriate notion depends on the level of truncation.

For sets (0-truncated types), a set A is *compact* if every surjective map from A to a discrete type factors through a finite quotient. This captures the finite-subcover property at the level of discrete geometry.

For more general types, compactness relates to the finiteness properties of maps — whether a map has finitely many "sheets" over each point, in the covering space sense.

The most relevant connection: the theorem that every compact metric space is separable (has a countable dense subset) implies that compact metric spaces can be described by countable data — a fact that makes them computationally tractable. In constructive mathematics, this is the foundation for computable analysis: a real number is "computable" if it is the limit of a computable sequence of rationals, and compact metric spaces are those where computable approximation is possible.

Compactness, ultimately, is the mathematical formalization of "finite in spirit." In a compact space, the global can always be deduced from the local — and that is the deepest reason why compactness appears everywhere.
