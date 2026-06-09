"""
Ordinary Differential Equations — linear first order (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """y' + ay = b·e^(cx),  y(0) = y0  (a ≠ c for non-resonance)"""
    a = random.choice([1, 2, 3])
    c = random.choice([-2, -1, 1, 2, 4])
    while c == -a:
        c = random.choice([-2, -1, 1, 2, 4])
    b = random.choice([1, 2, 3])
    y0 = random.choice([0, 1, 2])
    # Particular: yp = b/(a+c) * e^(cx)
    yp_coeff = sp.Rational(b, a + c)
    C = y0 - yp_coeff
    ans = yp_coeff * sp.exp(c*x) + C * sp.exp(-a*x)
    return Problem(
        topic=TOPIC, subtopic="linear_first_order", difficulty=2,
        question=(
            f"Solve the IVP:\n\n"
            f"  y' + {a}y = {b}e^({c}x),    y(0) = {y0}"
        ),
        answer=ans,
        hint=f"Integrating factor: e^({a}x). Or guess yₚ = Ae^({c}x).",
        explanation=(
            f"Homogeneous solution: yₕ = Ce^(-{a}x)\n"
            f"Try particular: yₚ = Ae^({c}x) → Ace^({c}x) + {a}Ae^({c}x) = {b}e^({c}x)\n"
            f"A(c+{a}) = {b} → A = {yp_coeff}\n"
            f"General: y = {yp_coeff}e^({c}x) + Ce^(-{a}x)\n"
            f"Apply y(0) = {y0}: C = {C}\n"
            f"∴ y = {sp.pretty(ans, use_unicode=True)}"
        ),
        problem_type="symbolic",
    )
