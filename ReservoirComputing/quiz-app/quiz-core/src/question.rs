use serde::{Deserialize, Serialize};

/// A single quiz question. The `type` field in JSON selects the variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Question {
    MultipleChoice {
        text: String,
        choices: Vec<String>,
        /// 0-based index of the correct choice.
        correct: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        explanation: Option<String>,
    },
    TrueFalse {
        text: String,
        correct: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        explanation: Option<String>,
    },
    FreeText {
        text: String,
        /// Case-insensitive canonical answer.
        correct: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        explanation: Option<String>,
    },
}

impl Question {
    pub fn text(&self) -> &str {
        match self {
            Self::MultipleChoice { text, .. }
            | Self::TrueFalse { text, .. }
            | Self::FreeText { text, .. } => text,
        }
    }

    pub fn explanation(&self) -> Option<&str> {
        match self {
            Self::MultipleChoice { explanation, .. }
            | Self::TrueFalse { explanation, .. }
            | Self::FreeText { explanation, .. } => explanation.as_deref(),
        }
    }

    /// Check whether `answer` is correct.
    ///
    /// - MultipleChoice: accepts "A"/"B"/"C"/"D" (case-insensitive)
    /// - TrueFalse: accepts "true"/"t"/"yes"/"y" or "false"/"f"/"no"/"n"
    /// - FreeText: case-insensitive exact match
    pub fn check(&self, answer: &str) -> bool {
        let a = answer.trim();
        match self {
            Self::MultipleChoice { correct, choices, .. } => {
                let idx = match a.to_uppercase().as_str() {
                    "A" => 0,
                    "B" => 1,
                    "C" => 2,
                    "D" => 3,
                    _ => return false,
                };
                idx < choices.len() && idx == *correct
            }
            Self::TrueFalse { correct, .. } => {
                match a.to_lowercase().as_str() {
                    "true" | "t" | "yes" | "y" => *correct,
                    "false" | "f" | "no" | "n" => !*correct,
                    _ => false,
                }
            }
            Self::FreeText { correct, .. } => a.to_lowercase() == correct.to_lowercase(),
        }
    }

    /// Display-ready list of choices.
    /// MultipleChoice: the choices vec; TrueFalse: ["True", "False"]; FreeText: empty.
    pub fn display_choices(&self) -> Vec<String> {
        match self {
            Self::MultipleChoice { choices, .. } => choices.clone(),
            Self::TrueFalse { .. } => vec!["True".to_string(), "False".to_string()],
            Self::FreeText { .. } => vec![],
        }
    }

    /// Human-readable string for the correct answer (for showing after a wrong guess).
    pub fn correct_display(&self) -> String {
        match self {
            Self::MultipleChoice { correct, choices, .. } => {
                let label = (b'A' + *correct as u8) as char;
                format!("{}: {}", label, choices.get(*correct).map_or("?", |s| s.as_str()))
            }
            Self::TrueFalse { correct, .. } => {
                if *correct { "True".to_string() } else { "False".to_string() }
            }
            Self::FreeText { correct, .. } => correct.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub questions: Vec<Question>,
}
