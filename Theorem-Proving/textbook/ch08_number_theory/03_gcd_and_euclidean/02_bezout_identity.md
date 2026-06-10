# Bézout's Identity

> "The theory of divisibility is entirely determined by the single identity: gcd(a,b) = sa + tb."
> — André Weil (paraphrased)

## The Fundamental Identity

**Bézout's Identity** (also called Bézout's lemma): For any integers $a$ and $b$, not both zero, there exist integers $s$ and $t$ such that:
$$\gcd(a, b) = sa + tb$$

Moreover, $\gcd(a, b)$ is the *smallest positive* integer expressible as $sa + tb$ for integers $s, t$.

This is remarkable: the GCD — a purely divisibility-theoretic concept — can be expressed as a linear combination of the two numbers. And conversely, every integer expressible as a linear combination of $a$ and $b$ is a multiple of $\gcd(a, b)$.

**Example**: $\gcd(12, 8) = 4$. Can we write $4 = 12s + 8t$?

Yes: $4 = 12 \cdot 1 + 8 \cdot (-1)$ (check: $12 - 8 = 4$). ✓

**Example**: $\gcd(35, 13) = 1$. Find $s, t$ with $35s + 13t = 1$.

Using back-substitution from the extended Euclidean algorithm:
$35 = 2 \cdot 13 + 9$, $13 = 1 \cdot 9 + 4$, $9 = 2 \cdot 4 + 1$.

Back-substituting: $1 = 9 - 2 \cdot 4 = 9 - 2(13 - 9) = 3 \cdot 9 - 2 \cdot 13 = 3(35 - 2 \cdot 13) - 2 \cdot 13 = 3 \cdot 35 - 8 \cdot 13$.

So $s = 3$, $t = -8$: $35 \cdot 3 + 13 \cdot (-8) = 105 - 104 = 1$. ✓

## The Extended Euclidean Algorithm

The **extended Euclidean algorithm** computes both $\gcd(a, b)$ and the Bézout coefficients $s, t$:

```python
def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
    # Returns (gcd, s, t) such that a*s + b*t = gcd
    if b == 0:
        return a, 1, 0
    g, s1, t1 = extended_gcd(b, a % b)
    # From recursive call: b*s1 + (a%b)*t1 = g
    # a%b = a - (a//b)*b, so: b*s1 + (a - (a//b)*b)*t1 = g
    # => a*t1 + b*(s1 - (a//b)*t1) = g
    return g, t1, s1 - (a // b) * t1

g, s, t = extended_gcd(35, 13)
print(f"gcd(35,13) = {g}, s = {s}, t = {t}")
print(f"Verify: 35*{s} + 13*{t} = {35*s + 13*t}")
```

This runs in $O(\log(\min(a, b)))$ steps — the same as the basic Euclidean algorithm.

## Proof of Bézout's Identity

**Proof**: Consider the set $S = \{xa + yb \mid x, y \in \mathbb{Z}\} \cap \mathbb{Z}_{>0}$ — all positive integers expressible as integer linear combinations of $a$ and $b$.

$S$ is non-empty (e.g., $|a|$ or $|b|$ is positive and expressible). Let $d$ be the smallest element of $S$, with $d = sa + tb$.

**Claim**: $d = \gcd(a, b)$.

- $d \mid a$: By the division algorithm, $a = dq + r$ with $0 \leq r < d$. Then $r = a - dq = a - (sa+tb)q = a(1-sq) + b(-tq)$ — an integer linear combination of $a$ and $b$. If $r > 0$, then $r \in S$, contradicting minimality of $d$. So $r = 0$ and $d \mid a$. Similarly $d \mid b$.

- $d$ is the largest such divisor: if $c \mid a$ and $c \mid b$, then $c \mid (sa + tb) = d$, so $c \leq d$.

Therefore $d = \gcd(a, b)$. $\square$

## Key Applications

**Modular inverses**: The inverse of $a$ modulo $n$ exists iff $\gcd(a, n) = 1$, in which case $s$ from Bézout's identity ($sa + tn = 1$) gives $a \cdot s \equiv 1 \pmod{n}$ — so $s \bmod n$ is the modular inverse of $a$.

**Solving linear Diophantine equations**: $ax + by = c$ has an integer solution iff $\gcd(a, b) \mid c$. If so, the general solution is $x = x_0 + (b/d)k$, $y = y_0 - (a/d)k$ for any integer $k$, where $d = \gcd(a, b)$ and $(x_0, y_0)$ is one particular solution.

**Proving Euclid's lemma**: If $p$ is prime and $p \mid ab$, then $p \mid a$ or $p \mid b$. Proof: if $p \nmid a$, then $\gcd(p, a) = 1$, so by Bézout $1 = sp + ta$. Multiply by $b$: $b = spb + tab$. Since $p \mid p$ and $p \mid ab$, we have $p \mid b$. This uses Bézout crucially!

**Chinese Remainder Theorem**: The CRT algorithm constructs the simultaneous solution using Bézout coefficients for the pairwise coprime moduli.

## In Lean 4

```lean
import Mathlib.Data.Int.GCD

-- Bézout's identity in Lean
#check Int.gcd_eq_gcd_ab  -- ↑(Int.gcd a b) = a * Int.gcdA a b + b * Int.gcdB a b

-- Modular inverse via Bézout
example (a n : ℤ) (h : Int.gcd a n = 1) : ∃ s : ℤ, a * s ≡ 1 [ZMOD n] := by
  obtain ⟨s, t, hst⟩ := Int.gcd_eq_one_iff_coprime.mp h
  exact ⟨s, by linarith [hst]⟩
```

## Exercises
See [problems/ch08_number_theory/03_gcd_exercises.md](../../../problems/ch08_number_theory/03_gcd_exercises.md)
