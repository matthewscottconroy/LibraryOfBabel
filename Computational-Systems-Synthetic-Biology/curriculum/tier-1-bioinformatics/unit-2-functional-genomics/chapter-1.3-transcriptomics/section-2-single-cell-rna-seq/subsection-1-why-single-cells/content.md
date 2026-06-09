# Why Single Cells?

In 2016, a team studying the immune response to intestinal parasites published a result that would have been completely invisible using standard RNA-seq. They were trying to understand how the body defends itself against helminths — parasitic worms that infect over a billion people worldwide. Using single-cell RNA sequencing on intestinal epithelial cells, they found a rare cell type called tuft cells — previously considered an obscure curiosity — dramatically expanded during infection. These cells constituted fewer than 1% of the intestinal epithelium. They produced interleukin-25, which triggered the type 2 immune response necessary to expel the worms. In bulk RNA-seq, their signal would have been diluted into invisibility. In single cells, they were the story.

This is the fundamental motivation for single-cell RNA sequencing. Bulk RNA-seq measures the average gene expression across all cells in a sample. For a milliliter of blood or a milligram of tumor tissue, this average can be informative — but it conceals an enormous amount of biologically important information. Single-cell RNA sequencing (scRNA-seq) resolves gene expression at the resolution of individual cells, unlocking questions that bulk RNA-seq cannot address.

## The Problem with Averages

Consider a tumor biopsy containing cancer cells, immune cells, stromal fibroblasts, and endothelial cells. A bulk RNA-seq measurement of a gene like *CD8A* reflects its average expression across all these cell types. If the tumor is highly infiltrated with CD8+ T cells, *CD8A* will appear highly expressed. If CD8+ T cells are rare, it will appear low. But neither measurement tells you whether the CD8+ T cells are exhausted, activated, or memory cells — information critical for predicting immunotherapy response.

**Cell type heterogeneity** is the rule, not the exception, in tissues. Even nominally homogeneous cell cultures contain cells at different stages of the cell cycle, in different metabolic states, or responding differently to microenvironmental signals. You might expect that averaging thousands of cells produces a stable, representative measurement. It turns out that averages are stable precisely because they are blind to the variation that most often matters.

## Discovering Novel Cell Types

One of the most striking early successes of scRNA-seq was the discovery of rare cell types invisible in bulk data. In 2016, Bhatt and colleagues used scRNA-seq to identify **tuft cells** as a previously underappreciated cell type in the intestinal epithelium — and crucially, to show that they were dramatically expanded in the context of helminth infection, revealing an unexpected role in type 2 immunity. These cells constituted fewer than 1% of intestinal epithelial cells and would have been completely invisible in bulk RNA-seq.

This was not an isolated discovery. Since then, scRNA-seq has identified: ionocytes (a rare lung epithelial cell type that highly expresses *CFTR*, the cystic fibrosis gene, suggesting a concentrated site of disease biology), numerous brain cell subtypes previously lumped together as "astrocytes," and functionally distinct tumor-infiltrating T cell populations with profoundly different therapeutic implications. Each of these discoveries required the resolution that only individual-cell measurement provides.

## Cell State Transitions

Bulk RNA-seq of a differentiating cell population produces an average of all differentiation states, smearing out the progression from progenitor to terminally differentiated cell. If you take a blood stem cell culture halfway through differentiation into red blood cells and measure it by bulk RNA-seq, you see a blend of stem cell genes and red blood cell genes in proportions that do not correspond to any real cell in the culture. You cannot tell whether the cells are in a true intermediate state or whether you simply have a mixture of two populations at the extremes.

scRNA-seq captures cells at every point along this continuum, enabling **pseudotime analysis** (see Subsection 4) that reconstructs the developmental trajectory. This has transformed our understanding of hematopoiesis, neural development, and stem cell biology — turning static population averages into dynamic movies of cell fate decisions.

## Comparison: Bulk vs. scRNA-seq

| Feature | Bulk RNA-seq | scRNA-seq |
|---|---|---|
| Cell resolution | Population average | Per cell |
| Input material | 1 µg total RNA | ~hundreds to thousands of cells |
| Cost per sample | ~$150–300 | ~$1,000–3,000+ |
| Read depth per cell | N/A | ~5,000–50,000 reads/cell |
| Genes detected | ~15,000–20,000 | ~2,000–5,000 per cell |
| Statistical power for DE | High (many reads per gene) | Lower (sparse per cell) |
| Cell type composition | Confounded | Resolved |
| Rare cell type discovery | No | Yes |

## Cost-Benefit Trade-offs

scRNA-seq is substantially more expensive than bulk RNA-seq and produces sparser data per gene (because reads are distributed across thousands of cells rather than concentrated per gene). This sparsity creates **dropout** — the technical failure to detect a gene that is actually expressed — which introduces zeros that are not biologically meaningful. Statistical methods for scRNA-seq must account for this.

For a question like "how does treatment X affect the transcriptome of my cell line?" where cells are expected to be homogeneous, bulk RNA-seq with n ≥ 3 replicates and 30 million reads is more powerful and cheaper than scRNA-seq. For questions involving tissue composition, rare cells, developmental trajectories, or cell-type-specific responses, scRNA-seq is the appropriate choice.

It turns out that scRNA-seq and bulk RNA-seq are not competitors — they are complementary. Bulk RNA-seq provides deep, statistically powerful measurement of the average transcriptome. scRNA-seq provides cellular resolution at the cost of depth and sparsity. The best studies often use both: bulk RNA-seq to identify the most differentially expressed genes with high statistical power, and scRNA-seq to determine which cell types drive those expression changes.

## A Practical Decision Framework

- **Homogeneous samples, known cell type, DE question** → Bulk RNA-seq
- **Complex tissue, cell type composition question** → scRNA-seq
- **Developmental trajectory** → scRNA-seq with trajectory analysis
- **Spatial organization of cell types** → Spatial transcriptomics (Subsection 7)
- **Budget-constrained, initial exploration** → Bulk RNA-seq, then scRNA-seq on key samples

## Why This Matters

Single-cell transcriptomics has fundamentally changed how biologists understand cellular identity and state. It revealed that tissues long considered simple are composed of dozens of distinct cell types, each with its own gene expression program and functional specialization. It has shown that cell states are dynamic and continuous rather than discrete. And it has provided the cellular resolution necessary to understand disease heterogeneity: why two patients with the same cancer diagnosis respond differently to the same treatment, why some tumors are infiltrated with functional immune cells and others are immune deserts. This resolution is now essential for understanding disease heterogeneity, therapy resistance, and the molecular logic of development.
