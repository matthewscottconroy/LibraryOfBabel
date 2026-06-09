# Replicates: Biological vs. Technical

Pick up almost any cell biology paper and count the n. Three. The number three appears so ubiquitously that it has become a kind of tacit rule — three experiments, three lanes on a Western blot, three wells on a plate. But n = 3 for what? That question turns out to matter enormously, because the word "replicate" is used in two fundamentally different senses in biological research, and confusing them is one of the most common statistical errors in published biology. **Biological replicates** are independent biological samples; **technical replicates** are repeated measurements of the same sample. These two types capture completely different sources of variation, and they are not interchangeable in statistical analysis.

## The Core Distinction

**Biological replicates:** Independent biological units — different organisms, different cultures, different patients, different litters. Each biological replicate is a separate individual with its own genetics (if outbred), cell-to-cell variability, and microenvironmental history. Biological replicates capture **biological variability** — the variability in the phenomenon being studied across independent realizations of it.

**Technical replicates:** Repeated measurements of the same biological sample. The same RNA extract measured by qPCR in triplicate. The same protein lysate run on three parallel Western blot lanes. The same bacterial culture split into three wells for a plate reader assay. Technical replicates capture **measurement variability** — instrument noise, pipetting error, signal detection variability.

## Why the Distinction Matters for Statistics

Statistical tests (t-test, ANOVA, Wilcoxon) make inferences about populations based on samples. The sample must consist of **independent observations**. Three measurements of the same RNA extract are not three independent observations of gene expression — they are one observation measured three times with instrumental noise. Using the three technical replicate measurements as if they were three biological observations inflates the apparent precision of your estimate and produces **pseudoreplication** — a well-documented statistical error.

**Example of pseudoreplication:**

You grow one flask of cells expressing a synthetic gene circuit and one flask of cells with the control. You split each flask into three wells and measure GFP fluorescence from each well. If you report n=3 for each condition and perform a t-test on those six values, you are pseudoreplicating: you have 1 biological replicate per condition (one flask), with 3 technical replicates of the GFP measurement. The t-test requires independent biological replicates. The correct analysis would either average the three technical replicates and report them as a single observation, or acknowledge that n=1 and provide no statistical test.

**The correct design:** Three independently grown flasks per condition, each measured once (or each averaged across technical replicates before statistical analysis).

## When Technical Replicates Are and Are Not Needed

Modern instruments are extremely precise. The coefficient of variation (CV) of technical measurements is typically:

- Automated plate reader (OD600): < 2%
- Flow cytometry (fluorescence intensity): 3–5%
- Next-generation sequencing (library preparation): 5–10%
- qPCR (Ct value): 0.1–0.3 Ct (corresponding to ~1–2% in expression units)
- Mass spectrometry (metabolomics): 5–15% depending on metabolite and platform

For most experiments, **technical variability is small relative to biological variability**. If your biological CV is 30% (typical for gene expression), the 2% technical CV contributes essentially nothing to your uncertainty. In these cases, technical replicates are unnecessary and waste resources.

**When technical replicates are justified:**
- During assay development, to characterize measurement precision
- For samples that are difficult or impossible to prepare independently (e.g., patient biopsies with limited material)
- For measurements with high technical variability (some metabolomics or proteomics assays)
- When you need an accurate estimate of technical variance for modeling purposes

**The general rule:** **Invest resources in biological replicates, not technical replicates.** Three independently grown cultures measured once each is almost always better than one culture measured three times.

## Minimum Biological Replicates for Common Experiments

**Cell biology (in vitro):**
- Most assays: n ≥ 3 independent cultures/experiments; n ≥ 5 for high-variance assays
- Flow cytometry: n ≥ 3 independent transfections or cultures; cells per sample ≥ 10,000

**Animal experiments:**
- Typically 5–10 animals per group, depending on effect size and variability
- Power calculation required (see Statistical Power section); n=3 is almost always underpowered for animal studies

**RNA-seq (differential expression):**
- Minimum: n = 3 per condition for basic analysis
- Recommended: n ≥ 5–6 per condition for detection of small effect sizes (2-fold change or less)
- Clinical samples: n ≥ 8–10 per group minimum; power analysis required

**CRISPR screens:**
- 300–500x coverage (number of guide RNAs × desired coverage = number of cells needed)
- Typically n = 2–3 biological replicates of the entire screen

## Handling Paired vs. Unpaired Designs

Some experimental designs produce paired measurements — for example, measuring gene expression before and after treatment in the same culture, or measuring the same cell line under two conditions in the same experiment. In paired designs:

- The unit of analysis is the **paired difference** (after − before, or condition A − condition B)
- The appropriate test is a **paired t-test** (parametric) or **Wilcoxon signed-rank test** (nonparametric)
- Paired tests are more powerful than unpaired tests when the pairing reduces variability

Incorrectly using an unpaired test on paired data loses statistical power (you fail to use the information that reduces variability). Incorrectly using a paired test on unpaired data is an error (you create spurious pairings).

## Reporting Replicates in Papers

The publication standard for reporting replicates has tightened considerably in the past decade, driven by the replication crisis. The correct format is:

- "n = X independent experiments, each performed in Y technical replicates" — specifying both n and the replicate structure
- "Mean ± SD from n = 4 biological replicates" — reporting the biological n and the error type together
- Flow cytometry: "n = 10,000 cells per sample from three independent cultures"

**What to never write:**
- "n = 9" when you have 3 biological replicates in triplicate (this is misleading about statistical power)
- "Experiments were repeated three times" without specifying whether these were biological or technical replicates

**In figure captions:** Each figure should state n, the error bar type (SD, SEM, or CI), and the statistical test used. These are not optional details — they are required for the figure to be interpretable.

## Dealing with Outliers

Outliers in biological data may represent:
- Technical failures (the outlier replicate had a pipetting error, contamination, or instrument malfunction)
- True biological variability (one culture genuinely behaved differently)
- True biology that should be investigated (an interesting non-typical response)

**The correct procedure:**
1. Establish an outlier exclusion rule *before* collecting data (in the experimental plan)
2. Investigate outliers rather than immediately excluding them
3. If exclusion is justified (documented technical failure), report the exclusion and the reason
4. If exclusion is not clearly justified, report the data including the outlier

GRUBBS test and ROUT test (implemented in GraphPad Prism) provide statistical criteria for outlier identification, but these are guides, not automatic justifications for exclusion.

## Takeaway

The biological/technical replicate distinction is not a technicality — it is fundamental to whether an experiment's statistical analysis is valid. Biological replicates capture the variance that matters for biological inference; technical replicates capture measurement noise. Pseudoreplication (treating technical replicates as biological) inflates statistical significance and produces false positives. The practical rule: invest in biological replicates, minimize technical replicates, and always report which type of replicate n refers to in every figure caption.
