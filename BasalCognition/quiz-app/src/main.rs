mod questions;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::seq::SliceRandom;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Clear, Gauge, List, ListItem, ListState, Padding, Paragraph,
        Wrap,
    },
    Frame, Terminal,
};
use std::{io, panic};

use questions::{Difficulty, Question, QUESTIONS};

// ── Palette ───────────────────────────────────────────────────────────────────
const TEAL: Color = Color::Rgb(0, 180, 180);
const GOLD: Color = Color::Rgb(220, 180, 60);
const GREEN: Color = Color::Rgb(60, 200, 100);
const RED: Color = Color::Rgb(220, 80, 80);
const LAVENDER: Color = Color::Rgb(170, 130, 220); // Advanced difficulty (not RED)
const MUTED: Color = Color::DarkGray;
const BG: Color = Color::Rgb(16, 20, 28);
const SURFACE: Color = Color::Rgb(24, 30, 40);

const UNIT_LABELS: &[&str] = &[
    "All Units",
    " I    First Principles",
    " II   The Intelligent Cell",
    " III  Bacterial Cognition",
    " IV   Protists & Simple Eukaryotes",
    " V    Plant Cognition",
    " VI   Fungal Cognition",
    " VII  Collective Cognition",
    " VIII Evolution of Nervous Systems",
    " IX   Theoretical Frameworks",
    " X    Extended & Distributed Cognition",
    " XI   Basal Cognition & AI",
    " XII  Philosophy & Ethics",
    " XIII Frontiers",
];

// ── State ─────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Menu,
    Quiz,
    Feedback,
    Review,
    Results,
    Quit,
}

// unit_counts[unit_idx 0-13][diff_idx 0-3]  (0 = All, 1-3 = F/I/A)
type UnitCounts = [[usize; 4]; 14];

fn build_unit_counts() -> UnitCounts {
    let mut c = [[0usize; 4]; 14];
    for q in QUESTIONS {
        let u = q.unit as usize;
        let d = diff_to_idx(Some(q.difficulty));
        c[0][0] += 1;
        c[0][d] += 1;
        c[u][0] += 1;
        c[u][d] += 1;
    }
    c
}

fn diff_to_idx(d: Option<Difficulty>) -> usize {
    match d {
        None => 0,
        Some(Difficulty::Foundational) => 1,
        Some(Difficulty::Intermediate) => 2,
        Some(Difficulty::Advanced) => 3,
    }
}

fn cycle_difficulty(d: Option<Difficulty>) -> Option<Difficulty> {
    match d {
        None => Some(Difficulty::Foundational),
        Some(Difficulty::Foundational) => Some(Difficulty::Intermediate),
        Some(Difficulty::Intermediate) => Some(Difficulty::Advanced),
        Some(Difficulty::Advanced) => None,
    }
}

fn difficulty_color(d: Difficulty) -> Color {
    match d {
        Difficulty::Foundational => GREEN,
        Difficulty::Intermediate => GOLD,
        Difficulty::Advanced => LAVENDER,
    }
}

fn filter_color(d: Option<Difficulty>) -> Color {
    d.map_or(Color::White, difficulty_color)
}


struct App {
    mode: Mode,
    menu_state: ListState,
    difficulty_filter: Option<Difficulty>,
    unit_counts: UnitCounts,
    questions: Vec<usize>,          // indices into QUESTIONS
    q_idx: usize,
    opt_idx: usize,
    chosen: Option<usize>,
    score: u32,
    wrong_answers: Vec<(usize, usize)>, // (QUESTIONS[i], chosen_option)
    explanation_scroll: u16,
    review_idx: usize,
}

impl App {
    fn new() -> Self {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));
        App {
            mode: Mode::Menu,
            menu_state,
            difficulty_filter: None,
            unit_counts: build_unit_counts(),
            questions: vec![],
            q_idx: 0,
            opt_idx: 0,
            chosen: None,
            score: 0,
            wrong_answers: vec![],
            explanation_scroll: 0,
            review_idx: 0,
        }
    }

    fn start_quiz(&mut self, unit_filter: Option<u8>) {
        let mut rng = rand::thread_rng();
        let diff = self.difficulty_filter;
        let mut indices: Vec<usize> = QUESTIONS
            .iter()
            .enumerate()
            .filter(|(_, q)| {
                unit_filter.map_or(true, |u| q.unit == u)
                    && diff.map_or(true, |d| q.difficulty == d)
            })
            .map(|(i, _)| i)
            .collect();
        indices.shuffle(&mut rng);
        self.questions = indices;
        self.q_idx = 0;
        self.opt_idx = 0;
        self.chosen = None;
        self.score = 0;
        self.wrong_answers = vec![];
        self.explanation_scroll = 0;
        self.mode = if self.questions.is_empty() {
            Mode::Menu
        } else {
            Mode::Quiz
        };
    }

    fn current_question(&self) -> Option<&'static Question> {
        self.questions.get(self.q_idx).map(|&i| &QUESTIONS[i])
    }

    fn submit_answer(&mut self) {
        if self.chosen.is_some() {
            return;
        }
        self.chosen = Some(self.opt_idx);
        if let Some(q) = self.current_question() {
            if self.opt_idx == q.correct {
                self.score += 1;
            } else if let Some(&qi) = self.questions.get(self.q_idx) {
                self.wrong_answers.push((qi, self.opt_idx));
            }
        }
        self.explanation_scroll = 0;
        self.mode = Mode::Feedback;
    }

    fn next_question(&mut self) {
        self.q_idx += 1;
        self.opt_idx = 0;
        self.chosen = None;
        self.explanation_scroll = 0;
        if self.q_idx >= self.questions.len() {
            self.review_idx = 0;
            self.mode = Mode::Results;
        } else {
            self.mode = Mode::Quiz;
        }
    }

    fn grade(&self) -> &'static str {
        if self.questions.is_empty() {
            return "—";
        }
        let pct = self.score as f32 / self.questions.len() as f32;
        match (pct * 100.0) as u32 {
            95..=100 => "S",
            80..=94 => "A",
            65..=79 => "B",
            50..=64 => "C",
            _ => "D",
        }
    }

    fn grade_color(&self) -> Color {
        match self.grade() {
            "S" => Color::Rgb(255, 215, 0),
            "A" => GREEN,
            "B" => TEAL,
            "C" => GOLD,
            _ => RED,
        }
    }
}

// ── Terminal lifecycle ────────────────────────────────────────────────────────

fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
}

fn setup_panic_hook() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        restore_terminal();
        default_hook(info);
    }));
}

fn main() -> io::Result<()> {
    setup_panic_hook();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| draw(f, &mut app))?;

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match app.mode {
                Mode::Menu => handle_menu_key(&mut app, key.code),
                Mode::Quiz => handle_quiz_key(&mut app, key.code),
                Mode::Feedback => handle_feedback_key(&mut app, key.code),
                Mode::Review => handle_review_key(&mut app, key.code),
                Mode::Results => handle_results_key(&mut app, key.code),
                Mode::Quit => break,
            },
            Event::Resize(_, _) => {} // redraws on next loop iteration
            _ => {}
        }

        if app.mode == Mode::Quit {
            break;
        }
    }

    restore_terminal();
    Ok(())
}

// ── Key handlers ──────────────────────────────────────────────────────────────

fn handle_menu_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Up | KeyCode::Char('k') => {
            let i = app.menu_state.selected().unwrap_or(0);
            app.menu_state.select(Some(i.saturating_sub(1)));
        }
        KeyCode::Down | KeyCode::Char('j') => {
            let i = app.menu_state.selected().unwrap_or(0);
            app.menu_state.select(Some((i + 1).min(UNIT_LABELS.len() - 1)));
        }
        KeyCode::Char('d') => {
            app.difficulty_filter = cycle_difficulty(app.difficulty_filter);
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            let sel = app.menu_state.selected().unwrap_or(0);
            let unit_filter = if sel == 0 { None } else { Some(sel as u8) };
            app.start_quiz(unit_filter);
        }
        KeyCode::Char('q') | KeyCode::Esc => app.mode = Mode::Quit,
        _ => {}
    }
}

fn handle_quiz_key(app: &mut App, code: KeyCode) {
    if let Some(q) = app.current_question() {
        let max_opt = q.options.len() - 1;
        match code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.opt_idx = app.opt_idx.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.opt_idx = (app.opt_idx + 1).min(max_opt);
            }
            // support both 1-4 and a-d / A-D
            KeyCode::Char('1') | KeyCode::Char('a') | KeyCode::Char('A') => app.opt_idx = 0,
            KeyCode::Char('2') | KeyCode::Char('b') | KeyCode::Char('B') => app.opt_idx = 1,
            KeyCode::Char('3') | KeyCode::Char('c') | KeyCode::Char('C') => app.opt_idx = 2,
            KeyCode::Char('4') | KeyCode::Char('d') | KeyCode::Char('D') => {
                app.opt_idx = 3.min(max_opt)
            }
            KeyCode::Enter | KeyCode::Char(' ') => app.submit_answer(),
            KeyCode::Char('q') | KeyCode::Esc => app.mode = Mode::Menu,
            _ => {}
        }
    }
}

fn handle_feedback_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Right | KeyCode::Char('l') => {
            app.next_question();
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.explanation_scroll = app.explanation_scroll.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.explanation_scroll = app.explanation_scroll.saturating_add(1);
        }
        KeyCode::Char('q') | KeyCode::Esc => app.mode = Mode::Menu,
        _ => {}
    }
}

fn handle_review_key(app: &mut App, code: KeyCode) {
    let total = app.wrong_answers.len();
    match code {
        KeyCode::Right | KeyCode::Char('l') => {
            if total > 0 && app.review_idx < total - 1 {
                app.review_idx += 1;
                app.explanation_scroll = 0;
            }
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if app.review_idx > 0 {
                app.review_idx -= 1;
                app.explanation_scroll = 0;
            }
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.explanation_scroll = app.explanation_scroll.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.explanation_scroll = app.explanation_scroll.saturating_add(1);
        }
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter => {
            app.mode = Mode::Menu;
        }
        _ => {}
    }
}

fn handle_results_key(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Enter | KeyCode::Char('r') => app.mode = Mode::Menu,
        KeyCode::Char('v') if !app.wrong_answers.is_empty() => {
            app.review_idx = 0;
            app.explanation_scroll = 0;
            app.mode = Mode::Review;
        }
        KeyCode::Char('q') | KeyCode::Esc => app.mode = Mode::Quit,
        _ => {}
    }
}

// ── Drawing ───────────────────────────────────────────────────────────────────

fn draw(f: &mut Frame, app: &mut App) {
    f.render_widget(Block::default().style(Style::default().bg(BG)), f.area());
    match app.mode {
        Mode::Menu => draw_menu(f, app),
        Mode::Quiz => draw_quiz(f, app),
        Mode::Feedback => draw_feedback(f, app),
        Mode::Review => draw_review(f, app),
        Mode::Results => draw_results(f, app),
        Mode::Quit => {}
    }
}

fn draw_menu(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // title
            Constraint::Length(3), // difficulty filter
            Constraint::Min(8),    // unit list
            Constraint::Length(2), // footer
        ])
        .margin(2)
        .split(area);

    // Title
    f.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(
                "BASAL COGNITION",
                Style::default()
                    .fg(TEAL)
                    .add_modifier(Modifier::BOLD | Modifier::ITALIC),
            )),
            Line::from(Span::styled(
                "Intelligence Before the Brain — Quiz Companion",
                Style::default().fg(GOLD),
            )),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(MUTED)),
        ),
        chunks[0],
    );

    // Difficulty filter bar — precomputed colors/labels, no per-frame alloc of counts
    let diff_opts: [(Option<Difficulty>, &str); 4] = [
        (None, "All"),
        (Some(Difficulty::Foundational), "Foundational"),
        (Some(Difficulty::Intermediate), "Intermediate"),
        (Some(Difficulty::Advanced), "Advanced"),
    ];
    let mut fspans: Vec<Span> = vec![Span::styled("  Filter: ", Style::default().fg(MUTED))];
    for (diff, label) in diff_opts {
        let active = diff == app.difficulty_filter;
        let circle = if active { "●" } else { "◯" };
        fspans.push(Span::styled(
            format!("{circle} {label}  "),
            if active {
                Style::default()
                    .fg(filter_color(diff))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(MUTED)
            },
        ));
    }
    fspans.push(Span::styled(
        "[d] cycle",
        Style::default().fg(TEAL).add_modifier(Modifier::DIM),
    ));
    f.render_widget(
        Paragraph::new(Line::from(fspans)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(MUTED))
                .style(Style::default().bg(SURFACE)),
        ),
        chunks[1],
    );

    // Unit list — counts reflect active difficulty filter
    let di = diff_to_idx(app.difficulty_filter);
    let items: Vec<ListItem> = UNIT_LABELS
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let count = app.unit_counts[i][di];
            let text_col = if count == 0 { MUTED } else { Color::White };
            ListItem::new(Line::from(vec![
                Span::styled(format!("  {label:<42}"), Style::default().fg(text_col)),
                Span::styled(format!("{count:>3} q"), Style::default().fg(MUTED)),
            ]))
        })
        .collect();

    f.render_stateful_widget(
        List::new(items)
            .block(
                Block::default()
                    .title(Span::styled(
                        " Select a Unit ",
                        Style::default().fg(TEAL).add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(TEAL))
                    .style(Style::default().bg(SURFACE))
                    .padding(Padding::uniform(1)),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Rgb(30, 60, 80))
                    .fg(GOLD)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ "),
        chunks[2],
        &mut app.menu_state,
    );

    f.render_widget(
        Paragraph::new(footer_line(&[
            ("↑↓/jk", "navigate"),
            ("d", "difficulty"),
            ("Enter", "start"),
            ("q", "quit"),
        ]))
        .alignment(Alignment::Center),
        chunks[3],
    );
}

fn draw_quiz(f: &mut Frame, app: &App) {
    let q = match app.current_question() {
        Some(q) => q,
        None => return,
    };
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),  // progress gauge
            Constraint::Length(2),  // metadata
            Constraint::Min(5),     // question
            Constraint::Min(12),    // options
            Constraint::Length(2),  // footer
        ])
        .margin(2)
        .split(area);

    // Progress — label shows 1-based question number (fixes off-by-one)
    let total = app.questions.len();
    let ratio = if total == 0 { 0.0 } else { app.q_idx as f64 / total as f64 };
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(TEAL).bg(SURFACE))
            .ratio(ratio)
            .label(format!("Question {} of {}", app.q_idx + 1, total)),
        chunks[0],
    );

    // Metadata
    let d_col = difficulty_color(q.difficulty);
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(format!("  Unit {:>2}", q.unit), Style::default().fg(MUTED)),
            Span::raw("   "),
            Span::styled(
                format!("● {}", q.difficulty.label()),
                Style::default().fg(d_col),
            ),
            Span::raw("   "),
            Span::styled(
                format!("Score: {}/{}", app.score, app.q_idx),
                Style::default().fg(TEAL),
            ),
        ])),
        chunks[1],
    );

    // Question block
    f.render_widget(
        Paragraph::new(q.text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Rgb(60, 80, 100)))
                    .style(Style::default().bg(SURFACE))
                    .padding(Padding::new(2, 2, 1, 1)),
            )
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        chunks[2],
    );

    // Options: each rendered as its own Paragraph so long text wraps
    let opt_border = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(TEAL))
        .style(Style::default().bg(SURFACE));
    // Compute inner rect manually (border = 1 each side)
    let ob = chunks[3];
    let opt_inner = Rect::new(ob.x + 1, ob.y + 1, ob.width.saturating_sub(2), ob.height.saturating_sub(2));
    f.render_widget(opt_border, chunks[3]);

    let opt_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .margin(1)
        .split(opt_inner);

    for (i, opt_text) in q.options.iter().enumerate() {
        let letter = ['A', 'B', 'C', 'D'][i];
        let selected = i == app.opt_idx;
        let (marker, marker_col, text_style) = if selected {
            ("▶", GOLD, Style::default().fg(GOLD).add_modifier(Modifier::BOLD))
        } else {
            (" ", MUTED, Style::default().fg(Color::White))
        };
        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("{marker} [{letter}] "),
                    Style::default().fg(marker_col),
                ),
                Span::styled(*opt_text, text_style),
            ]))
            .wrap(Wrap { trim: true }),
            opt_chunks[i],
        );
    }

    f.render_widget(
        Paragraph::new(footer_line(&[
            ("↑↓/jk", "select"),
            ("a-d/1-4", "jump"),
            ("Enter", "submit"),
            ("q", "menu"),
        ]))
        .alignment(Alignment::Center),
        chunks[4],
    );
}

fn draw_feedback(f: &mut Frame, app: &App) {
    let q = match app.current_question() {
        Some(q) => q,
        None => return,
    };
    let chosen = match app.chosen {
        Some(c) => c,
        None => return,
    };
    let correct = chosen == q.correct;
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // verdict
            Constraint::Min(8),    // options
            Constraint::Min(6),    // explanation
            Constraint::Length(2), // footer
        ])
        .margin(2)
        .split(area);

    // Verdict banner
    let (verdict_text, verdict_col) = if correct {
        ("✓  CORRECT", GREEN)
    } else {
        ("✗  INCORRECT", RED)
    };
    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            verdict_text,
            Style::default()
                .fg(verdict_col)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC),
        )))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(verdict_col))
                .style(Style::default().bg(SURFACE)),
        ),
        chunks[0],
    );

    // Options with correct/wrong marks
    let items: Vec<ListItem> = q
        .options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let letter = ['A', 'B', 'C', 'D'][i];
            let (mark, col) = if i == q.correct {
                ("✓", GREEN)
            } else if i == chosen && !correct {
                ("✗", RED)
            } else {
                (" ", MUTED)
            };
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("  [{letter}] {mark} "),
                    Style::default().fg(col).add_modifier(Modifier::BOLD),
                ),
                Span::styled(*opt, Style::default().fg(col)),
            ]))
        })
        .collect();
    f.render_widget(
        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(MUTED))
                .style(Style::default().bg(SURFACE))
                .padding(Padding::new(1, 1, 0, 0)),
        ),
        chunks[1],
    );

    // Explanation — scrollable; border turns gold when scrolled
    let expl_border_col = if app.explanation_scroll > 0 { GOLD } else { TEAL };
    let scroll_badge = if app.explanation_scroll > 0 {
        format!(" ↑{} ", app.explanation_scroll)
    } else {
        String::new()
    };
    f.render_widget(
        Paragraph::new(q.explanation)
            .block(
                Block::default()
                    .title(Line::from(vec![
                        Span::styled(
                            " Explanation ",
                            Style::default().fg(TEAL).add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(scroll_badge, Style::default().fg(GOLD)),
                    ]))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(expl_border_col))
                    .style(Style::default().bg(SURFACE))
                    .padding(Padding::new(2, 2, 1, 1)),
            )
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .scroll((app.explanation_scroll, 0)),
        chunks[2],
    );

    // Footer with remaining count
    let remaining = app.questions.len().saturating_sub(app.q_idx + 1);
    let remaining_text = if remaining > 0 {
        format!("{remaining} remaining")
    } else {
        "last question".to_string()
    };
    let mut fspans = footer_line(&[("Enter/→", "next"), ("jk", "scroll expl."), ("q", "menu")]);
    fspans.spans.push(Span::raw("    "));
    fspans
        .spans
        .push(Span::styled(remaining_text, Style::default().fg(MUTED)));
    f.render_widget(
        Paragraph::new(fspans).alignment(Alignment::Center),
        chunks[3],
    );
}

fn draw_review(f: &mut Frame, app: &App) {
    if app.wrong_answers.is_empty() {
        return;
    }
    let (qi, chosen) = app.wrong_answers[app.review_idx];
    let q = &QUESTIONS[qi];
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // header
            Constraint::Min(4),    // question
            Constraint::Min(8),    // options
            Constraint::Min(6),    // explanation
            Constraint::Length(2), // footer
        ])
        .margin(2)
        .split(area);

    // Header
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("Review — Wrong Answers  ", Style::default().fg(MUTED)),
            Span::styled(
                format!("{}/{}", app.review_idx + 1, app.wrong_answers.len()),
                Style::default().fg(RED).add_modifier(Modifier::BOLD),
            ),
        ]))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(MUTED)),
        ),
        chunks[0],
    );

    // Question
    f.render_widget(
        Paragraph::new(q.text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Rgb(60, 80, 100)))
                    .style(Style::default().bg(SURFACE))
                    .padding(Padding::new(2, 2, 1, 1)),
            )
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        chunks[1],
    );

    // Options
    let items: Vec<ListItem> = q
        .options
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let letter = ['A', 'B', 'C', 'D'][i];
            let (mark, col) = if i == q.correct {
                ("✓", GREEN)
            } else if i == chosen {
                ("✗", RED)
            } else {
                (" ", MUTED)
            };
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("  [{letter}] {mark} "),
                    Style::default().fg(col).add_modifier(Modifier::BOLD),
                ),
                Span::styled(*opt, Style::default().fg(col)),
            ]))
        })
        .collect();
    f.render_widget(
        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(MUTED))
                .style(Style::default().bg(SURFACE))
                .padding(Padding::new(1, 1, 0, 0)),
        ),
        chunks[2],
    );

    // Explanation — scrollable
    let expl_border_col = if app.explanation_scroll > 0 { GOLD } else { TEAL };
    f.render_widget(
        Paragraph::new(q.explanation)
            .block(
                Block::default()
                    .title(Span::styled(
                        " Explanation ",
                        Style::default().fg(TEAL).add_modifier(Modifier::BOLD),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(expl_border_col))
                    .style(Style::default().bg(SURFACE))
                    .padding(Padding::new(2, 2, 1, 1)),
            )
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .scroll((app.explanation_scroll, 0)),
        chunks[3],
    );

    f.render_widget(
        Paragraph::new(footer_line(&[
            ("←→/hl", "navigate"),
            ("jk", "scroll expl."),
            ("Enter/q", "back to menu"),
        ]))
        .alignment(Alignment::Center),
        chunks[4],
    );
}

fn draw_results(f: &mut Frame, app: &App) {
    let area = f.area();
    let total = app.questions.len();
    let score = app.score;
    let pct = if total == 0 {
        0
    } else {
        (score * 100) / total as u32
    };
    let grade = app.grade();
    let grade_color = app.grade_color();
    let quote = match grade {
        "S" => "The bacterium and the philosopher share one commitment: to sense the world and respond wisely.",
        "A" => "Intelligence is older than neurons. You seem to understand that.",
        "B" => "Every living cell has been asking the same questions you just tried to answer.",
        "C" => "Even E. coli gets it wrong sometimes. The important thing is the next gradient.",
        _ => "The slime mold found the optimal path eventually. So will you.",
    };

    // Clamp popup to terminal size — prevents overflow on small terminals
    let popup_width = area.width.min(72);
    let popup_height = area.height.min(22);
    let popup_x = area.width.saturating_sub(popup_width) / 2;
    let popup_y = area.height.saturating_sub(popup_height) / 2;
    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    f.render_widget(Clear, popup_area);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(grade_color))
            .style(Style::default().bg(SURFACE))
            .title(Span::styled(
                " Results ",
                Style::default()
                    .fg(grade_color)
                    .add_modifier(Modifier::BOLD),
            )),
        popup_area,
    );

    // Inner area (inside double border = 1px each side)
    let inner = Rect::new(
        popup_area.x + 1,
        popup_area.y + 1,
        popup_area.width.saturating_sub(2),
        popup_area.height.saturating_sub(2),
    );

    let pc = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // grade
            Constraint::Length(2), // score
            Constraint::Length(2), // gauge
            Constraint::Min(3),    // quote
            Constraint::Length(1), // divider
            Constraint::Length(3), // actions
        ])
        .margin(1)
        .split(inner);

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("Grade: {grade}"),
            Style::default()
                .fg(grade_color)
                .add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        pc[0],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            format!("{score} / {total}  ({pct}%)"),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        )))
        .alignment(Alignment::Center),
        pc[1],
    );

    let ratio = if total == 0 { 0.0 } else { score as f64 / total as f64 };
    f.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(grade_color).bg(Color::Rgb(30, 35, 45)))
            .ratio(ratio),
        pc[2],
    );

    f.render_widget(
        Paragraph::new(format!("\"{quote}\""))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .style(Style::default().fg(MUTED).add_modifier(Modifier::ITALIC)),
        pc[3],
    );

    f.render_widget(
        Paragraph::new(Line::from(Span::styled(
            "─".repeat(popup_width.saturating_sub(6) as usize),
            Style::default().fg(MUTED),
        )))
        .alignment(Alignment::Center),
        pc[4],
    );

    // Actions: show [v] review only when there are wrong answers
    let mut action_spans: Vec<Span> = Vec::new();
    push_hint(&mut action_spans, "Enter/r", "menu");
    if !app.wrong_answers.is_empty() {
        action_spans.push(Span::raw("   "));
        push_hint(
            &mut action_spans,
            "v",
            &format!("review {} wrong", app.wrong_answers.len()),
        );
    }
    action_spans.push(Span::raw("   "));
    push_hint(&mut action_spans, "q", "quit");

    f.render_widget(
        Paragraph::new(Line::from(action_spans)).alignment(Alignment::Center),
        pc[5],
    );
}

// ── UI helpers ────────────────────────────────────────────────────────────────

fn footer_line(hints: &[(&str, &str)]) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    for (i, (key, label)) in hints.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("    "));
        }
        spans.push(Span::styled(
            format!("[{key}]"),
            Style::default().fg(TEAL).add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            format!(" {label}"),
            Style::default().fg(MUTED),
        ));
    }
    Line::from(spans)
}

fn push_hint(spans: &mut Vec<Span<'static>>, key: &str, label: &str) {
    spans.push(Span::styled(
        format!("[{key}]"),
        Style::default().fg(TEAL).add_modifier(Modifier::BOLD),
    ));
    spans.push(Span::styled(
        format!(" {label}"),
        Style::default().fg(MUTED),
    ));
}
