# Genome-Wide Association Studies (GWAS)

For most of the twentieth century, the genetic basis of complex diseases like schizophrenia, type 2 diabetes, and coronary artery disease was almost completely opaque. Twin studies established that these diseases had substantial heritable components — identical twins were much more concordant than fraternal twins — but the specific variants responsible remained elusive. The problem was not a lack of interest. It was a lack of data, and a lack of the right hypothesis.

The hypothesis that changed everything was remarkably simple: if a variant is more common in people with a disease than in people without it, that variant may be causally related to the disease. You do not need to know the mechanism in advance. You do not need a candidate gene. You just need enough people, enough variants, and a statistical test. Genome-wide association studies operationalize this idea at the scale of millions of variants and hundreds of thousands of individuals. The results — tens of thousands of robust variant-trait associations across hundreds of phenotypes — have transformed cardiology, oncology, psychiatry, and pharmacogenomics.

A **GWAS** tests for statistical association between genetic variants distributed across the entire genome and a phenotype of interest. By simultaneously testing millions of SNPs in thousands to millions of individuals, GWAS has identified tens of thousands of robust variant-trait associations, revolutionizing our understanding of the genetic architecture of complex diseases and traits.

## The Core Statistical Test

For each SNP, test the null hypothesis that allele frequency does not differ between cases and controls (binary trait) or is not correlated with the trait value (quantitative trait).

**Binary traits** (case-control): logistic regression

$$\log\left(\frac{P(\text{case})}{1-P(\text{case})}\right) = \beta_0 + \beta_1 G + \beta_2 C_1 + \ldots + \beta_k C_k$$

where $G$ is the genotype (0/1/2 copies of the effect allele), and $C_1, \ldots, C_k$ are covariates (age, sex, principal components for population stratification).

**Quantitative traits** (e.g., height, BMI): linear regression

$$Y = \beta_0 + \beta_1 G + \beta_2 C_1 + \ldots + \beta_k C_k + \epsilon$$

The test statistic: $\beta_1 / \text{SE}(\beta_1)$, converted to a $p$-value with 1 degree of freedom.

The regression coefficient $\beta_1$ is the effect size — roughly, how much does having one extra copy of the effect allele shift the phenotype? For most common disease-associated variants, this is quite small. The FTO variant most strongly associated with BMI shifts body weight by about 300 grams per allele — detectable in a genome-wide study of hundreds of thousands of people, but far below any clinical threshold of concern for an individual carrier. This is the nature of polygenic architecture: many variants, each with modest effects, together accounting for substantial heritability.

## Multiple Testing Correction

A typical GWAS tests ~7 million SNPs (after imputation from a reference panel like 1000 Genomes). The **Bonferroni threshold** for genome-wide significance corrects for all independent tests:

$$\alpha_\text{genome-wide} = \frac{0.05}{1{,}000{,}000} = 5 \times 10^{-8}$$

The choice of $10^6$ rather than $7 \times 10^6$ reflects that SNPs in LD are not independent tests — the number of effectively independent loci across the human genome is approximately $10^6$.

**Genome-wide significant**: $p < 5 \times 10^{-8}$  
**Suggestive**: $5 \times 10^{-8} < p < 10^{-5}$

This threshold deserves to be appreciated on its own terms. You might expect that after correcting for a million tests, finding anything would be nearly impossible. And for small studies, this is true — early underpowered GWAS produced hundreds of false positives that failed to replicate, a cautionary period that prompted the field to adopt strict replication requirements. But with sample sizes of 100,000–1,000,000 individuals, variants with even modest effect sizes generate test statistics that sail past the $5 \times 10^{-8}$ threshold with room to spare. The field learned, painfully, that statistical significance is not the limiting factor — sample size and replication are.

## The Manhattan Plot

A **Manhattan plot** displays $-\log_{10}(p)$ on the y-axis against genomic position on the x-axis. The name comes from the skyline-like appearance of peaks:

- Horizontal red line: genome-wide significance threshold ($-\log_{10}(5 \times 10^{-8}) \approx 7.3$)
- Peaks above the threshold: associated loci
- The vertical spread of dots reflects LD structure: multiple SNPs in a locus peak together

```python
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np

def manhattan_plot(gwas_results):
    """gwas_results: DataFrame with CHR, BP, P columns"""
    fig, ax = plt.subplots(figsize=(20, 6))
    
    chromosomes = sorted(gwas_results['CHR'].unique())
    colors = ['#1f77b4', '#ff7f0e']  # alternate colors
    
    running_pos = 0
    xticks = []
    
    for i, chrom in enumerate(chromosomes):
        chrom_data = gwas_results[gwas_results['CHR'] == chrom].copy()
        chrom_data['abs_pos'] = chrom_data['BP'] + running_pos
        
        ax.scatter(chrom_data['abs_pos'], -np.log10(chrom_data['P']),
                  c=colors[i % 2], s=1, alpha=0.8)
        
        xticks.append(running_pos + chrom_data['BP'].max() / 2)
        running_pos += chrom_data['BP'].max()
    
    ax.axhline(y=-np.log10(5e-8), color='red', linestyle='--', label='p = 5×10⁻⁸')
    ax.set_xticks(xticks)
    ax.set_xticklabels(chromosomes, fontsize=8)
    ax.set_xlabel('Chromosome')
    ax.set_ylabel('-log₁₀(p-value)')
    ax.set_title('GWAS Manhattan Plot')
    plt.tight_layout()
    return fig
```

Reading a Manhattan plot is a skill worth developing. The peaks above the red line are your significant hits. The width of each peak reflects the extent of LD at that locus — a wide peak means many SNPs are correlated with the causal variant, which is useful (you are robust to exactly which SNP you tested) but problematic (you cannot easily identify the causal variant by statistics alone). A narrow peak means less LD, tighter localization, and potentially easier fine-mapping. The relative height of peaks is less informative than it might seem — it reflects both effect size and sample size.

## Genomic Inflation Factor (λ)

The **genomic inflation factor** $\lambda$ (lambda) detects systematic bias in GWAS:

$$\lambda = \frac{\text{median}(\chi^2)}{\text{expected median of } \chi^2_1 \text{ under null}} = \frac{\text{median}(\chi^2)}{0.456}$$

- $\lambda = 1.0$: no inflation; well-calibrated test statistics
- $\lambda > 1.05$: systematic inflation; likely due to population stratification or cryptic relatedness
- $\lambda < 1.0$: deflation; possibly over-correction or very small sample size

**QQ plot**: plots observed $-\log_{10}(p)$ vs. expected $-\log_{10}(p)$ under the null. Points should fall on the diagonal, deviating only at the tail where true associations appear.

Population stratification (e.g., mixing European and African ancestry individuals without correction) inflates all test statistics — SNPs with different allele frequencies between ancestral groups spuriously associate with a trait that differs by ancestry (e.g., a disease more common in one ancestry).

**Correction**: include top principal components (typically 5–20) as covariates. Advanced methods: BOLT-LMM, REGENIE (linear mixed models that simultaneously model all SNPs as random effects).

The QQ plot is one of those diagnostic tools that looks simple but tells you something profound about whether your statistical model is appropriate. Inflated test statistics across the entire distribution (lifting the whole curve upward) suggest a systematic confounder affecting all tests equally — the hallmark of population stratification. True genetic signal only inflates the tail of the distribution. If your QQ plot shows the entire curve shifted upward, you need to add more principal components, or restructure your study design, before any individual association can be trusted.

## LD Clumping and Fine-Mapping

A GWAS peak spanning 50 kb may contain 100+ SNPs with $p < 5 \times 10^{-8}$ due to LD. **LD clumping** identifies the independent lead SNP per locus:

```bash
# PLINK LD clumping
plink --bfile genotypes \
      --clump gwas_results.txt \
      --clump-p1 5e-8 \
      --clump-r2 0.1 \
      --clump-kb 3000 \
      --out clumped_results
```

**Fine-mapping** identifies the most likely causal variant(s) within an associated locus using:
- Statistical fine-mapping (FINEMAP, SuSiE): Bayesian credible sets of variants
- Functional annotation: variants in regulatory elements, coding regions
- Colocalization with eQTLs (GTEx): does the GWAS signal colocalize with an expression quantitative trait locus?

The insight behind fine-mapping is that statistical association does not equal causality. A GWAS hit might point to any of 100 variants in LD, most of which are not causal — they simply co-inherited with the causal variant because recombination has not yet separated them in the human population. Fine-mapping methods like SuSiE compute a 95% credible set: the smallest set of variants that collectively contain the causal variant with 95% probability. For a well-powered study at a locus with low LD, this credible set might contain a single variant. For many GWAS loci, it contains 10–50, and functional evidence is required to prioritize further.

## Polygenic Risk Scores

GWAS summary statistics can be aggregated into **polygenic risk scores (PRS)**:

$$PRS_i = \sum_{j=1}^{M} \hat\beta_j \cdot G_{ij}$$

where $\hat\beta_j$ is the effect size from GWAS and $G_{ij}$ is the genotype dosage. PRS are used for risk stratification in medicine (coronary artery disease, type 2 diabetes) and phenotype prediction in agriculture.

## SNP Heritability

The fraction of phenotypic variance explained by all common SNPs:

$$h^2_{SNP} = \frac{\text{Var(genetic component from common SNPs)}}{\text{Var(phenotype)}}$$

Estimated by LD score regression (LDSC): regresses $\chi^2$ statistics on LD scores ($\ell_j = \sum_k r^2_{jk}$). The slope of this regression is proportional to $h^2_{SNP}$.

Missing heritability: for many traits, $h^2_{SNP}$ is much less than twin-study heritability — partly explained by rare variants, gene-environment interactions, and non-additive effects.

## Why This Matters

GWAS is the dominant approach for identifying the genetic basis of complex human traits. It has yielded over 70,000 variant-trait associations across hundreds of phenotypes, transforming cardiology, oncology, psychiatry, and pharmacogenomics. Understanding GWAS methodology — the multiple testing problem, inflation factor, LD structure, and fine-mapping — is essential for correctly interpreting published GWAS results, conducting new GWAS, and applying GWAS findings to polygenic risk prediction and drug target identification.

The most underappreciated consequence of GWAS is its role in drug target validation. Variants near a gene that increase disease risk provide human genetic evidence that the gene is causally involved in disease — a form of evidence that has proven more predictive of clinical success than animal model data alone. Drugs targeting genes with human genetic support are roughly twice as likely to succeed in clinical trials as those without it. The GWAS literature is, among other things, a systematic map of druggable targets, written in the language of population genetics.
