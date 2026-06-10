# Ordinals and Cardinals

## Two Kinds of Infinite Quantity

When we count a finite set, we do two things simultaneously: we impose an order on the elements (first, second, third...) and we record how many there are. For finite sets, these two measurements — ordinal position and cardinal size — are intimately related: a set of five elements, counted in any order, always yields the answer "five."

For infinite sets, ordinal and cardinal quantities become genuinely distinct, and the mathematics of each is a rich subject in its own right.

*Ordinal numbers* measure "how far along" a process has gone. They answer: in what position does this element come? They extend the natural numbers into the transfinite: ω (the first infinite ordinal), ω + 1, ω + 2, ..., ω + ω = ω·2, ..., ω², ..., ωω, ..., ε₀, ....

*Cardinal numbers* measure size. They answer: how many elements does this set have? Two sets have the same cardinality if there is a bijection between them. The first infinite cardinal is ℵ₀ = |ℕ|. The next is ℵ₁. Whether |ℝ| = ℵ₁ is the Continuum Hypothesis — independent of ZFC.

## Von Neumann Ordinals

**Definition.** An *ordinal* is a *transitive set* that is *well-ordered by ∈*.

A set A is *transitive* if x ∈ y ∈ A implies x ∈ A. Equivalently, every element of A is a subset of A.

A set is *well-ordered by ∈* if ∈ is a strict total order on A and every non-empty subset of A has a ∈-minimal element.

**The von Neumann ordinals:**
- 0 = ∅
- 1 = {0} = {∅}
- 2 = {0, 1} = {∅, {∅}}
- 3 = {0, 1, 2}
- n+1 = n ∪ {n}
- ω = {0, 1, 2, 3, ...} = ℕ (the natural numbers as sets)
- ω + 1 = ω ∪ {ω} = {0, 1, 2, ..., ω}
- ω + 2 = {0, 1, 2, ..., ω, ω+1}
- ω · 2 = ω + ω = {0, 1, ..., ω, ω+1, ω+2, ...}

Each ordinal α is the set of all ordinals strictly less than α. This is an elegant canonical definition: ordinals *are* their predecessors.

**Arithmetic of ordinals:**

*Successor.* α + 1 = α ∪ {α}.

*Limit ordinals.* An ordinal λ is a *limit ordinal* if it has no predecessor — equivalently, λ = ⋃_{α<λ} α. The ordinals ω, ω·2, ω², ωω, ε₀ are all limit ordinals.

*Addition.* α + β is defined by transfinite recursion: α + 0 = α; α + (β+1) = (α+β)+1; α + λ = ⋃_{β<λ} (α+β) for limit λ.

**Warning:** Ordinal addition is *not commutative*. 1 + ω = ω ≠ ω + 1. The difference: 1 + ω means "start with a 1-element set, then append an ω-sequence" — which is just an ω-sequence. While ω + 1 means "start with an ω-sequence, then append one more element" — which has a last element.

*Multiplication.* α · β counts β copies of α arranged in sequence. Again non-commutative: 2 · ω = ω (two copies of ω interleaved is still ω-sequence) but ω · 2 = ω + ω (two sequential copies of ω has two "kinds" of elements).

## Transfinite Induction and Recursion

**Transfinite Induction.** Let P(α) be a property of ordinals. If:
- P(0) holds, and
- For every ordinal α, if P(β) holds for all β < α, then P(α) holds,

then P(α) holds for every ordinal α.

This is exactly the well-founded induction principle from Chapter 0, applied to ordinals ordered by <. The crucial feature: the well-foundedness of ordinals (every non-empty class of ordinals has a least element) means there is no infinitely descending chain, so the induction terminates.

**Transfinite Recursion.** By the same argument, we can define functions on ordinals by transfinite recursion: specify f(0), specify f(α+1) in terms of f(α), and specify f(λ) = lim_{α<λ} f(α) for limit ordinals. This builds up functions on the entire ordinal sequence.

The cumulative hierarchy Vα is defined by transfinite recursion:
- V₀ = ∅
- Vα₊₁ = 𝒫(Vα)
- Vλ = ⋃_{α<λ} Vα

## Cardinals and Cardinality

**Definition.** Two sets A, B have the same *cardinality*, written |A| = |B|, if there exists a bijection f: A → B.

**Theorem (Cantor-Schröder-Bernstein).** If |A| ≤ |B| and |B| ≤ |A|, then |A| = |B|. (Where |A| ≤ |B| means there is an injection A → B.)

This theorem is crucial and non-trivial. Its proof constructs the bijection explicitly from the two injections, using a fixed-point argument.

**Cardinals as initial ordinals.** Under AC, every set can be well-ordered, and we define the *cardinality* |A| as the least ordinal in bijection with A. The infinite cardinals are the *initial ordinals* — ordinals not in bijection with any smaller ordinal:
- ℵ₀ = ω (the first infinite cardinal)
- ℵ₁ = the first uncountable cardinal
- ℵ₂ = the second uncountable cardinal
- ...and so on, transfinitely.

**Cantor's Theorem.** For any set A, |A| < |𝒫(A)|.

*Proof.* There is an injection A → 𝒫(A) sending a ↦ {a}, so |A| ≤ |𝒫(A)|. We show there is no surjection f: A → 𝒫(A), hence no bijection.

Given any f: A → 𝒫(A), consider D = {a ∈ A | a ∉ f(a)}. We claim D ≠ f(a) for any a ∈ A.

If D = f(a₀) for some a₀:
- If a₀ ∈ D, then by definition of D, a₀ ∉ f(a₀) = D. Contradiction.
- If a₀ ∉ D, then a₀ ∉ f(a₀), so a₀ satisfies the condition for D, giving a₀ ∈ D. Contradiction.

In either case we reach a contradiction. So f is not surjective. Since f was arbitrary, no surjection exists. □

This is the diagonal argument: D is the diagonal set of all elements that do not contain themselves. It evades every potential preimage by disagreeing with f at the point itself.

**Cardinal arithmetic:**
- ℵ₀ + ℵ₀ = ℵ₀ (ℕ and ℕ + ℕ have the same cardinality)
- ℵ₀ · ℵ₀ = ℵ₀ (ℕ × ℕ is countable)
- 2^ℵ₀ = |ℝ| = |𝒫(ℕ)| > ℵ₀ (the reals are uncountable)

**The Continuum Hypothesis (CH).** Is 2^ℵ₀ = ℵ₁? That is, is there a cardinal strictly between ℵ₀ and |ℝ|? Cantor conjectured yes (CH says: no such intermediate cardinal exists). Gödel (1938) showed CH is consistent with ZFC. Cohen (1963) showed ¬CH is also consistent. CH is *independent* of ZFC — undecidable by the axioms alone.

## Cofinality and Regular Cardinals

**Definition.** The *cofinality* cf(α) of an ordinal α is the smallest ordinal β such that there is a cofinal sequence of length β in α — a sequence (αᵢ)_{i<β} with sup αᵢ = α.

For successor ordinals, cf(α+1) = 1. For ω, cf(ω) = ω (you need ω-many steps to reach ω). For ω₁ (the first uncountable ordinal), cf(ω₁) = ω₁ (no countable sequence is cofinal in ω₁).

A cardinal κ is *regular* if cf(κ) = κ. The cardinal ℵ₀ is regular. Every successor cardinal is regular. The question of which limit cardinals are regular is deep and set-theoretically rich.

## Ordinals in Type Theory

The type-theoretic analogue of ordinals is the notion of a *well-founded relation* (introduced in Chapter 0 for induction) or, in HoTT, the notion of a *mere ordinal* in the sense of a well-founded extensional relation on a set.

More subtly: the universe levels in MLTT (U₀, U₁, U₂, ...) are indexed by natural numbers — a countable well-ordered sequence. Large cardinal axioms in set theory correspond to assuming the existence of more universes. The Mahlo cardinals correspond to having a universe closed under certain operations.

Ordinal arithmetic also appears in proof theory: the *proof-theoretic ordinal* of a formal system is the supremum of the ordinals whose well-foundedness the system can prove. The proof-theoretic ordinal of Peano Arithmetic is ε₀. Of stronger systems: much larger. This ordinal is a measure of the system's strength.

Cantor opened mathematics to the infinite. The ordinals and cardinals are the machinery by which the infinite is tamed, measured, and reasoned about. They are the conceptual infrastructure of every theory that goes beyond the finite — and HoTT, which reasons about all of mathematics, must have something to say about the infinite. It does: through universe levels, truncations, and the homotopy-theoretic study of types at every level of complexity.
