# Evaluating Papers Critically

In 2012, a team of scientists at Amgen attempted to reproduce 53 landmark cancer biology papers — results that had influenced drug development programs, shaped grant portfolios, and been cited thousands of times in subsequent research. They succeeded with 6 of them. Six out of fifty-three. This is not a story about fraud; nearly all of those papers were written in good faith by careful scientists. It is a story about what happens when the incentive structures of publishing — reward positive results, penalize ambiguity, demand clean narratives — systematically distort the literature. Critical evaluation is the skill that separates a scientist from a reader. Every paper you encounter is a persuasive argument — constructed by authors who believe their interpretation, reviewed by peers who may share their assumptions, and published in a venue that rewards positive results. Your job as a reader is to engage with the argument honestly: to give appropriate credit to what is genuinely established while identifying where the evidence falls short of the claims, where alternative explanations exist, and where the methods introduce systematic biases.

This section addresses the most common and consequential failure modes in published biology: p-hacking, HARKing, replication failures, and methodological inadequacy. Understanding these failure modes makes you a better reader, a better designer of experiments, and a better author.

## The Replication Crisis: Background

Between 2011 and 2018, a series of high-profile failures to replicate published findings shook multiple scientific disciplines. The Open Science Collaboration (2015, Science) found that only 36–39% of psychology findings replicated in independent experiments. Begley & Ellis (2012, Nature) reported that scientists at Amgen could replicate only 6 of 53 landmark cancer biology papers. A 2011 Bayer analysis found that 65% of published oncology targets could not be confirmed internally.

The causes are well understood, if not yet fully addressed:

- **Publication bias:** Journals preferentially publish positive results. Studies with p > 0.05 are rarely submitted, and when submitted, rarely accepted. This means the published record over-represents false positives.
- **Underpowered studies:** Small sample sizes combined with publication bias create the "winner's curse" — the first published estimate of an effect is almost always an overestimate, because only the most extreme results cross the significance threshold.
- **P-hacking and flexible analysis:** Running multiple statistical tests and reporting only the significant one; stopping data collection when a result crosses p = 0.05; excluding outliers selectively.
- **HARKing:** Presenting post-hoc hypotheses as if they were pre-specified, inflating the apparent strength of confirmatory evidence.
- **Inadequate reporting:** Methods that cannot be reproduced prevent replication attempts.

These problems are systemic, not attributable to individual misconduct. Most p-hacking is unconscious. Most HARKing occurs when authors convince themselves, after seeing the data, that this was always what they expected to find.

## Detecting P-Hacking

P-hacking is the practice of analyzing data in multiple ways until p < 0.05 is obtained. It can take many forms:

**Outcome switching:** Running an experiment with multiple possible outcome measures (gene A expression, gene B expression, cell size, growth rate) and reporting only the one that is significant.

**Subsetting:** Running an experiment, finding a non-significant result overall, then examining subgroups ("this effect was significant in males but not females," "this effect was only present in the high-dose group") without pre-specifying the subgroup analysis.

**Sequential testing:** Collecting data in batches, checking significance after each batch, and stopping when significant. Without correction for sequential testing (which requires methods like alpha-spending or the O'Brien-Fleming boundary), this dramatically inflates the false positive rate.

**Sign of p-hacking to look for:**

- Results clustered just below p = 0.05 (suspicious distribution of p-values)
- Multiple outcome measures listed in the methods but only one reported in results
- Subgroup analyses without pre-registration
- "Data from N–M independent experiments" with ambiguous N and M
- Different statistical tests used in different figures without justification

**Statcheck** (statcheck.io) automatically extracts t-statistics, degrees of freedom, and p-values from Word or PDF papers and checks whether the reported p-value is consistent with the reported test statistic and df. It catches computational errors, and occasionally catches inconsistencies that suggest selective reporting.

## Detecting HARKing

HARKing (Hypothesizing After Results are Known) occurs when authors run an experiment, examine the data, formulate a hypothesis that explains the results, and then write the paper as if the hypothesis preceded the experiment. It is widespread because the scientific community rewards confirmatory stories.

**Signs of HARKing:**

- The hypothesis fits the data with implausible precision — every predicted direction matches, every control works perfectly, the magnitude of the effect was exactly as expected.
- The introduction motivates the study with reasoning that feels post-hoc — it explains why the result was expected given what was found, rather than why it was expected before the experiment.
- The methods describe a full experiment, but the paper was published very quickly (suggesting the experiment was designed around the result).
- Pilot studies are absent or not mentioned.

**Pre-registration as the antidote:** Pre-registration requires researchers to specify their hypothesis, sample size, design, and analysis plan before collecting data, with the specification deposited in a public registry (AsPredicted.org, OSF.io/prereg). Pre-registered studies are immune to HARKing because the hypothesis is demonstrably prior to the data. Some journals (Registered Reports track at PLOS ONE, Nature Human Behaviour) commit to publishing pre-registered studies regardless of outcome, eliminating publication bias.

In biology, pre-registration is less common than in psychology or clinical trials, but it is growing. When you see a pre-registered study, weight its confirmatory evidence more heavily than an equivalent unregistered study.

## Evaluating the Methods Section

The methods section is where reproducibility lives or dies. A paper with excellent results but an inadequate methods section cannot be reproduced — and a result that cannot be reproduced is not established science.

**Checklist for methods evaluation:**

**Biological materials:**
- Are strains, cell lines, plasmids, and antibodies specified by name and source?
- For antibodies: lot number, catalog number, and RRID (Research Resource Identifier)?
- For cell lines: source (ATCC number), passage number, authentication (STR profiling for human cell lines)?

**Reagents and consumables:**
- Brand and catalog number for critical reagents (restriction enzymes, sequencing kits)?
- For chemicals: purity grade, supplier, concentration?

**Equipment:**
- Instrument model and settings for critical measurements?
- Software version for image analysis, flow cytometry gating, or sequencing alignment?

**Statistical methods:**
- Which test was used for each comparison?
- Was it one-tailed or two-tailed?
- Was multiple comparisons correction applied?
- What was the alpha threshold?
- How were outliers handled?
- Was normality assumed, and if so, was it tested?

**Data availability:**
- Are raw data available in a public repository (GEO, SRA, Zenodo, Dryad)?
- Is analysis code available (GitHub link with specific commit hash)?

A methods section that cannot answer these questions prevents any reader from reproducing the work.

## Evaluating Statistics Without Running the Analysis

You do not need to re-analyze a paper's data to evaluate its statistics. Several structural features of a paper's statistical reporting are interpretable from the text alone:

**n matters more than p:** A p-value of 0.001 from n=5 is often less convincing than p=0.01 from n=50, because the smaller study is more likely to be a chance finding. Always ask: what is the biological n?

**Effect size vs. significance:** A statistically significant result with a small effect size may be meaningless biologically. Ask: what is the fold change? What is Cohen's d? Is this effect large enough to matter for the biology?

**Error bars:** Identify what the error bars represent. SD (standard deviation) shows the spread of the data. SEM (standard error of the mean) shows the uncertainty in the mean estimate — SEM is always smaller than SD by a factor of √n. A paper using SEM for figures with small n makes results look more consistent than they are. CI (confidence interval) is the most interpretable — it directly shows the range of plausible true values.

**The supplementary figures:** Main-text figures are the best-looking subset of the data. Supplementary figures often contain the full dataset, failed experiments, and troubleshooting. Reading them reveals the true scope and messiness of the work.

## When to Be More vs. Less Skeptical

Calibrate skepticism to evidence quality:

**More skeptical when:**
- N is small (< 5 biological replicates for a cell biology claim)
- The claim is surprising and would overturn established consensus
- The paper has no pre-registration and no code/data availability
- Multiple outcomes were measured and only one is reported significant
- The lab has a track record of non-replicable results (check PubPeer)

**Less skeptical when:**
- N is large and the experiment is well-controlled
- The result has been replicated in independent labs or by different methods
- The paper is pre-registered
- Effect sizes are large and biologically interpretable
- The finding is consistent with the broader mechanistic understanding

## Takeaway

Critical evaluation is not cynicism — it is calibrated skepticism applied systematically. The failure modes of published science (p-hacking, HARKing, underpowering, inadequate reporting) are identifiable from structural features of the paper without re-analyzing the data. Understanding the replication crisis context helps calibrate how much weight to place on any single study. The goal is to be neither credulous nor dismissive, but to hold beliefs with appropriate confidence relative to the quality of the evidence.
