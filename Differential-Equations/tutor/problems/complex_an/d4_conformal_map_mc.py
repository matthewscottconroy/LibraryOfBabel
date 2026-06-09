"""
Complex Analysis — conformal maps (difficulty 4).
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
            "The Möbius transformation w = (z−i)/(z+i) maps:",
            "Upper half-plane → unit disk",
            "w = (z−i)/(z+i): check boundary z=x (real): |w|=|x−i|/|x+i|=1.\n"
            "At z=0: w = −1 (boundary). At z=i: w=0 (center).\n"
            "So the real axis maps to |w|=1, upper half-plane maps inside."
        ),
        (
            "The map w = z² from the first quadrant {Re z > 0, Im z > 0}:",
            "Maps onto the upper half-plane Im w > 0",
            "z = r·e^(iθ), 0<θ<π/2. w = r²·e^(2iθ), 0<2θ<π.\n"
            "Image: arg w ∈ (0, π) → upper half-plane."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="conformal_maps", difficulty=4,
        question=f"Conformal mapping:\n\n  {case}",
        answer=correct,
        hint="Track where boundaries and key points map to determine the image.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "Lower half-plane", "Right half-plane", "The entire complex plane"],
    )
