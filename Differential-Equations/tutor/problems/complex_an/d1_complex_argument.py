"""
Complex Analysis — complex numbers (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    case, arg_str, expl = random.choice([
        (
            "1 + i",       "π/4",
            "arg(1+i) = arctan(1/1) = π/4.  (First quadrant, equal parts.)"
        ),
        (
            "−1 + 0·i",    "π",
            "arg(−1) = π.  (Negative real axis.)"
        ),
        (
            "0 + i",       "π/2",
            "arg(i) = π/2.  (Positive imaginary axis.)"
        ),
        (
            "1 − i",       "−π/4",
            "arg(1−i) = arctan(−1/1) = −π/4.  (Fourth quadrant.)"
        ),
        (
            "−1 − i",      "−3π/4",
            "In third quadrant: arg = −π + arctan(1/1) = −π + π/4 = −3π/4."
        ),
        (
            "√3 + i",      "π/6",
            "arg(√3 + i) = arctan(1/√3) = π/6."
        ),
    ])
    choices = ["π/6", "π/4", "π/3", "π/2", "π", "−π/4", "−π/2", "−3π/4", "−π/3"]
    choices = list(dict.fromkeys([arg_str] + choices))[:5]
    random.shuffle(choices)
    return Problem(
        topic=TOPIC, subtopic="complex_numbers", difficulty=1,
        question=f"Find the principal argument (in radians) of:\n\n  z = {case}",
        answer=arg_str,
        hint="arg(a+bi) = arctan(b/a), adjusted for the correct quadrant.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
