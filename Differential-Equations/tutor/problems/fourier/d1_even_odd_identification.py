"""
Fourier Analysis — even odd (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "fourier"


def generate() -> Problem:
    f_desc, parity, expl = random.choice([
        ("x²",        "Even",  "f(−x) = (−x)² = x² = f(x). Even function."),
        ("x³",        "Odd",   "f(−x) = −x³ = −f(x). Odd function."),
        ("sin(x)",    "Odd",   "sin(−x) = −sin(x). Odd function."),
        ("cos(x)",    "Even",  "cos(−x) = cos(x). Even function."),
        ("x + x²",   "Neither", "f(−x) = −x+x² ≠ f(x) and ≠ −f(x). Neither."),
        ("x·sin(x)", "Even",  "f(−x) = (−x)sin(−x) = (−x)(−sin x) = x sin x = f(x). Even."),
    ])
    choices = ["Even", "Odd", "Neither"]
    return Problem(
        topic=TOPIC, subtopic="even_odd", difficulty=1,
        question=f"Is  f(x) = {f_desc}  an even function, odd function, or neither?",
        answer=parity,
        hint="Even: f(−x) = f(x).  Odd: f(−x) = −f(x).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=choices,
    )
