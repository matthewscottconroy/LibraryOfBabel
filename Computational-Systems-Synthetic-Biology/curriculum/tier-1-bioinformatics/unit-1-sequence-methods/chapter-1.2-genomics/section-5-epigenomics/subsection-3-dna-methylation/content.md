# DNA Methylation

In the early 1990s, a puzzling observation had accumulated in the genetics of cancer: some tumors had silenced the same tumor suppressor genes — RB1, CDKN2A, MLH1 — without any detectable mutation in those genes' sequences. The coding regions were intact, the splice sites were functional, but the genes were completely silent. The explanation, when it emerged, was methylation: the promoter CpG islands of these genes had acquired methyl groups, and methylated promoters are transcriptionally silenced.

This finding established epigenetic silencing — gene inactivation without genetic change — as a central mechanism in cancer biology. It also implied something with profound consequences for aging, environmental health, and heritability: the genome can be functionally altered without a single nucleotide changing. Environmental exposures that alter DNA methylation patterns can silence tumor suppressor genes, activate oncogenes, or dysregulate imprinted loci. And unlike most mutations, DNA methylation patterns are potentially reversible — which is why DNMT inhibitors like azacitidine are used clinically to reactivate silenced tumor suppressor genes in myeloid malignancies.

**DNA methylation** is the addition of a methyl group to the 5-position of cytosine (5-methylcytosine, 5mC) or, less commonly, N6 of adenine (6mA in bacteria and some eukaryotes). In mammals, CpG dinucleotide methylation is the primary epigenetic mark associated with gene silencing, X-chromosome inactivation, genomic imprinting, and transposable element suppression. Bisulfite sequencing remains the gold standard for genome-wide methylation profiling.

## Biology of CpG Methylation

**CpG dinucleotides**: cytosine followed by guanine. Methylation occurs almost exclusively at CpGs in mammals (with notable non-CpG methylation in embryonic stem cells and neurons).

**CpG islands**: regions of > 200 bp with CpG frequency close to expected (~60% of promoters overlap CpG islands). CpG islands are predominantly unmethylated regardless of gene expression status. However, when CpG island promoters become methylated (e.g., in cancer), associated genes are silenced.

**Functional roles**:
- **Gene body methylation**: positively correlated with transcription; suppresses spurious transcription initiation from repetitive elements within genes
- **Promoter methylation**: inversely correlated with transcription; silences genes
- **Imprinting**: allele-specific methylation marks parental origin (IGF2/H19, SNRPN)
- **X-chromosome inactivation**: methylation of CpG islands on the inactive X

**Maintenance methylation**: DNMT1 copies methylation patterns to newly synthesized strands after replication (using the hemimethylated template). DNMT3A/B establish de novo methylation. TET enzymes oxidize 5mC to 5-hydroxymethylcytosine (5hmC) as the first step in active demethylation.

The distinction between DNMT1 (maintenance) and DNMT3A/B (de novo) is functionally important. During early development, the genome undergoes a dramatic wave of demethylation followed by de novo methylation that re-establishes cell-type-specific patterns. DNMT3A and DNMT3B are responsible for this re-establishment; mutations in both are common in hematological malignancies. DNMT1 then maintains these patterns through every subsequent cell division. The entire epigenetic memory of cell identity, encoded as methylation patterns, is faithfully copied every time a cell divides — a remarkable feat of molecular bookkeeping.

## Bisulfite Sequencing Principle

Bisulfite treatment converts unmethylated cytosine (C) to uracil (U), which reads as thymine (T) after PCR amplification. Methylated cytosines (5mC) are protected and remain as cytosine:

$$\text{Unmethylated C} \xrightarrow{\text{bisulfite}} \text{U} \xrightarrow{\text{PCR}} \text{T}$$
$$\text{Methylated C (5mC)} \xrightarrow{\text{bisulfite}} \text{C} \xrightarrow{\text{PCR}} \text{C}$$

By comparing the bisulfite-converted sequence to the reference, we determine the methylation status at each CpG:
- Read shows C at CpG position → methylated
- Read shows T at CpG position → unmethylated

**Methylation level** at a CpG:

$$\beta = \frac{\text{methylated reads}}{\text{methylated reads} + \text{unmethylated reads}}$$

$\beta$ ranges from 0 (fully unmethylated) to 1 (fully methylated).

The bisulfite chemistry is chemically harsh — it degrades a substantial fraction of DNA during treatment — which is why WGBS requires high-quality input DNA and why damaged samples (FFPE, ancient DNA) are challenging. It also cannot distinguish 5mC from 5-hydroxymethylcytosine (5hmC), since both are protected from bisulfite conversion. In tissues with high 5hmC levels (notably neurons and embryonic stem cells), WGBS overestimates true methylation. Special protocols (oxidative bisulfite sequencing, TAB-seq) are needed to distinguish the two marks.

## WGBS: Whole Genome Bisulfite Sequencing

WGBS provides single-base resolution methylation data across the entire genome:

```bash
# Bismark: bisulfite-aware aligner
# Step 1: Build bisulfite genome index
bismark_genome_preparation --parallel 4 genome/

# Step 2: Align bisulfite-treated reads
bismark --genome genome/ \
    --paired_end R1.fastq.gz R2.fastq.gz \
    --non_directional \  # For non-directional libraries
    --score_min L,0,-0.6 \
    -p 4 -o bismark_output/

# Step 3: Deduplicate
deduplicate_bismark --paired bismark_output/*.bam

# Step 4: Extract methylation information
bismark_methylation_extractor \
    --paired-end --comprehensive \
    --CX_context \  # Report all cytosine contexts (CpG, CHG, CHH)
    --genome_folder genome/ \
    --output bismark_output/ \
    bismark_output/*_deduplicated.bam

# Output: CpG_context_sample.txt with columns:
# read_id, strand, chromosome, position, methylation_call (Z=methylated, z=unmethylated)
```

**Coverage requirement**: 10–15× per strand (30× total) for reliable methylation estimates. CpGs covered by fewer than 5 reads have unreliable β values.

## RRBS: Reduced Representation Bisulfite Sequencing

RRBS uses MspI (cuts at C^CGG) to enrich for CpG-rich regions:

```bash
# Adapter trimming with RRBS-specific settings
trim_galore --rrbs R1.fastq.gz R2.fastq.gz

# Same alignment and extraction as WGBS
```

RRBS advantages: lower cost, higher CpG coverage at covered sites. Disadvantages: only covers ~5–10% of CpGs (biased toward CpG islands).

RRBS captures the most informative fraction of the methylome — CpG islands and their shores — at a fraction of the cost of WGBS. For studies where promoter methylation and gene regulation are the primary questions, this coverage is often sufficient. For studies investigating non-CpG methylation, intergenic methylation, or the full landscape of methylation variation including gene body and repetitive element methylation, WGBS is necessary. The choice depends on your scientific question and your sequencing budget.

## Methylation Array (Illumina EPIC)

The **Illumina Infinium EPIC array** measures DNA methylation at ~850,000 CpG sites using bead-chip technology:

- Much cheaper than WGBS (~$300/sample vs. ~$1000+)
- Limited to pre-defined CpG probes (biased toward promoters and CpG islands)
- Provides β values (0–1) for each probe

```r
library(minfi)
library(limma)

# Load EPIC array data
rgset <- read.metharray.exp("idat_files/")
mset <- preprocessQuantile(rgset)  # Quantile normalization

# Get M-values (logit(beta); better for statistical testing)
M_values <- getM(mset)
beta_values <- getBeta(mset)

# Differential methylation analysis
design <- model.matrix(~ Condition, data=pData(mset))
fit <- lmFit(M_values, design)
fit <- eBayes(fit)
dmps <- topTable(fit, coef=2, n=Inf, adjust.method="BH")
sig_dmps <- dmps[dmps$adj.P.Val < 0.05 & abs(dmps$logFC) > 1, ]

# DMR (Differentially Methylated Region) calling
library(DMRcate)
myannotation <- cpgAnnotation(siglevel=0.05, arraytype="EPIC")
dmrcate_results <- DMRcate(myannotation)
extractRanges(dmrcate_results)
```

The M-value transformation deserves brief explanation. Beta values (0–1) are biologically intuitive but statistically poorly behaved: they are bounded and their variance is highest at intermediate values (near 0.5) and lowest near the boundaries. M-values are the logit transformation of beta values, $M = \log_2(\beta/(1-\beta))$, which produces an unbounded, approximately normally distributed variable — much better suited for linear modeling. Use beta values for visualization and biological interpretation; use M-values for statistical testing.

## Bisulfite Conversion Efficiency

Incomplete conversion (unmethylated C not converted to T) inflates apparent methylation. Always check:
- **Spike-in controls** (unmethylated lambda phage DNA): should show > 99.5% conversion
- **CHH context methylation**: non-CpG methylation should be < 1% in most somatic cells; high CHH methylation indicates incomplete conversion

## Why This Matters

DNA methylation is a stable epigenetic mark that integrates developmental history, environmental exposures, and aging. **Epigenetic clocks** (Horvath clock) use CpG methylation patterns to estimate biological age with remarkable accuracy. **Epigenome-wide association studies (EWAS)** identify CpG methylation differences associated with exposures (smoking, diet) and diseases (cancer, psychiatric disorders). In cancer, promoter hypermethylation silences tumor suppressor genes (MLH1, CDKN2A/p16, BRCA1) in the absence of genetic mutation — creating a second pathway to loss of function. Bisulfite sequencing and array methylation analysis are standard tools in clinical epigenomics and fundamental to understanding how environment shapes gene expression through epigenetic mechanisms.

The epigenetic clock is perhaps the most conceptually striking application of DNA methylation profiling. The Horvath clock — trained on methylation data from hundreds of tissue types — can predict biological age from blood DNA to within a few years. More remarkably, accelerated epigenetic aging (clock age exceeding chronological age) is associated with increased all-cause mortality, cancer risk, and cardiovascular disease risk. This implies that the methylation patterns encoding the epigenetic clock are somehow causally linked to — or at least correlated with — the biological processes that drive aging. Whether the clock reflects aging or drives it remains an active area of research, but the precision with which genome-wide methylation patterns encode biological age suggests that DNA methylation is far more than an accessory epigenetic mark. It is a molecular record of a cell's life.
