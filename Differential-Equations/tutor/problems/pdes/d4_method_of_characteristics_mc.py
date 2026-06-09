"""
Partial Differential Equations — characteristics (difficulty 4).
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
            "uₓ + 2uₜ = 0,   u(x,0) = f(x)  (first-order linear PDE)",
            "u(x,t) = f(x − 2t)",
            "Characteristics: dx/ds=1, dt/ds=2 → t = 2x + const.\n"
            "Along characteristics: u is constant.\n"
            "Solution: u(x,t) = u(x−2t, 0) = f(x−2t)."
        ),
        (
            "uₜ + uₓ = 0,   u(x,0) = e^(−x²)",
            "u(x,t) = e^(−(x−t)²)",
            "Transport equation. Characteristics: x−t = const.\n"
            "u is constant along x−t=c, so u(x,t) = u(x−t, 0) = e^(−(x−t)²)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="characteristics", difficulty=4,
        question=f"Solve using the method of characteristics:\n\n  {case}",
        answer=correct,
        hint="Find the characteristic curves dx/a = dt/b = du/c; u is constant along them.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "u(x,t) = f(x+2t)", "u(x,t) = f(x)e^(-2t)", "u(x,t) = f(x)·t"],
    )
