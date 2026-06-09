# End-to-End Genomics Workflow

The sequencer produces FASTQ files. Inside each file are millions of short strings — 150 characters, on average, each representing a read of DNA that some instrument recorded after fluorescence chemistry and base-calling algorithms processed the light from a flow cell. These strings are the raw data of genomics. They are also, by themselves, nearly useless.

A genomics experiment does not end when the sequencer produces FASTQ files — that is where the real analytical work begins. Between raw base calls and an interpretable result — a list of variants in a cancer patient's tumor, a set of differentially expressed genes, a catalog of structural variants in a population cohort — lies a pipeline with six or more steps, each designed to address a specific failure mode of the previous approach. Skipping a step, running it with wrong parameters, or misinterpreting its output can propagate errors silently through the entire analysis. The final VCF file will look plausible whether your pipeline ran correctly or not.

A well-designed computational pipeline transforms raw base calls into biologically interpretable variants, copy number changes, or structural rearrangements. This section walks through each stage of a canonical short-read whole-genome or exome sequencing pipeline, explains the purpose of every step, and sketches how a workflow manager ties them together.

## Stage 1: Quality Control with FastQC and MultiQC

Every pipeline begins with **FastQC**, which reads FASTQ files and reports per-base quality scores, GC content distribution, adapter contamination, sequence duplication levels, and k-mer overrepresentation. For a multi-sample project, **MultiQC** aggregates individual FastQC reports into a single HTML dashboard, making cross-sample comparison effortless.

Key metrics to check:
- **Per-base quality score**: Phred Q30 means 1-in-1000 error probability; aim for >80% of bases above Q30.
- **Adapter content**: If adapters appear in reads, trimming is required.
- **Duplication rate**: >50% duplication in a whole-genome library suggests either very low input or PCR over-amplification.

Before you do anything else with your data, look at your FastQC reports. Not as a formality — as a diagnostic. Per-base quality that drops catastrophically after cycle 100 tells you to trim more aggressively. A spike in GC content at specific positions suggests adapter contamination. High k-mer overrepresentation might indicate library preparation artifacts. FastQC is a cheap, fast, and information-rich first look at whether your data is suitable for analysis, and whether any preprocessing steps are required. The one thing worse than having bad data is not knowing you have bad data.

## Stage 2: Adapter Trimming with Trim Galore

**Trim Galore** wraps Cutadapt and FastQC to automatically detect and remove Illumina adapter sequences. For paired-end data it ensures both mates are trimmed consistently and removes read pairs where one or both reads fall below a minimum length threshold (typically 20 bp). Low-quality 3' ends are trimmed using a sliding window or quality cutoff (e.g., `--quality 20`). Trimming is especially important for short-insert libraries where reads read into the adapter.

## Stage 3: Alignment

For short Illumina reads, **BWA-MEM** (Burrows-Wheeler Aligner, maximal exact matches) is the standard choice. It performs split-read alignment, handling structural variant signatures and chimeric reads better than its predecessor BWA-ALN. For variant calling, always pass the read group header (`-R`) so downstream tools can distinguish samples and library batches.

```bash
bwa mem -R "@RG\tID:sample1\tSM:sample1\tPL:ILLUMINA\tLB:lib1" \
    reference.fa sample_R1.fastq.gz sample_R2.fastq.gz \
    | samtools sort -o sample.sorted.bam
```

For RNA-seq or when reads may span splice junctions, use **STAR** (see Chapter 1.3). For long reads (PacBio CLR/HiFi, Oxford Nanopore), **minimap2** with the `-ax map-pb` or `-ax map-ont` preset handles the higher error rates and indel profiles of long-read technology.

**Bowtie2** remains an alternative for short reads when speed is prioritized over sensitivity in certain applications (e.g., ChIP-seq where reads are typically 50 bp and alignment is straightforward).

The read group header deserves special attention. When you process multiple samples in the same pipeline and eventually merge or jointly genotype them, GATK needs to know which reads came from which sample, which library preparation, and which sequencing run. The read group tags — SM (sample), LB (library), PL (platform), ID (run identifier) — encode this information in the BAM header. GATK's BQSR model is trained per read group; joint genotyping operates per sample. Getting these tags wrong can corrupt every downstream analysis, and the errors are difficult to diagnose after the fact. Set your read groups correctly during alignment.

## Stage 4: Duplicate Marking with Picard

PCR amplification during library preparation creates **PCR duplicates** — multiple reads with identical start and end positions that do not represent independent DNA fragments. **Picard MarkDuplicates** (now also available in GATK) identifies and flags these in the BAM file. For most downstream analyses (variant calling, coverage calculation), duplicates are excluded from counting.

```bash
picard MarkDuplicates \
    INPUT=sample.sorted.bam \
    OUTPUT=sample.markdup.bam \
    METRICS_FILE=sample.dup_metrics.txt
```

Optical duplicates (arising from the same cluster on a patterned flow cell) should be distinguished from PCR duplicates using the `--OPTICAL_DUPLICATE_PIXEL_DISTANCE` parameter.

## Stage 5: Base Quality Score Recalibration (BQSR)

The quality scores assigned by the sequencer are empirical and systematically biased by cycle position, preceding dinucleotide context, and machine model. **BQSR** (Base Quality Score Recalibration, part of GATK) builds a covariate model from known variant sites (dbSNP, gnomAD) and adjusts raw quality scores so that a stated Q30 base actually has a 1-in-1000 error rate. Two steps: `BaseRecalibrator` builds the model, `ApplyBQSR` applies it.

BQSR is worth thinking about carefully. The Phred quality score a sequencer assigns to a base is an estimate — based on signal strength, chemistry, and an internal model — of the probability that base is wrong. But these estimates are systematically biased. After a run of three guanines, the fourth base is more likely to be miscalled than the base quality score suggests. BQSR learns these biases from your actual data and corrects them. The result is that a Q30 quality score after BQSR actually means what Q30 is supposed to mean: a 0.1% per-base error rate. This calibration matters for the downstream probabilistic models in HaplotypeCaller that use quality scores to compute genotype likelihoods.

## Stage 6: Variant Calling

**GATK HaplotypeCaller** is the gold-standard for germline SNP and indel calling. It performs local de novo assembly in candidate regions (active regions with unusually many mismatches) using a de Bruijn graph, then evaluates haplotypes against reads. For population-scale studies, HaplotypeCaller runs in GVCF mode and genotypes are called jointly with `GenomicsDBImport` + `GenotypeGVCFs`. For somatic variant calling (tumor/normal pairs), **Mutect2** is used.

## Stage 7: Variant Annotation

Raw variant calls require **biological annotation** to be interpretable. Tools like **ANNOVAR**, **VEP** (Variant Effect Predictor), or **SnpEff** annotate each variant with:
- Gene and transcript context (missense, synonymous, frameshift, splice site)
- Population allele frequencies (gnomAD, 1000 Genomes)
- Predicted functional impact (SIFT, PolyPhen-2, CADD score)
- Known clinical significance (ClinVar)

The annotation step is where genomic coordinates become biological hypotheses. A raw VCF says: "at position chr17:43,094,833, the reference allele is G and we observed A in this sample." After annotation, VEP says: "this is the p.Arg1699Gln variant in BRCA1, which is a missense mutation in the BRCT domain, with a CADD score of 32.7, absent from gnomAD, and classified as Likely Pathogenic in ClinVar." The raw variant becomes a clinical finding. The quality of this transformation depends entirely on the quality of the variant call and the completeness of the annotation databases — which is why every step in the pipeline, from BQSR to VQSR to annotation, matters.

## Snakemake Workflow Sketch

**Snakemake** is a Python-based workflow manager that encodes the pipeline as a directed acyclic graph of rules. Each rule declares its inputs, outputs, and shell command; Snakemake resolves execution order, handles parallelism, and restarts failed jobs.

```python
rule bwa_align:
    input:
        r1="data/{sample}_R1.fastq.gz",
        r2="data/{sample}_R2.fastq.gz",
        ref="reference/hg38.fa"
    output:
        bam="results/{sample}.sorted.bam"
    shell:
        "bwa mem -R '@RG\\tID:{wildcards.sample}\\tSM:{wildcards.sample}' "
        "{input.ref} {input.r1} {input.r2} | samtools sort -o {output.bam}"

rule mark_duplicates:
    input: "results/{sample}.sorted.bam"
    output:
        bam="results/{sample}.markdup.bam",
        metrics="results/{sample}.dup_metrics.txt"
    shell:
        "picard MarkDuplicates INPUT={input} OUTPUT={output.bam} "
        "METRICS_FILE={output.metrics}"
```

Snakemake can target a cluster scheduler (SLURM, SGE) with a single `--cluster` flag, enabling thousands of samples to be processed in parallel without manual job submission.

Workflow management is not just computational convenience. It is reproducibility infrastructure. A Snakemake pipeline is a precise, executable specification of every command that was run, with every parameter, in every order. When you submit a paper, you can share the Snakemake file and your collaborators can reproduce your analysis exactly. When you discover a bug in your pipeline after analyzing 500 samples, Snakemake will identify exactly which steps need to be re-run based on which output files are now outdated. When you want to add a new sample to a completed analysis, Snakemake will run only the rules necessary to produce the new outputs without re-running anything that is already complete. These capabilities are not luxuries — they are necessities for reproducible, auditable genomics.

## Why This Matters

Mastering the end-to-end genomics pipeline is the entry point to almost every human genetics, cancer genomics, and population genomics study; errors introduced at early stages (poor trimming, skipped BQSR, misidentified duplicates) propagate silently to variant calls and can produce false discoveries that waste enormous experimental resources.

There is a broader lesson here that extends beyond the specific tools. The history of genomics is, in significant part, a history of recognizing and correcting systematic errors. Every step of this pipeline exists because someone noticed that a simpler approach was producing wrong results. PCR duplicates were inflating apparent confidence. Quality scores were systematically miscalibrated. Alignment was missing variants in repetitive regions. Joint genotyping was missing rare variants. Each fix improved the accuracy of the final result — and each fix required understanding what was going wrong and why.

When you encounter an unexpected result — too many variants, too few, the wrong Ti/Tv ratio, an unexpected distribution of VAFs — the pipeline is the first place to look. Is the quality trimming appropriate for this library type? Were duplicates marked before BQSR? Are the read groups correct? Was the VQSR threshold appropriate for this cohort size? The pipeline is not a black box that produces outputs you accept. It is a series of decisions, each with biological and statistical rationale, that you are responsible for understanding and validating.
