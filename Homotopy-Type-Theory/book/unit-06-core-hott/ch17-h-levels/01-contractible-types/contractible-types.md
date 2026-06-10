# Contractible Types

## The Simplest Possible Type

What is the simplest type you can imagine? Not the empty type — that has no elements, which is its own kind of complexity. Not the boolean type — that has two elements with a non-trivial equality structure distinguishing them. The simplest type is one with exactly one element: the unit type 1. Everything is there, nothing is hidden, there is nothing to choose.

But "exactly one element" requires precision. A type A has "exactly one element" in the homotopy-theoretic sense if there is an element c : A such that every other element a : A is *equal* to c. Not merely "there is only one element" in the set-theoretic sense — that would mean the type has no other elements at all. In homotopy type theory, we allow many elements as long as they are all connected to a center by paths.

This is the definition of contractibility.

## The Definition

**Definition.** A type A is *contractible* if there exists:
- A center of contraction c : A
- A contracting homotopy: ∀(a : A), c = a

Formally: `isContr(A) := Σ(c : A). Π(a : A). (c = a)`

An element of `isContr(A)` is a pair (c, h) where c : A is the center and h : Π(a:A). c=a assigns to each element a a specific path from c to a.

Note the data carefully: we need not just the *existence* of paths from c to every element, but the *specific* paths. Contractibility is a structure, not just a property. This matters — different choices of contracting homotopy can give different, non-equal elements of `isContr(A)`.

(Though we will shortly prove that `isContr(A)` is itself a proposition — any two elements of it are equal. So contractibility as a property of A is still propositional, even though the data includes specific paths.)

## The Unit Type is Contractible

The unit type 1 has a single constructor `* : 1`. The contracting homotopy is: `h(*) = refl_*`. Since 1 has only one element, there is nothing else to verify.

`isContr(1)` is witnessed by `(*, λ*. refl_*)`.

## Contractible Types are Equivalent to the Unit Type

The characterizing theorem:

**Theorem.** A type A is contractible if and only if A ≃ 1.

*Proof.* (⇒) Suppose (c, h) : isContr(A). Define f : A → 1 by f(a) = * and g : 1 → A by g(*) = c. The homotopies: g∘f ~ id_A is h (since (g∘f)(a) = g(*) = c and id(a) = a, and h(a) : c = a). f∘g ~ id_1 is trivial since 1 has one element.

(⇐) Suppose e : A ≃ 1 with underlying functions f : A → 1 and g : 1 → A. Take c = g(*). For any a : A, the left homotopy η : g∘f ~ id_A gives η(a) : g(f(a)) = a, i.e., g(*) = a, i.e., c = a. ∎

**Corollary.** Any two contractible types are equivalent to each other.

In particular, contractibility characterizes a type up to equivalence. There is "essentially one" contractible type, and it is 1.

## The Based Path Space is Contractible

This is the theorem that underlies the J rule.

**Theorem.** For any type A and any a : A, the based path space `Σ(b:A).(a = b)` is contractible.

*Proof.* The center is `(a, refl_a)`. For any `(b, p) : Σ(b:A).(a=b)`, we need a path `(a, refl_a) = (b, p)`.

By the Sigma-type path characterization (Chapter 16), such a path consists of:
1. A path q : a = b in A. Take q = p.
2. A path transport^{a=(-)}(q, refl_a) = p. Since transport in the path family concatenates: transport^{a=(-)}(p, refl_a) = refl_a · p = p (by left unit).

So the contracting path is (p, left-unit(p)). ∎

**Why this matters.** The J rule says: to prove something for all (b, p) : Σ(b:A).(a=b), prove it for the center (a, refl_a). This is valid *because* the total path space is contractible: there is only one thing here (up to paths), so proving something for that one thing is enough.

J and the contractibility of the based path space are two faces of the same mathematical fact.

## Closure Properties

Contractible types are closed under several operations.

**Π over contractible domain.** If A is contractible with center c, then for any family P : A → Type, `Π(x:A).P(x) ≃ P(c)`. A function on A is determined by its value at c, since everything else is equal to c.

**Σ with contractible total space.** If A is contractible with center c and P : A → Type, then `Σ(x:A).P(x) ≃ P(c)`. Elements of the Sigma-type are pairs (a, y) with y : P(a), and since a = c (via the contracting homotopy), transport gives y ≈ transport^P(h(a)⁻¹, y) : P(c).

**Π with contractible fibers.** If P : A → Type and each P(a) is contractible, then Π(a:A).P(a) is contractible. The center is the function a ↦ (center of P(a)), and the contracting homotopy uses funext and the contracting homotopies of each P(a).

## Contractibility is a Proposition

A crucial meta-theorem: the type `isContr(A)` is itself a proposition — any two proofs of contractibility are equal.

**Theorem.** isProp(isContr(A)).

*Proof.* Let (c₁, h₁) and (c₂, h₂) be two witnesses of contractibility. We need (c₁, h₁) = (c₂, h₂) in isContr(A) = Σ(c:A).Π(a:A).(c=a).

By the Sigma path characterization:
1. Need p : c₁ = c₂. Take p = h₂(c₁)⁻¹ · h₁(c₂)... actually, take p = h₁(c₂) (a path from c₁ to c₂ using the first contracting homotopy).
2. Need transport^{Π(a:A).((-) = a)}(p, h₁) = h₂. By funext, need for all a: transport^{((-) = a)}(p, h₁(a)) = h₂(a).

Since A is contractible, every path type x = y in A is contractible (in a contractible type, path spaces are also contractible). Therefore transport^{((-) = a)}(p, h₁(a)) and h₂(a) are both elements of the contractible type c₂ = a, so they are equal. ∎

## Equivalences via Contractible Fibers

There is an important characterization of equivalences in terms of contractibility:

**Theorem.** A function f : A → B is an equivalence iff all its fibers are contractible:

```
isEquiv(f)  ≃  Π(b : B). isContr(fib_f(b))
```

where `fib_f(b) = Σ(a:A).(f(a) = b)`.

This is the definition of equivalence via contractible fibers (Chapter 18). It says: f is an equivalence iff every b : B has exactly one preimage — the fiber is a contractible type (one element up to paths).

Contractibility is the type-theoretic way to say "exactly one." Not "at most one" (that is `isProp(fib_f(b))`) and not "at least one" (that is `‖fib_f(b)‖`), but "exactly one."

## Contractibility at the Bottom of the Hierarchy

Contractible types are h-level -2 (the lowest h-level). The definition of h-level is:
- h-level -2 = contractible
- h-level -(n+1) = h-level n for all path types

Contractible types satisfy this vacuously: a type at h-level -2 is one where all elements are equal (via paths), and all paths are equal (via 2-paths), and so on. Everything is trivial at every level. This is the correct notion of "maximally simple" in the homotopy-theoretic sense.

Moving up from h-level -2:
- h-level -1 (propositions): path types a = b are contractible (or empty)
- h-level 0 (sets): path types a = b are propositions
- h-level n: path types are at h-level n-1

Contractibility is the base case of the recursive definition of h-level. It is the point at which the tower of identity types collapses to triviality.

## Summary

| Property | Definition |
|---|---|
| isContr(A) | Σ(c:A). Π(a:A). c = a |
| Equivalent to | A ≃ 1 |
| Example | 1, based path spaces |
| isProp(isContr(A)) | Yes — contractibility is a proposition |
| Key use | Definition of equivalences (contractible fibers) |
| In hierarchy | h-level -2 |

Contractible types are the simplest types. They serve as the base case of the h-level hierarchy and as the definition of equivalences. Every subsequent h-level is defined by saying "paths are contractible" (or, recursively, "paths are at the previous level"), so contractibility is the foundation on which the entire classification rests.
