# Chromatin Accessibility

Here is a puzzle. Every cell in your body contains essentially the same DNA sequence — the same 3.2 billion base pairs, encoding the same ~20,000 protein-coding genes. Yet a liver cell and a neuron look and behave completely differently. The liver cell produces albumin and cytochrome P450 enzymes; the neuron produces voltage-gated ion channels and neurotransmitter receptors. Both cells have the genes for both proteins. What determines which genes are expressed?

A large part of the answer lies not in the sequence itself but in the physical state of the chromatin. The DNA in a human cell is wrapped around histone octamers to form nucleosomes, which pack into higher-order structures that can be open (accessible to transcription factors and the RNA polymerase machinery) or closed (compacted, inaccessible). In a liver cell, the albumin gene promoter is in open chromatin; in a neuron, it is packed away. The regulatory information is encoded not just in the DNA sequence but in the three-dimensional physical state of the chromatin — and this state differs across cell types even when the underlying sequence is identical.

**Chromatin accessibility** refers to the degree to which genomic DNA is physically accessible to regulatory factors. Open chromatin regions are depleted of nucleosomes, enabling transcription factors, co-activators, and the transcription machinery to bind. These open regions mark active regulatory elements — promoters, enhancers, insulators, and CTCF binding sites. ATAC-seq is the dominant method for genome-wide chromatin accessibility profiling.

## Biological Significance

The human genome contains 3.2 billion base pairs, but only ~1–3% is in open chromatin in any given cell type. These open regions are cell-type-specific and define the active regulatory landscape:

- **Active promoters**: open chromatin at transcription start sites
- **Active enhancers**: distal open regions enriched for H3K27ac
- **CTCF binding sites**: insulators and loop anchors
- **Transcription factor binding footprints**: small protected regions within open chromatin where TF binding occludes the Tn5 transposase

The 1–3% figure is striking. The genome is mostly inaccessible in any given cell. The accessible fraction — the regulatory genome — is small, dynamic, and cell-type-specific. This is why GWAS variants in non-coding regions are so informative when combined with ATAC-seq data: a variant in an ATAC-seq peak in the relevant cell type is far more likely to be functionally important than one in closed chromatin. ENCODE has generated ATAC-seq maps for hundreds of cell types, and overlaying GWAS hits with these maps has revealed the likely causal cell types for dozens of complex diseases.

## ATAC-seq: Assay for Transposase-Accessible Chromatin

**ATAC-seq (Assay for Transposase-Accessible Chromatin using Sequencing)** uses the hyperactive Tn5 transposase to simultaneously cut and ligate sequencing adapters at accessible chromatin regions. The key advantage: requires only 500–50,000 cells (compared to millions for ChIP-seq), making it feasible for rare cell types and clinical samples.

### Tn5 Transposase Mechanism

Tn5 inserts with a 9-bp duplication; in ATAC-seq, it inserts sequencing adapters at positions where chromatin is accessible (nucleosome-free). Two Tn5 insertions flanking an accessible region produce a short fragment that is efficiently sequenced:

- **Subnucleosomal fragments** (< 100 bp): in nucleosome-free regions (direct accessibility signal)
- **Mononucleosomal fragments** (~200 bp): fragments wrapping one nucleosome
- **Dinucleosomal fragments** (~400 bp): two nucleosomes

The fragment size distribution reflects nucleosome positioning.

### ATAC-seq Protocol Notes

- Standard: 50,000 cells; minimal ATAC: 500 cells; single-nucleus (snATAC-seq): single cells
- Immediately after isolation, cells are lysed, nuclei isolated, and Tn5 added — rapid processing prevents chromatin state changes
- **Mitochondrial contamination**: mitochondrial DNA is naked (no nucleosomes), so Tn5 cuts it extensively. Filtering out mitochondrial reads (which can be > 50% of raw reads) is essential.

The mitochondrial contamination issue is one of those practical details that every ATAC-seq practitioner learns the hard way. Mitochondria have high copy number (hundreds to thousands per cell), their DNA has no nucleosomes, and Tn5 therefore cuts it at essentially every position. In a protocol where nuclei have not been cleanly separated from cytoplasmic mitochondria, the majority of your sequencing reads will map to chrM — wasting enormous sequencing capacity. The solution is rigorous nuclear isolation before Tn5 treatment, with quality control checkpoints to verify that the mitochondrial fraction is below 5%.

### ATAC-seq Analysis Pipeline

```bash
# Step 1: Quality control and adapter trimming
fastp -i R1.fastq.gz -I R2.fastq.gz \
      -o R1_trim.fastq.gz -O R2_trim.fastq.gz \
      --adapter_sequence CTGTCTCTTATACACATCT \
      --detect_adapter_for_pe -q 20 --thread 4

# Step 2: Alignment (very short fragments require careful settings)
bowtie2 -x genome_index \
        -1 R1_trim.fastq.gz -2 R2_trim.fastq.gz \
        --very-sensitive -X 2000 \  # Allow large fragments
        -p 8 | samtools sort -o aligned.bam
samtools index aligned.bam

# Step 3: Remove mitochondrial reads, duplicates
samtools view -b aligned.bam chr1 chr2 ... chrX > nuclear.bam  # exclude chrM
picard MarkDuplicates I=nuclear.bam O=deduped.bam M=dup_metrics.txt REMOVE_DUPLICATES=true

# Step 4: Tn5 insertion site correction
# Tn5 inserts at +4 (forward) and -5 (reverse) relative to read start
# Correct shift to identify true cut sites
samtools view -h deduped.bam | \
    awk 'BEGIN{OFS="\t"} /^@/{print} !/^@/{
        if($2==0){$4=$4+4}
        if($2==16){$4=$4-5}
        print}' | samtools view -b > shifted.bam

# Step 5: Peak calling
macs2 callpeak \
    -t deduped.bam \
    -f BAMPE \
    -n sample_name \
    --outdir peaks/ \
    --shift -75 --extsize 150 \
    --nomodel -B --SPMR \
    --keep-dup all -q 0.05

# Step 6: Blacklist filtering
bedtools subtract -a peaks/sample_name_peaks.narrowPeak \
    -b ENCFF356LFX_blacklist.bed > peaks_filtered.bed
```

### Nucleosome Positioning Analysis

```python
import pyBigWig
import numpy as np
import matplotlib.pyplot as plt

# Fragment size distribution reveals nucleosome pattern
# Load ATAC-seq BAM and extract fragment sizes
import pysam

bam = pysam.AlignmentFile("deduped.bam", "rb")
fragment_sizes = []

for read in bam.fetch():
    if read.is_proper_pair and read.template_length > 0:
        fragment_sizes.append(abs(read.template_length))

plt.figure(figsize=(10, 5))
plt.hist(fragment_sizes, bins=range(0, 1000), density=True, color='steelblue')
plt.xlabel('Fragment Size (bp)')
plt.ylabel('Density')
plt.axvline(x=200, color='red', linestyle='--', label='Mononucleosome (~200bp)')
plt.axvline(x=400, color='orange', linestyle='--', label='Dinucleosome (~400bp)')
plt.title('ATAC-seq Fragment Size Distribution')
plt.legend()
```

The nucleosomal ladder in the fragment size distribution is more than a quality control metric — it is a direct readout of nucleosome organization at accessible loci. A high fraction of sub-nucleosomal (< 100 bp) fragments indicates good enrichment for nucleosome-free regions. The 200 bp mononucleosomal peak reflects Tn5 insertions flanking a single nucleosome. A clean ladder (sub-nucleosomal, mono-, di-, tri-nucleosomal peaks) indicates that the Tn5 reaction was well-controlled and the nuclear preparation was of high quality.

### Transcription Factor Footprinting

Within ATAC-seq peaks, individual TF binding sites create **footprints**: small protected regions where the TF occludes Tn5 insertions. Flanking regions show elevated Tn5 insertions (high accessibility), creating a characteristic dip pattern:

```bash
# TOBIAS: TF footprinting from ATAC-seq
TOBIAS ATACorrect --bam shifted.bam --genome genome.fa --peaks peaks.bed \
    --outdir TOBIAS_correct/ --cores 8

TOBIAS FootprintScores --signal TOBIAS_correct/sample_corrected.bw \
    --regions peaks.bed --output footprints.bw --cores 8

TOBIAS BINDetect --motifs motif_database.jaspar --signals footprints.bw \
    --genome genome.fa --peaks peaks.bed \
    --outdir TOBIAS_bindetect/ --cores 8
```

## Differential Accessibility Analysis

```r
library(DiffBind)

# Create DiffBind sample sheet
db <- dba(sampleSheet="ATAC_samples.csv")
db <- dba.count(db)
db <- dba.normalize(db)
db <- dba.contrast(db, categories=DBA_CONDITION)
db <- dba.analyze(db)

# Get differentially accessible regions
da_regions <- dba.report(db, th=0.05)
```

## Why This Matters

Chromatin accessibility is the epigenomic correlate of regulatory activity. ATAC-seq peaks in disease-relevant cell types are enriched for GWAS variants, revealing the regulatory mechanisms underlying genetic associations. Single-cell ATAC-seq (scATAC-seq) maps chromatin accessibility heterogeneity within tissues, identifying cell-type-specific regulatory programs. For drug discovery, ATAC-seq identifies accessible chromatin at transcription factor binding sites — enabling discovery of cell-type-specific regulatory vulnerabilities. Understanding ATAC-seq analysis — from Tn5 shift correction to peak calling to footprinting — is essential for interpreting the epigenomic basis of gene regulation.

The single-cell dimension deserves emphasis as the current frontier. Bulk ATAC-seq averages chromatin accessibility across all cells in a sample, obscuring cellular heterogeneity within tissues. Single-cell ATAC-seq (scATAC-seq) performs the same measurement in individual cells, producing a matrix of accessibility profiles across thousands of cells simultaneously. This reveals the regulatory identity of individual cells — you can cluster cells by their accessibility profile and find cell types, cell states, and even rare transitional states that would be invisible in bulk measurements. The ENCODE consortium's effort to map scATAC-seq profiles across human tissues is building a regulatory atlas of human cell identity, one cell at a time.
