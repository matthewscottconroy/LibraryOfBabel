"""
Complex Analysis — contour integration (difficulty 4).
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
            "∫_{-∞}^{∞} 1/(1+x²) dx",
            "π",
            "Close in upper half-plane. Pole at z=i (inside).\n"
            "Res_{z=i} 1/(z²+1) = 1/(2i).\n"
            "Integral = 2πi · 1/(2i) = π."
        ),
        (
            "∫_{-∞}^{∞} 1/(1+x²)² dx",
            "π/2",
            "Pole of order 2 at z=i in the upper half-plane.\n"
            "Res = lim_{z→i} d/dz[(z−i)²/(z²+1)²] = 1/(4i) · (−2i)/(2i)² ... = 1/(4i).\n"
            "Actually: Res = −i/4. Integral = 2πi·(−i/4) = π/2."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="contour_integration", difficulty=4,
        question=f"Evaluate using the Residue Theorem:\n\n  {case}",
        answer=correct,
        hint="Close the contour in the upper half-plane; sum residues inside.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "0", "2π", "π/2", "π"],
    )
