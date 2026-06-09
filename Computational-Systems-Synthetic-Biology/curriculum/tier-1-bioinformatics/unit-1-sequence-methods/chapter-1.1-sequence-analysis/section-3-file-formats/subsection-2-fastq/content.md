# FASTQ Format

FASTQ is the standard file format for raw sequencing reads. It extends FASTA by adding per-base quality scores — a critical piece of information that quantifies the confidence of each base call. All modern short-read sequencers (Illumina, MGI) produce FASTQ output, and the format is the starting point for virtually every sequencing data analysis pipeline.

The addition of quality scores might seem like a minor technical detail, but it is conceptually important. A sequencing read is not a sequence — it is a sequence together with a confidence estimate at each position. When a sequencer calls an A at position 37, it is not certain that the base is A. It might be a G that was read incorrectly, or a base that could not be resolved confidently. The Phred quality score quantifies that uncertainty, and how you handle uncertainty in each base propagates through every downstream analysis: variant calling thresholds, read filtering criteria, error correction, and assembly quality all depend on these scores. Treating FASTQ as "just FASTA with extra lines" misses the central role that quality information plays in modern sequencing analysis.

## Format Specification

Each FASTQ record is exactly **four lines**:

```
@SRR001666.1 071112_SLXA-EAS1_s_7:5:1:817:345 length=36
GGGTGATGGCCGCTGCCGATGGCGTCAAATCCCACC
+
IIIIIIIIIIIIIIIIIIIIIIIIIIIIII9IG9IC
```

**Line 1**: `@` followed by the sequence identifier and optional description  
**Line 2**: The nucleotide sequence  
**Line 3**: `+` optionally followed by the identifier again (usually just `+`)  
**Line 4**: The quality string — must be the same length as the sequence

## Phred Quality Scores

The quality string encodes **Phred quality scores** as ASCII characters. Each character's quality score is:

$$Q = -10 \log_{10}(P_\text{error})$$

where $P_\text{error}$ is the probability that the base call is incorrect. The ASCII encoding:

$$\text{ASCII}(\text{char}) = Q + 33$$

(the `+33` offset is the Phred+33 or Sanger encoding, used by all modern Illumina data)

| ASCII char | ASCII value | Phred Q | P(error) | Accuracy |
|-----------|------------|---------|----------|---------|
| `!` | 33 | 0 | 1.0 | 0% |
| `5` | 53 | 20 | 0.01 | 99% |
| `?` | 63 | 30 | 0.001 | 99.9% |
| `I` | 73 | 40 | 0.0001 | 99.99% |
| `K` | 75 | 42 | 0.00006 | 99.994% |

**Q30** is the standard quality threshold: a base with Q ≥ 30 has < 0.1% probability of being wrong. Modern Illumina runs typically achieve 80–90% of bases at Q ≥ 30.

The logarithmic scaling of Phred scores is worth absorbing intuitively. Moving from Q20 to Q30 is not a 50% improvement in accuracy — it is a 10-fold reduction in error probability, from 1 in 100 to 1 in 1000. Moving from Q30 to Q40 reduces error by another factor of 10. When you set a quality filter threshold, you are making a decision about how much error you are willing to tolerate, and the logarithmic scale means that small differences in Q score correspond to large differences in error probability.

## Parsing and Quality Assessment

```python
from Bio import SeqIO

# Read FASTQ and compute per-read average quality
for record in SeqIO.parse("reads.fastq", "fastq"):
    quals = record.letter_annotations["phred_quality"]
    avg_q = sum(quals) / len(quals)
    print(f"{record.id}: avg Q = {avg_q:.1f}, length = {len(record.seq)}")

# Filter reads by quality
high_quality = []
for record in SeqIO.parse("reads.fastq", "fastq"):
    quals = record.letter_annotations["phred_quality"]
    if sum(quals) / len(quals) >= 30:
        high_quality.append(record)

SeqIO.write(high_quality, "filtered.fastq", "fastq")
```

## FastQC Quality Reports

**FastQC** generates comprehensive quality reports from FASTQ files:

```bash
fastqc reads.fastq -o qc_output/ -t 4
```

Key modules in a FastQC report:

| Module | What to look for |
|--------|-----------------|
| Per-base sequence quality | Q < 30 at 3' end: normal (trim if severe) |
| Per-base N content | Spikes indicate chemistry problems |
| Sequence length distribution | Uniform for most protocols |
| GC content | Should match organism; bimodal = contamination |
| Adapter content | Signals need for trimming |
| Overrepresented sequences | rRNA contamination, adapter dimers |

**MultiQC** aggregates FastQC reports across many samples into a single interactive report:

```bash
multiqc qc_output/ -o multiqc_report/
```

Running FastQC and MultiQC before any downstream analysis is not optional — it is the first step of every sequencing pipeline. Quality problems discovered after mapping and variant calling cost far more time than those caught at the raw read stage. Particular patterns to watch for: per-base quality that drops sharply after position 100 (common in lower-quality runs), adapter content above 5% (requires trimming), bimodal GC distribution (may indicate contamination with another organism).

## Read Trimming

Low-quality bases at read ends and adapter sequences must be removed before alignment. **fastp** is the modern standard:

```bash
# Basic adapter trimming and quality filtering
fastp -i reads_R1.fastq.gz -I reads_R2.fastq.gz \
      -o trimmed_R1.fastq.gz -O trimmed_R2.fastq.gz \
      --detect_adapter_for_pe \
      -q 20 \
      --length_required 50 \
      --thread 4 \
      --html fastp_report.html

# Single-end
fastp -i reads.fastq.gz -o trimmed.fastq.gz \
      -q 20 --length_required 50
```

fastp auto-detects adapters, trims low-quality tails, and produces a QC report — all in a single fast pass.

## Paired-End FASTQ

Most modern sequencing is **paired-end**: both ends of each DNA fragment are sequenced, producing two reads per fragment. Paired reads are stored in two synchronized FASTQ files:

- `reads_R1.fastq.gz`: forward reads
- `reads_R2.fastq.gz`: reverse reads

The $n$-th read in R1 is paired with the $n$-th read in R2. Pairing must be preserved during all processing steps, as aligners use mate information for read placement and insert size estimation.

```bash
# Verify pairing is intact (read counts should match)
wc -l reads_R1.fastq reads_R2.fastq
```

## Compression and Storage

Raw FASTQ files are large: a 30× human whole-genome sequencing experiment produces ~100 GB of uncompressed FASTQ. **gzip compression** reduces this by 3–4×:

```bash
# FASTQ files are almost always gzip-compressed
ls *.fastq.gz

# All tools accept .gz input directly
fastqc reads.fastq.gz
fastp -i reads_R1.fastq.gz -I reads_R2.fastq.gz ...

# View compressed FASTQ
zcat reads.fastq.gz | head -8
```

**CRAM** format (discussed in SAM/BAM section) provides additional compression by storing only differences from a reference.

## Why This Matters

FASTQ is the entry point for all high-throughput sequencing analyses. Understanding its structure — especially Phred quality encoding — is essential for making correct decisions about quality filtering thresholds, understanding FastQC reports, and diagnosing alignment problems. A Q20 threshold at a base means 1% error probability; a Q30 threshold means 0.1%. These differences propagate into variant calling sensitivity, transcript quantification accuracy, and assembly contiguity. Every bioinformatics pipeline begins with FASTQ, and quality assessment at this stage is the first line of defense against analytical errors.
