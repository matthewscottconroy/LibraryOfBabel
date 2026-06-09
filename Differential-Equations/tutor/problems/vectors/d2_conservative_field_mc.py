"""
Vector Calculus — conservative fields (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "vectors"


def generate() -> Problem:
    case, conservative, expl = random.choice([
        (
            "F = (2xy, x²+3y²)",
            True,
            "Check: ∂P/∂y = 2x = ∂Q/∂x. Yes! curl = 0 → F is conservative.\n"
            "Potential: φ = x²y + y³."
        ),
        (
            "F = (y, −x)",
            False,
            "∂P/∂y = 1,  ∂Q/∂x = −1.  Since 1 ≠ −1, curl F ≠ 0 → NOT conservative.\n"
            "(This field rotates — think of water circling a drain.)"
        ),
        (
            "F = (eˣ cos y, −eˣ sin y)",
            True,
            "∂P/∂y = −eˣ sin y = ∂Q/∂x. Curl = 0 → conservative.\n"
            "This is the real part of the analytic function eᶻ."
        ),
        (
            "F = (x², y²)",
            True,
            "∂P/∂y = 0 = ∂Q/∂x. Curl = 0 → conservative.\n"
            "Potential: φ = x³/3 + y³/3."
        ),
    ])
    choices = ["Conservative", "Not conservative"]
    correct = "Conservative" if conservative else "Not conservative"
    return Problem(
        topic=TOPIC, subtopic="conservative_fields", difficulty=2,
        question=f"Is the following vector field conservative?\n\n  {case}",
        answer=correct,
        hint="Check if ∂P/∂y = ∂Q/∂x (in 2D). If yes, F = ∇φ for some potential φ.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
