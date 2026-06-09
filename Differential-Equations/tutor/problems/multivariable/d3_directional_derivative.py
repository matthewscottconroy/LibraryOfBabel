"""
Multivariable Calculus — directional derivatives (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    # f(x,y) = ax² + by² at point (p,q) in direction of unit vector
    a, b = random.choice([(1,2),(2,1),(1,1),(2,3)])
    p, q = random.randint(1, 3), random.randint(1, 3)
    # Direction: (cos θ, sin θ) for θ = 0, π/4, π/2
    angle_opt = random.choice([
        ("(1, 0)", sp.Integer(1), sp.Integer(0)),
        ("(1/√2, 1/√2)", sp.Rational(1,2)**sp.Rational(1,2), sp.Rational(1,2)**sp.Rational(1,2)),
        ("(0, 1)", sp.Integer(0), sp.Integer(1)),
    ])
    dir_str, ux, uy = angle_opt
    f = a*x**2 + b*y**2
    fx_val = sp.diff(f, x).subs([(x,p),(y,q)])
    fy_val = sp.diff(f, y).subs([(x,p),(y,q)])
    ans = sp.simplify(fx_val * ux + fy_val * uy)
    return Problem(
        topic=TOPIC, subtopic="directional_derivatives", difficulty=3,
        question=(
            f"Find the directional derivative of  f(x,y) = {f}\n"
            f"at the point ({p},{q})  in the direction  u = {dir_str}."
        ),
        answer=ans,
        hint="Dᵤf = ∇f · u = (∂f/∂x)u₁ + (∂f/∂y)u₂.",
        explanation=(
            f"∂f/∂x = {sp.diff(f,x)}  →  {fx_val}  at ({p},{q})\n"
            f"∂f/∂y = {sp.diff(f,y)}  →  {fy_val}  at ({p},{q})\n"
            f"Dᵤf = {fx_val}·({ux}) + {fy_val}·({uy}) = {ans}"
        ),
        problem_type="numeric",
    )
