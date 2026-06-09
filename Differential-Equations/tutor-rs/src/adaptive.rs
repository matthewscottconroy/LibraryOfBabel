use rand::Rng;
use crate::db::MasteryRow;
use crate::problems::types::Topic;

// ── ELO constants ─────────────────────────────────────────────────────────────

pub const K: f64 = 24.0;
pub const D1_MID: f64 = 480.0;
pub const ELO_PER_DIFF: f64 = 120.0;
pub const UNLOCK_ELO: f64 = 620.0;

// ── ELO math ──────────────────────────────────────────────────────────────────

pub fn problem_elo(difficulty: u8, type_offset: f64) -> f64 {
    D1_MID + (difficulty as f64 - 1.0) * ELO_PER_DIFF + type_offset
}

pub fn expected_score(player_elo: f64, item_elo: f64) -> f64 {
    1.0 / (1.0 + 10f64.powf((item_elo - player_elo) / 400.0))
}

pub fn update_elo(player_elo: f64, difficulty: u8, actual: f64, type_offset: f64) -> f64 {
    let item_elo = problem_elo(difficulty, type_offset);
    let exp = expected_score(player_elo, item_elo);
    (player_elo + K * (actual - exp)).clamp(100.0, 1200.0)
}

// ── Difficulty / mastery helpers ──────────────────────────────────────────────

pub fn elo_to_difficulty(elo: f64) -> u8 {
    let d = ((elo - D1_MID) / ELO_PER_DIFF) as i32 + 1;
    d.clamp(1, 5) as u8
}

pub fn elo_to_pct(elo: f64) -> f64 {
    ((elo - 100.0) / (1200.0 - 100.0) * 100.0).clamp(0.0, 100.0)
}

/// 70% at natural difficulty, 20% one below, 10% one above.
pub fn select_difficulty(elo: f64, rng: &mut impl Rng) -> u8 {
    let base = elo_to_difficulty(elo);
    let roll: f64 = rng.gen();
    if roll < 0.70 {
        base
    } else if roll < 0.90 {
        base.saturating_sub(1).max(1)
    } else {
        (base + 1).min(5)
    }
}

// ── Topic availability ────────────────────────────────────────────────────────

pub fn unlocked_topics(mastery: &[MasteryRow]) -> Vec<Topic> {
    let elo_of = |t: &Topic| -> f64 {
        mastery.iter()
            .find(|m| m.topic == t.as_str())
            .map(|m| m.elo)
            .unwrap_or(400.0)
    };

    Topic::all().iter()
        .filter(|t| {
            t.prerequisites().iter().all(|prereq| elo_of(prereq) >= UNLOCK_ELO)
        })
        .cloned()
        .collect()
}

// ── Topic selection ───────────────────────────────────────────────────────────

pub fn select_topic(
    mastery: &[MasteryRow],
    unlocked: &[Topic],
    last_topic: Option<&str>,
    rng: &mut impl Rng,
) -> Topic {
    if unlocked.len() == 1 {
        return unlocked[0].clone();
    }

    let candidates: Vec<&Topic> = unlocked.iter()
        .filter(|t| Some(t.as_str()) != last_topic)
        .collect();
    let candidates = if candidates.is_empty() { unlocked.iter().collect::<Vec<_>>() } else { candidates };

    let now = chrono::Utc::now();
    let weights: Vec<f64> = candidates.iter().map(|t| {
        let row = mastery.iter().find(|m| m.topic == t.as_str());
        let elo = row.map(|r| r.elo).unwrap_or(400.0);
        let pct = elo_to_pct(elo) / 100.0;

        // Recency boost: topics not seen recently get higher weight
        let recency = if let Some(Some(last)) = row.map(|r| r.last_seen) {
            let hours = (now - last).num_seconds() as f64 / 3600.0;
            1.0 + (hours / 24.0).min(2.0)
        } else {
            3.0 // never seen: high priority
        };

        recency * (1.0 - 0.7 * pct + 0.05)
    }).collect();

    let total: f64 = weights.iter().sum();
    let mut r: f64 = rng.gen::<f64>() * total;
    for (i, w) in weights.iter().enumerate() {
        r -= w;
        if r <= 0.0 {
            return candidates[i].clone();
        }
    }
    (*candidates.last().unwrap()).clone()
}
