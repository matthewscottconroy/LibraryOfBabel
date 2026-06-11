//! Question types and the Quiz container.
//!
//! Supports four question kinds:
//!   - `MultipleChoice` — one correct index into a choices vec
//!   - `TrueFalse`      — boolean answer (stored as 0=true / 1=false index)
//!   - `FillBlank`      — short text answer (primary + synonyms in `choices`)
//!   - `Proof`          — partial proof with blanked steps; answer is pipe-joined fills
//!
//! Every variant carries the adaptive metadata needed by the engine:
//! `chapter`, `phase`, `difficulty`, `tags`, and `generated`.

use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── Difficulty ────────────────────────────────────────────────────────────────

/// The three difficulty tiers used by the adaptive engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Beginner => "beginner",
            Self::Intermediate => "intermediate",
            Self::Advanced => "advanced",
        }
    }

    /// Distance between two difficulty levels (0, 1, or 2).
    pub fn distance(&self, other: &Difficulty) -> u8 {
        let idx = |d: &Difficulty| match d {
            Difficulty::Beginner => 0u8,
            Difficulty::Intermediate => 1,
            Difficulty::Advanced => 2,
        };
        idx(self).abs_diff(idx(other))
    }
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Self::Intermediate
    }
}

// ── Question ──────────────────────────────────────────────────────────────────

/// A single quiz question.
///
/// The `kind` tag in JSON (`"mc"`, `"tf"`, `"blank"`, `"proof"`) selects
/// the variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Question {
    /// Multiple-choice: `answer` is a 0-based index into `choices`.
    #[serde(rename = "mc")]
    MultipleChoice {
        question_id: String,
        chapter: u32,
        phase: u32,
        text: String,
        choices: Vec<String>,
        /// 0-based index of the correct choice.
        answer: usize,
        explanation: String,
        #[serde(default)]
        tags: Vec<String>,
        #[serde(default)]
        difficulty: Difficulty,
        #[serde(default)]
        generated: bool,
    },
    /// True/false: `answer` 0 = True, 1 = False; `choices` = ["True","False"].
    #[serde(rename = "tf")]
    TrueFalse {
        question_id: String,
        chapter: u32,
        phase: u32,
        text: String,
        choices: Vec<String>,
        answer: usize,
        explanation: String,
        #[serde(default)]
        tags: Vec<String>,
        #[serde(default)]
        difficulty: Difficulty,
        #[serde(default)]
        generated: bool,
    },
    /// Fill-in-the-blank: `choices` holds acceptable answers (primary first);
    /// `answer` is the primary canonical answer (lowercase).
    #[serde(rename = "blank")]
    FillBlank {
        question_id: String,
        chapter: u32,
        phase: u32,
        text: String,
        choices: Vec<String>,
        answer: String,
        explanation: String,
        #[serde(default)]
        tags: Vec<String>,
        #[serde(default)]
        difficulty: Difficulty,
        #[serde(default)]
        generated: bool,
    },
    /// Proof scaffold: `choices` = proof lines (some with `___`), `answer` =
    /// pipe-separated canonical fills for each blank in order.
    #[serde(rename = "proof")]
    Proof {
        question_id: String,
        chapter: u32,
        phase: u32,
        text: String,
        choices: Vec<String>,
        answer: String,
        explanation: String,
        #[serde(default)]
        tags: Vec<String>,
        #[serde(default)]
        difficulty: Difficulty,
        #[serde(default)]
        generated: bool,
    },
}

impl Question {
    // ── Accessors ─────────────────────────────────────────────────────────────

    pub fn question_id(&self) -> &str {
        match self {
            Self::MultipleChoice { question_id, .. }
            | Self::TrueFalse { question_id, .. }
            | Self::FillBlank { question_id, .. }
            | Self::Proof { question_id, .. } => question_id,
        }
    }

    pub fn chapter(&self) -> u32 {
        match self {
            Self::MultipleChoice { chapter, .. }
            | Self::TrueFalse { chapter, .. }
            | Self::FillBlank { chapter, .. }
            | Self::Proof { chapter, .. } => *chapter,
        }
    }

    pub fn phase(&self) -> u32 {
        match self {
            Self::MultipleChoice { phase, .. }
            | Self::TrueFalse { phase, .. }
            | Self::FillBlank { phase, .. }
            | Self::Proof { phase, .. } => *phase,
        }
    }

    pub fn text(&self) -> &str {
        match self {
            Self::MultipleChoice { text, .. }
            | Self::TrueFalse { text, .. }
            | Self::FillBlank { text, .. }
            | Self::Proof { text, .. } => text,
        }
    }

    pub fn choices(&self) -> &[String] {
        match self {
            Self::MultipleChoice { choices, .. }
            | Self::TrueFalse { choices, .. }
            | Self::FillBlank { choices, .. }
            | Self::Proof { choices, .. } => choices,
        }
    }

    pub fn explanation(&self) -> &str {
        match self {
            Self::MultipleChoice { explanation, .. }
            | Self::TrueFalse { explanation, .. }
            | Self::FillBlank { explanation, .. }
            | Self::Proof { explanation, .. } => explanation,
        }
    }

    pub fn tags(&self) -> &[String] {
        match self {
            Self::MultipleChoice { tags, .. }
            | Self::TrueFalse { tags, .. }
            | Self::FillBlank { tags, .. }
            | Self::Proof { tags, .. } => tags,
        }
    }

    pub fn difficulty(&self) -> &Difficulty {
        match self {
            Self::MultipleChoice { difficulty, .. }
            | Self::TrueFalse { difficulty, .. }
            | Self::FillBlank { difficulty, .. }
            | Self::Proof { difficulty, .. } => difficulty,
        }
    }

    pub fn generated(&self) -> bool {
        match self {
            Self::MultipleChoice { generated, .. }
            | Self::TrueFalse { generated, .. }
            | Self::FillBlank { generated, .. }
            | Self::Proof { generated, .. } => *generated,
        }
    }

    pub fn kind_str(&self) -> &'static str {
        match self {
            Self::MultipleChoice { .. } => "mc",
            Self::TrueFalse { .. } => "tf",
            Self::FillBlank { .. } => "blank",
            Self::Proof { .. } => "proof",
        }
    }

    // ── Mutation helpers ──────────────────────────────────────────────────────

    /// Replace the question_id with a fresh UUID (used when popping from cache).
    pub fn with_fresh_id(mut self) -> Self {
        let new_id = Uuid::new_v4().to_string();
        match &mut self {
            Self::MultipleChoice { question_id, .. }
            | Self::TrueFalse { question_id, .. }
            | Self::FillBlank { question_id, .. }
            | Self::Proof { question_id, .. } => *question_id = new_id,
        }
        self
    }

    // ── Answer checking ───────────────────────────────────────────────────────

    /// Check whether `answer_str` is correct for this question.
    ///
    /// - `MultipleChoice` / `TrueFalse`: accepts `"A"`/`"B"`/`"C"`/`"D"`
    ///   (case-insensitive) or a bare integer index.
    /// - `FillBlank`: case-insensitive match against any acceptable answer in
    ///   `choices`.
    /// - `Proof`: pipe-separated fills, each compared case-insensitively to the
    ///   corresponding canonical fill.
    pub fn check(&self, answer_str: &str) -> bool {
        let a = answer_str.trim();
        match self {
            Self::MultipleChoice { answer, choices, .. }
            | Self::TrueFalse { answer, choices, .. } => {
                let idx: Option<usize> = match a.to_uppercase().as_str() {
                    "A" => Some(0),
                    "B" => Some(1),
                    "C" => Some(2),
                    "D" => Some(3),
                    _ => a.parse().ok(),
                };
                idx.map_or(false, |i| i < choices.len() && i == *answer)
            }
            Self::FillBlank { choices, .. } => {
                let lower = a.to_lowercase();
                choices.iter().any(|c| c.to_lowercase() == lower)
            }
            Self::Proof { answer, .. } => {
                let given: Vec<&str> = a.split('|').map(str::trim).collect();
                let canonical: Vec<&str> = answer.split('|').map(str::trim).collect();
                given.len() == canonical.len()
                    && given
                        .iter()
                        .zip(canonical.iter())
                        .all(|(g, c)| g.to_lowercase() == c.to_lowercase())
            }
        }
    }

    // ── Validation ────────────────────────────────────────────────────────────

    /// Return a list of validation errors.  Empty vec = valid.
    pub fn validate(&self) -> Vec<String> {
        let mut errs: Vec<String> = Vec::new();
        if self.text().trim().is_empty() {
            errs.push("text is empty".into());
        }
        if self.explanation().trim().is_empty() {
            errs.push("explanation is empty".into());
        }
        match self {
            Self::MultipleChoice { answer, choices, .. } => {
                if choices.len() < 2 || choices.len() > 4 {
                    errs.push(format!(
                        "mc questions must have 2-4 choices, got {}",
                        choices.len()
                    ));
                }
                if *answer >= choices.len() {
                    errs.push(format!(
                        "answer index {} out of range for {} choices",
                        answer,
                        choices.len()
                    ));
                }
            }
            Self::TrueFalse { answer, choices, .. } => {
                if choices.len() != 2 {
                    errs.push("tf questions must have exactly 2 choices".into());
                }
                if *answer > 1 {
                    errs.push(format!("tf answer index {} must be 0 or 1", answer));
                }
            }
            Self::FillBlank { choices, answer, .. } => {
                if choices.is_empty() {
                    errs.push("blank questions need at least one acceptable answer".into());
                }
                if answer.trim().is_empty() {
                    errs.push("blank answer must be a non-empty string".into());
                }
            }
            Self::Proof { choices, answer, .. } => {
                if choices.is_empty() {
                    errs.push("proof questions need at least one proof line".into());
                }
                let n_blanks: usize = choices.iter().map(|l| l.matches("___").count()).sum();
                let n_fills = answer
                    .split('|')
                    .filter(|f| !f.trim().is_empty())
                    .count();
                if n_blanks == 0 {
                    errs.push("proof choices must contain at least one '___' blank".into());
                } else if n_fills != n_blanks {
                    errs.push(format!(
                        "proof answer has {} fills but choices have {} blanks",
                        n_fills, n_blanks
                    ));
                }
            }
        }
        errs
    }

    // ── Display helpers ───────────────────────────────────────────────────────

    /// Human-readable label for the correct answer.
    pub fn correct_display(&self) -> String {
        match self {
            Self::MultipleChoice { answer, choices, .. } => {
                let label = (b'A' + *answer as u8) as char;
                format!("{}: {}", label, choices.get(*answer).map_or("?", |s| s.as_str()))
            }
            Self::TrueFalse { answer, .. } => {
                if *answer == 0 { "True".into() } else { "False".into() }
            }
            Self::FillBlank { answer, .. } => answer.clone(),
            Self::Proof { answer, .. } => answer.replace('|', " | "),
        }
    }
}

// ── Quiz container ────────────────────────────────────────────────────────────

/// A named collection of questions (e.g., one chapter's static bank).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub questions: Vec<Question>,
}

// ── I/O helpers ───────────────────────────────────────────────────────────────

/// Load a `Vec<Question>` from a flat JSON array (generator cache / chapter banks).
pub fn load_questions<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path.as_ref())
        .map_err(|e| format!("Cannot read '{}': {}", path.as_ref().display(), e))?;
    let qs: Vec<Question> = serde_json::from_str(&data)
        .map_err(|e| format!("Invalid JSON in '{}': {}", path.as_ref().display(), e))?;
    Ok(qs)
}

/// Load a [`Quiz`] from a JSON object file.
pub fn load_quiz<P: AsRef<Path>>(path: P) -> Result<Quiz, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path.as_ref())
        .map_err(|e| format!("Cannot read '{}': {}", path.as_ref().display(), e))?;
    let quiz: Quiz = serde_json::from_str(&data)
        .map_err(|e| format!("Invalid JSON in '{}': {}", path.as_ref().display(), e))?;
    Ok(quiz)
}

/// Scan `dir` for `*.json` files, try each as a flat `Vec<Question>`, and
/// collect all valid questions into one pool.
pub fn load_question_pool<P: AsRef<Path>>(
    dir: P,
) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
    let mut pool = Vec::new();
    let dir = dir.as_ref();
    if !dir.is_dir() {
        return Ok(pool);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("json") {
            if let Ok(qs) = load_questions(&path) {
                pool.extend(qs);
            }
        }
    }
    Ok(pool)
}
