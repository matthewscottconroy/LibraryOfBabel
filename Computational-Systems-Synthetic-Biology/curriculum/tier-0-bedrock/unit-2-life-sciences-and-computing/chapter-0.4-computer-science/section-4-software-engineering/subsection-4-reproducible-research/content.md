# Reproducible Research

In 2011, a team at Bayer HealthCare tried to reproduce 67 published "landmark" studies in oncology and preclinical biology. They could fully reproduce only 11 of them — about 16%. The causes were varied: different cell lines, different protocols, missing reagents. But for computational studies, the causes are more prosaic: the code was not shared, the software versions were not recorded, the analysis was not automated in a way that could be rerun. A computational biology analysis that cannot be reproduced is not science — it is an observation.

Reproducibility requires that another researcher (or your future self) can start from the same raw data and arrive at the same results using documented, version-controlled code in a defined software environment. This is harder than it sounds: bioinformatics tools have different behaviors across versions, Python packages have complex interdependencies, and cluster environments vary. The tools in this section — workflow managers, environment management, containerization — are the infrastructure that makes reproducibility achievable.

## Workflow Managers: Making Analysis a DAG

An analysis pipeline is a directed acyclic graph (DAG) of steps with explicit input/output dependencies. **Workflow managers** enforce this structure: they determine which steps need to run (based on whether outputs are up-to-date), run independent steps in parallel, and handle job submission to cluster schedulers.

### Snakemake

Snakemake uses a Python-based rule system:

```python
# Snakefile

SAMPLES = ["ctrl_1", "ctrl_2", "treat_1", "treat_2"]

# Target rule: what do you want in the end?
rule all:
    input:
        "results/multiqc_report.html",
        expand("results/counts/{sample}.tsv", sample=SAMPLES)

# Rules define transformations: input → output
rule trim_reads:
    input:
        r1 = "data/raw/{sample}_R1.fastq.gz",
        r2 = "data/raw/{sample}_R2.fastq.gz",
    output:
        r1 = "results/trimmed/{sample}_R1.fastq.gz",
        r2 = "results/trimmed/{sample}_R2.fastq.gz",
        json = "results/qc/{sample}_fastp.json",
    log:
        "logs/fastp/{sample}.log"
    threads: 4
    shell:
        "fastp -i {input.r1} -I {input.r2} "
        "-o {output.r1} -O {output.r2} "
        "-j {output.json} -w {threads} "
        "2>{log}"

rule align:
    input:
        r1 = "results/trimmed/{sample}_R1.fastq.gz",
        r2 = "results/trimmed/{sample}_R2.fastq.gz",
        ref = "data/reference/genome.fa",
    output:
        bam = "results/aligned/{sample}.bam",
        bai = "results/aligned/{sample}.bam.bai",
    log:
        "logs/bwa/{sample}.log"
    threads: 8
    shell:
        "bwa mem -t {threads} {input.ref} {input.r1} {input.r2} 2>{log} "
        "| samtools sort -o {output.bam} && samtools index {output.bam}"
```

Execute on a SLURM cluster:

```bash
# Run with 32 simultaneous jobs on SLURM
snakemake --cluster "sbatch --ntasks=1 --cpus-per-task={threads} \
           --mem={resources.mem_mb}mb --time=4:00:00" \
          --jobs 32 --latency-wait 60 \
          --use-conda  # use per-rule conda environments
```

Key Snakemake features:
- **Wildcard expansion**: `{sample}` matches any string; Snakemake determines which rules to run based on requested outputs
- **Resource specification**: `threads`, `resources` (memory, GPU) are passed to the scheduler
- **Dry-run**: `snakemake -n` shows what would run without executing
- **Rerun incomplete**: `snakemake --rerun-incomplete` picks up after a failure

### Nextflow

Nextflow uses a Groovy-based DSL2 and is preferred for large-scale production pipelines with containerization:

```groovy
// nextflow.config
process.executor = 'slurm'
process.memory   = '8 GB'

// main.nf
process TRIM_READS {
    input:
    tuple val(sample), path(r1), path(r2)

    output:
    tuple val(sample), path("${sample}_R1_trimmed.fastq.gz"),
                       path("${sample}_R2_trimmed.fastq.gz")

    script:
    """
    fastp -i $r1 -I $r2 \
          -o ${sample}_R1_trimmed.fastq.gz \
          -O ${sample}_R2_trimmed.fastq.gz \
          -j ${sample}_fastp.json
    """
}
```

The **nf-core** project provides community-maintained, validated Nextflow pipelines for RNA-seq (nf-core/rnaseq), ATAC-seq (nf-core/atacseq), variant calling (nf-core/sarek), and dozens more.

## Environment Management: Conda and pip

**Conda** (and the faster Mamba) manages packages and environments across languages (Python, R, C libraries):

```bash
# Create environment from specification file
conda env create -f environment.yml
conda activate myproject_env

# Environment specification (environment.yml)
```

```yaml
name: rnaseq-analysis
channels:
  - conda-forge
  - bioconda
  - defaults
dependencies:
  - python=3.11
  - numpy=1.26
  - pandas=2.1
  - scipy=1.11
  - bwa=0.7.17
  - samtools=1.18
  - fastqc=0.12.1
  - fastp=0.23.4
  - pip:
    - pydeseq2==0.4.5
```

Always **pin versions** in the environment file — `numpy=1.26` not just `numpy`. Pinning ensures the environment is reproducible months later when numpy 2.0 introduces breaking changes.

## Containerization: Docker and Singularity

Containers package the software and its dependencies (including system libraries) into an isolated, portable image that runs identically on any Linux system.

**Dockerfile** for a bioinformatics tool:

```dockerfile
FROM continuumio/miniconda3:24.1.2-0

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    zlib1g-dev libncurses5-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy and install conda environment
COPY environment.yml .
RUN conda env create -f environment.yml && conda clean -afy

# Make conda env the default
SHELL ["conda", "run", "-n", "rnaseq-analysis", "/bin/bash", "-c"]

# Copy analysis scripts
COPY scripts/ /opt/scripts/
WORKDIR /data

ENTRYPOINT ["conda", "run", "--no-capture-output", "-n", "rnaseq-analysis"]
```

```bash
# Build and run Docker container
docker build -t rnaseq-pipeline:v1.2 .
docker run -v $(pwd)/data:/data rnaseq-pipeline:v1.2 \
    python /opt/scripts/run_deseq2.py --counts counts.tsv

# Convert to Singularity (required on most HPC clusters — Docker needs root)
singularity build rnaseq-pipeline_v1.2.sif docker://username/rnaseq-pipeline:v1.2
singularity exec rnaseq-pipeline_v1.2.sif python /opt/scripts/run_deseq2.py
```

## Configuration Files: YAML and Hydra

**Separate configuration from code.** Analysis parameters (paths, thresholds, sample IDs) should live in configuration files, not hardcoded in scripts:

```yaml
# config/config.yaml
reference:
  genome: /data/reference/GRCh38.fa
  annotation: /data/reference/gencode.v44.annotation.gtf

samples:
  - ctrl_rep1
  - ctrl_rep2
  - treat_rep1
  - treat_rep2

alignment:
  threads: 8
  min_mapping_quality: 20

differential_expression:
  fdr_threshold: 0.05
  log2fc_threshold: 1.0
  reference_condition: "control"
```

Load in Snakemake: `configfile: "config/config.yaml"` then access as `config["samples"]`.

**Hydra** (Facebook Research) enables hierarchical configuration for complex experiment sweeps:

```bash
python train_model.py model=transformer data.batch_size=32 optimizer.lr=1e-4
```

## Why This Matters for Computational Biology

Journals increasingly require submission of analysis code — but code alone is not enough for reproducibility. A Snakemake workflow with pinned conda environments and a tagged git commit means anyone with the raw data can reproduce every figure in a paper in a single command. The nf-core community has standardized workflows for the most common bioinformatics analyses; using them (rather than bespoke scripts) means reviewers can evaluate your methods against a known-good baseline. Docker containers solve the "it works on my machine" problem — the exact tool versions, library versions, and even system libraries are captured. Snakemake's rule-based structure makes the DAG of analysis steps explicit and auditable — it is impossible to accidentally skip a processing step or run steps out of order.
