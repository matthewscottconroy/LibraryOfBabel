use std::io::{self, Write};

use crate::adaptive::{mastery_summary, Scope};
use crate::config::{cathedral_prereqs, chapter_meta, phase_names, DIFF_BEGINNER_MAX, DIFF_INTERMEDIATE_MAX};
use crate::models::{Question, UserProfile};

// ── QuizQuit ──────────────────────────────────────────────────────────────────

pub struct QuizQuit;

// ── ANSI helpers ──────────────────────────────────────────────────────────────

fn c(code: &str, text: &str) -> String { format!("\x1b[{}m{}\x1b[0m", code, text) }

pub fn bold(t: &str)    -> String { c("1",  t) }
pub fn green(t: &str)   -> String { c("32", t) }
pub fn red(t: &str)     -> String { c("31", t) }
pub fn yellow(t: &str)  -> String { c("33", t) }
pub fn cyan(t: &str)    -> String { c("36", t) }
pub fn dim(t: &str)     -> String { c("2",  t) }
pub fn magenta(t: &str) -> String { c("35", t) }

const WIDTH: usize = 80;

pub fn hr(ch: char) {
    println!("{}", dim(&ch.to_string().repeat(WIDTH)));
}

pub fn section(title: &str) {
    println!("\n{}\n", bold(&cyan(title)));
}

pub fn print_wrap(text: &str) {
    let indent = "  ";
    let max_w = WIDTH - 2;
    let mut line = String::from(indent);
    for word in text.split_whitespace() {
        if line.len() + word.len() + 1 > max_w + 2 {
            println!("{}", line);
            line = format!("{}{}", indent, word);
        } else {
            if line == indent { line.push_str(word); } else { line.push(' '); line.push_str(word); }
        }
    }
    if line != indent { println!("{}", line); }
}

pub fn ask(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(0) | Err(_) => std::process::exit(0),
        _ => buf.trim().to_string(),
    }
}

pub fn ask_int(prompt: &str, lo: i32, hi: i32, default: Option<i32>) -> i32 {
    loop {
        let raw = ask(prompt);
        if raw.is_empty() {
            if let Some(d) = default { return d; }
        }
        if let Ok(v) = raw.parse::<i32>() {
            if v >= lo && v <= hi { return v; }
        }
        println!("{}", yellow(&format!("  Please enter a number between {} and {}.", lo, hi)));
    }
}

pub fn confirm(prompt: &str, default: bool) -> bool {
    let hint = if default { "[Y/n]" } else { "[y/N]" };
    let raw = ask(&format!("  {} {} → ", prompt, hint)).to_lowercase();
    if raw.is_empty() { return default; }
    raw == "y" || raw == "yes"
}

pub fn pause() {
    ask(&dim("  [Enter to continue] "));
}

// ── Question presentation ─────────────────────────────────────────────────────

const LABELS: &[char] = &['A', 'B', 'C', 'D'];

fn diff_badge(diff: &str) -> String {
    match diff {
        "beginner"     => green(&format!("[{}]", diff)),
        "intermediate" => yellow(&format!("[{}]", diff)),
        "advanced"     => red(&format!("[{}]", diff)),
        _              => dim(&format!("[{}]", diff)),
    }
}

fn ask_confidence() -> u8 {
    let raw = ask("  How well?  1=guessed  2=unsure  3=certain  [2] → ");
    match raw.trim() { "1" => 1, "3" => 3, _ => 2 }
}

/// Returns (correct, flagged, confidence).
pub fn present_question(
    q: &Question,
    num: usize,
    total: usize,
    running_correct: Option<usize>,
) -> Result<(bool, bool, u8), QuizQuit> {
    let ch_name = chapter_meta().get(&q.chapter)
        .map(|m| m.name.to_string())
        .unwrap_or_else(|| format!("Ch.{}", q.chapter));

    println!();
    hr('─');

    let score_note = running_correct
        .map(|s| dim(&format!("  score so far: {}", s)))
        .unwrap_or_default();
    let gen_badge = if q.generated { dim("  [AI-generated]") } else { String::new() };
    println!("  {}  {}{}{}",
        dim(&ch_name), diff_badge(&q.difficulty), gen_badge, score_note);

    // Wrap the question text
    let wrapped = wrap_text(&q.text, WIDTH - 10);
    print!("\n  {}  ", bold(&format!("Q{}/{}.", num, total)));
    if let Some(first) = wrapped.first() { println!("{}", first); }
    for line in wrapped.iter().skip(1) { println!("      {}", line); }
    println!();

    let correct = match q.kind.as_str() {
        "mc"    => present_mc(q)?,
        "tf"    => present_tf(q)?,
        "blank" => present_blank(q)?,
        _       => return Ok((false, true, 2)),
    };

    let confidence = ask_confidence();
    pause();
    Ok((correct, !correct, confidence))
}

fn present_mc(q: &Question) -> Result<bool, QuizQuit> {
    for (i, choice) in q.choices.iter().enumerate() {
        println!("    {}. {}", bold(&LABELS[i].to_string()), choice);
    }
    println!();
    let user_idx = loop {
        let raw = ask("  Answer (A/B/C/D or Q to quit) → ").to_uppercase();
        if raw == "Q" { return Err(QuizQuit); }
        if let Some(c) = raw.chars().next() {
            if let Some(idx) = LABELS[..q.choices.len()].iter().position(|&l| l == c) {
                break idx;
            }
        }
        println!("{}", yellow("  Please enter A–D (or Q to quit)."));
    };
    let correct_idx = match &q.answer {
        crate::models::Answer::Index(i) => *i,
        _ => 0,
    };
    let correct = user_idx == correct_idx;
    show_result(correct, &LABELS[correct_idx].to_string(), &q.choices[correct_idx], &q.explanation);
    Ok(correct)
}

fn present_tf(q: &Question) -> Result<bool, QuizQuit> {
    println!("    A. True");
    println!("    B. False");
    println!();
    let user_idx = loop {
        let raw = ask("  Answer (A/B or T/F or Q to quit) → ").to_uppercase();
        if raw == "Q" { return Err(QuizQuit); }
        match raw.as_str() {
            "A" | "T" | "TRUE"  => break 0usize,
            "B" | "F" | "FALSE" => break 1usize,
            _ => println!("{}", yellow("  Please enter A (True) or B (False), or Q to quit.")),
        }
    };
    let correct_idx = match &q.answer {
        crate::models::Answer::Index(i) => *i,
        _ => 0,
    };
    let correct = user_idx == correct_idx;
    let label = if correct_idx == 0 { "A (True)" } else { "B (False)" };
    show_result(correct, label, &q.choices[correct_idx], &q.explanation);
    Ok(correct)
}

fn normalize_blank(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .to_lowercase()
        .trim()
        .to_string()
}

fn match_blank(raw: &str, acceptable: &[String]) -> bool {
    let raw_n = normalize_blank(raw);
    let raw_ns = raw_n.replace(' ', "");
    for a in acceptable {
        let a_n = normalize_blank(a);
        if raw_n == a_n { return true; }
        if raw_ns == a_n.replace(' ', "") { return true; }
        let a_tokens: std::collections::HashSet<&str> = a_n.split_whitespace().collect();
        let raw_tokens: std::collections::HashSet<&str> = raw_n.split_whitespace().collect();
        if a_tokens.len() > 1 && a_tokens.is_subset(&raw_tokens)
            && raw_tokens.len() <= a_tokens.len() + 2
        {
            return true;
        }
    }
    false
}

fn present_blank(q: &Question) -> Result<bool, QuizQuit> {
    let raw = ask("  Your answer (or Q to quit) → ");
    if raw.to_uppercase() == "Q" { return Err(QuizQuit); }
    if raw.is_empty() {
        let primary = q.choices.first().map(|s| s.as_str()).unwrap_or("");
        show_result(false, primary, primary, &q.explanation);
        return Ok(false);
    }
    let correct = match_blank(&raw, &q.choices);
    let primary = q.choices.first().map(|s| s.as_str()).unwrap_or("");
    show_result(correct, primary, primary, &q.explanation);
    Ok(correct)
}

fn show_result(correct: bool, label: &str, _text: &str, explanation: &str) {
    println!();
    if correct {
        println!("  {}", green(&bold("✓  Correct!")));
    } else {
        println!("  {}  Answer: {}", red(&bold("✗  Incorrect.")), bold(label));
    }
    println!();
    println!("{}", dim("  Explanation:"));
    for line in wrap_text(explanation, WIDTH - 6) {
        println!("    {}", line);
    }
}

// ── Progress bar ──────────────────────────────────────────────────────────────

pub fn progress_bar(score: usize, total: usize, width: usize) -> String {
    let pct = score as f64 / total.max(1) as f64;
    let fill = (pct * width as f64) as usize;
    let bar = format!("{}{}", "█".repeat(fill), "░".repeat(width - fill));
    let coloured = if pct >= 0.7 { green(&bar) } else if pct >= 0.5 { yellow(&bar) } else { red(&bar) };
    format!("{} {}/{}", coloured, score, total)
}

// ── Session summary ───────────────────────────────────────────────────────────

pub fn show_session_summary(
    score: usize,
    total: usize,
    wrong_names: &[String],
    duration_secs: f64,
    streak_max: u32,
    per_difficulty: &std::collections::HashMap<String, [u32; 2]>,
    n_generated: u32,
    per_chapter: &std::collections::HashMap<u32, [u32; 2]>,
) {
    println!();
    hr('═');
    let pct = if total > 0 { 100 * score / total } else { 0 };
    let col_result = if pct >= 70 { green(&bold(&format!("{}/{}", score, total))) }
                     else if pct >= 50 { yellow(&bold(&format!("{}/{}", score, total))) }
                     else { red(&bold(&format!("{}/{}", score, total))) };
    print!("\n  {}  {}  ({}%)", bold("Result:"), col_result, pct);

    print!("  {}", dim(&format!("Time: {:.0}s", duration_secs)));
    if streak_max > 1 { print!("   {}", yellow(&format!("Best streak: {}", streak_max))); }
    if n_generated > 0 { print!("   {}", dim(&format!("{} AI-generated", n_generated))); }
    println!();

    let diff_order = ["beginner", "intermediate", "advanced"];
    let parts: Vec<String> = diff_order.iter().filter_map(|&d| {
        per_difficulty.get(d).and_then(|v| {
            if v[1] > 0 {
                let p = 100 * v[0] / v[1];
                Some(format!("{}: {}/{} ({}%)", d, v[0], v[1], p))
            } else { None }
        })
    }).collect();
    if !parts.is_empty() {
        println!("  {}  {}", dim("By difficulty:"), dim(&parts.join(" · ")));
    }

    let meta = chapter_meta();
    if per_chapter.len() > 1 {
        println!("  {}", dim("By chapter:"));
        let mut chs: Vec<u32> = per_chapter.keys().copied().collect();
        chs.sort();
        for ch in chs {
            let [c, t] = per_chapter[&ch];
            let name = meta.get(&ch).map(|m| m.name).unwrap_or("Unknown");
            let ch_pct = if t > 0 { 100 * c / t } else { 0 };
            let col = if ch_pct >= 70 { green } else if ch_pct >= 50 { yellow } else { red };
            println!("    Ch.{:02} {:<24}  {}  ({}%)", ch, &name[..name.len().min(24)], col(&format!("{}/{}", c, t)), ch_pct);
        }
    }
    println!();

    if pct == 100 { println!("{}", green("  Perfect score!")); }
    else if pct >= 70 { println!("{}", yellow("  Good work. Review the topics you missed.")); }
    else { println!("{}", red("  Keep studying — re-read the relevant chapters and try again.")); }

    if !wrong_names.is_empty() {
        println!("\n  {}", bold("Topics to review:"));
        for name in wrong_names { println!("    • {}", name); }
    }
    hr('═');
    pause();
}

// ── Dashboard ─────────────────────────────────────────────────────────────────

pub fn show_dashboard(profile: &UserProfile) {
    let rows = mastery_summary(profile);
    section(&format!("Mastery Dashboard — {}", bold(&profile.name)));
    println!("  Total answered: {}", bold(&profile.total_answered().to_string()));
    let due_count = profile.chapters_due_for_review().len();
    if due_count > 0 {
        println!("  {}", yellow(&bold(&format!("{} topic(s) due for review today", due_count))));
    }
    println!();

    let pnames = phase_names();
    let mut current_phase = u32::MAX;

    for row in &rows {
        let ph = row["phase"].as_u64().unwrap_or(0) as u32;
        if ph != current_phase {
            current_phase = ph;
            let pname = pnames.get(&ph).map(|s| *s).unwrap_or("Unknown");
            println!("\n  {}:", cyan(&bold(pname)));
        }
        let ch     = row["chapter"].as_u64().unwrap_or(0) as u32;
        let score  = row["score"].as_f64().unwrap_or(0.0);
        let pct    = (score * 100.0) as usize;
        let fill   = (score * 20.0) as usize;
        let bar    = format!("{}{}", "█".repeat(fill), "░".repeat(20 - fill));
        let col    = if pct >= 70 { green(&bar) } else if pct >= 40 { yellow(&bar) } else { red(&bar) };
        let due    = if row["due"].as_bool().unwrap_or(false) { yellow(" ⟳") } else { "  ".to_string() };
        let star   = if row["started"].as_bool().unwrap_or(false) { "★" } else { "·" };
        let seen   = row["seen"].as_u64().unwrap_or(0);
        let seen_s = if seen > 0 { format!("({} seen)", seen) } else { "(not started)".into() };
        let name   = row["name"].as_str().unwrap_or("");

        println!(
            "    {} T{}.{:02} {:<26} {} {:3}%{}  {}",
            star, ph, ch, &name[..name.len().min(26)], col, pct, due, dim(&seen_s)
        );
    }
    println!();
    show_session_history(profile);
    hr('─');
    pause();
}

fn show_session_history(profile: &UserProfile) {
    let history = &profile.session_history;
    if history.is_empty() { return; }
    println!("\n  {}", cyan(&bold("Recent sessions:")));
    let start = if history.len() > 8 { history.len() - 8 } else { 0 };
    for sess in history[start..].iter().rev() {
        let ts      = &sess["timestamp"].as_str().unwrap_or("")[..10.min(sess["timestamp"].as_str().unwrap_or("").len())];
        let n_q     = sess["n_questions"].as_u64().unwrap_or(0) as usize;
        let n_c     = sess["n_correct"].as_u64().unwrap_or(0) as usize;
        let pct     = if n_q > 0 { 100 * n_c / n_q } else { 0 };
        let scope   = sess["scope_label"].as_str().unwrap_or("");
        let dur     = format!("{:.0}s", sess["duration_secs"].as_f64().unwrap_or(0.0));
        let streak  = sess["streak_max"].as_u64().unwrap_or(0);
        let col_pct = if pct >= 70 { green } else if pct >= 50 { yellow } else { red };
        let streak_s = if streak > 1 { format!(" streak:{}", streak) } else { String::new() };
        let scope_trunc = &scope[..scope.len().min(28)];
        println!(
            "    {}  {}  {}/{}  {:<28}  {}",
            dim(ts), col_pct(&format!("{:3}%", pct)), n_c, n_q,
            scope_trunc, dim(&format!("{}{}", dur, streak_s))
        );
    }

    if history.len() >= 4 {
        let avg_pct = |sessions: &[serde_json::Value]| -> f64 {
            let total_q: u64 = sessions.iter().map(|s| s["n_questions"].as_u64().unwrap_or(0)).sum();
            let total_c: u64 = sessions.iter().map(|s| s["n_correct"].as_u64().unwrap_or(0)).sum();
            100.0 * total_c as f64 / total_q.max(1) as f64
        };
        let n = history.len();
        let recent = avg_pct(&history[n-3..]);
        let prev   = avg_pct(&history[n.saturating_sub(6)..n-3]);
        let delta  = recent - prev;
        let trend  = if delta > 3.0 { green("↑ improving") }
                     else if delta < -3.0 { red("↓ declining") }
                     else { yellow("→ steady") };
        println!("  Trend (last 3 vs previous): {}", trend);
    }
    println!();
}

// ── Menus ─────────────────────────────────────────────────────────────────────

pub fn show_main_menu(profile_name: &str, api_available: bool, due_count: usize) -> &'static str {
    println!();
    hr('─');
    let api_note = if api_available {
        green("  (Claude AI enabled — dynamic questions on)")
    } else {
        yellow("  (set ANTHROPIC_API_KEY to enable AI-generated questions)")
    };
    println!("\n  {}  •  {}", bold(&magenta("CSSB Adaptive Quiz")), dim(profile_name));
    println!("{}", api_note);
    if due_count > 0 {
        println!("  {}", yellow(&bold(&format!("  {} topic(s) due for review", due_count))));
    }
    println!();
    println!("  {}. Adaptive quiz   {}", bold("1"), dim("(focuses on your weakest areas)"));
    println!("  {}. Custom quiz      {}", bold("2"), dim("(choose tier, topic, or keyword)"));
    println!("  {}. Study a topic    {}", bold("3"), dim("(read, then optionally quiz)"));
    println!("  {}. Mastery dashboard", bold("4"));
    println!("  {}. Switch profile", bold("5"));
    println!("  {}. Cathedral readiness {}", bold("6"), dim("(research project prerequisites)"));
    println!("  {}. Quit", bold("7"));
    println!();
    hr('─');
    loop {
        let ch = ask("  → ");
        match ch.as_str() {
            "1" => return "1", "2" => return "2", "3" => return "3",
            "4" => return "4", "5" => return "5", "6" => return "6",
            "7" => return "7",
            _ => println!("{}", yellow("  Please enter 1–7.")),
        }
    }
}

pub fn show_scope_menu() -> Scope {
    println!();
    hr('─');
    section("Choose scope");
    println!("  [1] All topics");
    println!("  [2] A specific tier");
    println!("  [3] A specific topic");
    println!("  [4] A keyword / tag");
    hr('─');
    let ch = ask("  → ");

    match ch.as_str() {
        "2" => {
            println!();
            for (k, v) in phase_names() { println!("    [{}] {}", k, v); }
            let ph = ask_int("\n  Tier number → ", 0, 5, Some(0)) as u32;
            Scope::phase(ph)
        }
        "3" => {
            println!();
            let meta = chapter_meta();
            let mut chs: Vec<u32> = meta.keys().copied().collect();
            chs.sort();
            for ch in &chs {
                let m = &meta[ch];
                println!("    [{:2}] T{}  {}", ch, m.phase, m.name);
            }
            let max_ch = *chs.last().unwrap_or(&0) as i32;
            let ch_num = ask_int("\n  Topic number → ", 0, max_ch, Some(0)) as u32;
            Scope::chapter(ch_num)
        }
        "4" => {
            let tag = ask("  Keyword → ");
            if tag.is_empty() { Scope::all() } else { Scope::tag(tag) }
        }
        _ => Scope::all(),
    }
}

pub fn show_chapter_menu() -> u32 {
    println!();
    hr('─');
    section("Choose a topic to study");
    let meta = chapter_meta();
    let mut chs: Vec<u32> = meta.keys().copied().collect();
    chs.sort();
    for ch in &chs {
        let m = &meta[ch];
        println!("    [{:2}] T{}  {}", ch, m.phase, m.name);
    }
    hr('─');
    let max_ch = *chs.last().unwrap_or(&0) as i32;
    ask_int("  Topic number → ", 0, max_ch, Some(0)) as u32
}

pub fn show_study_mode(chapter: u32) -> bool {
    use crate::config::chapters_dir;
    let meta = chapter_meta();
    let Some(m) = meta.get(&chapter) else {
        print_wrap(&format!("Topic {} not found.", chapter));
        return false;
    };
    let path = chapters_dir().join(m.file);
    if !path.exists() {
        print_wrap(&format!("Topic file not found: {}", m.file));
        return false;
    }
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    // strip code blocks
    let prose = strip_code_blocks(&raw);
    let truncated = prose.len() > 6_000;
    let prose = &prose[..prose.len().min(6_000)];

    section(&format!("Topic {:02} — {}", chapter, m.name));
    for paragraph in prose.split("\n\n") {
        let paragraph = paragraph.trim();
        if paragraph.is_empty() { continue; }
        if paragraph.starts_with('#') {
            println!("\n  {}\n", bold(&cyan(paragraph.trim_start_matches('#').trim())));
        } else if paragraph.starts_with("[code") {
            println!("  {}\n", dim(paragraph));
        } else {
            print_wrap(paragraph);
            println!();
        }
    }
    if truncated {
        println!("{}", dim("  [Content truncated — see the full chapter file for complete coverage.]\n"));
    }
    hr('─');
    confirm("Quiz on this topic now?", true)
}

fn strip_code_blocks(text: &str) -> String {
    let mut out = String::new();
    let mut in_block = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_block = !in_block;
            if in_block { out.push_str("\n[code block omitted]\n"); }
        } else if !in_block {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

pub fn show_cathedral_readiness(profile: &UserProfile, cathedral_id: &str) {
    let prereqs = cathedral_prereqs();
    let uid = cathedral_id.to_uppercase();
    let data = match prereqs.get(uid.as_str()) {
        Some(d) => d,
        None => {
            let mut avail_keys: Vec<&str> = prereqs.keys().copied().collect();
            avail_keys.sort();
            let avail = avail_keys.join(", ");
            print_wrap(&format!("Unknown cathedral '{}'. Available: {}", cathedral_id, avail));
            return;
        }
    };

    section(&format!("Cathedral {} — {}", uid, data.title));
    print_wrap(data.description);
    println!();

    let ready_threshold = DIFF_INTERMEDIATE_MAX;
    let meta = chapter_meta();
    let mut all_ready = true;

    for &ch in data.chapters {
        let name = meta.get(&ch).map(|m| m.name).unwrap_or("Unknown");
        let rec  = profile.mastery.get(&ch);
        let score = rec.map(|r| r.score).unwrap_or(0.0);
        let seen  = rec.map(|r| r.total_seen).unwrap_or(0);
        let pct   = (score * 100.0) as usize;
        let fill  = (score * 20.0) as usize;
        let bar   = format!("{}{}", "█".repeat(fill), "░".repeat(20 - fill));

        let (col_bar, flag) = if score >= ready_threshold {
            (green(&bar), green("  ✓ ready"))
        } else if seen == 0 {
            all_ready = false;
            (dim(&bar), yellow("  ○ not started"))
        } else {
            all_ready = false;
            let col = if score >= DIFF_BEGINNER_MAX { yellow(&bar) } else { red(&bar) };
            (col, yellow("  ↑ needs work"))
        };

        println!("    Ch.{:02} {:<28} {} {:3}%{}", ch, &name[..name.len().min(28)], col_bar, pct, flag);
    }

    println!();
    if all_ready {
        println!("{}", green(&bold("  ✓ All prerequisites met — you are ready to begin this cathedral.")));
    } else {
        let missing = data.chapters.iter()
            .filter(|&&ch| profile.mastery.get(&ch).map(|r| r.score).unwrap_or(0.0) < ready_threshold)
            .count();
        println!("{}", yellow(&format!("  {} chapter(s) below the readiness threshold (≥{}%).",
            missing, (ready_threshold * 100.0) as usize)));
        print_wrap("Work through those chapters in the quiz until mastery turns green, then return.");
    }
    println!();
    hr('─');
    pause();
}

pub fn show_cathedral_menu(profile: &UserProfile) {
    let prereqs = cathedral_prereqs();
    section("Cathedral Readiness");
    print_wrap(
        "Each cathedral is a substantial research project. The table below shows \
         how ready you are based on prerequisite chapter mastery (≥68% = ready)."
    );
    println!();

    for (cid, data) in prereqs.iter() {
        let scores: Vec<f64> = data.chapters.iter()
            .map(|&ch| profile.mastery.get(&ch).map(|r| r.score).unwrap_or(0.0))
            .collect();
        let ready_count = scores.iter().filter(|&&s| s >= DIFF_INTERMEDIATE_MAX).count();
        let total = scores.len();
        let overall = ready_count as f64 / total.max(1) as f64;
        let fill = (overall * 12.0) as usize;
        let bar = format!("{}{}", "█".repeat(fill), "░".repeat(12 - fill));
        let col = if ready_count == total { green(&bar) }
                  else if ready_count > 0 { yellow(&bar) }
                  else { red(&bar) };
        println!("  [{}] {:<38}  {}  {}/{} prereqs", cid, &data.title[..data.title.len().min(38)], col, ready_count, total);
    }

    println!();
    hr('─');
    let cid = ask("  Cathedral ID (I–VII) or Enter to go back → ").to_uppercase();
    if prereqs.contains_key(cid.as_str()) {
        show_cathedral_readiness(profile, &cid);
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= max_width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current = word.to_string();
        }
    }
    if !current.is_empty() { lines.push(current); }
    lines
}
