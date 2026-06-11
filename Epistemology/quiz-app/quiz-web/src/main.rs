use std::{env, path::PathBuf, sync::Arc};

use axum::{
    extract::{Form, Query, State},
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use quiz_core::{load_quiz, question::Question, Quiz};
use serde::Deserialize;

// ── Shared state ──────────────────────────────────────────────────────────────

#[derive(Clone)]
struct AppState {
    quiz: Arc<Quiz>,
}

// ── Query / form types ────────────────────────────────────────────────────────

#[derive(Deserialize, Default)]
struct QuestionQuery {
    #[serde(default)]
    q: usize,
    /// JSON array of previous answers, URL-encoded.
    #[serde(default)]
    answers: String,
}

#[derive(Deserialize)]
struct SubmitForm {
    answer: String,
    q: usize,
    answers: String, // accumulated previous answers (JSON array string)
}

#[derive(Deserialize, Default)]
struct ResultsQuery {
    #[serde(default)]
    answers: String,
}

// ── HTML helpers ──────────────────────────────────────────────────────────────

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const STYLE: &str = r#"
<style>
  *{box-sizing:border-box;margin:0;padding:0}
  body{font-family:system-ui,sans-serif;background:#f0f4f8;min-height:100vh;padding:24px 16px}
  .card{background:#fff;border-radius:12px;padding:32px;max-width:680px;margin:0 auto;
        box-shadow:0 2px 12px rgba(0,0,0,.1)}
  .meta{color:#64748b;font-size:14px;margin-bottom:16px}
  .progress-bar{height:8px;background:#e2e8f0;border-radius:4px;overflow:hidden;margin-bottom:24px}
  .progress-fill{height:100%;background:#22c55e;border-radius:4px;transition:width .3s}
  h1{font-size:22px;font-weight:700;color:#1e293b;margin-bottom:20px;line-height:1.4}
  h2{font-size:16px;font-weight:600;color:#334155;margin-bottom:20px}
  .choices{display:flex;flex-direction:column;gap:10px;margin-bottom:24px}
  .choice{display:flex;align-items:center;gap:12px;padding:14px 18px;border:2px solid #e2e8f0;
          border-radius:8px;cursor:pointer;transition:border-color .15s,background .15s}
  .choice:hover{border-color:#22c55e;background:#f0fdf4}
  .choice input[type=radio]{display:none}
  .choice:has(input:checked){border-color:#22c55e;background:#f0fdf4}
  .choice-key{font-weight:700;color:#22c55e;width:20px;flex-shrink:0}
  .text-input{width:100%;padding:12px 16px;border:2px solid #e2e8f0;border-radius:8px;
              font-size:16px;margin-bottom:16px;outline:none;transition:border-color .15s}
  .text-input:focus{border-color:#22c55e}
  .btn{display:block;width:100%;padding:14px;background:#22c55e;color:#fff;border:none;
       border-radius:8px;font-size:16px;font-weight:600;cursor:pointer;text-align:center;
       text-decoration:none;transition:background .15s}
  .btn:hover{background:#16a34a}
  .btn-outline{background:#fff;color:#334155;border:2px solid #e2e8f0}
  .btn-outline:hover{background:#f8fafc;border-color:#cbd5e1}
  .result-row{display:flex;align-items:baseline;gap:10px;padding:8px 0;
              border-bottom:1px solid #f1f5f9;font-size:14px}
  .tick{font-weight:700;width:20px;flex-shrink:0}
  .ok{color:#22c55e} .fail{color:#ef4444}
  .score-big{font-size:42px;font-weight:800;text-align:center;margin:16px 0}
  .grade{text-align:center;font-size:18px;color:#64748b;margin-bottom:24px}
  .explanation{background:#f8fafc;border-left:3px solid #cbd5e1;padding:12px 16px;
               border-radius:0 6px 6px 0;color:#64748b;font-size:14px;margin-top:6px}
  .tag{display:inline-block;padding:2px 8px;border-radius:999px;font-size:12px;font-weight:600;
       background:#dbeafe;color:#1d4ed8;margin-bottom:12px}
</style>
"#;

fn page(title: &str, body: &str) -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html><html lang="en"><head>
<meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>{title}</title>{STYLE}</head>
<body>{body}</body></html>"#
    ))
}

// ── Route handlers ────────────────────────────────────────────────────────────

async fn index() -> Redirect {
    Redirect::to("/quiz")
}

async fn quiz_start(State(state): State<AppState>) -> Html<String> {
    let quiz = &state.quiz;
    let desc = quiz
        .description
        .as_deref()
        .map(|d| format!(r#"<p style="color:#64748b;margin-bottom:20px">{}</p>"#, esc(d)))
        .unwrap_or_default();

    let body = format!(
        r#"<div class="card">
  <h1>{}</h1>
  {}
  <p class="meta">{} questions &middot; multiple choice, true/false, and short answer</p>
  <br>
  <a href="/quiz/question?q=0&answers=%5B%5D" class="btn">Start Quiz &rarr;</a>
</div>"#,
        esc(&quiz.title),
        desc,
        quiz.questions.len()
    );
    page(&quiz.title, &body)
}

async fn show_question(
    State(state): State<AppState>,
    Query(params): Query<QuestionQuery>,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let quiz = &state.quiz;
    let q_idx = params.q;

    if q_idx >= quiz.questions.len() {
        return Redirect::to(&format!(
            "/quiz/results?answers={}",
            urlencoding::encode(&params.answers)
        ))
        .into_response();
    }

    let q = &quiz.questions[q_idx];
    let total = quiz.questions.len();
    let pct = (q_idx as f64 / total as f64 * 100.0).round() as u32;

    let tag = match q {
        Question::MultipleChoice { .. } => "Multiple choice",
        Question::TrueFalse { .. } => "True / False",
        Question::FillBlank { .. } => "Short answer",
        Question::Proof { .. } => "Proof scaffold",
    };

    let choices_html = make_choices_html(q, q_idx, &params.answers);

    let body = format!(
        r#"<div class="card">
  <div class="meta">Q{q_num}/{total} &mdash; {title}</div>
  <div class="progress-bar"><div class="progress-fill" style="width:{pct}%"></div></div>
  <span class="tag">{tag}</span>
  <h1>{text}</h1>
  <form method="post" action="/quiz/submit">
    <input type="hidden" name="q" value="{q_idx}">
    <input type="hidden" name="answers" value="{prev}">
    {choices_html}
    <button type="submit" class="btn">Submit &rarr;</button>
  </form>
</div>"#,
        q_num = q_idx + 1,
        title = esc(&quiz.title),
        text = esc(q.text()),
        prev = esc(&params.answers),
    );
    page(&quiz.title, &body).into_response()
}

fn make_choices_html(q: &Question, _q_idx: usize, _prev: &str) -> String {
    match q {
        Question::MultipleChoice { choices, .. } => {
            let labels = ['A', 'B', 'C', 'D'];
            let items: String = choices
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let label = labels.get(i).copied().unwrap_or('?');
                    format!(
                        r#"<label class="choice">
  <input type="radio" name="answer" value="{label}" required>
  <span class="choice-key">{label}</span>
  <span>{}</span>
</label>"#,
                        esc(c)
                    )
                })
                .collect();
            format!(r#"<div class="choices">{items}</div>"#)
        }
        Question::TrueFalse { .. } => {
            r#"<div class="choices">
  <label class="choice">
    <input type="radio" name="answer" value="true" required>
    <span class="choice-key">T</span><span>True</span>
  </label>
  <label class="choice">
    <input type="radio" name="answer" value="false" required>
    <span class="choice-key">F</span><span>False</span>
  </label>
</div>"#
            .to_string()
        }
        Question::FillBlank { .. } => {
            r#"<input class="text-input" type="text" name="answer"
       placeholder="Type your answer here…" required autofocus>"#
                .to_string()
        }
        Question::Proof { choices, .. } => {
            let proof_lines: String = choices
                .iter()
                .map(|line| format!("<div class=\"proof-line\">{}</div>", esc(line)))
                .collect();
            format!(
                r#"<div class="proof-scaffold">{proof_lines}</div>
<input class="text-input" type="text" name="answer"
       placeholder="Enter fills separated by | (pipe)…" required autofocus>"#
            )
        }
    }
}

async fn submit_answer(
    State(state): State<AppState>,
    Form(form): Form<SubmitForm>,
) -> Redirect {
    let quiz = &state.quiz;
    let mut prev: Vec<String> = serde_json::from_str(&form.answers).unwrap_or_default();
    prev.push(form.answer.clone());

    let next_q = form.q + 1;
    let encoded = urlencoding::encode(&serde_json::to_string(&prev).unwrap_or_default()).into_owned();

    if next_q >= quiz.questions.len() {
        Redirect::to(&format!("/quiz/results?answers={encoded}"))
    } else {
        Redirect::to(&format!("/quiz/question?q={next_q}&answers={encoded}"))
    }
}

async fn show_results(
    State(state): State<AppState>,
    Query(params): Query<ResultsQuery>,
) -> Html<String> {
    let quiz = &state.quiz;
    let answers: Vec<String> = serde_json::from_str(&params.answers).unwrap_or_default();

    let total = quiz.questions.len();
    let correct_count = answers
        .iter()
        .enumerate()
        .filter(|(i, a)| quiz.questions.get(*i).map(|q| q.check(a)).unwrap_or(false))
        .count();

    let pct = if total > 0 {
        (correct_count as f64 / total as f64 * 100.0).round() as u32
    } else {
        0
    };

    let (color, grade) = match pct {
        90..=100 => ("#22c55e", "Excellent! 🏆"),
        70..=89  => ("#84cc16", "Good work! 👍"),
        50..=69  => ("#f59e0b", "Keep studying 📚"),
        _        => ("#ef4444", "Needs practice 🔄"),
    };

    let bar_filled = pct;
    let rows: String = quiz
        .questions
        .iter()
        .enumerate()
        .map(|(i, q)| {
            let ans = answers.get(i).map(|s| s.as_str()).unwrap_or("—");
            let ok = q.check(ans);
            let (cls, tick) = if ok { ("ok", "✓") } else { ("fail", "✗") };
            let exp_text = q.explanation();
            let exp = if exp_text.is_empty() {
                String::new()
            } else {
                format!(r#"<div class="explanation">{}</div>"#, esc(exp_text))
            };
            format!(
                r#"<div class="result-row">
  <span class="tick {cls}">{tick}</span>
  <div>
    <div><strong>Q{}.</strong> {}</div>
    <div style="color:#64748b;font-size:13px">Your answer: <em>{}</em>
    {} Correct: <em>{}</em></div>
    {exp}
  </div>
</div>"#,
                i + 1,
                esc(q.text()),
                esc(ans),
                if ok { "" } else { "&mdash;" },
                esc(&q.correct_display()),
            )
        })
        .collect();

    let body = format!(
        r#"<div class="card">
  <h2>{}</h2>
  <div class="score-big" style="color:{color}">{correct_count}/{total}</div>
  <div class="grade">{grade}</div>
  <div class="progress-bar" style="margin-bottom:24px">
    <div class="progress-fill" style="width:{bar_filled}%;background:{color}"></div>
  </div>
  <h2 style="margin-bottom:16px">Question breakdown</h2>
  {rows}
  <br>
  <a href="/quiz" class="btn">Retake Quiz</a>
</div>"#,
        esc(&quiz.title),
    );
    page("Results", &body)
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "questions/sample.json".to_string());

    let quiz = load_quiz(PathBuf::from(&path))?;
    println!("Loaded quiz: \"{}\" ({} questions)", quiz.title, quiz.questions.len());

    let state = AppState { quiz: Arc::new(quiz) };

    let app = Router::new()
        .route("/", get(index))
        .route("/quiz", get(quiz_start))
        .route("/quiz/question", get(show_question))
        .route("/quiz/submit", post(submit_answer))
        .route("/quiz/results", get(show_results))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    println!("Quiz app running at http://localhost:3000");
    println!("Press Ctrl+C to stop.");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
