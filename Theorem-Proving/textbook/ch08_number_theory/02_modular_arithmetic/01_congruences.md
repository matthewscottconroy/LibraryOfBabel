# Congruences and Modular Arithmetic

> "Gauss introduced the language of congruences in *Disquisitiones Arithmeticae* (1801) and thereby transformed number theory from a collection of clever tricks into a systematic science."
> — Standard history of mathematics

## A New Language for Old Ideas

The idea behind modular arithmetic is ancient — people have always worked with "clock arithmetic" intuitively. $7 + 8 = 15$, but 15 o'clock is 3 o'clock: we subtract 12 and work with the remainder. Days of the week cycle with period 7. Binary arithmetic works modulo 2.

Carl Friedrich Gauss gave this intuition precise algebraic language in 1801. His notation and theorems transformed elementary number theory from a collection of tricks into a structured algebraic theory.

## Definition: Congruence

Let $a, b \in \mathbb{Z}$ and $n \in \mathbb{Z}_{>0}$ (the **modulus**). We say $a$ is **congruent to $b$ modulo $n$**, written $a \equiv b \pmod{n}$, if $n \mid (a - b)$ — that is, $a$ and $b$ differ by a multiple of $n$.

$$a \equiv b \pmod{n} \iff n \mid (a - b) \iff \exists k \in \mathbb{Z},\; a - b = kn$$

**Examples:**
- $17 \equiv 2 \pmod{5}$ (since $17 - 2 = 15 = 3 \cdot 5$)
- $-3 \equiv 9 \pmod{6}$ (since $-3 - 9 = -12 = -2 \cdot 6$)
- $100 \equiv 0 \pmod{4}$ (since $100 = 25 \cdot 4$)

Alternative characterization: $a \equiv b \pmod{n}$ iff $a$ and $b$ have the same remainder when divided by $n$ — that is, $a \bmod n = b \bmod n$.

## Congruence is an Equivalence Relation

Congruence modulo $n$ is an **equivalence relation** on $\mathbb{Z}$:

**Reflexive**: $a \equiv a \pmod{n}$ (since $n \mid 0$) ✓

**Symmetric**: If $a \equiv b$ then $b \equiv a$ (since $n \mid (a-b)$ implies $n \mid (b-a) = -(a-b)$) ✓

**Transitive**: If $a \equiv b$ and $b \equiv c$, then $a \equiv c$ (since $n \mid (a-b)$ and $n \mid (b-c)$ gives $n \mid (a-b)+(b-c) = (a-c)$) ✓

The **equivalence class** of $a$ modulo $n$ is $[a]_n = \{b \in \mathbb{Z} \mid b \equiv a \pmod{n}\} = \{a, a \pm n, a \pm 2n, \ldots\}$.

There are exactly $n$ distinct equivalence classes: $[0]_n, [1]_n, \ldots, [n-1]_n$. The set of these classes is $\mathbb{Z}/n\mathbb{Z}$ (read "Z mod n") or $\mathbb{Z}_n$.

## Arithmetic of Congruences

The fundamental property: **congruence is compatible with arithmetic**.

**Theorem**: If $a \equiv a' \pmod{n}$ and $b \equiv b' \pmod{n}$, then:
- $a + b \equiv a' + b' \pmod{n}$
- $a - b \equiv a' - b' \pmod{n}$
- $a \cdot b \equiv a' \cdot b' \pmod{n}$

**Proof** (for multiplication): $a = a' + kn$ and $b = b' + jn$. Then:
$$ab = (a' + kn)(b' + jn) = a'b' + a'jn + b'kn + kjn^2 = a'b' + n(a'j + b'k + kjn)$$
So $n \mid (ab - a'b')$, giving $ab \equiv a'b' \pmod{n}$. $\square$

This compatibility means we can compute congruences by reducing at each step, keeping numbers small.

**Example**: What is $7^{100} \pmod{10}$?

$7^1 \equiv 7$, $7^2 \equiv 49 \equiv 9$, $7^3 \equiv 7 \cdot 9 = 63 \equiv 3$, $7^4 \equiv 7 \cdot 3 = 21 \equiv 1 \pmod{10}$.

Since $7^4 \equiv 1$, the pattern repeats with period 4: $7^{100} = 7^{4 \cdot 25} = (7^4)^{25} \equiv 1^{25} = 1 \pmod{10}$.

So the last digit of $7^{100}$ is 1.

## The Integers Modulo n: A Ring (and Sometimes a Field)

The set $\mathbb{Z}/n\mathbb{Z} = \{[0], [1], \ldots, [n-1]\}$ with the induced addition and multiplication forms a **commutative ring**. The compatibility theorem above shows these operations are well-defined.

**When is $\mathbb{Z}/n\mathbb{Z}$ a field?** Exactly when $n$ is prime.

**Theorem**: In $\mathbb{Z}/n\mathbb{Z}$, every nonzero element has a multiplicative inverse iff $n$ is prime.

**Proof**: $[a]$ has an inverse iff $\gcd(a, n) = 1$ (by Bézout's identity: $as + nt = 1$ gives $as \equiv 1 \pmod{n}$). If $n$ is prime, every $1 \leq a < n$ satisfies $\gcd(a, n) = 1$. Conversely, if $n$ is composite, say $n = ab$ with $1 < a < n$, then $[a]$ has no inverse (since $\gcd(a, n) = a > 1$). $\square$

The field $\mathbb{Z}/p\mathbb{Z}$ for prime $p$ is called $\mathbb{F}_p$ — a fundamental object in algebra and cryptography.

## Fermat's Little Theorem

**Theorem (Fermat)**: If $p$ is prime and $p \nmid a$, then $a^{p-1} \equiv 1 \pmod{p}$.

**Proof sketch**: The nonzero elements $\{1, 2, \ldots, p-1\}$ form a group under multiplication mod $p$. Multiplying each by $a$ (mod $p$) permutes these elements: the multiset $\{a \cdot 1, a \cdot 2, \ldots, a \cdot (p-1)\}$ is the same as $\{1, 2, \ldots, p-1\}$ (mod $p$). So their products are equal:
$$a^{p-1} \cdot (p-1)! \equiv (p-1)! \pmod{p}$$
Dividing by $(p-1)!$ (which is nonzero mod $p$): $a^{p-1} \equiv 1 \pmod{p}$. $\square$

**Application (RSA)**: The RSA decryption step $c^d \equiv m \pmod{n}$ follows from Euler's generalization of Fermat's theorem (using the totient $\phi(n)$ instead of $p-1$).

## Chinese Remainder Theorem

**Theorem (CRT)**: If $n_1, n_2, \ldots, n_k$ are pairwise coprime (i.e., $\gcd(n_i, n_j) = 1$ for $i \neq j$), then the system:
$$x \equiv a_1 \pmod{n_1}, \quad x \equiv a_2 \pmod{n_2}, \quad \ldots, \quad x \equiv a_k \pmod{n_k}$$
has a unique solution modulo $n_1 n_2 \cdots n_k$.

**Example**: Find $x$ with $x \equiv 2 \pmod{3}$, $x \equiv 3 \pmod{5}$, $x \equiv 2 \pmod{7}$.

Answer: $x \equiv 23 \pmod{105}$ (since $3 \cdot 5 \cdot 7 = 105$). Verify: $23 = 3 \cdot 7 + 2$ ✓, $23 = 5 \cdot 4 + 3$ ✓, $23 = 7 \cdot 3 + 2$ ✓.

**Applications**:
- Secret sharing (Shamir threshold scheme)
- Parallel computation in modular arithmetic (reduce large modulus to several small ones)
- Signal processing (Number Theoretic Transforms for fast multiplication)
- RSA (using $p$ and $q$ separately for efficient decryption via CRT)

## Lean 4

```lean
import Mathlib.Data.ZMod.Basic

-- Fermat's little theorem in Mathlib
#check ZMod.pow_card_sub_one_eq_one  -- a^(p-1) = 1 in ZMod p (for a ≠ 0)

-- Working in ZMod 7
example : (3 : ZMod 7) ^ 6 = 1 := by decide
example : (5 : ZMod 7) ^ 6 = 1 := by decide

-- The ring structure of ZMod n
example (n : ℕ) : CommRing (ZMod n) := inferInstance
example (p : ℕ) [Fact (Nat.Prime p)] : Field (ZMod p) := inferInstance
```

## Exercises
See [problems/ch08_number_theory/02_modular_arithmetic_exercises.md](../../../problems/ch08_number_theory/02_modular_arithmetic_exercises.md)
