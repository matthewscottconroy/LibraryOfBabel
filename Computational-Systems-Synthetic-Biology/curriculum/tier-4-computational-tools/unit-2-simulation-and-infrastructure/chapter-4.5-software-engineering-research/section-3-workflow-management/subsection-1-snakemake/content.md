# Snakemake: Rule-Based Workflow Management

Picture this scenario, familiar to anyone who has processed genomic data. You have 48 RNA-seq samples. Your analysis has twelve steps: quality control, adapter trimming, alignment, feature counting, normalization, batch correction, differential expression, pathway enrichment, network construction, clustering, visualization, and final figure generation. You run the full pipeline. Everything works. Then a reviewer asks you to use a different reference genome annotation. Now which of the twelve steps need to be rerun? Probably eight of them — but which eight, in what order, and how do you make sure you don't accidentally mix outputs from the old annotation with results from the new one? If your pipeline is "a series of shell commands documented in a README", the answer is: very carefully, by hand, hoping you don't make a mistake.

**Snakemake** is a workflow management system for scientific computing that defines analysis pipelines as collections of **rules** — each rule specifies input files, output files, and the command that transforms inputs to outputs. Snakemake then infers the directed acyclic graph (DAG) of dependencies automatically and executes only the rules needed to produce missing or outdated outputs. It was designed for bioinformatics, scales transparently from laptop to HPC cluster, and integrates with conda, containers, and cloud computing.

## Core Concepts

A Snakemake workflow is a **Snakefile** containing rules. Each rule defines:
- **input**: files this rule requires
- **output**: files this rule produces
- **params**: non-file parameters
- **log**: log file path
- **shell** or **run**: the command or Python block that generates output from input

Snakemake works backward from requested **targets**: given a target output file, it finds which rule produces it, recursively resolves inputs, and builds the dependency DAG.

**Wildcards** are the key abstraction: `{sample}` in a rule's input/output paths is resolved against available files, enabling one rule to describe processing of hundreds of samples without repetition.

## Basic Snakefile Structure

```python
# Snakefile — RNA-seq analysis pipeline example

import pandas as pd

# Configuration
configfile: "config/config.yaml"

# Load sample manifest
samples = pd.read_csv(config["samples"])["sample_id"].tolist()

# ── Final target: tell Snakemake what we want ──────────────────────────────
rule all:
    input:
        expand("results/counts/{sample}.counts.txt", sample=samples),
        "results/multiqc/multiqc_report.html",
        "results/deseq2/deseq2_results.tsv"

# ── Per-sample rules ────────────────────────────────────────────────────────

rule trim_adapters:
    input:
        r1 = "data/raw/{sample}_R1.fastq.gz",
        r2 = "data/raw/{sample}_R2.fastq.gz"
    output:
        r1 = "data/trimmed/{sample}_R1_trimmed.fastq.gz",
        r2 = "data/trimmed/{sample}_R2_trimmed.fastq.gz",
        json = "results/fastp/{sample}.json",
        html = "results/fastp/{sample}.html"
    log:
        "logs/fastp/{sample}.log"
    params:
        extra = "--detect_adapter_for_pe --qualified_quality_phred 20"
    threads: 4
    shell:
        """
        fastp -i {input.r1} -I {input.r2} \
              -o {output.r1} -O {output.r2} \
              -j {output.json} -h {output.html} \
              {params.extra} --thread {threads} \
              2> {log}
        """

rule align_star:
    input:
        r1 = "data/trimmed/{sample}_R1_trimmed.fastq.gz",
        r2 = "data/trimmed/{sample}_R2_trimmed.fastq.gz",
        index = config["star_index"]
    output:
        bam = "results/alignments/{sample}/Aligned.sortedByCoord.out.bam",
        log_final = "results/alignments/{sample}/Log.final.out"
    log:
        "logs/star/{sample}.log"
    threads: 8
    resources:
        mem_mb = 32000
    shell:
        """
        STAR --runThreadN {threads} \
             --genomeDir {input.index} \
             --readFilesIn {input.r1} {input.r2} \
             --readFilesCommand zcat \
             --outSAMtype BAM SortedByCoordinate \
             --outFileNamePrefix results/alignments/{wildcards.sample}/ \
             2> {log}
        """

rule count_features:
    input:
        bam = "results/alignments/{sample}/Aligned.sortedByCoord.out.bam",
        gtf = config["gtf"]
    output:
        counts = "results/counts/{sample}.counts.txt"
    log:
        "logs/featurecounts/{sample}.log"
    threads: 2
    shell:
        """
        featureCounts -T {threads} \
                      -a {input.gtf} \
                      -o {output.counts} \
                      {input.bam} \
                      2> {log}
        """

# ── Aggregate rules ─────────────────────────────────────────────────────────

rule multiqc:
    input:
        expand("results/fastp/{sample}.json", sample=samples),
        expand("results/alignments/{sample}/Log.final.out", sample=samples),
        expand("results/counts/{sample}.counts.txt", sample=samples)
    output:
        "results/multiqc/multiqc_report.html"
    log:
        "logs/multiqc.log"
    shell:
        """
        multiqc results/ -o results/multiqc/ --force 2> {log}
        """

rule deseq2:
    input:
        counts = expand("results/counts/{sample}.counts.txt", sample=samples),
        metadata = config["samples"]
    output:
        results = "results/deseq2/deseq2_results.tsv",
        volcano = "results/deseq2/volcano_plot.pdf"
    log:
        "logs/deseq2.log"
    script:
        "scripts/deseq2_analysis.R"
```

## Running Snakemake

```bash
# Dry run: show what would execute without running anything
snakemake --dry-run --snakefile Snakefile

# Execute with 8 cores (local)
snakemake --cores 8

# Execute specific target
snakemake --cores 4 results/counts/sample01.counts.txt

# Visualize DAG
snakemake --dag | dot -Tpdf > dag.pdf
snakemake --rulegraph | dot -Tpdf > rulegraph.pdf

# Rerun all rules, even if outputs exist
snakemake --cores 8 --forceall

# Touch (mark as up to date) without rerunning
snakemake --touch
```

## Conda Integration

Each rule can declare a conda environment, ensuring exact software versions:

```python
rule align_star:
    input:
        r1 = "data/trimmed/{sample}_R1_trimmed.fastq.gz",
        ...
    output:
        bam = "results/alignments/{sample}/Aligned.sortedByCoord.out.bam"
    conda:
        "envs/star.yaml"   # path to environment YAML
    shell:
        "STAR ..."
```

```yaml
# envs/star.yaml
name: star_env
channels:
  - conda-forge
  - bioconda
dependencies:
  - star=2.7.11a
  - samtools=1.19
```

```bash
# Run with conda integration
snakemake --cores 8 --use-conda
# Snakemake creates environments automatically on first run
```

## SLURM Cluster Submission

Snakemake integrates with HPC job schedulers. The modern approach uses **executor plugins**:

```bash
# Install SLURM executor plugin
pip install snakemake-executor-plugin-slurm

# Run on SLURM cluster
snakemake \
    --executor slurm \
    --jobs 100 \
    --default-resources "mem_mb=4000 runtime=60" \
    --use-conda \
    --cores 1
```

```python
# Snakefile: per-rule resource specifications
rule align_star:
    input:  ...
    output: ...
    threads: 8
    resources:
        mem_mb  = 32000,    # memory in MB
        runtime = 120,      # walltime in minutes
        slurm_partition = "highmem"
    shell: "STAR ..."
```

```yaml
# config/cluster.yaml (alternative: profiles)
__default__:
    time: "01:00:00"
    mem: "4G"
    cpus-per-task: 1

align_star:
    time: "04:00:00"
    mem: "32G"
    cpus-per-task: 8
    partition: "highmem"
```

## Advanced Features

### Checkpoints

**Checkpoints** handle cases where the number of output files is not known until a rule runs — for example, when a clustering step determines how many clusters exist:

```python
checkpoint split_by_cluster:
    input:
        "results/clustering/cluster_assignments.csv"
    output:
        directory("results/clusters/")
    run:
        import pandas as pd, os
        os.makedirs(output[0], exist_ok=True)
        df = pd.read_csv(input[0])
        for cluster_id, group in df.groupby("cluster"):
            group.to_csv(f"{output[0]}/{cluster_id}.csv", index=False)

def aggregate_cluster_results(wildcards):
    checkpoint_output = checkpoints.split_by_cluster.get(**wildcards).output[0]
    clusters = glob_wildcards(os.path.join(checkpoint_output, "{cluster_id}.csv")).cluster_id
    return expand("results/per_cluster/{cluster_id}_results.csv", cluster_id=clusters)

rule collect_results:
    input:
        aggregate_cluster_results
    output:
        "results/all_cluster_results.csv"
    run:
        import pandas as pd
        pd.concat([pd.read_csv(f) for f in input]).to_csv(output[0], index=False)
```

### Parameterized Runs with Config

```python
# Run with parameter override from command line
# snakemake --config fdr_threshold=0.01 lfc_threshold=2.0

rule deseq2:
    params:
        fdr = config.get("fdr_threshold", 0.05),
        lfc = config.get("lfc_threshold", 1.0)
    script:
        "scripts/deseq2_analysis.R"
```

## The Snakemake Execution Model

Snakemake's power comes from its **declarative** approach: you describe what you want (inputs and outputs), not how to get it in sequence. This has three important consequences:

1. **Incremental execution**: if 9 of 10 samples have already been processed and outputs exist, only the 10th sample is reprocessed. No manual bookkeeping required.
2. **Automatic parallelism**: independent rules run in parallel up to the specified core/job limit.
3. **Provenance tracking**: the Snakefile is a complete record of every step that produced every output — a critical component of reproducible research.

## Why This Matters

Snakemake solves the "analysis as a series of shell commands in a README" problem that plagues computational biology. When an analysis requires 12 steps, processes 200 samples, and must be rerun after updating a reference genome or adding new samples, manually tracking which steps need to be redone becomes impossible. Snakemake's DAG-based execution engine handles this automatically, rerunning only the minimum set of steps needed. The conda integration means that even complex multi-tool pipelines with conflicting dependencies can be version-locked and reproduced exactly on any system with Snakemake installed. For published analyses, a Snakefile combined with a conda environment specification provides a complete, executable methods section — reviewers can re-run the entire analysis from raw data with a single command.
