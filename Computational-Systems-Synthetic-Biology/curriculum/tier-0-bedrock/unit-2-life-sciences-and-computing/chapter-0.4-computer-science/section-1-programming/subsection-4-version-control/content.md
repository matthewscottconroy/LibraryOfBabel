# Version Control with Git

Imagine you have been working for three weeks on an RNA-seq pipeline. You run it on your samples, generate beautiful figures, write up the methods section. Then a collaborator points out a potential bug in your normalization step. You fix it — but now the figures have changed slightly. Did you fix a real problem or introduce a new one? Which version of the code produced which figures? You realize you have no idea what you changed, or when, or what the pipeline looked like before.

This scenario plays out constantly in computational biology labs without version control. It is not a failure of diligence or intelligence — it is the natural consequence of working on code without a system designed to track its history. Git is that system. It gives you a complete, searchable, navigable record of every change you have ever made to your code, who made it, and why. It lets you travel back in time to any previous state. It lets multiple people work on the same codebase simultaneously without stepping on each other's changes. And it is how the field documents computational methods well enough to be reproduced.

Version control is not optional — it is professional hygiene. Every analysis you write is research output. Without version control, you cannot reproduce your own results in six months, you cannot collaborate without sending files back and forth via email, and you cannot recover from accidental file overwrites. Git is the universal standard; GitHub and GitLab are the platforms. This is the minimum set of skills to work in any modern computational biology group.

## Mental Model: What Git Tracks

Git tracks changes to files as a **directed acyclic graph of commits**. Each commit is a snapshot of all tracked files at a point in time, linked to its parent commit(s). The history is immutable — you can always go back to any previous state. The working directory is your current view; the staging area (index) is where you assemble the next commit.

```
Working directory → [git add] → Staging area → [git commit] → Repository
```

## Core Workflow

```bash
# Initialize a new repository
git init my-analysis
cd my-analysis

# Track files
git add analysis.py data/metadata.tsv
git status            # show what is staged, modified, untracked

# Commit with a descriptive message
git commit -m "Add initial RNA-seq differential expression analysis"

# See history
git log --oneline     # compact one-line-per-commit view
git diff HEAD~1       # diff current state vs. one commit ago
```

**Write good commit messages**: The imperative mood, capitalized, ≤50 chars for the subject line. "Add Monod equation ODE solver" not "added stuff" or "wip". A well-written commit message is documentation — six months from now, when you need to understand why you changed the normalization approach, the commit message tells you.

## What to Put in .gitignore

**Never commit:**
- Large data files (FASTQ, BAM, VCF): use DVC or link from a data store
- Credentials (API keys, passwords): these leak to GitHub and are immediately scraped
- Derived files that can be regenerated (`.pyc`, `__pycache__/`, `*.bam.bai`)
- Operating system files (`.DS_Store`, `Thumbs.db`)
- Jupyter notebook checkpoint directories (`.ipynb_checkpoints/`)

```bash
# Create .gitignore
cat > .gitignore << 'EOF'
# Data
*.fastq
*.fastq.gz
*.bam
*.bam.bai
*.vcf
*.vcf.gz

# Derived
*.pyc
__pycache__/
.ipynb_checkpoints/
*.egg-info/

# System
.DS_Store
.Rhistory
.RData
EOF
```

## Branches: Working on Features Without Breaking Main

Branches allow you to develop a new feature, fix a bug, or test an idea in isolation — without touching the working code on `main`. The key discipline is: `main` always works. New development happens on branches. This is especially important for bioinformatics pipelines that run for hours or days; you never want to discover mid-run that you accidentally committed broken code:

```bash
# Create and switch to a new branch
git checkout -b feature/add-normalization

# ... make changes, commit ...

# Switch back to main
git checkout main

# Merge the feature branch
git merge feature/add-normalization

# Delete the merged branch
git branch -d feature/add-normalization
```

In collaborative projects, use **pull requests** (GitHub) or **merge requests** (GitLab): push your branch, open a PR, get code review, then merge. This is the standard workflow for any team project.

## Remote Repositories and Collaboration

```bash
# Connect to a remote (GitHub/GitLab)
git remote add origin https://github.com/username/my-analysis.git

# Push your commits
git push -u origin main

# Get someone else's changes
git fetch origin        # download new commits (doesn't change working dir)
git pull origin main    # fetch + merge into current branch

# Clone an existing repository
git clone https://github.com/lab/rnaseq-pipeline.git
```

## Resolving Merge Conflicts

When two branches modify the same lines, Git inserts conflict markers. This is not a failure of version control — it is version control working correctly, surfacing a conflict that needs a human decision rather than silently choosing one version over another:

```
<<<<<<< HEAD
k_deg = 0.347   # your version
=======
k_deg = 0.25    # the other branch's version
>>>>>>> feature/update-parameters
```

Edit the file to choose one version (or combine), remove the markers, then:

```bash
git add conflicted_file.py
git commit -m "Resolve merge conflict: use updated degradation rate"
```

## Git for Reproducible Analysis

For a computational biology paper or project to be reproducible:
1. **Tag releases**: `git tag -a v1.0.0 -m "Version used for Figure 3"` — lets you return to the exact code state used for any figure
2. **Use submodules or Git-LFS** for large reference data (do not store in the main repo)
3. **Archive to Zenodo**: at submission, GitHub integrates with Zenodo to create a DOI for the specific tagged commit — required by many journals and funding agencies

```bash
# Create a tag for your submitted analysis
git tag -a v2024-submission -m "Code state at manuscript submission, revision 1"
git push origin --tags
```

## Practical Example: Collaborative RNA-seq Pipeline

A typical collaborative project structure:

```
rnaseq-project/
├── .gitignore          # excludes raw data, large outputs
├── README.md           # project overview, setup instructions
├── environment.yml     # conda environment (reproducible dependencies)
├── config/
│   └── config.yaml     # parameters (sample names, paths, thresholds)
├── workflow/
│   └── Snakefile       # Snakemake pipeline definition
├── scripts/
│   ├── deseq2_analysis.R
│   └── plot_results.R
├── notebooks/
│   └── exploratory.ipynb
└── results/            # output directory (in .gitignore or git-lfs)
    └── .gitkeep        # keep the directory tracked, not the contents
```

The raw data lives on a shared data store (not in git). The Snakefile, scripts, and config are version-controlled. Any collaborator can clone the repo, set up the conda environment, and re-run the entire analysis from raw data.

## Why This Matters for Computational Biology

Reproducibility crises in bioinformatics are largely traced to undocumented software versions, untracked code changes, and lack of version pinning. Git solves the code-version problem. Combined with conda environments (reproducible dependencies) and Snakemake workflows (explicit input/output dependencies), a git repository with a tagged release makes an analysis fully reproducible by any reader of the paper. GitHub-hosted code with a Zenodo DOI is increasingly required by high-impact journals. Code review via pull requests catches bugs before they propagate into results — an essential quality control step for algorithmic code. Understanding git well enough to recover from mistakes (using `git reflog`, `git stash`, or `git bisect` to find the commit that introduced a bug) is a skill that will save hours of work.
