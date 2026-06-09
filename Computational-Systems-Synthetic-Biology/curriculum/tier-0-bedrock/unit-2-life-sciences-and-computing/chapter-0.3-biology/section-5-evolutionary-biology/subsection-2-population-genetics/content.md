# Population Genetics

In 1908, the mathematician G.H. Hardy and the physician Wilhelm Weinberg independently pointed out something that should have been obvious to everyone but wasn't: if you start with any mixture of genotypes in a randomly mating population, a single generation of random mating will bring genotype frequencies to a specific equilibrium that depends only on allele frequencies. This Hardy-Weinberg equilibrium is the null model of population genetics — the baseline from which all real data deviate. And deviations from equilibrium are the signal. When real genotype frequencies differ from Hardy-Weinberg predictions, something interesting has happened: selection, drift, inbreeding, admixture, or genotyping error. Population genetics is essentially the science of reading those deviations.

Population genetics provides the mathematical framework for describing how allele frequencies change across populations and over time. It bridges the molecular level (DNA sequences, mutations) with the population level (variation, divergence, demographic history). The central tools of population genetics — Hardy-Weinberg equilibrium, FST, Tajima's D, coalescent theory — are the statistical tests applied to genome sequencing data to detect selection, estimate demographic history, and understand the structure of genetic variation.

## Hardy-Weinberg Equilibrium

**Hardy-Weinberg equilibrium (HWE)** is the null model of population genetics: genotype frequencies expected in a large, randomly mating population with no selection, mutation, migration, or drift.

For a biallelic locus with alleles A (frequency $p$) and a (frequency $q = 1 - p$):

$$\text{Genotype frequencies: } p^2 (AA) : 2pq (Aa) : q^2 (aa)$$

This is simply the product of independent allele draws — a binomial expansion.

HWE is reached in a **single generation** of random mating (from any initial genotype frequencies). It is maintained indefinitely in the absence of evolutionary forces.

**Using HWE to detect departures:**
- Excess heterozygotes ($2pq$ observed > expected): recent admixture between differentiated populations, or heterozygote advantage
- Deficit of heterozygotes: population structure (Wahlund effect — HWE holds within subpopulations, but mixing subpopulations with different allele frequencies creates apparent heterozygote deficit at the meta-population level), inbreeding, or genotyping error
- GWAS quality control: loci that deviate from HWE in controls are flagged as potential genotyping artifacts

## Population Differentiation: FST

When populations are geographically or reproductively separated, allele frequencies diverge through drift and local selection. **FST (fixation index)** measures this differentiation:

$$F_{ST} = \frac{H_T - H_S}{H_T}$$

where $H_T$ is the expected heterozygosity in the total (pooled) population and $H_S$ is the average expected heterozygosity within subpopulations.

Interpretation:
- $F_{ST} = 0$: no differentiation; same allele frequencies in all subpopulations
- $F_{ST} = 1$: complete differentiation; each subpopulation is fixed for a different allele
- Human continental populations: $F_{ST} \approx 0.10$–0.15 at most loci; loci with $F_{ST} > 0.3$ are candidates for local adaptation

**Example calculation**: Population 1 has allele A at frequency 0.9; Population 2 has A at frequency 0.1. Equal sizes.

$p_{total} = 0.5$, $H_T = 2(0.5)(0.5) = 0.5$

$H_{S1} = 2(0.9)(0.1) = 0.18$; $H_{S2} = 2(0.1)(0.9) = 0.18$; $H_S = 0.18$

$F_{ST} = (0.5 - 0.18)/0.5 = 0.64$ — strong differentiation.

In practice, FST is computed genome-wide from SNP array or sequencing data; outlier loci with unusually high FST are candidates for local adaptation (selected differently in the two populations).

## Nucleotide Diversity

**Nucleotide diversity $\pi$** measures the average pairwise sequence difference per nucleotide within a population:

$$\pi = \frac{\sum_{i<j} d_{ij}}{\binom{n}{2}}$$

where $d_{ij}$ is the number of differences per site between sequences $i$ and $j$ and $n$ is the sample size. For neutral sites at mutation-drift equilibrium in a diploid population: $\pi = 4N_e \mu$ (where $\mu$ is the per-site per-generation mutation rate).

Human $\pi \approx 0.001$ (1 difference per 1000 bp) → $N_e \approx 10,000$ (using $\mu = 10^{-8}$)
*Drosophila melanogaster* $\pi \approx 0.01$ → $N_e \approx 10^6$

## Tajima's D: Detecting Departures from Neutral Evolution

**Tajima's D** (1989) detects selection or demographic change by comparing two estimators of $\theta = 4N_e\mu$:

- $\hat{\theta}_\pi = \pi$ (average pairwise diversity)
- $\hat{\theta}_W$ = Watterson's estimator, derived from the number of segregating sites $S$: $\hat{\theta}_W = S / a_1$, where $a_1 = \sum_{i=1}^{n-1} 1/i$ (harmonic number)

$$D = \frac{\hat{\theta}_\pi - \hat{\theta}_W}{\text{Var}(\hat{\theta}_\pi - \hat{\theta}_W)^{1/2}}$$

Interpretation:
- $D \approx 0$: consistent with neutral evolution
- $D < 0$: excess rare variants (low-frequency) relative to intermediate-frequency variants. Caused by: positive (directional) selection (selective sweep generates many new rare variants), population expansion after a bottleneck (excess rare mutations from newly expanded lineages)
- $D > 0$: excess intermediate-frequency variants. Caused by: balancing selection (maintains alleles at intermediate frequency), population bottleneck/contraction (drift removes rare variants, leaving intermediate-frequency alleles)

## Effective Population Size and Demographic History

**Effective population size $N_e$** is the size of an idealized Wright-Fisher population that would experience the same amount of genetic drift as the actual population. It is almost always smaller than census size due to:
- **Variance in reproductive success**: individuals with many offspring reduce effective size
- **Bottlenecks**: past population crashes reduce $N_e$ averaged over time ($N_e$ for a series of populations is the harmonic mean: $1/N_e = (1/t)\sum 1/N_t$)
- **Sex ratio imbalance**: $N_e = 4N_m N_f/(N_m + N_f)$
- **Population structure**: subpopulation structure can increase or decrease effective size

**Coalescent theory** (Kingman, 1982) traces the ancestry of sampled sequences backward in time. In a population of size $N_e$, any two lineages coalesce (find a common ancestor) with probability $1/(2N_e)$ per generation. The expected time to coalescence of two lineages is $2N_e$ generations (diploid).

Coalescent-based methods (PSMC, SMC++, MSMC) infer $N_e$ over time from single-genome sequences or population genomic data. Human demographic history inferred by PSMC shows:
- Expansion out of Africa ~50,000 years ago
- Bottleneck during out-of-Africa dispersal
- Rapid population growth in the past 10,000 years (agriculture)

## Selective Sweeps and Linkage Disequilibrium

When a beneficial mutation fixes, it carries along nearby neutral variants on the same chromosome — a **selective sweep**. The resulting pattern in sequence data:
- Reduced diversity (π) near the selected site — a "valley" of diversity
- Extended haplotype homozygosity (EHH): long haplotype blocks with high frequency (measured by iHS and XP-EHH statistics)
- High FST at the swept locus between populations where the sweep occurred and those where it didn't

**Linkage disequilibrium (LD)** $r^2$ between two SNPs: the correlation of allele frequencies. $r^2 = 1$ for perfect LD (alleles always co-inherited); $r^2 = 0$ for independence (equilibrium). LD decays with distance and recombination rate. GWAS takes advantage of LD: a causal variant need not be typed directly if it is in strong LD with a typed SNP.

## Why This Matters for Computational Biology

Population genetics is the interpretive framework for all population genomic data. GWAS, admixture analysis, selection scans, demographic inference — all are population genetics. In directed evolution, understanding the evolutionary dynamics of your selection experiment (effective population size of the library, selection stringency) determines expected outcomes. In synthetic biology, regulatory sequences and genetic circuits are subject to evolutionary pressure when expressed in cells; population genetics predicts how rapidly neutral or deleterious variants will accumulate and when purifying selection can maintain function. Metagenomic analyses of microbiome diversity — measuring π, FST, or detecting sweeps in microbial populations — use the same mathematical toolkit as human population genetics, applied at the species/strain level.
