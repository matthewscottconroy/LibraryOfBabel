"""
Ordinary Differential Equations — phase plane (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "The system x' = y,  y' = −x (undamped harmonic oscillator) has:",
            "Center equilibrium — closed orbits (ellipses)",
            "Eigenvalues: ±i (purely imaginary). No real part → center.\n"
            "Trajectories are closed curves (ellipses) around the origin."
        ),
        (
            "The system x' = x,  y' = −2y has equilibrium at origin. Its type is:",
            "Saddle point (one stable, one unstable manifold)",
            "Eigenvalues: 1 (unstable) and −2 (stable). Mixed signs → saddle.\n"
            "Trajectories approach along the y-axis and leave along the x-axis."
        ),
        (
            "x' = −x + y,  y' = −y  (trace T = −2 < 0, det D = 1 > 0)",
            "Stable node or spiral (asymptotically stable)",
            "T < 0, D > 0: eigenvalues have negative real parts → stable.\n"
            "Discriminant T²−4D = 4−4 = 0 → repeated eigenvalue −1: stable node."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="phase_plane", difficulty=5,
        question=f"Phase plane analysis:\n\n  {case}",
        answer=correct,
        hint="Use eigenvalue signs: both negative → stable; mixed → saddle; pure imaginary → center.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct, "Unstable spiral", "Unstable node", "Saddle point"],
    )
