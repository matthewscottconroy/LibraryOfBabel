"""
Complex Analysis — advanced theorems (difficulty 5).
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
            "Every bounded entire function is constant  (Liouville's theorem).",
            True,
            "True. If f is entire and |f| ≤ M everywhere, then f is constant.\n"
            "Proof via Cauchy estimates: |f'(z)| ≤ M/R → 0 as R → ∞."
        ),
        (
            "A non-constant entire function takes every complex value except at most one\n"
            "  (Picard's little theorem).",
            True,
            "True. An entire function omits at most ONE value.\n"
            "e.g., eᶻ omits z=0. A polynomial omits nothing (surjective by FTA)."
        ),
        (
            "If f is analytic in a region Ω and f = 0 on a sequence with an accumulation\n"
            "  point in Ω, then f ≡ 0 in Ω  (Identity theorem).",
            True,
            "True. The Identity Theorem: an analytic function is determined by its values\n"
            "on any set with a limit point in the domain."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="advanced_theorems", difficulty=5,
        question=f"True or False:\n\n  {statement}",
        answer=truth,
        hint="Key theorems: Liouville, Picard, Identity Theorem — all about global behavior of analytic functions.",
        explanation=expl,
        problem_type="true_false",
    )
