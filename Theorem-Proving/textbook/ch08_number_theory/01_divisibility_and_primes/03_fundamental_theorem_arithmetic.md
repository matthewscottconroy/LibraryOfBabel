# The Fundamental Theorem of Arithmetic

> "The Fundamental Theorem of Arithmetic is so called because it is an assertion about the *structure* of the natural numbers — not a formula, but a characterization of how numbers are built."
> — G. H. Hardy

## The Claim

Every integer $n \geq 2$ can be written as a product of prime numbers, and this factorization is **unique** (up to the order of factors):
$$n = p_1^{a_1} \cdot p_2^{a_2} \cdots p_k^{a_k}$$

where $p_1 < p_2 < \cdots < p_k$ are distinct primes and $a_1, a_2, \ldots, a_k \geq 1$.

This is the **Fundamental Theorem of Arithmetic (FTA)**, also called **unique prime factorization** or **unique factorization**.

Why "fundamental"? Because it means primes are the *multiplicative atoms* of arithmetic — every positive integer is built from primes in exactly one way. This gives us a canonical representation for every natural number, and most of elementary number theory flows from it.

## Existence: Every Integer Factors into Primes

**Theorem**: Every integer $n \geq 2$ is a product of primes.

**Proof** (by strong induction):

*Base case*: $n = 2$. Then $n$ is itself prime, and $2$ is a product of one prime. ✓

*Inductive step*: Let $n \geq 3$ and assume every integer $m$ with $2 \leq m < n$ is a product of primes.

- If $n$ is prime: done — $n$ itself is a product of one prime.
- If $n$ is composite: there exist integers $a, b$ with $2 \leq a, b < n$ and $n = ab$. By the induction hypothesis, $a$ and $b$ are both products of primes. Therefore $n = ab$ is also a product of primes. $\square$

Note that this proof needs **strong induction** — ordinary induction only gives the hypothesis for $n-1$, but $a$ and $b$ could be any integers smaller than $n$, not just $n-1$.

## Uniqueness: The Factorization is Unique

Uniqueness is harder and requires Euclid's lemma:

**Lemma (Euclid's Lemma)**: If $p$ is prime and $p \mid ab$, then $p \mid a$ or $p \mid b$.

**Proof of Euclid's Lemma**: Suppose $p \mid ab$ but $p \nmid a$. Since $p$ is prime, its only positive divisors are $1$ and $p$. Since $p \nmid a$, we have $\gcd(p, a) = 1$. By Bézout's identity (section 03), there exist integers $s, t$ with $sp + ta = 1$. Multiplying both sides by $b$: $spb + tab = b$. Now $p \mid spb$ (trivially) and $p \mid tab$ (since $p \mid ab$). By linearity, $p \mid (spb + tab) = b$. $\square$

Euclid's lemma extends by induction: if $p \mid a_1 a_2 \cdots a_k$, then $p \mid a_i$ for some $i$.

**Proof of Uniqueness**: Suppose $n = p_1 p_2 \cdots p_r = q_1 q_2 \cdots q_s$ are two prime factorizations. We show $r = s$ and (after reordering) $p_i = q_i$ for all $i$.

Since $p_1 \mid q_1 q_2 \cdots q_s$ and $p_1$ is prime, by Euclid's lemma $p_1 \mid q_j$ for some $j$. But $q_j$ is prime, so $p_1 = q_j$. Cancel $p_1 = q_j$ from both sides and continue. By induction, the factorizations match. $\square$

## Why Unique Factorization Can Fail

Unique factorization is not automatic in all algebraic structures. Consider the ring:
$$\mathbb{Z}[\sqrt{-5}] = \{a + b\sqrt{-5} \mid a, b \in \mathbb{Z}\}$$

In this ring:
$$6 = 2 \cdot 3 = (1 + \sqrt{-5})(1 - \sqrt{-5})$$

Both factorizations use "irreducible" elements (elements that cannot be factored further), yet they are different. Unique factorization fails here!

This discovery in the 19th century (in the context of attempts to prove Fermat's Last Theorem) led Ernst Kummer to introduce **ideal numbers** and Richard Dedekind to reformulate them as **ideals** — collections of ring elements that restore unique factorization at the level of ideals rather than elements. This is the genesis of modern **algebraic number theory**.

The Fundamental Theorem of Arithmetic, far from being a triviality, is a special property of $\mathbb{Z}$ — a *Unique Factorization Domain (UFD)* — that does not hold in general rings.

## Applications

**Computing GCD and LCM**: If $a = \prod p_i^{a_i}$ and $b = \prod p_i^{b_i}$ (with $a_i = 0$ or $b_i = 0$ when the prime does not appear):
$$\gcd(a, b) = \prod p_i^{\min(a_i, b_i)} \qquad \text{lcm}(a, b) = \prod p_i^{\max(a_i, b_i)}$$

$$\gcd(12, 18) = \gcd(2^2 \cdot 3, 2 \cdot 3^2) = 2^{\min(2,1)} \cdot 3^{\min(1,2)} = 2 \cdot 3 = 6$$

**Euler's Totient Function**: $\varphi(n)$ counts integers in $\{1, \ldots, n\}$ coprime to $n$. Using FTA:
$$\varphi(n) = n \prod_{p \mid n} \left(1 - \frac{1}{p}\right)$$

This formula is foundational in RSA cryptography.

**Dirichlet's Theorem on Primes in Arithmetic Progressions**: If $\gcd(a, d) = 1$, there are infinitely many primes of the form $a + nd$. The proof uses **Dirichlet series** and the multiplicative structure given by unique factorization.

**Cryptography**: RSA encryption relies on the fact that while factoring $n = pq$ (the product of two large primes) is computationally hard, the structure of $\mathbb{Z}/n\mathbb{Z}$ given by the FTA allows efficient encryption and decryption.

## Python: Factoring and Verifying FTA

```python
def prime_factors(n: int) -> dict[int, int]:
    # Return the prime factorization of n as {prime: exponent}.
    factors = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors

def gcd_from_factorization(a: int, b: int) -> int:
    fa, fb = prime_factors(a), prime_factors(b)
    result = 1
    for p in set(fa) & set(fb):
        result *= p ** min(fa[p], fb[p])
    return result

# Verify: factorization then reconstruction
n = 360
factors = prime_factors(n)
print(f"360 = {factors}")  # {2: 3, 3: 2, 5: 1}
reconstruction = 1
for p, e in factors.items():
    reconstruction *= p ** e
assert reconstruction == n
print(f"Reconstruction: {reconstruction} = 360 ✓")
```

## Lean 4

In Mathlib, the Fundamental Theorem of Arithmetic is captured by the `UniqueFactorizationDomain` typeclass, of which `ℤ` and `ℕ` are instances:

```lean
import Mathlib.RingTheory.UniqueFactorizationDomain

-- ℕ is a unique factorization monoid
#check Nat.factors          -- n.factors : List ℕ (sorted prime list)
#check Nat.factors_unique   -- uniqueness of prime factorization

-- Example: factors of 12
#eval Nat.factors 12  -- [2, 2, 3]
#eval Nat.factors 360 -- [2, 2, 2, 3, 3, 5]

-- Fundamental theorem statement in Lean style:
-- Every n ≥ 2 is the product of its prime factors,
-- and the multiset of factors is unique.
theorem fta (n : ℕ) (hn : 2 ≤ n) : ∃! (factors : Multiset ℕ),
    (∀ p ∈ factors, Nat.Prime p) ∧ factors.prod = n :=
  Nat.exists_unique_prime_factorization n hn
```

## A Philosophical Observation

The Fundamental Theorem of Arithmetic tells us that the natural numbers have a kind of *canonical representation*. There is no ambiguity about what a number "is" at the prime level. This is not true in other settings — the failure of unique factorization in rings like $\mathbb{Z}[\sqrt{-5}]$ shows that the FTA is a special structural property, not a logical inevitability.

When Kummer tried to restore unique factorization by inventing "ideal numbers," and when Dedekind formalized this into the theory of ideals, they were uncovering a deep structural principle: at the right level of abstraction (ideals rather than elements), unique factorization can often be recovered. This is one of the foundational insights of modern abstract algebra.

## Exercises
See [problems/ch08_number_theory/01_divisibility_exercises.md](../../../problems/ch08_number_theory/01_divisibility_exercises.md)
