# Weak Induction: Worked Examples

## A Gallery of Induction Proofs

The best way to master induction is through varied examples. Each proof has the same structure — base case, inductive step using the IH — but the creative challenge is different each time.

## Example 1: Geometric Sum

**Claim**: For $r \neq 1$ and $n \geq 0$: $\sum_{i=0}^{n} r^i = \frac{r^{n+1} - 1}{r - 1}$

**Base case** ($n = 0$): $r^0 = 1 = \frac{r - 1}{r - 1}$ ✓ (for $r \neq 1$)

**Inductive step**: Assume $\sum_{i=0}^{k} r^i = \frac{r^{k+1} - 1}{r - 1}$ (IH).

$$\sum_{i=0}^{k+1} r^i = r^{k+1} + \sum_{i=0}^{k} r^i \stackrel{\text{IH}}{=} r^{k+1} + \frac{r^{k+1}-1}{r-1} = \frac{r^{k+1}(r-1) + r^{k+1} - 1}{r-1} = \frac{r^{k+2} - 1}{r-1}$$  ✓

## Example 2: Divisibility

**Claim**: $3 \mid 4^n - 1$ for all $n \geq 1$.

**Base case** ($n = 1$): $4^1 - 1 = 3$, and $3 \mid 3$ ✓

**Inductive step**: Assume $3 \mid 4^k - 1$ (IH). Then $4^k - 1 = 3m$ for some $m$.

$4^{k+1} - 1 = 4 \cdot 4^k - 1 = 4(4^k - 1) + 4 - 1 = 4 \cdot 3m + 3 = 3(4m + 1)$.

So $3 \mid 4^{k+1} - 1$. ✓

## Example 3: Inequality

**Claim**: $2^n > n$ for all $n \geq 1$.

**Base case** ($n = 1$): $2^1 = 2 > 1$ ✓

**Inductive step**: Assume $2^k > k$ (IH). Then $2^{k+1} = 2 \cdot 2^k > 2k \geq k + 1$ (since $k \geq 1$). ✓

## Example 4: Fibonacci and Explicit Formula

The Fibonacci sequence: $F_0 = 0, F_1 = 1, F_n = F_{n-1} + F_{n-2}$.

**Claim**: $F_{2n} = F_n(2F_{n+1} - F_n)$ for all $n \geq 0$.

This requires **strong induction** (the step uses two previous values), covered in the next section.

## In Lean 4

```lean
-- Geometric sum in Lean (sketch)
theorem geom_sum (r : ℝ) (hr : r ≠ 1) (n : ℕ) :
    (Finset.range (n+1)).sum (fun i => r^i) = (r^(n+1) - 1) / (r - 1) := by
  induction n with
  | zero => simp
  | succ k ih =>
    rw [Finset.sum_range_succ, ih]
    field_simp
    ring
```

## Exercises
See [problems/ch07_induction_and_recursion/01_weak_induction_exercises.md](../../../problems/ch07_induction_and_recursion/01_weak_induction_exercises.md)
