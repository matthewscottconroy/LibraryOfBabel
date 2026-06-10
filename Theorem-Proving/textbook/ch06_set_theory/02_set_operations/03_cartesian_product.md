# Cartesian Product

## Ordered Pairs

A crucial concept for building structured data from sets: the **ordered pair** $(a, b)$.

Unlike sets, order matters in pairs: $(a, b) \neq (b, a)$ in general (unless $a = b$). The standard set-theoretic encoding of the pair $(a, b)$ is due to Kuratowski (1921):

$$(a, b) = \{\{a\}, \{a, b\}\}$$

This encoding satisfies the essential property: $(a, b) = (c, d)$ iff $a = c$ and $b = d$.

## Cartesian Product

Given sets $A$ and $B$, the **Cartesian product** is:

$$A \times B = \{(a, b) \mid a \in A \wedge b \in B\}$$

the set of all ordered pairs with first component from $A$ and second from $B$.

**Examples:**
- $\{1, 2\} \times \{a, b\} = \{(1,a), (1,b), (2,a), (2,b)\}$
- $\mathbb{R} \times \mathbb{R} = \mathbb{R}^2$ — the Euclidean plane
- $A \times \emptyset = \emptyset = \emptyset \times A$
- $|A \times B| = |A| \cdot |B|$ for finite sets (the product principle in combinatorics)

The name "Cartesian" honors René Descartes, who introduced coordinate geometry — the idea that points in the plane are pairs of real numbers.

## Higher Products and Tuples

$A^n = \underbrace{A \times A \times \cdots \times A}_{n}$ consists of $n$-tuples of elements from $A$.

$\mathbb{R}^3$ is the Cartesian space of 3D geometry. $\{0,1\}^n$ is the set of all binary strings of length $n$ — the foundation of information theory.

## In Lean 4

```lean
-- Cartesian product in Lean
def pair_example : ℕ × ℤ := (3, -5)
#eval pair_example.1  -- 3 (first component)
#eval pair_example.2  -- -5 (second component)

-- Set-theoretic Cartesian product
example : (3, 'a') ∈ ({1, 2, 3} ×ˢ {'a', 'b'} : Set (ℕ × Char)) := by
  constructor <;> simp

-- Size: |A × B| = |A| * |B|
example (A : Finset ℕ) (B : Finset ℕ) : (A ×ˢ B).card = A.card * B.card :=
  Finset.card_product A B
```

## Exercises
See [problems/ch06_set_theory/01_set_operations_exercises.md](../../../problems/ch06_set_theory/01_set_operations_exercises.md)
