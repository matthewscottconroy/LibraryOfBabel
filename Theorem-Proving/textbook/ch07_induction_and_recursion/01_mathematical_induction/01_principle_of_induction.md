# The Principle of Mathematical Induction

> "The principle of mathematical induction is the axiom that distinguishes the natural numbers from all other infinite sets."
> — Dedekind, paraphrased

## A Proof Technique or a Definition?

When you first encounter mathematical induction in a calculus or discrete mathematics course, it is introduced as a *proof technique*: a method for establishing statements about all natural numbers. You learn the pattern: prove a base case, prove an inductive step, conclude for all $n$.

But there is something deeper going on. Induction is not merely a useful trick — it is a **defining characteristic** of the natural numbers. To understand why, we need to understand what makes the natural numbers the natural numbers.

## The Peano Axioms

In the 1880s, Giuseppe Peano gave an axiomatic characterization of the natural numbers with five axioms (in modern form):

1. $0 \in \mathbb{N}$
2. For every $n \in \mathbb{N}$, there is a unique **successor** $S(n) \in \mathbb{N}$
3. $0$ is not the successor of any natural number: $\forall n, S(n) \neq 0$
4. The successor function is injective: $S(m) = S(n) \implies m = n$
5. **Induction**: If $P(0)$ holds and $P(n) \implies P(S(n))$ for all $n$, then $P(n)$ holds for all $n$

The first four axioms describe a structure with a starting point (0) and a "next element" function (S). But such structures are not unique — the integers, the positive rationals, and many other structures have a starting point and a successor-like operation. What makes $\mathbb{N}$ special is **Axiom 5**: the induction principle.

Induction says that $\mathbb{N}$ is the *smallest* set containing 0 and closed under $S$. Every element of $\mathbb{N}$ is *reachable* from 0 by finitely many applications of $S$. This is the key: $\mathbb{N}$ has no "extra" elements floating around that are not connected to 0.

To prove a property holds for all natural numbers, you show:
- It holds at 0 (the starting point)
- It propagates forward through $S$ (the inductive step)
- Therefore it holds everywhere in $\mathbb{N}$ — because everywhere *is* reachable from 0 by the construction above

## The Formal Principle

**Theorem (Principle of Mathematical Induction)**: Let $P$ be any property of natural numbers. If:
1. $P(0)$ *(base case)*
2. $\forall k \in \mathbb{N}, P(k) \implies P(k+1)$ *(inductive step)*

Then $\forall n \in \mathbb{N}, P(n)$.

The hypothesis $P(k)$ in the inductive step is called the **induction hypothesis (IH)**. It is not circular reasoning to *assume* $P(k)$: the inductive step says "if P holds at k, then it holds at k+1." You are not assuming the conclusion; you are establishing a conditional.

## A Worked Example: Gauss's Sum

Legend has it that a young Carl Friedrich Gauss, given the task of adding all integers from 1 to 100 as a school exercise, produced the answer in seconds by spotting the pattern: pair up the numbers as $(1 + 100), (2 + 99), \ldots, (50 + 51)$ — fifty pairs, each summing to 101, giving $50 \times 101 = 5050$.

This suggests a general formula:
$$\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$$

Let us prove it by induction.

**Base case** ($n = 1$): $\sum_{i=1}^{1} i = 1 = \frac{1 \cdot 2}{2} = 1$. ✓

**Inductive step**: Assume $\sum_{i=1}^{k} i = \frac{k(k+1)}{2}$ (IH). We must show $\sum_{i=1}^{k+1} i = \frac{(k+1)(k+2)}{2}$.

$$\sum_{i=1}^{k+1} i = \left(\sum_{i=1}^{k} i\right) + (k+1) \stackrel{\text{IH}}{=} \frac{k(k+1)}{2} + (k+1) = (k+1)\left(\frac{k}{2} + 1\right) = (k+1) \cdot \frac{k+2}{2} = \frac{(k+1)(k+2)}{2}$$

This is exactly what we needed to show. $\square$

Notice how the proof uses the induction hypothesis at a specific, identifiable step. This is essential: the IH must be applied, not just assumed.

## Why Induction Works: A Meta-Proof

Here is an argument that the induction principle is correct, assuming the Peano axioms:

Let $S = \{n \in \mathbb{N} \mid P(n)\}$ — the set of naturals for which $P$ holds. We are given:
- $P(0)$, so $0 \in S$
- $\forall k, k \in S \implies S(k) \in S$, so $S$ is closed under the successor operation

By Peano Axiom 5, any subset of $\mathbb{N}$ containing 0 and closed under $S$ must equal $\mathbb{N}$ itself (that is the minimality statement). Therefore $S = \mathbb{N}$, which means $P(n)$ holds for all $n$. $\square$

This shows that the induction principle is not an independent postulate — it follows directly from the *minimality* of $\mathbb{N}$ expressed in the fifth Peano axiom.

## Common Pitfalls

**Pitfall 1: Forgetting the base case.**
The inductive step might hold vacuously for all $k$, but without the base case, you cannot get started. A famous example: the "proof" that all horses are the same color. The argument goes by induction on the number of horses in a group. The base case (one horse) is trivially true. The inductive step (supposedly) argues that in any group of $k+1$ horses, overlapping subgroups of $k$ horses establish that all $k+1$ share a color. But the argument breaks at the base case of the inductive step — it assumes two overlapping subgroups of one horse, which requires the group to have at least 3 horses, not 2. The inductive step fails precisely at $k = 1$.

**Pitfall 2: Assuming $P(n)$ directly in the inductive step.**
The induction hypothesis is $P(k)$, not $P(k+1)$. If you find yourself assuming $P(k+1)$ to prove $P(k+1)$, you are reasoning in a circle.

**Pitfall 3: Off-by-one errors.**
Check that the base case and inductive step are compatible. If you prove $P(0)$ but the inductive step only covers $k \geq 2$, you have a gap at $k = 1$.

**Pitfall 4: Using $n$ instead of $k$ in the inductive step.**
The inductive step quantifies over *all* values of the preceding index. Using $n$ for both the "all $n$" and the particular value in the IH invites confusion.

## Lean 4 Proof

```lean
-- Gauss's sum formula in Lean 4
theorem gauss_sum (n : ℕ) : 2 * (Finset.range (n + 1)).sum id = n * (n + 1) := by
  induction n with
  | zero => simp
  | succ k ih =>
    rw [Finset.sum_range_succ]
    simp [id]
    linarith
```

Lean's `induction` tactic mirrors the mathematical structure exactly: the `zero` case handles the base, and `succ k ih` handles the inductive step with `ih : 2 * ∑ i in range (k+1), id i = k * (k+1)` available as the IH.

## Connection to Recursion

Induction and recursion are two sides of the same coin. While induction proves properties of recursively defined objects, **recursion** defines functions on $\mathbb{N}$ by:
- Giving the value at 0: $f(0) = c$
- Giving the value at $k+1$ in terms of $f(k)$: $f(k+1) = g(k, f(k))$

This is the **recursion theorem** for $\mathbb{N}$: such definitions always produce unique, well-defined functions. The Peano axioms guarantee this just as they guarantee induction.

In functional programming, this is *primitive recursion* — the computational analog of mathematical induction.

## Exercises
See [problems/ch07_induction_and_recursion/01_weak_induction_exercises.md](../../../problems/ch07_induction_and_recursion/01_weak_induction_exercises.md)
