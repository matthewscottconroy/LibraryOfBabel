"""
HoTT question generator — app-specific system prompt + BaseGenerator engine.
Re-exports base_generator internals so existing tests import from one place.
"""
from __future__ import annotations

from .base_generator import (
    BaseGenerator,
    _add_to_cache,
    _build_user_prompt,
    _cache_path,
    _load_cache,
    _load_chapter_excerpt,
    _parse_and_validate,
    _pop_from_cache,
    _save_cache,
)
from .config import (
    ANTHROPIC_API_KEY,
    CACHE_CAP_PER_DIFFICULTY,
    CACHE_DIR,
    CHAPTER_EXCERPT_CHARS,
    CHAPTERS_DIR,
)


class ClaudeGenerator(BaseGenerator):

    _SYSTEM_PROMPT = """\
You are an expert quiz question generator for a graduate-level course on \
Homotopy Type Theory (HoTT). Topics include: dependent types, identity types \
and path induction, the univalence axiom, higher inductive types (circles, \
suspensions, pushouts), homotopy levels (contractible, propositions, sets), \
the Curry-Howard correspondence, synthetic homotopy theory (π₁(S¹)≅ℤ, \
Blakers-Massey, Hopf fibration, Freudenthal suspension theorem), cubical type \
theory (CCHM interval, hcomp, Glue types, canonicity), simplicial type theory \
(Segal types, Rezk completeness, synthetic Yoneda), modal HoTT (flat/sharp \
modalities, cohesion), and formalisation in Lean 4/Mathlib4 and Cubical Agda \
(agda/cubical library). \
Your questions test genuine mathematical understanding — not surface memorisation. \
Every wrong answer (distractor) must be plausible to a student who has a \
partial understanding of the topic. \
Prefer formal notation (type-theoretic judgments, inference rules) where it \
tests understanding, and use fill-in-the-blank format for key definitions.\
"""

    def __init__(self, api_key: str = ANTHROPIC_API_KEY):
        super().__init__(api_key)
