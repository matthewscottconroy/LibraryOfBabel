# Batch Effects and Normalization

In 2007, a series of gene expression studies claimed to predict chemotherapy response in breast cancer patients using genomic signatures. The stakes were high: these signatures were being considered for clinical use to guide treatment decisions. Then statisticians Keith Baggerly and Kevin Coombes at MD Anderson began a forensic analysis of the raw data. They found that the sample labels had been shifted by one position in the data matrix. They found that some cancer subtypes correlated perfectly with which batch the samples had been processed in. They found that what the papers called "biological signals" were artifacts of sample processing order. The retractions followed. The clinical trials were halted.

This is not an obscure historical episode. It is a cautionary tale that is still playing out in genomics — because **batch effects** are among the most pervasive and damaging sources of error in expression studies, and because they are insidious precisely because they produce systematic patterns that look biological until you check.

Batch effects arise whenever samples are processed under different technical conditions and can completely mask or artificially create biological differences. Understanding what batch effects are, how to detect them, and how to appropriately handle them is essential for producing reproducible results.

## What Is a Batch Effect?

A **batch effect** is systematic, non-biological variation in measured expression levels that is correlated with the technical conditions under which samples were processed rather than with the biological variable of interest. Common sources include:

- **Library preparation date**: Reagent lots change over time; gel run conditions vary.
- **Sequencer run**: Different flow cells can produce different base-calling characteristics.
- **Technician**: Subtle differences in technique (RNA extraction timing, lysis efficiency, incubation temperatures).
- **RNA quality**: Samples from different collection dates or storage conditions may have different degradation levels.
- **Kit lot numbers**: Even the same kit from different lots can produce systematic differences.

These effects are not random noise — they are systematic signals that can dominate the biological signal in PCA analysis. A batch effect is invisible in individual samples but becomes apparent when you compare across groups. Each batch creates its own systematic distortion — upregulating some genes, downregulating others, in a way that is consistent within the batch. When two batches differ, the batch-to-batch variation can easily exceed the biological signal you are trying to detect.

## How to Detect Batch Effects

The primary diagnostic tool is **PCA (Principal Component Analysis)** applied to the normalized count matrix. Color-code the PCA plot by the biological variable of interest (treatment vs. control) and by known technical covariates (batch, date, technician, kit lot).

**Expected pattern**: Samples should cluster by biological condition (treated samples cluster together; control samples cluster together), with batches mixed within biological groups.

**Batch effect pattern**: Samples cluster by batch rather than by condition. Batch separation on PC1 while biological condition only separates on PC2–3 indicates the batch effect dominates the data. If batch and biological condition are confounded (all treated in batch 1, all control in batch 2), it is mathematically impossible to separate their effects.

Additional diagnostic: **hierarchical clustering heatmap** of Pearson correlation between samples. Samples should cluster by biological condition, not by batch.

It turns out that PCA is the single most informative quality control plot in bulk RNA-seq. It should be the first thing you look at after normalization — before any differential expression analysis. A PCA that shows samples clustering by batch rather than biology is not just an inconvenience; it is evidence that the experiment's power to detect biological effects is compromised.

## Batch Correction Methods

### For Bulk RNA-seq: ComBat

**ComBat** (from the sva R package) removes batch effects from the expression matrix using an empirical Bayes framework. It estimates batch-specific parameters (location and scale shifts for each gene in each batch) and adjusts the data to remove them. ComBat-seq is the count-specific version that operates on raw counts rather than log-normalized data.

```r
library(sva)
# For normalized log-expression data:
corrected <- ComBat(dat = log_expression_matrix,
                    batch = metadata$batch,
                    mod = model.matrix(~condition, data = metadata))
```

### For Single-Cell RNA-seq: Harmony and scVI

**Harmony** (Korsunsky et al., 2019) operates in PCA space: it iteratively adjusts cell embeddings to remove batch-specific clustering while preserving biological variation. It is fast (minutes for 100,000 cells) and integrates seamlessly with Seurat and Scanpy.

```python
import scanpy as sc
sc.external.pp.harmony_integrate(adata, key='batch')
# Use adata.obsm['X_pca_harmony'] as input for downstream neighbor graph
```

**scVI** (Lopez et al., 2018) is a variational autoencoder that explicitly models batch as a covariate in a deep generative model of count data. It learns a batch-corrected latent space and can impute missing values. scVI is more powerful than Harmony for complex batch effects but requires more computational resources and careful hyperparameter tuning.

## The Critical Distinction: Visualization vs. DE Analysis

Here is the most important conceptual point in this section, and one of the most commonly violated rules in practice: **batch correction is appropriate for visualization and clustering, but NOT as a preprocessing step for differential expression analysis**.

For visualization (UMAP, PCA), batch-corrected data prevents batches from dominating the plot and enables identification of biologically meaningful clusters. This is appropriate.

For DE analysis (DESeq2, edgeR, limma), **never input batch-corrected counts**. Instead, include the batch variable as a covariate in the statistical model:

```r
# CORRECT: Include batch as a covariate in the design formula
dds <- DESeqDataSetFromMatrix(countData = raw_counts,
                               colData = metadata,
                               design = ~ batch + condition)  # batch before condition
dds <- DESeq(dds)
res <- results(dds, contrast = c("condition", "treated", "control"))
```

This design formula accounts for batch effects statistically during the DE test, without removing them from the data. The test for `condition` is conditioned on `batch`, correctly isolating the biological effect. If you pre-correct counts and then run DESeq2, you: (1) violate the count model assumptions (corrected values are not integers), and (2) underestimate variance, inflating false positives.

Why does this distinction matter so much? Because the two operations — visualization and differential expression — have different requirements. For visualization, you want the dominant structure of the data to reflect biology, not batch. Batch-corrected values serve this purpose well. For DE analysis, you want the statistical test to correctly account for the variance that is attributable to batch. The model does this by estimating the batch effect during fitting and testing only for the condition effect after partitioning out the batch. Pre-correcting the data removes information the model needs to make this partition correctly.

## Surrogate Variable Analysis (SVA)

When batches are unknown or unmeasured, **Surrogate Variable Analysis (SVA)** can identify hidden confounders. SVA estimates latent variables ("surrogate variables") that capture systematic variation in the data that is not explained by the known covariates of interest. These surrogate variables can then be included in the DE model to control for the unknown confounders.

SVA is remarkably useful for studies of patient samples, where many potential confounders — collection site, sample processing time, patient medications — are often not recorded in the metadata but leave detectable signatures in the expression data. Running SVA as a standard check before DE analysis has caught spurious effects in many published datasets.

## Why This Matters

Batch effects have caused multiple high-profile problems in genomics, where cancer subtypes or gene signatures turned out to be artifacts of processing batches rather than biology. The Baggerly-Coombes investigation of the Potti breast cancer data is the most famous example, but it is far from the only one. Understanding how to design experiments to minimize batch effects and how to statistically account for them is therefore a core competency for any computational biologist. The price of ignoring batch effects is not just incorrect papers — it is clinical decisions made on the basis of artifacts.
