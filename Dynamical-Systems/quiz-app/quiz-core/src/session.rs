//! A single interactive quiz session backed by an ordered list of questions.
//!
//! This module is intentionally lightweight — it tracks answers and computes
//! scores but does not touch mastery or storage.  Callers feed results back into
//! [`crate::profile::UserProfile`] after the session ends.

use crate::question::Question;
use crate::profile::SessionResult;

/// Tracks progress through a single quiz session.
#[derive(Debug, Clone)]
pub struct Session {
    /// The ordered list of questions for this session.
    pub questions: Vec<Question>,
    /// One slot per question: `None` = not yet answered, `Some(s)` = submitted answer.
    pub answers: Vec<Option<String>>,
    /// Index of the next unanswered question.
    pub current: usize,
    /// Scope label carried from the selection step (used in `SessionResult`).
    pub scope_label: String,
    /// Wall-clock start time (seconds since Unix epoch).
    pub started_at: std::time::Instant,
}

impl Session {
    pub fn new(questions: Vec<Question>, scope_label: impl Into<String>) -> Self {
        let n = questions.len();
        Self {
            answers: vec![None; n],
            questions,
            current: 0,
            scope_label: scope_label.into(),
            started_at: std::time::Instant::now(),
        }
    }

    /// The current unanswered question, or `None` when the session is finished.
    pub fn current_question(&self) -> Option<&Question> {
        self.questions.get(self.current)
    }

    /// Submit an answer for `self.current`, advance the cursor, and return
    /// whether the answer was correct.
    ///
    /// Returns `None` when the session is already finished.
    pub fn submit(&mut self, answer: String) -> Option<bool> {
        let q = self.questions.get(self.current)?;
        let correct = q.check(&answer);
        self.answers[self.current] = Some(answer);
        self.current += 1;
        Some(correct)
    }

    /// `(correct_count, total_questions)`.
    pub fn score(&self) -> (usize, usize) {
        let total = self.questions.len();
        let correct = self
            .answers
            .iter()
            .enumerate()
            .filter(|(i, a)| {
                a.as_deref()
                    .map(|ans| self.questions[*i].check(ans))
                    .unwrap_or(false)
            })
            .count();
        (correct, total)
    }

    /// True once all questions have been answered.
    pub fn is_finished(&self) -> bool {
        self.current >= self.questions.len()
    }

    /// `(answered_so_far, total)`.
    pub fn progress(&self) -> (usize, usize) {
        let answered = self.answers.iter().filter(|a| a.is_some()).count();
        (answered, self.questions.len())
    }

    /// Build a `SessionResult` from the completed session.
    ///
    /// `wrong_chapters` is populated from the chapters of incorrectly answered
    /// questions.
    pub fn to_result(&self) -> SessionResult {
        let (n_correct, n_questions) = self.score();
        let duration = self.started_at.elapsed().as_secs_f64();

        let wrong_chapters: Vec<u32> = self
            .answers
            .iter()
            .enumerate()
            .filter_map(|(i, a)| {
                let ans = a.as_deref()?;
                if self.questions[i].check(ans) {
                    None
                } else {
                    Some(self.questions[i].chapter())
                }
            })
            .collect();

        // Per-difficulty breakdown
        let mut per_difficulty: std::collections::HashMap<String, [u32; 2]> =
            std::collections::HashMap::new();
        for (i, answer) in self.answers.iter().enumerate() {
            if let Some(ans) = answer {
                let diff = self.questions[i].difficulty().to_string();
                let correct = self.questions[i].check(ans);
                let entry = per_difficulty.entry(diff).or_insert([0, 0]);
                if correct {
                    entry[0] += 1;
                }
                entry[1] += 1;
            }
        }

        // Maximum correct streak
        let streak_max = self
            .answers
            .iter()
            .enumerate()
            .map(|(i, a)| {
                a.as_deref()
                    .map(|ans| self.questions[i].check(ans))
                    .unwrap_or(false)
            })
            .fold((0u32, 0u32), |(max, cur), correct| {
                let next = if correct { cur + 1 } else { 0 };
                (max.max(next), next)
            })
            .0;

        let n_generated = self.questions.iter().filter(|q| q.generated()).count() as u32;

        SessionResult {
            n_questions: n_questions as u32,
            n_correct: n_correct as u32,
            duration_secs: duration,
            wrong_chapters,
            per_difficulty,
            streak_max,
            n_generated,
            ..SessionResult::new(self.scope_label.clone())
        }
    }
}
