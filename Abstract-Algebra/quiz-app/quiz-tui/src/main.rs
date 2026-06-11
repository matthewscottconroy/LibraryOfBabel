use std::{env, io, path::PathBuf, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};

use quiz_core::{load_quiz, question::Question, Session};

// ── App state ─────────────────────────────────────────────────────────────────

enum Screen {
    Question,
    Feedback { was_correct: bool, question: Question },
    Results,
}

struct App {
    session: Session,
    title: String,
    screen: Screen,
    /// Highlighted choice index (for MC and TrueFalse questions).
    selected: usize,
    /// Accumulated text for FillBlank/Proof questions.
    input: String,
    should_quit: bool,
}

impl App {
    fn new(session: Session, title: String) -> Self {
        Self {
            session,
            title,
            screen: Screen::Question,
            selected: 0,
            input: String::new(),
            should_quit: false,
        }
    }

    fn handle_event(&mut self, key: crossterm::event::KeyEvent) {
        // Global quit bindings
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
                return;
            }
            _ => {}
        }

        match &self.screen {
            Screen::Results => {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter) {
                    self.should_quit = true;
                }
            }

            Screen::Feedback { .. } => {
                if matches!(key.code, KeyCode::Enter | KeyCode::Char(' ')) {
                    if self.session.is_finished() {
                        self.screen = Screen::Results;
                    } else {
                        self.screen = Screen::Question;
                        self.selected = 0;
                        self.input.clear();
                    }
                }
            }

            Screen::Question => {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                    _ => self.handle_question_input(key),
                }
            }
        }
    }

    fn handle_question_input(&mut self, key: crossterm::event::KeyEvent) {
        let is_text_input = matches!(
            self.session.current_question(),
            Some(Question::FillBlank { .. } | Question::Proof { .. })
        );

        if is_text_input {
            match key.code {
                KeyCode::Enter => {
                    if !self.input.is_empty() {
                        let q_idx = self.session.current;
                        let q = self.session.questions[q_idx].clone();
                        let was_correct = self.session.submit(self.input.clone()).unwrap_or(false);
                        self.input.clear();
                        self.screen = Screen::Feedback { was_correct, question: q };
                    }
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Char(c) => {
                    self.input.push(c);
                }
                _ => {}
            }
            return;
        }

        // MC / TrueFalse
        let n_choices = self
            .session
            .current_question()
            .map(|q| q.choices().len())
            .unwrap_or(2);

        let is_tf = matches!(
            self.session.current_question(),
            Some(Question::TrueFalse { .. })
        );

        match key.code {
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected + 1 < n_choices {
                    self.selected += 1;
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Char('1') => {
                self.selected = 0;
            }
            KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Char('2') if n_choices > 1 => {
                self.selected = 1;
            }
            KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Char('3') if n_choices > 2 => {
                self.selected = 2;
            }
            KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Char('4') if n_choices > 3 => {
                self.selected = 3;
            }
            KeyCode::Char('t') | KeyCode::Char('T') if is_tf => {
                self.selected = 0;
            }
            KeyCode::Char('f') | KeyCode::Char('F') if is_tf => {
                self.selected = 1;
            }
            KeyCode::Enter => {
                let q_idx = self.session.current;
                let q = self.session.questions[q_idx].clone();
                let answer = if is_tf {
                    if self.selected == 0 { "true".to_string() } else { "false".to_string() }
                } else {
                    let labels = ["A", "B", "C", "D"];
                    labels.get(self.selected).copied().unwrap_or("A").to_string()
                };
                let was_correct = self.session.submit(answer).unwrap_or(false);
                self.selected = 0;
                self.screen = Screen::Feedback { was_correct, question: q };
            }
            _ => {}
        }
    }
}

// ── Drawing ───────────────────────────────────────────────────────────────────

fn ui(frame: &mut Frame, app: &App) {
    match &app.screen {
        Screen::Question => draw_question(frame, app),
        Screen::Feedback { was_correct, question } => {
            draw_feedback(frame, *was_correct, question)
        }
        Screen::Results => draw_results(frame, &app.session),
    }
}

fn draw_question(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // progress / title
            Constraint::Length(7), // question text
            Constraint::Min(6),    // choices or input
            Constraint::Length(1), // key hints
        ])
        .split(area);

    let (answered, total) = app.session.progress();
    let q_num = app.session.current + 1;
    let ratio = if total > 0 { answered as f64 / total as f64 } else { 0.0 };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.title)),
        )
        .gauge_style(Style::default().fg(Color::Green).bg(Color::DarkGray))
        .ratio(ratio)
        .label(format!("Q {q_num}/{total}"));
    frame.render_widget(gauge, outer[0]);

    let q = match app.session.current_question() {
        Some(q) => q,
        None => return,
    };

    let q_para = Paragraph::new(q.text())
        .block(Block::default().borders(Borders::ALL).title(" Question "))
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));
    frame.render_widget(q_para, outer[1]);

    // Choices or text input: FillBlank/Proof use free text entry
    let is_text_input = matches!(q, Question::FillBlank { .. } | Question::Proof { .. });
    let choices = if is_text_input { &[][..] } else { q.choices() };
    if choices.is_empty() {
        let display = format!("{}\u{2588}", app.input); // block cursor at end
        let input_widget = Paragraph::new(display.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Your answer — [Enter] submit  [Backspace] delete "),
            )
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(input_widget, outer[2]);

        // Position the real cursor just after the typed text
        let cursor_x = outer[2].x + 1 + app.input.len() as u16;
        let cursor_y = outer[2].y + 1;
        if cursor_x < outer[2].right().saturating_sub(1) {
            frame.set_cursor_position((cursor_x, cursor_y));
        }
    } else {
        let labels = ['A', 'B', 'C', 'D'];
        let items: Vec<ListItem> = choices
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let label = labels.get(i).copied().unwrap_or('?');
                ListItem::new(format!(" {label}. {c} "))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Choices — [↑↓/ABCD] navigate  [Enter] confirm "),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("► ");

        let mut state = ListState::default();
        state.select(Some(app.selected));
        frame.render_stateful_widget(list, outer[2], &mut state);
    }

    let hints = if choices.is_empty() {
        "[Enter] Submit  [Backspace] Delete  [Esc/q] Quit"
    } else {
        "[↑↓] Navigate  [A-D] Jump  [Enter] Confirm  [Esc/q] Quit"
    };
    let footer = Paragraph::new(hints)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    frame.render_widget(footer, outer[3]);
}

fn draw_feedback(frame: &mut Frame, was_correct: bool, question: &Question) {
    let area = frame.area();

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Min(12),
            Constraint::Percentage(20),
        ])
        .split(area);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Min(40),
            Constraint::Percentage(10),
        ])
        .split(vert[1]);

    let popup = horiz[1];

    let (icon, color, title) = if was_correct {
        ("✓  Correct!", Color::Green, " ✓ Correct! ")
    } else {
        ("✗  Incorrect", Color::Red, " ✗ Incorrect ")
    };

    let mut lines = vec![
        Line::from(Span::styled(
            icon,
            Style::default()
                .fg(color)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    if !was_correct {
        lines.push(Line::from(Span::styled(
            format!("Correct answer: {}", question.correct_display()),
            Style::default().fg(Color::Yellow),
        )));
        lines.push(Line::from(""));
    }

    let exp = question.explanation();
    if !exp.is_empty() {
        lines.push(Line::from(Span::styled(
            exp,
            Style::default().fg(Color::Gray),
        )));
        lines.push(Line::from(""));
    }

    lines.push(Line::from(Span::styled(
        "[Enter / Space] Continue",
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color))
                .title(title),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(para, popup);
}

fn draw_results(frame: &mut Frame, session: &Session) {
    let area = frame.area();

    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Min(14),
            Constraint::Percentage(15),
        ])
        .split(area);

    let horiz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Min(50),
            Constraint::Percentage(10),
        ])
        .split(vert[1]);

    let popup = horiz[1];

    let (correct, total) = session.score();
    let pct = if total > 0 {
        (correct as f64 / total as f64 * 100.0).round() as u32
    } else {
        0
    };

    let (color, grade) = match pct {
        90..=100 => (Color::Green, "Excellent! 🏆"),
        70..=89  => (Color::LightGreen, "Good work! 👍"),
        50..=69  => (Color::Yellow, "Keep studying 📚"),
        _        => (Color::Red, "Needs practice 🔄"),
    };

    let bar_width: usize = 28;
    let filled = (bar_width * pct as usize) / 100;
    let bar = format!(
        "[{}{}]",
        "█".repeat(filled),
        "░".repeat(bar_width - filled)
    );

    let mut lines = vec![
        Line::from(Span::styled(
            format!("Score: {correct}/{total}  ({pct}%)"),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(bar, Style::default().fg(color))),
        Line::from(Span::styled(grade, Style::default().fg(Color::White))),
        Line::from(""),
        Line::from(Span::styled(
            "Per-question breakdown:",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::UNDERLINED),
        )),
    ];

    for (i, (q, ans)) in session
        .questions
        .iter()
        .zip(session.answers.iter())
        .enumerate()
    {
        let ok = ans.as_deref().map(|a| q.check(a)).unwrap_or(false);
        let (tick, c) = if ok { ("✓", Color::Green) } else { ("✗", Color::Red) };
        let text = q.text();
        let short: &str = if text.len() > 44 { &text[..44] } else { text };
        lines.push(Line::from(Span::styled(
            format!("  Q{:>2}. {tick}  {short}…", i + 1),
            Style::default().fg(c),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "[Enter / q] Quit",
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(Text::from(lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(color))
                .title(" Quiz Complete! "),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(para, popup);
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "questions/sample.json".to_string());

    let quiz = load_quiz(PathBuf::from(&path))?;
    let title = quiz.title.clone();
    let session = Session::new(quiz.questions, &title);
    let mut app = App::new(session, title);

    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &mut app);

    // Always restore the terminal even on error
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| ui(frame, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_event(key);
            }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
