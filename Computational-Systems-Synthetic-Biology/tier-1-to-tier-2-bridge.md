# Bridge: Tier 1 (Bioinformatics) → Tier 2 (Systems Biology)

## What You Carry Forward from Tier 1

Tier 1 taught you to read and interpret the genome: to extract information from sequencing data and translate it into biologically meaningful patterns. Tier 2 asks a different kind of question — not "what is expressed?" but "why is it expressed?", "how do the interactions among these genes produce a coherent cellular behaviour?", and "can we predict what happens when we perturb the system?". This shift from description to mechanism is the central conceptual move of Tier 2.

### From Sequence Analysis and Genomics
- You can identify genes, variants, and their functional annotations. In Tier 2, you will use this to populate the nodes of a regulatory or metabolic network with real biological identity.
- **Differential expression results as data for network models**: the DEG lists you produced in Tier 1 are the empirical starting points for inferring regulatory networks (Gene Regulatory Network inference) and for validating the predictions of genome-scale metabolic models (FBA under gene knockout conditions).

### From Transcriptomics
- RNA-seq gives you the state of the transcriptome at a snapshot in time. Tier 2 asks: what *dynamics* produced that snapshot? This requires moving from static gene expression data to dynamic models — ODEs, Boolean networks, stochastic simulations.
- Time-series RNA-seq (measuring transcriptomes at multiple time points after a perturbation) is the bridge: it is a Tier 1 data type that requires Tier 2 analysis tools to interpret.

### From Structural Bioinformatics
- Protein structure determines protein function and interaction specificity. In Tier 2, the interaction graph (who interacts with whom, with what affinity) is the data; structural bioinformatics provides the mechanistic basis for those interactions.

---

## The Conceptual Leap Being Made

Tier 1 works on **lists**: lists of genes, lists of variants, lists of enriched GO terms. The world of Tier 1 is fundamentally enumerative — you are identifying, counting, and characterising the components of a biological system.

Tier 2 works on **networks and dynamics**: the question is how the *interactions* among components produce emergent, time-dependent behaviour. This requires a completely different conceptual apparatus:

**From correlation to causation.** Bioinformatics can tell you that gene A and gene B are co-expressed (their expression levels are correlated). Systems biology asks whether A *causes* B, whether B *causes* A, or whether they are both downstream of some third regulator C. This distinction requires mechanistic modelling — and mechanistic models make predictions that can be tested by perturbation experiments (knockouts, overexpression, drug treatment).

**From genes to systems.** A differentially expressed list of 3,000 genes is almost uninterpretable on its own. The systems biology framing — organising those genes into network modules, signalling cascades, and metabolic pathways — provides the conceptual structure within which those 3,000 genes become interpretable.

**From qualitative to quantitative prediction.** Tier 1 produces qualitative predictions ("gene A is upregulated under condition X"). Tier 2 produces quantitative predictions: "the phosphorylation of protein B increases by 3.2-fold after 10 minutes of stimulus, and the steady-state level of metabolite C decreases by 40%." Quantitative predictions are more falsifiable and more useful.

**From description to design.** The ultimate goal of Tier 2 is not just to understand existing systems but to design new ones — a capacity that will be fully deployed in Tier 3. Systems biology models provide the design rules; Tier 1 provides the characterisation data for those rules.

---

## Self-Assessment Questions

**From Tier 1 — ensuring readiness:**
1. You performed differential expression analysis on a bacterial RNA-seq dataset and found 800 upregulated genes. How would you determine which of these are likely to be primary targets of a transcription factor vs. secondary effects?
2. What is the difference between a read count and a normalised expression value (TPM, RPKM, or DESeq2-normalised counts)? Why does the choice matter for specific analyses?
3. You have identified a set of co-expressed genes across 20 RNA-seq samples. Describe how you would use this co-expression data to generate hypotheses about regulatory modules.

**Mathematical readiness for Tier 2:**
4. Write the ODE for the concentration of protein $P$ that is synthesised at rate $k_s \cdot [mRNA]$ and degraded at rate $\delta_p [P] + \mu [P]$ (where $\mu$ is the growth rate/dilution). What is the steady-state concentration of $P$?
5. What is a Jacobian matrix? How would you use the eigenvalues of the Jacobian to determine whether a fixed point of an ODE system is stable?
6. What is a bifurcation? Give one biological example of a bifurcation and describe what biological behaviour it corresponds to.

**Biological background for Tier 2:**
7. What is the difference between a metabolic pathway and a gene regulatory network? How are they related in practice (hint: think about how enzyme levels are regulated)?
8. What is flux balance analysis? State the assumptions of FBA and describe the optimisation problem it solves.
9. What is the Michaelis-Menten equation? What are $V_{\max}$ and $K_m$, and how are they measured experimentally?

---

## Recommended Review if You Feel Shaky

| Topic | Review resource | Time estimate |
|-------|-----------------|---------------|
| ODE systems | *Introduction to Systems Biology* (Uri Alon), Ch. 1–3 | 1 week |
| Phase plane analysis | Strogatz *Nonlinear Dynamics and Chaos*, Ch. 2–3 | 1 week |
| Linear algebra for dynamics | MIT 18.06 lectures on eigenvectors and eigenvalues | 3 days |
| Metabolic pathway biochemistry | Berg, Tymoczko, Stryer *Biochemistry*, Part III | 1 week |
| Python for scientific computing | SciPy documentation (odeint, optimize) + examples | 3 days |

---

## What Tier 2 Demands That Tier 1 Did Not

**Tolerance for abstraction.** A Boolean network model of a gene regulatory network is not "the biology" — it is an abstraction that captures certain logical features while ignoring many quantitative details. Tier 2 requires comfort with this level of abstraction: the willingness to use a simplified model for insight, while remembering that the simplification has costs.

**Mathematical maturity.** ODE modelling, stability analysis, and bifurcation theory require facility with calculus, linear algebra, and some familiarity with dynamical systems. If the mathematical sections of Tier 0 were completed superficially, this is the moment to return and solidify them.

**Iterative model refinement.** In Tier 1, when a tool produces an output, you accept it and move on. In Tier 2, when an ODE model produces a prediction that disagrees with data, you ask *why* and modify the model. This iterative cycle — model, predict, compare, revise — is the core activity of systems biology and requires a different cognitive rhythm than bioinformatics pipeline execution.

**Biological depth.** The systems biology questions of Tier 2 are harder to answer without deep biological knowledge. Understanding *why* a toggle switch is bistable requires knowing what biological systems are bistable (the lac operon, the lambda phage lysis/lysogeny switch, cell-fate decisions in development) and why bistability matters biologically (irreversible decisions, memory, robustness). Read primary papers, not just review articles.
