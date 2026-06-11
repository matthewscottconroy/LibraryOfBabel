# Bridge: Tier 0 (Bedrock) → Tier 1 (Bioinformatics)

## What You Carry Forward from Tier 0

Tier 0 built a foundation across four disciplines that were, at first glance, independent. The bridge into Tier 1 is the moment when these disciplines begin to connect in the service of biological questions at the scale of the genome.

### From Mathematics
You built facility with linear algebra, differential equations, and basic probability. In Tier 1, you will use:
- **Linear algebra**: alignment algorithms are fundamentally about finding optimal paths through matrices (the Smith-Waterman scoring matrix is a form of dynamic programming on a 2D array). Principal component analysis — your primary tool for exploring RNA-seq datasets — requires matrix decomposition.
- **Probability and statistics**: every statistical test in bioinformatics (the negative binomial distribution in DESeq2, the hypergeometric test in GO enrichment) is a direct application of the probability theory from Tier 0. If the binomial and Poisson distributions are not completely clear, revisit them now.
- **What is new in Tier 1**: the scale of data. You move from fitting 4-parameter ODEs to analysing 20,000-gene expression matrices. The mathematical concepts are the same; the computational challenge is orders of magnitude larger.

### From Chemistry and Biochemistry
You built an understanding of enzyme kinetics, nucleic acid chemistry, and metabolic pathways. In Tier 1:
- **Nucleic acid chemistry** is directly operative: the physical chemistry of hydrogen bonding determines why a 20-nt guide RNA with 2 mismatches has lower affinity for its target, and why Tm-based PCR primer design matters.
- **Molecular biology** provides the interpretation layer for bioinformatics outputs. Knowing the difference between introns, exons, UTRs, and regulatory elements is not optional — it is required for interpreting alignment statistics and differential expression results.
- **What is new in Tier 1**: you move from understanding individual reactions to understanding the information encoded in biological sequences. The sequence is the primary object; biochemistry provides the context for interpreting it.

### From Biology
You built an understanding of cell biology, molecular biology, genetics, and microbiology. In Tier 1:
- **Genetics** is essential: understanding ploidy, alleles, linkage disequilibrium, and mutation rates is required for interpreting variant calling results.
- **Microbiology**: if you work with bacterial or yeast genomics, the biology of the organism — its metabolism, regulatory circuits, and ecology — provides the interpretive context for your computational results. Numbers without biology are noise.

### From Computer Science and Programming
You built Python fluency and an understanding of data structures and algorithms. In Tier 1:
- **Python** is your primary tool. Every step of the bioinformatics pipeline — from FASTQ parsing to statistical analysis in Jupyter notebooks — requires Python.
- **Algorithmic thinking**: the dynamic programming algorithms underlying sequence alignment will feel natural if you understood algorithm design in Tier 0.
- **What is new in Tier 1**: you will use command-line bioinformatics tools (STAR, featureCounts, Trimmomatic) that require Unix shell fluency. If the Unix command line is unfamiliar, invest a week before starting Tier 1 in learning: navigation, pipes, redirection, shell scripting, and job submission on HPC clusters.

---

## The Conceptual Leap Being Made

In Tier 0, you understood biology at the level of individual molecules, reactions, and cells. In Tier 1, you begin to understand biology at the level of the **genome** — the complete informational content of a cell.

This is not just a quantitative leap (from one gene to 20,000). It is a qualitative shift in how you ask questions. Instead of asking "how fast does this enzyme catalyse this reaction?", you ask "which of these 20,000 genes changes expression under this condition, and what does that tell us about the regulatory network?". Instead of "what is the Km of this enzyme?", you ask "which genetic variants in this population are associated with this phenotype, and through what molecular mechanism?"

This shift requires developing two capacities:

**1. Comfort with large-scale pattern recognition.** Bioinformatics data is high-dimensional. A gene expression matrix with 20,000 genes and 50 samples has 10^6 data points. Making sense of it requires dimensional reduction (PCA, UMAP), clustering (hierarchical clustering, k-means), and statistical filtering (differential expression testing). These tools are in your toolbox from Tier 0; using them at scale requires practice.

**2. Biological judgement.** Statistical significance (p-value) is not biological significance (importance). A gene that changes 1.5-fold with p = 10^{-20} may be less important than a gene that changes 10-fold with p = 0.01, depending on what you know about the biology. Tier 1 trains this judgement through repetition: you will encounter many results that are statistically significant but biologically uninformative, and many that are biologically striking but statistically fragile.

---

## Self-Assessment Questions

Before starting Tier 1, answer the following questions. If you cannot answer them confidently, identify which section of Tier 0 to revisit.

**Mathematics:**
1. What is the rank of a matrix, and how is it related to the number of linearly independent columns?
2. What is a p-value, and why does it need to be adjusted for multiple testing? What is the Benjamini-Hochberg procedure?
3. What is the difference between the Poisson and negative binomial distributions? When is each appropriate for count data?

**Biology:**
4. What is the central dogma of molecular biology? Describe each step (replication, transcription, translation) and where in the cell each occurs in eukaryotes.
5. What is alternative splicing, and why does it matter for RNA-seq analysis?
6. What is a genetic variant? Distinguish between SNPs, indels, and structural variants.

**Programming:**
7. Write a Python function that reads a FASTQ file (4-line format: header, sequence, +, quality) and returns the average quality score across all reads.
8. What is a Unix pipe? Give an example of a command that chains three tools together with pipes.
9. What is the difference between `grep`, `awk`, and `sed`? When would you use each?

**Bioinformatics fundamentals:**
10. What is the Smith-Waterman algorithm, and how does it differ from the Needleman-Wunsch algorithm?
11. What is a SAM file format? What information does each field contain?
12. What is the purpose of read trimming before alignment?

---

## Recommended Review if You Feel Shaky

| Topic | Review resource | Time estimate |
|-------|-----------------|---------------|
| Statistics for genomics | Irizarry & Love *Data Analysis for the Life Sciences* (free online) | 1 week |
| Unix command line | MIT OpenCourseWare 6.0001 shell scripting module | 3 days |
| Molecular biology review | Alberts *Molecular Biology of the Cell*, Chapters 6–8 | 1 week |
| Python for data science | Python Data Science Handbook (Jake VanderPlas, free online) | 1 week |

---

## A Note on Pace

Tier 1 is the most tooling-intensive tier: you will spend more time installing, configuring, and debugging computational tools than in any other tier. This is normal and expected. Do not confuse "the tool doesn't work" with "I don't understand the concept." They are different problems with different solutions. When a tool fails:
1. Read the error message carefully.
2. Check the tool's documentation and GitHub issues.
3. Check the input file format against the tool's expected format.
4. Reduce to a minimal example (a small test dataset).

Invest in a reproducible working environment: use conda environments, document your tool versions, and write shell scripts that reproduce your entire pipeline from raw data. This habit will save enormous time when you revisit your analyses or share them with others.
