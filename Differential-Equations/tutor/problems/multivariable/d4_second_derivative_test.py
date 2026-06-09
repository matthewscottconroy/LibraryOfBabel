"""
Multivariable Calculus — optimization (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    case, result, expl = random.choice([
        (
            "f(x,y) = x²+y²,  critical point (0,0)",
            "Local minimum",
            "H = [[2,0],[0,2]]. D = 4 > 0, fₓₓ = 2 > 0 → local minimum."
        ),
        (
            "f(x,y) = −x²−y²,  critical point (0,0)",
            "Local maximum",
            "H = [[-2,0],[0,-2]]. D = 4 > 0, fₓₓ = −2 < 0 → local maximum."
        ),
        (
            "f(x,y) = x²−y²,  critical point (0,0)",
            "Saddle point",
            "H = [[2,0],[0,-2]]. D = −4 < 0 → saddle point."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="optimization", difficulty=4,
        question=f"Classify the critical point using the second derivative test:\n\n  {case}",
        answer=result,
        hint="Compute D = fₓₓfᵧᵧ − fₓᵧ². D>0, fₓₓ>0: min; D>0, fₓₓ<0: max; D<0: saddle.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=["Local minimum", "Local maximum", "Saddle point", "Inconclusive"],
    )
