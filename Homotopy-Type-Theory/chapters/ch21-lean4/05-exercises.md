# Chapter 21 Exercises: Lean 4 and Mathlib

---

## Section 1: Lean 4 Basics

**Exercise 1.1.** Open a Lean 4 project with Mathlib and type the following in a `.lean` file. Record what Lean displays for each `#check`:

```lean
#check Nat          -- What universe does Nat live in?
#check 1 + 1 = 2    -- What type does this proposition have?
#check @Eq          -- What is the type of the equality former?
#check @Eq.refl     -- What is the type of reflexivity?
#check Type         -- What type does Type have?
#check Prop         -- What universe is Prop in?
```

**Exercise 1.2.** Define the following types and terms in Lean 4:

1. A dependent function type: a function taking `n : Nat` and returning a proof that `n + n = 2 * n`.
2. A dependent pair type: the type of pairs `(n, p)` where `n : Nat` and `p : Nat.Prime n`.
3. The type of functions from `Bool` to `Nat`.
4. The identity function on an arbitrary type `α`.

**Exercise 1.3.** Prove the following in term mode (without `by`):
1. `fun (P Q : Prop) (hp : P) (hq : Q) => And.intro hp hq : P → Q → P ∧ Q`
2. The function `fun (P Q R : Prop) (h : P ∧ Q ∧ R) => h.1 : P ∧ Q ∧ R → P`
3. The function that sends `n : Nat` to `Nat.succ n : Nat`

**Exercise 1.4.** Define your own inductive type for binary trees with data at the nodes:
```lean
inductive BinTree (α : Type) : Type where
  | leaf : BinTree α
  | node : BinTree α → α → BinTree α → BinTree α
```
Define a `size` function (number of nodes), a `depth` function (length of longest path to a leaf), and a `mirror` function (swap left and right subtrees). Prove `depth (mirror t) = depth t`.

---

## Section 2: Tactics and Proofs

**Exercise 2.1.** Prove the following using tactic mode. Try to use different tactics for each:

1. `∀ (P Q R : Prop), (P → Q) → (Q → R) → P → R` (function composition)
2. `∀ (P Q : Prop), P ∧ Q → Q ∧ P` (commutativity of and)
3. `∀ (P Q : Prop), ¬(P ∨ Q) → ¬P ∧ ¬Q` (De Morgan)
4. `∀ (P Q : Prop), ¬P ∧ ¬Q → ¬(P ∨ Q)` (De Morgan, other direction)

**Exercise 2.2.** Prove by induction:
1. `∀ n : Nat, 0 + n = n`
2. `∀ n : Nat, n * 0 = 0`
3. `∀ n : Nat, n * (n + 1) / 2 * 2 = n * (n + 1)` (or state this without division)
4. `∀ n : Nat, 1 + 2 + ... + n = n * (n + 1) / 2`

For (4), you'll need to define the sum function first.

**Exercise 2.3.** Use `exact?` or `apply?` to find Mathlib lemmas that prove:
1. `∀ (n m : ℤ), n + m = m + n`
2. `∀ (x : ℝ), x^2 ≥ 0`
3. `Nat.Prime 1000003`
4. `∀ (l₁ l₂ : List α), (l₁ ++ l₂).length = l₁.length + l₂.length`

Record the lemma names that `exact?` suggests.

**Exercise 2.4.** Write a `calc` proof of:
```lean
theorem calc_exercise (a b c : ℝ) (h₁ : a ≤ b) (h₂ : b < c) : a < c
```

**Exercise 2.5.** Prove the following using `induction` on lists:
```lean
theorem reverse_reverse (l : List α) : l.reverse.reverse = l
```

*Hint:* You'll need the lemma `List.reverse_append`.

---

## Section 3: Mathlib

**Exercise 3.1.** Find Mathlib lemmas (using `exact?`, Loogle, or the documentation) for each of the following:

1. The Chinese Remainder Theorem: `ZMod.chineseRemainder`
2. Lagrange's theorem: the order of a subgroup divides the order of the group
3. Every finite group of prime order is cyclic
4. The kernel of a group homomorphism is a normal subgroup

For each, write down the full Lean 4 statement of the theorem as Mathlib states it.

**Exercise 3.2.** Using Mathlib's category theory:

1. Find the definition of `Functor.comp` (composition of functors).
2. Find the proof that `Functor.comp` is associative.
3. Find `NatTrans.vcomp` (vertical composition of natural transformations).
4. What is `CategoryTheory.Adjunction.unit`? (The unit of an adjunction.)

**Exercise 3.3.** Explore `GroupTheory.FreeProduct`:

1. What is the type of `FreeProduct.of`? (How do you inject a group into the free product?)
2. What is the universal property stated as `FreeProduct.lift`?
3. How would you state: the free product $G * H$ satisfies the van Kampen property?

**Exercise 3.4.** Formalize a complete proof in Lean 4 that the integers $\mathbb{Z}$ have no zero divisors (i.e., if $mn = 0$ in $\mathbb{Z}$, then $m = 0$ or $n = 0$). This should use the `mul_eq_zero` lemma from Mathlib.

---

## Section 4: Formalization Projects

**Exercise 4.1 (Group axiom consequences).** Starting from the `Group` typeclass, prove the following *without* using any Mathlib automation beyond `ring` and `group`:

1. The identity element is unique: if `e` satisfies `∀ g, e * g = g`, then `e = 1`
2. Inverses are unique: if `a * b = 1`, then `b = a⁻¹`
3. `(a⁻¹)⁻¹ = a` for all `a : G`
4. `(a * b)⁻¹ = b⁻¹ * a⁻¹`

**Exercise 4.2 (Quotient groups).** Using Mathlib:

1. Define the quotient group `ℤ / nℤ` as `ZMod n` and verify it's a group.
2. Define the quotient map `ℤ → ℤ/nℤ` as a group homomorphism.
3. Verify the first isomorphism theorem: `ℤ / nℤ ≃ ℤ / ker(φ)` where `φ : ℤ → ℤ/nℤ` is the quotient map.

**Exercise 4.3 (Type equivalences).** In Lean 4, prove the following equivalences of types:

1. `α × β ≃ β × α` (commutativity of product)
2. `(α × β) × γ ≃ α × (β × γ)` (associativity of product)
3. `α ⊕ β ≃ β ⊕ α` (commutativity of sum)
4. `(α ⊕ β) × γ ≃ (α × γ) ⊕ (β × γ)` (distributivity)

For each, write the explicit functions and prove they are mutual inverses.

**Exercise 4.4 (Connecting to HoTT).** Consider the following in Lean 4:

```lean
-- In Lean 4, the equality type is a Prop
example (n m : Nat) : Prop := n = m

-- So there's always at most one proof
-- This corresponds to: Lean 4 satisfies the K axiom for Prop

-- Question: In HoTT, the type (n = m : ℤ) can in principle have multiple proofs
-- (as paths in an ∞-groupoid). But ℤ is a set in HoTT (h-level 0), so...
```

1. Prove in Lean 4 that `Nat` is a `DecidableEq` type (or find the Mathlib instance).
2. Using Hedberg's theorem (Section 17.3), argue informally why `Nat` should be a set in HoTT.
3. In Lean 4, can you state (even if you can't prove) "there exists a path in the type universe that is not reflexivity"? What happens when you try?

**Exercise 4.5 (Research project).** The Mathlib library has a proof of the Brouwer fixed-point theorem for dimension 2:

> Every continuous function $f : D^2 \to D^2$ (from the closed disk to itself) has a fixed point.

1. Find this theorem in Mathlib (search for `BrouwerFixedPoint` or similar).
2. Write down its exact statement in Lean 4.
3. Identify the key lemmas used in its proof (look at the proof dependencies).
4. How does this relate to the fundamental group $\pi_1(S^1) = \mathbb{Z}$? (Hint: the Brouwer fixed-point theorem for dimension 2 can be proved using the fact that $\pi_1(S^1) \neq 0$.)

---

## Section 5: Lean 4 vs. HoTT

**Exercise 5.1.** Explain, in precise type-theoretic terms, why the following Lean 4 "proof" is valid in Lean 4 but would not be valid in HoTT (without the K axiom):

```lean
-- In Lean 4, all proofs of a proposition are definitionally equal
example (P : Prop) (h₁ h₂ : P) : h₁ = h₂ := rfl
```

What precisely does `rfl` prove here? What type is it inhabiting? Why does this force P to be an hProp in HoTT terms?

**Exercise 5.2.** In HoTT, the type `A = B` (for types `A B : Type`) can be non-trivial — paths between types are equivalences (by Univalence). In Lean 4, try:

```lean
-- What does Lean 4 say about paths between types?
#check @Eq (Type) Bool Nat   -- Is this a valid type? What is its universe?
example : Bool ≠ Nat := by
  intro h
  -- What can you derive from h : Bool = Nat?
```

Is it possible to prove `Bool ≠ Nat` in Lean 4? What method would you use?

**Exercise 5.3.** The circle $S^1$ as a HIT has `loop : base = base` where `loop ≠ refl` (the loop is a non-trivial path). Explain why this is impossible to state in Lean 4 (given Lean 4's treatment of `Prop`-valued equality). What would go wrong?

**Exercise 5.4.** The following are equivalent in classical type theory (with K):
1. $K$ axiom: for any proof $p : a = a$, we have $p = \mathsf{refl}$
2. UIP (uniqueness of identity proofs): for any $p, q : a = b$, we have $p = q$
3. Homotopy level: every type is a set (h-level 0)

Show that these three statements are equivalent (you can argue informally or in pseudo-Lean). Which one does Lean 4 satisfy, and in what sense?
