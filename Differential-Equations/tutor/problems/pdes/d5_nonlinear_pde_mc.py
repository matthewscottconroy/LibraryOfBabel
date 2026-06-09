"""
Partial Differential Equations — nonlinear pdes (difficulty 5).
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
            "The inviscid Burgers equation  uₜ + u·uₓ = 0  can develop:",
            "Shock discontinuities in finite time from smooth initial data",
            "Even for smooth initial data, characteristics can cross for Burgers' equation,\n"
            "leading to gradient blow-up (shock formation) in finite time.\n"
            "Weak (entropy) solutions with shocks must be admitted."
        ),
        (
            "The Hopf-Cole substitution u = −2∂ₓ ln v transforms Burgers' equation uₜ+uuₓ = νuₓₓ into:",
            "The linear heat equation vₜ = ν·vₓₓ",
            "This remarkable transformation linearizes the viscous Burgers equation.\n"
            "u = −2ν∂ₓ ln v transforms uₜ + uuₓ = νuₓₓ → vₜ = νvₓₓ."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="nonlinear_pdes", difficulty=5,
        question=f"Nonlinear PDEs:\n\n  {case}",
        answer=correct,
        hint="Burgers' equation is the canonical nonlinear transport equation.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct,
                 "Only smooth solutions for all time",
                 "The wave equation",
                 "Laplace's equation"],
    )
