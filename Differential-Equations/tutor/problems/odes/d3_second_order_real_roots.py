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
    """y'' + (r1+r2)y' + r1*r2·y = 0  with distinct real roots"""
    r1 = random.choice([-4, -3, -2, -1])
    r2 = random.choice([-4, -3, -1, 1, 2])
    while r1 == r2:
        r2 = random.choice([-4, -3, -1, 1, 2])
    b = -(r1 + r2)
    c = r1 * r2
    # IVP: y(0)=1, y'(0)=0
    y0, yp0 = 1, 0
    # C1 + C2 = y0, r1*C1 + r2*C2 = yp0
    M = sp.Matrix([[1, 1], [r1, r2]])
    C_vec = M.solve(sp.Matrix([y0, yp0]))
    C1, C2 = C_vec
    ans = C1*sp.exp(r1*x) + C2*sp.exp(r2*x)
    return Problem(
        topic=TOPIC, subtopic="second_order_homogeneous", difficulty=3,
        question=(
            f"Solve the IVP:\n\n"
            f"  y'' + {b}y' + {c}y = 0,    y(0) = {y0},  y'(0) = {yp0}"
        ),
        answer=sp.expand(ans),
        hint="Find roots of the characteristic equation r² + br + c = 0.",
        explanation=(
            f"Characteristic equation: r² + {b}r + {c} = 0\n"
            f"Roots: r₁ = {r1},  r₂ = {r2}\n"
            f"General solution: y = C₁e^({r1}x) + C₂e^({r2}x)\n"
            f"Apply ICs → C₁ = {C1}, C₂ = {C2}\n"
            f"∴ y = {sp.pretty(sp.expand(ans), use_unicode=True)}"
        ),
        problem_type="symbolic",
    )
