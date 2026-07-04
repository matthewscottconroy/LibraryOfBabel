//! Adaptive question-selection engine.
//!
//! Selects questions from a pool based on:
//!
//! 1. **Chapter mastery** — weakest chapters get higher weight.
//! 2. **Difficulty match** — preferred difficulty matched to mastery score.
//! 3. **Freshness** — recently seen questions are de-prioritised.
//! 4. **Phase gating** — chapters far ahead of the user's frontier are suppressed.
//! 5. **Spaced repetition (chapter)** — chapters due for review get a 2× boost.
//! 6. **Spaced repetition (question)** — individual questions due for SR get 3×.
//! 7. **Fill-blank bonus** — FillBlank questions get a 1.5× weight bonus.
//! 8. **Scope** — the user's chosen chapter/phase/tag filter is applied first.

use std::collections::HashMap;

use rand::Rng;

use crate::mastery::INITIAL_MASTERY;
use crate::profile::UserProfile;
use crate::question::{Difficulty, Question};

// ── Constants ─────────────────────────────────────────────────────────────────

/// Mastery thresholds that determine preferred difficulty.
pub const DIFF_BEGINNER_MAX: f64 = 0.35;
pub const DIFF_INTERMEDIATE_MAX: f64 = 0.68;

/// Weight multiplier for fill-in-the-blank questions.
pub const BLANK_WEIGHT_BONUS: f64 = 1.5;

/// Weight assigned to a question seen in the last `RECENCY_WINDOW` answers.
pub const RECENCY_PENALTY: f64 = 0.05;

/// Number of recent question IDs considered "fresh" for the recency penalty.
pub const RECENCY_WINDOW: usize = 60;

// ── Scope ─────────────────────────────────────────────────────────────────────

/// Describes which portion of the question bank to quiz on.
#[derive(Debug, Clone)]
pub enum Scope {
    /// All questions (no filter, no adaptive weighting).
    All,
    /// Only questions in a given curriculum phase.
    Phase(u32),
    /// Only questions in a specific chapter.
    Chapter(u32),
    /// Only questions that carry a specific tag (case-insensitive substring).
    Tag(String),
    /// All questions, with full adaptive weighting (default play mode).
    Adaptive,
    /// Only questions from chapters that are due for spaced-repetition review.
    Due,
    /// A specific list of question IDs (e.g. "re-quiz wrong answers").
    Review(Vec<String>),
}

impl Scope {
    pub fn label(
        &self,
        chapter_name: impl Fn(u32) -> String,
        phase_name: impl Fn(u32) -> String,
    ) -> String {
        match self {
            Self::All => "All chapters".into(),
            Self::Adaptive => "Adaptive (focus on weaknesses)".into(),
            Self::Due => "Due for review".into(),
            Self::Phase(p) => phase_name(*p),
            Self::Chapter(c) => format!("Ch.{} — {}", c, chapter_name(*c)),
            Self::Tag(t) => format!("Tag: {}", t),
            Self::Review(ids) => format!("Re-quiz wrong answers ({})", ids.len()),
        }
    }
}

// ── Chapter metadata trait ────────────────────────────────────────────────────

/// Minimal view of a chapter's curriculum position, needed by the adaptive
/// engine.  Callers supply this via a `HashMap<u32, ChapterInfo>`.
#[derive(Debug, Clone)]
pub struct ChapterInfo {
    pub phase: u32,
    pub name: String,
}

// ── Preferred difficulty ──────────────────────────────────────────────────────

/// Map a mastery score to the preferred difficulty tier.
pub fn preferred_difficulty(mastery: f64) -> Difficulty {
    if mastery < DIFF_BEGINNER_MAX {
        Difficulty::Beginner
    } else if mastery < DIFF_INTERMEDIATE_MAX {
        Difficulty::Intermediate
    } else {
        Difficulty::Advanced
    }
}

/// Weight for a question given the user's current mastery for that chapter.
///
/// Preferred difficulty → 1.0; 1 level away → 0.35; 2 levels away → 0.05.
pub fn difficulty_weight(question_diff: &Difficulty, mastery: f64) -> f64 {
    let pref = preferred_difficulty(mastery);
    match pref.distance(question_diff) {
        0 => 1.0,
        1 => 0.35,
        _ => 0.05,
    }
}

// ── Phase gating ──────────────────────────────────────────────────────────────

/// The highest phase index where the user has seen ≥ 3 questions in at least
/// one chapter belonging to that phase.
pub fn user_phase_frontier(
    profile: &UserProfile,
    chapter_info: &HashMap<u32, ChapterInfo>,
) -> u32 {
    profile
        .mastery
        .iter()
        .filter(|(ch, rec)| {
            chapter_info
                .get(ch)
                .map_or(false, |_| rec.total_seen >= 3)
        })
        .filter_map(|(ch, _)| chapter_info.get(ch).map(|m| m.phase))
        .max()
        .unwrap_or(0)
}

/// Reduce weight for chapters far ahead of the user's current phase frontier.
///
/// | gap (ch_phase − frontier) | factor |
/// |---------------------------|--------|
/// | ≤ 1                       | 1.0    |
/// | 2                         | 0.5    |
/// | 3                         | 0.2    |
/// | ≥ 4                       | 0.05   |
fn phase_gate_factor(chapter: u32, frontier: u32, chapter_info: &HashMap<u32, ChapterInfo>) -> f64 {
    let ch_phase = chapter_info
        .get(&chapter)
        .map_or(0, |m| m.phase);
    let gap = ch_phase.saturating_sub(frontier);
    match gap {
        0 | 1 => 1.0,
        2 => 0.5,
        3 => 0.2,
        _ => 0.05,
    }
}

// ── Per-chapter weight ────────────────────────────────────────────────────────

fn chapter_weight(
    profile: &UserProfile,
    chapter: u32,
    adaptive: bool,
    frontier: u32,
    chapter_info: &HashMap<u32, ChapterInfo>,
) -> f64 {
    if !adaptive {
        return 1.0;
    }

    let base = if !profile.mastery.contains_key(&chapter) {
        0.8 // exploration boost for unseen chapters
    } else {
        let mastery = profile.chapter_score(chapter);
        (1.2 - mastery).max(0.1)
    };

    // Review boost: SR-due chapters override phase gating
    if profile
        .mastery
        .get(&chapter)
        .map_or(false, |r| r.is_due())
    {
        return base * 2.0;
    }

    base * phase_gate_factor(chapter, frontier, chapter_info)
}

// ── Per-question SR factor ────────────────────────────────────────────────────

fn question_sr_factor(question_id: &str, profile: &UserProfile) -> f64 {
    match profile.question_mastery.get(question_id) {
        None => 1.0,
        Some(rec) if rec.is_due() => 3.0,
        Some(rec) if !rec.next_review.is_empty() => 0.7,
        _ => 1.0,
    }
}

// ── Recency factor ────────────────────────────────────────────────────────────

fn recency_factor(question_id: &str, profile: &UserProfile) -> f64 {
    let recent_start = profile
        .seen_question_ids
        .len()
        .saturating_sub(RECENCY_WINDOW);
    let recent = &profile.seen_question_ids[recent_start..];
    if recent.iter().any(|id| id == question_id) {
        RECENCY_PENALTY
    } else {
        1.0
    }
}

// ── Scope filter ──────────────────────────────────────────────────────────────

/// Reduce the question pool to those that match the chosen scope.
pub fn filter_by_scope<'a>(
    questions: &'a [Question],
    scope: &Scope,
    profile: &UserProfile,
) -> Vec<&'a Question> {
    match scope {
        Scope::All | Scope::Adaptive => questions.iter().collect(),
        Scope::Phase(ph) => questions.iter().filter(|q| q.phase() == *ph).collect(),
        Scope::Chapter(ch) => questions.iter().filter(|q| q.chapter() == *ch).collect(),
        Scope::Tag(tag) => {
            let tag_lower = tag.to_lowercase();
            questions
                .iter()
                .filter(|q| q.tags().iter().any(|t| t.to_lowercase().contains(&tag_lower)))
                .collect()
        }
        Scope::Due => {
            let due: std::collections::HashSet<u32> = profile
                .chapters_due_for_review()
                .into_iter()
                .collect();
            questions.iter().filter(|q| due.contains(&q.chapter())).collect()
        }
        Scope::Review(ids) => {
            let id_set: std::collections::HashSet<&str> =
                ids.iter().map(|s| s.as_str()).collect();
            questions
                .iter()
                .filter(|q| id_set.contains(q.question_id()))
                .collect()
        }
    }
}

// ── Core selector ─────────────────────────────────────────────────────────────

/// Return up to `n` questions sampled from `pool` according to the adaptive
/// weighting policy.  Sampling is weighted and **without replacement**.
///
/// `chapter_info` maps chapter index to phase + name metadata and is needed for
/// phase-gating and frontier calculation.
pub fn select_questions<R: Rng>(
    pool: &[Question],
    profile: &UserProfile,
    n: usize,
    scope: &Scope,
    chapter_info: &HashMap<u32, ChapterInfo>,
    rng: &mut R,
) -> Vec<Question> {
    let candidates = filter_by_scope(pool, scope, profile);
    if candidates.is_empty() {
        return Vec::new();
    }

    let adaptive = matches!(scope, Scope::Adaptive);
    let frontier = if adaptive {
        user_phase_frontier(profile, chapter_info)
    } else {
        0
    };

    // Build weight vector
    let weights: Vec<f64> = candidates
        .iter()
        .map(|q| {
            let ch_mastery = profile.chapter_score(q.chapter());
            let blank_bonus = if q.kind_str() == "blank" { BLANK_WEIGHT_BONUS } else { 1.0 };
            let w = chapter_weight(profile, q.chapter(), adaptive, frontier, chapter_info)
                * difficulty_weight(q.difficulty(), ch_mastery)
                * recency_factor(q.question_id(), profile)
                * question_sr_factor(q.question_id(), profile)
                * blank_bonus;
            w.max(1e-6) // never exactly zero
        })
        .collect();

    // Weighted sampling without replacement
    let take = n.min(candidates.len());
    let mut remaining_idx: Vec<usize> = (0..candidates.len()).collect();
    let mut remaining_weights: Vec<f64> = weights;
    let mut result: Vec<Question> = Vec::with_capacity(take);

    for _ in 0..take {
        let total: f64 = remaining_weights.iter().sum();
        let r: f64 = rng.gen_range(0.0..total);
        let mut cumul = 0.0;
        // Default to last index as floating-point fallback
        let mut chosen = remaining_idx.len() - 1;
        for (i, &w) in remaining_weights.iter().enumerate() {
            cumul += w;
            if r <= cumul {
                chosen = i;
                break;
            }
        }
        let q_idx = remaining_idx.remove(chosen);
        remaining_weights.remove(chosen);
        result.push(candidates[q_idx].clone());
    }

    result
}

// ── Summary helpers ───────────────────────────────────────────────────────────

/// A row in the per-chapter mastery table.
#[derive(Debug, Clone)]
pub struct MasteryRow {
    pub chapter: u32,
    pub phase: u32,
    pub name: String,
    pub score: f64,
    pub seen: u32,
    pub correct: u32,
    pub started: bool,
    pub due: bool,
    pub next_review: String,
}

/// Return per-chapter mastery rows for every chapter defined in `chapter_info`,
/// sorted by chapter index.
pub fn mastery_summary(
    profile: &UserProfile,
    chapter_info: &HashMap<u32, ChapterInfo>,
) -> Vec<MasteryRow> {
    let mut chapters: Vec<u32> = chapter_info.keys().copied().collect();
    chapters.sort_unstable();

    chapters
        .into_iter()
        .map(|ch| {
            let meta = &chapter_info[&ch];
            let rec = profile.mastery.get(&ch);
            MasteryRow {
                chapter: ch,
                phase: meta.phase,
                name: meta.name.clone(),
                score: rec.map_or(INITIAL_MASTERY, |r| r.score),
                seen: rec.map_or(0, |r| r.total_seen),
                correct: rec.map_or(0, |r| r.total_correct),
                started: rec.is_some(),
                due: rec.map_or(false, |r| r.is_due()),
                next_review: rec.map_or_else(String::new, |r| r.next_review.clone()),
            }
        })
        .collect()
}

/// Return the names of the `n` weakest **started** chapters (lowest score
/// first).
pub fn weak_topics(
    profile: &UserProfile,
    chapter_info: &HashMap<u32, ChapterInfo>,
    n: usize,
) -> Vec<String> {
    let mut rows: Vec<MasteryRow> = mastery_summary(profile, chapter_info)
        .into_iter()
        .filter(|r| r.started)
        .collect();
    rows.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal));
    rows.truncate(n);
    rows.into_iter().map(|r| r.name).collect()
}

/// Return the top `top_n` weakest tags by average mastery over the chapters
/// that contain at least one question with that tag.
pub fn tag_weakness(
    profile: &UserProfile,
    questions: &[Question],
    top_n: usize,
) -> Vec<(String, f64)> {
    let mut tag_chapters: HashMap<String, std::collections::HashSet<u32>> = HashMap::new();
    for q in questions {
        for tag in q.tags() {
            tag_chapters
                .entry(tag.clone())
                .or_default()
                .insert(q.chapter());
        }
    }

    let mut rows: Vec<(String, f64)> = tag_chapters
        .into_iter()
        .filter_map(|(tag, chapters)| {
            let started: Vec<u32> = chapters
                .into_iter()
                .filter(|ch| profile.mastery.contains_key(ch))
                .collect();
            if started.is_empty() {
                return None;
            }
            let avg = started.iter().map(|ch| profile.chapter_score(*ch)).sum::<f64>()
                / started.len() as f64;
            Some((tag, avg))
        })
        .collect();

    rows.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    rows.truncate(top_n);
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::question::Question;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn mc(id: &str, chapter: u32, phase: u32) -> Question {
        Question::MultipleChoice {
            question_id: id.to_string(),
            chapter,
            phase,
            text: format!("Q for ch{chapter}"),
            choices: vec!["A".into(), "B".into(), "C".into(), "D".into()],
            answer: 0,
            explanation: "because".into(),
            tags: vec![],
            difficulty: Difficulty::Intermediate,
            generated: false,
        }
    }

    fn chapter_info() -> HashMap<u32, ChapterInfo> {
        let mut m = HashMap::new();
        m.insert(0, ChapterInfo { phase: 0, name: "near-a".into() });
        m.insert(1, ChapterInfo { phase: 0, name: "near-b".into() });
        m.insert(10, ChapterInfo { phase: 5, name: "far-a".into() });
        m.insert(11, ChapterInfo { phase: 5, name: "far-b".into() });
        m
    }

    #[test]
    fn empty_pool_never_panics() {
        let profile = UserProfile::new("t");
        let info = chapter_info();
        let mut rng = StdRng::seed_from_u64(1);
        let out = select_questions(&[], &profile, 10, &Scope::Adaptive, &info, &mut rng);
        assert!(out.is_empty());
    }

    #[test]
    fn scope_filtering_to_empty_never_panics() {
        let pool = vec![mc("a", 0, 0), mc("b", 1, 0)];
        let profile = UserProfile::new("t");
        let info = chapter_info();
        let mut rng = StdRng::seed_from_u64(2);
        // No questions in chapter 99 → empty result, no panic.
        let out = select_questions(&pool, &profile, 5, &Scope::Chapter(99), &info, &mut rng);
        assert!(out.is_empty());
    }

    #[test]
    fn selection_returns_at_most_n_without_replacement() {
        let pool = vec![mc("a", 0, 0), mc("b", 1, 0), mc("c", 10, 5)];
        let profile = UserProfile::new("t");
        let info = chapter_info();
        let mut rng = StdRng::seed_from_u64(3);
        let out = select_questions(&pool, &profile, 2, &Scope::All, &info, &mut rng);
        assert_eq!(out.len(), 2);
        // Without replacement: the two ids differ.
        assert_ne!(out[0].question_id(), out[1].question_id());
    }

    #[test]
    fn adaptive_respects_phase_gating() {
        // Two near (phase 0) and two far (phase 5) questions; a fresh user's
        // phase frontier is 0, so far-ahead phases are heavily suppressed.
        let pool = vec![
            mc("near-a", 0, 0),
            mc("near-b", 1, 0),
            mc("far-a", 10, 5),
            mc("far-b", 11, 5),
        ];
        let profile = UserProfile::new("t");
        let info = chapter_info();
        let mut rng = StdRng::seed_from_u64(42);

        let mut near = 0u32;
        let mut far = 0u32;
        for _ in 0..300 {
            let picked = select_questions(&pool, &profile, 1, &Scope::Adaptive, &info, &mut rng);
            match picked[0].chapter() {
                0 | 1 => near += 1,
                _ => far += 1,
            }
        }
        // Phase gating gives near chapters ~20x the weight of far ones, so
        // near questions must dominate by a wide margin.
        assert!(
            near > far * 3,
            "phase gating should favour near chapters: near={near} far={far}"
        );
    }
}
