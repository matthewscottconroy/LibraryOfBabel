use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, ClearType},
};
use rand::seq::SliceRandom;

mod questions;
use questions::QUESTIONS;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = run(&mut stdout);

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    result
}

#[derive(Debug)]
enum Action {
    Next,
    Prev,
    Shuffle,
    Quit,
    Redraw,
}

fn read_action() -> io::Result<Action> {
    loop {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => return Ok(Action::Quit),
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                    return Ok(Action::Quit)
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Right | KeyCode::Enter => {
                    return Ok(Action::Next)
                }
                KeyCode::Char('p') | KeyCode::Char('P') | KeyCode::Left | KeyCode::Backspace => {
                    return Ok(Action::Prev)
                }
                KeyCode::Char('s') | KeyCode::Char('S') => return Ok(Action::Shuffle),
                _ => {} // ignore
            },
            Event::Resize(_, _) => return Ok(Action::Redraw),
            _ => {} // ignore mouse events etc.
        }
    }
}

fn run(stdout: &mut impl Write) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..QUESTIONS.len()).collect();
    let mut current: usize = 0;

    loop {
        let (width, height) = terminal::size()?;
        render(stdout, &indices, current, width as usize, height as usize)?;

        match read_action()? {
            Action::Quit => return Ok(()),
            Action::Next => {
                if current + 1 < indices.len() {
                    current += 1;
                }
            }
            Action::Prev => {
                if current > 0 {
                    current -= 1;
                }
            }
            Action::Shuffle => {
                indices.shuffle(&mut rng);
                current = 0;
            }
            Action::Redraw => {} // just re-render
        }
    }
}

fn word_wrap(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    let mut lines: Vec<String> = Vec::new();
    for paragraph in text.split('\n') {
        let trimmed = paragraph.trim();
        if trimmed.is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current = String::new();
        let mut current_len: usize = 0;
        for word in trimmed.split_whitespace() {
            let word_len = word.chars().count();
            if current.is_empty() {
                current.push_str(word);
                current_len = word_len;
            } else if current_len + 1 + word_len <= width {
                current.push(' ');
                current.push_str(word);
                current_len += 1 + word_len;
            } else {
                lines.push(std::mem::take(&mut current));
                current.push_str(word);
                current_len = word_len;
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    lines
}

fn render(
    stdout: &mut impl Write,
    indices: &[usize],
    current: usize,
    width: usize,
    height: usize,
) -> io::Result<()> {
    let q = &QUESTIONS[indices[current]];
    let total = indices.len();

    // Layout constants
    let margin: usize = 3;
    // Cap text width at 78 so it stays readable on very wide terminals
    let text_width = width.saturating_sub(margin * 2).min(78);
    let pad = " ".repeat(margin);
    // "─" is a single display column; repeat to fill width (capped at 82)
    let divider: String = "─".repeat(width.min(82));

    queue!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    // ── Title bar ─────────────────────────────────────────────────────────────
    let title = "THE PHILOSOPHY OF TIME";
    let subtitle = "Examination";
    let bar_inner = width.saturating_sub(margin * 2);
    let title_gap = bar_inner
        .saturating_sub(title.chars().count() + subtitle.chars().count())
        .min(bar_inner);
    queue!(
        stdout,
        SetAttribute(Attribute::Bold),
        SetForegroundColor(Color::White),
        Print(format!(
            "{}{}{}{}{}\r\n",
            pad,
            title,
            " ".repeat(title_gap),
            subtitle,
            pad
        )),
        SetAttribute(Attribute::Reset),
        ResetColor,
    )?;

    // ── Top divider ───────────────────────────────────────────────────────────
    queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(format!("{}\r\n", divider)),
        ResetColor,
    )?;

    // blank
    queue!(stdout, Print("\r\n"))?;

    // ── Unit / topic + counter ────────────────────────────────────────────────
    let unit_label = format!("{}  ·  {}", q.unit, q.topic);
    let num_label = format!("{} / {}", current + 1, total);
    let label_total = unit_label.chars().count() + num_label.chars().count() + margin * 2;
    let mid_gap = if label_total < width {
        " ".repeat(width - label_total)
    } else {
        String::new()
    };
    queue!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(format!(
            "{}{}{}{}{}\r\n",
            pad, unit_label, mid_gap, num_label, pad
        )),
        ResetColor,
    )?;

    // blank
    queue!(stdout, Print("\r\n"))?;

    // ── Question text ─────────────────────────────────────────────────────────
    for line in word_wrap(q.text, text_width) {
        queue!(
            stdout,
            SetForegroundColor(Color::White),
            Print(format!("{}{}\r\n", pad, line)),
            ResetColor,
        )?;
    }

    // blank
    queue!(stdout, Print("\r\n"))?;

    // ── Reflection prompt ─────────────────────────────────────────────────────
    let prompt_indent = " ".repeat(margin + 3);
    let prompt_width = text_width.saturating_sub(3);
    let prompt_lines = word_wrap(q.prompt, prompt_width);
    if !prompt_lines.is_empty() {
        queue!(
            stdout,
            SetAttribute(Attribute::Dim),
            SetForegroundColor(Color::Cyan),
        )?;
        let mut first = true;
        for line in &prompt_lines {
            if first {
                queue!(stdout, Print(format!("{}▸  {}\r\n", pad, line)))?;
                first = false;
            } else {
                queue!(stdout, Print(format!("{}{}\r\n", prompt_indent, line)))?;
            }
        }
        queue!(stdout, SetAttribute(Attribute::Reset), ResetColor)?;
    }

    // ── Navigation bar pinned to bottom ───────────────────────────────────────
    if height >= 3 {
        queue!(
            stdout,
            cursor::MoveTo(0, height.saturating_sub(2) as u16),
            SetForegroundColor(Color::DarkGrey),
            Print(format!("{}\r\n", divider)),
            ResetColor,
        )?;

        queue!(stdout, Print(pad.as_str()))?;

        // [P] Previous — dim when at start
        if current > 0 {
            queue!(
                stdout,
                SetForegroundColor(Color::White),
                Print("[P] Prev"),
                ResetColor,
            )?;
        } else {
            queue!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("    Prev"),
                ResetColor,
            )?;
        }

        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("   "),
            ResetColor,
        )?;

        // [N] Next — dim when at end
        if current + 1 < total {
            queue!(
                stdout,
                SetForegroundColor(Color::White),
                Print("[N / \u{21b5}] Next"),
                ResetColor,
            )?;
        } else {
            queue!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("          Next"),
                ResetColor,
            )?;
        }

        queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("   [S] Shuffle   [Q] Quit"),
            ResetColor,
        )?;
    }

    stdout.flush()
}
