# Versioning Data with DVC

Six months after publishing a paper on gene expression dynamics, a collaborator emails to say they found an error in the raw data — one batch of samples had a calibration problem and needs to be excluded. You reprocess the data. The figures change. Some conclusions are the same; a few are subtly different. The question now is: which version of the data was used to generate the original published figures? Is there a record of it? Can you cleanly compare the new analysis to the old one? If your data management strategy is "files in a directory", the answer is probably no. You might have `data_v2_final.h5` and `data_v2_final_corrected.h5` and `data_v3_use_this.h5`, with no record of which produced which results.

**DVC (Data Version Control)** is a Git extension for versioning large data files and ML experiments. Git is designed for code — committing a 10 GB dataset to a repository is impractical. DVC solves this by storing large files in a remote storage backend (S3, GCS, Azure, SSH, local NAS) while tracking only small **pointer files** (`.dvc` files) in Git. The result: Git tracks which version of your data corresponds to which version of your code, without inflating the repository size.

## The Core Problem

Consider a typical computational biology analysis workflow:
1. Download raw sequencing data (FASTQ files, ~50 GB)
2. Process to count matrix (intermediate HDF5, ~500 MB)
3. Normalize and filter (processed CSV, ~50 MB)
4. Run analysis, generate figures

Without DVC:
- Large files cannot be committed to Git
- Collaborators must manually download data and place it in the right locations
- There is no record of which data version was used for which analysis
- Regenerating results after updating raw data requires manual bookkeeping

With DVC:
- Large files tracked by SHA256 hash, pointer stored in Git
- `dvc pull` downloads exact data version corresponding to current Git commit
- `dvc repro` reruns only pipeline steps whose inputs have changed
- Complete experiment lineage: code version + data version + parameters → results

## Setup and Basic Usage

```bash
# Initialize DVC in an existing git repository
cd my_project
git init
dvc init
git add .dvc/ .dvcignore
git commit -m "Initialize DVC"

# Configure remote storage (S3)
dvc remote add -d myremote s3://my-bucket/dvc-storage
dvc remote modify myremote region us-east-1
git add .dvc/config
git commit -m "Configure DVC remote storage"

# Or use local storage (for development)
dvc remote add -d localremote /mnt/data/dvc_cache

# Track a large file
dvc add data/raw/rnaseq_counts.h5
# Creates: data/raw/rnaseq_counts.h5.dvc (small pointer file, committed to git)
# Adds:    data/raw/rnaseq_counts.h5 to .gitignore (so git ignores the actual file)

git add data/raw/rnaseq_counts.h5.dvc data/raw/.gitignore
git commit -m "Add RNA-seq count matrix to DVC"

# Push data to remote
dvc push

# On another machine: clone repo + pull data
git clone https://github.com/lab/project.git
cd project
dvc pull   # downloads exact data version corresponding to this git commit
```

## DVC Pipelines with dvc.yaml

DVC's most powerful feature is **pipeline tracking**: defining analysis steps as a `dvc.yaml` file that records inputs, outputs, parameters, and commands:

```yaml
# dvc.yaml — defines the complete analysis pipeline

stages:
  # Stage 1: Download and validate raw data
  download_data:
    cmd: python scripts/download_data.py --output data/raw/counts.h5
    deps:
      - scripts/download_data.py
    outs:
      - data/raw/counts.h5
    params:
      - config.yaml:
        - data.accession

  # Stage 2: Normalize and filter
  preprocess:
    cmd: python scripts/preprocess.py \
           --input data/raw/counts.h5 \
           --output data/processed/normalized.h5 \
           --config config.yaml
    deps:
      - scripts/preprocess.py
      - data/raw/counts.h5
    outs:
      - data/processed/normalized.h5
    params:
      - config.yaml:
        - analysis.min_cpm
        - analysis.min_samples

  # Stage 3: Network analysis
  network_analysis:
    cmd: python scripts/build_network.py \
           --input data/processed/normalized.h5 \
           --output results/network/
    deps:
      - scripts/build_network.py
      - data/processed/normalized.h5
    outs:
      - results/network/adjacency.npz
      - results/network/communities.csv
    metrics:
      - results/network/metrics.json:
          cache: false   # track metrics in git, not DVC

  # Stage 4: Generate figures
  figures:
    cmd: python scripts/make_figures.py --outdir results/figures/
    deps:
      - scripts/make_figures.py
      - results/network/adjacency.npz
      - results/network/communities.csv
    outs:
      - results/figures/:
          persist: true   # don't delete figures on dvc repro
```

```yaml
# config.yaml — all analysis parameters in one file
data:
  accession: "GSE123456"

analysis:
  min_cpm: 1.0
  min_samples: 3
  correlation_threshold: 0.7
  community_resolution: 1.0
```

```bash
# Run the full pipeline (only runs stages with changed inputs)
dvc repro

# Force rerun everything
dvc repro --force

# Show pipeline DAG
dvc dag

# Show what would change if a parameter changes
dvc status

# Run pipeline and show metrics
dvc repro && dvc metrics show
```

## Experiment Tracking

DVC integrates with **DVC experiments** for tracking parameter sweeps:

```bash
# Run experiment with a different parameter
dvc exp run --set-param analysis.min_cpm=0.5

# Run a grid of experiments
dvc exp run --set-param analysis.min_cpm=0.5,1.0,2.0

# List all experiments
dvc exp show

# Compare experiments
dvc exp diff exp1 exp2

# Apply best experiment to workspace
dvc exp apply my_best_exp_name
```

```python
# scripts/preprocess.py — integrating DVC params into scripts

import yaml
import argparse
import h5py
import numpy as np

def load_params(config_file="config.yaml"):
    """Load parameters tracked by DVC."""
    with open(config_file) as f:
        config = yaml.safe_load(f)
    return config

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input")
    parser.add_argument("--output")
    parser.add_argument("--config", default="config.yaml")
    args = parser.parse_args()

    params = load_params(args.config)
    min_cpm     = params["analysis"]["min_cpm"]
    min_samples = params["analysis"]["min_samples"]

    print(f"Parameters: min_cpm={min_cpm}, min_samples={min_samples}")

    # Load data
    with h5py.File(args.input, "r") as f:
        counts = f["counts"][:]
        genes  = f["gene_names"][:].astype(str)
        samples = f["sample_names"][:].astype(str)

    # Normalize and filter
    from bioanalysis import normalize_counts, filter_low_expression
    normalized       = normalize_counts(counts, method="cpm")
    filtered, mask   = filter_low_expression(counts, min_cpm=min_cpm,
                                             min_samples=min_samples)

    # Save output
    with h5py.File(args.output, "w") as f:
        f.create_dataset("counts", data=filtered.astype(np.float32))
        str_dtype = h5py.special_dtype(vlen=str)
        f.create_dataset("gene_names",   data=genes[mask],  dtype=str_dtype)
        f.create_dataset("sample_names", data=samples,      dtype=str_dtype)
        f.attrs["min_cpm"]     = min_cpm
        f.attrs["min_samples"] = min_samples

    # Record metrics for DVC
    import json
    metrics = {
        "n_genes_original": int(len(genes)),
        "n_genes_filtered": int(mask.sum()),
        "fraction_retained": float(mask.mean())
    }
    with open("results/preprocessing_metrics.json", "w") as f:
        json.dump(metrics, f, indent=2)

    print(f"Output: {filtered.shape[0]} genes × {filtered.shape[1]} samples")

if __name__ == "__main__":
    main()
```

## The DVC Git Integration Pattern

```bash
# Typical DVC-enabled workflow
git checkout -b feature/update-normalization

# Modify script
vim scripts/preprocess.py

# Rerun only affected stages
dvc repro

# Inspect results
dvc metrics show

# If results are good: commit code + data pointers together
git add scripts/preprocess.py dvc.lock
git commit -m "Try min_cpm=0.5 filtering threshold"

# To reproduce any historical result:
git checkout main~3        # go to 3 commits ago
dvc checkout               # download the data version that matches
dvc repro                  # regenerate results
```

The last three commands are the key capability. `git checkout main~3` gives you the code as it was three commits ago. `dvc checkout` gives you the data as it was at that commit — not as it is now, but the exact version recorded by the `.dvc` pointer files in that commit. `dvc repro` regenerates the results from that code and that data. You now have an exact reconstruction of the state of the analysis at any point in the project's history. This is what "reproducibility" means in practice for a continuously evolving computational project.

## Why This Matters

DVC solves the data provenance problem that is ubiquitous in computational biology: "which version of the data was used to generate these figures?" Without DVC (or an equivalent system), this question is often unanswerable. With DVC, every Git commit records not just the code version but the exact hash of every input data file. `dvc checkout` after `git checkout` restores data to the exact state it was in at that commit — making any historical result exactly reproducible. The pipeline tracking in `dvc.yaml` goes further: it provides a computational graph that determines what needs to be recomputed when any input changes, combining the dependency tracking of Snakemake with the data versioning of DVC into a unified provenance record. For collaborative projects, this means that when a collaborator adds new samples and reruns the pipeline, only the affected stages execute — automatically, correctly, and with full audit trail.
