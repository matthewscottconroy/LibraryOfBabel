"""Recursion and recursive definitions in Python."""
# All recursive functions here have explicit termination arguments.

from functools import lru_cache
from typing import TypeVar, Callable

# --- Structural recursion on natural numbers ---

def factorial(n: int) -> int:
    # Decreases on n; terminates by well-foundedness of <
    if n == 0:
        return 1
    return n * factorial(n - 1)

def fibonacci(n: int) -> int:
    # Two recursive calls on strictly smaller arguments
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

@lru_cache(maxsize=None)
def fibonacci_memo(n: int) -> int:
    if n <= 1:
        return n
    return fibonacci_memo(n - 1) + fibonacci_memo(n - 2)

# --- Structural recursion on lists ---

def my_length(lst: list) -> int:
    if not lst:
        return 0
    return 1 + my_length(lst[1:])

def my_sum(lst: list) -> int:
    if not lst:
        return 0
    return lst[0] + my_sum(lst[1:])

def flatten(nested: list) -> list:
    if not nested:
        return []
    head, *tail = nested
    if isinstance(head, list):
        return flatten(head) + flatten(tail)
    return [head] + flatten(tail)

# --- Mutual recursion ---

def is_even(n: int) -> bool:
    if n == 0:
        return True
    return is_odd(n - 1)

def is_odd(n: int) -> bool:
    if n == 0:
        return False
    return is_even(n - 1)

# --- Ackermann function: terminates but not primitive recursive ---

def ackermann(m: int, n: int) -> int:
    # Terminates on the lexicographic ordering (m, n)
    if m == 0:
        return n + 1
    if n == 0:
        return ackermann(m - 1, 1)
    return ackermann(m - 1, ackermann(m, n - 1))

# Ackermann grows faster than any primitive recursive function.
# ackermann(3, 4) = 125, ackermann(4, 0) = 13, ackermann(4, 2) is astronomical.

# --- Recursion on trees ---

class Tree:
    pass

class Leaf(Tree):
    def __init__(self, value):
        self.value = value

class Node(Tree):
    def __init__(self, left: Tree, right: Tree):
        self.left = left
        self.right = right

def tree_size(t: Tree) -> int:
    if isinstance(t, Leaf):
        return 1
    return 1 + tree_size(t.left) + tree_size(t.right)

def tree_height(t: Tree) -> int:
    if isinstance(t, Leaf):
        return 0
    return 1 + max(tree_height(t.left), tree_height(t.right))

def tree_sum(t: Tree) -> int:
    if isinstance(t, Leaf):
        return t.value
    return tree_sum(t.left) + tree_sum(t.right)

# --- Fixed-point combinator (Y combinator in Python) ---
# The Y combinator makes anonymous recursion possible.

Y = lambda f: (lambda x: f(lambda v: x(x)(v)))(lambda x: f(lambda v: x(x)(v)))

fact = Y(lambda self: lambda n: 1 if n == 0 else n * self(n - 1))
assert fact(5) == 120

# --- Termination and well-foundedness ---

def euclid_gcd(a: int, b: int) -> int:
    # Terminates: b decreases at each step (by the division algorithm)
    if b == 0:
        return a
    return euclid_gcd(b, a % b)

def collatz(n: int) -> list[int]:
    # Collatz conjecture: this terminates for all known n, but unprovable in ZFC?
    # (Conjectured but unproven)
    if n == 1:
        return [1]
    if n % 2 == 0:
        return [n] + collatz(n // 2)
    return [n] + collatz(3 * n + 1)
