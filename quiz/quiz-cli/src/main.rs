/*!
Adaptive Quiz CLI — Library of Babel
====================================
Interactive, subject-agnostic quiz application. Loads any book's
`subject.toml` + question bank and runs an adaptive quiz driven entirely by
the `quiz-core` engine (SM-2 spaced repetition, mastery-weighted selection,
per-chapter mastery tracking, session history, and progress export).

The active subject is chosen by (in priority order):
  1. `--subject <dir>`
  2. the `QUIZ_SUBJECT` environment variable
  3. the current working directory

Run `--stats` to print per-chapter question counts and exit.
*/

use std::{
    collections::{BTreeMap, HashMap},
    fs,
    io::{self, Write},
    path::PathBuf,
};

use chrono::Local;

use quiz_core::adaptive::{mastery_summary, MasteryRow};
use quiz_core::question::Difficulty;
use quiz_core::storage::{create_profile, list_profiles, load_profile};
use quiz_core::{
    load_or_create, load_question_bank, save_profile, select_questions, ChapterInfo, Question,
    Scope, Session, SessionResult, SubjectConfig, UserProfile,
};

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
    for line in wrap_text(text, WIDTH.saturating_sub(2)) {
        println!("  {line}");
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

// ── Per-subject data directory ─────────────────────────────────────────────────

/// Per-subject data root: `~/.local/share/quiz/<slug>/`.
///
/// The slug is derived from the subject title (lowercase, hyphenated), so each
/// book keeps its own profiles and exports.
fn data_dir(slug: &str) -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("quiz")
        .join(slug)
}

fn profiles_dir(slug: &str) -> PathBuf {
    data_dir(slug).join("profiles")
}

// ── Chapter metadata ───────────────────────────────────────────────────────────

fn build_chapter_info(config: &SubjectConfig) -> HashMap<u32, ChapterInfo> {
    config
        .chapters
        .values()
        .map(|m| (m.index, ChapterInfo { phase: m.phase, name: m.name.clone() }))
        .collect()
}

/// `Ch.N — Name` label for a chapter (falls back gracefully).
fn chapter_label(config: &SubjectConfig, chapter: u32) -> String {
    format!("Ch.{chapter} — {}", config.chapter_name(chapter))
}

/// Sorted (index, name) list of chapters that actually have questions.
fn chapters_with_questions(config: &SubjectConfig, questions: &[Question]) -> Vec<(u32, String)> {
    let present: std::collections::BTreeSet<u32> = questions.iter().map(|q| q.chapter()).collect();
    present.into_iter().map(|c| (c, config.chapter_name(c))).collect()
}

// ── Profile selection UI ───────────────────────────────────────────────────────

fn choose_or_create_profile(config: &SubjectConfig, dir: &PathBuf) -> UserProfile {
    let profiles = list_profiles(dir);

    if profiles.is_empty() {
        println!();
        section(&format!("Welcome to the {}!", config.title));
        print_wrap(
            "This quiz tracks your mastery per chapter and adapts question \
             selection to focus on where you need the most practice \
             (SM-2 spaced repetition).",
        );
        println!();
        let name = ask("  Your name → ").unwrap_or_default();
        let name = if name.is_empty() { "Student".to_string() } else { name };
        return load_or_create(dir, &name).unwrap_or_else(|_| UserProfile::new(&name));
    }

    println!();
    section("Select a profile");
    for (i, name) in profiles.iter().enumerate() {
        println!("  [{}] {name}", i + 1);
    }
    println!("  [{}] Create new profile", profiles.len() + 1);
    hr("─");

    let choice = ask_int("  → ", 1, (profiles.len() + 1) as u32, Some(1)).unwrap_or(1) as usize;

    if choice <= profiles.len() {
        let name = &profiles[choice - 1];
        load_profile(dir, name).unwrap_or_else(|| {
            create_profile(dir, name).unwrap_or_else(|_| UserProfile::new(name))
        })
    } else {
        let name = ask("  New profile name → ").unwrap_or_default();
        let name = if name.is_empty() { "Student".to_string() } else { name };
        load_or_create(dir, &name).unwrap_or_else(|_| UserProfile::new(&name))
    }
}

// ── Main menu ──────────────────────────────────────────────────────────────────

/// Returns Some(1..=7), or None when the user quits/EOF.
fn show_main_menu(title: &str, profile_name: &str, due_count: usize, flagged_count: usize) -> Option<u32> {
    println!();
    hr("─");
    println!("\n  {}  •  {}", bold(&magenta(title)), dim(profile_name));
    if due_count > 0 {
        println!("  {}", yellow(&bold(&format!("  {due_count} chapter(s) due for review"))));
    }
    if flagged_count > 0 {
        println!("  {}", cyan(&format!("  {flagged_count} question(s) flagged for review")));
    }
    println!();
    println!("  {}  {}", bold("1."), dim("Adaptive quiz      (focuses on your weakest chapters)"));
    println!("  {}  {}", bold("2."), dim("Custom quiz        (choose chapter, tag, due, or flagged)"));
    println!("  {}  {}", bold("3."), dim("Study a chapter    (preview questions, then optionally quiz)"));
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

fn show_scope_menu(
    config: &SubjectConfig,
    questions: &[Question],
    due_count: usize,
    flagged_count: usize,
) -> Option<Scope> {
    println!();
    hr("─");
    section("Choose scope");
    println!("  [1] All chapters");
    println!("  [2] A specific chapter");
    println!("  [3] A tag / keyword");
    if due_count > 0 {
        println!("  [4] Due for review     {}", dim(&format!("({due_count} chapters)")));
    }
    if flagged_count > 0 {
        println!("  [5] Flagged for review {}", dim(&format!("({flagged_count} questions)")));
    }
    hr("─");

    let ch = ask("  → ")?;
    if ch.to_lowercase() == "q" || ch.to_lowercase() == "quit" {
        return None;
    }

    match ch.trim() {
        "1" => Some(Scope::All),
        "2" => {
            let chapters = chapters_with_questions(config, questions);
            if chapters.is_empty() {
                return Some(Scope::All);
            }
            println!();
            for (i, (idx, name)) in chapters.iter().enumerate() {
                println!("  [{:2}] Ch.{idx} — {name}", i + 1);
            }
            let n = ask_int("\n  Chapter number → ", 1, chapters.len() as u32, Some(1))?;
            Some(Scope::Chapter(chapters[(n as usize) - 1].0))
        }
        "3" => {
            let tag = ask("  Tag/keyword → ")?.trim().to_string();
            if tag.is_empty() { Some(Scope::All) } else { Some(Scope::Tag(tag)) }
        }
        "4" if due_count > 0 => Some(Scope::Due),
        // Placeholder — the caller fills in the current review queue.
        "5" if flagged_count > 0 => Some(Scope::Review(Vec::new())),
        _ => Some(Scope::All),
    }
}

// ── Question presentation ──────────────────────────────────────────────────────

/// Present one question. Returns `Some((answer_string, flagged, confidence))`
/// or `None` if the user quits.
fn present_question(
    q: &Question,
    label: &str,
    num: usize,
    total: usize,
    running_correct: usize,
) -> Option<(String, bool, u8)> {
    println!();
    hr("─");
    println!(
        "  {}  {}  {}",
        dim(label),
        dim(&format!("Score: {running_correct}/{}", num - 1)),
        dim(&format!("Q {num}/{total}"))
    );

    let diff_badge = match q.difficulty() {
        Difficulty::Beginner     => green("[beginner]"),
        Difficulty::Intermediate => yellow("[intermediate]"),
        Difficulty::Advanced     => red("[advanced]"),
    };
    if q.generated() {
        println!("  {diff_badge}  {}", dim("[AI-generated]"));
    } else {
        println!("  {diff_badge}");
    }

    println!("\n  {} ", bold(&format!("Q{num}/{total}.")));
    for (i, line) in wrap_text(q.text(), WIDTH - 4).iter().enumerate() {
        if i == 0 { println!("  {line}"); } else { println!("      {line}"); }
    }
    println!();

    match q {
        Question::MultipleChoice { choices, .. } => {
            for (i, choice) in choices.iter().enumerate() {
                let letter = (b'A' + i as u8) as char;
                println!("    {}. {choice}", bold(&letter.to_string()));
            }
            println!();
            let letter = loop {
                let raw = ask("  Answer (A/B/C/D or q) → ")?.to_uppercase();
                if raw == "Q" || raw == "QUIT" { return None; }
                let idx = match raw.trim() {
                    "A" => Some(0usize), "B" => Some(1), "C" => Some(2), "D" => Some(3), _ => None,
                };
                if let Some(i) = idx {
                    if i < choices.len() { break raw.trim().to_string(); }
                }
                println!("{}", yellow("  Please enter A, B, C, or D."));
            };
            let correct = q.check(&letter);
            let (f, c) = show_result(correct, &q.correct_display(), q.explanation())?;
            Some((letter, f, c))
        }

        Question::TrueFalse { .. } => {
            println!("    {}. True", bold("A"));
            println!("    {}. False", bold("B"));
            println!();
            let ans = loop {
                let raw = ask("  Answer (A/B or T/F or q) → ")?.to_uppercase();
                if raw == "Q" || raw == "QUIT" { return None; }
                match raw.trim() {
                    "A" | "T" | "TRUE" => break "A".to_string(),
                    "B" | "F" | "FALSE" => break "B".to_string(),
                    _ => println!("{}", yellow("  Please enter A (True) or B (False).")),
                }
            };
            let correct = q.check(&ans);
            let (f, c) = show_result(correct, &q.correct_display(), q.explanation())?;
            Some((ans, f, c))
        }

        Question::FillBlank { .. } => {
            let raw = ask("  Your answer (or q to quit) → ")?;
            if raw.to_lowercase() == "q" || raw.to_lowercase() == "quit" { return None; }
            let correct = q.check(&raw);
            let (f, c) = show_result(correct, &q.correct_display(), q.explanation())?;
            Some((raw, f, c))
        }

        Question::Proof { choices, .. } => {
            println!("  Complete the proof (fill in the blanks marked ___):\n");
            for (i, line) in choices.iter().enumerate() {
                println!("    {i}. {line}");
            }
            println!();
            let n_blanks = choices.iter().map(|l| l.matches("___").count()).sum::<usize>();
            println!("  {}", dim(&format!("Enter {n_blanks} fill(s) separated by ' | ':")));
            let raw = ask("  Your answer (or q to quit) → ")?;
            if raw.to_lowercase() == "q" || raw.to_lowercase() == "quit" { return None; }
            let correct = q.check(&raw);
            let (f, c) = show_result(correct, &q.correct_display(), q.explanation())?;
            Some((raw, f, c))
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
        _ => 2,
    };
    let flagged = raw == "f";
    if flagged {
        println!("{}", dim("  [Flagged for review]"));
    }
    Some((flagged, confidence))
}

// ── Session runner ─────────────────────────────────────────────────────────────

/// Run a quiz over `selected`. Returns the completed [`SessionResult`], or
/// `None` if the user quit mid-session (a partial record is still saved).
fn run_session(
    config: &SubjectConfig,
    selected: Vec<Question>,
    profile: &mut UserProfile,
    dir: &PathBuf,
    scope_label: &str,
) -> Option<SessionResult> {
    let total = selected.len();
    let mut session = Session::new(selected, scope_label.to_string());
    let mut correct_count = 0usize;

    for i in 0..total {
        let q = session.questions[i].clone();
        let label = chapter_label(config, q.chapter());
        match present_question(&q, &label, i + 1, total, correct_count) {
            None => {
                // Quit mid-session: save a partial record and stop.
                let mut partial = SessionResult::new(format!("{scope_label} (partial)"));
                partial.n_questions = i as u32;
                partial.n_correct = correct_count as u32;
                profile.add_session(partial);
                let _ = save_profile(dir, profile);
                return None;
            }
            Some((answer, flagged, confidence)) => {
                let correct = session.submit(answer).unwrap_or(false);
                profile.record_answer(q.chapter(), correct, q.question_id(), confidence);
                if flagged {
                    profile.add_to_review_queue(q.question_id());
                }
                let _ = save_profile(dir, profile);
                if correct {
                    correct_count += 1;
                }
            }
        }
    }

    let result = session.to_result();
    profile.add_session(result.clone());
    let _ = save_profile(dir, profile);
    Some(result)
}

// ── Session summary ────────────────────────────────────────────────────────────

fn show_session_summary(config: &SubjectConfig, result: &SessionResult) {
    println!();
    hr("═");
    let total = result.n_questions as usize;
    let correct = result.n_correct as usize;
    let pct = result.score_pct();
    let colour_fn: fn(&str) -> String = if pct >= 70 { green } else if pct >= 50 { yellow } else { red };

    println!(
        "\n  {}  {}  ({}%)",
        bold("Result:"),
        colour_fn(&bold(&format!("{correct}/{total}"))),
        pct
    );
    print!("  {}", dim(&format!("Time: {:.0}s", result.duration_secs)));
    if result.streak_max > 1 {
        print!("   {}", yellow(&format!("Best streak: {}", result.streak_max)));
    }
    println!();
    println!();
    match pct {
        100 => println!("{}", green("  Perfect score! Outstanding work.")),
        70..=99 => println!("{}", yellow("  Good work. Review the chapters you missed.")),
        _ => println!("{}", red("  Keep studying — review the relevant material and try again.")),
    }

    let mut wrong: Vec<u32> = result.wrong_chapters.clone();
    wrong.sort_unstable();
    wrong.dedup();
    if !wrong.is_empty() {
        println!("\n  {}", bold("Chapters to review:"));
        for ch in &wrong {
            println!("    • {}", chapter_label(config, *ch));
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

fn show_dashboard(profile: &UserProfile, chapter_info: &HashMap<u32, ChapterInfo>) {
    section(&format!("Mastery Dashboard — {}", bold(&profile.name)));
    println!("  Total answered: {}", bold(&profile.total_answered().to_string()));

    let due = profile.chapters_due_for_review().len();
    if due > 0 {
        println!("  {}", yellow(&bold(&format!("  {due} chapter(s) due for review today"))));
    }
    if !profile.review_queue.is_empty() {
        println!("  {}", cyan(&format!("  {} question(s) flagged for review", profile.review_queue.len())));
    }
    println!();

    let rows: Vec<MasteryRow> = mastery_summary(profile, chapter_info);
    for row in &rows {
        let pct = (row.score * 100.0).round() as usize;
        let bar = mastery_bar(pct);
        let colour_fn: fn(&str) -> String = if pct >= 70 { green } else if pct >= 40 { yellow } else { red };
        let due_flag = if row.due { yellow(" ⟳") } else { "  ".to_string() };
        let star = if row.started { "★" } else { "·" };
        let seen_s = if row.started {
            dim(&format!("({} seen)", row.seen))
        } else {
            dim("(not started)")
        };
        let name = format!("Ch.{} {}", row.chapter, row.name);
        let label: String = name.chars().take(34).collect();
        println!("    {star} {label:<34} {} {:3}%{due_flag} {seen_s}", colour_fn(&bar), pct);
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
        let pct = sess.score_pct();
        let scope: String = sess.scope_label.chars().take(28).collect();
        let dur = format!("{:.0}s", sess.duration_secs);
        let colour_fn: fn(&str) -> String = if pct >= 70 { green } else if pct >= 50 { yellow } else { red };
        let streak_s = if sess.streak_max > 1 { format!(" streak:{}", sess.streak_max) } else { String::new() };
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

/// Returns true if the user wants to quiz on this chapter after studying.
fn show_study_mode(config: &SubjectConfig, questions: &[Question], chapter: u32) -> bool {
    section(&format!("Study: {}", chapter_label(config, chapter)));

    let relevant: Vec<&Question> = questions.iter().filter(|q| q.chapter() == chapter).collect();
    if relevant.is_empty() {
        println!("{}", yellow("  No questions found for this chapter."));
        pause();
        return false;
    }

    println!("  {} questions available on this chapter.\n", relevant.len());
    for (i, q) in relevant.iter().take(5).enumerate() {
        let kind_s = match q {
            Question::MultipleChoice { .. } => dim("[MC]"),
            Question::TrueFalse { .. }      => dim("[T/F]"),
            Question::FillBlank { .. }      => dim("[Fill]"),
            Question::Proof { .. }          => dim("[Proof]"),
        };
        println!("  {} {}. {}", kind_s, i + 1, q.text());
        println!();
    }
    if relevant.len() > 5 {
        println!("  {}", dim(&format!("... and {} more questions.", relevant.len() - 5)));
        println!();
    }

    hr("─");
    confirm("Quiz on this chapter now?", true)
}

// ── Progress export ────────────────────────────────────────────────────────────

fn export_progress(
    config: &SubjectConfig,
    profile: &UserProfile,
    chapter_info: &HashMap<u32, ChapterInfo>,
    slug: &str,
) {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let mut lines = vec![
        format!("# {} — Progress for {}", config.title, profile.name),
        format!("Exported: {today}  |  Total answered: {}", profile.total_answered()),
        String::new(),
        "| Chapter | Name | Score | Seen | Correct | Due | Next Review |".to_string(),
        "|--------:|------|------:|-----:|--------:|-----|-------------|".to_string(),
    ];

    for row in mastery_summary(profile, chapter_info) {
        lines.push(format!(
            "| {} | {} | {:.2} | {} | {} | {} | {} |",
            row.chapter,
            row.name,
            row.score,
            row.seen,
            row.correct,
            if row.due { "✓" } else { "" },
            row.next_review,
        ));
    }

    let export_dir = data_dir(slug).join("exports");
    fs::create_dir_all(&export_dir).ok();
    let path = export_dir.join(format!("progress_export_{today}.md"));
    match fs::write(&path, lines.join("\n")) {
        Ok(_) => {
            println!("\n  {} Progress exported to:", green("✓"));
            println!("    {}\n", bold(&path.display().to_string()));
        }
        Err(e) => println!("{}", red(&format!("  ✗ Export failed: {e}"))),
    }
    pause();
}

// ── --stats mode ───────────────────────────────────────────────────────────────

fn print_stats(config: &SubjectConfig, questions: &[Question]) {
    let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
    for q in questions {
        *counts.entry(q.chapter()).or_default() += 1;
    }

    // Union of configured chapters and chapters that actually have questions.
    let mut chapters: std::collections::BTreeSet<u32> = counts.keys().copied().collect();
    for idx in config.chapters.keys() {
        chapters.insert(*idx);
    }

    println!("{}", config.title);
    println!("Question bank: {}", config.questions_dir.display());
    println!();
    println!("{:>7}  {:<44}  {:>9}", "chapter", "name", "questions");
    println!("{}", "-".repeat(64));
    let mut total = 0usize;
    for ch in &chapters {
        let n = counts.get(ch).copied().unwrap_or(0);
        total += n;
        let name: String = config.chapter_name(*ch).chars().take(44).collect();
        println!("{:>7}  {:<44}  {:>9}", ch, name, n);
    }
    println!("{}", "-".repeat(64));
    println!("{:>7}  {:<44}  {:>9}", "TOTAL", format!("{} chapters", chapters.len()), total);
}

// ── CLI arg parsing ────────────────────────────────────────────────────────────

struct Args {
    subject: Option<PathBuf>,
    stats: bool,
    help: bool,
}

fn parse_args() -> Args {
    let mut a = Args { subject: None, stats: false, help: false };
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < argv.len() {
        let arg = &argv[i];
        match arg.as_str() {
            "--subject" | "-s" => {
                i += 1;
                a.subject = argv.get(i).map(PathBuf::from);
            }
            "--stats" => a.stats = true,
            "--help" | "-h" => a.help = true,
            other if other.starts_with("--subject=") => {
                a.subject = Some(PathBuf::from(&other["--subject=".len()..]));
            }
            _ => {}
        }
        i += 1;
    }
    a
}

fn print_help() {
    println!("Adaptive Quiz CLI (Library of Babel)\n");
    println!("USAGE:");
    println!("    quiz-cli [--subject <dir>] [--stats]\n");
    println!("OPTIONS:");
    println!("    -s, --subject <dir>   Subject directory containing subject.toml");
    println!("                          (else $QUIZ_SUBJECT, else current directory)");
    println!("        --stats           Print per-chapter question counts and exit");
    println!("    -h, --help            Show this help\n");
    println!("The question bank is loaded from <subject>/questions.");
}

// ── Entry point ────────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();
    if args.help {
        print_help();
        return Ok(());
    }

    // Resolve + load the subject config.
    let config = match SubjectConfig::resolve(args.subject.clone()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", red(&format!("Error: could not load subject: {e}")));
            eprintln!("Pass --subject <dir> (a directory containing subject.toml),");
            eprintln!("set QUIZ_SUBJECT, or run from inside a subject directory.");
            std::process::exit(1);
        }
    };

    let questions = load_question_bank(&config.questions_dir)?;

    // --stats: print and exit.
    if args.stats {
        print_stats(&config, &questions);
        return Ok(());
    }

    let slug = config.slug();
    let dir = profiles_dir(&slug);
    let chapter_info = build_chapter_info(&config);

    println!();
    hr("═");
    println!("\n  {}\n", bold(&magenta(&config.title)));
    hr("═");

    if questions.is_empty() {
        eprintln!(
            "{}",
            yellow(&format!(
                "  Warning: no questions found in '{}'.",
                config.questions_dir.display()
            ))
        );
        eprintln!("  Seed the bank with <chapter>/*.json files, or pass a different --subject.");
    } else {
        println!(
            "  {} questions loaded from {}.",
            dim(&questions.len().to_string()),
            dim(&config.questions_dir.display().to_string())
        );
    }

    let mut profile = choose_or_create_profile(&config, &dir);
    let mut rng = rand::thread_rng();

    loop {
        let due_count = profile.chapters_due_for_review().len();
        let flagged_count = profile.review_queue.len();

        let choice = match show_main_menu(&config.title, &profile.name, due_count, flagged_count) {
            None => {
                println!("\n  {}\n", dim("See you next time."));
                break;
            }
            Some(c) => c,
        };

        match choice {
            // 1. Adaptive quiz
            1 => {
                if questions.is_empty() {
                    println!("{}", yellow("  No questions loaded."));
                    pause();
                    continue;
                }
                let cap = (questions.len().min(50)) as u32;
                let default_n = 10_u32.min(cap);
                let n = ask_int(
                    &format!("  How many questions? (1–{cap}, Enter for {default_n}) → "),
                    1, cap, Some(default_n),
                ).unwrap_or(default_n) as usize;

                let selected = select_questions(
                    &questions, &profile, n, &Scope::Adaptive, &chapter_info, &mut rng,
                );
                if selected.is_empty() {
                    println!("{}", yellow("  No questions available."));
                    pause();
                    continue;
                }
                if let Some(result) = run_session(&config, selected, &mut profile, &dir, "Adaptive") {
                    show_session_summary(&config, &result);
                }
            }

            // 2. Custom quiz
            2 => {
                if questions.is_empty() {
                    println!("{}", yellow("  No questions loaded."));
                    pause();
                    continue;
                }
                let scope = match show_scope_menu(&config, &questions, due_count, flagged_count) {
                    None => continue,
                    Some(Scope::Review(_)) => Scope::Review(profile.review_queue.clone()),
                    Some(s) => s,
                };
                let scope_label = scope.label(|c| config.chapter_name(c), |p| config.phase_name(p));

                let selected = select_questions(&questions, &profile, 50, &scope, &chapter_info, &mut rng);
                if selected.is_empty() {
                    println!("{}", yellow("  No questions match that scope."));
                    pause();
                    continue;
                }

                let cap = (selected.len().min(50)) as u32;
                let default_n = 10_u32.min(cap);
                let n = ask_int(
                    &format!("  How many questions? (1–{cap}, Enter for {default_n}) → "),
                    1, cap, Some(default_n),
                ).unwrap_or(default_n) as usize;

                let picked: Vec<Question> = selected.into_iter().take(n).collect();
                if let Some(result) = run_session(&config, picked, &mut profile, &dir, &scope_label) {
                    show_session_summary(&config, &result);
                }
            }

            // 3. Study a chapter
            3 => {
                if questions.is_empty() {
                    println!("{}", yellow("  No questions loaded."));
                    pause();
                    continue;
                }
                let chapters = chapters_with_questions(&config, &questions);
                if chapters.is_empty() {
                    println!("{}", yellow("  No chapters available."));
                    pause();
                    continue;
                }
                println!();
                hr("─");
                section("Choose a chapter to study");
                for (i, (idx, name)) in chapters.iter().enumerate() {
                    println!("  [{:2}] Ch.{idx} — {name}", i + 1);
                }
                hr("─");
                let n = match ask_int("  Chapter number → ", 1, chapters.len() as u32, Some(1)) {
                    None => continue,
                    Some(n) => n as usize,
                };
                let chapter = chapters[n - 1].0;

                if show_study_mode(&config, &questions, chapter) {
                    let cap = questions.iter().filter(|q| q.chapter() == chapter).count().min(50) as u32;
                    let default_n = 5_u32.min(cap);
                    let n = ask_int(
                        &format!("  How many questions? (1–{cap}, Enter for {default_n}) → "),
                        1, cap, Some(default_n),
                    ).unwrap_or(default_n) as usize;

                    let selected = select_questions(
                        &questions, &profile, n, &Scope::Chapter(chapter), &chapter_info, &mut rng,
                    );
                    let label = format!("Study: {}", chapter_label(&config, chapter));
                    if let Some(result) = run_session(&config, selected, &mut profile, &dir, &label) {
                        show_session_summary(&config, &result);
                    }
                }
            }

            // 4. Mastery dashboard
            4 => show_dashboard(&profile, &chapter_info),

            // 5. Export progress
            5 => export_progress(&config, &profile, &chapter_info, &slug),

            // 6. Switch profile
            6 => profile = choose_or_create_profile(&config, &dir),

            // 7. Quit
            _ => {
                println!("\n  {}\n", dim("See you next time."));
                break;
            }
        }
    }

    Ok(())
}
