pub mod question;
pub mod session;

pub use question::{Question, Quiz};
pub use session::Session;

use std::path::Path;

/// Load a [`Quiz`] from a JSON file.
pub fn load_quiz<P: AsRef<Path>>(path: P) -> Result<Quiz, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path.as_ref())
        .map_err(|e| format!("Cannot read '{}': {}", path.as_ref().display(), e))?;
    let quiz = serde_json::from_str(&data)
        .map_err(|e| format!("Invalid JSON in '{}': {}", path.as_ref().display(), e))?;
    Ok(quiz)
}
