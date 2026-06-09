# Multiple Testing and Thresholds

In 2005, John Ioannidis published a paper titled "Why Most Published Research Findings Are False." Its central argument was that when researchers test many hypotheses simultaneously, when they have small sample sizes, and when they do not properly account for multiple comparisons, the majority of "significant" results in the published literature are expected to be false positives. The paper described the general problem. In genomics, it manifests with particular severity: differential expression analysis simultaneously tests tens of thousands of hypotheses — one per gene — and the problem of multiple testing is therefore not a theoretical concern but a daily practical reality.

Failing to account for multiple testing produces enormous numbers of spurious discoveries. Using overly conservative correction methods can suppress real discoveries. Understanding the statistical framework is essential for correctly interpreting and reporting results.

## The Multiple Testing Problem

When performing a single statistical test at $\alpha = 0.05$, we accept a 5% probability of a false positive. When testing $m = 20{,}000$ genes, even if no gene is truly differentially expressed, we expect $0.05 \times 20{,}000 = 1{,}000$ false positives by chance alone. The unadjusted p-value is therefore meaningless as a criterion for gene-by-gene significance in genomics.

Let that sink in: if you ran a differential expression experiment on two groups of random samples — no biological difference whatsoever — and reported all genes with p < 0.05, you would report approximately 1,000 genes as differentially expressed. Every single one would be a false positive. The p-value, the standard criterion for significance in most of science, is not sufficient alone for genomics.

## Family-Wise Error Rate (FWER) vs. False Discovery Rate (FDR)

Two different philosophies for controlling errors in multiple testing lead to very different thresholds.

The **FWER** is the probability of making at least one false positive among all rejected hypotheses. Controlling FWER at 0.05 means we accept at most a 5% chance of any false positive across the entire experiment. The **Bonferroni correction** achieves this by dividing $\alpha$ by the number of tests:

$$p_{\text{threshold}} = \frac{\alpha}{m} = \frac{0.05}{20{,}000} = 2.5 \times 10^{-6}$$

This is extremely conservative for genomics: if 1,000 genes are truly differentially expressed, Bonferroni may identify only 200 of them (low power), because it is calibrated for the worst-case scenario of all tests being independent. Gene expression tests are positively correlated (co-expressed genes), making Bonferroni even more conservative than necessary. You would be very confident in the genes you call, but you would miss most of the real biology.

The **False Discovery Rate (FDR)** is a weaker but more useful criterion for discovery-oriented genomics: it controls the expected fraction of rejected hypotheses that are false positives. If we report 500 genes as DE at FDR = 0.05, we expect approximately 25 of them to be false positives. This is acceptable for a screening analysis where false positives can be filtered by follow-up experiments.

The philosophical shift matters. FWER asks: "Are any of my calls wrong?" FDR asks: "What fraction of my calls are wrong?" For genome-wide discovery studies, where you are identifying candidates for experimental follow-up rather than making binary clinical decisions, FDR is the appropriate framework.

## The Benjamini-Hochberg Procedure

The **Benjamini-Hochberg (BH)** procedure is the standard FDR control method for RNA-seq. Algorithm:

1. Rank the $m$ p-values from smallest to largest: $p_{(1)} \le p_{(2)} \le \ldots \le p_{(m)}$.
2. For each $i$, compute the BH-adjusted threshold: $p_{(i)}^* = \frac{i}{m} \cdot q$, where $q$ is the target FDR.
3. Find the largest $k$ such that $p_{(k)} \le \frac{k}{m} \cdot q$.
4. Reject all hypotheses $1, 2, \ldots, k$ (declare all genes with $p \le p_{(k)}$ as significant).

The BH procedure guarantees that $\mathbb{E}[V/R] \le q$, where $V$ = false positives, $R$ = total rejections, under independence or positive correlation among tests. DESeq2 and edgeR both report BH-adjusted p-values as the `padj` or `FDR` column.

**Q-values** (Storey & Tibshirani, 2003) extend BH by estimating $\pi_0$ — the proportion of truly null hypotheses — from the data and using this estimate to improve power when $\pi_0 < 1$ (i.e., when many genes are truly DE).

## Choosing the FDR Threshold

Standard thresholds:
- **FDR < 0.05**: The conventional threshold; appropriate for most gene expression studies.
- **FDR < 0.10**: More permissive; appropriate when the experiment is discovery-oriented or when the expected effect sizes are small.
- **FDR < 0.01**: Conservative; appropriate for high-confidence gene lists used in targeted follow-up.

There is no universal correct answer — the appropriate threshold depends on the biological question, the cost of false positives (are you following up all genes experimentally?) vs. false negatives (are you trying to be comprehensive?), and downstream uses of the gene list.

It is also worth applying both a fold change threshold and an adjusted p-value threshold. Statistical significance alone — a very small padj — is obtainable for biologically trivial effects if your sample size is large enough. A gene expressed at 10.001 vs. 10.000 TPM can have a highly significant p-value in a well-powered experiment. Adding |log2 fold change| > 1 (a 2-fold change) as a filter ensures that statistically significant results are also biologically meaningful in magnitude.

## P-value Histogram Diagnostics

Before interpreting results, always plot the **p-value histogram**. The shape of this histogram is diagnostic:

**Expected histogram (good data)**: A mixture of two distributions: a flat uniform distribution (corresponding to null genes) and an enrichment near 0 (corresponding to truly DE genes). The histogram should show a spike near 0 and then a roughly flat tail.

**Anti-conservative histogram**: The entire distribution is shifted toward small p-values (the histogram rises toward 0 across its full range). This suggests inflation — possible causes include unmodeled batch effects, wrong reference samples, or liberal modeling assumptions.

**Conservative histogram**: The distribution is enriched near 1.0. This can happen when the statistical model is overdispersed relative to the data (overestimates variance), or when few truly DE genes exist.

**Bimodal at 0 and 1**: Suggests a mixture of highly DE and null genes — typical of a strongly responding dataset. This is a good sign.

The p-value histogram is the single most informative diagnostic plot for catching modeling problems. An anti-conservative p-value distribution — where more small p-values appear than expected — is a red flag for an unmodeled batch effect or other confounder inflating apparent significance. You might find 5,000 genes with padj < 0.05, but if the p-value distribution is anti-conservative, that FDR estimate is unreliable.

## Avoiding Post-Hoc Threshold Manipulation

A common problematic practice is to run DE analysis, observe how many genes pass at FDR < 0.05, and then adjust the threshold (to 0.10 or 0.01) to get a "better" result. This is **post-hoc threshold manipulation** — a form of p-hacking that inflates false discovery rates. The FDR threshold should be pre-specified before analysis as part of the study design, not chosen based on the results.

Similarly, using both a fold change threshold AND an adjusted p-value threshold is valid (and recommended to filter biologically small effects), but both criteria should be decided a priori.

The spirit of multiple testing correction is that you commit to a procedure before seeing the data and report exactly what it tells you. The moment you start adjusting thresholds based on how many genes you get, the formal statistical guarantees dissolve. You are no longer controlling the FDR at 5%; you are controlling it at whatever level happens to give you a satisfying number of genes. This is the route back to the problems Ioannidis described — and back to the unreproducible genomics literature of the early 2000s.

## Why This Matters

The multiple testing problem is one of the main reasons that early microarray studies were plagued by irreproducible findings; applying appropriate FDR control via the Benjamini-Hochberg procedure is now a non-negotiable standard for genome-wide association studies and differential expression analysis. Understanding it separates rigorous analysis from statistically invalid conclusions. Every gene list you report, every heatmap of "top DE genes," every pathway enrichment analysis — all of them inherit the reliability of the multiple testing correction you applied upstream. Get it right, and your results stand on firm statistical ground. Get it wrong, and you are building biology on noise.
