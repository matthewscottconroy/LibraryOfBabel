# Higher Inductive Types in Cubical Agda: Circles, Spheres, and Fundamental Groups

In axiomatic HoTT, higher inductive types are axioms: you postulate the circle, postulate its eliminator, postulate the computation rules. These rules hold propositionally — as paths — but not definitionally. The circle is an abstract object you reason about but cannot compute with.

In Cubical Agda, HITs are `data` declarations. The circle is defined the same way as a natural number or a list — by listing its constructors. The difference is that one of the constructors produces a path, not a point. And the computation rules for the eliminator hold definitionally, because paths in Cubical Agda are functions, and function application computes.

## The Circle

```agda
{-# OPTIONS --cubical #-}
module Circle where

open import Cubical.Core.Everything
open import Cubical.Foundations.Prelude

-- The circle as a HIT
data S¹ : Type where
  base : S¹
  loop : base ≡ base
```

That is the complete definition. `S¹` has exactly two constructors:
- `base : S¹` — the basepoint (an ordinary point constructor)
- `loop : base ≡ base` — the loop (a path constructor: an element of `base ≡ base`)

To work with the circle, you write functions by "pattern matching" on these constructors. For a function `f : S¹ → B`, you must specify:
1. `f base : B` — the value at the basepoint
2. A path `f base ≡ f base` — the image of the loop under `f`

```agda
-- Non-dependent elimination: functions S¹ → B
S¹-rec : {B : Type} → (b : B) → (ℓ : b ≡ b) → S¹ → B
S¹-rec b ℓ base     = b
S¹-rec b ℓ (loop i) = ℓ i

-- Dependent elimination: sections of B : S¹ → Type
S¹-elim : (B : S¹ → Type)
           (b : B base)
           (ℓ : PathP (λ i → B (loop i)) b b)
           → (x : S¹) → B x
S¹-elim B b ℓ base     = b
S¹-elim B b ℓ (loop i) = ℓ i
```

The computation rules:
- `S¹-rec b ℓ base` reduces to `b` definitionally
- `S¹-rec b ℓ (loop i)` reduces to `ℓ i` definitionally

For the dependent case, `ℓ` must be a `PathP` — a path from `b` to `b` lying *over* the loop. This accounts for the fact that `B (loop i)` is different from `B base` when `B` is not constant.

### Winding maps

Using the eliminator, we can define maps that wind around the circle:

```agda
-- The identity: sends loop to loop
idS¹ : S¹ → S¹
idS¹ = S¹-rec base loop

-- Double winding: sends loop to loop ∙ loop
double : S¹ → S¹
double = S¹-rec base (loop ∙ loop)

-- Opposite: sends loop to sym loop
neg : S¹ → S¹
neg = S¹-rec base (sym loop)

-- The constant map: sends loop to refl
constant : S¹ → S¹
constant = S¹-rec base refl
```

## π₁(S¹) = ℤ: The Fundamental Theorem

The proof that the fundamental group of the circle is the integers is the showcase computation of cubical HoTT. We follow the encode-decode method, now made fully computational.

### The code family

The key idea: define a family `code : S¹ → Type` that "unwinds" the circle. Going around the loop once should shift the integer fiber by one.

```agda
open import Cubical.Data.Int

-- The successor equivalence on ℤ
succEquiv : ℤ ≃ ℤ
succEquiv = isoToEquiv (iso sucℤ predℤ suc-pred pred-suc)
  where
    suc-pred : ∀ n → sucℤ (predℤ n) ≡ n
    suc-pred (pos zero)       = refl
    suc-pred (pos (suc n))    = refl
    suc-pred (negsuc n)       = refl
    pred-suc : ∀ n → predℤ (sucℤ n) ≡ n
    pred-suc (pos n)          = refl
    pred-suc (negsuc zero)    = refl
    pred-suc (negsuc (suc n)) = refl

-- The code family
code : S¹ → Type
code base     = ℤ
code (loop i) = ua succEquiv i
-- code base = ℤ
-- code (loop i) traces the path ua succEquiv : ℤ ≡ ℤ
-- so going around loop once changes the fiber by succ
```

The definition `code (loop i) = ua succEquiv i` is saying: the type over `loop i` is the univalence path at position `i`. When `i = i0`, this is `ℤ`. When `i = i1`, this is also `ℤ`. But in the interior, the type is "glued" by `succEquiv`.

### Encoding: winding number

```agda
-- encode: given a path from base to x, produce an element of code x
encode : (x : S¹) → base ≡ x → code x
encode x p = transport (cong code p) (pos 0)
-- Start with 0 : ℤ = code base
-- Transport along code(p) : code base ≡ code x
-- Result: element of code x

-- Key computation:
-- encode base loop = transport (ua succEquiv) (pos 0)
--                  = succEquiv .fst (pos 0)   [by uaβ]
--                  = sucℤ (pos 0)
--                  = pos 1
-- So loop has winding number 1! ✓
```

The `uaβ` computation rule is what makes this work: transport along `ua succEquiv` computes to applying `succEquiv`'s underlying function. Without this rule, `encode` would be stuck.

### Decoding: loop powers

```agda
-- loopn: the n-th power of loop
loopn : ℤ → base ≡ base
loopn (pos zero)       = refl
loopn (pos (suc n))    = loopn (pos n) ∙ loop
loopn (negsuc zero)    = sym loop
loopn (negsuc (suc n)) = loopn (negsuc n) ∙ sym loop

-- decode: given an element of code x, produce a path from base to x
decode : (x : S¹) → code x → base ≡ x
decode base      = loopn
decode (loop i)  = -- This requires a PathP!
  -- We need: PathP (λ i → code (loop i) → base ≡ loop i) loopn loopn
  -- meaning: the decoder over the loop transforms consistently with succ
  -- This uses the fact that loopn (sucℤ n) = loopn n ∙ loop
  λ n → loopn (unglue (i ∨ ~ i) n)
  -- (simplified; the full proof uses an explicit PathP)
```

### Round trips: the equivalence

```agda
-- encode (decode base n) ≡ n (encode after decode is identity)
encode-decode : ∀ (n : ℤ) → encode base (loopn n) ≡ n
encode-decode (pos zero)       = refl   -- trivial: loopn 0 = refl, encode base refl = 0
encode-decode (pos (suc n))    = -- by induction and uaβ
  cong sucℤ (encode-decode (pos n))
encode-decode (negsuc zero)    = refl
encode-decode (negsuc (suc n)) =
  cong predℤ (encode-decode (negsuc n))

-- decode base (encode base p) ≡ p (decode after encode is identity)
-- Uses path induction (J), reducing to the case p = refl
decode-encode : ∀ (x : S¹) (p : base ≡ x) → decode x (encode x p) ≡ p
decode-encode x = J (λ y q → decode y (encode y q) ≡ q) (loopn (pos 0) ∙ refl ≡⟨ ... ⟩ refl)

-- The main equivalence
ΩS¹≃ℤ : (base ≡ base) ≃ ℤ
ΩS¹≃ℤ = isoToEquiv (iso (encode base) loopn encode-decode (decode-encode base))
```

This is the computation: `encode base` is the winding number function, `loopn` is the loop-power function, and they are mutual inverses. The fundamental group of the circle is the integers.

## The Suspension

```agda
-- Suspension of A
data Susp (A : Type) : Type where
  north : Susp A
  south : Susp A
  merid : A → north ≡ south

-- The non-dependent eliminator
Susp-rec : {B : Type} → (n : B) → (s : B) → (∀ a → n ≡ s) → Susp A → B
Susp-rec n s m north      = n
Susp-rec n s m south      = s
Susp-rec n s m (merid a i) = m a i

-- Key fact: Susp Bool ≃ S¹
-- The two meridians of Susp Bool form the loop of S¹

SuspBool→S¹ : Susp Bool → S¹
SuspBool→S¹ north        = base
SuspBool→S¹ south        = base
SuspBool→S¹ (merid true  i) = loop i   -- merid true traces the loop
SuspBool→S¹ (merid false i) = refl i   -- merid false is constant

S¹→SuspBool : S¹ → Susp Bool
S¹→SuspBool base      = north
S¹→SuspBool (loop i)  = (merid true ∙ sym (merid false)) i
-- go via merid true (north to south) then back via merid false

-- One proves these are inverses, giving Susp Bool ≃ S¹
```

Similarly, `Susp S¹ ≃ S²`, `Susp S² ≃ S³`, and so on. Suspension is the fundamental operation for building spheres.

## The Torus

```agda
-- The torus T²: S¹ × S¹
-- Can be defined as a HIT directly
data T² : Type where
  point : T²                         -- basepoint
  line1 : point ≡ point              -- first loop
  line2 : point ≡ point              -- second loop
  square : PathP (λ i → line1 i ≡ line1 i) line2 line2
  -- The square says: line1 and line2 commute (the torus, not the Klein bottle)

-- Alternative: as a product (this is an equivalence)
-- T² ≃ S¹ × S¹
-- This follows from the fact that ΩT² ≃ ℤ × ℤ
```

The `square` constructor is a 2D HIT constructor: it produces a path between paths. This is what makes the torus a torus rather than a Klein bottle — the square fills in commutation of the two loops.

## Propositional Truncation

```agda
-- Propositional truncation: squash A to a proposition
data ∥_∥₁ (A : Type ℓ) : Type ℓ where
  ∣_∣₁   : A → ∥ A ∥₁                  -- constructor
  squash₁ : ∀ (x y : ∥ A ∥₁) → x ≡ y  -- all elements are equal

-- Universal property: map out of ∥ A ∥₁ into a proposition P
rec₁ : {A : Type ℓ} {P : Type ℓ'} → isProp P → (A → P) → ∥ A ∥₁ → P
rec₁ propP f ∣ a ∣₁            = f a
rec₁ propP f (squash₁ x y i)   = propP (rec₁ propP f x) (rec₁ propP f y) i
-- The squash case: P is a proposition, so rec₁ f x and rec₁ f y
-- are equal by propP, giving us the path we need
```

The `squash₁` case is instructive: we're defining a function out of a HIT, and the path constructor case requires us to produce a path in `P`. Since `P` is a proposition, `propP` gives us the path between any two elements.

## Set Truncation

```agda
-- Set truncation: squash to a set
data ∥_∥₂ (A : Type ℓ) : Type ℓ where
  ∣_∣₂   : A → ∥ A ∥₂
  squash₂ : ∀ (x y : ∥ A ∥₂) (p q : x ≡ y) → p ≡ q

-- Universal property: map out into a set
rec₂ : {A : Type ℓ} {B : Type ℓ'} → isSet B → (A → B) → ∥ A ∥₂ → B
rec₂ setB f ∣ a ∣₂               = f a
rec₂ setB f (squash₂ x y p q i j) =
  setB (rec₂ setB f x) (rec₂ setB f y)
       (cong (rec₂ setB f) p) (cong (rec₂ setB f) q) i j
-- The squash₂ case produces a 2D path (a square), handled by isSet B
```

## Pushouts

```agda
-- Pushout of f : C → A and g : C → B
data Pushout {A B C : Type} (f : C → A) (g : C → B) : Type where
  inl  : A → Pushout f g
  inr  : B → Pushout f g
  push : (c : C) → inl (f c) ≡ inr (g c)

-- The universal property: maps out of a pushout
Pushout-rec : {A B C D : Type} {f : C → A} {g : C → B}
              (l : A → D) (r : B → D) (h : ∀ c → l (f c) ≡ r (g c))
              → Pushout f g → D
Pushout-rec l r h (inl a)     = l a
Pushout-rec l r h (inr b)     = r b
Pushout-rec l r h (push c i)  = h c i
```

The pushout is fundamental: it generalizes coproducts (when `C = ⊥`) and quotients (when `A = B = ⊤`). The van Kampen theorem computes π₁ of a pushout in terms of the π₁'s of the components.

## The Computational Payoff

Let's see the computational payoff explicitly:

```agda
-- Compute: what is the winding number of loop ∙ loop?
test1 : encode base (loop ∙ loop) ≡ pos 2
test1 = refl   -- holds definitionally!

-- Compute: what is the winding number of sym loop?
test2 : encode base (sym loop) ≡ negsuc 0
test2 = refl   -- holds definitionally!

-- Compute: loopn 3 is loop ∙ loop ∙ loop
test3 : loopn (pos 3) ≡ (loop ∙ loop) ∙ loop
test3 = refl   -- or close to refl by computation
```

These are not just theorems. They are computations. The proof that π₁(S¹) = ℤ is a running program that computes winding numbers. Hand it a loop, and it gives you an integer. Hand it an integer, and it gives you a loop.

This is what distinguishes Cubical Agda from axiomatic HoTT. In axiomatic HoTT, these are true theorems. In Cubical Agda, they are executable programs. The mathematics and the computation are the same thing.
