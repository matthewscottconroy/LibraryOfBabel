use std::collections::HashMap;
use std::time::Instant;

use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::adaptive::{select_questions, weak_topics, Scope, ScopeKind};
use crate::config::{
    chapter_meta, AUTO_STOP_THRESHOLD, AUTO_STOP_WINDOW, LEARNING_RATE_CORRECT, LEARNING_RATE_WRONG,
};
use crate::generator::ClaudeGenerator;
use crate::models::{now_utc, Question, SessionResult, UserProfile};
use crate::question_bank::load_questions;
use crate::storage::save_profile;
use crate::ui;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn question_text_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.to_lowercase().trim().as_bytes());
    hex::encode(&hasher.finalize()[..8])
}

fn interleave(a: Vec<Question>, b: Vec<Question>) -> Vec<Question> {
    if b.is_empty() { return a; }
    let mut combined = Vec::with_capacity(a.len() + b.len());
    let step = (a.len() / (b.len() + 1)).max(1);
    let mut bi = 0;
    for (i, item) in a.into_iter().enumerate() {
        combined.push(item);
        if bi < b.len() && (i + 1) % step == 0 {
            combined.push(b[bi].clone());
            bi += 1;
        }
    }
    combined.extend_from_slice(&b[bi..]);
    combined
}

// ── QuizSession ───────────────────────────────────────────────────────────────

pub struct QuizSession {
    profile:         UserProfile,
    scope:           Scope,
    n:               usize,
    generator:       Option<ClaudeGenerator>,
    generated_ratio: f64,
    adaptive_stop:   bool,
}

impl QuizSession {
    pub fn new(
        profile: UserProfile,
        scope: Scope,
        n: usize,
        generator: Option<ClaudeGenerator>,
    ) -> Self {
        Self { profile, scope, n, generator, generated_ratio: 0.35, adaptive_stop: true }
    }

    pub fn run(mut self) -> (SessionResult, UserProfile) {
        let bank = load_questions();
        let questions = self.assemble_questions(&bank);

        if questions.is_empty() {
            ui::print_wrap("No questions match your selection. Try a broader scope.");
            let result = self.empty_result();
            return (result, self.profile);
        }

        let mut score:          usize = 0;
        let mut streak:         u32   = 0;
        let mut streak_max:     u32   = 0;
        let mut wrong_chs:      Vec<u32> = Vec::new();
        let mut n_generated:    u32   = 0;
        let mut mastery_deltas: Vec<f64> = Vec::new();
        let mut per_diff:    HashMap<String, [u32; 2]> = HashMap::new();
        let mut per_chapter: HashMap<u32, [u32; 2]>   = HashMap::new();
        let start = Instant::now();
        let total = questions.len();

        let mut i = 0;
        while i < total {
            let q = &questions[i];
            i += 1;

            let score_before = self.profile.mastery.get(&q.chapter).map(|r| r.score).unwrap_or(0.5);

            match ui::present_question(q, i, total, Some(score)) {
                Err(ui::QuizQuit) => break,
                Ok((correct, flagged, confidence)) => {
                    self.profile.record_answer(
                        q.chapter, correct, &q.question_id,
                        LEARNING_RATE_CORRECT, LEARNING_RATE_WRONG, confidence,
                    );
                    if correct && !flagged {
                        self.profile.remove_from_review_queue(&q.question_id);
                    } else {
                        self.profile.add_to_review_queue(&q.question_id);
                    }
                    if q.generated && !correct {
                        let h = question_text_hash(&q.text);
                        *self.profile.generated_strikes.entry(h).or_insert(0) += 1;
                    }

                    let diff_bucket = per_diff.entry(q.difficulty.clone()).or_insert([0, 0]);
                    let ch_bucket   = per_chapter.entry(q.chapter).or_insert([0, 0]);
                    diff_bucket[1] += 1;
                    ch_bucket[1]   += 1;
                    if correct {
                        score += 1;
                        diff_bucket[0] += 1;
                        ch_bucket[0]   += 1;
                        streak += 1;
                        streak_max = streak_max.max(streak);
                    } else {
                        wrong_chs.push(q.chapter);
                        streak = 0;
                    }
                    if q.generated { n_generated += 1; }

                    let score_after = self.profile.mastery.get(&q.chapter).map(|r| r.score).unwrap_or(0.5);
                    mastery_deltas.push((score_after - score_before).abs());

                    if self.adaptive_stop
                        && mastery_deltas.len() >= AUTO_STOP_WINDOW
                        && mastery_deltas[mastery_deltas.len()-AUTO_STOP_WINDOW..].iter().sum::<f64>() < AUTO_STOP_THRESHOLD
                        && i < total
                    {
                        save_profile(&self.profile);
                        println!();
                        ui::print_wrap(&format!(
                            "Your mastery has plateaued over the last {} questions. \
                             You may have reached your limit for this session.",
                            AUTO_STOP_WINDOW
                        ));
                        if !ui::confirm("Continue?", true) { break; }
                        mastery_deltas.clear();
                    }
                    save_profile(&self.profile);
                }
            }
        }

        let duration = start.elapsed().as_secs_f64();
        let total_answered: u32 = per_diff.values().map(|v| v[1]).sum();

        let mut seen_wrong: Vec<u32> = Vec::new();
        for ch in &wrong_chs {
            if !seen_wrong.contains(ch) { seen_wrong.push(*ch); }
        }
        let meta = chapter_meta();
        let wrong_names: Vec<String> = seen_wrong.iter()
            .map(|ch| meta.get(ch).map(|m| m.name.to_string()).unwrap_or_else(|| format!("Ch.{}", ch)))
            .collect();

        ui::show_session_summary(
            score, total_answered as usize, &wrong_names, duration,
            streak_max, &per_diff, n_generated, &per_chapter,
        );

        let mut wrong_set = wrong_chs.clone();
        wrong_set.sort();
        wrong_set.dedup();

        let result = SessionResult {
            session_id:     Uuid::new_v4().to_string(),
            timestamp:      now_utc(),
            scope_label:    self.scope.label(),
            n_questions:    total_answered,
            n_correct:      score as u32,
            duration_secs:  duration,
            wrong_chapters: wrong_set,
            per_difficulty: per_diff,
            streak_max,
            n_generated,
        };

        self.profile.session_history.push(result.to_json_value());
        save_profile(&self.profile);

        self.start_background_prefill();

        (result, self.profile)
    }

    fn assemble_questions(&self, bank: &[Question]) -> Vec<Question> {
        let n_generated = if self.generator.as_ref().map(|g| g.available()).unwrap_or(false) {
            ((self.n as f64 * self.generated_ratio) as usize).max(1)
        } else { 0 };
        let n_static = self.n - n_generated;

        // Select static questions
        let static_refs = select_questions(bank, &self.profile, n_static, &self.scope);
        let mut selected_ids: std::collections::HashSet<String> =
            static_refs.iter().map(|q| q.question_id.clone()).collect();
        let mut static_qs: Vec<Question> = static_refs.into_iter().cloned().collect();

        let mut generated: Vec<Question> = Vec::new();
        if n_generated > 0 {
            if let Some(gen) = &self.generator {
                let (chapter, difficulty) = self.target_chapter_and_difficulty();
                let weaks = weak_topics(&self.profile, 5);
                let mastery_pct = (self.profile.mastery.get(&chapter).map(|r| r.score).unwrap_or(0.5) * 100.0) as u32;

                for _ in 0..n_generated {
                    let mut used = false;
                    if let Some(q) = gen.get_question(chapter, &difficulty, mastery_pct, &weaks) {
                        let h = question_text_hash(&q.text);
                        if self.profile.generated_strikes.get(&h).copied().unwrap_or(0) < 2 {
                            generated.push(q);
                            used = true;
                        }
                    }
                    if !used {
                        // fallback: grab one more static question
                        let remaining: Vec<Question> = bank.iter()
                            .filter(|q| !selected_ids.contains(&q.question_id))
                            .cloned()
                            .collect();
                        let extra = select_questions(&remaining, &self.profile, 1, &self.scope);
                        if let Some(q) = extra.into_iter().next() {
                            selected_ids.insert(q.question_id.clone());
                            static_qs.push(q.clone());
                        }
                    }
                }
            }
        }

        let combined = interleave(static_qs, generated);
        combined.into_iter().take(self.n).collect()
    }

    fn target_chapter_and_difficulty(&self) -> (u32, String) {
        let ch = match &self.scope.kind {
            ScopeKind::Chapter(ch) => *ch,
            ScopeKind::Phase(ph) => {
                let meta = chapter_meta();
                let chs: Vec<u32> = meta.keys().copied().filter(|&k| meta[&k].phase == *ph).collect();
                chs.into_iter().min_by(|&a, &b| {
                    let sa = self.profile.mastery.get(&a).map(|r| r.score).unwrap_or(0.5);
                    let sb = self.profile.mastery.get(&b).map(|r| r.score).unwrap_or(0.5);
                    sa.partial_cmp(&sb).unwrap()
                }).unwrap_or(0)
            }
            _ => {
                let started: Vec<u32> = self.profile.mastery.iter()
                    .filter(|(_, r)| r.total_seen > 0)
                    .map(|(&ch, _)| ch)
                    .collect();
                started.into_iter().min_by(|&a, &b| {
                    self.profile.mastery[&a].score.partial_cmp(&self.profile.mastery[&b].score).unwrap()
                }).unwrap_or(0)
            }
        };
        let diff = crate::adaptive::preferred_difficulty(
            self.profile.mastery.get(&ch).map(|r| r.score).unwrap_or(0.5)
        );
        (ch, diff.to_string())
    }

    fn start_background_prefill(&self) {
        let Some(gen) = &self.generator else { return; };
        if !gen.available() { return; }

        let started: Vec<u32> = self.profile.mastery.iter()
            .filter(|(_, r)| r.total_seen > 0)
            .map(|(&ch, _)| ch)
            .collect();
        if started.is_empty() { return; }

        let mut weakest = started.clone();
        weakest.sort_by(|&a, &b| {
            self.profile.mastery[&a].score.partial_cmp(&self.profile.mastery[&b].score).unwrap()
        });
        weakest.truncate(3);
        let weaks = weak_topics(&self.profile, 5);
        let snapshot: HashMap<u32, f64> = weakest.iter()
            .map(|&ch| (ch, self.profile.mastery[&ch].score))
            .collect();

        std::thread::spawn(move || {
            let gen = ClaudeGenerator::new();
            for ch in weakest {
                let pct = (snapshot.get(&ch).copied().unwrap_or(0.5) * 100.0) as u32;
                gen.prefill_for_chapter(ch, pct, &weaks);
            }
        });
    }

    fn empty_result(&self) -> SessionResult {
        SessionResult {
            session_id:    Uuid::new_v4().to_string(),
            timestamp:     String::new(),
            scope_label:   self.scope.label(),
            n_questions:   0,
            n_correct:     0,
            duration_secs: 0.0,
            wrong_chapters: Vec::new(),
            per_difficulty: HashMap::new(),
            streak_max:    0,
            n_generated:   0,
        }
    }
}
