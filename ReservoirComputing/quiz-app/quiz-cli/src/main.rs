use std::{
    env,
    io::{self, BufRead, Write},
    path::PathBuf,
};

use quiz_core::{load_quiz, question::Question, Session};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "questions/sample.json".to_string());

    let quiz = load_quiz(PathBuf::from(&path))?;

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let stdin = io::stdin();

    // ── Header ────────────────────────────────────────────────────────────
    let bar = "═".repeat(quiz.title.len() + 4);
    writeln!(out, "\n╔{bar}╗")?;
    writeln!(out, "║  {}  ║", quiz.title)?;
    writeln!(out, "╚{bar}╝")?;
    if let Some(desc) = &quiz.description {
        writeln!(out, "{desc}")?;
    }
    writeln!(out, "{} questions\n", quiz.questions.len())?;
    out.flush()?;

    // ── Question loop ─────────────────────────────────────────────────────
    let total = quiz.questions.len();
    let mut session = Session::new(quiz);

    while !session.is_finished() {
        let idx = session.current;
        let q = session.quiz.questions[idx].clone();

        writeln!(out, "Q{}/{} ─────────────────────────────────────────", idx + 1, total)?;
        writeln!(out, "{}", q.text())?;
        writeln!(out)?;

        let choices = q.display_choices();
        for (i, choice) in choices.iter().enumerate() {
            let label = (b'A' + i as u8) as char;
            writeln!(out, "  {label}) {choice}")?;
        }

        let prompt = match &q {
            Question::MultipleChoice { .. } => "Your answer [A/B/C/D]: ",
            Question::TrueFalse { .. }      => "Your answer [T/F]: ",
            Question::FreeText { .. }       => "Your answer: ",
        };
        write!(out, "{prompt}")?;
        out.flush()?;

        let mut line = String::new();
        stdin.lock().read_line(&mut line)?;
        let answer = line.trim().to_string();

        if let Some(correct) = session.submit(answer) {
            if correct {
                writeln!(out, "  ✓ Correct!\n")?;
            } else {
                writeln!(out, "  ✗ Wrong.  Correct answer: {}\n", q.correct_display())?;
            }
            if let Some(exp) = q.explanation() {
                writeln!(out, "  ↳ {exp}\n")?;
            }
        }
        out.flush()?;
    }

    // ── Results ───────────────────────────────────────────────────────────
    let (correct, tot) = session.score();
    let pct = if tot > 0 { (correct as f64 / tot as f64 * 100.0).round() as u32 } else { 0 };

    let grade = match pct {
        90..=100 => "Excellent! 🎉",
        70..=89  => "Good work!  👍",
        50..=69  => "Keep studying 📚",
        _        => "Needs practice 🔄",
    };

    writeln!(out, "\n════════════════════════════════")?;
    writeln!(out, "  QUIZ COMPLETE")?;
    writeln!(out, "  Score: {correct}/{tot} ({pct}%)")?;
    writeln!(out, "  {grade}")?;
    writeln!(out, "════════════════════════════════\n")?;

    Ok(())
}
