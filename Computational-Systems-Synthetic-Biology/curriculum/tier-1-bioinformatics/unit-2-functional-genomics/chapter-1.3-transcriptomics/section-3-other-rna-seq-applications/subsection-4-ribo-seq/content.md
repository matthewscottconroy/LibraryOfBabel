# Ribosome Profiling (Ribo-seq)

In 2008, Jonathan Weissman's lab at UCSF set out to answer a question that should have been easy: which mRNAs is the cell actually translating? RNA abundance — easily measurable — was routinely used as a proxy for protein production. But everyone suspected the correlation was imperfect. There had to be post-transcriptional regulation, translational control, a gap between what the cell transcribes and what it actually makes into protein. The question was how large that gap was and where it mattered most.

The answer came in 2009, when Ingolia et al. published ribosome profiling in Science. The idea was elegantly simple: ribosomes are physical objects that protect the mRNA beneath them from digestion. Freeze the ribosomes in place with a drug, expose the whole cellular extract to an enzyme that destroys unprotected RNA, and sequence what is left. What remains are 28–30 nucleotide fragments of mRNA — one for every translating ribosome, at the exact codon the ribosome occupied when you froze it. For the first time, it was possible to measure translation at genome-wide scale with single-codon resolution.

**Ribosome profiling (Ribo-seq)** provides direct genome-wide measurement of translation by sequencing the mRNA fragments protected by translating ribosomes, revealing which transcripts are being translated, at what rate, and at which codons.

## The Experimental Principle

The core idea is elegantly simple: ribosomes physically protect approximately 28–30 nucleotides of mRNA from nuclease digestion. If you add a translation inhibitor (cycloheximide, to freeze ribosomes in place), lyse cells, and then treat the extract with **RNase I** (which degrades unprotected RNA), only the fragments protected by ribosomes survive — these are called **ribosome-protected fragments (RPFs)** or "ribosome footprints."

Experimental workflow:
1. Add cycloheximide to cells to halt translation elongation.
2. Lyse cells and sediment ribosomes via **sucrose gradient** or ultracentrifugation.
3. Digest unprotected RNA with RNase I.
4. Extract RNA from ribosome pellet, size-select for 28–30 nt fragments.
5. Ligate adapters, reverse transcribe, PCR amplify, and sequence.
6. Simultaneously collect a matched RNA-seq sample (without footprinting) for normalization.

Every step requires careful optimization. Cycloheximide must be added quickly to avoid artifacts — if elongating ribosomes continue moving for even a few seconds before freezing, their positions will not reflect steady-state occupancy. The RNase digestion must be complete but not excessive. The sucrose gradient must separate ribosome-protected fragments from degradation products. Getting these conditions right produces data of stunning clarity; getting them wrong produces noise that looks superficially like signal.

## Ribosome Site Offset: A/P/E Sites

A translating ribosome has three positions for tRNA accommodation: the **A site** (aminoacyl, where new amino acids enter), **P site** (peptidyl, where peptide bond formation occurs), and **E site** (exit, where deacylated tRNA leaves). The ribosome footprint (28–30 nt) protects sequence centered on the P-site codon, with the footprint beginning ~12–13 nt 5' of the P-site codon (this is the **A-site offset**, approximately 12 nt from the 5' end of the read to the A-site codon).

When visualizing Ribo-seq data, the 5' end of each read is offset by +12 nt to assign reads to the A-site codon. This allows precise single-nucleotide resolution of ribosome position. The precision is extraordinary: ribosome profiling can tell you which codon a ribosome is sitting on, averaged across the millions of ribosomes in a cell. No other method approaches this resolution.

## Metagene Analysis

A **metagene analysis** aligns all RPFs relative to translation start and stop codons and averages the signal, revealing systematic patterns. Key features of a high-quality Ribo-seq experiment:

- A strong 3-nt (triplet) periodicity in footprint density — ribosomes move exactly one codon at a time, producing a sawtooth pattern of every-3rd-position enrichment.
- Footprint density drops sharply at the stop codon.
- A ramp of increased ribosome density near the 5' end of ORFs (initiation is rate-limiting).

Poor triplet periodicity indicates sub-optimal footprinting (incomplete RNase digestion, cycloheximide artifacts, or library quality issues). The triplet periodicity is the quality control metric that cannot be faked: it reflects the fundamental mechanics of translation elongation, and if your data does not show it, something went wrong experimentally.

## Translation Efficiency

**Translation efficiency (TE)** measures how efficiently a given mRNA is translated:

$$\text{TE}_g = \frac{\text{Ribo-seq RPM}_g}{\text{RNA-seq RPM}_g}$$

where RPM = reads per million mapped reads for gene $g$. Genes with high TE are efficiently translated per mRNA molecule; low TE genes are transcribed but poorly translated. Changes in TE between conditions reveal post-transcriptional regulation that would be invisible in RNA-seq alone. For example, during the integrated stress response, mRNAs with upstream open reading frames (uORFs) are preferentially translated even as global translation is suppressed.

It turns out that translation efficiency varies enormously across the transcriptome — much more than most people expected before Ribo-seq made it measurable. The correlation between mRNA abundance and translation rate across genes is only moderate, around 0.6–0.7. This means that roughly 40% of the variation in protein production across genes is explained by translational regulation rather than transcript abundance. The cell has a complete second layer of gene regulation operating at the ribosome.

## Key Applications

**Upstream ORFs (uORFs)**: Many mRNAs contain short ORFs in the 5' UTR, upstream of the main coding sequence. Ribo-seq reveals ribosome occupancy on these uORFs. uORFs can regulate main ORF translation by causing ribosomes to terminate and re-initiate, and many uORFs are differentially "toggled" during stress responses (e.g., ATF4 mRNA translation is dramatically upregulated during stress through uORF bypass).

**Non-canonical translation**: Ribo-seq has revealed widespread translation of lncRNAs (producing short peptides), 3' UTRs (3' UTR-encoded sORFs), and alternative reading frames. Some of these non-canonical translation products have functional roles and potential immunological relevance (as neoantigens).

**Codon occupancy for bottleneck detection**: At high ribosome density on a specific codon, the ribosome is pausing — either because the cognate tRNA is rare, the amino acid is limiting, or downstream mRNA secondary structure stalls translocation. This **codon occupancy analysis** identifies rate-limiting steps in translation elongation, relevant for codon optimization in synthetic biology (Chapter 2) and understanding proteotoxic stress.

**Differential translation**: By comparing Ribo-seq RPM between conditions while normalizing to RNA-seq, differential TE can be detected using tools like **anota2seq** or **Babel**, revealing translational regulation independent of transcription.

## Why This Matters

Ribosome profiling bridges the gap between the transcriptome and proteome, revealing the layer of translational control that determines which mRNAs are productively used. It has overturned the assumption that mRNA abundance reliably predicts protein production and revealed the pervasive role of translational regulation in cell biology and disease. The discovery that uORFs are widespread regulatory elements, that lncRNAs encode peptides that can be immunogenic, and that codon usage affects translational efficiency and protein folding in ways that matter for disease — all of this came from Ribo-seq. For synthetic biology, the practical implications are immediate: designing an expression system without understanding translational regulation means optimizing only half the problem. Ribo-seq provides the other half.
