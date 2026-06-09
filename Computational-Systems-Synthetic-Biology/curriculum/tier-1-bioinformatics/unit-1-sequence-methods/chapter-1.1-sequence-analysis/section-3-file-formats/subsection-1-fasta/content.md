# FASTA Format

FASTA is the simplest and most universally used sequence file format in bioinformatics. Nearly every sequence analysis tool reads and writes FASTA. Its minimal structure makes it human-readable, easy to parse programmatically, and compatible across platforms and programming languages.

The name "FASTA" comes from the alignment program FASTA (pronounced "fast-A"), developed by William Pearson and David Lipman in 1988 as a precursor to BLAST. The format was designed around the program's needs: a sequence identifier followed by sequence data, nothing more. What started as a format for a single tool became the universal language of sequence data — a testament to the power of simplicity.

You will encounter FASTA constantly in computational biology. Reference genomes are distributed as FASTA files. Protein databases like UniProt are FASTA files. The output of sequencing that has been assembled or polished is FASTA. Every time you start a new analysis from genomic sequence data, you are almost certainly starting from a FASTA file. The format is so ubiquitous that understanding it is not really "learning a file format" — it is learning the alphabet of the field.

## Format Specification

A FASTA file consists of one or more **sequence records**. Each record contains:

1. A **header line** beginning with `>` (greater-than sign), followed immediately by the sequence identifier and optionally a description
2. One or more **sequence lines** containing the sequence data

```
>sequence_id optional description text
ATGCGTAGCTAGCTAGCTAGCTAGCGATCGATCGATCG
ATCGATCGATCGATCGATCGATCG
>second_sequence another description
MVLSPADKTNVKAAWGKVGAHAGEYGAEALERMFLSFP
TTKTYIFSSIKHVLHRDQSFLQKFLAAIASMPASYGN
```

Key rules:
- The `>` must be the first character of the header line
- Sequence lines may be any length (typically 60–80 characters per line by convention)
- Blank lines between records are usually tolerated by tools but should be avoided
- Sequence identifiers should not contain spaces (everything after the first space is treated as description by some parsers)

## Sequence Alphabet

For DNA/RNA sequences, FASTA uses:
- Standard bases: `ACGT` (DNA) or `ACGU` (RNA)
- IUPAC ambiguity codes: `N` (any), `R` (A or G), `Y` (C or T), `W` (A or T), `S` (G or C), `K` (G or T), `M` (A or C), `B` (not A), `D` (not C), `H` (not G), `V` (not T)
- Lowercase letters are valid (often used for soft-masked repetitive regions)

For protein sequences, FASTA uses the standard single-letter amino acid code: `ACDEFGHIKLMNPQRSTVWY`, plus `X` for unknown amino acid, `*` for stop codon, and `-` for gap.

## Parsing FASTA in Python

```python
from Bio import SeqIO

# Read all sequences from a FASTA file
records = list(SeqIO.parse("sequences.fasta", "fasta"))
for record in records:
    print(f"ID: {record.id}")
    print(f"Length: {len(record.seq)}")
    print(f"Sequence: {record.seq[:50]}...")

# Memory-efficient iteration (large files)
for record in SeqIO.parse("genome.fasta", "fasta"):
    gc_content = (record.seq.count('G') + record.seq.count('C')) / len(record.seq)
    print(f"{record.id}: GC = {gc_content:.3f}")

# Write sequences to FASTA
with open("output.fasta", "w") as handle:
    SeqIO.write(records, handle, "fasta")
```

For large files, command-line tools are faster:

```bash
# Count sequences in a FASTA file
grep -c "^>" genome.fasta

# Extract sequence by ID using samtools faidx
samtools faidx genome.fa chr1:1000-2000

# Index a FASTA file (required for random access)
samtools faidx genome.fasta

# Split multi-FASTA into individual files
awk '/^>/{filename=substr($0,2) ".fasta"} {print > filename}' seqs.fasta
```

## Multi-FASTA vs. Single-FASTA

A **multi-FASTA** file contains multiple sequence records. This is the standard for:
- Genome assemblies (one record per chromosome/scaffold/contig)
- Proteomes (one record per protein)
- Transcript collections
- Multiple sequence alignments (when stored in FASTA format; gaps represented as `-`)

A **single-FASTA** file contains exactly one record. Used for:
- Individual query sequences for BLAST
- Single chromosome assemblies

## Soft and Hard Masking

Repetitive elements in genomes are often **masked** in FASTA files:
- **Soft masking**: repeats represented in lowercase (`atcgatcg...`). Most aligners ignore soft-masked regions by default.
- **Hard masking**: repeats replaced with `N` characters. More aggressive; tools cannot recover masked sequence.

```bash
# Check masking status
grep -c "[a-z]" genome.fasta  # soft-masked positions exist if > 0
```

RepeatMasker generates soft-masked FASTA from an unmasked assembly.

Masking matters because repetitive elements — transposons, satellite DNA, segmental duplications — make up a large fraction of many genomes (nearly 50% of the human genome). Reads from these regions map to multiple locations and can confound alignments, variant calling, and gene expression analysis. Soft masking is the standard approach: the sequence is still there if you need it, but most tools are configured to ignore it by default.

## FASTA in Alignment Outputs

When FASTA stores a **multiple sequence alignment**, gap characters (`-`) are included:

```
>Human_HBA
MVLSPADKTNVKAAWGKVGAHAGEYGAEALERMFLSFPTTK
>Mouse_HBA
MVLSGEDKSNIKAAWGKIGGHGAEYGAEALERMFASFPTTK
>Chicken_HBA
MVLSAADKNNVKGIFTKIAGHA-EEYGAETLERMFTTYPPTK
```

This format (FASTA alignment, `.fa` or `.aln` extension) is read directly by phylogenetic tools like IQ-TREE and RAxML.

## Common Pitfalls

**Non-standard line endings**: Windows line endings (`\r\n`) can cause parsing failures in some tools. Convert with `dos2unix`.

**Duplicate sequence IDs**: Many tools assume unique identifiers. Check with:
```bash
grep "^>" sequences.fasta | sort | uniq -d
```

**Very long single-line sequences**: Some parsers have buffer limits. Most modern tools handle this, but it is best practice to wrap at 60–80 characters per line.

**Hidden characters**: Pasting sequences from web browsers or PDFs can introduce non-ASCII characters that corrupt parsing. Always inspect suspicious sequences.

## Why This Matters

FASTA is the universal currency of sequence data. Every genome in NCBI, every protein in UniProt, every transcript in Ensembl is stored and distributed in FASTA format. Understanding FASTA — including its edge cases — is prerequisite to working with any sequence data. Parsing errors due to FASTA format issues are among the most common sources of silent failures in bioinformatics pipelines, where a tool silently processes zero sequences or misidentifies sequence boundaries. A pipeline that runs without error but produces results based on a malformed FASTA file is arguably worse than a pipeline that crashes — the former gives you wrong answers with no warning.
