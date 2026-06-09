# R for Statistical Genomics

The year is 2014. Michael Love, Wolfgang Huber, and Simon Anders publish a paper in *Genome Biology* describing DESeq2, a method for differential expression analysis of RNA-seq data. Within months it becomes the standard tool in the field — used in thousands of studies, cited tens of thousands of times. It is written in R. Two years earlier, Hadley Wickham releases ggplot2, and scientists around the world quietly stop using MATLAB for figures. The publication-quality plots that appear in *Nature*, *Cell*, and *Science* are increasingly made not in specialized graphing software but in a script that anyone can read, modify, and rerun.

This is why you need to learn R. Not because it is a better general-purpose language than Python — it is not, and Python is often cleaner for everything except statistics. R's strength is in statistical modeling, publication-quality visualization, and the Bioconductor ecosystem — the richest collection of bioinformatics statistical tools anywhere. If you do RNA-seq, single-cell genomics, GWAS, or any statistical analysis of genomic data, you will use R.

## Core R: What Matters for Bioinformatics

R's data model differs from Python in important ways:

**Vectors are the atomic unit** (not scalars; R has no true scalars — a single number is a length-1 vector):
```r
# Vectorized operations are automatic
x <- c(1.5, 2.3, 0.8, 4.1)
log2(x)          # log2 of each element
x[x > 2]         # subsetting with logical vector
which(x > 2)     # indices where condition is TRUE
```

**Factors**: Categorical variables with defined levels. Critical for statistical modeling:
```r
condition <- factor(c("control", "treated", "control", "treated"),
                    levels = c("control", "treated"))
# levels determines reference level in regression models
```

The `levels` argument deserves emphasis. When you fit a linear model in R, the first level of a factor becomes the reference — the group against which others are compared. Getting this wrong means your fold changes are backwards, your coefficients have the wrong sign, and your p-values are testing the wrong hypothesis. It happens constantly to researchers who do not think carefully about reference levels.

**Data frames**: Like Pandas DataFrames but with formula-based model syntax:
```r
# Formula: outcome ~ predictor1 + predictor2
model <- lm(expression ~ condition + batch, data = sample_data)
summary(model)
```

**Apply family**: `sapply`, `lapply`, `vapply`, `tapply` — vectorized functions over lists/vectors without explicit loops. In practice, **dplyr** largely replaces these for tabular data.

## ggplot2: Grammar of Graphics

ggplot2 is the gold standard for publication-quality scientific visualization. It implements the **grammar of graphics**: every plot is a combination of data, aesthetic mappings, geometric objects (geoms), and statistical transformations. This sounds abstract until you realize what it means in practice: once you understand the grammar, you can make any plot you can imagine by composing its elements, rather than searching for a specific function that happens to make the exact chart you need.

```r
library(ggplot2)
library(dplyr)

# MA plot: log2 fold change vs. mean expression
ma_data <- data.frame(
  baseMean = rna_results$baseMean,
  log2FC   = rna_results$log2FoldChange,
  padj     = rna_results$padj,
  sig      = rna_results$padj < 0.05 & !is.na(rna_results$padj)
)

ggplot(ma_data, aes(x = log2(baseMean + 1), y = log2FC, color = sig)) +
  geom_point(alpha = 0.4, size = 0.8) +
  scale_color_manual(values = c("grey60", "red2")) +
  geom_hline(yintercept = c(-1, 1), linetype = "dashed") +
  labs(x = "log2 Mean Expression", y = "log2 Fold Change",
       title = "MA Plot: Treated vs. Control") +
  theme_bw() +
  theme(legend.position = "none")
```

Key ggplot2 concepts:
- **aes()**: maps data columns to visual properties (x, y, color, fill, size, shape)
- **geom_***: the visual mark (geom_point, geom_line, geom_bar, geom_histogram, geom_boxplot, geom_violin, geom_density)
- **facet_wrap / facet_grid**: small multiples by a categorical variable
- **scale_***: control axis scales, color palettes (scale_color_brewer, scale_fill_viridis_c)
- **theme()**: fine-grained control of all non-data elements

## dplyr: Tidy Data Manipulation

dplyr provides a consistent grammar for data manipulation, designed for tabular (tidy) data. The key insight behind tidy data is that each row is an observation and each column is a variable — a principle that sounds obvious but dramatically simplifies the manipulation and visualization of complex datasets:

```r
library(dplyr)

# Core verbs
counts_summary <- counts_df %>%
  filter(baseMean > 10) %>%                   # keep rows where baseMean > 10
  mutate(log2FC = log2FoldChange,              # add new column
         sig = padj < 0.05 & !is.na(padj)) %>%
  group_by(gene_biotype) %>%                  # split by category
  summarize(
    n_sig = sum(sig, na.rm = TRUE),
    median_fc = median(log2FC, na.rm = TRUE)
  ) %>%
  arrange(desc(n_sig))                         # sort
```

The pipe operator `%>%` (from magrittr; also native `|>` in R 4.1+) chains operations, reading left to right. This transforms code that would otherwise require either nested function calls (unreadable) or many intermediate variables (verbose) into a clean left-to-right narrative: start with the data, filter it, transform it, group it, summarize it.

## Bioconductor: DESeq2 for Differential Expression

**DESeq2** (Love, Huber, Anders 2014) is the standard tool for RNA-seq differential expression analysis. It uses negative binomial models with shrinkage estimators for dispersion and fold change. The key biological insight motivating this model is that RNA-seq counts are overdispersed relative to a Poisson distribution — the variance exceeds the mean, because gene expression varies across cells and conditions in ways that go beyond simple counting noise:

```r
library(DESeq2)

# Input: count matrix (genes x samples) and sample metadata
dds <- DESeqDataSetFromMatrix(
  countData = count_matrix,  # integer counts, no normalization
  colData   = metadata,      # data frame: samples x covariates
  design    = ~ condition    # formula: what to test
)

# Run DESeq2 (estimates size factors, dispersions, fits GLM, tests)
dds <- DESeq(dds)

# Extract results for one comparison
res <- results(dds, contrast = c("condition", "treated", "control"),
               alpha = 0.05)  # target FDR

# Shrink log2FC (for ranking/visualization — reduces noise for low-count genes)
res_shrunk <- lfcShrink(dds, contrast = c("condition", "treated", "control"),
                         type = "ashr")

# Summary
summary(res)
# Number of significant genes (FDR < 0.05)
sum(res$padj < 0.05, na.rm = TRUE)
```

**The DESeq2 statistical model**:
For gene $g$ in sample $j$ with condition $c_j$:

$$K_{gj} \sim \text{NegBin}(\mu_{gj},\ \alpha_g)$$
$$\mu_{gj} = s_j \cdot q_{gj},\quad \log_2(q_{gj}) = \beta_{g0} + \beta_{g1} x_j$$

where $s_j$ is the size factor (normalization), $\alpha_g$ is the gene-specific dispersion, and $\beta_{g1}$ is the log2 fold change tested against 0.

The `lfcShrink` step is important and frequently misunderstood. For genes with very low counts, the naive log2 fold change estimate has enormous variance — a gene expressed at 1 count in one condition and 2 in another has an estimated fold change of 2, but this estimate is unreliable. Shrinkage pulls these noisy estimates toward zero, producing fold changes that are more biologically interpretable and better-ranked when you sort by effect size.

## GenomicRanges: Interval Arithmetic for Genomics

**GenomicRanges** is the Bioconductor infrastructure for genomic intervals. The key insight is that most genomics analyses are really questions about intervals: which ChIP-seq peaks overlap which gene promoters? Which variants fall inside exons? Which ATAC-seq open chromatin regions are near transcription factor binding motifs? These are set-theoretic questions about intervals on chromosomes, and GenomicRanges gives you the vocabulary to ask them:

```r
library(GenomicRanges)

# Create GRanges from a BED-like data frame
peaks <- GRanges(
  seqnames = Rle(c("chr1", "chr1", "chr2"), c(2, 1, 1)),
  ranges   = IRanges(start = c(100, 500, 1000, 200),
                     end   = c(300, 700, 1500, 400)),
  strand   = c("+", "-", "+", "*")
)

# Find overlaps with gene annotations
genes <- import("annotations.gtf")  # rtracklayer
overlaps <- findOverlaps(peaks, genes)

# Resize peaks symmetrically around center (common for motif analysis)
peaks_centered <- resize(peaks, width = 500, fix = "center")
```

GenomicRanges operations: `findOverlaps`, `subsetByOverlaps`, `reduce`, `gaps`, `flank`, `resize`, `coverage` — the building blocks of any ChIP-seq, ATAC-seq, or RNA-seq peak analysis pipeline.

## Why This Matters for Computational Biology

R and Bioconductor host the definitive implementations of the statistical methods for genomics. DESeq2, edgeR, and limma are the three gold-standard tools for differential expression — knowing how to use and interpret them, and understanding the underlying statistical models, is non-negotiable for anyone analyzing RNA-seq data. ggplot2 produces the publication-quality figures that appear in journals — learning it properly (not just copying examples) means being able to create any visualization you need. GenomicRanges makes genomic interval operations as natural as set operations on numbers. The R/Bioconductor ecosystem is updated with new methods (often published in journals like *Genome Biology* and *Bioinformatics*) that typically appear in R before any other language — fluency in R means you can use the state of the art.
