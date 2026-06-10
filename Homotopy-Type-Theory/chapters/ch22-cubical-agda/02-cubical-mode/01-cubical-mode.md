# 2.1 Cubical Mode

## The Problem with Axiomatic Univalence

In the HoTT Book presentation, univalence is an *axiom*:

$$\mathsf{ua} : (A \simeq B) \to (A = B)$$

Adding this axiom is sound (relative to the existence of Kan-complex models), but it breaks *canonicity*. In a type theory with canonicity, every closed term of type `Nat` reduces to a numeral `suc (suc ... (zero))`. With axiomatic univalence, a term like:

```
transport (ua succ-equiv) (3 : ℤ)
```

doesn't reduce — it's stuck, because `ua succ-equiv` is an axiom with no computation rule. The result *should* be `4`, but the type theory can't figure that out.

This matters practically: proofs-as-programs don't work if they can't run. Canonicity is the guarantee that computations are meaningful.

## The Cubical Solution: Paths as Functions

Cubical type theory replaces the identity type `a = b` with a *path type* `a ≡ b`. The key difference:

**Identity type** (Martin-Löf): `a ≡ b` is a new type constructor, with `refl` as its only constructor, and J as its only eliminator. The endpoints are *propositional* — you can't compute them.

**Path type** (Cubical): `a ≡ b` is the type of functions `I → A` from the *interval* `I` to `A`, satisfying the boundary conditions that `p i0 = a` and `p i1 = b` *definitionally*. The endpoints are *definitional* — you can compute them.

This single change makes everything work out:
- Transport becomes computable (it's a primitive `transp` operation with computation rules)
- Composition becomes computable (primitive `hcomp`)
- Univalence becomes provable from the `Glue` type constructor
- HITs get actual computation rules on path constructors

## Enabling Cubical Mode

```agda
{-# OPTIONS --cubical #-}
module CubicalBasics where

open import Cubical.Core.Everything
open import Cubical.Foundations.Prelude
```

The `--cubical` pragma gives you:
- The interval type `I` with `i0 i1 : I` and `~_ : I → I` (complement)
- Path types `a ≡ b` defined as `(i : I) → A` with boundary `p i0 = a` and `p i1 = b`
- Primitive `transp` for transport
- Primitive `hcomp` for composition
- The `Glue` type constructor

The `Cubical.Core.Everything` and `Cubical.Foundations.Prelude` modules provide the standard toolkit built on these primitives.

## The Interval Type `I`

The interval `I` is a new sort, separate from `Set`. It's not a type in the usual sense — it's a formal interval with two distinct points and operations.

```agda
-- The two endpoints
i0 : I   -- "left endpoint" (the 0)
i1 : I   -- "right endpoint" (the 1)

-- Complement (flip)
~_ : I → I
-- ~ i0 = i1
-- ~ i1 = i0

-- Meet and join (min and max)
_∧_ : I → I → I    -- min(i, j)
_∨_ : I → I → I    -- max(i, j)

-- These satisfy:
-- i ∧ i0 = i0
-- i ∧ i1 = i
-- i ∨ i0 = i
-- i ∨ i1 = i1
-- ~ (i ∧ j) = ~ i ∨ ~ j   (De Morgan)
```

You can't do arithmetic on `I` or compare elements. It's not `Bool` — it doesn't have decidable equality. It's a pure formal interval for parametrizing paths.

## Path Types

A *path* from `a` to `b` in type `A` is a function `I → A` with specified endpoints:

```agda
-- The path type a ≡ b = PathP (λ i → A) a b
-- Expanded: a function I → A sending i0 to a and i1 to b

-- Reflexivity: the constant path
refl : {A : Type} {a : A} → a ≡ a
refl = λ i → a   -- the constant function at a
-- Check: (λ i → a) i0 = a ✓ and (λ i → a) i1 = a ✓
```

The endpoints are *definitional equalities*, not just propositional. If `p : a ≡ b`, then `p i0` *reduces to* `a` and `p i1` *reduces to* `b`. This is the crucial difference from the identity type, where you can only access endpoints through J.

### Path inversion

```agda
-- Inversion: reverse the path
sym : {A : Type} {a b : A} → a ≡ b → b ≡ a
sym p = λ i → p (~ i)
-- Check: sym p i0 = p (~ i0) = p i1 = b ✓
--        sym p i1 = p (~ i1) = p i0 = a ✓
```

Path inversion is just *precomposing with the complement* `~`. Elegant.

### Path concatenation (connection)

Concatenating two paths requires filling a *square* — a 2-dimensional version of the interval. This uses `hcomp`:

```agda
-- Concatenation: p : a ≡ b, q : b ≡ c, gives a ≡ c
infixr 30 _∙_
_∙_ : {A : Type} {a b c : A} → a ≡ b → b ≡ c → a ≡ c
p ∙ q = λ i →
  hcomp (λ j → λ { (i = i0) → a           -- left face: stays at a
                  ; (i = i1) → q j })      -- right face: traces q
        (p i)                              -- bottom: traces p
```

This is harder to read than path inversion. Let's unpack `hcomp`.

## The `hcomp` Composition Operation

`hcomp` is the primitive composition operation. It fills a "partial box" to a complete element.

The idea: imagine you have an open box in a space — three faces of a cube, say, and you want to find the fourth face that "closes" it. `hcomp` does this.

```agda
-- hcomp : given a partial element u : ∀ j → Partial φ A
--         and a base element a : A [ φ ↦ u i0 ]
--         produces an element A [ φ ↦ u i1 ]

hcomp : {A : Type} {φ : I}
        (u : ∀ (j : I) → Partial φ A)  -- the "box" sides
        (a : A)                          -- the "bottom" face
        → A
-- The result is the element at the "top" of the box
```

`Partial φ A` is the type of elements of `A` that are only defined when formula `φ` holds. A formula `φ` is built from `I` values: `i = i0`, `i = i1`, conjunctions `φ₁ ∧ φ₂`, etc.

For path concatenation:
- The formula `φ = (i = i0) ∨ (i = i1)` says "we provide values at the left and right endpoints of `i`"
- At `i = i0`, we stay at `a` (constant path)
- At `i = i1`, we trace `q j` (the second path)
- The base at `j = i0` is `p i` (the first path)
- `hcomp` fills in the interior

## Heterogeneous Paths

In HoTT, transport takes you from one fiber to another. Cubical Agda has *PathP* ("path over a path") for this:

```agda
-- PathP B p a b :
-- a path from a : B i0 to b : B i1 lying over p : A i0 ≡ A i1
-- where B : I → Type (a family of types parametrized by I)

-- PathP (λ i → A) a b is the same as a ≡ b when A doesn't depend on i

-- Example: transport gives a PathP
-- If e : A ≡ B, then for any a : A:
-- transport e a : B, and there's a PathP between a and (transport e a)
```

This is the dependent (heterogeneous) version of paths. It's used heavily when working with type families.

## Transport

Transport in cubical type theory is a primitive operation with a computation rule:

```agda
-- The primitive transport operation
transp : (A : I → Type) (i0 : I) (a : A i0) → A i1

-- For explicit usage:
transport : {A B : Type} → A ≡ B → A → B
transport p a = transp (λ i → p i) i0 a
-- This transports a : A along the path p : A ≡ B to get an element of B
```

The key feature: `transp` has *computation rules* for each type former. For example:

```agda
-- Transport in a Σ-type:
transp (λ i → Σ (A i) (B i)) i0 (a , b) =
  (transp A i0 a , transp (λ i → B i (transp A ... a)) i0 b)

-- Transport in a Π-type:
transp (λ i → A i → B i) i0 f = λ x → transp B i0 (f (transp (λ i → A (~ i)) i0 x))

-- Transport in the identity type:
-- transp (λ i → (p i) ≡ (q i)) i0 r = ...
```

These computation rules are the key that restores canonicity: transport in any type now has a concrete recipe.

## The `Glue` Type and Univalence

The `Glue` type is the final primitive needed to prove univalence. The idea: if you have a type `B` and, on part of the "boundary" (a subinterval or a face), an equivalence to `A`, then `Glue B equiv` is a type that "glues" `A` to `B` along the equivalence.

```agda
-- Glue : given B : Type, and partial data (A, e : A ≃ B) on φ
-- Glue B φ T e : Type
-- where T : Partial φ Type  (the types on the boundary)
-- and   e : Partial φ (Σ A, A ≃ B)  (the equivalences)

-- Univalence: paths between types correspond to equivalences
ua : {A B : Type} → A ≃ B → A ≡ B
ua {A} {B} e i = Glue B
  (λ { (i = i0) → A , e         -- at i = 0, A equipped with e
     ; (i = i1) → B , idEquiv B }) -- at i = 1, B with the identity
```

Let's unpack: `ua e i` is a type. When `i = i0`, it's `A` (via the Glue type's first face). When `i = i1`, it's `B` (via the second face). In the interior, it's "A glued to B along e." So `ua e : A ≡ B` is literally a path from type `A` to type `B`.

### The computation rule

The critical property:

```agda
-- uaβ : transport along ua e reduces to applying e
uaβ : {A B : Type} (e : A ≃ B) (a : A) → transport (ua e) a ≡ e .fst a
uaβ e a = refl   -- or a simple path; this holds definitionally (or nearly so)
```

When you transport along `ua e`, the result is the same as applying the underlying function of `e`. This is the computation rule for univalence. In MLTT + axiomatic univalence, this only holds propositionally (you get a path between them). In cubical type theory, this holds definitionally — it's a *computation rule*.

## Function Extensionality

In cubical type theory, function extensionality is immediate — no axiom needed:

```agda
-- funExt: if f x ≡ g x for all x, then f ≡ g
funExt : {A : Type} {B : A → Type} {f g : (x : A) → B x}
         → (∀ x → f x ≡ g x) → f ≡ g
funExt p = λ i x → p x i
```

Proof: given `p : ∀ x, f x ≡ g x`, we need a path `f ≡ g` in the function type `(x : A) → B x`. A path in a function type is itself a function, so we need `I → (x : A) → B x`. Just define `λ i x → p x i`.

Check: at `i = i0`, this is `λ x → p x i0 = λ x → f x = f`. At `i = i1`, it's `λ x → g x = g`. So it's a path from `f` to `g`. ✓

This is a two-line proof! In MLTT, function extensionality requires a separate axiom. In cubical type theory, it's a consequence of what it means to be a path in a function type.

The key insight: **a path in `A → B` is the same as a function `I → A → B`, which is the same as a function `A → I → B`**. So pointwise paths give a path between functions, just by rearranging arguments.

## Connections: Higher-Dimensional Path Operations

The meet `∧` and join `∨` on `I` allow you to build higher-dimensional paths. For example, the left unit law for path concatenation:

```agda
-- refl ∙ p ≡ p
-- This requires filling a 2D square
leftUnit : {A : Type} {a b : A} (p : a ≡ b) → refl ∙ p ≡ p
leftUnit p i j = p (i ∧ j)
-- At i = i0: p (i0 ∧ j) = p i0 = a (constant path) ✓
-- At i = i1: p (i1 ∧ j) = p j (the path p) ✓
-- At j = i0: p (i ∧ i0) = p i0 = a ✓
-- At j = i1: p (i ∧ i1) = p i ✓
```

The 2D nature appears: `leftUnit p` is a path between paths (i.e., a homotopy between `refl ∙ p` and `p`). It's a function `I → I → A`, and both `i` and `j` range over the interval.

This is a preview of the coherence structure in HoTT: every path groupoid law has a 2D proof using the cubical structure.

## Summary: The Cubical Primitives

| Primitive | Type | Meaning |
|-----------|------|---------|
| `I` | Sort | The interval |
| `i0`, `i1` | `I` | The two endpoints |
| `~_` | `I → I` | Complement (1-i) |
| `_∧_`, `_∨_` | `I → I → I` | Min, max |
| `_≡_` | `A → A → Type` | Path type (`I → A` with fixed endpoints) |
| `refl` | `a ≡ a` | Constant path |
| `sym` | `a ≡ b → b ≡ a` | Path reversal |
| `transp` | `(I → Type) → A → B` | Transport (primitive) |
| `hcomp` | partial box → element | Composition (filling) |
| `Glue` | type constructor | Gluing types along equivalences |
| `ua` | `A ≃ B → A ≡ B` | Univalence (provable!) |
| `PathP` | dependent path type | Path over a path |

These primitives, together, give a type theory where all of HoTT can be done computationally. In the next section, we put them to work defining higher inductive types.
