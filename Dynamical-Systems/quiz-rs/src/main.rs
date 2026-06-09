use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    env,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Clone)]
struct Question {
    ch: u32,
    part: u32,
    #[serde(rename = "chTitle")]
    ch_title: String,
    q: String,
    opts: Vec<String>,
    ans: usize,
    exp: String,
}

struct QuizResult {
    question: Question,
    chosen: Option<usize>,
}

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, cursor::Show);
    }
}

fn read_key() -> io::Result<KeyCode> {
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                drop(RawModeGuard); // triggers disable_raw_mode
                std::process::exit(0);
            }
            return Ok(key.code);
        }
    }
}

fn clear_screen(stdout: &mut impl Write) -> io::Result<()> {
    queue!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
}

fn print_header(stdout: &mut impl Write, title: &str) -> io::Result<()> {
    queue!(
        stdout,
        SetForegroundColor(Color::Cyan),
        SetAttribute(Attribute::Bold),
        Print(format!("  {}\n", title)),
        ResetColor,
        Print("  "),
        SetForegroundColor(Color::DarkGrey),
        Print("─".repeat(60)),
        ResetColor,
        Print("\n\n"),
    )
}

fn collect_json(dir: &Path, out: &mut Vec<Question>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            collect_json(&p, out);
        } else if p.extension().map(|e| e == "json").unwrap_or(false) {
            if let Ok(s) = fs::read_to_string(&p) {
                if let Ok(q) = serde_json::from_str::<Question>(&s) {
                    out.push(q);
                }
            }
        }
    }
}

fn load_questions(questions_dir: &Path) -> Vec<Question> {
    let mut qs = Vec::new();
    collect_json(questions_dir, &mut qs);
    qs
}

fn find_questions_dir() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("quiz/questions"),
        PathBuf::from("../quiz/questions"),
        PathBuf::from("questions"),
    ];
    for c in &candidates {
        if c.is_dir() {
            return Some(c.clone());
        }
    }
    None
}

// Scrollable selection list. Returns Some(index) or None (Esc/back).
fn select_list(
    stdout: &mut impl Write,
    title: &str,
    items: &[String],
    page_size: usize,
) -> io::Result<Option<usize>> {
    let mut cursor_pos = 0usize;
    let mut scroll = 0usize;

    loop {
        clear_screen(stdout)?;
        print_header(stdout, title)?;

        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("  ↑/↓ navigate · Enter select · Esc back\n\n"),
            ResetColor,
        )?;

        let visible = page_size.min(items.len());
        for i in scroll..scroll + visible {
            if i >= items.len() {
                break;
            }
            if i == cursor_pos {
                queue!(
                    stdout,
                    SetForegroundColor(Color::Yellow),
                    SetAttribute(Attribute::Bold),
                    Print(format!("  ▶ {}\n", items[i])),
                    ResetColor,
                )?;
            } else {
                queue!(
                    stdout,
                    Print(format!("    {}\n", items[i])),
                )?;
            }
        }

        if items.len() > page_size {
            queue!(
                stdout,
                Print("\n"),
                SetForegroundColor(Color::DarkGrey),
                Print(format!("  [{}/{}]\n", cursor_pos + 1, items.len())),
                ResetColor,
            )?;
        }

        stdout.flush()?;

        match read_key()? {
            KeyCode::Up | KeyCode::Char('k') => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                    if cursor_pos < scroll {
                        scroll = cursor_pos;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if cursor_pos + 1 < items.len() {
                    cursor_pos += 1;
                    if cursor_pos >= scroll + page_size {
                        scroll = cursor_pos + 1 - page_size;
                    }
                }
            }
            KeyCode::PageUp => {
                cursor_pos = cursor_pos.saturating_sub(page_size);
                if cursor_pos < scroll {
                    scroll = cursor_pos;
                }
            }
            KeyCode::PageDown => {
                cursor_pos = (cursor_pos + page_size).min(items.len().saturating_sub(1));
                if cursor_pos >= scroll + page_size {
                    scroll = cursor_pos + 1 - page_size;
                }
            }
            KeyCode::Enter => return Ok(Some(cursor_pos)),
            KeyCode::Esc | KeyCode::Char('q') => return Ok(None),
            _ => {}
        }
    }
}

fn part_menu(stdout: &mut impl Write, all_questions: &[Question]) -> io::Result<Option<Vec<Question>>> {
    let mut parts: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for q in all_questions {
        parts.entry(q.part).or_default();
    }

    // Build part labels from the questions
    let part_labels: Vec<(u32, String)> = {
        let part_names = [
            (1u32, "Part I: Foundations of Dynamical Systems"),
            (2, "Part II: Ergodic Theory and Chaos"),
            (3, "Part III: Topological Dynamics"),
            (4, "Part IV: Bridges and Applications"),
            (5, "Part V: Information Theory Foundations"),
            (6, "Part VI: Frontiers"),
            (7, "Part VII: Connections and Open Problems"),
        ];
        part_names
            .iter()
            .filter(|(p, _)| parts.contains_key(p))
            .map(|(p, n)| (*p, n.to_string()))
            .collect()
    };

    let labels: Vec<String> = part_labels.iter().map(|(_, n)| n.clone()).collect();
    match select_list(stdout, "Dynamical Systems Quiz — Select Part", &labels, 20)? {
        None => Ok(None),
        Some(idx) => {
            let part_num = part_labels[idx].0;
            let qs: Vec<Question> = all_questions
                .iter()
                .filter(|q| q.part == part_num)
                .cloned()
                .collect();
            Ok(Some(qs))
        }
    }
}

fn chapter_menu(stdout: &mut impl Write, all_questions: &[Question]) -> io::Result<Option<Vec<Question>>> {
    let mut chapters: BTreeMap<u32, String> = BTreeMap::new();
    for q in all_questions {
        chapters.entry(q.ch).or_insert_with(|| q.ch_title.clone());
    }

    let chapter_list: Vec<(u32, String)> = chapters
        .into_iter()
        .map(|(ch, title)| (ch, format!("Ch {:02}: {}", ch, title)))
        .collect();

    let labels: Vec<String> = chapter_list.iter().map(|(_, l)| l.clone()).collect();

    let (_cols, rows) = terminal::size().unwrap_or((80, 24));
    let page_size = (rows as usize).saturating_sub(8).max(5);

    match select_list(stdout, "Dynamical Systems Quiz — Select Chapter", &labels, page_size)? {
        None => Ok(None),
        Some(idx) => {
            let ch_num = chapter_list[idx].0;
            let qs: Vec<Question> = all_questions
                .iter()
                .filter(|q| q.ch == ch_num)
                .cloned()
                .collect();
            Ok(Some(qs))
        }
    }
}

fn wrap_text(text: &str, width: usize, indent: usize) -> String {
    let indent_str = " ".repeat(indent);
    let mut result = String::new();
    for line in text.split('\n') {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() {
            result.push('\n');
            continue;
        }
        let mut current_line = indent_str.clone();
        let mut line_len = indent;
        for word in words {
            if line_len + word.len() + 1 > width && line_len > indent {
                result.push('\n');
                current_line = indent_str.clone();
                line_len = indent;
            }
            if line_len > indent {
                current_line.push(' ');
                line_len += 1;
            }
            current_line.push_str(word);
            line_len += word.len();
        }
        result.push_str(&current_line);
        result.push('\n');
    }
    result
}

// Run a quiz with the given questions. Returns (results, retry_requested).
fn run_quiz(
    stdout: &mut impl Write,
    questions: &mut Vec<Question>,
    rng: &mut impl rand::Rng,
) -> io::Result<(Vec<QuizResult>, bool)> {
    questions.shuffle(rng);
    let total = questions.len();
    let mut results: Vec<QuizResult> = Vec::new();

    let (cols, _) = terminal::size().unwrap_or((80, 24));
    let width = (cols as usize).min(80);

    for (i, question) in questions.iter().enumerate() {
        clear_screen(stdout)?;

        // Header
        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            SetAttribute(Attribute::Bold),
            Print(format!(
                "  Question {} of {}",
                i + 1,
                total
            )),
            ResetColor,
        )?;

        // Progress bar
        let bar_width = width.saturating_sub(4);
        let filled = (bar_width * (i + 1)) / total;
        queue!(
            stdout,
            Print("    "),
            SetForegroundColor(Color::DarkGrey),
            Print("["),
            SetForegroundColor(Color::Green),
            Print("█".repeat(filled)),
            SetForegroundColor(Color::DarkGrey),
            Print("░".repeat(bar_width - filled)),
            Print("]\n\n"),
            ResetColor,
        )?;

        // Chapter badge
        queue!(
            stdout,
            SetForegroundColor(Color::Magenta),
            Print(format!("  Ch {:02}: {}\n\n", question.ch, question.ch_title)),
            ResetColor,
        )?;

        // Question text
        let wrapped_q = wrap_text(&question.q, width.saturating_sub(4), 2);
        queue!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print(&wrapped_q),
            ResetColor,
            Print("\n"),
        )?;

        // Options
        let labels = ['A', 'B', 'C', 'D'];
        for (j, opt) in question.opts.iter().enumerate() {
            let label = labels.get(j).copied().unwrap_or('?');
            let opt_text = format!("{}. {}", label, opt);
            let wrapped = wrap_text(&opt_text, width.saturating_sub(6), 4);
            queue!(stdout, Print(&wrapped))?;
        }

        queue!(
            stdout,
            Print("\n"),
            SetForegroundColor(Color::DarkGrey),
            Print("  [A/B/C/D] answer · [S] skip · [Q] quit\n"),
            ResetColor,
        )?;
        stdout.flush()?;

        // Wait for input
        let chosen: Option<usize> = loop {
            match read_key()? {
                KeyCode::Char('a') | KeyCode::Char('A') => break Some(0),
                KeyCode::Char('b') | KeyCode::Char('B') => break Some(1),
                KeyCode::Char('c') | KeyCode::Char('C') => break Some(2),
                KeyCode::Char('d') | KeyCode::Char('D') => break Some(3),
                KeyCode::Char('s') | KeyCode::Char('S') => break None,
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    // Push remaining as skipped and go to results
                    results.push(QuizResult {
                        question: question.clone(),
                        chosen: None,
                    });
                    for remaining in questions.iter().skip(i + 1) {
                        results.push(QuizResult {
                            question: remaining.clone(),
                            chosen: None,
                        });
                    }
                    return Ok((results, false));
                }
                _ => {}
            }
        };

        // Show result
        clear_screen(stdout)?;

        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            SetAttribute(Attribute::Bold),
            Print(format!("  Question {} of {}\n\n", i + 1, total)),
            ResetColor,
            SetForegroundColor(Color::Magenta),
            Print(format!("  Ch {:02}: {}\n\n", question.ch, question.ch_title)),
            ResetColor,
            SetAttribute(Attribute::Bold),
        )?;
        let wrapped_q = wrap_text(&question.q, width.saturating_sub(4), 2);
        queue!(stdout, Print(&wrapped_q), ResetColor, Print("\n"))?;

        // Show options with color highlighting
        for (j, opt) in question.opts.iter().enumerate() {
            let label = labels.get(j).copied().unwrap_or('?');
            let is_correct = j == question.ans;
            let is_chosen = chosen.map(|c| c == j).unwrap_or(false);

            if is_correct {
                queue!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    SetAttribute(Attribute::Bold),
                )?;
            } else if is_chosen && !is_correct {
                queue!(
                    stdout,
                    SetForegroundColor(Color::Red),
                )?;
            } else {
                queue!(stdout, SetForegroundColor(Color::DarkGrey))?;
            }

            let marker = if is_correct {
                "✓"
            } else if is_chosen {
                "✗"
            } else {
                " "
            };
            let opt_text = format!("{} {}. {}", marker, label, opt);
            let wrapped = wrap_text(&opt_text, width.saturating_sub(6), 4);
            queue!(stdout, Print(&wrapped), ResetColor)?;
        }
        queue!(stdout, Print("\n"))?;

        // Verdict
        let correct = chosen.map(|c| c == question.ans).unwrap_or(false);
        if chosen.is_none() {
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print("  Skipped\n\n"),
                ResetColor,
            )?;
        } else if correct {
            queue!(
                stdout,
                SetForegroundColor(Color::Green),
                SetAttribute(Attribute::Bold),
                Print("  Correct!\n\n"),
                ResetColor,
            )?;
        } else {
            let correct_label = labels.get(question.ans).copied().unwrap_or('?');
            queue!(
                stdout,
                SetForegroundColor(Color::Red),
                SetAttribute(Attribute::Bold),
                Print(format!(
                    "  Wrong. Correct answer: {}\n\n",
                    correct_label
                )),
                ResetColor,
            )?;
        }

        // Explanation
        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("  Explanation:\n"),
            ResetColor,
        )?;
        let wrapped_exp = wrap_text(&question.exp, width.saturating_sub(4), 2);
        queue!(stdout, Print(&wrapped_exp), Print("\n"))?;

        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("  [Enter/Space] continue\n"),
            ResetColor,
        )?;
        stdout.flush()?;

        loop {
            match read_key()? {
                KeyCode::Enter | KeyCode::Char(' ') => break,
                _ => {}
            }
        }

        results.push(QuizResult {
            question: question.clone(),
            chosen,
        });
    }

    Ok((results, false))
}

// Show results screen. Returns true if the user wants to retry.
fn show_results(
    stdout: &mut impl Write,
    results: &[QuizResult],
) -> io::Result<bool> {
    let total = results.len();
    let answered: Vec<&QuizResult> = results
        .iter()
        .filter(|r| r.chosen.is_some())
        .collect();
    let correct = answered
        .iter()
        .filter(|r| r.chosen.map(|c| c == r.question.ans).unwrap_or(false))
        .count();
    let attempted = answered.len();
    let skipped = total - attempted;

    let pct = if attempted > 0 {
        (correct * 100) / attempted
    } else {
        0
    };

    let score_color = if pct >= 75 {
        Color::Green
    } else if pct >= 50 {
        Color::Yellow
    } else {
        Color::Red
    };

    clear_screen(stdout)?;
    print_header(stdout, "Quiz Complete")?;

    queue!(
        stdout,
        SetForegroundColor(score_color),
        SetAttribute(Attribute::Bold),
        Print(format!("  {} / {} correct", correct, attempted)),
        ResetColor,
    )?;
    if skipped > 0 {
        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  ({} skipped)", skipped)),
            ResetColor,
        )?;
    }
    queue!(
        stdout,
        Print(format!("  · {}%\n\n", pct)),
    )?;

    // Wrong/skipped list
    let wrong: Vec<&QuizResult> = results
        .iter()
        .filter(|r| r.chosen.map(|c| c != r.question.ans).unwrap_or(true))
        .collect();

    if wrong.is_empty() {
        queue!(
            stdout,
            SetForegroundColor(Color::Green),
            SetAttribute(Attribute::Bold),
            Print("  Perfect score!\n\n"),
            ResetColor,
        )?;
    } else {
        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  {} questions to review:\n\n", wrong.len())),
            ResetColor,
        )?;

        let labels = ['A', 'B', 'C', 'D'];
        let (cols, _) = terminal::size().unwrap_or((80, 24));
        let width = (cols as usize).min(80);

        for (i, r) in wrong.iter().enumerate().take(20) {
            let q_preview: String = r.question.q.chars().take(70).collect();
            let ellipsis = if r.question.q.len() > 70 { "…" } else { "" };
            let correct_label = labels.get(r.question.ans).copied().unwrap_or('?');
            let status = if r.chosen.is_none() {
                "skipped".to_string()
            } else {
                let chosen_label = r.chosen.and_then(|c| labels.get(c).copied()).unwrap_or('?');
                format!("chose {}, correct {}", chosen_label, correct_label)
            };

            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print(format!("  {}. ", i + 1)),
                ResetColor,
                Print(format!("{}{}\n", q_preview, ellipsis)),
                SetForegroundColor(Color::DarkGrey),
                Print(format!("     {}\n", status)),
                ResetColor,
            )?;

            // Show correct answer text
            if let Some(opt) = r.question.opts.get(r.question.ans) {
                let wrapped = wrap_text(opt, width.saturating_sub(9), 5);
                queue!(
                    stdout,
                    SetForegroundColor(Color::Green),
                    Print(format!("     ✓ {}", wrapped.trim())),
                    ResetColor,
                    Print("\n\n"),
                )?;
            }
        }
        if wrong.len() > 20 {
            queue!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print(format!("  … and {} more\n", wrong.len() - 20)),
                ResetColor,
            )?;
        }
    }

    queue!(
        stdout,
        Print("\n"),
        SetForegroundColor(Color::DarkGrey),
        Print("  [R] retry same set · [H] home · [Q] quit\n"),
        ResetColor,
    )?;
    stdout.flush()?;

    loop {
        match read_key()? {
            KeyCode::Char('r') | KeyCode::Char('R') => return Ok(true),
            KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::Esc => return Ok(false),
            KeyCode::Char('q') | KeyCode::Char('Q') => std::process::exit(0),
            _ => {}
        }
    }
}

fn main_menu(
    stdout: &mut impl Write,
    all_questions: &[Question],
    rng: &mut impl rand::Rng,
) -> io::Result<()> {
    loop {
        clear_screen(stdout)?;
        print_header(stdout, "Dynamical Systems & Information Theory — Quiz")?;

        let total = all_questions.len();
        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print(format!("  {} questions · 43 chapters · 7 parts · Research-level\n\n", total)),
            ResetColor,
        )?;

        let menu_items = [
            ("A", "All Topics", format!("All {} questions shuffled", total)),
            ("P", "By Part", "Quiz one of the 7 parts".to_string()),
            ("C", "By Chapter", "Focus on a single chapter (24 questions)".to_string()),
            ("R", "Random 20", "20 random questions from all topics".to_string()),
            ("Q", "Quit", String::new()),
        ];

        for (key, name, desc) in &menu_items {
            queue!(
                stdout,
                SetForegroundColor(Color::Yellow),
                SetAttribute(Attribute::Bold),
                Print(format!("  [{}] ", key)),
                ResetColor,
                SetAttribute(Attribute::Bold),
                Print(name),
                ResetColor,
            )?;
            if !desc.is_empty() {
                queue!(
                    stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print(format!(" — {}", desc)),
                    ResetColor,
                )?;
            }
            queue!(stdout, Print("\n"))?;
        }

        stdout.flush()?;

        let mode = loop {
            match read_key()? {
                KeyCode::Char('a') | KeyCode::Char('A') => break 'a',
                KeyCode::Char('p') | KeyCode::Char('P') => break 'p',
                KeyCode::Char('c') | KeyCode::Char('C') => break 'c',
                KeyCode::Char('r') | KeyCode::Char('R') => break 'r',
                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                _ => {}
            }
        };

        // Get question pool for selected mode
        let pool: Option<Vec<Question>> = match mode {
            'a' => Some(all_questions.to_vec()),
            'p' => part_menu(stdout, all_questions)?,
            'c' => chapter_menu(stdout, all_questions)?,
            'r' => {
                let mut shuffled = all_questions.to_vec();
                shuffled.shuffle(rng);
                Some(shuffled.into_iter().take(20).collect())
            }
            _ => None,
        };

        let Some(mut pool) = pool else {
            continue;
        };

        if pool.is_empty() {
            continue;
        }

        // Quiz loop with retry support
        loop {
            let (results, _) = run_quiz(stdout, &mut pool, rng)?;
            let retry = show_results(stdout, &results)?;
            if !retry {
                break;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let questions_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        match find_questions_dir() {
            Some(d) => d,
            None => {
                eprintln!("Could not find questions directory.");
                eprintln!("Usage: quiz [path/to/questions]");
                std::process::exit(1);
            }
        }
    };

    let all_questions = load_questions(&questions_dir);
    if all_questions.is_empty() {
        eprintln!("No questions found in {:?}", questions_dir);
        std::process::exit(1);
    }

    let mut rng = rand::thread_rng();
    let mut stdout = io::stdout();

    let _guard = match RawModeGuard::new() {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Failed to enable raw mode: {}", e);
            std::process::exit(1);
        }
    };

    execute!(stdout, cursor::Hide).unwrap();

    if let Err(e) = main_menu(&mut stdout, &all_questions, &mut rng) {
        drop(_guard);
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    // Cleanup happens via RawModeGuard Drop
    clear_screen(&mut stdout).unwrap();
    stdout.flush().unwrap();
}
