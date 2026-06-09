"""
Partial Differential Equations — wave equation (difficulty 3).
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
            "uₜₜ = c²uₓₓ,   u(x,0) = f(x),   uₜ(x,0) = 0",
            "u(x,t) = ½[f(x+ct) + f(x−ct)]",
            "With zero initial velocity, d'Alembert's formula gives:\n"
            "u(x,t) = ½[f(x+ct) + f(x−ct)].\n"
            "Two waves travel in opposite directions."
        ),
        (
            "uₜₜ = 4uₓₓ,   u(x,0) = sin(x),   uₜ(x,0) = 0",
            "u(x,t) = ½[sin(x+2t) + sin(x−2t)] = sin(x)cos(2t)",
            "c = 2. d'Alembert: ½[sin(x+2t) + sin(x-2t)].\n"
            "Using sum-to-product: sin(x+2t)+sin(x-2t) = 2sin(x)cos(2t).\n"
            "So u = sin(x)cos(2t)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="wave_equation", difficulty=3,
        question=f"d'Alembert's solution:\n\n  {case}",
        answer=correct,
        hint="d'Alembert: u(x,t) = ½[f(x+ct) + f(x−ct)] + (1/2c)∫_{x-ct}^{x+ct} g(s)ds.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct,
                 "u(x,t) = f(x)·cos(ct)",
                 "u(x,t) = f(x+ct)",
                 "u(x,t) = f(x)·e^(−ct)"],
    )
