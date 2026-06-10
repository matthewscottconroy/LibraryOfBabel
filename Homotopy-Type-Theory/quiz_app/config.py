"""
Central configuration: paths, chapter metadata, difficulty bands.
"""
from __future__ import annotations

import os
from pathlib import Path

# ── Paths ─────────────────────────────────────────────────────────────────────

PACKAGE_DIR   = Path(__file__).parent
PROJECT_ROOT  = PACKAGE_DIR.parent
CHAPTERS_DIR  = PROJECT_ROOT / "chapters"
DATA_DIR      = PACKAGE_DIR / "data"
PROGRESS_DIR  = DATA_DIR / "progress"
CACHE_DIR     = DATA_DIR / "cache"

for _d in (DATA_DIR, PROGRESS_DIR, CACHE_DIR):
    _d.mkdir(parents=True, exist_ok=True)

# ── Claude settings ───────────────────────────────────────────────────────────

ANTHROPIC_API_KEY: str = os.environ.get("ANTHROPIC_API_KEY", "")
CLAUDE_MODEL = "claude-sonnet-4-6"

# Max tokens in the chapter excerpt sent to Claude (keeps costs low)
CHAPTER_EXCERPT_CHARS = 4_000
# How many questions to generate per cache batch
GENERATE_BATCH_SIZE = 5
# Questions per chapter before we stop caching (per difficulty)
CACHE_CAP_PER_DIFFICULTY = 20

# ── Adaptive engine knobs ─────────────────────────────────────────────────────

LEARNING_RATE_CORRECT = 0.15   # mastery gain per correct answer
LEARNING_RATE_WRONG   = 0.10   # mastery loss per wrong answer
INITIAL_MASTERY       = 0.30   # starting mastery for an unseen chapter
RECENCY_PENALTY       = 0.05   # weight for a question seen in last N answers
RECENCY_WINDOW        = 60     # "recent" = seen within this many answers

BLANK_WEIGHT_BONUS     = 1.5    # extra weight for fill-in-the-blank questions
CACHE_MAX_AGE_DAYS     = 90     # discard cached generated questions older than this

# Difficulty thresholds (mastery score → preferred difficulty)
DIFF_BEGINNER_MAX      = 0.35
DIFF_INTERMEDIATE_MAX  = 0.68

# Adaptive session auto-stop (plateau detection)
AUTO_STOP_WINDOW    = 5     # number of recent questions to examine
AUTO_STOP_THRESHOLD = 0.015 # total mastery delta below which to prompt for stop

# Paths to sibling apps for cross-app unified dashboard
PEER_APP_DIRS: list[Path] = [
    Path(__file__).parent.parent.parent / "Abstract-Algebra"                     / "quiz_app",
    Path(__file__).parent.parent.parent / "Computational-Systems-Synthetic-Biology" / "quiz_app",
]

# ── Chapter metadata ──────────────────────────────────────────────────────────

# Maps chapter index → (phase, short_name, file_glob)
CHAPTER_META: dict[int, dict] = {
    0:  dict(phase=0, name="Logic and Proof",              file="ch00-logic-and-proof.md"),
    1:  dict(phase=0, name="Set Theory",                   file="ch01-set-theory.md"),
    2:  dict(phase=0, name="Abstract Algebra",             file="ch02-abstract-algebra.md"),
    3:  dict(phase=0, name="Real Analysis",                file="ch03-real-analysis.md"),
    4:  dict(phase=1, name="Proof Theory",                 file="ch04-proof-theory.md"),
    5:  dict(phase=1, name="Intuitionistic Logic",         file="ch05-intuitionistic-logic.md"),
    6:  dict(phase=1, name="Curry-Howard",                 file="ch06-curry-howard.md"),
    7:  dict(phase=1, name="STLC and System F",            file="ch07-stlc-system-f.md"),
    8:  dict(phase=2, name="Dependent Types",              file="ch08-dependent-types.md"),
    9:  dict(phase=2, name="Martin-Löf Type Theory",       file="ch09-mltt.md"),
    10: dict(phase=3, name="Category Theory",              file="ch10-category-theory.md"),
    11: dict(phase=3, name="Categorical Logic",            file="ch11-categorical-logic.md"),
    12: dict(phase=3, name="Higher Categories",            file="ch12-higher-categories.md"),
    13: dict(phase=4, name="Topology",                     file="ch13-topology.md"),
    14: dict(phase=4, name="Homotopy Theory",              file="ch14-homotopy-theory.md"),
    15: dict(phase=4, name="Simplicial Sets",              file="ch15-simplicial-sets.md"),
    16: dict(phase=5, name="Identity Types",               file="ch16-identity-types.md"),
    17: dict(phase=5, name="H-Levels and Truncations",     file="ch17-h-levels.md"),
    18: dict(phase=5, name="Univalence",                   file="ch18-univalence.md"),
    19: dict(phase=5, name="Higher Inductive Types",       file="ch19-higher-inductive-types.md"),
    20: dict(phase=5, name="Synthetic Homotopy Theory",    file="ch20-synthetic-homotopy.md"),
    21: dict(phase=6, name="Lean 4 and Mathlib",           file="ch21-lean4.md"),
    22: dict(phase=6, name="Cubical Agda",                 file="ch22-cubical-agda.md"),
    23: dict(phase=7, name="Cubical Type Theory",          file="ch23-cubical-type-theory.md"),
    24: dict(phase=7, name="Simplicial Type Theory",       file="ch24-simplicial-type-theory.md"),
    25: dict(phase=7, name="Modal HoTT",                   file="ch25-modal-hott.md"),
    26: dict(phase=8, name="Research Frontiers",           file="ch26-research-frontiers.md"),
}

PHASE_NAMES: dict[int, str] = {
    0: "Phase 0 — Mathematical Foundations",
    1: "Phase 1 — Logic and Computation",
    2: "Phase 2 — Dependent Types and MLTT",
    3: "Phase 3 — Category Theory",
    4: "Phase 4 — Topology and Homotopy Theory",
    5: "Phase 5 — Core HoTT",
    6: "Phase 6 — Proof Assistants",
    7: "Phase 7 — Advanced Foundations",
    8: "Phase 8 — Research Frontiers",
}

DIFFICULTY_LEVELS = ("beginner", "intermediate", "advanced")
