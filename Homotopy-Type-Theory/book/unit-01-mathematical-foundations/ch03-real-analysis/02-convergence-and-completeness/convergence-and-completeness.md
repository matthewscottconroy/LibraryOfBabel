# Convergence and Completeness

## Sequences and Limits

A *sequence* in a metric space (X, d) is a function from the natural numbers to X: (xₙ)_{n∈ℕ} = (x₀, x₁, x₂, ...).

**Definition.** The sequence (xₙ) *converges* to a limit L ∈ X if for every ε > 0, there exists N ∈ ℕ such that for all n ≥ N, d(xₙ, L) < ε.

We write xₙ → L or lim_{n→∞} xₙ = L.

**Reading the definition.** The definition says: no matter how small a tolerance ε > 0 you demand, the sequence eventually stays within ε of L. The challenge (the ε) is set by the reader; the response (finding N) must be provided by the prover.

This ε-N pattern is one of the most important logical structures in mathematics: ∀ε > 0. ∃N. ∀n ≥ N. P(ε, N, n). The alternation of quantifiers is exactly what makes limits precise — and what makes analysis challenging to formalize.

**Uniqueness.** If xₙ → L and xₙ → L', then L = L'. (Proof: for any ε > 0, d(L, L') ≤ d(L, xₙ) + d(xₙ, L') < ε/2 + ε/2 = ε for large enough n. Since ε was arbitrary, d(L, L') = 0, so L = L'.)

**Examples:**
- In ℝ: xₙ = 1/n → 0. Given ε > 0, take N > 1/ε. Then n ≥ N implies 1/n ≤ 1/N < ε.
- In ℝ: xₙ = (-1)ⁿ does not converge (oscillates between 1 and -1).
- In ℝ: xₙ = (1 + 1/n)ⁿ → e. Non-trivial to prove; uses the completeness of ℝ.
- In C([0,1]) with sup-norm: the sequence of functions fₙ(x) = xⁿ. Does this converge? Yes, pointwise (to 0 on [0,1) and to 1 at x=1), but the pointwise limit is not continuous. So (fₙ) does not converge in C([0,1]) — the limit would need to be in C([0,1]), and the pointwise limit is not continuous.

## Cauchy Sequences

A sequence can "try to converge" without having anything to converge to. In ℚ, the sequence 1, 1.4, 1.41, 1.414, ... is getting its terms closer and closer to each other — yet ℚ has no element that serves as the limit (because √2 ∉ ℚ). This motivates the notion of a Cauchy sequence.

**Definition.** A sequence (xₙ) is *Cauchy* if for every ε > 0, there exists N ∈ ℕ such that for all m, n ≥ N, d(xₘ, xₙ) < ε.

A Cauchy sequence is one where the terms are *mutually approaching* — the sequence becomes internally consistent — without necessarily approaching any specific limit in the space.

**Theorem.** Every convergent sequence is Cauchy.

*Proof.* Suppose xₙ → L. Given ε > 0, find N such that n ≥ N implies d(xₙ, L) < ε/2. Then for m, n ≥ N: d(xₘ, xₙ) ≤ d(xₘ, L) + d(L, xₙ) < ε/2 + ε/2 = ε. □

The converse fails in general: in ℚ, Cauchy sequences need not converge (as the √2 example shows). But in *complete* spaces, the converse holds.

## Completeness

**Definition.** A metric space X is *complete* if every Cauchy sequence in X converges to a point in X.

Completeness says: the space has no "gaps." If the terms of a sequence are getting mutually close, there must be something there for them to converge to.

**Examples:**
- ℝ is complete. This is the fundamental theorem of real analysis. Every Cauchy sequence of reals converges to a real number. The proof uses the completeness of ℝ as an axiom, or derives it from the construction of ℝ from Cauchy sequences.
- ℚ is not complete: the sequence of rational approximations to √2 is Cauchy in ℚ but has no limit in ℚ.
- ℝⁿ is complete (consequence of ℝ being complete).
- C([0,1]) with the sup-norm is complete (the *Weierstrass M-test* and related results).
- C([0,1]) with the L² norm (d(f,g) = √∫|f-g|²) is *not* complete — the completion is L²([0,1]), the space of square-integrable functions. This completion process is the construction of Hilbert spaces.
- Any discrete metric space is complete (Cauchy sequences are eventually constant).

**Theorem (Baire Category Theorem).** A complete metric space is *not* a countable union of nowhere-dense sets. Equivalently, the intersection of countably many dense open sets is dense.

This theorem, which seems abstract, has striking consequences. It implies: there exist continuous nowhere-differentiable functions (the set of differentiable functions is "small" in the sense of the Baire category). It implies: the reals cannot be written as a countable union of nowhere-dense sets (and in particular are uncountable by a separate argument).

## The Completion Theorem

Every metric space has a "completed version" — a universal way to add limits for all Cauchy sequences.

**Theorem (Completion).** For every metric space (X, d), there exists a complete metric space (X̂, d̂) and an isometric embedding i: X → X̂ such that i(X) is dense in X̂. Moreover, this completion is unique up to isometric isomorphism: any other completion is isometrically isomorphic to X̂.

The completion X̂ can be constructed as the set of equivalence classes of Cauchy sequences in X, where two Cauchy sequences are equivalent if d(xₙ, yₙ) → 0. The distance in X̂ is d̂([(xₙ)], [(yₙ)]) = lim_{n→∞} d(xₙ, yₙ) (which exists because (d(xₙ, yₙ))_{n∈ℕ} is Cauchy in ℝ).

**Examples:**
- The completion of ℚ (with the absolute value metric) is ℝ. This is *the* construction of the real numbers via Cauchy sequences.
- The completion of C([0,1]) with the L² metric is L²([0,1]).
- The completion of the space of smooth functions on a manifold with appropriate Sobolev norms gives the Sobolev spaces, the natural setting for the theory of partial differential equations.

**The Universal Property.** The completion satisfies a universal property: for any complete metric space (Y, d_Y) and any uniformly continuous function f: X → Y, there is a unique uniformly continuous extension f̂: X̂ → Y with f̂ ∘ i = f.

This is a universal property in the same form as the free group, the Dedekind reals, and every other canonical construction in mathematics. The completion is the "best possible" complete extension of X.

## Fixed Point Theorems

**Theorem (Banach Fixed Point / Contraction Mapping).** Let (X, d) be a complete metric space and f: X → X a *contraction*: there exists c ∈ (0, 1) with d(f(x), f(y)) ≤ c·d(x, y) for all x, y ∈ X. Then f has a unique fixed point x* ∈ X with f(x*) = x*, and for any x₀ ∈ X, the sequence x₀, f(x₀), f(f(x₀)), ... converges to x*.

*Proof sketch.* The orbit x₀, x₁ = f(x₀), x₂ = f(x₁), ... satisfies d(xₙ₊₁, xₙ) ≤ cⁿ d(x₁, x₀). By the geometric series, the sequence is Cauchy. By completeness, it converges to some x*. Taking limits in f(xₙ) = xₙ₊₁ gives f(x*) = x*. Uniqueness: if f(x) = x and f(y) = y, then d(x,y) = d(f(x), f(y)) ≤ c·d(x,y), so (1-c)d(x,y) ≤ 0, giving d(x,y) = 0. □

The Banach fixed point theorem is one of the most useful theorems in analysis. It proves:
- Existence and uniqueness of solutions to differential equations (by treating the differential equation as a fixed point problem).
- Convergence of Newton's method for finding roots.
- Existence of fractals (iterated function systems are contractions on complete metric spaces).

The pattern — completeness providing a limit, the limit being a fixed point — is broadly applicable. Completeness is the technical condition that makes the abstract existence arguments work.

## Completeness and HoTT

The connection between completeness and HoTT is through the *Cauchy real numbers* as a higher inductive type.

In HoTT, the real numbers ℝ_C can be defined as a higher inductive type: take ℚ as the base, add a constructor for "limit of a Cauchy sequence," and impose a path-constructor saying that any two Cauchy sequences with the same limit are equal. The resulting type is the Cauchy completion of ℚ in the type-theoretic sense.

The universal property of this HIT is: for any type T with a ℚ-algebra structure (a map ℚ → T preserving the arithmetic) and with the property that every Cauchy sequence in the image has a limit, there is a unique map ℝ_C → T extending the ℚ-map. This is the universal property of the completion, translated into type theory.

The Dedekind real numbers ℝ_D (defined via Dedekind cuts) have a different HIT description. That ℝ_C and ℝ_D are equivalent — that there is a type equivalence ℝ_C ≃ ℝ_D — is a theorem that requires choosing the right definitions and proving they characterize the same abstract structure. Under Univalence, this equivalence becomes an equality: ℝ_C = ℝ_D. The foundational puzzle that plagued ZFC-based mathematics — which construction *is* the real numbers — is resolved.
