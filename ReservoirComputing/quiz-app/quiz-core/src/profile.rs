//! UserProfile: per-user mastery map, session history, and review queue.

use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::mastery::{MasteryRecord, INITIAL_MASTERY, confidence_to_quality};

/// How many recently-seen question IDs to keep for recency penalty.
pub const RECENCY_WINDOW: usize = 60;

/// Learning rate constants (can be overridden at call site).
pub const LR_CORRECT: f64 = 0.15;
pub const LR_WRONG: f64 = 0.10;

// ── SessionResult ─────────────────────────────────────────────────────────────

/// Immutable record of one completed quiz session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    pub session_id: String,
    pub timestamp: String,
    pub scope_label: String,
    pub n_questions: u32,
    pub n_correct: u32,
    pub duration_secs: f64,
    #[serde(default)]
    pub wrong_chapters: Vec<u32>,
    /// `{difficulty: [correct_count, total_count]}`
    #[serde(default)]
    pub per_difficulty: HashMap<String, [u32; 2]>,
    #[serde(default)]
    pub streak_max: u32,
    #[serde(default)]
    pub n_generated: u32,
}

impl SessionResult {
    pub fn new(scope_label: impl Into<String>) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now().to_rfc3339(),
            scope_label: scope_label.into(),
            n_questions: 0,
            n_correct: 0,
            duration_secs: 0.0,
            wrong_chapters: Vec::new(),
            per_difficulty: HashMap::new(),
            streak_max: 0,
            n_generated: 0,
        }
    }

    /// Score as a percentage (0–100).
    pub fn score_pct(&self) -> u32 {
        if self.n_questions == 0 {
            return 0;
        }
        100 * self.n_correct / self.n_questions
    }
}

// ── UserProfile ───────────────────────────────────────────────────────────────

/// Persistent user record: mastery per chapter, session history, review queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub name: String,
    /// Chapter index (as string key in JSON) → mastery record.
    #[serde(default)]
    pub mastery: HashMap<u32, MasteryRecord>,
    /// Rolling window of the last `RECENCY_WINDOW` question IDs seen.
    #[serde(default)]
    pub seen_question_ids: Vec<String>,
    /// Serialised `SessionResult` objects (kept as raw JSON values for
    /// forward-compatibility).
    #[serde(default)]
    pub session_history: Vec<SessionResult>,
    /// Question IDs flagged for manual re-quiz.
    #[serde(default)]
    pub review_queue: Vec<String>,
    /// Per-question SM-2 records.
    #[serde(default)]
    pub question_mastery: HashMap<String, MasteryRecord>,
    /// Consecutive parse/validation failures for generated content, keyed by
    /// `"ch{N}_{difficulty}"`.
    #[serde(default)]
    pub generated_strikes: HashMap<String, u32>,
    /// Has the user completed the onboarding flow?
    #[serde(default)]
    pub onboarded: bool,
    pub created_at: String,
    pub last_seen: String,
}

impl UserProfile {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            user_id: Uuid::new_v4().to_string(),
            name: name.into(),
            mastery: HashMap::new(),
            seen_question_ids: Vec::new(),
            session_history: Vec::new(),
            review_queue: Vec::new(),
            question_mastery: HashMap::new(),
            generated_strikes: HashMap::new(),
            onboarded: false,
            created_at: now.clone(),
            last_seen: now,
        }
    }

    // ── Mastery helpers ───────────────────────────────────────────────────────

    /// Get (or create) the mastery record for `chapter`.
    pub fn get_mastery(&mut self, chapter: u32) -> &mut MasteryRecord {
        self.mastery.entry(chapter).or_insert_with(MasteryRecord::new)
    }

    /// Get the mastery score for `chapter` (returns `INITIAL_MASTERY` if unseen).
    pub fn chapter_score(&self, chapter: u32) -> f64 {
        self.mastery
            .get(&chapter)
            .map_or(INITIAL_MASTERY, |r| r.score)
    }

    /// Update chapter mastery and per-question SR after answering a question.
    ///
    /// Returns the updated chapter mastery score.
    pub fn record_answer(
        &mut self,
        chapter: u32,
        correct: bool,
        question_id: &str,
        confidence: u8,
    ) -> f64 {
        let now = Utc::now().to_rfc3339();
        let quality = confidence_to_quality(correct, confidence);

        // Chapter-level mastery
        let ch_rec = self.mastery.entry(chapter).or_insert_with(MasteryRecord::new);
        let stability = (ch_rec.total_seen as f64 / 20.0).min(1.0);
        let eff_correct = LR_CORRECT * (1.0 - 0.5 * stability);
        let eff_wrong = LR_WRONG * (1.0 - 0.5 * stability);

        ch_rec.total_seen += 1;
        if correct {
            ch_rec.total_correct += 1;
            ch_rec.score = (ch_rec.score + eff_correct * (1.0 - ch_rec.score)).min(1.0);
        } else {
            ch_rec.score = (ch_rec.score - eff_wrong * ch_rec.score).max(0.0);
        }
        ch_rec.last_seen = now.clone();
        ch_rec.schedule_review(quality);
        let new_score = ch_rec.score;

        // Per-question SR tracking
        let q_rec = self
            .question_mastery
            .entry(question_id.to_string())
            .or_insert_with(MasteryRecord::new);
        q_rec.total_seen += 1;
        if correct {
            q_rec.total_correct += 1;
        }
        q_rec.schedule_review(quality);

        // Update last_seen timestamp on profile
        self.last_seen = now.clone();

        // Rolling recency window
        self.seen_question_ids.push(question_id.to_string());
        if self.seen_question_ids.len() > RECENCY_WINDOW {
            let drain_to = self.seen_question_ids.len() - RECENCY_WINDOW;
            self.seen_question_ids.drain(..drain_to);
        }

        new_score
    }

    // ── Review queue ──────────────────────────────────────────────────────────

    pub fn add_to_review_queue(&mut self, question_id: &str) {
        if !self.review_queue.contains(&question_id.to_string()) {
            self.review_queue.push(question_id.to_string());
        }
    }

    pub fn remove_from_review_queue(&mut self, question_id: &str) {
        self.review_queue.retain(|id| id != question_id);
    }

    // ── Analytics helpers ─────────────────────────────────────────────────────

    /// Chapter indices whose next_review date is today or earlier.
    pub fn chapters_due_for_review(&self) -> Vec<u32> {
        self.mastery
            .iter()
            .filter(|(_, rec)| rec.is_due())
            .map(|(ch, _)| *ch)
            .collect()
    }

    /// Up to `top_n` chapter indices sorted by ascending mastery (weakest first).
    pub fn weakest_chapters(&self, top_n: usize) -> Vec<u32> {
        let mut chapters: Vec<u32> = self.mastery.keys().copied().collect();
        chapters.sort_by(|a, b| {
            self.mastery[a]
                .score
                .partial_cmp(&self.mastery[b].score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        chapters.truncate(top_n);
        chapters
    }

    /// Sum of `total_seen` across all chapter mastery records.
    pub fn total_answered(&self) -> u32 {
        self.mastery.values().map(|r| r.total_seen).sum()
    }

    /// Append a completed session to `session_history`.
    pub fn add_session(&mut self, result: SessionResult) {
        self.session_history.push(result);
    }
}
