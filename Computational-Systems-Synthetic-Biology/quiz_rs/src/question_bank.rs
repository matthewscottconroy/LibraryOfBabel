use crate::config::questions_path;
use crate::models::Question;

pub fn load_questions() -> Vec<Question> {
    let path = questions_path();
    if !path.exists() {
        eprintln!("  Warning: questions.json not found at {}", path.display());
        return Vec::new();
    }
    let data = std::fs::read_to_string(&path).expect("read questions.json");
    serde_json::from_str::<Vec<Question>>(&data).expect("parse questions.json")
}
