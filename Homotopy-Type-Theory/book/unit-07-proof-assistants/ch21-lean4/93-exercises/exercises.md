# Exercises: Lean 4 and Mathlib

*All exercises should be completed in a Lean 4 project with Mathlib. Install via elan (`lake new project math && lake exe cache get`). For exercises marked [Formalize], write actual Lean 4 code that compiles.*

---

## Section 1: Lean 4 Basics

**Exercise 1.1 [Formalize]** Open a Lean 4 file and type the following `#check` queries. Record what Lean displays for each:

```lean
#check Nat
#check (1 + 1 = 2)
#check @Eq
#check @Eq.refl
#check Type
#check Prop
#check (fun n : Nat => n + 1)
#check @List.length
```

Explain the universe of each type you observe. Why does `Prop : Type` rather than `Prop : Prop`?

**Exercise 1.2 [Formalize]** Define the following in Lean 4:

1. A function `square : Nat → Nat` computing `n^2`.
2. A function `cube : Nat → Nat` computing `n^3`.
3. A proof `square_pos : ∀ n : Nat, n > 0 → square n > 0` using tactic mode.
4. A function `isPerfect : Nat → Bool` that returns `true` if `n` equals the sum of its proper divisors. (You may use `List.filter` and `List.sum`.)

**Exercise 1.3 [Formalize]** Prove the following in *term mode* (without using `by`):

```lean
-- 1. And introduction
theorem and_intro (P Q : Prop) (hp : P) (hq : Q) : P ∧ Q := ⟨hp, hq⟩

-- 2. And elimination (prove it yourself without using And.left)
theorem and_elim_left (P Q : Prop) (h : P ∧ Q) : P := ?

-- 3. Function composition
theorem comp (P Q R : Prop) (f : P → Q) (g : Q → R) : P → R := ?

-- 4. Double negation introduction (in classical logic, elimination also holds)
theorem dne_intro (P : Prop) (hp : P) : ¬¬P := ?
```

**Exercise 1.4 [Formalize]** Define an inductive type for arithmetic expressions:

```lean
inductive Expr : Type where
  | num : Nat → Expr
  | add : Expr → Expr → Expr
  | mul : Expr → Expr → Expr
```

Define an `eval : Expr → Nat` function. Then prove `eval (Expr.add e₁ e₂) = eval e₁ + eval e₂`.

**Exercise 1.5** The `Prop`/`Type` distinction in Lean 4 corresponds to the distinction between proof-irrelevant propositions and data types. Explain in your own words:
- Why does Lean 4 make propositions proof-irrelevant (all proofs of `P : Prop` are definitionally equal)?
- What would go wrong if propositions were not proof-irrelevant in Lean 4?
- How does this relate to the h-level hierarchy in HoTT?

---

## Section 2: Tactics and Proofs

**Exercise 2.1 [Formalize]** Prove the following using tactic mode. Use at least four different tactics in your solutions:

```lean
-- 1. Modus ponens
theorem mp (P Q : Prop) (h1 : P → Q) (h2 : P) : Q := by ?

-- 2. Hypothetical syllogism
theorem hyp_syl (P Q R : Prop) (h1 : P → Q) (h2 : Q → R) : P → R := by ?

-- 3. De Morgan's law
theorem de_morgan (P Q : Prop) : ¬(P ∨ Q) → ¬P ∧ ¬Q := by ?

-- 4. Contrapositive
theorem contrapositive (P Q : Prop) (h : P → Q) : ¬Q → ¬P := by ?
```

**Exercise 2.2 [Formalize]** Prove by `induction`:

```lean
-- 1. Sum of first n natural numbers
def sumTo : Nat → Nat
  | 0     => 0
  | n + 1 => (n + 1) + sumTo n

theorem sumTo_eq (n : Nat) : 2 * sumTo n = n * (n + 1) := by ?

-- 2. Power of a product
theorem mul_pow (n m k : Nat) : (n * m) ^ k = n ^ k * m ^ k := by ?

-- 3. List reversal is an involution
theorem reverse_reverse (l : List α) : l.reverse.reverse = l := by ?
```

**Exercise 2.3 [Formalize]** Write `calc` proofs for:

```lean
-- 1. Integer arithmetic
theorem int_calc (a b c : ℤ) (h1 : a = 2 * b) (h2 : b = c + 3) : a = 2 * c + 6 := by
  calc a = ? := ?
    _ = ? := ?

-- 2. List length
theorem length_calc (l₁ l₂ l₃ : List Nat) :
    (l₁ ++ l₂ ++ l₃).length = l₁.length + l₂.length + l₃.length := by
  calc ?
```

**Exercise 2.4 [Formalize]** Use `exact?` to find Mathlib lemmas for each goal, then copy the suggestion into your proof:

```lean
-- 1. Commutativity of integer multiplication
example (n m : ℤ) : n * m = m * n := by exact?

-- 2. Every natural number is at most its square
example (n : Nat) : n ≤ n^2 := by exact?

-- 3. List append length
example (l₁ l₂ : List α) : (l₁ ++ l₂).length = l₁.length + l₂.length := by exact?

-- 4. Square root is monotone
example (x y : ℝ) (hx : x ≥ 0) (hxy : x ≤ y) : Real.sqrt x ≤ Real.sqrt y := by exact?
```

Record the lemma names that `exact?` suggests for each.

**Exercise 2.5 [Formalize]** Prove using `omega` and `linarith`:

```lean
-- 1. If n is even and m is odd, then n + m is odd
theorem even_add_odd (n m : Nat) (hn : n % 2 = 0) (hm : m % 2 = 1) :
    (n + m) % 2 = 1 := by omega

-- 2. Triangle inequality for naturals (subtracted version)
theorem nat_triangle (a b c : Nat) (h1 : a ≤ b + c) (h2 : b ≤ a + c) :
    a - b ≤ c := by omega

-- 3. Real number inequality chain
theorem real_chain (x y z : ℝ) (h1 : x ≤ 2 * y) (h2 : y ≤ z + 1) : x ≤ 2 * z + 2 := by
  linarith
```

**Exercise 2.6 [Formalize]** Prove the following "from scratch" — without using any Mathlib lemmas about this exact theorem (you may use `ring`, `simp`, basic operations):

```lean
-- The Bezout identity for specific values
-- Prove that gcd(6, 10) | 2 by finding coefficients:
-- 2 = 6 * 2 + 10 * (-1)
theorem bezout_example : ∃ (a b : ℤ), 6 * a + 10 * b = 2 := by
  use ?
  ring
```

---

## Section 3: Mathlib

**Exercise 3.1** Use Loogle (loogle.lean-lang.org) or `exact?` to find Mathlib theorems for:

1. Lagrange's theorem: the order of a subgroup divides the order of a finite group
2. The Chinese Remainder Theorem for $\mathbb{Z}/n\mathbb{Z}$
3. Every finite field has prime-power order
4. The fundamental theorem of finitely generated abelian groups

For each, write down the exact Lean 4 statement (the type of the theorem in Lean 4, as shown by `#check`).

**Exercise 3.2 [Formalize]** Explore `GroupTheory.FreeProduct`:

```lean
import Mathlib.GroupTheory.FreeProduct

-- 1. What is the type of FreeProduct.of?
#check @FreeProduct.of

-- 2. What is the universal property?
#check @FreeProduct.lift

-- 3. Prove: the cyclic group Z/2Z maps into Z/2Z * Z/2Z
-- (find the right map and verify it)
example : ∃ f : ZMod 2 →* FreeProduct (fun _ : Fin 2 => ZMod 2), Function.Injective f := by
  ?
```

**Exercise 3.3 [Formalize]** Use Mathlib's group theory to prove:

```lean
import Mathlib.GroupTheory.Subgroup.Basic
import Mathlib.GroupTheory.QuotientGroup

variable {G : Type*} [Group G]

-- The intersection of two subgroups is a subgroup
-- (Mathlib has this; find it and use it)
example (H K : Subgroup G) : Subgroup G := ?

-- The center of a group is a normal subgroup
-- (also in Mathlib; find it)
#check Subgroup.center_normal
```

**Exercise 3.4 [Formalize]** Formalize the following completely in Lean 4:

> **Theorem:** If $G$ is a group and $a, b \in G$, then $(ab)^{-1} = b^{-1}a^{-1}$.

Write the proof without using `group` (which would close it immediately) — use `calc` or explicit steps. Then also write the one-line `group` proof and reflect on what the tactic is doing.

**Exercise 3.5** Use Mathlib's category theory (`import Mathlib.CategoryTheory.Functor.Basic`) to:

1. Find the definition of `Functor.id` (the identity functor)
2. Find the definition of `Functor.comp` (composition of functors)
3. Find the statement that functor composition is associative
4. State (even if you don't prove it): what is the category whose objects are categories and whose morphisms are functors?

---

## Section 4: Formalization Projects

**Exercise 4.1 [Formalize]** Formalize the following theorem completely in Lean 4:

> **Theorem:** In a group $G$, the identity element is unique. That is, if $e \in G$ satisfies $eg = g$ for all $g$, then $e = 1$.

```lean
theorem unique_identity {G : Type*} [Group G] (e : G)
    (he : ∀ g : G, e * g = g) : e = 1 := by
  ?
```

*Hint:* Apply `he` to `1 : G`.

**Exercise 4.2 [Formalize]** Formalize the following:

> **Theorem:** If $\phi : G \to H$ is a group homomorphism and $G$ is abelian, then $\phi(G)$ is abelian.

```lean
theorem image_of_abelian_is_abelian {G H : Type*} [Group G] [Group H]
    [CommGroup G] (φ : G →* H) : ∀ (h₁ h₂ : H), h₁ ∈ φ.range → h₂ ∈ φ.range →
    h₁ * h₂ = h₂ * h₁ := by
  ?
```

**Exercise 4.3 [Formalize]** The integers modulo $n$:

```lean
import Mathlib.Data.ZMod.Basic

-- 1. Verify ZMod 5 is a field
#check (inferInstance : Field (ZMod 5))

-- 2. Prove that ZMod p is a field when p is prime
-- (find the Mathlib instance)

-- 3. Prove: in ZMod 6, we have 2 * 3 = 0
example : (2 : ZMod 6) * 3 = 0 := by decide

-- 4. Prove: ZMod 6 is NOT an integral domain
example : ¬ IsDomain (ZMod 6) := by
  ?
```

**Exercise 4.4 [Formalize]** Prove a small piece of algebraic topology formally:

> In Lean 4, the type `Equiv α β` (a bijection between `α` and `β`) is the classical analogue of HoTT's equivalence type. Prove the following:

```lean
-- Equiv is an equivalence relation (reflexive, symmetric, transitive)
theorem equiv_refl (α : Type*) : α ≃ α := Equiv.refl α

theorem equiv_symm {α β : Type*} (e : α ≃ β) : β ≃ α := ?

theorem equiv_trans {α β γ : Type*} (e₁ : α ≃ β) (e₂ : β ≃ γ) : α ≃ γ := ?

-- Now prove: if α ≃ β and β ≃ γ, then card α = card β = card γ
-- (for finite types)
theorem equiv_card {α β : Type*} [Fintype α] [Fintype β] (e : α ≃ β) :
    Fintype.card α = Fintype.card β := ?
```

**Exercise 4.5 [Formalize]** (Longer project) Formalize the following entirely in Lean 4:

> **Theorem:** The integers $\mathbb{Z}$ form a Euclidean domain (with the Euclidean function $|n|$).

This means: for any $a, b \in \mathbb{Z}$ with $b \neq 0$, there exist $q, r \in \mathbb{Z}$ such that $a = bq + r$ and $|r| < |b|$.

Find the Mathlib theorem for this (search for `EuclideanDomain`), write down its exact statement, and use it to derive: $\mathbb{Z}$ is a PID (principal ideal domain).

---

## Section 5: Lean 4 and HoTT

**Exercise 5.1** Explain precisely why the following is valid in Lean 4 but would not be valid in HoTT without the K axiom:

```lean
-- In Lean 4, all proofs of a Prop are definitionally equal
example (P : Prop) (h₁ h₂ : P) : h₁ = h₂ := rfl
```

What type does `rfl` have here? Why does proof-irrelevance (`Prop` being the universe of propositions) make this work? How does this relate to the K axiom?

**Exercise 5.2** In Lean 4, can you prove `Bool ≠ Nat` (as types)? Try:

```lean
example : Bool ≠ Nat := by
  intro h   -- h : Bool = Nat
  -- What can you derive from h?
  -- Hint: Bool has exactly 2 elements; Nat has infinitely many
  ?
```

What strategy would you use? (Hint: find a proposition true of one but not the other, then transport.)

**Exercise 5.3** The circle $S^1$ as a HIT has a non-trivial loop `loop : base = base` with `loop ≠ refl`. Try to state this in Lean 4:

```lean
-- Attempt to state that S¹ has a non-trivial loop
-- (This should fail or be unprovable in Lean 4)
-- Why?
```

Explain what Lean 4's treatment of `Prop`-valued equality prevents. How does Cubical Agda (Chapter 22) solve this?

**Exercise 5.4** The univalence axiom says: `(A ≡ B) ≃ (A ≃ B)`. In Lean 4, we have `propext : (P ↔ Q) → P = Q`.

1. Is `propext` a special case of univalence? In what sense?
2. In Lean 4, what is `@Eq Type Bool Nat`? Is this inhabited?
3. State, informally, what the full univalence axiom gives that `propext` alone does not.

**Exercise 5.5** (Research) Find the axioms used by a substantial Mathlib theorem:

```lean
#print axioms Nat.Prime.infinite    -- Infinitely many primes
-- What axioms does this use?

#print axioms Real.sqrt_two_mul_self  -- or some other real-analysis theorem
-- What axioms does this use?
```

Interpret the result: which axioms are "logical" (part of pure type theory) and which are "mathematical" (like choice or propext)?
