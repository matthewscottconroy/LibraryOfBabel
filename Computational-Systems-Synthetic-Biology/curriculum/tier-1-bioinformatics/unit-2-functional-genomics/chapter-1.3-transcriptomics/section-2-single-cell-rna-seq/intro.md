# Section 2: Single-Cell RNA-seq

In 2009, a paper appeared in Nature Methods describing a method for sequencing the transcriptome of a single mouse blastomere — one cell from an early embryo. It detected about 75% of known transcripts, compared to essentially 100% from bulk RNA-seq of many cells. The sensitivity was lower. The cost was orders of magnitude higher. And yet the paper caused enormous excitement, because it opened a window onto something that bulk RNA-seq could never show: what individual cells are actually doing.

Tissues are not homogeneous. A liver, a tumor, a brain section — each contains dozens of distinct cell types, each with its own gene expression program, each responding to its environment in its own way. Averaging across all of them, as bulk RNA-seq does, conceals exactly the biological variation that most often matters. Single-cell RNA-seq resolves that variation, one cell at a time.

This section covers the complete arc of single-cell transcriptomics — from the motivation for single-cell resolution, through the technologies that achieve it, through the computational methods that analyze the resulting data, and out to the most recent frontiers of the field.

## What You Will Learn in This Section

**Subsection 1: Why Single Cells?** frames the problem that single-cell RNA-seq solves and helps you decide when to use it. Through the story of tuft cells and intestinal immunity — a cell type invisible in bulk data that turned out to be central to parasite defense — you will understand why averages are not just incomplete but actively misleading in heterogeneous tissues. The subsection includes a direct comparison of bulk and single-cell methods and a practical decision framework for choosing between them.

**Subsection 2: Technology Overview** explains the engineering behind the major platforms — how 10x Genomics Chromium uses microfluidic droplets and barcoded beads to tag thousands of cells simultaneously, what UMIs are and why they are necessary, and what artifacts (ambient RNA, doublets) are introduced by the technology and must be computationally corrected. The comparison of 10x with Smart-seq2 and Split-seq illustrates the throughput/resolution/cost trade-offs that determine platform choice.

**Subsection 3: The scRNA-seq Analysis Pipeline** walks through the canonical Scanpy workflow from Cell Ranger output to annotated cell types. Each step — QC filtering, normalization, highly variable gene selection, PCA, UMAP, clustering, and marker gene identification — is explained with the logic behind it, not just the commands. Special attention is paid to the distinctions between visualization and analysis, and to the choices (like clustering resolution) that require biological judgment rather than algorithmic defaults.

**Subsections 4 and 5: Trajectory, Pseudotime, and RNA Velocity** address the dynamic dimension of single-cell data. You will learn how DPT, Monocle3, and PAGA reconstruct the temporal ordering of cells along differentiation trajectories from static snapshots — and what pseudotime can and cannot tell you about causality. RNA velocity goes further: by comparing unspliced to spliced mRNA ratios, it infers the direction and speed of each cell's transcriptional change, turning a static map into a dynamic flow field.

**Subsections 6 and 7: Cell-Cell Communication and Spatial Transcriptomics** represent the expanding frontier of the field. CellChat, NicheNet, and CellPhoneDB mine scRNA-seq data for ligand-receptor interactions to infer cellular signaling networks — with important caveats about what transcriptional co-expression can and cannot confirm. Spatial transcriptomics (Visium, Slide-seq, MERFISH) adds the dimension that dissociation-based scRNA-seq discards: physical location within the tissue.

## The Logical Flow

The section moves from motivation (why?) to technology (how?) to standard analysis (what do we compute?) to advanced methods (what else can we learn?). The advanced methods in subsections 4–7 each depend on the standard pipeline established in subsection 3 — you need cell type annotations and a neighborhood graph before you can compute trajectories or infer communication. Plan to work through the subsections in order before exploring specific tools in depth.
