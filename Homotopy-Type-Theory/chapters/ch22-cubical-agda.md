# Chapter 22: Cubical Agda — HoTT with Computational Content

## Introduction

Agda is a dependently typed programming language and proof assistant developed by Ulf Norell and the Agda team at Chalmers University. Unlike Lean 4, which focuses on classical mathematics (via Mathlib), Agda is closer to the type-theoretic foundations — it can be used with or without classical axioms, and in its *cubical mode*, it provides a computational interpretation of homotopy type theory.

Cubical Agda is particularly important for HoTT research because:
1. Univalence is a **theorem** (not an axiom) — it has computational content
2. Higher inductive types are defined constructively
3. All computations reduce to canonical forms (canonicity)

This chapter introduces Agda's syntax and the Cubical library, focusing on HoTT-relevant content.

---

## 1. Agda Basics

### 1.1 Installation

```bash
# Install Agda via cabal
cabal install Agda

# Or via nix
nix-env -iA nixpkgs.agda

# Install the standard library
# Download from github.com/agda/agda-stdlib
```

VS Code extension: `agda-mode` or `agda2-mode` (for Emacs).

### 1.2 Basic Syntax

```agda
module Introduction where

-- Type of types
_ : Set₁
_ = Set

-- Function types
id : {A : Set} → A → A
id x = x

-- Dependent function type (Π type)
const : {A B : Set} → A → B → A
const x _ = x

-- Data types
data Nat : Set where
  zero : Nat
  suc  : Nat → Nat

-- Pattern matching
add : Nat → Nat → Nat
add zero    m = m
add (suc n) m = suc (add n m)
```

### 1.3 Universe Levels

```agda
open import Agda.Primitive using (Level; lzero; lsuc; _⊔_)

-- Universe-polymorphic identity
id : {ℓ : Level} {A : Set ℓ} → A → A
id x = x

-- The type of types at level ℓ
variable ℓ : Level
```

### 1.4 Without K

The `--without-K` pragma disables the K axiom (Uniqueness of Identity Proofs), making Agda suitable for HoTT:

```agda
{-# OPTIONS --without-K --safe #-}
module WithoutK where
```

With this option, `refl` is the only canonical element of an identity type, but J does not force UIP.

---

## 2. Cubical Agda

### 2.1 Enabling Cubical Mode

```agda
{-# OPTIONS --cubical #-}
module CubicalBasics where

open import Cubical.Core.Everything
open import Cubical.Foundations.Prelude
```

With `--cubical`:
- Path types replace propositional equality
- The interval `I` is available as a primitive type
- Composition (`hcomp`) and transport (`transp`) are definitional

### 2.2 The Interval Type

```agda
-- The interval I has two endpoints
i0 i1 : I

-- A path from a to b in A is a function I → A
-- taking i0 to a and i1 to b
_ : {A : Type} {a b : A} → Type
_ = a ≡ b

-- Equivalently:
-- a ≡ b = (i : I) → A  where p i0 = a and p i1 = b (definitionally)
```

### 2.3 Path Types

```agda
-- Reflexivity: the constant path
refl : {A : Type} {a : A} → a ≡ a
refl {a = a} = λ i → a

-- Path induction (J) is derivable, but direct path reasoning is simpler
-- Working with paths as functions:
pathFlip : {A : Type} {a b : A} → a ≡ b → b ≡ a
pathFlip p = λ i → p (~ i)   -- ~ i is the "complement" (1 - i)

pathConcat : {A : Type} {a b c : A} → a ≡ b → b ≡ c → a ≡ c
pathConcat p q i = hcomp (λ j → λ { (i = i0) → p i0
                                   ; (i = i1) → q j }) (p i)
```

**Key difference from MLTT:** In cubical Agda, a path `p : a ≡ b` is literally a function `I → A` with `p i0 = a` and `p i1 = b` (definitionally, not just propositionally). This means:
- You can compute with path endpoints directly: `p i0` reduces to `a`
- No J eliminator needed — just apply the path to the interval

### 2.4 Transport and Composition

```agda
-- Transport along a path
transport : {A B : Type} → A ≡ B → A → B
transport p a = transp (λ i → p i) i0 a

-- Heterogeneous paths (paths over other paths)
-- PathP B p a b : a path from a to b lying over the path p : A₀ ≡ A₁
-- where a : B i0 and b : B i1

-- The composition operation hcomp:
-- hcomp u a : fills a "partial element" u to a full element
-- u : ∀ i → Partial φ A  (a partial box)
-- a : A                   (the bottom face)
```

---

## 3. The Glue Type and Univalence

In Cubical Agda, univalence is provable using the **Glue type**.

### 3.1 The Glue Type

```agda
-- Glue : given a type T and a partial equivalence,
-- construct a new type that "glues" T to other types along the equivalence

-- ua : proves univalence
ua : {A B : Type} → A ≃ B → A ≡ B
ua {A} {B} e i = Glue B (λ { (i = i0) → A , e
                             ; (i = i1) → B , idEquiv B })

-- uaβ : the computation rule for ua
uaβ : {A B : Type} (e : A ≃ B) (x : A) → transport (ua e) x ≡ e .fst x
uaβ e x = transportRefl (e .fst x)
```

### 3.2 Function Extensionality from Paths

In Cubical Agda, function extensionality is immediate from the definition of paths:

```agda
-- funExt: homotopy implies equality of functions
funExt : {A : Type} {B : A → Type}
         {f g : (x : A) → B x}
         → (∀ x → f x ≡ g x)
         → f ≡ g
funExt p i x = p x i
-- p x : f x ≡ g x, so p x i : B x
-- The function λ i x → p x i has type I → ((x : A) → B x)
-- At i0 it's λ x → p x i0 = f x, and at i1 it's λ x → p x i1 = g x
```

This is definitional function extensionality — no axiom needed, it's just rearrangement of function arguments.

---

## 4. Higher Inductive Types in Cubical Agda

```agda
-- The circle
data S¹ : Type where
  base : S¹
  loop : base ≡ base

-- Elimination principle for S¹
S¹-elim : ∀ {ℓ} (B : S¹ → Type ℓ)
           (b : B base)
           (ℓ' : PathP (λ i → B (loop i)) b b)
           → (x : S¹) → B x
S¹-elim B b ℓ' base = b
S¹-elim B b ℓ' (loop i) = ℓ' i
```

### 4.1 The Integers and winding number

```agda
-- The code family for π₁(S¹)
-- In Cubical Agda, we can use ua to define families

helix : S¹ → Type
helix base = ℤ
helix (loop i) = ua sucℤ-equiv i  -- ua of the successor equivalence

-- encode: transport along a path
encode : {x : S¹} → base ≡ x → helix x
encode {x} p = transport (λ i → helix (p i)) (pos zero)

-- The main theorem
ΩS¹≃ℤ : (base ≡ base) ≃ ℤ
ΩS¹≃ℤ = encode , ...  -- (proof that encode is an equivalence)
```

---

## 5. The Cubical Agda Library

The Cubical library (github.com/agda/cubical) contains:

### 5.1 Key Modules

```
Cubical.Core.Everything           -- Core primitives
Cubical.Foundations.Prelude       -- Basic definitions (≡, transport, ...)
Cubical.Foundations.Equiv         -- Equivalences
Cubical.Foundations.Univalence    -- Univalence (ua, uaβ, ...)
Cubical.Foundations.HLevels       -- isProp, isSet, ...
Cubical.Foundations.Path          -- Path operations
Cubical.HITs.Circle               -- The circle S¹
Cubical.HITs.Susp                 -- Suspension
Cubical.HITs.Pushout              -- Pushouts
Cubical.HITs.Truncation           -- Propositional and n-truncations
Cubical.Homotopy.Group.Base       -- Homotopy groups
```

### 5.2 Working with Propositions and Sets

```agda
open import Cubical.Foundations.HLevels

-- isProp: mere propositions
isPropEmpty : isProp ⊥
isPropEmpty ()

-- isSet: h-sets
isSetNat : isSet ℕ
isSetNat = isSet-ℕ  -- from Cubical.Data.Nat

-- Hedberg's theorem (decidable equality implies set)
Discrete→isSet : {A : Type} → Discrete A → isSet A
```

### 5.3 The Propositional Truncation

```agda
open import Cubical.HITs.PropositionalTruncation

-- ∥ A ∥₁ : propositional truncation of A
-- ∣ a ∣₁ : constructor
-- squash₁ : isProp (∥ A ∥₁)

-- Universal property
rec→Set : {A : Type} {B : Type}
          → isSet B
          → (A → B)
          → ∥ A ∥₁ → B
```

---

## 6. Proving Theorems in Cubical Agda

### 6.1 A Complete Example: isSet for ℕ

```agda
{-# OPTIONS --cubical #-}
module NatIsSet where

open import Cubical.Core.Everything
open import Cubical.Foundations.Prelude
open import Cubical.Foundations.HLevels
open import Cubical.Data.Nat

-- We already have isSetNat in the library, but let's reprove it
-- The key lemma: ℕ has decidable equality
discreteNat : Discrete ℕ
discreteNat zero zero = yes refl
discreteNat zero (suc m) = no (λ ())
discreteNat (suc n) zero = no (λ ())
discreteNat (suc n) (suc m) with discreteNat n m
... | yes p = yes (cong suc p)
... | no ¬p = no (λ q → ¬p (injSuc q))

-- Hedberg implies isSet
isSetNat' : isSet ℕ
isSetNat' = Discrete→isSet discreteNat
```

### 6.2 The Hopf Fibration

```agda
-- (Sketch) The Hopf fibration in Cubical Agda
-- Requires S³ and S² as HITs
open import Cubical.HITs.S3
open import Cubical.HITs.S2
open import Cubical.HITs.S1

hopf : S³ → S²
hopf = ...  -- Defined using complex number multiplication

-- The fiber over any point is S¹
hopfFiber : (b : S²) → hopf ⁻¹ b ≡ S¹
```

---

## 7. Projects in Cubical Agda

**Project 1 (Foundational, 1 month):** Work through the Cubical Agda library systematically. Reproduce the key results:
- `π₁(S¹) = ℤ` (the main computation)
- Function extensionality from the path definition
- The Mayer-Vietoris sequence for pushouts

**Project 2 (Intermediate, 2-3 months):** Contribute to the Cubical Agda library:
- Formalize a theorem from the HoTT Book not yet in the library
- Examples: the Seifert-van Kampen theorem, a specific case of Blakers-Massey, or properties of Eilenberg-MacLane spaces

**Project 3 (Advanced, 3+ months):** Formalize a new result in synthetic homotopy theory:
- A new homotopy group computation
- A connectivity result for a new class of HITs
- The Freudenthal suspension theorem (if not yet in the library)

---

## Exercises

**22.1.** In Cubical Agda:
  - Define path concatenation `_∙_` using `hcomp`
  - Prove path inversion `sym`
  - Prove the left unit law `refl ∙ p ≡ p` (this should hold definitionally or with one `refl`)

**22.2.** Show that `funExt` (as defined in this chapter) satisfies the expected computation rule: for any `x : A`, `funExt p i0` reduces to `f` and `funExt p i1` reduces to `g`.

**22.3.** In Cubical Agda, define the circle `S¹` as a HIT and compute:
  - `S¹-elim` for the case where `B` is a constant type
  - The map `windingNumber : (base ≡ base) → ℤ`

**22.4.** Show that in Cubical Agda, `ua` satisfies: `transport (ua e) a ≡ e .fst a` (this should hold by reduction or by a simple path).

**22.5.** Prove in Cubical Agda that `ℤ` is a set (`isSet ℤ`).

**22.6.** The difference between `--without-K` (classical Agda without UIP) and `--cubical` (computational HoTT): give an example of something provable in `--cubical` that is not provable in `--without-K`. (*Hint:* `funExt` is a theorem in cubical mode but not in `--without-K` alone.)

**22.7 (Research).** Read the paper "Cubical Agda: A Dependently Typed Programming Language with Univalence and Higher Inductive Types" (Vezzosi, Mörtberg, Abel, ICFP 2019). Summarize the key ideas in one page, focusing on: how path types differ from identity types, how the composition operation works, and why this gives canonicity.
