# Overlap and Fusion

With the primitive P(x, y) in hand, the two concepts that do most of the structural work in mereology are overlap and fusion. Overlap tells us when objects share material; fusion tells us how many things can combine into one. Together they define the characteristic shape of the mereological universe.

## Overlap

**Definition:** O(x, y) =df ∃z [P(z, x) ∧ P(z, y)]

Objects x and y overlap just in case something is a part of both. Your hand and your arm overlap because your hand is a part of each. Two countries overlap if they share a border region. Two events overlap if they share a temporal phase. Disjointness is the complement: D(x, y) =df ¬O(x, y).

Overlap is reflexive (O(x, x) holds since P(x, x) by M1) and symmetric (if z witnesses the overlap of x with y, it witnesses the overlap of y with x). It is *not* transitive: O(x, y) and O(y, z) does not imply O(x, z), because the witnessing parts may be entirely distinct.

Here is a useful reformulation that makes the connection between overlap and parthood explicit. Given the right axioms, parthood can be *defined* from overlap:

P(x, y) ↔ ∀z [O(z, x) → O(z, y)]

x is a part of y if and only if everything that overlaps x also overlaps y. This is a theorem of classical mereology and can serve as an alternative route to the parthood relation, taking overlap as primitive instead of P.

*Proof:* (→) Assume P(x, y) and O(z, x). Then ∃w [P(w, z) ∧ P(w, x)]. By M3, P(w, y). So O(z, y). (←) Assume ∀z [O(z, x) → O(z, y)]. Since O(x, x), we get O(x, y), so ∃w [P(w, x) ∧ P(w, y)]. Suppose ¬P(x, y). By M5, ∃v [P(v, x) ∧ D(v, y)]. But then O(v, x), so by hypothesis O(v, y) — contradiction. Hence P(x, y). □

## Fusion

**Definition (Fusion):** z = σx φ(x) iff:
1. ∀x [φ(x) → P(x, z)]     (z includes every φ-er as a part)
2. ∀y [P(y, z) → ∃x (φ(x) ∧ O(y, x))]     (no excess: every part of z overlaps some φ-er)

Condition (1) makes z an upper bound on the φ-ers. Condition (2) makes z *minimal* — it contains nothing extraneous. Together they uniquely characterize the fusion (by the uniqueness theorem). An equivalent one-clause definition: z overlaps exactly what overlaps at least one φ-er.

Binary sum: a + b = σx (x = a ∨ x = b). Product: a × b = σx (P(x, a) ∧ P(x, b)), defined when O(a, b) — the fusion of all common parts, the greatest lower bound.

## The Philosophical Significance of Unrestricted Fusion

In classical mereology, the fusion axiom (M6) is unrestricted: for any non-empty condition φ, there exists σx φ(x). Three consequences follow.

First, *ontological generosity*: the object consisting of the Eiffel Tower and the left nostril of Napoleon exists as a mereological sum. We may never name it, refer to it, or care about it, but it is there. Whether to take this seriously as a metaphysical commitment is contested — but the axiom does not allow us to be selective.

Second, and more interesting, David Lewis's claim of *ontological innocence* in *Parts of Classes* (1991): admitting fusions is "nothing over and above" its parts. If you already believe in a and b, acknowledging a + b requires no further existential commitment. The parts are already there; the sum just *is* them, collectively. This innocence claim is one of the most debated theses in contemporary mereology. Critics point out that it seems to assume exactly what is at issue — whether the sum is over and above its parts is precisely the question. But Lewis's point is that sums are not additions to the fundamental inventory; they are reconfigurations of what was already there.

Third, *formal completeness*: unrestricted fusion guarantees that the universe of individuals forms a complete lattice under the parthood ordering. Every non-empty collection has a least upper bound. This algebraic completeness makes the theory tractable and its models well-understood.

## Fusion Versus Set-Theoretic Union

We should be careful not to confuse fusion with set union. Set union is defined on sets and produces sets; mereological fusion is defined on individuals and produces individuals.

| Feature | Set Union A ∪ B | Mereological Sum a + b |
|---|---|---|
| Type of result | A set | An individual |
| Structure | {{a}, {b}} ≠ {a, b} (nesting matters) | (a + b) + c = a + b + c (flat) |
| Extensionality | Sets equal iff same members | Fusions equal iff same parts |

The flatness of mereological structure is crucial: the sum of sums is just the sum. Mereological structure lacks the nested hierarchy of set theory, which is why Lewis needed both mereology *and* the singleton function to reconstruct set theory — mereology alone cannot generate the requisite nesting.

## Objections to the Fusion Concept

The *gerrymandered objects objection* holds that unrestricted fusion commits us to absurd objects — the fusion of the Milky Way and a coffee cup — that lack any causal or qualitative unity, and that it is unclear what it would mean for them to exist. The defender of unrestricted fusion (Lewis, Sider) responds that "exists" is not a predicate tracking salience or relevance; it is unrestricted. Gerrymandered objects are not *more* real than ordinary objects, but they are real in exactly the same sense. Our failure to recognize them in ordinary life reflects our interests, not their absence.

The *causal criterion objection* is more principled: an object should have some causal unity, some capacity to act and be acted upon as a unit. Arbitrary fusions fail this test. The reply is that the causal criterion, while intuitively appealing, is notoriously difficult to make precise without circularity. What counts as acting "as a unit"? If this is cashed out mereologically, the criterion is circular. If it is cashed out causally, we need an independent account of causal structure that does not presuppose mereological unity. Until such an account is available, the causal criterion cannot do the restricting work required of it.
