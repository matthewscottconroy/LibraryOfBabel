"""
Ordinary Differential Equations — nonhomogeneous (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "y'' + y = sec(x)  (cannot use undetermined coefficients)",
            "Variation of parameters: yₚ = u₁(x)cos x + u₂(x)sin x",
            "Undetermined coefficients doesn't apply to sec(x).\n"
            "VoP: W = cos²x+sin²x = 1.\n"
            "u₁' = −y₂g/W = −sin(x)sec(x) = −tan(x) → u₁ = ln|cos x|\n"
            "u₂' = y₁g/W = cos(x)sec(x) = 1 → u₂ = x\n"
            "yₚ = cos(x)ln|cos x| + x·sin(x)"
        ),
        (
            "y'' − y = e^x·x",
            "Variation of parameters (undetermined coeff also works with modification)",
            "For e^x·x, undetermined coefficients requires modifying the guess due\n"
            "to r=1 being a root of the characteristic equation.\n"
            "VoP always works: yₚ = u₁e^x + u₂e^(-x)."
        ),
    ])
    choices = [correct, "Undetermined coefficients directly", "Separation of variables",
               "Power series method"]
    random.shuffle(choices)
    return Problem(
        topic=TOPIC, subtopic="nonhomogeneous", difficulty=4,
        question=f"What method is BEST suited for:\n\n  {case}",
        answer=correct,
        hint="Variation of parameters works for ANY continuous right-hand side.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
