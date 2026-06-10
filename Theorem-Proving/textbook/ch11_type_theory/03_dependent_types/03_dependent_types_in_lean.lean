-- Dependent types in Lean 4
-- Types that depend on values: the hallmark of expressive type theory

import Mathlib.Tactic

-- =====================================================
-- Π-types: dependent function types
-- =====================================================

-- A Π-type (x : A) → B x is a function where the return TYPE depends on the value
-- In Lean, this is written: (x : A) → B x or ∀ x : A, B x

-- Example: a function returning a type-indexed tuple
def repeatN (A : Type) : ℕ → Type
  | 0 => Unit
  | n + 1 => A × repeatN A n

-- repeatN Bool 3 = Bool × Bool × Bool × Unit
#check repeatN Bool 3  -- Type

-- The vector type: length-indexed lists
def Vec (A : Type) : ℕ → Type
  | 0 => Unit
  | n + 1 => A × Vec A n

def vecNil {A : Type} : Vec A 0 := ()
def vecCons {A : Type} {n : ℕ} (x : A) (xs : Vec A n) : Vec A (n + 1) := (x, xs)

-- Safe head: only callable on non-empty vectors
def vecHead {A : Type} {n : ℕ} : Vec A (n + 1) → A := Prod.fst

-- =====================================================
-- Σ-types: dependent pair types
-- =====================================================

-- A Σ-type is a pair where the TYPE of the second component depends on the first
-- In Lean: { x : A // P x } or Σ x : A, B x

-- Example: the type of even numbers
def EvenNat : Type := { n : ℕ // n % 2 = 0 }

-- Constructing members
def four : EvenNat := ⟨4, by norm_num⟩
def zero_even : EvenNat := ⟨0, by norm_num⟩

-- The sum of two even numbers is even
def even_add (x y : EvenNat) : EvenNat :=
  ⟨x.1 + y.1, by omega⟩

-- =====================================================
-- The Curry-Howard correspondence in dependent types
-- =====================================================

-- In dependent type theory, propositions ARE types:
-- P : Prop is just a type
-- A proof of P is a term of type P

-- Universal quantification = Π-type
theorem all_nats_ge_zero : ∀ n : ℕ, 0 ≤ n := Nat.zero_le

-- Existential quantification = Σ-type
theorem exists_large : ∃ n : ℕ, n > 100 := ⟨101, by norm_num⟩

-- =====================================================
-- Equality types (identity types)
-- =====================================================

-- The type a = b is the identity type (Eq a b)
-- It is propositional: we can only pattern match on it as `rfl`

theorem add_comm_example : 3 + 5 = 5 + 3 := by norm_num

-- Congruence: if a = b, then f a = f b
theorem cong_example (f : ℕ → ℕ) (a b : ℕ) (h : a = b) : f a = f b :=
  congr_arg f h

-- =====================================================
-- Inductive families (GADTs)
-- =====================================================

-- The canonical length-indexed vector
inductive Vector (A : Type) : ℕ → Type where
  | nil  : Vector A 0
  | cons : A → Vector A n → Vector A (n + 1)

-- Type-safe append: the output length is the sum of input lengths
def append {A : Type} : Vector A m → Vector A n → Vector A (m + n)
  | .nil, ys => ys
  | .cons x xs, ys => .cons x (append xs ys)

-- Type-safe zip: both vectors must have the same length
def zip {A B : Type} : Vector A n → Vector B n → Vector (A × B) n
  | .nil, .nil => .nil
  | .cons x xs, .cons y ys => .cons (x, y) (zip xs ys)

-- These are *correct by construction*: the types prevent length mismatches
