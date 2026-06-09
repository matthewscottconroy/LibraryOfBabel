use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, params};
use std::path::Path;

const SCHEMA: &str = "
PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS mastery (
    profile      TEXT NOT NULL,
    topic        TEXT NOT NULL,
    elo          REAL NOT NULL DEFAULT 400.0,
    streak       INTEGER NOT NULL DEFAULT 0,
    best_streak  INTEGER NOT NULL DEFAULT 0,
    seen         INTEGER NOT NULL DEFAULT 0,
    correct      INTEGER NOT NULL DEFAULT 0,
    last_seen    TEXT,
    PRIMARY KEY (profile, topic)
);

CREATE TABLE IF NOT EXISTS subtopic_mastery (
    profile   TEXT NOT NULL,
    topic     TEXT NOT NULL,
    subtopic  TEXT NOT NULL,
    elo       REAL NOT NULL DEFAULT 400.0,
    seen      INTEGER NOT NULL DEFAULT 0,
    correct   INTEGER NOT NULL DEFAULT 0,
    last_seen TEXT,
    PRIMARY KEY (profile, topic, subtopic)
);

CREATE TABLE IF NOT EXISTS history (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    profile      TEXT NOT NULL,
    session_id   INTEGER NOT NULL,
    topic        TEXT NOT NULL,
    subtopic     TEXT NOT NULL,
    difficulty   INTEGER NOT NULL,
    problem_type TEXT NOT NULL,
    correct      INTEGER NOT NULL,
    skipped      INTEGER NOT NULL DEFAULT 0,
    hint_used    INTEGER NOT NULL DEFAULT 0,
    elo_before   REAL NOT NULL,
    elo_after    REAL NOT NULL,
    timestamp    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    profile          TEXT NOT NULL,
    started_at       TEXT NOT NULL,
    ended_at         TEXT,
    problems_seen    INTEGER NOT NULL DEFAULT 0,
    problems_correct INTEGER NOT NULL DEFAULT 0
);
";

pub struct Db {
    conn: Connection,
}

// ── Open / init ───────────────────────────────────────────────────────────────

impl Db {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        let db = Db { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(SCHEMA)?;
        Ok(())
    }
}

// ── Mastery ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MasteryRow {
    pub topic:       String,
    pub elo:         f64,
    pub streak:      i64,
    pub best_streak: i64,
    pub seen:        i64,
    pub correct:     i64,
    pub last_seen:   Option<DateTime<Utc>>,
}

impl Db {
    pub fn get_mastery(&self, profile: &str, topic: &str) -> Result<MasteryRow> {
        // Ensure row exists
        self.conn.execute(
            "INSERT OR IGNORE INTO mastery (profile, topic) VALUES (?1, ?2)",
            params![profile, topic],
        )?;
        let row = self.conn.query_row(
            "SELECT elo, streak, best_streak, seen, correct, last_seen
             FROM mastery WHERE profile=?1 AND topic=?2",
            params![profile, topic],
            |r| {
                Ok(MasteryRow {
                    topic:       topic.to_string(),
                    elo:         r.get(0)?,
                    streak:      r.get(1)?,
                    best_streak: r.get(2)?,
                    seen:        r.get(3)?,
                    correct:     r.get(4)?,
                    last_seen:   None, // filled below
                })
            },
        )?;
        Ok(row)
    }

    pub fn all_mastery(&self, profile: &str) -> Result<Vec<MasteryRow>> {
        // Ensure all topics exist
        for topic in crate::problems::types::Topic::all() {
            self.conn.execute(
                "INSERT OR IGNORE INTO mastery (profile, topic) VALUES (?1, ?2)",
                params![profile, topic.as_str()],
            )?;
        }
        let mut stmt = self.conn.prepare(
            "SELECT topic, elo, streak, best_streak, seen, correct, last_seen
             FROM mastery WHERE profile=?1"
        )?;
        let rows = stmt.query_map(params![profile], |r| {
            let last_seen_str: Option<String> = r.get(6)?;
            let last_seen = last_seen_str
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|d| d.with_timezone(&Utc));
            Ok(MasteryRow {
                topic:       r.get(0)?,
                elo:         r.get(1)?,
                streak:      r.get(2)?,
                best_streak: r.get(3)?,
                seen:        r.get(4)?,
                correct:     r.get(5)?,
                last_seen,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn update_mastery(
        &self,
        profile: &str,
        topic: &str,
        subtopic: &str,
        new_elo: f64,
        correct: bool,
        _actual_score: f64,
        difficulty: u8,
        problem_type: &str,
        hint_used: bool,
        skipped: bool,
        elo_before: f64,
        session_id: i64,
    ) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        // Compute new streak
        let old = self.get_mastery(profile, topic)?;
        let new_streak = if correct && !skipped { old.streak + 1 } else { 0 };
        let new_best = new_streak.max(old.best_streak);

        self.conn.execute(
            "UPDATE mastery SET
               elo=?1, streak=?2, best_streak=?3,
               seen=seen+1, correct=correct+?4, last_seen=?5
             WHERE profile=?6 AND topic=?7",
            params![
                new_elo, new_streak, new_best,
                if correct { 1 } else { 0 },
                now, profile, topic
            ],
        )?;

        // Subtopic mastery
        self.conn.execute(
            "INSERT OR IGNORE INTO subtopic_mastery (profile, topic, subtopic)
             VALUES (?1, ?2, ?3)",
            params![profile, topic, subtopic],
        )?;
        self.conn.execute(
            "UPDATE subtopic_mastery SET
               seen=seen+1, correct=correct+?1, last_seen=?2
             WHERE profile=?3 AND topic=?4 AND subtopic=?5",
            params![
                if correct { 1 } else { 0 },
                now, profile, topic, subtopic
            ],
        )?;

        // History
        self.conn.execute(
            "INSERT INTO history
               (profile, session_id, topic, subtopic, difficulty, problem_type,
                correct, skipped, hint_used, elo_before, elo_after, timestamp)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
            params![
                profile, session_id, topic, subtopic, difficulty, problem_type,
                if correct { 1 } else { 0 },
                if skipped { 1 } else { 0 },
                if hint_used { 1 } else { 0 },
                elo_before, new_elo, now
            ],
        )?;

        Ok(())
    }

    pub fn get_overdue_subtopics(&self, profile: &str, topic: &str) -> Result<Vec<String>> {
        let cutoff = (Utc::now() - chrono::Duration::days(3)).to_rfc3339();
        let mut stmt = self.conn.prepare(
            "SELECT subtopic FROM subtopic_mastery
             WHERE profile=?1 AND topic=?2
               AND (last_seen IS NULL OR last_seen < ?3)
             ORDER BY last_seen ASC LIMIT 10"
        )?;
        let rows = stmt.query_map(params![profile, topic, cutoff], |r| r.get(0))?
            .collect::<rusqlite::Result<Vec<String>>>()?;
        Ok(rows)
    }
}

// ── Sessions ──────────────────────────────────────────────────────────────────

impl Db {
    pub fn start_session(&self, profile: &str) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO sessions (profile, started_at) VALUES (?1, ?2)",
            params![profile, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn end_session(&self, session_id: i64, seen: u32, correct: u32) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE sessions SET ended_at=?1, problems_seen=?2, problems_correct=?3
             WHERE id=?4",
            params![now, seen, correct, session_id],
        )?;
        Ok(())
    }

    pub fn session_count(&self, profile: &str) -> Result<i64> {
        Ok(self.conn.query_row(
            "SELECT COUNT(*) FROM sessions WHERE profile=?1 AND ended_at IS NOT NULL",
            params![profile],
            |r| r.get(0),
        )?)
    }
}

// ── Stats ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TopicStats {
    pub topic:       String,
    pub elo:         f64,
    pub streak:      i64,
    pub best_streak: i64,
    pub seen:        i64,
    pub correct:     i64,
    pub last_seen:   Option<DateTime<Utc>>,
}

impl Db {
    pub fn get_stats(&self, profile: &str) -> Result<Vec<TopicStats>> {
        self.all_mastery(profile).map(|rows| {
            rows.into_iter().map(|m| TopicStats {
                topic:       m.topic,
                elo:         m.elo,
                streak:      m.streak,
                best_streak: m.best_streak,
                seen:        m.seen,
                correct:     m.correct,
                last_seen:   m.last_seen,
            }).collect()
        })
    }
}

// ── Reset ─────────────────────────────────────────────────────────────────────

impl Db {
    pub fn reset(&self, profile: &str) -> Result<()> {
        self.conn.execute("DELETE FROM mastery WHERE profile=?1", params![profile])?;
        self.conn.execute("DELETE FROM subtopic_mastery WHERE profile=?1", params![profile])?;
        self.conn.execute("DELETE FROM history WHERE profile=?1", params![profile])?;
        self.conn.execute("DELETE FROM sessions WHERE profile=?1", params![profile])?;
        Ok(())
    }
}
