# Uniqueness Proofs

## Proving Exactly One Thing Exists

A **uniqueness proof** establishes that at most one object satisfies a given property. Combined with an existence proof, it gives a unique existence result:

$$\exists! x\, P(x) \quad \text{means} \quad \exists x\, P(x) \wedge \forall x\, \forall y\, (P(x) \wedge P(y) \to x = y)$$

"There exists exactly one $x$ satisfying $P$."

## Standard Strategies

**Strategy 1: Assume two and show equal**.
Suppose $x$ and $y$ both satisfy $P$. Derive $x = y$.

**Example**: The identity element of a group is unique.

*Proof*: Suppose $e$ and $e'$ are both identity elements. Then $e = e \cdot e' = e'$ (using $e'$ as identity for the first step, $e$ as identity for the second). $\square$

**Strategy 2: Assume two and derive contradiction**.
Suppose $x \neq y$ both satisfy $P$. Derive a contradiction.

**Example**: A non-zero polynomial of degree $n$ has at most $n$ roots.

*Proof*: If a polynomial of degree $n$ had $n+1$ roots, by the Factor Theorem it would be divisible by $n+1$ distinct linear factors, making its degree at least $n+1$ — contradiction. $\square$

## Examples of Unique Existence

- **GCD**: $\gcd(a, b)$ is unique (the largest common divisor)
- **Division algorithm**: quotient and remainder are unique (given $a = bq + r$, $0 \leq r < b$)
- **Least upper bound**: in a complete ordered field, the supremum of a bounded set is unique
- **Prime factorization**: unique (up to order) by the Fundamental Theorem of Arithmetic

## In Lean 4

```lean
-- Unique existence statement
example : ∃! n : ℕ, n + n = 0 := by
  use 0
  constructor
  · rfl
  · intro m hm; omega  -- show m = 0 from m + m = 0

-- Uniqueness of the GCD (by antisymmetry of divisibility):
theorem gcd_unique (d a b : ℕ) (h1 : d ∣ a) (h2 : d ∣ b)
    (hmax : ∀ e, e ∣ a → e ∣ b → e ∣ d) : d = Nat.gcd a b :=
  Nat.dvd_antisymm (hmax _ (Nat.gcd_dvd_left a b) (Nat.gcd_dvd_right a b))
    (Nat.dvd_gcd h1 h2)
```

## Exercises
See [problems/ch05_proof_strategies/](../../../problems/ch05_proof_strategies/)
