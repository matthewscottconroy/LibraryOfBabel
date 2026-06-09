"""
Vector Calculus — divergence theorem (difficulty 4).
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
            "F = (x³, y³, z³),  S is the surface of the unit sphere",
            "12π/5",
            "div F = 3x²+3y²+3z². In spherical coords, ∫∫∫ (3r²) r²sinφ dr dφ dθ\n"
            "= 3·(4π)·∫₀¹ r⁴ dr = 12π·(1/5) = 12π/5."
        ),
        (
            "F = (x, y, 0),  S is the closed cylinder x²+y²=1, 0≤z≤1 (including caps)",
            "2π",
            "div F = 1+1+0 = 2. Volume of cylinder = π·1²·1 = π.\n"
            "Flux = ∫∫∫ 2 dV = 2π."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="divergence_theorem", difficulty=4,
        question=f"Use the Divergence Theorem to compute the flux:\n\n  {case}",
        answer=correct,
        hint="Compute div F, then integrate over the enclosed volume.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "0", "4π/3", "2π/5"],
    )
