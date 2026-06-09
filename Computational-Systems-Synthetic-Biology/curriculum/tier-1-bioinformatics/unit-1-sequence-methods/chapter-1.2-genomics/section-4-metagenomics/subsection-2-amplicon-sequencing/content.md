# Amplicon Sequencing (16S/18S/ITS)

Before metagenomics was computationally tractable, microbial ecologists faced an uncomfortable choice: culture organisms (missing 99% of diversity) or develop a method that could characterize communities without cultivation. The solution came from an unlikely source — ribosomal RNA.

The 16S ribosomal RNA gene has a property that is almost unique in biology: it is essential in every living cell, it is ancient enough to be present in all bacteria and archaea, and it contains both ultra-conserved regions (invariant enough for universal primer design) and hypervariable regions (divergent enough to distinguish species). Carl Woese recognized in the 1970s that 16S rRNA sequences were the universal phylogenetic clock, and used them to establish that life on Earth falls into three domains — Bacteria, Archaea, and Eukarya — overturning a century of biological classification. The same properties that made 16S useful for phylogenetics make it ideal as a taxonomic marker for environmental surveys.

By designing PCR primers to the conserved flanking regions and sequencing the hypervariable regions between them, you can amplify and sequence the 16S genes from every bacterium in a complex community simultaneously. The result is a snapshot of community composition — which organisms are present and in what relative proportions — obtained from a tiny amount of sample, at low cost, and without any culturing.

**Amplicon sequencing** targets a specific phylogenetically informative genomic region — the 16S rRNA gene for bacteria/archaea, 18S rRNA for eukaryotes, ITS for fungi — to characterize microbial community composition at high throughput and low cost. It is the most widely used method for microbiome studies and environmental microbiology.

## The 16S rRNA Gene as a Phylogenetic Marker

The 16S rRNA gene is ~1,550 bp and consists of **conserved regions** (identical across nearly all bacteria, used for PCR primer design) flanking **variable regions** (V1–V9) that differ across taxa. The variable regions provide taxonomic resolution:

- **V3-V4** (most common): good balance of resolution and amplifiability; 400–500 bp amplicon
- **V4** alone: shorter amplicon (250 bp), very high throughput; lower resolution at genus level
- **V1-V3**: better resolution for some lineages; less amplifiable

**Resolution limits**: 16S typically resolves to genus level; species-level discrimination requires > 99% identity threshold and often multiple markers. Strain-level resolution is not possible.

The choice of variable region is not trivial and affects the biological conclusions you can draw. V4 is the workhorse of large-scale studies like the Human Microbiome Project and American Gut Project because its 250 bp length fits neatly on a MiSeq paired-end run. But its lower resolution means that "Bacteroides" in your sample might be several different species with very different ecological roles and metabolic activities. V3-V4 provides better species-level discrimination at the cost of higher cost per sample. For clinical diagnostics where species identity matters, this tradeoff often favors V3-V4 or full-length 16S sequencing with long-read technology.

## DADA2: Denoising to ASVs

**DADA2 (Divisive Amplicon Denoising Algorithm 2)** models sequencing error to produce **Amplicon Sequence Variants (ASVs)** — exact biological sequences rather than clustered OTUs:

```r
library(dada2)

# Input: primer-trimmed, quality-filtered paired-end FASTQ files
# Forward and reverse reads sorted in matching order

path <- "fastq_trimmed/"
fnFs <- sort(list.files(path, pattern="_R1_001.fastq.gz"))
fnRs <- sort(list.files(path, pattern="_R2_001.fastq.gz"))
sample.names <- sub("_R1_001.fastq.gz", "", fnFs)

# Quality profiles
plotQualityProfile(file.path(path, fnFs[1:2]))
plotQualityProfile(file.path(path, fnRs[1:2]))

# Filter and trim
out <- filterAndTrim(fnFs, filtFs, fnRs, filtRs,
                     truncLen=c(230, 200),   # truncate at these lengths
                     maxN=0, maxEE=c(2,2),
                     truncQ=2, rm.phix=TRUE,
                     compress=TRUE, multithread=TRUE)

# Learn error rates (key DADA2 step)
errF <- learnErrors(filtFs, multithread=TRUE)
errR <- learnErrors(filtRs, multithread=TRUE)
plotErrors(errF, nominalQ=TRUE)

# Dereplicate and denoise
dadaFs <- dada(filtFs, err=errF, multithread=TRUE)
dadaRs <- dada(filtRs, err=errR, multithread=TRUE)

# Merge paired reads
mergers <- mergePairs(dadaFs, filtFs, dadaRs, filtRs, verbose=TRUE)

# Make ASV table
seqtab <- makeSequenceTable(mergers)
dim(seqtab)  # samples × ASVs

# Remove chimeras
seqtab.nochim <- removeBimeraDenovo(seqtab, method="consensus", multithread=TRUE)

# Taxonomic assignment
taxa <- assignTaxonomy(seqtab.nochim, "silva_nr_v138.1_train_set.fa.gz", multithread=TRUE)
taxa <- addSpecies(taxa, "silva_species_assignment_v138.1.fa.gz")
```

**ASVs vs. OTUs**: traditional OTU clustering at 97% identity conflates different sequences; DADA2 ASVs represent exact sequences and can distinguish strains differing by a single nucleotide.

The DADA2 error model is worth understanding conceptually. Sequencing errors occur with a frequency that depends on the quality score, the cycle position, and the preceding sequence context. DADA2 learns these dependencies from the data itself — by examining the relationship between quality scores and actual error rates across the millions of reads in your sample. Once the error model is learned, it can distinguish biological sequence variants (real ASVs present in nature) from sequencing errors (artifacts that would not appear if you sequenced the same sample again). The result is a cleaner, more reproducible feature table than OTU clustering produces — ASVs from one study can be directly compared to ASVs from another without re-clustering.

## QIIME2: Pipeline Framework

**QIIME2** provides a plugin-based framework for amplicon analysis:

```bash
# Import FASTQ data
qiime tools import \
    --type 'SampleData[PairedEndSequencesWithQuality]' \
    --input-path manifest.csv \
    --output-path demux.qza \
    --input-format PairedEndFastqManifestPhred33

# Trim primers with cutadapt
qiime cutadapt trim-paired \
    --i-demultiplexed-sequences demux.qza \
    --p-front-f GTGYCAGCMGCCGCGGTAA \  # 515F primer
    --p-front-r GGACTACNVGGGTWTCTAAT \ # 806R primer
    --o-trimmed-sequences trimmed.qza

# Denoise with DADA2
qiime dada2 denoise-paired \
    --i-demultiplexed-seqs trimmed.qza \
    --p-trim-left-f 0 --p-trim-left-r 0 \
    --p-trunc-len-f 230 --p-trunc-len-r 200 \
    --o-table table.qza \
    --o-representative-sequences rep-seqs.qza \
    --o-denoising-stats stats.qza \
    --p-n-threads 16

# Taxonomy classification
qiime feature-classifier classify-sklearn \
    --i-classifier silva-138-99-515-806-nb-classifier.qza \
    --i-reads rep-seqs.qza \
    --o-classification taxonomy.qza

# Alpha diversity
qiime diversity alpha \
    --i-table table.qza \
    --p-metric shannon \
    --o-alpha-diversity shannon.qza

# Beta diversity
qiime diversity beta \
    --i-table table.qza \
    --p-metric braycurtis \
    --o-distance-matrix bc_matrix.qza
```

## Diversity Metrics

**Alpha diversity** (within-sample diversity):

| Metric | Description |
|--------|-------------|
| Observed ASVs | Count of unique ASVs |
| Shannon index ($H'$) | $-\sum p_i \ln p_i$ ; accounts for evenness |
| Simpson's index | $1 - \sum p_i^2$ ; less sensitive to rare species |
| Faith's PD | Phylogenetic diversity; total branch length of observed ASVs |
| Chao1 | Estimated true richness accounting for unobserved rare species |

$$H' = -\sum_{i=1}^{S} p_i \ln(p_i)$$

where $p_i$ = relative abundance of ASV $i$ and $S$ = number of ASVs.

**Beta diversity** (between-sample diversity):

- **Bray-Curtis dissimilarity**: $BC_{ij} = \frac{\sum |a_k - b_k|}{\sum (a_k + b_k)}$ — unweighted by phylogeny
- **Weighted UniFrac**: phylogeny-weighted abundance dissimilarity — preferred for microbiome studies
- **Unweighted UniFrac**: presence/absence only; sensitive to rare taxa

Visualized by **PCoA (Principal Coordinates Analysis)** — an ordination method that places samples in 2D/3D space such that similar communities cluster together.

The choice between Bray-Curtis and UniFrac metrics captures a real biological question: do you care about phylogenetic relatedness when measuring community dissimilarity? Bray-Curtis treats every ASV as equally different from every other, so losing one common ASV and gaining another has the same effect regardless of whether they are close or distant relatives. Weighted UniFrac accounts for phylogeny: losing a species with many close relatives matters less than losing one on a long, isolated branch of the tree of life. For questions about ecosystem function and redundancy, UniFrac is often more appropriate. For clinical applications where the presence or absence of specific taxa matters directly, Bray-Curtis may capture the biologically relevant signal more cleanly.

## Differential Abundance Analysis

```r
library(DESeq2)
library(phyloseq)

# Convert phyloseq object to DESeq2
ps_deseq <- phyloseq_to_deseq2(physeq, ~ Condition)
ps_deseq <- DESeq(ps_deseq, test="Wald", fitType="parametric")

# Results: which taxa differ between conditions?
res <- results(ps_deseq, contrast=c("Condition", "Disease", "Control"))
sig_taxa <- res[which(res$padj < 0.05), ]
```

**ANCOM-BC**: addresses compositional nature of microbiome data (all samples sum to 1 — absolute counts are unknown):
```r
library(ANCOMBC)
result = ancombc(phyloseq = physeq, formula = "Condition",
                 p_adj_method = "BH", zero_cut = 0.90)
```

The compositionality problem is one of the most important and least appreciated issues in microbiome statistics. Because 16S sequencing produces relative abundances (not absolute counts), if one taxon increases in abundance, all other taxa appear to decrease — even if their actual cell counts are unchanged. This creates spurious negative correlations throughout the data and makes standard statistical tests inappropriate. ANCOM-BC addresses this by estimating the true log-fold changes while accounting for the unknown sampling fractions. Using DESeq2 on compositional data is formally incorrect (it assumes absolute counts), though in practice it often produces similar results — which is why the field is still debating best practices.

## Why This Matters

16S amplicon sequencing is the workhorse of microbiome science. It has been applied to characterize the gut microbiome in hundreds of diseases (IBD, obesity, autism, cancer), the soil microbiome in agricultural systems, and the ocean microbiome across latitudinal gradients. Understanding DADA2's denoising approach (error model + chimera removal) versus OTU clustering explains why modern studies produce reproducible ASVs that can be compared across studies without re-sequencing. The choice of diversity metric — Faith's PD vs. Bray-Curtis vs. UniFrac — determines what biological signal is detected and must be matched to the research question.

The transition from OTUs to ASVs deserves to be recognized as a genuine methodological advance. OTU clustering at 97% identity was the dominant approach for fifteen years because it seemed to match species-level resolution, it was computationally tractable, and it reduced sequencing noise. But 97% identity OTUs lump genuinely distinct organisms together, they are not reproducible across studies (because clustering depends on the composition of the database being analyzed), and they obscure strain-level variation. DADA2 ASVs are exact sequences, reproducible across studies, comparable without re-clustering, and capable of resolving strain-level variation that OTUs would miss. This is the kind of methodological improvement that seems incremental but changes what questions you can ask with the data.
