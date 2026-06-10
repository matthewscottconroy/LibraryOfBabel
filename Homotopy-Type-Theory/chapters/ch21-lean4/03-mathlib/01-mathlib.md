# 3.1 Mathlib — Formal Mathematics at Scale

## What Is Mathlib?

Mathlib4 (Mathlib for Lean 4) is the largest formalized mathematics library in existence. As of 2025, it contains over 150,000 theorems spanning undergraduate and graduate mathematics across nearly every area: number theory, algebra, analysis, topology, category theory, algebraic geometry, probability, and more.

This is not just "a bunch of proofs." Mathlib is a coherent, carefully organized library where:
- Every definition is precise and machine-checkable
- Every theorem is proven from axioms
- Everything is searchable and composable
- The library builds on itself: high-level theorems use lower-level ones

For our purposes, Mathlib contains most of the mathematical background for HoTT: group theory, topological spaces, simplicial sets, category theory, homology. It's the formalized version of everything in the non-HoTT chapters of our curriculum.

## Getting Mathlib

Adding Mathlib to a Lean project is done via `lake`:

```toml
-- In lakefile.toml:
[[require]]
name = "mathlib"
from = git "https://github.com/leanprover-community/mathlib4"
rev = "v4.X.0"  -- pin to a specific version
```

Or create a project from the Mathlib template:

```bash
lake new my_project math
```

Mathlib is large — the first build takes 20-30 minutes. After that, Lean caches the compiled `.olean` files and subsequent builds are fast.

## Navigating Mathlib

The challenge with Mathlib isn't that theorems don't exist — it's finding them.

### The naming convention

Mathlib uses a systematic naming convention that, once learned, lets you guess lemma names:

```
<namespace>.<thing>_<property>
```

Examples:
- `Nat.add_comm` — commutativity of `+` in `Nat`
- `List.length_append` — length of a concatenated list
- `Finset.card_union_add_card_inter` — inclusion-exclusion for finite sets
- `Subgroup.mem_coset_iff` — membership in a coset

The convention: types/namespaces in `CamelCase`, lemma names in `snake_case`. Operations are abbreviated: `comm` for commutative, `assoc` for associative, `mem` for membership, `card` for cardinality.

### Search tools

**`exact?`** — the single most useful tool. Type this in a `by` block and Lean searches all of Mathlib for a term that closes your goal.

**`apply?`** — searches for lemmas that apply to reduce your goal.

**`rw?`** — searches for rewrites that make progress.

**`#check`** — queries the type of a term:
```lean
#check Nat.Prime          -- Nat.Prime : ℕ → Prop
#check List.map_append    -- List.map_append : ∀ ... l₁ l₂, map f (l₁ ++ l₂) = map f l₁ ++ map f l₂
```

**`#lookup` and `example?`** — search by keyword:
```lean
#lookup "prime"    -- shows all Mathlib names containing "prime"
```

**Loogle** (loogle.lean-lang.org) — a web search engine for Mathlib theorems. Search by conclusion shape: type in `n + m = m + n` and it finds `Nat.add_comm`.

**Moogle** (moogle.ai) — semantic search using natural language: "commutativity of addition in natural numbers."

### The Mathlib documentation

`leanprover-community.github.io/mathlib4_docs` has the full API documentation with every theorem, its statement, and a link to the source. Searchable by name.

## Key Areas Relevant to HoTT

### Algebra

Mathlib's algebra hierarchy is built on type classes:

```lean
-- The group hierarchy
class Semigroup (G : Type*) extends Mul G where
  mul_assoc : ∀ a b c : G, a * b * c = a * (b * c)

class Monoid (M : Type*) extends Semigroup M, One M where
  mul_one : ∀ a : M, a * 1 = a
  one_mul : ∀ a : M, 1 * a = a

class Group (G : Type*) extends Monoid G, Inv G where
  inv_mul_cancel : ∀ a : G, a⁻¹ * a = 1

class CommGroup (G : Type*) extends Group G, CommMonoid G
```

These form a hierarchy: every `CommGroup` is a `Group` is a `Monoid` is a `Semigroup`.

For HoTT: the fundamental group $\pi_1(X)$ is a `Group`. The homology groups $H_n(X)$ are `CommGroup`s. Everything about these algebraic objects is in Mathlib.

Key namespaces:
- `Group`, `Subgroup`, `GroupHom`, `QuotientGroup`
- `Ring`, `CommRing`, `Ideal`, `QuotientRing`
- `Module`, `LinearMap`, `TensorProduct`
- `Algebra`, `AlgebraHom`

**Free groups.** The free group $F(S)$ on a set $S$ is `FreeGroup S` in Mathlib:

```lean
#check FreeGroup           -- FreeGroup : Type → Type
#check FreeGroup.lift      -- The universal property of the free group
-- FreeGroup.lift : (α → G) → FreeGroup α →* G
```

The free product $G * H$ is `FreeProduct G H` — directly relevant to van Kampen.

**Fundamental groups.** Mathlib has `FundamentalGroupoid` for topological spaces:

```lean
#check FundamentalGroupoid  -- The Π₁ groupoid of paths
```

### Topology

Mathlib has point-set topology fully formalized:

```lean
-- Topological spaces
class TopologicalSpace (α : Type*) where
  IsOpen : Set α → Prop
  -- ... axioms

-- Key theorems
#check IsConnected          -- connectedness
#check PathConnected        -- path-connectedness
#check IsCompact            -- compactness
#check ContinuousMap        -- the type of continuous maps
```

For HoTT background:
- `ContinuousMap X Y` is the type of continuous maps — the topological analogue of function types
- `Homeomorph X Y` is a homeomorphism — the topological analogue of equivalences
- `FiberBundle` formalizes fiber bundles — the classical version of fibrations

**Homotopy.** Mathlib has homotopy theory, though less developed than algebraic topology:

```lean
#check Continuous.homotopy  -- A homotopy between continuous maps
#check HomotopyEquiv         -- A homotopy equivalence
```

### Category Theory

Mathlib has a substantial category theory library, which is foundational for understanding Kan complexes, $\infty$-categories, and the homotopy hypothesis:

```lean
-- Category
class CategoryStruct (obj : Type u) where
  Hom : obj → obj → Type v  -- morphism type
  id  : ∀ X, Hom X X
  comp : Hom X Y → Hom Y Z → Hom X Z

-- Functor
structure Functor (C : Type u₁) [Category.{v₁} C] (D : Type u₂) [Category.{v₂} D] where
  obj : C → D
  map : ∀ {X Y : C}, (X ⟶ Y) → (obj X ⟶ obj Y)
  -- ... functoriality laws

-- Natural transformation
structure NatTrans (F G : C ⥤ D) where
  app : ∀ X : C, F.obj X ⟶ G.obj X
  naturality : ∀ {X Y} (f : X ⟶ Y), F.map f ≫ app Y = app X ≫ G.map f
```

Key modules: `CategoryTheory.Category`, `CategoryTheory.Functor`, `CategoryTheory.NatTrans`, `CategoryTheory.Equivalence`, `CategoryTheory.Limits` (limits and colimits), `CategoryTheory.Adjunction`.

**The topos library.** Mathlib has `CategoryTheory.Topos` — elementary toposes, including `Grothendieck.Topos`. This is the classical setting in which the homotopy hypothesis lives.

**Simplicial objects.** `CategoryTheory.SimplicialObject` is the category $[\Delta^{op}, \mathcal{C}]$ — directly relevant to simplicial sets and the model structure for homotopy theory.

### Homological Algebra

For algebraic topology backgrounds:

```lean
#check ChainComplex          -- Chain complexes
#check HomologicalComplex    -- The general version
#check CategoryTheory.ShortExact  -- Short exact sequences
```

Mathlib has:
- Long exact sequences
- Snake lemma
- Five lemma
- Derived functors (Ext, Tor)

These are the algebraic machinery behind homology and cohomology.

### Number Theory

For background on $\pi_1(S^1) = \mathbb{Z}$ and the integers:

```lean
#check Int               -- The integers ℤ
#check ZMod              -- ℤ/nℤ
#check Int.add_comm_group  -- (ℤ, +) is a commutative group
```

## Using Mathlib to Formalize HoTT Background

Here's how you'd use Mathlib to formalize the algebraic background needed for a HoTT proof.

**Example: Free product for van Kampen.** The van Kampen theorem says $\pi_1(A \sqcup_C B) = \pi_1(A) *_{\pi_1(C)} \pi_1(B)$. In Mathlib:

```lean
import Mathlib.GroupTheory.FreeProduct

-- The amalgamated free product is AmalgamatedProduct or FreeProduct.{G₁ ← H → G₂}
#check FreeProduct  -- FreeProduct : ∀ {ι : Type}, (ι → Type) → Type

-- Free product of groups given by index
-- FreeProduct.lift : (∀ i, G i →* M) → FreeProduct G →* M
-- The universal property: maps out of the free product are collections of maps
```

**Example: The integers as a group.** For encoding $\pi_1(S^1)$:

```lean
import Mathlib.Algebra.Group.Basic
import Mathlib.Data.Int.Basic

-- ℤ as an additive group
#check (inferInstance : AddCommGroup ℤ)

-- Successor as a group automorphism
def succ_auto : ℤ ≃+ ℤ := { -- additive group automorphism
  toFun    := (· + 1),
  invFun   := (· - 1),
  left_inv := fun n => by ring,
  right_inv := fun n => by ring,
  map_add'  := fun n m => by ring
}
```

## What Mathlib Does NOT Have (Yet)

As of 2025, Mathlib does not have:
- Homotopy groups of spheres ($\pi_n(S^n)$, $\pi_3(S^2)$)
- Higher inductive types (these require HoTT foundations, not available in Lean 4)
- Synthetic homotopy theory (same reason)
- Full computations of $H^*(K(\mathbb{Z}, n))$ or other HoTT-specific constructions

These live in Cubical Agda's HoTT library, not Mathlib. The division of labor is:
- **Mathlib**: classical background (groups, topological spaces, category theory)
- **HoTT/Cubical**: synthetic HoTT content

## Importing and Using Mathlib

To use a Mathlib theorem, import the relevant file:

```lean
import Mathlib.Algebra.Group.Basic   -- basic group theory
import Mathlib.Data.Int.Basic        -- integers
import Mathlib.Topology.Basic        -- point-set topology
import Mathlib.CategoryTheory.Category.Basic  -- category theory
```

Or import everything (slow but convenient during exploration):

```lean
import Mathlib
```

Then use namespace qualifiers:

```lean
-- Using the Int.add_comm_group instance:
example : (7 : ℤ) + (-7) = 0 := by
  exact add_neg_cancel 7

-- Using Nat.Prime:
#check Nat.Prime  -- : ℕ → Prop
example : Nat.Prime 17 := by decide
```

## Tips for Mathlib Exploration

**1. Start with `import Mathlib` and `exact?`.** When learning a new area, use the nuclear option — import everything, try to prove your goal, let `exact?` tell you what lemma Lean found. Then you can narrow down the import later.

**2. Learn the naming pattern.** `thing_comm` is commutativity, `thing_assoc` is associativity, `thing_add_thing` is something about addition. `Nat.*` is for naturals, `Int.*` for integers, `Real.*` for reals.

**3. Read the Mathlib docs.** The online documentation at `leanprover-community.github.io/mathlib4_docs` has everything. Use the search.

**4. Use `#check` aggressively.** Any time you're curious about a type or theorem, `#check` it. Lean will tell you exactly what it is.

**5. Break goals down.** If `simp` or `ring` doesn't close a goal, break it into smaller pieces with `have` and handle each piece.

**6. The Mathlib Zulip.** The Mathlib community on Zulip (leanprover.zulipchat.com) is active and helpful. If you can't find a theorem, ask there — the answer is usually either "it's called X" or "it doesn't exist yet, want to contribute it?"

Mathlib is the single most powerful tool for formalizing classical mathematics. For our HoTT curriculum, it provides the algebraic, topological, and categorical background that HoTT rests on — and it does so with the full precision of machine-checked proofs.
