# Injections, Surjections, and Bijections

## Overview
Functions can be classified by how they map their domain to their codomain.
These classifications are essential for understanding cardinality, invertibility,
and isomorphism throughout mathematics.

## Learning Objectives
- Define injection (one-to-one), surjection (onto), and bijection
- Prove or disprove that a given function is injective/surjective/bijective
- Connect bijections to cardinality

## Definitions
Let f : A → B.

**Injective (one-to-one)**: ∀x,y ∈ A, f(x) = f(y) → x = y
(distinct inputs give distinct outputs; no "collisions")

**Surjective (onto)**: ∀b ∈ B, ∃a ∈ A, f(a) = b
(every element of the codomain is hit)

**Bijective**: injective and surjective
(perfect pairing between domain and codomain)

## Examples
- f(x) = 2x on ℤ: injective, not surjective (odd integers not hit)
- f(x) = x mod 2 on ℤ: surjective onto {0,1}, not injective
- f(x) = x + 1 on ℤ: bijective
- f(x) = x² on ℝ: neither injective (f(-1)=f(1)) nor surjective (negatives not hit)

## Inverses
f has a left inverse iff f is injective.
f has a right inverse iff f is surjective (requires Axiom of Choice for general sets).
f has a two-sided inverse iff f is bijective.

## Haskell Connection
```haskell
-- In Haskell, the type system can encode injections via newtypes and smart constructors
-- but not enforce surjectivity directly. Bijections ≈ Iso in lens/optics libraries.
```

## Exercises
See `problems/ch06_set_theory/03_function_proofs.md`
