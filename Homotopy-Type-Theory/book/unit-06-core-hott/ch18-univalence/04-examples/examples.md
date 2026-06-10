# Examples: Paths in the Universe

## What Are the Paths on Bool?

The simplest non-trivial example of Univalence in action: the type `Bool = Bool` of paths from Bool to itself.

By Univalence, `(Bool = Bool) ≃ (Bool ≃ Bool)`. How many equivalences from Bool to itself are there?

An equivalence e : Bool ≃ Bool is a bijection from Bool to Bool. Bool = {true, false}. There are exactly two bijections:
- The identity: id(true) = true, id(false) = false
- The swap: swap(true) = false, swap(false) = true

So `Bool ≃ Bool` has exactly two elements: id and swap.

**Theorem.** `Bool = Bool` has exactly two elements: `refl_{Bool}` (corresponding to id) and `ua(swap)` (corresponding to the swap equivalence).

**The group structure.** The type `Bool = Bool` is a group under concatenation of paths:
- refl_{Bool} is the identity element
- ua(swap) composed with ua(swap) gives refl_{Bool} (since swap ∘ swap = id)

So the group `π₁(Type, Bool)` = the group of paths `Bool = Bool` is isomorphic to Z/2Z.

**The loop space of the universe at Bool.** `Ω(Type, Bool) = (Bool = Bool) ≃ (Bool ≃ Bool) ≃ Z/2Z`.

This is a concrete computation: the loop space of the universe at the boolean type is the two-element group. Voevodsky saw this and said: the universe has a non-trivial loop space. Types are spaces. This is homotopy theory.

## Aut(Fin(n)) = Sₙ

More generally: the automorphisms of the type `Fin(n)` (the type with exactly n elements) form the symmetric group on n elements.

`Aut(Fin(n)) = (Fin(n) ≃ Fin(n))` = the type of bijections from Fin(n) to Fin(n) = Sₙ.

By Univalence, `Fin(n) = Fin(n) ≃ Sₙ`. So the loop space `Ω(Type, Fin(n))` is the symmetric group Sₙ.

This is the type-theoretic version of the classical fact: the automorphism group of a set with n elements is the symmetric group. In HoTT, the "automorphism group" is literally the loop space of the universe at that type.

## Paths in the Universe and Equivalences

The general pattern: paths in the universe are equivalences.

- `A = A` ≃ `Aut(A)` (the automorphisms of A)
- `A = B` ≃ `A ≃ B` (when non-empty, these are the ways A and B are "the same")

For a path p : A = B and x : A, the transport `transport^{id}(p)(x) : B` gives the "image" of x under the equivalence corresponding to p. If p = ua(e), then `transport^{id}(p)(x) = fun(e)(x)` by the computation rule.

**Non-trivial example: Z and 2Z.** The types Z (all integers) and 2Z (even integers) are equivalent as *sets*: the bijection n ↦ 2n from Z to 2Z is an equivalence. By Univalence, there is a path `Z = 2Z` in the universe.

But as *ordered sets*, Z and 2Z are not order-isomorphic (Z has no minimal element; 2Z, as a subset of Z, also has no minimal element, but their order structures are isomorphic as ordered sets). As *groups under addition*, Z and 2Z are isomorphic (the bijection n ↦ 2n preserves addition up to a factor of 2). Depending on which structure we equip them with, they may or may not be "the same."

The point: `Z = 2Z` in the universe of *types* (forgetting structure) — Univalence gives this. But `Z = 2Z` in the universe of *groups* requires a group isomorphism, which does exist (n ↦ 2n, as a group isomorphism between (Z,+) and (2Z,+)). So `Z = 2Z` in the universe of groups as well.

## The Loop Space of the Universe

The loop space of the universe at a type A is:

```
Ω(Type, A) = (A = A) ≃ Aut(A)
```

This is the group of self-equivalences of A. It is the "symmetry group" of A.

For different types:
- A = 1: Aut(1) = 1 (only the identity — the trivial symmetry group)
- A = Bool: Aut(Bool) ≃ Z/2Z
- A = Fin(n): Aut(Fin(n)) ≃ Sₙ
- A = Z: Aut(Z) is the group of bijections Z → Z (countably infinite)
- A = S^1 (the circle): Aut(S^1) is the group of self-homotopy-equivalences of S^1, which is... complex (it includes the identity and all degree-±1 maps)

The loop space of the universe is not simple. The universe has a rich, complex homotopy type. This is not a problem — it is the mathematical content of the fact that there are many ways types can be equivalent to themselves.

## Paths Between Propositions

For propositions P and Q (types at h-level -1):

```
(P = Q)  ≃  (P ≃ Q)  ≃  (P ↔ Q)
```

The last equivalence: since P and Q are propositions, any function P → Q is automatically an equivalence (the fibers are contractible, since Q is a proposition). So `P ≃ Q ≃ (P → Q) × (Q → P) = (P ↔ Q)`.

Therefore: `(P = Q) ≃ (P ↔ Q)` for propositions. This is propositional extensionality, recovered as a special case of Univalence.

## Paths Between Sets

For sets A and B (types at h-level 0):

```
(A = B)  ≃  (A ≃ B)  ≃  (A ≅ B)
```

where `A ≅ B` denotes the *set of bijections* from A to B (since for sets, an equivalence is a bijection — the fibers are automatically propositions, so contractibility of fibers means there is exactly one preimage).

The type `A = B` for sets A and B is itself a set (since Aut(A) is a set for any set A — bijections from a set to itself form a set). This confirms that the universe of sets is a groupoid (h-level 1).

## Univalence Makes the Universe a Genuine Space

The examples in this section show that the universe Type has genuine homotopy-theoretic structure:

- It has non-trivial loops (paths Bool = Bool, i.e., Z/2Z worth of loops at Bool).
- Its loop spaces at different points are different groups (Sₙ at Fin(n), Z/2Z at Bool).
- It is not a groupoid (since its loop spaces can be non-trivial groups, its 2-path space is also non-trivial).

In classical type theory (without Univalence), the universe is "inert" — its path type `A = B` carries no useful information, since there are no non-trivial paths. With Univalence, the universe becomes a living space, with rich homotopy structure reflecting the mathematical structure of types.

This is what Voevodsky meant when he said the univalent foundations are "foundations based on homotopy theory." The universe itself is a homotopy type, not a flat set of types. And this homotopy structure is not added by hand — it falls out of the Univalence Axiom, which says that the universe's path structure is the same as its equivalence structure.

## Summary

| Path type | Description | Group |
|---|---|---|
| Bool = Bool | Two paths: refl and ua(swap) | Z/2Z |
| Fin(n) = Fin(n) | All permutations of n elements | Sₙ |
| P = Q (props) | Logical equivalences | — |
| A = B (sets) | Bijections from A to B | — |
| Ω(Type, A) | Self-equivalences of A | Aut(A) |

The universe Type is a space. Its points are types. Its paths are equivalences. Its loop spaces are automorphism groups. Univalence makes all of this precise and computationally meaningful.
