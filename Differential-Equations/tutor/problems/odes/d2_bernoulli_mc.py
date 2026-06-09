"""
Ordinary Differential Equations — substitution (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    case, method, expl = random.choice([
        (
            "y' + y = y³",
            "Bernoulli substitution v = y^(1-3) = y^(-2)",
            "This is a Bernoulli equation with n=3.\n"
            "Let v = y^(1-n) = y^(-2).\n"
            "Then v' = −2y^(-3)y' → divide the ODE by y³:\n"
            "y^(-3)y' + y^(-2) = 1 → −v'/2 + v = 1 (linear in v)."
        ),
        (
            "y' − y = xy²",
            "Bernoulli substitution v = y^(1-2) = y^(-1)",
            "Bernoulli with n=2. Let v = y^(-1).\n"
            "v' = −y^(-2)y' → divide by y²:\n"
            "y^(-2)y' − y^(-1) = x → −v' − v = x → v' + v = −x (linear)."
        ),
    ])
    choices = [method, "Separation of variables", "Exact equation", "Integrating factor (as written)"]
    random.shuffle(choices)
    return Problem(
        topic=TOPIC, subtopic="substitution", difficulty=2,
        question=f"What substitution reduces to a linear ODE?\n\n  {case}",
        answer=method,
        hint="Bernoulli equation: y' + P(x)y = Q(x)yⁿ → substitute v = y^(1-n).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
