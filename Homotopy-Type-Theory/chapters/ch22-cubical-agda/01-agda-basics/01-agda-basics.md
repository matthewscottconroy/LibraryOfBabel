# 1.1 Agda Basics

## What Is Agda?

Agda is a dependently typed proof assistant and programming language developed at Chalmers University. It's been around since the early 2000s (Ulf Norell's thesis formalized a significant part of the implementation), and it occupies a particular niche: it's more foundationally transparent than Lean 4, more willing to let you control exactly what axioms and type-theoretic features you use.

Where Lean 4 has a large collection of automation tactics and Mathlib for math, Agda is more of a *bare metal* dependent type theory. You have to do more work manually, but you have more control. And crucially for us, Agda supports the `--cubical` pragma that gives us the full computational version of HoTT.

Agda is also a real programming language: you can compile Agda to Haskell and run your programs. This is relevant because, in the cubical setting, every proof has computational content — proofs are programs.

## Installation

```bash
# Install via Haskell's cabal
cabal install Agda

# Or via nix (recommended for reproducibility)
nix-env -iA nixpkgs.agda agda-stdlib

# Or via the package manager
# Ubuntu/Debian: apt install agda agda-stdlib
# Arch: yay -S agda agda-stdlib
```

For editor support:
- **Emacs**: `agda2-mode` (the traditional, most feature-complete option)
- **VS Code**: `agda-mode` extension (newer, more accessible)
- **Neovim**: `cornelis` plugin

The editor integration is important: Agda is interactive. You type-check interactively, fill in holes (`?` or `{! !}`) one at a time, and ask Agda to show you the type of the current hole context. This is similar to Lean 4's tactic mode but more explicit.

## Basic Syntax

Every Agda file starts with a module declaration:

```agda
module Introduction where
```

The module name must match the filename (`Introduction.agda`).

### Types and terms

```agda
-- Type annotations use :
x : Nat
x = 42

-- Functions are defined by giving the type and then the definition
-- (similar to Lean 4's term mode)
add : Nat → Nat → Nat
add zero    m = m
add (suc n) m = suc (add n m)

-- Implicit arguments use curly braces
id : {A : Set} → A → A
id x = x
```

`Set` in Agda is the universe of small types, equivalent to `Type 0` in Lean 4. We'll say more about universe levels shortly.

### Inductive types

```agda
-- Natural numbers (already in the standard library, but shown for illustration)
data Nat : Set where
  zero : Nat
  suc  : Nat → Nat

-- Lists
data List (A : Set) : Set where
  []  : List A
  _∷_ : A → List A → List A

-- Boolean
data Bool : Set where
  true  : Bool
  false : Bool

-- The empty type
data ⊥ : Set where   -- no constructors

-- The unit type
data ⊤ : Set where
  tt : ⊤
```

Pattern matching is the primary way to work with inductive types:

```agda
-- Negation: ⊥ → A
¬_ : Set → Set
¬ A = A → ⊥

-- Boolean negation
not : Bool → Bool
not true  = false
not false = true

-- Length of a list
length : {A : Set} → List A → Nat
length []       = zero
length (x ∷ xs) = suc (length xs)
```

### Dependent types

Agda has full dependent types — types that depend on values:

```agda
-- Π types (dependent function types)
-- The syntax ∀ (x : A) → B x or (x : A) → B x
-- is the dependent function type

-- An example: a function returning a proof depending on the input
isZero : Nat → Set
isZero zero    = ⊤
isZero (suc n) = ⊥

-- So isZero 0 = ⊤ and isZero 1 = isZero (suc 0) = ⊥

-- Dependent pair types (Σ types)
-- The standard library uses record syntax
record Σ (A : Set) (B : A → Set) : Set where
  constructor _,_
  field
    fst : A
    snd : B fst

-- Shorthand: ∃ (x : A) , B x   or   Σ A B
-- Example: pairs (n, proof that n is even)
data IsEven : Nat → Set where
  even-zero : IsEven zero
  even-suc  : {n : Nat} → IsEven n → IsEven (suc (suc n))

-- A type of even naturals
EvenNat : Set
EvenNat = Σ Nat IsEven   -- pairs (n, proof that n is even)

-- An inhabitant: (4, proof that 4 is even)
four-is-even : EvenNat
four-is-even = (4 , even-suc (even-suc even-zero))
```

### Records

Agda uses `record` for structured data (like Lean 4's `structure`):

```agda
-- A point in the plane
record Point : Set where
  constructor mkPoint
  field
    x : Nat
    y : Nat

-- Create a point
origin : Point
origin = mkPoint 0 0

-- Access fields
getX : Point → Nat
getX p = Point.x p   -- or: p .Point.x  in newer Agda
```

Records are codata types: they're defined by their projections. Agda allows `record` to define type classes (interfaces), just like Lean 4:

```agda
-- A "monoid" record
record IsMonoid (M : Set) : Set where
  field
    ε    : M              -- the unit element
    _·_  : M → M → M     -- the multiplication
    assoc : ∀ a b c → (a · b) · c ≡ a · (b · c)
    unitL : ∀ a → ε · a ≡ a
    unitR : ∀ a → a · ε ≡ a
```

## Universe Levels

Agda has a universe hierarchy: `Set : Set₁ : Set₂ : ...`. More precisely:

```agda
-- Set = Set₀ = Set lzero (the universe of "small" types)
-- Set₁ contains Set₀, etc.

-- Universe-polymorphic programming uses Level:
open import Agda.Primitive using (Level; lzero; lsuc; _⊔_)

-- id works for types at any universe level:
id' : {ℓ : Level} {A : Set ℓ} → A → A
id' x = x

-- The type of types at level ℓ
Type : (ℓ : Level) → Set (lsuc ℓ)
Type ℓ = Set ℓ

-- Implicit universe variables: modern Agda allows
variable ℓ ℓ₁ ℓ₂ : Level
-- Then any definition using ℓ is automatically universe-polymorphic
```

This is the same hierarchy as Lean 4's `Type u`, `Type (u+1)`, etc. For HoTT purposes, `ℓ ⊔ ℓ'` is the *join* of two universe levels (the larger of the two), needed for constructions like Σ-types where the two components live in different universes.

## The Identity Type

In standard Agda (`--without-K` mode), the identity type is the usual Martin-Löf equality:

```agda
data _≡_ {A : Set} (a : A) : A → Set where
  refl : a ≡ a
```

Just like in the HoTT Book: the only constructor is reflexivity, but by J (path induction), you can prove things by induction on paths.

**The J eliminator** (path induction):

```agda
-- J : the path induction principle
J : {A : Set} (a : A)
    (P : (b : A) → a ≡ b → Set)
    → P a refl
    → (b : A) (p : a ≡ b) → P b p
J a P pr .a refl = pr
-- The only case is when b = a and p = refl
-- In this case P a refl = P a refl, and we have pr
```

This is definitional: `J a P pr a refl` *reduces to* `pr`. The dot pattern `.a` means "this is forced to be `a` by the pattern match on `refl`."

### The `--without-K` pragma

The K axiom says: any proof of `a ≡ a` is equal to `refl`. This is equivalent to UIP (uniqueness of identity proofs) and to saying all types are sets.

To work with HoTT (where K fails), add:

```agda
{-# OPTIONS --without-K #-}
```

With `--without-K`:
- Pattern matching on `refl` is only allowed when it's sound (roughly: when the argument is clearly a variable)
- The K axiom becomes unprovable
- You're working in type theory where non-trivial paths can exist

**Example of what `--without-K` blocks:**

```agda
{-# OPTIONS --without-K #-}
module WithoutK where

-- This is NOT provable (without K):
K : {A : Set} {a : A} (P : a ≡ a → Set) → P refl → (p : a ≡ a) → P p
-- K would need to conclude that any loop p : a ≡ a equals refl,
-- but in HoTT loops can be non-trivial

-- This would be fine with K but fails without K:
-- (pattern matching on p when a is not a variable isn't allowed)
```

`--without-K` is the mode for "Agda as a HoTT proof assistant without full computational content." You can state and prove HoTT theorems, but univalence is still an axiom (you have to postulate it), and HITs are also axioms.

For full computational HoTT — where univalence is a theorem and HITs are first-class — you need `--cubical`, which we cover in the next section.

## Working Interactively

One thing that takes getting used to: Agda is meant to be used interactively. Here's the workflow:

1. Write the type signature.
2. Put `?` in the body: `myProof = ?`
3. Load the file (C-c C-l in Emacs, or the corresponding VS Code command).
4. Agda shows you the goal: what type you need to prove.
5. Use commands to:
   - `C-c C-,` (or click): show the current goal context
   - `C-c C-c`: case split on a variable (like `cases` in Lean 4)
   - `C-c C-a`: try to automatically fill the hole
   - `C-c C-r`: refine: type something in the hole and let Agda fill the rest
6. Repeat until all holes are filled.

This is more manual than Lean 4's tactic mode, but it's also more transparent: you see exactly what type-theoretic operations you're performing.

### A complete worked example

Let's prove that `not (not b) = b` for `Bool`:

```agda
{-# OPTIONS --without-K #-}
module BoolExample where

data Bool : Set where
  true  : Bool
  false : Bool

not : Bool → Bool
not true  = false
not false = true

-- Proof by case analysis
not-not : (b : Bool) → not (not b) ≡ b
not-not true  = refl   -- not (not true) = not false = true ≡ true
not-not false = refl   -- not (not false) = not true = false ≡ false
```

Both cases reduce definitionally to `refl`, so the proof is trivial. This is the simplest kind of proof in Agda: by computation.

## From Agda to Cubical Agda

Standard Agda (even with `--without-K`) is not enough for full HoTT:
- Univalence must be postulated as an axiom
- HITs must be postulated as axioms
- The resulting theory lacks canonicity (you can't always evaluate terms)

Cubical Agda changes all of this by replacing the identity type with a fundamentally different notion of paths. Section 2 explains how.

The key point to take away from this section: Agda gives you a bare-metal dependent type theory where you can precisely control what you assume. `--without-K` removes UIP. `--cubical` gives you paths-as-functions and makes HoTT computable. Understanding the basic Agda syntax here will let you read and write cubical proofs in the next sections.
