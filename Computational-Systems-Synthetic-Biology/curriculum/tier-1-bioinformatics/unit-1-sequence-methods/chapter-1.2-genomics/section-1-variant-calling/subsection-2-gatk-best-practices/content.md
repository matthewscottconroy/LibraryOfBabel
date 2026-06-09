# GATK Best Practices

Imagine you have sequenced a patient's genome and want to find the mutation causing their unexplained neurological disease. You have 150-bp paired-end Illumina reads, ~30× coverage, and a reference genome. You also have, subtly embedded in your data, a serious problem: the reads contain errors. Some are sequencing errors introduced by the chemistry. Some are PCR artifacts introduced during library preparation. Some are alignment errors at repetitive regions. And some are systematic biases in the quality scores the instrument assigned to each base. If you simply count up mismatches between reads and reference, you will report thousands of false positives for every true variant.

The **GATK (Genome Analysis Toolkit)** best practices pipeline is the standard answer to this problem. Developed and maintained by the Broad Institute, it is used in large-scale sequencing projects (gnomAD, UK Biobank, All of Us) and clinical diagnostics. The pipeline combines careful error correction, probabilistic variant detection, and statistical filtering to produce well-calibrated, high-confidence variant calls. Each step has a specific purpose, and understanding that purpose is what distinguishes someone who can run the pipeline from someone who can troubleshoot it.

## Pipeline Overview

```
FASTQ reads
    ↓ BWA-MEM2
Aligned BAM
    ↓ MarkDuplicates
Deduplicated BAM
    ↓ Base Quality Score Recalibration (BQSR)
Recalibrated BAM
    ↓ HaplotypeCaller
Per-sample GVCF
    ↓ GenomicsDBImport + GenotypeGVCFs (joint genotyping)
Raw VCF (cohort)
    ↓ VQSR or hard filtering
Final filtered VCF
    ↓ Functional annotation (VEP/ANNOVAR)
Annotated variants
```

## Step 1: Mark Duplicates

PCR amplification during library preparation creates identical copies of the same original DNA fragment. These **PCR duplicates** inflate apparent coverage and must be identified and flagged (or removed) before variant calling.

```bash
gatk MarkDuplicates \
    --INPUT aligned.bam \
    --OUTPUT deduped.bam \
    --METRICS_FILE metrics.txt \
    --OPTICAL_DUPLICATE_PIXEL_DISTANCE 2500  # for patterned flowcells
```

Duplicates are identified as reads with identical start positions and orientations on both ends of a pair. They are flagged with FLAG bit 0x400 (not removed by default).

Why does this matter? Consider a heterozygous variant in a genomic region that happened to be amplified 10× during library prep. You might observe 25 reads with the reference allele and 25 with the alternate allele — but 40 of those 50 reads could be duplicates of just 5 independent molecules. The apparent 50% allele balance is real, but the statistical confidence in the call depends on the number of independent observations, not the total read count. Failing to mark duplicates inflates your confidence in variant calls without improving their accuracy.

## Step 2: Base Quality Score Recalibration (BQSR)

Sequencing instruments systematically miscalibrate base quality scores due to:
- Context-dependent sequencing errors (e.g., after GGG sequences)
- Machine-specific artifacts
- Reagent batch effects

**BQSR** corrects these systematic errors by comparing observed base quality scores to the actual error rate (estimated from sites that are known NOT to vary — e.g., dbSNP):

```bash
# Step A: build recalibration model
gatk BaseRecalibrator \
    -I deduped.bam \
    -R reference.fa \
    --known-sites dbsnp_138.hg38.vcf.gz \
    --known-sites Mills_and_1000G_gold_standard.indels.hg38.vcf.gz \
    -O recal_data.table

# Step B: apply recalibration
gatk ApplyBQSR \
    -I deduped.bam \
    -R reference.fa \
    --bqsr-recal-file recal_data.table \
    -O recalibrated.bam
```

BQSR covariates: read group, reported quality score, cycle position, and dinucleotide context. The model adjusts Phred scores to better reflect actual error probabilities.

The intuition is elegant: at sites where we know the true sequence (because they are monomorphic in dbSNP), any "mismatch" between the read and reference must be a sequencing error. By tallying these errors across thousands of positions, stratified by quality score, cycle, and context, the model learns the machine's actual error rate — then writes corrected quality scores back to the BAM file. This is empirical calibration, not a theoretical model.

## Step 3: HaplotypeCaller

**HaplotypeCaller** is GATK's primary germline variant caller. Unlike simple pileup-based callers, it performs local de-novo assembly at candidate variant regions:

1. **Active region detection**: identify genomic windows where reads show evidence of variation
2. **Local de-novo assembly**: construct a graph of possible haplotypes in the active region
3. **Read likelihood computation**: for each candidate haplotype, compute the probability that each read was generated from that haplotype using a **PairHMM** model
4. **Genotyping**: determine the most likely genotype (0/0, 0/1, 1/1) using a diploid genotyping model

**GVCF mode**: HaplotypeCaller should be run in `-ERC GVCF` mode to produce a Genomic VCF:

```bash
gatk HaplotypeCaller \
    -R reference.fa \
    -I recalibrated.bam \
    -O sample.g.vcf.gz \
    -ERC GVCF \
    --sample-name MySample
```

The GVCF format records genotype likelihoods at every position in the genome (not just variant sites), enabling accurate joint genotyping of multiple samples.

You might wonder why local assembly is necessary when you have already aligned the reads to the reference. The reason is that alignment is a local operation — it maps each read independently. But variants do not occur in isolation. A deletion in one haplotype changes the alignment of every read spanning it. HaplotypeCaller's local assembly considers all reads in a window simultaneously, constructing the most likely set of haplotypes consistent with the data. This dramatically improves accuracy in the repetitive regions and clustered variant sites where simple pileup callers fail.

## Step 4: Joint Genotyping

A key advantage of the GATK pipeline is **joint genotyping**: variant calling across all samples simultaneously. This improves sensitivity (rare variants supported in one sample can be confirmed by absence in others) and enables population-level statistics.

```bash
# Import per-sample GVCFs into a joint database
gatk GenomicsDBImport \
    --sample-name-map sample_map.txt \
    --genomicsdb-workspace-path genomicsdb/ \
    -L chr1 -L chr2 ...

# Joint genotyping
gatk GenotypeGVCFs \
    -R reference.fa \
    -V gendb://genomicsdb/ \
    -O cohort_raw.vcf.gz
```

The GVCF mode is precisely what enables this: because each per-sample GVCF records evidence at every position (not just where variants were called), the joint genotyper can go back and call a variant in sample A even if sample A alone didn't provide enough evidence — because samples B, C, and D show the same variant, and sample A shows weak but consistent signal. Joint genotyping effectively borrows statistical power across samples.

## Step 5: Variant Quality Score Recalibration (VQSR)

Raw variant calls contain both true variants and false positives. **VQSR** uses machine learning to model the distribution of true and false variants based on multiple quality annotations:

- QD (quality by depth)
- FS (Fisher strand bias)
- MQ (mapping quality)
- MQRankSum
- ReadPosRankSum

```bash
# SNP recalibration (train on HapMap, 1000G, Omni, dbSNP)
gatk VariantRecalibrator \
    -V cohort_raw.vcf.gz \
    --resource:hapmap,known=false,training=true,truth=true,prior=15.0 hapmap.vcf.gz \
    --resource:omni,known=false,training=true,truth=false,prior=12.0 omni.vcf.gz \
    --resource:1000G,known=false,training=true,truth=false,prior=10.0 1000G.vcf.gz \
    --resource:dbsnp,known=true,training=false,truth=false,prior=2.0 dbsnp.vcf.gz \
    -an QD -an MQ -an MQRankSum -an ReadPosRankSum -an FS -an SOR \
    -mode SNP -O snp_recal.recal --tranches-file snp.tranches

# Apply recalibration (filter at 99.5% sensitivity tranche)
gatk ApplyVQSR \
    -V cohort_raw.vcf.gz \
    --recal-file snp_recal.recal \
    --tranches-file snp.tranches \
    --truth-sensitivity-filter-level 99.5 \
    -mode SNP -O cohort_snp_filtered.vcf.gz
```

For small cohorts (< 30 samples), VQSR lacks sufficient data and **hard filtering** is used instead:

```bash
# SNP hard filters
gatk VariantFiltration \
    -V raw_snps.vcf.gz \
    --filter-expression "QD < 2.0" --filter-name "QD2" \
    --filter-expression "FS > 60.0" --filter-name "FS60" \
    --filter-expression "MQ < 40.0" --filter-name "MQ40" \
    -O filtered_snps.vcf.gz
```

The difference between VQSR and hard filtering captures a key tension in statistical analysis: VQSR is principled — it learns the joint distribution of quality metrics from a gold-standard training set — but it needs data. Hard filtering applies fixed thresholds that work approximately well across most datasets. For a single-sample clinical study, hard filtering is often the only option. For a population cohort of thousands, VQSR is clearly superior. Knowing which regime you are in is part of designing an appropriate pipeline.

## Genotype Quality and Depth Filters

After VQSR, additional per-genotype filters are applied:

```bash
bcftools filter \
    -e 'FORMAT/GQ < 20 || FORMAT/DP < 10' \
    -S '.' \          # Set failing genotypes to missing
    cohort_filtered.vcf.gz > final.vcf.gz
```

## Why This Matters

The GATK best practices pipeline represents the gold standard for germline variant calling because it addresses systematic errors at each step: BQSR corrects instrument bias, HaplotypeCaller's local assembly handles repetitive regions and indels more accurately than pileup methods, joint genotyping improves sensitivity and enables population-level allele frequency estimation, and VQSR provides principled statistical filtering. Understanding each step — and its failure modes — is essential for interpreting QC metrics, troubleshooting unexpected results, and adapting the pipeline to non-human organisms or non-standard protocols.

Every step of the GATK pipeline exists because a naive approach fails in a specific way. BQSR exists because instruments lie about their error rates. Duplicate marking exists because PCR inflates counts. HaplotypeCaller's local assembly exists because alignment fails at complex variants. VQSR exists because no single quality metric cleanly separates true calls from false positives. When you run this pipeline and it produces a filtered VCF, you are not running an algorithm — you are synthesizing decades of lessons learned from every misleading result that a simpler approach produced.
