# √2 Is Irrational: Paper Proof

## Theorem
√2 ∉ ℚ. Equivalently: there is no pair of integers (p, q) with q ≠ 0 and (p/q)² = 2.

## Proof (by contradiction)

Suppose √2 = p/q where p, q ∈ ℤ, q ≠ 0, and gcd(p, q) = 1 (the fraction is in lowest terms).

Then (p/q)² = 2, so p² = 2q².

Since 2 | p², and 2 is prime, 2 | p. (Lemma: if prime p | n², then p | n.)

Write p = 2k. Then (2k)² = 2q², so 4k² = 2q², so q² = 2k².

Therefore 2 | q², and by the same lemma, 2 | q.

But then 2 | gcd(p, q), contradicting gcd(p, q) = 1. □

## Lemma Used
**Lemma**: If p is prime and p | n², then p | n.
**Proof**: By prime factorization (Fundamental Theorem of Arithmetic), if p appears in
the factorization of n², it must appear in n's factorization (since prime factorization
is unique, and squaring doubles each prime's exponent). □

## Alternative: Via the Rational Root Theorem
√2 is a root of x² - 2 = 0. By the Rational Root Theorem, any rational root p/q (in lowest
terms) must have p | 2 and q | 1, so the candidates are ±1, ±2. None of these squares to 2. □

## Historical Note
The discovery that √2 is irrational allegedly caused a crisis in the Pythagorean school,
which held that all of geometry could be understood in terms of ratios of whole numbers.
One legend says Hippasus of Metapontum was drowned for revealing this secret.
