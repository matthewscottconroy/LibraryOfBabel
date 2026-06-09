"""Central configuration for the CSSB Adaptive Quiz."""
from __future__ import annotations

import os
from pathlib import Path

PACKAGE_DIR   = Path(__file__).parent
PROJECT_ROOT  = PACKAGE_DIR.parent
CHAPTERS_DIR  = PROJECT_ROOT / "curriculum"
DATA_DIR      = PACKAGE_DIR / "data"
PROGRESS_DIR  = DATA_DIR / "progress"
CACHE_DIR     = DATA_DIR / "cache"

for _d in (DATA_DIR, PROGRESS_DIR, CACHE_DIR):
    _d.mkdir(parents=True, exist_ok=True)

ANTHROPIC_API_KEY: str = os.environ.get("ANTHROPIC_API_KEY", "")
CLAUDE_MODEL = "claude-sonnet-4-6"
CHAPTER_EXCERPT_CHARS = 4_000
GENERATE_BATCH_SIZE = 5
CACHE_CAP_PER_DIFFICULTY = 20

LEARNING_RATE_CORRECT = 0.15
LEARNING_RATE_WRONG   = 0.10
INITIAL_MASTERY       = 0.30
RECENCY_PENALTY       = 0.05
RECENCY_WINDOW        = 60
DIFF_BEGINNER_MAX      = 0.35
DIFF_INTERMEDIATE_MAX  = 0.68
BLANK_WEIGHT_BONUS     = 1.25   # upweight fill-in-the-blank vs mc/tf of equal priority
CACHE_MAX_AGE_DAYS     = 90     # discard cached generated questions older than this

# Adaptive session auto-stop (plateau detection)
AUTO_STOP_WINDOW    = 5
AUTO_STOP_THRESHOLD = 0.015

CHAPTER_META: dict[int, dict] = {
    # Tier 0 — Bedrock
    0:  dict(phase=0, name="Mathematics",                  file="tier-0-bedrock/0.1-mathematics.md"),
    1:  dict(phase=0, name="Chemistry",                    file="tier-0-bedrock/0.2-chemistry.md"),
    2:  dict(phase=0, name="Biology",                      file="tier-0-bedrock/0.3-biology.md"),
    3:  dict(phase=0, name="Computer Science",             file="tier-0-bedrock/0.4-computer-science.md"),
    # Tier 1 — Bioinformatics
    4:  dict(phase=1, name="Sequence Analysis",            file="tier-1-bioinformatics/1.1-sequence-analysis.md"),
    5:  dict(phase=1, name="Genomics",                     file="tier-1-bioinformatics/1.2-genomics.md"),
    6:  dict(phase=1, name="Transcriptomics",              file="tier-1-bioinformatics/1.3-transcriptomics.md"),
    7:  dict(phase=1, name="Proteomics and Metabolomics",  file="tier-1-bioinformatics/1.4-proteomics-metabolomics.md"),
    8:  dict(phase=1, name="Structural Bioinformatics",    file="tier-1-bioinformatics/1.5-structural-bioinformatics.md"),
    9:  dict(phase=1, name="Phylogenetics",                file="tier-1-bioinformatics/1.6-phylogenetics.md"),
    # Tier 2 — Systems Biology
    10: dict(phase=2, name="Mathematical Modeling",        file="tier-2-systems-biology/2.1-mathematical-modeling.md"),
    11: dict(phase=2, name="Metabolic Modeling",           file="tier-2-systems-biology/2.2-metabolic-modeling.md"),
    12: dict(phase=2, name="Gene Regulatory Networks",     file="tier-2-systems-biology/2.3-gene-regulatory-networks.md"),
    13: dict(phase=2, name="Signaling Networks",           file="tier-2-systems-biology/2.4-signaling-networks.md"),
    14: dict(phase=2, name="Multiscale and Whole-Cell",    file="tier-2-systems-biology/2.5-multiscale-wholecell.md"),
    # Tier 3 — Synthetic Biology
    15: dict(phase=3, name="Genetic Parts and Devices",    file="tier-3-synthetic-biology/3.1-genetic-parts-devices.md"),
    16: dict(phase=3, name="Genetic Circuit Design",       file="tier-3-synthetic-biology/3.2-genetic-circuit-design.md"),
    17: dict(phase=3, name="Genome Editing",               file="tier-3-synthetic-biology/3.3-genome-editing.md"),
    18: dict(phase=3, name="Metabolic Engineering",        file="tier-3-synthetic-biology/3.4-metabolic-engineering.md"),
    19: dict(phase=3, name="Directed Evolution",           file="tier-3-synthetic-biology/3.5-directed-evolution.md"),
    20: dict(phase=3, name="Cell-Free Systems",            file="tier-3-synthetic-biology/3.6-cell-free-systems.md"),
    21: dict(phase=3, name="Biosafety and Ethics",         file="tier-3-synthetic-biology/3.7-biosafety-ethics.md"),
    # Tier 4 — Computational Tools
    22: dict(phase=4, name="Scientific Computing",         file="tier-4-computational-tools/4.1-scientific-computing.md"),
    23: dict(phase=4, name="Machine Learning in Biology",  file="tier-4-computational-tools/4.2-machine-learning-biology.md"),
    24: dict(phase=4, name="Molecular Dynamics",           file="tier-4-computational-tools/4.3-molecular-dynamics.md"),
    25: dict(phase=4, name="Network Analysis",             file="tier-4-computational-tools/4.4-network-analysis.md"),
    26: dict(phase=4, name="Software Engineering for Research", file="tier-4-computational-tools/4.5-software-engineering-research.md"),
    # Tier 5 — Literature and Research
    27: dict(phase=5, name="Foundational Papers",          file="tier-5-literature-research/5.1-foundational-papers.md"),
    28: dict(phase=5, name="Research Craft",               file="tier-5-literature-research/5.2-research-craft.md"),
}

PHASE_NAMES: dict[int, str] = {
    0: "Tier 0 — Bedrock (Math, Chemistry, Biology, CS)",
    1: "Tier 1 — Bioinformatics",
    2: "Tier 2 — Systems Biology",
    3: "Tier 3 — Synthetic Biology",
    4: "Tier 4 — Computational Tools",
    5: "Tier 5 — Literature and Research",
}

DIFFICULTY_LEVELS = ("beginner", "intermediate", "advanced")

# ── App → chapter mapping ────────────────────────────────────────────────────
# Maps interactive app number (01–22) to the primary curriculum chapter it teaches.
APP_CHAPTER_MAP: dict[int, int] = {
     1: 10,   # ODE phase plane           → Mathematical Modeling
     2: 10,   # Bifurcation diagram        → Mathematical Modeling
     3:  1,   # Enzyme kinetics            → Chemistry
     4:  4,   # Pairwise alignment         → Sequence Analysis
     5:  4,   # k-mer / de Bruijn          → Sequence Analysis
     6:  4,   # Sequence logo / PWM        → Sequence Analysis
     7:  9,   # Phylogenetics              → Phylogenetics
     8: 16,   # Repressilator              → Genetic Circuit Design
     9: 12,   # Toggle switch              → Gene Regulatory Networks
    10: 10,   # Gillespie SSA              → Mathematical Modeling
    11: 11,   # FBA metabolic              → Metabolic Modeling
    12: 10,   # Turing patterns            → Mathematical Modeling
    13: 12,   # Network motifs             → Gene Regulatory Networks
    14: 16,   # Genetic circuit simulator  → Genetic Circuit Design
    15: 17,   # CRISPR guide design        → Genome Editing
    16: 19,   # Fitness landscape          → Directed Evolution
    17: 22,   # ODE solver comparison      → Scientific Computing
    18: 22,   # MCMC parameter estimation  → Scientific Computing
    19: 23,   # Sequence ML                → Machine Learning in Biology
    20: 25,   # Network analysis           → Network Analysis
    21: 24,   # Molecular dynamics toy     → Molecular Dynamics
    22: 15,   # RBS translation            → Genetic Parts and Devices
    # New apps
    23:  6,   # RNA-seq DE explorer        → Transcriptomics
    24:  6,   # PCA expression clustering  → Transcriptomics
    25:  5,   # Variant annotator          → Genomics
    26:  7,   # Mass spectrum annotator    → Proteomics and Metabolomics
    27:  8,   # Ramachandran explorer      → Structural Bioinformatics
    28: 13,   # MAPK cascade              → Signaling Networks
    29: 18,   # Gene knockout FBA         → Metabolic Engineering
    30:  0,   # Eigenvalue/stability      → Mathematics
}

# ── Cathedral → prerequisite chapters ───────────────────────────────────────
CATHEDRAL_PREREQS: dict[str, dict] = {
    "I": {
        "title": "Predictive Metabolic Engineering Campaign",
        "chapters": [1, 2, 5, 11, 18],
        "description": "GEM construction, FBA optimisation, strain design, metabolomics validation",
    },
    "II": {
        "title": "Genetic Circuit with Predictive Design",
        "chapters": [1, 10, 15, 16],
        "description": "ODE modelling, parts characterisation, circuit design and debugging",
    },
    "III": {
        "title": "Multi-Omics Integration Analysis",
        "chapters": [6, 7, 11, 25],
        "description": "RNA-seq, proteomics, metabolomics, network-level integration",
    },
    "IV": {
        "title": "Molecular Dynamics Study",
        "chapters": [1, 8, 24],
        "description": "Force fields, structural bioinformatics, FEP/enhanced sampling",
    },
    "V": {
        "title": "ML-Guided Directed Evolution",
        "chapters": [19, 23],
        "description": "Protein fitness landscape, Bayesian optimisation, sequence-function ML",
    },
    "VI": {
        "title": "Spatial Stochastic Simulation",
        "chapters": [10, 14],
        "description": "Reaction-diffusion PDEs, spatial SSA, emergent pattern formation",
    },
    "VII": {
        "title": "Computational Tool Publication",
        "chapters": [22, 26],
        "description": "Algorithm design, software engineering, benchmarking, open-source release",
    },
}
