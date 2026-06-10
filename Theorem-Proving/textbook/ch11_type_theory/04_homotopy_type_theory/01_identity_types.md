# Identity Types

In ordinary mathematics, two things are equal or they are not — equality is a proposition with a truth value. Homotopy Type Theory (HoTT) radically reframes this: *equality is a type*, and that type can have rich internal structure.

## The Identity Type

Given a type `A` and two elements `a b : A`, the *identity type* `a =_A b` (or just `a = b`) is the type of *proofs that a equals b*.

- If `a` and `b` are equal, this type is inhabited — it contains at least one term, called `refl a : a = a` (reflexivity).
- If `a` and `b` are not equal, this type is empty.

This looks innocent, but the payoff comes from asking: can there be *multiple distinct proofs of equality*? In classical mathematics, equality is a bare fact — but in HoTT, the identity type can have multiple inhabitants, and those inhabitants can themselves be compared for equality.

## Paths

The geometric intuition: think of `a = b` as the type of *paths* from `a` to `b` in a space `A`.

- `refl a` is the *constant path* at `a`
- A proof `p : a = b` is a path from `a` to `b`
- A proof `q : p = p'` (equality of proofs of equality) is a *homotopy* between paths — a continuous deformation from one path to another

This is not merely metaphor. HoTT's models are *homotopy types* (∞-groupoids), and the higher structure of the identity type corresponds exactly to the higher structure of homotopy theory.

## J-Elimination

The identity type comes with an elimination rule called *J* (path induction):

Given:
- A family `C : (a b : A) → (a = b) → Type`
- A term `c : C a a (refl a)` for all `a : A`

Conclude: for any `p : a = b`, there is a term `J(c, p) : C a b p`.

In words: to prove something about all paths `p : a = b`, it suffices to prove it for the constant path `refl`. Every proof about arbitrary equalities reduces to the reflexive case.

J is powerful: from it, we can derive symmetry (`p : a = b` implies `p⁻¹ : b = a`), transitivity (`p · q : a = c` from `p : a = b` and `q : b = c`), and congruence (`ap f p : f a = f b` from `p : a = b`).

## Higher Identity Types

Here's where it gets remarkable. Given `p q : a = b`, the type `p = q` is the type of *paths between paths* — a 2-dimensional identity type.

```
a -----p----> b
a -----q----> b
     ↕ α          (α : p = q)
```

And `α = β` for `α β : p = q` would be 3-dimensional paths. This hierarchy continues indefinitely. Types that have trivial structure above dimension n are called *n-types* or *homotopy n-truncations*:

| n | Name | Example |
|---|------|---------|
| -2 | Contractible | The unit type, any singleton |
| -1 | Proposition (Prop) | `a = b` for a set A |
| 0 | Set | ℕ, ℤ, any discrete type |
| 1 | Groupoid | The circle S¹ |
| ∞ | ∞-groupoid | General types |

## The Circle

The circle `S¹` in HoTT is defined by:
- One point: `base : S¹`
- One loop: `loop : base = base`

The fundamental group `π₁(S¹) = ℤ` — you can wind around the circle any integer number of times. This is a theorem about the *identity type structure* of `S¹`, proved entirely within type theory without any topology.

HoTT can reason about homotopy-theoretic properties of spaces using only type-theoretic constructions. This is the program of *synthetic homotopy theory*.

## Why This Matters for Proof Assistants

In Coq and Agda (without HoTT extensions), the identity type on sets behaves as expected — proofs of equality are unique (proof irrelevance holds for propositions). But HoTT opens up reasoning about *spaces* where equality proofs have computational content.

In Lean 4, the `Eq` type is the identity type. For propositions (types in `Prop`), proof irrelevance holds. For data types, identity types can have non-trivial structure — and `congr` lemmas, `Eq.subst`, and the entire `Mathlib` equality API are built on J-elimination.

The identity type is where computation meets geometry, where type theory touches topology, and where the foundations of mathematics reveal unexpected depth.
