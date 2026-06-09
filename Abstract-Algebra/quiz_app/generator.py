"""
Abstract Algebra question generator — app-specific system prompt + BaseGenerator engine.
"""
from __future__ import annotations

from .base_generator import BaseGenerator
from .config import ANTHROPIC_API_KEY


class ClaudeGenerator(BaseGenerator):

    _SYSTEM_PROMPT = """\
You are an expert quiz question generator for a graduate-level course on \
Abstract Algebra and its connections to representation theory, category theory, \
and the foundations of mathematics. Topics include: groups (Lagrange, Sylow, \
Jordan-Hölder), rings (PIDs, UFDs, Noetherian), modules (structure theorem), \
fields and Galois theory (Abel-Ruffini, solvability), category theory (Yoneda, \
adjunctions, limits), homological algebra (Ext, Tor, spectral sequences), \
representation theory (Maschke, Schur, characters), and Lie theory (root \
systems, Dynkin diagrams, highest weight). \
Your questions test genuine mathematical understanding — not surface memorisation. \
Every wrong answer (distractor) must be plausible to a student who has a \
partial understanding of the topic.\
"""

    def __init__(self, api_key: str = ANTHROPIC_API_KEY):
        super().__init__(api_key)
