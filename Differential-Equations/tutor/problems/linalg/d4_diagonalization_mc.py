"""
Linear Algebra — eigentheory (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    case, diagonalizable, expl = random.choice([
        (
            "A 2×2 matrix with two distinct real eigenvalues",
            True,
            "Distinct eigenvalues → independent eigenvectors → diagonalizable."
        ),
        (
            "A 2×2 matrix with characteristic polynomial (λ−3)²",
            False,
            "A repeated eigenvalue may or may not be diagonalizable.\n"
            "If the eigenspace has dimension 1 (not 2), it is NOT diagonalizable.\n"
            "(E.g. A = [[3,1],[0,3]] has only one eigenvector direction.)"
        ),
        (
            "A real symmetric matrix",
            True,
            "The Spectral Theorem: every real symmetric matrix is orthogonally diagonalizable."
        ),
        (
            "A nilpotent matrix with Aⁿ = 0 (n > 1) but A ≠ 0",
            False,
            "A nilpotent matrix has only eigenvalue 0. If it's not the zero matrix,\n"
            "it cannot be diagonalizable (D = 0 would imply A = PDP⁻¹ = 0)."
        ),
    ])
    choices = ["Always diagonalizable", "Not necessarily diagonalizable"]
    correct = "Always diagonalizable" if diagonalizable else "Not necessarily diagonalizable"
    return Problem(
        topic=TOPIC, subtopic="eigentheory", difficulty=4,
        question=f"Is the following always diagonalizable?\n\n  {case}",
        answer=correct,
        hint="A matrix is diagonalizable iff it has n linearly independent eigenvectors.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
