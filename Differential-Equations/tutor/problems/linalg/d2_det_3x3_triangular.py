"""
Linear Algebra — determinants (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    d1, d2, d3 = [random.choice([-3,-2,-1,1,2,3]) for _ in range(3)]
    det = d1 * d2 * d3
    off = random.randint(-2, 2)
    return Problem(
        topic=TOPIC, subtopic="determinants", difficulty=2,
        question=(
            f"Compute the determinant of the upper-triangular matrix:\n\n"
            f"  | {d1}  {off}  {off} |\n"
            f"  |  0  {d2}  {off} |\n"
            f"  |  0   0  {d3} |"
        ),
        answer=sp.Integer(det),
        hint="The determinant of a triangular matrix is the product of its diagonal entries.",
        explanation=f"det = {d1} × {d2} × {d3} = {det}",
        problem_type="numeric",
    )
