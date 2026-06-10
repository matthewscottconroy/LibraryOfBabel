# 3.1 Weak and Strong Mathematical Induction

## The Principle Behind Induction

Mathematical induction is the formal justification for a very natural way of thinking about the natural numbers: they form a sequence where each number is reached from the previous one by adding 1. If something holds at the start (0 or 1), and whenever it holds for one number it holds for the next, then it holds everywhere.

This principle is not an arbitrary rule — it follows from the *definition* of the natural numbers. The natural numbers are the smallest set satisfying: 0 is a natural number, and if $n$ is a natural number then $n + 1$ is a natural number. "Smallest" means: every property that holds at 0 and is preserved by the successor operation holds for all natural numbers. That's induction.

## Weak (Simple) Induction

**Principle of Mathematical Induction.** Let $P(n)$ be a property of natural numbers. If:
1. **Base case:** $P(0)$ holds.
2. **Inductive step:** For all $n \geq 0$, $P(n) \to P(n+1)$.

Then $P(n)$ holds for all $n \geq 0$.

The step from $P(n)$ to $P(n+1)$ uses the *induction hypothesis* $P(n)$ — you are allowed to assume $P(n)$ when proving $P(n+1)$.

**A note on the base case:** Sometimes induction starts at $n = 1$ or $n = 2$ rather than $n = 0$. Adjust accordingly: prove $P(k_0)$ for the first value $k_0$ you care about, and prove $P(n) \to P(n+1)$ for all $n \geq k_0$. The conclusion is $P(n)$ for all $n \geq k_0$.

**Example 1: Gaussian sum.**

*Claim:* For all $n \geq 0$, $\displaystyle\sum_{k=0}^{n} k = \frac{n(n+1)}{2}$.

*Proof.* By induction on $n$.

*Base case ($n = 0$):* $\sum_{k=0}^{0} k = 0 = \frac{0 \cdot 1}{2}$. ✓

*Inductive step:* Assume $\sum_{k=0}^{n} k = \frac{n(n+1)}{2}$ (induction hypothesis). We need to show $\sum_{k=0}^{n+1} k = \frac{(n+1)(n+2)}{2}$.

$$\sum_{k=0}^{n+1} k = \left(\sum_{k=0}^{n} k\right) + (n+1) = \frac{n(n+1)}{2} + (n+1) = (n+1)\left(\frac{n}{2} + 1\right) = \frac{(n+1)(n+2)}{2}$$

The inductive step holds. By induction, the formula holds for all $n \geq 0$. $\square$

**Example 2: Geometric series.**

*Claim:* For all $n \geq 0$ and $r \neq 1$, $\displaystyle\sum_{k=0}^{n} r^k = \frac{r^{n+1} - 1}{r - 1}$.

*Proof.* By induction on $n$.

*Base case ($n = 0$):* $r^0 = 1 = \frac{r - 1}{r - 1}$. ✓

*Inductive step:* Assume $\sum_{k=0}^{n} r^k = \frac{r^{n+1}-1}{r-1}$.
$$\sum_{k=0}^{n+1} r^k = \frac{r^{n+1}-1}{r-1} + r^{n+1} = \frac{r^{n+1}-1 + r^{n+1}(r-1)}{r-1} = \frac{r^{n+2}-1}{r-1}$$
This is the formula with $n+1$. $\square$

**Example 3: Divisibility.**

*Claim:* For all $n \geq 0$, $6 \mid n^3 - n$ (i.e., $n^3 - n$ is divisible by 6).

*Proof.* By induction.

*Base case ($n = 0$):* $0^3 - 0 = 0$, which is divisible by 6. ✓

*Inductive step:* Assume $6 \mid n^3 - n$. We want $6 \mid (n+1)^3 - (n+1)$.

$(n+1)^3 - (n+1) = n^3 + 3n^2 + 3n + 1 - n - 1 = (n^3 - n) + 3n^2 + 3n = (n^3 - n) + 3n(n+1)$

By hypothesis $6 \mid n^3 - n$. For the second term: one of $n, n+1$ is even, so $2 \mid n(n+1)$, giving $6 \mid 3n(n+1)$ (since $3 \cdot 2 = 6$). Therefore $6 \mid (n+1)^3 - (n+1)$. $\square$

## The Anatomy of an Induction Proof

A well-structured induction proof has these parts, stated explicitly:

1. **State the variable and what you're proving:** "By induction on $n$, we prove $P(n)$."
2. **Base case:** "Base case ($n = k_0$):" followed by the proof.
3. **Inductive step:** "Inductive step: Assume $P(n)$." (State the hypothesis explicitly.) "We prove $P(n+1)$."
4. **Conclusion:** "By induction, $P(n)$ holds for all $n \geq k_0$."

Omitting any of these is a proof defect.

## Strong (Complete) Induction

In *weak* induction, the inductive step assumes only $P(n)$ to prove $P(n+1)$. In *strong* induction, it assumes *all* of $P(0), P(1), \ldots, P(n)$ to prove $P(n+1)$.

**Principle of Strong Induction.** Let $P(n)$ be a property. If for all $n \geq 0$:
$$\left(\forall k < n,\, P(k)\right) \to P(n)$$
then $P(n)$ holds for all $n \geq 0$.

*Note:* In strong induction, the base case is implicit: when $n = 0$, the hypothesis $\forall k < 0, P(k)$ is vacuously true (there are no $k < 0$), so we must prove $P(0)$ outright. The inductive hypothesis then covers all smaller values.

**Why strong induction is valid:** Strong induction is equivalent to weak induction. (Proof: apply weak induction to $Q(n) = \forall k \leq n, P(k)$.)

**When to use strong induction:** When proving $P(n+1)$ requires $P(m)$ for some $m < n$, not just $P(n)$ itself.

**Example 4: Prime factorization.**

*Claim:* Every integer $n \geq 2$ has a prime factorization.

*Proof.* By strong induction on $n$.

*Inductive step:* Let $n \geq 2$. Assume every integer $m$ with $2 \leq m < n$ has a prime factorization.

If $n$ is prime: $n$ is its own (trivial) prime factorization.

If $n$ is not prime: $n = ab$ for some $2 \leq a, b < n$. By the induction hypothesis, $a$ and $b$ each have prime factorizations. Concatenating them gives a prime factorization of $n$.

By strong induction, every $n \geq 2$ has a prime factorization. $\square$

Weak induction would not work here: knowing that $n-1$ has a prime factorization doesn't help with $n$. We need the factorization of the divisors $a$ and $b$, which could be anywhere from 2 to $n-1$.

**Example 5: Fibonacci bound.**

Define the Fibonacci sequence: $F_0 = 0, F_1 = 1, F_n = F_{n-1} + F_{n-2}$ for $n \geq 2$.

*Claim:* For all $n \geq 0$, $F_n < 2^n$.

*Proof.* By strong induction.

*Base cases:* $F_0 = 0 < 1 = 2^0$. $F_1 = 1 < 2 = 2^1$. ✓

*Inductive step ($n \geq 2$):* Assume $F_k < 2^k$ for all $k < n$.
$$F_n = F_{n-1} + F_{n-2} < 2^{n-1} + 2^{n-2} = 2^{n-2}(2 + 1) = 3 \cdot 2^{n-2} < 4 \cdot 2^{n-2} = 2^n \quad \square$$

Two base cases were needed because the recurrence $F_n = F_{n-1} + F_{n-2}$ requires two previous values.

## Induction as a Proof Rule

In natural deduction, mathematical induction corresponds to the *elimination rule for natural numbers* $\mathbb{N}$:

$$\frac{\Gamma \vdash P(0) \quad \Gamma \vdash \forall n:\mathbb{N},\, P(n) \to P(S(n))}{\Gamma \vdash \forall n:\mathbb{N},\, P(n)} \quad [\mathbb{N}\text{-elim}]$$

In type theory (Chapter 9), this rule becomes the *recursor* for $\mathbb{N}$: to define a function $\mathbb{N} \to C$, give a value at zero and a step function $C \to C$. Induction is the *logical* (proof-relevant) version of this recursion. The correspondence between recursion and induction is part of the Curry-Howard isomorphism (Chapter 6).

## Common Mistakes in Induction Proofs

**Missing base case.** Without the base case, the inductive step alone proves nothing. There is a famous "proof" that all horses are the same color, which fails because the base case $n = 1$ does not support the inductive step for $n = 2$:

For $n = 1$: a single horse is the same color as itself. ✓
For $n = 2$: two horses $\{A, B\}$. By inductive hypothesis, all horses in $\{A\}$ are one color and all horses in $\{B\}$ are one color... but these might be different colors! The sets $\{A\}$ and $\{B\}$ don't overlap, so you can't conclude $A$ and $B$ are the same color.

**Circular inductive step.** Assuming $P(n+1)$ in the proof of $P(n+1)$ invalidates the proof.

**Wrong induction variable.** Make sure you're doing induction on the right quantity. A proof by induction on $n$ must actually have the step size go down toward the base case.

**Off-by-one errors.** The formula might start at $n = 1$, not $n = 0$. Check your base case carefully.
