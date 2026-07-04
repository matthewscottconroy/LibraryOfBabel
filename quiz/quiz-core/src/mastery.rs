//! Per-question and per-chapter mastery tracking using the SM-2 algorithm.
//!
//! ## SM-2 interval schedule
//! - I(1) = 1 day
//! - I(2) = 6 days
//! - I(n) = round(I(n-1) × EF)  for n > 2
//!
//! ## Easiness-factor update
//! ```text
//! EF' = EF + 0.1 − (5 − q) × (0.08 + (5 − q) × 0.02)
//! ```
//! Clamped to [1.3, 3.0].  If q < 3: reset interval to 1 and rep_count to 0.
//!
//! ## Quality mapping (`confidence_to_quality`)
//! | correct | confidence | q |
//! |---------|-----------|---|
//! | false   | any       | 1 |
//! | true    | 3 (sure)  | 5 |
//! | true    | 2 (unsure)| 4 |
//! | true    | 1 (guess) | 3 |

use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

/// Starting mastery score for chapters the user hasn't touched yet.
pub const INITIAL_MASTERY: f64 = 0.30;

// ── Quality mapping ───────────────────────────────────────────────────────────

/// Map `(correct, confidence)` to an SM-2 quality score in `{0..=5}`.
///
/// `confidence` is 1 (guess), 2 (unsure), or 3 (certain).
/// Wrong answers always yield quality 1.
pub fn confidence_to_quality(correct: bool, confidence: u8) -> u8 {
    if !correct {
        return 1;
    }
    match confidence {
        3 => 5,
        2 => 4,
        1 => 3,
        _ => 4,
    }
}

// ── MasteryRecord ─────────────────────────────────────────────────────────────

/// Tracks learning progress for a single chapter (or question).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryRecord {
    /// Rolling mastery score in [0.0, 1.0].
    pub score: f64,
    /// Total questions seen (for this chapter/question).
    pub total_seen: u32,
    /// Total correct answers.
    pub total_correct: u32,
    /// ISO-8601 datetime string of the last attempt (`""`= never).
    pub last_seen: String,
    /// ISO date (`YYYY-MM-DD`) when this item is next due for review.
    /// Empty string means "not yet scheduled".
    pub next_review: String,
    /// Current spaced-repetition interval in days.
    pub interval_days: u32,
    /// SM-2 easiness factor, clamped to [1.3, 3.0].
    pub easiness_factor: f64,
    /// Consecutive successful recalls (resets to 0 on quality < 3).
    pub rep_count: u32,
}

impl Default for MasteryRecord {
    fn default() -> Self {
        Self {
            score: INITIAL_MASTERY,
            total_seen: 0,
            total_correct: 0,
            last_seen: String::new(),
            next_review: String::new(),
            interval_days: 1,
            easiness_factor: 2.5,
            rep_count: 0,
        }
    }
}

impl MasteryRecord {
    pub fn new() -> Self {
        Self::default()
    }

    /// Fraction of seen questions answered correctly.
    pub fn accuracy(&self) -> f64 {
        if self.total_seen == 0 {
            return 0.0;
        }
        self.total_correct as f64 / self.total_seen as f64
    }

    /// True when today's date is on or past `next_review`.
    pub fn is_due(&self) -> bool {
        if self.next_review.is_empty() {
            return false;
        }
        let today = Local::now().date_naive().format("%Y-%m-%d").to_string();
        today >= self.next_review
    }

    /// Advance the SM-2 schedule given a response quality `q ∈ {0..=5}`.
    ///
    /// Updates `interval_days`, `easiness_factor`, `rep_count`, and
    /// `next_review`.
    pub fn schedule_review(&mut self, quality: u8) {
        let q = quality as f64;
        let ef = self.easiness_factor + 0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02);
        self.easiness_factor = ef.max(1.3).min(3.0);

        if quality < 3 {
            self.interval_days = 1;
            self.rep_count = 0;
        } else {
            self.interval_days = match self.rep_count {
                0 => 1,
                1 => 6,
                _ => {
                    let next = (self.interval_days as f64 * self.easiness_factor).round() as u32;
                    next.max(1)
                }
            };
            self.interval_days = self.interval_days.min(365);
            self.rep_count += 1;
        }

        let review_date = Local::now()
            .date_naive()
            .checked_add_days(chrono::Days::new(self.interval_days as u64))
            .unwrap_or_else(|| Local::now().date_naive());
        self.next_review = review_date.format("%Y-%m-%d").to_string();
    }

    /// Record an answer and update the mastery score + SR schedule.
    ///
    /// `lr_correct` / `lr_wrong` are the base learning rates (0.15 / 0.10).
    /// A stability factor reduces the effective rate as the user gains
    /// experience (prevents thrashing on well-known material).
    ///
    /// Returns the updated chapter score.
    pub fn record_answer(
        &mut self,
        correct: bool,
        confidence: u8,
        lr_correct: f64,
        lr_wrong: f64,
        timestamp: &str,
    ) -> f64 {
        let quality = confidence_to_quality(correct, confidence);
        let stability = (self.total_seen as f64 / 20.0).min(1.0);
        let eff_correct = lr_correct * (1.0 - 0.5 * stability);
        let eff_wrong = lr_wrong * (1.0 - 0.5 * stability);

        self.total_seen += 1;
        if correct {
            self.total_correct += 1;
            self.score = (self.score + eff_correct * (1.0 - self.score)).min(1.0);
        } else {
            self.score = (self.score - eff_wrong * self.score).max(0.0);
        }
        self.last_seen = timestamp.to_string();
        self.schedule_review(quality);
        self.score
    }
}

// ── Parsing helper for ISO date comparisons ───────────────────────────────────

/// Parse a `YYYY-MM-DD` string into a `NaiveDate`, returning `None` on failure.
pub fn parse_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn confidence_mapping() {
        assert_eq!(confidence_to_quality(false, 3), 1);
        assert_eq!(confidence_to_quality(true, 3), 5);
        assert_eq!(confidence_to_quality(true, 2), 4);
        assert_eq!(confidence_to_quality(true, 1), 3);
    }

    #[test]
    fn correct_answer_raises_score_wrong_lowers_it() {
        let mut rec = MasteryRecord::new();
        let start = rec.score;
        rec.record_answer(true, 3, LR_CORRECT, LR_WRONG, "2026-01-01T00:00:00Z");
        assert!(rec.score > start, "correct answer must raise mastery");
        assert_eq!(rec.total_correct, 1);

        let after_correct = rec.score;
        rec.record_answer(false, 1, LR_CORRECT, LR_WRONG, "2026-01-02T00:00:00Z");
        assert!(rec.score < after_correct, "wrong answer must lower mastery");
        assert_eq!(rec.total_seen, 2);
    }

    #[test]
    fn sm2_interval_grows_on_success_and_resets_on_failure() {
        let mut rec = MasteryRecord::new();
        // Two successful reviews: interval should progress 1 -> 6 territory.
        rec.schedule_review(5);
        let i1 = rec.interval_days;
        rec.schedule_review(5);
        let i2 = rec.interval_days;
        assert!(i2 >= i1, "interval should not shrink on repeated success");
        assert!(rec.rep_count >= 2);

        // A failed review resets the interval to 1 day and rep_count to 0.
        rec.schedule_review(1);
        assert_eq!(rec.interval_days, 1);
        assert_eq!(rec.rep_count, 0);
    }

    // Local constants mirror profile.rs learning rates so mastery tests are
    // self-contained.
    const LR_CORRECT: f64 = 0.15;
    const LR_WRONG: f64 = 0.10;
}
