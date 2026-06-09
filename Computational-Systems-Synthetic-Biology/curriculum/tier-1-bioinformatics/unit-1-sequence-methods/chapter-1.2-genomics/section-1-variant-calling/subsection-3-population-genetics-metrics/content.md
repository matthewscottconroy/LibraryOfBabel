# Population Genetics Metrics

In 1908, Godfrey Hardy and Wilhelm Weinberg independently worked out something deceptively simple: in a randomly mating population, allele and genotype frequencies reach a stable equilibrium in a single generation and stay there indefinitely — provided nothing disturbs them. Their result was not meant to describe reality. It was meant to define a baseline against which deviations could be measured. Whenever a population departs from Hardy-Weinberg equilibrium, something biologically interesting is happening.

This is the spirit that animates all of population genetics: rather than studying populations directly, we study the ways they deviate from idealized expectations, and we ask what forces — selection, drift, migration, mutation — could have produced those deviations. Applied to genomics, these metrics are not merely descriptive. They reveal which variants are under selection, which populations share ancestry, and which apparent associations in a GWAS study are artifacts of unaccounted population structure.

Population genetics provides the statistical framework for understanding how genetic variants are distributed within and between populations, how selection acts on variants, and how evolutionary forces (drift, migration, mutation) shape genomic diversity. These metrics underlie GWAS, demographic inference, and population stratification analysis.

## Allele Frequency and Hardy-Weinberg Equilibrium

For a biallelic locus with alleles $A$ (reference) and $a$ (alternate):
- **Allele frequency** of $a$: $q = \frac{\text{count of } a \text{ alleles}}{2N}$ (for diploid population of size $N$)
- **Minor allele frequency (MAF)**: frequency of the less common allele

**Hardy-Weinberg Equilibrium (HWE)**: under random mating, no selection, no mutation, and no genetic drift, genotype frequencies follow:

$$P(AA) = (1-q)^2, \quad P(Aa) = 2q(1-q), \quad P(aa) = q^2$$

**HWE test**: deviations from HWE can indicate:
- Genotyping errors (common cause of excess homozygosity)
- Population stratification (mixed populations appear to deviate from HWE)
- Strong selection
- Inbreeding

```python
from scipy import stats

def hwe_test(n_aa, n_ab, n_bb):
    """Test for Hardy-Weinberg equilibrium"""
    n = n_aa + n_ab + n_bb
    q = (2 * n_bb + n_ab) / (2 * n)  # alt allele frequency
    p = 1 - q
    expected_aa = p**2 * n
    expected_ab = 2 * p * q * n
    expected_bb = q**2 * n
    chi2, pval = stats.chisquare([n_aa, n_ab, n_bb],
                                  [expected_aa, expected_ab, expected_bb])
    return chi2, pval
```

In practice, the most important use of the HWE test is as a quality control filter: a locus with a severe HWE deviation in a supposedly homogeneous population almost certainly has a genotyping artifact, not a biological signal. When you see thousands of variants failing HWE in your GWAS, you have a sequencing or calling problem — not evidence for pervasive selection at millions of loci.

## Linkage Disequilibrium

**Linkage disequilibrium (LD)** is the non-random association of alleles at different loci. Loci in LD are inherited together more often than expected by chance.

For two biallelic loci with alleles $(A, a)$ and $(B, b)$:

**D statistic**:
$$D = P_{AB} - P_A \cdot P_B$$

where $P_{AB}$ is the observed haplotype frequency and $P_A \cdot P_B$ is the expected frequency under independence.

**D' (normalized)**:
$$D' = \frac{D}{D_\text{max}}$$

where $D_\text{max}$ depends on allele frequencies. $|D'| = 1$ indicates no evidence of recombination between the loci.

**r² statistic**:
$$r^2 = \frac{D^2}{p_A(1-p_A)p_B(1-p_B)}$$

$r^2 = 1$: perfect LD (the two loci carry identical information). $r^2 > 0.8$: strong LD; variants are nearly interchangeable for association analysis. $r^2 < 0.2$: weak LD; variants are largely independent.

**LD decay**: LD decreases with physical distance as recombination accumulates between loci over generations. In humans, LD typically decays to $r^2 < 0.1$ within 100–500 kb. African populations have shorter LD blocks (more ancient recombination) than European or Asian populations.

The practical consequence of LD for GWAS is profound. Because nearby variants are correlated, you do not need to genotype every one of the tens of millions of SNPs in the human genome — you only need to genotype a representative "tag SNP" for each LD block. The 1000 Genomes Project reference panel enables imputation of ungenotyped variants from neighboring tag SNPs, effectively expanding a 500,000-SNP array into a 7 million-SNP analysis. The structure of LD across the genome is what makes GWAS at population scale computationally tractable.

## FST: Population Differentiation

**FST (fixation index)** measures the degree of genetic differentiation between populations:

$$F_{ST} = \frac{H_T - H_S}{H_T}$$

where:
- $H_T$ = expected heterozygosity in the total combined population
- $H_S$ = average expected heterozygosity within subpopulations

**Interpretation**:
- $F_{ST} = 0$: no differentiation; allele frequencies identical between populations
- $F_{ST} = 1$: complete differentiation; different fixed alleles in each population
- Human continental population FST: ~0.10–0.15 (low — humans are a young, genetically similar species)

Genome-wide FST scans identify loci under local adaptation: regions with unusually high FST indicate selective sweeps specific to one population (e.g., lactase persistence variant LCT with FST ~0.85 between European and African populations).

The lactase example is one of the best-documented cases of recent human evolution. The ability to digest lactose into adulthood is ancestrally absent — all mammals lose lactase expression after weaning. But in populations with a history of dairy farming (northern European, some East African pastoral communities), variants near the LCT gene reached high frequency, driving an FST value that stands out dramatically from the genome-wide baseline. This signal was detected computationally before the functional variant was identified — a preview of how FST scans can flag biologically important loci before the mechanism is understood.

## Principal Component Analysis (PCA) for Population Structure

PCA on genotype data ($N$ individuals × $M$ SNPs matrix) reveals population structure without assumptions about the number of populations:

```python
import numpy as np
from sklearn.decomposition import PCA
import pandas as pd

# genotypes: N x M matrix of 0, 1, 2 (dosage)
# Center and scale
X = genotypes - genotypes.mean(axis=0)
X = X / X.std(axis=0)

# PCA
pca = PCA(n_components=10)
pcs = pca.fit_transform(X)

# PC1 and PC2 often separate continental populations
# PC3-10 may reveal finer structure
```

PCA is used to:
1. Identify population substructure that confounds GWAS
2. Include as covariates in association tests to control for stratification
3. Visualize population relationships

It turns out that the first two principal components of human SNP data famously recapitulate the geography of Europe with striking fidelity — PC1 separates north from south, PC2 separates east from west. This is not because genetics determines geography; it reflects the pattern of human migrations, bottlenecks, and gene flow that shaped European populations over tens of thousands of years. The same analysis applied globally separates continental populations, though with the sobering reminder that those separations explain only ~10–15% of total human genetic variation. Most variation is within populations, not between them.

## Nucleotide Diversity (π)

**π (nucleotide diversity)** is the average pairwise sequence difference between randomly chosen sequences in a population:

$$\pi = \frac{1}{\binom{n}{2}} \sum_{i<j} k_{ij} / L$$

where $k_{ij}$ is the number of differences between sequences $i$ and $j$ and $L$ is the sequence length.

For humans, $\pi \approx 0.001$ (1 in 1000 bases differs between two randomly chosen chromosomes).

**Tajima's D** compares π to the number of segregating sites $S$:

$$D = \frac{\hat\pi - \hat\theta_W}{\sqrt{\text{Var}(\hat\pi - \hat\theta_W)}}$$

where $\theta_W = S / a_1$ (Watterson's estimator, $a_1 = \sum_{i=1}^{n-1} 1/i$).

- $D < 0$: excess rare variants → recent positive selection or population expansion
- $D > 0$: excess intermediate-frequency variants → balancing selection or population bottleneck
- $D \approx 0$: consistent with neutral evolution

Tajima's D is elegant because it compares two estimates of the same underlying quantity (the population mutation rate θ) that are equally valid under neutral evolution but respond differently to selection. After a selective sweep, one adaptive variant rapidly rises to high frequency, dragging nearby variants along with it — but leaving a "star-like" genealogy with many rare, recent mutations. The genome looks like it has too many singletons relative to the number of segregating sites. Tajima's D captures this imbalance in a single number.

## Transition/Transversion Ratio (Ti/Tv)

The **Ti/Tv ratio** is used as a quality control metric for variant calling:
- Expected: ~2.0–2.1 for whole genome, ~3.0–3.3 for whole exome
- Too low (< 1.8 for WGS): excess false positive variants (transitions are more common biologically)
- Too high: possible filtering that preferentially removed transversions

```bash
bcftools stats variants.vcf.gz | grep "Ts/Tv"
```

## Why This Matters

Population genetics metrics are the foundation of GWAS, demographic history reconstruction, and evolutionary genomics. FST enables identification of locally adapted variants; LD structure determines which tag SNPs are informative in association studies; HWE testing identifies genotyping errors; Tajima's D reveals signatures of selection. Every analysis involving variants from multiple individuals — whether a GWAS, a population genomics study, or a clinical study with population-matched controls — requires understanding these metrics to correctly interpret results and detect confounders.

Zoom out, and the larger picture is this: the genome of any individual is not just a personal medical record. It is a historical document encoding the migrations, bottlenecks, selective pressures, and population dynamics that shaped that person's ancestors. Population genetics gives you the tools to read that history. And understanding that history — particularly population structure — is what separates a GWAS study with valid controls from one with systematic confounders that generate false discoveries you will later have to retract.
