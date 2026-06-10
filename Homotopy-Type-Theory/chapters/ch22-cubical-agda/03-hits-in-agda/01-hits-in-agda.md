# 3.1 Higher Inductive Types in Cubical Agda

## HITs: The Payoff of Cubical Type Theory

In MLTT with axiomatic univalence (the HoTT Book's setting), higher inductive types are additional axioms. You postulate the circle `S¹` with `base` and `loop`, postulate its elimination principle, and postulate the computation rules. These rules hold *propositionally* — as paths — but not definitionally.

In Cubical Agda, HITs are *proper inductive types*. You define them with constructors for both points and paths, and the computation rules hold *definitionally*. The path constructors are like ordinary constructors, except they output path types instead of points.

This is not a minor difference. It means:
- HIT eliminators have actual computation rules (not just propositional rewrites)
- You can define functions out of HITs by pattern matching, just like ordinary types
- The resulting proofs have computational content — you can run them

## The Circle

The circle `S¹` is the canonical example of a HIT:

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

That's it. `S¹` has two constructors:
- `base : S¹` — the basepoint (a point constructor)
- `loop : base ≡ base` — the loop (a path constructor)

Compare with the non-HIT version: `data S¹ where base : S¹` gives a type with one point and only trivial paths. The HIT adds a genuine loop.

### The elimination principle

To define a function `f : S¹ → B` (or prove a property of all elements of `S¹`), you need:
1. A value for `f base : B`
2. A path `f base ≡ f base` (the image of `loop` under `f`)

```agda
-- Non-dependent elimination (for functions S¹ → B)
S¹-rec : {B : Type} (b : B) (ℓ : b ≡ b) → S¹ → B
S¹-rec b ℓ base      = b
S¹-rec b ℓ (loop i)  = ℓ i

-- Dependent elimination (for sections of B : S¹ → Type)
S¹-elim : (B : S¹ → Type)
           (b : B base)
           (ℓ : PathP (λ i → B (loop i)) b b)
           → (x : S¹) → B x
S¹-elim B b ℓ base      = b
S¹-elim B b ℓ (loop i)  = ℓ i
```

The computation rules:
- `S¹-rec b ℓ base` reduces to `b` definitionally
- `S¹-rec b ℓ (loop i)` reduces to `ℓ i` definitionally

For `loop i`, as `i` varies from `i0` to `i1`, `S¹-rec b ℓ (loop i)` traces the path `ℓ`. At `i0`: `S¹-rec b ℓ base = b`. At `i1`: `S¹-rec b ℓ base = b`. So `ℓ` must be a path from `b` to `b`, which is what we required.

### Looping maps

Using `S¹-rec`, we can define maps that go around the circle a fixed number of times:

```agda
-- The identity map: loop ↦ loop
idS¹ : S¹ → S¹
idS¹ = S¹-rec base loop

-- The "double winding" map: loop ↦ loop ∙ loop
double : S¹ → S¹
double = S¹-rec base (loop ∙ loop)

-- The "constant" map: loop ↦ refl
constant : S¹ → S¹
constant = S¹-rec base refl
```

## The Fundamental Group of the Circle

The fundamental theorem of synthetic homotopy theory: $\pi_1(S^1) = \mathbb{Z}$.

We'll follow the encode-decode method from Chapter 20, now with actual Cubical Agda code.

### Step 1: The code family

```agda
open import Cubical.Data.Int

-- The code family: S¹ → Type
-- code base = ℤ
-- code (loop i) = ua succ-equiv i  (the path in Type given by the successor equivalence)

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

-- The code family via ua
code : S¹ → Type
code base      = ℤ
code (loop i)  = ua succEquiv i
```

Note: `ua succEquiv i` is a path `ℤ ≡ ℤ` in the universe. When we define `code (loop i) = ua succEquiv i`, we're saying that going around `loop` in the base type corresponds to applying `succEquiv` to the code. This is the crucial definition.

### Step 2: Encoding

```agda
-- encode: transport along a path gives an integer (the winding number)
encode : ∀ (x : S¹) → base ≡ x → code x
encode x p = transport (cong code p) (pos zero)
-- transport the "seed" integer 0 along the path in Type given by code ∘ p
```

When `p = loop`: `transport (cong code loop) (pos zero)` = `transport (ua succEquiv) (pos zero)` = `succEquiv .fst (pos zero)` = `pos 1` = `1`. (Using the computation rule `uaβ`.)

So `encode base loop = 1`. Repeating: `encode base (loop ∙ loop) = 2`, etc. This is the winding number.

### Step 3: Decoding

```agda
-- loop^n : the n-th power of loop
loopn : ℤ → base ≡ base
loopn (pos zero)       = refl
loopn (pos (suc n))    = loopn (pos n) ∙ loop
loopn (negsuc zero)    = sym loop
loopn (negsuc (suc n)) = loopn (negsuc n) ∙ sym loop

-- decode: given a code, produce a path
decode : ∀ (x : S¹) → code x → base ≡ x
decode base      = loopn
decode (loop i)  = ?  -- we need a dependent path!
```

The `decode (loop i)` case requires a `PathP` — a path over `loop`. We need to show that as `i` varies from `i0` to `i1`, `decode (loop i)` varies from `loopn` to `loopn` in a specific way.

The required path is:
```agda
-- PathP (λ i → code (loop i) → base ≡ loop i) loopn loopn
-- This says: going around loop transforms the decoder
-- by precomposing with succ (since loop encodes succ)
```

This requires showing `loopn (sucℤ n) = loopn n ∙ loop`, which is provable by cases on `n`.

### Step 4: Round trips

```agda
-- encode (decode n) = n (for x = base)
encode-decode : ∀ (n : ℤ) → encode base (decode base n) ≡ n
encode-decode (pos zero)       = refl
encode-decode (pos (suc n))    = -- by induction and uaβ
  transport (ua succEquiv) (encode base (decode base (pos n)))
    ≡⟨ uaβ ... ⟩ sucℤ (encode base (decode base (pos n)))
    ≡⟨ cong sucℤ (encode-decode (pos n)) ⟩ sucℤ (pos n) = pos (suc n) ∎
encode-decode (negsuc n)       = -- similar, using pred

-- decode (encode p) = p
decode-encode : ∀ (x : S¹) (p : base ≡ x) → decode x (encode x p) ≡ p
decode-encode x p = J (λ y q → decode y (encode y q) ≡ q)
                      (loopn (pos zero) ≡⟨ refl ⟩ refl ∎) p
-- J reduces to the base case x = base, p = refl
-- In this case: decode base (encode base refl)
--             = decode base (transport (cong code refl) (pos 0))
--             = decode base (pos 0)      [transport along refl is id]
--             = loopn (pos 0) = refl ✓
```

### The main theorem

```agda
-- The fundamental group of the circle
ΩS¹≃ℤ : (base ≡ base) ≃ ℤ
ΩS¹≃ℤ = isoToEquiv (iso encode' decode' encode-decode decode-encode')
  where
    encode' = encode base
    decode' = decode base
    -- ...
```

This is the theorem: the loop space of `S¹` at `base` is equivalent to `ℤ`. And more: from this equivalence, `π₁(S¹)` (the group of homotopy classes of loops) is isomorphic to `ℤ` as a group.

## The Suspension

The suspension `Σ A` has two points and a path for each element of `A`:

```agda
-- Suspension
data Susp (A : Type) : Type where
  north : Susp A
  south : Susp A
  merid : A → north ≡ south
```

Recall from Chapter 19: `Susp Bool ≃ S¹`, `Susp S¹ ≃ S²`, etc. Let's verify the first:

```agda
-- Susp Bool ≃ S¹
-- The idea: merid true : north ≡ south
--           merid false : north ≡ south
-- The loop is merid true ∙ sym (merid false)

SuspBool→S¹ : Susp Bool → S¹
SuspBool→S¹ north        = base
SuspBool→S¹ south        = base
SuspBool→S¹ (merid b i) = if b then loop i else refl i
  -- if b = true: trace loop (going from base to base via loop)
  -- if b = false: stay at base (constant path)

S¹→SuspBool : S¹ → Susp Bool
S¹→SuspBool base      = north
S¹→SuspBool (loop i)  = (merid true ∙ sym (merid false)) i
  -- go along merid true (north to south) then back along merid false
```

One can then prove these are mutual inverses, giving `Susp Bool ≃ S¹`.

## Pushouts

The pushout is one of the most important HITs — it generalizes both quotient types and coproducts:

```agda
-- Pushout A ← C → B
data Pushout {A B C : Type} (f : C → A) (g : C → B) : Type where
  inl : A → Pushout f g
  inr : B → Pushout f g
  push : (c : C) → inl (f c) ≡ inr (g c)
```

Special cases:
- `A = B = ⊤`, `f c = g c = tt`: quotient of `C` by the equivalence relation generated by `f c = g c`
- `C = ⊥`: coproduct `A ⊔ B`
- General case: the pushout square

The van Kampen theorem says $\pi_1(A \sqcup_C B) = \pi_1(A) *_{\pi_1(C)} \pi_1(B)$ — and in Cubical Agda, this can be proved by the encode-decode method applied to the pushout.

## Truncations

The propositional truncation of `A` is the HIT that "squashes" all elements together:

```agda
-- Propositional truncation (from Cubical library)
data ∥_∥₁ (A : Type) : Type where
  ∣_∣₁   : A → ∥ A ∥₁                         -- constructor
  squash₁ : ∀ (x y : ∥ A ∥₁) → x ≡ y          -- truncation path
```

The truncation path `squash₁` says: any two elements of `∥ A ∥₁` are connected by a path. This makes `∥ A ∥₁` a proposition (an h-level -1 type).

```agda
-- Universal property: maps out of ∥ A ∥₁ into a proposition P
-- factor uniquely through ∣_∣₁
rec₁ : {A : Type} {P : Type} → isProp P → (A → P) → ∥ A ∥₁ → P
rec₁ propP f ∣ a ∣₁          = f a
rec₁ propP f (squash₁ x y i) = propP (rec₁ propP f x) (rec₁ propP f y) i
```

The `squash₁ x y i` case is handled by using `propP` (the proof that `P` is a proposition) to get a path between `rec₁ f x` and `rec₁ f y`. Since we're defining a function into a proposition, any two values must be equal, and `propP` gives us the path.

Similarly for set truncation:

```agda
-- Set truncation: make every path space a proposition
data ∥_∥₂ (A : Type) : Type where
  ∣_∣₂   : A → ∥ A ∥₂
  squash₂ : ∀ (x y : ∥ A ∥₂) (p q : x ≡ y) → p ≡ q
```

`squash₂` says: any two paths between the same endpoints are themselves equal. This makes `∥ A ∥₂` a set (h-level 0).

## Eilenberg-MacLane Spaces

In Cubical Agda, Eilenberg-MacLane spaces can be defined as HITs:

```agda
-- K(G, 1) for an abelian group G
-- as a HIT (sketch):
data K[G,1] (G : AbelianGroup) : Type where
  base : K[G,1] G
  loop : G .carrier → base ≡ base
  loop-comp : ∀ g h → loop g ∙ loop h ≡ loop (g + h)  -- path-path constructor!
  trunc : isGroupoid (K[G,1] G)  -- make it a 1-type
```

`K[G,1]` has not just point and path constructors but *path-between-path* constructors (`loop-comp`), making it a 2D HIT. The truncation constructor `trunc` makes all 2-paths trivial, which is needed to get an actual 1-type (not a higher-dimensional structure).

The Cubical library has full implementations of Eilenberg-MacLane spaces and their cohomological properties.

## The Power of HITs in Cubical Agda

Let's step back and appreciate what we've achieved:

**Definability:** We defined the circle, suspension, pushouts, and truncations as ordinary data types. No axioms, no postulates — just `data` declarations.

**Computability:** The elimination principles have computation rules. `S¹-rec b ℓ base = b` definitionally. This means proofs and programs involving HITs actually run.

**Generality:** The encode-decode method is not just a proof technique — it's a program. `encode` and `decode` are functions you can execute on specific inputs.

For example:
```agda
-- Compute the winding number of loop ∙ loop ∙ loop
test : encode base (loop ∙ loop ∙ loop) ≡ pos 3
test = refl   -- or by computation
```

This is mathematics and computation unified: the proof that $\pi_1(S^1) = \mathbb{Z}$ is the same as a working implementation of the winding number function.

In the next section, we survey the full Cubical Agda library and see how these primitives are assembled into a comprehensive formalization of HoTT.
