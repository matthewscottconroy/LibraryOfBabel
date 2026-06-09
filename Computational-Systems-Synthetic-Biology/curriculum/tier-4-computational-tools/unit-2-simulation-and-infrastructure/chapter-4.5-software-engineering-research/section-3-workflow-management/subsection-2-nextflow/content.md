# Nextflow: Dataflow-Based Scientific Workflows

A common complaint about bioinformatics pipeline work is that the same analysis behaves differently depending on where it runs. On your laptop with Docker, it works. On the HPC cluster with modules loaded manually, it works but takes a different code path. On the cloud VM a collaborator set up, it fails because a dependency version doesn't match. Switching between these environments means rewriting configuration files, changing executor settings, and hoping nothing breaks at 3 AM when the cluster job finally starts. What you want is a workflow system where the same pipeline description runs identically everywhere — you just tell it the environment and it handles the rest.

**Nextflow** is a workflow management system built on the **dataflow programming model**: computations are defined as independent processes that communicate through **channels** — asynchronous queues of data items. Unlike Snakemake's file-based dependency inference, Nextflow channels can carry any data type (strings, files, tuples, maps), enabling more flexible data routing patterns. Nextflow's defining strengths are its native container support, seamless cloud execution (AWS Batch, Google Cloud Life Sciences, Azure), and the **nf-core** ecosystem of community-maintained bioinformatics pipelines.

## DSL2 Syntax

Nextflow **DSL2** (Domain Specific Language version 2) organizes workflows into three building blocks:

- **Process**: a computational unit with inputs, outputs, and a script block
- **Channel**: an asynchronous data queue connecting processes
- **Workflow**: a composition of processes connected via channels

```nextflow
// nextflow.config — project configuration
params {
    reads        = "data/raw/*_{R1,R2}.fastq.gz"
    genome_index = "/references/star_index_GRCh38/"
    gtf          = "/references/gencode.v44.annotation.gtf"
    outdir       = "results"
    cpus         = 8
    memory       = "32 GB"
    fdr          = 0.05
}

process {
    errorStrategy = "retry"
    maxRetries    = 2
    container     = "quay.io/biocontainers/fastp:0.23.4--h5ef7fe3_0"
}
```

```nextflow
// main.nf — RNA-seq pipeline in DSL2

nextflow.enable.dsl = 2

// ── Process definitions ───────────────────────────────────────────────────

process TRIM_ADAPTERS {
    tag          "$sample_id"
    publishDir   "${params.outdir}/trimmed", mode: "copy"
    container    "quay.io/biocontainers/fastp:0.23.4--h5ef7fe3_0"

    input:
    tuple val(sample_id), path(r1), path(r2)

    output:
    tuple val(sample_id), path("${sample_id}_R1_trimmed.fastq.gz"),
                          path("${sample_id}_R2_trimmed.fastq.gz"), emit: trimmed
    path "${sample_id}_fastp.json", emit: json

    script:
    """
    fastp \\
        -i ${r1} -I ${r2} \\
        -o ${sample_id}_R1_trimmed.fastq.gz \\
        -O ${sample_id}_R2_trimmed.fastq.gz \\
        -j ${sample_id}_fastp.json \\
        --detect_adapter_for_pe \\
        --thread ${task.cpus}
    """
}

process ALIGN_STAR {
    tag          "$sample_id"
    cpus         params.cpus
    memory       params.memory
    publishDir   "${params.outdir}/alignments/${sample_id}", mode: "copy"
    container    "quay.io/biocontainers/star:2.7.11a--h0033a41_0"

    input:
    tuple val(sample_id), path(r1), path(r2)
    path index

    output:
    tuple val(sample_id), path("${sample_id}.Aligned.sortedByCoord.out.bam"), emit: bam
    path "${sample_id}.Log.final.out", emit: log

    script:
    """
    STAR \\
        --runThreadN ${task.cpus} \\
        --genomeDir ${index} \\
        --readFilesIn ${r1} ${r2} \\
        --readFilesCommand zcat \\
        --outSAMtype BAM SortedByCoordinate \\
        --outFileNamePrefix ${sample_id}. \\
        --outSAMattributes NH HI AS NM
    """
}

process COUNT_FEATURES {
    tag          "$sample_id"
    publishDir   "${params.outdir}/counts", mode: "copy"
    container    "quay.io/biocontainers/subread:2.0.6--he4a0461_0"

    input:
    tuple val(sample_id), path(bam)
    path gtf

    output:
    path "${sample_id}.counts.txt", emit: counts

    script:
    """
    featureCounts \\
        -a ${gtf} \\
        -o ${sample_id}.counts.txt \\
        -T ${task.cpus} \\
        ${bam}
    """
}

process MULTIQC {
    publishDir "${params.outdir}/multiqc", mode: "copy"
    container  "quay.io/biocontainers/multiqc:1.19--pyhdfd78af_0"

    input:
    path "*"   // collect all files into working directory

    output:
    path "multiqc_report.html"
    path "multiqc_data/"

    script:
    """
    multiqc . --force
    """
}

// ── Workflow definition ───────────────────────────────────────────────────

workflow {
    // Create channel from paired-end FASTQ files
    reads_ch = Channel
        .fromFilePairs(params.reads, checkIfExists: true)
        .map { sample_id, files -> tuple(sample_id, files[0], files[1]) }

    // Reference files as value channels (broadcast to all samples)
    index_ch = Channel.value(file(params.genome_index))
    gtf_ch   = Channel.value(file(params.gtf))

    // Connect processes
    TRIM_ADAPTERS(reads_ch)
    ALIGN_STAR(TRIM_ADAPTERS.out.trimmed, index_ch)
    COUNT_FEATURES(ALIGN_STAR.out.bam, gtf_ch)

    // Collect all QC files for MultiQC
    qc_files_ch = TRIM_ADAPTERS.out.json
        .mix(ALIGN_STAR.out.log)
        .mix(COUNT_FEATURES.out.counts)
        .collect()
    MULTIQC(qc_files_ch)
}
```

## Running Nextflow

```bash
# Run locally with Docker containers
nextflow run main.nf -profile docker

# Run on SLURM with Singularity containers
nextflow run main.nf -profile slurm,singularity

# Resume after failure (Nextflow caches completed tasks)
nextflow run main.nf -resume

# Use an nf-core pipeline directly
nextflow run nf-core/rnaseq \
    --input samplesheet.csv \
    --outdir results/ \
    --genome GRCh38 \
    -profile singularity

# Tower (Nextflow cloud monitoring)
nextflow run main.nf -with-tower
```

The `-resume` flag deserves special attention. Nextflow caches the outputs of every completed process using a content hash of the inputs. When a run fails partway through — say, one sample's alignment job runs out of memory — `-resume` restarts the pipeline from the last successful cached result rather than from scratch. For a 48-sample pipeline where 47 samples completed before the failure, this means you rerun only the one failed sample, not all 48. In a world where HPC jobs routinely fail for transient reasons, `-resume` transforms a potential multi-day setback into a minor inconvenience.

## Execution Profiles

**Profiles** bundle executor configuration for different environments:

```nextflow
// nextflow.config — profiles section
profiles {
    standard {
        process.executor = "local"
    }

    docker {
        docker.enabled  = true
        docker.runOptions = "-u \$(id -u):\$(id -g)"
    }

    singularity {
        singularity.enabled    = true
        singularity.autoMounts = true
    }

    slurm {
        process.executor = "slurm"
        process.clusterOptions = "--account myproject"
        process {
            withLabel: "high_memory" {
                memory = "128 GB"
                cpus   = 16
                time   = "24h"
                queue  = "highmem"
            }
            withName: "ALIGN_STAR" {
                memory = "32 GB"
                cpus   = 8
                time   = "4h"
            }
        }
    }

    aws {
        process.executor  = "awsbatch"
        process.queue     = "arn:aws:batch:us-east-1:123456789:job-queue/my-queue"
        aws.region        = "us-east-1"
        aws.batch.cliPath = "/home/ec2-user/miniconda/bin/aws"
        workDir           = "s3://my-bucket/nextflow-work"
    }
}
```

## The nf-core Ecosystem

**nf-core** is a community repository of production-grade Nextflow pipelines covering common bioinformatics workflows:

```bash
# Browse available pipelines
nf-core list

# Key pipelines:
# nf-core/rnaseq       — RNA-seq from FASTQ to differential expression
# nf-core/chipseq      — ChIP-seq / ATAC-seq
# nf-core/sarek        — germline and somatic variant calling
# nf-core/fetchngs     — download from SRA/ENA
# nf-core/scrnaseq     — single-cell RNA-seq

# Download pipeline for offline use
nf-core download nf-core/rnaseq --revision 3.14.0 --container singularity

# Create a new pipeline from nf-core template
nf-core create --name mypipeline --description "My custom pipeline"
```

## Nextflow vs. Snakemake Comparison

| Feature | Snakemake | Nextflow |
|---|---|---|
| Primary model | File-based DAG | Dataflow channels |
| Language | Python superset | Groovy DSL |
| Container support | Conda-native; containers via profile | Native; containers per process |
| Cloud support | Via executor plugins | Built-in (AWS, GCP, Azure) |
| Community pipelines | Snakemake wrappers | nf-core (200+ pipelines) |
| Incremental reruns | Timestamp/checksum-based | `-resume` via cache hash |
| Learning curve | Lower (Python-like) | Higher (Groovy syntax) |
| Best for | Custom analyses, HPC-heavy | Standardized pipelines, cloud |

Both systems are excellent; the choice often depends on whether an nf-core pipeline already exists for your task (use Nextflow) or whether you are building a custom analysis (either works, but Snakemake's Python familiarity may be preferable).

## Why This Matters

Nextflow addresses a key barrier to reproducible bioinformatics at scale: running the same analysis across different computing environments — a local laptop for development, an HPC cluster for production runs, and a cloud platform for collaboration — while maintaining identical results. The channel-based dataflow model naturally expresses the parallelism inherent in genomics: processing 500 samples independently, then aggregating results. The `-resume` flag, which restarts failed runs from the last successful cached task, transforms a potentially days-long pipeline failure from a catastrophic event requiring a full restart into a minor interruption. The nf-core community pipelines represent thousands of person-hours of pipeline engineering, container builds, and edge-case handling — using an nf-core pipeline for a standard analysis task gives you best practices and validated results without reinventing the wheel.
