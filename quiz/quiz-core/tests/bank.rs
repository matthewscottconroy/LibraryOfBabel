//! Integration test for `load_question_bank` against a realistic on-disk
//! layout: `<bank>/chNN.../NNN.json`, one Question object per file.
//!
//! Fixture content in `tests/fixtures/` is copied verbatim from real seed
//! files (`Homotopy-Type-Theory/questions/ch00-logic-and-proof/`).

use std::fs;
use std::path::PathBuf;

use quiz_core::load_question_bank;

// Real seed files (single Question object each).
const SEED_001: &str = include_str!("fixtures/ch00-logic-and-proof/001.json");
const SEED_002: &str = include_str!("fixtures/ch00-logic-and-proof/002.json");
const SEED_003: &str = include_str!("fixtures/ch00-logic-and-proof/003.json");

/// A seed file with no `question_id` (must still parse — id is defaulted).
const NO_ID: &str = r#"{
  "chapter": 1, "phase": 0, "kind": "mc",
  "text": "Which is a set?", "choices": ["{1,2}", "1", "true", "->"],
  "answer": 0, "explanation": "A collection of distinct elements."
}"#;

/// A JSON array file (two questions in one file).
const ARRAY: &str = r#"[
  { "chapter": 1, "phase": 0, "kind": "tf",
    "text": "The empty set is a subset of every set.",
    "choices": ["True","False"], "answer": 0, "explanation": "Vacuously true." },
  { "chapter": 1, "phase": 0, "kind": "blank",
    "text": "The set with no elements is the ___ set.", "choices": ["empty"],
    "answer": "empty", "explanation": "Denoted ∅." }
]"#;

fn unique_tmp() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("quiz_bank_test_{nanos}"))
}

#[test]
fn loads_chapter_dirs_arrays_and_defaults_and_skips_legacy() {
    let root = unique_tmp();
    let bank = root.join("questions");

    let ch00 = bank.join("ch00-logic-and-proof");
    let ch01 = bank.join("ch01-set-theory");
    let notes = bank.join("notes"); // NOT a chapter dir — must be ignored
    fs::create_dir_all(&ch00).unwrap();
    fs::create_dir_all(&ch01).unwrap();
    fs::create_dir_all(&notes).unwrap();

    // ch00: three real single-object seed files.
    fs::write(ch00.join("001.json"), SEED_001).unwrap();
    fs::write(ch00.join("002.json"), SEED_002).unwrap();
    fs::write(ch00.join("003.json"), SEED_003).unwrap();

    // ch01: one array file (2 questions) + one file lacking question_id.
    fs::write(ch01.join("arr.json"), ARRAY).unwrap();
    fs::write(ch01.join("noid.json"), NO_ID).unwrap();

    // A legacy underscore-prefixed file must be skipped.
    fs::write(ch01.join("_legacy_sample.json"), r#"{"garbage": true}"#).unwrap();

    // A json file outside any chapter dir must be ignored entirely.
    fs::write(notes.join("scratch.json"), r#"{"garbage": true}"#).unwrap();

    let pool = load_question_bank(&bank).expect("bank should load");

    // 3 (ch00 objects) + 2 (ch01 array) + 1 (ch01 noid) = 6.
    assert_eq!(pool.len(), 6, "expected 6 parsed questions, got {}", pool.len());

    // Every question must have a non-empty id (defaulted where absent).
    assert!(pool.iter().all(|q| !q.question_id().is_empty()));

    // Chapters 0 and 1 are both represented.
    assert!(pool.iter().any(|q| q.chapter() == 0));
    assert!(pool.iter().any(|q| q.chapter() == 1));

    let _ = fs::remove_dir_all(&root);
}
