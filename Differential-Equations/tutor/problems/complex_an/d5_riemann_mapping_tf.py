"""
Complex Analysis — conformal maps (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    statement, truth, expl = random.choice([
        (
            "Every simply connected proper open subset of ℂ is conformally equivalent to the unit disk.",
            True,
            "True. This is the Riemann Mapping Theorem.\n"
            "Moreover, the mapping is essentially unique once you fix the image of one point\n"
            "and the argument of the derivative there."
        ),
        (
            "The entire complex plane ℂ is conformally equivalent to the unit disk.",
            False,
            "False. ℂ is simply connected but not conformally equivalent to the unit disk.\n"
            "By Liouville's theorem, any bounded entire function is constant, so there is\n"
            "no bounded conformal bijection ℂ → D."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="conformal_maps", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="The Riemann Mapping Theorem applies to simply connected PROPER subsets of ℂ.",
        explanation=expl,
        problem_type="true_false",
    )
