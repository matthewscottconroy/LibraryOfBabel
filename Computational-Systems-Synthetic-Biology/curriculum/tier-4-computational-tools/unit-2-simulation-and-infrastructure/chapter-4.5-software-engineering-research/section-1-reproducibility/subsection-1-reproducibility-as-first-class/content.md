# Reproducibility as a First-Class Concern

Here is a thought experiment that should unsettle you. Imagine you've just published a paper reporting that a transcription factor you identified is responsible for driving a cell fate transition. Your analysis was careful, your statistics were sound, you manually verified the key result. A year later, another lab tries to reproduce your finding — and cannot. Not because the biology was wrong, but because one of your filtering thresholds was buried in a script that was never shared, the analysis depended on a version of a Python package that has since changed behavior, and the intermediate processed data files were on a server that was decommissioned when a postdoc left. The science was real. But without the infrastructure to reproduce it, no one can verify that, extend it, or build on it. In this precise sense, an irreproducible result is not a scientific finding. It is an anecdote.

Computational biology is in the middle of a reproducibility crisis, and unlike the crisis in clinical psychology — which is largely about statistical power and flexibility in data analysis — a significant fraction of the failures in our field have a different cause. The code exists. The data exists. The analysis was done correctly. But undocumented dependencies, missing configuration files, hardcoded parameters, and absent provenance records mean that no one, including the original authors, can re-execute the pipeline. Building reproducible research infrastructure from the first day of a project is not bureaucracy. It is the minimum cost of doing computational science correctly.

## The Reproducibility Spectrum

Reproducibility exists on a spectrum, from strongest to weakest guarantee:

| Level | What is reproduced | Requirements |
|---|---|---|
| **Exact replication** | Bit-for-bit identical output | Same code, same data, same environment (containers) |
| **Numerical reproduction** | Same quantitative results within tolerance | Same code and data; environment documented |
| **Conceptual reproduction** | Same conclusions | Independent implementation, different data |
| **Replication** | Same conclusions in new lab, new samples | Independent experiment + independent analysis |

Computational biology should target at minimum numerical reproduction (same quantitative results). Exact replication may not be achievable for non-deterministic analyses (stochastic simulations, ML training with random initialization) — document random seeds explicitly.

## The Four Pillars of Reproducible Computational Research

```
1. CODE    — version controlled, documented, tested
2. DATA    — archived, versioned, with metadata
3. ENVIRONMENT — containers or precise dependency specification
4. WORKFLOW — documented, automated, end-to-end
```

Every published computational analysis should provide all four. Notice that none of these are exotic demands: they describe what good engineering practice already requires. The trouble is that research workflows — iterative, exploratory, deadline-driven — tend to accumulate technical debt rapidly. The pillar framework is a checklist for paying that debt before it becomes a liability.

## Setting Up a Reproducible Project Structure

```bash
# Standard reproducible research project layout
project/
├── .git/                    # Version control (git)
├── .gitignore               # Exclude: raw large data, __pycache__, .env
├── README.md                # Project overview, how to reproduce
├── LICENSE                  # Open source license (MIT, Apache 2.0)
│
├── config/
│   ├── config.yaml          # Analysis parameters (never hardcode)
│   └── samples.csv          # Sample manifest
│
├── data/
│   ├── raw/                 # Raw data (read-only; tracked by DVC)
│   │   └── .gitkeep
│   ├── processed/           # Processed data (also tracked by DVC)
│   └── README.md            # Data provenance documentation
│
├── workflow/
│   ├── Snakefile            # OR nextflow.nf / workflow.nf
│   └── rules/               # Modular workflow rules
│
├── src/
│   └── mypackage/           # Python package (pip installable)
│       ├── __init__.py
│       └── analysis.py
│
├── notebooks/               # Jupyter notebooks (exploration only)
│   └── 01_exploratory.ipynb
│
├── results/                 # Generated outputs (tracked by DVC or gitignored)
│   ├── figures/
│   └── tables/
│
├── tests/                   # Automated tests
│   └── test_analysis.py
│
├── pyproject.toml           # Package and dependency specification
├── environment.yaml         # Conda environment (for reproducibility)
└── Dockerfile               # Container specification
```

```bash
# Initialize a new reproducible project
mkdir my_project && cd my_project
git init
git branch -m main

# Set up DVC for data versioning
pip install dvc dvc-s3  # or dvc-gdrive, dvc-azure
dvc init
git add .dvc/
git commit -m "Initialize project with git and DVC"

# Create conda environment file
cat > environment.yaml << 'EOF'
name: my_project
channels:
  - conda-forge
  - bioconda
dependencies:
  - python=3.11
  - numpy=1.26
  - scipy=1.12
  - pandas=2.2
  - matplotlib=3.8
  - snakemake=8.4
  - bioconda::star=2.7.11
  - pip:
    - pyproject-toml-based-package
EOF

conda env create -f environment.yaml
conda activate my_project
```

## Documenting Analysis Parameters

**Never hardcode parameters** in scripts. Use a configuration file:

```python
# config/config.yaml
analysis:
  fdr_threshold: 0.05
  lfc_threshold: 1.0
  min_count: 10
  n_pca_components: 50

data:
  genome: "GRCh38"
  annotation: "GENCODE_v44"
  star_index: "/data/references/star_index_GRCh38/"

samples:
  metadata: "config/samples.csv"
  batch_correction: true
  batch_column: "sequencing_batch"
```

```python
# src/mypackage/config.py
import yaml
from pathlib import Path

def load_config(config_file="config/config.yaml"):
    """Load analysis configuration and validate required fields."""
    with open(config_file) as f:
        config = yaml.safe_load(f)

    # Validate required fields
    required = ["analysis.fdr_threshold", "data.genome"]
    for field in required:
        keys = field.split(".")
        val = config
        for k in keys:
            if k not in val:
                raise ValueError(f"Missing required config field: {field}")
            val = val[k]

    return config

# Usage in analysis scripts
config = load_config()
fdr_threshold = config["analysis"]["fdr_threshold"]
```

The value of this pattern is not merely tidiness. When a reviewer asks why you chose FDR 0.05 rather than 0.01, or when you want to re-run the analysis with a stricter threshold, the threshold lives in exactly one place. There is no risk of updating it in one script and forgetting another. When a collaborator opens the project, the first thing they read is a structured declaration of every choice that shapes the results.

## Logging and Provenance

Every analysis run should produce a log that enables reconstruction of exactly what was done:

```python
import logging
import json
import sys
import hashlib
from datetime import datetime
from pathlib import Path

def setup_logging(log_file, level=logging.INFO):
    """Configure logging to both file and stdout."""
    logging.basicConfig(
        level=level,
        format="%(asctime)s %(levelname)s %(name)s: %(message)s",
        handlers=[
            logging.FileHandler(log_file),
            logging.StreamHandler(sys.stdout)
        ]
    )
    return logging.getLogger(__name__)

def log_analysis_metadata(config, input_files, output_dir):
    """
    Log all information needed to reproduce an analysis run.
    Saves: timestamp, git commit, config, input file checksums.
    """
    import subprocess

    metadata = {
        "timestamp": datetime.utcnow().isoformat() + "Z",
        "python_version": sys.version,
        "git_commit": subprocess.run(["git", "rev-parse", "HEAD"],
                                      capture_output=True, text=True).stdout.strip(),
        "git_status": subprocess.run(["git", "status", "--short"],
                                      capture_output=True, text=True).stdout.strip(),
        "config": config,
        "input_checksums": {}
    }

    # Record SHA256 of all input files
    for file_path in input_files:
        file_path = Path(file_path)
        if file_path.exists():
            sha256 = hashlib.sha256(file_path.read_bytes()).hexdigest()
            metadata["input_checksums"][str(file_path)] = sha256

    # Write metadata to output directory
    output_dir = Path(output_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    metadata_file = output_dir / "run_metadata.json"
    with open(metadata_file, "w") as f:
        json.dump(metadata, f, indent=2)

    print(f"Run metadata saved to {metadata_file}")
    if metadata["git_status"]:
        print(f"WARNING: Uncommitted changes detected:\n{metadata['git_status']}")
        print("Commit all changes before running a final analysis!")
    return metadata
```

The SHA256 checksum is the key ingredient here. It is not enough to know which file was used — you need to know that the file has not been silently modified. A SHA256 hash is a unique fingerprint for a file's contents; two files with the same hash are guaranteed to be identical. Storing checksums of every input file alongside the results creates an unambiguous record of what data was analyzed.

## The Reproducibility Checklist

Before submitting a paper or sharing an analysis, verify:

```python
REPRODUCIBILITY_CHECKLIST = """
CODE:
  [ ] All code committed to git (no uncommitted changes in final run)
  [ ] Repository is public or will be made public upon publication
  [ ] Code is documented (docstrings, README)
  [ ] All analysis steps can be run from a single command (Snakemake/Nextflow)
  [ ] Random seeds set and documented

DATA:
  [ ] Raw data archived on Zenodo, figshare, or GEO/SRA (if genomics)
  [ ] DOI assigned to raw data
  [ ] Data README describes format, provenance, accession numbers
  [ ] DVC or equivalent tracks data versions

ENVIRONMENT:
  [ ] environment.yaml or requirements.txt specifies exact package versions
  [ ] Dockerfile or Singularity image available
  [ ] Container image pushed to a public registry (Docker Hub, Quay.io)

WORKFLOW:
  [ ] Snakefile / Nextflow pipeline runs end-to-end without manual steps
  [ ] Parameters in config file, not hardcoded
  [ ] All intermediate steps produce checksummed outputs

RESULTS:
  [ ] Run metadata (commit hash, config, checksums) saved with results
  [ ] Figures generated programmatically (not manually edited)
  [ ] Tables generated programmatically
"""

def check_reproducibility(project_dir="."):
    """Automated reproducibility checks."""
    import subprocess
    from pathlib import Path

    checks = {}

    # Check: no uncommitted changes
    result = subprocess.run(["git", "status", "--short"],
                            capture_output=True, text=True, cwd=project_dir)
    checks["clean_git"] = len(result.stdout.strip()) == 0

    # Check: DVC initialized
    checks["dvc_initialized"] = (Path(project_dir) / ".dvc").exists()

    # Check: workflow file exists
    has_snakemake = (Path(project_dir) / "Snakefile").exists()
    has_nextflow  = (Path(project_dir) / "main.nf").exists()
    checks["workflow_exists"] = has_snakemake or has_nextflow

    # Check: Docker or Singularity
    has_docker = (Path(project_dir) / "Dockerfile").exists()
    has_singularity = len(list(Path(project_dir).glob("*.def"))) > 0
    checks["container_exists"] = has_docker or has_singularity

    # Check: tests exist
    checks["tests_exist"] = (Path(project_dir) / "tests").exists()

    print("Reproducibility audit:")
    for check, passed in checks.items():
        status = "✓" if passed else "✗"
        print(f"  [{status}] {check}")

    n_passed = sum(checks.values())
    print(f"\n{n_passed}/{len(checks)} checks passed")
    return checks
```

## Why This Matters

The reproducibility crisis in computational biology is not hypothetical — it has real consequences. Published ML models for drug discovery that cannot be reproduced waste millions in follow-up experiments. Genomic analysis pipelines that produce different results on different operating systems contaminate clinical databases with artifacts. GWAS analyses that were not corrected for population stratification generated years of false leads. Building reproducible research infrastructure is not altruism — it protects your own work. When reviewers ask for revised analyses, or when collaborators extend your pipeline six months later, or when you need to rerun an analysis with updated data, the investment in reproducibility pays dividends immediately. More fundamentally, science that cannot be independently verified is not science.
