"""
Ordinary Differential Equations — laplace (difficulty 5).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


x, t = sp.symbols("x t")
TOPIC = "odes"


def generate() -> Problem:
    func, transform, expl = random.choice([
        (
            "L{t·e^(at)}",
            "1/(s−a)²",
            "Frequency-shift: L{t·e^(at)} = L{t} shifted: 1/s² → 1/(s−a)²."
        ),
        (
            "L{sin(ωt)}",
            "ω/(s²+ω²)",
            "Standard Laplace table entry: L{sin(ωt)} = ω/(s²+ω²)."
        ),
        (
            "L{t²}",
            "2/s³",
            "L{tⁿ} = n!/sⁿ⁺¹. For n=2: L{t²} = 2!/s³ = 2/s³."
        ),
        (
            "L{δ(t−a)}  (Dirac delta shifted by a)",
            "e^(−as)",
            "L{δ(t−a)} = e^(−as) for a > 0. The sifting property gives ∫₀^∞ e^(−st)δ(t−a)dt = e^(−as)."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="laplace", difficulty=5,
        question=f"What is the Laplace transform:\n\n  {func}",
        answer=transform,
        hint="Use the Laplace table and shift theorems.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[transform, "1/(s+a)", "s/(s²+ω²)", "1/s²"],
    )
