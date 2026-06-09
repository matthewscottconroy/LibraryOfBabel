# Singularity/Apptainer: Containers for HPC

Here is the problem with Docker on a shared HPC cluster: Docker requires a daemon that runs with root privileges. On a cluster where thousands of users submit jobs, root privilege for any user is a catastrophic security risk — a user who can run Docker can escape the container and become root on the host. So system administrators at HPC facilities do not allow Docker. And yet, you've carefully containerized your analysis with Docker. The container works perfectly locally. But you need 32 nodes and 512 cores for the genome-wide analysis, and those live on the cluster. You are stuck.

**Singularity** (rebranded **Apptainer** by the Linux Foundation in 2021) is the container system designed for high-performance computing environments. Docker requires a daemon running as root, making it a security liability on shared HPC systems where users must not gain root access. Singularity/Apptainer solves this: containers run as the user who invokes them, require no daemon, integrate transparently with the host filesystem, and are distributed as single image files (`.sif` — **Singularity Image Format**) that can be moved like any other file. Every major HPC facility supports Apptainer; Docker typically does not run on shared clusters.

## The HPC Security Problem

On a typical HPC cluster:
- Users submit jobs to a scheduler (SLURM, PBS, LSF)
- Jobs run on compute nodes under the user's UID/GID
- Root access is forbidden — users cannot install system packages
- Docker requires a root-owned daemon, making it a privilege escalation risk

Apptainer bypasses these restrictions:
- Images run as the invoking user, not as root
- No persistent daemon required
- Read-only image files cannot escalate privileges
- The host filesystem (`$HOME`, `/scratch`, `/data`) is automatically mounted inside the container

## Building and Pulling Images

The most common workflow is pulling an existing Docker image from a registry and converting it to a `.sif` file:

```bash
# Pull a Docker image and convert to SIF (no Docker needed)
apptainer pull docker://quay.io/biocontainers/star:2.7.11a--h0033a41_0
# Creates: star_2.7.11a--h0033a41_0.sif

# Pull from Docker Hub
apptainer pull docker://ubuntu:22.04
# Creates: ubuntu_22.04.sif

# Pull specific version, give explicit name
apptainer pull --name rnaseq_pipeline_v1.sif \
    docker://myusername/rnaseq-pipeline:1.0.0

# Cache location (set to scratch to avoid home quota)
export APPTAINER_CACHEDIR=/scratch/$USER/apptainer_cache
export SINGULARITY_CACHEDIR=/scratch/$USER/apptainer_cache
```

## Running Containers

```bash
# Run a command inside a container
apptainer exec star_2.7.11a.sif STAR --version

# Run with specific bind mounts (host_path:container_path)
apptainer exec \
    --bind /scratch/$USER/data:/data \
    --bind /scratch/$USER/results:/results \
    star_2.7.11a.sif \
    STAR --runThreadN 8 \
         --genomeDir /data/star_index \
         --readFilesIn /data/sample_R1.fastq.gz /data/sample_R2.fastq.gz \
         --readFilesCommand zcat \
         --outSAMtype BAM SortedByCoordinate \
         --outFileNamePrefix /results/sample/

# Interactive shell inside container
apptainer shell star_2.7.11a.sif
Apptainer> which STAR
Apptainer> STAR --version
Apptainer> exit

# Run the container's default CMD/ENTRYPOINT
apptainer run rnaseq_pipeline_v1.sif --input /data/samples.csv

# Overlay writable layer (for testing only, not for reproducibility)
apptainer shell --writable-tmpfs star_2.7.11a.sif
```

## Writing Apptainer Definition Files

For custom images, write a **definition file** (`.def`):

```singularity
# rnaseq_pipeline.def

Bootstrap: docker
From: ubuntu:22.04

%labels
    Author  researcher@example.edu
    Version 1.0.0
    Description RNA-seq analysis pipeline

%environment
    export PATH=/opt/venv/bin:$PATH
    export LC_ALL=C.UTF-8
    export LANG=C.UTF-8

%post
    # System packages
    apt-get update && apt-get install -y \
        python3 \
        python3-pip \
        python3-venv \
        libhdf5-dev \
        curl \
        && apt-get clean

    # Create virtual environment
    python3 -m venv /opt/venv
    /opt/venv/bin/pip install --no-cache-dir --upgrade pip

    # Install analysis packages
    /opt/venv/bin/pip install --no-cache-dir \
        numpy==1.26.4 \
        scipy==1.12.0 \
        pandas==2.2.1 \
        matplotlib==3.8.3 \
        pysam==0.22.0 \
        scanpy==1.9.8 \
        h5py==3.10.0

    # Clean up build artifacts
    apt-get clean
    rm -rf /var/lib/apt/lists/*

%runscript
    exec python3 "$@"

%test
    # Run when building with --test flag
    python3 -c "import numpy, pandas, scanpy; print('All imports OK')"
```

```bash
# Build SIF from definition file (requires root or --fakeroot)
sudo apptainer build rnaseq_pipeline.sif rnaseq_pipeline.def

# Or use --fakeroot if available (rootless build on supported systems)
apptainer build --fakeroot rnaseq_pipeline.sif rnaseq_pipeline.def

# Remote build (uses Apptainer cloud build service)
apptainer build --remote rnaseq_pipeline.sif rnaseq_pipeline.def
```

## SLURM Integration

```bash
#!/bin/bash
#SBATCH --job-name=rnaseq_align
#SBATCH --nodes=1
#SBATCH --ntasks=1
#SBATCH --cpus-per-task=8
#SBATCH --mem=32G
#SBATCH --time=04:00:00
#SBATCH --partition=regular
#SBATCH --output=logs/align_%A_%a.out
#SBATCH --error=logs/align_%A_%a.err
#SBATCH --array=1-200  # one job per sample

# Load the sample ID from a manifest file
SAMPLE=$(sed -n "${SLURM_ARRAY_TASK_ID}p" config/sample_list.txt)

# Set cache directory to scratch
export APPTAINER_CACHEDIR=/scratch/$USER/apptainer_cache

# Run container
apptainer exec \
    --bind /scratch/$USER:/scratch/$USER \
    /scratch/$USER/containers/star_2.7.11a.sif \
    STAR \
        --runThreadN $SLURM_CPUS_PER_TASK \
        --genomeDir /scratch/$USER/references/star_index_GRCh38 \
        --readFilesIn /scratch/$USER/data/${SAMPLE}_R1.fastq.gz \
                      /scratch/$USER/data/${SAMPLE}_R2.fastq.gz \
        --readFilesCommand zcat \
        --outSAMtype BAM SortedByCoordinate \
        --outFileNamePrefix /scratch/$USER/results/alignments/${SAMPLE}/
```

```bash
# Submit the job array
sbatch scripts/align_array.sh

# Monitor job status
squeue -u $USER

# Check resource usage for completed job
sacct -j 12345678 --format=JobID,Elapsed,MaxRSS,CPUTime,ExitCode
```

## Snakemake with Singularity

```python
# Snakefile: per-rule Singularity containers on HPC
rule align_star:
    input:
        r1 = "data/trimmed/{sample}_R1.fastq.gz",
        r2 = "data/trimmed/{sample}_R2.fastq.gz"
    output:
        bam = "results/alignments/{sample}.bam"
    threads: 8
    resources:
        mem_mb = 32000,
        runtime = 240
    container:
        "docker://quay.io/biocontainers/star:2.7.11a--h0033a41_0"
    shell:
        """
        STAR --runThreadN {threads} ...
        """
```

```bash
# Run Snakemake with Singularity and SLURM
snakemake \
    --executor slurm \
    --jobs 100 \
    --use-singularity \
    --singularity-args "--bind /scratch/$USER" \
    --default-resources "mem_mb=4000 runtime=60" \
    --cores 1
```

## Caching and Distribution Strategy

On HPC systems, container I/O at job start is a bottleneck if images are pulled fresh each time:

```bash
# Pre-pull all containers to a shared location
mkdir -p /scratch/$USER/containers

apptainer pull --name /scratch/$USER/containers/star_2.7.11a.sif \
    docker://quay.io/biocontainers/star:2.7.11a--h0033a41_0

apptainer pull --name /scratch/$USER/containers/fastp_0.23.4.sif \
    docker://quay.io/biocontainers/fastp:0.23.4--h5ef7fe3_0

# In your pipeline, reference the pre-pulled SIF paths
STAR_SIF=/scratch/$USER/containers/star_2.7.11a.sif
apptainer exec $STAR_SIF STAR ...
```

## Why This Matters

Singularity/Apptainer bridges the gap between Docker's accessibility on local machines and the security requirements of shared HPC infrastructure — where the majority of large-scale bioinformatics work is actually done. Without Apptainer, HPC users face a choice between maintaining system-wide module installations (inflexible, version conflicts) or conda environments (more flexible, but not hermetically sealed against host system changes). Apptainer provides true isolation: a `.sif` file is a complete, immutable software environment. The ability to build from Docker images means the same container tested locally (with Docker) runs on the HPC (with Apptainer) — no separate build step for each platform. For long-running projects, storing SIF files alongside data in archival storage (e.g., Zenodo) preserves the computational environment for the lifetime of the data, ensuring analyses can be reproduced years after the original software ecosystem has evolved.
