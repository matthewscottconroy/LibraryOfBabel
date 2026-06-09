# Approximate Likelihood Tests and Concordance Factors

Here is a disturbing finding: many phylogenetic clades in published literature that show bootstrap support of 95% or higher are supported by only 30–40% of individual sites, or by only a minority of genes in a phylogenomic analysis. How is this possible? It turns out that bootstrap support measures consistency across resampled datasets, but it says nothing about how many individual pieces of evidence — sites, genes — actually support the relationship. A small fraction of highly concordant data can drive high bootstrap values while the majority of the data is indifferent or even contradictory.

Bootstrap resampling measures how consistently a clade appears across random subsamples of sites. But bootstrap support has limitations: it can be inflated when all genes support the same incorrect topology (systematic bias), and it conflates statistical and phylogenetic signal. **Approximate likelihood ratio tests** and **concordance factors** provide complementary measures of branch support that address different aspects of phylogenetic confidence.

## aLRT: Approximate Likelihood Ratio Test

The **aLRT** (Approximate Likelihood Ratio Test) was developed by Anisimova & Gascuel (2006) and tests whether the ML tree topology is significantly better than the best alternative arrangement of taxa around each internal branch.

**Logic**: For a given internal branch $b$ in the ML tree, the best competitor topology is the one obtained by performing the best NNI swap around $b$. The LRT statistic is:

$$\Lambda_b = 2(\ell_1 - \ell_2)$$

where $\ell_1$ = log-likelihood of the best ML tree and $\ell_2$ = log-likelihood of the best NNI alternative at branch $b$. Under the null hypothesis (the branch has zero length — the two alternative arrangements are equivalent), $\Lambda_b$ approximately follows a $\chi^2_1$ distribution for large datasets.

aLRT is extremely fast: it requires only the likelihood of the best ML tree and the best NNI competitors (3 trees per internal branch, rather than 1000 bootstrap replicates). However, it only tests one alternative topology per branch, potentially missing other good alternatives.

## SH-aLRT: The Conservative Variant

**SH-aLRT** (Shimodaira-Hasegawa aLRT) applies a more conservative test based on the SH test framework, accounting for multiple testing across all branches. It uses simulated bootstrap replicates from the site log-likelihood distribution (rather than the full alignment resampling in the standard bootstrap). SH-aLRT values ≥ 80% are considered support, analogous to BS ≥ 70%.

IQ-TREE2 computes both UFBoot and SH-aLRT in a single run:

```bash
iqtree2 -s alignment.fasta -m GTR+G4 -B 1000 -alrt 1000 --bnni
# Node labels in output tree: SH-aLRT_support/UFBoot_support
```

A node with SH-aLRT = 85/UFBoot = 95 means: aLRT supports the branch (85 ≥ 80) and UFBoot strongly supports it (95 ≥ 95). Both measures provide independent evidence. When both agree, confidence is higher. When they disagree — for example, high UFBoot but low SH-aLRT — it is worth investigating why.

## Site Concordance Factors (sCF)

Both bootstrap and aLRT measure whether there is sufficient signal in the full alignment to support a clade. They do not reveal *what fraction of the data* (individual sites) actually support the clade vs. the two possible alternative resolutions.

**Site Concordance Factor (sCF)** (Minh et al., 2020) addresses this:

For each internal branch $b$ in the tree, randomly sample a large number of parsimony-informative sites from the alignment. For each sampled site, determine which of the three possible resolutions of the four taxa flanking branch $b$ is supported:
- Concordant with the focal branch (supporting the ML topology at this branch)
- Alternative 1 (supports one NNI neighbor)
- Alternative 2 (supports the other NNI neighbor)

$$sCF_b = \frac{\text{sites concordant with branch } b}{\text{concordant + alternative 1 + alternative 2 sites}}$$

**Interpretation**: sCF = 100% → all informative sites support the branch. sCF = 33% → sites are equally split among three resolutions (random noise / no signal). A branch with bootstrap = 95% but sCF = 34% has high resampling support for the overall tree but very little per-site signal for this specific branch — indicating the support comes from a few influential sites, not broad genomic signal.

## Gene Concordance Factors (gCF)

For phylogenomic datasets with multiple gene alignments, **Gene Concordance Factor (gCF)** measures the fraction of gene trees that contain a given branch:

$$gCF_b = \frac{\text{gene trees containing branch } b}{\text{total gene trees}}$$

gCF and sCF together distinguish several informative scenarios:

| gCF | sCF | Interpretation |
|---|---|---|
| High | High | Robust clade: consistent across genes and sites |
| Low | Low | Poorly supported: ILS, model error, or short branch |
| High | Low | Genes support the clade but sites within genes are discordant |
| Low | High | Individual sites support but different genes tell different stories |

High bootstrap + low gCF is a warning sign of gene tree discordance due to **incomplete lineage sorting (ILS)** or HGT — the concatenated alignment shows the species tree, but individual gene trees frequently disagree, indicating that the true evolutionary history of each gene differs from the species tree.

It turns out that this pattern — high bootstrap, low gCF — is common in rapid radiations. The bird radiation at the K-Pg boundary, for example, produces a concatenation tree with high bootstrap support at many nodes, but gCF values as low as 30–40% at some internal nodes, revealing that the true evolutionary signal is far weaker than bootstrap implies. The high bootstrap comes from the sheer amount of data, not from broad phylogenetic concordance.

## Concordance Factors vs. Bootstrap: Complementarity

Bootstrap support is a measure of **data support** (how consistently the full alignment supports the clade). Concordance factors are measures of **genomic concordance** (what fraction of the data — sites or genes — actually support the clade). A complete picture of phylogenetic evidence requires both:

```bash
# Compute concordance factors in IQ-TREE2
# After inferring species tree and individual gene trees:
iqtree2 -t species_tree.treefile \
        --gcf gene_trees.treefile \
        -s alignment.fasta --scf 100
```

## Why This Matters

Concordance factors have revealed that many phylogenetic branches that appear strongly supported by bootstrap (>95%) are actually supported by only 30–40% of sites or genes — indicating that high bootstrap can be generated by a small fraction of highly concordant data rather than broad genomic consensus — a finding with major implications for interpreting the reliability of phylogenomic trees used to resolve evolutionary controversies. Reporting only bootstrap values without concordance factors can paint a falsely confident picture. For any phylogenomic analysis, computing both is rapidly becoming standard practice.
