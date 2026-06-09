# Somatic Variant Calling

Every cell in your body carries essentially the same germline genome — the sequence you inherited from your parents. But the genome in your cells is not static. Each time a cell divides, replication introduces approximately one new mutation. Over a lifetime, a typical somatic cell accumulates dozens to hundreds of mutations; a stem cell with rapid division history may accumulate thousands. In most cells, this is biologically inconsequential. But occasionally, a mutation lands in a driver gene — TP53, KRAS, EGFR, BRCA1 — and begins pushing the cell toward uncontrolled growth.

Cancer is, fundamentally, a disease of somatic mutation. Tumor cells have accumulated somatic variants that their neighboring normal cells do not carry. Detecting those variants — distinguishing the mutations that arose in the tumor from the germline variants the patient was born with — is the central challenge of cancer genomics. It is also technically much harder than germline variant calling, for reasons that are built into the biology of cancer itself.

**Somatic variants** arise de novo in individual somatic cells during an organism's lifetime, not in the germline. In cancer, somatic mutations in driver genes confer growth advantages that enable tumor progression. Somatic variant calling differs fundamentally from germline calling: instead of comparing an individual to a population reference, it compares a tumor to its matched normal tissue to find variants present only in the tumor.

## Why Somatic Calling Is Different

**Tumor heterogeneity**: a tumor mass contains subclonal populations with different mutation profiles. A somatic variant may be present in 10% or 80% of tumor cells, unlike germline variants which are present in (nearly) 100% of cells.

**Variant allele fraction (VAF)**: the fraction of reads at a position carrying the alternate allele. For germline: expected 50% (heterozygous) or 100% (homozygous). For somatic: can range from < 5% to 100% depending on tumor purity and clonality.

**No reference population**: somatic variants cannot be filtered against population databases for germline variants. A variant absent from gnomAD is not necessarily somatic.

This last point is easy to overlook but critical. The standard germline pipeline uses population frequency to filter out common benign variants — if a variant is in 40% of the population, it is almost certainly not causing disease in your patient. But somatic variants are, by definition, not in the population database. You cannot use population frequency to distinguish a true somatic variant from a sequencing artifact in a new position. You need a different strategy entirely.

## Tumor-Normal Paired Analysis

The gold standard: sequence both tumor DNA and matched normal (blood or adjacent normal tissue) from the same patient.

$$\text{Somatic variant} = \text{variant present in tumor} - \text{variant present in normal}$$

This controls for:
- Germline variants present in both samples (most common variants)
- Mapping artifacts (same in both samples)
- Patient-specific SNPs

```bash
# Mutect2: GATK somatic caller (tumor-normal mode)
gatk Mutect2 \
    -R reference.fa \
    -I tumor.bam \
    -I normal.bam \
    -tumor TumorSample \
    -normal NormalSample \
    --germline-resource gnomad.vcf.gz \
    --panel-of-normals pon.vcf.gz \
    -O raw_somatic.vcf.gz

# Filter somatic calls
gatk FilterMutectCalls \
    -V raw_somatic.vcf.gz \
    -R reference.fa \
    -O filtered_somatic.vcf.gz
```

**Panel of Normals (PoN)**: a VCF compiled from many normal samples. Variants appearing in the PoN are likely sequencing artifacts or germline variants, not somatic mutations. Using a PoN dramatically reduces false positives.

The PoN is one of those practical engineering solutions that looks ad hoc but is deeply principled. Systematic sequencing artifacts — caused by specific sequence contexts, GC content, instrument chemistry, or library preparation — appear repeatedly across samples. A variant that shows up as a low-level "hit" in 20 different normal samples is almost certainly not a real mutation in any of them. By blacklisting these recurrent artifact sites, the PoN filters thousands of false positives without requiring any model of why those particular artifacts occur.

## Mutect2 Statistical Model

Mutect2 uses a Bayesian somatic genotyping model. For each candidate site, it computes:

$$\text{TLOD} = \log\frac{P(\text{data} | \text{variant present in tumor})}{P(\text{data} | \text{no variant})}$$

$$\text{NLOD} = \log\frac{P(\text{data in normal} | \text{no variant})}{P(\text{data in normal} | \text{variant present})}$$

High TLOD (variant evidence in tumor) and high NLOD (no evidence in normal) together indicate a somatic variant.

The dual log-odds structure is elegant: a true somatic variant should have high evidence in the tumor and high evidence of absence in the normal. These are independent evidence streams that together produce high confidence. A germline variant in both samples will have high TLOD but low NLOD (there is variant evidence in the normal too). An artifact will have neither: it may show some signal in the tumor but the pattern of supporting reads will be inconsistent with a true variant.

## Variant Allele Fraction Analysis

```python
import pandas as pd
import matplotlib.pyplot as plt

# Parse VAF from a somatic VCF
# AF field in FORMAT/INFO contains tumor allele fraction

def get_vaf(vcf_line):
    format_fields = vcf_line['FORMAT'].split(':')
    sample_fields = vcf_line['TUMOR'].split(':')
    field_dict = dict(zip(format_fields, sample_fields))
    return float(field_dict.get('AF', 0))

# VAF distribution reveals tumor heterogeneity
vafs = [get_vaf(var) for var in somatic_variants]

plt.hist(vafs, bins=50, edgecolor='black')
plt.xlabel('Variant Allele Fraction')
plt.ylabel('Count')
plt.title('Somatic VAF Distribution')
plt.axvline(x=0.5, color='red', linestyle='--', label='Expected clonal (diploid, pure)')
```

VAF distribution patterns:
- Single peak at 0.5: clonal tumor, diploid, high purity
- Multiple peaks: subclonal mutations at different cancer cell fractions
- Very low VAF peaks (< 0.1): either very low purity or rare subclonal mutations

The VAF histogram is, in a real sense, a portrait of tumor evolution. A clean single peak at 0.5 describes a simple, clonal tumor that arose from a single cell and expanded uniformly. Multiple peaks reveal a tumor with subclones: an early "trunk" mutation present in all cells at high VAF, then "branch" mutations acquired later in only some cells at lower VAF. Reconstructing the clonal architecture from VAF data — a field called tumor phylogenetics — can reveal the order in which mutations were acquired and the selective dynamics that drove tumor evolution.

## Tumor Mutational Burden (TMB)

**TMB** is the number of somatic mutations per megabase of genome sequenced. High TMB indicates a "hypermutator" phenotype:

$$\text{TMB} = \frac{\text{number of somatic coding mutations}}{\text{megabases sequenced}}$$

- Low: < 5 mut/Mb (e.g., pediatric cancers, prostate)
- Intermediate: 5–20 mut/Mb
- High: > 20 mut/Mb (e.g., POLE-mutant colorectal, melanoma)

TMB is a biomarker for response to immune checkpoint inhibitors (pembrolizumab FDA approval for TMB-high tumors).

The biological logic behind TMB as an immunotherapy biomarker is compelling: high mutation burden means more neoantigens — novel peptides encoded by somatic mutations that the immune system has never seen before. Tumors with many neoantigens are more immunogenic and therefore more responsive to checkpoint blockade that unleashes T cells to attack them. A melanoma with 50 mutations per megabase (from UV damage to DNA) is immunologically a very different target than a pediatric brain tumor with 0.5 mutations per megabase.

## Mutational Signatures

Somatic mutations arise from specific mutational processes, each leaving a characteristic pattern (signature) in the spectrum of base substitutions and their trinucleotide context.

**COSMIC mutational signatures**: the Catalogue of Somatic Mutations in Cancer provides 60+ reference signatures:
- **SBS1**: age-related (deamination of methylated cytosines)
- **SBS4**: tobacco smoking (C>A transversions, typically in lung cancer)
- **SBS7a/b**: UV light (C>T at dipyrimidines, melanoma)
- **SBS6/15**: mismatch repair deficiency (MSI)
- **SBS10a/b**: POLE exonuclease mutations (ultramutator phenotype)

```python
# Decompose tumor mutations into COSMIC signatures using SigProfiler
from SigProfilerAssignment import Analyzer as SPA
SPA.decompose_fit(samples="mutations.vcf", output="output/",
                  genome_build="GRCh38",
                  signatures_database="COSMIC_v3.3.1_SBS_GRCh38.txt")
```

Mutational signatures are among the most remarkable results of large-scale cancer sequencing. The notion that a tumor's mutation spectrum could literally identify its cause — that you could look at a lung cancer genome and read "this patient smoked tobacco" in the pattern of C>A transversions — was not anticipated before TCGA-scale analyses were available. SBS4 is as recognizable as a fingerprint, and its presence in a never-smoker's lung tumor raises immediate questions about passive smoke exposure, radon, or a novel mutagen. The signatures are, in a very real sense, the scars that life leaves on the genome.

## Copy Number Alterations in Cancer

Somatic CNVs (amplifications and deletions) are as common as point mutations in cancer:

- **Oncogene amplification**: ERBB2 in breast cancer, MYC in multiple tumor types
- **Tumor suppressor deletion**: TP53, RB1, CDKN2A

```bash
# CNVkit: coverage-based CNV calling in tumor-normal pairs
cnvkit.py batch tumor.bam --normal normal.bam \
    --targets targets.bed --fasta reference.fa \
    --output-reference cnv_ref.cnn \
    --output-dir cnvkit_output/
```

## Why This Matters

Somatic variant calling drives precision oncology: identifying driver mutations (KRAS G12C, EGFR exon 19 deletions, BRAF V600E) guides targeted therapy selection. Mutational signatures inform etiology and predict prognosis. TMB predicts immunotherapy response. The analytical challenges — handling low VAF, tumor heterogeneity, copy number alterations, and tumor purity — make somatic calling significantly more complex than germline variant calling. Understanding the Mutect2 statistical model, the role of the panel of normals, and the interpretation of VAF distributions is essential for anyone working with cancer genomics data.

The practical payoff is direct and immediate. A patient with non-small cell lung cancer whose tumor carries an EGFR exon 19 deletion responds dramatically to erlotinib — a drug that would be no more effective than placebo in a tumor without that mutation. Finding that deletion, reliably, from a tumor biopsy with 60% tumor purity and 120× coverage, requires every piece of the somatic calling pipeline to work correctly. That is why this material matters: it stands between a tissue sample and a therapy decision.
