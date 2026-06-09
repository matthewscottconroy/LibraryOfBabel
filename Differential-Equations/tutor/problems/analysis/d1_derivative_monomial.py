"""
Real Analysis — differentiation (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    n_val = random.choice([2, 3, 4, 5])
    c = random.choice([-3, -2, 2, 3, 4])
    f = c * x**n_val
    ans = sp.diff(f, x)
    return Problem(
        topic=TOPIC, subtopic="differentiation", difficulty=1,
        question=f"Differentiate:\n\n  f(x) = {c}x^{n_val}\n\nFind f'(x).",
        answer=ans,
        hint=f"Power rule: d/dx(xⁿ) = nxⁿ⁻¹.",
        explanation=f"f'(x) = {c}·{n_val}·x^{n_val-1} = {ans}",
        problem_type="symbolic",
    )
