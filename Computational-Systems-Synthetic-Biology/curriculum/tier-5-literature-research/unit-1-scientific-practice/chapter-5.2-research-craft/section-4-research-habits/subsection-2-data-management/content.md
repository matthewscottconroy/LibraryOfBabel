# Data Management for Research Scientists

In 2012, a genomics lab discovered that a critical dataset — two years of sequencing runs representing thousands of samples — had been corrupted. The raw files existed, but a well-meaning graduate student had "cleaned up" the directory and overwritten the original FASTQ files with processed versions, deleting the provenance of every downstream analysis in the process. The published papers remained intact; the ability to re-analyze with updated methods, or to respond to reviewer requests for additional analyses, was gone. Data is the foundation of science. Losing data through poor management, or being unable to access data in a usable format years after collection, is a failure that undermines the value of the experimental work that produced it. Good data management prevents loss, enables reproducibility, and makes it possible to answer questions you did not anticipate when you designed the experiment. This section covers file naming, directory structure, version control, and backup strategies appropriate for computational biology research.

## The Core Principle: Raw Data Is Sacred

**Never modify raw data.** Raw data — the output of instruments before any processing — should be kept in original, unmodified form in a read-only repository. All processing steps should be applied to copies of the raw data using documented, versioned scripts. This ensures that if a processing error is discovered, you can return to the raw data and re-process correctly.

**What "raw data" means in different contexts:**
- **Flow cytometry:** .fcs files as exported from the cytometer
- **RNA-seq:** .fastq files as received from the sequencing facility
- **Western blot:** raw image files from the imaging system (before cropping, adjusting, or converting)
- **Plate reader:** .xlsx or .csv export from the plate reader software
- **Mass spectrometry:** .raw files from the instrument; peak tables from the instrument software

**What you should never do:**
- Open raw .fastq files in Excel (Excel auto-converts some gene names to dates, famously corrupting genomic data — a problem documented extensively, including Zeeberg et al. 2004 in Nature Genetics)
- Overwrite raw data with processed versions
- Apply brightness/contrast adjustments to raw microscopy or blot images (image manipulation is a data integrity violation in most journals)
- Delete raw data after analysis (data should be retained for at least 10 years under most funder guidelines)

## File Naming Conventions

A consistent, informative file naming convention prevents the most common data management failure: creating files named "final.csv," "final2.csv," "final_real.csv," "final_USE_THIS.csv," with no way to determine which is the correct version or when each was created.

**The recommended format:**

`YYYYMMDD_project_sample_condition_replicate.extension`

**Examples:**
- `20260505_repressilator_BL21_aTc10nM_rep1.fcs`
- `20260505_repressilator_BL21_aTc10nM_rep2.fcs`
- `20260101_FBA_ecoli_glucose_aerobic_v2.xlsx`
- `20260312_RNAseq_HEK293_CRISPR_KO_AAVS1_rep1.fastq.gz`

**Key elements:**
- **Date in ISO format (YYYYMMDD):** Sorts chronologically; unambiguous internationally (02/03 means March 2 in the US and February 3 in Europe)
- **Project identifier:** Brief project name or code
- **Sample identifier:** What biological material (organism, cell line, strain)
- **Condition:** The experimental variable (concentration, time, treatment)
- **Replicate number:** Which biological replicate (rep1, rep2, rep3)
- **No spaces or special characters in file names:** Use underscores or hyphens; spaces cause problems in command-line tools; avoid parentheses, slashes, and dots in the name (only in the extension)

## Directory Structure

A consistent directory structure across projects makes navigating between projects intuitive and allows automated processing pipelines to find files reliably.

**Recommended project structure:**

```
project_name/
  README.md              # project overview: description, data sources, how to run
  data/
    raw/                 # original, unmodified data files; organized by experiment
      20260505_expt01/
        .fcs files
        instrument_settings.txt
    processed/           # output of processing scripts; NOT raw data
      counts_matrix.csv
      normalized_expression.csv
    metadata/            # sample information tables
      sample_manifest.csv
  analysis/
    notebooks/           # Jupyter or R Markdown notebooks
      01_quality_control.ipynb
      02_differential_expression.ipynb
      03_network_analysis.ipynb
    scripts/             # standalone scripts called by notebooks
      normalize_counts.py
      fit_ode_model.R
  results/
    figures/             # figure output files (PDF, SVG, PNG)
      fig1_repressilator.pdf
    tables/              # table output files
      table1_gene_list.csv
  docs/                  # documentation
    lab_notebook.md
    protocol_versions.md
```

**The key disciplines:**
- **Never put raw data in the processed/ folder.** If a file is in raw/, it should be unmodified.
- **Never put figures in the analysis/ folder.** Figures belong in results/.
- **Version your scripts** with Git (see below). Never have scripts named `analysis_v2.py` vs `analysis_v3.py` — use version control instead.
- **Write a README.md** for every project. It should be possible for a new lab member to understand the project structure from the README alone.

## Version Control with Git

Version control — tracking changes to files over time so that any previous version can be recovered — is essential for analysis code. **Git** is the universal version control system in computational biology and software development.

**What to version control:**
- Analysis scripts (.py, .R, .m files)
- Jupyter notebooks (.ipynb files)
- Configuration files (.yaml, .json, .toml)
- Small data files and metadata tables (< 50 MB per file)
- Documentation files (.md, .txt)

**What NOT to commit to Git:**
- Large data files (> 50 MB): use Git LFS (Large File Storage) or store externally with a link in the README
- Sensitive data (patient data, personal information): never commit to a public repository
- Generated files (compiled binaries, figures generated by scripts): these can be regenerated from the code

**Minimal Git workflow:**

```bash
# Initialize a new repository
git init

# Track new or modified files
git add analysis/scripts/normalize_counts.py

# Commit with a descriptive message
git commit -m "Add log-normalization step before PCA; fixes batch effect in Fig 2"

# Push to remote repository (GitHub, GitLab, institutional server)
git push origin main
```

**Commit message conventions:** A commit message should describe *why* the change was made, not just what was changed. "Fix bug in normalization" is better than "Update script." "Switch from DESeq2 to edgeR after reviewer request; see reviewer_comments.md" is even better.

**GitHub** (github.com) and **GitLab** (gitlab.com) provide free remote repositories for public projects; institutional GitLab instances are often available for sensitive research data.

## Backup Strategy: The 3-2-1 Rule

The 3-2-1 backup rule is the standard for data protection:

- **3 copies** of all data
- **2 different media types** (e.g., internal hard drive + external hard drive; or local server + cloud)
- **1 copy off-site** (cloud storage or physical drive at a different location)

**Practical implementation for a researcher:**

| Copy | Location | Media | Update frequency |
|------|----------|-------|-----------------|
| Primary | Local computer SSD | Internal SSD | Continuous |
| Backup 1 | Lab server or NAS | Network-attached storage | Daily (automated) |
| Backup 2 | Cloud storage | Amazon S3, Google Drive, or institutional cloud | Weekly |

**Tools:**
- **rclone** (rclone.org): command-line tool for syncing local files to cloud storage (S3, Google Drive, Box, Dropbox). Supports encryption.
- **rsync**: Linux/Mac command for syncing directories to a remote server. Ideal for lab server backups.
- **Time Machine** (Mac), **Windows Backup**: local automated backup to external drive.
- **GitHub/GitLab**: automatically backs up version-controlled code to a remote server.

**For raw sequencing data:** Most genomics raw data (FASTQ files) from sequencing facilities is available for download for a limited time (30–90 days). Download promptly and store in your backup system before the facility's link expires.

## Data Sharing and Publication Requirements

Most funding agencies and journals now require data sharing:

- **NIH:** All NIH-funded research must share data, with specific requirements by data type (NIMH, NCI each have specific repositories)
- **Nature family journals:** Raw data underlying all figures must be deposited in an appropriate repository
- **Code sharing:** Most computational biology journals now require code availability for published analyses

**Appropriate repositories by data type:**
- **Sequencing data (FASTQ, BAM):** NCBI Gene Expression Omnibus (GEO), SRA (Sequence Read Archive), EBI ArrayExpress
- **Proteomics:** PRIDE (Proteomics Identifications Database)
- **Metabolomics:** MetaboLights, Metabolomics Workbench
- **General data and code:** Zenodo (zenodo.org), Figshare (figshare.com)
- **Code:** GitHub with a Zenodo DOI (Zenodo can mint a DOI for a GitHub repository snapshot)

**Best practice:** Deposit data in the appropriate repository before or concurrent with paper submission. The accession number goes in the Data Availability statement. This prevents the common scenario of a paper being published with "data available on request" — requests that are often not fulfilled.

## Takeaway

Good data management is a competitive advantage, not an overhead cost. The 3-2-1 backup rule, ISO date file naming, consistent directory structure, and Git version control for code collectively make your research more reproducible, protect against data loss, and make the paper-writing process dramatically easier. Raw data is sacred: once lost or modified, it cannot be recovered. The habits established early in a research career determine whether your data will be accessible and interpretable for the decade of follow-up work that every important result requires.
