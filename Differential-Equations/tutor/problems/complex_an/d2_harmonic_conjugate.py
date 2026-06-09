"""
Complex Analysis — harmonic functions (difficulty 2).
"""
from __future__ import annotations
import random
import sympy as sp
from ..base import Problem


z = sp.Symbol("z")
TOPIC = "complex"


def generate() -> Problem:
    case, conj, expl = random.choice([
        (
            "u(x,y) = x² − y²  (is harmonic: uₓₓ + uᵧᵧ = 0?)",
            "Yes; harmonic conjugate v = 2xy",
            "uₓₓ = 2, uᵧᵧ = −2. Sum = 0. ✓ Harmonic.\n"
            "Find v: vₓ = −uᵧ = 2y → v = 2xy + g(y).\n"
            "vᵧ = uₓ = 2x → 2x + g'(y) = 2x → g'(y)=0 → g=const.\n"
            "So v = 2xy (taking const=0). Then f = u+iv = z²."
        ),
        (
            "u(x,y) = eˣ cos y  (is harmonic?)",
            "Yes; harmonic conjugate v = eˣ sin y",
            "uₓ = eˣcos y, uₓₓ = eˣcos y. uᵧ = −eˣsin y, uᵧᵧ = −eˣcos y.\n"
            "Sum = 0. ✓ Harmonic. Conjugate v = eˣsin y gives f = eᶻ."
        ),
    ])
    return Problem(
        topic=TOPIC, subtopic="harmonic_functions", difficulty=2,
        question=f"Verify and find the harmonic conjugate:\n\n  {case}",
        answer=conj,
        hint="Verify Δu = 0, then integrate the C-R equations to find v.",
        explanation=expl,
        problem_type="multiple_choice",
        choices=[conj, "v = x² + y²", "v = x − y", "Not harmonic"],
    )
