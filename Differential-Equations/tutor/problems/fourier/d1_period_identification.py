"""
Fourier Analysis — periodic functions (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "fourier"


def generate() -> Problem:
    f_desc, period, expl = random.choice([
        ("sin(3x)",            "2π/3",    "sin(kx) has period 2π/k = 2π/3."),
        ("cos(πx)",            "2",       "cos(πx) has period 2π/π = 2."),
        ("sin(x) + cos(2x)",   "2π",      "Periods are 2π and π; LCM = 2π."),
        ("tan(x)",             "π",       "tan(x) has period π (not 2π)."),
        ("sin(2x)·cos(2x)",    "π/2",     "sin(2x)cos(2x) = ½sin(4x), period = 2π/4 = π/2."),
    ])
    return Problem(
        topic=TOPIC, subtopic="periodic_functions", difficulty=1,
        question=f"What is the fundamental period of:\n\n  f(x) = {f_desc}",
        answer=period,
        hint="sin(kx) and cos(kx) have period 2π/k.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[period, "2π", "π", "π/2", "1"],
    )
