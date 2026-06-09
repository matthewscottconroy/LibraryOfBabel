"""
Vector Calculus — divergence theorem (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "vectors"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "F = (x, y, z),  S is the unit sphere",
            "3·(4π/3) = 4π",
            "div F = 1+1+1 = 3. Volume of unit sphere = 4π/3.\n"
            "By Divergence Theorem: flux = ∫∫∫ 3 dV = 3·(4π/3) = 4π."
        ),
        (
            "F = (x², y², z²),  S is the surface of the cube [0,1]³",
            "3",
            "div F = 2x+2y+2z. ∫₀¹∫₀¹∫₀¹ (2x+2y+2z) dV\n"
            "= 2(1/2)+2(1/2)+2(1/2) = 3."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="divergence_theorem", difficulty=2,
        question=f"Use the Divergence Theorem to find the flux of:\n\n  {case}",
        answer=correct,
        hint="Flux = ∯ F·dS = ∫∫∫ div(F) dV",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "0", "2π", "π"],
    )
