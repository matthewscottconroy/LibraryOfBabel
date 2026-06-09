"""
Complex Analysis — residues (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    case, res, expl = random.choice([
        (
            "f(z) = 1/(z−2)  at z = 2",
            "1",
            "Simple pole. Residue = lim_{z→2} (z−2)·1/(z−2) = 1."
        ),
        (
            "f(z) = z/(z²−1)  at z = 1",
            "1/2",
            "z²−1=(z−1)(z+1). Simple pole at z=1.\n"
            "Res = lim_{z→1}(z−1)·z/((z−1)(z+1)) = 1/2."
        ),
        (
            "f(z) = eᶻ/(z(z−1))  at z = 0",
            "−1",
            "Simple pole at z=0. Res = lim_{z→0} z·eᶻ/(z(z−1)) = e⁰/(0−1) = −1."
        ),
        (
            "f(z) = 1/z²  at z = 0",
            "0",
            "z=0 is a pole of order 2. Residue = coefficient of 1/z in Laurent series.\n"
            "1/z² has no 1/z term, so Res = 0."
        ),
    ])
    choices = [str(res), "0", "1", "−1", "1/2", "2"]
    choices = list(dict.fromkeys([str(res)] + choices))[:4]
    random.shuffle(choices)
    return Problem(
        topic=TOPIC, subtopic="residues", difficulty=3,
        question=f"Find the residue of:\n\n  {case}",
        answer=str(res),
        hint="For simple pole at z=a: Res = lim_{z→a} (z−a)f(z).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
