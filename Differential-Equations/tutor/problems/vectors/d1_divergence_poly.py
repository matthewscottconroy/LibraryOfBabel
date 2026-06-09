"""
Vector Calculus — divergence (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "vectors"


def generate() -> Problem:
    a, b, c = [random.randint(1, 4) for _ in range(3)]
    p = random.choice([1, 2, 3])
    # F = (ax^p, bx*y, cz)
    F1 = a * x**p
    F2 = b * x * y
    F3 = c * z
    div_val = sp.diff(F1, x) + sp.diff(F2, y) + sp.diff(F3, z)
    return Problem(
        topic=TOPIC, subtopic="divergence", difficulty=1,
        question=(
            f"Compute the divergence of:\n\n"
            f"  F = ({F1}, {F2}, {F3})"
        ),
        answer=div_val,
        hint="div F = ∂P/∂x + ∂Q/∂y + ∂R/∂z",
        explanation=(
            f"∂/∂x({F1}) = {sp.diff(F1,x)}\n"
            f"∂/∂y({F2}) = {sp.diff(F2,y)}\n"
            f"∂/∂z({F3}) = {sp.diff(F3,z)}\n"
            f"div F = {div_val}"
        ),
        problem_type="symbolic",
    )
