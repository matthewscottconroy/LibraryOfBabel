"""
Set Operations in Python
Chapter 6, Section 2

Python's built-in set type demonstrates the core set operations.
We also implement a simple predicate-based 'Set' class for infinite conceptual sets.
"""

from typing import Callable, TypeVar, Iterable

T = TypeVar('T')


# ── Finite sets using Python built-ins ────────────────────────────────────────

A = {1, 2, 3, 4, 5}
B = {3, 4, 5, 6, 7}
U = set(range(1, 11))   # universal set for this example

print("A =", A)
print("B =", B)
print("A ∪ B =", A | B)
print("A ∩ B =", A & B)
print("A \ B =", A - B)
print("B \ A =", B - A)
print("A △ B (symmetric diff) =", A ^ B)
print("Complement of A in U =", U - A)
print()

# Subset / superset
print("A ⊆ {1,2,3,4,5,6}:", A.issubset({1,2,3,4,5,6}))
print("U ⊇ A:", U.issuperset(A))
print()

# Power set
def power_set(s):
    """Return the power set of a finite set s."""
    lst = list(s)
    n = len(lst)
    return {frozenset(lst[j] for j in range(n) if i & (1 << j)) for i in range(2 ** n)}

print("Power set of {1,2,3}:")
for s in sorted(power_set({1, 2, 3}), key=lambda x: (len(x), sorted(x))):
    print(" ", set(s))
print()

# Cartesian product
from itertools import product as cart_product
print("A × B (first 5):", list(cart_product({1,2}, {3,4})))
print()

# ── Predicate-based sets (conceptual, for infinite sets) ──────────────────────

class PredicateSet:
    """A set defined by a predicate. Membership testing only — not enumerable."""
    def __init__(self, predicate: Callable[[T], bool], description: str = ""):
        self.predicate = predicate
        self.description = description

    def __contains__(self, item):
        return self.predicate(item)

    def __and__(self, other):
        return PredicateSet(lambda x: x in self and x in other,
                           f"({self.description} ∩ {other.description})")

    def __or__(self, other):
        return PredicateSet(lambda x: x in self or x in other,
                           f"({self.description} ∪ {other.description})")

    def complement(self):
        return PredicateSet(lambda x: x not in self, f"¬{self.description}")

    def __repr__(self):
        return f"Set({self.description})"


evens = PredicateSet(lambda n: n % 2 == 0, "Even")
positives = PredicateSet(lambda n: n > 0, "Positive")
even_positives = evens & positives

print("4 ∈ Even:", 4 in evens)
print("3 ∈ Even:", 3 in evens)
print("4 ∈ Even ∩ Positive:", 4 in even_positives)
print("-2 ∈ Even ∩ Positive:", -2 in even_positives)
