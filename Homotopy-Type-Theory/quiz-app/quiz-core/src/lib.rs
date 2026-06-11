//! `quiz-core` — adaptive quiz engine for the Library of Babel quiz apps.
//!
//! ## Module layout
//!
//! | Module       | Responsibility                                             |
//! |:-------------|:-----------------------------------------------------------|
//! | `config`     | Load `subject.toml` (chapter/phase metadata)               |
//! | `question`   | Question enum (MC, TF, FillBlank, Proof) + load helpers    |
//! | `mastery`    | SM-2 `MasteryRecord`, `confidence_to_quality`              |
//! | `profile`    | `UserProfile`, `SessionResult`                             |
//! | `adaptive`   | `Scope`, `select_questions`, mastery summary helpers       |
//! | `storage`    | Atomic JSON persistence for profiles                       |
//! | `generator`  | `ClaudeGenerator` — Anthropic API + cache                  |
//! | `session`    | `Session` — interactive session tracker                    |

pub mod adaptive;
pub mod config;
pub mod generator;
pub mod mastery;
pub mod profile;
pub mod question;
pub mod session;
pub mod storage;

// ── Convenience re-exports ────────────────────────────────────────────────────

pub use adaptive::{
    ChapterInfo, MasteryRow, Scope,
    difficulty_weight, filter_by_scope, mastery_summary,
    preferred_difficulty, select_questions, tag_weakness, weak_topics,
    user_phase_frontier,
};
pub use config::{ChapterMeta, SubjectConfig};
pub use generator::ClaudeGenerator;
pub use mastery::{MasteryRecord, confidence_to_quality, INITIAL_MASTERY};
pub use profile::{SessionResult, UserProfile, LR_CORRECT, LR_WRONG, RECENCY_WINDOW};
pub use question::{Difficulty, Question, Quiz, load_question_pool, load_questions, load_quiz};
pub use session::Session;
pub use storage::{
    ProfileSummary, create_profile, delete_profile, list_profiles,
    list_profiles_with_summary, load_or_create, load_profile, save_profile,
};
