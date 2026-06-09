"""
Complex Analysis — complex numbers (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    a = random.choice([-3, -1, 1, 2, 3, 4])
    b = random.choice([-4, -3, -1, 1, 3, 4])
    mod_sq = a**2 + b**2
    mod = sp.sqrt(mod_sq)
    b_sign = "+" if b >= 0 else "−"
    return Problem(
        topic=TOPIC, subtopic="complex_numbers", difficulty=1,
        question=(
            f"Compute the modulus:\n\n"
            f"  |{a} {b_sign} {abs(b)}i|"
        ),
        answer=mod,
        hint="|a + bi| = √(a² + b²)",
        explanation=(
            f"|{a} {b_sign} {abs(b)}i| = √({a}² + {b}²) = √{mod_sq} = {sp.pretty(mod, use_unicode=True)}"
        ),
        problem_type="numeric",
    )
