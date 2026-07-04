# Strong Induction

## When Weak Induction Falls Short

In weak (ordinary) mathematical induction, the induction hypothesis gives us only $P(k)$ — the statement for the *immediately preceding* value. Sometimes, to prove $P(k+1)$, we need to assume $P$ holds for *all* $m \leq k$, not just $k$.

**Strong induction** (also called **complete induction** or **course-of-values induction**):

To prove $\forall n \in \mathbb{N}, P(n)$:
1. Prove $P(0)$ (base case)
2. Prove: for all $k$, if $P(m)$ holds for all $m \leq k$, then $P(k+1)$ holds

The IH in step 2 is $\{P(m) \mid m \leq k\}$ — a stronger assumption.

**Strong and weak induction are equivalent in power**: each implies the other. Strong induction is more convenient when the recursive structure reaches back more than one step.

## The Fibonacci Example

**Claim**: Every Fibonacci number $F_n$ satisfies $F_n < 2^n$.

($F_0 = 0, F_1 = 1, F_2 = 1, F_3 = 2, F_4 = 3, F_5 = 5, \ldots$)

**Base cases**: $F_0 = 0 < 1 = 2^0$ ✓ and $F_1 = 1 < 2 = 2^1$ ✓

**Inductive step** (strong IH: assume $F_m < 2^m$ for all $m \leq k$, prove $F_{k+1} < 2^{k+1}$):

Since $k \geq 1$, we have $k+1 \geq 2$, so $F_{k+1} = F_k + F_{k-1}$.

By the strong IH: $F_k < 2^k$ and $F_{k-1} < 2^{k-1}$.

$F_{k+1} = F_k + F_{k-1} < 2^k + 2^{k-1} < 2^k + 2^k = 2^{k+1}$ ✓

The proof needed both $F_k$ and $F_{k-1}$, which weak induction would not provide in one step.

## Prime Factorization (Existence via Strong Induction)

We can prove every $n \geq 2$ has a prime factorization by strong induction:

**Base case** ($n = 2$): 2 is prime, done.

**Inductive step**: Assume every $m$ with $2 \leq m \leq k$ has a prime factorization. Consider $k+1$:
- If $k+1$ is prime: done (it is its own factorization).
- If $k+1$ is composite: $k+1 = ab$ with $2 \leq a, b \leq k$. By IH, both $a$ and $b$ have prime factorizations. Their product is a prime factorization of $k+1$. ✓

## In Lean 4

```lean
-- Strong induction in Lean via Nat.rec or Nat.strongInduction
theorem fib_lt_pow (n : ℕ) : Nat.fib n < 2 ^ n := by
  induction n using Nat.strong_rec_on with
  | _ n ih => ...
```

## Exercises
See [problems/ch07_induction_and_recursion/01_weak_induction_exercises.md](../../../problems/ch07_induction_and_recursion/01_weak_induction_exercises.md)
