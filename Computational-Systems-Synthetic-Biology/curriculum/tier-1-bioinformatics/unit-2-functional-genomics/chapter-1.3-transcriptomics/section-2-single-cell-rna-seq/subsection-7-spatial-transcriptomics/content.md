# Spatial Transcriptomics

When you dissociate a tissue into single cells for scRNA-seq, you gain cellular resolution and lose everything else. You learn what each cell type expresses, but you have no idea where in the tissue those cells live. A tumor-associated macrophage that sits right at the invasive front of a tumor has different neighbors — and therefore different signals and behaviors — than a macrophage in the tumor core or the periphery. A cortical neuron in layer II expresses different genes than one in layer VI, even if both are "excitatory neurons" by standard cluster annotation. The spatial organization of cells is not incidental to their function — in many cases, it is what determines their function.

**Spatial transcriptomics** technologies measure gene expression while preserving the physical location of cells within tissue sections, enabling questions that scRNA-seq simply cannot address: which cell types border the tumor, how gradients of morphogen signaling pattern a tissue, or which niches support stem cells.

## 10x Visium: Capture Spots

**10x Visium** is the most widely adopted commercial spatial transcriptomics platform. A histological tissue section (~10 µm thick) is placed on a slide containing an array of ~5,000 spots, each 55 µm in diameter and separated by 100 µm center-to-center. Each spot contains oligo-dT probes with a unique spatial barcode. Permeabilization releases mRNA from the tissue, which diffuses down to the capture probes on the nearest spot.

After sequencing, each spot's barcode is decoded to its x/y position, yielding a spatial expression matrix: ~5,000 spots × ~30,000 genes. The key limitation is that each 55 µm spot typically contains 2–20 cells, so Visium measures **neighborhood-level** expression rather than truly single-cell resolution. This is a fundamental constraint of the spot-capture approach: you get the spatial map, but you lose cellular resolution within each spot.

The combination of the spatial expression matrix and the histology image — you can see exactly where on the tissue each spot lands — is one of Visium's most powerful features. For a tumor section, you can correlate expression patterns with histological features like areas of necrosis, stroma-dense regions, and immune infiltrates, all visible in the H&E stain.

## Slide-seq: Near-Cellular Resolution

**Slide-seq** (Rodriques et al., 2019) and its improved version **Slide-seqV2** achieve ~10 µm bead diameter, approaching single-cell resolution. Beads are randomly deposited on a slide and their positions decoded by hybridization. The near-cellular resolution allows direct cell-type identification from the expression profile of individual beads. Slide-seq has lower sensitivity than Visium (fewer genes detected per location) due to the smaller capture area.

It turns out that there is a fundamental trade-off between spatial resolution and RNA capture efficiency: smaller capture areas catch fewer RNA molecules. Visium's relatively large spots catch RNA efficiently but smear several cells together. Slide-seq's small beads approach true single-cell resolution but at the cost of detecting fewer genes per location. Technology development in this space is fundamentally about pushing both numbers upward simultaneously.

## Single-Molecule Methods: MERFISH, seqFISH+, Xenium

These approaches directly image mRNA molecules in intact tissue using **fluorescence in situ hybridization (FISH)** with combinatorial barcoding. Rather than capturing RNA on a surface, they hybridize fluorescent probes directly to mRNAs in situ and image them through the microscope. This circumvents the capture-efficiency trade-off entirely.

**MERFISH** (Multiplexed Error-Robust FISH) assigns each gene a unique binary barcode by designing probe sets that hybridize in multiple rounds of imaging. After $N$ rounds of hybridization/imaging/stripping, each gene is identified by its pattern of signals across rounds ($2^N$ possible barcodes). A typical experiment profiles 1,000–10,000 genes with 20–30 imaging rounds at ~100–200 nm spatial resolution.

**seqFISH+** (sequential FISH+) uses pseudocolor barcoding — probes are hybridized in sequential rounds with different fluorescent dye combinations — enabling simultaneous detection of ~10,000 genes per cell at subcellular resolution.

**10x Xenium** is a commercial implementation of FISH-based spatial transcriptomics offering pre-designed gene panels (typically 100–500 genes) with single-molecule sensitivity and ~200 nm resolution. It is increasingly used for clinical tissue analysis.

**STARmap** uses in situ sequencing (padlock probes + rolling circle amplification) to sequence mRNA barcodes directly within the tissue section, providing subcellular resolution.

The subcellular resolution of FISH-based methods opens a new level of analysis: **subcellular localization of mRNAs**. Some mRNAs are asymmetrically distributed within cells — *ACTB* mRNA localizes to the leading edge of migrating cells; *Nanos* mRNA localizes to the posterior pole of developing Drosophila embryos. These localizations are functionally important and completely invisible to any sequencing-based approach.

## Analysis: Spatially Variable Genes

A key analysis task is identifying **spatially variable genes (SVGs)** — genes whose expression varies significantly as a function of spatial position (beyond random noise). Tools like **SPARK-X**, **SpatialDE**, and **nnSVG** apply spatial statistical models (Gaussian processes or spatial variance component analysis) to identify SVGs. For example, a morphogen receptor might be expressed only in cells near the organizer, appearing as a gradient in the tissue.

The output of SVG analysis is a list of genes whose expression is spatially organized. Pathway enrichment of SVGs often reveals the signaling systems that organize tissue architecture — Wnt, Hedgehog, Notch — providing a genome-wide view of spatial gene regulation that was previously accessible only one gene at a time.

## Cell Type Deconvolution

Because Visium spots contain multiple cells, a critical analysis step is **deconvolution**: inferring the composition of cell types within each spot using a scRNA-seq reference. Two popular methods:

**RCTD** (Robust Cell Type Decomposition, Cable et al., 2021) fits a doublet model (two cell types per spot) or full mode (fractional contributions from multiple cell types). It uses a maximum likelihood framework with reference expression profiles from matched scRNA-seq data.

**SPOTlight** uses non-negative matrix factorization (NMF) seeded with scRNA-seq marker gene profiles to deconvolve spot compositions.

After deconvolution, each spot receives a probability vector across cell types, enabling visualization of cell type spatial distributions overlaid on the histology image. The resulting image — cell type composition painted onto the tissue section — is one of the most biologically informative plots in modern genomics. You can see, for example, that CD8+ T cells accumulate specifically at the tumor-stroma interface, or that oligodendrocyte progenitors are concentrated in specific cortical layers.

## Integrating scRNA-seq and Spatial Data

Spatial transcriptomics and scRNA-seq are complementary: scRNA-seq provides cell-type identity and subtype resolution; spatial transcriptomics provides location. Integration tools like **Seurat's anchor-based integration**, **Tangram**, and **Cell2location** transfer cell type labels or full expression profiles from scRNA-seq onto spatial data (or vice versa), combining the resolution advantages of each platform.

The integration strategy is clever: use the rich cell type annotation from scRNA-seq (which has the depth to distinguish subtle subtypes) to label the spots in spatial data (which has the position information). This gives you both: the fine cell type resolution of scRNA-seq and the spatial coordinates of the tissue.

## Why This Matters

Spatial transcriptomics is enabling a new field of **tissue architecture genomics** — understanding how the spatial organization of cell types and gene expression patterns underlies tissue function and disease. The first comprehensive spatial maps of human brain regions have revealed previously unknown cell type distributions and gene expression gradients. Spatial atlases of tumor microenvironments are revealing the precise spatial relationships between tumor cells, immune cells, and stroma that determine immunotherapy response. The principle that where a cell is located shapes what it does — elementary in histology, invisible in bulk or single-cell sequencing — is now, for the first time, directly accessible at genomic scale.
