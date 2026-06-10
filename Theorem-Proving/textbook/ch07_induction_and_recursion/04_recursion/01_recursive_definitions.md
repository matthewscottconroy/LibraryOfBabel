# Recursive Definitions

## Defining Functions by Recursion

A **recursive definition** defines a function on a structured set by:
1. Specifying its value on the base cases (leaves, zero, empty list)
2. Expressing its value on composite cases in terms of the function applied to smaller inputs

The **recursion theorem** guarantees these always define unique, well-defined total functions — provided the structure is well-founded (no infinite regress).

## Examples

**Factorial**:
$$0! = 1 \qquad (n+1)! = (n+1) \cdot n!$$

**List length**:
$$\text{len}([]) = 0 \qquad \text{len}(x :: xs) = 1 + \text{len}(xs)$$

**List reversal**:
$$\text{rev}([]) = [] \qquad \text{rev}(x :: xs) = \text{rev}(xs) ++ [x]$$

**Merge sort** (structural recursion on the half-sized list — needs well-founded induction):
$$\text{msort}([]) = [] \qquad \text{msort}([x]) = [x] \qquad \text{msort}(l) = \text{merge}(\text{msort}(\text{firstHalf}(l)), \text{msort}(\text{secondHalf}(l)))$$

## Primitive Recursion

**Primitive recursion** on $\mathbb{N}$: define $f : \mathbb{N} \to A$ by:
$$f(0) = c \qquad f(n+1) = g(n, f(n))$$

for some constant $c : A$ and function $g : \mathbb{N} \times A \to A$. This always terminates.

**Primitive recursive functions** — sums, products, factorial, Fibonacci, primality, etc. — are exactly those definable by primitive recursion. They are a strict subset of all computable functions (the Ackermann function is computable but not primitive recursive).

## In Lean 4

```lean
-- Factorial by pattern matching (= primitive recursion)
def factorial : ℕ → ℕ
  | 0     => 1
  | n + 1 => (n + 1) * factorial n

-- Proof by induction mirrors the definition by recursion
theorem factorial_pos : ∀ n : ℕ, 0 < factorial n := by
  intro n
  induction n with
  | zero     => simp [factorial]
  | succ k h => simp [factorial]; exact Nat.mul_pos (Nat.succ_pos k) h
```

## Exercises
See [problems/ch07_induction/](../../../problems/ch07_induction/)
