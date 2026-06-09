"""
Multivariable Calculus — change of variables (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "∫∫_D x dA where D is the unit disk x²+y² ≤ 1  (convert to polar)",
            "0",
            "In polar: x = r cos θ. Symmetry: ∫₀^{2π} cos θ dθ = 0.\n"
            "Therefore ∫∫_D x dA = ∫₀¹ r² dr · ∫₀^{2π} cos θ dθ = 0."
        ),
        (
            "The Jacobian for polar coordinates (r,θ) → (x,y) is:",
            "r",
            "Jacobian = |∂(x,y)/∂(r,θ)| = |cos θ  −r sin θ; sin θ  r cos θ| = r.\n"
            "So dA = r dr dθ in polar coordinates."
        ),
        (
            "∫∫_D 1 dA where D is the ellipse x²/4 + y²/9 ≤ 1",
            "6π",
            "Use the substitution x = 2u, y = 3v; Jacobian = 6.\n"
            "Area = ∫∫_{u²+v²≤1} 6 du dv = 6π·1² = 6π.\n"
            "(Or: area of ellipse = πab = π·2·3 = 6π.)"
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="change_of_variables", difficulty=4,
        question=f"Change of variables in multiple integrals:\n\n  {case}",
        answer=correct,
        hint="dA = r dr dθ in polar; in general dA = |J| du dv for Jacobian J.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "π", "2π", "4π/3"],
    )
