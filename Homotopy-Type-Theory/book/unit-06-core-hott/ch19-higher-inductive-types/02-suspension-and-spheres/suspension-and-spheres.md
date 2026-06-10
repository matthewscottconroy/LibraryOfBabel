# Suspension and Spheres

## Building Spheres Inductively

A sphere S^n has n-dimensional geometry — a circle is 1-dimensional, a sphere is 2-dimensional, and so on. In algebraic topology, each sphere is fundamental: they are the building blocks of homotopy theory, and computing their homotopy groups is one of the central problems of the field.

In HoTT, we can define all spheres using a single construction: the *suspension*. The suspension ΣA of a type A turns A into a "sphere-like" type by adding two poles (north and south) and a path from north to south for each element of A.

## The Suspension

**Definition.** The suspension ΣA of a type A is the HIT with:
- Point constructor: `N : ΣA` (north pole)
- Point constructor: `S : ΣA` (south pole)
- Path constructor: `merid(a) : N = S` for each a : A (a "meridian" for each element of A)

**The eliminator for ΣA.** To define f : ΣA → B:
- `f(N) = n : B`
- `f(S) = s : B`
- `ap_f(merid(a)) = p(a) : n = s` for each a : A

**The dependent eliminator.** To define s : Π(x:ΣA). P(x):
- `s(N) = n : P(N)`
- `s(S) = s_val : P(S)`
- `apd_s(merid(a)) = t(a) : transport^P(merid(a), n) = s_val`

## Spheres via Suspension

The spheres arise by iterated suspension of the two-element type Bool:

```
S^0 = Bool = {true, false}     (0-sphere: two points)
S^1 = ΣS^0 = ΣBool              (1-sphere: circle)
S^2 = ΣS^1                      (2-sphere)
S^n = ΣS^{n-1}                  (n-sphere)
```

**S^0 = Bool.** The 0-sphere is the type with two isolated points — exactly Bool.

**S^1 = ΣBool.** The suspension of Bool has two poles N and S, plus meridians merid(true) : N = S and merid(false) : N = S. This gives a circle: two paths from N to S, which together form a loop `merid(true) · merid(false)⁻¹ : N = N`.

To see that ΣBool ≃ S^1, define:
- `f : S^1 → ΣBool` by `f(base) = N` and `ap_f(loop) = merid(true) · merid(false)⁻¹`
- `g : ΣBool → S^1` by `g(N) = base`, `g(S) = base`, `ap_g(merid(true)) = loop`, `ap_g(merid(false)) = refl_base`

These form a homotopy equivalence.

**S^2 = ΣS^1.** The suspension of S^1 has poles N, S and meridians merid(x) : N = S for each x : S^1. Since S^1 has a non-trivial loop, the meridians form a 2-cell (a "hemisphere" bounded by the loop). This gives the 2-sphere.

## The Suspension Functor

Suspension is not just a construction on types — it is functorial. Given a map f : A → B, there is an induced map Σf : ΣA → ΣB:
- `(Σf)(N) = N`
- `(Σf)(S) = S`
- `ap_{Σf}(merid(a)) = merid(f(a))`

This makes Σ : Type → Type a functor.

**Equivalences are preserved.** If f : A ≃ B, then Σf : ΣA ≃ ΣB. The suspension of an equivalence is an equivalence.

## The Loop Space-Suspension Adjunction

A fundamental relationship between the suspension and the loop space:

**Theorem (Σ ⊣ Ω adjunction).** For any pointed types A and B:
```
(ΣA →* B)  ≃  (A →* ΩB)
```

Maps from the suspension of A to B correspond (bijectively) to maps from A to the loop space of B.

*Informal proof.* A pointed map f : ΣA →* B sends N to the basepoint b₀ and S to some point, and the meridians to paths from b₀ to that point. But if f is based (N ↦ b₀, S ↦ b₀), the meridians become loops in B based at b₀ — elements of ΩB. And giving a loop in ΩB for each element of A is exactly a map A →* ΩB.

This adjunction is the type-theoretic version of the classical Σ ⊣ Ω adjunction in topology, and it is the foundation for the Freudenthal Suspension Theorem (Chapter 20).

## The Freudenthal Theorem (Preview)

The Freudenthal Suspension Theorem says:

**Theorem.** If A is n-connected (meaning πₖ(A) = 0 for k ≤ n), then the suspension map `A → ΩΣA` (the unit of the Σ ⊣ Ω adjunction) is (2n+1)-connected.

In particular, the map A → ΩΣA induces an isomorphism on πₖ for k ≤ 2n and a surjection on π_{2n+1}.

For A = S^n (the n-sphere, which is (n-1)-connected): the map S^n → ΩS^{n+1} induces isomorphisms on πₖ for k ≤ 2n-1. This means the homotopy groups of spheres *stabilize* in a certain range: πₙ₊ₖ(S^n) is the same for all sufficiently large n. These are the *stable homotopy groups of spheres*.

## The Join Construction

The *join* A * B of two types is a HIT that combines A and B by adding a path between every element of A and every element of B:

**Definition.** A * B is the HIT with:
- `inl(a) : A * B` for a : A
- `inr(b) : A * B` for b : B
- `jn(a, b) : inl(a) = inr(b)` for a : A and b : B

The join A * B is a "cone" over A with "apex" given by B, and symmetrically a cone over B with apex A.

**Key examples:**
- `Bool * A ≃ ΣA` (the suspension of A is the join with Bool)
- `A * B ≃ ΣA ∨ ΣB` (the join factors through the suspensions, in a suitable sense)
- `S^1 * S^1 ≃ S^3` (the join of two circles is a 3-sphere)

The last example is fundamental to the Hopf fibration (Chapter 20): the map S^3 → S^2 arises from the join structure of S^3 = S^1 * S^1 over S^2.

## Why Spheres Matter

The spheres S^n play a central role in homotopy theory:

**Freudenthal.** The stable homotopy groups of spheres πₙ₊ₖ(S^n) (stable = independent of n for large n) are one of the deepest objects in algebraic topology. They are known in low dimensions (k ≤ 64 or so), but the general pattern is unknown.

**Cohomology generators.** H^n(S^n; Z) = Z — the n-sphere generates the n-dimensional cohomology of any space (via maps to S^n). This is the basis for the definition of Chern classes and other characteristic classes.

**CW complexes.** Every CW complex is built from spheres by attaching cells. The HITs of this chapter are the type-theoretic counterpart of CW complexes: built from points (0-cells), paths (1-cells), 2-paths (2-cells), etc.

**The Hopf invariant.** The Hopf invariant detects when maps S^{2n-1} → S^n are "non-trivial" (have non-trivial Hopf invariant). The Adams Hopf invariant problem — which n admit maps of Hopf invariant 1 — was solved by Adams in 1960 (the answer: n = 1, 2, 4, 8). The HoTT proof of the Hopf fibration (S^1 → S^3 → S^2) is the n=2 case.

## Summary

| Construction | Definition | Key property |
|---|---|---|
| ΣA (suspension) | N, S, merid(a):N=S | Σ ⊣ Ω adjunction |
| S^n | Σ^n(Bool) | All spheres via iteration |
| A * B (join) | inl, inr, jn(a,b):inl=inr | S^1 * S^1 = S^3 |

Suspension is the engine that generates all spheres. The loop space-suspension adjunction is the key structural property. Together, they provide the type-theoretic foundation for all of synthetic homotopy theory.
