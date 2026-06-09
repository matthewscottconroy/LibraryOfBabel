"""
Real Analysis — limits (difficulty 1).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, n = sp.symbols("x n")
TOPIC = "analysis"


def generate() -> Problem:
    """lim_{x→a} (x^2 - a^2)/(x - a) = 2a  (0/0 form)"""
    a = random.choice([-3, -2, 2, 3])
    num = x**2 - a**2
    den = x - a
    ans = sp.Integer(2 * a)
    return Problem(
        topic=TOPIC, subtopic="limits", difficulty=1,
        question=(
            f"Evaluate the limit (0/0 form):\n\n"
            f"  lim   (x² − {a**2}) / (x − {a})\n"
            f"  x→{a}"
        ),
        answer=ans,
        hint="Factor the numerator as a difference of squares.",
        explanation=(
            f"Factor: x² − {a**2} = (x − {a})(x + {a})\n"
            f"Cancel: (x − {a})(x + {a}) / (x − {a}) = x + {a}   (for x ≠ {a})\n"
            f"Limit: lim_{{x→{a}}} (x + {a}) = {a} + {a} = {ans}"
        ),
        problem_type="numeric",
    )
