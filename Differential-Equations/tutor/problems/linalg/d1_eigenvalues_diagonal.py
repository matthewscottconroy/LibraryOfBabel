"""
Linear Algebra — eigenvalues (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    d1, d2, d3 = random.sample(range(-4, 5), 3)
    choices = [
        f"{d1}, {d2}, {d3}",
        f"{d1+1}, {d2+1}, {d3+1}",
        f"0, 0, 0",
        f"{d1*d2*d3}",
    ]
    random.shuffle(choices)
    correct = f"{d1}, {d2}, {d3}"
    return Problem(
        topic=TOPIC, subtopic="eigenvalues", difficulty=1,
        question=(
            f"What are the eigenvalues of the diagonal matrix:\n\n"
            f"  diag({d1}, {d2}, {d3})"
        ),
        answer=correct,
        hint="Eigenvalues of a diagonal matrix are its diagonal entries.",
        explanation=(
            f"For a diagonal matrix D = diag(d₁, d₂, d₃),\n"
            f"det(D − λI) = (d₁−λ)(d₂−λ)(d₃−λ) = 0 gives λ = d₁, d₂, d₃.\n"
            f"Answer: λ = {d1}, {d2}, {d3}"
        ),
        problem_type="multiple_choice",
        choices=choices,
    )
