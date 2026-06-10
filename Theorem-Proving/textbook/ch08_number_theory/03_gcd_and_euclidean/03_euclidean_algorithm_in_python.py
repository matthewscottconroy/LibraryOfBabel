"""
GCD and the Euclidean Algorithm
Chapter 8, Section 3

The Euclidean algorithm is one of the oldest algorithms (c. 300 BCE).
It computes gcd(a,b) and, in its extended form, finds Bézout coefficients.
"""


def gcd_recursive(a: int, b: int) -> int:
    """Euclidean algorithm: gcd(a,b) = gcd(b, a mod b)."""
    return a if b == 0 else gcd_recursive(b, a % b)


def gcd_iterative(a: int, b: int) -> int:
    while b:
        a, b = b, a % b
    return a


def extended_euclidean(a: int, b: int):
    """
    Returns (g, x, y) such that g = gcd(a,b) and ax + by = g.
    Proof of Bézout's identity is constructive via this algorithm.
    """
    if b == 0:
        return a, 1, 0
    g, x, y = extended_euclidean(b, a % b)
    return g, y, x - (a // b) * y


def lcm(a: int, b: int) -> int:
    """lcm(a,b) = |a*b| / gcd(a,b)."""
    return abs(a * b) // gcd_iterative(a, b)


if __name__ == '__main__':
    pairs = [(48, 18), (100, 75), (17, 5), (1071, 462)]
    print("=== GCD ===")
    for a, b in pairs:
        g = gcd_iterative(a, b)
        print(f"  gcd({a}, {b}) = {g}")

    print()
    print("=== Bézout Coefficients ===")
    for a, b in pairs:
        g, x, y = extended_euclidean(a, b)
        print(f"  {a}·{x} + {b}·{y} = {a*x + b*y} = gcd = {g}")

    print()
    print("=== LCM ===")
    for a, b in pairs:
        print(f"  lcm({a}, {b}) = {lcm(a, b)}")
