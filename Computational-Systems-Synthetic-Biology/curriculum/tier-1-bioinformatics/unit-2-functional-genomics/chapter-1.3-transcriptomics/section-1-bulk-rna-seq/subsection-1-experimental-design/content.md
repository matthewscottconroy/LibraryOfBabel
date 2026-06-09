# Bulk RNA-seq Experimental Design

Here is a truth that every experienced genomicist has learned the hard way: no amount of computational sophistication can save a badly designed experiment. The bioinformatics pipeline runs regardless. The alignment finishes. The count matrix is produced. DESeq2 outputs a list of "differentially expressed" genes. And only later — when the results refuse to replicate, or a collaborator points out that all the treated samples were processed on a Tuesday and all the controls on a Thursday — does it become clear that the biological question was never answered at all. The sequencer measured something, just not what you thought.

This is why experimental design is the highest-leverage knowledge in the entire RNA-seq workflow. The decisions made at the bench — before a single lane of sequencing is run — determine whether the resulting data can answer the biological question. It turns out that the most important things to get right are not computational.

## Library Preparation Strategy: Poly-A vs. Ribo-Depletion

You might think that capturing RNA is simple: extract it, sequence it. But roughly 80–90% of total cellular RNA is ribosomal RNA, a structural RNA that tells you almost nothing about which genes are transcriptionally active. If you sequenced total RNA without any enrichment, the vast majority of your reads would map to rRNA and be largely uninformative. The two dominant strategies for capturing mRNA differ in how they get rid of this problem.

**Poly-A selection** uses oligo-dT beads to capture polyadenylated transcripts. This enriches for mature, processed mRNA from eukaryotic cells, producing libraries with excellent signal-to-noise for protein-coding gene expression. The limitation is that it requires high-quality, intact RNA (RIN ≥ 7) — degraded RNA loses the poly-A tail, causing 3' bias. It is the default for standard mammalian cell and tissue experiments.

**Ribo-depletion** (e.g., Illumina Ribo-Zero) hybridizes and removes rRNA species using complementary probes, retaining all remaining RNA: mRNAs, lncRNAs, circular RNAs, primary transcripts, and nascent RNA. This is the preferred approach when: (1) RNA quality is poor (FFPE tissue, RIN < 6); (2) the organism lacks poly-A tails (bacteria, some viruses); (3) the experiment aims to capture non-polyadenylated transcripts.

The choice between these strategies is not just technical — it defines the biological universe you are sampling. Poly-A selection will miss every non-polyadenylated RNA in your sample. If you care about lncRNAs, circular RNAs, or bacterial transcriptomics, ribo-depletion is the only option.

## Stranded vs. Unstranded Libraries

This choice trips up many newcomers because it sounds like a minor technical detail. It is not. About 30% of human genes have an overlapping gene on the opposite strand. If your library does not preserve strand information, reads from these overlapping pairs become ambiguous — you cannot tell which gene produced them.

**Stranded libraries** preserve the information about which DNA strand was transcribed. The dUTP method (most common for Illumina) incorporates dUTP during second-strand synthesis and then degrades that strand, leaving only reads from the original strand. Stranded data allows: correct read assignment for overlapping genes on opposite strands, accurate quantification of antisense transcripts, and correct orientation for novel transcript assembly.

**Unstranded libraries** are cheaper and simpler but lose strand information. For most standard differential expression (DE) studies with well-annotated genomes, unstranded data is acceptable, but stranded is strongly recommended as the default for any new experiment. The modest cost difference is almost always worth the analytical flexibility you gain.

## Sequencing Depth

How deeply do you need to sequence? The answer depends entirely on what you are trying to measure. It turns out that depth requirements scale dramatically with the biological question:

- **Basic differential expression of abundant genes**: 20–30 million paired-end reads per sample
- **Comprehensive DE including lowly expressed genes**: 40–60 million reads
- **Alternative splicing analysis**: 100–200 million reads (requires coverage of individual exon-exon junctions)
- **lncRNA discovery**: ≥100 million reads (lncRNAs are often lowly expressed)

Under-sequencing a sample causes low counts for most genes, inflating variance and reducing power to detect DE. The intuition here is straightforward: if you only get 5 counts for a gene in control and 3 in treatment, is that a real 40% decrease, or just noise from sampling? With 500 and 300 counts, you can answer that question. Over-sequencing, however, provides diminishing returns once all expressed genes are sufficiently covered — you are mostly re-sequencing molecules you have already seen.

## Biological vs. Technical Replicates

This is the most consequential design decision you will make for any RNA-seq experiment. Get it wrong and nothing downstream can fix it.

**Biological replicates** are independent biological samples (different animals, patients, or independently grown cell cultures) subjected to the same condition. **Technical replicates** are measurements of the same sample run multiple times (e.g., two library preps from one RNA extraction).

**The minimum requirement is n ≥ 3 biological replicates per condition.** With n = 2, it is mathematically impossible to estimate within-group variance reliably, rendering statistical DE analysis invalid. For regulatory submissions and publication, n ≥ 3 is the expectation; n ≥ 4–6 is preferred when variability is expected (in vivo experiments, patient samples).

Technical replicates add little value in modern RNA-seq — sequencing is highly reproducible, and technical variance is dwarfed by biological variance. The same sequencing budget is better spent on additional biological replicates. You might expect that running the same sample twice would tell you something useful about measurement error. It turns out the measurement error is not the problem; the biological variability between individuals is.

Power calculation for DE studies uses the formula relating effect size (fold change), desired FDR, power (1 - β), and estimated within-group coefficient of variation (CV). Tools like **RNASeqPower** (R package) or **Scotty** can estimate the number of replicates and depth required to detect a given fold change at a target power of 0.8.

## Paired-End vs. Single-End

**Paired-end** (PE) sequencing reads both ends of each DNA fragment (e.g., 2 × 150 bp). Benefits: better alignment rates, improved duplicate detection, required for splicing analysis, and more accurate quantification at gene boundaries. **Single-end** (SE) is cheaper and sufficient for simple abundance quantification in well-annotated genomes. For new experiments, PE is the standard — the informational and alignment benefits generally outweigh the modest cost increase.

## Sequencing Batch Effects and Study Design

A **batch effect** is systematic, non-biological variation introduced by processing samples at different times, on different sequencers, or by different technicians. Batch effects are one of the most common causes of irreproducible results in genomics studies. They are treacherous precisely because they are often invisible until you look for them.

Consider what happens in practice: you receive 24 samples for a treatment vs. control experiment. The core facility processes 12 samples one week and 12 the next. If all 12 control samples were processed in week 1 and all 12 treated samples in week 2, the experiment is ruined before it starts. Any batch-to-batch technical variation will be inseparable from the biological signal. No computational tool can unconfound a perfectly confounded design.

Design principles to minimize batch effects:

1. **Randomize samples across batches**: Never process all control samples in one batch and all treatment samples in another.
2. **Balance covariates**: Within each batch, include equal numbers of each condition.
3. **Record all potential batch variables**: Date of RNA extraction, library prep kit lot number, sequencer run ID.
4. **Include the batch as a covariate** in the DE model (not correct it away before DE analysis — see Section 4).

If batches are confounded with the biological variable of interest (e.g., all treated samples in batch 1, all controls in batch 2), the experiment is unrescuable computationally.

## Why This Matters

Every gene list, every pathway enrichment result, every biological conclusion you draw from RNA-seq data traces back to these foundational choices. Poor experimental design is discovered only after sequencing — when it is too late and too expensive to fix. The two hours you spend thinking carefully about replication strategy, stranding, depth, and batch randomization before any samples are processed will return more scientific value than any downstream analysis tool you ever learn. This section is not just technical orientation — it is the intellectual foundation of reliable transcriptomics.
