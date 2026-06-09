# Interactive Learning Applications

A comprehensive suite of 22 interactive Python applications spanning every tier of the curriculum. Each app is self-contained, runs from the command line, and renders a dark-themed multi-panel matplotlib figure.

## Quick Start

```bash
# Install dependencies (all standard scientific Python)
pip install numpy scipy matplotlib networkx scikit-learn

# Run any app
python apps/tier-0-bedrock/01_ode_phase_plane.py
python apps/tier-2-systems-biology/08_repressilator.py --n 3
```

---

## Application Index

### Tier 0 — Bedrock (Mathematics, Chemistry)

| # | File | Concepts | Key Parameter |
|---|------|----------|---------------|
| 01 | [01_ode_phase_plane.py](tier-0-bedrock/01_ode_phase_plane.py) | Phase plane, nullclines, fixed points, Jacobian stability, trajectories | Click to add trajectories; radio buttons for system |
| 02 | [02_bifurcation_diagram.py](tier-0-bedrock/02_bifurcation_diagram.py) | Saddle-node, pitchfork, bistable switch bifurcations, hysteresis, phase-line portrait | Slider sweeps parameter r |
| 03 | [03_enzyme_kinetics.py](tier-0-bedrock/03_enzyme_kinetics.py) | Michaelis-Menten, Hill equation, inhibition modes, full mechanism ODE, Lineweaver-Burk | Radio buttons for mode |

### Tier 1 — Bioinformatics

| # | File | Concepts | Key Parameter |
|---|------|----------|---------------|
| 04 | [04_pairwise_alignment.py](tier-1-bioinformatics/04_pairwise_alignment.py) | Needleman-Wunsch, Smith-Waterman, BLOSUM62, DP matrix visualization, alignment statistics | `--mode nw\|sw`, `--seq1`, `--seq2` |
| 05 | [05_kmer_debruijn.py](tier-1-bioinformatics/05_kmer_debruijn.py) | k-mer decomposition, frequency spectrum, de Bruijn graph, Eulerian path assembly | `--seq`, `--k` |
| 06 | [06_sequence_logo_pwm.py](tier-1-bioinformatics/06_sequence_logo_pwm.py) | Position frequency/weight matrix, information content, sequence logo, PSSM scanning | `--demo` (4 built-in motifs) |
| 07 | [07_phylogenetics.py](tier-1-bioinformatics/07_phylogenetics.py) | Jukes-Cantor distances, UPGMA, Neighbor-Joining, distance matrix heatmap | `--demo` (mammals, bacteria, simulated) |

### Tier 2 — Systems Biology

| # | File | Concepts | Key Parameter |
|---|------|----------|---------------|
| 08 | [08_repressilator.py](tier-2-systems-biology/08_repressilator.py) | Repressilator ODE + Gillespie SSA, Hopf bifurcation, Hill coefficient sweep | `--n`, `--alpha`, `--beta` |
| 09 | [09_toggle_switch.py](tier-2-systems-biology/09_toggle_switch.py) | Bistable toggle switch, nullclines, hysteresis, interactive sliders | Sliders for n₁, n₂, α₁, α₂ |
| 10 | [10_gillespie_ssa.py](tier-2-systems-biology/10_gillespie_ssa.py) | Gillespie SSA, constitutive/bursty/feedback gene expression, Fano factor, distributions | `--model simple\|bursting\|feedback` |
| 11 | [11_fba_metabolic.py](tier-2-systems-biology/11_fba_metabolic.py) | FBA from scratch (linprog), stoichiometric matrix, flux variability analysis, knockout scan | `--obj biomass\|ethanol` |
| 12 | [12_turing_patterns.py](tier-2-systems-biology/12_turing_patterns.py) | Reaction-diffusion PDEs, Turing instability, Gierer-Meinhardt, Gray-Scott, Schnakenberg | `--model`, `--d` (diffusion ratio) |
| 13 | [13_network_motifs.py](tier-2-systems-biology/13_network_motifs.py) | C1-FFL pulse generator, I1-FFL adaptation, negative/positive autoregulation | `--motif c1ffl\|i1ffl\|negfb\|posfb` |

### Tier 3 — Synthetic Biology

| # | File | Concepts | Key Parameter |
|---|------|----------|---------------|
| 14 | [14_genetic_circuit_simulator.py](tier-3-synthetic-biology/14_genetic_circuit_simulator.py) | NOT/AND/NAND gates, transfer functions, toggle switch, pulse generator, truth tables | `--circuit`, `--parts` |
| 15 | [15_crispr_guide_design.py](tier-3-synthetic-biology/15_crispr_guide_design.py) | PAM recognition, gRNA on-target scoring (Doench), off-target mismatch analysis, seed region | `--demo eGFP\|lacZ\|PCSK9` |
| 16 | [16_fitness_landscape.py](tier-3-synthetic-biology/16_fitness_landscape.py) | NK fitness landscape, epistasis, hill-climbing, ruggedness, PCA projection | `--model smooth\|rugged\|gb1_proxy` |
| 22 | [22_rbs_translation.py](tier-3-synthetic-biology/22_rbs_translation.py) | Shine-Dalgarno thermodynamics, SD-AUG spacing, RBS strength comparison, Salis model | No arguments needed |

### Tier 4 — Computational Tools

| # | File | Concepts | Key Parameter |
|---|------|----------|---------------|
| 17 | [17_ode_solver_comparison.py](tier-4-computational-tools/17_ode_solver_comparison.py) | Euler, RK4, RK45, BDF, LSODA; stiffness; error vs. step size; Robertson kinetics | `--system lotka\|stiff\|repressilator` |
| 18 | [18_mcmc_parameter_estimation.py](tier-4-computational-tools/18_mcmc_parameter_estimation.py) | Metropolis-Hastings MCMC, posterior distributions, trace plots, credible intervals | `--model mm\|hill\|osc` |
| 19 | [19_sequence_ml.py](tier-4-computational-tools/19_sequence_ml.py) | One-hot/k-mer encoding, random forest, feature importance, learning curves | `--encoding onehot\|kmer\|pssm` |
| 20 | [20_network_analysis.py](tier-4-computational-tools/20_network_analysis.py) | Scale-free networks, centrality (degree/betweenness/PageRank), Louvain communities | `--network yeast_ppi\|grn\|metabolic` |
| 21 | [21_molecular_dynamics_toy.py](tier-4-computational-tools/21_molecular_dynamics_toy.py) | LJ potential, velocity Verlet, energy conservation, thermostat, g(r), phases | `--phase gas\|liquid\|solid` |

---

## Design Principles

All applications share:
- **Self-contained** — no external data files required; synthetic or built-in data
- **Dark theme** — `#1a1a2e` / `#0f3460` backgrounds; high-contrast accent colors
- **Multi-panel** — each figure shows 4–8 related views of the same concept
- **CLI arguments** — default to a good pedagogical example; override for exploration
- **Terminal summary** — key numerical results printed after the figure
- **Inline math** — equations in docstrings and panel titles for reference

## Dependencies

```
numpy>=1.22
scipy>=1.9
matplotlib>=3.5
networkx>=2.8
scikit-learn>=1.1
```

All available via `pip install numpy scipy matplotlib networkx scikit-learn`.
