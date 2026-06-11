//! Loads `subject.toml` from the workspace root.
//!
//! Expected format:
//! ```toml
//! title = "Subject Adaptive Quiz"
//! system_prompt = "You are an expert..."
//! chapters_dir = "chapters"
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

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

// ── Raw TOML shapes ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
struct RawConfig {
    pub title: String,
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
    pub system_prompt: String,
    /// Absolute path to the directory containing chapter markdown files.
    pub chapters_dir: PathBuf,
    /// Chapter index → metadata.
    pub chapters: HashMap<u32, ChapterMeta>,
    /// Phase index → display name.
    pub phases: HashMap<u32, String>,
}

impl SubjectConfig {
    /// Load from a `subject.toml` file.
    ///
    /// `workspace_root` is the directory that contains `subject.toml`.
    /// All relative paths inside the config are resolved relative to it.
    pub fn load<P: AsRef<Path>>(workspace_root: P) -> Result<Self, Box<dyn std::error::Error>> {
        let root = workspace_root.as_ref().to_path_buf();
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
            system_prompt: raw.system_prompt,
            chapters_dir,
            chapters,
            phases,
        })
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

    /// Absolute path to a chapter's markdown file.
    pub fn chapter_path(&self, index: u32) -> Option<PathBuf> {
        self.chapters
            .get(&index)
            .map(|m| self.chapters_dir.join(&m.file))
    }
}
