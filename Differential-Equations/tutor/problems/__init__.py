"""Problem registry — maps (topic, difficulty) → list of generator functions."""

from __future__ import annotations

import random
from typing import Callable, Optional

from .base import Problem

# Import each module's generator lists
from . import analysis, linalg, multivariable, vectors, odes, fourier, pdes, complex_an

# Registry: topic → difficulty → [generator_fn, ...]
_REGISTRY: dict[str, dict[int, list[Callable[[], Problem]]]] = {
    "analysis":     analysis.GENERATORS,
    "linalg":       linalg.GENERATORS,
    "multivariable": multivariable.GENERATORS,
    "vectors":      vectors.GENERATORS,
    "odes":         odes.GENERATORS,
    "fourier":      fourier.GENERATORS,
    "pdes":         pdes.GENERATORS,
    "complex":      complex_an.GENERATORS,
}


def get_problem(topic: str, difficulty: int) -> Optional[Problem]:
    """
    Pick a random generator for (topic, difficulty) and call it.
    Returns None if no generator is available at that level.
    Falls back to nearest available difficulty.
    """
    topic_map = _REGISTRY.get(topic)
    if topic_map is None:
        return None

    # Try exact difficulty, then walk towards available levels
    for d in _fallback_order(difficulty):
        generators = topic_map.get(d)
        if generators:
            fn = random.choice(generators)
            try:
                return fn()
            except Exception:
                continue   # generator failed with unlucky params — try another

    return None


def _fallback_order(d: int) -> list[int]:
    """Return [d, d-1, d+1, d-2, d+2, ...] clamped to 1–5."""
    result = [d]
    for delta in range(1, 5):
        lo, hi = d - delta, d + delta
        if 1 <= lo <= 5:
            result.append(lo)
        if 1 <= hi <= 5:
            result.append(hi)
    return result
