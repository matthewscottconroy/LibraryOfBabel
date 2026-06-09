# Alignment Tools Overview

Sequence alignment encompasses a wide spectrum of computational tasks — pairwise protein comparison, database searching, short-read genome mapping, RNA-seq alignment, long-read mapping, and genome assembly — each requiring specialized tools. This reference consolidates the current best-in-class tools for each task, their key parameters, and practical guidance on tool selection.

A recurring failure mode in bioinformatics pipelines is using a tool that works for one alignment task in a context designed for another. BWA-MEM2 and STAR both align DNA sequences to a genome, but STAR is built for RNA-seq and models splice junctions; using BWA-MEM2 for RNA-seq will misalign or discard reads that cross exon-exon boundaries. The inverse — using STAR for DNA sequencing — wastes compute and may introduce alignment artifacts near known junction sites. Knowing not just how to run these tools but which one to reach for first is the practical goal of this section.

## Pairwise Alignment Tools

| Tool | Application | Algorithm | Notes |
|------|-------------|-----------|-------|
| EMBOSS Needle | Global pairwise alignment | Needleman-Wunsch | Web: https://www.ebi.ac.uk/Tools/psa/ |
| EMBOSS Water | Local pairwise alignment | Smith-Waterman | Best for finding conserved domains |
| BLAST+ (blastp/blastn) | Database search | Seed-and-extend | Default for homology search |
| DIAMOND | Protein database search | Compressed seed | 100× faster than BLAST for large DBs |
| LAST | Database search | Adaptive seeds | Better for divergent sequences |

## Multiple Sequence Alignment Tools

| Tool | Best Use | Speed | Accuracy |
|------|---------|-------|---------|
| MAFFT L-INS-i | < 500 seqs, phylogenetics | Slow | Best |
| MAFFT E-INS-i | Multi-domain proteins | Slow | Best for domain structures |
| MAFFT FFT-NS-2 | 500–10,000 seqs | Fast | Good |
| MUSCLE | General purpose | Medium | Very good |
| ClustalΩ | Large datasets | Fast | Good |
| T-Coffee | Difficult alignments | Very slow | Excellent |
| ProbCons | Probabilistic accuracy | Slow | Excellent |

```bash
# Quick reference for common MSA tasks

# Standard phylogenetics alignment
mafft --localpair --maxiterate 1000 --thread 8 input.fasta > msa.fasta

# Large-scale alignment
mafft --auto --thread 8 large_dataset.fasta > msa_large.fasta

# Post-alignment trimming
trimal -in msa.fasta -out msa_trimmed.fasta -automated1
```

## Read Alignment Tools by Application

### DNA Sequencing

```bash
# BWA-MEM2: standard WGS
bwa-mem2 mem -t 16 reference.fa R1.fastq.gz R2.fastq.gz \
    | samtools sort -o aligned.bam && samtools index aligned.bam

# Bowtie2: ChIP-seq, ATAC-seq
bowtie2 -x genome_idx -1 R1.fastq.gz -2 R2.fastq.gz \
    -p 8 --no-mixed --no-discordant | samtools sort -o aligned.bam
```

### RNA-seq

```bash
# STAR: standard RNA-seq
STAR --genomeDir star_idx/ --readFilesIn R1.fq.gz R2.fq.gz \
     --readFilesCommand zcat --outSAMtype BAM SortedByCoordinate \
     --runThreadN 8 --twopassMode Basic --outFileNamePrefix sample_

# HISAT2: lower memory alternative
hisat2 -x genome_idx -1 R1.fastq.gz -2 R2.fastq.gz \
       -p 8 --rna-strandness RF | samtools sort -o aligned.bam
```

### Bisulfite Sequencing (WGBS/RRBS)

```bash
# Bismark: bisulfite-aware alignment
bismark --genome bismark_idx/ -1 R1.fastq.gz -2 R2.fastq.gz \
        -p 4 --non_directional
bismark_methylation_extractor --paired-end --comprehensive \
    --CX_context *_bismark_bt2_pe.bam
```

### Long Reads

```bash
# minimap2: Nanopore/PacBio
minimap2 -ax map-ont -t 16 reference.fa reads.fastq.gz \
    | samtools sort -o aligned.bam && samtools index aligned.bam

# PacBio HiFi
minimap2 -ax map-hifi reference.fa hifi.fastq.gz \
    | samtools sort -o hifi_aligned.bam
```

## Quantification Tools (RNA-seq)

| Tool | Method | Outputs | Notes |
|------|--------|---------|-------|
| featureCounts | Alignment-based counting | Gene counts matrix | Fast; part of Subread package |
| HTSeq-count | Alignment-based counting | Gene counts | Slower but widely used |
| Salmon | Pseudo-alignment | Transcript/gene TPM + counts | GC bias correction |
| kallisto | k-mer equivalence | Transcript TPM + counts | Fastest |

```bash
# featureCounts (from a sorted BAM)
featureCounts -a annotation.gtf -o counts.txt \
    -p -s 2 -T 8 sample1.bam sample2.bam sample3.bam

# Salmon (quasi-mapping, no BAM needed)
salmon quant -i salmon_index -l A \
    -1 R1.fastq.gz -2 R2.fastq.gz \
    --validateMappings -p 8 -o quant_output/
```

## Post-Alignment Processing Tools

| Tool | Function |
|------|---------|
| samtools | Sort, index, flagstat, mpileup, merge |
| Picard/GATK | MarkDuplicates, BQSR, metrics |
| deepTools | Coverage (bamCoverage), normalization, correlation |
| bedtools | Interval operations on BED/BAM |

```bash
# Standard post-alignment pipeline
samtools sort -o sorted.bam aligned.bam
samtools index sorted.bam
picard MarkDuplicates I=sorted.bam O=deduped.bam M=dup_metrics.txt REMOVE_DUPLICATES=false
samtools flagstat deduped.bam

# Generate normalized coverage track (bigWig)
bamCoverage -b deduped.bam -o coverage.bw --normalizeUsing RPKM \
    --binSize 10 --smoothLength 50 -p 8

# Check insert size distribution (paired-end)
picard CollectInsertSizeMetrics I=deduped.bam O=insert_sizes.txt \
    H=insert_size_histogram.pdf
```

## Profile HMM Tools

```bash
# HMMER3
hmmbuild profile.hmm msa.fasta
hmmsearch --tblout results.tbl profile.hmm database.fasta
hmmscan --domtblout domains.tbl Pfam-A.hmm query.fasta

# HHblits (profile-profile)
hhblits -i query.fasta -d uniclust30 -o query.hhr -n 3 -cpu 8
```

## Quality Control Summary

Always run QC at multiple stages:

```bash
# Raw reads
fastqc *.fastq.gz -o qc/
multiqc qc/ -o multiqc/

# Trimmed reads (re-run FastQC)
fastp -i R1.fastq.gz -I R2.fastq.gz -o R1_trim.fastq.gz -O R2_trim.fastq.gz
fastqc R1_trim.fastq.gz R2_trim.fastq.gz -o qc_trimmed/

# Alignment QC
samtools flagstat aligned.bam
samtools stats aligned.bam | grep "^SN"
```

## Why This Matters

Knowing which tool to use for a given task is as important as understanding the underlying algorithms. Tool choice affects runtime (hours vs. minutes for large datasets), accuracy (alignment rate, false positive rate), and compatibility with downstream tools. A common failure mode in bioinformatics is using the wrong aligner for the data type — for example, using BWA-MEM2 for RNA-seq (which cannot handle splicing), or using STAR for ChIP-seq (unnecessary overhead, potential issues with multi-mappers). This reference provides the conceptual map for making correct choices across the full spectrum of sequence alignment tasks.
