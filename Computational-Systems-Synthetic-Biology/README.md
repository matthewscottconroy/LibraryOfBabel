# Computational & Synthetic Biology Mastery Curriculum

A complete knowledge ladder from mathematical bedrock to research-grade contribution in bioinformatics, computational biology, systems biology, and synthetic biology.

## Philosophy

The **ladder** is sequential — each rung must hold before you climb. The **cathedrals** are the integrated structures built *with* that foundation: simulations, tools, and research contributions that could not exist without every layer beneath them.

---

## Structure

The curriculum is not a flat set of files — it is a deeply nested tree of
roughly 770 markdown files: **tier → unit → chapter → section**. Each tier also
keeps top-level summary files (`0.1-mathematics.md`, …) alongside the full
nested treatment.

```
Computational-Systems-Synthetic-Biology/
├── curriculum/                     ~770 files; the sequential knowledge ladder
│   ├── tier-0-bedrock/             Math, chemistry, biology, CS foundation
│   │   ├── 0.1-mathematics.md      … tier-level summary files
│   │   ├── intro.md
│   │   └── unit-1-quantitative-foundations/
│   │       └── chapter-0.1-mathematics/
│   │           └── …               section-level content files
│   ├── tier-1-bioinformatics/      Sequence analysis, genomics, structure
│   ├── tier-2-systems-biology/     ODE/stochastic modeling, metabolic networks, GRNs
│   ├── tier-3-synthetic-biology/   Circuit design, genome editing, metabolic engineering
│   ├── tier-4-computational-tools/ Scientific computing, ML, MD, software engineering
│   └── tier-5-literature-research/ Foundational papers and research craft
│
├── apps/                           ~30 interactive Python simulations, by tier
│   ├── README.md                   Application index and usage
│   ├── tier-0-bedrock/             e.g. 01_ode_phase_plane.py, 02_bifurcation_diagram.py
│   ├── tier-1-bioinformatics/      e.g. 04_pairwise_alignment.py, 07_phylogenetics.py
│   ├── tier-2-systems-biology/     e.g. 08_repressilator.py
│   ├── tier-3-synthetic-biology/
│   └── tier-4-computational-tools/
│
├── cathedrals/                     Seven integrated research-project specs
│   ├── overview.md
│   ├── I-metabolic-engineering-campaign.md
│   ├── II-genetic-circuit-predictive-design.md
│   ├── III-multi-omics-integration.md
│   ├── IV-molecular-dynamics-study.md
│   ├── V-ml-guided-directed-evolution.md
│   ├── VI-spatial-stochastic-simulation.md
│   └── VII-computational-tool-publication.md
│
├── tier-N-...-capstone.md          Per-tier capstone projects
├── tier-N-to-tier-M-bridge.md      Inter-tier bridge documents
├── glossary.md                     Consolidated glossary
├── learning-sequence.md            Recommended year-by-year progression
└── resources.md                    Key texts, tools, and databases by domain
```

---

## Tiers at a Glance

| Tier | Domain | Core Competency |
|------|--------|-----------------|
| 0 | Bedrock | Math, chemistry, biology, CS fundamentals |
| 1 | Bioinformatics | Sequence analysis, genomics, transcriptomics, structure |
| 2 | Systems Biology | ODE/stochastic modeling, metabolic networks, GRNs |
| 3 | Synthetic Biology | Circuit design, genome editing, metabolic engineering |
| 4 | Computational Tools | Simulation, ML, MD, software engineering |
| 5 | Research Craft | Literature, experimental design, writing |

---

## The Cathedrals

Seven integrated projects that demand every tier beneath them:

1. A predictive metabolic engineering campaign
2. A genetic circuit with predictive design
3. A multi-omics integration analysis
4. A molecular dynamics study of a protein of interest
5. A machine-learning-guided directed evolution campaign
6. A spatial stochastic simulation of a cellular process
7. A computational tool that others can use

---

## Recommended Learning Sequence

See [learning-sequence.md](learning-sequence.md) for the year-by-year progression.

## Key Resources

See [resources.md](resources.md) for primary texts, tools, databases, and communities by domain.

## Commands

Run from the repository root unless noted.

```bash
# Build the book (also --html, --markdown, --check)
python3 tools/build_book.py Computational-Systems-Synthetic-Biology --pdf

# Take the adaptive quiz
cd quiz && cargo run -p quiz-cli -- --subject ../Computational-Systems-Synthetic-Biology

# Validate before opening a PR
python3 tools/validate.py
```

The interactive simulations in `apps/` are standalone Python (NumPy, SciPy,
Matplotlib, NetworkX, scikit-learn); see [apps/README.md](apps/README.md) for
the full index and per-app usage.

See [PROCESS.md](../PROCESS.md) for the full pipeline and
[CONTRIBUTING.md](../CONTRIBUTING.md) to contribute.
