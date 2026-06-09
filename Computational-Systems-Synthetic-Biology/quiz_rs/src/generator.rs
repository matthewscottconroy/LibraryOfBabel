use std::fs;
use std::thread;
use std::time::Duration;

use serde_json::Value;
use uuid::Uuid;

use crate::config::{
    cache_dir, chapters_dir, chapter_meta, CACHE_CAP_PER_DIFFICULTY, CACHE_MAX_AGE_DAYS,
    CHAPTER_EXCERPT_CHARS, CLAUDE_MODEL, DIFFICULTY_LEVELS,
};
use crate::models::{Answer, Question};

// ── System prompt ─────────────────────────────────────────────────────────────

const SYSTEM_PROMPT: &str = "\
You are an expert quiz question generator for a graduate-level course on \
Computational Systems & Synthetic Biology (CSSB). Topics span mathematical \
modelling (ODEs, bifurcation theory, stochastic simulation), bioinformatics \
(sequence analysis, genomics, transcriptomics, structural biology, phylogenetics), \
systems biology (metabolic modelling via FBA, gene regulatory networks, signalling), \
synthetic biology (genetic parts and circuits, CRISPR-Cas9/base editing/prime editing, \
directed evolution, cell-free systems, biosafety), and computational tools \
(scientific computing RK4/Monte Carlo, machine learning CNNs/VAEs/AlphaFold2, \
molecular dynamics force fields/REMD/FEP, network analysis). \
Your questions test genuine scientific understanding — not surface memorisation. \
Every wrong answer (distractor) must be plausible to a student who has a \
partial understanding of the topic.";

// ── Kind selection ────────────────────────────────────────────────────────────

fn pick_kind(difficulty: &str, rng: &mut impl rand::Rng) -> &'static str {
    // (kind, weight) by difficulty
    let weights: &[(&str, f64)] = match difficulty {
        "beginner"     => &[("mc", 0.55), ("tf", 0.45), ("blank", 0.00)],
        "intermediate" => &[("mc", 0.65), ("tf", 0.10), ("blank", 0.25)],
        "advanced"     => &[("mc", 0.50), ("tf", 0.00), ("blank", 0.50)],
        _              => &[("mc", 1.00), ("tf", 0.00), ("blank", 0.00)],
    };
    let total: f64 = weights.iter().map(|(_, w)| w).sum();
    let r = rng.gen::<f64>() * total;
    let mut cumul = 0.0;
    for &(kind, w) in weights {
        cumul += w;
        if r <= cumul { return kind; }
    }
    "mc"
}

// ── Cache helpers ─────────────────────────────────────────────────────────────

fn cache_path(chapter: u32, difficulty: &str) -> std::path::PathBuf {
    cache_dir().join(format!("ch{:02}_{}.json", chapter, difficulty))
}

fn load_cache(chapter: u32, difficulty: &str) -> Vec<Value> {
    let p = cache_path(chapter, difficulty);
    if !p.exists() { return Vec::new(); }
    fs::read_to_string(&p).ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_cache(chapter: u32, difficulty: &str, items: &[Value]) {
    let p = cache_path(chapter, difficulty);
    if let Some(parent) = p.parent() { let _ = fs::create_dir_all(parent); }
    let _ = fs::write(&p, serde_json::to_string_pretty(items).unwrap_or_default());
}

fn is_stale(item: &Value) -> bool {
    let Some(ts) = item["_cached_at"].as_str() else { return false; };
    let Ok(cached) = chrono::DateTime::parse_from_rfc3339(ts) else { return false; };
    let age = chrono::Utc::now().signed_duration_since(cached.with_timezone(&chrono::Utc));
    age.num_days() > CACHE_MAX_AGE_DAYS
}

fn add_to_cache(chapter: u32, difficulty: &str, mut item: Value) {
    let mut items = load_cache(chapter, difficulty);
    items.retain(|i| !is_stale(i));
    if items.len() < CACHE_CAP_PER_DIFFICULTY {
        item["_cached_at"] = Value::String(chrono::Utc::now().to_rfc3339());
        items.push(item);
        save_cache(chapter, difficulty, &items);
    }
}

fn pop_from_cache(chapter: u32, difficulty: &str) -> Option<Value> {
    let mut items = load_cache(chapter, difficulty);
    while items.first().map(|i| is_stale(i)).unwrap_or(false) {
        items.remove(0);
    }
    if items.is_empty() {
        save_cache(chapter, difficulty, &[]);
        return None;
    }
    let item = items.remove(0);
    save_cache(chapter, difficulty, &items);
    Some(item)
}

// ── Chapter excerpt ───────────────────────────────────────────────────────────

fn load_chapter_excerpt(chapter: u32) -> String {
    let meta = chapter_meta();
    let Some(m) = meta.get(&chapter) else { return String::new(); };
    let path = chapters_dir().join(m.file);
    if !path.exists() { return format!("Chapter: {}", m.name); }
    let text = fs::read_to_string(&path).unwrap_or_default();
    // strip code blocks roughly
    let mut out = String::new();
    let mut in_block = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_block = !in_block;
            if in_block { out.push_str("[code block omitted]\n"); }
        } else if !in_block {
            out.push_str(line);
            out.push('\n');
        }
    }
    out.chars().take(CHAPTER_EXCERPT_CHARS).collect()
}

// ── Prompt builders ───────────────────────────────────────────────────────────

fn build_prompt(chapter: u32, name: &str, kind: &str, difficulty: &str,
                mastery_pct: u32, weak_topics: &[String], excerpt: &str) -> String {
    let kind_label = match kind { "tf" => "true/false", "blank" => "fill-in-the-blank", _ => "multiple-choice" };
    let weak_str = if weak_topics.is_empty() { "none identified yet".to_string() } else { weak_topics.join(", ") };

    let schema = match kind {
        "tf" => r#"Respond with ONLY valid JSON:
{
  "text": "a clear statement that is definitively true or false",
  "answer": true,
  "explanation": "explanation here (≥2 sentences)",
  "tags": ["tag1", "tag2"]
}"#,
        "blank" => r#"Respond with ONLY valid JSON:
{
  "text": "sentence with ___ where the key term belongs",
  "acceptable": ["primary answer", "accepted synonym"],
  "explanation": "explanation here (≥2 sentences)",
  "tags": ["tag1", "tag2"]
}"#,
        _ => r#"Respond with ONLY valid JSON:
{
  "text": "question text here",
  "choices": ["choice A", "choice B", "choice C", "choice D"],
  "answer": 0,
  "explanation": "explanation here (≥2 sentences)",
  "tags": ["tag1", "tag2"]
}"#,
    };

    let requirements = match kind {
        "mc" => format!("- Exactly 4 answer choices (A–D).\n- Exactly one correct answer (0-indexed).\n- Distractors must represent common misconceptions.\n{}", schema),
        "tf" => format!("- The statement must be clearly true or false.\n- Avoid trivially obvious statements.\n{}", schema),
        _    => format!("- Use ___ for exactly one blank.\n- The blank should be a single key term or short phrase.\n- List 1–3 acceptable answers.\n{}", schema),
    };

    format!(
        "Generate one {} {} question for Chapter {}: {}.\n\n\
         Student mastery level: {}% ({}).\n\
         Recent weak areas: {}.\n\n\
         Use this chapter excerpt as context (do not quote it verbatim):\n\
         ---\n{}\n---\n\n\
         Requirements:\n{}\n\n\
         Difficulty \"{}\" means:\n\
         - beginner: recall of key definitions or straightforward application\n\
         - intermediate: deeper understanding, knowing *why*, cross-connections\n\
         - advanced: subtle distinctions, open problems, or research-level nuance",
        difficulty, kind_label, chapter, name,
        mastery_pct, difficulty,
        weak_str,
        excerpt,
        requirements,
        difficulty,
    )
}

// ── Response parsers ──────────────────────────────────────────────────────────

fn strip_fences(raw: &str) -> String {
    let s = raw.trim();
    let s = if s.starts_with("```json") || s.starts_with("```") {
        s.trim_start_matches("```json").trim_start_matches("```").trim_start()
    } else { s };
    let s = if s.ends_with("```") { &s[..s.len()-3] } else { s };
    s.trim().to_string()
}

fn parse_response(raw: &str, kind: &str, chapter: u32, difficulty: &str) -> Option<Question> {
    let cleaned = strip_fences(raw);
    let data: Value = serde_json::from_str(&cleaned).ok()?;
    let meta = chapter_meta();
    let phase = meta.get(&chapter)?.phase;

    let q = match kind {
        "mc" => Question {
            question_id: Uuid::new_v4().to_string(),
            chapter, phase,
            kind: "mc".into(),
            text: data["text"].as_str()?.trim().to_string(),
            choices: data["choices"].as_array()?
                .iter().map(|c| c.as_str().unwrap_or("").to_string()).collect(),
            answer: Answer::Index(data["answer"].as_u64()? as usize),
            explanation: data["explanation"].as_str()?.trim().to_string(),
            tags: data["tags"].as_array().map(|a| a.iter().map(|t| t.as_str().unwrap_or("").to_string()).collect()).unwrap_or_default(),
            difficulty: difficulty.to_string(),
            generated: true,
        },
        "tf" => {
            let raw_ans = &data["answer"];
            let answer_idx = if raw_ans.is_boolean() {
                if raw_ans.as_bool().unwrap() { 0 } else { 1 }
            } else {
                let s = raw_ans.as_str().unwrap_or("").to_lowercase();
                if s == "true" || s == "1" || s == "yes" { 0 } else { 1 }
            };
            Question {
                question_id: Uuid::new_v4().to_string(),
                chapter, phase,
                kind: "tf".into(),
                text: data["text"].as_str()?.trim().to_string(),
                choices: vec!["True".into(), "False".into()],
                answer: Answer::Index(answer_idx),
                explanation: data["explanation"].as_str()?.trim().to_string(),
                tags: data["tags"].as_array().map(|a| a.iter().map(|t| t.as_str().unwrap_or("").to_string()).collect()).unwrap_or_default(),
                difficulty: difficulty.to_string(),
                generated: true,
            }
        }
        "blank" => {
            let synonyms: Vec<String> = data["acceptable"].as_array()?
                .iter().map(|s| s.as_str().unwrap_or("").to_string()).collect();
            if synonyms.is_empty() { return None; }
            let primary = synonyms[0].to_lowercase();
            Question {
                question_id: Uuid::new_v4().to_string(),
                chapter, phase,
                kind: "blank".into(),
                text: data["text"].as_str()?.trim().to_string(),
                choices: synonyms,
                answer: Answer::Text(primary),
                explanation: data["explanation"].as_str()?.trim().to_string(),
                tags: data["tags"].as_array().map(|a| a.iter().map(|t| t.as_str().unwrap_or("").to_string()).collect()).unwrap_or_default(),
                difficulty: difficulty.to_string(),
                generated: true,
            }
        }
        _ => return None,
    };

    if q.validate().is_empty() { Some(q) } else { None }
}

// ── Generator ─────────────────────────────────────────────────────────────────

pub struct ClaudeGenerator {
    api_key: String,
}

impl ClaudeGenerator {
    pub fn new() -> Self {
        let api_key = std::env::var("ANTHROPIC_API_KEY").unwrap_or_default();
        Self { api_key }
    }

    pub fn available(&self) -> bool {
        !self.api_key.is_empty()
    }

    pub fn get_question(
        &self,
        chapter: u32,
        difficulty: &str,
        mastery_pct: u32,
        weak_topics: &[String],
    ) -> Option<Question> {
        if let Some(cached) = pop_from_cache(chapter, difficulty) {
            // Remove cache timestamp before deserializing
            let mut item = cached.clone();
            if let Some(obj) = item.as_object_mut() { obj.remove("_cached_at"); }
            if let Ok(mut q) = serde_json::from_value::<Question>(item) {
                q.question_id = Uuid::new_v4().to_string();
                return Some(q);
            }
        }
        if !self.available() { return None; }
        self.generate(chapter, difficulty, mastery_pct, weak_topics, 2)
    }

    fn generate(
        &self,
        chapter: u32,
        difficulty: &str,
        mastery_pct: u32,
        weak_topics: &[String],
        retries: u32,
    ) -> Option<Question> {
        let meta = chapter_meta();
        let name = meta.get(&chapter).map(|m| m.name).unwrap_or("Unknown");
        let excerpt = load_chapter_excerpt(chapter);
        let mut rng = rand::thread_rng();
        let kind = pick_kind(difficulty, &mut rng);
        let prompt = build_prompt(chapter, name, kind, difficulty, mastery_pct, weak_topics, &excerpt);

        for attempt in 0..retries {
            match self.call_api(&prompt) {
                Ok(raw) => {
                    if let Some(q) = parse_response(&raw, kind, chapter, difficulty) {
                        // kick off background cache fill
                        let key = self.api_key.clone();
                        let ch = chapter;
                        let diff = difficulty.to_string();
                        let mp = mastery_pct;
                        let wt = weak_topics.to_vec();
                        thread::spawn(move || {
                            let gen = ClaudeGenerator { api_key: key };
                            gen.prefill_cache(ch, &diff, mp, &wt, 2);
                        });
                        return Some(q);
                    }
                }
                Err(_) => {
                    if attempt < retries - 1 {
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        }
        None
    }

    fn call_api(&self, prompt: &str) -> Result<String, String> {
        let body = serde_json::json!({
            "model": CLAUDE_MODEL,
            "max_tokens": 700,
            "system": SYSTEM_PROMPT,
            "messages": [{"role": "user", "content": prompt}],
        });

        let client = reqwest::blocking::Client::new();
        let resp = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("API error {}", resp.status()));
        }

        let data: Value = resp.json().map_err(|e| e.to_string())?;
        data["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "no text in response".to_string())
    }

    fn prefill_cache(
        &self,
        chapter: u32,
        difficulty: &str,
        mastery_pct: u32,
        weak_topics: &[String],
        n: usize,
    ) {
        let existing = load_cache(chapter, difficulty);
        let remaining = CACHE_CAP_PER_DIFFICULTY.saturating_sub(existing.len());
        for _ in 0..n.min(remaining) {
            if let Some(q) = self.generate(chapter, difficulty, mastery_pct, weak_topics, 1) {
                add_to_cache(chapter, difficulty, serde_json::to_value(&q).unwrap_or_default());
            }
        }
    }

    pub fn prefill_for_chapter(
        &self,
        chapter: u32,
        mastery_pct: u32,
        weak_topics: &[String],
    ) {
        if !self.available() { return; }
        for &diff in DIFFICULTY_LEVELS {
            let existing = load_cache(chapter, diff);
            let needed = 5usize.saturating_sub(existing.len());
            for _ in 0..needed {
                if let Some(q) = self.generate(chapter, diff, mastery_pct, weak_topics, 1) {
                    add_to_cache(chapter, diff, serde_json::to_value(&q).unwrap_or_default());
                }
                thread::sleep(Duration::from_millis(200));
            }
        }
    }
}
