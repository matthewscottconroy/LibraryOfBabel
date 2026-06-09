"""
Complex Analysis — singularities (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    case, sing_type, expl = random.choice([
        (
            "f(z) = sin(z)/z  at z = 0",
            "Removable singularity",
            "lim_{z→0} sin(z)/z = 1 (finite). The singularity is removable;\n"
            "define f(0) = 1 to make f analytic at 0."
        ),
        (
            "f(z) = 1/z³  at z = 0",
            "Pole of order 3",
            "f has Laurent expansion 1/z³. The principal part is 1/z³ (finite).\n"
            "This is a pole of order 3."
        ),
        (
            "f(z) = e^(1/z)  at z = 0",
            "Essential singularity",
            "e^(1/z) = Σ 1/(n!·zⁿ): the Laurent series has infinitely many\n"
            "negative power terms. This is an essential singularity.\n"
            "By Picard's theorem, e^(1/z) takes every nonzero value infinitely often."
        ),
        (
            "f(z) = (z−2)/((z−1)(z−2))  at z = 2",
            "Removable singularity",
            "The factor (z−2) cancels: f = 1/(z−1) for z≠2.\n"
            "lim_{z→2} f(z) = 1. Removable."
        ),
    ])
    choices = ["Removable singularity", "Pole of order 1", "Pole of order 2",
               "Pole of order 3", "Essential singularity"]
    choices = list(dict.fromkeys([sing_type] + choices))[:4]
    random.shuffle(choices)
    return Problem(
        topic=TOPIC, subtopic="singularities", difficulty=3,
        question=f"Classify the singularity of:\n\n  {case}",
        answer=sing_type,
        hint="Removable: lim exists. Pole: Laurent has finite principal part. Essential: infinite principal part.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
