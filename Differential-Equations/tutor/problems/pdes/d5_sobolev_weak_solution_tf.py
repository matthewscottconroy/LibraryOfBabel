"""
Partial Differential Equations — weak solutions (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "pdes"


def generate() -> Problem:
    statement, truth, expl = random.choice([
        (
            "A weak solution of a PDE need not be differentiable in the classical sense.",
            True,
            "True. Weak solutions satisfy the PDE in an integral (variational) sense,\n"
            "using integration by parts to move derivatives onto smooth test functions.\n"
            "This allows solutions with limited regularity (e.g., in Sobolev spaces)."
        ),
        (
            "The Lax-Milgram theorem guarantees existence and uniqueness of weak solutions\n"
            "  when the bilinear form is coercive and bounded.",
            True,
            "True. Lax-Milgram: if a: V×V→ℝ is bilinear, bounded, and coercive,\n"
            "then for any f ∈ V', there is a unique u ∈ V with a(u,v) = f(v) for all v.\n"
            "Generalizes the Riesz representation theorem."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="weak_solutions", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Weak solutions use integration by parts to relax differentiability requirements.",
        explanation=expl,
        problem_type="true_false",
    )
