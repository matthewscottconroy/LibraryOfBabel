"""
Linear Algebra — determinants (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    a, b, c, d = [random.randint(-4, 4) for _ in range(4)]
    det = a*d - b*c
    return Problem(
        topic=TOPIC, subtopic="determinants", difficulty=1,
        question=(
            f"Compute the determinant of the matrix:\n\n"
            f"  | {a:3}  {b:3} |\n"
            f"  | {c:3}  {d:3} |"
        ),
        answer=sp.Integer(det),
        hint="det([[a,b],[c,d]]) = ad − bc",
        explanation=f"det = ({a})({d}) − ({b})({c}) = {a*d} − {b*c} = {det}",
        problem_type="numeric",
    )
