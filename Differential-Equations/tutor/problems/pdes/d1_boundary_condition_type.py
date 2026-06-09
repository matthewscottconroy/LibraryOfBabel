"""
Partial Differential Equations — boundary conditions (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "pdes"


def generate() -> Problem:
    case, bc_type, expl = random.choice([
        (
            "u(0, t) = 0  and  u(L, t) = 0  (fixed endpoints)",
            "Dirichlet",
            "Dirichlet BCs specify the VALUE of u on the boundary.\n"
            "Also called 'essential' boundary conditions."
        ),
        (
            "uₓ(0, t) = 0  and  uₓ(L, t) = 0  (insulated endpoints)",
            "Neumann",
            "Neumann BCs specify the NORMAL DERIVATIVE on the boundary.\n"
            "uₓ = 0 means zero heat flux (insulated) or zero slope."
        ),
        (
            "uₓ(0,t) + h·u(0,t) = 0  (Newton's law of cooling)",
            "Robin",
            "Robin (or mixed) BCs are a linear combination of u and its normal derivative.\n"
            "Models convective heat transfer at the boundary."
        ),
    ])
    choices = ["Dirichlet", "Neumann", "Robin", "Cauchy"]
    return Problem(
        topic=TOPIC, subtopic="boundary_conditions", difficulty=1,
        question=f"What type of boundary condition is:\n\n  {case}",
        answer=bc_type,
        hint="Dirichlet: value given.  Neumann: derivative given.  Robin: linear combination.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
