# 4.1 HoTT in the Cubical Agda Library

## The Cubical Library

The Cubical Agda library (github.com/agda/cubical) is the primary library for HoTT formalization. It contains:

- **Foundations:** path types, h-levels, equivalences, univalence
- **Data:** natural numbers, integers, rationals, finite sets
- **Higher Inductive Types:** circle, suspension, spheres, pushouts, truncations
- **Homotopy theory:** fundamental groups, homotopy groups, Eilenberg-MacLane spaces
- **Algebra:** groups, abelian groups, group cohomology
- **Category theory:** categories, functors, adjunctions (in progress)

Unlike Mathlib (which is vast and covers classical mathematics), the Cubical library is focused on *HoTT-specific* content — the things that require the full power of cubical type theory.

Let's tour the key modules.

## Module: `Cubical.Foundations.Prelude`

The starting point for almost everything:

```agda
open import Cubical.Foundations.Prelude
```

This provides:

```agda
-- Path types
_≡_ : {A : Type ℓ} → A → A → Type ℓ

-- Basic operations
refl  : a ≡ a
sym   : a ≡ b → b ≡ a
_∙_   : a ≡ b → b ≡ c → a ≡ c

-- Transport
transport : A ≡ B → A → B
subst     : (B : A → Type ℓ) → x ≡ y → B x → B y

-- Action on paths (ap)
cong  : (f : A → B) → x ≡ y → f x ≡ f y
cong₂ : (f : A → B → C) → x ≡ y → u ≡ v → f x u ≡ f y v

-- Function extensionality
funExt   : (∀ x → f x ≡ g x) → f ≡ g
funExtDep : (∀ x → PathP (λ i → B x i) (f x) (g x)) → PathP (λ i → ∀ x → B x i) f g
```

## Module: `Cubical.Foundations.HLevels`

The h-level hierarchy:

```agda
open import Cubical.Foundations.HLevels

-- The predicates
isContr   : Type ℓ → Type ℓ   -- contractible
isProp    : Type ℓ → Type ℓ   -- proposition (h-level -1)
isSet     : Type ℓ → Type ℓ   -- set (h-level 0)
isGroupoid : Type ℓ → Type ℓ  -- groupoid (h-level 1)
is2Groupoid : Type ℓ → Type ℓ -- 2-groupoid (h-level 2)

-- General n-type predicate (using HLevel as a Nat-indexed family)
isOfHLevel : ℕ → Type ℓ → Type ℓ

-- The n-truncated type
hLevelTrunc : (n : ℕ) (A : Type ℓ) → Type ℓ
```

Working with these predicates:

```agda
-- Propositions: any two elements are equal
isProp→isSet : isProp A → isSet A

-- The h-level hierarchy is cumulative
isContr→isProp : isContr A → isProp A
isProp→isSet   : isProp A → isSet A

-- Products preserve h-levels
isProp× : isProp A → isProp B → isProp (A × B)
isSet×  : isSet A → isSet B → isSet (A × B)

-- Π-types preserve h-levels
isPropΠ : (∀ x → isProp (B x)) → isProp ((x : A) → B x)
isSetΠ  : (∀ x → isSet (B x)) → isSet ((x : A) → B x)

-- The type of propositions is a set
isSetHProp : isSet (Σ Type isProp)
```

## Module: `Cubical.Foundations.Equiv`

Equivalences, the correct HoTT notion of "isomorphism between types":

```agda
open import Cubical.Foundations.Equiv

-- Equivalence: contractible fibers
isEquiv : (A → B) → Type _
isEquiv f = ∀ b → isContr (fiber f b)
  where fiber f b = Σ[ a ∈ A ] (f a ≡ b)

-- The type of equivalences
_≃_ : Type ℓ → Type ℓ → Type _
A ≃ B = Σ[ f ∈ (A → B) ] isEquiv f

-- Constructing equivalences from isomorphisms
-- (when you have explicit inverse functions)
isoToEquiv : Iso A B → A ≃ B

-- The Iso type (quasi-inverse)
record Iso (A B : Type ℓ) : Type ℓ where
  constructor iso
  field
    fun    : A → B
    inv    : B → A
    rightInv : ∀ b → fun (inv b) ≡ b
    leftInv  : ∀ a → inv (fun a) ≡ a

-- Basic equivalences
idEquiv   : A ≃ A
invEquiv  : A ≃ B → B ≃ A
compEquiv : A ≃ B → B ≃ C → A ≃ C
```

The key lemma connecting `Iso` to `isEquiv`:

```agda
-- An isomorphism is an equivalence
-- (the "biInvertible implies contractible fibers" direction)
isoToIsEquiv : Iso A B → isEquiv (Iso.fun e)
-- Proof: for any b : B, the fiber fun⁻¹(b) is contractible
-- Center: (inv b, rightInv b)
-- Contraction: use leftInv and rightInv for the path
```

## Module: `Cubical.Foundations.Univalence`

The main event:

```agda
open import Cubical.Foundations.Univalence

-- ua: the function from equivalences to paths
ua : A ≃ B → A ≡ B

-- Computation rule
uaβ : (e : A ≃ B) (a : A) → transport (ua e) a ≡ e .fst a

-- Equivalence
univalence : (A ≡ B) ≃ (A ≃ B)
univalence = isoToEquiv (iso equivToPath pathToEquiv ... ...)

-- The inverse: pathToEquiv
pathToEquiv : A ≡ B → A ≃ B
pathToEquiv p = isoToEquiv (iso (transport p) (transport (sym p)) ...)
```

Note: `univalence` is not just a theorem but a proof of *equivalence*. The path `ua e : A ≡ B` and the equivalence `e : A ≃ B` are in bijective correspondence, and the bijection is an equivalence itself. This is the full univalence axiom: `(A ≡ B) ≃ (A ≃ B)`.

## Module: `Cubical.HITs.Circle`

```agda
open import Cubical.HITs.Circle

-- The circle
data S¹ : Type where
  base : S¹
  loop : base ≡ base

-- The code family and winding number
winding : base ≡ base → ℤ
winding = encode base

-- The main equivalence
ΩS¹Isoℤ : Iso (base ≡ base) ℤ
ΩS¹Isoℤ = iso winding (loopn) encode-decode decode-encode

-- The first homotopy group
π₁S¹ : GroupEquiv (fundamentalGroup S¹ base) ℤGroup
```

The last line requires the `Cubical.Homotopy.Group.Base` module:

```agda
open import Cubical.Homotopy.Group.Base

-- The fundamental group: paths up to homotopy
fundamentalGroup : Type ℓ → Type ℓ → Group ℓ
fundamentalGroup X x = loopGroup X x

-- π₁(S¹) ≅ ℤ as groups (not just as types!)
π₁S¹≅ℤ : GroupEquiv (fundamentalGroup S¹ base) ℤGroup
-- This proves that the group structure on (base ≡ base) corresponds to
-- addition in ℤ under the winding number isomorphism
```

## Working with H-Levels

One of the most important operations in the Cubical library: proving that specific types are sets, propositions, or groupoids.

### Showing ℕ is a set

```agda
open import Cubical.Data.Nat
open import Cubical.Foundations.HLevels

-- ℕ has decidable equality
discreteNat : Discrete ℕ
discreteNat zero zero = yes refl
discreteNat zero (suc n) = no (λ ())
discreteNat (suc n) zero = no (λ ())
discreteNat (suc m) (suc n) with discreteNat m n
... | yes p = yes (cong suc p)
... | no ¬p = no (λ q → ¬p (injSuc q))

-- Hedberg's theorem: discrete types are sets
isSetNat : isSet ℕ
isSetNat = Discrete→isSet discreteNat
```

### Showing ℤ is a set

```agda
open import Cubical.Data.Int

isSetInt : isSet ℤ
isSetInt = Discrete→isSet discreteInt
  -- discreteInt : Discrete ℤ (decidable equality on integers)
```

### The universe of propositions is a set

```agda
-- hProp ℓ : the type of propositions at universe level ℓ
hProp : (ℓ : Level) → Type (lsuc ℓ)
hProp ℓ = Σ (Type ℓ) isProp

-- hProp is a set
isSetHProp : isSet (hProp ℓ)
```

This is a non-trivial result: it requires propositional extensionality (which follows from univalence) to prove that any two elements of `hProp ℓ` are equal iff the underlying propositions are logically equivalent.

## A Complete Proof: `isProp` is a Proposition

Let's prove that `isProp A` is itself a proposition — it has at most one proof:

```agda
-- If A is a proposition, then isProp A is contractible (there's only one proof)
isPropIsProp : isProp (isProp A)
isPropIsProp p q = funExt₂ (λ x y → squash p q x y)
  -- p q : ∀ x y : A, x ≡ y
  -- We need: p ≡ q as functions A → A → Path
  -- Pointwise: for each x y, we need p x y ≡ q x y
  -- But both p x y and q x y are paths in A
  -- And p : isProp A, so p x (p x y i) = p x y (using that A is a prop)
  -- ...actually, we use the fact that a prop's path space is also contractible
  where
    squash : isProp A → isProp A → ∀ x y → ∀ (r s : x ≡ y) → r ≡ s
    squash prop _ x y = isContr→isProp (prop x y)
    -- prop x y : isContr (x ≡ y)... wait, isContr or isProp?
    -- If A is a prop, then for any x y : A, x ≡ y is *contractible*
    -- (there's exactly one path between any two elements)
```

The actual proof in the Cubical library uses `isPropIsOfHLevel`:

```agda
isPropIsProp : isProp (isProp A)
isPropIsProp = isPropIsOfHLevel 1
-- isPropIsOfHLevel n : isProp (isOfHLevel n A)
-- for n = 1, this says isProp (isProp A), which is exactly what we want
```

## The h-Level Hierarchy in Action

Let's see h-levels at work in a non-trivial example: showing that the loop space of a 1-type is a set.

```agda
-- If A is a groupoid (isGroupoid A = isOfHLevel 2 A),
-- then (a ≡ b) is a set for any a b : A
isGroupoid→PathIsSet : isGroupoid A → (a b : A) → isSet (a ≡ b)
isGroupoid→PathIsSet hA a b = hA a b
-- isGroupoid A = ∀ x y : A, isSet (x ≡ y)
-- So hA a b : isSet (a ≡ b) directly!
```

And the converse direction — showing that `S¹` is a 1-type (groupoid):

```agda
open import Cubical.HITs.Circle
open import Cubical.Foundations.HLevels

-- S¹ is a groupoid (1-type)
isGroupoidS¹ : isGroupoid S¹
-- Proof: we know π₁(S¹) = ℤ
-- Every path space base ≡ base ≃ ℤ, which is a set
-- So S¹ satisfies isGroupoid (the path spaces are sets)
-- This is derived from the ΩS¹≃ℤ equivalence
```

## Homotopy Groups

The Cubical library has a developing theory of homotopy groups:

```agda
open import Cubical.Homotopy.Group.Base

-- The n-th homotopy group of X at basepoint x
πₙ : (n : ℕ) (X : Type ℓ) (x : X) → Group ℓ

-- π₁ is the fundamental group
π₁ : Type ℓ → Type ℓ → Group ℓ
π₁ X x = πₙ 1 X x

-- π₁(S¹) = ℤ
π₁S¹ : GroupEquiv (π₁ S¹ base) ℤGroup
```

For higher homotopy groups, the story is more complex. As of 2025, the Cubical library has:
- `π₁(S¹) = ℤ` (fully formalized)
- `π₂(S²) = ℤ` (formalized)
- `πₙ(Sⁿ) = ℤ` for all `n` (formalized using Freudenthal)
- `π₃(S²) = ℤ` (formalized using Hopf fibration)
- `π₄(S³) = ℤ/2ℤ` (Brunerie's theorem, formalized with computable verification of the Brunerie number)

## Brunerie's Theorem in Cubical Agda

The computation `π₄(S³) = ℤ/2ℤ` is one of the great achievements of the Cubical Agda project. Let's look at how the computation is set up:

```agda
open import Cubical.Homotopy.Brunerie

-- The Brunerie number: an integer defined by the proof
-- of π₄(S³) = ℤ/nℤ
brunerie : ℤ

-- The theorem
π₄S³ : GroupEquiv (π₄ S³ north) (ℤ/ brunerie)

-- The computation (this is what required 2022 optimization work)
-- brunerieComputes : brunerie ≡ 2
-- (In the original 2016 proof, this took hours to normalize)
-- In the 2022 Ljungström-Mörtberg version, it normalizes quickly
```

The remarkable thing: `brunerie` is not defined to be `2`. It's the result of a chain of abstract constructions (join, Hopf construction, cohomology ring). The fact that it *computes* to `2` is the verification that the proof is correct and that the result matches the classically known value.

## Comparing the Libraries

| Feature | Cubical Agda | Lean 4 / Mathlib |
|---------|-------------|-----------------|
| `π₁(S¹) = ℤ` | Full proof with computation | Not available |
| `π₄(S³) = ℤ/2ℤ` | Computable proof | Not available |
| Univalence | Theorem with computation rule | Axiom (propext+funext) |
| HITs | First-class | Not available |
| Classical algebra | Limited | Extensive (Mathlib) |
| Analysis | Limited | Extensive (Mathlib) |
| Category theory | Developing | Extensive (Mathlib) |

## Using the Cubical Library in Practice

A typical workflow for a HoTT formalization project in Cubical Agda:

```agda
{-# OPTIONS --cubical #-}
module MyHoTTProject where

-- Import what you need
open import Cubical.Core.Everything
open import Cubical.Foundations.Prelude
open import Cubical.Foundations.HLevels
open import Cubical.Foundations.Equiv
open import Cubical.Foundations.Univalence
open import Cubical.HITs.Circle
open import Cubical.Data.Int

-- Your definitions
myType : Type
myType = ...

-- Your proofs
myTheorem : isProp myType
myTheorem = ...
```

The interactive development workflow:
1. Define the types you need.
2. State the theorem.
3. Use holes `?` to explore what needs to be proved.
4. Use `C-c C-,` (in Emacs) to see the current goal and context.
5. Fill in proofs step by step, using library lemmas.
6. Run a normalization check (`C-c C-n`) to verify computations.

## Research Frontiers in Cubical Agda

The most active areas of formalization research using Cubical Agda (as of 2025):

**1. Stable homotopy theory.** The stable homotopy groups $\pi_k^s = \pi_{n+k}(S^n)$ for large $n$ form the *stable stems*. Computing these synthetically in Cubical Agda is an open research problem.

**2. Cohomology theories.** The Cubical library has cohomology via Eilenberg-MacLane spaces. Extending this to generalized cohomology theories (K-theory, cobordism) is ongoing.

**3. Univalent foundations for mathematics.** Using univalence to formalize parts of mathematics where structure matters up to isomorphism (category theory, algebraic geometry). The *univalent categories* approach of Ahrens-Kapulkin-Shulman is being extended.

**4. Synthetic algebraic geometry.** In the `--cohesion` mode, Agda can express *cohesive* type theory, which includes the "infinitesimal" disc type. This is the setting for formalization of algebraic geometry via the Clausen-Scholze condensed mathematics approach.

**5. Type-theoretic semantics.** Using Cubical Agda to formalize the *semantics* of type theories themselves — a deeply meta application.

The field is young. Many of the computations in classical algebraic topology that are still out of reach for synthetic proof assistants will be within reach in the next 5-10 years, as the tools mature and the libraries grow.

## Projects in Cubical Agda

The following projects form a ladder from guided study to original research:

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

## Summary

Cubical Agda provides:
- A computational implementation of HoTT
- First-class HITs with computation rules
- Univalence as a theorem (not an axiom)
- A growing library of formalized mathematics
- The ability to run synthetic homotopy theory computations

Together with Lean 4 and Mathlib for classical mathematics, Cubical Agda gives a complete toolkit for the formalization of the mathematical content in this book — from the algebraic and topological background to the synthetic homotopy groups of spheres.

The HoTT Book presented a vision: that homotopy type theory could be a foundation for mathematics where classical reasoning and homotopy theory are unified. Cubical Agda is the computational realization of that vision.
