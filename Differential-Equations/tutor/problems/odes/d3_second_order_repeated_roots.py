"""
Ordinary Differential Equations — second order homogeneous (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    """y'' + 2r·y' + r²·y = 0  → repeated root r"""
    r = random.choice([-3, -2, -1, 1, 2])
    b, c = 2*r, r**2
    # IVP: y(0)=1, y'(0)=0
    # y = (1 - r·x)·e^(r·x)
    ans = (1 - r*x) * sp.exp(r*x)
    return Problem(
        topic=TOPIC, subtopic="second_order_homogeneous", difficulty=3,
        question=(
            f"Solve the IVP:\n\n"
            f"  y'' + {b}y' + {c}y = 0,    y(0) = 1,  y'(0) = 0"
        ),
        answer=sp.expand(ans),
        hint="Double root r: general solution is y = (C₁ + C₂x)·e^(rx).",
        explanation=(
            f"Char. eq: r² + {b}r + {c} = (r + {-r})² = 0 → r = {r} (double)\n"
            f"General: y = (C₁ + C₂x)e^({r}x)\n"
            f"y(0)=1 → C₁=1;  y'(0)=0 → {r}C₁ + C₂ = 0 → C₂ = {-r}\n"
            f"∴ y = (1 + {-r}x)e^({r}x) = (1 − {r}x)e^({r}x)"
        ),
        problem_type="symbolic",
    )
