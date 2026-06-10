# Formalization in Practice: From Sketch to Machine-Verified Proof

Formalizing mathematics is not transcription. You do not take an informal proof, translate it word for word into tactics, and receive a verified result. The process is more like surgery: you dissect the informal proof, identify its logical skeleton, discover what it assumed implicitly, search for the library lemmas that handle the routine steps, and reconstruct the argument in a form the type checker can verify.

The reward is not just confidence. When Georges Gonthier formalized the four-color theorem, he found errors in the informal proof — errors that had gone undetected for thirty years. Formalization is not just verification. It is a form of mathematical examination that finds what informal review cannot.

This section walks through a complete formalization project: formalizing the statement and proof that a group homomorphism's image is a subgroup, then building upward to the first isomorphism theorem. Along the way, we examine the workflow, the pitfalls, and what formalization reveals.

## The Formalization Workflow

The workflow for formalizing a mathematical result has five stages:

1. **State the theorem precisely.** Write the Lean 4 statement. This often forces you to be more precise than the informal statement required.

2. **Identify what Mathlib already has.** Use `exact?`, Loogle, and `#check` to find relevant lemmas. You want to reuse as much as possible.

3. **Identify the gaps.** What does Mathlib not have? What do you need to prove from scratch?

4. **Fill the gaps with tactics.** Write proofs for the missing pieces.

5. **Connect the pieces.** Combine library results and your proofs into the final theorem.

Let's see this in action.

## Example: The Image of a Group Homomorphism

The theorem: if $\phi : G \to H$ is a group homomorphism, then $\phi(G) = \{\phi(g) : g \in G\}$ is a subgroup of $H$.

A subgroup of $H$ must:
1. Contain the identity $1_H$
2. Be closed under multiplication
3. Be closed under inverses

### Stage 1: State the theorem

```lean
import Mathlib.GroupTheory.Subgroup.Basic
import Mathlib.GroupTheory.Hom.Group

variable {G H : Type*} [Group G] [Group H]

-- First, let's understand what "range" means in Mathlib
#check MonoidHom.range
-- MonoidHom.range : (G →* H) → Subgroup H
-- The range (image) of a homomorphism is already a subgroup!
-- Mathlib has this. But let's prove it from scratch to learn.
```

### Stage 2: The search

```lean
-- What do we need?
-- 1. φ(1) = 1 — map_one
#check MonoidHom.map_one   -- φ 1 = 1

-- 2. φ(g₁ * g₂) = φ(g₁) * φ(g₂) — definition of homomorphism
#check MonoidHom.map_mul   -- φ (g₁ * g₂) = φ g₁ * φ g₂

-- 3. φ(g⁻¹) = (φ g)⁻¹ — map_inv
#check MonoidHom.map_inv   -- φ g⁻¹ = (φ g)⁻¹
```

### Stage 3: The gaps

Mathlib already has `MonoidHom.range` as a `Subgroup`. Let's prove each subgroup property manually to see the argument:

```lean
-- The range as a set (not yet a Subgroup)
def φ_range (φ : G →* H) : Set H := {h | ∃ g : G, φ g = h}

-- Property 1: contains the identity
theorem range_one (φ : G →* H) : (1 : H) ∈ φ_range φ := by
  -- We need to show ∃ g : G, φ g = 1
  use 1          -- the witness: g = 1
  exact φ.map_one  -- φ(1) = 1

-- Property 2: closed under multiplication
theorem range_mul (φ : G →* H) {h₁ h₂ : H}
    (h₁_in : h₁ ∈ φ_range φ) (h₂_in : h₂ ∈ φ_range φ) :
    h₁ * h₂ ∈ φ_range φ := by
  -- h₁_in says ∃ g₁, φ g₁ = h₁; extract it
  obtain ⟨g₁, rfl⟩ := h₁_in   -- rfl: substitutes h₁ = φ g₁ everywhere
  obtain ⟨g₂, rfl⟩ := h₂_in   -- rfl: substitutes h₂ = φ g₂ everywhere
  -- Goal: φ g₁ * φ g₂ ∈ φ_range φ
  -- Use witness g₁ * g₂
  use g₁ * g₂
  exact φ.map_mul g₁ g₂  -- φ(g₁ * g₂) = φ g₁ * φ g₂

-- Property 3: closed under inverses
theorem range_inv (φ : G →* H) {h : H} (h_in : h ∈ φ_range φ) :
    h⁻¹ ∈ φ_range φ := by
  obtain ⟨g, rfl⟩ := h_in
  -- Goal: (φ g)⁻¹ ∈ φ_range φ
  use g⁻¹
  exact φ.map_inv g   -- φ(g⁻¹) = (φ g)⁻¹
```

### Stage 4: Connecting to Mathlib's `Subgroup`

Mathlib's `Subgroup H` is a bundled structure: it packages the set together with proofs of the subgroup properties. We can construct one:

```lean
-- Construct the range as a proper Subgroup
def myRange (φ : G →* H) : Subgroup H where
  carrier  := φ_range φ    -- the underlying set
  one_mem' := range_one φ
  mul_mem' := fun {h₁ h₂} h₁_in h₂_in => range_mul φ h₁_in h₂_in
  inv_mem' := fun {h} h_in => range_inv φ h_in

-- This should match Mathlib's version
example (φ : G →* H) : myRange φ = φ.range := by
  ext h        -- prove extensionality: same elements
  simp [myRange, φ_range, MonoidHom.mem_range]
```

## Pitfall: Definitional vs. Propositional Equality

One of the most common sources of confusion in Lean 4 formalization is the distinction between *definitional equality* (provable by `rfl`) and *propositional equality* (requires a proof step):

```lean
-- Definitional equality: Lean's type checker can verify this by reduction
example : 2 + 2 = 4 := rfl   -- True definitionally

-- Propositional equality: requires an argument
-- (even if it looks "obvious")
theorem not_obviously_refl (n : Nat) : n + 0 = n := by
  -- This is NOT definitionally true in all Lean 4 versions
  -- n + 0 reduces to n by the definition of addition: n + 0 = n
  -- But this depends on which argument addition recurses on
  simp   -- or: rw [Nat.add_zero]
```

The issue arises particularly with **recursive definitions**. Lean 4 defines `n + m` by recursion on `m`:
- `n + 0 = n` (base case) — definitionally true
- `n + (m + 1) = (n + m) + 1` — definitionally true

But `0 + n = n` is *not* definitionally true — it requires induction on `n`. The informal proof treats both as "obvious," but the formal proof must match the computational behavior of the definition.

**Universe issues.** Another common pitfall: universe polymorphism errors.

```lean
-- This fails with a universe error:
-- def bad : Type → Prop := fun A => A = A
-- Error: type mismatch: expected Prop, found Type

-- Fix: be explicit about what universe you want
def good : Type → Prop := fun A => A = A   -- wait, this might be OK...
-- Actually (A = A : Prop) since = lives in Prop
-- The issue is more subtle: Type equality lives in a higher universe
```

**Instance synthesis failures.** Lean 4's type class inference can fail silently:

```lean
-- If Lean can't find a Group instance, you get a confusing error
-- about missing fields or type mismatches
-- Always check: does Lean know your type has the structure you're using?
#check (inferInstance : Group ℤ)    -- Fails! ℤ is not a multiplicative group
#check (inferInstance : AddGroup ℤ) -- Succeeds! ℤ is an additive group
```

When you see `failed to synthesize instance`, the fix is usually:
1. Check if the instance exists with `#check (inferInstance : ClassName YourType)`
2. Check if you're using the right notation (additive vs. multiplicative)
3. Check if you need to import the right Mathlib file

## The First Isomorphism Theorem

Now a larger example: the first isomorphism theorem. For a group homomorphism $\phi : G \to H$:

$$G / \ker(\phi) \cong \phi(G)$$

```lean
import Mathlib.GroupTheory.QuotientGroup

variable {G H : Type*} [Group G] [Group H] (φ : G →* H)

-- Mathlib has this:
#check QuotientGroup.quotientKerEquivRange
-- quotientKerEquivRange : G ⧸ φ.ker ≃* φ.range

-- Let's use it:
theorem first_iso_theorem : G ⧸ φ.ker ≃* φ.range :=
  QuotientGroup.quotientKerEquivRange φ
```

One line. The theorem is in Mathlib. The formalization task reduced to finding it.

But what if we want to understand it? Let's prove a special case from scratch: if $\ker(\phi) = \{1\}$, then $\phi$ is injective.

```lean
-- Trivial kernel implies injective
theorem injective_of_trivial_ker {φ : G →* H}
    (hker : ∀ g : G, φ g = 1 → g = 1) : Function.Injective φ := by
  -- Injective means: φ g₁ = φ g₂ → g₁ = g₂
  intro g₁ g₂ h_eq
  -- From h_eq : φ g₁ = φ g₂, derive g₁ = g₂
  -- Key: φ g₁ = φ g₂ iff φ (g₁ * g₂⁻¹) = 1
  have h : g₁ * g₂⁻¹ = 1 := by
    apply hker
    -- Show φ (g₁ * g₂⁻¹) = 1
    rw [φ.map_mul, φ.map_inv, h_eq, mul_inv_cancel]
  -- g₁ * g₂⁻¹ = 1 implies g₁ = g₂
  exact mul_inv_eq_one.mp h
```

This proof is instructive: the informal argument says "if $\phi(g_1) = \phi(g_2)$, then $g_1 g_2^{-1} \in \ker(\phi)$, so $g_1 g_2^{-1} = 1$, so $g_1 = g_2$." The Lean proof makes this explicit, step by step.

## What Formalization Reveals

Here is what formalizing mathematics teaches you that informal work does not:

**Where the work actually is.** Informal proofs distribute effort unevenly. They spend paragraphs on conceptually interesting steps and a single word ("clearly") on technically difficult ones. Formalization forces you to spend time proportional to actual logical complexity. The "clearly" steps are often where the real work is.

**What is being assumed.** Every `import` is an assumption. When you write `ring`, you're assuming Lean's ring normalization algorithm is correct. When you use `Nat.add_comm`, you're assuming the Mathlib library's proof is correct. These are good assumptions — Mathlib's proofs are machine-checked — but they're assumptions nonetheless. Formalization makes them visible.

**The gap between classical and constructive.** Lean 4 includes classical logic (via `Classical.choice`). Many Mathlib proofs use `Classical.em` (law of excluded middle) or `Classical.choose` (the axiom of choice) without advertising it. When you formalize a proof, you can check which axioms it depends on:

```lean
#print axioms Nat.Prime.infinite  -- what axioms does this use?
-- [Classical.choice, propext, Quot.sound, funext]
-- These are the standard Lean 4 axioms, which include choice
```

**The difference between Lean 4 and HoTT.** Some things you want to formalize simply cannot be done in Lean 4. If you try to define the circle $S^1$ as a HIT with a non-trivial loop, you will fail — the type theory won't allow it. If you try to make two different proofs of the same proposition behave differently, you will fail — Lean 4 is proof-irrelevant. These failures are informative. They tell you that some mathematical content genuinely requires HoTT foundations, and for that content, Cubical Agda is the tool.

## Contributing to Mathlib

When you formalize something that isn't in Mathlib yet, you can contribute it. The process:

1. Find the right file in the Mathlib source tree (the directory structure mirrors the mathematical area)
2. Add your theorem, following the naming conventions
3. Write a docstring
4. Run `lake build` to check everything compiles
5. Run `scripts/lint-style.py` to check formatting
6. Open a pull request on GitHub

The Mathlib community reviews PRs quickly (often within days) and is generally welcoming of new contributors. The mathematical community's accumulated formalization work — all of it publicly available, all of it machine-checked — grows with every contribution.

This is the enterprise: not just verifying existing mathematics, but building a resource that did not previously exist. A library of formalized mathematics at the scale of Mathlib is something that has no analog in the history of the subject. Every theorem that enters it becomes permanently available, permanently checkable, and permanently composable with everything else.

The proof assistant is not just a verification tool. It is a new kind of mathematical infrastructure.
