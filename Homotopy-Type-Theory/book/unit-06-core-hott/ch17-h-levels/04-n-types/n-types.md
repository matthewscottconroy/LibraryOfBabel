# N-Types and the Hierarchy

## Extending the Pattern

We have three levels:
- h-level -2: contractible types (one element up to paths)
- h-level -1: propositions (at most one element up to paths; paths are contractible or empty)
- h-level 0: sets (elements may vary; paths are propositions; 2-paths are trivial)

The pattern is clear: each level is defined by saying that all path types are at the previous level. We continue this recursion to all non-negative integers and obtain the full h-level hierarchy.

## The Recursive Definition

**Definition.** The predicate `is-n-type` (or `is-h-level-n`) is defined by:

```
is-(-2)-type(A) := isContr(A)
is-(n+1)-type(A) := Π(a b : A). is-n-type(a = b)
```

Explicitly for the first few levels:

- **h-level -2 (contractible):** A ≃ 1
- **h-level -1 (proposition):** All path types a=b are contractible or empty
- **h-level 0 (set):** All path types a=b are propositions
- **h-level 1 (groupoid):** All path types a=b are sets
- **h-level 2 (2-groupoid):** All path types a=b are groupoids
- **h-level n:** All path types are at h-level n-1

This is a hierarchy of increasing complexity, measured by how far down the tower of identity types we must go before everything becomes trivial.

## The Cumulative Property

**Theorem.** If A is an n-type, then A is an (n+1)-type.

*Proof.* By induction on n. For n = -2: contractible types have all path types contractible (by Lemma 1.10 of Section 1), hence propositions. For the inductive step: if all path types are at h-level n, then all path types of path types are at h-level n-1 ≤ h-level n, so path types of path types are at h-level n. ∎

**Corollary.** The hierarchy is cumulative:

```
h-level -2 ⊂ h-level -1 ⊂ h-level 0 ⊂ h-level 1 ⊂ ...
```

Every contractible type is a proposition. Every proposition is a set. Every set is a groupoid. And so on.

## Examples at Each Level

**h-level -2 (contractible):** The unit type 1. The based path space Σ(b:A).(a=b) for any a:A. The type of contractibility proofs isContr(A) when A is contractible.

**h-level -1 (propositions):** The empty type ∅. The unit type 1. The type `n is even` (for a specific n). Any type A where all elements are equal.

**h-level 0 (sets):** N, Z, Q, Bool, Fin(n). Any type with decidable equality. Any proposition (by cumulativity). Any quotient of a set by an equivalence relation (when the quotient is taken correctly, as a set truncation).

**h-level 1 (groupoids):** The circle S^1. More precisely, S^1 is a groupoid in the sense that path types base = base ≃ Z are sets. Any ordinary groupoid (category where all morphisms are invertible) can be realized as a 1-type.

**h-level 2 (2-groupoids):** The 2-sphere S^2. The type S^2 has base = base contractible (only the trivial path, since S^2 is simply connected), but the 2-path type has non-trivial elements.

**No finite h-level:** The universe Type. The type base = base in S^1 is Z, which is a set (h-level 0). But as we go to higher spheres S^n, the path structure accumulates. Ultimately, the universe Type is not an n-type for any n.

## Spheres as the Canonical Examples

The spheres S^n provide the canonical examples of types at each h-level:

- S^0 = Bool is a set (h-level 0) — actually it's the set with two elements.
- S^1 is a 1-type (groupoid): `base = base ≃ Z` is a set.
- S^2 is a 2-type: `base = base` is contractible (S^2 is simply connected), but the 2-path type refl = refl has Z as its fundamental group (π₂(S^2) = Z).
- S^n is an n-type: the path types stabilize at the n-th level, below which homotopy groups are zero.

This is not the same as saying S^n has h-level n — that would mean S^n is an n-type but not an (n-1)-type. Let us be more careful:

- S^0 is a set but not contractible.
- S^1 is a groupoid but not a set (loop space is Z, not a proposition).
- S^2 is a 2-type but not a groupoid (fundamental group is trivial, but π₂ = Z).
- S^n has h-level... actually, all spheres have infinitely complex homotopy groups (by Serre's theorem), so no S^n for n ≥ 1 has finite h-level.

What is true: S^n is an (n+1)-connected type for n ≥ 0, meaning the first n homotopy groups vanish. But the homotopy groups above dimension n are generally nontrivial (and computing them is one of the hardest problems in algebraic topology).

## The Universe of n-Types

**Theorem.** The universe of n-types, `Type_n := Σ(A : Type). is-n-type(A)`, is an (n+1)-type.

This is a key structural fact: the universe of n-types is itself an (n+1)-type, not just an n-type. The reason: equivalences between n-types can be complex (for n ≥ 0), so the path type between two n-types (which is the type of equivalences, by Univalence) can be an n-type but not an (n-1)-type.

For n = -1 (propositions): the universe Prop = Type_{-1} is a set (0-type). Paths between propositions are equivalences of propositions, which are propositions (since propositions have at most one element, an equivalence between them is determined by its existence). So Prop is a set.

For n = 0 (sets): the universe Set = Type_0 is a groupoid (1-type). Paths between sets are equivalences = bijections, which form sets themselves. So Type_0 is a groupoid.

## Closure Properties of n-Types

n-types are closed under the main type formers:

**Products and Sigma-types.** If A is an n-type and B : A → Type has each B(a) an n-type, then Σ(a:A).B(a) is an n-type. (Proved by showing path types in the sigma are at h-level n-1.)

**Function types.** If B is an n-type, then A → B is an n-type for any A. (Paths in function types are pointwise paths by funext; pointwise paths in an n-type are (n-1)-types.)

**Propositions are closed.** If A is a proposition and B : A → Type has each B(a) a proposition, then Π(a:A).B(a) is a proposition.

## The Postnikov Tower

Every type A has a *Postnikov tower*: a sequence of types and maps

```
A → ... → τ₂(A) → τ₁(A) → τ₀(A) → τ₋₁(A) → τ₋₂(A)
```

where τ_n(A) is the n-truncation of A (the "best n-type approximation to A") and each map is an n-connected map (inducing isomorphisms on all homotopy groups in dimension ≤ n).

The Postnikov tower decomposes A into its homotopy-theoretic layers. Recovering A from its Postnikov tower requires specifying "k-invariants" — additional data at each level. This is the HoTT analog of the Postnikov system in algebraic topology.

## n-Types and n-Groupoids

The correspondence between n-types in HoTT and n-groupoids in higher category theory is the content of the *Homotopy Hypothesis* (Grothendieck's conjecture, proved for various definitions of n-groupoid):

**Types of h-level n correspond to weak n-groupoids.**

- h-level -2 (contractible) ↔ trivial groupoid (one object, one morphism)
- h-level -1 (proposition) ↔ {∅} or {*} (two possible trivial groupoids)
- h-level 0 (set) ↔ discrete groupoid (0-groupoid: objects, no non-identity morphisms)
- h-level 1 (groupoid) ↔ ordinary groupoid (1-groupoid: objects and invertible morphisms)
- h-level 2 (2-type) ↔ 2-groupoid (objects, morphisms, invertible 2-cells)

This correspondence is exact in the simplicial set model: n-types correspond to Kan complexes whose simplicial homotopy groups vanish above dimension n.

## Summary

| h-level | Name | Defining property | Examples |
|---|---|---|---|
| -2 | Contractible | A ≃ 1 | 1, based path spaces |
| -1 | Proposition | Π(a b:A). isContr(a=b) | ∅, 1, decidable props |
| 0 | Set | Π(a b:A). isProp(a=b) | N, Z, Bool |
| 1 | Groupoid | Π(a b:A). isSet(a=b) | S^1, Aut(G) |
| n | n-groupoid | Π(a b:A). is-(n-1)-type(a=b) | S^n (roughly) |

The h-level hierarchy is the organizing framework for all of HoTT. Every type lives at some h-level (possibly no finite h-level). Every operation respects h-levels in a predictable way. And the classification is sharp: knowing the h-level of a type tells you exactly how much higher path structure it can support.
