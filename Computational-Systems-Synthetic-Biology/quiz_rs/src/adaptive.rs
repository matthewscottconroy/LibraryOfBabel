use rand::Rng;

use crate::config::{
    BLANK_WEIGHT_BONUS, DIFF_BEGINNER_MAX, DIFF_INTERMEDIATE_MAX, INITIAL_MASTERY,
    RECENCY_PENALTY, RECENCY_WINDOW,
};
use crate::models::{Question, UserProfile};

// ── Scope ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum ScopeKind {
    All,
    Phase(u32),
    Chapter(u32),
    Tag(String),
    Adaptive,
    Due,
    Review(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub kind: ScopeKind,
}

impl Scope {
    pub fn all()                             -> Self { Self { kind: ScopeKind::All } }
    pub fn phase(ph: u32)                    -> Self { Self { kind: ScopeKind::Phase(ph) } }
    pub fn chapter(ch: u32)                  -> Self { Self { kind: ScopeKind::Chapter(ch) } }
    pub fn tag(t: String)                    -> Self { Self { kind: ScopeKind::Tag(t) } }
    pub fn adaptive()                        -> Self { Self { kind: ScopeKind::Adaptive } }
    pub fn due()                             -> Self { Self { kind: ScopeKind::Due } }
    pub fn review(ids: Vec<String>)          -> Self { Self { kind: ScopeKind::Review(ids) } }

    pub fn label(&self) -> String {
        use crate::config::{chapter_meta, phase_names};
        match &self.kind {
            ScopeKind::Adaptive  => "Adaptive (focus on weaknesses)".into(),
            ScopeKind::All       => "All chapters".into(),
            ScopeKind::Phase(ph) => {
                phase_names().get(ph).map(|s| s.to_string())
                    .unwrap_or_else(|| format!("Phase {}", ph))
            }
            ScopeKind::Chapter(ch) => {
                let name = chapter_meta().get(ch)
                    .map(|m| m.name.to_string())
                    .unwrap_or_else(|| format!("Ch.{}", ch));
                format!("Ch.{} — {}", ch, name)
            }
            ScopeKind::Tag(t)      => format!("Tag: {}", t),
            ScopeKind::Due         => "Due for review".into(),
            ScopeKind::Review(ids) => format!("Re-quiz wrong answers ({})", ids.len()),
        }
    }

    pub fn is_adaptive(&self) -> bool {
        matches!(self.kind, ScopeKind::Adaptive)
    }
}

// ── Difficulty helpers ────────────────────────────────────────────────────────

pub fn preferred_difficulty(mastery: f64) -> &'static str {
    if mastery < DIFF_BEGINNER_MAX    { "beginner" }
    else if mastery < DIFF_INTERMEDIATE_MAX { "intermediate" }
    else                              { "advanced" }
}

pub fn difficulty_weight(question_diff: &str, mastery: f64) -> f64 {
    let preferred = preferred_difficulty(mastery);
    if question_diff == preferred { return 1.0; }
    let diffs = ["beginner", "intermediate", "advanced"];
    let qi = diffs.iter().position(|&d| d == question_diff).unwrap_or(0) as i32;
    let pi = diffs.iter().position(|&d| d == preferred).unwrap_or(1) as i32;
    let distance = (qi - pi).abs();
    if distance == 1 { 0.35 } else { 0.05 }
}

// ── Phase gating ──────────────────────────────────────────────────────────────

pub fn user_phase_frontier(profile: &UserProfile) -> u32 {
    use crate::config::chapter_meta;
    let meta = chapter_meta();
    profile.mastery.iter()
        .filter(|(ch, rec)| meta.contains_key(ch) && rec.total_seen >= 3)
        .map(|(ch, _)| meta[ch].phase)
        .max()
        .unwrap_or(0)
}

fn phase_gate_factor(chapter: u32, frontier: u32) -> f64 {
    use crate::config::chapter_meta;
    let ch_phase = chapter_meta().get(&chapter).map(|m| m.phase).unwrap_or(0);
    if ch_phase <= frontier + 1 { return 1.0; }
    let gap = ch_phase - frontier;
    match gap {
        2 => 0.5,
        3 => 0.2,
        _ => 0.05,
    }
}

// ── Core selector ─────────────────────────────────────────────────────────────

fn chapter_mastery(profile: &UserProfile, chapter: u32) -> f64 {
    profile.mastery.get(&chapter).map(|r| r.score).unwrap_or(INITIAL_MASTERY)
}

fn chapter_weight(profile: &UserProfile, chapter: u32, adaptive: bool, frontier: u32) -> f64 {
    if !adaptive { return 1.0; }

    let base = if !profile.mastery.contains_key(&chapter) {
        0.8
    } else {
        let mastery = chapter_mastery(profile, chapter);
        (1.2 - mastery).max(0.1)
    };

    if let Some(rec) = profile.mastery.get(&chapter) {
        if rec.is_due() { return base * 2.0; }
    }

    base * phase_gate_factor(chapter, frontier)
}

fn recency_factor(question_id: &str, profile: &UserProfile) -> f64 {
    let recent = &profile.seen_question_ids;
    let start = if recent.len() > RECENCY_WINDOW { recent.len() - RECENCY_WINDOW } else { 0 };
    if recent[start..].contains(&question_id.to_string()) {
        RECENCY_PENALTY
    } else {
        1.0
    }
}

fn question_sr_factor(question_id: &str, profile: &UserProfile) -> f64 {
    match profile.question_mastery.get(question_id) {
        None => 1.0,
        Some(rec) if rec.is_due() => 3.0,
        Some(rec) if !rec.next_review.is_empty() => 0.7,
        _ => 1.0,
    }
}

pub fn filter_by_scope<'a>(questions: &'a [Question], scope: &Scope, profile: Option<&UserProfile>) -> Vec<&'a Question> {
    match &scope.kind {
        ScopeKind::All | ScopeKind::Adaptive => questions.iter().collect(),
        ScopeKind::Phase(ph) => questions.iter().filter(|q| q.phase == *ph).collect(),
        ScopeKind::Chapter(ch) => questions.iter().filter(|q| q.chapter == *ch).collect(),
        ScopeKind::Tag(tag) => {
            let t = tag.to_lowercase();
            questions.iter().filter(|q| q.tags.iter().any(|qt| qt.to_lowercase().contains(&t))).collect()
        }
        ScopeKind::Due => {
            if let Some(p) = profile {
                let due_chs: std::collections::HashSet<u32> =
                    p.chapters_due_for_review().into_iter().collect();
                questions.iter().filter(|q| due_chs.contains(&q.chapter)).collect()
            } else {
                questions.iter().collect()
            }
        }
        ScopeKind::Review(ids) => {
            let id_set: std::collections::HashSet<&str> = ids.iter().map(|s| s.as_str()).collect();
            questions.iter().filter(|q| id_set.contains(q.question_id.as_str())).collect()
        }
    }
}

pub fn select_questions<'a>(
    pool: &'a [Question],
    profile: &UserProfile,
    n: usize,
    scope: &Scope,
) -> Vec<&'a Question> {
    let mut rng = rand::thread_rng();
    let candidates = filter_by_scope(pool, scope, Some(profile));
    if candidates.is_empty() { return Vec::new(); }

    let adaptive = scope.is_adaptive();
    let frontier = if adaptive { user_phase_frontier(profile) } else { 0 };

    let weights: Vec<f64> = candidates.iter().map(|q| {
        let ch_mastery = chapter_mastery(profile, q.chapter);
        let blank_bonus = if q.kind == "blank" { BLANK_WEIGHT_BONUS } else { 1.0 };
        let w = chapter_weight(profile, q.chapter, adaptive, frontier)
            * difficulty_weight(&q.difficulty, ch_mastery)
            * recency_factor(&q.question_id, profile)
            * question_sr_factor(&q.question_id, profile)
            * blank_bonus;
        w.max(1e-6)
    }).collect();

    let n = n.min(candidates.len());
    let mut result = Vec::with_capacity(n);
    let mut remaining: Vec<(&Question, f64)> =
        candidates.into_iter().zip(weights).collect();

    for _ in 0..n {
        let total: f64 = remaining.iter().map(|(_, w)| w).sum();
        let r = rng.gen::<f64>() * total;
        let mut cumul = 0.0;
        let mut chosen = remaining.len() - 1;
        for (i, (_, w)) in remaining.iter().enumerate() {
            cumul += w;
            if r <= cumul { chosen = i; break; }
        }
        let (q, _) = remaining.remove(chosen);
        result.push(q);
    }
    result
}

// ── Summary helpers ───────────────────────────────────────────────────────────

pub fn mastery_summary(profile: &UserProfile) -> Vec<serde_json::Value> {
    use crate::config::chapter_meta;
    let meta = chapter_meta();
    let mut chs: Vec<u32> = meta.keys().copied().collect();
    chs.sort();
    chs.iter().map(|&ch| {
        let m = &meta[&ch];
        let rec = profile.mastery.get(&ch);
        serde_json::json!({
            "chapter":     ch,
            "phase":       m.phase,
            "name":        m.name,
            "score":       rec.map(|r| r.score).unwrap_or(INITIAL_MASTERY),
            "seen":        rec.map(|r| r.total_seen).unwrap_or(0),
            "correct":     rec.map(|r| r.total_correct).unwrap_or(0),
            "started":     rec.is_some(),
            "due":         rec.map(|r| r.is_due()).unwrap_or(false),
            "next_review": rec.map(|r| r.next_review.as_str()).unwrap_or(""),
        })
    }).collect()
}

pub fn weak_topics(profile: &UserProfile, n: usize) -> Vec<String> {
    let mut rows = mastery_summary(profile);
    rows.retain(|r| r["started"].as_bool().unwrap_or(false));
    rows.sort_by(|a, b| {
        a["score"].as_f64().unwrap_or(0.0)
            .partial_cmp(&b["score"].as_f64().unwrap_or(0.0))
            .unwrap()
    });
    rows.iter().take(n)
        .map(|r| r["name"].as_str().unwrap_or("").to_string())
        .collect()
}
