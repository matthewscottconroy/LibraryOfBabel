# Structural Induction on Trees

## Binary Trees as Recursive Data

A **binary tree** over type $\alpha$ is either:
- A **leaf** (empty tree), or
- A **node** with a value of type $\alpha$, a left subtree, and a right subtree

```lean
inductive Tree (α : Type) where
  | leaf : Tree α
  | node : α → Tree α → Tree α → Tree α
```

## Structural Induction on Trees

**Principle**: To prove $\forall t : \text{Tree}\; \alpha,\; P(t)$:
1. **Base case** (leaf): Prove $P(\text{leaf})$
2. **Inductive step** (node): For any $a$, and trees $l$, $r$ with $P(l)$ and $P(r)$, prove $P(\text{node}\; a\; l\; r)$

## Classic Tree Lemmas

**Claim**: For any binary tree $t$, $\text{leaves}(t) = 1 + \text{internals}(t)$ (a tree with $n$ internal nodes has $n+1$ leaves).

**Proof** by structural induction:

**Base** (leaf): $\text{leaves}(\text{leaf}) = 1$, $\text{internals}(\text{leaf}) = 0$. $1 = 1 + 0$ ✓

**Step** (node $a\; l\; r$): By IH, $\text{leaves}(l) = 1 + \text{internals}(l)$ and same for $r$.

$\text{leaves}(\text{node}\; a\; l\; r) = \text{leaves}(l) + \text{leaves}(r)$
$= (1 + \text{internals}(l)) + (1 + \text{internals}(r)) = 2 + (\text{internals}(l) + \text{internals}(r))$
$= 1 + (1 + \text{internals}(l) + \text{internals}(r)) = 1 + \text{internals}(\text{node}\; a\; l\; r)$ ✓

## In Lean 4

```lean
def height : Tree α → ℕ
  | Tree.leaf     => 0
  | Tree.node _ l r => max (height l) (height r) + 1

def size : Tree α → ℕ
  | Tree.leaf     => 0
  | Tree.node _ l r => size l + size r + 1

-- A tree of height h has at most 2^(h+1) - 1 nodes
theorem size_le_pow (t : Tree α) : size t ≤ 2^(height t + 1) - 1 := by
  induction t with
  | leaf => simp [size, height]
  | node _ l r ihl ihr => simp [size, height]; linarith [ihl, ihr]
```

## Exercises
See [problems/ch07_induction/02_structural_induction_exercises.md](../../../problems/ch07_induction/02_structural_induction_exercises.md)
