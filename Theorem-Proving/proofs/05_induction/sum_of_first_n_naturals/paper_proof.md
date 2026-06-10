# Sum of First n Naturals: Paper Proof

## Theorem
For all n ≥ 0: ∑_{i=0}^{n} i = n(n+1)/2.

## Proof by Mathematical Induction

**Base case** (n = 0):
∑_{i=0}^{0} i = 0 = 0(1)/2 = 0. ✓

**Inductive step**:
Assume the formula holds for n = k (induction hypothesis):
∑_{i=0}^{k} i = k(k+1)/2.

We must show it holds for n = k+1:
∑_{i=0}^{k+1} i = ∑_{i=0}^{k} i + (k+1)
                 = k(k+1)/2 + (k+1)          [by IH]
                 = (k+1)(k/2 + 1)
                 = (k+1)(k+2)/2
                 = (k+1)((k+1)+1)/2           ✓

By the principle of mathematical induction, the formula holds for all n ≥ 0. □

## Gauss's Argument (non-inductive)
Write S = 1 + 2 + ... + n
     S = n + (n-1) + ... + 1
Adding: 2S = (n+1) + (n+1) + ... + (n+1)  [n times]
So 2S = n(n+1), thus S = n(n+1)/2. □
