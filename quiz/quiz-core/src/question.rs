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

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Default `question_id` for seed files that omit it: a fresh UUID.
fn gen_id() -> String {
    Uuid::new_v4().to_string()
}

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
        #[serde(default = "gen_id")]
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
        #[serde(default = "gen_id")]
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
        #[serde(default = "gen_id")]
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
        #[serde(default = "gen_id")]
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
            Self::MultipleChoice { answer, choices, .. } => {
                let idx: Option<usize> = match a.to_uppercase().as_str() {
                    "A" => Some(0),
                    "B" => Some(1),
                    "C" => Some(2),
                    "D" => Some(3),
                    _ => a.parse().ok(),
                };
                idx.map_or(false, |i| i < choices.len() && i == *answer)
            }
            Self::TrueFalse { answer, choices, .. } => {
                // Accept letter (A/B), word (true/false), single letter (t/f),
                // or a bare index — the CLI, TUI, and web submit different forms.
                let idx: Option<usize> = match a.to_uppercase().as_str() {
                    "A" | "T" | "TRUE" => Some(0),
                    "B" | "F" | "FALSE" => Some(1),
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

/// True if `name` is a chapter directory: matches `^ch\d+` (any suffix), e.g.
/// `ch00`, `ch07-stlc-system-f`, `ch3`.
fn is_chapter_dir(name: &str) -> bool {
    let rest = match name.strip_prefix("ch") {
        Some(r) => r,
        None => return false,
    };
    // Require at least one leading digit; any suffix is allowed.
    rest.chars().next().map_or(false, |c| c.is_ascii_digit())
}

/// Recursively collect `*.json` file paths under `dir` into `out`.
fn collect_json_files(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("json") {
            out.push(path);
        }
    }
    Ok(())
}

/// Parse one bank file's contents, accepting either a single [`Question`]
/// object or a JSON array of questions, and append the result to `pool`.
fn parse_bank_file(
    data: &str,
    path: &Path,
    pool: &mut Vec<Question>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Try a single object first (the on-disk seed format), then an array.
    match serde_json::from_str::<Question>(data) {
        Ok(q) => {
            pool.push(q);
            Ok(())
        }
        Err(single_err) => match serde_json::from_str::<Vec<Question>>(data) {
            Ok(qs) => {
                pool.extend(qs);
                Ok(())
            }
            Err(arr_err) => Err(format!(
                "Invalid question JSON in '{}': not a Question object ({}) nor an array ({})",
                path.display(),
                single_err,
                arr_err
            )
            .into()),
        },
    }
}

/// Load a full question bank from `dir`.
///
/// Scans direct subdirectories whose name matches `^ch\d+` (any suffix) and
/// reads **every** `*.json` file found beneath them (recursively). Each file
/// may contain either a single [`Question`] object *or* a JSON array of
/// questions. Files whose name starts with `_` are skipped (reserved for
/// legacy/uncoverted seeds, e.g. `_legacy_sample.json`).
///
/// A missing or non-directory `dir` yields an empty pool (not an error), so
/// subjects that have not been seeded yet load cleanly.
pub fn load_question_bank<P: AsRef<Path>>(
    dir: P,
) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
    let root = dir.as_ref();
    let mut pool = Vec::new();
    if !root.is_dir() {
        return Ok(pool);
    }

    // Gather chapter directories (deterministic order).
    let mut chapter_dirs: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir()
            && path
                .file_name()
                .and_then(|n| n.to_str())
                .map_or(false, is_chapter_dir)
        {
            chapter_dirs.push(path);
        }
    }
    chapter_dirs.sort();

    // Collect and sort all JSON files for stable ordering.
    let mut files: Vec<PathBuf> = Vec::new();
    for cdir in &chapter_dirs {
        collect_json_files(cdir, &mut files)?;
    }
    files.sort();

    for path in files {
        // Skip legacy/underscore-prefixed files.
        if path
            .file_name()
            .and_then(|n| n.to_str())
            .map_or(true, |n| n.starts_with('_'))
        {
            continue;
        }
        let data = std::fs::read_to_string(&path)
            .map_err(|e| format!("Cannot read '{}': {}", path.display(), e))?;
        parse_bank_file(&data, &path, &mut pool)?;
    }

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MC: &str = r#"{
        "question_id": "id-mc", "chapter": 0, "phase": 0, "kind": "mc",
        "text": "2 + 2 = ?", "choices": ["3","4","5","6"], "answer": 1,
        "explanation": "Basic arithmetic.", "tags": ["math"],
        "difficulty": "beginner", "generated": false
    }"#;

    const TF: &str = r#"{
        "question_id": "id-tf", "chapter": 1, "phase": 0, "kind": "tf",
        "text": "The sky is green.", "choices": ["True","False"], "answer": 1,
        "explanation": "It is blue.", "difficulty": "beginner"
    }"#;

    const BLANK: &str = r#"{
        "question_id": "id-blank", "chapter": 2, "phase": 1, "kind": "blank",
        "text": "Water is made of hydrogen and ___.", "choices": ["oxygen"],
        "answer": "oxygen", "explanation": "H2O.", "difficulty": "intermediate"
    }"#;

    const PROOF: &str = r#"{
        "question_id": "id-proof", "chapter": 3, "phase": 2, "kind": "proof",
        "text": "Prove n+0 = n.", "choices": ["By ___ on n.", "Base: 0+0=0."],
        "answer": "induction", "explanation": "Induction on n.",
        "difficulty": "advanced"
    }"#;

    /// An `mc` question with no `question_id` field — must get a generated one.
    const MC_NO_ID: &str = r#"{
        "chapter": 0, "phase": 0, "kind": "mc",
        "text": "Pick A.", "choices": ["A","B"], "answer": 0,
        "explanation": "A is correct."
    }"#;

    #[test]
    fn deserialize_single_of_each_kind() {
        let mc: Question = serde_json::from_str(MC).unwrap();
        assert_eq!(mc.kind_str(), "mc");
        assert_eq!(mc.chapter(), 0);
        assert!(mc.check("B")); // index 1

        let tf: Question = serde_json::from_str(TF).unwrap();
        assert_eq!(tf.kind_str(), "tf");
        assert!(tf.check("false"));

        let blank: Question = serde_json::from_str(BLANK).unwrap();
        assert_eq!(blank.kind_str(), "blank");
        assert!(blank.check("Oxygen"));

        let proof: Question = serde_json::from_str(PROOF).unwrap();
        assert_eq!(proof.kind_str(), "proof");
        assert!(proof.check("induction"));
    }

    #[test]
    fn deserialize_array() {
        let arr = format!("[{MC},{TF}]");
        let qs: Vec<Question> = serde_json::from_str(&arr).unwrap();
        assert_eq!(qs.len(), 2);
        assert_eq!(qs[0].kind_str(), "mc");
        assert_eq!(qs[1].kind_str(), "tf");
    }

    #[test]
    fn missing_question_id_gets_default_uuid() {
        let q: Question = serde_json::from_str(MC_NO_ID).unwrap();
        // A fresh, non-empty UUID string should have been generated.
        assert!(!q.question_id().is_empty());
        // Two independent parses generate distinct ids.
        let q2: Question = serde_json::from_str(MC_NO_ID).unwrap();
        assert_ne!(q.question_id(), q2.question_id());
    }

    #[test]
    fn parse_bank_file_accepts_object_or_array() {
        let mut pool = Vec::new();
        parse_bank_file(MC, Path::new("a.json"), &mut pool).unwrap();
        assert_eq!(pool.len(), 1);
        let arr = format!("[{MC},{TF},{BLANK}]");
        parse_bank_file(&arr, Path::new("b.json"), &mut pool).unwrap();
        assert_eq!(pool.len(), 4);
    }

    #[test]
    fn parse_bank_file_rejects_garbage() {
        let mut pool = Vec::new();
        let err = parse_bank_file("{ not json", Path::new("bad.json"), &mut pool);
        assert!(err.is_err());
    }

    #[test]
    fn is_chapter_dir_matches_ch_prefix() {
        assert!(is_chapter_dir("ch00"));
        assert!(is_chapter_dir("ch7"));
        assert!(is_chapter_dir("ch00-logic-and-proof"));
        assert!(!is_chapter_dir("chapters"));
        assert!(!is_chapter_dir("unit-01"));
        assert!(!is_chapter_dir("ch"));
    }

    #[test]
    fn load_question_bank_missing_dir_is_empty() {
        let pool = load_question_bank("/no/such/dir/hopefully").unwrap();
        assert!(pool.is_empty());
    }
}
