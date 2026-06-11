/*!
Reservoir Computing Adaptive Quiz CLI
======================================
Full-featured interactive quiz application with profile management,
adaptive question selection (SM-2 spaced repetition), mastery dashboard,
session history, and progress export.
*/

use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    time::Instant,
};

use chrono::Local;
use serde::{Deserialize, Serialize};

use quiz_core::question::{load_question_pool, load_quiz, Question};

// ── ANSI colour helpers ────────────────────────────────────────────────────────

fn bold(s: &str) -> String    { format!("\x1b[1m{s}\x1b[0m") }
fn green(s: &str) -> String   { format!("\x1b[32m{s}\x1b[0m") }
fn red(s: &str) -> String     { format!("\x1b[31m{s}\x1b[0m") }
fn yellow(s: &str) -> String  { format!("\x1b[33m{s}\x1b[0m") }
fn cyan(s: &str) -> String    { format!("\x1b[36m{s}\x1b[0m") }
fn dim(s: &str) -> String     { format!("\x1b[2m{s}\x1b[0m") }
fn magenta(s: &str) -> String { format!("\x1b[35m{s}\x1b[0m") }

const WIDTH: usize = 80;

fn hr(ch: &str) {
    println!("{}", dim(&ch.repeat(WIDTH)));
}

fn section(title: &str) {
    println!("\n{}\n", bold(&cyan(title)));
}

fn print_wrap(text: &str) {
    let indent = "  ";
    let max_w = WIDTH.saturating_sub(2);
    let mut line_len = 0usize;
    let mut line = String::from(indent);
    for word in text.split_whitespace() {
        if line_len > 0 && line_len + 1 + word.len() > max_w {
            println!("{line}");
            line = String::from(indent);
            line_len = 0;
        }
        if line_len > 0 {
            line.push(' ');
            line_len += 1;
        }
        line.push_str(word);
        line_len += word.len();
    }
    if line_len > 0 {
        println!("{line}");
    }
}

// ── Input helpers ──────────────────────────────────────────────────────────────

/// Read a line from stdin. Returns None on EOF. Returns Some(trimmed).
fn ask(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().ok();
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some(line.trim().to_string()),
        Err(_) => None,
    }
}

/// Loop until the user provides an integer in [lo, hi]. None on quit/EOF.
fn ask_int(prompt: &str, lo: u32, hi: u32, default: Option<u32>) -> Option<u32> {
    loop {
        let raw = ask(prompt)?;
        if raw.is_empty() {
            if let Some(d) = default {
                return Some(d);
            }
            println!("{}", yellow(&format!("  Please enter a number between {lo} and {hi}.")));
            continue;
        }
        if raw.to_lowercase() == "q" || raw.to_lowercase() == "quit" {
            return None;
        }
        if let Ok(v) = raw.parse::<u32>() {
            if v >= lo && v <= hi {
                return Some(v);
            }
        }
        println!("{}", yellow(&format!("  Please enter a number between {lo} and {hi}.")));
    }
}

fn confirm(prompt: &str, default: bool) -> bool {
    let hint = if default { "[Y/n]" } else { "[y/N]" };
    let raw = ask(&format!("  {prompt} {hint} → ")).unwrap_or_default();
    if raw.is_empty() {
        return default;
    }
    matches!(raw.to_lowercase().as_str(), "y" | "yes")
}

fn pause() {
    ask(&dim("  [Enter to continue] "));
}

// ── Profile / storage types ────────────────────────────────────────────────────

/// Per-source mastery state with SM-2 scheduling.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct ChapterMastery {
    total_seen: u32,
    total_correct: u32,
    /// SM-2 easiness factor (starts at 2.5).
    ef: f64,
    /// SM-2 repetition interval in days.
    interval: f64,
    /// ISO date of last answer.
    last_date: Option<String>,
    /// ISO date next review is due.
    next_review: Option<String>,
}

impl ChapterMastery {
    fn new() -> Self {
        Self { ef: 2.5, interval: 1.0, ..Default::default() }
    }

    fn score(&self) -> f64 {
        if self.total_seen == 0 {
            0.5
        } else {
            self.total_correct as f64 / self.total_seen as f64
        }
    }

    fn is_due(&self) -> bool {
        let today = Local::now().date_naive().format("%Y-%m-%d").to_string();
        match &self.next_review {
            None => true,
            Some(d) => d.as_str() <= today.as_str(),
        }
    }

    /// Update SM-2 state. quality: 0..=5.
    fn update_sm2(&mut self, quality: u8) {
        if self.ef == 0.0 {
            self.ef = 2.5;
            self.interval = 1.0;
        }
        let q = quality as f64;
        self.ef = (self.ef + (0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02))).max(1.3);
        if quality < 3 {
            self.interval = 1.0;
        } else if self.interval < 1.5 {
            self.interval = 1.0;
        } else if self.interval < 2.5 {
            self.interval = 6.0;
        } else {
            self.interval = (self.interval * self.ef).round();
        }
        let today = Local::now().date_naive();
        let next = today + chrono::Duration::days(self.interval as i64);
        self.last_date = Some(today.format("%Y-%m-%d").to_string());
        self.next_review = Some(next.format("%Y-%m-%d").to_string());
    }
}

/// A completed quiz session record.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionRecord {
    timestamp: String,
    n_questions: u32,
    n_correct: u32,
    scope_label: String,
    duration_secs: f64,
    streak_max: u32,
}

/// A user profile serialised as JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserProfile {
    name: String,
    /// Mastery keyed by source label.
    mastery: HashMap<String, ChapterMastery>,
    session_history: Vec<SessionRecord>,
    /// Question texts flagged for review.
    flagged: Vec<String>,
}

impl UserProfile {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            mastery: HashMap::new(),
            session_history: Vec::new(),
            flagged: Vec::new(),
        }
    }

    fn total_answered(&self) -> u32 {
        self.mastery.values().map(|m| m.total_seen).sum()
    }

    fn record_answer(&mut self, source: &str, correct: bool, confidence: u8) {
        let entry = self
            .mastery
            .entry(source.to_string())
            .or_insert_with(ChapterMastery::new);
        entry.total_seen += 1;
        if correct {
            entry.total_correct += 1;
        }
        let quality: u8 = match (correct, confidence) {
            (true, 3)  => 5,
            (true, 2)  => 4,
            (true, 1)  => 3,
            (false, 3) => 2,
            (false, 2) => 1,
            (false, 1) => 0,
            _          => if correct { 4 } else { 1 },
        };
        entry.update_sm2(quality);
    }

    fn add_session(&mut self, rec: SessionRecord) {
        self.session_history.push(rec);
        if self.session_history.len() > 50 {
            let drain = self.session_history.len() - 50;
            self.session_history.drain(0..drain);
        }
    }
}

// ── Profile storage ────────────────────────────────────────────────────────────

fn data_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".local").join("share").join("quiz-rc")
}

fn profiles_dir() -> PathBuf {
    data_dir().join("profiles")
}

fn list_profiles() -> Vec<String> {
    let dir = profiles_dir();
    if !dir.exists() {
        return vec![];
    }
    let mut names: Vec<String> = fs::read_dir(&dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("json"))
                .filter_map(|e| {
                    e.path()
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_string())
                })
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

fn load_profile(name: &str) -> Option<UserProfile> {
    let path = profiles_dir().join(format!("{name}.json"));
    let data = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&data).ok()
}

fn save_profile(profile: &UserProfile) {
    let dir = profiles_dir();
    fs::create_dir_all(&dir).ok();
    let path = dir.join(format!("{}.json", profile.name));
    if let Ok(data) = serde_json::to_string_pretty(profile) {
        fs::write(path, data).ok();
    }
}

fn create_profile(name: &str) -> UserProfile {
    let p = UserProfile::new(name);
    save_profile(&p);
    p
}

// ── Question loading ───────────────────────────────────────────────────────────

/// A question tagged with its source file label.
#[derive(Debug, Clone)]
struct TaggedQuestion {
    q: Question,
    /// Human-readable source label (used as mastery key).
    source: String,
}

/// Walk `dir` recursively, loading all .json question banks.
/// Tries the new flat-array format first (`load_question_pool`), then the
/// legacy Quiz-object format (`load_quiz`).
fn load_all_questions(dir: &Path) -> Vec<TaggedQuestion> {
    let mut out = Vec::new();
    if !dir.exists() {
        return out;
    }
    walk_dir(dir, dir, &mut out);
    out
}

fn walk_dir(root: &Path, dir: &Path, out: &mut Vec<TaggedQuestion>) {
    let rd = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut entries: Vec<_> = rd.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            walk_dir(root, &path, out);
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        // Derive a readable source label from path relative to root.
        let rel = path.strip_prefix(root).unwrap_or(&path);
        let source = rel
            .with_extension("")
            .to_string_lossy()
            .replace(['/', '\\'], " › ");

        // Try new flat-array format first.
        if let Ok(qs) = load_question_pool(&path) {
            for q in qs {
                out.push(TaggedQuestion { q, source: source.clone() });
            }
            continue;
        }

        // Fallback: legacy Quiz-object format.
        if let Ok(quiz) = load_quiz(&path) {
            for q in quiz.questions {
                out.push(TaggedQuestion { q, source: source.clone() });
            }
        }
    }
}

// ── Question scope & selection ─────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Scope {
    Adaptive,
    All,
    Source(String),
    Tag(String),
    Flagged,
}

fn filter_by_scope<'a>(
    questions: &'a [TaggedQuestion],
    scope: &Scope,
    flagged: &[String],
) -> Vec<&'a TaggedQuestion> {
    match scope {
        Scope::All | Scope::Adaptive => questions.iter().collect(),
        Scope::Source(s) => questions.iter().filter(|tq| tq.source == *s).collect(),
        Scope::Tag(tag) => {
            let lower = tag.to_lowercase();
            questions
                .iter()
                .filter(|tq| {
                    tq.source.to_lowercase().contains(&lower)
                        || tq.q.text().to_lowercase().contains(&lower)
                        || tq.q.tags().iter().any(|t| t.to_lowercase().contains(&lower))
                })
                .collect()
        }
        Scope::Flagged => questions
            .iter()
            .filter(|tq| flagged.contains(&tq.q.text().to_string()))
            .collect(),
    }
}

/// Weighted reservoir sampling — higher weight for weak / due sources.
fn select_questions<'a>(
    pool: &[&'a TaggedQuestion],
    scope: &Scope,
    profile: &UserProfile,
    n: usize,
    rng: &mut SimpleRng,
) -> Vec<&'a TaggedQuestion> {
    if pool.is_empty() {
        return vec![];
    }
    let n = n.min(pool.len());

    let weights: Vec<f64> = if matches!(scope, Scope::Adaptive) {
        pool.iter()
            .map(|tq| {
                let m = profile.mastery.get(&tq.source);
                let score = m.map(|x| x.score()).unwrap_or(0.5);
                let due_bonus = m.map(|x| if x.is_due() { 0.3 } else { 0.0 }).unwrap_or(0.3);
                (1.0 - score + due_bonus).max(0.05)
            })
            .collect()
    } else {
        vec![1.0; pool.len()]
    };

    // Efraimidis-Spirakis weighted reservoir sampling key r^(1/w).
    let mut keyed: Vec<(f64, usize)> = pool
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let r = rng.next_f64();
            let key = r.powf(1.0 / weights[i].max(1e-9));
            (key, i)
        })
        .collect();
    keyed.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    keyed[..n].iter().map(|(_, i)| pool[*i]).collect()
}

// ── Minimal xorshift64 RNG ─────────────────────────────────────────────────────

struct SimpleRng(u64);

impl SimpleRng {
    fn new() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0xcafe_babe_dead_beef);
        Self(seed ^ 0x123456789abcdef0)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    fn shuffle<T>(&mut self, v: &mut Vec<T>) {
        for i in (1..v.len()).rev() {
            let j = (self.next_u64() as usize) % (i + 1);
            v.swap(i, j);
        }
    }
}

// ── Text wrapping ──────────────────────────────────────────────────────────────

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut len = 0usize;
    for word in text.split_whitespace() {
        if len > 0 && len + 1 + word.len() > max_width {
            lines.push(current.clone());
            current.clear();
            len = 0;
        }
        if len > 0 {
            current.push(' ');
            len += 1;
        }
        current.push_str(word);
        len += word.len();
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

// ── Profile selection UI ───────────────────────────────────────────────────────

fn choose_or_create_profile() -> UserProfile {
    let profiles = list_profiles();

    if profiles.is_empty() {
        println!();
        section("Welcome to the Reservoir Computing Adaptive Quiz!");
        print_wrap(
            "This quiz covers Echo State Networks, Liquid State Machines, \
             memory capacity, chaotic systems, and the full Reservoir Computing curriculum.",
        );
        print_wrap(
            "It tracks your mastery per topic and adjusts question selection \
             based on where you need the most practice (SM-2 spaced repetition).",
        );
        println!();
        let name = ask("  Your name → ")
            .unwrap_or_default();
        let name = if name.is_empty() { "Student".to_string() } else { name };
        return create_profile(&name);
    }

    println!();
    section("Select a profile");
    for (i, name) in profiles.iter().enumerate() {
        println!("  [{}] {name}", i + 1);
    }
    println!("  [{}] Create new profile", profiles.len() + 1);
    hr("─");

    let choice = ask_int(
        "  → ",
        1,
        (profiles.len() + 1) as u32,
        Some(1),
    )
    .unwrap_or(1) as usize;

    if choice <= profiles.len() {
        let name = &profiles[choice - 1];
        load_profile(name).unwrap_or_else(|| create_profile(name))
    } else {
        let name = ask("  New profile name → ").unwrap_or_default();
        let name = if name.is_empty() { "Student".to_string() } else { name };
        create_profile(&name)
    }
}

// ── Main menu ──────────────────────────────────────────────────────────────────

/// Returns Some(1..=7), or None when the user quits/EOF.
fn show_main_menu(profile_name: &str, due_count: usize, flagged_count: usize) -> Option<u32> {
    println!();
    hr("─");
    println!(
        "\n  {}  •  {}",
        bold(&magenta("Reservoir Computing Quiz")),
        dim(profile_name)
    );
    if due_count > 0 {
        println!("  {}", yellow(&bold(&format!("  {due_count} topic(s) due for review"))));
    }
    if flagged_count > 0 {
        println!("  {}", cyan(&format!("  {flagged_count} question(s) flagged for review")));
    }
    println!();
    println!("  {}  {}", bold("1."), dim("Adaptive quiz      (focuses on your weakest areas)"));
    println!("  {}  {}", bold("2."), dim("Custom quiz        (choose topic or tag)"));
    println!("  {}  {}", bold("3."), dim("Study a topic      (preview questions, then optionally quiz)"));
    println!("  {}  {}", bold("4."), dim("Mastery dashboard"));
    println!("  {}  {}", bold("5."), dim("Export progress    (saves a Markdown report)"));
    println!("  {}  {}", bold("6."), dim("Switch profile"));
    println!("  {}  {}", bold("7."), dim("Quit"));
    println!();
    hr("─");

    loop {
        let raw = ask("  → ")?;
        if raw.to_lowercase() == "q" || raw.to_lowercase() == "quit" {
            return None;
        }
        if let Ok(n) = raw.parse::<u32>() {
            if (1..=7).contains(&n) {
                return Some(n);
            }
        }
        println!("{}", yellow("  Please enter 1–7."));
    }
}

// ── Custom quiz scope menu ─────────────────────────────────────────────────────

fn show_scope_menu(questions: &[TaggedQuestion], flagged_count: usize) -> Option<Scope> {
    println!();
    hr("─");
    section("Choose scope");
    println!("  [1] All questions");
    println!("  [2] A specific topic (by source file)");
    println!("  [3] A tag / keyword");
    if flagged_count > 0 {
        println!(
            "  [4] Flagged for review  {}",
            dim(&format!("({flagged_count} questions)"))
        );
    }
    hr("─");

    let ch = ask("  → ")?;
    if ch.to_lowercase() == "q" || ch.to_lowercase() == "quit" {
        return None;
    }

    match ch.trim() {
        "1" => Some(Scope::All),
        "2" => {
            let mut sources: Vec<String> = questions
                .iter()
                .map(|tq| tq.source.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            sources.sort();

            println!();
            for (i, s) in sources.iter().enumerate() {
                println!("  [{:2}] {s}", i + 1);
            }
            let n = ask_int("\n  Topic number → ", 1, sources.len() as u32, Some(1))?;
            Some(Scope::Source(sources[(n as usize) - 1].clone()))
        }
        "3" => {
            let tag = ask("  Tag/keyword → ")?.trim().to_string();
            if tag.is_empty() {
                Some(Scope::All)
            } else {
                Some(Scope::Tag(tag))
            }
        }
        "4" if flagged_count > 0 => Some(Scope::Flagged),
        _ => Some(Scope::All),
    }
}

// ── Question presentation ──────────────────────────────────────────────────────

/// Returns Some((correct, flagged, confidence)) or None if user quits.
fn present_question(
    tq: &TaggedQuestion,
    num: usize,
    total: usize,
    running_correct: usize,
) -> Option<(bool, bool, u8)> {
    println!();
    hr("─");
    println!(
        "  {}  {}  {}",
        dim(&tq.source),
        dim(&format!("Score: {running_correct}/{}", num - 1)),
        dim(&format!("Q {num}/{total}"))
    );

    let q = &tq.q;

    // Difficulty badge
    let diff_badge = match q.difficulty() {
        quiz_core::question::Difficulty::Beginner     => green("[beginner]"),
        quiz_core::question::Difficulty::Intermediate => yellow("[intermediate]"),
        quiz_core::question::Difficulty::Advanced     => red("[advanced]"),
    };
    if q.generated() {
        println!("  {diff_badge}  {}", dim("[AI-generated]"));
    } else {
        println!("  {diff_badge}");
    }

    // Question text (word-wrapped)
    println!("\n  {} ", bold(&format!("Q{num}/{total}.")));
    for (i, line) in wrap_text(q.text(), WIDTH - 4).iter().enumerate() {
        if i == 0 {
            println!("  {line}");
        } else {
            println!("      {line}");
        }
    }
    println!();

    match q {
        Question::MultipleChoice { choices, answer: correct_idx, .. } => {
            for (i, choice) in choices.iter().enumerate() {
                let label = (b'A' + i as u8) as char;
                println!("    {}. {choice}", bold(&label.to_string()));
            }
            println!();

            let user_idx = loop {
                let raw = ask("  Answer (A/B/C/D or q) → ")?.to_uppercase();
                if raw == "Q" || raw == "QUIT" {
                    return None;
                }
                let idx = match raw.trim() {
                    "A" => Some(0usize),
                    "B" => Some(1),
                    "C" => Some(2),
                    "D" => Some(3),
                    _   => None,
                };
                if let Some(i) = idx {
                    if i < choices.len() {
                        break i;
                    }
                }
                println!("{}", yellow("  Please enter A, B, C, or D."));
            };

            let correct = user_idx == *correct_idx;
            let label_str = format!(
                "{}: {}",
                (b'A' + *correct_idx as u8) as char,
                choices.get(*correct_idx).map_or("?", |s| s.as_str())
            );
            let (f, c) = show_result(correct, &label_str, q.explanation())?;
            Some((correct, f, c))
        }

        Question::TrueFalse { choices, answer: correct_idx, .. } => {
            println!("    {}. True", bold("A"));
            println!("    {}. False", bold("B"));
            println!();

            let user_idx = loop {
                let raw = ask("  Answer (A/B or T/F or q) → ")?.to_uppercase();
                if raw == "Q" || raw == "QUIT" {
                    return None;
                }
                match raw.trim() {
                    "A" | "T" | "TRUE"  => break 0usize,
                    "B" | "F" | "FALSE" => break 1usize,
                    _ => println!("{}", yellow("  Please enter A (True) or B (False).")),
                }
            };

            let correct = user_idx == *correct_idx;
            let label_str = if *correct_idx == 0 { "A (True)" } else { "B (False)" };
            // choices vec exists but we use the index display
            let _ = choices;
            let (f, c) = show_result(correct, label_str, q.explanation())?;
            Some((correct, f, c))
        }

        Question::FillBlank { answer: correct_ans, choices, .. } => {
            let raw = ask("  Your answer (or q to quit) → ")?;
            if raw.to_lowercase() == "q" || raw.to_lowercase() == "quit" {
                return None;
            }
            let lower = raw.trim().to_lowercase();
            // Accept exact match against any acceptable answer, or substring (≥3 chars)
            let correct = choices.iter().any(|c| {
                let cl = c.to_lowercase();
                cl == lower || (!lower.is_empty() && lower.len() >= 3 && cl.contains(&lower))
            });
            let (f, c) = show_result(correct, correct_ans.as_str(), q.explanation())?;
            Some((correct, f, c))
        }

        Question::Proof { answer: correct_ans, choices, .. } => {
            // Display the proof scaffold
            println!("  Complete the proof (fill in the blanks marked ___):\n");
            for (i, line) in choices.iter().enumerate() {
                println!("    {i}. {line}");
            }
            println!();
            let n_blanks = choices.iter().map(|l| l.matches("___").count()).sum::<usize>();
            println!(
                "  {}",
                dim(&format!("Enter {n_blanks} fill(s) separated by ' | ':"))
            );

            let raw = ask("  Your answer (or q to quit) → ")?;
            if raw.to_lowercase() == "q" || raw.to_lowercase() == "quit" {
                return None;
            }
            let correct = q.check(&raw);
            let (f, c) = show_result(correct, correct_ans.as_str(), q.explanation())?;
            Some((correct, f, c))
        }
    }
}

/// Display the result + explanation; collect confidence. Returns (flagged, confidence).
fn show_result(correct: bool, label: &str, explanation: &str) -> Option<(bool, u8)> {
    println!();
    if correct {
        println!("  {}", green(&bold("✓  Correct!")));
    } else {
        println!("  {}  Answer: {}", red(&bold("✗  Incorrect.")), bold(label));
    }
    println!();

    if !explanation.is_empty() {
        println!("{}", dim("  Explanation:"));
        for line in wrap_text(explanation, WIDTH - 6) {
            println!("    {line}");
        }
        println!();
    }

    let hint = dim("1=guess · 2=unsure · 3=sure · f=flag · Enter=continue");
    let raw = ask(&format!("  {hint} → "))?.to_lowercase();

    let confidence = match raw.as_str() {
        "1" => 1u8,
        "3" => 3,
        _   => 2,
    };
    let flagged = raw == "f";
    if flagged {
        println!("{}", dim("  [Flagged for review]"));
    }

    Some((flagged, confidence))
}

// ── Session runner ─────────────────────────────────────────────────────────────

struct SessionResult {
    correct: usize,
    total: usize,
    duration_secs: f64,
    streak_max: u32,
    wrong_sources: Vec<String>,
}

fn run_session(
    questions: &[&TaggedQuestion],
    profile: &mut UserProfile,
    scope_label: &str,
) -> Option<SessionResult> {
    let total = questions.len();
    let start = Instant::now();
    let mut correct_count = 0usize;
    let mut streak = 0u32;
    let mut streak_max = 0u32;
    let mut wrong_sources: Vec<String> = Vec::new();

    for (i, tq) in questions.iter().enumerate() {
        match present_question(tq, i + 1, total, correct_count) {
            None => {
                // User quit mid-session: save partial record.
                let elapsed = start.elapsed().as_secs_f64();
                profile.add_session(SessionRecord {
                    timestamp: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
                    n_questions: i as u32,
                    n_correct: correct_count as u32,
                    scope_label: format!("{scope_label} (partial)"),
                    duration_secs: elapsed,
                    streak_max,
                });
                save_profile(profile);
                return None;
            }
            Some((is_correct, flagged, confidence)) => {
                profile.record_answer(&tq.source, is_correct, confidence);

                if flagged && !profile.flagged.contains(&tq.q.text().to_string()) {
                    profile.flagged.push(tq.q.text().to_string());
                }

                save_profile(profile);

                if is_correct {
                    correct_count += 1;
                    streak += 1;
                    streak_max = streak_max.max(streak);
                } else {
                    streak = 0;
                    wrong_sources.push(tq.source.clone());
                }
            }
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    profile.add_session(SessionRecord {
        timestamp: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        n_questions: total as u32,
        n_correct: correct_count as u32,
        scope_label: scope_label.to_string(),
        duration_secs: elapsed,
        streak_max,
    });
    save_profile(profile);

    Some(SessionResult {
        correct: correct_count,
        total,
        duration_secs: elapsed,
        streak_max,
        wrong_sources,
    })
}

// ── Session summary ────────────────────────────────────────────────────────────

fn show_session_summary(result: &SessionResult) {
    println!();
    hr("═");
    let pct = if result.total > 0 {
        (100 * result.correct / result.total) as u32
    } else {
        0
    };
    let colour_fn: fn(&str) -> String = if pct >= 70 { green }
                                        else if pct >= 50 { yellow }
                                        else { red };

    println!(
        "\n  {}  {}  ({}%)",
        bold("Result:"),
        colour_fn(&bold(&format!("{}/{}", result.correct, result.total))),
        pct
    );
    print!("  {}", dim(&format!("Time: {:.0}s", result.duration_secs)));
    if result.streak_max > 1 {
        print!("   {}", yellow(&format!("Best streak: {}", result.streak_max)));
    }
    println!();

    println!();
    match pct {
        100      => println!("{}", green("  Perfect score! Outstanding work.")),
        70..=99  => println!("{}", yellow("  Good work. Review the topics you missed.")),
        _        => println!("{}", red("  Keep studying — review the relevant material and try again.")),
    }

    let mut wrong: Vec<String> = result.wrong_sources.clone();
    wrong.sort();
    wrong.dedup();
    if !wrong.is_empty() {
        println!("\n  {}", bold("Topics to review:"));
        for w in &wrong {
            println!("    • {w}");
        }
    }

    hr("═");
    pause();
}

// ── Mastery dashboard ──────────────────────────────────────────────────────────

fn mastery_bar(pct: usize) -> String {
    let fill = (pct / 5).min(20);
    format!("{}{}", "█".repeat(fill), "░".repeat(20 - fill))
}

fn show_dashboard(profile: &UserProfile, questions: &[TaggedQuestion]) {
    section(&format!("Mastery Dashboard — {}", bold(&profile.name)));
    println!("  Total answered: {}", bold(&profile.total_answered().to_string()));

    let due_count = profile
        .mastery
        .values()
        .filter(|m| m.is_due() && m.total_seen > 0)
        .count();
    if due_count > 0 {
        println!(
            "  {}",
            yellow(&bold(&format!("  {due_count} topic(s) due for review today")))
        );
    }
    let flagged_count = profile.flagged.len();
    if flagged_count > 0 {
        println!("  {}", cyan(&format!("  {flagged_count} question(s) flagged for review")));
    }
    println!();

    // Unique sources in sorted order
    let mut sources: Vec<String> = questions
        .iter()
        .map(|tq| tq.source.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    sources.sort();

    for source in &sources {
        let m = profile.mastery.get(source.as_str());
        let score = m.map(|x| x.score()).unwrap_or(0.0);
        let pct = (score * 100.0).round() as usize;
        let bar = mastery_bar(pct);
        let colour_fn: fn(&str) -> String = if pct >= 70 { green }
                                             else if pct >= 40 { yellow }
                                             else { red };
        let due_flag = if m.map(|x| x.is_due() && x.total_seen > 0).unwrap_or(false) {
            yellow(" ⟳")
        } else {
            "  ".to_string()
        };
        let started = m.map(|x| x.total_seen > 0).unwrap_or(false);
        let star = if started { "★" } else { "·" };
        let seen_n = m.map(|x| x.total_seen).unwrap_or(0);
        let seen_s = if started {
            dim(&format!("({seen_n} seen)"))
        } else {
            dim("(not started)")
        };

        let label: String = source.chars().take(32).collect();
        println!(
            "    {star} {label:<32} {} {:3}%{due_flag} {seen_s}",
            colour_fn(&bar),
            pct
        );
    }

    println!();
    show_session_history(profile);
    hr("─");
    pause();
}

fn show_session_history(profile: &UserProfile) {
    if profile.session_history.is_empty() {
        return;
    }

    println!("\n  {}", cyan(&bold("Recent sessions:")));
    for sess in profile.session_history.iter().rev().take(8) {
        let ts = &sess.timestamp[..10.min(sess.timestamp.len())];
        let pct = if sess.n_questions > 0 {
            100 * sess.n_correct / sess.n_questions
        } else {
            0
        };
        let scope: String = sess.scope_label.chars().take(28).collect();
        let dur = format!("{:.0}s", sess.duration_secs);
        let colour_fn: fn(&str) -> String = if pct >= 70 { green }
                                             else if pct >= 50 { yellow }
                                             else { red };
        let streak_s = if sess.streak_max > 1 {
            format!(" streak:{}", sess.streak_max)
        } else {
            String::new()
        };
        println!(
            "    {}  {}  {}/{}  {:<28}  {}",
            dim(ts),
            colour_fn(&format!("{pct:3}%")),
            sess.n_correct,
            sess.n_questions,
            scope,
            dim(&format!("{dur}{streak_s}"))
        );
    }
    println!();
}

// ── Study mode ─────────────────────────────────────────────────────────────────

/// Returns true if the user wants to quiz after studying.
fn show_study_mode(questions: &[TaggedQuestion], source: &str) -> bool {
    section(&format!("Study: {source}"));

    let relevant: Vec<&TaggedQuestion> = questions
        .iter()
        .filter(|tq| tq.source == source)
        .collect();

    if relevant.is_empty() {
        println!("{}", yellow("  No questions found for this topic."));
        pause();
        return false;
    }

    println!("  {} questions available on this topic.\n", relevant.len());

    // Show a preview of question texts (no answers).
    for (i, tq) in relevant.iter().take(5).enumerate() {
        let kind_s = match &tq.q {
            Question::MultipleChoice { .. } => dim("[MC]"),
            Question::TrueFalse { .. }      => dim("[T/F]"),
            Question::FillBlank { .. }      => dim("[Fill]"),
            Question::Proof { .. }          => dim("[Proof]"),
        };
        println!("  {} {}. {}", kind_s, i + 1, tq.q.text());
        println!();
    }
    if relevant.len() > 5 {
        println!("  {}", dim(&format!("... and {} more questions.", relevant.len() - 5)));
        println!();
    }

    hr("─");
    confirm("Quiz on this topic now?", true)
}

// ── Progress export ────────────────────────────────────────────────────────────

fn export_progress(profile: &UserProfile, questions: &[TaggedQuestion]) {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let mut lines = vec![
        format!("# Reservoir Computing Quiz Progress — {}", profile.name),
        format!(
            "Exported: {today}  |  Total answered: {}",
            profile.total_answered()
        ),
        String::new(),
        "| Source | Score | Seen | Correct | Due | Next Review |".to_string(),
        "|--------|------:|-----:|--------:|-----|-------------|".to_string(),
    ];

    let mut sources: Vec<String> = questions
        .iter()
        .map(|tq| tq.source.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    sources.sort();

    for source in &sources {
        let m = profile.mastery.get(source.as_str());
        let score   = m.map(|x| x.score()).unwrap_or(0.0);
        let seen    = m.map(|x| x.total_seen).unwrap_or(0);
        let correct = m.map(|x| x.total_correct).unwrap_or(0);
        let due     = m.map(|x| x.is_due() && x.total_seen > 0).unwrap_or(false);
        let nr      = m.and_then(|x| x.next_review.clone()).unwrap_or_default();
        lines.push(format!(
            "| {source} | {score:.2} | {seen} | {correct} | {} | {nr} |",
            if due { "✓" } else { "" }
        ));
    }

    let export_dir = data_dir().join("exports");
    fs::create_dir_all(&export_dir).ok();
    let path = export_dir.join(format!("progress_export_{today}.md"));
    match fs::write(&path, lines.join("\n")) {
        Ok(_) => {
            println!("\n  {} Progress exported to:", green("✓"));
            println!("    {}\n", bold(&path.display().to_string()));
        }
        Err(e) => {
            println!("{}", red(&format!("  ✗ Export failed: {e}")));
        }
    }
    pause();
}

// ── Locate questions directory ─────────────────────────────────────────────────

fn find_questions_dir() -> PathBuf {
    for candidate in &[
        PathBuf::from("questions"),
        PathBuf::from("../questions"),
        PathBuf::from("quiz-app/questions"),
        PathBuf::from("ReservoirComputing/quiz-app/questions"),
    ] {
        if candidate.exists() {
            return candidate.clone();
        }
    }
    PathBuf::from("questions")
}

// ── Entry point ────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    hr("═");
    println!("\n  {}\n", bold(&magenta("Reservoir Computing Adaptive Quiz")));
    hr("═");

    // Load questions
    let questions_dir = find_questions_dir();
    let questions = load_all_questions(&questions_dir);

    if questions.is_empty() {
        eprintln!(
            "{}",
            yellow(&format!(
                "  Warning: no questions found in '{}'.",
                questions_dir.display()
            ))
        );
        eprintln!("  Run from the quiz-app directory, or ensure questions/*.json files exist.");
    } else {
        println!(
            "  {} questions loaded from {}.",
            dim(&questions.len().to_string()),
            dim(&questions_dir.display().to_string())
        );
    }

    // Profile selection
    let mut profile = choose_or_create_profile();
    let mut rng = SimpleRng::new();

    // ── Main loop ─────────────────────────────────────────────────────────────
    loop {
        let due_count = profile
            .mastery
            .values()
            .filter(|m| m.is_due() && m.total_seen > 0)
            .count();
        let flagged_count = profile.flagged.len();

        let choice = match show_main_menu(&profile.name, due_count, flagged_count) {
            None => {
                println!("\n  {}\n", dim("See you next time."));
                break;
            }
            Some(c) => c,
        };

        match choice {
            // ── 1. Adaptive quiz ──────────────────────────────────────────────
            1 => {
                if questions.is_empty() {
                    println!("{}", yellow("  No questions loaded."));
                    pause();
                    continue;
                }
                let pool: Vec<&TaggedQuestion> = questions.iter().collect();
                let cap = (pool.len().min(50)) as u32;
                let default_n = 10_u32.min(cap);
                let n = ask_int(
                    &format!("  How many questions? (1–{cap}, Enter for {default_n}) → "),
                    1, cap, Some(default_n),
                )
                .unwrap_or(default_n) as usize;

                let selected = select_questions(&pool, &Scope::Adaptive, &profile, n, &mut rng);
                if selected.is_empty() {
                    println!("{}", yellow("  No questions available."));
                    pause();
                    continue;
                }

                if let Some(result) = run_session(&selected, &mut profile, "Adaptive") {
                    show_session_summary(&result);
                }
            }

            // ── 2. Custom quiz ────────────────────────────────────────────────
            2 => {
                if questions.is_empty() {
                    println!("{}", yellow("  No questions loaded."));
                    pause();
                    continue;
                }

                let scope = match show_scope_menu(&questions, flagged_count) {
                    None    => continue,
                    Some(s) => s,
                };

                let scope_label = match &scope {
                    Scope::All        => "All questions".to_string(),
                    Scope::Adaptive   => "Adaptive".to_string(),
                    Scope::Source(s)  => s.clone(),
                    Scope::Tag(t)     => format!("Tag: {t}"),
                    Scope::Flagged    => "Flagged questions".to_string(),
                };

                let flagged = profile.flagged.clone();
                let pool = filter_by_scope(&questions, &scope, &flagged);
                if pool.is_empty() {
                    println!("{}", yellow("  No questions match that scope."));
                    pause();
                    continue;
                }

                let cap = (pool.len().min(50)) as u32;
                let default_n = 10_u32.min(cap);
                let n = ask_int(
                    &format!("  How many questions? (1–{cap}, Enter for {default_n}) → "),
                    1, cap, Some(default_n),
                )
                .unwrap_or(default_n) as usize;

                let selected = select_questions(&pool, &scope, &profile, n, &mut rng);
                if selected.is_empty() {
                    println!("{}", yellow("  No questions available."));
                    pause();
                    continue;
                }

                if let Some(result) = run_session(&selected, &mut profile, &scope_label) {
                    show_session_summary(&result);
                }
            }

            // ── 3. Study a topic ──────────────────────────────────────────────
            3 => {
                if questions.is_empty() {
                    println!("{}", yellow("  No questions loaded."));
                    pause();
                    continue;
                }

                let mut sources: Vec<String> = questions
                    .iter()
                    .map(|tq| tq.source.clone())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                sources.sort();

                println!();
                hr("─");
                section("Choose a topic to study");
                for (i, s) in sources.iter().enumerate() {
                    println!("  [{:2}] {s}", i + 1);
                }
                hr("─");

                let n = match ask_int("  Topic number → ", 1, sources.len() as u32, Some(1)) {
                    None    => continue,
                    Some(n) => n as usize,
                };
                let source = sources[n - 1].clone();

                let should_quiz = show_study_mode(&questions, &source);
                if should_quiz {
                    let mut pool: Vec<&TaggedQuestion> = questions
                        .iter()
                        .filter(|tq| tq.source == source)
                        .collect();
                    rng.shuffle(&mut pool);

                    let cap = (pool.len().min(50)) as u32;
                    let default_n = 5_u32.min(cap);
                    let n = ask_int(
                        &format!("  How many questions? (1–{cap}, Enter for {default_n}) → "),
                        1, cap, Some(default_n),
                    )
                    .unwrap_or(default_n) as usize;

                    let selected: Vec<&TaggedQuestion> = pool.into_iter().take(n).collect();
                    if let Some(result) =
                        run_session(&selected, &mut profile, &format!("Study: {source}"))
                    {
                        show_session_summary(&result);
                    }
                }
            }

            // ── 4. Mastery dashboard ──────────────────────────────────────────
            4 => {
                show_dashboard(&profile, &questions);
            }

            // ── 5. Export progress ────────────────────────────────────────────
            5 => {
                export_progress(&profile, &questions);
            }

            // ── 6. Switch profile ─────────────────────────────────────────────
            6 => {
                profile = choose_or_create_profile();
            }

            // ── 7. Quit ───────────────────────────────────────────────────────
            7 | _ => {
                println!("\n  {}\n", dim("See you next time."));
                break;
            }
        }
    }

    Ok(())
}
