# Statistical Power: Designing Experiments That Can Answer Your Question

Suppose you are testing a new synthetic gene circuit designed to suppress a cancer-associated gene. You grow three flasks of cells with the circuit and three without, measure expression, and find no significant difference. You conclude the circuit doesn't work and abandon the approach. But wait — what if the circuit really does suppress expression by 40%, but your experiment was too small to detect it? You have just discarded a working biological tool because you ran an underpowered test. This is not a hypothetical failure mode. It is a documented, quantifiable problem that explains a significant fraction of irreproducible biology. Statistical power is the probability that your experiment will detect a real effect if one exists. An experiment with low power may fail to detect genuine biological phenomena, waste resources, and produce ambiguous results. Planning for adequate power before collecting data is one of the most important — and most neglected — practices in experimental biology.

## The Four Elements of a Hypothesis Test

Any statistical test operates with four interrelated quantities. Fixing any three determines the fourth:

**1. Alpha (α) — Significance level (Type I error rate)**
The probability of a false positive: rejecting the null hypothesis when it is true. By convention, α = 0.05 (5% false positive rate). In genomics with many simultaneous tests, α is adjusted downward by multiple comparisons correction.

**2. Power (1 − β) — Sensitivity**
The probability of correctly detecting a true effect. A study with 80% power will detect a real effect 80% of the time; it will miss it 20% of the time. The conventional minimum target is 80% power; 90% is preferred for well-resourced studies.

**3. Effect size — The magnitude of the biological difference**
How large is the effect you are trying to detect? Effect sizes are typically expressed in standardized units (Cohen's d for comparing two means, r for correlation, f for ANOVA). A large effect size requires smaller n; a small effect size requires larger n.

**4. Sample size (n)**
The number of independent observations per group. The required n increases as effect size decreases and as the desired power increases.

## Why n = 3 Is Almost Always Underpowered

In a typical two-group comparison (t-test) with α = 0.05 and 80% power:

| Effect size (Cohen's d) | N per group required |
|---|---|
| 2.0 (very large) | 6 |
| 1.0 (large) | 17 |
| 0.8 (large) | 26 |
| 0.5 (medium) | 64 |
| 0.2 (small) | 394 |

**Cohen's d = 0.8** is classified as a "large" effect, yet requires 26 observations per group for 80% power. The ubiquitous n = 3 in cell biology has approximately 15–20% power to detect a Cohen's d of 0.8 — meaning it misses genuine effects 80% of the time. The only scenario in which n = 3 has adequate power is when the effect size is extremely large (d > 2, corresponding to a 2-standard-deviation separation between group means) — which is rarely the case in the noisy context of cell biology.

**Why does n = 3 persist?** Custom, tradition, cost, and time. Most cell biology labs established their replicate norms before power analysis was a standard consideration. Changing to n = 5–10 biological replicates adds cost and time but dramatically improves the reliability of results.

## Power Calculation: Practical Tools

### G*Power (Desktop Application)

G*Power (gpower.hhu.de) is the standard tool for power calculation. It handles t-tests, ANOVA, correlation, regression, chi-square, and many other test families.

**Workflow for a t-test:**
1. Select Test family: t tests
2. Statistical test: Means: Difference between two independent means
3. Type of power analysis: A priori (compute N given power, α, effect size)
4. Input: Effect size d = 0.8, α = 0.05, Power = 0.8
5. Output: N = 26 per group

**For a one-way ANOVA:**
1. Test family: F tests → ANOVA: Fixed effects, omnibus, one-way
2. Input: Effect size f = 0.4 (corresponding to η² = 0.14, "large"), α = 0.05, Power = 0.8, number of groups = 3
3. Output: Total N = 66 (22 per group)

### R (pwr package)

```r
library(pwr)

# Two-sample t-test: what n for 80% power, Cohen's d = 0.8?
pwr.t.test(d = 0.8, sig.level = 0.05, power = 0.8, type = "two.sample")
# n = 25.52 → 26 per group

# What power do I have with n=3 per group and d=0.8?
pwr.t.test(d = 0.8, sig.level = 0.05, n = 3, type = "two.sample")
# power = 0.196 → 20%

# What is the minimum detectable effect size with n=5 per group at 80% power?
pwr.t.test(n = 5, sig.level = 0.05, power = 0.8, type = "two.sample")
# d = 1.84 → very large effect required
```

### RNA-seq Power (RNASeqPower package)

RNA-seq differential expression analysis has its own power considerations, because the relevant parameters include read depth, biological variability (coefficient of variation), and fold change.

```r
library(RNASeqPower)

# What power for n=3 per group, depth=20M, CV=0.4, fold change=2?
rnapower(depth = 20, n = 3, cv = 0.4, effect = 2, alpha = 0.05)
# Power ≈ 0.31 → 31% — extremely underpowered

# What n for 80% power with the same parameters?
rnapower(depth = 20, n = NULL, cv = 0.4, effect = 2, alpha = 0.05, power = 0.8)
# n ≈ 8–10 per group
```

These numbers explain why RNA-seq studies routinely fail to replicate — the default n = 3 has only 31% power to detect a 2-fold change at a CV of 0.4.

## Estimating Effect Size Before the Experiment

Power calculation requires an effect size estimate, which creates a chicken-and-egg problem: you don't know the effect size until you've done the experiment. Common sources of effect size estimates:

**From pilot data:** Run a small (n = 3–5) pilot experiment and use the observed mean difference and standard deviation to estimate d. Use this estimate for the main experiment's power calculation. Be aware that effect sizes from small pilots are overestimated (winner's curse); add a safety margin.

**From published literature:** Find papers that measured a similar effect in similar conditions. Extract means and standard deviations from figures (WebPlotDigitizer, webplotdigitizer.com, can extract values from published figures).

**Minimum biologically meaningful difference:** Rather than estimating the expected effect size, specify the minimum effect you would care about biologically (e.g., "a fold change of less than 1.5 is not meaningful for our application") and power the study to detect that minimum.

## Type I vs. Type II Errors in Context

**Type I error (false positive):** You conclude there is an effect when there is none. Controlled by α. In clinical diagnostics, false positives lead to unnecessary treatment; the cost is often low tolerance for false positives.

**Type II error (false negative):** You miss a real effect. Controlled by β = 1 − power. In drug discovery or gene function studies, false negatives mean abandoning real targets; the cost of high false negative rates is missing discoveries.

**The asymmetry:** For discovery-stage research, false negatives (missing real effects) are often the greater harm — they lead to abandoning potentially important biology. For confirmatory research (clinical trials, safety studies), false positives are more dangerous. Set your power threshold accordingly: 80% for exploratory research, 90% or higher for confirmatory research.

## Power for -Omics Experiments

**GWAS (Genome-Wide Association Studies):** For common variants (MAF > 0.05) with realistic effect sizes (OR ≈ 1.2), studies typically require N = 50,000–100,000 participants per group. This is why GWAS requires consortium-scale collaborations.

**Proteomics:** Power depends on the number of proteins quantified, the false discovery rate target, the expected fold change, and the sample-to-sample variability. The general minimum is n = 5 per group; n ≥ 10 is preferred.

**Metabolomics:** Similar to proteomics. High CV of metabolite measurements (10–30% for many metabolites) requires larger n than gene expression studies.

## Reporting Power in Papers

The methods section of a publication should include a sample size justification:

*"Sample size was determined by power analysis using G*Power. Assuming an effect size of Cohen's d = 0.8, α = 0.05, and desired power of 80%, we determined that n = 26 biological replicates per group were required for a two-sided t-test."*

Or for pilot-data-based estimates:

*"Based on a pilot experiment (n = 3 per condition), we estimated the standard deviation of our reporter assay to be σ = 0.15 (in normalized units) and the expected mean difference to be 0.35. We therefore calculated a required n of 8 per condition to achieve 80% power at α = 0.05 (G*Power, version 3.1.9)."*

## Takeaway

Statistical power is the probability of detecting a real effect. The ubiquitous n = 3 in cell biology experiments is severely underpowered for all but the largest effect sizes, explaining a major fraction of the biological literature's replication failures. Power calculations require estimating effect size (from pilot data or literature) and target power (minimum 80%), and should be performed before data collection, not after. Tools including G*Power and the pwr and RNASeqPower R packages make power calculation accessible and should be part of every experimental design workflow.
