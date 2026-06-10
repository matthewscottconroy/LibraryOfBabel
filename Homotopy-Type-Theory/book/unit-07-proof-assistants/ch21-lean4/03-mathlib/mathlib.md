# Mathlib: The World's Largest Library of Formalized Mathematics

In 1994, a group of mathematicians launched a project to formalize the proof of the Feit-Thompson theorem — the result that every finite group of odd order is solvable, originally proved by Feit and Thompson in a 255-page paper in 1963. The formalization took fifteen years and produced 150,000 lines of Coq code. It was a landmark achievement, but it was also a warning: formalization at scale was brutally expensive.

Mathlib changed the economics. By building a library that mathematicians could actually use — not just admire from a distance — Mathlib created a flywheel effect. Each new theorem added to Mathlib became a tool for proving the next theorem. Each new contributor who learned the library could build on the work of the contributors before them. By 2025, Mathlib4 contains over 150,000 theorems, spanning virtually every area of graduate mathematics, and it grows by hundreds of theorems per week.

This section teaches you to use it.

## Getting Mathlib

Add Mathlib to your project's `lakefile.toml`:

```toml
[[require]]
name = "mathlib"
from = git "https://github.com/leanprover-community/mathlib4"
rev = "v4.x.0"   -- pin to a specific version for reproducibility
```

Or create a project from the Mathlib template:

```bash
lake new my_project math
cd my_project
lake update
lake exe cache get   -- CRITICAL: download precompiled files
lake build
```

Then import Mathlib in your file:

```lean
import Mathlib   -- import everything (convenient for exploration)
-- or:
import Mathlib.Data.Int.Basic        -- just integers
import Mathlib.GroupTheory.Basic     -- just group theory
import Mathlib.Topology.Basic        -- just topology
```

The `import Mathlib` nuclear option is convenient when you're exploring, but production code should import only what's needed. Lean's module system handles dependencies automatically.

## The Naming Convention

The single most useful thing you can learn about Mathlib is its naming convention. Once you internalize it, you can often guess theorem names without searching.

The pattern: `Namespace.subject_verb_object`

- Namespaces: `Nat`, `Int`, `Real`, `Complex`, `List`, `Finset`, `Set`, `Group`, `Ring`, ...
- Verbs: `add`, `mul`, `sub`, `pow`, `neg`, `inv`, `le`, `lt`, `mem`, `card`, ...
- Adjectives: `comm` (commutative), `assoc` (associative), `zero`, `one`, `succ`, `pred`, ...

Examples:

```lean
-- Commutativity: thing_comm
#check Nat.add_comm     -- ∀ (n m : ℕ), n + m = m + n
#check Int.mul_comm     -- ∀ (a b : ℤ), a * b = b * a
#check Real.add_comm    -- ∀ (a b : ℝ), a + b = b + a

-- Associativity: thing_assoc
#check Nat.add_assoc    -- ∀ (n m k : ℕ), n + m + k = n + (m + k)
#check List.append_assoc -- ∀ (l₁ l₂ l₃ : List α), l₁ ++ l₂ ++ l₃ = l₁ ++ (l₂ ++ l₃)

-- Length / cardinality: length_thing or card_thing
#check List.length_append  -- length (l₁ ++ l₂) = length l₁ + length l₂
#check Finset.card_union_add_card_inter  -- |A ∪ B| + |A ∩ B| = |A| + |B|

-- Membership: mem_thing
#check List.mem_append   -- a ∈ l₁ ++ l₂ ↔ a ∈ l₁ ∨ a ∈ l₂
#check Set.mem_inter_iff -- x ∈ A ∩ B ↔ x ∈ A ∧ x ∈ B

-- Properties: thing_property
#check Nat.Prime         -- ℕ → Prop  (primality predicate)
#check Even              -- ∀ {α}, α → Prop
#check Odd               -- ∀ {α}, α → Prop
```

When you need a theorem about `X + Y`, think "what namespace is X in? What property do I want? What is the pattern?" Then try `#check X_namespace.add_property` and see if Lean finds it.

## Finding Theorems: The Four Methods

### Method 1: `exact?` and `apply?`

The most direct method: try `exact?` in tactic mode and let Lean search the library:

```lean
-- Prove that prime numbers are at least 2
example (p : Nat) (hp : Nat.Prime p) : p ≥ 2 := by
  exact?
  -- Lean suggests: Try this: exact Nat.Prime.two_le hp

-- Prove the Cauchy-Schwarz inequality (if it's in Mathlib)
example (a b c d : ℝ) : (a*c + b*d)^2 ≤ (a^2 + b^2) * (c^2 + d^2) := by
  nlinarith [sq_nonneg (a*d - b*c)]
  -- nlinarith is a nonlinear arithmetic solver
```

The `exact?` tactic searches all imported Mathlib declarations for a term that matches the current goal. When it finds one, it suggests `Try this: exact <name>`. Copy that suggestion into your proof.

### Method 2: Loogle — Web Search by Shape

Loogle (loogle.lean-lang.org) is a web search engine for Mathlib. You type a theorem *shape* — a pattern for what you're looking for — and it finds matches:

- Search `n + m = m + n` → finds `Nat.add_comm`, `Int.add_comm`, `add_comm`
- Search `_ ≤ _ → _ ≤ _ → _ ≤ _` (transitivity pattern) → finds `le_trans` variants
- Search `Nat.Prime _ → _` (implications from primality) → finds all primality consequences

Loogle understands Lean 4 syntax and does *semantic* matching, not string matching. It's invaluable when you know roughly what you need but not its exact name.

### Method 3: Moogle — Natural Language Search

Moogle (moogle.ai) accepts English descriptions and returns Mathlib results:

- "commutativity of addition for natural numbers"
- "every prime number is at least 2"
- "the Chinese Remainder Theorem"

Moogle uses a neural language model to match your description to theorem statements. It's less precise than Loogle but more accessible when you don't know the Lean 4 terminology.

### Method 4: The Mathlib Documentation

The official Mathlib4 documentation at `leanprover-community.github.io/mathlib4_docs` has every theorem, its statement, and links to its source and dependencies. The search is name-based but fast.

## Key Namespaces for HoTT Background

### Nat, Int, Rat, Real, Complex

The basic number types and their arithmetic:

```lean
-- Natural numbers
#check Nat.add_comm      -- ∀ n m : ℕ, n + m = m + n
#check Nat.div_add_mod   -- ∀ n k : ℕ, k * (n / k) + n % k = n
#check Nat.Prime.eq_one_or_self_of_dvd  -- if p is prime, d | p implies d = 1 or d = p

-- Integers (the fundamental group codomain)
#check Int.add_comm_group   -- (ℤ, +) is a commutative group
#check Int.units_eq_iff_abs_eq  -- units of ℤ are ±1

-- Integers mod n
#check ZMod           -- ZMod : ℕ → Type
#check ZMod.val_cast_of_lt  -- coercion properties

-- Reals
#check Real.sqrt      -- the square root function
#check Real.exp_add   -- exp(x+y) = exp(x) * exp(y)
```

### Group Theory

The group hierarchy, essential for understanding $\pi_1$:

```lean
-- Group hierarchy type classes
#check Semigroup    -- associative multiplication
#check Monoid       -- semigroup with identity
#check Group        -- monoid with inverses
#check CommGroup    -- commutative group
#check AddCommGroup -- same, with + notation

-- Key substructures
#check Subgroup     -- subgroup of a group
#check Subgroup.Normal  -- normal subgroup
#check QuotientGroup    -- G / N for normal N
#check MonoidHom        -- group homomorphisms

-- The fundamental theorem of group homomorphisms
#check QuotientGroup.quotientKerEquivRange
-- G ⧸ φ.ker ≃* φ.range   (first isomorphism theorem)

-- Free groups (for van Kampen)
#check FreeGroup        -- the free group on a type
#check FreeGroup.lift   -- universal property: maps out of free groups
#check FreeProduct      -- free product of groups G * H

-- Fundamental group (classical version)
#check FundamentalGroupoid  -- the Π₁ groupoid of paths in a space
```

A worked example — proving that group homomorphisms preserve inverses:

```lean
import Mathlib.GroupTheory.GroupAction.Basic

theorem hom_map_inv {G H : Type*} [Group G] [Group H] (φ : G →* H) (g : G) :
    φ g⁻¹ = (φ g)⁻¹ := by
  exact φ.map_inv g
  -- Or discover this with exact?:
  -- MonoidHom.map_inv : φ g⁻¹ = (φ g)⁻¹
```

### Topology

Point-set topology, the classical setting for homotopy:

```lean
-- The topological space type class
#check TopologicalSpace  -- the class of topological spaces
#check IsOpen            -- the predicate of open sets
#check ContinuousMap     -- the type X →C Y (continuous maps)
#check Homeomorph        -- homeomorphisms X ≃ₜ Y

-- Connectedness and path-connectedness
#check IsConnected       -- connected spaces
#check IsPathConnected   -- path-connected spaces
#check PathConnectedSpace -- type class for path-connected spaces

-- Homotopy
#check ContinuousMap.HomotopyRel  -- homotopy relative to a subset
#check HomotopyEquiv              -- homotopy equivalence

-- The fundamental group (classical)
#check FundamentalGroupoid.fundamentalGroup  -- π₁(X, x)
```

### Category Theory

The categorical infrastructure, essential for the homotopy hypothesis:

```lean
import Mathlib.CategoryTheory.Category.Basic
import Mathlib.CategoryTheory.Functor.Basic

-- Categories
#check CategoryTheory.Category      -- the type class
#check CategoryTheory.Functor       -- functors between categories
#check CategoryTheory.NatTrans      -- natural transformations
#check CategoryTheory.Iso           -- categorical isomorphisms
#check CategoryTheory.Equivalence   -- categorical equivalences

-- Limits and colimits
#check CategoryTheory.Limits.HasLimit    -- existence of limits
#check CategoryTheory.Limits.HasColimit  -- existence of colimits
#check CategoryTheory.Limits.Pushout     -- pushouts (van Kampen)

-- Simplicial sets
#check CategoryTheory.SimplicialObject
-- SimplicialObject C = (Δᵒᵖ ⥤ C)  the category of simplicial objects

-- Adjunctions
#check CategoryTheory.Adjunction.ofLeftRight

-- Topos theory
#check CategoryTheory.Topos  -- elementary toposes
```

A worked example — proving a functor preserves isomorphisms:

```lean
import Mathlib.CategoryTheory.Functor.Basic
import Mathlib.CategoryTheory.Iso

open CategoryTheory

theorem functor_maps_iso {C D : Type*} [Category C] [Category D]
    (F : C ⥤ D) {X Y : C} (e : X ≅ Y) : F.obj X ≅ F.obj Y where
  hom := F.map e.hom
  inv := F.map e.inv
  hom_inv_id := by
    rw [← F.map_comp, e.hom_inv_id, F.map_id]
  inv_hom_id := by
    rw [← F.map_comp, e.inv_hom_id, F.map_id]
-- Or just use the existing F.mapIso e
```

### Checking What `@` Means

Lean 4 implicitly inserts arguments. The `@` prefix makes them explicit:

```lean
-- Without @: implicit arguments are hidden
#check Nat.add_comm
-- Nat.add_comm : ∀ (n m : ℕ), n + m = m + n

-- With @: all arguments shown
#check @Nat.add_comm
-- Nat.add_comm : ∀ (n m : ℕ), n + m = m + n
-- (same here, but for universe-polymorphic functions, @ reveals universe args)

-- For the equality type:
#check @Eq
-- Eq : {α : Sort u_1} → α → α → Prop
-- Two implicit args: the universe u_1 and the type α; two explicit: a and b

#check @Eq.refl
-- Eq.refl : ∀ {α : Sort u_1} (a : α), a = a
```

The `@` trick is useful when you want to understand exactly what arguments a theorem takes, especially for universe-polymorphic theorems.

## What Mathlib Does Not Have

As of 2025, Mathlib does not have:

- Homotopy groups of spheres computed synthetically ($\pi_n(S^n) = \mathbb{Z}$ in the HoTT sense)
- Higher inductive types (requires HoTT foundations)
- Univalence as a computation rule (requires cubical foundations)
- Brunerie's theorem ($\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$) in computable form

These live in Cubical Agda. The division is clear:

| Mathlib has | Cubical Agda has |
|-------------|------------------|
| Group theory | Synthetic HoTT |
| Topological spaces | HITs: circle, suspension, spheres |
| Classical homotopy | Computational univalence |
| Category theory | Homotopy groups of spheres |
| Homological algebra | Brunerie's theorem |

Together they cover the curriculum. Separately they each have gaps. Use both.

## A Complete Example: Formalizing a Mathlib Search

Here is the workflow for a typical Mathlib formalization task: prove that the integers form a PID (principal ideal domain).

```lean
import Mathlib.RingTheory.PrincipalIdealDomain
import Mathlib.Data.Int.Basic

-- Step 1: Check if Mathlib has it
#check Int.isPrincipalIdealRing
-- Int.isPrincipalIdealRing : IsPrincipalIdealRing ℤ
-- Yes! It's there.

-- Step 2: Use it
example : IsPrincipalIdealRing ℤ := inferInstance
-- "inferInstance" finds the typeclass instance automatically

-- Step 3: Derive a consequence
-- Every ideal of ℤ is generated by a single element
example (I : Ideal ℤ) : ∃ n : ℤ, I = Ideal.span {n} := by
  obtain ⟨n, hn⟩ := (IsPrincipalIdealRing.principal I)
  exact ⟨n, hn.symm⟩
```

Three lines of searching, two lines of proof. This is what Mathlib makes possible: the theorem was already there, waiting. The formalization task reduced to finding it and applying it.

The Lean 4 workflow, at its best, is not about writing proofs from scratch. It is about knowing enough of the library to find what you need and combine it into what you want.
