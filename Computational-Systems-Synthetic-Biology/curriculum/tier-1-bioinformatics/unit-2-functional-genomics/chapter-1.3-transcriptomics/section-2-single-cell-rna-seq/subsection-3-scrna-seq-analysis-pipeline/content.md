# The scRNA-seq Analysis Pipeline

You have run your 10x experiment and received a hard drive with FASTQ files. Inside those files is a molecular census of thousands of individual cells — their gene expression states frozen at the moment you added lysis buffer. Your goal is to transform that information into something biologically interpretable: a map of cell types, a portrait of cellular states, a picture of which cells express which genes. The scRNA-seq analysis pipeline is the series of computational steps that accomplish this transformation.

The two primary software ecosystems — **Seurat** (R) and **Scanpy** (Python/AnnData) — implement the same conceptual steps. This section walks through the full pipeline with Scanpy pseudocode. The logic of each step matters as much as the commands themselves.

## Step 1: Alignment and UMI Counting with Cell Ranger

Before analysis can begin, the raw sequencing reads must be decoded: each read carries a cell barcode (which cell did this RNA come from?) and a UMI (which molecular copy of that RNA is this?). These reads are aligned to the genome and the result is a count matrix — one number per gene per cell.

**Cell Ranger** (10x Genomics) is the standard preprocessing tool for 10x data. It aligns reads to the reference genome using STAR, assigns reads to cell barcodes, counts UMIs per gene per cell, and produces a filtered cell-by-gene count matrix (sparse format). The output includes: `barcodes.tsv.gz`, `features.tsv.gz`, and `matrix.mtx.gz`, which are loaded into Scanpy as an `AnnData` object.

```python
import scanpy as sc
import numpy as np

# Load Cell Ranger output
adata = sc.read_10x_mtx('path/to/cellranger/filtered_feature_bc_matrix/',
                          var_names='gene_symbols')
adata  # Shape: (n_cells, n_genes)
```

The resulting `AnnData` object has shape (n_cells × n_genes). With a typical 10x experiment targeting 5,000 cells and a human genome annotation of ~33,000 genes, you begin with a 5,000 × 33,000 matrix that is ~95% zeros — hence the sparse format.

## Step 2: Quality Control Filtering

Not all barcodes in the Cell Ranger output correspond to real cells. Some are empty droplets that captured ambient RNA but no cell. Others are damaged cells that are dying. Still others are doublets — two cells in one droplet. All of these contaminate your data in different ways, and removing them is the first analytical priority.

Not all barcodes represent viable single cells. QC filtering removes:
- **Empty droplets**: Very low nCounts (<200 genes detected) — probably empty droplets.
- **Dead/damaged cells**: High mitochondrial gene fraction. Damaged cells lose cytoplasmic mRNA but retain mitochondrial mRNA (encoded in mitochondria). A threshold of >20–25% mitochondrial reads is commonly used, but the appropriate threshold is dataset-specific.
- **Doublets**: Very high nCounts or nGenes (>5,000–6,000 genes in a single barcode suggests two cells). Formal doublet detection with Scrublet is preferred.

```python
# Calculate QC metrics
adata.var['mt'] = adata.var_names.str.startswith('MT-')
sc.pp.calculate_qc_metrics(adata, qc_vars=['mt'], inplace=True)

# Filter
sc.pl.violin(adata, ['n_genes_by_counts', 'total_counts', 'pct_counts_mt'])
adata = adata[adata.obs['n_genes_by_counts'] > 200]
adata = adata[adata.obs['n_genes_by_counts'] < 6000]
adata = adata[adata.obs['pct_counts_mt'] < 20]
```

It is worth pausing on the mitochondrial filter. The logic is elegant: when a cell's membrane ruptures, cytoplasmic RNA leaks out, but mitochondria — which are enclosed organelles — remain intact and continue releasing their RNA into the lysate. A dying cell therefore produces a library dominated by mitochondrial transcripts and depleted of cytoplasmic ones. High mitochondrial fraction is a molecular death certificate.

## Step 3: Normalization

After filtering, cells have different total UMI counts — not because they differ in overall transcriptional activity, but because of technical variation in capture efficiency. A cell with 3,000 total counts and a cell with 1,500 counts might have the same underlying expression state, but all their absolute count values differ by a factor of 2. Normalization removes this technical scaling factor.

After filtering, counts are normalized to correct for differences in sequencing depth across cells. The **normalize_total** step scales each cell's counts so that they sum to 10,000 (often called "counts per 10k" or CP10K):

$$x_{ij}' = \frac{x_{ij}}{\sum_j x_{ij}} \times 10{,}000$$

Then **log1p** transformation ($\log(x + 1)$) is applied to compress the dynamic range and make the data more normally distributed for downstream linear methods:

```python
sc.pp.normalize_total(adata, target_sum=1e4)
sc.pp.log1p(adata)
```

The log transformation is important because gene expression spans several orders of magnitude. Without it, a gene expressed at 5,000 counts would dominate PCA simply by its scale, even if it does not vary meaningfully between cell types. Log transformation compresses these extremes and gives lowly expressed genes a fighting chance to contribute to the analysis.

## Step 4: Highly Variable Gene (HVG) Selection

With ~20,000–30,000 genes but only ~3,000 informative ones per cell, most genes are zeros in most cells. **HVG selection** retains only genes that vary substantially across cells — these carry the signal distinguishing cell types. Typically 2,000–5,000 HVGs are selected using a mean-variance trend: genes with higher variance than expected for their mean expression are retained.

```python
sc.pp.highly_variable_genes(adata, min_mean=0.0125, max_mean=3, min_disp=0.5)
adata = adata[:, adata.var.highly_variable]
```

You might expect that using all genes would give the most complete picture. It turns out that including non-variable genes adds noise without signal — their uniform pattern across cells dilutes the signal from informative genes and distorts the geometry of downstream dimensionality reduction. This is not feature selection in the machine-learning sense of avoiding overfitting; it is more like tuning out static on a radio to hear the signal clearly.

## Step 5: PCA

**Principal Component Analysis** reduces the high-dimensional expression space (2,000+ HVGs) to the top 50 principal components that capture the most variance. PCA is a linear dimensionalization that groups cells by their transcriptional similarity. The data is optionally scaled (zero mean, unit variance per gene) before PCA to prevent high-expression genes from dominating.

```python
sc.pp.scale(adata, max_value=10)
sc.tl.pca(adata, svd_solver='arpack')
sc.pl.pca_variance_ratio(adata, log=True)  # Choose number of PCs (typically 20-50)
```

The scree plot of variance explained by each PC helps determine how many PCs to retain. Typically the variance explained drops steeply for the first 20–30 PCs and then flattens — the "elbow" in the curve suggests the number of PCs carrying real biological signal. Using too few PCs will merge distinct cell types that differ on lower PCs; using too many will include noise.

## Step 6: Neighborhood Graph and UMAP

A **k-nearest-neighbor graph** is constructed in PCA space: each cell is connected to its k most similar cells (typically k=15). This graph represents the local topology of the data. **UMAP** (Uniform Manifold Approximation and Projection) then embeds this graph into 2D for visualization, preserving local neighborhood structure.

```python
sc.pp.neighbors(adata, n_neighbors=15, n_pcs=40)
sc.tl.umap(adata)
sc.pl.umap(adata)
```

UMAP is a visualization tool, not an analytical tool. This distinction matters enormously. The distances between clusters in UMAP space are not quantitatively meaningful — you cannot say that two clusters 5 units apart are "more different" than two clusters 2 units apart. UMAP optimizes to preserve local neighborhoods, sacrificing global structure. All clustering and statistical analyses should be performed on the PCA or graph representation, not on UMAP coordinates.

## Step 7: Leiden/Louvain Clustering

Clustering identifies groups of cells with similar transcriptional profiles. The **Leiden algorithm** (an improvement on Louvain) optimizes modularity on the k-NN graph — grouping cells that are more connected to each other than expected by chance. The **resolution** parameter controls granularity: higher resolution → more, smaller clusters.

```python
sc.tl.leiden(adata, resolution=0.5)
sc.pl.umap(adata, color='leiden')
```

Choosing resolution is as much biology as statistics. Start at resolution 0.5 and examine the clusters biologically — do the marker genes make sense? Are well-established cell types split into multiple clusters (over-clustering) or merged together (under-clustering)? Adjust accordingly. The "correct" clustering is the one that best represents the biological structure in your data, not a numerical optimum.

## Step 8: Marker Gene Identification and Cell Type Annotation

For each cluster, a statistical test identifies **marker genes** — genes differentially expressed in that cluster vs. all others. The **Wilcoxon rank-sum test** is preferred (robust, no distributional assumptions). Results include log fold change, p-value, and fraction of cells expressing the gene in cluster vs. rest.

```python
sc.tl.rank_genes_groups(adata, groupby='leiden', method='wilcoxon')
sc.pl.rank_genes_groups(adata, n_genes=10, sharey=False)
```

**Cell type annotation** maps cluster marker genes to known biology using reference databases (CellMarker, PanglaoDB), published datasets, or automated tools like CellTypist. For example, a cluster with high *CD3D*, *CD3E*, *CD8A* expression is annotated as CD8+ T cells.

It turns out that automated annotation tools are useful for well-characterized tissues but fail for novel cell types or species without comprehensive references. Manual annotation — carefully reading the top marker genes for each cluster and cross-referencing the literature — remains essential, especially for validating automated calls and for unusual or poorly annotated cell types.

## Why This Matters

The scRNA-seq analysis pipeline is now an essential skill for any biologist working with tissues or complex cell mixtures. Its output — a map of cell types and states in a sample — has become the foundation for understanding disease heterogeneity, developmental biology, and targeted therapy design. Every step in this pipeline embodies a conceptual choice about how to handle the particular challenges of single-cell data: sparsity, heterogeneity, and the curse of high dimensionality. Mastering the logic of each step, not just the commands, gives you the ability to diagnose failures, adapt to novel data types, and interpret results with appropriate confidence.
