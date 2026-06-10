# Agda Basics: Dependent Types at the Metal

Agda is a dependently typed proof assistant and programming language developed at Chalmers University. It occupies a distinctive position in the proof assistant landscape: more foundationally transparent than Lean 4, more willing to expose the raw type theory, and crucially, the platform for Cubical Agda — the computational implementation of HoTT.

Where Lean 4 prioritizes ergonomics (rich tactic language, large library, active automation), Agda prioritizes precision. You see more; you control more; you do more manually. The tradeoff is productive: when Agda accepts your proof, you understand it at a finer level than the corresponding Lean 4 proof would require.

## Installation

```bash
# Method 1: via Cabal (Haskell package manager)
cabal install Agda

# Method 2: via Nix (recommended for reproducibility)
nix-env -iA nixpkgs.agda nixpkgs.agdaPackages.cubical

# Method 3: via your OS package manager
# Ubuntu/Debian:
apt install agda agda-stdlib

# Arch:
yay -S agda agda-stdlib

# After installation, check:
agda --version   # Should print: Agda version 2.x.y
```

For the Cubical Agda library specifically, you need to download it separately:

```bash
# Clone the Cubical library
git clone https://github.com/agda/cubical.git
cd cubical

# Point Agda to it by creating ~/.agda/libraries with content:
# /path/to/cubical/cubical.agda-lib
```

## Editor Support

Agda is designed for interactive use. You need an editor that connects to Agda's interactive mode:

- **Emacs with agda2-mode**: the traditional, most feature-complete option. Install Agda, then `M-x package-install agda2-mode`. Key bindings: `C-c C-l` (load), `C-c C-,` (show goal), `C-c C-a` (auto), `C-c C-c` (case split).
- **VS Code with agda-mode extension**: newer, more accessible. Install from the marketplace.
- **Neovim with cornelis**: for Neovim users.

Unlike Lean 4 (which does background checking continuously), Agda requires you to explicitly load a file (`C-c C-l` in Emacs). Once loaded, Agda highlights the file: orange means "still checking," green means "checked," red means "error." Holes (written `?` or `{! !}`) are goals to fill.

## Module System

Every Agda file begins with a module declaration matching the filename:

```agda
-- File: Basics.agda
module Basics where
```

The module name must match the filename exactly. Modules can be parameterized:

```agda
-- File: TypedExpressions.agda
module TypedExpressions (Base : Set) where

-- Everything in the module uses Base
```

Importing other modules:

```agda
module MyFile where

-- Import from standard library
open import Data.Nat           -- natural numbers
open import Data.Bool          -- booleans
open import Data.List          -- lists
open import Relation.Binary.PropositionalEquality  -- identity type

-- The "open" makes names available without module prefix
-- Without "open": Data.Nat.ℕ, Data.Bool.Bool, etc.
-- With "open":    ℕ, Bool, etc.
```

## Basic Syntax

Agda uses a mix of Haskell-like and mathematical notation:

```agda
module Syntax where

-- Type annotations use :
-- Definitions use =
n : ℕ
n = 42

-- Functions: type signature on one line, definition on the next
double : ℕ → ℕ
double n = n + n

-- Anonymous functions use λ (or \lambda → \)
double' : ℕ → ℕ
double' = λ n → n + n

-- Multiple arguments
add : ℕ → ℕ → ℕ
add m n = m + n

-- Implicit arguments use curly braces; Agda infers them
-- id : {A : Set} → A → A
-- When calling id, Agda infers A from the argument
id : {A : Set} → A → A
id x = x

-- Using implicit arguments explicitly with {}
example : ℕ
example = id {ℕ} 42   -- explicit: A = ℕ
```

## Data Types

Agda uses `data` for inductive type definitions:

```agda
-- Booleans
data Bool : Set where
  true  : Bool
  false : Bool

-- Natural numbers
data ℕ : Set where
  zero : ℕ
  suc  : ℕ → ℕ

-- List (parameterized)
data List (A : Set) : Set where
  []  : List A
  _∷_ : A → List A → List A

-- The empty type (no constructors — nothing inhabits it)
data ⊥ : Set where

-- The unit type
data ⊤ : Set where
  tt : ⊤

-- Binary trees
data Tree (A : Set) : Set where
  leaf : Tree A
  node : Tree A → A → Tree A → Tree A
```

Pattern matching on inductive types:

```agda
-- Boolean operations
not : Bool → Bool
not true  = false
not false = true

_&&_ : Bool → Bool → Bool
true  && b = b
false && _ = false

-- Natural number operations
_+_ : ℕ → ℕ → ℕ
zero  + n = n
suc m + n = suc (m + n)

-- Length of a list
length : {A : Set} → List A → ℕ
length []       = zero
length (x ∷ xs) = suc (length xs)
```

## Dependent Types

The essential feature — types that depend on values:

```agda
-- A type predicate: IsZero n : Set
IsZero : ℕ → Set
IsZero zero    = ⊤     -- IsZero 0 = ⊤ (true)
IsZero (suc n) = ⊥     -- IsZero (suc n) = ⊥ (false)

-- A proof that 0 is zero
zeroIsZero : IsZero zero
zeroIsZero = tt

-- Vector: lists indexed by their length
data Vec (A : Set) : ℕ → Set where
  []  : Vec A zero
  _∷_ : {n : ℕ} → A → Vec A n → Vec A (suc n)

-- A safe head: only callable on non-empty vectors (guaranteed by type!)
head : {A : Set} {n : ℕ} → Vec A (suc n) → A
head (x ∷ _) = x
-- No need for an error case: the type prevents calling head on []

-- Σ-types: dependent pairs
-- Agda's standard library uses:
record Σ (A : Set) (B : A → Set) : Set where
  constructor _,_
  field
    fst : A
    snd : B fst

-- Type of natural numbers paired with their square
NatWithSquare : Set
NatWithSquare = Σ ℕ (λ n → ℕ)  -- really: Σ ℕ (λ n → n^2)

-- Example: (3, 9)
example₁ : NatWithSquare
example₁ = (3 , 9)
```

## Records

Agda uses `record` for structured data (similar to Lean 4's `structure`):

```agda
-- A point in the plane
record Point : Set where
  constructor mkPoint
  field
    x : ℕ
    y : ℕ

-- Construct a point
origin : Point
origin = mkPoint zero zero

-- Access fields (using field names directly)
getX : Point → ℕ
getX p = Point.x p

-- Pattern matching on records
distance-from-origin : Point → ℕ
distance-from-origin record { x = x ; y = y } = x + y  -- taxicab distance

-- Records can define type classes
record IsMonoid (M : Set) : Set where
  field
    ε    : M
    _·_  : M → M → M
    assoc : ∀ a b c → (a · b) · c ≡ a · (b · c)
    unitL : ∀ a → ε · a ≡ a
    unitR : ∀ a → a · ε ≡ a
```

## Universe Levels

Agda has a cumulative universe hierarchy: `Set : Set₁ : Set₂ : ...`

```agda
open import Agda.Primitive using (Level; lzero; lsuc; _⊔_)

-- Universe-polymorphic identity
id' : {ℓ : Level} {A : Set ℓ} → A → A
id' x = x

-- The universe at level ℓ is Set ℓ
-- Set = Set lzero = Set₀

-- Σ-type at arbitrary universe levels
record Σ' {ℓ₁ ℓ₂ : Level} (A : Set ℓ₁) (B : A → Set ℓ₂) : Set (ℓ₁ ⊔ ℓ₂) where
  constructor _,_
  field fst : A ; snd : B fst

-- Using "variable" for implicit universe levels (modern Agda)
private variable
  ℓ ℓ₁ ℓ₂ : Level
  A B : Set ℓ
```

The `⊔` operation takes the maximum of two levels: if `A : Set ℓ₁` and `B : Set ℓ₂`, then `A × B : Set (ℓ₁ ⊔ ℓ₂)`.

## The Identity Type (Standard Agda)

In standard Agda (without `--cubical`), the identity type is Martin-Löf equality:

```agda
{-# OPTIONS --without-K #-}  -- disable the K axiom
module WithoutK where

-- The identity type: a ≡ b
data _≡_ {A : Set} (a : A) : A → Set where
  refl : a ≡ a

-- J: path induction (the fundamental principle)
J : {A : Set} {a : A}
    (P : (b : A) → a ≡ b → Set)
    → P a refl
    → {b : A} → (p : a ≡ b) → P b p
J P pr refl = pr
-- The only pattern for p is refl, which forces b = a

-- Basic path operations
sym : {A : Set} {a b : A} → a ≡ b → b ≡ a
sym refl = refl

trans : {A : Set} {a b c : A} → a ≡ b → b ≡ c → a ≡ c
trans refl q = q

cong : {A B : Set} (f : A → B) {a₁ a₂ : A} → a₁ ≡ a₂ → f a₁ ≡ f a₂
cong f refl = refl
```

The `--without-K` pragma is crucial for HoTT: it disables the K axiom (which would collapse all paths to `refl`). With `--without-K`, you can state and prove HoTT theorems — but univalence is still an axiom, HITs are still axioms, and computations may be stuck.

## The Interactive Workflow

Agda development is interactive. Here's the workflow:

```agda
-- Step 1: Write the type signature
myProof : ∀ (n : ℕ) → n + zero ≡ n

-- Step 2: Use ? to introduce a hole
myProof = ?

-- Step 3: Load the file (C-c C-l)
-- Agda shows: Goal: ∀ (n : ℕ) → n + zero ≡ n

-- Step 4: Introduce n
myProof n = ?
-- Goal: n + zero ≡ n

-- Step 5: Case split on n (C-c C-c on n)
-- Agda generates two cases:
myProof zero    = ?   -- Goal: zero + zero ≡ zero
myProof (suc n) = ?   -- Goal: suc n + zero ≡ suc n

-- Step 6: Fill the base case
myProof zero    = refl  -- zero + zero reduces to zero by definition

-- Step 7: Fill the inductive case using recursion
myProof (suc n) = cong suc (myProof n)
-- myProof n : n + zero ≡ n
-- cong suc : n ≡ n' → suc n ≡ suc n'
-- result: suc n + zero ≡ suc (n + zero) ≡ suc n
```

Key Agda interactive commands:
- `C-c C-l`: load the file (check all definitions)
- `C-c C-,`: show the type of the current hole
- `C-c C-c`: case split on a variable in the current hole
- `C-c C-a`: try to fill the hole automatically
- `C-c C-r`: refine: type an expression and let Agda fill the rest
- `C-c C-n`: normalize an expression (compute its value)

## From --without-K to --cubical

The `--without-K` flag removes the K axiom but doesn't give you full computational HoTT. Univalence and HITs must still be postulated as axioms, breaking canonicity.

The `--cubical` flag changes the foundational layer: it replaces the identity type with path types (functions out of the interval) and adds the primitives `I`, `i0`, `i1`, `~_`, `hcomp`, `transp`, and `Glue`. With `--cubical`:

- Univalence is a *theorem* (from `Glue`)
- Function extensionality is a *theorem* (from how paths in function types work)
- HITs are *data types* (with path constructors)
- Everything computes

The next section shows you how.
