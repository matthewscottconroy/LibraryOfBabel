# Chapter 22 Exercises: Cubical Agda

---

## Section 1: Agda Basics

**Exercise 1.1.** Install Agda and set up a project. Verify the following in a `Test.agda` file:

```agda
{-# OPTIONS --without-K #-}
module Test where

open import Agda.Builtin.Nat

-- Verify these typecheck:
myFun : Nat → Nat
myFun n = n + 1

_ : Nat
_ = myFun 5
```

Check that Agda accepts the file. Then try:
```agda
-- Does this typecheck?
notK : {A : Set} {a : A} (p : a ≡ a) → p ≡ refl
```
What error do you get, and why?

**Exercise 1.2.** In Agda (without cubical), define:

1. The `Maybe` type: `data Maybe (A : Set) : Set where nothing : Maybe A; just : A → Maybe A`
2. A safe division function `safeDiv : Nat → Nat → Maybe Nat` that returns `nothing` when the divisor is 0.
3. A function `fromMaybe : {A : Set} → A → Maybe A → A` with a suitable default value.

**Exercise 1.3.** Prove the following in Agda using `--without-K`:

1. `sym (sym p) ≡ p` for any path `p : a ≡ b`
2. `cong id p ≡ p` for any `p`
3. `cong f (cong g p) ≡ cong (f ∘ g) p`

For (1), you'll need to think carefully about what `sym` is and what J gives you.

**Exercise 1.4.** Show that in Agda with `--without-K`, you cannot prove UIP:

```agda
-- This should NOT be provable:
UIP : {A : Set} {a b : A} (p q : a ≡ b) → p ≡ q
```

Explain *why* this isn't provable — what would need to be true about the model for this to fail?

---

## Section 2: Cubical Mode

**Exercise 2.1.** In Cubical Agda, verify the following by filling in the proofs:

```agda
{-# OPTIONS --cubical #-}
module Exercises2 where
open import Cubical.Foundations.Prelude

-- 1. Sym is an involution
sym-involutive : {A : Type} {a b : A} (p : a ≡ b) → sym (sym p) ≡ p
sym-involutive p = ?   -- fill this in!

-- 2. refl is the left unit for _∙_
lUnit : {A : Type} {a b : A} (p : a ≡ b) → refl ∙ p ≡ p
lUnit p = ?

-- 3. refl is the right unit for _∙_
rUnit : {A : Type} {a b : A} (p : a ≡ b) → p ∙ refl ≡ p
rUnit p = ?
```

For `sym-involutive`, the answer is `λ i j → p j`. Explain why this works — what are the boundary conditions?

**Exercise 2.2.** Define the following using cubical path operations:

1. The *whiskering* operations: for `p : a ≡ b` and `q : f b ≡ c`, define `p ▷ q : f a ≡ c` by concatenation through `cong f p`.
2. The *naturality square*: for `f g : A → B`, `H : ∀ x → f x ≡ g x`, and `p : a ≡ b`, define the 2-path showing that `cong f p ∙ H b ≡ H a ∙ cong g p`.

**Exercise 2.3.** In Cubical Agda, function extensionality is `funExt p = λ i x → p x i`. Verify:

1. `funExt (λ x → refl) ≡ refl` (funext of reflexivity)
2. `cong (λ f → f x) (funExt H) ≡ H x` (funext computation rule)

**Exercise 2.4.** Using `ua` and `uaβ`:

1. Construct a path `ℕ ≡ ℕ` corresponding to the "successor equivalence" (add 1 to everything).
2. Show that transporting along this path sends `n` to `n + 1`.
3. Construct a non-trivial element of `Bool ≡ Bool` and describe what it does to booleans.

**Exercise 2.5.** Prove in Cubical Agda that `Bool ≠ ℕ`:

```agda
Bool≠Nat : ¬ (Bool ≡ ℕ)
Bool≠Nat : p → ⊥
-- Hint: transport an element of Bool along p to get an element of ℕ,
-- then derive a contradiction from the structure of ℕ
```

---

## Section 3: HITs in Agda

**Exercise 3.1.** Define the interval as a HIT in Cubical Agda:

```agda
data I' : Type where
  zero' : I'
  one'  : I'
  seg   : zero' ≡ one'
```

(This is different from the built-in `I` — this one is a type, not a sort.)

1. Prove that `I'` is contractible: `isContr I'`.
2. Define a function `I' → ℕ` that sends `zero'` to 0 and `one'` to 1.
3. Show that any two functions `I' → A` that agree on `zero'` and `one'` are homotopic.

**Exercise 3.2.** Working with the circle:

```agda
open import Cubical.HITs.Circle
```

1. Define the double cover of `S¹` as a map `S¹ → Type` that maps `base` to `Bool` and `loop` to `ua notEquiv` (where `notEquiv : Bool ≃ Bool` swaps the elements).
2. Using this cover, show that the map `double : S¹ → S¹` (that goes around twice) is not homotopic to the identity.
3. Define the "figure-eight" space `S¹ ∨ S¹` and compute its fundamental group using van Kampen.

**Exercise 3.3.** Working with the suspension:

```agda
open import Cubical.HITs.Susp
```

1. Define a map `Susp (Susp Bool) → S²` (or describe it carefully if you can't write the Agda directly).
2. Show that `Susp Unit ≃ Bool`: the suspension of the one-point type has two points.
3. Show that `Susp Empty ≃ Unit`: the suspension of the empty type has one point.

**Exercise 3.4.** Using propositional truncation:

```agda
open import Cubical.HITs.PropositionalTruncation
```

1. Prove: `∥ A ∥₁ → ∥ B ∥₁` given `A → B`.
2. Prove: `∥ A × B ∥₁ → ∥ A ∥₁ × ∥ B ∥₁`.
3. Is the converse of (2) true? Prove it or give a counterexample.
4. Prove: `∥ ∥ A ∥₁ ∥₁ ≡ ∥ A ∥₁` (truncation is idempotent).

---

## Section 4: HoTT in the Cubical Library

**Exercise 4.1.** Working with h-levels:

1. Prove that `isProp A → isProp (A × A)` from first principles (using `isProp×` or by hand).
2. Prove that `isProp A → isProp (A → A)` from first principles.
3. Prove that `isContr A ↔ isProp A × A` (a type is contractible iff it's a proposition and inhabited).

**Exercise 4.2.** Prove that univalence (restricted to propositions) gives "propositional extensionality":

```agda
propExt : (A B : Prop) → (A → B) → (B → A) → A ≡ B
```

where `Prop = Σ Type isProp`. This is the proposition version of `ua`.

**Exercise 4.3.** In the Cubical library, find and understand the proof of:

```agda
-- πₙ(Sⁿ) = ℤ
π₁S¹≃ℤ : GroupEquiv (πₙ 1 S¹ base) ℤGroup
```

Trace through the proof to identify:
- Where the code family is defined
- Where `encode` and `decode` are defined
- Which lemma establishes the round-trip identities
- Where the group structure on `base ≡ base` is established

**Exercise 4.4.** Explain the difference between `Iso A B` and `A ≃ B` in the Cubical library. Under what conditions are they equivalent? Give an example where an `Iso` is easier to construct than an `≃` directly.

**Exercise 4.5.** The Eilenberg-MacLane space `K(G, 1)` for a group `G` satisfies:
- `π₁(K(G, 1)) = G`
- `πₙ(K(G, 1)) = 0` for `n ≥ 2`

Using the Cubical library:
1. Find `Cubical.HITs.EilenbergMacLane1` (or similar) and check the definition.
2. What constructors does `K(G, 1)` have?
3. Verify that `K(ℤ, 1) ≃ S¹` by finding (or constructing) the equivalence.

---

## Section 5: Research-Level Exercises

**Exercise 5.1 (Brunerie's number).** The Brunerie number is a specific integer `n` defined by:

```agda
brunerie : ℤ
brunerie = Brunerie.n  -- defined in Cubical.Homotopy.Brunerie
```

1. Install the Cubical library and load the Brunerie module.
2. Normalize `brunerie` (using Agda's normalization command). Does it compute to 2?
3. How long does the normalization take? (This is a computational benchmark for Cubical Agda.)

**Exercise 5.2 (Formalization contribution).** Choose one of the following from the HoTT Book that is *not yet fully formalized* in the Cubical library and formalize it:

- The van Kampen theorem for $\pi_1(S^1 \vee S^1) = F_2$ (free group on two generators)
- Whitehead's theorem for sets: a surjection between sets that is an equivalence on path spaces is an equivalence
- The statement and proof that `isOfHLevel n` implies `isOfHLevel (n+1)` (cumulativity of h-levels)

Submit a `.agda` file with your formalization.

**Exercise 5.3 (The Hopf fibration).** The Cubical library has the Hopf fibration:

```agda
open import Cubical.HITs.Hopf
hopf : S³ → S²
hopf-fiber : (b : S²) → fiber hopf b ≃ S¹
```

1. Trace through the definition of `hopf` in the library. How is it defined using the join construction?
2. Using the long exact sequence of the Hopf fibration, state (as a Cubical Agda theorem) that `π₃(S²) = ℤ`.
3. Verify that `π₃(S²) = ℤ` is in the Cubical library and identify the key proof steps.

**Exercise 5.4 (Computational content).** One of the remarkable features of Cubical Agda is that proofs can run:

```agda
-- The winding number is computable
test₁ : winding loop ≡ pos 1
test₁ = refl   -- or: reduce to refl by computation

test₂ : winding (loop ∙ loop ∙ loop) ≡ pos 3
test₂ = refl
```

1. Verify these computations in Cubical Agda (they may need a few steps).
2. Define a loop that winds -2 times and verify the winding number is `negsuc 1`.
3. Write a Cubical Agda program that, given `n : ℤ`, produces a path `base ≡ base` with winding number `n`, and verify this by composing with the winding number function.
