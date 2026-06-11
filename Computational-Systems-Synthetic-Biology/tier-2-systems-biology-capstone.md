# Tier 2 Capstone: Systems Biology Integration Project

## "Design and Analysis of a Synthetic Oscillator with Noise Characterisation"

---

## Overview

The Tier 2 Systems Biology capstone synthesises the mathematical modelling and network analysis skills of the tier into a complete cycle: from biological concept through mathematical design, computational analysis, stochastic characterisation, and experimental proposal. The central project is the design and analysis of a synthetic genetic oscillator based on the repressilator architecture (Elowitz and Leibler 2000), but generalised to allow rigorous parameter exploration and comparison with the original experiment.

---

## Biological Motivation

Biological oscillators underlie many critical processes: the circadian clock, the cell cycle, somitogenesis oscillators in vertebrate development, and the NF-κB inflammatory oscillator. Understanding what makes a biological circuit oscillate — and why oscillations might be preferred over bistability or graded responses in specific contexts — is a foundational question in systems biology.

The repressilator is an instructive case precisely because it was designed, rather than evolved: it demonstrates that oscillations can emerge from a simple negative feedback topology, and it provides a controlled setting for studying how noise affects oscillation period, amplitude, and coherence.

---

## Project Components

### Component 1: ODE Model Design and Analysis (Weeks 1–2)

**The repressilator equations.** The three-gene repressilator (genes $A$, $B$, $C$, with proteins $a$, $b$, $c$) is described by:

$$\frac{dm_A}{dt} = \frac{\alpha}{1 + c^n} - m_A, \quad \frac{da}{dt} = m_A - a$$

with cyclic permutations for $m_B$, $b$, $m_C$, $c$. Here $\alpha$ is the maximum transcription rate, $n$ is the Hill coefficient, and units are chosen so that degradation rates are 1.

**Tasks:**
- Implement these equations in Python. Non-dimensionalise to identify the key parameter combinations.
- Find the fixed point analytically (by symmetry, the unique fixed point is $m_A = m_B = m_C = a = b = c = \alpha/(1+(\alpha/(1+\alpha))^n)$; solve numerically for specific $(\alpha, n)$ pairs).
- Perform linear stability analysis of the fixed point: compute the Jacobian, find eigenvalues, and determine the conditions on $\alpha$ and $n$ for the fixed point to be unstable (a prerequisite for limit cycle oscillations).
- Map the parameter space: for $\alpha \in [1, 1000]$ and $n \in [1, 4]$, determine (numerically) whether the system oscillates or converges to the fixed point. Plot this as a bifurcation diagram.
- For parameter values that produce oscillations, compute: (a) the period $T$ as a function of $\alpha$ and $n$; (b) the amplitude of oscillations; (c) the phase relationships among the three proteins.

**Key result to derive:** The Hopf bifurcation boundary in $(\alpha, n)$ space. For $n = 2$, find the critical $\alpha$ above which oscillations occur.

**Deliverable:** Bifurcation diagram, period/amplitude heatmaps over parameter space, phase portrait for an oscillatory parameter set, linear stability analysis (eigenvalue computation).

### Component 2: Extension to More Realistic Models (Week 3)

The simple repressilator model ignores important biological features. In this component, you systematically extend the model and assess the impact of each extension.

**Extension 1: mRNA/protein timescale separation.** In the original model, mRNA and protein have equal degradation rates. When the protein is more stable than the mRNA (a common biological scenario), the dynamics change qualitatively. Repeat the bifurcation analysis with separate mRNA and protein degradation rates $(\gamma_m, \gamma_p)$.

**Extension 2: Cooperative repression.** The Hill coefficient $n$ parametrises cooperativity. In biological repressilators, cooperative binding is achieved by protein oligomerisation or multiple operator sites. Explore how $n = 1, 2, 3, 4$ affects the oscillation amplitude, period, and the robustness of oscillations (size of the parameter region giving oscillations).

**Extension 3: Protein dimerisation.** Some repressilator implementations use dimers as repressors. Add protein dimerisation: $2a \rightleftharpoons a_2$ with equilibrium constant $K_d$. Only the dimer represses gene $B$. How does this change the effective Hill coefficient and the oscillation dynamics?

**Deliverable:** Comparative analysis (plots and brief discussion) showing how each extension modifies the oscillation properties. Identify which extension has the largest effect.

### Component 3: Gillespie Stochastic Simulation and Noise Analysis (Weeks 4–5)

**Tasks:**
- Translate the ODE model into a set of elementary stochastic reactions:
  - mRNA synthesis (rate $\alpha/(1+c^n)$ in the appropriate units)
  - mRNA degradation (first-order)
  - Protein synthesis (rate proportional to mRNA)
  - Protein degradation (first-order)
- Implement the Gillespie SSA. Scale the system to realistic copy numbers (e.g., 10–1000 molecules per cell).
- Simulate 500 cells for 10 oscillation periods. For each cell, compute:
  - Mean period (time between successive peaks)
  - Coefficient of variation of the period (CV = σ_T / μ_T) — a measure of oscillation coherence
  - Mean amplitude
- Plot: (a) Overlay of 20 single-cell trajectories showing phase drift over time; (b) histogram of single-cell periods; (c) CV of period as a function of protein copy number (showing how noise decreases as copy number increases).
- At what protein copy number does the repressilator become a reliable, coherent oscillator? Compare with the copy numbers reported in Elowitz and Leibler (2000).

**The noise-period relationship.** Theory predicts that for a biochemical oscillator, the coherence time scales as $\tau_{coh} \sim N^{1/2}$ where $N$ is the molecular copy number (Cao, Zheng, and Petzold 2006). Test this relationship by simulating systems with different cell volumes (which scale all copy numbers proportionally) and computing $\tau_{coh}$ from the autocorrelation function of the oscillating variable.

**Deliverable:** Stochastic simulation code, trajectory plots, period distribution histograms, CV vs. copy number curves, analysis of noise-period relationship.

### Component 4: Metabolic Modelling Integration — FBA Analysis of a Related System (Week 6)

This component previews the metabolic modelling tools of Tier 2 by connecting the oscillator analysis to metabolic considerations.

**Tasks:**
- Using COBRApy and the *E. coli* iJO1366 genome-scale model:
  - Run FBA under aerobic growth on glucose. Record the predicted growth rate and ATP production flux.
  - Simulate the effect of overexpressing repressilator genes: add a "repressilator burden" term (a constant drain on amino acid pools corresponding to the extra protein synthesis). How much does this reduce growth rate?
  - This is a simplified model of the metabolic burden of synthetic circuits — a key consideration in synthetic biology design.
- Discuss: how does metabolic burden affect the oscillator's behaviour? (If growth rate decreases, the effective protein dilution rate changes — this shifts the bifurcation point.)

**Deliverable:** FBA burden analysis code and results, one-page discussion of metabolic coupling.

### Component 5: Experimental Design and Literature Comparison (Week 7)

**Tasks:**
- Compare your ODE model predictions quantitatively with the published repressilator data from Elowitz and Leibler (2000) and subsequent improvements (Potvin-Trottier et al. 2016, *Nature*).
  - What parameter values from your model best fit the reported period (~150 min in the original, ~30 min in the improved version)?
  - What structural changes (differences in degradation tags, reporter, copy number) explain the difference in oscillation quality between the two studies?
- Design an experiment to test whether your model's prediction about noise-period relationship is correct. Specify: the organism, the circuit design, the measurement method, the statistical analysis, and the expected outcome.

**Deliverable:** Model-data comparison (plot), experimental design (1 page).

---

## Assessment Rubric

| Criterion | Weight | Excellent | Proficient | Developing |
|-----------|--------|-----------|------------|------------|
| ODE analysis | 25% | Bifurcation analysis correct, Hopf boundary derived analytically, parameter space mapped | ODE model correct, bifurcation found numerically, partial parameter mapping | ODE errors, no bifurcation analysis |
| Model extensions | 15% | All extensions implemented and compared rigorously | Two of three extensions completed | One or zero extensions |
| Stochastic simulation | 25% | SSA correct, noise characterisation complete, coherence time analysed | SSA correct, basic noise analysis | SSA errors or no noise analysis |
| FBA integration | 10% | Burden analysis correct, metabolic coupling discussed | FBA runs but analysis shallow | FBA not performed |
| Literature comparison | 15% | Quantitative comparison with literature, experimental design rigorous | Qualitative comparison, vague experimental proposal | No literature comparison |
| Scientific writing | 10% | Clear, precise, figures well-captioned | Adequate | Unclear |

---

## Extension Challenges

**Extension A: Phase model and synchronisation.** Reduce the repressilator to a phase model (each oscillator described by a single phase variable $\theta$) and analyse synchronisation between coupled oscillators (cells connected by quorum sensing). Use Kuramoto model theory to predict the synchronisation threshold.

**Extension B: Evolutionary robustness.** Using the bifurcation diagram, estimate the evolutionary robustness of the repressilator: what fraction of random mutations (modelled as multiplicative perturbations to the parameters) maintain oscillatory behaviour? Compare with the natural circadian clock.

**Extension C: Comparison with the NF-κB oscillator.** The NF-κB transcription factor oscillates with a period of ~1–2 hours in response to TNF-α stimulation. Obtain the published NF-κB ODE model (Lipniacki et al. 2004) and compare its bifurcation structure with the repressilator. What is topologically similar? What is different?
