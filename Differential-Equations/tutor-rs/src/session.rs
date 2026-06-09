use anyhow::Result;
use rand::SeedableRng;
use std::time::Instant;

use crate::adaptive::{
    select_difficulty, select_topic, unlocked_topics, update_elo, UNLOCK_ELO,
};
use crate::db::Db;
use crate::problems::{dispatch, types::Topic};
use crate::ui;

pub struct SessionConfig {
    pub profile:      String,
    pub n_problems:   u32,
    pub force_topic:  Option<Topic>,
}

pub fn run(db: &Db, cfg: SessionConfig) -> Result<()> {
    let mut rng = rand::rngs::SmallRng::from_entropy();

    let mastery   = db.all_mastery(&cfg.profile)?;
    let unlocked  = unlocked_topics(&mastery);
    let session_id = db.start_session(&cfg.profile)?;

    ui::banner();
    ui::show_active_topics(&mastery, &unlocked);

    let start     = Instant::now();
    let mut correct_count = 0u32;
    let mut elo_changes: Vec<(String, f64)> = Vec::new();
    let mut last_topic_str: Option<String> = None;
    let mut streak = 0u32;

    for problem_num in 1..=cfg.n_problems {
        // ── Select topic and difficulty ─────────────────────────────────────
        let topic = match &cfg.force_topic {
            Some(t) => t.clone(),
            None    => select_topic(&mastery, &unlocked, last_topic_str.as_deref(), &mut rng),
        };

        // Re-fetch mastery for this topic (may have changed)
        let topic_mastery = db.all_mastery(&cfg.profile)?;
        let topic_row = topic_mastery.iter().find(|m| m.topic == topic.as_str());
        let elo_before = topic_row.map(|r| r.elo).unwrap_or(400.0);
        let streak_val = topic_row.map(|r| r.streak as u32).unwrap_or(0);

        let difficulty = select_difficulty(elo_before, &mut rng);

        // ── Select subtopic ─────────────────────────────────────────────────
        let overdue = db.get_overdue_subtopics(&cfg.profile, topic.as_str())?;
        let _subtopic_hint = overdue.first().cloned();

        // ── Generate problem ────────────────────────────────────────────────
        let problem = dispatch(&topic, difficulty, &mut rng);
        let subtopic = problem.subtopic.clone();

        ui::session_header(
            problem_num, cfg.n_problems,
            &problem.topic, &subtopic,
            difficulty, streak_val,
        );
        ui::show_problem(
            &problem.question,
            problem.problem_type.as_str(),
            problem.choices.as_ref(),
        );

        // ── Answer loop ─────────────────────────────────────────────────────
        let mut hint_used = false;
        let result = loop {
            let raw = ui::get_answer();
            match raw.to_lowercase().as_str() {
                "quit" | "q" | "exit" => {
                    // End session early
                    db.end_session(session_id, problem_num - 1, correct_count)?;
                    ui::session_summary(
                        correct_count, problem_num - 1,
                        start.elapsed().as_secs_f64(),
                        &elo_changes,
                    );
                    return Ok(());
                }
                "hint" | "h" => {
                    hint_used = true;
                    ui::show_hint(&problem.hint);
                    continue;
                }
                "skip" | "s" => {
                    ui::show_skip();
                    let type_offset = problem.problem_type.elo_offset();
                    let new_elo = update_elo(elo_before, difficulty, 0.0, type_offset);
                    record_elo_change(&mut elo_changes, topic.as_str(), elo_before, new_elo);
                    db.update_mastery(
                        &cfg.profile, topic.as_str(), &subtopic,
                        new_elo, false, 0.0, difficulty,
                        problem.problem_type.as_str(),
                        false, true, elo_before, session_id,
                    )?;
                    ui::show_explanation(&problem.explanation);
                    last_topic_str = Some(topic.as_str().to_string());
                    streak = 0;
                    break LoopResult::Skip;
                }
                _ => {}
            }

            let (ok, parse_err) = problem.check_answer(&raw);
            if let Some(msg) = parse_err {
                ui::show_parse_error(&msg);
                continue;
            }

            // Compute ELO delta
            let actual = if ok {
                if hint_used { 0.5 } else { 1.0 }
            } else {
                0.0
            };
            let type_offset = problem.problem_type.elo_offset();
            let new_elo = update_elo(elo_before, difficulty, actual, type_offset);
            let delta = new_elo - elo_before;

            if ok {
                streak += 1;
                correct_count += 1;
                ui::show_correct(delta, streak, hint_used);
            } else {
                streak = 0;
                ui::show_incorrect(&problem.answer_display(), delta);
            }
            ui::show_explanation(&problem.explanation);

            // Check for unlock
            let prev_mastery = db.all_mastery(&cfg.profile)?;
            let prev_unlocked = unlocked_topics(&prev_mastery);

            db.update_mastery(
                &cfg.profile, topic.as_str(), &subtopic,
                new_elo, ok, actual, difficulty,
                problem.problem_type.as_str(),
                hint_used, false, elo_before, session_id,
            )?;

            record_elo_change(&mut elo_changes, topic.as_str(), elo_before, new_elo);

            // Unlock notifications
            if new_elo >= UNLOCK_ELO {
                let new_mastery = db.all_mastery(&cfg.profile)?;
                let new_unlocked = unlocked_topics(&new_mastery);
                for t in &new_unlocked {
                    if !prev_unlocked.contains(t) {
                        ui::show_unlock(t);
                    }
                }
            }

            last_topic_str = Some(topic.as_str().to_string());
            break LoopResult::Answered;
        };

        let _ = result;
    }

    db.end_session(session_id, cfg.n_problems, correct_count)?;
    ui::session_summary(
        correct_count, cfg.n_problems,
        start.elapsed().as_secs_f64(),
        &elo_changes,
    );

    Ok(())
}

enum LoopResult {
    Answered,
    Skip,
}

fn record_elo_change(changes: &mut Vec<(String, f64)>, topic: &str, before: f64, after: f64) {
    let delta = after - before;
    if let Some(entry) = changes.iter_mut().find(|(t, _)| t == topic) {
        entry.1 += delta;
    } else {
        changes.push((topic.to_string(), delta));
    }
}
