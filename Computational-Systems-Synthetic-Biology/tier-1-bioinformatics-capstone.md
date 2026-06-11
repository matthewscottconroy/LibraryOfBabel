# Tier 1 Capstone: Bioinformatics Integration Project

## "End-to-End RNA-seq Analysis with Biological Interpretation"

---

## Overview

The Tier 1 Bioinformatics capstone is an end-to-end RNA-seq analysis of a publicly available dataset that requires you to apply every computational genomics skill developed across the six modules of Tier 1. You will move from raw sequencing reads to biological interpretation: identifying differentially expressed genes, characterising the biological pathways they belong to, and constructing a mechanistic hypothesis about the system's response to its perturbation.

The project is deliberately open in its choice of biological system — the emphasis is on methodological thoroughness and scientific interpretation. You are expected to know *why* you are performing each step, not just *how* to run the relevant tool.

---

## Biological Motivation

Transcriptomics is the gateway to understanding gene regulation at the genomic scale. Bulk RNA-seq, now a mature technology, can reveal how thousands of genes simultaneously adjust their expression in response to a perturbation. But the computational analysis is only as valuable as its biological interpretation: a list of 3,000 differentially expressed genes is not a result — it is a starting point. The goal of this capstone is to teach you to transform raw reads into a coherent biological story.

---

## Dataset Selection

Choose **one** of the following publicly available datasets from NCBI GEO:

1. **Carbon source shift in *E. coli*** (GSE55661): comparison of glucose vs. acetate growth — a classic model of metabolic reprogramming. Ideal if you want to integrate with FBA (Tier 2 preview).

2. **Heat shock response in yeast** (GSE35386): comparison of *S. cerevisiae* before and after heat shock — a model of stress response. Canonical regulatory network.

3. **Antibiotic response in *Staphylococcus aureus*** (GSE46819): comparison of MRSA before and after antibiotic treatment — a clinically relevant system with regulatory complexity.

---

## Project Components

### Component 1: Data Acquisition and Quality Control (Week 1)

**Tasks:**
- Download raw FASTQ files from NCBI SRA using `fastq-dump` or `fasterq-dump`. Confirm file integrity with checksums.
- Run FastQC on all samples. Summarise quality metrics for each sample: per-base quality scores, GC content, adapter contamination, sequence duplication rates.
- Write a QC report identifying any samples of concern and justifying the decision to include or exclude them.
- Trim adapters and low-quality bases using Trimmomatic or cutadapt. Re-run FastQC to confirm improvement.

**Deliverable:** QC report (1 page) with FastQC summary figures before and after trimming.

### Component 2: Read Alignment and Quantification (Week 2)

**Tasks:**
- Download the reference genome and annotation (GTF file) for your organism from Ensembl or NCBI.
- Build a genome index for STAR or HISAT2.
- Align trimmed reads to the reference genome. Report overall alignment rate; investigate samples with unexpectedly low alignment rates.
- Quantify gene-level read counts using featureCounts (Subread) or STAR's built-in counting. Inspect the count matrix for library size variation.
- Produce a MultiQC report integrating quality metrics from all samples.

**Key quality metrics:** Alignment rate ≥ 80%; duplication rate ≤ 40% for most organisms; library size variation ≤ 2× across samples (flag if greater).

**Deliverable:** Alignment statistics table, count matrix (first 10 genes shown), MultiQC report.

### Component 3: Exploratory Analysis (Week 3)

**Tasks:**
- Perform PCA on rlog-transformed count data. Plot PC1 vs. PC2, labelling samples by condition and (if applicable) batch.
- Produce a sample-to-sample distance heatmap.
- Verify that biological replicates cluster together. If an outlier sample is identified, perform a sensitivity analysis (include and exclude it) to assess its impact on downstream results.
- Plot a histogram of raw count distributions across samples to confirm approximate normality after log-transformation.

**Deliverable:** PCA plot, distance heatmap, outlier analysis if applicable.

### Component 4: Differential Expression Analysis (Week 4)

**Tasks:**
- Run DESeq2 on the count matrix with appropriate experimental design formula.
- Apply shrinkage of log2 fold change estimates using `lfcShrink` (apeglm method).
- Filter results: adjusted p-value ≤ 0.05, |log2FC| ≥ 1.
- Produce: (a) a volcano plot annotating the top 20 differentially expressed genes by gene name; (b) a heatmap of the top 50 DEGs with hierarchical clustering of samples and genes; (c) MA plot.
- Report the number of significantly up- and down-regulated genes.

**Deliverable:** Volcano plot, heatmap, MA plot, table of top 20 DEGs with gene names, log2FC, and adjusted p-values.

### Component 5: Functional Enrichment and Pathway Analysis (Week 5)

**Tasks:**
- Perform GO enrichment analysis on the list of significantly upregulated genes and separately on downregulated genes, using clusterProfiler or g:Profiler.
- Perform KEGG pathway enrichment analysis.
- Plot the top 10 GO terms for each direction as a dot plot (showing enrichment ratio and adjusted p-value).
- Identify the most significantly affected biological processes and molecular functions.
- Cross-reference your enrichment results with the primary literature: are the enriched pathways consistent with known biology of the perturbation?

**Deliverable:** GO enrichment plots (up and down), KEGG dot plot, one-page narrative connecting enrichment results to known biology.

### Component 6: Biological Synthesis and Mechanistic Hypothesis (Week 6)

**Tasks:**
- Write a 3-page scientific narrative that synthesises your results into a coherent biological story:
  - What is the organism doing in response to the perturbation?
  - Which transcription factors, signalling pathways, or regulatory networks are likely responsible for the observed gene expression changes?
  - Propose a mechanistic model (a simple circuit diagram) of the key regulatory events.
  - What additional experiments would you perform to test your hypothesis?
- Identify one result that was unexpected (not predicted by prior literature or your initial hypothesis) and discuss its potential significance.

**Deliverable:** 3-page scientific narrative with circuit diagram and experimental proposals.

---

## Expected Deliverables

| Component | Deliverable | Word Count / Format |
|-----------|-------------|---------------------|
| Quality control | QC report + figures | 500 words |
| Alignment | Alignment table + MultiQC | — |
| Exploratory | PCA + heatmap | — |
| Differential expression | Results table + figures | — |
| Enrichment | GO/KEGG plots + narrative | 500 words |
| Biological synthesis | Scientific narrative | ~1500 words |

---

## Assessment Rubric

| Criterion | Weight | Excellent | Proficient | Developing |
|-----------|--------|-----------|------------|------------|
| Technical execution | 25% | All steps completed correctly, tools used appropriately | Minor errors, tools mostly appropriate | Significant errors in pipeline |
| Data quality assessment | 15% | Thorough QC, outliers identified and handled | Basic QC performed | QC skipped or superficial |
| Statistical rigour | 20% | Correct DESeq2 design, shrinkage applied, multiple testing corrected | Mostly correct, some statistical errors | Incorrect model or no multiple testing correction |
| Biological interpretation | 30% | Deep mechanistic interpretation connecting DEGs to regulatory biology | Superficial interpretation (lists enriched terms without insight) | No biological interpretation |
| Scientific writing | 10% | Clear, precise, figure captions informative | Adequate | Unclear |

---

## Extension Challenges

**Extension A: Single-cell RNA-seq.** If a paired single-cell dataset is available for your system (check GEO for 10x Genomics datasets), perform a Seurat/Scanpy analysis: preprocessing, dimensionality reduction (UMAP), clustering, and cell type annotation. Identify which cell types show the most differential response to the perturbation.

**Extension B: Regulatory network inference.** Using your DEG list, query the JASPAR transcription factor database to identify TF binding motifs enriched in the promoters of upregulated genes. Propose which TFs might be the master regulators of the response.

**Extension C: Integration with Tier 2.** If you chose the *E. coli* carbon source dataset, preview Tier 2 by performing FBA on the iJO1366 genome-scale model under glucose and acetate conditions, and compare the predicted flux changes with your observed gene expression changes.
