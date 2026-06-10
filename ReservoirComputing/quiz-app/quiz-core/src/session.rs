use crate::question::{Question, Quiz};

/// Tracks progress through a single quiz attempt.
#[derive(Debug, Clone)]
pub struct Session {
    pub quiz: Quiz,
    /// Recorded answer strings, one slot per question (None = not yet answered).
    pub answers: Vec<Option<String>>,
    /// Index of the next question to answer.
    pub current: usize,
}

impl Session {
    pub fn new(quiz: Quiz) -> Self {
        let n = quiz.questions.len();
        Self {
            answers: vec![None; n],
            quiz,
            current: 0,
        }
    }

    pub fn current_question(&self) -> Option<&Question> {
        self.quiz.questions.get(self.current)
    }

    /// Submit an answer for `self.current`, advance the cursor, return correctness.
    /// Returns `None` when the quiz is already finished.
    pub fn submit(&mut self, answer: String) -> Option<bool> {
        if self.current >= self.quiz.questions.len() {
            return None;
        }
        let correct = self.quiz.questions[self.current].check(&answer);
        self.answers[self.current] = Some(answer);
        self.current += 1;
        Some(correct)
    }

    /// `(correct_count, total_questions)`.
    pub fn score(&self) -> (usize, usize) {
        let total = self.quiz.questions.len();
        let correct = self
            .answers
            .iter()
            .enumerate()
            .filter(|(i, a)| {
                a.as_deref()
                    .map(|a| self.quiz.questions[*i].check(a))
                    .unwrap_or(false)
            })
            .count();
        (correct, total)
    }

    /// True once all questions have been answered.
    pub fn is_finished(&self) -> bool {
        self.current >= self.quiz.questions.len()
    }

    /// `(answered_so_far, total)`.
    pub fn progress(&self) -> (usize, usize) {
        let answered = self.answers.iter().filter(|a| a.is_some()).count();
        (answered, self.quiz.questions.len())
    }
}
