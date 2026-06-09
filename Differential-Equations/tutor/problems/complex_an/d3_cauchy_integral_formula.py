"""
Complex Analysis — cauchy theorem (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "∮_{|z|=2} z²/(z−1) dz",
            "2πi · 1² = 2πi",
            "z=1 is inside |z|=2. Cauchy's formula: ∮f(z)/(z−a)dz = 2πi·f(a).\n"
            "f(z)=z², a=1: result = 2πi·1 = 2πi."
        ),
        (
            "∮_{|z|=1} eᶻ/(z−0) dz",
            "2πi · e⁰ = 2πi",
            "a=0 inside |z|=1. f(z)=eᶻ. Result = 2πi·e⁰ = 2πi."
        ),
        (
            "∮_{|z|=3} sin(z)/(z−π) dz",
            "2πi · sin(π) = 0",
            "a=π inside |z|=3 (|π|≈3.14 > 3? Actually |π|≈3.14 > 3 so outside!)\n"
            "Wait — π ≈ 3.14 > 3, so z=π is OUTSIDE |z|=3. Result = 0 by Cauchy's theorem."
        ),
    ])
    choices = [correct, "0", "πi", "−2πi"]
    choices = list(dict.fromkeys([correct] + choices))[:4]
    return Problem(
        topic=TOPIC, subtopic="cauchy_theorem", difficulty=3,
        question=f"Evaluate using Cauchy's Integral Formula:\n\n  {case}",
        answer=correct,
        hint="∮ f(z)/(z−a) dz = 2πi·f(a) if a is inside the contour; 0 if outside.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
