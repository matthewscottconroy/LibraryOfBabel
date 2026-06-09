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
    case, correct, expl = random.choice([
        (
            "On ℝⁿ (which is contractible), every closed form is:",
            "Exact (Poincaré lemma)",
            "The Poincaré lemma: on any contractible open set, every closed form\n"
            "(dω = 0) is exact (ω = dη for some η). ℝⁿ is contractible, so H^k(ℝⁿ) = 0 for k ≥ 1."
        ),
        (
            "The 1-form ω = (−y dx + x dy)/(x²+y²) on ℝ²\\{0}  satisfies dω = 0 but:",
            "Is NOT exact (its integral around the origin is 2π ≠ 0)",
            "ω = dθ in polar coordinates, which is locally exact but not globally exact\n"
            "because ℝ²\\{0} is not simply connected.\n"
            "∮ ω = 2π around the origin, so ω ≠ df for any smooth f on ℝ²\\{0}."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="differential_forms", difficulty=5,
        question=f"De Rham cohomology:\n\n  {case}",
        answer=correct,
        hint="Exactness fails on domains with holes — even when the form is closed.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "Closed but not exact everywhere",
                 "Neither closed nor exact", "Always integrable"],
    )
