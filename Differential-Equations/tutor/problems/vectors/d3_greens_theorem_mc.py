"""
Vector Calculus — greens theorem (difficulty 3).
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
            "∮_C (y dx − x dy)  where C is the unit circle (counterclockwise)",
            "−2π",
            "By Green's theorem: ∮ P dx + Q dy = ∫∫ (∂Q/∂x − ∂P/∂y) dA.\n"
            "Q = −x → ∂Q/∂x = −1.  P = y → ∂P/∂y = 1.\n"
            "Integrand = −1−1 = −2.  Area of unit disk = π.\n"
            "Result = −2π."
        ),
        (
            "∮_C (x² y dx + x dy)  where C bounds the unit square [0,1]²",
            "2/3",
            "∂Q/∂x − ∂P/∂y = ∂(x)/∂x − ∂(x²y)/∂y = 1 − x².\n"
            "∫₀¹∫₀¹ (1−x²) dx dy = ∫₀¹ (1−x²) dx · 1 = [x − x³/3]₀¹ = 2/3."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="greens_theorem", difficulty=3,
        question=f"Use Green's Theorem to evaluate:\n\n  {case}",
        answer=correct,
        hint="Green's Theorem: ∮_C P dx + Q dy = ∫∫_D (∂Q/∂x − ∂P/∂y) dA",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "0", "π", "2π"],
    )
