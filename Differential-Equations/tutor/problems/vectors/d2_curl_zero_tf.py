"""
Vector Calculus — vector identities (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "vectors"


def generate() -> Problem:
    statement, truth, expl = random.choice([
        (
            "The curl of any gradient field is identically zero.",
            True,
            "True. curl(∇φ) = 0 for any smooth scalar field φ.\n"
            "This is the vector identity curl∘grad = 0."
        ),
        (
            "If curl F = 0 everywhere, then F is conservative on any domain.",
            False,
            "False. curl F = 0 is necessary but not sufficient — the domain must also\n"
            "be simply connected. On a domain with holes (e.g., ℝ²\\{0}),\n"
            "F = (−y, x)/(x²+y²) has zero curl but is NOT conservative."
        ),
        (
            "The divergence of any curl field is identically zero.",
            True,
            "True. div(curl F) = 0 for any smooth vector field F.\n"
            "This is the identity div∘curl = 0."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="vector_identities", difficulty=2,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Recall the vector calculus identities: curl(grad) = 0 and div(curl) = 0.",
        explanation=expl,
        problem_type="true_false",
    )
