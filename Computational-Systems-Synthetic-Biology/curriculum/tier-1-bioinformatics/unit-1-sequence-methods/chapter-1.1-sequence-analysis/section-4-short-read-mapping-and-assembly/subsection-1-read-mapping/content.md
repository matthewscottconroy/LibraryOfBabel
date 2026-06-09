# Read Mapping

After sequencing produces millions to billions of short reads (50–300 bp), each read must be located within a reference genome (billions of bases). This is the **read mapping** (or read alignment) problem: find the reference location where each read best fits, accounting for sequencing errors, genetic variants, and the repetitive nature of genomes.

Think about what we are actually doing. We have taken a living cell, extracted its DNA, sheared it into millions of fragments, sequenced the fragments with some per-base error rate, and we now want to know where each fragment came from in the original 3.2 billion base genome. The process is analogous to shredding a massive encyclopedia, then trying to reassemble it by finding where each scrap fits. And we need to do this for 500 million scraps, in an hour or two, on a standard server. That this is possible at all is a consequence of clever data structures — primarily the Burrows-Wheeler Transform — that are worth understanding conceptually even if you will never implement them.

## The Computational Challenge

Consider mapping 500 million reads of 150 bp each to the 3.2 Gb human genome. Naïve Smith-Waterman alignment for each read against the entire genome would require $500 \times 10^6 \times 3.2 \times 10^9 = 1.6 \times 10^{18}$ character comparisons — computationally impossible in any reasonable time. Efficient read mapping requires fundamentally different data structures.

## The Burrows-Wheeler Transform and FM-Index

Modern read mappers for short reads are built on the **Burrows-Wheeler Transform (BWT)** and the **FM-index**. These allow the entire reference genome to be searched in time proportional to the query length, not the reference length.

**BWT** is a reversible string transformation. For a string $T$ of length $n$:
1. Form all $n$ cyclic rotations of $T$
2. Sort them lexicographically
3. The BWT $B$ is the last column of this sorted matrix

The BWT clusters similar characters together, enabling efficient compression and search. The FM-index combines the BWT with auxiliary tables to enable **backward search**: finding all positions where a pattern $P$ occurs in $T$ in $O(|P|)$ time.

**Practical impact**: BWA and Bowtie2 can map 50 million reads to the human genome in under 1 hour on a standard server using this index, where Smith-Waterman would take years.

## Key Short-Read Aligners

### BWA-MEM2

**BWA-MEM2** (Li, 2019) is the standard aligner for DNA sequencing:

- Uses BWT-FM index for seeding; Smith-Waterman extension for alignment
- Handles reads 70–10,000 bp (optimized for 100–300 bp Illumina reads)
- Supports split-read alignment for detecting structural variants
- Outputs SAM/BAM format with full CIGAR strings

```bash
# Build BWA index (done once per reference)
bwa-mem2 index reference.fa

# Align paired-end reads
bwa-mem2 mem -t 8 reference.fa reads_R1.fastq.gz reads_R2.fastq.gz \
    | samtools sort -o aligned.bam

# Mark duplicates and index
picard MarkDuplicates I=aligned.bam O=deduped.bam M=metrics.txt
samtools index deduped.bam
```

### Bowtie2

**Bowtie2** is fast and memory-efficient, making it preferred for:
- ChIP-seq (shorter reads, no splice junctions needed)
- ATAC-seq
- Amplicon sequencing

```bash
# Build index
bowtie2-build reference.fa reference_index

# Align single-end
bowtie2 -x reference_index -U reads.fastq -S aligned.sam -p 8
```

### STAR (Spliced Transcripts Alignment to a Reference)

**STAR** is the standard aligner for RNA-seq, designed for splice-aware alignment:

- Builds a genome index including annotated splice junctions
- Two-pass mode: first pass discovers novel junctions, second pass aligns using them
- Handles reads crossing exon-exon boundaries (critical for RNA-seq)
- Requires significant RAM (~30 GB for human)

```bash
# Build STAR genome index
STAR --runMode genomeGenerate \
     --genomeDir star_index/ \
     --genomeFastaFiles genome.fa \
     --sjdbGTFfile annotation.gtf \
     --runThreadN 8

# Two-pass alignment
STAR --runMode alignReads \
     --genomeDir star_index/ \
     --readFilesIn reads_R1.fastq.gz reads_R2.fastq.gz \
     --readFilesCommand zcat \
     --outSAMtype BAM SortedByCoordinate \
     --outFileNamePrefix sample1_ \
     --runThreadN 8 \
     --twopassMode Basic \
     --outSAMattributes NH HI NM MD AS
```

### minimap2

**minimap2** handles long reads (Oxford Nanopore, PacBio) using a different algorithm (minimizer-based seeding):

```bash
# Long-read DNA alignment
minimap2 -ax map-ont reference.fa reads.fastq.gz \
    | samtools sort -o aligned.bam

# RNA-seq long reads (splice-aware)
minimap2 -ax splice reference.fa rnaseq_reads.fastq.gz \
    | samtools sort -o rna_aligned.bam
```

## Handling Repetitive Regions

Repetitive elements (LINEs, SINEs, satellite DNA, segmental duplications) make up ~50% of the human genome. Reads from these regions map to multiple locations with equal score — **multi-mapping reads**.

Strategies for multi-mappers:
1. **Discard**: set MAPQ threshold (e.g., MAPQ ≥ 20). Cleanest but loses information in repetitive regions
2. **Report all locations**: `samtools view -F 256` includes secondary alignments
3. **Fractional assignment**: assign weight 1/n to each of n equally scoring locations (used by some quantification tools)

```bash
# Keep only uniquely mapped reads (MAPQ >= 20)
samtools view -q 20 -b aligned.bam > unique.bam

# Check mapping statistics
samtools flagstat aligned.bam
```

## Post-Alignment Processing

The standard post-alignment workflow:

```bash
# 1. Sort by coordinate
samtools sort -o sorted.bam aligned.bam

# 2. Mark PCR duplicates (optical + PCR amplification)
gatk MarkDuplicates -I sorted.bam -O deduped.bam -M dup_metrics.txt

# 3. Index
samtools index deduped.bam

# 4. Base Quality Score Recalibration (BQSR) — for GATK variant calling
gatk BaseRecalibrator \
    -I deduped.bam -R reference.fa \
    --known-sites dbsnp.vcf.gz \
    -O recal.table

gatk ApplyBQSR \
    -I deduped.bam -R reference.fa \
    --bqsr-recal-file recal.table \
    -O recalibrated.bam
```

## Why This Matters

Read mapping is the bridge between raw sequencing data and biological interpretation. Every downstream analysis — variant calling, differential expression, chromatin accessibility, DNA methylation — requires correctly mapped reads as input. Aligner choice matters: STAR vs. HISAT2 differences affect how splice junctions are discovered; BWA-MEM2 vs. Bowtie2 differences affect multi-mapper handling in ChIP-seq. MAPQ thresholds control the sensitivity-specificity tradeoff. Understanding read mapping at a mechanistic level — the BWT index, CIGAR string generation, MAPQ calculation — enables correct interpretation of alignment statistics and informed troubleshooting when alignment rates are unexpectedly low.
