# Critical Evaluation of Scientific Papers

In 2011, a team of scientists at Bayer Healthcare sat down and tried to reproduce the findings of 67 published oncology studies — papers that had passed peer review, been published in reputable journals, and were already influencing decisions about which drug candidates to pursue. They could confirm the results in only 14 of the 67 cases: a failure rate of about 79%. A similar exercise by Amgen reproduced only 11 of 53 "landmark" cancer biology papers. These are not stories about fraud — most of the failures reflected ordinary flaws in experimental design: inadequate controls, insufficient statistical power, selective reporting. Problems that a careful reader could often identify without running a single experiment.

Reading a paper is not the same as believing it. Science advances through claims that can be checked, challenged, and revised; the reader's job is not passive absorption but active interrogation. This section develops a systematic framework for evaluating the quality of evidence in both experimental and computational papers. The skills here are essential for any scientist who reads primary literature — which is every scientist.

## Why Critical Reading Matters

The popular conception of science as a self-correcting system that eventually eliminates error is broadly true but dangerously slow. The replication crisis of the 2010s revealed that a substantial fraction of published findings in psychology, biomedicine, and biology could not be reproduced. A 2015 analysis in Science (Open Science Collaboration) found that only 36–39% of psychological findings replicated when tested by independent labs. A 2011 analysis by Bayer scientists found that 65% of published preclinical oncology studies could not be confirmed internally. These failures stem from a predictable set of flaws: underpowered studies, p-hacking, HARKing (Hypothesizing After Results are Known), selective reporting, and inadequate controls. The system does correct itself — but on timescales of years to decades, during which the faulty results circulate, get cited, and inform grant proposals, clinical decisions, and subsequent experiments. A critical reader who can identify these problems on first encounter does not have to wait.

## The Framework for Experimental Papers

### 1. Identify the Central Claim

Every paper has one central claim — the finding the authors most want you to believe. The rest of the paper is scaffolding around it. This claim is usually stated in the abstract's final sentence, the last sentence of the introduction, and the opening of the discussion. Write it down in one sentence before you do anything else. Keep it visible throughout your evaluation. As you read, ask at every step: does this experiment actually support that claim, or only something related to it?

### 2. Evaluate the Controls

**The most common flaw in experimental biology is insufficient or absent controls.** A result without an adequate control is not a result — it is an observation awaiting interpretation. For every experimental condition, ask:

- Is there a **negative control** demonstrating that the assay produces no signal in the absence of the expected effect? Without a negative control, you cannot distinguish signal from background.
- Is there a **positive control** demonstrating that the assay can detect a known effect? Without a positive control, a negative result is uninterpretable — was there no effect, or did the assay fail?
- Are the controls **isogenic**? If comparing a mutant to wild type, genetic background differences between strains can produce large phenotypic differences unrelated to the mutation of interest. The correct control is the parent strain, not an unrelated strain.
- Is there a **vehicle control**? Any compound dissolved in DMSO, ethanol, or PBS requires a vehicle-only control, because DMSO at concentrations above 0.5% has measurable effects on mammalian cells.

### 3. Assess the Statistics

Statistical errors are among the most pervasive and underappreciated problems in the biological literature — not because biologists are innumerate, but because statistical power and multiple testing correction are genuinely counterintuitive at first encounter. Work through the statistics for every key result:

- **What is n?** Distinguish biological replicates (independent organisms, cultures, or samples) from technical replicates (repeated measurements of the same sample). An experiment reported as "n=9" that consists of three biological replicates each measured in triplicate has an effective biological n of 3, not 9. Conclusions about biological variability require biological replicates.
- **Is n sufficient?** For detecting a moderate effect size (Cohen's d ≈ 0.5), a two-sample t-test requires approximately 64 samples per group to achieve 80% power. The ubiquitous n=3 in cell biology papers is almost universally underpowered.
- **Was the correct test used?** Parametric tests (t-test, ANOVA) assume approximately normal distributions. Small n (< 30) with skewed distributions warrant nonparametric alternatives (Mann-Whitney, Kruskal-Wallis). Repeated measurements on the same subject require paired or mixed-effects analyses, not unpaired tests.
- **Was multiple-comparisons correction applied?** If a paper tests 20 conditions against a null hypothesis at p < 0.05, one false positive is expected by chance alone. Bonferroni correction, Benjamini-Hochberg FDR, or similar adjustments are required when testing multiple hypotheses simultaneously. Omitting correction — particularly in genomics, proteomics, or metabolomics — is a major error.
- **Are p-values overinterpreted?** A p-value below 0.05 does not mean an effect is biologically important — it means the effect is statistically distinguishable from zero given the sample size. A large enough experiment can produce p < 0.05 for a biologically trivial effect. Report and evaluate **effect sizes** (fold changes, Cohen's d, odds ratios) alongside p-values; effect size is what matters biologically.

### 4. Look for HARKing and p-Hacking

**HARKing** (Hypothesizing After Results are Known) occurs when authors present post-hoc hypotheses as if they were pre-specified predictions. Every experienced scientist knows this happens, though nobody discusses it openly. The tell-tale sign is a paper where the hypothesis fits the data with suspicious precision — every predicted direction is confirmed, every control works perfectly, and no experiment was ambiguous. Real biology is messier. Real experiments produce results that don't quite fit, controls that fail, and data that requires interpretation. A paper without any mess should raise your eyebrow.

**p-Hacking** — testing many variants of an analysis until p < 0.05 is achieved — is often invisible in the final paper. Signs include: reporting only a subset of measured outcomes, switching statistical tests without justification, ambiguous sample size ("data from 3–5 independent experiments"), or results that cluster just below p = 0.05. A funnel plot asymmetry in meta-analyses is a quantitative indicator of publication bias toward significant results.

### 5. Assess the Mechanism Claim

There is a crucial logical staircase — correlation, causation, mechanism — and papers frequently claim to be on a higher step than their evidence places them. Demonstrating that A and B co-occur is correlation. Demonstrating that manipulating A changes B is causation. Explaining why, at a molecular level, A affects B is mechanism. Many papers that claim mechanism have only shown causation; many that claim causation have only shown correlation. Ask:

- Does the evidence show correlation or causation? Correlation requires demonstrating that A and B co-occur; causation additionally requires showing that manipulating A changes B, and that no third variable C causes both.
- Is the proposed mechanism the only explanation for the data, or are alternative mechanisms equally consistent with the observations?
- Was the mechanism tested directly, or is it inferred from indirect measurements?

## The Framework for Computational Papers

Computational papers require a distinct checklist, because the most significant failure modes are structurally different from those in experimental work. The data cannot be "faked" in the same sense, but the analysis can be designed — consciously or not — to produce impressive-looking results that don't hold up outside the specific evaluation conditions chosen. The following failures are far more common than the literature would suggest:

- **Train/test leakage**: Does the model's evaluation use data that was also used in training or model selection? This inflates performance estimates. Look for explicit description of held-out test sets, cross-validation procedures, and whether hyperparameter selection was performed on training data only.
- **Appropriate baseline comparisons**: Is the method compared against the simplest possible baseline (random predictor, nearest neighbor, linear model) as well as the current best method? A paper that only reports performance versus a weak baseline is cherry-picking.
- **Parameter sensitivity**: Are results robust to changes in key parameters? A model that only works well with a specific parameter setting may not generalize.
- **Code and data availability**: Reproducibility is essentially impossible without accessible code and data. Papers without code should be treated with proportionally greater skepticism.
- **Independent validation**: Was the method validated on a dataset different from the one used for development? Independent validation by a different lab is the gold standard.

## Recognizing When to Update Your Prior

Critical reading is not the same as reflexive skepticism, and there is no virtue in finding fault with everything. The goal is calibrated confidence, not dismissal. A well-controlled, appropriately powered study with sound statistics from a lab with a strong track record deserves substantially more credence than a preliminary report with n=3 and no multiple-comparisons correction. Develop the habit of asking: what additional evidence would make me more or less confident in this claim? This framing is Bayesian — you are updating prior beliefs based on the quality and quantity of evidence, not making binary believe/disbelieve decisions.

When you find a paper with a significant flaw, the appropriate response depends on the severity. A minor statistical issue does not invalidate a result with large effect size and multiple independent experiments. A fundamental design flaw — no controls, systematic confound, inappropriate statistical model — does invalidate the central claim. In practice, most papers fall between these extremes: they make a real contribution while leaving genuine questions about the magnitude, mechanism, or generality of their findings. Holding both thoughts simultaneously — this result is real, and these questions remain open — is the mark of a scientifically mature reader.

## Practical Tools

Several resources support critical evaluation:

- **Statcheck** (statcheck.io): automatically checks statistical reporting consistency in PDF papers
- **GRIM test**: checks whether reported means are numerically possible given the reported n and scale
- **PubPeer** (pubpeer.com): community annotation of published papers, including identified errors and data concerns
- **Retraction Watch** (retractionwatch.com): tracks retracted papers and the reasons for retraction
- **PREreview** and **eLife's review process**: examples of open peer review that shows how professional referees evaluate specific papers

## Takeaway

Critical evaluation is a systematic practice, not an attitude. For experimental papers, examine controls, assess statistical power and test appropriateness, check for multiple-comparisons correction, and distinguish correlation from causation. For computational papers, focus on train/test integrity, baseline comparisons, parameter sensitivity, and code availability. HARKing and p-hacking are identifiable from structural features of the paper; learning to recognize them builds a kind of intellectual immunity to the literature's most pervasive failure modes. The goal is calibrated confidence: appropriately high when evidence is strong, appropriately provisional when it is weak. That calibration is, ultimately, what it means to think scientifically.
