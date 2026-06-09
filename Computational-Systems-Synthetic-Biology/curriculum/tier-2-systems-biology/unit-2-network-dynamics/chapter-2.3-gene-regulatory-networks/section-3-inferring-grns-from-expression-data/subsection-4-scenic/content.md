# SCENIC: Single-Cell Regulatory Network Inference and Clustering

## What SCENIC Solves

Two genes that are co-expressed across many cells might share a common regulator — or one might directly regulate the other. These two scenarios look identical in a pure expression analysis, and the difference matters enormously for biology and for therapy. Standard GRN inference from expression data identifies statistically correlated TF-target pairs but cannot confirm that the TF actually binds to the target gene's regulatory region. Many correlations arise from indirect effects — gene A and gene B are co-expressed because they share a common regulator, not because A directly regulates B.

**SCENIC** (Single-Cell rEgulatory Network Inference and Clustering; Aibar et al. 2017) addresses this by combining expression-based correlation (to identify candidate TF-target links) with motif enrichment analysis (to confirm direct regulatory relationships). The result is a set of **regulons** — TFs with their statistically validated direct target genes — and a cell-by-regulon activity matrix that characterizes each cell's regulatory state.

## The Three-Step Workflow

### Step 1: GRN Inference (GRNBoost2)

Run GRNBoost2 (or GENIE3) on the single-cell expression matrix to produce a ranked list of TF-target candidate links:

```python
import scanpy as sc
from arboreto.algo import grnboost2

# Load preprocessed scRNA-seq data
adata = sc.read_h5ad('pbmc_processed.h5ad')
expr_df = pd.DataFrame(adata.X.toarray(), 
                        index=adata.obs_names,
                        columns=adata.var_names)

# GRN inference: all TFs as potential regulators of all genes
adjacencies = grnboost2(expression_data=expr_df,
                         tf_names=tf_list,
                         verbose=True,
                         seed=42)
# adjacencies: DataFrame with [TF, target, importance]
adjacencies.to_csv('adj.tsv', sep='\t', index=False)
```

This step produces ~100,000–1,000,000 candidate edges for a typical single-cell dataset. Most are indirect.

### Step 2: Regulon Identification (RcisTarget)

**RcisTarget** (Imrichová et al. 2015) filters candidate TF-target links by requiring that the TF's binding motif be enriched in the promoter or enhancer regions of the proposed targets:

```python
import pyscenic

# Prune adjacencies using motif enrichment
# Requires: ranking databases (hg38-*.feather files) and motif-to-TF mapping
pyscenic.aucell.create_rankings(
    ex_matrix=expr_df,
    auc_threshold=0.05
)

# Run RcisTarget pruning
regulons = pyscenic.prune2df(
    adjacencies,
    dbs=['hg38_500bp_up_100bp_down_full_tx.mc9nr.genes_vs_motifs.rankings.feather'],
    motif_annotations='motifs-v9-nr.hgnc-m0.001-o0.0.tbl',
    mask_dropouts=True
)
```

For each TF-target set, RcisTarget computes a **Normalized Enrichment Score (NES)** measuring whether the TF's known binding motifs are significantly enriched in the regulatory regions of the proposed targets. Only TF-target sets with NES > 3.0 (default) are retained as regulons.

This step typically reduces the edge set by 50–90%, retaining only biologically plausible direct regulatory interactions.

Think about what this pruning achieves. Step 1 gives you a list of statistically associated TF-target pairs — many will be indirect. Step 2 asks: "Is there actually a binding site for this TF in the regulatory region of this proposed target?" The combination of statistical evidence from expression and mechanistic evidence from genomic sequence is substantially more reliable than either alone.

### Step 3: Regulon Activity Scoring (AUCell)

**AUCell** scores the activity of each regulon in each individual cell based on the expression ranks of the regulon's target genes:

$$\text{AUC}_{ij} = \text{Area under the recovery curve for regulon } j \text{ in cell } i$$

The AUC measures whether the top-ranking genes in cell $i$ (by expression) are enriched for the targets of regulon $j$. High AUC → regulon $j$ is active in cell $i$.

```python
# Compute AUCell scores
auc_mtx = pyscenic.aucell.create_rankings(expr_df)
auc_scores = pyscenic.aucell.aucell(regulons, auc_mtx, 
                                     auc_threshold=0.05,
                                     num_workers=8)
# auc_scores: cells × regulons matrix
```

The resulting **regulon activity matrix** (cells × regulons) can be used for:
- Cell type annotation independent of clustering
- Identification of master regulators for each cell type
- Trajectory analysis (regulon activity changes along differentiation)
- Cross-species comparison of regulatory programs

## Interpreting SCENIC Output

**Regulon size**: larger regulons (more targets) are generally better supported but may include indirect targets. Smaller regulons are more specific.

**Regulon specificity**: regulons active in only one cell type are likely master regulators of that cell type. Broadly active regulons regulate housekeeping functions.

**Worked example — PBMC analysis:**

In peripheral blood mononuclear cells (PBMCs), SCENIC typically identifies:
- **IRF7 regulon**: active specifically in plasmacytoid dendritic cells; known interferon response master regulator
- **SPI1 (PU.1) regulon**: active in monocytes and B cells; myeloid master regulator
- **GATA3 regulon**: active in T helper cells; CD4+ T cell identity

These recapitulate known cell-type-defining TFs, validating the SCENIC approach. The fact that SCENIC recovers IRF7 as a plasmacytoid dendritic cell master regulator — without any prior knowledge of cell type labels — is a meaningful validation. The regulon activity matrix contains enough structure to independently reconstruct known cell type identity.

## Limitations and Considerations

**Dropout sparsity**: single-cell RNA-seq data has high dropout rates (many genes falsely appear as zero). SCENIC handles this by using gene rankings rather than raw expression values in AUCell, but very low-abundance TFs may still be missed.

**Motif database completeness**: RcisTarget requires comprehensive TF binding motif databases. For less-studied organisms, motif databases are incomplete, reducing specificity.

**Computational cost**: for datasets with >50,000 cells and 20,000 genes, the full SCENIC pipeline can require several hours even on a high-memory workstation (>64 GB RAM). Cloud computing or approximation methods are practical necessities.

**Static snapshot**: SCENIC from steady-state scRNA-seq does not capture dynamic regulatory changes. Combining SCENIC with RNA velocity or time-course data is an active research direction.

## Why This Matters

SCENIC represents a major advance in GRN inference because it combines statistical and mechanistic evidence. Requiring both expression correlation and motif enrichment dramatically reduces false positives compared to expression-based methods alone. The regulon activity matrix provides a compressed, interpretable representation of each cell's regulatory state, enabling cell type annotation, trajectory analysis, and master regulator identification without prior knowledge of marker genes. SCENIC has become a standard component of single-cell multi-omics analysis pipelines.
