# Structural Induction

## Overview
Structural induction generalizes mathematical induction from natural numbers to arbitrary
inductively defined data structures: lists, trees, formulas, derivations.
It is the native induction principle for inductive types in Lean and Coq.

## Learning Objectives
- Define structural induction for lists and binary trees
- Apply structural induction to prove properties of recursive programs
- Understand the connection to recursive type definitions

## The Principle
For an inductively defined type T with constructors C₁,...,Cₙ:
To prove ∀x : T, P(x):
1. For each constructor Cᵢ, assuming P holds for all sub-components of the result,
   prove P holds for Cᵢ(...).

## Example: Lists
A list is either [] (nil) or x :: xs (cons).
To prove ∀l : List α, P(l):
- Base: prove P([])
- Inductive: assuming P(xs) (IH), prove P(x :: xs)

**Theorem**: length (l ++ m) = length l + length m
- Base: length ([] ++ m) = length m = 0 + length m = length [] + length m ✓
- Inductive: length ((x::xs) ++ m) = length (x::(xs++m)) = 1 + length(xs++m)
  By IH = 1 + length xs + length m = length (x::xs) + length m ✓

## Example: Binary Trees
```
data Tree a = Leaf | Node (Tree a) a (Tree a)
```
To prove P(t) for all t:
- Base: prove P(Leaf)
- Inductive: assuming P(l) and P(r), prove P(Node l v r)

## Haskell
See `textbook/ch07_induction_and_recursion/04_recursion/03_recursion_in_haskell.hs`

## Lean 4 / Coq
Both handle structural induction automatically via the `induction` tactic.

## Exercises
See `problems/ch07_induction/03_structural_induction_problems.md`
