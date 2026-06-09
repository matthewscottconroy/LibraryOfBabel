"""
Ordinary Differential Equations — cauchy euler (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "x²y'' + xy' − y = 0",
            "y = C₁x + C₂x⁻¹",
            "Substitute y = xᵐ: m(m−1) + m − 1 = m² − 1 = 0 → m = ±1.\n"
            "General solution: y = C₁x + C₂x⁻¹."
        ),
        (
            "x²y'' − 2y = 0",
            "y = C₁x² + C₂x⁻¹",
            "Substitute y = xᵐ: m(m−1) − 2 = m² − m − 2 = (m−2)(m+1) = 0.\n"
            "Roots m = 2, −1. General: y = C₁x² + C₂x⁻¹."
        ),
        (
            "x²y'' + 4xy' + 2y = 0",
            "y = C₁x⁻¹ + C₂x⁻²",
            "Substitute y = xᵐ: m(m−1) + 4m + 2 = m² + 3m + 2 = (m+1)(m+2) = 0.\n"
            "Roots m = −1, −2. General: y = C₁x⁻¹ + C₂x⁻²."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="cauchy_euler", difficulty=4,
        question=f"Solve the Cauchy-Euler equation:\n\n  {case}",
        answer=correct,
        hint="Try y = xᵐ; this gives an algebraic equation for m (the indicial equation).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "y = C₁eˣ + C₂e⁻ˣ", "y = (C₁+C₂x)eˣ",
                 "y = C₁cos(x)+C₂sin(x)"],
    )
