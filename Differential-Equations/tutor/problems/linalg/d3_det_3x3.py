"""
Linear Algebra — determinants (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    while True:
        M = sp.Matrix([[random.randint(-3,3) for _ in range(3)] for _ in range(3)])
        d = M.det()
        if abs(d) <= 20:
            break
    rows = [f"  | {int(M[i,0]):3}  {int(M[i,1]):3}  {int(M[i,2]):3} |" for i in range(3)]
    return Problem(
        topic=TOPIC, subtopic="determinants", difficulty=3,
        question="Compute the determinant:\n\n" + "\n".join(rows),
        answer=sp.Integer(d),
        hint="Expand along any row or column using cofactors.",
        explanation=(
            f"Expanding along row 1:\n"
            f"det = {M[0,0]}·M₁₁ − {M[0,1]}·M₁₂ + {M[0,2]}·M₁₃ = {d}\n"
            f"(where Mᵢⱼ are the 2×2 minor determinants)"
        ),
        problem_type="numeric",
    )
