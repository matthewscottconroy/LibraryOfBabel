# Lean 4 Basics: Terms, Types, and the Shape of a Proof

Every proof assistant begins with a question: what is the primitive notion? For Lean 4, the answer is the *type*. Every object in Lean 4 — every number, every function, every proposition, every proof — has a type, and the type determines what you can do with the object. This is not a limitation. It is the source of all the system's power.

Before any of that becomes meaningful, you need a running Lean 4 installation. Let's start there.

## Installation via elan

The recommended way to install Lean 4 is through `elan`, Lean's version manager (analogous to `rustup` for Rust):

```bash
# Install elan — the Lean version manager
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh

# Verify the installation
lean --version   # Should print something like: Lean (version 4.x.0, ...)
lake --version   # Lake is Lean's build system
```

After elan installs, the `lean` and `lake` commands are available in your shell. `lean` is the Lean 4 compiler and type checker; `lake` is the build system and package manager.

### Creating a project with Mathlib

For most work in this curriculum, you want Mathlib available:

```bash
# Create a new project with the Mathlib template
lake new my_hott_project math
cd my_hott_project

# Download and cache Mathlib (first time: 20-30 minutes)
lake update
lake exe cache get   # Fetch precompiled .olean files (saves hours of build time)
lake build
```

The `lake exe cache get` command is crucial. Mathlib has tens of thousands of files, and building from source takes hours. The cache downloads precompiled object files.

Your project now has a `lakefile.toml` that declares the Mathlib dependency, and a `MyHottProject.lean` file where you write code.

### VS Code setup

Install the `lean4` extension from the VS Code marketplace. Once installed:
- Open a `.lean` file
- The extension connects to the Lean server
- A *Lean Infoview* panel appears, showing goals, errors, and type information in real time

The infoview is not optional — it is the primary way you interact with Lean 4. As you move your cursor through a proof, the infoview updates to show you the current proof state. This real-time feedback is what makes interactive theorem proving tractable.

## Your First Lean 4 File

Create a file `Basics.lean` and type:

```lean
-- This is a comment

-- #check queries the type of an expression
#check Nat          -- Nat : Type
#check 42           -- 42 : ℕ
#check (1 + 1 = 2)  -- 1 + 1 = 2 : Prop
#check Type         -- Type : Type 1
#check Prop         -- Prop : Type

-- #eval evaluates an expression
#eval 2 + 2         -- 4
#eval [1, 2, 3].length  -- 3

-- #reduce reduces a term to normal form (more detailed than #eval)
#reduce 2 + 2       -- 4
#reduce Nat.succ (Nat.succ Nat.zero)  -- 2
```

Each `#check`, `#eval`, and `#reduce` command is a query to the Lean 4 kernel. Hover your cursor over any of them in VS Code and the infoview shows the answer. This exploratory mode — type a question, get an answer — is how you learn Lean 4.

## The Universe Hierarchy

Everything in Lean 4 has a type. But what is the type of `Nat`? And what is the type of *that* type?

Lean 4 resolves this with a hierarchy of *universes*:

```lean
-- Nat is a type, and it lives in the universe Type (= Type 0)
#check Nat      -- Nat : Type

-- Propositions live in Prop, which is a sort
#check (1 + 1 = 2)   -- 1 + 1 = 2 : Prop
#check Prop           -- Prop : Type

-- Type itself lives in Type 1 (the next universe up)
#check Type           -- Type : Type 1
#check Type 1         -- Type 1 : Type 2

-- Universe-polymorphic definitions work at any level
-- using Sort u (= Type u for u ≥ 1, = Prop for u = 0)
variable {α : Type*}  -- Type* means "some universe Type u"
```

The hierarchy `Prop : Type : Type 1 : Type 2 : ...` is infinite, and each level contains the previous. This resolves Russell's paradox: `Type` does not contain itself, because `Type : Type 1` and `Type 1 : Type 2`, and so on.

**The `Prop`/`Type` distinction.** This is the most important conceptual divide in Lean 4:

- `Prop` is the universe of *propositions*: types whose elements are proofs, and where any two proofs are definitionally equal. If you have `h₁ h₂ : P` where `P : Prop`, then `h₁ = h₂` holds definitionally. Proof irrelevance is built in.
- `Type` (and `Type 1`, etc.) is the universe of *data types*: types where elements can genuinely differ, and equality is a non-trivial proposition.

In HoTT terms: `Prop` corresponds to h-level $-1$ (propositions, at most one element up to homotopy). `Type` corresponds to arbitrary types with no h-level constraint. The `Prop`/`Type` distinction is Lean 4's way of handling the fact that classical mathematics wants propositions to be proof-irrelevant, while general type theory does not.

## Terms and Types

Every term has a type; every type is a term. Here are the core examples:

```lean
-- Nat: the natural numbers
-- Elements: 0, 1, 2, ... (built from Nat.zero and Nat.succ)
#check (0 : Nat)       -- 0 : ℕ
#check (42 : Nat)      -- 42 : ℕ
#check Nat.succ 0      -- Nat.succ 0 : ℕ  (this is 1)

-- Function types: α → β
-- The type of functions from α to β
#check (Nat.succ : Nat → Nat)     -- Nat.succ : ℕ → ℕ
#check (fun n => n + 1 : Nat → Nat)  -- fun n => n + 1 : ℕ → ℕ

-- Anonymous functions (lambda abstractions)
#eval (fun n => n * n) 7    -- 49

-- Let bindings
#eval
  let x := 5
  let y := x * x
  y + 1         -- 26

-- Pattern matching with match
def isZero : Nat → Bool
  | 0     => true
  | _ + 1 => false

#eval isZero 0   -- true
#eval isZero 5   -- false
```

## Basic Definitions

The `def` keyword introduces a new definition:

```lean
-- A simple function
def double (n : Nat) : Nat := n * 2

-- With pattern matching
def factorial : Nat → Nat
  | 0     => 1
  | n + 1 => (n + 1) * factorial n

-- With a docstring (documentation comment)
/-- `fib n` computes the n-th Fibonacci number. -/
def fib : Nat → Nat
  | 0     => 0
  | 1     => 1
  | n + 2 => fib n + fib (n + 1)

#eval factorial 10   -- 3628800
#eval fib 10        -- 55
```

The type annotation `: Nat → Nat` is optional when Lean can infer it, but it's good practice to include it. Types are documentation.

## Dependent Types

The essential innovation of Lean 4 — and all modern proof assistants — is *dependent types*: types that depend on values.

```lean
-- A dependent function type: Π (n : Nat), Vec Nat n
-- The return type (Vec Nat n) depends on the argument value n
-- This is written (n : Nat) → Vec Nat n in Lean 4

-- A simple dependent type: a function returning a proof
def evenDoubled (n : Nat) : n + n = 2 * n := by ring
-- The type (n + n = 2 * n) depends on n!

-- Vectors: lists whose length is tracked in the type
inductive Vec (α : Type) : Nat → Type where
  | nil  : Vec α 0
  | cons : α → Vec α n → Vec α (n + 1)

-- A length-indexed head function (safe! no out-of-bounds)
def head {α : Type} : Vec α (n + 1) → α
  | .cons x _ => x

-- The type (Vec α (n+1)) guarantees the vector is non-empty

-- Σ-types: dependent pairs
-- Σ (n : Nat), Vec Nat n  is the type of "vectors of some length"
-- Written as (n : Nat) × Vec Nat n in Lean 4
example : (n : Nat) × Vec Nat n :=
  ⟨3, .cons 1 (.cons 2 (.cons 3 .nil))⟩
```

The key point: in a dependent type, the type of one component can mention the value of another. This is what allows types to express properties (like "this vector has length n") rather than just shapes.

## The Identity Type

The identity type is the heart of both Lean 4 and HoTT:

```lean
-- The identity type: a = b is the type of proofs that a equals b
-- In Lean 4, this is a Prop
#check @Eq         -- Eq : {α : Sort u} → α → α → Prop
#check @Eq.refl    -- Eq.refl : ∀ {α : Sort u} (a : α), a = a

-- The unique constructor: reflexivity
example : 2 + 2 = 4 := rfl    -- rfl : a = a, works when both sides reduce to the same term
example : "hello" = "hello" := rfl

-- Derived operations (from the rules of type theory)
example (a b c : Nat) (h₁ : a = b) (h₂ : b = c) : a = c :=
  h₁.trans h₂   -- transitivity

example (a b : Nat) (h : a = b) : b = a :=
  h.symm         -- symmetry

-- Using an equation: if h : a = b, then anything true of a is true of b
example (n : Nat) (h : n = 5) : n + 1 = 6 := by rw [h]  -- rewrite using h
```

In Lean 4, `a = b : Prop`, which means all proofs of `a = b` are definitionally equal. This is the K axiom. In HoTT, `a = b` is a type in `Type`, and different proofs can be genuinely different paths. The distinction is foundational: Lean 4 is built for classical mathematics; Cubical Agda (Chapter 22) is built for HoTT.

## Inductive Types

Lean 4 defines data types inductively — by listing their constructors:

```lean
-- Natural numbers (already in the standard library)
inductive MyNat : Type where
  | zero : MyNat
  | succ : MyNat → MyNat

-- Lists
inductive MyList (α : Type) : Type where
  | nil  : MyList α
  | cons : α → MyList α → MyList α

-- Binary trees
inductive Tree (α : Type) : Type where
  | leaf : Tree α
  | node : Tree α → α → Tree α → Tree α

-- Sum type (disjoint union)
inductive Sum (α β : Type) : Type where
  | inl : α → Sum α β
  | inr : β → Sum α β

-- The empty type (no constructors — nothing inhabits it)
inductive Empty : Type

-- Functions on inductive types use pattern matching
def Tree.size : Tree α → Nat
  | .leaf        => 0
  | .node l _ r  => 1 + l.size + r.size

def Tree.mirror : Tree α → Tree α
  | .leaf        => .leaf
  | .node l x r  => .node (r.mirror) x (l.mirror)
```

Lean 4's pattern matching is exhaustive and termination-checked (by default). If you forget a case, Lean reports an error. If you write a recursive function that might not terminate, Lean reports an error. These are features, not bugs: they ensure that every definition in Lean 4 is total and well-founded.

## Structures and Type Classes

Lean 4's mathematical infrastructure uses *structures* (record types) and *type classes* (interfaces):

```lean
-- A structure: a named record type
structure Point where
  x : Float
  y : Float

def origin : Point := { x := 0.0, y := 0.0 }
def distFromOrigin (p : Point) : Float :=
  Float.sqrt (p.x * p.x + p.y * p.y)

-- A type class: an interface that types can implement
class Describable (α : Type) where
  describe : α → String

instance : Describable Nat where
  describe n := s!"The natural number {n}"

instance : Describable Bool where
  describe b := if b then "true" else "false"

-- Lean infers which instance to use based on the type
#eval Describable.describe (42 : Nat)   -- "The natural number 42"
#eval Describable.describe true          -- "true"
```

The type class system is what makes Lean 4's mathematical hierarchy work. When you write `a + b` for `a b : ℤ`, Lean looks up the `Add ℤ` instance, which provides the `+` operation for integers. When you write `a * b + c = a * b + c` and use `ring`, Lean looks up the `CommRing ℤ` instance to know the algebraic laws.

## The Hierarchy: Prop, Type 0, Type 1

To consolidate the picture: the full universe hierarchy in Lean 4 is:

```
Prop  (propositions, proof-irrelevant)
 |
Type  = Type 0  (small data types, like Nat, Bool, List)
 |
Type 1  (types whose elements are small types, like Type)
 |
Type 2  (types whose elements are Type 1 types)
...
```

`Sort u` is the general universe at level `u`, where `Sort 0 = Prop` and `Sort (u+1) = Type u`. Most mathematical definitions live in `Type` or `Type 1`. Propositions (things you prove) live in `Prop`.

```lean
-- Quick examples of where things live
#check Nat              -- Nat : Type       (a data type)
#check Prop             -- Prop : Type      (propositions are in Type)
#check Type             -- Type : Type 1    (the universe is in the next level)
#check (Nat → Prop)     -- Nat → Prop : Type  (a predicate on naturals)
#check (Type → Type)    -- Type → Type : Type 1  (a type constructor)
```

For working Lean 4 users, the practical upshot is: if you get a universe error (`type mismatch: expected Type, found Prop`), it usually means you're mixing up the data/proposition distinction. The fix is usually to check whether your type is meant to be a proposition (use `Prop`) or a piece of data (use `Type`).

## Comments and Documentation

Lean 4 has two comment styles:

```lean
-- Single-line comments: everything to the end of the line

/- 
  Multi-line comments: can span multiple lines.
  Useful for longer explanations.
-/

/-- 
  Documentation strings: use /-- ... -/ for definitions.
  These appear in the Lean infoview and in generated docs.
  They use Markdown formatting.
-/
def importantFunction : Nat → Nat := fun n => n + 1
```

Docstrings on definitions are not decoration. Mathlib requires them for contributions, and the `#check` command in the infoview will display them. Get into the habit of writing them.

## Putting It Together: A First Complete File

Here is a complete, self-contained Lean 4 file that demonstrates the concepts from this section:

```lean
-- Basics.lean: A first complete Lean 4 file

/-- The Fibonacci sequence, computed recursively. -/
def fib : Nat → Nat
  | 0     => 0
  | 1     => 1
  | n + 2 => fib n + fib (n + 1)

/-- The first 10 Fibonacci numbers. -/
#eval (List.range 10).map fib   -- [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

/-- A binary tree with data at nodes. -/
inductive BTree (α : Type) : Type where
  | leaf : BTree α
  | node : BTree α → α → BTree α → BTree α

/-- Count the nodes in a tree. -/
def BTree.size : BTree α → Nat
  | .leaf        => 0
  | .node l _ r  => 1 + l.size + r.size

/-- A simple example tree. -/
def exTree : BTree Nat :=
  .node (.node .leaf 2 .leaf) 5 (.node .leaf 7 .leaf)

-- Size should be 3 (three nodes)
#eval exTree.size   -- 3

/-- Prove that fib 4 = 3. -/
example : fib 4 = 3 := by decide
-- "decide" works for decidable propositions with small computations

/-- A proof about the tree size. -/
theorem leaf_size : BTree.size (.leaf : BTree Nat) = 0 := rfl
```

Save this file, open it in VS Code with the Lean 4 extension active, and watch the infoview respond to each `#check`, `#eval`, and proof. This is the environment in which everything else in this chapter happens.

The next section takes the vocabulary established here and adds the essential skill: writing *proofs*.
