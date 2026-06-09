"""
Real Analysis — limits (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    template = random.choice([
        (
            sp.sin(x)/x,         "sin(x)/x",       "x→0",  0,  sp.Integer(1),
            "Both sin(0)=0 and 0, so use L'Hôpital.\nd/dx sin(x) = cos(x), d/dx x = 1.\n"
            "lim cos(x)/1 = cos(0) = 1."
        ),
        (
            (sp.exp(x)-1)/x,     "(eˣ−1)/x",      "x→0",  0,  sp.Integer(1),
            "0/0 form. L'Hôpital: lim eˣ/1 = e⁰ = 1."
        ),
        (
            (x**2-4)/(x-2),      "(x²−4)/(x−2)",  "x→2",  2,  sp.Integer(4),
            "0/0 form. L'Hôpital: lim 2x/1 = 4.  (Or factor: (x-2)(x+2)/(x-2) = x+2 → 4.)"
        ),
        (
            sp.log(x)/(x-1),     "ln(x)/(x−1)",   "x→1",  1,  sp.Integer(1),
            "0/0 at x=1. L'Hôpital: (1/x)/1 → 1 as x→1."
        ),
    ])
    f, f_str, approach, a_val, ans, expl = template
    return Problem(
        topic=TOPIC, subtopic="limits", difficulty=3,
        question=(
            f"Evaluate using L'Hôpital's Rule (0/0 form):\n\n"
            f"  lim    {f_str}\n"
            f"  {approach}"
        ),
        answer=ans,
        hint="Differentiate numerator and denominator separately, then take the limit.",
        explanation=expl,
        problem_type="numeric",
    )
