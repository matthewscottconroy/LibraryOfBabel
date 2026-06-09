"""
Ordinary Differential Equations — existence uniqueness (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    statement, truth, expl = random.choice([
        (
            "The IVP  y' = y²,  y(0) = 1  has a unique solution on all of ℝ.",
            False,
            "False. The solution y = 1/(1−x) blows up at x = 1.\n"
            "Picard's theorem guarantees a solution exists only on a NEIGHBORHOOD of 0."
        ),
        (
            "If f(x,y) and ∂f/∂y are continuous on a rectangle containing (x₀,y₀),\n"
            "  then the IVP y' = f(x,y), y(x₀) = y₀ has a unique local solution.",
            True,
            "True. This is the Picard-Lindelöf theorem (existence and uniqueness).\n"
            "The Lipschitz condition (bounded ∂f/∂y) guarantees uniqueness."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="existence_uniqueness", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Picard's theorem gives local (not global) existence and uniqueness.",
        explanation=expl,
        problem_type="true_false",
    )
