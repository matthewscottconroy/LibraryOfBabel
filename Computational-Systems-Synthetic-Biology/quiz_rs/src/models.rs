use std::collections::HashMap;

use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::{INITIAL_MASTERY, RECENCY_WINDOW};

// ── Answer ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Answer {
    Index(usize),
    Text(String),
}

// ── Question ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub question_id: String,
    pub chapter:     u32,
    pub phase:       u32,
    pub kind:        String,  // "mc" | "tf" | "blank"
    pub text:        String,
    pub choices:     Vec<String>,
    pub answer:      Answer,
    pub explanation: String,
    pub tags:        Vec<String>,
    #[serde(default = "default_difficulty")]
    pub difficulty:  String,  // "beginner" | "intermediate" | "advanced"
    #[serde(default)]
    pub generated:   bool,
}

fn default_difficulty() -> String { "intermediate".into() }

impl Question {
    pub fn new_id() -> String { Uuid::new_v4().to_string() }

    pub fn validate(&self) -> Vec<String> {
        let mut errs = Vec::new();
        if self.text.trim().is_empty() {
            errs.push("text is empty".into());
        }
        match (&self.kind[..], &self.answer) {
            ("mc", Answer::Index(i)) => {
                if *i >= self.choices.len() {
                    errs.push(format!("answer index {} out of range", i));
                }
                if self.choices.len() < 2 || self.choices.len() > 4 {
                    errs.push("mc must have 2-4 choices".into());
                }
            }
            ("mc", _) => errs.push("mc answer must be an int index".into()),
            ("tf", Answer::Index(i)) => {
                if self.choices.len() != 2 {
                    errs.push("tf must have exactly 2 choices".into());
                }
                if *i > 1 {
                    errs.push("tf answer index must be 0 or 1".into());
                }
            }
            ("tf", _) => errs.push("tf answer must be an int index".into()),
            ("blank", Answer::Text(t)) => {
                if t.is_empty() {
                    errs.push("blank answer must be non-empty".into());
                }
                if self.choices.is_empty() {
                    errs.push("blank needs at least one acceptable answer".into());
                }
            }
            ("blank", _) => errs.push("blank answer must be a string".into()),
            _ => {}
        }
        if self.explanation.trim().is_empty() {
            errs.push("explanation is empty".into());
        }
        if !["beginner", "intermediate", "advanced"].contains(&&self.difficulty[..]) {
            errs.push(format!("unknown difficulty: {}", self.difficulty));
        }
        errs
    }
}

// ── MasteryRecord ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryRecord {
    pub score:         f64,
    pub total_seen:    u32,
    pub total_correct: u32,
    pub last_seen:     String,
    pub next_review:   String,
    pub interval_days: i64,
}

impl Default for MasteryRecord {
    fn default() -> Self {
        Self {
            score:         INITIAL_MASTERY,
            total_seen:    0,
            total_correct: 0,
            last_seen:     String::new(),
            next_review:   String::new(),
            interval_days: 1,
        }
    }
}

impl MasteryRecord {
    pub fn accuracy(&self) -> f64 {
        if self.total_seen == 0 { 0.0 } else { self.total_correct as f64 / self.total_seen as f64 }
    }

    pub fn is_due(&self) -> bool {
        if self.next_review.is_empty() { return false; }
        let today = Local::now().date_naive().to_string();
        today.as_str() >= self.next_review.as_str()
    }

    pub fn schedule_review(&mut self, correct: bool, confidence: u8) {
        if !correct {
            self.interval_days = 1;
        } else {
            self.interval_days = if self.interval_days < 2 {
                3
            } else if self.interval_days < 5 {
                7
            } else if self.interval_days < 10 {
                14
            } else {
                (self.interval_days * 2).min(180)
            };
            let scale = match confidence { 1 => 0.5, 3 => 1.5, _ => 1.0 };
            self.interval_days = ((self.interval_days as f64 * scale).round() as i64).max(1);
        }
        let next = Local::now().date_naive()
            + chrono::Duration::days(self.interval_days);
        self.next_review = next.to_string();
    }
}

// ── UserProfile ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id:           String,
    pub name:              String,
    #[serde(default)]
    pub mastery:           HashMap<u32, MasteryRecord>,
    #[serde(default)]
    pub seen_question_ids: Vec<String>,
    #[serde(default)]
    pub session_history:   Vec<serde_json::Value>,
    #[serde(default)]
    pub review_queue:      Vec<String>,
    #[serde(default)]
    pub question_mastery:  HashMap<String, MasteryRecord>,
    #[serde(default)]
    pub generated_strikes: HashMap<String, u32>,
    #[serde(default)]
    pub onboarded:         bool,
    pub created_at:        String,
    pub last_seen:         String,
}

impl UserProfile {
    pub fn new(name: String) -> Self {
        let now = now_utc();
        Self {
            user_id:           Uuid::new_v4().to_string(),
            name,
            mastery:           HashMap::new(),
            seen_question_ids: Vec::new(),
            session_history:   Vec::new(),
            review_queue:      Vec::new(),
            question_mastery:  HashMap::new(),
            generated_strikes: HashMap::new(),
            onboarded:         false,
            created_at:        now.clone(),
            last_seen:         now,
        }
    }

    pub fn get_mastery(&mut self, chapter: u32) -> &mut MasteryRecord {
        self.mastery.entry(chapter).or_default()
    }

    pub fn record_answer(
        &mut self,
        chapter: u32,
        correct: bool,
        question_id: &str,
        lr_correct: f64,
        lr_wrong: f64,
        confidence: u8,
    ) -> f64 {
        let rec = self.mastery.entry(chapter).or_default();
        let stability = (rec.total_seen as f64 / 20.0).min(1.0);
        let eff_correct = lr_correct * (1.0 - 0.5 * stability);
        let eff_wrong   = lr_wrong   * (1.0 - 0.5 * stability);

        rec.total_seen += 1;
        if correct {
            rec.total_correct += 1;
            rec.score = (rec.score + eff_correct * (1.0 - rec.score)).min(1.0);
        } else {
            rec.score = (rec.score - eff_wrong * rec.score).max(0.0);
        }
        rec.last_seen = now_utc();
        rec.schedule_review(correct, confidence);

        let q_rec = self.question_mastery.entry(question_id.to_string()).or_default();
        q_rec.total_seen += 1;
        if correct { q_rec.total_correct += 1; }
        q_rec.schedule_review(correct, confidence);

        self.last_seen = now_utc();
        self.seen_question_ids.push(question_id.to_string());
        if self.seen_question_ids.len() > RECENCY_WINDOW {
            let drain = self.seen_question_ids.len() - RECENCY_WINDOW;
            self.seen_question_ids.drain(..drain);
        }

        self.mastery[&chapter].score
    }

    pub fn add_to_review_queue(&mut self, question_id: &str) {
        if !self.review_queue.iter().any(|id| id == question_id) {
            self.review_queue.push(question_id.to_string());
        }
    }

    pub fn remove_from_review_queue(&mut self, question_id: &str) {
        self.review_queue.retain(|id| id != question_id);
    }

    pub fn chapters_due_for_review(&self) -> Vec<u32> {
        self.mastery.iter()
            .filter(|(_, rec)| rec.is_due())
            .map(|(&ch, _)| ch)
            .collect()
    }

    pub fn weakest_chapters(&self, top_n: usize) -> Vec<u32> {
        let mut chs: Vec<u32> = self.mastery.keys().copied().collect();
        chs.sort_by(|a, b| {
            self.mastery[a].score.partial_cmp(&self.mastery[b].score).unwrap()
        });
        chs.truncate(top_n);
        chs
    }

    pub fn total_answered(&self) -> u32 {
        self.mastery.values().map(|r| r.total_seen).sum()
    }
}

// ── SessionResult ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    pub session_id:      String,
    pub timestamp:       String,
    pub scope_label:     String,
    pub n_questions:     u32,
    pub n_correct:       u32,
    pub duration_secs:   f64,
    pub wrong_chapters:  Vec<u32>,
    #[serde(default)]
    pub per_difficulty:  HashMap<String, [u32; 2]>,
    #[serde(default)]
    pub streak_max:      u32,
    #[serde(default)]
    pub n_generated:     u32,
}

impl SessionResult {
    pub fn score_pct(&self) -> u32 {
        if self.n_questions == 0 { return 0; }
        100 * self.n_correct / self.n_questions
    }

    pub fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

pub fn now_utc() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub fn today_iso() -> String {
    Local::now().date_naive().to_string()
}
