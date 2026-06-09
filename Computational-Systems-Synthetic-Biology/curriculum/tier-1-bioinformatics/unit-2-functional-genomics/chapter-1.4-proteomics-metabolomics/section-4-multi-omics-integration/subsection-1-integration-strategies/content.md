# Multi-Omics Integration Strategies

In 2013, a study of colorectal cancer published in Nature assembled the most comprehensive molecular portrait of a cancer type then available: whole-exome sequencing, DNA copy number, DNA methylation, RNA-seq, microRNA, and reverse-phase protein array data, all from the same set of tumors. The researchers expected that integrating these layers would simply confirm what each layer showed independently. Instead, they found that samples clustered differently in the integrated analysis than in any single layer alone, and that integrative subtypes had stronger associations with survival and pathway activity than transcriptomics-only subtypes. The integration revealed biology that each layer had missed in isolation.

No single omics layer provides a complete picture of a biological system. The genome encodes what is possible; the transcriptome what is currently being produced; the proteome what is functionally present; and the metabolome what is being actively processed. **Multi-omics integration** combines measurements from two or more of these layers to achieve a more complete understanding of biological state than any single layer can provide. The key challenge is that each omics layer is high-dimensional, technically noisy, and correlated with the others in complex and often non-linear ways.

## The Proteome-Transcriptome Correlation Problem

A sobering observation motivates multi-omics integration rather than simply using one surrogate for another: **the correlation between mRNA abundance and protein abundance for the same gene, in the same cell type, is approximately 0.4** (Pearson r across mammalian datasets). This means only ~16% of variance in protein levels is explained by mRNA levels. The remaining variance arises from translational regulation, protein stability differences (half-lives vary 1,000-fold between proteins), post-translational modifications, and protein complex assembly constraints. Omitting any layer means missing the majority of the biological signal.

This low mRNA-protein correlation is one of the most important and most underappreciated facts in molecular biology. It means that measuring the transcriptome — which is far cheaper and easier than measuring the proteome — does not give you a good surrogate for the proteome. You might expect that if EGFR mRNA is elevated, EGFR protein is elevated too. In most cases, you would be wrong. The correlation varies enormously by gene: highly expressed housekeeping proteins tend to have good mRNA-protein correlation, while regulated signaling proteins often have almost none. If you are using transcriptomics as a proxy for proteomics, you are systematically biasing your analysis toward the biology of housekeeping genes.

## Three Levels of Integration

**Early fusion** (feature concatenation): All omics feature matrices are concatenated horizontally into a single large feature matrix before any analysis. A sample is described by its genome features, transcriptome features, and proteome features simultaneously.

Pros: Simple; allows all interactions to be discovered.  
Cons: Different omics layers have very different scales, noise structures, and missing data patterns. The curse of dimensionality is severe (tens of thousands of combined features for hundreds of samples). Requires careful normalization and feature selection.

**Intermediate fusion** (latent factor models): Each omics layer is processed separately, and then shared latent factors that explain variance across multiple layers simultaneously are identified. These latent factors represent coordinated biological programs visible across multiple omics layers.

The canonical tool for this is **MOFA** (Multi-Omics Factor Analysis) and its extensions.

**Late fusion** (layer-specific analysis then combination): Each omics layer is analyzed independently (e.g., DE genes, DE proteins, differential metabolites), and then the results are combined using enrichment analysis, network integration, or voting schemes.

Pros: Uses established single-omics methods for each layer; interpretable.  
Cons: Misses cross-layer interactions; fails when a biological effect is subtle in any one layer but coordinated across layers.

The choice among these fusion strategies depends on your question and your data. Early fusion is appropriate when you want to use all omics features simultaneously in a predictive model, and you have enough samples to manage the dimensionality. Intermediate fusion is the right choice when you want to discover coordinated biological programs without knowing what to look for. Late fusion is the natural starting point when you want to use existing, validated single-omics analysis tools and combine their outputs without making strong modeling assumptions about cross-layer relationships.

## MOFA+: Multi-Omics Factor Analysis

**MOFA** (Argelaguet et al., 2018; MOFA+, 2020) is a probabilistic framework that learns **factors** — latent variables that capture coordinated variation across multiple omics datasets. Each factor is a weighted combination of features from one or more omics layers, and together the factors explain the major axes of variation in the multi-omics data.

The model assumes:

$$\mathbf{Y}^{(m)} = \mathbf{Z} \mathbf{W}^{(m)\top} + \boldsymbol{\epsilon}^{(m)}$$

where $\mathbf{Y}^{(m)}$ is the feature matrix for omics layer $m$, $\mathbf{Z}$ is the shared factor matrix (samples × factors), and $\mathbf{W}^{(m)}$ are the factor loadings (features × factors) for layer $m$. The model is fit using variational inference. Factors that load strongly on features from multiple layers represent biologically coordinated programs.

MOFA+ adds group structure (e.g., different patient groups or time points) to model variation within and between groups. It handles missing data naturally (not all samples need to have all omics measured).

MOFA+'s handling of missing data is a practical strength that deserves emphasis. In real multi-omics studies, not every sample will have every data type — some patients may lack proteomic data, or a subset of samples may have been too degraded for RNA-seq. A model that requires complete data would exclude these samples or impute missing omics layers, introducing artifacts. MOFA+ treats missing data correctly within its probabilistic framework, allowing it to use all available information from all samples even when coverage is incomplete.

## Similarity Network Fusion (SNF)

**SNF** (Wang et al., 2014) constructs a patient similarity network for each omics layer (where nodes are patients/samples and edge weights represent similarity in that omics layer), then iteratively fuses these networks into a single consensus network. This combined network captures shared and complementary information across layers.

SNF has been particularly successful for patient stratification in cancer: integrating gene expression + miRNA + DNA methylation networks (TCGA data) reveals cancer subtypes that are more clinically meaningful than those from any single layer.

## Causal Integration: Mendelian Randomization

**Mendelian randomization (MR)** uses genetic variants (SNPs) as instrumental variables to infer causal effects of one molecular trait on another. Because genetic variants are assigned at conception (before disease onset) and are unaffected by confounders, they can be used to test causal hypotheses:

$$\text{SNP} \rightarrow \text{Exposure (e.g., protein level)} \rightarrow \text{Outcome (e.g., disease)}$$

If a SNP is known to affect protein X abundance (a **pQTL**), and that SNP is also associated with disease Y, then there is causal evidence that protein X affects disease Y. Protein-level MR (pQTL MR) and metabolite MR (mQTL MR) have identified drug targets and causal biomarkers from observational studies.

Mendelian randomization is, in a sense, the gold standard of multi-omics integration — because it can address causality rather than mere correlation. The challenge in observational multi-omics studies is that everything is correlated with everything else: disease, lifestyle, environment, genetics, and all omics layers are simultaneously entangled. MR uses the randomization inherent in genetic inheritance to disentangle these relationships. A protein that has a pQTL (a genetic variant that raises protein levels) and whose pQTL also associates with disease risk is causally implicated in disease, not merely correlated with it. Several pQTL MR studies have independently converged on the same drug targets that subsequently succeeded in clinical trials — a remarkable validation of the approach.

## Correlation-Based Integration

Simple Pearson or Spearman correlations between matched omics features (e.g., mRNA vs. protein for each gene across samples) reveal which genes are transcriptionally vs. post-transcriptionally regulated. Correlation networks (WGCNA for transcriptomics; analogous tools for cross-omics) identify modules of co-varying features across layers.

## Why This Matters

Multi-omics integration reflects the true complexity of biological systems — no single measurement captures gene expression, protein function, metabolic activity, and regulatory state simultaneously — and has already identified therapeutic targets, patient stratifications, and mechanistic insights that individual omics approaches missed, making it increasingly the standard approach for large-scale systems biology studies. As the cost of generating multi-omics data continues to fall, the bottleneck shifts further toward analysis: integrating these datasets intelligently, interpreting the results biologically, and distinguishing genuine cross-layer coordination from statistical artifact. The methods in this section are the current toolkit for doing that, but the field is evolving rapidly — the best integration approaches of five years from now likely do not exist yet.
