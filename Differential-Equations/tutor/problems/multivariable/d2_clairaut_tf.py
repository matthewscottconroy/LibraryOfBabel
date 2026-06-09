"""
Multivariable Calculus — partial derivatives (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    statement, truth, expl = random.choice([
        (
            "For f(x,y) = x²y + sin(xy), we have ∂²f/∂x∂y = ∂²f/∂y∂x.",
            True,
            "By Clairaut's Theorem (Schwarz's theorem): if the mixed partials are\n"
            "continuous, they are equal. For smooth f, ∂²f/∂x∂y = ∂²f/∂y∂x."
        ),
        (
            "Mixed partial derivatives are always equal for any function f(x,y).",
            False,
            "False. Clairaut's theorem requires the mixed partials to be continuous.\n"
            "Pathological functions exist where fₓᵧ ≠ fᵧₓ at isolated points."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="partial_derivatives", difficulty=2,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Clairaut's theorem applies when mixed partials are continuous.",
        explanation=expl,
        problem_type="true_false",
    )
