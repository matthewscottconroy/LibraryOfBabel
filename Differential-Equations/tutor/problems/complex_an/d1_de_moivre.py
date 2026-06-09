"""
Complex Analysis — complex numbers (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    n = random.choice([2, 3, 4, 6])
    case, result, expl = random.choice([
        (
            f"(cos(π/4) + i·sin(π/4))^{n}",
            f"cos({n}π/4) + i·sin({n}π/4)",
            f"De Moivre: (cos θ + i sin θ)ⁿ = cos(nθ) + i sin(nθ). Here nθ = {n}π/4."
        ),
        (
            f"(cos(π/3) + i·sin(π/3))^{n}",
            f"cos({n}π/3) + i·sin({n}π/3)",
            f"De Moivre: nθ = {n}π/3."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="complex_numbers", difficulty=1,
        question=f"Apply De Moivre's Theorem:\n\n  {case}",
        answer=result,
        hint="De Moivre: (cos θ + i sin θ)ⁿ = cos(nθ) + i sin(nθ).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[result, f"cos(π/{n}) + i·sin(π/{n})",
                 f"n·cos(π/4) + i·n·sin(π/4)", "1 + 0i"],
    )
