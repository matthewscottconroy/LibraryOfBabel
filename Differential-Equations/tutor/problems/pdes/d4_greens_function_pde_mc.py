"""
Partial Differential Equations — greens functions (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "pdes"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "The fundamental solution (free-space Green's function) for the 3D Laplacian is:",
            "G(x) = −1/(4π|x|)",
            "Δ(−1/4π|x|) = δ(x) in ℝ³.\n"
            "This models the potential from a point charge in electrostatics."
        ),
        (
            "The heat kernel (fundamental solution for uₜ = uₓₓ in ℝ) is:",
            "K(x,t) = (1/√(4πt))·exp(−x²/4t)",
            "K satisfies Kₜ = Kₓₓ with K(x,0) = δ(x).\n"
            "The general solution is u(x,t) = ∫K(x−y,t)f(y)dy."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="greens_functions", difficulty=4,
        question=f"Green's function / fundamental solution:\n\n  {case}",
        answer=correct,
        hint="Fundamental solutions satisfy LG = δ (the PDE with a point source).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "G(x) = e^(-|x|)", "G(x) = |x|", "G(x) = 1/(2π|x|²)"],
    )
