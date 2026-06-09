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
    """dy/dx = axy,  y(0) = y0"""
    a = random.choice([-2, -1, 1, 2])
    y0 = random.choice([1, 2, 3])
    # y = y0·exp(a·x²/2)
    ans = y0 * sp.exp(sp.Rational(a, 2) * x**2)
    return Problem(
        topic=TOPIC, subtopic="separable", difficulty=1,
        question=(
            f"Solve the IVP:\n\n"
            f"  dy/dx = {a}xy,    y(0) = {y0}"
        ),
        answer=ans,
        hint="Separate: dy/y = ax dx, then integrate both sides.",
        explanation=(
            f"Separate: dy/y = {a}x dx\n"
            f"Integrate: ln|y| = {a}x²/2 + C\n"
            f"y = A·e^({a}x²/2)\n"
            f"Apply y(0) = {y0}: A = {y0}\n"
            f"∴ y = {y0}·e^({a}x²/2)"
        ),
        problem_type="symbolic",
    )
