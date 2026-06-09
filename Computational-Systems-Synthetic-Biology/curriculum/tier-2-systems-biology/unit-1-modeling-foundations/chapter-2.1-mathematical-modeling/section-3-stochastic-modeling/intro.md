# Section 3: Stochastic Modeling

In 2002, Michael Elowitz looked through a fluorescence microscope at *E. coli* cells expressing two identical fluorescent reporters and found that the two reporters were not always equal. The cells were genetically identical. The reporters were driven by the same promoter. And yet, within individual cells, one reporter was sometimes higher than the other — not by a tiny amount, but by a factor of two or more.

This observation — stochastic gene expression — was not surprising in retrospect, but it crystallized a recognition that the field had been avoiding: at the molecular copy numbers where gene regulatory decisions are made, random fluctuations are not small corrections to the deterministic average. They are the dynamics.

This section develops the mathematical framework for stochastic biological modeling from the ground up. It is longer and more technically demanding than the ODE section, reflecting the fact that stochastic modeling is genuinely harder — not because the concepts are more abstract, but because probability distributions are harder to visualize and interpret than trajectories.

**Why stochasticity matters** (subsection 3.1) motivates the need for stochastic models with data and biological examples. The key insight: at typical transcription factor copy numbers (10–100 per cell), the coefficient of variation from Poisson fluctuations alone is 10–30%. Stochastic effects are not noise — they are biology.

**The Chemical Master Equation** (subsection 3.2) is the theoretical foundation: the exact governing equation for the probability distribution over molecular states. It provides the analytical solutions (Poisson, negative binomial) that anchor the intuition for all simulation methods.

**The Gillespie algorithm** (subsection 3.3) is the exact method for sampling trajectories from the CME. Understanding why it is exact — and what "exact" means in this context — is key to interpreting stochastic simulation results.

**Tau-leaping** (subsection 3.4) accelerates the Gillespie algorithm by allowing multiple reactions per step. The Chemical Langevin Equation that emerges from this approximation bridges discrete stochastic and continuous ODE models.

**Stochastic differential equations** (subsection 3.5) provide a continuous-variable framework for stochastic dynamics, with the CLE as the biologically relevant example. Itô calculus and the Euler-Maruyama method make SDEs numerically tractable.

**Noise decomposition** (subsection 3.6) provides the quantitative framework for separating intrinsic and extrinsic contributions to gene expression noise — a framework with direct experimental tools (the two-reporter assay) and clear biological interpretations.

**Stochastic gene expression models** (subsection 3.7) applies all of the above to the biologically most important case: the two-state promoter model of transcriptional bursting. The negative binomial mRNA distribution emerges from the interplay of promoter switching and mRNA turnover, providing a direct connection between molecular mechanism and single-molecule FISH data.
