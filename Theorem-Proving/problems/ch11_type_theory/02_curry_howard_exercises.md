# Curry-Howard Correspondence Exercises

## Section 1: Dictionary Translation (★)

**1.** Translate each logic proof into a program (in Haskell or Python), and each program
into a logic proof:

  a. Logic: P ∧ Q → Q ∧ P
     Program: ?

  b. Program: `const :: a -> b -> a; const x _ = x`
     Logic: ?

  c. Logic: (P → Q) → (Q → R) → P → R
     Program: ?

  d. Program: `apply :: (a -> b) -> a -> b; apply f x = f x`
     Logic: ?

## Section 2: Writing Proofs as Programs (★★)

**2.** In Haskell, write functions with these types. Then identify the logical statement:
  a. `f :: (a, b) -> a`
  b. `g :: a -> (a, a)`
  c. `h :: (a -> b, a -> c) -> a -> (b, c)`
  d. `k :: Either a b -> Either b a`

**3.** Write a Lean 4 proof of each (they should match your Haskell functions above):
```lean
-- 2a
example {α β : Type} : α × β → α := by sorry
-- 2b
example {α : Type} : α → α × α := by sorry
-- 2c
example {α β γ : Type} : (α → β) × (α → γ) → α → β × γ := by sorry
-- 2d
example {α β : Type} : α ⊕ β → β ⊕ α := by sorry
```

## Section 3: Non-Constructive Reasoning (★★★)

**4.** Why is there no Haskell function of type `forall a. Not (Not a) -> a`
(where `type Not a = a -> Void`)?

  In Haskell, every function must be computable. `Not (Not a) -> a` would require
  constructing a value of type `a` given only the knowledge that the type `a` is inhabited.
  But without a witness, we cannot construct the value.

  This corresponds exactly to the non-constructibility of DNE (double negation elimination).

**5.** In Lean 4, try to prove `¬¬p → p` without using `Classical.em` or `by_contra`.
What happens? What does this tell you about intuitionistic logic?
