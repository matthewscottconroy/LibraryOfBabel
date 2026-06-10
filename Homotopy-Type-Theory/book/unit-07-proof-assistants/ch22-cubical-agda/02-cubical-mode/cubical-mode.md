# Cubical Mode: Paths as Functions, Univalence as Theorem

The central idea of cubical type theory can be stated in one sentence: a path from `a` to `b` in type `A` is a function `p : I → A` such that `p i0 = a` and `p i1 = b`, *definitionally*. Not propositionally — definitionally. The endpoints are literally the values of the function at the interval endpoints.

This single idea has consequences that cascade through the entire theory.

## Enabling Cubical Mode

```agda
{-# OPTIONS --cubical #-}
module CubicalBasics where

open import Cubical.Core.Everything
open import Cubical.Foundations.Prelude
```

The `--cubical` pragma gives you new primitive types and operations. The `Cubical.Core.Everything` import provides access to the raw primitives. `Cubical.Foundations.Prelude` provides the standard toolkit built on them.

## The Interval I

The interval `I` is a new sort in cubical type theory. It is not a type in the usual sense — it is not in `Set` or `Set ℓ`. It is a new kind of thing: a formal interval with two endpoints and operations.

```agda
-- The two endpoints
i0 : I   -- the "left" endpoint, representing 0
i1 : I   -- the "right" endpoint, representing 1

-- Complement (flip endpoints)
~_ : I → I
-- ~ i0 = i1  (definitionally)
-- ~ i1 = i0  (definitionally)

-- Meet (min) and Join (max)
_∧_ : I → I → I   -- min(i, j): both must be "positive"
_∨_ : I → I → I   -- max(i, j): at least one must be "positive"

-- The De Morgan laws hold definitionally:
-- ~ (i ∧ j) = ~ i ∨ ~ j
-- ~ (i ∨ j) = ~ i ∧ ~ j
```

You cannot compare interval elements with each other (no decidable equality), do arithmetic on them, or treat them as `Bool`. They are purely formal parameters for constructing paths.

## Path Types: The Fundamental Change

In cubical type theory, the type `a ≡ b` is defined as:

```
{ p : I → A | p i0 = a, p i1 = b }
```

— functions from `I` to `A` with specified boundary values, where the boundary conditions hold *definitionally*.

```agda
-- Reflexivity: the constant path
refl : {A : Type ℓ} {a : A} → a ≡ a
refl {a = a} = λ i → a
-- Check: (λ i → a) i0 = a ✓   (holds definitionally)
--        (λ i → a) i1 = a ✓

-- Using refl
example : (42 : ℕ) ≡ 42
example = refl

-- Applying a path: if p : a ≡ b, then p i0 reduces to a
-- This holds definitionally — you can use it in computations
```

The critical difference from Martin-Löf paths: `p i0` and `p i1` reduce to the endpoints by computation, not just by a proof. This is what enables all the subsequent machinery.

## Path Inversion: Precompose with Complement

In Martin-Löf type theory, path inversion (`sym`) requires the J eliminator. In cubical type theory, it is just precomposition with `~`:

```agda
-- Symmetric path
sym : {A : Type ℓ} {a b : A} → a ≡ b → b ≡ a
sym p = λ i → p (~ i)
-- Check:
-- (sym p) i0 = p (~ i0) = p i1 = b  ✓  (p : a ≡ b, so p i1 = b)
-- (sym p) i1 = p (~ i1) = p i0 = a  ✓
```

One line. No J, no pattern matching on the path. This is a genuine function: it maps `I → A` by precomposing with the interval complement. The beauty of paths-as-functions is that every operation on paths becomes a familiar operation on functions.

## Function Extensionality: A Two-Line Theorem

In Martin-Löf type theory, function extensionality requires an axiom. In cubical type theory, it is an immediate consequence of what paths in function types are:

```agda
-- funExt: if f x ≡ g x for all x, then f ≡ g
funExt : {A : Type ℓ₁} {B : A → Type ℓ₂} {f g : (x : A) → B x}
         → (∀ x → f x ≡ g x) → f ≡ g
funExt h = λ i x → h x i
-- h x : f x ≡ g x  is a function I → B x
-- We want: I → (x : A) → B x
-- Just swap the arguments: λ i x → h x i
-- Check: at i = i0: λ x → h x i0 = λ x → f x = f  ✓
--        at i = i1: λ x → h x i1 = λ x → g x = g  ✓
```

The key insight: **a path in `A → B` is a function `I → A → B`, which is the same as a function `A → I → B` (since functions are extensional)**. Pointwise paths give a path between functions by rearranging arguments.

This is not a trick. It is a deep fact about how cubical type theory works: the path type is defined by the structure of the target type, and for function types, that structure says "paths are determined pointwise."

## Heterogeneous Paths: PathP

Sometimes you need a path between elements of *different* types, where the types themselves are connected by a path:

```agda
-- PathP B p a b:
-- a path from a : B i0 to b : B i1
-- lying over the path of types B : I → Type

-- PathP (λ i → A) a b is the same as a ≡ b (when A doesn't vary)

-- Example: transport gives a PathP
-- If e : A ≡ B and a : A, then there's a PathP from a to transport e a
-- PathP (λ i → e i) a (transport e a)

-- Using PathP in practice
example-pathp : {A : Type ℓ} {a b : A} (p : a ≡ b)
                → PathP (λ i → A) (p i0) (p i1)
example-pathp p = p   -- trivial: a path from p i0 to p i1 is just p
```

PathP is the dependent version of `≡`. It appears when you work with type families parametrized by `I` — exactly what happens with HITs, transport, and the `Glue` type.

## Transport: Computing with Type Paths

Transport takes an element of one type and moves it to another type along a path between the types:

```agda
-- transport: move along a type path
transport : {A B : Type ℓ} → A ≡ B → A → B
transport p a = transp (λ i → p i) i0 a

-- transp is the primitive. Its signature:
-- transp : (A : I → Type ℓ) → (φ : I) → A i0 → A i1
-- The second argument φ is for the "triviality condition":
-- transp A i1 a = a  (transport along a constant path is the identity)
```

The key property: `transp` has computation rules for each type former. For example:

```agda
-- Transport in function types
-- transp (λ i → A i → B i) i0 f
-- = λ x → transp B i0 (f (transp (λ i → A (~ i)) i0 x))
-- (transport the output, contravariantly transport the input)

-- Transport in Σ-types
-- transp (λ i → Σ (A i) (B i)) i0 (a , b)
-- = (transp A i0 a, transp (λ i → B i (transp A ... a)) i0 b)
-- (transport each component)

-- Transport in the path type: the square-filling operation
-- transp (λ i → a i ≡ b i) i0 p  fills a square
```

These computation rules are what restores canonicity. Transport in any type has a concrete recipe, so it always reduces.

## The hcomp Operation: Filling Boxes

`hcomp` is the composition operation. It takes an "open box" — a set of faces of a cube with specified values — and fills the missing face.

For path concatenation, we need to fill a square:

```agda
-- Path concatenation: p ∙ q for p : a ≡ b, q : b ≡ c
infixr 30 _∙_
_∙_ : {A : Type ℓ} {a b c : A} → a ≡ b → b ≡ c → a ≡ c
(p ∙ q) i = hcomp (λ j → λ { (i = i0) → a    -- left face: constant at a
                              ; (i = i1) → q j }) -- right face: traces q
                  (p i)                          -- bottom: traces p
```

Let's unpack:
- We're defining a path `I → A`, parameterized by `i`
- The `hcomp` "fills" a 2D square. The `λ j` gives the "top" dimension
- When `i = i0`: the path stays at `a` (constant)
- When `i = i1`: the path traces `q j` from `b` (at `j = i0`) to `c` (at `j = i1`)
- The "bottom" (base face, at `j = i0`) is `p i`, tracing from `a` to `b`
- The "top" (the face `hcomp` fills, at `j = i1`) is the concatenated path

This is literally box-filling in a 2-dimensional cube. The two paths `p` and `q` form three faces of a square; `hcomp` provides the fourth.

## Connections: Higher-Dimensional Path Operations

The interval operations `∧` and `∨` enable higher-dimensional constructions:

```agda
-- Left unit law: refl ∙ p ≡ p
-- Requires a 2D path (a homotopy between two paths)
lUnit : {A : Type ℓ} {a b : A} (p : a ≡ b) → (refl ∙ p) ≡ p
lUnit p i j = p (i ∧ j)
-- At i = i0: p (i0 ∧ j) = p i0 = a    (constant path)
-- At i = i1: p (i1 ∧ j) = p j         (the path p)
-- At j = i0: p (i ∧ i0) = p i0 = a    (left endpoint)
-- At j = i1: p (i ∧ i1) = p i         (right endpoint of lUnit p i)

-- Right unit law: p ∙ refl ≡ p
rUnit : {A : Type ℓ} {a b : A} (p : a ≡ b) → (p ∙ refl) ≡ p
rUnit p i j = p (i ∨ j)
-- Similarly, using ∨ instead of ∧
```

These are 2-dimensional proofs — proofs that two paths are equal, which are themselves paths between paths. The interval's meet and join operations parametrize the 2D structure naturally.

## The Glue Type and Univalence

The `Glue` type is the final primitive needed to prove univalence from scratch:

```agda
-- Univalence: equivalent types are identical
ua : {A B : Type ℓ} → A ≃ B → A ≡ B
ua {A = A} {B = B} e i =
  Glue B (λ { (i = i0) → A , e
            ; (i = i1) → B , idEquiv B })
-- At i = i0: the type is A, with the equivalence e : A ≃ B
-- At i = i1: the type is B, with the identity equivalence
-- In between: B "glued to" A along e
```

The `Glue` type constructor takes:
- A type `B` (the "underlying" type)
- A partial specification: on which faces of the interval, what type `A` is glued to `B` via what equivalence

The resulting type at any interior point looks like `B` but is decorated with the knowledge of how `A` relates to it via the equivalence.

### The computation rule: uaβ

This is the crucial property that makes everything work:

```agda
-- Transport along ua e reduces to applying e
uaβ : {A B : Type ℓ} (e : A ≃ B) (a : A) → transport (ua e) a ≡ e .fst a
uaβ e a = refl
-- This is definitionally true! (In the Cubical library, sometimes a path, not refl)
-- But definitionally or nearly-definitionally:
-- transport (ua e) a computes to e .fst a
```

When you transport along `ua e`, the result is the underlying function of `e` applied to the input. This is the computation rule for univalence. It is what was missing from axiomatic HoTT and what cubical type theory provides.

## A Complete Example: Bool ≡ Bool

Let's use `ua` to construct a non-trivial path in the universe:

```agda
open import Cubical.Data.Bool
open import Cubical.Foundations.Equiv
open import Cubical.Foundations.Univalence

-- The swap equivalence: Bool ≃ Bool
swapEquiv : Bool ≃ Bool
swapEquiv = isoToEquiv (iso not not notnot notnot)
  where
    notnot : ∀ b → not (not b) ≡ b
    notnot true  = refl
    notnot false = refl

-- A path: Bool ≡ Bool (not the trivial path!)
swapPath : Bool ≡ Bool
swapPath = ua swapEquiv

-- Transport along this path: swaps the boolean
swapTest : transport swapPath true ≡ false
swapTest = uaβ swapEquiv true
-- transport swapPath true = swapEquiv .fst true = not true = false ✓
```

Here we see the power: `swapPath` is a genuine non-trivial path in the universe `Type`. Transporting along it computes — it gives `false` when applied to `true`. This is impossible in axiomatic HoTT (without a separate computation rule), but in Cubical Agda, it is definitional.

## Summary Table: The Cubical Primitives

| Primitive | Type | What it does |
|-----------|------|--------------|
| `I` | Sort | The formal interval |
| `i0`, `i1` | `I` | The two endpoints |
| `~_` | `I → I` | Complement: `~ i0 = i1`, `~ i1 = i0` |
| `_∧_`, `_∨_` | `I → I → I` | Min and max (meet and join) |
| `_≡_` | `A → A → Type` | Path type: functions `I → A` with boundary |
| `refl` | `a ≡ a` | The constant path `λ i → a` |
| `sym` | `a ≡ b → b ≡ a` | Path reversal: `λ i → p (~ i)` |
| `transp` | `(I → Type) → I → A → B` | Primitive transport (with computation rules) |
| `hcomp` | partial box → fill | Box-filling for composition |
| `Glue` | type constructor | Gluing types along equivalences |
| `ua` | `A ≃ B → A ≡ B` | Univalence (theorem, not axiom!) |
| `PathP` | dependent path type | Paths lying over paths of types |
| `funExt` | `(∀ x → f x ≡ g x) → f ≡ g` | Function extensionality (theorem!) |

These primitives are not arbitrary. Each one corresponds to a geometric operation on the interval: reversing direction (`~`), intersecting directions (`∧`), taking the union of directions (`∨`), filling a partially-specified box (`hcomp`). Cubical type theory is, literally, geometry internalized in type theory.

The next section puts these primitives to work in the definition and proof of higher inductive types.
