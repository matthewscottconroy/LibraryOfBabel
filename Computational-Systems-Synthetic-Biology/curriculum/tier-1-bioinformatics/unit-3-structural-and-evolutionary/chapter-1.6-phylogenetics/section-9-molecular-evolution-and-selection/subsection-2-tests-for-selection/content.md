# Tests for Molecular Selection

dN/dS is the workhorse of between-species selection analysis. But it operates on deep evolutionary time — the substitutions fixed over thousands to millions of years between species. Within a species, evolution is still happening. Mutations arise, drift or selection moves them through the population, and some become fixed while others are lost. The within-population frequency spectrum of variants contains a different kind of information about selection — one that complements the between-species signal. This section surveys the full toolkit of molecular selection tests, from population genetics to phylogenomics.

Beyond dN/dS analysis, a rich toolkit of statistical tests detects the signature of natural selection in molecular sequence data. These tests operate at different scales — population genetics (within species) vs. phylogenetics (between species) — and detect different modes of selection: directional (positive), purifying, and balancing.

## McDonald-Kreitman Test

The **McDonald-Kreitman (MK) test** (McDonald & Kreitman, 1991) compares the ratio of nonsynonymous to synonymous changes within a species (polymorphism) vs. between species (divergence):

|  | Nonsynonymous | Synonymous |
|---|---|---|
| Polymorphism (within species) | Pn | Ps |
| Divergence (between species) | Dn | Ds |

**Null hypothesis** (neutral): Synonymous and nonsynonymous changes accumulate at the same ratio in polymorphism and divergence: Pn/Ps = Dn/Ds.

**Positive selection signature**: If adaptive evolution drives nonsynonymous divergence, Dn/Ds > Pn/Ps. The ratio is tested by a $2 \times 2$ Fisher's exact test (or $\chi^2$). A significant excess of nonsynonymous divergence relative to polymorphism is interpreted as evidence that positive selection fixed nonsynonymous changes.

**The Neutrality Index (NI)**:

$$\text{NI} = \frac{Pn/Ps}{Dn/Ds}$$

NI < 1 → excess nonsynonymous divergence (positive selection); NI > 1 → excess nonsynonymous polymorphism (slightly deleterious mutations segregating as polymorphisms).

**α (fraction adaptive substitutions)**: The fraction of nonsynonymous divergence driven by positive selection:

$$\alpha = 1 - \frac{Ds \cdot Pn}{Dn \cdot Ps} = 1 - NI$$

For Drosophila, α ≈ 0.5 — approximately 50% of amino acid divergence between Drosophila species was driven by positive selection. For humans, α is lower (0.05–0.25), reflecting smaller effective population size and higher prevalence of slightly deleterious mutations.

**Complication — Slightly deleterious mutations**: Slightly deleterious nonsynonymous mutations segregate at low frequency as polymorphisms but rarely reach fixation as divergence. This inflates Pn relative to Dn, making NI > 1 even in the absence of balancing selection. The **Direction of Selection (DoS)** and **asymptotic MK test** correct for this by excluding or downweighting rare polymorphisms.

The MK test is elegant precisely because it uses a within-gene control: synonymous sites provide the neutral expectation, so you don't need to know the actual neutral mutation rate. Any gene can serve as its own control, as long as you have both intraspecific polymorphism data and interspecific divergence data.

## PAML Branch-Site Model LRT: Episodic Positive Selection

As described in the previous subsection, the **PAML branch-site model LRT** detects positive selection acting on specific sites in specific lineages. The key implementation:

```bash
# In PAML codeml, run two models and compare:
# Null model: ω2 = 1 fixed (no positive selection on foreground branch)
# Alternative model: ω2 estimated freely (positive selection allowed)
# LRT: 2*(lnL_alt - lnL_null) ~ chi2_1 df
# p < 0.05 supports positive selection
```

**HyPhy BUSTED** (Branch-site Unrestricted Test for Episodic Diversification) is a more statistically robust alternative to PAML's branch-site model, available at the Datamonkey web server.

## Population Genetics Selection Tests

### Tajima's D

**Tajima's D** (Tajima, 1989) tests whether the population frequency spectrum of variants deviates from neutral expectation:

$$D = \frac{\hat{\theta}_\pi - \hat{\theta}_W}{\sqrt{\text{Var}(\hat{\theta}_\pi - \hat{\theta}_W)}}$$

where $\hat{\theta}_\pi$ = average pairwise nucleotide diversity (sensitive to intermediate-frequency variants) and $\hat{\theta}_W$ = Watterson's estimator based on the number of segregating sites (equally weighted by all frequencies).

**Interpretation**:
- **D ≈ 0**: Neutral equilibrium (or insufficient power to detect deviation).
- **D < 0**: Excess rare variants relative to intermediate-frequency variants. Consistent with: (a) **selective sweep** — positive selection recently fixed a beneficial mutation, dragging linked variants to low frequencies; (b) **purifying selection** — removing deleterious variants; (c) population expansion (many new rare variants).
- **D > 0**: Excess intermediate-frequency variants. Consistent with: **balancing selection** (heterozygote advantage or frequency-dependent selection maintains diversity, as at MHC loci); or population contraction/bottleneck.

Tajima's D requires large sample sizes (≥ 20 sequences) and is computed in sliding windows across the genome to detect regions of atypical selection.

### Selective Sweep Detection: iHS and XP-EHH

**Selective sweeps** leave genomic signatures of extended haplotype homozygosity (EHH): when a beneficial mutation rises rapidly to high frequency, it drags a long block of linked sequence with it, producing unusually long, common haplotypes in the population.

**iHS** (integrated Haplotype Score): Compares the extent of haplotype homozygosity around derived vs. ancestral alleles at each SNP. Derived alleles on long haplotypes (high EHH extending far from the focal SNP) are under positive selection:

$$|iHS| = \left|\frac{iHH_{\text{derived}} - iHH_{\text{ancestral}}}{\text{SD}}\right|$$

$|iHS| > 2$ indicates selection signal.

**XP-EHH** (Cross-Population EHH): Compares haplotype homozygosity between two populations, identifying loci where one population has unusually long haplotypes relative to another — evidence for population-specific selective sweeps. Used to identify signatures of local adaptation (e.g., lactase persistence in European populations, sickle cell trait in malaria-endemic populations).

It turns out that iHS and XP-EHH have revealed extensive signatures of recent positive selection in human populations, identifying loci involved in diet, immunity, and pathogen resistance that distinguish populations adapted to different environments. This has been transformative for understanding how recent human evolution has shaped health-relevant genetic variation.

## HyPhy: A Comprehensive Selection Analysis Platform

**HyPhy** (Hypothesis testing using Phylogenetics and Evolution) provides a suite of molecular evolution tests:

- **BUSTED**: Branch-site test for episodic diversification (more powerful than PAML branch-site model).
- **aBSREL** (adaptive Branch-Site Random Effects Likelihood): Tests each branch for episodic positive selection; automatically identifies foreground branches.
- **MEME** (Mixed Effects Model of Evolution): Site-level test for episodic positive selection; detects sites under positive selection on a subset of branches without requiring pre-specification of foreground branches.

HyPhy is available as a command-line tool and through the **Datamonkey** web server (datamonkey.org), making these analyses accessible without bioinformatics infrastructure.

```bash
# HyPhy MEME via command line:
hyphy meme --alignment alignment.fasta \
           --tree species_tree.nwk \
           --output meme_results.json
```

MEME is particularly powerful because it does not require you to know in advance which lineages are under positive selection. It identifies specific sites where selection was episodic on some branches but not others — a more realistic model of adaptive evolution than methods that assume uniform selection across all branches.

## Why This Matters

Tests for molecular selection connect genomic sequence variation to the evolutionary forces shaping genomes — revealing the molecular basis of local adaptation, pathogen escape from immunity, and the genetic architecture of disease resistance — making this toolkit fundamental for population genomics, evolutionary medicine, and the comparative genomics of adaptive traits. Together, the methods in this section span the full range from deep evolutionary time (dN/dS) to recent population-level dynamics (iHS, XP-EHH), giving you the tools to detect selection operating across every timescale.
