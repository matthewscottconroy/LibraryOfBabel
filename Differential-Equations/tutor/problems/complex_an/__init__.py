"""
Complex Analysis generators — one file per question, auto-discovered by difficulty.

To add a question: create d{n}_your_name.py with a generate() -> Problem function.
"""
from __future__ import annotations
import importlib
import re
from pathlib import Path


def _discover() -> dict[int, list]:
    here = Path(__file__).parent
    gens: dict[int, list] = {}
    for path in sorted(here.glob("d[1-5]_*.py")):
        m = re.match(r"d([1-5])_", path.name)
        if m:
            d = int(m.group(1))
            mod = importlib.import_module(f".{path.stem}", package=__name__)
            gens.setdefault(d, []).append(mod.generate)
    return gens


GENERATORS = _discover()
