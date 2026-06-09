# Key Statistical Papers for Computational Biology

Imagine you run a genomic screen and test 20,000 genes simultaneously for differential expression. At a significance threshold of p < 0.05, you expect one thousand false positives by chance alone — regardless of whether any gene is truly differentially expressed. If you report 1,200 hits without correction, you have no idea whether the excess 200 represent real biology or statistical noise. This is not a hypothetical: it was a routine practice in the early days of microarray analysis, and a large fraction of the genome-wide associations from that era have never been independently confirmed.

Statistics is the language in which biological data speaks, but it is a language that can be used carelessly or precisely. The papers in this section define the statistical foundations of modern computational biology: multiple testing correction, RNA-seq differential expression analysis, and dimensionality reduction for high-dimensional biological data. These are not papers that most biologists read cover-to-cover — they are papers that underpin tools used every day, and understanding them transforms tool use from black-box procedure to informed analysis.

---

## 1. Benjamini & Hochberg (1995) — False Discovery Rate

**Full citation:** Benjamini, Y., & Hochberg, Y. (1995). Controlling the false discovery rate: a practical and powerful approach to multiple testing. *Journal of the Royal Statistical Society: Series B*, 57(1), 289–300.

**What it contributes:** Yoav Benjamini and Yosef Hochberg solved a problem that had plagued large-scale biological experiments: how to do multiple testing correction in a way that is strict enough to control false positives, but not so strict that every real signal is buried. The **False Discovery Rate (FDR)** procedure provides a method for controlling the expected proportion of falsely rejected hypotheses among all rejected hypotheses when testing many hypotheses simultaneously. Before this paper, multiple testing correction used the Family-Wise Error Rate (FWER) — controlling the probability of making any false positive at all. FWER control (e.g., Bonferroni correction) is very conservative and loses enormous statistical power when testing thousands of hypotheses simultaneously, exactly the situation genomics experiments face.

**The FDR framework:**

Define the FDR as the expected proportion of false positives among all positives (rejected hypotheses):

FDR = E(V/R | R > 0) × P(R > 0)

where V is the number of false positives and R is the total number of rejections. The Benjamini-Hochberg (BH) procedure controls FDR at level q:

1. Sort the m p-values: p(1) ≤ p(2) ≤ ... ≤ p(m)
2. Find the largest k such that p(k) ≤ k × q / m
3. Reject all null hypotheses with p(i) for i = 1, ..., k

**Why this matters for genomics:** A differential expression analysis might test 20,000 genes simultaneously. At α = 0.05 with Bonferroni correction, the threshold becomes 0.05/20000 = 2.5 × 10^(-6) — extremely conservative, missing many real effects. BH at FDR = 0.05 is far less conservative: it controls the fraction of false positives among the called positives at 5%, which is the appropriate framework for discovery-oriented genomics.

**How to read it:** Read the introduction and main theorem. The proof is accessible with a statistics background. Focus on Table 1 (simulation results showing power advantages over Bonferroni). After reading, implement the BH procedure manually in R to verify you understand it:

```r
# Manual BH
p_vals <- c(0.001, 0.008, 0.039, 0.041, 0.042, 0.08, 0.4, 0.5)
n <- length(p_vals)
sorted_p <- sort(p_vals)
q <- 0.05
threshold_index <- max(which(sorted_p <= (1:n) * q / n))
threshold <- sorted_p[threshold_index]
# Or simply: p.adjust(p_vals, method = "BH")
```

**q-values vs. BH-adjusted p-values:** Storey & Tibshirani (2003, PNAS) introduced the q-value, which incorporates an estimate of the proportion of true null hypotheses (π₀) to provide an empirical Bayes version of FDR correction that is even more powerful when π₀ < 1 (which it almost always is in genomics). The `qvalue` R package implements this.

**Why it remains important:** BH FDR correction is mandatory in genomics, transcriptomics, proteomics, and metabolomics analyses. Every differential expression result, every GWAS, every proteomics screen uses it. Understanding why it is more appropriate than Bonferroni in these contexts — and knowing when even BH is too permissive (clinical diagnostic settings) — is essential statistical literacy.

---

## 2. Love, Huber & Anders (2014) — DESeq2 (Statistical Details)

**Full citation:** Love, M. I., Huber, W., & Anders, S. (2014). Moderated estimation of fold change and dispersion for RNA-seq data with DESeq2. *Genome Biology*, 15, 550.

(Also covered in the Genomics Papers section; this entry focuses on the statistical methodology.)

**The statistical model:**

DESeq2 models RNA-seq counts K(ij) (read counts for gene i in sample j) as:

K(ij) ~ NB(μ(ij), α(i))

where μ(ij) = s(j) × q(ij) is the mean (s(j) is a size factor for sample j, q(ij) is the normalized mean expression), and α(i) is the dispersion parameter (α = 0 reduces to Poisson; α > 0 adds overdispersion).

**Empirical Bayes shrinkage of dispersion:**
Raw per-gene dispersion estimates are noisy, especially for low-count genes. DESeq2 fits a mean-dispersion trend across all genes and shrinks individual gene dispersions toward this trend using a log-normal prior. The resulting "shrunken" dispersions reduce false positives among high-count genes (where the raw estimate is reliable) and reduce false negatives among low-count genes.

**LFC shrinkage (apeglm):**
The `lfcShrink` function applies a Cauchy prior on log-fold changes (using the apeglm method, Zhu et al. 2019) to reduce extreme LFC estimates for low-count genes. This is essential for volcano plots and ranked gene lists — without shrinkage, the most extreme LFCs are often from low-count genes that are statistically unreliable.

**Practical points:**
- **Size factor estimation:** DESeq2 uses median-of-ratios normalization (not total count normalization) to account for library size differences. This is robust to a few highly expressed genes dominating the count total.
- **Multiple testing:** DESeq2 applies Independent Filtering (removing genes below a mean count threshold) before BH correction, to increase power without inflating false positives.
- **Outlier detection:** Cook's distance is used to detect samples that are outliers for a given gene; outlier counts are removed and the gene is not tested.

---

## 3. van der Maaten & Hinton (2008) — t-SNE

**Full citation:** van der Maaten, L., & Hinton, G. (2008). Visualizing data using t-SNE. *Journal of Machine Learning Research*, 9, 2579–2605.

**What it contributes:** A single-cell RNA-seq experiment measures the expression of 20,000 genes in thousands of individual cells. You cannot visualize a 20,000-dimensional dataset directly. Dimensionality reduction compresses that information down to two or three dimensions for visualization, and the challenge is to do so in a way that preserves the biologically meaningful structure. t-SNE (t-Distributed Stochastic Neighbor Embedding) is the **nonlinear dimensionality reduction method** that dominated single-cell biology for a decade: it maps high-dimensional data to two or three dimensions while preserving **local structure** — points that are similar in high-dimensional space are placed near each other in the 2D embedding.

**How t-SNE works:**
1. In high-dimensional space, compute a pairwise similarity matrix using a Gaussian kernel (nearby points have high similarity; distant points approach zero).
2. In the 2D embedding, compute similarities using a t-distribution (heavy-tailed; allows distant points to be placed farther apart without penalty).
3. Minimize the Kullback-Leibler divergence between the high-dimensional and low-dimensional similarity distributions using gradient descent.

**What t-SNE preserves and what it does not:**

**t-SNE preserves:**
- Local neighborhood structure (which cells cluster together)
- Relative similarity within clusters

**t-SNE does not preserve:**
- Global structure (the distance between clusters is not meaningful)
- Distances between clusters (two clusters that appear far apart may not actually be more different than two clusters that appear close)
- Density (cluster size in the t-SNE plot does not reflect the number of cells)

**Common misinterpretations:**
- "The two clusters are separated, so they are very different." NOT necessarily — t-SNE global structure is not preserved.
- "This cluster is larger, so it is more abundant." NOT necessarily — cluster size depends on perplexity and other parameters.
- "The t-SNE shows three distinct groups, so there are three cell types." NOT necessarily — the number of apparent clusters depends heavily on perplexity and random seed.

**Parameters:**
- **Perplexity** (typically 5–50): controls the balance between local and global structure. Low perplexity emphasizes local clusters; high perplexity reveals more global structure. Run multiple perplexity values and compare.
- **Random seed:** t-SNE results are not deterministic. Run with multiple seeds and check that clustering patterns are consistent.

**UMAP as an alternative:** McInnes et al. (2018, arXiv:1802.03426) introduced UMAP (Uniform Manifold Approximation and Projection), which is faster than t-SNE and better preserves global structure. UMAP is now preferred over t-SNE in most single-cell applications. The same caveats about misinterpretation of cluster distances apply to UMAP.

**Implementation:**
- Python: `sklearn.manifold.TSNE` (scikit-learn), `umap-learn` package
- R: `Rtsne` package, `uwot` package for UMAP
- Seurat (single-cell RNA-seq analysis) and Scanpy (Python) provide integrated t-SNE and UMAP workflows

**How to read it:** The paper is mathematically dense (KL divergence, gradient derivation). Read the introduction and Figure 1–3 for the conceptual contribution. The Barnes-Hut approximation (van der Maaten 2014) makes t-SNE computationally tractable for large datasets.

---

## Other Essential Statistical References

**Cohen (1988) — Power analysis:** *Statistical Power Analysis for the Behavioral Sciences* (textbook) defines effect sizes (Cohen's d, Cohen's h, f²) and provides sample size tables for common tests. G*Power (software) implements these calculations.

**Gelman & Hill (2006)** — *Data Analysis Using Regression and Multilevel/Hierarchical Models* (textbook): the definitive reference for mixed-effects models, which are essential for experiments with repeated measures, nested data, or batch effects.

**Efron & Hastie (2016)** — *Computer Age Statistical Inference* (textbook): covers bootstrap, EM algorithm, empirical Bayes, and other modern statistical methods used in computational biology.

---

## Takeaway

The key statistical papers for computational biology address three of the most common and consequential analytical failures in the literature: multiple testing without correction (BH FDR), inappropriate modeling of count data (DESeq2's negative binomial), and misinterpretation of dimensionality reduction visualizations (t-SNE/UMAP). Understanding these papers transforms statistical practice from tool application to principled analysis. The published literature is full of false discoveries from genomics analyses that ignored multiple testing, spurious clusters from t-SNE plots taken too literally, and differential expression results distorted by improper normalization. Reading these three papers puts you in a position to avoid those failures in your own work and to recognize them in others'.
