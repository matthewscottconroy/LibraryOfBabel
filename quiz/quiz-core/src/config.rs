//! Loads `subject.toml` describing a quiz subject.
//!
//! A **subject directory** is any directory that contains a `subject.toml`.
//! Its question bank lives at `<subject_dir>/questions` and its chapter
//! source material lives at `<subject_dir>/<chapters_dir>`.
//!
//! Expected `subject.toml` format:
//! ```toml
//! title = "Subject Adaptive Quiz"
//! model = "claude-sonnet-5"          # optional; used for AI generation
//! system_prompt = "You are an expert..."
//! chapters_dir = "./chapters"
//!
//! [[chapters]]
//! index = 0
//! phase = 0
//! name  = "Chapter Name"
//! file  = "ch00-filename.md"
//!
//! [[phases]]
//! index = 0
//! name  = "Phase 0 — Description"
//! ```
//!
//! ## Subject resolution
//! The active subject directory is chosen, in priority order:
//! 1. an explicit path (e.g. the CLI `--subject <dir>` flag),
//! 2. the `QUIZ_SUBJECT` environment variable,
//! 3. the current working directory.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

// ── Raw TOML shapes ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
struct RawConfig {
    #[serde(default = "default_title")]
    pub title: String,
    /// Optional model id for AI question generation.
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub system_prompt: String,
    #[serde(default = "default_chapters_dir")]
    pub chapters_dir: String,
    #[serde(default)]
    pub chapters: Vec<RawChapter>,
    #[serde(default)]
    pub phases: Vec<RawPhase>,
}

#[derive(Debug, Clone, Deserialize)]
struct RawChapter {
    pub index: u32,
    pub phase: u32,
    pub name: String,
    pub file: String,
}

#[derive(Debug, Clone, Deserialize)]
struct RawPhase {
    pub index: u32,
    pub name: String,
}

fn default_chapters_dir() -> String {
    "chapters".to_string()
}

fn default_title() -> String {
    "Adaptive Quiz".to_string()
}

// ── Public types ──────────────────────────────────────────────────────────────

/// Metadata for a single chapter.
#[derive(Debug, Clone)]
pub struct ChapterMeta {
    pub index: u32,
    pub phase: u32,
    pub name: String,
    pub file: String,
}

/// The parsed subject configuration.
#[derive(Debug, Clone)]
pub struct SubjectConfig {
    pub title: String,
    /// Optional model id for AI generation (from the `model` key).
    pub model: Option<String>,
    pub system_prompt: String,
    /// The subject directory (the one containing `subject.toml`).
    pub root: PathBuf,
    /// Absolute path to the directory containing chapter markdown files.
    pub chapters_dir: PathBuf,
    /// Absolute path to the question bank (`<root>/questions`).
    pub questions_dir: PathBuf,
    /// Chapter index → metadata.
    pub chapters: HashMap<u32, ChapterMeta>,
    /// Phase index → display name.
    pub phases: HashMap<u32, String>,
}

impl SubjectConfig {
    /// Load from the `subject.toml` in `subject_dir`.
    ///
    /// `subject_dir` is the directory that contains `subject.toml`. All
    /// relative paths inside the config are resolved relative to it.
    pub fn load<P: AsRef<Path>>(subject_dir: P) -> Result<Self, Box<dyn std::error::Error>> {
        let root = subject_dir.as_ref().to_path_buf();
        let toml_path = root.join("subject.toml");
        let raw_text = std::fs::read_to_string(&toml_path)
            .map_err(|e| format!("Cannot read '{}': {}", toml_path.display(), e))?;
        let raw: RawConfig = toml::from_str(&raw_text)
            .map_err(|e| format!("Invalid TOML in '{}': {}", toml_path.display(), e))?;

        let chapters_dir = if Path::new(&raw.chapters_dir).is_absolute() {
            PathBuf::from(&raw.chapters_dir)
        } else {
            root.join(&raw.chapters_dir)
        };
        let questions_dir = root.join("questions");

        let chapters = raw
            .chapters
            .into_iter()
            .map(|c| {
                (
                    c.index,
                    ChapterMeta {
                        index: c.index,
                        phase: c.phase,
                        name: c.name,
                        file: c.file,
                    },
                )
            })
            .collect();

        let phases = raw
            .phases
            .into_iter()
            .map(|p| (p.index, p.name))
            .collect();

        Ok(SubjectConfig {
            title: raw.title,
            model: raw.model.filter(|m| !m.trim().is_empty()),
            system_prompt: raw.system_prompt,
            root,
            chapters_dir,
            questions_dir,
            chapters,
            phases,
        })
    }

    /// Resolve the active subject directory and load its config.
    ///
    /// Resolution priority: explicit `path` → `QUIZ_SUBJECT` env →
    /// current working directory.
    pub fn resolve(path: Option<PathBuf>) -> Result<Self, Box<dyn std::error::Error>> {
        let dir = resolve_subject_dir(path)?;
        Self::load(dir)
    }

    /// A filesystem-safe slug derived from the title (lowercase, hyphenated),
    /// used to namespace per-subject data (e.g. profile storage).
    pub fn slug(&self) -> String {
        title_slug(&self.title)
    }

    /// Return chapter metadata for the given index, if it exists.
    pub fn chapter(&self, index: u32) -> Option<&ChapterMeta> {
        self.chapters.get(&index)
    }

    /// Return the display name for a phase, falling back to `"Phase {n}"`.
    pub fn phase_name(&self, index: u32) -> String {
        self.phases
            .get(&index)
            .cloned()
            .unwrap_or_else(|| format!("Phase {index}"))
    }

    /// Return the display name for a chapter, falling back to `"Chapter {n}"`.
    pub fn chapter_name(&self, index: u32) -> String {
        self.chapters
            .get(&index)
            .map(|m| m.name.clone())
            .unwrap_or_else(|| format!("Chapter {index}"))
    }

    /// Absolute path to a chapter's markdown file.
    pub fn chapter_path(&self, index: u32) -> Option<PathBuf> {
        self.chapters
            .get(&index)
            .map(|m| self.chapters_dir.join(&m.file))
    }
}

// ── Subject resolution ────────────────────────────────────────────────────────

/// Resolve the active subject directory.
///
/// Priority: explicit `path` → `QUIZ_SUBJECT` env var → current directory.
pub fn resolve_subject_dir(
    path: Option<PathBuf>,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(p) = path {
        return Ok(p);
    }
    if let Ok(env) = std::env::var("QUIZ_SUBJECT") {
        if !env.trim().is_empty() {
            return Ok(PathBuf::from(env));
        }
    }
    Ok(std::env::current_dir()?)
}

/// Lowercase, hyphenated slug of a title (used for per-subject data dirs).
pub fn title_slug(title: &str) -> String {
    let mut slug = String::new();
    let mut prev_dash = false;
    for c in title.chars() {
        if c.is_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash && !slug.is_empty() {
            slug.push('-');
            prev_dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        "quiz".to_string()
    } else {
        slug
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_is_lowercase_hyphenated() {
        assert_eq!(title_slug("Reservoir Computing Adaptive Quiz"), "reservoir-computing-adaptive-quiz");
        assert_eq!(title_slug("HoTT Adaptive Quiz"), "hott-adaptive-quiz");
        assert_eq!(title_slug("  Weird!!  Name  "), "weird-name");
        assert_eq!(title_slug(""), "quiz");
    }

    #[test]
    fn resolve_prefers_explicit_over_env() {
        std::env::set_var("QUIZ_SUBJECT", "/from/env");
        let got = resolve_subject_dir(Some(PathBuf::from("/explicit"))).unwrap();
        assert_eq!(got, PathBuf::from("/explicit"));
        std::env::remove_var("QUIZ_SUBJECT");
    }
}
