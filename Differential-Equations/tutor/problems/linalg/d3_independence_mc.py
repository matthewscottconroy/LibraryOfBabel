"""
Linear Algebra — linear independence (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    case, independent, expl = random.choice([
        (
            "v₁ = (1,0,0),  v₂ = (0,1,0),  v₃ = (0,0,1)",
            True,
            "These are the standard basis vectors — clearly linearly independent.\n"
            "The only solution to c₁v₁ + c₂v₂ + c₃v₃ = 0 is c₁=c₂=c₃=0."
        ),
        (
            "v₁ = (1,2,3),  v₂ = (2,4,6),  v₃ = (1,1,1)",
            False,
            "v₂ = 2·v₁, so they are linearly dependent.\n"
            "2·v₁ − v₂ + 0·v₃ = 0 is a non-trivial relation."
        ),
        (
            "v₁ = (1,0),  v₂ = (0,1)",
            True,
            "Standard basis in ℝ². The determinant [[1,0],[0,1]] = 1 ≠ 0."
        ),
        (
            "v₁ = (1,1),  v₂ = (2,2)",
            False,
            "v₂ = 2·v₁, so det = 0 and the vectors are linearly dependent."
        ),
    ])
    choices = ["Linearly independent", "Linearly dependent"]
    correct = "Linearly independent" if independent else "Linearly dependent"
    return Problem(
        topic=TOPIC, subtopic="linear_independence", difficulty=3,
        question=f"Are the following vectors linearly independent?\n\n  {case}",
        answer=correct,
        hint="Check if det ≠ 0, or if any vector is a linear combination of the others.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
