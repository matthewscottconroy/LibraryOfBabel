# Thought Experiments: Real Analysis

## 1. The Rational Analyst

Suppose you are a mathematician who lives in a world where only rational numbers exist. You have ℚ with its usual metric |p - q|, and you want to do analysis. You can define Cauchy sequences, and you can prove that every Cauchy sequence is "trying to converge." But some of them — like (1, 1.4, 1.41, 1.414, ...) — have no limit in your world.

You notice that whenever your analysis fails, it fails because of these "missing" numbers. You decide to add them: define a "complete rational world" by formally adjoining limits for all Cauchy sequences.

Question: what do you get? (You get ℝ.) But here is the deeper question: is this construction *natural*? You have taken ℚ and asked for the "best" complete extension. The result is ℝ. But you could have done something different: adjoin only limits of specific sequences (rational approximations to algebraic numbers), getting algebraic numbers. Or only computable sequences, getting computable reals.

Each choice gives a different complete metric space, with a different collection of "real numbers." In what sense is the completion of ℚ *the* real numbers? The answer involves the universal property: ℝ is the *unique* complete ordered field. Any other completion that is also an ordered field is isomorphic to ℝ.

This thought experiment motivates why universal properties matter: they pick out not just *a* construction but *the* canonical one, up to isomorphism. And in HoTT, "up to isomorphism" becomes "literally equal."

## 2. The Devil's Staircase

The Cantor function (or "devil's staircase") is defined on [0,1] as follows: on each interval removed in the Cantor set construction, the function takes the value k/2ⁿ (for the appropriate k and n). On the Cantor set itself, the function is defined by continuity.

The result: the Cantor function is continuous (!) and non-decreasing. It equals 0 at 0 and 1 at 1. But it is *constant on each removed interval* — its derivative is 0 almost everywhere. Yet it climbs from 0 to 1 without ever increasing on a set of positive measure.

Question: where does the "increase" happen? All the increase happens on the Cantor set itself — a set of measure zero. The function is continuous, non-decreasing, and yet its derivative is zero almost everywhere. It should be constant — but it isn't.

This seems paradoxical. The resolution: the Fundamental Theorem of Calculus (in the form "f(b) - f(a) = ∫_a^b f'(x) dx") requires f to be *absolutely continuous*, not just differentiable almost everywhere. The Cantor function fails absolute continuity.

What does this tell us about continuity? That being continuous is far from being "well-behaved" in the analytic sense. The space of continuous functions on [0,1] contains monstrosities: nowhere-differentiable functions, functions that increase on a measure-zero set, functions whose Fourier series diverge pointwise. The rich structure of C([0,1]) is both a resource and a warning.

## 3. The Topologist's Sine Curve

The topologist's sine curve: the closure in ℝ² of the graph of sin(1/x) for x ∈ (0, 1]. This is the graph of sin(1/x) together with the segment {0} × [-1, 1] (the y-axis from -1 to 1).

This set is connected: you cannot split it into two disjoint open sets, because the oscillating part of the curve comes arbitrarily close to every point on the segment.

But it is not path-connected: you cannot draw a continuous path from a point on the oscillating part to a point on the segment. To reach the segment, your path would have to "oscillate infinitely fast" at the moment it hits the segment — but a continuous function cannot do that.

Question: does this feel like a "real" mathematical object or a counterexample fabricated to break a theorem? Both. It is a real mathematical object — a subset of ℝ² — with the properties it has. And it is a counterexample: it shows that "connected" and "path-connected" are genuinely different concepts, that intuition about curves can mislead.

In HoTT: types can be "connected" (no non-trivial maps to Bool that separate the type) but not "path-connected" (not every pair of terms is connected by an identity proof). For sets (0-truncated types), these notions coincide with the analytic ones. For higher types, the distinction becomes more subtle.

## 4. Uniform Continuity and the Speed of Change

A function is uniformly continuous if its "speed of change" is globally bounded: given ε, there is a single δ that works everywhere. A function is continuous but not uniformly continuous if the speed of change can increase without bound.

The function f(x) = x² on ℝ is continuous but not uniformly continuous: near x = n for large n, f changes at approximately rate 2n, so the same change in x produces a 2n-fold larger change in f. You need smaller and smaller δ as x grows.

On [0, 1], f(x) = x² is uniformly continuous: the derivative is bounded by 2 on [0,1], so δ = ε/2 works everywhere on [0,1].

Question: is uniform continuity a property of the function or of the domain? The answer is: both. The same function can be uniformly continuous on one domain and not on another. Compactness is the key: on compact domains, continuity implies uniform continuity (Heine-Cantor). Non-compact domains can allow non-uniform continuity.

Deeper question: what does uniform continuity mean for paths? A path γ: [0,1] → X on a compact domain [0,1] is automatically uniformly continuous. This means paths have a uniform "speed" bound — they cannot oscillate infinitely fast. This uniform regularity is part of why paths are tractable in homotopy theory.

## 5. The Completion as a HIT

The completion of ℚ gives ℝ. The completion of a normed vector space gives a Banach space. The completion of an inner product space gives a Hilbert space. These are all instances of the same abstract construction.

In HoTT, the completion is modeled as a higher inductive type. Given ℚ (as a type with its ring structure), define ℝ as the HIT generated by:
- A term r : ℝ for each rational q : ℚ (embedding).
- A term lim(s) : ℝ for each Cauchy sequence s : ℕ → ℝ (formally adjoining limits).
- A path lim(c_r) = r for each constant Cauchy sequence (where c_r(n) = r for all n).
- A path lim(s) = lim(t) whenever s and t are equivalent Cauchy sequences.

Question: does this construction "work"? That is, does the resulting HIT satisfy the axioms for a complete ordered field? The answer is yes — but proving it requires showing the HIT satisfies all the ordered field axioms. This is a substantial piece of formalization, carried out in the HoTT book.

The existence of the HIT itself (by the principle of higher inductive types) and the proof of the axioms (by induction on the HIT) are separate tasks. The univalence axiom ensures that any two complete ordered fields are equal (not just isomorphic). So ℝ is the *only* complete ordered field in HoTT, not just "unique up to isomorphism."

## 6. ε and δ as Strategies in a Game

The ε-δ definition of continuity is often taught as a formula to memorize. But it has a game-theoretic interpretation that clarifies its structure.

The *ε-δ game* for "f is continuous at x₀":
- Player Ε (the Challenger) picks ε > 0 (how close the outputs must be).
- Player Δ (the Defender) picks δ > 0 (how close the inputs must be).
- The Challenger checks: is there any x with |x - x₀| < δ and |f(x) - f(x₀)| ≥ ε?

f is continuous at x₀ iff the Defender has a winning strategy: for every ε Ε picks, Δ can find δ such that no such x exists.

This game-theoretic framing is not just pedagogical — it connects to *verification games* in logic (the Ehrenfeucht-Fraïssé game for first-order equivalence) and to the theory of *computability* (can the Defender compute δ from ε?).

For *computable* continuity: f is computably continuous if the Defender's strategy is a computable function δ(ε). For uniform continuity: Δ is *independent of the Challenger's choice of x₀*. The modulus of uniform continuity ω(ε) = δ gives the Defender's winning strategy for all x₀ simultaneously.

In HoTT, the ε-δ definition can be formalized as a type: the type of proofs that f is continuous at x₀ contains a function ε ↦ (δ, proof-that-δ-works). The proof is a program. The program computes the Defender's strategy. Computable mathematics and proof theory meet.
