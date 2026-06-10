# Greatest Common Divisor and Least Common Multiple

## Definitions

The **greatest common divisor** of integers $a$ and $b$ (not both zero):
$$\gcd(a, b) = \max\{d \in \mathbb{Z}_{>0} \mid d \mid a \text{ and } d \mid b\}$$

The **least common multiple**:
$$\text{lcm}(a, b) = \min\{m \in \mathbb{Z}_{>0} \mid a \mid m \text{ and } b \mid m\}$$

**Key relation**: $\gcd(a, b) \cdot \text{lcm}(a, b) = |ab|$ for nonzero $a, b$.

## The Euclidean Algorithm

Computing $\gcd(a, b)$ efficiently uses the key identity:
$$\gcd(a, b) = \gcd(b, a \bmod b)$$

**Proof**: Every common divisor of $a$ and $b$ divides $a - qb = a \bmod b$ (linearity of divisibility). So the sets of common divisors of $(a, b)$ and $(b, a \bmod b)$ are the same. Their greatest elements agree. $\square$

```python
def gcd(a: int, b: int) -> int:
    while b != 0:
        a, b = b, a % b
    return abs(a)

def lcm(a: int, b: int) -> int:
    return abs(a * b) // gcd(a, b)
```

**Complexity**: $O(\log(\min(a, b)))$ steps — the number of steps is bounded by the number of digits in $\min(a, b)$ (Fibonacci numbers are the worst case).

## Properties

- $\gcd(a, 0) = a$, $\gcd(0, 0) = 0$
- $\gcd(a, b) = \gcd(b, a)$ (symmetric)
- $\gcd(a, b) \mid \gcd(ma, mb) = m \cdot \gcd(a, b)$ for $m \geq 0$
- $\gcd(a, b) = 1$ (coprime): $a$ and $b$ share no common prime factor

## Coprimeness and Modular Arithmetic

$\gcd(a, n) = 1$ iff $a$ has a multiplicative inverse modulo $n$ iff $a$ and $n$ are coprime.

Euler's totient: $\phi(n) = |\{a \mid 1 \leq a \leq n, \gcd(a, n) = 1\}|$.

## Lean 4

```lean
#eval Nat.gcd 12 8   -- 4
#eval Nat.lcm 4 6    -- 12

-- GCD properties in Mathlib
#check Nat.gcd_comm       -- gcd a b = gcd b a
#check Nat.gcd_dvd_left   -- gcd a b ∣ a
#check Nat.dvd_gcd        -- if d∣a and d∣b then d∣gcd a b
```

## Exercises
See [problems/ch08_number_theory/03_gcd_exercises.md](../../../problems/ch08_number_theory/03_gcd_exercises.md)
