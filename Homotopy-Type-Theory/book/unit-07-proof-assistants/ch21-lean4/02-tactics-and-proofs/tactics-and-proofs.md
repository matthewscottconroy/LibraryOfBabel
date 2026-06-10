# Tactics and Proofs: The Art of Guiding Lean to the Answer

There are two ways to write a proof in Lean 4. In *term mode*, you write the proof directly as a term: a lambda expression, a function application, a pair constructor. This is mathematics as programming — you give the proof object directly. In *tactic mode*, you describe the proof as a sequence of instructions — manipulations of a *proof state* — and Lean assembles the term from those instructions.

Both modes produce the same thing: a term that the kernel type-checks. Tactic mode is not magic. It is syntactic sugar for term construction, elaborated into explicit terms before the kernel ever sees them. But tactic mode is how most proofs are written, because it scales to complexity. Reading a proof state is easier than parsing a term; writing steps one at a time is easier than constructing the whole thing at once.

## The Proof State

When you enter tactic mode with `by`, Lean displays a *proof state* in the infoview. The proof state shows you:

1. **Hypotheses**: names and types of everything in the local context
2. **The goal**: the type you are trying to construct (the proposition you are trying to prove)

A proof state looks like:

```
n k : Nat
ih : k + 0 = k
⊢ k + 1 + 0 = k + 1
```

Here, `n`, `k`, and `ih` are in the context, and you need to prove `k + 1 + 0 = k + 1`. The `⊢` symbol (the *turnstile*) separates context from goal.

Every tactic transforms the proof state. Some tactics add hypotheses. Some tactics change the goal. Some tactics close the goal entirely (when the goal becomes provable immediately). A proof is complete when there are no remaining goals.

Reading the proof state is the central skill. Once you can read it, you know what to do next.

## Propositions and Types

Before tactics, a clarification. In Lean 4:

```lean
-- "theorem" is the same as "def" when the type is a Prop
theorem my_theorem (n : Nat) : n + 0 = n := by simp

-- "def" works for Prop-valued things too, but "theorem" signals intent
def my_theorem' (n : Nat) : n + 0 = n := by simp

-- The difference: "theorem" marks something as a proof (in Prop)
-- The kernel doesn't care which you use, but convention does
```

Use `theorem` (or `lemma`) for propositions, `def` for definitions. The distinction is documentation.

## Core Tactics

### `intro` — Introducing Hypotheses

When your goal is an implication `A → B` or a universal `∀ x : A, P x`, use `intro` to bring the assumption into the context:

```lean
theorem intro_example (P Q : Prop) : P → Q → P := by
  intro hp   -- goal was P → Q → P; now context has hp : P, goal is Q → P
  intro _hq  -- goal was Q → P; now context has _hq : Q, goal is P
  exact hp   -- goal is P, which is exactly hp

-- Multiple introductions at once
theorem intro_multi (P Q R : Prop) : P → Q → R → P ∧ Q := by
  intro hp hq _hr
  exact ⟨hp, hq⟩
```

The underscore prefix `_hq` tells Lean "I'm not using this hypothesis" and suppresses an unused-variable warning.

### `exact` — Providing the Proof Directly

`exact e` closes the goal by asserting that `e` has exactly the goal's type:

```lean
theorem exact_example (n : Nat) : n = n := exact rfl
-- Or in tactic mode:
theorem exact_tactic (n : Nat) : n = n := by exact rfl
-- Or even:
theorem exact_rfl (n : Nat) : n = n := by rfl
```

`exact?` — with a question mark — is the indispensable sibling: it searches Mathlib for a term that closes the current goal. Use it whenever you think "there must be a Mathlib lemma for this":

```lean
theorem add_comm_example (n m : Nat) : n + m = m + n := by exact?
-- Lean suggests: Try this: exact Nat.add_comm n m
```

### `apply` — Backward Reasoning

`apply f` when `f : A → B` and the goal is `B`. After applying, the goal becomes `A`. You're saying "I'll prove this using `f`, so now prove `f`'s premise":

```lean
theorem apply_example (P Q R : Prop) (hPQ : P → Q) (hQR : Q → R) (h : P) : R := by
  apply hQR    -- goal was R, now is Q (because hQR : Q → R)
  apply hPQ    -- goal was Q, now is P (because hPQ : P → Q)
  exact h      -- goal is P, which is h

-- apply works with multiple-argument functions too
-- For f : A → B → C and goal C, "apply f" creates goals A and B
```

`apply?` searches Mathlib for applicable lemmas, like `exact?` but allowing partial matches.

### `rw` — Rewriting with Equations

`rw [h]` rewrites the goal using an equation `h : a = b`, replacing `a` with `b`. `rw [← h]` rewrites right-to-left:

```lean
theorem rw_example (a b c : Nat) (h1 : a = b) (h2 : b = c) : a = c := by
  rw [h1]    -- goal: b = c  (replaced a with b using h1)
  rw [h2]    -- goal: c = c
  -- rfl closes it automatically after rw [h2]

-- rw on a hypothesis
theorem rw_hyp (a b c : Nat) (h1 : a = b) (h2 : a + 1 = c) : b + 1 = c := by
  rw [← h1] at h2   -- in h2, replace b with a (right-to-left)
  exact h2

-- Multiple rewrites at once
theorem rw_multi (a b c d : Nat) (h1 : a = b) (h2 : c = d) : a + c = b + d := by
  rw [h1, h2]
```

### `simp` — The General Simplifier

`simp` applies a collection of rewrite rules (marked `@[simp]`) repeatedly until nothing more simplifies. It's a workhorse:

```lean
theorem simp_examples : True := by
  -- These are all closed by simp:
  have h1 : ([] : List Nat) ++ [] = [] := by simp
  have h2 : Nat.succ 0 = 1 := by simp
  have h3 : [1, 2, 3].length = 3 := by simp
  trivial

-- simp with specific lemmas
theorem simp_with_lemma (n : Nat) (h : n = 5) : n + n = 10 := by
  simp [h]   -- uses h as an extra simp rule; simplifies n + n to 5 + 5 to 10

-- simp only: more controlled, only uses the listed lemmas
theorem simp_only_example (n : Nat) : n + 0 = n := by
  simp only [Nat.add_zero]   -- only uses Nat.add_zero, not the full simp set
```

### `ring` — Ring Arithmetic

`ring` proves equalities that hold in any commutative ring, by normalizing both sides:

```lean
theorem ring_examples : True := by
  have h1 : (3 : ℤ) * (4 + 2) = 18 := by ring
  have h2 : ∀ (a b : ℝ), (a + b)^2 = a^2 + 2*a*b + b^2 := by intro a b; ring
  have h3 : ∀ (a b c : ℚ), (a - b) * (a + b) = a^2 - b^2 := by intro a b c; ring
  trivial

-- ring for polynomial identities
theorem polynomial_identity (x y : ℝ) : (x + y)^3 = x^3 + 3*x^2*y + 3*x*y^2 + y^3 := by ring
```

### `linarith` and `omega` — Linear Arithmetic

`linarith` proves inequalities over ordered fields/rings; `omega` proves linear arithmetic over integers and naturals, including divisibility:

```lean
theorem linarith_examples (x y z : ℝ) (h1 : x > 0) (h2 : y > x) (h3 : z = y + 1) :
    z > 1 := by linarith

theorem omega_example (n : Nat) (h : n % 2 = 1) : n ≠ 0 := by omega

-- omega can handle divisibility conditions
theorem div_example (n : Nat) : ∃ q r, n = 2 * q + r ∧ r < 2 := by omega
```

### `induction` — Proof by Induction

`induction n` splits the proof into cases based on the constructors of `n`'s type:

```lean
theorem add_zero (n : Nat) : n + 0 = n := by
  induction n with
  | zero      => rfl       -- base case: 0 + 0 = 0
  | succ k ih =>           -- inductive case: k + 1 + 0 = k + 1
    rw [Nat.succ_add]      -- rewrites (k+1) + 0 to k + 0 + 1 ... wait, the other way
    -- Actually Nat.succ_add says: Nat.succ k + n = Nat.succ (k + n)
    -- Let's check and use simp
    simp [Nat.succ_add, ih]

-- List induction
theorem length_append (l₁ l₂ : List α) : (l₁ ++ l₂).length = l₁.length + l₂.length := by
  induction l₁ with
  | nil         => simp
  | cons x xs ih => simp [List.cons_append, ih]; ring
```

### `cases` and `rcases` — Case Analysis

`cases h` performs case analysis without induction (no inductive hypothesis). `rcases` is more powerful, handling nested patterns:

```lean
theorem or_comm (P Q : Prop) (h : P ∨ Q) : Q ∨ P := by
  cases h with
  | inl hp => exact Or.inr hp   -- h was P; conclude Q ∨ P from P on the right
  | inr hq => exact Or.inl hq   -- h was Q; conclude Q ∨ P from Q on the left

-- rcases handles nested destruction in one step
theorem and_or_example (P Q R : Prop) (h : P ∧ (Q ∨ R)) : (P ∧ Q) ∨ (P ∧ R) := by
  rcases h with ⟨hp, hq | hr⟩
  · exact Or.inl ⟨hp, hq⟩
  · exact Or.inr ⟨hp, hr⟩
```

### `constructor` and `use` — Building Structures

When your goal is a conjunction `P ∧ Q` or an existential `∃ x, P x`:

```lean
theorem constructor_example (P Q : Prop) (hp : P) (hq : Q) : P ∧ Q := by
  constructor
  · exact hp   -- prove P
  · exact hq   -- prove Q

theorem use_example : ∃ n : Nat, n > 100 ∧ n < 200 := by
  use 150
  constructor
  · norm_num   -- 150 > 100
  · norm_num   -- 150 < 200
```

### `contradiction` — Deriving Anything from False

`contradiction` closes any goal when the context contains contradictory hypotheses:

```lean
theorem contradiction_example (h1 : True) (h2 : False) : 42 = 17 := by
  contradiction   -- h2 : False means anything follows

theorem succ_ne_zero (n : Nat) : Nat.succ n ≠ 0 := by
  intro h         -- assume Nat.succ n = 0
  contradiction   -- this is structurally impossible; Lean knows it
```

## The `calc` Block: Equational Chains

For multi-step equational arguments:

```lean
theorem calc_example (a b c : ℤ) (h1 : a = b + 3) (h2 : b = c - 1) : a = c + 2 := by
  calc a = b + 3       := h1
    _ = (c - 1) + 3    := by rw [h2]
    _ = c + 2          := by ring

-- calc works for inequalities too
theorem calc_ineq (x y : ℝ) (hx : x ≥ 0) (hy : y ≥ 2) : x + y ≥ 2 := by
  calc x + y ≥ 0 + 2   := by linarith
    _ = 2               := by ring
```

Each step `_ = next := proof` must produce a path that the current relation can follow. The `_` on the left refers to the right-hand side of the previous step.

## Automation Tactics

### `decide` — Decidable Propositions

`decide` proves propositions that can be computed:

```lean
example : Nat.Prime 17 := by decide
example : ¬ (2 + 2 = 5) := by decide
example : [1, 2, 3].length = 3 := by decide
```

`decide` is slow for large numbers (it's computing), but perfect for small finite checks.

### `norm_num` — Numerical Goals

`norm_num` handles numerical goals more efficiently than `decide`:

```lean
example : (1 : ℝ) / 3 + 1 / 6 = 1 / 2 := by norm_num
example : Nat.Prime 1000003 := by norm_num
example : (2 : ℝ) ^ 10 = 1024 := by norm_num
```

### `aesop` and `tauto` — General Automation

`aesop` combines search, simp, and case analysis for goals that don't fit a single tactic. `tauto` proves propositional tautologies:

```lean
-- tauto handles pure propositional logic
theorem tauto_example (P Q R : Prop) : (P → Q) → (Q → R) → (P → R) := by tauto

-- aesop handles more complex goals
theorem aesop_example (l : List Nat) : [] ++ l = l := by aesop
```

### `push_neg` and `contrapose` — Negation

For goals involving negations:

```lean
-- push_neg moves negations inward
theorem push_neg_example : ¬ ∀ n : Nat, n < 10 := by
  push_neg         -- goal: ∃ n, n ≥ 10
  use 100

-- contrapose switches to the contrapositive
theorem contrapose_example (n : Nat) : n ≠ 0 → Nat.succ n ≠ 1 := by
  contrapose!      -- assumes ¬(Nat.succ n ≠ 1), proves ¬(n ≠ 0)
  simp
```

## Structuring Long Proofs

For complex proofs with many subgoals, Lean 4 provides structure:

```lean
-- "·" focuses on one subgoal
theorem focused_example (P Q R : Prop) (hp : P) (hq : Q) (hr : R) : P ∧ Q ∧ R := by
  refine ⟨?_, ?_, ?_⟩
  · exact hp
  · exact hq
  · exact hr

-- "have" introduces intermediate results
theorem have_example (n : Nat) : n^2 + n = n * (n + 1) := by
  have h : n * (n + 1) = n^2 + n := by ring
  linarith

-- "show" changes the goal to a definitionally equal form (for readability)
theorem show_example (n : Nat) : 2 * n = n + n := by
  show n + n = n + n  -- 2 * n is definitionally n + n in Lean 4
  rfl

-- "<;>" applies a tactic to all goals at once
theorem all_goals_example (P Q : Prop) (hp : P) (hq : Q) : P ∧ Q := by
  constructor <;> assumption
```

## A Complete Worked Example

Here is a full proof of commutativity of natural number addition, built from scratch:

```lean
-- Step 1: 0 + n = n
theorem zero_add : ∀ n : Nat, 0 + n = n := by
  intro n
  induction n with
  | zero      => rfl
  | succ k ih =>
    -- Goal: 0 + (k + 1) = k + 1
    -- Lean defines n + (k+1) as (n + k) + 1
    rw [Nat.add_succ]   -- 0 + succ k = succ (0 + k)
    rw [ih]             -- succ (0 + k) = succ k

-- Step 2: (n + 1) + m = (n + m) + 1
theorem succ_add : ∀ n m : Nat, (n + 1) + m = (n + m) + 1 := by
  intros n m
  induction m with
  | zero      => rfl
  | succ k ih =>
    rw [Nat.add_succ, Nat.add_succ, ih]

-- Step 3: commutativity
theorem add_comm' : ∀ n m : Nat, n + m = m + n := by
  intros n m
  induction m with
  | zero      =>
    rw [Nat.add_zero, zero_add]
  | succ k ih =>
    rw [Nat.add_succ, succ_add, ih]
```

Each step is explicit, each transformation is justified, and the result is machine-checked. This is the gold standard: a proof that cannot have hidden gaps, because there is nowhere for them to hide.

## Tactics as Type Theory

Every tactic is a shorthand for a term-level operation:

| Tactic | Term-level meaning |
|--------|--------------------|
| `intro h` | Lambda abstraction: `λ h => ...` |
| `exact e` | Direct term: `e` |
| `apply f` | Function application: `f _` where `_` is the new goal |
| `constructor` | Pair constructor: `⟨_, _⟩` |
| `cases h` | Pattern matching eliminator |
| `induction n` | Recursor / induction principle |
| `rw [h]` | Transport / substitution: `h ▸ _` |
| `calc` | Transitivity chain: `a.trans b |>.trans c` |

From a HoTT perspective, `rw [h : a = b]` is using path induction (the J rule): you transport the goal from `a` to `b` along the path `h`. When you write `rw [Eq.symm h]`, you're using the inverse path `h⁻¹`. The `calc` block is composition of paths: each step is a single path, chained transitively.

Tactic mode is thus not a separate formalism from HoTT. It is a human-friendly interface to the same type-theoretic operations. The tactics are vocabulary; the proof state is the type context; and the `by` elaborator translates your high-level steps into the low-level proof term that the kernel checks.

The next section shows you how to find the right lemmas in Mathlib — so that your tactics have the tools they need.
