"""
Vector Calculus — potentials (difficulty 4).
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
            "F = (2xy, x²+3y², 0).  Find a potential φ such that F = ∇φ.",
            "φ = x²y + y³",
            "Check: ∂φ/∂x = 2xy ✓,  ∂φ/∂y = x²+3y² ✓.\n"
            "Integrate ∂φ/∂x = 2xy → φ = x²y + g(y).\n"
            "Then ∂φ/∂y = x² + g'(y) = x²+3y² → g'(y) = 3y² → g(y) = y³.\n"
            "So φ = x²y + y³."
        ),
        (
            "The existence of a vector potential A such that F = curl A requires:",
            "div F = 0",
            "By the vector identity div(curl A) = 0, a necessary condition for F = curl A\n"
            "is div F = 0. On simply connected domains, this is also sufficient."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="potentials", difficulty=4,
        question=f"Vector potentials and scalar potentials:\n\n  {case}",
        answer=correct,
        hint="For a scalar potential: integrate F component by component, checking consistency.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "φ = x²y + y²", "φ = xy² + y³", "div F = 1"],
    )
