use std::collections::HashMap;
use std::path::PathBuf;

// ── Tuneable constants ────────────────────────────────────────────────────────

pub const CLAUDE_MODEL: &str = "claude-sonnet-4-6";
pub const CHAPTER_EXCERPT_CHARS: usize = 4_000;
pub const GENERATE_BATCH_SIZE: usize = 5;
pub const CACHE_CAP_PER_DIFFICULTY: usize = 20;

pub const LEARNING_RATE_CORRECT: f64 = 0.15;
pub const LEARNING_RATE_WRONG: f64 = 0.10;
pub const INITIAL_MASTERY: f64 = 0.30;
pub const RECENCY_PENALTY: f64 = 0.05;
pub const RECENCY_WINDOW: usize = 60;
pub const DIFF_BEGINNER_MAX: f64 = 0.35;
pub const DIFF_INTERMEDIATE_MAX: f64 = 0.68;
pub const BLANK_WEIGHT_BONUS: f64 = 1.25;
pub const CACHE_MAX_AGE_DAYS: i64 = 90;

pub const AUTO_STOP_WINDOW: usize = 5;
pub const AUTO_STOP_THRESHOLD: f64 = 0.015;

pub const DIFFICULTY_LEVELS: &[&str] = &["beginner", "intermediate", "advanced"];

// ── Directory resolution ──────────────────────────────────────────────────────

pub fn data_dir() -> PathBuf {
    // ~/.local/share/cssb_quiz  (falls back to ~/cssb_quiz)
    let base = dirs::data_local_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("cssb_quiz")
}

pub fn progress_dir() -> PathBuf {
    data_dir().join("progress")
}

pub fn cache_dir() -> PathBuf {
    data_dir().join("cache")
}

/// Curriculum directory: sibling of the quiz_rs directory.
pub fn chapters_dir() -> PathBuf {
    // Try path relative to executable first, then CWD.
    if let Ok(exe) = std::env::current_exe() {
        // dev build: target/debug/quiz_rs → ../../curriculum
        let candidate = exe
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.join("curriculum"));
        if let Some(c) = candidate {
            if c.is_dir() {
                return c;
            }
        }
    }
    std::env::current_dir()
        .unwrap_or_default()
        .join("curriculum")
}

pub fn questions_path() -> PathBuf {
    // Look next to the binary first, then next to CWD
    if let Ok(exe) = std::env::current_exe() {
        let p = exe.parent().map(|d| d.join("questions.json"));
        if let Some(p) = p {
            if p.exists() {
                return p;
            }
        }
        // dev: target/debug → repo root
        let p = exe
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.join("quiz_rs/questions.json"));
        if let Some(p) = p {
            if p.exists() {
                return p;
            }
        }
    }
    PathBuf::from("questions.json")
}

// ── Chapter metadata ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ChapterMeta {
    pub phase: u32,
    pub name: &'static str,
    pub file: &'static str,
}

pub fn chapter_meta() -> HashMap<u32, ChapterMeta> {
    let mut m = HashMap::new();
    macro_rules! ch {
        ($id:expr, $phase:expr, $name:expr, $file:expr) => {
            m.insert($id, ChapterMeta { phase: $phase, name: $name, file: $file });
        };
    }
    // Tier 0 — Bedrock
    ch!(0,  0, "Mathematics",                  "tier-0-bedrock/0.1-mathematics.md");
    ch!(1,  0, "Chemistry",                    "tier-0-bedrock/0.2-chemistry.md");
    ch!(2,  0, "Biology",                      "tier-0-bedrock/0.3-biology.md");
    ch!(3,  0, "Computer Science",             "tier-0-bedrock/0.4-computer-science.md");
    // Tier 1 — Bioinformatics
    ch!(4,  1, "Sequence Analysis",            "tier-1-bioinformatics/1.1-sequence-analysis.md");
    ch!(5,  1, "Genomics",                     "tier-1-bioinformatics/1.2-genomics.md");
    ch!(6,  1, "Transcriptomics",              "tier-1-bioinformatics/1.3-transcriptomics.md");
    ch!(7,  1, "Proteomics and Metabolomics",  "tier-1-bioinformatics/1.4-proteomics-metabolomics.md");
    ch!(8,  1, "Structural Bioinformatics",    "tier-1-bioinformatics/1.5-structural-bioinformatics.md");
    ch!(9,  1, "Phylogenetics",                "tier-1-bioinformatics/1.6-phylogenetics.md");
    // Tier 2 — Systems Biology
    ch!(10, 2, "Mathematical Modeling",        "tier-2-systems-biology/2.1-mathematical-modeling.md");
    ch!(11, 2, "Metabolic Modeling",           "tier-2-systems-biology/2.2-metabolic-modeling.md");
    ch!(12, 2, "Gene Regulatory Networks",     "tier-2-systems-biology/2.3-gene-regulatory-networks.md");
    ch!(13, 2, "Signaling Networks",           "tier-2-systems-biology/2.4-signaling-networks.md");
    ch!(14, 2, "Multiscale and Whole-Cell",    "tier-2-systems-biology/2.5-multiscale-wholecell.md");
    // Tier 3 — Synthetic Biology
    ch!(15, 3, "Genetic Parts and Devices",    "tier-3-synthetic-biology/3.1-genetic-parts-devices.md");
    ch!(16, 3, "Genetic Circuit Design",       "tier-3-synthetic-biology/3.2-genetic-circuit-design.md");
    ch!(17, 3, "Genome Editing",               "tier-3-synthetic-biology/3.3-genome-editing.md");
    ch!(18, 3, "Metabolic Engineering",        "tier-3-synthetic-biology/3.4-metabolic-engineering.md");
    ch!(19, 3, "Directed Evolution",           "tier-3-synthetic-biology/3.5-directed-evolution.md");
    ch!(20, 3, "Cell-Free Systems",            "tier-3-synthetic-biology/3.6-cell-free-systems.md");
    ch!(21, 3, "Biosafety and Ethics",         "tier-3-synthetic-biology/3.7-biosafety-ethics.md");
    // Tier 4 — Computational Tools
    ch!(22, 4, "Scientific Computing",         "tier-4-computational-tools/4.1-scientific-computing.md");
    ch!(23, 4, "Machine Learning in Biology",  "tier-4-computational-tools/4.2-machine-learning-biology.md");
    ch!(24, 4, "Molecular Dynamics",           "tier-4-computational-tools/4.3-molecular-dynamics.md");
    ch!(25, 4, "Network Analysis",             "tier-4-computational-tools/4.4-network-analysis.md");
    ch!(26, 4, "Software Engineering for Research", "tier-4-computational-tools/4.5-software-engineering-research.md");
    // Tier 5 — Literature and Research
    ch!(27, 5, "Foundational Papers",          "tier-5-literature-research/5.1-foundational-papers.md");
    ch!(28, 5, "Research Craft",               "tier-5-literature-research/5.2-research-craft.md");
    m
}

pub fn phase_names() -> HashMap<u32, &'static str> {
    let mut m = HashMap::new();
    m.insert(0, "Tier 0 — Bedrock (Math, Chemistry, Biology, CS)");
    m.insert(1, "Tier 1 — Bioinformatics");
    m.insert(2, "Tier 2 — Systems Biology");
    m.insert(3, "Tier 3 — Synthetic Biology");
    m.insert(4, "Tier 4 — Computational Tools");
    m.insert(5, "Tier 5 — Literature and Research");
    m
}

pub fn app_chapter_map() -> HashMap<u32, u32> {
    let pairs = [
        (1, 10), (2, 10), (3, 1),  (4, 4),  (5, 4),  (6, 4),
        (7, 9),  (8, 16), (9, 12), (10,10), (11,11), (12,10),
        (13,12), (14,16), (15,17), (16,19), (17,22), (18,22),
        (19,23), (20,25), (21,24), (22,15), (23,6),  (24,6),
        (25,5),  (26,7),  (27,8),  (28,13), (29,18), (30,0),
    ];
    pairs.iter().map(|&(a, c)| (a, c)).collect()
}

// ── Cathedral metadata ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CathedralData {
    pub title:       &'static str,
    pub chapters:    &'static [u32],
    pub description: &'static str,
}

pub fn cathedral_prereqs() -> HashMap<&'static str, CathedralData> {
    let mut m = HashMap::new();
    macro_rules! cath {
        ($id:expr, $title:expr, $chapters:expr, $desc:expr) => {
            m.insert($id, CathedralData { title: $title, chapters: $chapters, description: $desc });
        };
    }
    cath!("I",   "Predictive Metabolic Engineering Campaign",
          &[1u32, 2, 5, 11, 18] as &[u32],
          "GEM construction, FBA optimisation, strain design, metabolomics validation");
    cath!("II",  "Genetic Circuit with Predictive Design",
          &[1u32, 10, 15, 16] as &[u32],
          "ODE modelling, parts characterisation, circuit design and debugging");
    cath!("III", "Multi-Omics Integration Analysis",
          &[6u32, 7, 11, 25] as &[u32],
          "RNA-seq, proteomics, metabolomics, network-level integration");
    cath!("IV",  "Molecular Dynamics Study",
          &[1u32, 8, 24] as &[u32],
          "Force fields, structural bioinformatics, FEP/enhanced sampling");
    cath!("V",   "ML-Guided Directed Evolution",
          &[19u32, 23] as &[u32],
          "Protein fitness landscape, Bayesian optimisation, sequence-function ML");
    cath!("VI",  "Spatial Stochastic Simulation",
          &[10u32, 14] as &[u32],
          "Reaction-diffusion PDEs, spatial SSA, emergent pattern formation");
    cath!("VII", "Computational Tool Publication",
          &[22u32, 26] as &[u32],
          "Algorithm design, software engineering, benchmarking, open-source release");
    m
}
