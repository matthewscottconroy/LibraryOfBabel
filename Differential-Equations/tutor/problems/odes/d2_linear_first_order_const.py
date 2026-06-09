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
    """y' + ay = b,  y(0) = y0"""
    a = random.choice([1, 2, 3])
    b = random.choice([-4, -2, 2, 4, 6])
    y0 = random.choice([-1, 0, 1, 2, 3])
    eq_pt = sp.Rational(b, a)
    C = y0 - eq_pt
    ans = eq_pt + C * sp.exp(-a * x)
    return Problem(
        topic=TOPIC, subtopic="linear_first_order", difficulty=2,
        question=(
            f"Solve the IVP:\n\n"
            f"  y' + {a}y = {b},    y(0) = {y0}"
        ),
        answer=ans,
        hint=f"Use integrating factor μ(x) = e^({a}x).",
        explanation=(
            f"Integrating factor: μ = e^({a}x)\n"
            f"Multiply: d/dx[e^({a}x)·y] = {b}e^({a}x)\n"
            f"Integrate: e^({a}x)·y = ({b}/{a})e^({a}x) + C\n"
            f"y = {b}/{a} + Ce^(-{a}x)\n"
            f"Apply y(0)={y0}: {y0} = {b}/{a} + C → C = {C}\n"
            f"∴ y = {sp.pretty(ans, use_unicode=True)}"
        ),
        problem_type="symbolic",
    )
