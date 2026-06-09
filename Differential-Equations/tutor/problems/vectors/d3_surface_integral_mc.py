"""
Vector Calculus — surface integrals (difficulty 3).
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
            "F = (0, 0, z),  S: the top face of the unit cube (z=1, 0≤x,y≤1), normal pointing up",
            "1",
            "F·n̂ = (0,0,1)·(0,0,1) = 1 on the surface z=1.\n"
            "∫∫_S F·dS = ∫₀¹∫₀¹ 1 dx dy = 1."
        ),
        (
            "F = (x, y, z),  S: sphere of radius R centered at origin",
            "4πR³",
            "By Divergence Theorem: div F = 3. Volume of sphere = (4/3)πR³.\n"
            "∫∫_S F·dS = 3·(4π/3)R³ = 4πR³."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="surface_integrals", difficulty=3,
        question=f"Compute the flux integral:\n\n  {case}",
        answer=correct,
        hint="Flux = ∫∫_S F·n̂ dS.  Use the Divergence Theorem when integrating over a closed surface.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "0", "2π", "R²"],
    )
