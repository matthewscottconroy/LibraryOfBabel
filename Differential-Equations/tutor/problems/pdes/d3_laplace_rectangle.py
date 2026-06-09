"""
Partial Differential Equations — laplaces equation (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "pdes"


def generate() -> Problem:
    case, correct, expl = random.choice([
        (
            "uₓₓ + uᵧᵧ = 0 on {0<x<π, 0<y<1},\n"
            "  u=0 on all sides except u(x,0) = sin(x)",
            "u(x,y) = sin(x)·sinh(1−y)/sinh(1)",
            "Separation: X=sin(nx), Y=sinh(n(1−y))/sinh(n).\n"
            "With u(x,0)=sin(x): only n=1 mode is excited.\n"
            "u = sin(x)·sinh(1−y)/sinh(1)."
        ),
        (
            "For Laplace's equation on a disk of radius R, the Poisson integral formula gives:",
            "u(r,θ) = (R²−r²)/(2π) ∫₀^{2π} f(φ)/(R²−2Rr cos(θ−φ)+r²) dφ",
            "The Poisson kernel for the disk solves the Dirichlet problem:\n"
            "given boundary data f(θ) on |z|=R, the harmonic extension inside is\n"
            "the Poisson integral."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="laplaces_equation", difficulty=3,
        question=f"Solve Laplace's equation:\n\n  {case}",
        answer=correct,
        hint="Use separation of variables X(x)Y(y) or convert to polar coordinates.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[correct,
                 "u(x,y) = sin(x)·e^(-y)",
                 "u(x,y) = sin(x)·cos(y)",
                 "u(x,y) = sin(x)·cosh(y)"],
    )
