# SAM/BAM Format

**SAM (Sequence Alignment/Map)** is the standard format for storing read alignments. After reads are mapped to a reference genome, the result is stored as a SAM file — a tab-delimited text format encoding the position, orientation, and quality of each alignment. **BAM** is the binary equivalent, compressed using BGZF (a blocked gzip variant), reducing file size by 10–15× while enabling random access via an index file.

If FASTQ is the starting line of a sequencing analysis, SAM/BAM is the central hub. Every downstream step — variant calling, RNA expression quantification, chromatin accessibility analysis, DNA methylation analysis — reads from a BAM file. Understanding the format is therefore not just useful for parsing purposes; it is essential for correctly filtering reads, interpreting alignment statistics, and debugging the inevitable problems that arise when an analysis produces unexpected results.

## SAM File Structure

A SAM file has two sections: a **header** (lines beginning with `@`) and **alignment records** (one per read).

### Header Section

```
@HD  VN:1.6  SO:coordinate
@SQ  SN:chr1  LN:248956422
@SQ  SN:chr2  LN:242193529
@RG  ID:sample1  SM:Patient_A  LB:lib1  PL:ILLUMINA
@PG  ID:bwa  PN:bwa-mem2  VN:2.2.1  CL:bwa-mem2 mem reference.fa reads.fastq
```

- `@HD`: header; `SO` = sort order (unsorted, queryname, coordinate)
- `@SQ`: sequence dictionary; one line per reference chromosome/contig; `LN` = length
- `@RG`: read group; associates reads with a sample, library, and instrument run
- `@PG`: program; records the software used to generate the file

### Alignment Records

Each alignment is one tab-separated line with 11 mandatory fields:

| Field | Name | Example | Description |
|-------|------|---------|-------------|
| 1 | QNAME | `read001` | Read name |
| 2 | FLAG | `99` | Bitwise flag |
| 3 | RNAME | `chr1` | Reference chromosome |
| 4 | POS | `1000` | 1-based leftmost mapping position |
| 5 | MAPQ | `60` | Mapping quality |
| 6 | CIGAR | `75M` | CIGAR string describing alignment |
| 7 | RNEXT | `=` | Chromosome of mate (`=` if same chrom) |
| 8 | PNEXT | `1250` | Position of mate |
| 9 | TLEN | `325` | Template length (insert size); negative for reverse |
| 10 | SEQ | `ACGT...` | Query sequence |
| 11 | QUAL | `IIII...` | Base quality (Phred+33) |

Optional tags follow as `TAG:TYPE:VALUE` triplets.

## The FLAG Field

The FLAG is a 16-bit integer where each bit encodes a property of the read:

| Bit | Value | Meaning |
|-----|-------|---------|
| 0x1 | 1 | Read is paired |
| 0x2 | 2 | Read is in a proper pair |
| 0x4 | 4 | Read is unmapped |
| 0x8 | 8 | Mate is unmapped |
| 0x10 | 16 | Read maps to reverse strand |
| 0x20 | 32 | Mate maps to reverse strand |
| 0x40 | 64 | Read is first in pair (R1) |
| 0x80 | 128 | Read is second in pair (R2) |
| 0x100 | 256 | Not primary alignment |
| 0x200 | 512 | Fails quality filters |
| 0x400 | 1024 | PCR or optical duplicate |
| 0x800 | 2048 | Supplementary alignment (chimeric) |

**Example**: FLAG = 99 = 64 + 32 + 2 + 1 = first in pair + mate maps to reverse strand + proper pair + paired. FLAG = 147 = 128 + 16 + 2 + 1 = second in pair + maps to reverse strand + proper pair + paired.

```bash
# Decode a FLAG value
samtools flags 99
# output: 0x63  99  PAIRED,PROPER_PAIR,MATE_REVERSE,READ1
```

The bitwise encoding is worth understanding at an intuitive level. FLAG = 4 means the read is unmapped; FLAG = 1796 (1024 + 512 + 256 + 4) means the read is unmapped AND a duplicate AND fails quality filters AND has a non-primary alignment. Any filtering operation on a BAM file should be thought of in terms of these bits: you want to include or exclude reads based on their properties, and the FLAG field is how you ask about those properties.

## CIGAR Strings

The **CIGAR string** describes how the read aligns to the reference — which positions match, where insertions/deletions occur, and where the read was clipped.

| Operation | Code | Consumes query? | Consumes reference? |
|-----------|------|-----------------|---------------------|
| Match/mismatch | M | yes | yes |
| Insertion | I | yes | no |
| Deletion | D | no | yes |
| Intron (N) | N | no | yes |
| Soft clip | S | yes | no |
| Hard clip | H | no | no |
| Padding | P | no | no |
| Sequence match | = | yes | yes |
| Sequence mismatch | X | yes | yes |

**Examples**:

`75M` — 75 bases match the reference (standard short read alignment)

`50M2D25M` — 50 matches, then 2-base deletion in the read, then 25 more matches; read is 75 bp, covers 77 reference bases

`10S65M` — first 10 bases soft-clipped (adapter? low quality?), then 65 bases aligned

`45M1500N30M` — 45 match, intron of 1500 bases (RNA-seq splice junction), 30 match

`20M3I52M` — 20 match, 3-base insertion in the read, 52 match; read is 75 bp, covers 72 reference bases

CIGAR strings are the compact encoding of everything a pairwise sequence alignment communicates: matches, mismatches, gaps, and clipping. When you see an unexpectedly complex CIGAR like `15S30M5D20M8I7M10S`, you are reading the alignment's full history — soft-clipping at both ends from adapter contamination or low quality, a deletion relative to the reference, and an insertion relative to the reference. Each operation tells you something about what happened to this read's source fragment in evolution or in the sequencing process.

## Working with SAM/BAM: samtools

```bash
# Convert SAM to BAM and sort
samtools view -bS alignments.sam | samtools sort -o alignments.sorted.bam

# Index BAM (required for random access)
samtools index alignments.sorted.bam

# View alignments at a specific region
samtools view alignments.sorted.bam chr1:1000-2000

# Alignment statistics
samtools flagstat alignments.sorted.bam
# Example output:
# 50000000 + 0 in total
# 48500000 + 0 mapped (97.00%)
# 50000000 + 0 paired in sequencing
# 25000000 + 0 read1
# 25000000 + 0 read2
# 48000000 + 0 properly paired (96.00%)
# 1500000 + 0 singletons (3.00%)

# Mark PCR duplicates
picard MarkDuplicates I=aligned.bam O=deduped.bam M=metrics.txt

# CRAM: reference-compressed BAM (smaller)
samtools view -C -T reference.fa -o aligned.cram aligned.bam
```

## Mapping Quality (MAPQ)

MAPQ is defined as:
$$MAPQ = -10 \log_{10}(P_\text{mapping error})$$

- MAPQ 60: uniquely mapped (probability of mapping to wrong location < $10^{-6}$; BWA-MEM2 convention)
- MAPQ 0: unmapped or maps equally well to multiple locations (multi-mapper)
- MAPQ 255: not available / not meaningful

Multi-mapping reads (MAPQ 0) are routinely filtered out before downstream analysis:
```bash
samtools view -q 20 -b alignments.bam > uniquely_mapped.bam
```

## Why This Matters

SAM/BAM is the universal intermediate in all sequencing analysis pipelines. After aligning reads, every downstream step — variant calling, peak calling, transcript quantification, ChIP-seq analysis — reads from BAM files. Understanding FLAG values allows correct filtering of paired reads, duplicates, and supplementary alignments. CIGAR strings are essential for interpreting insertions, deletions, and splicing patterns. MAPQ thresholds directly control the balance between sensitivity (including multi-mappers) and specificity (using only uniquely mapped reads). Proficiency with samtools is a non-negotiable practical skill for any bioinformatician working with sequencing data.
