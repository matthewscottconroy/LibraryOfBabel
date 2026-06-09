"""
Linear Algebra — matrix algebra (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    a, b, c, d = [random.randint(-3, 3) for _ in range(4)]
    v1, v2 = random.randint(-3, 3), random.randint(-3, 3)
    r1 = a*v1 + b*v2
    r2 = c*v1 + d*v2
    choices = [
        f"[{r1}, {r2}]",
        f"[{a+v1}, {b+v2}]",
        f"[{a*v2+b*v1}, {c*v2+d*v1}]",
        f"[{r2}, {r1}]",
    ]
    random.shuffle(choices)
    correct = f"[{r1}, {r2}]"
    return Problem(
        topic=TOPIC, subtopic="matrix_algebra", difficulty=1,
        question=(
            f"Compute A·v where:\n\n"
            f"  A = [[{a}, {b}], [{c}, {d}]]   v = [{v1}, {v2}]"
        ),
        answer=correct,
        hint="Multiply each row of A by the vector v (dot product).",
        explanation=(
            f"Row 1: {a}·{v1} + {b}·{v2} = {r1}\n"
            f"Row 2: {c}·{v1} + {d}·{v2} = {r2}\n"
            f"Result: [{r1}, {r2}]"
        ),
        problem_type="multiple_choice",
        choices=choices,
    )
