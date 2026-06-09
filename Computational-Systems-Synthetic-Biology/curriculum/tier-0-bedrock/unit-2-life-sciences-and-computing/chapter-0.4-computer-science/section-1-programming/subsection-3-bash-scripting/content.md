# Bash Scripting for Bioinformatics

Every genome aligner you will ever use — BWA, STAR, HISAT2, minimap2 — is a command-line program. Every variant caller, every assembler, every quality control tool outputs text to standard output and accepts files as arguments. The work of connecting these tools into a coherent analysis pipeline, processing dozens of samples in parallel, parsing their outputs, checking for errors, and organizing results into a directory structure — all of this is done in bash. Bash is the plumbing that makes bioinformatics work. You can be fluent in Python and R, deeply knowledgeable about statistics and algorithms, and still be unable to run a genome analysis on a real dataset if you cannot write a bash script.

Bash is the shell of every Linux server, every HPC cluster, and every cloud compute instance you will use. This is not optional — it is the lingua franca of the command line, and command-line fluency is a prerequisite for computational biology work at any serious scale.

## Essential Bash Concepts

### Variables and Quoting

```bash
# Variable assignment (no spaces around =)
SAMPLE="SRR12345"
THREADS=8
REF="/data/reference/hg38.fa"

# Use variables with ${}
echo "Processing ${SAMPLE} with ${THREADS} threads"

# Always quote variables that contain file paths
bwa mem -t "${THREADS}" "${REF}" "${SAMPLE}_R1.fastq.gz" "${SAMPLE}_R2.fastq.gz"
```

**Quoting rules**: use double quotes to protect spaces in paths while still expanding `$variables`; use single quotes for literal strings with no expansion; use `$()` for command substitution.

One of the most common sources of mysterious failures in bash scripts is unquoted variables. If a file path contains a space — `/data/my project/sample.fastq` — an unquoted `$FILE` will be split into two words by the shell, and your command will fail in a confusing way. Always quote.

### Pipes and Redirection

Pipes are the bioinformatician's compose operator. They let you chain tools together without writing intermediate files — a critical advantage when working with hundreds of gigabytes of genomic data. Intermediate BAM files from an alignment are large; streaming the aligner's output directly into samtools sort skips writing and re-reading that data entirely:

```bash
# Count reads in a FASTQ file (4 lines per read)
echo $(( $(wc -l < sample.fastq) / 4 )) reads

# Extract read lengths from FASTQ
awk 'NR%4==2 {print length($0)}' sample.fastq | sort -n | uniq -c

# Stream FASTQ through quality trimming into aligner (no intermediate file)
fastp --stdin --stdout --adapter_sequence AGATCGGAAGAGC \
      --in1 R1.fastq.gz --in2 R2.fastq.gz 2>fastp.log \
  | bwa mem -t 8 ref.fa - \
  | samtools sort -o aligned.bam

# Redirect: > overwrites, >> appends, 2> stderr, &> both stdout+stderr
bwa mem ref.fa reads.fastq > aligned.sam 2> bwa.log
```

### Loops for Batch Processing

```bash
# Process all samples in a directory
for SAMPLE in sample1 sample2 sample3 sample4; do
    echo "Starting ${SAMPLE}"
    bwa mem -t 8 ref.fa "${SAMPLE}_R1.fastq.gz" "${SAMPLE}_R2.fastq.gz" \
        | samtools sort -o "${SAMPLE}.bam"
    samtools index "${SAMPLE}.bam"
    echo "Done: ${SAMPLE}"
done

# Glob-based loop: process all R1 files
for R1 in /data/raw/*_R1.fastq.gz; do
    SAMPLE=$(basename "${R1}" _R1.fastq.gz)
    R2="${R1/_R1/_R2}"
    echo "Processing ${SAMPLE}"
    # ... alignment command
done

# Array-based loop (cleaner for indexed access)
SAMPLES=(ctrl_rep1 ctrl_rep2 treat_rep1 treat_rep2)
for s in "${SAMPLES[@]}"; do
    echo "${s}"
done
```

### Conditionals and Error Handling

```bash
#!/bin/bash
set -euo pipefail
# -e: exit on any error
# -u: treat unset variables as errors
# -o pipefail: fail if any step in a pipe fails

# Conditional: check if file exists
if [[ ! -f "${SAMPLE}.bam.bai" ]]; then
    samtools index "${SAMPLE}.bam"
fi

# Check exit code explicitly
bwa mem ref.fa reads.fastq > aligned.sam
if [[ $? -ne 0 ]]; then
    echo "bwa failed" >&2
    exit 1
fi
```

`set -euo pipefail` at the top of every script — this is professional practice and will save you from silent failures. The most insidious bugs in bioinformatics pipelines are not errors that crash the pipeline; they are errors that silently produce empty or incorrect output files. Without `set -e`, a failing alignment step produces an empty BAM file, and the next step — sorting the empty BAM — succeeds, and the step after that succeeds, and you end up with a complete pipeline run that produced zero valid results. With `set -e`, the failure is caught at the source.

## awk: Record-Oriented Text Processing

`awk` is indispensable for parsing bioinformatics file formats (TSV, VCF, SAM, BED). Key model: awk processes text line by line; `$1`, `$2`, ... are fields; `NR` is the line number; `NF` is the number of fields. The payoff is that operations which would take 20 lines of Python can be written in one line of awk, and they run faster because awk processes the file in a single pass without loading it into memory:

```bash
# Extract specific columns from a VCF (skip header lines)
awk '!/^#/ {print $1, $2, $4, $5}' variants.vcf

# Filter BED file: keep intervals longer than 500 bp on chr1
awk 'BEGIN{OFS="\t"} $1=="chr1" && ($3-$2) > 500' peaks.bed

# Compute mean read length from SAM (flag 0 or 16 = mapped reads)
samtools view -F 4 aligned.bam \
  | awk '{sum+=length($10); n++} END{print sum/n " bp mean read length"}'

# Sum the 5th column, grouping by column 1 (sample)
awk '{sum[$1]+=$5} END{for(k in sum) print k, sum[k]}' coverage.tsv
```

## sed: Stream Editing

`sed` performs line-by-line text substitution and extraction:

```bash
# Replace spaces with underscores in FASTA headers
sed 's/ /_/g' input.fa > output.fa

# Extract lines between patterns (e.g., one entry from GenBank)
sed -n '/LOCUS SRR123/,/\/\//p' sequences.gb

# In-place edit (add a column of "0" to a BED file)
sed -i 's/$/\t0/' peaks.bed  # careful with -i on macOS (different syntax)

# Remove lines matching a pattern
sed '/^#/d' variants.vcf   # remove header lines from VCF
```

## Practical Pipeline: FASTQ → Aligned, Sorted, Indexed BAM

Here is a complete, production-quality alignment script. Notice the structural choices: `set -euo pipefail` at the top, positional arguments for the inputs so the script can be called for any sample, and logging of each step to a separate file so failures can be diagnosed:

```bash
#!/bin/bash
set -euo pipefail

SAMPLE=$1
REF=$2
THREADS=${3:-8}

# Step 1: QC and trim
fastp -i "${SAMPLE}_R1.fastq.gz" -I "${SAMPLE}_R2.fastq.gz" \
      -o "${SAMPLE}_R1_trimmed.fastq.gz" -O "${SAMPLE}_R2_trimmed.fastq.gz" \
      -j "${SAMPLE}_fastp.json" -h "${SAMPLE}_fastp.html" \
      -w "${THREADS}" 2>"${SAMPLE}_fastp.log"

# Step 2: Align
bwa mem -t "${THREADS}" \
    -R "@RG\tID:${SAMPLE}\tSM:${SAMPLE}\tPL:ILLUMINA" \
    "${REF}" \
    "${SAMPLE}_R1_trimmed.fastq.gz" "${SAMPLE}_R2_trimmed.fastq.gz" \
    2>"${SAMPLE}_bwa.log" \
  | samtools sort -@ "${THREADS}" -o "${SAMPLE}.bam"

# Step 3: Index
samtools index "${SAMPLE}.bam"

# Step 4: Flagstat QC
samtools flagstat "${SAMPLE}.bam" > "${SAMPLE}.flagstat"

echo "Complete: ${SAMPLE}"
```

Invoke for multiple samples:

```bash
for SAMPLE in SRR001 SRR002 SRR003; do
    bash align_sample.sh "${SAMPLE}" /data/ref/hg38.fa 16 &
done
wait  # wait for all background jobs
```

The `&` suffix sends each job to the background; `wait` ensures the script does not exit until all background jobs finish.

## Why This Matters for Computational Biology

Bioinformatics pipelines are bash scripts wrapping command-line tools. Even when using workflow managers (Snakemake, Nextflow), the individual steps are bash commands. Understanding pipes means understanding how data flows without intermediate disk I/O — critical for performance on large genomic datasets. Loops over samples are how you scale a single-sample analysis to 100 samples without manually running 100 commands. awk and sed are faster than loading text into Python for simple line-by-line operations on large files. `set -euo pipefail` is non-negotiable: bioinformatics pipelines that silently fail and produce empty outputs are a major source of incorrect results in published papers. Bash proficiency means you can debug a pipeline that breaks on step 47 of 50 — a daily occurrence in genomics computing.
