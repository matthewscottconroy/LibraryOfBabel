# Bioinformatics File Formats

Consider what happens when you run an RNA-seq experiment. Raw reads come off the sequencer as FASTQ files. You trim adapters and low-quality bases, still in FASTQ. You align to a reference genome and get a SAM file. You compress and sort that SAM into a BAM file, index it to a BAI file. You call variants and get a VCF file, compress it to BCF, index it with tabix. You count reads per gene to get a count matrix in TSV format. You annotate genes with coordinates from a GTF file. You visualize coverage as a BigWig. Every step in this pipeline speaks a different language — and every format was designed for a specific reason, with specific tradeoffs. Not knowing these formats means not being able to debug the step where things go wrong. And something always goes wrong.

Bioinformatics has accumulated dozens of file formats, each optimized for a specific data type and use case. Fluency with these formats — knowing their structure, how to read and write them, and which tools manipulate them — is the practical prerequisite for any bioinformatics analysis. This is not optional background knowledge; you will encounter every format on this page within the first month of any real bioinformatics project.

## Sequence Formats

### FASTA

The simplest and most universal format. One sequence per entry:

```
>sequence_identifier optional_description
ACGTACGTACGTACGTACGT
ACGTACGTACGTACGTACGT
```

- Header line: starts with `>`, identifier is the first word, rest is free text
- Sequence: one or more lines (wrapping at any column width is valid; 60 or 80 is conventional)
- Common extensions: `.fa`, `.fasta`, `.fna` (nucleotide), `.faa` (amino acid)

**Index with `.fai`** (samtools faidx): enables $O(\log n)$ lookup of any sequence by name:

```bash
samtools faidx genome.fa          # creates genome.fa.fai
samtools faidx genome.fa chr17    # extract chr17 sequence
samtools faidx genome.fa chr1:1000000-2000000  # extract region
```

### FASTQ

Extends FASTA with per-base quality scores (Phred scores):

```
@read_name optional_description
ACGTACGTACGTACGTACGT          (sequence)
+                              (separator — optionally repeats header)
IIIIIIIIIIIIIIIIIIII          (quality: ASCII-encoded Phred+33)
```

**Phred quality score**: $Q = -10 \log_{10}(P_{\text{error}})$. ASCII encoding: character = chr(Q + 33). Common characters: `I` = Q=40 (1 error/10,000 bases), `?` = Q=30 (1/1000), `)` = Q=8 (1/6).

```python
def phred_to_prob(phred_char: str) -> float:
    Q = ord(phred_char) - 33
    return 10 ** (-Q / 10)

# Parse FASTQ efficiently
def parse_fastq(path: str):
    with open(path) as f:
        while True:
            header = f.readline().rstrip()
            if not header:
                break
            seq  = f.readline().rstrip()
            _    = f.readline()  # '+'
            qual = f.readline().rstrip()
            yield header[1:], seq, qual
```

FASTQ files are almost always gzip-compressed in practice (`.fastq.gz`). Use `gzip.open()` in Python or `zcat` in bash.

## Alignment Formats

### SAM/BAM/CRAM

**SAM** (Sequence Alignment/Map) stores aligned sequencing reads. 11 mandatory tab-separated fields:

```
QNAME  FLAG  RNAME  POS   MAPQ  CIGAR  RNEXT PNEXT TLEN SEQ   QUAL   [optional tags]
read1  99    chr1   1000  60    100M   =     1200  301  ACG.. !!@...  NM:i:0 MD:Z:100
```

Critical fields:
- **FLAG**: bitwise flags (0x1=paired, 0x4=unmapped, 0x10=reverse complement, 0x40=read1, 0x80=read2, 0x100=secondary, 0x800=supplementary)
- **CIGAR**: alignment description (100M = 100 match/mismatch; 5S95M = 5 soft-clipped, 95 aligned; 10M3I87M = 10 match, 3 insertion, 87 match)
- **MAPQ**: mapping quality (-10 log10 P(mapping wrong)); 255 = not available; 60 = 10^{-6} error rate

**BAM**: Binary compressed SAM. Requires index (`.bai`) for random access. This is the standard format.

**CRAM**: Reference-based compression. Achieves ~3–5× smaller files than BAM by storing only differences from the reference. Requires the reference genome to decode.

```bash
# Convert SAM → BAM → index
samtools view -bS aligned.sam | samtools sort -o aligned.bam
samtools index aligned.bam

# Filter: keep only properly paired, uniquely mapped reads
samtools view -b -f 0x2 -q 30 aligned.bam > filtered.bam

# Coverage statistics
samtools coverage aligned.bam
samtools depth -a aligned.bam | awk '{sum+=$3; n++} END{print sum/n}' # mean depth

# Count reads per flag category
samtools flagstat aligned.bam
```

## Variant Formats

### VCF/BCF

**VCF** (Variant Call Format) stores genetic variants. Header lines start with `##`; the column header starts with `#CHROM`:

```
##fileformat=VCFv4.2
##INFO=<ID=AF,Number=A,Type=Float,Description="Allele frequency">
##FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
##FORMAT=<ID=DP,Number=1,Type=Integer,Description="Read depth">
#CHROM  POS     ID        REF  ALT   QUAL  FILTER  INFO              FORMAT    SAMPLE1
chr17   43092919  rs28897672  A    G     .     PASS    AF=0.001;DP=45    GT:DP    0/1:45
chr17   43094077  .           ATCG A     .     PASS    AF=0.0001;DP=32   GT:DP    0/1:32
```

- **CHROM/POS**: chromosome and 1-based position of REF
- **REF/ALT**: reference and alternate alleles (REF/ALT length difference encodes SNP vs. indel)
- **INFO**: semicolon-delimited key=value annotations
- **FORMAT/SAMPLE**: per-sample genotype data; GT=0/0 (homozygous ref), 0/1 (het), 1/1 (hom alt)

**BCF**: Binary compressed VCF. Index with `.tbi` (tabix) for random access:

```bash
# Compress and index VCF
bgzip variants.vcf && tabix -p vcf variants.vcf.gz

# Random-access query
tabix variants.vcf.gz chr17:43000000-44000000

# Filter: PASS variants with AF > 0.01
bcftools view -f PASS variants.vcf.gz | \
    bcftools filter -i 'INFO/AF > 0.01' > common_variants.vcf
```

## Genome Annotation Formats

### BED

**BED** format stores genomic intervals. Core BED3 has 3 fields (BED6 adds name, score, strand; BED12 adds block structure for transcripts):

```
chr1    1000    2000    gene_A    1000    +
chr1    3000    4500    gene_B    850     -
```

- **CHROM, START, END**: 0-based half-open interval [START, END) — END is not included. This differs from VCF (1-based).

```bash
# Intersect peaks with gene bodies
bedtools intersect -a peaks.bed -b genes.bed -wa -wb > peaks_in_genes.bed

# Merge overlapping intervals
bedtools merge -i peaks.bed

# Compute coverage of genome by peaks
bedtools genomecov -i peaks.bed -g genome.sizes
```

### GFF3/GTF

**GFF3** and **GTF** store hierarchical gene feature annotations:

```
# GFF3
chr1  Ensembl  gene        1000  9000  .  +  .  ID=ENSG001;Name=TP53
chr1  Ensembl  mRNA        1000  9000  .  +  .  ID=ENST001;Parent=ENSG001
chr1  Ensembl  exon        1000  1200  .  +  .  Parent=ENST001
chr1  Ensembl  CDS         1050  1200  .  +  0  Parent=ENST001
```

**GTF** (GFF2 variant): same concept, slightly different syntax (used by STAR, HISAT2, featureCounts):

```
chr1  HAVANA  gene  1000  9000  .  +  .  gene_id "ENSG001"; gene_name "TP53";
chr1  HAVANA  exon  1000  1200  .  +  .  gene_id "ENSG001"; transcript_id "ENST001";
```

## Structure and Systems Biology Formats

### PDB / mmCIF

**PDB format** (legacy, still widely used): fixed-column text format for protein structure coordinates:

```
ATOM      1  N   MET A   1      23.456  12.345   8.901  1.00 15.23           N
ATOM      2  CA  MET A   1      24.123  11.987   9.456  1.00 14.87           C
```

**mmCIF** (current standard for PDB depositions): key-value and loop-based format; more extensible and parser-friendly.

```python
from Bio.PDB import MMCIFParser

parser = MMCIFParser()
structure = parser.get_structure("BRCA1", "6vyx.cif")
for model in structure:
    for chain in model:
        for residue in chain:
            for atom in residue:
                print(atom.get_coord())  # xyz coordinates
```

### HDF5 / h5ad

Already covered in the NoSQL section. **h5ad** (AnnData format) stores single-cell data: sparse count matrix, cell metadata, gene metadata, and embedding coordinates (UMAP, PCA) in a single file.

### SBML and Antimony

**SBML** (Systems Biology Markup Language): XML-based format for biochemical network models. Used by COPASI, tellurium, libRoadRunner, BioNetGen. Stores species, reactions, rate laws, and kinetic parameters.

**Antimony**: Human-readable model definition language that compiles to SBML:

```
# Antimony: simple lac operon model
var mRNA, Protein
const IPTG = 1.0  # mM

J1: -> mRNA;   k_tx * 1/(1 + (K_i/IPTG)^n)
J2: mRNA -> ;  k_deg_m * mRNA
J3: -> Protein; k_tl * mRNA
J4: Protein -> ; k_deg_p * Protein

k_tx = 1.0; k_deg_m = 0.347; k_tl = 0.1; k_deg_p = 0.02
K_i = 0.1; n = 2
```

### Phylogenetic Formats

**Newick format**: A parenthetical representation of tree topology with branch lengths:

```
((raccoon:19.19959, bear:6.80041):0.84600, ((sea_lion:11.99700, seal:12.00300):7.52973, ((monkey:100.85172, cat:47.14128):20.59201, weasel:18.87953):2.09460):3.87382, dog:25.46154);
```

**NEXUS format**: Richer format including character matrices, trees, and model specifications; used by MrBayes, PAUP*.

## Why This Matters for Computational Biology

Every bioinformatics tool reads and writes specific file formats. Knowing the format specifications means you can:
- Debug pipeline failures caused by format mismatches (SAM coordinate system errors, BED 0-vs-1-based confusion are among the most common bioinformatics bugs)
- Write custom parsers when standard tools are insufficient
- Validate output files before passing them to the next step
- Choose the right tool based on input/output format compatibility

The 0-based (BED, BAM) vs. 1-based (VCF, GFF, samtools view output) coordinate system distinction alone has caused published errors in dozens of papers. The CIGAR string is the key to understanding what an alignment reports — a `5S90M5H` alignment is not a 90 bp aligned read, it is a 100 bp read with 5 soft-clipped bases at each end and 90 bp aligned. Tabix indexing with bgzip compression is the universal solution to random-access queries in large annotation files. Knowing Antimony and SBML means you can share models with the systems biology community using their standard exchange format.
