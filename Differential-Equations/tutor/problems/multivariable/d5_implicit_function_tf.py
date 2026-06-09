"""
Multivariable Calculus — implicit functions (difficulty 5).
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
            "If F(x,y)=0 defines y implicitly near (a,b) with F(a,b)=0,\n"
            "  then dy/dx = −Fₓ/Fᵧ whenever Fᵧ ≠ 0.",
            True,
            "True. This is a direct consequence of the Implicit Function Theorem:\n"
            "differentiate F(x,y)=0 with respect to x:\n"
            "Fₓ + Fᵧ(dy/dx) = 0  →  dy/dx = −Fₓ/Fᵧ."
        ),
        (
            "The Inverse Function Theorem guarantees a local inverse whenever the Jacobian is non-zero.",
            True,
            "True. If Jf(a) ≠ 0 (det of Jacobian), then f is locally invertible near a,\n"
            "and the inverse is also smooth (C¹)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="implicit_functions", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Recall the conditions required by the Implicit Function Theorem.",
        explanation=expl,
        problem_type="true_false",
    )
