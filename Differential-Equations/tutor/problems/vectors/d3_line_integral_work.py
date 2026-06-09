"""
Vector Calculus — line integrals (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "vectors"


def generate() -> Problem:
    case, ans_val, ans_str, expl = random.choice([
        (
            "F = (1, 0),  C: straight line from (0,0) to (2,3)",
            2,
            "2",
            "Parametrize: r(t)=(2t, 3t), r'=(2,3), F=(1,0).\n"
            "∫₀¹ F·r' dt = ∫₀¹ 1·2 dt = 2."
        ),
        (
            "F = (y, x),  C: straight line from (0,0) to (1,1)",
            1,
            "1",
            "F = (y,x) = ∇(xy) → conservative. ∫_C F·dr = φ(1,1)−φ(0,0) = 1−0 = 1."
        ),
        (
            "F = (x, y),  C: unit circle (full loop)",
            0,
            "0",
            "F = ∇(x²/2+y²/2) is conservative. Closed curve → ∮ F·dr = 0."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="line_integrals", difficulty=3,
        question=f"Compute the line integral (work):\n\n  ∫_C F·dr  where  {case}",
        answer=sp.Integer(ans_val),
        hint="If F is conservative, use the fundamental theorem: ∫_C F·dr = φ(B) − φ(A).",
        explanation=expl,
        problem_type="numeric",
    )
