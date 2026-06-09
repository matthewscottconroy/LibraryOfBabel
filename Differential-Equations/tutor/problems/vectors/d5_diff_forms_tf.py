"""
Vector Calculus — differential forms (difficulty 5).
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
            "The exterior derivative d satisfies d∘d = 0  (i.e. d²=0).",
            True,
            "True. This is the fundamental property of the exterior derivative:\n"
            "d(dω) = 0 for any differential form ω.\n"
            "It unifies curl(grad) = 0 and div(curl) = 0 in a single statement."
        ),
        (
            "Stokes' Theorem in its generalized form states: ∫_∂M ω = ∫_M dω.",
            True,
            "True. This is the generalized Stokes' theorem, which includes\n"
            "as special cases: the fundamental theorem of calculus, Green's theorem,\n"
            "the classical Stokes' theorem, and the Divergence theorem."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="differential_forms", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Think about how d² = 0 generalizes the vector identities curl(grad)=0 and div(curl)=0.",
        explanation=expl,
        problem_type="true_false",
    )
