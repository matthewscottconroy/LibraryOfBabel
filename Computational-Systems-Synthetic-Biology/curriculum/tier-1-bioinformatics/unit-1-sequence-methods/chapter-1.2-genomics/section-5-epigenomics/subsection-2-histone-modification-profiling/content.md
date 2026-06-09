# Histone Modification Profiling

The idea that chemical modifications to histones could encode regulatory information — a "histone code" readable by the cell's transcriptional machinery — was proposed by Strahl and Allis in 2000. At the time, individual modifications had been linked to specific transcriptional states by biochemical experiments, but the idea that these marks constituted a systematic code, with defined "writers," "readers," and "erasers," was new. It was also, it turned out, largely correct.

What genome-wide profiling revealed is both richer and more complex than the original histone code hypothesis anticipated. Different modifications at different positions carry distinct functional meanings, with combinatorial logic that goes beyond simple on/off switches. H3K4me3 at a promoter marks active transcription; H3K27me3 at the same promoter marks Polycomb-mediated repression. The same residue — lysine 27 of histone H3 — can be either acetylated (activating) or trimethylated (repressive), and the enzymes that install these marks (EZH2 for trimethylation, p300/CBP for acetylation) are among the most frequently mutated in cancer. Understanding the histone code has therefore become essential for understanding cancer biology, developmental biology, and the mechanisms of drug resistance.

Histone modifications are post-translational modifications (acetylation, methylation, phosphorylation, ubiquitination) at specific residues on histone tails. They create the **histone code** that regulates chromatin compaction, gene expression, and DNA repair. ChIP-seq (Chromatin Immunoprecipitation followed by sequencing) and its successors (CUT&RUN, CUT&TAG) map these modifications genome-wide.

## The Histone Modification Language

Different modifications at different positions carry distinct functional meanings:

| Mark | Location | Association |
|------|----------|-------------|
| H3K4me3 | H3, Lysine 4, trimethylation | Active promoters |
| H3K4me1 | H3, Lysine 4, monomethylation | Active/poised enhancers |
| H3K27ac | H3, Lysine 27, acetylation | Active enhancers and promoters |
| H3K27me3 | H3, Lysine 27, trimethylation | Polycomb-repressed genes |
| H3K9me3 | H3, Lysine 9, trimethylation | Constitutive heterochromatin |
| H3K36me3 | H3, Lysine 36, trimethylation | Transcribed gene bodies |
| H3K4me2 | H3, Lysine 4, dimethylation | Promoters and enhancers |
| H2AK119ub1 | H2A, K119, ubiquitination | Polycomb repression |

Reading histone modifications: A chromatin region marked with H3K4me3 and H3K27ac (bivalent active promoter); H3K4me1 without H3K27ac (poised enhancer); H3K27me3 (Polycomb-repressed). The ENCODE project has used these patterns to define chromatin state annotations genome-wide.

The bivalent chromatin state is particularly important for developmental biology. In embryonic stem cells, many developmental genes carry both H3K4me3 (activating) and H3K27me3 (repressive) marks simultaneously — a "bivalent" configuration that keeps the gene poised but not expressed. Upon differentiation, one mark dominates: the gene is either activated (H3K27me3 removed, H3K4me3 retained) or silenced (H3K4me3 removed, H3K27me3 retained). Bivalency is thought to allow developmental genes to respond rapidly to differentiation signals without having to wait for de novo establishment of activating marks. It is a molecular hair-trigger.

## ChIP-seq Protocol

**ChIP-seq** requires:
1. Crosslinking DNA and histones (formaldehyde, ~1%)
2. Sonication or MNase treatment to fragment chromatin to ~200 bp
3. Immunoprecipitation with a histone-modification-specific antibody
4. Reversal of crosslinks, proteinase K treatment
5. Library preparation and sequencing

**Critical consideration**: antibody quality is paramount. A poor antibody precipitates non-specific chromatin, inflating false positive peaks. Validated antibodies (ENCODE Antibody Registry) should be used.

### ChIP-seq Analysis Pipeline

```bash
# 1. Alignment (with spike-in normalization option)
bowtie2 -x genome_index -1 R1.fastq.gz -2 R2.fastq.gz \
    -p 8 | samtools sort -o aligned.bam
samtools index aligned.bam

# 2. Remove duplicates
picard MarkDuplicates I=aligned.bam O=deduped.bam M=dup_metrics.txt

# 3. Peak calling (narrow peaks: H3K4me3, H3K27ac, TF)
macs2 callpeak \
    -t ChIP.bam -c Input.bam \  # Always use Input control!
    -f BAMPE -n sample \
    --outdir macs2_output/ \
    -q 0.05 -g hs  # genome size

# 4. Broad peaks (H3K27me3, H3K9me3, H3K36me3)
macs2 callpeak \
    -t ChIP.bam -c Input.bam \
    -f BAMPE -n sample_broad \
    --outdir macs2_broad/ \
    --broad --broad-cutoff 0.1

# 5. Coverage track for visualization
bamCoverage -b deduped.bam -o coverage.bw \
    --binSize 10 --normalizeUsing RPKM \
    --ignoreDuplicates --effectiveGenomeSize 2913022398 -p 8
```

**Input control**: essential. The Input (sonicated chromatin before immunoprecipitation) corrects for:
- GC content bias in sonication/sequencing
- Mappability issues at repetitive regions
- Open chromatin accessibility bias

Without input, all accessible regions appear as ChIP-seq peaks.

The importance of the input control cannot be overstated. Chromatin is not uniformly sonicated — some regions (GC-rich, early-replicating, nucleosome-depleted) are consistently over-represented in sonicated chromatin libraries. Without subtracting the input, these regions will appear as false-positive peaks in any ChIP-seq experiment, regardless of what antibody you used. A "promoter peak" in an H3K9me3 ChIP-seq without input normalization might simply reflect the high accessibility of promoters, not genuine H3K9me3 enrichment. The input is not a trivial control — it is what makes the difference between identifying real histone marks and identifying sonication artifacts.

## CUT&RUN and CUT&TAG: Next-Generation Methods

**CUT&RUN (Cleavage Under Targets and Release Using Nuclease)** and **CUT&TAG (Cleavage Under Targets and Tagmentation)** avoid formaldehyde crosslinking and require far fewer cells:

CUT&RUN protocol:
1. Bind antibody to native chromatin in permeabilized cells/nuclei
2. Add Protein A-MNase fusion; it binds the antibody
3. Activate MNase with Ca²⁺ → cuts chromatin near the antibody-bound region
4. Released fragments are sequenced

CUT&TAG: replaces MNase with Tn5 transposase → direct sequencing adapter insertion at antibody-bound regions.

Advantages vs. ChIP-seq:
- Fewer cells (1,000–100,000 vs. millions)
- Lower background (targeted cutting vs. immunoprecipitation)
- No crosslinking artifacts
- Suitable for FFPE tissue

```bash
# CUT&RUN analysis uses SEACR peak caller (optimized for low background)
# Normalize to IgG control
SEACR.sh CUT_RUN.bedgraph IgG_control.bedgraph non stringent output_peaks
```

The conceptual shift from ChIP-seq to CUT&RUN is elegant. ChIP-seq works by collecting everything that is bound to the antibody-decorated chromatin, which includes a large amount of non-specific background. CUT&RUN flips the logic: instead of pulling chromatin out, it sends a molecular scissors to the antibody location and cuts the local chromatin, releasing only the genomic DNA immediately adjacent to the target. The signal-to-noise ratio improves dramatically because only local DNA is released, not bulk chromatin. This is why CUT&RUN works with 1,000 cells when ChIP-seq requires millions.

## ChromHMM: Chromatin State Segmentation

**ChromHMM** uses a multivariate HMM to learn chromatin states from multiple histone marks simultaneously:

```bash
# Learn chromatin states (5 or 15 states are common)
java -jar ChromHMM.jar LearnModel \
    -p 8 binarized_data/ chromhmm_output/ 15 hg38

# Outputs: 15 chromatin state annotations across the genome
# State 1: Active TSS (H3K4me3 + H3K27ac + H3K4me1)
# State 2: Flanking TSS
# State 5: Strong enhancer (H3K4me1 + H3K27ac)
# State 8: Transcription (H3K36me3)
# State 12: Polycomb repressed (H3K27me3)
```

ENCODE's 200 cell-type chromatin state maps reveal how different cell types use the same genome differently — the same DNA sequence is in active chromatin in one cell type and constitutive heterochromatin in another.

ChromHMM is, in effect, a dimensionality reduction tool for the histone code. Rather than analyzing each histone mark separately, it finds the combinatorial patterns that recur across the genome and assigns each genomic position to one of a small number of chromatin states. These states have biological interpretations — active promoter, strong enhancer, transcribed region, repressed — that emerge from the data without being specified in advance. The result is a compact, interpretable annotation of the regulatory genome that can be compared across cell types, species, and disease states.

## Differential Histone Modification Analysis

```r
library(DiffBind)
library(ChIPseeker)

# Load ChIP-seq peaks for multiple samples
db <- dba(sampleSheet="chip_samples.csv")
db <- dba.count(db)
db <- dba.contrast(db, categories=DBA_CONDITION)
db <- dba.analyze(db)
diff_regions <- dba.report(db, th=0.05)

# Annotate with genomic features
library(TxDb.Hsapiens.UCSC.hg38.knownGene)
txdb <- TxDb.Hsapiens.UCSC.hg38.knownGene
annot <- annotatePeak(diff_regions, tssRegion=c(-3000, 3000), TxDb=txdb)
plotAnnoPie(annot)
```

## Why This Matters

Histone modifications encode the regulatory history of a cell — which genes are active, which are poised for activation, and which are stably repressed. ChIP-seq and CUT&RUN have revealed that enhancers, not just promoters, are the primary regulatory elements that differentiate cell types; that Polycomb repression maintains developmental gene silencing; and that cancer epigenomes are globally disrupted relative to normal tissue. For drug development, histone-modifying enzymes (EZH2, HDAC, KDM) are targets for cancer therapy, and understanding their genome-wide activity patterns guides mechanistic studies. The transition from ChIP-seq to CUT&RUN/CUT&TAG is expanding histone profiling to rare cell types including primary patient-derived samples, enabling clinical epigenomics at scale.

The pharmaceutical implications are immediate and practical. EZH2, the enzyme that writes H3K27me3, is overactive in many lymphomas and drives Polycomb-mediated silencing of tumor suppressor genes. Tazemetostat, an EZH2 inhibitor approved for relapsed/refractory follicular lymphoma, works precisely by removing the repressive H3K27me3 marks from genes that need to be active for normal cell behavior. Understanding which genes are affected — which requires ChIP-seq or CUT&RUN — is how the mechanism of drug action was established, and it is how resistance mutations are being identified. Epigenomic profiling is not just a research tool; it is a clinical tool for understanding and overcoming drug resistance in cancer.
