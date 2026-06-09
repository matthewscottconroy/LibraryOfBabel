# CRISPRi/a Genome-Wide Screens

Here is a question that would have taken a decade to answer twenty years ago: which genes, when suppressed, make cancer cells more sensitive to a particular drug? You could have answered it one gene at a time with individual knockdowns, or imprecisely with RNAi screens that suffered from off-target effects and incomplete knockdown. Today you can answer it in a single pooled experiment, in eight weeks, with a resolution and specificity that RNAi could never provide. The tool is a genome-wide CRISPRi screen — 200,000 guide RNAs, each repressing a single gene in a different cell, all competing in a single flask under your experimental condition. The cells that survive tell you which gene repressions confer advantage. The cells that disappear tell you which repressions are lethal. It is functional genomics at a scale and precision that changes what questions you can even think to ask.

CRISPR interference (CRISPRi) and CRISPR activation (CRISPRa) use catalytically dead Cas9 (dCas9) fused to transcriptional effectors to repress or activate genes without altering the underlying DNA sequence. When paired with genome-scale sgRNA libraries targeting every gene in the genome, CRISPRi and CRISPRa become powerful tools for systematic functional genomics — identifying which genes are required for a phenotype of interest.

## dCas9 Effector Fusions

**dCas9 (catalytically dead Cas9)**: both HNH and RuvC domains inactivated (D10A + H840A). dCas9 retains guide RNA-dependent DNA binding but cannot cleave. Targeted to a promoter, dCas9 physically blocks RNA polymerase access — causing modest repression (~2–5-fold) through steric occlusion alone.

**dCas9-KRAB (CRISPRi)**: KRAB (Krüppel-associated box) is a potent transcriptional repression domain that recruits KAP1 and SETDB1, installing H3K9me3 histone marks and chromatin compaction. dCas9-KRAB achieves **20–1000-fold repression** when targeted to within 200 bp of the transcription start site (TSS). This is the standard CRISPRi effector for mammalian cells.

**dCas9-VP64/VPR (CRISPRa)**: VP64 is a tetrameric fusion of the VP16 transactivation domain. VPR (VP64-p65-Rta) recruits additional coactivators and achieves 100–1000-fold activation of endogenous genes — sufficient to drive expression from silenced or low-expression genes to high levels. SAM (synergistic activation mediator) and SunTag-based CRISPRa systems provide alternative architectures with similar or greater activation.

## Designing Genome-Wide sgRNA Libraries

A genome-wide screen requires a **pooled sgRNA library** — typically 4–10 guides per gene × ~20,000 human genes = 80,000–200,000 unique sgRNA sequences in one library.

**Guide design for CRISPRi**: guides must target within 200 bp upstream of the TSS or the first 200 bp of the coding sequence. Guides targeting downstream positions have reduced repression. This TSS constraint means guides cannot simply be distributed uniformly along the gene; TSS positions must be annotated from ENCODE or RefSeq data.

**Guide design for CRISPRa**: similar TSS proximity requirement, but optimal position is typically 50–400 bp upstream of the TSS. TSS annotation is critical.

**Library synthesis**: oligonucleotide synthesis arrays produce all guides in a single synthesis reaction. Libraries are designed computationally, ordered from synthesis companies (Twist Bioscience, Agilent), and cloned in bulk into sgRNA expression vectors.

## Pooled Screen Protocol

```
1. Lentiviral packaging: library plasmid pool → lentiviral particles
   (each particle carries one sgRNA sequence)

2. Transduction: infect target cell line at MOI ~0.3 (ensures one integration per cell)
   → Each cell expresses one sgRNA from random integration

3. Selection (if needed): select for stable integration with antibiotic

4. Apply experimental condition:
   - Growth selection: grow for 2–3 weeks; essential genes deplete from the pool
   - Drug treatment: cells with resistance-conferring knockdowns survive
   - FACS sort: sort cells by fluorescence linked to phenotype

5. DNA extraction from initial timepoint and final timepoint

6. PCR amplification of sgRNA sequences (Illumina amplicon protocol)

7. NGS sequencing: count reads for each sgRNA in both conditions

8. Analyze: compute log2 fold change for each sgRNA; use MAGeCK or BAGEL
```

## Analysis: From Read Counts to Gene Rankings

### MAGeCK (Model-based Analysis of Genome-wide CRISPR-Knockout)

MAGeCK is the standard analysis pipeline for pooled CRISPR screens:

1. Align sequencing reads to library sgRNA sequences
2. Count reads per sgRNA per sample
3. Normalize counts between samples (total read depth normalization)
4. Compute log2 fold change (LFC) for each sgRNA between conditions
5. For each gene, aggregate LFCs across its multiple sgRNAs using a robust statistical model
6. Output: ranked gene list with p-value and FDR for enrichment or depletion

**Positive vs. negative selection screens**:
- **Negative selection (dropout screen)**: essential genes have sgRNAs depleted over time because cells lacking these genes die. Used to find essential genes, synthetic lethal interactions.
- **Positive selection screen**: sgRNAs enriched when their gene knockdown confers a survival advantage (e.g., drug resistance). Used to find resistance mechanisms, tumor suppressor genes.

### BAGEL: Bayesian Gene Essentiality

BAGEL uses a Bayesian framework to classify genes as essential or non-essential by comparing their sgRNA depletion to a training set of known essential and non-essential genes. More sensitive than MAGeCK for essential gene identification.

## Published Genome-Wide Screen Examples

**Essential gene screens**: Systematic CRISPRi screens in human cells identified ~2,000 genes essential for growth — consistent with prior RNAi screens but with higher resolution. Comparison across cell types reveals context-specific essentiality.

**Synthetic lethality screens**: Combined CRISPRi knockdown of PARP1 (drug target) with sgRNA library screen identifies genes that become essential when PARP1 is suppressed — revealing synthetic lethal partners relevant to cancer therapy.

**CRISPRa gain-of-function screens**: Gilbert et al. (2014) used genome-wide CRISPRa to identify genes whose overexpression confers resistance to the chemotherapy vemurafenib. Identified MEK and CDK8 as resistance genes, consistent with subsequent clinical observations.

**Functional characterization of non-coding genome**: CRISPRi can target non-coding regions (enhancers, lncRNA promoters) — allowing genome-wide screens of regulatory elements, not just genes.

## Challenges and Controls

**Library representation**: every sgRNA must be represented by sufficient reads (>300 reads/sgRNA at initial timepoint) for statistical power. This requires maintaining cells at >500× library coverage throughout the screen.

**Off-target effects**: some sgRNAs may have off-target repression effects that confound phenotype interpretation. Using multiple sgRNAs per gene and requiring concordance across them mitigates this.

**Transcriptional noise**: CRISPRi creates partial rather than complete knockdown (typically 80–95% repression). Genes where partial knockdown is insufficient for a phenotype may appear as false negatives.

**Hit validation**: top screen hits must be individually validated with independent sgRNAs and by rescue experiments (re-expressing the target gene in knockdown cells to confirm phenotype reversal).

## Why This Matters

CRISPRi/a genome-wide screens have transformed functional genomics in the same way that RNAi screens did in the 2000s — but with better specificity, higher dynamic range, and the ability to activate as well as inhibit genes. For synthetic biology, these screens identify genetic bottlenecks in engineered pathways, discover cellular responses to heterologous gene expression, and map the regulatory networks that must be rewired for desired phenotypes. For medicine, CRISPRa screens identify potential therapeutic targets — genes whose activation (rather than inhibition) might treat disease. The capacity to systematically query the function of every gene in the genome, using the same molecular machinery used for precision editing, represents one of the most powerful convergences in modern biology.
