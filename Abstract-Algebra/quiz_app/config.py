"""Central configuration for the Abstract Algebra Adaptive Quiz."""
from __future__ import annotations

import os
from pathlib import Path

PACKAGE_DIR   = Path(__file__).parent
PROJECT_ROOT  = PACKAGE_DIR.parent
CHAPTERS_DIR  = PROJECT_ROOT / "curriculum"
DATA_DIR      = PACKAGE_DIR / "data"
PROGRESS_DIR  = DATA_DIR / "progress"
CACHE_DIR     = DATA_DIR / "cache"

for _d in (DATA_DIR, PROGRESS_DIR, CACHE_DIR):
    _d.mkdir(parents=True, exist_ok=True)

ANTHROPIC_API_KEY: str = os.environ.get("ANTHROPIC_API_KEY", "")
CLAUDE_MODEL = "claude-sonnet-4-6"
CHAPTER_EXCERPT_CHARS = 4_000
GENERATE_BATCH_SIZE = 5
CACHE_CAP_PER_DIFFICULTY = 20

LEARNING_RATE_CORRECT = 0.15
LEARNING_RATE_WRONG   = 0.10
INITIAL_MASTERY       = 0.30
RECENCY_PENALTY       = 0.05
RECENCY_WINDOW        = 60
DIFF_BEGINNER_MAX      = 0.35
DIFF_INTERMEDIATE_MAX  = 0.68
BLANK_WEIGHT_BONUS     = 1.5    # extra weight for fill-in-the-blank questions
CACHE_MAX_AGE_DAYS     = 90     # discard cached generated questions older than this

# Adaptive session auto-stop (plateau detection)
AUTO_STOP_WINDOW    = 5
AUTO_STOP_THRESHOLD = 0.015

# Paths to sibling apps for cross-app unified dashboard
PEER_APP_DIRS: list[Path] = [
    Path(__file__).parent.parent.parent / "Homotopy-Type-Theory"                     / "quiz_app",
    Path(__file__).parent.parent.parent / "Computational-Systems-Synthetic-Biology" / "quiz_app",
]

CHAPTER_META: dict[int, dict] = {
    0:  dict(phase=0, name="Logic, Sets, and Proof",            file="ch01-logic-sets-proof.md",                 demos=["ch01-logic", "ch02-sets"]),
    1:  dict(phase=0, name="Relations, Functions, Cardinality", file="ch02-relations-functions-cardinality.md",  demos=["ch02-sets", "ch03-cardinality"]),
    2:  dict(phase=1, name="Fields and Vector Spaces",          file="ch03-fields-and-vector-spaces.md",         demos=["ch04-vector-spaces", "ch05-bases-dimension"]),
    3:  dict(phase=1, name="Linear Maps and Matrices",          file="ch04-linear-maps-and-matrices.md",         demos=["ch06-linear-maps", "ch07-matrices"]),
    4:  dict(phase=1, name="Determinants and Multilinear Forms",file="ch05-determinants-and-multilinear-forms.md",demos=["ch08-determinants"]),
    5:  dict(phase=1, name="Eigentheory and Canonical Forms",   file="ch06-eigentheory-and-canonical-forms.md",  demos=["ch09-eigentheory", "ch10-canonical-forms"]),
    6:  dict(phase=1, name="Inner Product Spaces",              file="ch07-inner-product-spaces.md",             demos=["ch11-inner-products"]),
    7:  dict(phase=1, name="Multilinear Algebra and Tensors",   file="ch08-multilinear-algebra-tensors.md",      demos=["ch12-multilinear", "ch28-tensor-products"]),
    8:  dict(phase=2, name="Group Theory Foundations",          file="ch09-group-theory-foundations.md",         demos=["ch13-groups", "ch14-cosets", "ch15-homomorphisms"]),
    9:  dict(phase=2, name="Group Theory Structure",            file="ch10-group-theory-structure.md",           demos=["ch16-group-actions", "ch17-sylow", "ch18-group-structure", "ch19-abelian-groups"]),
    10: dict(phase=3, name="Ring Theory",                       file="ch11-ring-theory.md",                      demos=["ch20-rings", "ch21-ideals", "ch22-divisibility", "ch23-polynomials"]),
    11: dict(phase=3, name="Module Theory",                     file="ch12-module-theory.md",                    demos=["ch25-modules", "ch26-projective-injective", "ch27-structure-theorem", "ch28-tensor-products"]),
    12: dict(phase=3, name="Field Theory and Galois",           file="ch13-field-theory-galois.md",              demos=["ch29-field-extensions", "ch30-normal-separable", "ch31-galois-theory", "ch32-galois-applications"]),
    13: dict(phase=4, name="Category Theory",                   file="ch14-category-theory.md",                  demos=["ch33-categories", "ch34-yoneda", "ch35-adjoints", "ch36-limits-colimits"]),
    14: dict(phase=5, name="Homological Algebra Basics",        file="ch15-homological-algebra-basics.md",       demos=["ch37-abelian-categories", "ch38-chain-complexes", "ch39-resolutions"]),
    15: dict(phase=5, name="Derived Functors",                  file="ch16-derived-functors.md",                 demos=["ch39-resolutions", "ch40-ext-tor"]),
    16: dict(phase=5, name="Spectral Sequences",                file="ch17-spectral-sequences.md",               demos=["ch41-spectral-sequences"]),
    17: dict(phase=6, name="Representations of Finite Groups",  file="ch18-representations-finite-groups.md",    demos=["ch42-representations", "ch43-complete-reducibility"]),
    18: dict(phase=6, name="Character Theory",                  file="ch19-character-theory.md",                 demos=["ch44-character-theory", "ch45-induced-representations"]),
    19: dict(phase=7, name="Lie Groups and Algebras",           file="ch20-lie-groups-algebras.md",              demos=["ch46-lie-groups", "ch47-lie-algebras"]),
    20: dict(phase=7, name="Semisimple Lie Algebras",           file="ch21-semisimple-lie-algebras.md",          demos=["ch48-solvable-semisimple", "ch49-root-systems"]),
    21: dict(phase=7, name="Highest Weight Theory",             file="ch22-highest-weight-theory.md",            demos=["ch50-highest-weight"]),
    22: dict(phase=7, name="Advanced Representation Theory",    file="ch23-advanced-representation-theory.md",   demos=["ch51-modular", "ch52-geometric", "ch53-quantum-groups"]),
    23: dict(phase=8, name="Set Theory and Logic",              file="ch24-set-theory-logic.md",                 demos=["ch01-logic", "ch02-sets"]),
    24: dict(phase=8, name="Model Theory",                      file="ch25-model-theory.md",                     demos=[]),
    25: dict(phase=8, name="Category Theory (Foundations)",     file="ch26-category-theory-foundation.md",       demos=["ch33-categories", "ch34-yoneda", "ch35-adjoints"]),
    26: dict(phase=8, name="Topos and Homotopy Type Theory",    file="ch27-topos-homotopy-type-theory.md",       demos=["ch54-langlands"]),
    27: dict(phase=8, name="Overview and Index",                file="index.md",                                 demos=[]),
}

PHASE_NAMES: dict[int, str] = {
    0: "Phase 0 — Mathematical Foundations",
    1: "Phase 1 — Linear Algebra",
    2: "Phase 2 — Group Theory",
    3: "Phase 3 — Rings, Modules, and Fields",
    4: "Phase 4 — Category Theory",
    5: "Phase 5 — Homological Algebra",
    6: "Phase 6 — Representation Theory",
    7: "Phase 7 — Lie Theory",
    8: "Phase 8 — Foundations and Logic",
}

DIFFICULTY_LEVELS = ("beginner", "intermediate", "advanced")
