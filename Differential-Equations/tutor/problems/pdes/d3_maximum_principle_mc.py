"""
Partial Differential Equations — maximum principles (difficulty 3).
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
            "For Laplace's equation uₓₓ+uᵧᵧ=0 in domain Ω with boundary ∂Ω:",
            "u attains its max and min on ∂Ω (not in the interior)",
            "Maximum Principle: a nonconstant harmonic function cannot attain\n"
            "its maximum or minimum in the interior of the domain.\n"
            "This means the solution is completely determined by its boundary values."
        ),
        (
            "For the heat equation uₜ = kuₓₓ on [0,L]×[0,T], the maximum of u is attained:",
            "On the parabolic boundary (t=0 or x=0,L)",
            "The parabolic maximum principle: the max is achieved on the 'parabolic boundary'\n"
            "(initial or lateral boundary), not in the interior or at t=T."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="maximum_principles", difficulty=3,
        question=f"Maximum principle:\n\n  {case}",
        answer=correct,
        hint="Maximum principles prevent interior extrema for elliptic and parabolic PDEs.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct,
                 "u attains its max only in the interior",
                 "u is constant throughout Ω",
                 "No maximum principle exists"],
    )
