"""
Ordinary Differential Equations — second order homogeneous (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """Identify the correct form of the general solution from roots."""
    case, correct, expl = random.choice([
        (
            "r² − 5r + 6 = 0  (characteristic equation)",
            "y = C₁e^(2x) + C₂e^(3x)",
            "Roots: r = 2, 3 (distinct real). General solution: C₁e^(2x) + C₂e^(3x)."
        ),
        (
            "r² + 4 = 0  (characteristic equation)",
            "y = C₁cos(2x) + C₂sin(2x)",
            "Roots: r = ±2i (purely imaginary). General: C₁cos(2x) + C₂sin(2x)."
        ),
        (
            "r² − 4r + 4 = 0  (characteristic equation)",
            "y = (C₁ + C₂x)e^(2x)",
            "r² − 4r + 4 = (r−2)² = 0 → repeated root r=2. General: (C₁+C₂x)e^(2x)."
        ),
        (
            "r² + 2r + 5 = 0  (characteristic equation)",
            "y = e^(-x)(C₁cos(2x) + C₂sin(2x))",
            "Roots: r = −1 ± 2i.  α=−1, β=2. General: e^(-x)(C₁cos(2x)+C₂sin(2x))."
        ),
    ])
    choices = [
        correct,
        "y = C₁e^(x) + C₂e^(-x)",
        "y = C₁cos(x) + C₂sin(x)",
        "y = (C₁+C₂x)e^(-x)",
    ]
    # Always include the correct answer; shuffle
    choices = list(dict.fromkeys([correct] + choices))[:4]
    random.shuffle(choices)
    return Problem(
        topic=TOPIC, subtopic="second_order_homogeneous", difficulty=3,
        question=f"What is the general solution corresponding to:\n\n  {case}",
        answer=correct,
        hint="Factor the characteristic polynomial; identify root type (real/complex/repeated).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
