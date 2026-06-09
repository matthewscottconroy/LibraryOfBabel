"""
Multivariable Calculus — double integrals (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "∫₀¹ ∫₀¹ (x + y) dx dy",
            "1",
            "∫₀¹ [x²/2 + xy]₀¹ dy = ∫₀¹ (1/2 + y) dy = [y/2 + y²/2]₀¹ = 1/2+1/2 = 1."
        ),
        (
            "∫₀¹ ∫_y^1 e^(x²) dx dy  (switch order of integration)",
            "½(e − 1)",
            "Switch: D = {0≤y≤x≤1}.\n"
            "∫₀¹ ∫₀^x e^(x²) dy dx = ∫₀¹ x e^(x²) dx = [e^(x²)/2]₀¹ = (e−1)/2."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="double_integrals", difficulty=4,
        question=f"Compute the iterated integral:\n\n  {case}",
        answer=correct,
        hint="For switching order: sketch the region D and re-express the limits.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "0", "e/2", "2"],
    )
