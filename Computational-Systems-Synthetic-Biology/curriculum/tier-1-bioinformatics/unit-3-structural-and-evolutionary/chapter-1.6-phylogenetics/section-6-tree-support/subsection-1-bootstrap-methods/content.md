# Bootstrap Methods

You have inferred the best tree. Now someone asks: but how confident are you? This question has a precise statistical answer, and it requires you to think carefully about what "confidence" means for a phylogenetic clade. Bootstrap support provides the most widely used answer — it estimates how reproducible the clade is across random subsamples of the data. It is not the probability that the clade is correct, but it is a principled measure of how consistently the data support it.

**Bootstrap support** is the most widely used measure of phylogenetic confidence in maximum likelihood analyses. It provides an empirical estimate of how reproducible a given clade is across different subsamples of the data — a form of resampling-based confidence that does not require distributional assumptions.

## Felsenstein's Nonparametric Bootstrap

The **nonparametric bootstrap** (Felsenstein, 1985) applies the general statistical bootstrap principle to sequence alignment data:

**Algorithm**:
1. From the original alignment of $L$ sites (columns), create a **bootstrap replicate** by sampling $L$ sites **with replacement** from the original alignment. This new alignment has the same number of sites but some columns are duplicated and others are absent.
2. Infer a ML (or NJ, or parsimony) tree from this bootstrap alignment.
3. Repeat steps 1–2 for $B$ replicates (typically $B = 100$–$1000$).
4. For each clade in the original tree, count the percentage of bootstrap replicates in which that clade appears. This is the **bootstrap support (BS)** value.

**Interpretation**:
- **BS ≥ 95%**: Strong support — the clade is recovered in 95%+ of resampled datasets. This is the conventional threshold for "strong phylogenetic support."
- **BS 70–94%**: Moderate support — the clade is likely correct but with some uncertainty.
- **BS 50–69%**: Weak support — treat with caution; may be resolved differently with more data.
- **BS < 50%**: Not typically reported on published trees (collapsed to polytomy).

The **BS ≥ 70% threshold** for "reasonable support" was established empirically by Hillis & Bull (1993) through simulation studies showing this corresponds roughly to a 95% probability of correctness under various conditions. However, this calibration is approximate and varies with dataset properties.

## What Bootstrap Tests

An important conceptual point: bootstrap support is **not** the probability that the clade is correct. Rather, it measures **repeatability** — how consistently the same clade is recovered when the data are resampled. A clade with BS = 95% means: if you repeatedly drew random samples of sites from the same underlying distribution, you would recover this clade 95% of the time.

This measures **data support** (how consistently the data support the clade) rather than a probability statement about the true tree. Bootstrap support can be 100% for an incorrect clade if the data have a systematic bias (e.g., long-branch attraction is consistent across all resampled datasets). You might expect that a BS of 100 means the clade must be correct — but it turns out that systematic model misspecification can produce high bootstrap values for wrong relationships, because the bias is present in every bootstrap replicate. This is one of the most important cautions in phylogenetic practice.

## UFBoot: Ultrafast Bootstrap Approximation in IQ-TREE2

Standard bootstrap requires running a full ML analysis for each of $B = 1000$ replicates — computationally prohibitive for large datasets (days of computation for a genome-scale alignment with 100+ taxa). **UFBoot2** (Hoang et al., 2018), implemented in IQ-TREE2, provides a fast approximation:

**Key innovation**: Rather than running full ML on each bootstrap replicate from scratch, UFBoot uses a resampling of estimated log-likelihoods (RELL) approximation: the log-likelihood of the best tree and competitor trees are estimated on each bootstrap replicate using the site-specific log-likelihoods already computed during the original ML analysis, without re-running the full optimization. Trees are then selected based on their bootstrap alignment likelihoods.

**Speed**: UFBoot runs ~10–100× faster than standard bootstrap, making 1000 bootstrap replicates practical even for large phylogenomic datasets.

**Calibration concern**: UFBoot values are systematically inflated relative to standard bootstrap — a UFBoot value of 95 does not correspond exactly to a standard bootstrap value of 95. IQ-TREE2 recommends using the `--bnni` flag (Bootstrap NNI) to improve calibration: this additionally performs NNI optimization on candidate bootstrap trees, substantially improving the correspondence between UFBoot and standard bootstrap values.

```bash
# IQ-TREE2 with UFBoot
iqtree2 -s alignment.fasta -m GTR+G4 -B 1000 --bnni -T 8 -o outgroup
```

## Distinguishing Bootstrap Support from Posterior Probability

A common mistake is to assume that a Bayesian posterior probability of 0.95 (95%) is directly comparable to a bootstrap support value of 95%. They are fundamentally different quantities:

- **Bootstrap 95%** (ML): Resample-based frequency — the clade appears in 95% of bootstrap replicates.
- **Posterior probability 0.95** (Bayesian): Probability statement — 95% of sampled trees in the posterior contain the clade, given the data and model.

In practice, Bayesian PPs tend to be higher than ML bootstrap values for the same clades, sometimes substantially. A PP of 0.95 often corresponds to an ML bootstrap of only 70–80%. Neither is universally "more correct" — they measure related but distinct things. When reading a paper, always note which support measure is being reported. A tree with nodes labeled "95" could have either very strong support (bootstrap) or only moderate support (posterior probability expressed as a percentage) depending on which method was used.

## Why This Matters

Bootstrap support is the primary evidence reported for phylogenetic clades in the literature — every figure caption with a phylogenetic tree will describe how bootstrap support was computed — and understanding what bootstrap tests and what it doesn't (probability of truth vs. resampling consistency) prevents systematic misinterpretation of phylogenetic confidence in comparative biology studies. Bootstrap support is necessary but not sufficient evidence for a clade. The sections that follow introduce additional support measures — concordance factors — that complement bootstrap by measuring the fraction of the data that actually support each relationship.
