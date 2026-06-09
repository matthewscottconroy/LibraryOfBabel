# Docker: Containerizing Computational Environments

You've almost certainly encountered this. You spend a week getting a complex bioinformatics pipeline working on your laptop — the right versions of STAR, GATK, samtools, Python, and a half-dozen packages all installed and configured correctly. Then you try to run it on the lab cluster. Something about the glibc version is wrong. Or samtools was compiled against a different HTS library. Or Python 3.9 was available but your code requires 3.11. The error messages are cryptic. You spend three days not doing science, debugging software installation on a remote machine. The promise of "it works on my machine" is not a promise at all.

**Docker** packages an application and all its dependencies — system libraries, runtime, Python packages, tool binaries — into a **container image**: a portable, self-contained unit that runs identically on any machine with Docker installed. Unlike virtual machines, containers share the host OS kernel, making them lightweight (seconds to start, megabytes overhead). For computational biology, Docker solves the "it works on my machine" problem: a container that runs on a laptop will run identically on an HPC node, a cloud VM, or a collaborator's workstation five years later.

## Core Concepts

A **Dockerfile** is a text recipe that builds an image layer by layer:
- Each `RUN`, `COPY`, or `ADD` instruction creates an immutable layer
- Layers are cached: if earlier layers have not changed, Docker reuses them
- The final image is a stack of layers that together constitute the complete filesystem

A **container** is a running instance of an image — the image is read-only, but the running container has a writable overlay layer.

An **image registry** (Docker Hub, Quay.io, GitHub Container Registry) stores and distributes images by `name:tag`.

## Writing a Dockerfile

```dockerfile
# Dockerfile for a Python bioinformatics environment
# Multi-stage build: separate build environment from runtime environment

# ── Stage 1: builder ───────────────────────────────────────────────────────
FROM ubuntu:22.04 AS builder

# Prevent interactive prompts during apt-get
ENV DEBIAN_FRONTEND=noninteractive

# System dependencies needed for building Python packages
RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        gcc \
        python3-dev \
        python3-pip \
        python3-venv \
        curl \
        git \
    && rm -rf /var/lib/apt/lists/*

# Create virtual environment for clean isolation
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Install Python packages
COPY requirements.txt /tmp/requirements.txt
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir -r /tmp/requirements.txt

# ── Stage 2: runtime ───────────────────────────────────────────────────────
FROM ubuntu:22.04 AS runtime

ENV DEBIAN_FRONTEND=noninteractive

# Minimal runtime system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
        python3 \
        libgomp1 \
        libhdf5-103 \
        curl \
    && rm -rf /var/lib/apt/lists/*

# Copy virtual environment from builder stage
COPY --from=builder /opt/venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"

# Create non-root user (security best practice)
RUN useradd --create-home --shell /bin/bash bioinfo
USER bioinfo
WORKDIR /home/bioinfo

# Copy application code
COPY --chown=bioinfo:bioinfo src/ /home/bioinfo/src/
COPY --chown=bioinfo:bioinfo scripts/ /home/bioinfo/scripts/

# Metadata
LABEL maintainer="research@example.edu" \
      version="1.0.0" \
      description="RNA-seq analysis pipeline"

# Default command
CMD ["python3", "-m", "src.pipeline"]
```

```text
# requirements.txt
numpy==1.26.4
scipy==1.12.0
pandas==2.2.1
matplotlib==3.8.3
seaborn==0.13.2
scikit-learn==1.4.1
statsmodels==0.14.1
biopython==1.83
pysam==0.22.0
pydeseq2==0.4.9
scanpy==1.9.8
anndata==0.10.6
h5py==3.10.0
zarr==2.17.2
snakemake==8.4.6
```

The multi-stage build deserves comment. The first stage installs all the compilers, headers, and build tools needed to compile Python packages. The second stage copies only the finished virtual environment — no compilers, no build artifacts, no source trees. The result is an image that is much smaller than a naive single-stage build and has a dramatically reduced attack surface. This distinction matters in shared scientific computing environments where image size affects both pull time and storage costs.

## Building and Running Containers

```bash
# Build image (. means use Dockerfile in current directory)
docker build -t rnaseq-pipeline:1.0.0 .

# Tag for registry
docker tag rnaseq-pipeline:1.0.0 docker.io/username/rnaseq-pipeline:1.0.0
docker tag rnaseq-pipeline:1.0.0 docker.io/username/rnaseq-pipeline:latest

# Push to Docker Hub
docker login
docker push docker.io/username/rnaseq-pipeline:1.0.0

# Run interactively (mount local data directory)
docker run -it \
    --rm \
    -v /home/user/data:/home/bioinfo/data \
    -v /home/user/results:/home/bioinfo/results \
    rnaseq-pipeline:1.0.0 \
    bash

# Run a specific command
docker run --rm \
    -v $(pwd)/data:/home/bioinfo/data \
    rnaseq-pipeline:1.0.0 \
    python3 scripts/process_sample.py --sample data/sample01.fastq.gz

# Run with GPU support (for deep learning)
docker run --rm --gpus all \
    -v $(pwd)/data:/data \
    pytorch-bio:latest \
    python3 train.py

# Inspect image layers
docker history rnaseq-pipeline:1.0.0

# List running containers
docker ps

# View container resource usage
docker stats
```

## Environment Reproducibility

Docker images encode complete computational environments. To maximize reproducibility:

```dockerfile
# Pin the base image by digest, not just tag
# (tags can be overwritten; digests are immutable)
FROM ubuntu:22.04@sha256:a6d2b38300ce017add71440577d5b0a90460d0e57fd7aec21dd0d1b0761bbfb2

# Pin all package versions
RUN apt-get update && apt-get install -y \
        python3=3.10.12-1~22.04 \
        libhdf5-dev=1.10.7+repack-4ubuntu2 \
    && rm -rf /var/lib/apt/lists/*

# Record build metadata
ARG BUILD_DATE
ARG GIT_COMMIT
LABEL build_date="$BUILD_DATE" \
      git_commit="$GIT_COMMIT"
```

```bash
# Build with metadata
docker build \
    --build-arg BUILD_DATE=$(date -u +'%Y-%m-%dT%H:%M:%SZ') \
    --build-arg GIT_COMMIT=$(git rev-parse HEAD) \
    -t rnaseq-pipeline:1.0.0 .

# Save image to file for offline transfer / archiving
docker save rnaseq-pipeline:1.0.0 | gzip > rnaseq-pipeline-1.0.0.tar.gz

# Load image from archive
docker load < rnaseq-pipeline-1.0.0.tar.gz
```

## Docker Compose for Multi-Container Services

For analysis environments requiring multiple services (e.g., Jupyter notebook + database + API):

```yaml
# docker-compose.yml
version: "3.9"
services:
  jupyter:
    build: .
    image: rnaseq-pipeline:1.0.0
    ports:
      - "8888:8888"
    volumes:
      - ./notebooks:/home/bioinfo/notebooks
      - ./data:/home/bioinfo/data
    command: >
      jupyter lab
      --ip=0.0.0.0
      --no-browser
      --NotebookApp.token=''

  db:
    image: postgres:15
    environment:
      POSTGRES_DB: bioinfo
      POSTGRES_USER: researcher
      POSTGRES_PASSWORD_FILE: /run/secrets/db_password
    volumes:
      - db_data:/var/lib/postgresql/data
    secrets:
      - db_password

volumes:
  db_data:

secrets:
  db_password:
    file: ./secrets/db_password.txt
```

```bash
# Start all services
docker compose up -d

# View logs
docker compose logs jupyter

# Stop and remove containers (keep volumes)
docker compose down

# Stop and remove everything including volumes
docker compose down -v
```

## Workflow Integration: Docker in Snakemake

```python
# Snakefile using Docker containers per rule
rule align_star:
    input:
        r1 = "data/trimmed/{sample}_R1.fastq.gz",
        r2 = "data/trimmed/{sample}_R2.fastq.gz",
        index = config["star_index"]
    output:
        bam = "results/{sample}.bam"
    container:
        "docker://quay.io/biocontainers/star:2.7.11a--h0033a41_0"
    threads: 8
    shell:
        "STAR --runThreadN {threads} --genomeDir {input.index} ..."
```

```bash
# Run Snakemake with container support
snakemake --cores 8 --use-singularity  # converts Docker → Singularity on HPC
```

## Why This Matters

Docker resolves a fundamental tension in computational biology: complex analyses require dozens of tools with complex, often conflicting dependency trees, yet these analyses must be reproducible across systems and time. Without containers, a pipeline that requires Python 3.8, STAR 2.7.10, GATK 4.3.0, and a specific version of samtools must be installed and configured on every machine that will ever run it — and any system upgrade can silently break it. With Docker, the entire software environment is a versioned artifact, as reproducible as the code itself. The multi-stage build pattern is especially important in scientific computing: it separates the build-time requirements (compilers, headers) from the runtime image, dramatically reducing image size and attack surface. Archiving images alongside data and code on platforms like Zenodo ensures that a published analysis remains executable even when the software ecosystem evolves.
