# Strict and Relaxed Molecular Clocks

In 1965, Emile Zuckerkandl and Linus Pauling noticed something remarkable in their comparisons of hemoglobin sequences across species: the number of amino acid differences between species seemed to correlate with the time since their divergence from the fossil record. If human and horse hemoglobin differ by 18 amino acids, and horse and fish differ by 68 amino acids, and the fish-tetrapod divergence happened roughly 4× longer ago than the horse-human divergence, then hemoglobin appears to evolve at an approximately constant rate. They called this the molecular clock.

The molecular clock hypothesis, if true, has an extraordinary implication: you can use sequence divergence as a proxy for time. Every pair of sequences becomes a potential chronometer. It turns out the clock is real but imperfect — rates vary between lineages, between genes, and between different periods in the same lineage — and the challenge is building models that accommodate this variation while still extracting temporal information.

The **molecular clock hypothesis** proposes that DNA and protein sequences evolve at approximately constant rates over time, analogous to a physical clock. If true, the amount of molecular divergence between two lineages directly reflects the time elapsed since their common ancestor — enabling calibration of phylogenetic trees on absolute timescales. In practice, the strict clock is almost always violated to some degree, necessitating relaxed clock models.

## The Strict Molecular Clock

Under a **strict molecular clock**, every lineage in the tree evolves at the same rate $\mu$ (in substitutions per site per year). The branch length $t \cdot \mu$ (in substitutions per site) is the product of time and rate. If $\mu$ is the same for all branches, then branch lengths directly measure time.

Mathematically, the strict clock constrains the tree to be **ultrametric**: all tips must be equidistant from the root in terms of substitutions per site. This is a testable constraint — deviations from ultrametricity reveal rate variation among lineages.

**Testing the strict clock**: A likelihood ratio test compares the log-likelihood of the ML tree with branch lengths unconstrained vs. with branch lengths constrained to satisfy the ultrametric condition. The test statistic $\Lambda = 2(\ell_{\text{unconstrained}} - \ell_{\text{clock}})$ follows a $\chi^2$ distribution with degrees of freedom equal to the number of branches minus the number of free rate parameters (1 for strict clock). Rejection of the null (clock model) indicates rate variation.

## Rate Variation Across Lineages

Rate variation is pervasive in molecular evolution. Key factors driving rate differences between lineages:

**Generation time effect**: Organisms with shorter generation times (rodents, insects) evolve faster per year than those with longer generation times (elephants, whales). This is because replication errors (the primary source of mutations in nuclear DNA) accumulate per generation.

**Effective population size ($N_e$)**: Under neutral evolution, smaller populations fix neutral mutations faster (due to stronger genetic drift), while larger populations maintain more polymorphism but fix individual mutations more slowly.

**Metabolic rate (body size)**: For mitochondrial DNA, oxidative damage is correlated with metabolic rate, which scales with body size.

**Life history**: Parasites often evolve faster than their hosts. RNA viruses (no proofreading) evolve ~10^6 times faster per year than mammalian nuclear DNA.

The generation time effect alone makes a strict clock untenable for datasets that include organisms with very different generation times. A mouse and a whale both exist in the present day, but the mouse has undergone vastly more cell divisions — and thus accumulated far more replication errors — in the same calendar time. Treating their branches as having the same rate per year is simply wrong.

## Relaxed Molecular Clocks

**Relaxed clocks** allow different lineages to have different rates, accommodating the reality of rate variation:

**Uncorrelated Lognormal Clock (UCLN)**: Each branch has an independent evolutionary rate drawn from a lognormal distribution with mean $\mu$ and standard deviation $\sigma$. This is the most widely used relaxed clock in BEAST2. The lognormal is appropriate because rates are strictly positive, typically vary over a few orders of magnitude, and show some clustering around a mean. Parameters estimated: the mean rate, the variance of the lognormal (coefficient of variation, CV = $\sigma/\mu$), and the actual rate for each branch (MCMC samples from the prior).

**Autocorrelated clock**: Rates are correlated between parent and child branches — a branch inherits its rate partly from its parent, with deviation drawn from a distribution. This models the biological intuition that rate changes are gradual (a lineage doesn't suddenly become 10× faster). The **lognormal autocorrelated clock** (TK02 model in BEAST2) implements this.

**Local clock model**: A few pre-specified lineages are allowed to have independent rates, while the remaining lineages share a single rate. Used when only specific clades are known to have accelerated evolution (e.g., viral evolution within a host vs. between hosts).

## Testing Which Clock Model to Use

In BEAST2, model comparison uses:
1. **Path sampling / Stepping-stone sampling**: Estimates the marginal likelihood for each clock model. Compare using Bayes factors: $\text{BF} = P(D \mid M_1)/P(D \mid M_2)$. BF > 10 is considered strong evidence for $M_1$.
2. **AICM** (Akaike Information Criterion in an MCMC context): Approximates the marginal likelihood from the harmonic mean of the posterior likelihoods (but the harmonic mean estimator is unreliable — stepping-stone is preferred).
3. **Coefficient of variation test**: In a UCLN clock model, if the estimated CV for branch rates is close to 0, the strict clock is adequate. If CV >> 0, rate variation is significant.

## Substitution Rate Variation: Examples

| Organism/Sequence | Rate (substitutions/site/year) |
|---|---|
| SARS-CoV-2 | ~1 × 10^-3 |
| HIV-1 | ~3 × 10^-3 |
| Influenza A | ~5 × 10^-3 |
| Mammalian mtDNA | ~2 × 10^-8 |
| Mammalian nuclear DNA | ~2 × 10^-9 |
| Bacteria (16S rRNA) | ~1 × 10^-9 |

The 10^6-fold rate difference between RNA viruses and mammalian nuclear DNA reflects the combined effect of error-prone RNA replication, high generation rate, and selection on the virus population. This is why tip dating works for SARS-CoV-2 (enough evolution accumulates in weeks to months to detect) but requires fossil calibrations for mammalian phylogenies where rates are so slow that detectable change takes millions of years.

## Why This Matters

The choice of clock model directly affects divergence time estimates — using a strict clock when rates genuinely vary can produce node age estimates that are off by factors of 2–10 — making the relaxed clock UCLN model the appropriate default for most real biological datasets, with formal model testing to justify the choice. The molecular clock, for all its imperfections, remains one of the most powerful ideas in evolutionary biology: it transformed phylogenetics from a tool for describing relationships into a tool for reconstructing evolutionary timelines.
