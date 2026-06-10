# Truncations

## Forgetting Higher Structure

Every type lives somewhere in the h-level hierarchy. But sometimes a type lives *too high* — it has more homotopy structure than we need, and we want to collapse that structure away, keeping only the piece we care about.

Truncations do this. The n-truncation of A, written ‖A‖_n, is the "best n-type approximation to A" — it keeps all the information up to h-level n and collapses everything above. When we truncate to h-level -1, we get a proposition (the propositional truncation, just telling us whether A is inhabited). When we truncate to h-level 0, we get a set (the set of connected components of A).

These are the most important examples, and they are the ones we develop in detail.

## Propositional Truncation

**Definition.** The propositional truncation of A, written ‖A‖ (or ‖A‖_{-1} or ∥A∥), is the higher inductive type with:

- Constructor: |−| : A → ‖A‖
- Path constructor: squash : Π(x y : ‖A‖). x = y

The path constructor forces ‖A‖ to be a proposition: any two elements are equal.

**Universal property.** For any proposition P, the map:

```
(‖A‖ → P)  ≃  (A → P)
```

Maps from ‖A‖ to any proposition factor uniquely through the constructor |−| : A → ‖A‖. In other words: to define a function from ‖A‖ to a proposition, it suffices to define it on A and the propositional truncation will handle the rest.

This universal property is the *defining property* of propositional truncation — it is what it means to take the "propositional reflection" of A.

**Intuition.** ‖A‖ is the type that says "A is merely inhabited" — there exists an element of A, but we're not telling you which one. The map |−| : A → ‖A‖ sends every element to its equivalence class under the relation "all elements are equal," and the squash path constructor enforces this.

**Key example.** The law of excluded middle in HoTT can be stated as: `Π(P : Prop). P + ¬P`. This is the *propositional* excluded middle — it gives you a specific element of P + ¬P. The *weak* excluded middle is: `Π(P : Prop). ‖P + ¬P‖` — it says P + ¬P is merely inhabited, but we can't necessarily extract which side is inhabited. The two formulations are generally inequivalent in constructive mathematics.

## Proof-Relevant vs. Proof-Irrelevant Existence

The central application of propositional truncation is distinguishing two kinds of existential statements.

**Proof-relevant existence:** `Σ(x : A). P(x)` — there exists an x in A satisfying P, and we have a specific witness.

**Proof-irrelevant existence:** `‖Σ(x : A). P(x)‖` — there merely exists an x in A satisfying P, but we may not be able to extract which one.

In ordinary mathematics, these are not distinguished. "There exists a prime greater than 100" is just true, and we don't ask whether we need a specific prime or just the existence of one. In HoTT, the distinction matters:

- From `Σ(x:A).P(x)`, we can extract both the witness x and the proof P(x). We have full computational access to both.
- From `‖Σ(x:A).P(x)‖`, we know the type is inhabited, but to use this in a proof we can only "map out" to propositions — we can conclude propositional statements but cannot extract the witness.

This is the type-theoretic version of the choice principle: if we want to "use" the witness in a proof, we need the proof-relevant version. If we only want to know existence, the truncated version suffices.

## The Axiom of Choice in HoTT

The axiom of choice interacts interestingly with truncations.

**Type-theoretic choice (tautology):** `Π(x:A). Σ(y:B). P(x,y)  →  Σ(f:A→B). Π(x:A). P(x,f(x))`

This is provable in type theory: given a function that produces a witness for each x, simply define f by applying that function.

**Propositionally truncated choice:** `Π(x:A). ‖Σ(y:B). P(x,y)‖  →?  ‖Σ(f:A→B). Π(x:A). P(x,f(x))‖`

This says: if for each x there merely exists a y satisfying P, can we find a function f that witnesses this? In general, this requires the axiom of choice. Without AC, we cannot "choose" the y uniformly without being given the specific witnesses.

The axiom of choice in HoTT is:

```
AC := Π(A : Set). Π(B : A → Set). Π(P : Π(x:A). B(x) → Prop).
      (Π(x:A). ‖Σ(y:B(x)). P(x,y)‖) → ‖Σ(f:Π(x:A).B(x)). Π(x:A). P(x,f(x))‖
```

This is independent of HoTT and must be added as an axiom if desired.

## Set Truncation

The set truncation ‖A‖_0 is the higher inductive type with:

- Constructor: |−|₀ : A → ‖A‖_0
- For any x, y : ‖A‖_0 and p, q : x = y: p = q

The second condition forces all path types to be propositions — making ‖A‖_0 a set.

**Universal property.** For any set S, the map:

```
(‖A‖_0 → S)  ≃  (A → S)
```

Maps from ‖A‖_0 to any set factor uniquely through |−|₀ : A → ‖A‖_0.

**Geometrically.** ‖A‖_0 is the set of connected components of A. If two points of A are connected by a path, they land in the same element of ‖A‖_0. Elements of ‖A‖_0 are equivalence classes under the "path-connected" relation.

**Example.** ‖S^1‖_0 = 1 — the circle is connected, so its set-truncation is a single point. ‖Bool‖_0 = Bool — Bool has two components, each a single point. ‖S^0‖_0 = Bool = S^0 (since S^0 = Bool is already a set).

## General n-Truncation

For any n ≥ -2, the n-truncation ‖A‖_n is the higher inductive type that:
- Has the same elements as A (via the constructor |−|_n : A → ‖A‖_n)
- Forces all path types to be at h-level n

It satisfies: ‖A‖_n is an n-type, and for any n-type B, `(‖A‖_n → B) ≃ (A → B)`.

The special cases:
- n = -2: ‖A‖_{-2} is the propositional truncation... wait, no: ‖A‖_{-2} contracts everything to a single point — it's the "contractibilization" of A. Actually ‖A‖_{-2} = 1 always (sending everything to the unique element of 1). This is not very useful.
- n = -1: ‖A‖ = ‖A‖_{-1} is the propositional truncation.
- n = 0: ‖A‖_0 is the set truncation (π_0 of A).
- n = 1: ‖A‖_1 is the "groupoidification" — the fundamental groupoid of A.

The n-truncation ‖A‖_n gives the n-th level of the Postnikov tower of A.

## Relationship Between Truncations

**Commutativity.** If m ≤ n, then `‖‖A‖_n‖_m ≃ ‖A‖_m`. Truncating twice at the lower level gives the lower truncation. (You can't "gain back" information you've already discarded.)

**The tower.** There is a natural tower of maps:

```
A → ‖A‖_n → ‖A‖_{n-1} → ... → ‖A‖_0 → ‖A‖_{-1} → ‖A‖_{-2} = 1
```

Each map discards the next level of homotopy information.

## Summary

| Truncation | Type | Universal property | Geometrically |
|---|---|---|---|
| ‖A‖ = ‖A‖_{-1} | Proposition | (‖A‖ → P) ≃ (A → P) for P : Prop | Inhabited or not |
| ‖A‖_0 | Set | (‖A‖_0 → S) ≃ (A → S) for S : Set | Connected components |
| ‖A‖_n | n-type | (‖A‖_n → B) ≃ (A → B) for B : n-type | Postnikov section |

Truncations are the operations that let us work at the right level of the h-level hierarchy. When we need only propositional existence (mere inhabitation), we use ‖−‖. When we need the set of connected components, we use ‖−‖_0. The truncations bridge between the full homotopy-theoretic world and the classical mathematical world where sets are the objects of study.
