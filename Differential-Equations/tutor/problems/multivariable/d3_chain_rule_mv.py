"""
Multivariable Calculus — chain rule (difficulty 3).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, y, z = sp.symbols("x y z")
TOPIC = "multivariable"


def generate() -> Problem:
    # f(x,y) with x(t), y(t); find df/dt
    a, b = random.choice([(1,2),(2,1),(3,1),(1,3)])
    t = sp.Symbol("t")
    # x(t) = cos(t), y(t) = sin(t)  →  x'= -sin t, y'= cos t
    f = a*x**2 + b*y**2
    xoft = sp.cos(t)
    yoft = sp.sin(t)
    dfdt = (sp.diff(f,x).subs([(x,xoft),(y,yoft)])*sp.diff(xoft,t)
            + sp.diff(f,y).subs([(x,xoft),(y,yoft)])*sp.diff(yoft,t))
    dfdt = sp.simplify(dfdt)
    return Problem(
        topic=TOPIC, subtopic="chain_rule", difficulty=3,
        question=(
            f"Let  f(x,y) = {a}x² + {b}y²  and  x(t) = cos t,  y(t) = sin t.\n\n"
            f"Use the chain rule to find  df/dt."
        ),
        answer=dfdt,
        hint="df/dt = (∂f/∂x)(dx/dt) + (∂f/∂y)(dy/dt).",
        explanation=(
            f"∂f/∂x = {a*2}x,  ∂f/∂y = {b*2}y\n"
            f"dx/dt = −sin t,  dy/dt = cos t\n"
            f"df/dt = {a*2}cos(t)(−sin t) + {b*2}sin(t)(cos t)\n"
            f"      = ({b*2}−{a*2}) sin t cos t = {sp.pretty(dfdt, use_unicode=True)}"
        ),
        problem_type="symbolic",
    )
