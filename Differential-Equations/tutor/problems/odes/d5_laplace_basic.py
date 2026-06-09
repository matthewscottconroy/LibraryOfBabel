"""
Ordinary Differential Equations — laplace (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """Solve IVP using Laplace transform."""
    a = random.choice([1, 2, 3])
    y0 = random.choice([0, 1, 2])
    # y' + ay = 0,  y(0) = y0  → Y = y0/(s+a) → y = y0·e^(-at)
    ans = y0 * sp.exp(-a*x)
    return Problem(
        topic=TOPIC, subtopic="laplace", difficulty=5,
        question=(
            f"Use the Laplace Transform to solve:\n\n"
            f"  y' + {a}y = 0,    y(0) = {y0}\n\n"
            f"(Enter the time-domain solution as a function of x)"
        ),
        answer=ans,
        hint=f"L{{y'}} = sY − y(0). Solve for Y(s), then invert.",
        explanation=(
            f"Taking L{{...}}: sY − {y0} + {a}Y = 0\n"
            f"Y(s)(s + {a}) = {y0}  →  Y(s) = {y0}/(s + {a})\n"
            f"Inverse: L⁻¹{{1/(s+a)}} = e^(-at)\n"
            f"∴ y(x) = {y0}·e^(-{a}x)"
        ),
        problem_type="symbolic",
    )
