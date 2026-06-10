"""
Computational exploration of infinitely many primes.
Verifies Euclid's construction for small cases.
"""

from math import prod
from sympy import factorint, isprime


def euclid_candidate(primes):
    """N = p₁·p₂·...·pₙ + 1 from Euclid's proof."""
    return prod(primes) + 1


def sieve(n):
    """Sieve of Eratosthenes: all primes up to n."""
    is_prime = [True] * (n + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(n**0.5) + 1):
        if is_prime[i]:
            for j in range(i*i, n+1, i):
                is_prime[j] = False
    return [i for i in range(2, n+1) if is_prime[i]]


if __name__ == '__main__':
    print("Euclid's construction: N = p₁·...·pₙ + 1")
    primes_so_far = []
    for _ in range(8):
        # Find next prime not yet in our list
        p = 2
        while p in primes_so_far or not isprime(p):
            p += 1
        primes_so_far.append(p)
        N = euclid_candidate(primes_so_far)
        factors = factorint(N)
        new_primes = [f for f in factors if f not in primes_so_far]
        print(f"  Primes: {primes_so_far}")
        print(f"  N = {N}, factors = {dict(factors)}, new prime(s): {new_primes}")
        print()

    print("First 50 primes (sieve):", sieve(230))
