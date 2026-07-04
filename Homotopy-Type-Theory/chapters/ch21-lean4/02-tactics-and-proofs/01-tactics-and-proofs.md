# 2.1 Tactics and Proofs in Lean 4

## Two Ways to Write Proofs

In Lean 4, you can prove theorems in two modes: **term mode** and **tactic mode**.

In *term mode*, you write the proof as an explicit term — a program that inhabits the type you want to prove. Since propositions are types and proofs are programs, this is always possible in principle. But for complex theorems, it can be like writing assembly: complete and correct, but painful.

In *tactic mode*, you describe *how* to build the proof term rather than specifying the term directly. Tactics are commands that manipulate a *proof state* — a structured list of goals — until all goals are discharged. Lean's kernel then assembles the tactic execution into an actual proof term and typechecks it.

Both modes produce the same objects — proof terms. Tactic mode is just a more convenient way to construct them for complex proofs.

```lean
-- Term mode proof (direct construction)
theorem add_comm_term (n m : Nat) : n + m = m + n :=
  Nat.add_comm n m

-- Tactic mode proof (the "by" keyword enters tactic mode)
theorem add_comm_tactic (n m : Nat) : n + m = m + n := by
  exact Nat.add_comm n m

-- A more complex tactic proof
theorem add_assoc_zero (n : Nat) : n + 0 = n := by
  induction n with
  | zero => rfl
  | succ k ih => simp [Nat.succ_add, ih]
```

The `by` keyword switches from term mode to tactic mode. Everything after it is a sequence of tactics.

## The Proof State

When you enter tactic mode, Lean shows you the **proof state** — a display of what remains to be proved. In VS Code with the Lean extension, this updates in real time as you type.

The proof state looks like:

```
case zero
⊢ 0 + 0 = 0

case succ
k : Nat
ih : k + 0 = k
⊢ k + 1 + 0 = k + 1
```

Each block is a **goal**: everything above `⊢` is the *context* (local hypotheses and variables), and `⊢ P` means "prove P."

Reading the proof state is the central skill of tactic-mode proving. You look at the goal, figure out what tactic will simplify it or break it apart, apply the tactic, and check the new state. It's interactive mathematics.

## Core Tactics

### `intro` and `intros`

`intro h` introduces a hypothesis. If your goal is `A → B`, `intro h` adds `h : A` to the context and changes the goal to `B`. If your goal is `∀ x : A, P x`, `intro x` adds `x : A` and changes the goal to `P x`.

```lean
theorem imp_intro (P Q : Prop) (h : P) : P → Q → P := by
  intro _hP  -- introduces the P assumption (ignore it with _)
  intro _hQ  -- introduces the Q assumption
  exact h    -- we already have h : P in context
```

`intros` introduces multiple things at once: `intros h1 h2 h3` is `intro h1; intro h2; intro h3`.

### `exact`

`exact e` closes the goal by providing the exact term `e` that inhabits the goal type. This is the simplest tactic: you just directly give the proof.

```lean
theorem exact_example (P : Prop) (h : P) : P := by
  exact h
```

If `e` doesn't have the right type, `exact` fails. Use `exact?` to search for a term that closes the current goal — it searches Mathlib for applicable lemmas.

### `apply`

`apply f` when `f : A → B` (or `f : ∀ x, P x → Q x`, etc.) and the goal is `B`. It reduces the goal to proving `A` — you tell Lean "I'll prove this using `f`, now you need to prove `f`'s premises."

```lean
theorem apply_example (P Q R : Prop) (hPQ : P → Q) (hQR : Q → R) (h : P) : R := by
  apply hQR   -- goal becomes: Q
  apply hPQ   -- goal becomes: P
  exact h     -- done
```

This is how you do backward reasoning: instead of building up from hypotheses, you work backward from the goal.

### `rw` (rewrite)

`rw [h]` rewrites using an equation `h : a = b`, replacing `a` with `b` in the goal. `rw [← h]` rewrites right-to-left (replacing `b` with `a`).

```lean
theorem rw_example (a b c : Nat) (h1 : a = b) (h2 : b = c) : a = c := by
  rw [h1]   -- goal: b = c
  rw [h2]   -- goal: c = c
  rfl       -- reflexivity closes it

-- Or in one step:
theorem rw_example' (a b c : Nat) (h1 : a = b) (h2 : b = c) : a = c := by
  rw [h1, h2]
```

`rw [h] at hyp` rewrites in hypothesis `hyp` rather than the goal. Very useful when you need to simplify a hypothesis.

### `simp`

`simp` is Lean's general-purpose simplifier. It applies a collection of rewrite rules (the "simp lemmas") to simplify the goal as much as possible. Many simple goals are just `by simp`.

```lean
theorem simp_example (n : Nat) : n + 0 = n := by simp
theorem simp_example2 (l : List Nat) : l ++ [] = l := by simp
```

You can tell `simp` to use specific lemmas: `simp [h1, h2, lemma_name]`. The lemma `@[simp]` attribute marks a lemma as a default simp rule.

`simp only [...]` uses *only* the listed lemmas (no defaults) — more predictable, better for large proofs.

### `ring` and `linarith`

For algebraic goals:
- `ring` proves equalities that hold in any commutative ring by normalization. It handles `+`, `*`, `-`, `^` with integer exponents, etc.
- `linarith` proves linear arithmetic goals (`a + b ≤ c`, `2 * x < y`, etc.) by finding a linear combination of hypotheses.
- `omega` proves linear arithmetic goals over integers and naturals, including divisibility.

```lean
theorem ring_example (a b : ℤ) : (a + b)^2 = a^2 + 2*a*b + b^2 := by ring

theorem linarith_example (x y : ℝ) (h1 : x > 0) (h2 : y > x) : y > 0 := by linarith

theorem omega_example (n : Nat) (h : n % 2 = 0) : ∃ k, n = 2 * k := by omega
```

These tactics are workhorses for mathematical formalization. A surprising amount of algebraic bookkeeping is just `by ring` or `by linarith`.

### `induction` and `cases`

`induction x` performs induction on an inductive type. The proof state splits into one goal per constructor.

```lean
theorem add_zero (n : Nat) : n + 0 = n := by
  induction n with
  | zero      => rfl
  | succ k ih => rw [Nat.succ_add]; rw [ih]
  --              ↑ turns (k+1) + 0 into k + 0 + 1    ↑ uses induction hypothesis

theorem list_append_nil (l : List α) : l ++ [] = l := by
  induction l with
  | nil         => rfl
  | cons x xs ih => simp [List.cons_append, ih]
```

`cases h` performs case analysis on `h : T` without generating induction hypotheses. Use this when `T` is a simple type like `Bool` or `Or`.

```lean
theorem or_comm (P Q : Prop) (h : P ∨ Q) : Q ∨ P := by
  cases h with
  | inl hp => exact Or.inr hp
  | inr hq => exact Or.inl hq
```

`rcases` is a more powerful destructuring tactic that can handle nested patterns at once:

```lean
theorem and_comm (P Q : Prop) (h : P ∧ Q) : Q ∧ P := by
  rcases h with ⟨hp, hq⟩   -- destructure the pair
  exact ⟨hq, hp⟩
```

### `constructor` and `use`

When your goal is a conjunction `P ∧ Q` or an existential `∃ x, P x`:

- `constructor` splits `P ∧ Q` into two subgoals: prove `P`, then prove `Q`.
- `use e` resolves `∃ x, P x` by providing the witness: `use 42` turns `∃ n, n > 40` into `42 > 40`.

```lean
theorem exists_example : ∃ n : Nat, n > 100 := by
  use 101
  norm_num   -- numeric normalization: closes 101 > 100

theorem and_example (P Q : Prop) (hp : P) (hq : Q) : P ∧ Q := by
  constructor
  · exact hp
  · exact hq
```

The `·` (dot) notation focuses on a specific subgoal. It's good practice to use it whenever you split a goal.

### `have` and `obtain`

`have h : P := proof` introduces an intermediate lemma `h : P` using an explicit proof. Then `h` is available as a hypothesis for the rest of the proof.

```lean
theorem have_example (n : Nat) : n + n = 2 * n := by
  have h : 2 * n = n + n := by ring
  linarith
```

`obtain ⟨h1, h2⟩ := hyp` is like `rcases` but used for a hypothesis already in context.

### `calc` blocks

For equational reasoning with multiple steps:

```lean
theorem calc_example (a b c d : ℤ) (h1 : a = b + 1) (h2 : b = c - 3) (h3 : c = d + 4) : a = d + 2 := by
  calc a = b + 1     := h1
    _ = (c - 3) + 1  := by rw [h2]
    _ = (d + 4 - 3) + 1 := by rw [h3]
    _ = d + 2        := by ring
```

Each step `_ = next := proof` must close the equality. The `_` on the left refers to the right-hand side of the previous step.

## Automation Tactics

### `decide`

`decide` works on decidable propositions — propositions where membership can be computed. It's perfect for finite computations.

```lean
#eval Nat.Prime 17   -- true (as a computation)
example : Nat.Prime 17 := by decide
example : ¬ (2 + 2 = 5) := by decide
```

### `norm_num`

`norm_num` proves numerical goals — arithmetic with integers, rationals, reals. Much more powerful than `decide` for numerical goals.

```lean
example : (17 : ℝ) is a prime := by norm_num  -- well, approximately
example : (1 : ℝ) / 3 + 1 / 6 = 1 / 2 := by norm_num
example : Nat.Prime 1000003 := by norm_num
```

### `aesop`

`aesop` is an automated tactic that combines search, simplification, and case analysis. It can close many goals that require combining several simpler steps.

### `tauto`

`tauto` proves propositional tautologies — goals that follow from the truth-table structure of `∧`, `∨`, `¬`, `→`.

### `field_simp` and `push_neg`

- `field_simp` clears denominators in field expressions.
- `push_neg` pushes negations inward: `¬ ∀ x, P x` becomes `∃ x, ¬ P x`.

```lean
theorem push_neg_example : ¬ ∀ n : Nat, n < 10 := by
  push_neg        -- goal: ∃ n, n ≥ 10
  use 100
  norm_num
```

## Finding Lemmas: `exact?`, `apply?`, `rw?`

The most important automation tools are the *search tactics*:

- `exact?` — searches Mathlib for a lemma that exactly closes the current goal
- `apply?` — searches for a lemma that applies to reduce the current goal
- `rw?` — searches for a rewrite that makes progress

These are indispensable when exploring Mathlib. You write down a goal, type `exact?`, and Lean searches the entire library for something that works. It suggests terms like `Try this: exact Nat.add_comm n m`, which you can then copy into your proof.

The `#check` command queries types, and `#lookup` searches for names containing a pattern:

```lean
#check Nat.add_comm   -- Nat.add_comm : ∀ (n m : ℕ), n + m = m + n
example : ∀ n m : ℕ, n + m = m + n := Nat.add_comm
```

## Structuring Long Proofs

For proofs with multiple cases, Lean provides `·` (focused mode), `next`, `case`, and `show` to keep things organized.

```lean
theorem structured_proof (P Q R : Prop) (h : (P ∨ Q) ∧ R) : R ∧ (Q ∨ P) := by
  obtain ⟨hPQ, hR⟩ := h
  constructor
  · -- Prove R
    exact hR
  · -- Prove Q ∨ P
    cases hPQ with
    | inl hp => exact Or.inr hp
    | inr hq => exact Or.inl hq
```

`show P` changes the goal to `P` when `P` is definitionally equal to the current goal. It's a documentation tool — it makes the proof easier to read.

## A Complete Example: Proving `n + m = m + n` from First Principles

Here's a fully worked example proving commutativity of natural number addition, building from scratch:

```lean
-- First we need: 0 + n = n (zero on the left)
theorem zero_add : ∀ n : Nat, 0 + n = n := by
  intro n
  induction n with
  | zero      => rfl
  | succ k ih =>
    -- Goal: 0 + (k + 1) = k + 1
    -- 0 + (k + 1) = (0 + k) + 1  by def of addition
    rw [Nat.add_succ]   -- 0 + succ k = succ (0 + k)
    rw [ih]             -- succ (0 + k) = succ k  (by IH: 0 + k = k)

-- Then: succ m + n = succ (m + n)
theorem succ_add : ∀ m n : Nat, (m + 1) + n = (m + n) + 1 := by
  intros m n
  induction n with
  | zero      => rfl
  | succ k ih =>
    rw [Nat.add_succ, Nat.add_succ, ih]

-- Finally: commutativity
theorem add_comm : ∀ n m : Nat, n + m = m + n := by
  intros n m
  induction m with
  | zero      =>
    -- Goal: n + 0 = 0 + n
    rw [Nat.add_zero, zero_add]
  | succ k ih =>
    -- Goal: n + (k + 1) = (k + 1) + n
    rw [Nat.add_succ, succ_add, ih]
```

This is more work than just `exact Nat.add_comm n m` — but it shows the full structure of the inductive argument, which is exactly what you'd write in a traditional math proof.

## The Proof State as a Mental Model

The key mental shift for tactic-mode proving: you don't think "what is the proof?", you think "what is the simplest next step that makes progress?"

The workflow is:
1. Look at the goal.
2. Ask: is this provable by a single lemma? → `exact?` or `apply?`
3. Can I break it apart? → `constructor`, `cases`, `rcases`
4. Can I simplify it? → `simp`, `ring`, `linarith`, `omega`
5. Do I need to introduce hypotheses? → `intro`, `intros`
6. Is there a key intermediate step? → `have`

With practice, this becomes fast. Experienced Lean users can write proofs of moderate complexity in minutes, with the interactive proof state guiding every step.

## Tactic Combinators

Lean 4 has several combinator tactics for controlling flow:

- `try t` — runs `t`, ignores failure
- `first | t1 | t2 | t3` — tries each in order, uses first that succeeds
- `repeat t` — applies `t` repeatedly until failure
- `all_goals t` — applies `t` to all current goals
- `<;> t` — after the previous tactic creates multiple goals, applies `t` to all of them

```lean
theorem all_goals_example (P Q : Prop) (hp : P) (hq : Q) : P ∧ Q ∧ P := by
  refine ⟨?_, ?_, ?_⟩ <;> assumption
  -- <;> applies "assumption" to all three goals at once
```

`assumption` closes a goal if it's already a hypothesis in context.

## Tactic Writing: Lean 4 Metaprogramming

The tactics above are not built into the kernel — they are written in Lean 4 itself, using its metaprogramming system. You can extend the tactic language with your own tactics.

The simplest route is the macro system, which expands new syntax into existing tactics:

```lean
-- A simple custom tactic using macros
macro "myTactic" : tactic => `(tactic| simp; ring)

-- Use it
example (n : Nat) : n + 0 = n := by myTactic
```

For tactics that need to inspect the proof state, use the `elab` interface, which gives full programmatic access to goals and terms:

```lean
import Lean.Elab.Tactic

open Lean.Elab.Tactic

-- A tactic that tries `rfl` then `simp`
elab "try_rfl_then_simp" : tactic => do
  let goal ← getMainGoal
  try
    closeMainGoal (← mkAppM `rfl #[])
  catch _ =>
    evalTactic (← `(tactic| simp))
```

This macro-by-reflection design — where the object language and the meta-language are the same — is one of Lean 4's distinctive features. Writing a new tactic or decision procedure (for example, a tactic automating group-theory calculations, a normalization procedure for Σ/Π types, or an integration with an external SAT/SMT solver) is a genuine research-level project.

## Where This Connects to HoTT

In type-theoretic terms, every tactic is constructing a term:
- `intro h` is λ-abstraction
- `apply f` is function application (backward)
- `exact e` is providing the term directly
- `constructor` is the pair constructor `⟨_, _⟩`
- `cases h` is the eliminator for an inductive type
- `induction` is the recursion principle

The `by` block is syntactic sugar for a complex proof term. Lean elaborates it into the actual term that the kernel checks.

From a HoTT perspective, `rw` is using path induction (the J rule): `rw [h : a = b]` rewrites the goal using the path `h`, exactly as transport does. When you write `rw [Eq.symm h]`, you're using the inverse path `h⁻¹`.

The `calc` block is path composition: each `_ = _ := step` is a single path, and they're composed transitively via `Eq.trans`.

Tactic mode is thus a human-friendly interface to the same type-theoretic operations we've been doing abstractly throughout this book. The tactics are vocabulary; the proof state is the type-theoretic context; and the `by` elaborator translates your high-level proof sketch into a machine-verified proof term.
