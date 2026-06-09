pub mod types;
mod analysis;
mod linalg;
mod multivariable;
mod vectors;
mod odes;
mod fourier;
mod pdes;
mod complex_an;

use rand::RngCore;
use types::{Topic, Problem, GeneratorEntry};

pub fn all_generators() -> Vec<GeneratorEntry> {
    let mut v = Vec::new();
    v.extend(analysis::generators());
    v.extend(linalg::generators());
    v.extend(multivariable::generators());
    v.extend(vectors::generators());
    v.extend(odes::generators());
    v.extend(fourier::generators());
    v.extend(pdes::generators());
    v.extend(complex_an::generators());
    v
}

/// Pick a random generator matching topic+difficulty, fall back to nearest difficulty.
pub fn dispatch(topic: &Topic, difficulty: u8, rng: &mut dyn RngCore) -> Problem {
    let all = all_generators();
    let matching: Vec<&GeneratorEntry> = all.iter()
        .filter(|g| &g.topic == topic && g.difficulty == difficulty)
        .collect();

    if !matching.is_empty() {
        let idx = (rng.next_u64() as usize) % matching.len();
        return (matching[idx].generate)(rng);
    }

    // Fall back: any generator for this topic
    let topic_gens: Vec<&GeneratorEntry> = all.iter()
        .filter(|g| &g.topic == topic)
        .collect();

    if !topic_gens.is_empty() {
        let idx = (rng.next_u64() as usize) % topic_gens.len();
        return (topic_gens[idx].generate)(rng);
    }

    // Last resort: something
    let idx = (rng.next_u64() as usize) % all.len();
    (all[idx].generate)(rng)
}
