# Differential Expression Analysis

In 2001, a landmark study of diffuse large B-cell lymphoma used gene expression data to split what oncologists had classified as a single cancer type into two molecularly distinct diseases — one with 5-year survival above 60%, the other below 30%. The key analytical step was identifying which genes were expressed at different levels between the two groups. This is differential expression analysis: the computational tool that has, more than perhaps any other, transformed our understanding of disease by asking a simple question — which genes are expressed at significantly different levels between two or more biological conditions?

The question sounds simple. The statistics are not. RNA-seq count data has properties — overdispersion, sparse counts for lowly expressed genes, and large numbers of simultaneous tests — that violate the assumptions of classical tests like the t-test. Understanding why these properties require specialized methods will protect you from the most common analytical mistakes.

## The Negative Binomial Model

You might expect that if a gene has a mean expression of 100 counts across replicates, the variance across replicates should be modest and predictable. The Poisson distribution, which describes random sampling events, says variance should equal the mean. But RNA-seq data is consistently more variable than this — a phenomenon called **overdispersion**.

RNA-seq counts are not normally distributed. Even for a gene with a true mean expression of 100 counts, the observed variance across biological replicates exceeds what the Poisson distribution (where variance = mean) would predict. This **overdispersion** arises from biological variability: different cells in the same condition do not express genes at exactly the same level.

The **negative binomial (NB) distribution** models this by adding a gene-specific dispersion parameter $\alpha$ (sometimes called the BCV, biological coefficient of variation):

$$\text{Var}(K_{gj}) = \mu_{gj} + \alpha_g \mu_{gj}^2$$

where $\mu_{gj}$ is the expected count for gene $g$ in sample $j$, and $\alpha_g$ captures the extra-Poisson variability. When $\alpha_g \to 0$, the NB collapses to the Poisson. For most genes, $\alpha_g$ is in the range 0.01–0.5. The key insight is that the overdispersion term grows as the square of the mean — high-expression genes can be highly variable in absolute counts while being reliably consistent as proportions.

## Dispersion Estimation and Shrinkage

Here is a practical problem: you typically have only 3–6 biological replicates. Estimating the dispersion for each of 20,000 genes independently from 3 data points produces wildly unreliable estimates. Some genes will appear to have near-zero variance (making them look confidently differentially expressed) and some will appear to have enormous variance (making real differences invisible) — simply by chance.

With only 3–6 biological replicates, the per-gene dispersion estimate is highly unreliable. **DESeq2** addresses this by sharing information across genes using empirical Bayes shrinkage: it fits a smooth mean-dispersion relationship across all genes (a trend line), then shrinks individual gene dispersions toward this trend in proportion to how much information is available for that gene.

Genes with low counts or few replicates are shrunk more strongly toward the trend; well-measured genes retain their data-driven estimate. This approach dramatically reduces false discovery rates compared to per-gene maximum likelihood dispersion estimation. It is a beautiful example of borrowing statistical strength from the ensemble — the genome-wide pattern of how variance scales with expression is more reliably estimated than any individual gene's variance.

## DESeq2 Wald Test vs. Likelihood Ratio Test

For two-condition comparisons, DESeq2 uses the **Wald test**: the estimated log2 fold change divided by its standard error follows an approximately standard normal distribution under the null hypothesis of no differential expression. This is fast and appropriate for pairwise comparisons.

For multi-factor designs or testing whether a gene responds to any level of a multi-level factor (e.g., time course with 5 time points), the **likelihood ratio test (LRT)** compares the full model to a reduced model without the factor of interest. The LRT statistic follows a $\chi^2$ distribution with degrees of freedom equal to the difference in model parameters.

The choice between these tests matters for time-course and dose-response experiments. If you are asking "does this gene respond to any dose of treatment X?" rather than "does this gene respond to the highest dose compared to control?", the LRT is the appropriate tool. Using pairwise Wald tests for every time point comparison and then taking the union inflates false discoveries.

## Log2 Fold Change Shrinkage with apeglm

Imagine a gene with very low expression: 1 count in control and 3 counts in treatment. The naive log2 fold change is log2(3/1) = 1.58 — a seemingly impressive upregulation. But these numbers have enormous sampling uncertainty; the true fold change might be anywhere from 0.5 to 10. Reporting this as a 3-fold upregulation would be misleading.

The naive log2 fold change estimate for a gene with very few counts is unreliable and can be extremely large (positive or negative) by chance. **apeglm** (adaptive t prior shrinkage estimator) applies a heavy-tailed prior to shrink large fold change estimates toward zero when evidence is weak, while leaving the fold changes of highly expressed, strongly differential genes largely unchanged.

```r
library(DESeq2)
dds <- DESeqDataSetFromMatrix(countData = counts,
                               colData = metadata,
                               design = ~ condition + batch)
dds <- DESeq(dds)

# Apply apeglm shrinkage
res <- lfcShrink(dds, coef = "condition_treated_vs_control",
                 type = "apeglm")
```

The shrunken fold changes are preferable for ranking genes, plotting, and functional enrichment analysis. An MA plot of shrunken vs. unshrunken fold changes is visually striking: genes at low expression that previously showed extreme fold changes compress toward zero, while genes at high expression retain their original estimates almost unchanged.

## Multiple Testing Correction: Benjamini-Hochberg FDR

Testing 20,000 genes simultaneously means that even with a per-test $\alpha = 0.05$, we expect $0.05 \times 20{,}000 = 1{,}000$ false positives under the null hypothesis. The **Benjamini-Hochberg (BH) procedure** controls the **false discovery rate (FDR)** — the expected fraction of rejected hypotheses that are truly null — rather than the family-wise error rate.

The procedure: sort p-values $p_{(1)} \le p_{(2)} \le \ldots \le p_{(m)}$ and reject all hypotheses up to the largest $k$ such that:

$$p_{(k)} \le \frac{k}{m} \cdot q$$

where $q$ is the desired FDR level (typically 0.05 or 0.10). DESeq2 outputs these adjusted p-values as `padj`. Genes with `padj < 0.05` are declared differentially expressed at a 5% FDR.

Notice what this means: at FDR = 0.05, if you report 200 genes as differentially expressed, approximately 10 of them are expected to be false positives. This is not a bug — it is a feature of working in a discovery-oriented scientific mode where downstream experiments can filter out false positives, and where the cost of missing true positives (false negatives from overly conservative correction) is also real.

## Visualization of DE Results

Numbers in a table are hard to interpret. Visualization transforms them into intuition.

**Volcano plot**: x-axis = log2 fold change, y-axis = $-\log_{10}(\text{padj})$. Significantly upregulated genes appear in the upper right; downregulated in the upper left. The plot immediately communicates the fold change vs. statistical significance trade-off. A volcano plot that shows many points with large fold changes but poor significance (horizontal cluster near the bottom) suggests the experiment was underpowered. One with narrow fold changes but high significance (vertical spike near the center) suggests a subtle but consistent transcriptional response.

**MA plot**: x-axis = mean expression (log scale), y-axis = log2 fold change. Used to check for systematic biases — the fold changes should be centered around zero for non-DE genes across all expression levels. After apeglm shrinkage, the MA plot shows fold changes compressing toward zero at low counts.

**Heatmap**: Hierarchical clustering of the top 50–100 DE genes (by padj) across all samples using `pheatmap` or `ComplexHeatmap`. Z-score normalization per gene (subtract mean, divide by SD) ensures that visual patterns reflect relative expression differences rather than absolute count magnitudes.

```python
# Pseudocode for a Scanpy-based DE analysis
import scanpy as sc
import pandas as pd

# After loading counts into AnnData object:
sc.tl.rank_genes_groups(adata, groupby='condition',
                         method='wilcoxon',
                         groups=['treated'],
                         reference='control')
de_results = sc.get.rank_genes_groups_df(adata, group='treated')
de_results[de_results['pvals_adj'] < 0.05].head(20)
```

## Why This Matters

Differential expression analysis is the workhorse of functional genomics, connecting molecular phenotypes to biological conditions in disease, development, and perturbation studies. It is what let researchers first distinguish cancer subtypes by their molecular signatures, identify the transcriptional programs activated by viral infection, and characterize the gene expression changes that accompany every stage of embryonic development. Mastering its statistical foundations — the negative binomial model, dispersion shrinkage, apeglm, and FDR control — prevents the common failure modes that produce irreproducible results: naive fold change thresholding, ignoring multiple testing, or misspecifying the model. These are not abstract statistical concerns; they are the difference between publishable science and a list of noise.
