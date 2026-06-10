# The Univalence Axiom

## Building the Bridge

We have two notions of sameness for types:

1. **Equality:** A path p : A =_{Type} B in the universe.
2. **Equivalence:** An element e : A ≃ B — a function with contractible fibers.

These should be the same thing. Intuitively, if A and B are equivalent, they have the same structure, the same elements (up to the bijection), the same properties. They *are* the same type, in any mathematically meaningful sense. The Univalence Axiom formalizes this intuition.

## The Map idToEquiv

Start with what we can prove without any axiom. A path p : A = B in the universe gives an equivalence A ≃ B.

**Definition.** `idToEquiv : (A = B) → (A ≃ B)` is defined by:

```
idToEquiv(refl_A) = (id_A, isEquiv(id_A))
```

and extended to all paths by J.

More explicitly: given p : A = B, define the underlying function of `idToEquiv(p)` to be `transport^{id}(p) : A → B` — transport along p in the identity type family. Then show this transport is an equivalence (its fibers are contractible, proved by J on p: at p = refl_A, the transport is id_A and the fibers of id_A are contractible).

**Computation:** `idToEquiv(refl_A) = (id_A, isEquiv-id_A)`.

So we always get an equivalence from a path. The question is: does every equivalence come from a path?

## The Univalence Axiom

**Axiom (Univalence).** The map `idToEquiv : (A = B) → (A ≃ B)` is an equivalence.

That is:

```
univalence : (A = B) ≃ (A ≃ B)
```

The inverse of `idToEquiv` is the function `ua : (A ≃ B) → (A = B)`:

```
ua : (A ≃ B) → (A = B)
```

The Univalence Axiom says that ua and idToEquiv are mutual inverses:

```
idToEquiv(ua(e)) = e         for all e : A ≃ B
ua(idToEquiv(p)) = p         for all p : A = B
```

## The Computation Rule for Univalence

The most important consequence of univalence: transport along `ua(e)` is the underlying function of e.

**Theorem.** For e : A ≃ B and x : A:
```
transport^{id}(ua(e), x) = fun(e)(x)
```

where `fun(e) : A → B` is the underlying function of the equivalence e.

*Proof.* By the right-to-left round trip: `transport^{id}(ua(e))` is the underlying function of `idToEquiv(ua(e)) = e`. ∎

This computation rule is the *key* to using univalence in proofs. When we have a type family P : Type → Type and a path p = ua(e) in the universe, transport along p in P is the action of e on P. This is how the circle's loop (defined via ua) computes as the successor function in the fundamental group computation.

## Why Univalence is an Axiom (Not a Theorem)

In standard Martin-Lof type theory, the identity type of the universe is not well-determined. The map `idToEquiv` is definable, but its inverse `ua` cannot be constructed from the elimination rules alone.

This is because J for the universe says: to prove something about all paths p : A = B, prove it for refl_A. But the universe may have paths (equivalences) that do not arise from definitional equality. J tells us nothing about non-reflexive paths.

To add `ua` as an element of `(A≃B) → (A=B)`, we must add it as an axiom. The consistency of this axiom is established by the simplicial set model.

**The simplicial set model.** Voevodsky proved that there is a simplicial set model of type theory in which:
- Types are interpreted as Kan complexes
- The universe is the "classifying space of Kan fibrations" — a Kan complex U where elements correspond to Kan complexes
- Paths in U correspond to homotopy equivalences of Kan complexes
- The Univalence Axiom holds: `idToEquiv` is a homotopy equivalence

Since the simplicial set model is a model of ZFC set theory (plus some inaccessible cardinals), the consistency of HoTT with Univalence follows from the consistency of ZFC.

**Cubical type theory.** In cubical type theory (Chapter 23), the Univalence Axiom is not an axiom but a *theorem*. The type theory has an explicit interval type I with endpoints 0 and 1, and the Kan operations (composition and filling) of cubical complexes. From these computational rules, univalence can be proved — the function `ua` is explicitly constructed and its computation rules hold definitionally.

For now, in Book HoTT (the standard formulation), we treat Univalence as an axiom, knowing it is consistent and knowing that cubical type theory provides a computational realization.

## What Univalence Does Not Say

Univalence does not say that every bijection between types is an equality. A bijection is just a function with a set-theoretic inverse. Univalence is about *equivalences* — bijections with contractible fibers, or equivalently with a coherent inverse.

Univalence does not say that the notion of "type" is trivial. It says that the universe *Type* has non-trivial path structure — paths correspond to equivalences, and equivalences can be complex.

Univalence does not collapse all mathematical distinctions. Two groups G and H that are isomorphic are equal *as groups* — but their elements may be different, and the path (the isomorphism) between them carries the specific identification.

## The Philosophical Significance

The Univalence Axiom is the formal expression of what Leibniz called the *Principle of Identity of Indiscernibles*: if two things are indistinguishable by any property, they are identical. In HoTT, two types that are equivalent are indistinguishable by any type-theoretically definable property — and univalence says they are therefore equal.

More precisely: any type-theoretically definable predicate P : Type → Type (a "property of types") is invariant under equivalence. If A ≃ B, then P(A) ≃ P(B). (This follows from transport: transport^P along the path ua(e) gives an equivalence P(A) ≃ P(B).) So no property can distinguish equivalent types.

Conversely, if A and B are not equivalent, they *can* be distinguished: there exists a property P such that P(A) is inhabited but P(B) is empty (or vice versa). (Take P(X) = (X ≃ A).)

So equivalent types are exactly the indistinguishable types. And univalence says indistinguishable types are identical. The formal system is finally as good as the informal principle.

## Summary

| Concept | Type | Key fact |
|---|---|---|
| idToEquiv | (A=B) → (A≃B) | Defined by J; computes as transport |
| ua | (A≃B) → (A=B) | The inverse, given by Univalence |
| Univalence | (A=B) ≃ (A≃B) | The axiom |
| Computation | transport^{id}(ua(e)) = fun(e) | The key computation rule |
| Consistency | Proved via simplicial set model | Follows from ZFC consistency |

The Univalence Axiom is the deepest axiom in HoTT. It connects the logical notion of equality with the mathematical notion of equivalence, and it does so in a way that is computationally meaningful (via the transport computation rule) and philosophically principled (expressing the Leibniz criterion for identity).
