# BED, GFF3, and GTF Formats

Genomic feature annotation requires formats that specify the position, strand, and identity of features — genes, exons, peaks, binding sites — on a reference genome. Three formats dominate: BED for interval data, GFF3 for hierarchical biological annotations, and GTF for RNA-seq quantification. Understanding their coordinate systems and feature hierarchies is essential for genome analysis.

Here is a situation you will encounter regularly: you have a list of transcription factor binding peaks from a ChIP-seq experiment (BED format) and you want to know which of those peaks overlap with gene promoters (defined in a GTF annotation file). The answer is simple to compute with bedtools — but only if you understand that BED uses 0-based half-open coordinates while GTF uses 1-based closed coordinates. Convert one to the other incorrectly, and every overlap calculation is off by one base. That single off-by-one error can shift hundreds or thousands of peaks into the wrong gene assignment. This is not an edge case; it is one of the most common systematic errors in genomics analysis, and it flows directly from not understanding these formats deeply.

## BED Format

**BED (Browser Extensible Data)** represents genomic intervals. It is intentionally flexible: only three fields are required, and additional optional fields provide richer annotation.

### Coordinate System

BED uses **0-based, half-open** coordinates:
- The first base of a chromosome is position 0
- An interval [start, end) includes position `start` but excludes position `end`
- `chr1  0  100` represents the first 100 bases of chromosome 1

This contrasts with 1-based coordinates (VCF, GFF3, SAM POS) where position 1 is the first base.

**Why it matters**: converting between coordinate systems is a frequent source of off-by-one errors.

| Format | Coordinate system | Example (first 100 bp of chr1) |
|--------|-------------------|-------------------------------|
| BED | 0-based, half-open | `chr1  0  100` |
| GFF3/GTF | 1-based, closed | `chr1  1  100` |
| VCF | 1-based | position 1 = first base |
| SAM POS | 1-based | position 1 = first base |

The 0-based half-open convention has mathematical advantages: the length of an interval is simply `end - start`, and adjacent intervals can be expressed without overlap — `0-100` and `100-200` share no bases. The 1-based closed convention is more natural for biologists reading genomic coordinates in papers. The fact that these two systems coexist is a historical accident, not a principled design decision, and it demands vigilance.

### BED Fields

| Col | Name | Required? | Description |
|-----|------|-----------|-------------|
| 1 | chrom | yes | Chromosome |
| 2 | chromStart | yes | 0-based start |
| 3 | chromEnd | yes | End (not included) |
| 4 | name | optional | Feature name |
| 5 | score | optional | 0–1000 (e.g., peak score) |
| 6 | strand | optional | `+`, `-`, or `.` |
| 7 | thickStart | optional | Coding start (CDS start) |
| 8 | thickEnd | optional | Coding end |
| 9 | itemRgb | optional | Display color as R,G,B |
| 10 | blockCount | optional | Number of sub-blocks (exons) |
| 11 | blockSizes | optional | Comma-separated exon sizes |
| 12 | blockStarts | optional | Comma-separated exon starts (relative to chromStart) |

BED3 (3 fields) is used for simple interval files (peak lists, repeat annotations). BED6 adds name, score, and strand. BED12 encodes full transcript structures with exons.

### bedtools: BED File Operations

```bash
# Intersect: find ChIP-seq peaks overlapping gene promoters
bedtools intersect -a chipseq_peaks.bed -b promoters.bed -wa -wb > overlap.bed

# Merge: combine overlapping intervals
bedtools merge -i sorted_peaks.bed > merged_peaks.bed

# Sort (required before merge)
bedtools sort -i peaks.bed > peaks_sorted.bed

# Get genomic sequences at BED intervals
bedtools getfasta -fi genome.fa -bed regions.bed -fo regions.fasta

# Compute coverage of BAM over BED regions
bedtools coverage -a genes.bed -b aligned.bam > coverage.txt

# Subtract: remove peaks in blacklisted regions
bedtools subtract -a peaks.bed -b blacklist.bed > filtered_peaks.bed
```

## GFF3 Format

**GFF3 (General Feature Format version 3)** is the standard for genome annotation. It represents a hierarchical model of biological features: gene → mRNA → exon, CDS, UTR.

### GFF3 Structure

Each line has 9 tab-separated fields:

```
seqid  source  type  start  end  score  strand  phase  attributes
```

| Field | Description |
|-------|-------------|
| seqid | Chromosome or sequence name |
| source | Source of annotation (e.g., `AUGUSTUS`, `Ensembl`) |
| type | Feature type (gene, mRNA, exon, CDS, UTR, etc.) |
| start | 1-based start position |
| end | 1-based end (inclusive) |
| score | Numeric score or `.` |
| strand | `+`, `-`, or `.` |
| phase | CDS reading frame: 0, 1, or 2 (`.` for non-CDS) |
| attributes | Key-value pairs; `ID=`, `Parent=`, `Name=` are special |

### GFF3 Hierarchy

The `ID` and `Parent` attributes create the feature hierarchy:

```
chr1  Ensembl  gene   10000  20000  .  +  .  ID=gene001;Name=BRCA1
chr1  Ensembl  mRNA   10000  20000  .  +  .  ID=mrna001;Parent=gene001
chr1  Ensembl  exon   10000  10200  .  +  .  Parent=mrna001
chr1  Ensembl  CDS    10000  10200  .  +  0  Parent=mrna001
chr1  Ensembl  exon   15000  15500  .  +  .  Parent=mrna001
chr1  Ensembl  CDS    15000  15500  .  +  1  Parent=mrna001
chr1  Ensembl  exon   19800  20000  .  +  .  Parent=mrna001
chr1  Ensembl  CDS    19800  19950  .  +  0  Parent=mrna001
chr1  Ensembl  three_prime_UTR  19951  20000  .  +  .  Parent=mrna001
```

This structure supports multiple mRNA isoforms per gene (each mRNA has `Parent=gene001`), alternative splicing, and nested features.

## GTF Format

**GTF (Gene Transfer Format)**, also called GFF2, is a simplified version used primarily for RNA-seq analysis. It is less flexible than GFF3 but is the format used by STAR, featureCounts, and other RNA-seq tools.

### GTF Structure

```
chr1  Ensembl  gene        10000  20000  .  +  .  gene_id "ENSG001"; gene_name "BRCA1";
chr1  Ensembl  transcript  10000  20000  .  +  .  gene_id "ENSG001"; transcript_id "ENST001";
chr1  Ensembl  exon        10000  10200  .  +  .  gene_id "ENSG001"; transcript_id "ENST001"; exon_number "1";
chr1  Ensembl  CDS         10000  10200  .  +  0  gene_id "ENSG001"; transcript_id "ENST001";
```

GTF differs from GFF3:
- No `ID`/`Parent` hierarchy; relationships encoded by matching `gene_id`/`transcript_id` values
- Attribute format: `key "value";` (quoted values, semicolon-separated)
- Only `gene` and `transcript` as feature types above exon level
- Used by: STAR, HISAT2 (for junction annotation), featureCounts, HTSeq

```bash
# Count reads per gene using featureCounts
featureCounts -a annotation.gtf -o counts.txt -T 4 \
    -p -s 2 aligned.bam

# Extract gene-level features from GTF
awk '$3 == "gene" {print $1"\t"($4-1)"\t"$5"\t"$9"\t.\t"$7}' \
    annotation.gtf > genes.bed
```

## Coordinate Conversion

```python
# BED to GFF3 coordinate conversion
def bed_to_gff3_coords(bed_start, bed_end):
    """BED is 0-based half-open; GFF3 is 1-based closed"""
    gff3_start = bed_start + 1
    gff3_end = bed_end
    return gff3_start, gff3_end

# GFF3 to BED
def gff3_to_bed_coords(gff3_start, gff3_end):
    bed_start = gff3_start - 1
    bed_end = gff3_end
    return bed_start, bed_end
```

## Why This Matters

BED, GFF3, and GTF are the annotation languages of genome biology. Every ChIP-seq peak, every RNA-seq quantification, every variant effect prediction references genome coordinates in one of these formats. Coordinate system errors — confusing 0-based and 1-based positions — produce systematic off-by-one errors that silently corrupt analyses. A gene wrongly located by 1 base may assign variants to the wrong gene or miss splice-site variants. Mastering these formats and the tools that manipulate them (bedtools, gffutils, pybedtools) is a fundamental practical competency.
