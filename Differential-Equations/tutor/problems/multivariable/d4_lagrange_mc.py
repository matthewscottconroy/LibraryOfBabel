"""
Multivariable Calculus — optimization (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "Minimize f(x,y) = x²+y² subject to x+y = 1",
            "x = y = 1/2,  minimum value = 1/2",
            "Lagrange: ∇f = λ∇g.  2x = λ, 2y = λ → x = y.\n"
            "Constraint: x+y = 1 → x = y = 1/2.  f(1/2,1/2) = 1/2."
        ),
        (
            "Maximize f(x,y) = xy subject to x+y = 10",
            "x = y = 5,  maximum value = 25",
            "Lagrange: y = λ, x = λ → x = y.  Constraint: 2x = 10 → x = 5.\n"
            "f(5,5) = 25."
        ),
        (
            "The method of Lagrange multipliers finds extrema of f subject to g = 0\n"
            "  by solving ∇f = λ∇g",
            "True — this is the statement of the method",
            "Correct. At a constrained extremum, ∇f must be parallel to ∇g,\n"
            "so ∇f = λ∇g for some scalar λ (the Lagrange multiplier)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="optimization", difficulty=4,
        question=f"Lagrange multipliers:\n\n  {case}",
        answer=correct,
        hint="Set ∇f = λ∇g and solve with the constraint equation.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "x = 1, y = 0", "No extremum exists", "x = y = 0"],
    )
