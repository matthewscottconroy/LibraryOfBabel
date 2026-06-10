# Composition and Inverse Functions

Functions compose and invert — operations that reveal the algebraic structure hidden in the set of all functions.

## Composition

Given functions f : A → B and g : B → C, their *composition* g ∘ f : A → C is defined by:

> **(g ∘ f)(x) = g(f(x))**

Apply f first, then g. The arrow conventions in category theory match this: f then g, but written g ∘ f (right to left).

```
f : ℕ → ℕ,  f(n) = n + 1      (successor)
g : ℕ → ℕ,  g(n) = 2 * n      (doubling)

(g ∘ f)(3) = g(f(3)) = g(4) = 8
(f ∘ g)(3) = f(g(3)) = f(6) = 7
```

Composition is not commutative in general: g ∘ f ≠ f ∘ g.

**Associativity**: (h ∘ g) ∘ f = h ∘ (g ∘ f). Composition is associative — parenthesization doesn't matter.

**Identity**: The identity function id_A(x) = x satisfies f ∘ id_A = f and id_B ∘ f = f for any f : A → B.

These properties make functions into a *category*: objects are sets, morphisms are functions, composition is the categorical composition, and identities are the identity functions. Category theory abstracts this structure.

## Inverse Functions

A function f : A → B has an *inverse* f⁻¹ : B → A if:
- f⁻¹(f(a)) = a for all a ∈ A (left inverse property)
- f(f⁻¹(b)) = b for all b ∈ B (right inverse property)

Equivalently: f ∘ f⁻¹ = id_B and f⁻¹ ∘ f = id_A.

A function has an inverse if and only if it is *bijective* (both injective and surjective). This is why bijectivity is sometimes called being *invertible*.

**Uniqueness of inverses**: If g and h are both inverses of f, then:
```
g = g ∘ id_B = g ∘ (f ∘ h) = (g ∘ f) ∘ h = id_A ∘ h = h
```
Inverses are unique when they exist.

## Left and Right Inverses

Partial inverses exist even for non-bijective functions:

- f has a **left inverse** (retraction) g : B → A with g ∘ f = id_A iff f is *injective*.
- f has a **right inverse** (section) h : B → A with f ∘ h = id_B iff f is *surjective*.

The existence of a right inverse is equivalent to the Axiom of Choice — choosing an element from each fiber f⁻¹(b) for every b ∈ B.

## Composition and Bijectivity

Key closure properties:

- If f and g are injective, so is g ∘ f.
- If f and g are surjective, so is g ∘ f.
- If f and g are bijective, so is g ∘ f, and (g ∘ f)⁻¹ = f⁻¹ ∘ g⁻¹.

The last formula — the inverse of a composition reverses the order — is the "socks and shoes" principle: to undo putting on socks then shoes, remove shoes first, then socks.

## Permutations

The set of bijections from A to itself (permutations of A) is closed under composition and inverses. This makes it into a *group* — the *symmetric group* Sym(A).

For A = {1, 2, ..., n}, Sym(A) = S_n has n! elements. The structure of S_n is the subject of group theory, with deep connections to polynomial equations (Galois theory showed that the unsolvability of the quintic is equivalent to S_5 being a non-solvable group).

## In Type Theory

In type theory and Lean, the composition of functions is:

```lean
def comp {A B C : Type} (g : B → C) (f : A → B) : A → C :=
  fun x => g (f x)

-- Notation: g ∘ f
```

And a bijection `f : A ≃ B` (an equivalence in Lean's type-theoretic sense) bundles together f, f⁻¹, and proofs that they are mutual inverses. The univalence axiom in HoTT asserts that such equivalences are the same as equalities of types.
