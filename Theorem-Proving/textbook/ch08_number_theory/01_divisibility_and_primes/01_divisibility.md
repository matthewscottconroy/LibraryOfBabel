# Divisibility

> "Number theory is the queen of mathematics, and arithmetic is the queen of number theory."
> — Carl Friedrich Gauss

## The Simplest Question with the Deepest Consequences

Divisibility is, on its face, a simple idea: does one number divide evenly into another, with no remainder? Yet this simple question — first studied systematically by Euclid around 300 BCE — opens into one of the richest areas of mathematics, with connections to cryptography, computer science, physics, and the deepest unsolved problems in mathematics.

We begin at the beginning: what does it mean for one integer to divide another?

## The Definition

Let $a, b \in \mathbb{Z}$ with $a \neq 0$. We say $a$ **divides** $b$, written $a \mid b$, if there exists an integer $k$ such that $b = a \cdot k$.

$$a \mid b \quad \iff \quad \exists k \in \mathbb{Z},\, b = a \cdot k$$

When $a \mid b$ we also say $a$ is a **divisor** of $b$, or $b$ is a **multiple** of $a$.

When $a$ does not divide $b$, we write $a \nmid b$. For example: $3 \mid 12$ (since $12 = 3 \cdot 4$), but $5 \nmid 12$.

**Edge cases to watch:**
- $1 \mid a$ for every integer $a$ (with $k = a$)
- $a \mid 0$ for every nonzero $a$ (with $k = 0$)
- $a \mid a$ for every nonzero $a$ (with $k = 1$)
- $0 \mid b$ only if $b = 0$ (since $b = 0 \cdot k = 0$ for any $k$)

Note that divisibility is a relation on integers, and negative divisors are permitted. We have $-3 \mid 12$ (with $k = -4$) and $3 \mid -12$ (with $k = -4$). In elementary number theory we often restrict to positive divisors for convenience, but the general definition is over $\mathbb{Z}$.

## Basic Properties

The following properties all follow directly from the definition by algebraic manipulation:

**Reflexivity**: $a \mid a$ (use $k = 1$)

**Transitivity**: If $a \mid b$ and $b \mid c$, then $a \mid c$.
*Proof*: $b = a \cdot k_1$ and $c = b \cdot k_2$, so $c = a \cdot (k_1 k_2)$. $\square$

**Linearity**: If $a \mid b$ and $a \mid c$, then $a \mid (mb + nc)$ for any integers $m, n$.
*Proof*: $b = a k_1$, $c = a k_2$, so $mb + nc = a(mk_1 + nk_2)$. $\square$

This linearity property is extremely useful. It says: if $a$ divides two things, it divides any *linear combination* of them. This is the key tool in GCD arguments.

**Antisymmetry**: If $a \mid b$ and $b \mid a$, then $a = \pm b$.
*Proof*: $b = ak$ and $a = bj$, so $a = akj$, giving $kj = 1$ (since $a \neq 0$). In $\mathbb{Z}$, this forces $k = j = 1$ or $k = j = -1$. $\square$

**Multiplication**: If $a \mid b$, then $a \mid bc$ for any integer $c$.

## The Division Algorithm

The most fundamental theorem about divisibility is that division with remainder always works:

**Theorem (Division Algorithm)**: For any integers $a$ and $b$ with $b > 0$, there exist **unique** integers $q$ (quotient) and $r$ (remainder) such that:
$$a = bq + r \quad \text{and} \quad 0 \leq r < b$$

**Proof (Existence)**: Consider the set $S = \{a - bk \mid k \in \mathbb{Z}\} \cap \mathbb{Z}_{\geq 0}$. This set is non-empty (for large enough negative $k$, $a - bk$ is positive). Let $r$ be the smallest element of $S$, and let $q$ be the corresponding $k$. Then $a = bq + r$. We need $r < b$: if $r \geq b$, then $r - b \geq 0$ and $r - b = a - b(q+1) \in S$, contradicting minimality of $r$. $\square$

**Proof (Uniqueness)**: Suppose $a = bq_1 + r_1 = bq_2 + r_2$ with $0 \leq r_1, r_2 < b$. Then $b(q_1 - q_2) = r_2 - r_1$. The left side is divisible by $b$; the right side satisfies $|r_2 - r_1| < b$. So $r_2 - r_1 = 0$, giving $r_1 = r_2$ and then $q_1 = q_2$. $\square$

**Notation**: $q = a \text{ div } b$ and $r = a \bmod b$ (or $r = a \% b$ in programming).

The Division Algorithm is the engine of the Euclidean algorithm, of Chinese Remainder Theorem, of modular arithmetic, and of countless other number-theoretic constructions.

## Divisibility and Proofs: Three Worked Examples

**Example 1**: Prove that $3 \mid n^3 - n$ for all integers $n$.

*Proof 1 (factoring)*: $n^3 - n = n(n^2 - 1) = n(n-1)(n+1) = (n-1) \cdot n \cdot (n+1)$. This is the product of three consecutive integers. Among any three consecutive integers, exactly one is divisible by 3. So their product is divisible by 3. $\square$

*Proof 2 (cases mod 3)*: Either $n \equiv 0, 1,$ or $2 \pmod{3}$.
- If $n \equiv 0$: then $n^3 - n \equiv 0 - 0 = 0 \pmod 3$.
- If $n \equiv 1$: then $n^3 - n \equiv 1 - 1 = 0 \pmod 3$.
- If $n \equiv 2$: then $n^3 - n \equiv 8 - 2 = 6 \equiv 0 \pmod 3$. $\square$

Both proofs are valid; the first is more elegant, the second more mechanical.

**Example 2**: Prove that if $7 \mid a$ and $7 \mid b$, then $7 \mid (3a - 2b)$.

*Proof*: By linearity: $7 \mid a$ and $7 \mid b$, so $7 \mid (3 \cdot a + (-2) \cdot b) = 3a - 2b$. $\square$

This is a one-line proof using the linearity property.

**Example 3**: Prove that if $n$ is odd, then $8 \mid n^2 - 1$.

*Proof*: If $n$ is odd, write $n = 2k + 1$ for some integer $k$. Then:
$$n^2 - 1 = (2k+1)^2 - 1 = 4k^2 + 4k + 1 - 1 = 4k^2 + 4k = 4k(k+1)$$

Among $k$ and $k+1$, one is even (consecutive integers). So $k(k+1) = 2m$ for some integer $m$. Therefore $n^2 - 1 = 4 \cdot 2m = 8m$, and $8 \mid n^2 - 1$. $\square$

## Lean 4 Proof of a Divisibility Lemma

```lean
-- a ∣ b is Lean's notation for divisibility
-- It unfolds to: ∃ k, b = a * k

theorem dvd_linear_comb (a b c : Int) (h1 : a ∣ b) (h2 : a ∣ c) (m n : Int) :
    a ∣ m * b + n * c := by
  obtain ⟨k1, hk1⟩ := h1
  obtain ⟨k2, hk2⟩ := h2
  exact ⟨m * k1 + n * k2, by rw [hk1, hk2]; ring⟩

-- The Division Algorithm in Lean (via Mathlib)
-- Int.ediv and Int.emod give Euclidean division
example (a b : Int) (hb : 0 < b) : a = b * (a / b) + a % b := by
  exact (Int.ediv_add_emod a b).symm
```

## Connection to Everything Else

Divisibility is the seed from which an enormous tree grows:

- **Primes** are numbers whose only positive divisors are 1 and themselves — the *atoms* of multiplication (section 02)
- **GCD** is the largest common divisor of two numbers — computable by the Euclidean algorithm (section 03)
- **Modular arithmetic** reduces integers modulo $n$, studying the remainder structure (section 04)
- **Cryptography**: RSA, Diffie-Hellman, and elliptic curve methods all rest on the difficulty of factoring and the structure of divisibility
- **Algebraic number theory**: Divisibility in rings other than $\mathbb{Z}$ reveals why unique factorization can fail — and how to fix it with ideals

Every concept in this chapter traces its roots back to the simple question: does $a$ divide $b$ evenly?

## Exercises
See [problems/ch08_number_theory/01_divisibility_exercises.md](../../../problems/ch08_number_theory/01_divisibility_exercises.md)
