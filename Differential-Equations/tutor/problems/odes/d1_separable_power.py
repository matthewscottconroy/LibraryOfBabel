"""
Ordinary Differential Equations — separable (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """dy/dx = ax^n  (pure quadrature)  with IC"""
    n = random.choice([1, 2, 3])
    a = random.choice([-2, -1, 1, 2, 3])
    y0 = random.choice([0, 1, 2, -1])
    ans = sp.Rational(a, n+1) * x**(n+1) + y0
    return Problem(
        topic=TOPIC, subtopic="separable", difficulty=1,
        question=(
            f"Solve the IVP:\n\n"
            f"  dy/dx = {a}x^{n},    y(0) = {y0}"
        ),
        answer=ans,
        hint="Integrate both sides directly (the right side has no y).",
        explanation=(
            f"dy = {a}x^{n} dx\n"
            f"Integrate: y = {a}·x^{n+1}/{n+1} + C\n"
            f"Apply y(0) = {y0}: C = {y0}\n"
            f"∴ y = {sp.pretty(ans, use_unicode=True)}"
        ),
        problem_type="symbolic",
    )
