"""
Multivariable Calculus — partial derivatives (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    a, b, c = [random.randint(1, 4) for _ in range(3)]
    d = random.randint(1, 3)
    f = a*x**d*y**2 + b*x*y + c*y**3
    ans = sp.diff(f, x)
    return Problem(
        topic=TOPIC, subtopic="partial_derivatives", difficulty=1,
        question=(
            f"Compute the partial derivative with respect to x:\n\n"
            f"  f(x, y) = {f}\n\n  ∂f/∂x = ?"
        ),
        answer=ans,
        hint="Treat y as a constant and differentiate with respect to x only.",
        explanation=f"Treating y as constant:\n  ∂f/∂x = {sp.pretty(ans, use_unicode=True)}",
        problem_type="symbolic",
    )
