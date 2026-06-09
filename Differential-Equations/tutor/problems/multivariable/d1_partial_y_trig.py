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
    a = random.choice([1, 2, 3])
    templates = [
        (sp.sin(x*y),       "sin(xy)",      sp.diff(sp.sin(x*y), y)),
        (sp.exp(x + a*y),   f"e^(x+{a}y)", sp.diff(sp.exp(x + a*y), y)),
        (sp.cos(x**2 + y),  "cos(x²+y)",   sp.diff(sp.cos(x**2 + y), y)),
        (x**2*sp.log(y),    "x²·ln(y)",    sp.diff(x**2*sp.log(y), y)),
    ]
    f_expr, f_str, ans = random.choice(templates)
    return Problem(
        topic=TOPIC, subtopic="partial_derivatives", difficulty=1,
        question=f"Compute:\n\n  f(x,y) = {f_str}\n\n  ∂f/∂y = ?",
        answer=ans,
        hint="Treat x as a constant; apply chain rule if needed.",
        explanation=f"∂f/∂y = {sp.pretty(ans, use_unicode=True)}",
        problem_type="symbolic",
    )
