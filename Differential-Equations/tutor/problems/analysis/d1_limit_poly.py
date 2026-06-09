"""
Real Analysis — limits (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    a = random.choice([-3, -2, -1, 1, 2, 3])
    coeffs = [random.randint(-4, 4) for _ in range(3)]
    f = coeffs[0]*x**2 + coeffs[1]*x + coeffs[2]
    ans = f.subs(x, a)
    return Problem(
        topic=TOPIC, subtopic="limits", difficulty=1,
        question=(
            f"Evaluate the limit:\n\n"
            f"  lim  ({sp.pretty(f, use_unicode=True)})\n"
            f"  x→{a}"
        ),
        answer=ans,
        hint="For polynomials, direct substitution always works.",
        explanation=(
            f"Since f(x) = {f} is a polynomial, it is continuous everywhere.\n"
            f"Therefore lim_{{x→{a}}} f(x) = f({a}) = {ans}."
        ),
        problem_type="numeric",
    )
