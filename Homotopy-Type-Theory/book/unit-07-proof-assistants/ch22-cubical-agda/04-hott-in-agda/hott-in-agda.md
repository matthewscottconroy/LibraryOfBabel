# HoTT in the Cubical Agda Library: A Tour from Foundations to the Frontier

The Cubical Agda library (github.com/agda/cubical) is the computational implementation of HoTT. It contains the foundations (paths, equivalences, univalence), the higher inductive types (circle, spheres, pushouts, truncations), the algebraic structures (groups, abelian groups), and the homotopy-theoretic results (fundamental groups, homotopy groups, Brunerie's theorem). This section tours the library, shows how to use it, and connects it to the mathematical content of this curriculum.

## The Library Structure

```
Cubical/
  Core/              -- Primitives: I, Path, Glue, hcomp, transp
  Foundations/       -- Prelude, HLevels, Equiv, Univalence, IsEquiv
  HITs/              -- S¹, S², Sⁿ, Susp, Pushout, Truncations, Torus
  Data/              -- Nat, Int, Rat, Bool, Fin, List
  Algebra/           -- Groups, AbelianGroups, GroupCohomology
  Homotopy/          -- Homotopy groups, Freudenthal, Brunerie
  CategoryTheory/    -- Categories, Functors, Univalent categories
```

This organization mirrors the mathematical structure: foundations first, then HITs, then data, then algebra, then homotopy theory.

## Cubical.Foundations.Prelude

The starting point for almost every Cubical Agda file:

```agda
{-# OPTIONS --cubical #-}
module MyModule where

open import Cubical.Foundations.Prelude
```

This provides the basic toolkit:

```agda
-- Path operations
_≡_   : {A : Type ℓ} → A → A → Type ℓ
refl  : {A : Type ℓ} {a : A} → a ≡ a
sym   : {A : Type ℓ} {a b : A} → a ≡ b → b ≡ a
_∙_   : {A : Type ℓ} {a b c : A} → a ≡ b → b ≡ c → a ≡ c

-- Transport and substitution
transport : {A B : Type ℓ} → A ≡ B → A → B
subst     : {A : Type ℓ} (B : A → Type ℓ') → {x y : A} → x ≡ y → B x → B y

-- Action on paths
cong  : {A : Type ℓ} {B : Type ℓ'} (f : A → B) → {x y : A} → x ≡ y → f x ≡ f y
cong₂ : (f : A → B → C) → a₁ ≡ a₂ → b₁ ≡ b₂ → f a₁ b₁ ≡ f a₂ b₂

-- Function extensionality (a theorem, not an axiom!)
funExt   : (∀ x → f x ≡ g x) → f ≡ g
funExtDep : (∀ x → PathP (λ i → B x i) (f x) (g x)) → PathP (λ i → ∀ x → B x i) f g
```

## Cubical.Foundations.HLevels

The h-level hierarchy:

```agda
open import Cubical.Foundations.HLevels

-- The predicates
isContr    : Type ℓ → Type ℓ   -- contractible: singleton up to homotopy
isProp     : Type ℓ → Type ℓ   -- proposition: at most one element
isSet      : Type ℓ → Type ℓ   -- set: path spaces are propositions
isGroupoid : Type ℓ → Type ℓ   -- groupoid: path spaces are sets

-- General h-level predicate
isOfHLevel : ℕ → Type ℓ → Type ℓ
-- isOfHLevel 0 A = isContr A
-- isOfHLevel 1 A = isProp A
-- isOfHLevel 2 A = isSet A

-- Truncation
hLevelTrunc : (n : ℕ) → Type ℓ → Type ℓ   -- the n-truncation
```

Working with h-levels:

```agda
-- The hierarchy is cumulative: lower levels imply higher
isContr→isProp : isContr A → isProp A
isProp→isSet   : isProp A → isSet A
isSet→isGroupoid : isSet A → isGroupoid A

-- Products preserve h-levels
isPropΠ : (∀ x → isProp (B x)) → isProp ((x : A) → B x)
isSetΠ  : (∀ x → isSet (B x))  → isSet  ((x : A) → B x)
isPropΣ : isProp A → (∀ x → isProp (B x)) → isProp (Σ A B)
isSetΣ  : isSet A  → (∀ x → isSet (B x))  → isSet  (Σ A B)

-- Key result: isProp is a proposition (self-referential!)
isPropIsProp : isProp (isProp A)
isPropIsProp = isPropIsOfHLevel 1
```

### Showing specific types have specific h-levels

```agda
-- ℕ is a set
isSetNat : isSet ℕ
isSetNat = Discrete→isSet discreteNat
  where
    discreteNat : Discrete ℕ   -- decidable equality
    discreteNat zero    zero    = yes refl
    discreteNat zero    (suc n) = no (λ ())
    discreteNat (suc n) zero    = no (λ ())
    discreteNat (suc m) (suc n) with discreteNat m n
    ... | yes p = yes (cong suc p)
    ... | no ¬p = no (λ q → ¬p (injSuc q))

-- ℤ is a set (similarly via decidable equality)
isSetInt : isSet ℤ
isSetInt = Discrete→isSet discreteInt

-- S¹ is a groupoid (1-type)
-- Its path spaces are sets (specifically, ≡ ℤ at the basepoint)
isGroupoidS¹ : isGroupoid S¹
-- Follows from ΩS¹≃ℤ and isSetInt
```

## Cubical.Foundations.Equiv

The correct HoTT notion of equivalence:

```agda
open import Cubical.Foundations.Equiv

-- Equivalence: contractible fibers
isEquiv : {A : Type ℓ₁} {B : Type ℓ₂} → (A → B) → Type (ℓ₁ ⊔ ℓ₂)
isEquiv f = ∀ b → isContr (fiber f b)
  where
    fiber : {A : Type ℓ₁} {B : Type ℓ₂} → (A → B) → B → Type (ℓ₁ ⊔ ℓ₂)
    fiber f b = Σ[ a ∈ A ] (f a ≡ b)

-- The type of equivalences
_≃_ : (A : Type ℓ₁) (B : Type ℓ₂) → Type (ℓ₁ ⊔ ℓ₂)
A ≃ B = Σ[ f ∈ (A → B) ] isEquiv f

-- Building equivalences from isomorphisms (explicit inverse functions)
record Iso (A : Type ℓ₁) (B : Type ℓ₂) : Type (ℓ₁ ⊔ ℓ₂) where
  constructor iso
  field
    fun      : A → B
    inv      : B → A
    rightInv : ∀ b → fun (inv b) ≡ b
    leftInv  : ∀ a → inv (fun a) ≡ a

isoToEquiv : Iso A B → A ≃ B
-- Converts an explicit-inverse pair to an equivalence with contractible fibers

-- Basic equivalences
idEquiv   : A ≃ A
invEquiv  : A ≃ B → B ≃ A
compEquiv : A ≃ B → B ≃ C → A ≃ C
```

A worked example — the equivalence `A × B ≃ B × A`:

```agda
open import Cubical.Foundations.Equiv

prodComm : (A : Type ℓ₁) (B : Type ℓ₂) → A × B ≃ B × A
prodComm A B = isoToEquiv (iso swap swap swap∘swap swap∘swap)
  where
    swap : A × B → B × A
    swap (a , b) = (b , a)
    swap∘swap : ∀ x → swap (swap x) ≡ x
    swap∘swap (a , b) = refl
```

## Cubical.Foundations.Univalence

The main event:

```agda
open import Cubical.Foundations.Univalence

-- ua: from equivalences to paths in the universe
ua : {A B : Type ℓ} → A ≃ B → A ≡ B

-- The computation rule (this is what makes everything work)
uaβ : {A B : Type ℓ} (e : A ≃ B) (a : A) → transport (ua e) a ≡ e .fst a

-- The full univalence equivalence
univalence : {A B : Type ℓ} → (A ≡ B) ≃ (A ≃ B)
-- This is the bidirectional statement: not only ua : A ≃ B → A ≡ B
-- but also pathToEquiv : A ≡ B → A ≃ B, and these are mutual inverses

-- The inverse: path to equivalence
pathToEquiv : {A B : Type ℓ} → A ≡ B → A ≃ B
pathToEquiv p = isoToEquiv (iso (transport p) (transport (sym p)) ...)
```

## π₁(S¹) = ℤ: The Full Proof

The Cubical library has the complete proof:

```agda
open import Cubical.HITs.Circle
open import Cubical.Homotopy.Group.Base

-- The winding number
winding : base ≡ base → ℤ
winding = encode base

-- The loop power
loopn : ℤ → base ≡ base
loopn = decode base

-- The isomorphism
ΩS¹Isoℤ : Iso (base ≡ base) ℤ
ΩS¹Isoℤ = iso winding loopn encode-decode decode-encode'

-- The equivalence
ΩS¹≃ℤ : (base ≡ base) ≃ ℤ
ΩS¹≃ℤ = isoToEquiv ΩS¹Isoℤ

-- The group isomorphism (fundamental group level)
π₁S¹≅ℤ : GroupEquiv (π₁ S¹ base) ℤGroup
-- This requires showing that winding is a group homomorphism:
-- winding (p ∙ q) = winding p + winding q
-- which follows from the definition of encode and the uaβ computation rule
```

## Homotopy Groups

The library has a systematic development of homotopy groups:

```agda
open import Cubical.Homotopy.Group.Base

-- The n-th homotopy group of X at x
π : (n : ℕ) (X : Pointed ℓ) → Group ℓ
-- Pointed X = Σ[ A ∈ Type ℓ ] A  (a type with a distinguished point)

-- π₁(S¹) = ℤ
π₁S¹≅ℤ : GroupEquiv (π 1 (S¹ , base)) ℤGroup

-- π₂(S²) = ℤ
π₂S²≅ℤ : GroupEquiv (π 2 (S² , north)) ℤGroup
-- S² is defined as Susp S¹ in the library

-- πₙ(Sⁿ) = ℤ for all n ≥ 1
πₙSⁿ≅ℤ : (n : ℕ) → GroupEquiv (π (suc n) (Sⁿ (suc n) , north)) ℤGroup
-- Uses Freudenthal suspension theorem
```

## Brunerie's Theorem: π₄(S³) = ℤ/2ℤ

The most spectacular computation in the Cubical library:

```agda
open import Cubical.Homotopy.Brunerie

-- The Brunerie number (defined by abstract HoTT constructions)
brunerie : ℤ

-- The theorem
π₄S³ : GroupEquiv (π 4 (S³ , north)) (ℤGroup/ brunerie)
-- where ℤGroup/ n = ℤ/nℤ as an additive group

-- The computation: brunerie computes to 2
-- brunerieNorm : brunerie ≡ 2
-- (After the 2022 Ljungström-Mörtberg optimizations, this normalizes in seconds)
```

The `brunerie` number is not defined to be 2. It is defined through a chain of abstract constructions:
1. The Hopf fibration: a map `S³ → S²` with fiber `S¹`
2. The Hopf invariant: a cohomological invariant of this map
3. A group-theoretic calculation using the long exact sequence of the Hopf fibration

The fact that this chain of abstract definitions computes to `2` is a non-trivial mathematical fact — and it is verified by Agda's normalizer. This is mathematics and computation unified: the proof that $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ is a running program that can be checked in seconds.

## The Practical Workflow

Here is how to work with the Cubical library in practice:

```agda
{-# OPTIONS --cubical #-}
module MyHoTTProject where

-- Import what you need from the library
open import Cubical.Core.Everything
open import Cubical.Foundations.Prelude
open import Cubical.Foundations.HLevels
open import Cubical.Foundations.Equiv
open import Cubical.Foundations.Univalence
open import Cubical.HITs.Circle
open import Cubical.Data.Int

-- State your theorem
myTheorem : isGroupoid S¹
myTheorem = -- use the library's result
  isOfHLevelRespEquiv 3 (invEquiv ΩS¹≃ℤ)
    (isOfHLevel 3 ℤ ...)  -- ℤ is a set, so it's also a groupoid

-- Or prove something new
-- For example: the fundamental group of the torus is ℤ × ℤ
open import Cubical.HITs.Torus

π₁T²≅ℤ×ℤ : GroupEquiv (π 1 (T² , point)) (ℤGroup ×Group ℤGroup)
π₁T²≅ℤ×ℤ = -- encode-decode argument for T²
```

The interactive development loop:
1. Write a type signature with a hole `?` or `{! !}`
2. Load the file (`C-c C-l`)
3. Check what type you need to fill (`C-c C-,`)
4. Search the library for relevant results (`C-c C-a` or manual `#check`)
5. Fill the hole, possibly introducing more holes
6. Normalize to verify computations (`C-c C-n`)

## What "Computes" Means

In the Cubical library, "computes" has a precise meaning: a term *computes* if Agda's normalizer can reduce it to a canonical form without getting stuck on axioms.

In axiomatic HoTT, `transport (ua e) a` is stuck — no computation rule.
In Cubical Agda, `transport (ua e) a` computes to `e .fst a` — a genuine reduction.

This means:
- You can use `C-c C-n` to evaluate expressions and check answers
- Proofs by `refl` work for things that are equal by computation (not just propositionally)
- Programs extracted from proofs actually run
- The Brunerie number literally normalizes to `2`

**The big picture:** Cubical Agda is the computational realization of the HoTT Book's vision. The HoTT Book said: propositions are types, proofs are programs, and the homotopy theory of types should be taken seriously. Cubical Agda makes this literal. Every HoTT proof in Cubical Agda is a running program. Every theorem has computational content. The abstract and the concrete are unified.

This is not just a philosophical position. It is a mathematical achievement with practical consequences: when you formalize $\pi_1(S^1) = \mathbb{Z}$ in Cubical Agda, you get a working implementation of the winding number. When you formalize Brunerie's theorem, you get a working computation of the Brunerie number. The formalization IS the implementation.

For researchers in automated theorem proving, this opens a new direction: use the computational content of HoTT proofs as a source of algorithms. The proof of $\pi_n(S^n) = \mathbb{Z}$ is also a program for computing the degree of a map $S^n \to S^n$. The proof of the Freudenthal suspension theorem is a program for computing stable homotopy groups. Mathematics and computation, converging.
