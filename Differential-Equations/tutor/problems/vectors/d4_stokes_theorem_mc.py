"""
Vector Calculus — stokes theorem (difficulty 4).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "vectors"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "∫∫_S (curl F)·dS where F=(−y,x,0) and S is the unit disk in the z=0 plane",
            "2π",
            "curl F = (0,0,2). Normal to disk: (0,0,1).\n"
            "∫∫ (0,0,2)·(0,0,1) dA = 2·π·1² = 2π.\n"
            "By Stokes: = ∮ F·dr = ∮(−y dx + x dy) = 2·Area = 2π. ✓"
        ),
        (
            "∮_C F·dr using Stokes, where curl F = (1,1,1) on any surface bounded by C",
            "Depends on the area of the surface and its normal",
            "Stokes: ∮ F·dr = ∫∫ (curl F)·dS = ∫∫ (1,1,1)·n̂ dS.\n"
            "The result depends on the surface area and orientation, not just curl F."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="stokes_theorem", difficulty=4,
        question=f"Apply Stokes' Theorem:\n\n  {case}",
        answer=correct,
        hint="Stokes: ∮_C F·dr = ∫∫_S (curl F)·dS (orientation matters).",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "0", "π", "4π"],
    )
