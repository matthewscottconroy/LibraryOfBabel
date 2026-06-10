# Prime Numbers

## Overview
A prime number has no divisors other than 1 and itself. Primes are the multiplicative
atoms of the integers. Their distribution, properties, and role in factorization have
fascinated mathematicians for millennia and are central to modern cryptography.

## Learning Objectives
- Define prime and composite numbers
- Prove there are infinitely many primes (multiple proofs)
- Understand the role of primes in the Fundamental Theorem of Arithmetic

## Definition
p > 1 is **prime** iff its only positive divisors are 1 and p.
p > 1 is **composite** iff it has a divisor d with 1 < d < p.
1 is neither prime nor composite.

## Infinitely Many Primes (Euclid's Proof)
Suppose p₁,...,pₙ are all the primes. Let N = p₁·p₂·...·pₙ + 1.
N > 1, so it has a prime factor p. But p ∤ N (since N ≡ 1 mod pᵢ for each i).
Contradiction. ∎

Formal version: `proofs/04_number_theory/infinitely_many_primes/`

## The Sieve of Eratosthenes
An efficient algorithm for finding all primes up to n:
1. List 2, 3, ..., n
2. Repeatedly: take the smallest unmarked number (prime), mark all its multiples
3. Remaining unmarked numbers are prime

## Primes in Cryptography
- RSA encryption: security relies on difficulty of factoring large semiprimes (n = p·q)
- Diffie-Hellman: uses discrete logarithm in ℤ_p*
- Elliptic curve cryptography: uses group structure over finite fields 𝔽_p

## Python
See `problems/ch08_number_theory/01_divisibility_exercises.md` for sieve implementation.

## Exercises
See `problems/ch08_number_theory/03_prime_proofs.md`
