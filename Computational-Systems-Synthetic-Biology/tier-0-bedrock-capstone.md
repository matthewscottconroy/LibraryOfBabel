# Tier 0 Capstone: Bedrock Integration Project

## "From Sequence to Mechanism: A Complete Quantitative Analysis of a Bacterial Regulatory System"

---

## Overview

The Tier 0 Bedrock capstone project demands that you deploy every foundational skill developed across mathematics, chemistry, biology, and computational science in a single integrated analysis. You will choose a well-characterised bacterial regulatory system — the arabinose (ara) operon or the lac operon are the canonical choices — and produce a rigorous multi-level analysis that begins with the gene sequence and ends with a quantitative, validated mechanistic model.

This project is designed to reveal the connections among disciplines that Tier 0 taught separately. You will discover that understanding a biological system quantitatively requires not just the individual skills, but the ability to move fluidly between levels of description: from biochemistry to mathematics, from experimental data to computational model, from genome to phenotype.

---

## Biological Motivation

Bacterial regulatory systems are the simplest settings in which gene regulation achieves sophisticated computational tasks. The lac operon — studied by Jacob and Monod, awarded the Nobel Prize in 1965 — encodes three genes for lactose metabolism, regulated by a repressor protein and an activator (CRP) that together implement an approximate AND gate: the genes are expressed only when lactose is present AND glucose is absent.

This apparently simple logic conceals a rich quantitative landscape: cooperative binding with Hill coefficients near 2, a bistable response at intermediate inducer concentrations, growth-rate coupling through CRP, and stochastic switching that produces bet-hedging in clonal populations. Understanding it quantitatively — not just qualitatively — requires the full toolkit of Tier 0.

---

## Project Components

### Component 1: Biological Background and System Characterisation (Week 1–2)

**Tasks:**
- Write a two-page mechanistic description of your chosen system, covering: the genes involved, their protein products, the regulatory proteins, the inducer molecules, and the logic of the regulation.
- Draw a circuit diagram showing the regulatory interactions.
- Identify the key biochemical parameters from the literature: $K_d$ values for repressor-operator binding, $K_d$ for inducer binding, Hill coefficients, promoter strengths (in Miller units or equivalent).
- Note the experimental conditions under which the system has been characterised.

**Deliverable:** A narrative biological description (2 pages) with a circuit diagram and a table of key parameters with citations.

### Component 2: Mathematical Modelling — Deterministic ODE Model (Week 3–4)

**Tasks:**
- Write a system of ODEs for the concentrations of mRNA and protein products, incorporating:
  - Transcription as a Hill function of the relevant transcription factors
  - First-order mRNA degradation
  - Translation proportional to mRNA concentration
  - First-order protein degradation (dilution by growth included)
- Implement the model in Python using `scipy.integrate.odeint` or `solve_ivp`.
- Find the steady-state concentrations analytically for limiting cases (saturating inducer, no inducer).
- Plot the steady-state protein level as a function of inducer concentration for various repressor concentrations.
- Perform a parameter sensitivity analysis: which parameters most strongly affect the switch point?

**Required equations (for lac operon):**
$$\frac{d[\text{mRNA}]}{dt} = k_\text{tx} \cdot \frac{[I]^n}{K_I^n + [I]^n} \cdot \frac{K_R^m}{K_R^m + [R]^m} - \gamma_m [\text{mRNA}]$$
$$\frac{d[\text{Protein}]}{dt} = k_\text{tl} [\text{mRNA}] - \gamma_p [\text{Protein}]$$

where $[I]$ is inducer concentration, $[R]$ is repressor concentration, $n$ and $m$ are Hill coefficients for inducer and repressor binding, and $\gamma_m$, $\gamma_p$ are degradation rates.

**Deliverable:** Annotated Python code, a phase portrait (for reduced two-variable system), dose-response curves, and a one-page analysis of sensitivity results.

### Component 3: Stochastic Simulation (Week 5–6)

**Tasks:**
- Implement the Gillespie stochastic simulation algorithm (SSA) for your system.
- Identify all elementary reactions (mRNA synthesis, mRNA degradation, translation, protein degradation, repressor binding/unbinding).
- Write the propensity functions for each reaction.
- Simulate 100+ trajectories for three inducer concentrations: below, near, and above the switch point.
- Plot: (a) single trajectories showing mRNA and protein fluctuations; (b) histogram of steady-state protein levels across 1000 simulated cells; (c) coefficient of variation (CV = σ/μ) as a function of protein level.
- Compare mean protein levels from SSA to the ODE steady state.

**Key question:** At what copy numbers does stochastic switching (spontaneous transitions between low and high expression states) occur? This determines whether the system can function as a reliable population-level switch or produces heterogeneous "noisy" expression.

**Deliverable:** Python code, trajectory plots, histograms, and a two-page analysis comparing deterministic and stochastic descriptions.

### Component 4: Data Fitting and Model Validation (Week 7–8)

**Tasks:**
- Obtain published dose-response data for your system (e.g., from the Uri Alon lab's supplementary data, or from Benchling/literature).
- Fit your ODE model to the experimental dose-response using `scipy.optimize.curve_fit` or a Bayesian approach (e.g., with PyMC or emcee).
- Report the best-fit parameters with 95% confidence intervals or credible intervals.
- Assess the goodness of fit quantitatively (chi-squared or RMSE) and qualitatively (residual plot).
- Identify at least one aspect of the data that your model does not capture, and propose a model extension.

**Deliverable:** Fitted dose-response curves plotted against data, parameter estimates with uncertainty, residual analysis, and a brief discussion of model limitations.

---

## Expected Deliverables

| Component | Format | Length |
|-----------|--------|--------|
| Biological background | Written narrative + figure | 2 pages |
| ODE model | Code + notebook | — |
| Deterministic analysis | Written analysis + figures | 3 pages |
| Stochastic simulation | Code + notebook | — |
| Stochastic analysis | Written analysis + figures | 2 pages |
| Data fitting | Code + written analysis | 2 pages |
| Final synthesis | Integrated report | 3 pages |

---

## Assessment Rubric

| Criterion | Weight | Excellent (A) | Proficient (B) | Developing (C) |
|-----------|--------|---------------|----------------|----------------|
| Biological accuracy | 20% | Circuit diagram correct, all regulatory logic explained, parameters from primary literature | Minor errors in diagram, some parameters from review articles | Significant biological errors |
| Mathematical rigour | 25% | ODE system correctly derived from biochemistry, steady-state analysis correct, sensitivity analysis complete | ODEs mostly correct, some errors in analysis | Significant mathematical errors |
| Stochastic simulation | 20% | SSA correctly implemented, propensities correct, results interpreted thoughtfully | SSA runs but propensities contain errors; interpretation shallow | SSA not implemented or non-functional |
| Data fitting | 20% | Fitting successful, confidence intervals reported, residuals analysed | Fitting attempted, goodness-of-fit reported | Fitting not performed |
| Scientific writing | 15% | Clear, precise, well-organised; figures properly captioned | Adequate but imprecise in places | Unclear or poorly organised |

---

## Extension Challenges

**Extension A: Growth-rate coupling.** Incorporate the dependence of dilution rate (and hence effective protein degradation rate) on growth rate, and the dependence of growth rate on gene expression burden. This creates a feedback loop between the circuit and cellular physiology.

**Extension B: Cell-to-cell variability.** If you have access to single-cell data (available for lac from Choi et al. 2008, *Science*), fit a model that includes extrinsic noise (cell-to-cell variation in repressor level) in addition to intrinsic noise (Gillespie noise in the circuit itself).

**Extension C: Evolutionary analysis.** Using the fitted model, map the parameter space corresponding to functional switch behaviour (appropriate switch point, acceptable Hill coefficient, low noise in the ON and OFF states). What fraction of "random" parameter sets produces functional switches? This provides a sense of the evolvability of the regulatory architecture.

---

*Complete this capstone before proceeding to Tier 1. If you cannot complete Components 2 and 3 with confidence, revisit the mathematics (ODEs, numerical methods) and programming (Python, NumPy, SciPy) sections of Tier 0 before advancing.*
