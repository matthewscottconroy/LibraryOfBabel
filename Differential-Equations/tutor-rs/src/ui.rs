use crossterm::style::{Color, SetForegroundColor, ResetColor, SetAttribute, Attribute};
use crossterm::execute;
use std::io::{self, Write};

use crate::adaptive::{elo_to_pct, elo_to_difficulty};
use crate::db::TopicStats;
use crate::problems::types::Topic;

const STARS: [&str; 5] = ["★☆☆☆☆", "★★☆☆☆", "★★★☆☆", "★★★★☆", "★★★★★"];
const DIFF_COLORS: [Color; 5] = [Color::Green, Color::Cyan, Color::Yellow, Color::DarkYellow, Color::Red];

// ── Helpers ───────────────────────────────────────────────────────────────────

fn print_color(text: &str, color: Color) {
    let mut stdout = io::stdout();
    let _ = execute!(stdout, SetForegroundColor(color));
    print!("{}", text);
    let _ = execute!(stdout, ResetColor);
}

fn print_bold(text: &str) {
    let mut stdout = io::stdout();
    let _ = execute!(stdout, SetAttribute(Attribute::Bold));
    print!("{}", text);
    let _ = execute!(stdout, SetAttribute(Attribute::Reset));
}

fn print_dim(text: &str) {
    let mut stdout = io::stdout();
    let _ = execute!(stdout, SetAttribute(Attribute::Dim));
    print!("{}", text);
    let _ = execute!(stdout, SetAttribute(Attribute::Reset));
}

fn rule(label: &str) {
    let width: usize = 78;
    let pad = (width.saturating_sub(label.len() + 2)) / 2;
    let line: String = "─".repeat(pad);
    print_color(&format!("\n{} {} {}\n", line, label, line), Color::Blue);
}

fn panel(text: &str, border_color: Color) {
    let width = 72;
    let top = format!("  ╭{}╮", "─".repeat(width));
    let bot = format!("  ╰{}╯", "─".repeat(width));
    print_color(&format!("{}\n", top), border_color);
    for line in text.lines() {
        print!("  │ ");
        print!("{:<width$}", line, width = width - 1);
        print_color("│\n", border_color);
    }
    print_color(&format!("{}\n", bot), border_color);
}

// ── Banner ────────────────────────────────────────────────────────────────────

pub fn banner() {
    println!();
    panel("  Math Tutor\n  Adaptive Mathematics Problem Trainer", Color::Blue);
    println!();
}

// ── Active topics list ────────────────────────────────────────────────────────

pub fn show_active_topics(mastery: &[crate::db::MasteryRow], unlocked: &[Topic]) {
    print_bold("  Active topics:\n");
    for t in unlocked {
        let elo = mastery.iter().find(|m| m.topic == t.as_str())
            .map(|m| m.elo).unwrap_or(400.0);
        let pct = elo_to_pct(elo);
        let filled = (pct / 10.0) as usize;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(10 - filled));
        println!("    {:<38} {} {:3.0}%", t.label(), bar, pct);
    }
    println!();
}

// ── Session header ────────────────────────────────────────────────────────────

pub fn session_header(
    num: u32, total: u32,
    topic: &Topic, subtopic: &str,
    difficulty: u8, streak: u32,
) {
    let stars = STARS[(difficulty as usize).saturating_sub(1).min(4)];
    let color = DIFF_COLORS[(difficulty as usize).saturating_sub(1).min(4)];
    let streak_part = if streak >= 3 { format!("  🔥×{}", streak) } else { String::new() };
    let sub = subtopic.replace('_', " ");

    println!();
    let label = format!(
        " Problem {}/{}{} — {} > {}  {} ",
        num, total, streak_part, topic.label(), sub, stars
    );
    let width = 78usize.saturating_sub(label.len());
    let dashes = "─".repeat(width / 2);
    print_color(&format!("{}{}{}\n", dashes, label, dashes), color);
    println!();
}

// ── Problem display ───────────────────────────────────────────────────────────

pub fn show_problem(question: &str, problem_type: &str, choices: Option<&Vec<String>>) {
    panel(question, Color::DarkBlue);

    if problem_type == "multiple_choice" {
        if let Some(cs) = choices {
            println!();
            for (i, ch) in cs.iter().enumerate() {
                let letter = (b'A' + i as u8) as char;
                print!("  ");
                print_bold(&format!("{}.", letter));
                println!(" {}", ch);
            }
        }
    }

    println!();
    let hint = match problem_type {
        "true_false"      => "Enter: True or False",
        "multiple_choice" => "Enter the letter (A, B, C ...) or full text",
        _                 => "Use math syntax: x^2, exp(x), sqrt(x), ln(x), C1, C2",
    };
    print_dim(&format!("  {}\n", hint));
    println!();
}

// ── Input ─────────────────────────────────────────────────────────────────────

pub fn get_answer() -> String {
    print_dim("  Commands: hint  skip  quit\n");
    print_bold("  Your answer: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    let s = buf.trim().to_string();
    if s.is_empty() { "quit".to_string() } else { s }
}

// ── Feedback ──────────────────────────────────────────────────────────────────

pub fn show_correct(elo_delta: f64, streak: u32, hint_used: bool) {
    let sign = if elo_delta >= 0.0 { "+" } else { "" };
    print!("\n  ");
    print_color("✓ Correct!", Color::Green);
    print_dim(&format!("  ELO {}{:.0}", sign, elo_delta));
    if hint_used { print_dim("  (hint used — half credit)"); }
    if streak >= 3 {
        print!("  ");
        print_color(&format!("🔥 {} in a row!", streak), Color::Yellow);
    }
    println!();
}

pub fn show_incorrect(correct_answer: &str, elo_delta: f64) {
    print!("\n  ");
    print_color("✗ Incorrect.", Color::Red);
    print_dim(&format!("  ELO {:.0}\n", elo_delta));
    print!("  ");
    print_dim("Correct answer: ");
    print_bold(correct_answer);
    println!();
}

pub fn show_explanation(text: &str) {
    println!();
    panel(&format!("Explanation\n\n{}", text), Color::DarkBlue);
}

pub fn show_hint(text: &str) {
    println!();
    panel(&format!("Hint: {}", text), Color::Yellow);
}

pub fn show_skip() {
    print_dim("  Skipped.\n");
}

pub fn show_parse_error(msg: &str) {
    print!("  ");
    print_color(&format!("{}\n", msg), Color::Yellow);
}

// ── Unlock notification ───────────────────────────────────────────────────────

pub fn show_unlock(topic: &Topic) {
    println!();
    panel(&format!("🔓 New topic unlocked: {}", topic.label()), Color::Yellow);
}

// ── Session summary ───────────────────────────────────────────────────────────

pub fn session_summary(correct: u32, total: u32, elapsed_secs: f64, elo_changes: &[(String, f64)]) {
    println!();
    rule(" Session Complete ");
    println!();

    let pct = if total > 0 { correct as f64 / total as f64 * 100.0 } else { 0.0 };
    let mins = (elapsed_secs / 60.0) as u64;
    let secs = (elapsed_secs % 60.0) as u64;
    let color = if pct >= 70.0 { Color::Green } else if pct >= 50.0 { Color::Yellow } else { Color::Red };

    println!("  Problems:  {}", total);
    print!("  Correct:   ");
    print_color(&format!("{} ({:.0}%)\n", correct, pct), color);
    println!("  Time:      {}m {:02}s\n", mins, secs);

    if !elo_changes.is_empty() {
        print_bold("  ELO changes this session:\n");
        for (topic_str, delta) in elo_changes {
            if let Some(t) = Topic::from_str(topic_str) {
                let sign = if *delta >= 0.0 { "+" } else { "" };
                let col = if *delta >= 0.0 { Color::Green } else { Color::Red };
                print!("    {:<38} ", t.label());
                print_color(&format!("{}{:.0}\n", sign, delta), col);
            }
        }
    }
    println!();
}

// ── Stats table ───────────────────────────────────────────────────────────────

pub fn stats_table(stats: &[TopicStats], unlocked: &[Topic], session_count: i64) {
    println!();
    println!("  Mastery Dashboard  (sessions: {})\n", session_count);

    // Header
    println!("  {:<38} {:<8} {:<12} {:<6} {:<10} {:<8}",
        "Topic", "ELO", "Level", "Seen", "Accuracy", "Streak");
    println!("  {}", "─".repeat(85));

    for stat in stats {
        let t = match Topic::from_str(&stat.topic) { Some(t) => t, None => continue };
        let locked = !unlocked.contains(&t);
        let diff = elo_to_difficulty(stat.elo);
        let diff_idx = (diff as usize).saturating_sub(1).min(4);
        let stars = STARS[diff_idx];
        let color = if locked { Color::DarkGrey } else { DIFF_COLORS[diff_idx] };
        let pct = elo_to_pct(stat.elo);
        let bar_filled = (pct / 10.0) as usize;
        let bar = format!("{}{}", "█".repeat(bar_filled), "░".repeat(10 - bar_filled));
        let acc = if stat.seen > 0 {
            format!("{:.0}%", stat.correct as f64 / stat.seen as f64 * 100.0)
        } else {
            "—".into()
        };
        let streak_str = if stat.best_streak > 0 {
            format!("{} (best: {})", stat.streak, stat.best_streak)
        } else {
            "—".into()
        };
        let label = if locked { format!("🔒 {}", t.label()) } else { t.label().to_string() };

        print!("  {:<38} ", label);
        print_color(&format!("{:<8.0} ", stat.elo), color);
        print_color(&format!("{} {:<8} ", stars, bar), color);
        print!("{:<6} {:<10} {}\n", stat.seen, acc, streak_str);
    }
    println!();
}

// ── Ask show dashboard ────────────────────────────────────────────────────────

pub fn ask_show_dashboard() -> bool {
    print!("  Show mastery dashboard? [y/N]: ");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap_or(0);
    matches!(buf.trim().to_lowercase().as_str(), "y" | "yes")
}
