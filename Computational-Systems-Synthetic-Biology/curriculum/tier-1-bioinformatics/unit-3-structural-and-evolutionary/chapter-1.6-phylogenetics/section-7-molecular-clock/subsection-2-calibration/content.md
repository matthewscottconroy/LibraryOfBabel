# Molecular Clock Calibration

A molecular clock, on its own, tells you relative time: lineage A diverged from lineage B twice as long ago as lineage C diverged from lineage D. To convert this into absolute time — in years or millions of years ago — you need an external anchor. This is calibration, and it is where molecular clock dating meets the real world of imperfect evidence from fossils, geology, and the physical record of Earth's history.

A molecular clock transforms relative divergence (branch lengths in substitutions per site) into absolute divergence times (in years or millions of years ago). This transformation requires **calibration** — anchoring the molecular evolutionary timescale to known time points from external evidence. Calibration is the most challenging and most consequential step in molecular clock dating.

## Fossil Calibration: Minimum Age Bounds

**Fossils** provide direct evidence that a clade existed at a certain time: the age of a fossil of a member of clade X is a **minimum bound** on the age of the clade (the clade must be at least as old as its oldest fossil). Fossils cannot provide maximum bounds (we cannot know when a lineage began from a fossil — only when it was at least present).

This asymmetry is crucial and frequently misunderstood. A 65-million-year-old fossil of a mammal does not mean mammals first appeared 65 million years ago. It means they existed at least by 65 million years ago. The true origin of the lineage could be earlier — and given the incompleteness of the fossil record, it almost certainly is. Setting a calibration prior that places all probability mass exactly at the fossil age is almost always wrong.

**Setting fossil calibrations in BEAST2**:

Fossil ages are incorporated as prior distributions on node ages. Because fossils provide minimum bounds, the prior distribution is typically:

**Lognormal prior**: The fossil age is the lower boundary of the lognormal distribution. The mean and standard deviation of the lognormal express uncertainty about how much older the clade might be (due to the incompleteness of the fossil record). A lognormal prior centered slightly above the minimum (e.g., with most probability mass 1–5 Myr above the fossil age) is often used.

**Uniform prior**: A hard minimum bound (= fossil age) and a soft maximum bound (chosen based on biogeographic or stratigraphic knowledge). Less commonly used because uniform priors are uninformative about the internal structure of the interval.

**Offset exponential**: Fossil age as offset; exponential distribution for additional uncertainty above the minimum. Mean represents typical ghost lineage length (time between the true clade origin and first fossil appearance).

In BEAUti, the calibration node is identified by constraining it to contain specific taxa (the MRCA of the clade), and the prior distribution is specified in the "Priors" panel.

## Tip Dating for Rapidly Evolving Organisms

For RNA viruses and other rapidly evolving organisms where sequence divergence over years is measurable, **tip dating** uses the **sampling dates** of the sequences themselves as calibration points. Since different sequences were collected at known calendar dates, and the virus has been evolving at a measurable rate, the sampling dates constrain the rate and timescale jointly.

BEAST2 automatically uses tip dates when they are provided in the sequence names or in a separate file. The tip dates must span a sufficient time range (typically ≥ 1 year for most RNA viruses) to constrain the rate. Temporal signal in the data is verified by a **root-to-tip regression**: the root-to-tip distance in the ML tree should correlate positively with sampling date (R² > 0.5 indicates adequate temporal signal).

```bash
# TempEst (Rambaut et al.) computes root-to-tip regression to check temporal signal
# in an ML tree before BEAST2 analysis
```

Tip dating is one of the most powerful features of modern phylodynamics. During the COVID-19 pandemic, BEAST2 analyses with tip dates from sequences collected in January through March 2020 were able to estimate that SARS-CoV-2 had been circulating since approximately November 2019 — a result with direct implications for understanding the outbreak's origins.

## Secondary Calibration from Published Divergence Times

When fossils are absent for a particular node, **secondary calibrations** use divergence time estimates from other published studies as calibration priors. For example, if the mammal-bird divergence is well-constrained at 310–340 Ma from multiple independent fossil-calibrated studies, this range can be used as a prior on the amniote MRCA node in a new analysis.

Secondary calibrations are less reliable than primary (fossil) calibrations because they incorporate uncertainty from the original study and propagate additional error. They should be used with wide prior distributions that reflect the full uncertainty from the primary study.

## Absolute vs. Relative Timing

Sometimes an absolute time calibration is not available, but the relative order of divergence events is informative. **Relative timing** analysis places all divergence events relative to one arbitrary root age. This is useful for comparing the tempo of diversification between clades without requiring an absolute clock.

## Calibration Uncertainty Propagation

A key advantage of Bayesian dating (BEAST2) over point estimate methods (r8s, BEAST 1) is that calibration uncertainty is properly **propagated** through the analysis: the prior distribution on calibration node ages is integrated over during MCMC, producing a posterior distribution on all node ages that correctly reflects uncertainty from both the molecular data and the calibration prior.

The **95% HPD interval** (Highest Posterior Density — the shortest interval containing 95% of the posterior probability) on each node age expresses the combined uncertainty from molecular data and calibration, and should be reported alongside point estimates. A divergence time estimate reported as "65 Ma" without a credible interval is misleading — the uncertainty is often 5–20 Myr or more, and that matters biologically when you are trying to correlate divergence times with geological events like continental drift or mass extinctions.

## Calibration in BEAUti

Example workflow for a mammalian phylogeny:
1. Import alignment in BEAUti.
2. Go to "Priors" → "Tree" → select MRCA of mammals node.
3. Assign a lognormal prior with offset = 65 Ma (K-Pg boundary, minimum age of placental mammals), mean = 3 Ma, SD = 1 (in log space) — expressing belief that the true mammal MRCA is likely 65–85 Ma, with uncertainty.
4. Run BEAST2; TreeAnnotator produces MCC tree with node age HPD intervals.

## Why This Matters

Molecular clock calibration transforms a topology into a timeline — enabling evolutionary biology to answer "when" in addition to "how many times" — but the quality of calibrations directly determines the reliability of divergence time estimates; overconfident calibration priors produce falsely precise node ages that propagate incorrectly into downstream analyses such as biogeographic reconstructions and diversification rate analyses. Calibration is simultaneously the most powerful and the most uncertain step in molecular clock dating, and every published dated phylogeny should be scrutinized for the quality and appropriateness of its calibration strategy.
