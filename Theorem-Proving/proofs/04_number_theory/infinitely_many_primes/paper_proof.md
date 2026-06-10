# Infinitely Many Primes: Paper Proof

## Euclid's Proof (by contradiction)

**Theorem**: There are infinitely many prime numbers.

**Proof**: Suppose, for contradiction, that there are only finitely many primes:
p₁, p₂, ..., pₙ.

Let N = p₁ · p₂ · ... · pₙ + 1.

Since N > 1, it must have at least one prime divisor (by the Fundamental Theorem of Arithmetic).
Call this prime divisor p.

Now, p must be one of p₁, ..., pₙ (since we assumed these are all primes).
But p | N and p | p₁·p₂·...·pₙ, so p | (N - p₁·p₂·...·pₙ) = 1.

But no prime divides 1. Contradiction.

Therefore, there are infinitely many primes. □

## Euler's Proof (using the product formula)

Consider the divergence of the harmonic series:
∑_{n=1}^∞ 1/n = ∞

Euler showed: ∑_{n=1}^∞ 1/n = ∏_p 1/(1 - 1/p)

where the product is over all primes. If there were only finitely many primes, the product
would be finite, contradicting the divergence of the harmonic series.

This proof connects prime distribution to analysis — a deeper insight. □

## Notes
- Euclid's proof does NOT claim that p₁·p₂·...·pₙ + 1 is prime (it may be composite).
  It only claims it has a prime factor not in our list.
- The prime 2 is special: 2 · 3 · 5 + 1 = 31 (prime!), but 2 · 3 · 5 · 7 · 11 · 13 + 1 = 30031 = 59 · 509.
