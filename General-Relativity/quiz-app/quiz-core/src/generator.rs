//! AI question generation via the Anthropic Messages API.
//!
//! `ClaudeGenerator` uses a blocking `reqwest` client to call
//! `POST https://api.anthropic.com/v1/messages` and parses the response JSON
//! into a [`Question`].
//!
//! Generated questions are cached to
//! `{cache_dir}/ch{N:02}_{difficulty}.json` so subsequent calls can pop from
//! the cache before hitting the network.  The cache cap per (chapter,
//! difficulty) pair is configurable.
//!
//! If `ANTHROPIC_API_KEY` is absent from the environment, all generation
//! attempts gracefully return `None`.

use std::path::PathBuf;
use std::time::Duration;


use uuid::Uuid;

use crate::question::{Difficulty, Question};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Maximum number of generated questions cached per (chapter, difficulty) pair.
pub const CACHE_CAP_PER_DIFFICULTY: usize = 20;

/// Discard cached entries older than this many days.
pub const CACHE_MAX_AGE_DAYS: u64 = 90;

/// Maximum characters of chapter text sent to Claude in a single prompt.
pub const CHAPTER_EXCERPT_CHARS: usize = 4_000;

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const API_MODEL: &str = "claude-sonnet-4-6";
const ANTHROPIC_VERSION: &str = "2023-06-01";

// ── ClaudeGenerator ───────────────────────────────────────────────────────────

/// Generates quiz questions via the Anthropic API, with file-based caching.
pub struct ClaudeGenerator {
    api_key: Option<String>,
    cache_dir: PathBuf,
    system_prompt: String,
    client: reqwest::blocking::Client,
}

impl ClaudeGenerator {
    /// Create a new generator.
    ///
    /// - `api_key`: pass `None` to read from `ANTHROPIC_API_KEY` env var.
    /// - `cache_dir`: where to write `.json` cache files.
    /// - `system_prompt`: the subject-specific system prompt sent to the API.
    pub fn new(
        api_key: Option<String>,
        cache_dir: impl Into<PathBuf>,
        system_prompt: impl Into<String>,
    ) -> Self {
        let key = api_key
            .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
            .filter(|k| !k.trim().is_empty());

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .expect("failed to build reqwest client");

        Self {
            api_key: key,
            cache_dir: cache_dir.into(),
            system_prompt: system_prompt.into(),
            client,
        }
    }

    /// True if an API key is available (generation will be attempted).
    pub fn available(&self) -> bool {
        self.api_key.is_some()
    }

    // ── Public interface ──────────────────────────────────────────────────────

    /// Return a `Question` for the given chapter and difficulty.
    ///
    /// Order of preference:
    /// 1. Pop a fresh question from the on-disk cache.
    /// 2. Generate a question live via the Anthropic API.
    /// 3. Return `None` (no key or API failure).
    pub fn get_question(
        &self,
        chapter: u32,
        chapter_name: &str,
        chapter_phase: u32,
        difficulty: &Difficulty,
        mastery_pct: u32,
        weak_topics: &[String],
        chapter_excerpt: &str,
    ) -> Option<Question> {
        if let Some(q) = self.pop_from_cache(chapter, difficulty) {
            return Some(q.with_fresh_id());
        }
        if !self.available() {
            return None;
        }
        self.generate(
            chapter,
            chapter_name,
            chapter_phase,
            difficulty,
            mastery_pct,
            weak_topics,
            chapter_excerpt,
            2, // retries
        )
    }

    // ── Cache helpers ─────────────────────────────────────────────────────────

    fn cache_path(&self, chapter: u32, difficulty: &Difficulty) -> PathBuf {
        self.cache_dir
            .join(format!("ch{:02}_{}.json", chapter, difficulty.as_str()))
    }

    fn load_cache(&self, chapter: u32, difficulty: &Difficulty) -> Vec<serde_json::Value> {
        let path = self.cache_path(chapter, difficulty);
        if !path.exists() {
            return Vec::new();
        }
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str::<Vec<serde_json::Value>>(&s).ok())
            .unwrap_or_default()
    }

    fn save_cache(
        &self,
        chapter: u32,
        difficulty: &Difficulty,
        items: &[serde_json::Value],
    ) {
        let path = self.cache_path(chapter, difficulty);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(items) {
            let _ = std::fs::write(&path, json);
        }
    }

    fn is_stale(item: &serde_json::Value) -> bool {
        let ts = match item.get("_cached_at").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return false,
        };
        // Parse ISO 8601 timestamp and compare age
        if let Ok(t) = chrono::DateTime::parse_from_rfc3339(ts) {
            let age = chrono::Utc::now().signed_duration_since(t.with_timezone(&chrono::Utc));
            age.num_days() > CACHE_MAX_AGE_DAYS as i64
        } else {
            false
        }
    }

    /// Pop (remove) the first non-stale item from the cache and return it as a
    /// `Question`.
    fn pop_from_cache(&self, chapter: u32, difficulty: &Difficulty) -> Option<Question> {
        let mut items = self.load_cache(chapter, difficulty);
        // Drop stale entries from the front
        while items.first().map_or(false, |i| Self::is_stale(i)) {
            items.remove(0);
        }
        if items.is_empty() {
            self.save_cache(chapter, difficulty, &[]);
            return None;
        }
        let raw = items.remove(0);
        self.save_cache(chapter, difficulty, &items);
        // Remove the _cached_at key before deserialising
        let mut q_val = raw;
        if let serde_json::Value::Object(ref mut map) = q_val {
            map.remove("_cached_at");
        }
        serde_json::from_value(q_val).ok()
    }

    /// Append a question to the cache (respecting the cap).
    pub fn add_to_cache(&self, chapter: u32, difficulty: &Difficulty, q: &Question) {
        let mut items = self.load_cache(chapter, difficulty);
        // Purge stale while we have the file
        items.retain(|i| !Self::is_stale(i));
        if items.len() >= CACHE_CAP_PER_DIFFICULTY {
            return;
        }
        if let Ok(mut val) = serde_json::to_value(q) {
            val["_cached_at"] = serde_json::Value::String(chrono::Utc::now().to_rfc3339());
            items.push(val);
            self.save_cache(chapter, difficulty, &items);
        }
    }

    // ── API call ──────────────────────────────────────────────────────────────

    fn generate(
        &self,
        chapter: u32,
        chapter_name: &str,
        chapter_phase: u32,
        difficulty: &Difficulty,
        mastery_pct: u32,
        weak_topics: &[String],
        chapter_excerpt: &str,
        retries: u32,
    ) -> Option<Question> {
        let api_key = self.api_key.as_deref()?;
        let kind = pick_kind(difficulty);
        let prompt = build_prompt(
            chapter,
            chapter_name,
            &kind,
            difficulty,
            mastery_pct,
            weak_topics,
            chapter_excerpt,
        );

        for attempt in 0..retries {
            let body = serde_json::json!({
                "model": API_MODEL,
                "max_tokens": 700,
                "system": self.system_prompt,
                "messages": [{ "role": "user", "content": prompt }]
            });

            let result = self
                .client
                .post(API_URL)
                .header("x-api-key", api_key)
                .header("anthropic-version", ANTHROPIC_VERSION)
                .header("content-type", "application/json")
                .json(&body)
                .send();

            match result {
                Ok(resp) if resp.status().is_success() => {
                    if let Ok(json) = resp.json::<serde_json::Value>() {
                        let raw_text = json
                            .get("content")
                            .and_then(|c| c.get(0))
                            .and_then(|m| m.get("text"))
                            .and_then(|t| t.as_str())
                            .unwrap_or("");
                        if let Some(q) = parse_response(
                            raw_text,
                            &kind,
                            chapter,
                            chapter_phase,
                            difficulty,
                        ) {
                            return Some(q);
                        }
                    }
                }
                Ok(_) | Err(_) => {
                    if attempt + 1 < retries {
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        }
        None
    }
}

// ── Kind selection ────────────────────────────────────────────────────────────

/// Question kind probabilities per difficulty level.
fn pick_kind(difficulty: &Difficulty) -> String {
    let weights: &[(&str, f64)] = match difficulty {
        Difficulty::Beginner => &[("mc", 0.55), ("tf", 0.45)],
        Difficulty::Intermediate => &[("mc", 0.50), ("tf", 0.10), ("blank", 0.20), ("proof", 0.20)],
        Difficulty::Advanced => &[("mc", 0.30), ("blank", 0.35), ("proof", 0.35)],
    };

    let total: f64 = weights.iter().map(|(_, w)| w).sum();
    let r: f64 = rand::random::<f64>() * total;
    let mut cumul = 0.0;
    for &(kind, w) in weights {
        cumul += w;
        if r <= cumul {
            return kind.to_string();
        }
    }
    weights.last().unwrap().0.to_string()
}

// ── Prompt builders ───────────────────────────────────────────────────────────

const MC_SCHEMA: &str = r#"Respond with ONLY valid JSON (no markdown, no prose outside the JSON):
{
  "text": "question text here (no leading letter/number)",
  "choices": ["choice A", "choice B", "choice C", "choice D"],
  "answer": 0,
  "explanation": "explanation here (≥2 sentences explaining WHY the answer is correct)",
  "tags": ["tag1", "tag2"]
}"#;

const TF_SCHEMA: &str = r#"Respond with ONLY valid JSON (no markdown, no prose outside the JSON):
{
  "text": "a clear statement that is definitively true or false",
  "answer": true,
  "explanation": "explanation here (≥2 sentences explaining WHY the statement is true or false)",
  "tags": ["tag1", "tag2"]
}"#;

const BLANK_SCHEMA: &str = r#"Respond with ONLY valid JSON (no markdown, no prose outside the JSON):
{
  "text": "sentence with ___ where the key term belongs",
  "acceptable": ["primary answer", "accepted synonym"],
  "explanation": "explanation here (≥2 sentences explaining the term)",
  "tags": ["tag1", "tag2"]
}"#;

const PROOF_SCHEMA: &str = r#"Respond with ONLY valid JSON (no markdown, no prose outside the JSON):
{
  "theorem": "the proposition to be proved (one sentence)",
  "proof_lines": [
    "Step 1: prose line, possibly containing ___ for a key term or short phrase.",
    "Step 2: another line, possibly with ___.",
    "Step 3: another line."
  ],
  "fills": ["fill for blank 1", "fill for blank 2"],
  "explanation": "the complete proof written out in full (≥3 sentences)",
  "tags": ["tag1", "tag2"]
}
Notes:
- Use ___ (three underscores) for exactly 2–3 blanks spread across the proof_lines.
- fills must list the canonical answer for each blank in order of appearance.
- The proof should be logically complete; the blanked steps are the pivotal ones."#;

const DIFF_GUIDANCE: &str = "- beginner: recall of key definitions or straightforward application\n\
- intermediate: deeper understanding, knowing *why*, cross-connections\n\
- advanced: subtle distinctions, open problems, or research-level nuance";

fn kind_requirements(kind: &str) -> String {
    match kind {
        "mc" => format!(
            "- Exactly 4 answer choices (A through D).\n\
             - Exactly one correct answer.\n\
             - Distractors must represent common misconceptions, not obvious wrong answers.\n{}",
            MC_SCHEMA
        ),
        "tf" => format!(
            "- The statement must be clearly true or false with no ambiguity.\n\
             - Avoid trivially obvious statements; test conceptual understanding.\n{}",
            TF_SCHEMA
        ),
        "blank" => format!(
            "- Use ___ (three underscores) for exactly one blank.\n\
             - The blank should be a single key term, symbol, or short phrase.\n\
             - List 1–3 acceptable answers (primary term + common synonyms/abbreviations).\n{}",
            BLANK_SCHEMA
        ),
        "proof" => format!(
            "- Present a partial proof with 2–3 key steps blanked out.\n\
             - The blanked steps must be the pivotal logical moves, not trivial transitions.\n\
             - The student fills in each blank in order.\n{}",
            PROOF_SCHEMA
        ),
        _ => MC_SCHEMA.to_string(),
    }
}

fn build_prompt(
    chapter: u32,
    chapter_name: &str,
    kind: &str,
    difficulty: &Difficulty,
    mastery_pct: u32,
    weak_topics: &[String],
    chapter_excerpt: &str,
) -> String {
    let kind_label = match kind {
        "mc" => "multiple-choice",
        "tf" => "true/false",
        "blank" => "fill-in-the-blank",
        "proof" => "proof-scaffold",
        _ => kind,
    };
    let weak_str = if weak_topics.is_empty() {
        "none identified yet".to_string()
    } else {
        weak_topics.join(", ")
    };
    format!(
        "Generate one {diff} {kind_label} question for Chapter {ch}: {name}.\n\n\
         Student mastery level: {mastery_pct}% ({diff}).\n\
         Recent weak areas: {weak_str}.\n\n\
         Use this chapter excerpt as context (do not quote it verbatim):\n\
         ---\n{excerpt}\n---\n\n\
         Requirements:\n{reqs}\n\n\
         Difficulty \"{diff}\" means:\n{diff_guidance}\n",
        diff = difficulty.as_str(),
        kind_label = kind_label,
        ch = chapter,
        name = chapter_name,
        mastery_pct = mastery_pct,
        weak_str = weak_str,
        excerpt = chapter_excerpt,
        reqs = kind_requirements(kind),
        diff_guidance = DIFF_GUIDANCE,
    )
}

// ── Response parsers ──────────────────────────────────────────────────────────

fn strip_fences(raw: &str) -> &str {
    let raw = raw.trim();
    // Remove leading ```json or ``` fences
    let raw = if raw.starts_with("```") {
        raw.splitn(2, '\n').nth(1).unwrap_or(raw)
    } else {
        raw
    };
    // Remove trailing ```
    if raw.ends_with("```") {
        raw.rsplitn(2, "```").nth(1).unwrap_or(raw).trim_end()
    } else {
        raw
    }
}

fn parse_response(
    raw: &str,
    kind: &str,
    chapter: u32,
    phase: u32,
    difficulty: &Difficulty,
) -> Option<Question> {
    let clean = strip_fences(raw);
    let data: serde_json::Value = serde_json::from_str(clean).ok()?;
    match kind {
        "mc" => parse_mc(&data, chapter, phase, difficulty),
        "tf" => parse_tf(&data, chapter, phase, difficulty),
        "blank" => parse_blank(&data, chapter, phase, difficulty),
        "proof" => parse_proof(&data, chapter, phase, difficulty),
        _ => None,
    }
}

fn str_val(v: &serde_json::Value, key: &str) -> Option<String> {
    v.get(key)?.as_str().map(|s| s.trim().to_string())
}

fn tags(v: &serde_json::Value) -> Vec<String> {
    v.get("tags")
        .and_then(|t| t.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default()
}

fn parse_mc(
    data: &serde_json::Value,
    chapter: u32,
    phase: u32,
    difficulty: &Difficulty,
) -> Option<Question> {
    let text = str_val(data, "text")?;
    let choices: Vec<String> = data
        .get("choices")?
        .as_array()?
        .iter()
        .filter_map(|c| c.as_str().map(|s| s.to_string()))
        .collect();
    let answer = data.get("answer")?.as_u64()? as usize;
    let explanation = str_val(data, "explanation")?;
    let q = Question::MultipleChoice {
        question_id: Uuid::new_v4().to_string(),
        chapter,
        phase,
        text,
        choices,
        answer,
        explanation,
        tags: tags(data),
        difficulty: difficulty.clone(),
        generated: true,
    };
    if q.validate().is_empty() { Some(q) } else { None }
}

fn parse_tf(
    data: &serde_json::Value,
    chapter: u32,
    phase: u32,
    difficulty: &Difficulty,
) -> Option<Question> {
    let text = str_val(data, "text")?;
    let raw_answer = data.get("answer")?;
    let answer_idx: usize = if raw_answer.is_boolean() {
        if raw_answer.as_bool().unwrap() { 0 } else { 1 }
    } else {
        let s = raw_answer.as_str().unwrap_or("").to_lowercase();
        if matches!(s.as_str(), "true" | "1" | "yes") { 0 } else { 1 }
    };
    let explanation = str_val(data, "explanation")?;
    let q = Question::TrueFalse {
        question_id: Uuid::new_v4().to_string(),
        chapter,
        phase,
        text,
        choices: vec!["True".into(), "False".into()],
        answer: answer_idx,
        explanation,
        tags: tags(data),
        difficulty: difficulty.clone(),
        generated: true,
    };
    if q.validate().is_empty() { Some(q) } else { None }
}

fn parse_blank(
    data: &serde_json::Value,
    chapter: u32,
    phase: u32,
    difficulty: &Difficulty,
) -> Option<Question> {
    let text = str_val(data, "text")?;
    let synonyms: Vec<String> = data
        .get("acceptable")?
        .as_array()?
        .iter()
        .filter_map(|c| c.as_str().map(|s| s.to_string()))
        .collect();
    if synonyms.is_empty() {
        return None;
    }
    let primary = synonyms[0].to_lowercase();
    let explanation = str_val(data, "explanation")?;
    let q = Question::FillBlank {
        question_id: Uuid::new_v4().to_string(),
        chapter,
        phase,
        text,
        choices: synonyms,
        answer: primary,
        explanation,
        tags: tags(data),
        difficulty: difficulty.clone(),
        generated: true,
    };
    if q.validate().is_empty() { Some(q) } else { None }
}

fn parse_proof(
    data: &serde_json::Value,
    chapter: u32,
    phase: u32,
    difficulty: &Difficulty,
) -> Option<Question> {
    let theorem = str_val(data, "theorem")?;
    let proof_lines: Vec<String> = data
        .get("proof_lines")?
        .as_array()?
        .iter()
        .filter_map(|l| l.as_str().map(|s| s.to_string()))
        .collect();
    let fills: Vec<String> = data
        .get("fills")?
        .as_array()?
        .iter()
        .filter_map(|f| f.as_str().map(|s| s.trim().to_string()))
        .collect();
    let explanation = str_val(data, "explanation")?;
    if theorem.is_empty() || proof_lines.is_empty() || fills.is_empty() {
        return None;
    }
    let q = Question::Proof {
        question_id: Uuid::new_v4().to_string(),
        chapter,
        phase,
        text: theorem,
        choices: proof_lines,
        answer: fills.join("|"),
        explanation,
        tags: tags(data),
        difficulty: difficulty.clone(),
        generated: true,
    };
    if q.validate().is_empty() { Some(q) } else { None }
}
