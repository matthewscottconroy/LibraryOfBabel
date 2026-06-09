"""
Real Analysis — differentiation (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    template = random.choice([
        (sp.sin(x**2), "sin(x²)",   "Chain rule: d/dx sin(u) = cos(u)·u'"),
        (sp.exp(3*x),  "e^(3x)",    "Chain rule: d/dx e^u = e^u·u'"),
        (sp.log(x**2 + 1), "ln(x²+1)", "Chain rule: d/dx ln(u) = u'/u"),
        (sp.cos(2*x),  "cos(2x)",   "Chain rule: d/dx cos(u) = -sin(u)·u'"),
        ((3*x + 1)**4, "(3x+1)⁴",   "Chain rule: d/dx uⁿ = n·uⁿ⁻¹·u'"),
    ])
    f, f_str, hint_str = template
    ans = sp.diff(f, x)
    return Problem(
        topic=TOPIC, subtopic="differentiation", difficulty=2,
        question=f"Find the derivative:\n\n  f(x) = {f_str}",
        answer=ans,
        hint=hint_str,
        explanation=f"f'(x) = {sp.pretty(ans, use_unicode=True)}",
        problem_type="symbolic",
    )
