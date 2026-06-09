# Chromosome Structure

Here is a packaging problem: take two meters of DNA and fit it into a sphere six micrometers in diameter — a packing ratio of 8,000:1. And don't just compact it randomly. The compaction must be selective: some regions need to be kept accessible for transcription at all times, others must be permanently silenced, and others must be rapidly accessible or silenced in response to developmental or environmental signals. The solution biology evolved is chromatin — a nucleoprotein complex whose organization, far from being a passive packing solution, is itself a major layer of gene regulation.

Eukaryotic genomes are not free DNA — they are organized into chromatin, a highly compacted nucleoprotein complex that must simultaneously protect DNA from damage, enable replication and transcription, and compact ~2 m of human DNA into a nucleus ~6 µm in diameter. Chromatin structure is a major regulatory layer, and its disruption is a hallmark of cancer. Understanding chromosome organization from nucleosome to chromosome territory is prerequisite knowledge for interpreting Hi-C, ATAC-seq, and ChIP-seq data.

## The Nucleosome: First Level of Compaction

The **nucleosome** is the fundamental unit of chromatin. It consists of ~147 bp of DNA wrapped 1.65 turns around an **octamer** of histone proteins: 2 copies each of H2A, H2B, H3, and H4. The histones are small (~100–130 aa), highly basic (positively charged Lys and Arg residues contact the negatively charged DNA), and among the most conserved proteins in eukaryotes.

The octamer forms by first assembling a (H3-H4)₂ tetramer, then adding two H2A-H2B dimers. The ~146 bp of DNA make 14 contacts with the histone core. Between nucleosomes, **linker DNA** of 20–80 bp is bound by **linker histone H1**, which seals the DNA entry/exit from the octamer.

Nucleosome positioning is not random — **nucleosome positioning sequences** (e.g., poly(dA:dT) tracts are depleted) and **chromatin remodelers** (SWI/SNF, ISWI, CHD, INO80 families) set the positions of nucleosomes across the genome. Promoters of active genes typically have a **nucleosome-free region (NFR)** flanked by well-positioned +1 and −1 nucleosomes.

The diameter of a nucleosome is ~11 nm; arrays of nucleosomes form a "beads-on-a-string" fiber ~11 nm wide.

## Higher-Order Chromatin Organization

The 11 nm fiber is further compacted. The classical model of the **30 nm fiber** (zigzag or solenoid arrangement of nucleosomes) was long proposed as the next level, but in vivo evidence suggests the 30 nm fiber is rare or absent in most of the nucleus; chromatin is more irregular than classical models implied.

**Chromatin loops**: ~10–1000 kb loops are extruded by **cohesin** and anchored at CTCF binding sites. Loop extrusion brings enhancers and promoters into proximity, enabling transcriptional activation. Loss of CTCF boundaries allows enhancers to contact inappropriate promoters (oncogene activation in cancer).

**Topologically Associating Domains (TADs)**: Genomic regions of ~100 kb–3 Mb that preferentially interact internally. TAD boundaries are enriched for CTCF, cohesin, and active transcription. TAD organization is partially conserved across mammals. Disruption of TAD boundaries by structural variants can cause developmental diseases by misdirecting enhancer activity.

**A/B compartments**: Hi-C data at lower resolution reveals two compartment types:
- **A compartment**: gene-dense, accessible, transcriptionally active; corresponds roughly to euchromatin
- **B compartment**: gene-sparse, compact, transcriptionally silent; corresponds roughly to heterochromatin

**Chromosome territories**: Each chromosome occupies a distinct, largely non-overlapping volume in the nucleus. Gene-rich chromosomes tend to be in the nuclear interior; gene-poor chromosomes at the nuclear periphery. The nuclear lamina (Lamin A/B/C) at the inner nuclear membrane associates with **lamina-associated domains (LADs)** — large heterochromatic regions (~0.1–10 Mb) containing silenced genes.

## Heterochromatin vs. Euchromatin

The distinction between compact, silent **heterochromatin** and open, active **euchromatin** is central to gene regulation:

| Feature | Euchromatin | Heterochromatin |
|---|---|---|
| Compaction | Open, accessible | Compact, inaccessible |
| Transcription | Active | Mostly silent |
| Replication timing | Early S phase | Late S phase |
| Histone marks | H3K4me3, H3K27ac, H3K36me3 | H3K9me3, H3K27me3 |
| Reader proteins | — | HP1 (H3K9me3), Polycomb (H3K27me3) |

**Constitutive heterochromatin** is permanently condensed — centromeres, telomeres, and pericentromeric repeats. Rich in H3K9me3 and HP1. Maintains genomic stability by suppressing transposable element transcription.

**Facultative heterochromatin** is silenced in a cell-type-specific or developmental manner. The inactive X chromosome (Xi) in female mammals is the paradigmatic example: the lncRNA **Xist** coats the Xi and recruits Polycomb repressive complexes (PRC2: deposits H3K27me3; PRC1: compacts chromatin, ubiquitinates H2AK119).

## Centromeres and Telomeres

**Centromeres** are the chromosomal attachment sites for kinetochore proteins and spindle microtubules during mitosis/meiosis. In humans, centromeres consist of arrays of **alpha-satellite repeat** (~171 bp unit) spanning 0.5–5 Mb. The centromere-specific histone variant **CENP-A** (a histone H3 variant) marks centromere identity — CENP-A, not sequence per se, is the epigenetic determinant of centromere location. Errors in centromere function lead to chromosome missegregation and aneuploidy.

**Telomeres** (TTAGGG repeats, 5–15 kb in humans) protect chromosome ends from degradation and end-to-end fusion. Telomere-binding **shelterin** complex (TRF1, TRF2, POT1, TPP1, TIN2, RAP1) distinguishes telomeres from DSBs, suppressing ATM/ATR kinase activation. Loss of shelterin or telomere shortening activates DNA damage checkpoint, causing senescence or apoptosis.

## Why This Matters for Computational Biology

Chromosome structure is increasingly measured at genome scale. **Hi-C** generates contact frequency matrices — the computational challenge of identifying TADs, loops, and compartments from these matrices is a rich area of methods development (tools: HiCPro, Juicer, Domainator). **ATAC-seq** (Assay for Transposase-Accessible Chromatin with sequencing) identifies open chromatin genome-wide; peak calling and motif enrichment analysis reveal active regulatory elements. **ChIP-seq** for histone marks and CTCF identifies chromatin state, which can be combined into **chromatin state annotations** (ChromHMM, Segway). Disruption of TAD structure by large structural variants is increasingly recognized as a pathogenic mechanism in cancer and developmental disease — computational structural variant callers must account for this. Understanding nucleosome positioning informs the design of synthetic gene circuits in yeast and mammalian cells where chromatin accessibility is a control point.
