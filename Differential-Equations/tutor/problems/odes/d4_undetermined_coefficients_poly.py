"""
Ordinary Differential Equations — undetermined coefficients (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """y'' + by' + cy = dx + e  →  particular solution is linear"""
    c_coeff = random.choice([1, 2, 4])   # must be nonzero
    b_coeff = random.choice([1, 2, 3])
    d = random.choice([-3, -2, -1, 1, 2, 3])
    e = random.choice([-2, -1, 0, 1, 2])
    # Particular: yp = Ax + B
    # y'p = A, y''p = 0 → c_coeff*(Ax+B) + b_coeff*A = dx + e
    # A = d/c_coeff,  B = (e - b_coeff*A)/c_coeff
    A = sp.Rational(d, c_coeff)
    B = sp.Rational(e - b_coeff*A, c_coeff)
    yp = A*x + B
    return Problem(
        topic=TOPIC, subtopic="undetermined_coefficients", difficulty=4,
        question=(
            f"Find a particular solution of:\n\n"
            f"  y'' + {b_coeff}y' + {c_coeff}y = {d}x + {e}"
        ),
        answer=yp,
        hint="Try yₚ = Ax + B. Substitute and match coefficients of x⁰ and x¹.",
        explanation=(
            f"Guess: yₚ = Ax + B,  y'ₚ = A,  y''ₚ = 0\n"
            f"Substitute: {b_coeff}A + {c_coeff}(Ax+B) = {d}x + {e}\n"
            f"Match x¹: {c_coeff}A = {d} → A = {A}\n"
            f"Match x⁰: {b_coeff}·{A} + {c_coeff}B = {e} → B = {B}\n"
            f"∴ yₚ = {sp.pretty(yp, use_unicode=True)}"
        ),
        problem_type="symbolic",
    )
