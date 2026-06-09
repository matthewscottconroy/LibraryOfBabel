"""
Linear Algebra — rank (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    scenario, rank, expl = random.choice([
        (
            "[[1, 0, 0], [0, 1, 0], [0, 0, 1]]  (3×3 identity)",
            3,
            "The identity matrix has 3 pivots → rank 3."
        ),
        (
            "[[1, 2, 3], [2, 4, 6], [3, 6, 9]]  (rows proportional)",
            1,
            "Row 2 = 2·Row 1, Row 3 = 3·Row 1. Only 1 independent row → rank 1."
        ),
        (
            "[[1, 0, 2], [0, 1, -1], [0, 0, 0]]",
            2,
            "Row echelon form: 2 pivots in rows 1 and 2 → rank 2."
        ),
        (
            "[[2, 4], [1, 2], [3, 6]]  (3×2 matrix)",
            1,
            "All rows are multiples of [1, 2] → rank 1."
        ),
    ])
    choices = ["0", "1", "2", "3"]
    correct = str(rank)
    return Problem(
        topic=TOPIC, subtopic="rank", difficulty=3,
        question=f"What is the rank of the matrix:\n\n  A = {scenario}",
        answer=correct,
        hint="The rank equals the number of non-zero rows in row echelon form (number of pivots).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
