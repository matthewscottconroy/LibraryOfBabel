# 1.1 Lean 4 Foundations

## What Is Lean 4?

Lean 4 is simultaneously:
- A **proof assistant**: a system for writing mathematical proofs that are checked by a computer
- A **functional programming language**: with a full runtime, metaprogramming, and package system
- A **foundation for mathematics**: based on the Calculus of Inductive Constructions (CIC)

The key insight of Lean (and all dependently typed proof assistants): **propositions are types, and proofs are programs**. A proof of $P \Rightarrow Q$ is literally a function from type $P$ to type $Q$. A proof of $\forall n : \mathbb{N}, P(n)$ is a function that takes a natural number and returns a proof of $P(n)$.

This is the Curry-Howard correspondence, and it means that Lean's type checker simultaneously verifies mathematical proofs and typechecks programs.

## Installation

```bash
# Install elan (Lean version manager, like rustup for Rust)
curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh

# Create a new Lean 4 project with Mathlib
lake new my_project math
cd my_project
lake update
lake build
```

For VS Code: install the `lean4` extension from the marketplace. It provides real-time feedback as you type.

## The Type Universe Hierarchy

Lean 4 has a hierarchy of type universes:
- `Prop`: the universe of *propositions* — types whose elements are proofs
- `Type` = `Type 0`: the universe of "small" types (data types, mathematical structures)
- `Type 1`, `Type 2`, ...: the hierarchy of universes, each containing the previous

```lean
-- Natural numbers live in Type
#check Nat        -- Nat : Type

-- Propositions live in Prop
#check 1 + 1 = 2  -- 1 + 1 = 2 : Prop

-- The universe itself lives one level up
#check Type       -- Type : Type 1
#check Prop       -- Prop : Type
```

**The `Prop` / `Type` distinction.** In Lean 4 (following CIC):
- `Prop`: proof-irrelevant types. Two proofs of the same proposition are definitionally equal. The Prop hierarchy collapses: there's only one `Prop` (not `Prop 0`, `Prop 1`, ...).
- `Type n`: type-relevant types. Different elements of a type can be different (e.g., `0 ≠ 1 : Nat`).

This corresponds to the HoTT distinction between propositions (h-level $-1$) and general types.

## Terms and Types

Everything in Lean 4 has a type:

```lean
-- Terms and their types
#check (42 : Nat)           -- 42 : Nat
#check (fun n => n + 1)     -- fun n => n + 1 : Nat → Nat
#check (⟨42, rfl⟩ : { n : Nat // n = 42 })  -- dependent pair

-- Definitional equality: these two are the same
#check (2 + 2)   -- 2 + 2 : Nat
#eval (2 + 2)    -- 4  (evaluation)
```

## Dependent Types

Lean 4 has full dependent types — types that depend on values:

```lean
-- Π type: dependent function type
-- (n : Nat) → Vec Nat n means: for each n, a vector of length n
-- Not the same as Nat → Vec Nat 0 (which is non-dependent)

def Vec (α : Type) : Nat → Type
  | 0     => Unit                      -- Vec α 0 = Unit
  | n + 1 => α × Vec α n              -- Vec α (n+1) = α × Vec α n

-- Σ type: dependent pair
-- ⟨n, v⟩ : Σ (n : Nat), Vec Nat n
-- The type of the second component depends on the first
#check (⟨3, (1, 2, 3, ())⟩ : (n : Nat) × Vec Nat n)

-- The identity type
-- a = b is a Prop
-- In Lean 4, it's actually defined as:
-- Eq : {α : Sort u} → α → α → Prop
-- with constructor rfl : a = a
```

## Inductive Types

Lean 4 defines types by induction — specifying their constructors:

```lean
-- Natural numbers
inductive Nat : Type where
  | zero : Nat
  | succ : Nat → Nat

-- Lists
inductive List (α : Type) : Type where
  | nil  : List α
  | cons : α → List α → List α

-- Binary trees
inductive Tree (α : Type) : Type where
  | leaf : Tree α
  | node : Tree α → α → Tree α → Tree α

-- The Empty type
inductive Empty : Type  -- no constructors!

-- The Unit type
inductive Unit : Type where
  | unit : Unit
```

**Functions on inductive types** are defined by pattern matching:

```lean
def length : List α → Nat
  | .nil        => 0
  | .cons _ xs  => 1 + length xs

def append : List α → List α → List α
  | .nil,        ys => ys
  | .cons x xs,  ys => .cons x (append xs ys)
```

## Propositions as Types

The fundamental idea: propositions are types, proofs are terms.

```lean
-- A proof of P ∧ Q is a pair of proofs
#check (And.intro : P → Q → P ∧ Q)
-- This is just the pair constructor for the type P × Q (essentially)

-- A proof of P → Q is a function P → Q
-- The "proof" is the function itself!

-- The identity type Eq
-- rfl : a = a  (reflexivity)
-- Eq.subst : a = b → P a → P b  (substitution = transport)
-- Eq.symm : a = b → b = a  (symmetry = path inversion)
-- Eq.trans : a = b → b = c → a = c  (transitivity = path concatenation)
```

## The Foundational Axioms

Lean 4 (for Mathlib) adds three axioms beyond the basic CIC rules:

```lean
-- Propositional extensionality: logically equivalent propositions are equal
axiom propext : ∀ {a b : Prop}, (a ↔ b) → a = b

-- Function extensionality: pointwise equal functions are equal
axiom funext : ∀ {α : Sort u} {β : α → Sort v} {f g : ∀ a, β a},
              (∀ a, f a = g a) → f = g

-- Quotient types: equivalence classes
-- (Built into Lean 4's core as a special inductive)
```

**Note:** These axioms together give something close to HoTT's Univalence for propositions (propext is propositional extensionality) and functions (funext). But they don't give the full Univalence axiom for types.

## Structures and Type Classes

Lean 4's mathematical infrastructure uses *structures* (records) and *type classes* (interfaces):

```lean
-- A structure (record type)
structure Point where
  x : Float
  y : Float

-- A type class (interface)
class Group (G : Type*) where
  mul  : G → G → G
  one  : G
  inv  : G → G
  mul_assoc : ∀ a b c : G, mul (mul a b) c = mul a (mul b c)
  mul_one   : ∀ a : G, mul a one = a
  inv_mul   : ∀ a : G, mul (inv a) a = one

-- An instance
instance : Group Int where
  mul := Int.add
  one := 0
  inv := Int.neg
  mul_assoc := Int.add_assoc
  mul_one   := Int.add_zero
  inv_mul   := Int.neg_add_cancel

-- Using the instance
#check @Group.mul_assoc Int _  -- The associativity law for Int (as an additive group)
```

Type classes allow Lean to automatically infer which group operations to use based on the type — you just write `a * b` and Lean finds the right multiplication.

## Where Lean 4 and HoTT Differ

Lean 4 uses `Prop` as a *proof-irrelevant* universe: two proofs of the same proposition are always definitionally equal in Lean 4. This corresponds to HoTT's h-level $-1$ (propositions).

But in HoTT, not all types are propositions — and identity types can have multiple elements. In Lean 4:
- The identity type `a = b` is a `Prop` — it has at most one proof
- This is "UIP" (uniqueness of identity proofs) for `Prop`-valued equality
- This is exactly what the K axiom says!

So **Lean 4 essentially has K for propositions** — all propositions satisfy UIP. This means:
- Lean 4 cannot directly model the circle $S^1$ as a HIT with a non-trivial loop
- Lean 4 cannot formalize synthetic homotopy theory
- But Lean 4 can formalize all of classical mathematics (sets, groups, topological spaces, etc.)

**The key distinction:** Lean 4 is for *classical mathematics* (where UIP holds for the equality we care about). Cubical Agda (Chapter 22) is for *HoTT-specific* mathematics (where UIP can fail).

For our purposes: use Lean 4 to formalize the algebraic, categorical, and topological background. Use Cubical Agda to formalize the HoTT-specific content.
