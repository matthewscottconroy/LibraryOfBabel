"""
Linear Algebra — null space (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    # Simple matrix with known null space
    M, null_desc, expl = random.choice([
        (
            "[[1, 2], [2, 4]]",
            "Span{(-2, 1)}",
            "Row reduce: R2 → R2 − 2R1 gives [[1,2],[0,0]].\n"
            "Free variable x₂ = t → x₁ = −2t.  Null space = Span{(−2, 1)}."
        ),
        (
            "[[1, 0, 1], [0, 1, 1]]",
            "Span{(-1, -1, 1)}",
            "Row echelon: pivots in columns 1,2; x₃ free.\n"
            "x₁ = −x₃, x₂ = −x₃.  Null space = Span{(−1, −1, 1)}."
        ),
        (
            "[[1, 2, 3], [0, 0, 0], [0, 0, 0]]",
            "Span{(-2,1,0), (-3,0,1)}",
            "One pivot (col 1); x₂, x₃ free.\n"
            "x₁ = −2x₂ − 3x₃.  Null space = Span{(−2,1,0), (−3,0,1)}."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="null_space", difficulty=4,
        question=f"Find the null space of:\n\n  A = {M}",
        answer=null_desc,
        hint="Row-reduce [A|0] and express free variables as parameters.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[null_desc, "Span{(1,0)}", "Span{(0,0,0)}", "Span{(1,1,1)}"],
    )
