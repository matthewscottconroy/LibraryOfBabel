# Exercises: Cubical Agda

*All exercises should be completed in Cubical Agda with the `--cubical` pragma. The Cubical library is required; install via `cabal install Agda` and download the Cubical library from `github.com/agda/cubical`. For exercises marked [Prove in Cubical Agda], write actual Agda code that type-checks.*

---

## Section 1: Agda Basics

**Exercise 1.1 [Prove in Cubical Agda]** Prove the following in Agda (without `--cubical`; standard Agda suffices):

```agda
{-# OPTIONS --without-K #-}
module Ex1 where

open import Data.Nat
open import Relation.Binary.PropositionalEquality

-- 1. Successor is injective
suc-injective : ∀ {n m : ℕ} → suc n ≡ suc m → n ≡ m
suc-injective refl = refl

-- 2. Zero is not a successor
zero≢suc : ∀ {n : ℕ} → zero ≢ suc n
zero≢suc ()

-- 3. Addition is right-zero
add-zero : ∀ (n : ℕ) → n + zero ≡ n
add-zero zero    = refl
add-zero (suc n) = cong suc (add-zero n)
```

**Exercise 1.2 [Prove in Cubical Agda]** Define and prove properties of a binary tree in Agda:

```agda
module Trees where

data Tree (A : Set) : Set where
  leaf : Tree A
  node : Tree A → A → Tree A → Tree A

-- Define:
size   : Tree A → ℕ
mirror : Tree A → Tree A
depth  : Tree A → ℕ

-- Prove:
mirror-mirror : ∀ (t : Tree A) → mirror (mirror t) ≡ t
size-mirror   : ∀ (t : Tree A) → size (mirror t) ≡ size t
depth-mirror  : ∀ (t : Tree A) → depth (mirror t) ≡ depth t
```

**Exercise 1.3** Explain why the following Agda program type-checks under `--without-K` but would be invalid if K were available in unrestricted form:

```agda
{-# OPTIONS --without-K #-}
-- The J eliminator, restricted version
-- (This should type-check)
J-restricted : {A : Set} {a : A}
               (P : ∀ b → a ≡ b → Set)
               → P a refl
               → ∀ b → (p : a ≡ b) → P b p
J-restricted P pr b refl = pr
```

What is the K axiom? Why does `--without-K` not prevent writing `J-restricted` but does prevent a particular form of "K pattern matching"?

**Exercise 1.4 [Prove in Cubical Agda]** Universe polymorphism. Fill in the definitions:

```agda
module Universes where

open import Agda.Primitive

-- Universe-polymorphic composition
_∘_ : {ℓ₁ ℓ₂ ℓ₃ : Level} {A : Set ℓ₁} {B : Set ℓ₂} {C : Set ℓ₃}
      → (B → C) → (A → B) → (A → C)
(f ∘ g) x = ?

-- Universe-polymorphic uncurry
uncurry : {ℓ₁ ℓ₂ ℓ₃ : Level} {A : Set ℓ₁} {B : A → Set ℓ₂} {C : Set ℓ₃}
          → (∀ a → B a → C) → Σ A B → C
uncurry f (a , b) = ?

-- Universe-polymorphic flip
flip : {ℓ₁ ℓ₂ ℓ₃ : Level} {A : Set ℓ₁} {B : Set ℓ₂} {C : A → B → Set ℓ₃}
       → (∀ a b → C a b) → ∀ b a → C a b
flip f b a = ?
```

**Exercise 1.5** Explain the interactive Agda workflow. What does each of these do?
- `C-c C-l` (load file)
- `C-c C-,` (show goal)
- `C-c C-c` (case split)
- `C-c C-a` (auto)
- `C-c C-n` (normalize)

When would you use each? Describe a proof you would write using case splitting.

---

## Section 2: Cubical Mode

**Exercise 2.1 [Prove in Cubical Agda]** In Cubical Agda, prove:

```agda
{-# OPTIONS --cubical #-}
module CubicalEx where

open import Cubical.Foundations.Prelude

-- 1. Path inversion is an involution
sym-sym : {A : Type} {a b : A} (p : a ≡ b) → sym (sym p) ≡ p
sym-sym p = ?
-- Hint: sym (sym p) = λ i → (λ i → p (~ i)) (~ i) = λ i → p (~ (~ i)) = λ i → p i = p

-- 2. Function extensionality (prove it from scratch, not from the library)
myFunExt : {A : Type} {B : A → Type} {f g : (x : A) → B x}
           → (∀ x → f x ≡ g x) → f ≡ g
myFunExt h = ?
-- Hint: use λ i x → h x i

-- 3. Transport along refl is the identity
transport-refl : {A : Type} (a : A) → transport (refl {x = A}) a ≡ a
transport-refl a = ?
```

**Exercise 2.2 [Prove in Cubical Agda]** Prove the following path algebra facts:

```agda
open import Cubical.Foundations.Prelude

-- 1. Left unit law for path concatenation
lUnit : {A : Type} {a b : A} (p : a ≡ b) → (refl ∙ p) ≡ p
lUnit p i j = ?
-- Hint: use p (i ∧ j)

-- 2. Right unit law
rUnit : {A : Type} {a b : A} (p : a ≡ b) → (p ∙ refl) ≡ p
rUnit p i j = ?
-- Hint: use p (i ∨ j)

-- 3. Left inverse law: sym p ∙ p ≡ refl
-- (This is harder; uses hcomp)
```

**Exercise 2.3 [Prove in Cubical Agda]** Construct a non-trivial path in the universe:

```agda
open import Cubical.Data.Bool
open import Cubical.Foundations.Univalence

-- The swap equivalence
swapEquiv : Bool ≃ Bool
swapEquiv = isoToEquiv (iso not not notnot notnot)
  where notnot : ∀ b → not (not b) ≡ b
        notnot = ?

-- The path in the universe
swapPath : Bool ≡ Bool
swapPath = ua swapEquiv

-- Verify that transport works correctly
swapTest₁ : transport swapPath true ≡ false
swapTest₁ = ?  -- Should be uaβ swapEquiv true

swapTest₂ : transport swapPath false ≡ true
swapTest₂ = ?
```

**Exercise 2.4** Explain in your own words, with examples from Cubical Agda:

1. Why does function extensionality hold in Cubical Agda without being an axiom?
2. What is the `Glue` type, and why is it the key to proving univalence?
3. What is the computation rule `uaβ`, and why is it significant?
4. What is the difference between `_≡_` and `PathP`?

**Exercise 2.5 [Prove in Cubical Agda]** Show that transport along `sym p` is the inverse of transport along `p`:

```agda
transport-sym : {A B : Type} (p : A ≡ B) (b : B)
                → transport p (transport (sym p) b) ≡ b
transport-sym p b = ?
-- Hint: this is transport-fillerExt or similar in the library
```

---

## Section 3: HITs in Cubical Agda

**Exercise 3.1 [Prove in Cubical Agda]** The circle and its elimination principle:

```agda
open import Cubical.HITs.Circle

-- 1. Define the "winding twice" map
double : S¹ → S¹
double = S¹-rec base ?   -- what path should loop map to?

-- 2. Define the "negation" map (reverses the winding direction)
neg : S¹ → S¹
neg = S¹-rec base ?

-- 3. Prove that double (neg x) is homotopic to x on the basepoint
double-neg-base : double (neg base) ≡ base
double-neg-base = refl   -- check this is correct

-- 4. What is (double ∘ neg) (loop i)?
-- Write this as a path and check it computes correctly
```

**Exercise 3.2 [Prove in Cubical Agda]** Compute winding numbers:

```agda
open import Cubical.HITs.Circle
open import Cubical.Data.Int

-- These should all hold definitionally or by refl
winding-loop : encode base loop ≡ pos 1
winding-loop = ?

winding-sym-loop : encode base (sym loop) ≡ negsuc 0
winding-sym-loop = ?

winding-loop-loop : encode base (loop ∙ loop) ≡ pos 2
winding-loop-loop = ?

-- And the decoder:
decode-pos-1 : loopn (pos 1) ≡ loop
decode-pos-1 = ?

decode-neg-1 : loopn (negsuc 0) ≡ sym loop
decode-neg-1 = ?
```

**Exercise 3.3 [Prove in Cubical Agda]** Define and work with the suspension:

```agda
open import Cubical.HITs.Suspension

-- 1. Susp Bool → S¹ (from the chapter)
SuspBool→S¹ : Susp Bool → S¹
SuspBool→S¹ north         = ?
SuspBool→S¹ south         = ?
SuspBool→S¹ (merid b i)   = ?

-- 2. S¹ → Susp Bool
S¹→SuspBool : S¹ → Susp Bool
S¹→SuspBool base      = ?
S¹→SuspBool (loop i)  = ?

-- 3. [Harder] Prove one direction of the round-trip:
-- SuspBool→S¹ (S¹→SuspBool x) ≡ x for x : S¹
-- (Prove at least for x = base)
```

**Exercise 3.4 [Prove in Cubical Agda]** Propositional truncation:

```agda
open import Cubical.HITs.PropositionalTruncation

-- 1. Define the map out of ∥ A ∥₁ into a proposition
rec : {A : Type} {P : Type} → isProp P → (A → P) → ∥ A ∥₁ → P
rec propP f ∣ a ∣₁            = ?
rec propP f (squash₁ x y i)   = ?

-- 2. Prove: ∥ A ∥₁ is a proposition
isProp∥∥ : {A : Type} → isProp ∥ A ∥₁
isProp∥∥ x y = squash₁ x y

-- 3. Prove: if A is a proposition, then ∥ A ∥₁ ≃ A
isProp→truncEquiv : {A : Type} → isProp A → ∥ A ∥₁ ≃ A
isProp→truncEquiv propA = ?
```

**Exercise 3.5 [Prove in Cubical Agda]** The pushout:

```agda
open import Cubical.HITs.Pushout

-- Define the pushout of the two projections from A × B to A and B
-- This gives the join of A and B (the cofiber of the diagonal A × B → A × B × A × B)
-- Actually: the join is the pushout of A ← A × B → B where f = fst, g = snd

Join : Type → Type → Type
Join A B = Pushout {A = A} {B = B} {C = A × B} fst snd

-- Prove: Join Bool Bool ≃ S¹
-- (Harder exercise; requires constructing explicit maps and showing they're inverses)
-- At minimum, construct the map Join Bool Bool → S¹
JoinBoolBool→S¹ : Join Bool Bool → S¹
JoinBoolBool→S¹ (inl true)     = ?
JoinBoolBool→S¹ (inl false)    = ?
JoinBoolBool→S¹ (inr true)     = ?
JoinBoolBool→S¹ (inr false)    = ?
JoinBoolBool→S¹ (push (b₁ , b₂) i) = ?
```

---

## Section 4: HoTT in Agda

**Exercise 4.1 [Prove in Cubical Agda]** H-levels:

```agda
open import Cubical.Foundations.HLevels
open import Cubical.Data.Bool
open import Cubical.Data.Nat

-- 1. ℕ is a set (show by providing decidable equality)
isSetNat : isSet ℕ
isSetNat = Discrete→isSet ?

-- 2. Bool is a set
isSetBool : isSet Bool
isSetBool = ?

-- 3. A proposition is a set
isProp→isSet' : {A : Type} → isProp A → isSet A
isProp→isSet' propA x y p q = ?
-- Hint: propA x y is a path, and propA x (propA x y i0) is... use props of propA

-- 4. Σ-types of sets are sets
isSetΣ' : {A : Type} {B : A → Type}
           → isSet A → (∀ a → isSet B a) → isSet (Σ A B)
isSetΣ' setA setB = ?
```

**Exercise 4.2 [Prove in Cubical Agda]** Equivalences:

```agda
open import Cubical.Foundations.Equiv

-- 1. The identity is an equivalence
idIsEquiv : (A : Type) → isEquiv (λ (x : A) → x)
idIsEquiv A b = ((b , refl) , λ { (a , p) → ? })

-- 2. Compose two equivalences
compEquiv' : {A B C : Type} → A ≃ B → B ≃ C → A ≃ C
compEquiv' (f , fe) (g , ge) = ?

-- 3. Equivalence is symmetric: A ≃ B → B ≃ A
invEquiv' : {A B : Type} → A ≃ B → B ≃ A
invEquiv' e = ?
-- Hint: the inverse function is the fiber projections from isEquiv
```

**Exercise 4.3 [Prove in Cubical Agda]** Univalence in action:

```agda
open import Cubical.Foundations.Univalence
open import Cubical.Data.Bool

-- 1. There are exactly 2 automorphisms of Bool (up to ≡)
-- Prove that ua idEquiv ≡ refl
ua-idEquiv : ua (idEquiv {A = Bool}) ≡ refl
ua-idEquiv = ?
-- Hint: ua (idEquiv) should be refl in Cubical Agda (or use uaIdEquiv from the library)

-- 2. The two paths Bool ≡ Bool are ua idEquiv and ua swapEquiv
-- (State this as: any e : Bool ≃ Bool is either idEquiv or swapEquiv)
-- This requires proving Bool has exactly 2 automorphisms; state the lemma:
boolAut : (e : Bool ≃ Bool) → (e ≡ idEquiv) ⊎ (e ≡ swapEquiv)
-- (Proving this is harder; just state and sketch the argument)
boolAut e = ?
```

**Exercise 4.4 [Research project]** State and (partially) prove the following in Cubical Agda:

The Seifert-van Kampen theorem, in its HoTT form, says: if $X = A \cup_C B$ (a pushout of spaces), then $\pi_1(X, x)$ is the pushout of groups $\pi_1(A, x) *_{\pi_1(C, x)} \pi_1(B, x)$.

In Cubical Agda:

```agda
open import Cubical.HITs.Pushout
open import Cubical.HITs.Circle
open import Cubical.Homotopy.Group.Base

-- The pushout of two copies of the interval at their endpoints gives S¹
-- (The interval [0,1] is contractible; gluing two copies at endpoints gives the circle)

-- State the van Kampen theorem for the circle as a pushout:
-- S¹ ≃ Pushout (pt → I) (pt → I)  where pt is a point and the maps pick endpoints
-- (or some equivalent formulation)

-- Then conclude π₁(S¹) from π₁(I) = trivial and the van Kampen theorem
```

Write down the Agda types involved, even if you cannot complete the full proof.

**Exercise 4.5 [Computational verification]** Compute using Cubical Agda's normalizer:

```agda
open import Cubical.HITs.Circle
open import Cubical.Data.Int

-- 1. Use C-c C-n to normalize these expressions:
-- encode base (loop ∙ loop ∙ loop)
-- encode base (sym loop ∙ sym loop)
-- encode base refl
-- loopn (pos 4)   -- what path does this produce?

-- 2. Verify the group homomorphism property:
-- encode base (p ∙ q) ≡ encode base p + encode base q
-- Prove this for specific small cases
winding-hom : ∀ (n m : ℤ) →
    encode base (loopn n ∙ loopn m) ≡ n + m
winding-hom n m = ?
-- Hint: this should follow from encode-decode and the definition of loopn
```

---

## Section 5: Connections and Contrasts

**Exercise 5.1** Compare the treatment of path inversion in standard Agda (`--without-K`) and Cubical Agda:

In standard Agda: `sym refl = refl` (pattern match on the path)
In Cubical Agda: `sym p = λ i → p (~ i)` (precompose with complement)

1. Which definition is more general? Can you apply the cubical definition in standard Agda?
2. What computation does `sym (sym p)` perform in each system?
3. Why is the cubical definition definitionally an involution (`sym (sym p) = p`), while the standard one requires a proof?

**Exercise 5.2** In Lean 4, the following holds by proof-irrelevance:

```lean
example (P : Prop) (h₁ h₂ : P) : h₁ = h₂ := rfl
```

In Cubical Agda, this does NOT hold in general (two paths can be different). However, if `A` is a proposition (`isProp A`), then any two elements are equal:

```agda
open import Cubical.Foundations.HLevels

isPropElim : {A : Type} → isProp A → (x y : A) → x ≡ y
isPropElim propA x y = propA x y
```

Explain the relationship between:
1. Lean 4's proof-irrelevance for `Prop`
2. Cubical Agda's `isProp`
3. HoTT's h-level -1 (propositions)

Are these the same concept? What are the differences?

**Exercise 5.3** The K axiom, the K pattern matching rule, and UIP are equivalent in standard type theory. In Cubical Agda (`--cubical`), the K axiom is NOT available as a principle, but something else is:

The *path spaces* in Cubical Agda are NOT necessarily propositions. The circle's loop space `base ≡ base` is equivalent to `ℤ`, which has many non-equal elements. This shows that UIP fails.

But: can you state UIP in Cubical Agda? Can you disprove it? Write Agda code that exhibits two *definitionally different* paths `p q : base ≡ base` (e.g., `refl` and `loop`) and the fact that `p ≢ q` (they are not equal as paths).

**Exercise 5.4** In Cubical Agda, univalence is a theorem. In Lean 4, it is not available (without additional axioms). In Coq with the HoTT library, it is an axiom.

Compare the three approaches:
1. **Cubical Agda**: ua is computable; transport along ua e reduces to e .fst applied to the input
2. **Lean 4**: no univalence; propext and funext are present, but they don't give the full theorem
3. **Coq + HoTT library**: ua is an axiom; transport along ua e is propositionally (not definitionally) equal to e .fst applied to the input

For each approach, state:
- What can you prove?
- What can you compute?
- What are the limitations?

**Exercise 5.5 [Extended research]** The Brunerie number is defined by:

1. The Hopf fibration $h : S^3 \to S^2$ with fiber $S^1$
2. The cup product structure on $H^*(S^2; \mathbb{Z})$
3. A calculation using the Serre spectral sequence (or its HoTT equivalent)

Research the definition of the Brunerie number in HoTT (see Brunerie's 2016 thesis or the Cubical Agda library). Write down:
1. The type of `brunerie` in Cubical Agda
2. Why the number is an integer
3. Why it should equal 2 (from classical algebraic topology)
4. What "computing to 2" means in the Cubical Agda context
