"""
Linear Algebra — inner products (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


TOPIC = "linalg"


def generate() -> Problem:
    statement, truth, expl = random.choice([
        (
            "The Cauchy-Schwarz inequality states: |⟨u, v⟩| ≤ ‖u‖·‖v‖.",
            True,
            "True. This holds in any inner product space and is used to prove\n"
            "the triangle inequality and to define the angle between vectors."
        ),
        (
            "In an inner product space, ‖u+v‖² = ‖u‖² + ‖v‖² always (Pythagorean theorem).",
            False,
            "False. The Pythagorean theorem holds only when u ⊥ v (i.e., ⟨u,v⟩ = 0).\n"
            "In general: ‖u+v‖² = ‖u‖² + 2⟨u,v⟩ + ‖v‖²."
        ),
        (
            "The Gram-Schmidt process converts any basis into an orthonormal basis.",
            True,
            "True. Gram-Schmidt takes a linearly independent set {v₁,…,vₙ} and\n"
            "produces an orthonormal set {e₁,…,eₙ} spanning the same space."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="inner_products", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Recall the Cauchy-Schwarz inequality and the parallelogram law.",
        explanation=expl,
        problem_type="true_false",
    )
