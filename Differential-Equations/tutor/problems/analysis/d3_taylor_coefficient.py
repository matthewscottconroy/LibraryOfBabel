"""
Real Analysis — series (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    template = random.choice([
        (sp.exp(x),  "eˣ",     2, sp.Rational(1, 2), "eˣ = Σ xⁿ/n!, so coefficient of x² is 1/2! = 1/2"),
        (sp.sin(x),  "sin(x)", 3, sp.Rational(1, 6),  "sin(x) = x − x³/3! + …, coeff of x³ is −1/6 (but |coeff| is 1/6)"),
        (sp.cos(x),  "cos(x)", 2, sp.Rational(-1, 2), "cos(x) = 1 − x²/2! + …, coeff of x² is −1/2"),
        (sp.log(1+x),"ln(1+x)",2, sp.Rational(-1, 2), "ln(1+x) = x − x²/2 + x³/3 − …, coeff of x² is −1/2"),
    ])
    f, f_str, order, coeff, expl = template
    actual = sp.series(f, x, 0, order+1).coeff(x, order)
    return Problem(
        topic=TOPIC, subtopic="series", difficulty=3,
        question=(
            f"Find the coefficient of x^{order} in the Maclaurin series of:\n\n"
            f"  f(x) = {f_str}"
        ),
        answer=actual,
        hint=f"The coefficient of xⁿ in the Maclaurin series is f^(n)(0)/n!",
        explanation=expl,
        problem_type="numeric",
    )
