"""
Modular Arithmetic in Python
Chapter 8, Section 2

Python's built-in % operator and the Fraction module support modular arithmetic.
We demonstrate key concepts: congruences, CRT, Fermat's little theorem, modular inverse.
"""

from math import gcd
from functools import reduce


def mod_exp(base: int, exp: int, mod: int) -> int:
    """Fast modular exponentiation via repeated squaring. O(log exp)."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp //= 2
        base = (base * base) % mod
    return result


def mod_inverse(a: int, m: int) -> int:
    """Modular multiplicative inverse of a mod m (via extended Euclidean).
    Requires gcd(a, m) = 1."""
    g, x, _ = extended_gcd(a, m)
    if g != 1:
        raise ValueError(f"gcd({a},{m}) = {g} ≠ 1; inverse does not exist")
    return x % m


def extended_gcd(a: int, b: int):
    """Returns (gcd, x, y) such that ax + by = gcd(a,b)."""
    if b == 0:
        return a, 1, 0
    g, x, y = extended_gcd(b, a % b)
    return g, y, x - (a // b) * y


def chinese_remainder_theorem(remainders, moduli):
    """
    Solve system x ≡ rᵢ (mod mᵢ) where all mᵢ are pairwise coprime.
    Returns (x, M) where M = ∏mᵢ and 0 ≤ x < M.
    """
    M = reduce(lambda a, b: a * b, moduli)
    x = 0
    for r, m in zip(remainders, moduli):
        Mi = M // m
        x += r * Mi * mod_inverse(Mi, m)
    return x % M, M


def fermat_little_theorem_demo(p: int, a: int) -> bool:
    """
    Fermat's Little Theorem: if p is prime and p ∤ a, then a^(p-1) ≡ 1 (mod p).
    Returns True if the theorem holds for this a and p.
    """
    if gcd(a, p) != 1:
        return False
    return mod_exp(a, p - 1, p) == 1


if __name__ == '__main__':
    print("=== Modular Exponentiation ===")
    print(f"3^100 mod 7 = {mod_exp(3, 100, 7)}")
    print(f"2^10 mod 1000 = {mod_exp(2, 10, 1000)}")

    print()
    print("=== Modular Inverse ===")
    print(f"3⁻¹ mod 7 = {mod_inverse(3, 7)}")
    print(f"Check: 3 * {mod_inverse(3,7)} mod 7 = {3 * mod_inverse(3,7) % 7}")

    print()
    print("=== Chinese Remainder Theorem ===")
    # x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7)
    x, M = chinese_remainder_theorem([2, 3, 2], [3, 5, 7])
    print(f"x ≡ 2 (mod 3), ≡ 3 (mod 5), ≡ 2 (mod 7): x = {x} (mod {M})")
    print(f"Check: {x}%3={x%3}, {x}%5={x%5}, {x}%7={x%7}")

    print()
    print("=== Fermat's Little Theorem ===")
    p = 17
    for a in range(1, 8):
        holds = fermat_little_theorem_demo(p, a)
        print(f"  {a}^{p-1} ≡ 1 (mod {p}): {holds}")

    print()
    print("=== RSA Mini-Demo ===")
    # Choose small primes p=61, q=53
    p_rsa, q_rsa = 61, 53
    n = p_rsa * q_rsa
    phi = (p_rsa - 1) * (q_rsa - 1)
    e = 17  # public exponent, gcd(17, phi) = 1
    d = mod_inverse(e, phi)  # private exponent
    message = 65
    ciphertext = mod_exp(message, e, n)
    decrypted  = mod_exp(ciphertext, d, n)
    print(f"n={n}, e={e}, d={d}")
    print(f"Encrypt {message}: {ciphertext}")
    print(f"Decrypt {ciphertext}: {decrypted} (original: {message})")
