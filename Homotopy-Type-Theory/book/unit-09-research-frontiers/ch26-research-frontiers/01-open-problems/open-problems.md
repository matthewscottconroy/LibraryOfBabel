# 01 — Open Problems in HoTT

## The Shape of the Unknown

Research problems in mathematics are not all the same shape. Some are gaps: a theorem everyone believes is true, where the question is only whether we can write a proof that satisfies the referees. Some are anomalies: a place where the expected result fails, or fails to be provable, in ways that suggest the underlying theory is more subtle than assumed. Some are voids: entire regions of mathematics that have not yet been explored, where the question is not "how do we prove X?" but "what are the right questions to ask?"

The open problems in HoTT span all three categories. The Brunerie problem is partly a gap (we believe we can find a cleaner proof) and partly an anomaly (the existing proof is computationally opaque in a way that should not, in principle, be necessary). Canonicity for Book HoTT is an anomaly: the axiomatic univalence approach is computationally inert in a way that cubical type theory resolves by construction. Directed univalence is a void: we do not even know if it is consistent, let alone how to prove it.

What follows is a map of the major open problems as of 2025–2026, organized by type and difficulty, with precise statements and honest assessments of what is known and what is not.

---

## Problem 1: The Brunerie Number and the Computational Content of π₄(S³)

**Status:** Proved. The computation of π₄(S³) = ℤ/2ℤ has been verified in Cubical Agda (Ljungström-Mörtberg, LICS 2023). The *computational* problem — extracting the Brunerie number by normalization from first principles in Book HoTT — remains open.

**The precise statement.** Brunerie's 2016 PhD thesis defines an integer n by the proof term: n is the degree of the composite

    S³ →^η S² → Ω Σ S²

where η is the Hopf map and the second arrow is the James splitting inclusion. The proof shows that π₄(S³) = ℤ/nℤ, so showing n = ±2 gives π₄(S³) = ℤ/2ℤ. But "showing n = ±2" requires a computation: it requires reducing a specific term of type ℤ to a numeral. In Brunerie's original Agda formalization, this computation ran out of memory before completing.

**What the 2023 simplification achieved.** Ljungström and Mörtberg, building on work by Brunerie and Mörtberg's earlier reformulations, found a version of the proof where the critical computation is small enough for a machine to check. Their key innovation: reformulating the smash product using symmetric monoidal structure, which dramatically reduces the size of the proof terms. This makes the proof *mechanically verifiable*. It does not make it *conceptually transparent*: the computation succeeds because it terminates, not because it reveals why n = 2.

**What remains open.** A "human-scale" proof: a proof where a person can follow each step of the computation of n without losing the thread in terabytes of term structure. The classical homotopy theory argument is a few lines, once you have the Hopf invariant and the EHP sequence. The HoTT proof currently requires machinery whose pieces are clear but whose combination is opaque. What new synthetic technique would make the computation transparent?

**Why it matters.** This is not just about π₄(S³). It is about the gap between what HoTT can prove and what HoTT can *understand*. A field that can verify computations it cannot comprehend has a fundamental problem. The Brunerie number is the sharpest example of that problem, which is why the community returns to it.

**Entry points for research.**
- Read Brunerie's thesis (arXiv:1606.05916), especially the introduction and Section 2.
- Read Ljungström-Mörtberg (LICS 2023) and understand what "symmetric monoidal smash product" buys computationally.
- Identify where the original proof's complexity explodes. Is there a specific term that is large? A specific reduction step that does not simplify?
- Explore whether the James-Hopf invariant can be computed in Cubical Agda using the newer library tools without the size explosion.

---

## Problem 2: Canonicity for Book HoTT

**Status:** Open for Book HoTT (axiomatic univalence). Solved for Cubical Agda (Huber 2018, canonicity via normalization for CCHM cubical type theory).

**The precise statement.** In Book HoTT, univalence is an axiom:

    ua : (A ≃ B) → (A = B)

with no computation rule: `ua e` is a path, but there is no rule saying what `transport (ua e) x` reduces to. The question: does every closed term of type ℕ reduce to a numeral `0`, `1`, `2`, ...?

If the answer is yes, Book HoTT is *canonical*: every natural number term has a definite value, and type theory is fully constructive. If the answer is no, then there are provably-existing natural numbers whose value cannot be computed from the proof — a serious violation of the constructive spirit of type theory.

**The difficulty.** Consider a term like:

    transport (ua succ-equiv) 3 : ℕ

where `succ-equiv : ℕ ≃ ℕ` is the successor equivalence. In Cubical Agda, this reduces to 4 (via the computation rule for transport along Glue types). In Book HoTT, it is stuck: `ua succ-equiv` is an element of `ℕ = ℕ` with no computation rule, so `transport (ua succ-equiv) 3` has no reduction. The term is well-typed and has a value (the univalence axiom guarantees this semantically), but the type theory cannot compute it.

**Shulman's homotopy canonicity.** Shulman's weaker result says that every closed term of type ℕ is *equal* (provably equal in HoTT) to a numeral — just not necessarily *definitionally equal*. This is "homotopy canonicity": the term `transport (ua succ-equiv) 3` is provably equal to 4, even though the computation does not reduce to 4 directly. Homotopy canonicity means Book HoTT is consistent and has no spurious natural numbers, but it does not give the computational content of full canonicity.

**What a proof would require.** Either:
(a) Show that no closed term of type ℕ in Book HoTT can involve `ua` in a way that prevents normalization — that is, find a structural argument that shows such terms never arise at base types. (This seems unlikely; there are indirect encodings.)
(b) Give a computational interpretation of `ua` at every type, so that `transport (ua e) x` has a computation rule for every equivalence e. This is essentially what CCHM cubical type theory does, via the Glue type. But giving such an interpretation *for* Book HoTT — while keeping the axiom as an axiom rather than a constructor — is much harder.
(c) Find a model of Book HoTT in which canonicity provably fails, which would close the question in the negative direction.

**Why it matters for the foundations.** Canonicity is the difference between a type theory being *computational* and merely *consistent*. Book HoTT has extraordinary expressive power and its consistency is not in doubt. But if canonicity fails, then Book HoTT is not fully constructive in the sense that matters to constructive mathematics: you can prove that a natural number exists without being able to compute it. Cubical type theory escaped this problem by construction. Book HoTT's status remains genuinely uncertain.

---

## Problem 3: Coherence for Higher Inductive Types

**Status:** Solved for 1-dimensional and many specific HITs (Lumsdaine-Shulman 2020). Open for the general case, especially for HITs with path-between-path constructors.

**The precise statement.** A higher inductive type (HIT) is specified by a list of point constructors and path constructors. The circle has one point constructor (`base : S¹`) and one path constructor (`loop : base = base`). The torus has two path constructors and a path-between-path constructor (`surf : loop_a ∙ loop_b = loop_b ∙ loop_a`). Eilenberg-MacLane spaces K(G, n) for n > 1 require arbitrarily high-dimensional path constructors.

The question: is there a *general* syntactic specification for all HITs — a grammar that describes which HIT specifications are valid, and a theorem that any valid specification has a model in any ∞-topos with the right properties?

**What is known.** Lumsdaine and Shulman (2020) proved that a large class of HITs can be given semantics as left adjoints in ∞-toposes. This handles the circle, the torus, suspension, pushouts, truncations, and many other specific HITs. The paper defines the notion of a "cell monad" and shows that any HIT specifiable as a cell monad has a semantics. This is a major result that resolves the consistency and semantics questions for most HITs that appear in practice.

**What remains open.** The fully general case. The van den Berg-Garner approach to HITs (using 2-cells) handles some HITs not covered by Lumsdaine-Shulman. But neither approach covers all possible HITs, especially those with exotic higher-path constructors. The question of whether there is a "master grammar" for HITs — a single syntactic criterion that covers all cases — is still open.

**The coherence problem specifically.** For HITs with higher path constructors (like K(ℤ, 2), the Eilenberg-MacLane space), the path-algebra becomes extremely involved. The HIT needs not just the path constructors but all the coherence data that makes the paths compose correctly. Writing down this coherence data for K(G, 2) requires explicit work; for K(G, n) with n > 2 it becomes very hard to specify by hand. A general framework for HITs should handle the coherence automatically — but specifying what "automatically" means in this context is itself a hard problem.

**Connection to ∞-operads.** The coherence problem for HITs is related to the coherence problem for ∞-operads: both require specifying infinite towers of compatible higher-dimensional data. Progress in one area tends to produce progress in the other.

---

## Problem 4: Directed Univalence in Simplicial Type Theory

**Status:** Conjectured. No proof or disproof in any setting.

**The precise statement.** In simplicial type theory (STT, Riehl-Shulman 2017), types can be Segal types (∞-categories) or Rezk types (∞-categories where isomorphisms and paths agree). The Rezk condition for a specific Segal type 𝒞 says: the natural map

    (a =_𝒞 b) → (a ≅_𝒞 b)

(from paths to isomorphisms in 𝒞) is an equivalence. This is proved for specific Segal types.

Directed univalence asks: is there a Segal type `Cat` — a "universe of ∞-categories" — such that for any two ∞-categories A, B,

    (A =_Cat B) ≃ (A ≃_∞ B)

where `A ≃_∞ B` means A and B are equivalent as ∞-categories (fully faithful and essentially surjective)? This would be the directed analogue of the univalence axiom: it says that in the universe of ∞-categories, equality is equivalence.

**The difficulty.** In ordinary HoTT, univalence is an axiom (Book HoTT) or a theorem (cubical HoTT). In STT, the directed univalence statement would be a global statement about the entire type theory — not about any specific Segal type, but about the universe of all Segal types. This requires:
(a) A definition of `Cat` as a Segal type (not just as a collection)
(b) Proof that `Cat` itself is Rezk — that is, that the Rezk condition for `Cat` itself holds
(c) This means the Rezk condition for the universe, which is a fixpoint statement and notoriously hard to prove

**Why it is central.** Directed univalence would be to ∞-category theory what univalence is to ∞-groupoid theory: it would make identity of ∞-categories definitionally equal to equivalence, opening the entire machinery of HoTT (transport, path induction, the fundamental theorem of identity types) to ∞-category theory. Without it, STT can reason about specific ∞-categories but not about the space of all ∞-categories uniformly.

Riehl and Shulman have identified this as the central open problem in STT. It appears in their 2017 founding paper as the obvious next conjecture and has remained open since.

---

## Problem 5: π₅(S⁴) — The Next Unknown Sphere Homotopy Group

**Status:** Known classically (π₅(S⁴) = ℤ/2ℤ, with an additional generator from the suspension of the Hopf map). Not computed synthetically in HoTT.

**Why this is the next target.** Brunerie's proof computed π₄(S³) — the first non-stable homotopy group of spheres that requires genuinely new input (the Hopf invariant, the EHP sequence, a non-trivial computation). The subsequent homotopy groups become progressively harder, as the number of generators and relations grows. π₅(S⁴) is the natural next target: it sits in the metastable range (where the EHP sequence is still useful) and its value is known from classical theory, giving a check on any synthetic computation.

**The classical computation.** Using the EHP sequence:

    ... → π₅(S³) → π₅(S⁴) → π₄(S³) → π₄(S³) → π₃(S²) → ...

Combined with:
- π₅(S³) = ℤ/2ℤ (from the suspension of the Hopf map)
- π₄(S³) = ℤ/2ℤ (Brunerie)
- π₃(S²) = ℤ (Hopf fibration)

The computation gives π₅(S⁴) = ℤ/2ℤ.

**The synthetic challenge.** To prove this in HoTT, you need all the ingredients that Brunerie assembled for π₄(S³), plus:
- The next level of the James splitting
- The Toda bracket (a higher-order cohomology operation)
- A computation of the next Brunerie-type number

The Toda bracket is particularly challenging: it is a higher-order operation that is defined only when certain lower maps compose to zero, and its value depends on choices of null-homotopies. Making this precise in type theory requires working with spaces of homotopies, not just individual homotopies.

**What progress looks like.** A formalization of the EHP sequence in Cubical Agda is a necessary prerequisite. The exact sequence alone gives the structure of the computation; the Toda bracket provides the key non-trivial step. A researcher who has understood Brunerie's proof and the structure of the EHP sequence is positioned to begin this project.

---

## Problem 6: The Blakers-Massey Bound is Sharp

**Status:** The bound is proved in HoTT (Anel-Biedermann-Finster-Joyal, 2020). Sharpness — the existence of examples achieving the exact bound — has not been formalized.

**The theorem and its sharpness.** The Blakers-Massey theorem states: if f : A → B is m-connected and g : A → C is n-connected, then the comparison map A → B ×_{B ∪_A C} C is (m + n - 1)-connected.

Sharpness says: for each m, n ≥ 1, there exist maps f, g (both cofiber sequences of spheres) such that the comparison map is exactly (m + n - 1)-connected, not more.

**The classical examples.** The James splitting provides the answer classically: the attaching maps Sᵐ⁺ⁿ⁻¹ → Sᵐ ∨ Sⁿ have the right connectivity. These are the maps whose homotopy classes generate the stable homotopy of the smash product.

**The formalization challenge.** To prove sharpness in Cubical Agda, you need to:
(a) Define the maps Sᵐ⁺ⁿ⁻¹ → Sᵐ ∨ Sⁿ explicitly (using the James-Hopf invariant)
(b) Show they are exactly (m + n - 1)-connected (using the EHP sequence)
(c) Show the comparison map is not (m + n)-connected (using the non-trivial element in π_{m+n-1}(Sᵐ ∨ Sⁿ) that the attaching map represents)

This is a tractable formalization project for someone who has understood the Blakers-Massey proof and the sphere homotopy groups up to the relevant range.

---

## Problem 7: Stable Homotopy Theory Synthetically

**Status:** Spectra have been defined in HoTT (via the delooping machinery for abelian groups and the Eilenberg-MacLane construction). A systematic synthetic development of stable homotopy theory is largely absent from any formalization library.

**What "stable homotopy theory" means in this context.** Classical stable homotopy theory studies the stable homotopy groups πₙˢ = colim_k πₙ₊ₖ(Sᵏ) — the homotopy groups that stabilize as you suspend. These groups are the "atoms" of homotopy theory: every homotopy-theoretic phenomenon eventually reduces to stable phenomena.

Synthetic stable homotopy theory would develop this in HoTT, using:
- Spectra as types with a coherent delooping structure
- The sphere spectrum 𝕊 as the spectrification of the spheres
- The chromatic filtration: the p-local sphere spectrum, Morava K-theories, and their relationship to formal group laws
- The Adams spectral sequence: a tool for computing stable homotopy groups from algebraic data

**The p-localization modality.** The p-localization modality (from modal HoTT, Chapter 25) is the right language for p-local phenomena: it inverts all primes except p and gives the p-local stable sphere. The chromatic filtration then stratifies the p-local sphere by chromatic height, corresponding to formal group laws of height n (Morava K-theories K(n)).

**Why this matters beyond HoTT.** The stable homotopy groups of spheres encode deep arithmetic information — connections to Bernoulli numbers, Bousfield-Kan spectral sequences, and the classification of exotic smooth structures on spheres. A synthetic development would both illuminate these connections and make them available for machine verification.

**The current state.** The Eilenberg-MacLane space K(G, n) is definable in Cubical Agda (for abelian groups G) using the n-fold delooping machinery. Cohomology theories are representable by spectra. The technical infrastructure for spectra is present in the library at a basic level. A systematic development — the analogues of stable homotopy theory's foundational theorems — remains to be done.

---

## A Map of the Terrain

These problems are not independent. Here is a rough dependency structure:

**Problems that unlock others:**
- Directed univalence (Problem 4) would likely clarify canonicity for STT
- General HIT syntax (Problem 3) would make Eilenberg-MacLane spaces cleaner and unlock stable homotopy development (Problem 7)
- Understanding Brunerie's computation (Problem 1) generates techniques applicable to π₅(S⁴) (Problem 5)

**Problems that are most approachable now:**
- Blakers-Massey sharpness (Problem 6): all the pieces exist in the library; this is a matter of assembling them
- π₄(S³) conceptual proof (Problem 1, partial): the Ljungström-Mörtberg machinery gives a starting point; alternative approaches are being explored

**Problems that require genuinely new mathematics:**
- Canonicity for Book HoTT (Problem 2): the obstruction is fundamental; any proof would require a new computational interpretation
- Directed univalence (Problem 4): this may require extending STT with a new axiom or new type constructor

The hierarchy matters for research planning. A student who has completed this curriculum can realistically begin working on Problems 1 (partial), 5, and 6. Problems 2, 3, and 4 require additional background and are likely PhD-level projects requiring advisor guidance. Problem 7 is a long-term program that could occupy a research group for years.

None of this is fixed. Open problems have a way of falling suddenly when the right idea arrives. The right idea could come from anyone — including someone who learned HoTT from this book.
