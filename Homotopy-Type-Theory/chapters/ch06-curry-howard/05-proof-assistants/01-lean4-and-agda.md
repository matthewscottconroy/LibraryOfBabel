# 5.1 Proof Assistants: Lean 4 and Agda

## The Correspondence in Practice

The Curry-Howard correspondence is not just a theoretical construction — it's the working principle behind modern proof assistants. In Lean 4, Coq, and Agda, every proof is a term, every theorem is a type, and the type checker verifies that the term has the claimed type.

This section shows the correspondence in concrete code, so you can see exactly how it works.

## Lean 4: Proofs as Terms

In Lean 4, propositions are types (they have type `Prop`, which is a special universe), and proofs are terms of those types.

```lean
-- The proposition P → P (identity)
-- The term: λ hp : P, hp (identity function)
theorem id_proof (P : Prop) (hp : P) : P := hp

-- Equivalently (showing the term explicitly):
theorem id_proof' (P : Prop) : P → P := fun hp => hp
```

The `fun hp => hp` is the lambda term $\lambda hp. hp$. The theorem statement `P → P` is the type. The body `fun hp => hp` is the proof term.

```lean
-- Conjunction introduction
-- Proposition: P → Q → P ∧ Q
-- Term: λ hp, λ hq, ⟨hp, hq⟩ (pair formation)
theorem and_intro (P Q : Prop) (hp : P) (hq : Q) : P ∧ Q := ⟨hp, hq⟩

-- Conjunction elimination
-- Proposition: P ∧ Q → P
-- Term: λ h, h.1 (first projection)
theorem and_elim_left (P Q : Prop) (h : P ∧ Q) : P := h.1

-- Modus ponens
-- Proposition: (P → Q) → P → Q
-- Term: λ f, λ hp, f hp (function application)
theorem modus_ponens (P Q : Prop) (f : P → Q) (hp : P) : Q := f hp
```

Every proof is a lambda term. The type checker verifies the term has the claimed type.

**Using tactics:** Lean 4 also supports a tactic language that constructs terms interactively:

```lean
theorem conjunction_comm (P Q : Prop) (h : P ∧ Q) : Q ∧ P := by
  obtain ⟨hp, hq⟩ := h  -- destruct the conjunction
  exact ⟨hq, hp⟩         -- construct the swapped conjunction
```

The `by` block runs tactics that build the proof term. The final term is $\lambda h. (\pi_2(h), \pi_1(h))$ — swapping the pair.

## Proofs by Induction

Induction in Lean 4 corresponds to the *recursor* (eliminator) for natural numbers:

```lean
-- Prove: every natural number is either 0 or a successor
theorem nat_cases (n : ℕ) : n = 0 ∨ ∃ m, n = m + 1 := by
  cases n with
  | zero => exact Or.inl rfl
  | succ m => exact Or.inr ⟨m, rfl⟩
```

The `cases` tactic uses the eliminator for `ℕ`. The two cases correspond to `Nat.zero` (returns `Or.inl rfl`, the left injection with a reflexivity proof) and `Nat.succ m` (returns `Or.inr ⟨m, rfl⟩`, the right injection with a witness `m` and a proof).

## Dependent Types in Lean 4

Lean 4 is a full dependent type system. Here's a dependent function:

```lean
-- A vector of length n
def Vec (α : Type) : ℕ → Type
  | 0     => Unit  -- empty vector is the unit type
  | n + 1 => α × Vec α n  -- a vector of length n+1 is a head and a tail

-- A length-safe head function
-- Only defined for non-empty vectors (n+1, not 0)
def head {α : Type} (v : Vec α (n + 1)) : α :=
  v.1  -- the first component of the pair
```

The type `Vec α (n + 1)` depends on the value `n+1`. The function `head` is only typed to work on non-empty vectors — there's no "head of empty list" case to handle. The type system *enforces* non-emptiness.

This is a simple example of dependent types providing "correct by construction" programming.

## Agda: Proofs are Programs

Agda is another dependently typed language, closer to the type theory. Here's the same identity proof:

```agda
-- Identity
id : {A : Set} → A → A
id x = x

-- This is both a program (identity function) and a proof (P → P)
```

In Agda, there's less distinction between "propositions" and "types" — everything is a type. The `Set` universe contains both computational types and logical propositions.

```agda
-- Curry-Howard in action: dependent function types
-- A function that, given a proof of ⊥, produces any type
exfalso : {A : Set} → ⊥ → A
exfalso ()  -- empty pattern: no cases for ⊥
```

The `()` pattern means "pattern matching on the empty type": there are no cases, so the function is vacuously defined. This is the computational content of ex falso quodlibet.

## Identity Types in Lean 4

The identity type appears as `=` in Lean 4:

```lean
-- Reflexivity: refl is the canonical equality proof
#check @rfl  -- @rfl : ∀ {α : Sort u} {a : α}, a = a

-- Symmetry: if a = b then b = a
theorem symm {α : Sort u} {a b : α} (h : a = b) : b = a :=
  h ▸ rfl  -- rewrite using h, then use refl

-- Transitivity: if a = b and b = c then a = c  
theorem trans {α : Sort u} {a b c : α} (h1 : a = b) (h2 : b = c) : a = c :=
  h2 ▸ h1  -- substitute
```

These are the groupoid operations on the identity type: reflexivity (identity path), symmetry (path reversal), transitivity (path composition).

## The `▸` Rewrite in Lean 4

The `▸` (called "rewrite" or "transport") is the Lean 4 version of the J eliminator:

```lean
-- h : a = b, P : Type, t : P a
-- h ▸ t : P b  (transport t along h)
example (a b : ℕ) (h : a = b) (P : ℕ → Prop) (t : P a) : P b :=
  h ▸ t
```

Given a proof `h : a = b` and a term `t : P a`, `h ▸ t` produces a term of type `P b`. This is "transporting" the proof along the equality path — the fundamental operation of path induction.

Under the homotopy interpretation: $h$ is a path from $a$ to $b$, and $h ▸ t$ is the result of transporting the element $t$ (at the "start" of the path) along the path to get an element at $b$.

## HoTT-Style Proofs in Agda

The `homotopy-type-theory` Agda library (and similar libraries) work explicitly with paths and homotopies:

```agda
open import HoTT

-- funext: function extensionality
-- Two functions equal on all inputs are equal
funext : {A B : Type} {f g : A → B} → ((x : A) → f x ≡ g x) → f ≡ g
funext p = ...  -- requires function extensionality axiom or Univalence

-- The fundamental groupoid operations
! : {A : Type} {x y : A} → x ≡ y → y ≡ x  -- path reversal
_∙_ : {A : Type} {x y z : A} → x ≡ y → y ≡ z → x ≡ z  -- composition
```

These operations on paths are the computational content of the groupoid laws for identity types.

## What Proof Assistants Reveal

Working with proof assistants makes the Curry-Howard correspondence concrete:

1. **Every proof is a program.** When you write a proof in Lean 4 or Agda, you are writing a program. The `by exact` tactic returns a term; the `obtain` tactic destructs a pair; the `cases` tactic applies an eliminator.

2. **Type checking is proof checking.** The type checker mechanically verifies that your term has the claimed type. No human judgment is needed. This is the formal meaning of "proof" in these systems.

3. **Running a proof is computing.** If you have a proof of `∃ n : ℕ, P n`, you can `#eval` it to extract the witness `n`. The computation terminates (strong normalization) and gives a specific answer.

4. **Identity types are data.** An equality `h : a = b` is not just a logical fact but a term that can be manipulated, stored in data structures, and transported along other equalities.

5. **Higher equalities are higher data.** In HoTT-style Agda, an element of `h₁ ≡ h₂` (two path-equality proofs being equal) is a homotopy. This is genuine computational data with structure.

## The Power and Limits

**Power:** Proof assistants can mechanically verify complex mathematical proofs. Feit-Thompson theorem, four color theorem, Kepler conjecture, prime number theorem — all formally verified in Coq or Lean.

**Limits:** 
- **Termination checking** is required for all recursive functions. Some legitimate algorithms are hard to express in a termination-checking way.
- **Propositions vs. types:** In Lean 4 with `Prop`, proof-irrelevance means proofs are erased at runtime. This is fine for pure math but means you can't extract computational content from `Prop` proofs.
- **Classical axioms** are available but some proofs "work" classically but can't be executed computationally.

These tensions — between computational and classical proofs, between expressiveness and normalization — are among the central practical issues in formalized mathematics, and they're resolved in different ways by different proof assistants.

HoTT takes a distinctive stance: types are all equal (no `Prop`/`Type` distinction in the foundational system), but propositions are types at h-level $-1$ (mere propositions), and the distinction is recovered semantically. This unification is one of HoTT's main contributions to foundations.
