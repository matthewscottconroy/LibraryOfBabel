"""Adaptive engine: ELO ratings, topic unlocking, and problem selection."""

from __future__ import annotations

import random
from datetime import datetime, timezone
from typing import Optional

from .db import ALL_TOPICS, get_overdue_subtopics

# ── ELO constants ─────────────────────────────────────────────────────────────

K_FACTOR = 24.0
ELO_PER_DIFF = 120.0
ELO_D1_MID = 480.0
UNLOCK_ELO = 620.0

# Problem-type difficulty modifiers: MC/TF are easier → lower item ELO
PTYPE_OFFSET: dict[str, float] = {
    "symbolic":       0.0,
    "numeric":      -30.0,
    "true_false":   -50.0,
    "multiple_choice": -70.0,
}

# ── Prerequisite graph ────────────────────────────────────────────────────────

PREREQUISITES: dict[str, list[str]] = {
    "analysis":      [],
    "linalg":        [],
    "multivariable": ["analysis", "linalg"],
    "vectors":       ["multivariable"],
    "odes":          ["analysis", "linalg"],
    "fourier":       ["odes"],
    "pdes":          ["fourier", "vectors"],
    "complex":       ["multivariable"],
}

TOPIC_LABELS = {
    "analysis":      "Real Analysis",
    "linalg":        "Linear Algebra",
    "multivariable": "Multivariable Calculus",
    "vectors":       "Vector Calculus",
    "odes":          "Ordinary Differential Equations",
    "fourier":       "Fourier Analysis",
    "pdes":          "Partial Differential Equations",
    "complex":       "Complex Analysis",
}


# ── ELO maths ─────────────────────────────────────────────────────────────────

def difficulty_elo(difficulty: int, problem_type: str = "symbolic") -> float:
    base = ELO_D1_MID + (difficulty - 1) * ELO_PER_DIFF
    return base + PTYPE_OFFSET.get(problem_type, 0.0)


def expected_score(player_elo: float, item_elo: float) -> float:
    return 1.0 / (1.0 + 10.0 ** ((item_elo - player_elo) / 400.0))


def update_elo(player_elo: float, difficulty: int, actual: float,
               problem_type: str = "symbolic") -> float:
    """
    actual: 1.0=correct, 0.5=correct-with-hint, 0.0=wrong.
    problem_type adjusts the item ELO so easier formats yield less gain.
    """
    item_elo = difficulty_elo(difficulty, problem_type)
    exp = expected_score(player_elo, item_elo)
    return max(100.0, min(1200.0, player_elo + K_FACTOR * (float(actual) - exp)))


def elo_to_difficulty(elo: float) -> int:
    d = int((elo - ELO_D1_MID) / ELO_PER_DIFF) + 1
    return max(1, min(5, d))


def elo_to_pct(elo: float) -> float:
    return max(0.0, min(100.0, (elo - 100.0) / (1200.0 - 100.0) * 100.0))


# ── Topic availability ────────────────────────────────────────────────────────

def unlocked_topics(mastery: dict[str, float]) -> list[str]:
    return [
        t for t in ALL_TOPICS
        if all(mastery.get(p, 0.0) >= UNLOCK_ELO
               for p in PREREQUISITES.get(t, []))
    ]


# ── Problem selection ─────────────────────────────────────────────────────────

def select_topic(mastery: dict[str, float], last_topic: Optional[str],
                 stats: Optional[list[dict]] = None,
                 profile: str = "default") -> str:
    available = unlocked_topics(mastery)
    if len(available) == 1:
        return available[0]

    candidates = [t for t in available if t != last_topic] or available

    now = datetime.now(timezone.utc)
    weights = []
    for t in candidates:
        elo = mastery.get(t, 450.0)
        pct = elo_to_pct(elo) / 100.0

        recency = 1.0
        if stats:
            row = next((r for r in stats if r["topic"] == t), None)
            if row and row.get("last_seen"):
                try:
                    ls = datetime.fromisoformat(row["last_seen"])
                    if ls.tzinfo is None:
                        ls = ls.replace(tzinfo=timezone.utc)
                    hours = (now - ls).total_seconds() / 3600.0
                    recency = 1.0 + min(hours / 24.0, 2.0)
                except ValueError:
                    pass

        # Boost topics with overdue SRS subtopics
        overdue = get_overdue_subtopics(profile, t)
        srs_boost = 1.0 + 0.5 * min(len(overdue), 4)

        weight = recency * srs_boost * (1.0 - 0.7 * pct + 0.05)
        weights.append(weight)

    return random.choices(candidates, weights=weights, k=1)[0]


def select_difficulty(elo: float) -> int:
    """70% at natural level, 20% one below (review), 10% one above (challenge)."""
    base = elo_to_difficulty(elo)
    roll = random.random()
    if roll < 0.70:
        return base
    elif roll < 0.90:
        return max(1, base - 1)
    else:
        return min(5, base + 1)
