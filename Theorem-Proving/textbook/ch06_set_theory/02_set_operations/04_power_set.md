# The Power Set

Of all set-theoretic constructions, the power set is the one that most dramatically expands the universe — it is the engine of the transfinite hierarchy.

## Definition

The *power set* of a set A, written 𝒫(A) or 2^A, is the set of all subsets of A:

> **𝒫(A) = {X | X ⊆ A}**

```
A = {1, 2, 3}
𝒫(A) = {∅, {1}, {2}, {3}, {1,2}, {1,3}, {2,3}, {1,2,3}}
```

Every set has exactly 2^|A| subsets (for finite A). Each element independently either belongs or doesn't belong to each subset — a binary choice — giving 2 × 2 × ... × 2 = 2^n possibilities.

## Why "2^A"?

There's a beautiful bijection between 𝒫(A) and the set of functions A → {0,1} (equivalently, 2^A in the function-exponential sense):

Given a subset X ⊆ A, define its *characteristic function* χ_X : A → {0,1} by:
```
χ_X(a) = 1 if a ∈ X
χ_X(a) = 0 if a ∉ X
```

This is a bijection: subsets correspond exactly to binary-valued functions. So `𝒫(A) ≅ {0,1}^A = 2^A`.

This notation is exact: the power set of A has the same cardinality as the set of functions from A to a two-element set.

## Cantor's Theorem

The power set is always strictly larger than the original set:

> **Theorem (Cantor, 1891)**: For any set A, |A| < |𝒫(A)|.

**Proof**: There is an injection A → 𝒫(A) (send each a to {a}). We show there is no surjection.

Suppose f : A → 𝒫(A) is any function. Define D = {a ∈ A | a ∉ f(a)}. Then D ⊆ A, so D ∈ 𝒫(A). Is D = f(a) for any a?

If D = f(a): either a ∈ D or a ∉ D.
- If a ∈ D, then by definition a ∉ f(a) = D. Contradiction.
- If a ∉ D, then a ∉ f(a) = D, so by definition a ∈ D. Contradiction.

So D is not in the range of f. Thus f is not surjective. Since f was arbitrary, no function A → 𝒫(A) is surjective. ∎

This is the same diagonalization argument as Gödel's incompleteness and the halting problem. The diagonal set D is constructed to *disagree* with each f(a) at position a — a witness that the enumeration missed something.

## The Cumulative Hierarchy

The power set operation generates the entire universe of set theory, iterating through the ordinals:

```
V_0 = ∅
V_1 = 𝒫(∅) = {∅}
V_2 = 𝒫({∅}) = {∅, {∅}}
V_3 = 𝒫(V_2) = ... (4 elements)
V_4 = 𝒫(V_3) = ... (16 elements)
...
V_ω = ⋃_{n<ω} V_n    (all hereditarily finite sets)
V_{ω+1} = 𝒫(V_ω)    (includes all sets of natural numbers, i.e., all real numbers!)
```

At each stage, the universe explodes in size. The real numbers live at level V_{ω+1}. The Continuum Hypothesis asks whether |𝒫(ℕ)| = ℵ₁ — Gödel and Cohen showed this is independent of ZFC.

## In Python

```python
from itertools import chain, combinations

def power_set(A):
    A = list(A)
    return list(chain.from_iterable(combinations(A, r) for r in range(len(A)+1)))

A = {1, 2, 3}
print(power_set(A))
# [(), (1,), (2,), (3,), (1, 2), (1, 3), (2, 3), (1, 2, 3)]
# (using tuples; empty tuple represents ∅)
```

## The Power Set Axiom

In ZFC, the *Power Set Axiom* asserts that the power set of any set is itself a set:

> ∀A. ∃P. ∀X. (X ⊆ A) ↔ X ∈ P

Without this axiom, we couldn't guarantee that 𝒫(A) exists as a collected totality. The axiom is what gives us access to uncountable infinities from countable ones — it is the gate through which ℝ enters the set-theoretic universe.
