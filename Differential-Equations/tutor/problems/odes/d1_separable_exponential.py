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
    """dy/dx = ay,  y(0) = y0"""
    a = random.choice([-3, -2, 2, 3])
    y0 = random.choice([1, 2, 3, 4])
    ans = y0 * sp.exp(a * x)
    sign = "+" if a > 0 else "−"
    return Problem(
        topic=TOPIC, subtopic="separable", difficulty=1,
        question=(
            f"Solve the IVP (initial value problem):\n\n"
            f"  dy/dx = {a}y,    y(0) = {y0}\n\n"
            f"Enter y as a function of x."
        ),
        answer=ans,
        hint="Separate variables: dy/y = a dx, then integrate both sides.",
        explanation=(
            f"Separate: dy/y = {a} dx\n"
            f"Integrate: ln|y| = {a}x + C\n"
            f"Exponentiate: y = Ae^({a}x)\n"
            f"Apply y(0) = {y0}: A = {y0}\n"
            f"∴ y(x) = {y0}·e^({a}x)"
        ),
        problem_type="symbolic",
    )
