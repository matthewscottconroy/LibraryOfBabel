"""SQLite persistence — ELO ratings, problem history, SRS, profiles."""

from __future__ import annotations

import sqlite3
from datetime import datetime, date, timedelta
from pathlib import Path
from typing import Optional

DB_PATH = Path.home() / ".math_tutor" / "progress.db"

ALL_TOPICS = [
    "analysis", "linalg", "multivariable", "vectors",
    "odes", "fourier", "pdes", "complex",
]
STARTING_ELO = 450.0

# ── ELO formula (inlined to avoid circular import with adaptive) ──────────────
_K = 24.0
_D1 = 480.0
_STEP = 120.0


def _elo_update(player: float, difficulty: int, actual: float) -> float:
    """actual ∈ {0.0, 0.5, 1.0}  (0.5 = hint-assisted correct)."""
    item = _D1 + (difficulty - 1) * _STEP
    expected = 1.0 / (1.0 + 10.0 ** ((item - player) / 400.0))
    return max(100.0, min(1200.0, player + _K * (actual - expected)))


# ── Connection ─────────────────────────────────────────────────────────────────

def _conn() -> sqlite3.Connection:
    DB_PATH.parent.mkdir(parents=True, exist_ok=True)
    c = sqlite3.connect(DB_PATH)
    c.row_factory = sqlite3.Row
    c.execute("PRAGMA journal_mode=WAL")
    return c


# ── Schema ─────────────────────────────────────────────────────────────────────

_DDL = """
CREATE TABLE IF NOT EXISTS mastery (
    profile     TEXT NOT NULL DEFAULT 'default',
    topic       TEXT NOT NULL,
    elo         REAL NOT NULL DEFAULT 450.0,
    seen        INTEGER NOT NULL DEFAULT 0,
    correct     INTEGER NOT NULL DEFAULT 0,
    streak      INTEGER NOT NULL DEFAULT 0,
    best_streak INTEGER NOT NULL DEFAULT 0,
    last_seen   TEXT,
    PRIMARY KEY (profile, topic)
);

CREATE TABLE IF NOT EXISTS subtopic_mastery (
    profile       TEXT NOT NULL DEFAULT 'default',
    topic         TEXT NOT NULL,
    subtopic      TEXT NOT NULL,
    elo           REAL NOT NULL DEFAULT 450.0,
    seen          INTEGER NOT NULL DEFAULT 0,
    correct       INTEGER NOT NULL DEFAULT 0,
    interval_days REAL NOT NULL DEFAULT 1.0,
    ease_factor   REAL NOT NULL DEFAULT 2.5,
    next_review   TEXT,
    last_seen     TEXT,
    PRIMARY KEY (profile, topic, subtopic)
);

CREATE TABLE IF NOT EXISTS history (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    profile    TEXT NOT NULL DEFAULT 'default',
    topic      TEXT NOT NULL,
    subtopic   TEXT NOT NULL,
    difficulty INTEGER NOT NULL,
    correct    INTEGER NOT NULL,
    hint_used  INTEGER NOT NULL DEFAULT 0,
    skipped    INTEGER NOT NULL DEFAULT 0,
    seconds    REAL,
    ts         TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    profile            TEXT NOT NULL DEFAULT 'default',
    started_at         TEXT NOT NULL,
    ended_at           TEXT,
    problems_attempted INTEGER NOT NULL DEFAULT 0,
    problems_correct   INTEGER NOT NULL DEFAULT 0,
    total_seconds      REAL
);

CREATE TABLE IF NOT EXISTS meta (
    key   TEXT PRIMARY KEY,
    value TEXT
);
"""


def _migrate(c: sqlite3.Connection) -> None:
    """Upgrade schema v1 (topic-only PK) → v2 (profile+topic PK)."""
    existing = {r[0] for r in c.execute(
        "SELECT name FROM sqlite_master WHERE type='table'")}
    cols = {r[1] for r in c.execute("PRAGMA table_info(mastery)")}

    if "profile" not in cols:
        # Rebuild mastery with composite PK
        c.executescript("""
            ALTER TABLE mastery RENAME TO _mastery_old;
            CREATE TABLE mastery (
                profile     TEXT NOT NULL DEFAULT 'default',
                topic       TEXT NOT NULL,
                elo         REAL NOT NULL DEFAULT 450.0,
                seen        INTEGER NOT NULL DEFAULT 0,
                correct     INTEGER NOT NULL DEFAULT 0,
                streak      INTEGER NOT NULL DEFAULT 0,
                best_streak INTEGER NOT NULL DEFAULT 0,
                last_seen   TEXT,
                PRIMARY KEY (profile, topic)
            );
            INSERT INTO mastery (profile, topic, elo, seen, correct, last_seen)
            SELECT 'default', topic, elo, seen, correct, last_seen
            FROM _mastery_old;
            DROP TABLE _mastery_old;
        """)

    if "history" in existing:
        hcols = {r[1] for r in c.execute("PRAGMA table_info(history)")}
        if "profile" not in hcols:
            c.executescript("""
                ALTER TABLE history RENAME TO _history_old;
                CREATE TABLE history (
                    id         INTEGER PRIMARY KEY AUTOINCREMENT,
                    profile    TEXT NOT NULL DEFAULT 'default',
                    topic      TEXT NOT NULL,
                    subtopic   TEXT NOT NULL,
                    difficulty INTEGER NOT NULL,
                    correct    INTEGER NOT NULL,
                    hint_used  INTEGER NOT NULL DEFAULT 0,
                    skipped    INTEGER NOT NULL DEFAULT 0,
                    seconds    REAL,
                    ts         TEXT NOT NULL
                );
                INSERT INTO history (profile, topic, subtopic, difficulty, correct, seconds, ts)
                SELECT 'default', topic, subtopic, difficulty, correct, seconds, ts
                FROM _history_old;
                DROP TABLE _history_old;
            """)

    # Create new tables that didn't exist in v1
    c.executescript("""
        CREATE TABLE IF NOT EXISTS subtopic_mastery (
            profile       TEXT NOT NULL DEFAULT 'default',
            topic         TEXT NOT NULL,
            subtopic      TEXT NOT NULL,
            elo           REAL NOT NULL DEFAULT 450.0,
            seen          INTEGER NOT NULL DEFAULT 0,
            correct       INTEGER NOT NULL DEFAULT 0,
            interval_days REAL NOT NULL DEFAULT 1.0,
            ease_factor   REAL NOT NULL DEFAULT 2.5,
            next_review   TEXT,
            last_seen     TEXT,
            PRIMARY KEY (profile, topic, subtopic)
        );
        CREATE TABLE IF NOT EXISTS sessions (
            id                 INTEGER PRIMARY KEY AUTOINCREMENT,
            profile            TEXT NOT NULL DEFAULT 'default',
            started_at         TEXT NOT NULL,
            ended_at           TEXT,
            problems_attempted INTEGER NOT NULL DEFAULT 0,
            problems_correct   INTEGER NOT NULL DEFAULT 0,
            total_seconds      REAL
        );
    """)


def init_db(profile: str = "default") -> None:
    with _conn() as c:
        tables = {r[0] for r in c.execute(
            "SELECT name FROM sqlite_master WHERE type='table'")}

        if "mastery" in tables:
            _migrate(c)
        else:
            c.executescript(_DDL)

        for topic in ALL_TOPICS:
            c.execute(
                "INSERT OR IGNORE INTO mastery (profile, topic, elo) VALUES (?,?,?)",
                (profile, topic, STARTING_ELO),
            )

        c.execute("INSERT OR REPLACE INTO meta VALUES ('schema_version','2')")


def reset_db(profile: str = "default") -> None:
    with _conn() as c:
        c.execute("DELETE FROM mastery WHERE profile=?", (profile,))
        c.execute("DELETE FROM subtopic_mastery WHERE profile=?", (profile,))
        c.execute("DELETE FROM history WHERE profile=?", (profile,))
        c.execute("DELETE FROM sessions WHERE profile=?", (profile,))
    init_db(profile)


# ── Read helpers ───────────────────────────────────────────────────────────────

def get_mastery(profile: str = "default") -> dict[str, float]:
    with _conn() as c:
        rows = c.execute(
            "SELECT topic, elo FROM mastery WHERE profile=?", (profile,)
        ).fetchall()
    return {r["topic"]: r["elo"] for r in rows}


def get_stats(profile: str = "default") -> list[dict]:
    with _conn() as c:
        rows = c.execute(
            """SELECT topic, elo, seen, correct, streak, best_streak, last_seen
               FROM mastery WHERE profile=? ORDER BY rowid""",
            (profile,)
        ).fetchall()
    return [dict(r) for r in rows]


def get_subtopic_elo(profile: str, topic: str, subtopic: str) -> float:
    with _conn() as c:
        row = c.execute(
            "SELECT elo FROM subtopic_mastery WHERE profile=? AND topic=? AND subtopic=?",
            (profile, topic, subtopic)
        ).fetchone()
    return float(row["elo"]) if row else STARTING_ELO


def get_overdue_subtopics(profile: str, topic: str) -> list[str]:
    """Subtopics whose SRS review date is today or past."""
    today = date.today().isoformat()
    with _conn() as c:
        rows = c.execute(
            """SELECT subtopic FROM subtopic_mastery
               WHERE profile=? AND topic=? AND next_review <= ?""",
            (profile, topic, today)
        ).fetchall()
    return [r["subtopic"] for r in rows]


def get_session_count(profile: str = "default") -> int:
    with _conn() as c:
        row = c.execute(
            "SELECT COUNT(*) AS n FROM sessions WHERE profile=? AND ended_at IS NOT NULL",
            (profile,)
        ).fetchone()
    return row["n"] if row else 0


# ── Write helpers ──────────────────────────────────────────────────────────────

def update_mastery(
    topic: str, elo: float, correct: bool, actual_score: float,
    seconds: float, subtopic: str, difficulty: int,
    hint_used: bool = False, skipped: bool = False,
    profile: str = "default",
) -> None:
    """
    actual_score: 1.0=correct, 0.5=correct-with-hint, 0.0=wrong/skip.
    `correct` tracks raw correctness for accuracy stats.
    """
    now = datetime.now().isoformat(timespec="seconds")
    with _conn() as c:
        row = c.execute(
            "SELECT streak, best_streak FROM mastery WHERE profile=? AND topic=?",
            (profile, topic)
        ).fetchone()
        streak = ((row["streak"] + 1) if correct else 0) if row else int(correct)
        best_streak = max(row["best_streak"] if row else 0, streak)

        c.execute(
            """UPDATE mastery
               SET elo=?, seen=seen+1, correct=correct+?,
                   streak=?, best_streak=?, last_seen=?
               WHERE profile=? AND topic=?""",
            (elo, int(correct), streak, best_streak, now, profile, topic),
        )
        c.execute(
            """INSERT INTO history
               (profile,topic,subtopic,difficulty,correct,hint_used,skipped,seconds,ts)
               VALUES (?,?,?,?,?,?,?,?,?)""",
            (profile, topic, subtopic, difficulty, int(correct),
             int(hint_used), int(skipped), seconds, now),
        )

        # Subtopic ELO + SRS
        sm = c.execute(
            """SELECT elo, seen, correct, interval_days, ease_factor
               FROM subtopic_mastery WHERE profile=? AND topic=? AND subtopic=?""",
            (profile, topic, subtopic)
        ).fetchone()

        if sm:
            sub_elo = _elo_update(sm["elo"], difficulty, actual_score)
            sub_seen = sm["seen"] + 1
            sub_correct = sm["correct"] + int(correct)
            interval = sm["interval_days"]
            ease = sm["ease_factor"]
        else:
            sub_elo = _elo_update(STARTING_ELO, difficulty, actual_score)
            sub_seen = 1
            sub_correct = int(correct)
            interval = 1.0
            ease = 2.5

        if correct:
            interval = max(1.0, round(interval * ease, 1))
            ease = min(4.0, ease + 0.1)
        else:
            interval = 1.0
            ease = max(1.3, ease - 0.2)

        next_review = (date.today() + timedelta(days=max(1, int(interval)))).isoformat()

        c.execute(
            """INSERT INTO subtopic_mastery
               (profile,topic,subtopic,elo,seen,correct,interval_days,ease_factor,next_review,last_seen)
               VALUES (?,?,?,?,?,?,?,?,?,?)
               ON CONFLICT(profile,topic,subtopic) DO UPDATE SET
               elo=excluded.elo, seen=excluded.seen, correct=excluded.correct,
               interval_days=excluded.interval_days, ease_factor=excluded.ease_factor,
               next_review=excluded.next_review, last_seen=excluded.last_seen""",
            (profile, topic, subtopic, sub_elo, sub_seen, sub_correct,
             interval, ease, next_review, now)
        )


def start_session(profile: str = "default") -> int:
    now = datetime.now().isoformat(timespec="seconds")
    with _conn() as c:
        cur = c.execute(
            "INSERT INTO sessions (profile, started_at) VALUES (?,?)", (profile, now)
        )
        return cur.lastrowid


def end_session(session_id: int, attempted: int, correct: int,
                total_seconds: float) -> None:
    now = datetime.now().isoformat(timespec="seconds")
    with _conn() as c:
        c.execute(
            """UPDATE sessions SET ended_at=?, problems_attempted=?,
               problems_correct=?, total_seconds=? WHERE id=?""",
            (now, attempted, correct, total_seconds, session_id)
        )
