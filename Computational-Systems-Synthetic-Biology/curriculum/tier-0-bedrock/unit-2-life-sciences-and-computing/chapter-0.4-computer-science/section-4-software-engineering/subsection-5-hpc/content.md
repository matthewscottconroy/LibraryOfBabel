# High-Performance Computing (HPC)

There is a moment that every computational biologist eventually reaches: your laptop starts and runs for eight hours, the cooling fan screaming at full speed, and finishes one of the hundred samples you need to process. You divide 100 samples by the throughput on your laptop and arrive at a number somewhere between "weeks" and "this is not going to work." That moment is when you need a computing cluster.

Most serious bioinformatics work eventually runs on a computing cluster. Your laptop cannot align 100 whole genomes, train a protein language model, or run 10,000 parameter combinations of an ODE model. HPC clusters provide hundreds to thousands of CPU cores and terabytes of memory — but accessing them requires understanding job schedulers, resource requests, and the design of parallelizable workloads. SLURM is the dominant scheduler at academic HPC centers.

## SLURM: Submitting and Managing Jobs

SLURM (Simple Linux Utility for Resource Management) queues and dispatches jobs to cluster nodes. Key commands:

```bash
sbatch job.sh          # submit a batch job script
squeue -u $USER        # show your queued/running jobs
scancel 12345          # cancel job 12345
sacct -j 12345         # accounting info for job 12345 (resources used)
sinfo                  # show partition/queue status
scontrol show job 12345  # detailed job information
```

## Writing SLURM Job Scripts

Every SLURM job is a bash script with `#SBATCH` directives:

```bash
#!/bin/bash
#SBATCH --job-name=bwa-align
#SBATCH --output=logs/bwa_%j.out    # %j = job ID
#SBATCH --error=logs/bwa_%j.err
#SBATCH --ntasks=1                  # number of tasks (MPI processes)
#SBATCH --cpus-per-task=16          # CPUs per task (for threaded tools)
#SBATCH --mem=32G                   # total memory per node
#SBATCH --time=4:00:00              # wall time limit (HH:MM:SS)
#SBATCH --partition=standard        # queue name (site-specific)
#SBATCH --mail-type=FAIL            # email on failure
#SBATCH --mail-user=user@example.com

# Load modules (cluster-specific environment management)
module load bwa/0.7.17
module load samtools/1.18

# The actual work
bwa mem -t $SLURM_CPUS_PER_TASK \
    /data/reference/hg38.fa \
    "$SAMPLE_R1" "$SAMPLE_R2" \
  | samtools sort -@ $SLURM_CPUS_PER_TASK -o "${SAMPLE}.bam"
samtools index "${SAMPLE}.bam"
```

Key resource directives:
- `--ntasks`: for MPI programs, set to the number of MPI processes
- `--cpus-per-task`: for multithreaded programs (bwa, samtools), set to thread count; use `$SLURM_CPUS_PER_TASK` in the script to respect what was allocated
- `--mem`: total memory for the job node; estimate conservatively and verify with `sacct`
- `--time`: hard wall-time limit; job is killed when exceeded; start with 2× your estimate
- `--gres=gpu:1`: request GPU resources

## Array Jobs: Many-Parameter Parallelism

Array jobs are the tool for embarrassingly parallel workloads — running the same analysis on many samples or parameter values. The term "embarrassingly parallel" is a compliment in computer science: it means there is no coordination needed between tasks, and scaling is linear in the number of available cores. Most genomics processing — alignment, variant calling, transcript quantification — is embarrassingly parallel at the sample level:

```bash
#!/bin/bash
#SBATCH --job-name=align-array
#SBATCH --array=0-99              # 100 tasks, indices 0-99
#SBATCH --cpus-per-task=8
#SBATCH --mem=16G
#SBATCH --time=2:00:00

# SLURM_ARRAY_TASK_ID is the index for this array element
SAMPLES=($(cat sample_list.txt))
SAMPLE=${SAMPLES[$SLURM_ARRAY_TASK_ID]}

# Run analysis for this sample
bwa mem -t $SLURM_CPUS_PER_TASK ref.fa "${SAMPLE}_R1.fastq.gz" "${SAMPLE}_R2.fastq.gz" \
  | samtools sort -o "${SAMPLE}.bam"

echo "Completed sample: ${SAMPLE}"
```

Submit: `sbatch --array=0-99 align_array.sh`

All 100 tasks run concurrently (or as resources allow). The scheduler manages dependencies, resource allocation, and partial failures. To throttle (run at most 10 simultaneously): `--array=0-99%10`.

## Memory Estimation

Under-requesting memory causes job failures; over-requesting wastes cluster resources and increases queue time. Estimate memory requirements:

**Rule of thumb for common tools:**
| Tool | Memory estimate |
|---|---|
| BWA-MEM (human genome) | 8 GB (index) + 2 GB/thread |
| STAR (RNA-seq, human) | 32 GB (genome + index loading) |
| GATK HaplotypeCaller | 4–8 GB per sample |
| Trinity (de novo assembly) | ~1 GB per million reads |
| MEGAHIT (meta-assembly) | 0.5 GB per million reads |
| HISAT2 | 8 GB (human genome index) |

**Measuring actual usage** after a test run:

```bash
# After job completes
sacct -j 12345 --format=JobID,MaxRSS,Elapsed,CPUTime

# For a running job, check memory usage
sstat -j 12345 --format=JobID,MaxRSS,AveRSS
```

`MaxRSS` (Maximum Resident Set Size) is the peak memory usage. Add 10–20% headroom to this for future requests.

## Parallel File Systems and I/O Optimization

HPC clusters use parallel file systems (Lustre, GPFS/Spectrum Scale, BeeGFS) that provide high aggregate bandwidth across many disks — but individual file access can be slower than a local SSD due to network latency.

**I/O best practices:**
- **Stage data locally**: If running many jobs that all read the same reference file (e.g., genome FASTA), copy it to the local node's scratch disk (`$TMPDIR`) before the job starts
- **Avoid small file I/O**: Many small file reads/writes are inefficient on Lustre; prefer writing to one large file, then split if needed
- **Use bgzip + tabix for random access**: bgzip-compressed files with tabix index enable random access without seeking through the whole file
- **Compress intermediate files**: gzip/bzip2 reduces storage and speeds network transfers

```bash
#!/bin/bash
#SBATCH ...

# Stage reference to local scratch at job start
mkdir -p $TMPDIR/reference
cp /shared/reference/hg38.fa.* $TMPDIR/reference/

# Run alignment against local copy (faster I/O)
bwa mem -t $SLURM_CPUS_PER_TASK \
    $TMPDIR/reference/hg38.fa \
    $SAMPLE_R1 $SAMPLE_R2 \
  | samtools sort -T $TMPDIR/sort_tmp -o $SAMPLE.bam
```

## Checkpointing Long Jobs

For jobs that may run near the wall-time limit, implement checkpointing — save intermediate state and resume if interrupted:

```python
import pickle
from pathlib import Path

checkpoint_file = Path(f"checkpoints/{sample}_step3.pkl")

if checkpoint_file.exists():
    # Resume from checkpoint
    with open(checkpoint_file, "rb") as f:
        state = pickle.load(f)
    print(f"Resuming from checkpoint at step {state['step']}")
else:
    state = {"step": 0, "results": []}

# Process
for i in range(state["step"], total_steps):
    result = process_step(i)
    state["results"].append(result)
    state["step"] = i + 1
    
    # Save checkpoint every 1000 steps
    if i % 1000 == 0:
        with open(checkpoint_file, "wb") as f:
            pickle.dump(state, f)
```

## Why This Matters for Computational Biology

Every large-scale genomics project runs on HPC. Whole-genome sequencing of 1000 samples, training a deep learning model on protein structures, running MCMC for phylogenetic inference — all of these are infeasible on a laptop and routine on a cluster. Knowing SLURM means you can submit your analysis and not babysit it; array jobs mean a single submission processes 500 samples simultaneously. Memory estimation prevents the frustrating cycle of submitting a job, waiting hours in the queue, running for 12 hours, and then failing with OOM (out of memory) — which happens to everyone who does not estimate memory first. Understanding parallel file systems prevents I/O from becoming the bottleneck that makes a 100-node job run no faster than a 4-node job.
