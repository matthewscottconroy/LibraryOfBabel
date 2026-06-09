# Ab Initio Gene Prediction

Imagine you have just assembled the genome of a newly discovered organism — a soil nematode, a deep-sea sponge, an Arctic lichen. You have 200 million base pairs of sequence, laid out in four chromosomes, and you know essentially nothing about what genes it contains. You have no RNA-seq data from this organism, no protein databases for its clade, no graduate students who have spent years studying its biology. You have only the sequence.

Can you find the genes?

It turns out that you can — imperfectly, but usefully — by exploiting statistical regularities in how genes are structured. Protein-coding genes have recognizable signatures: they have characteristic start codons and splice sites, their codons are used in species-specific ratios, their exons have different hexamer frequencies from intergenic sequence. An ab initio gene predictor is a statistical model that has learned these regularities from known genes and can apply them to novel sequence to identify candidate gene structures from first principles.

**Ab initio gene prediction** identifies protein-coding genes directly from genomic sequence using statistical models of gene structure — without requiring homology to known genes or RNA-seq evidence. The word "ab initio" (Latin: from the beginning) reflects that predictions are made from first principles of gene structure.

## What the Model Must Capture

A eukaryotic protein-coding gene has a characteristic structure: 5' UTR, start codon, alternating exons and introns, stop codon, and 3' UTR. Each element has statistical signals that distinguish it from non-genic sequence:

- **Start codon context (Kozak sequence)**: GCCRCCAUGG (with consensus C at -3 and G at +4)
- **Splice donor** (5' splice site): GT after the exon (GU in RNA)
- **Splice acceptor** (3' splice site): AG before the next exon; polypyrimidine tract upstream
- **Branch point**: ~20–50 nt upstream of acceptor; A_YYYYYY
- **Stop codons**: TAA, TAG, TGA — must maintain reading frame
- **Codon usage**: coding sequences show species-specific codon usage bias
- **Hexamer frequencies**: coding regions have different hexamer (6-mer) frequencies than non-coding

Ab initio tools model these signals using Hidden Markov Models or generalized HMMs.

The challenge is that these signals are individually weak. The canonical GT at a splice donor is present at over 99% of splice sites, but GT dinucleotides also occur randomly throughout the genome at high frequency. The strength of ab initio prediction comes from combining many weak signals into a coherent model of entire gene structures. A GT alone proves nothing; a GT in a specific sequence context, at the end of a region with coding hexamer statistics, preceded by exon-like sequence, is much more convincing.

## Generalized Hidden Markov Models (GHMMs)

A **generalized HMM (GHMM)** extends the standard HMM by allowing states to emit variable-length segments of sequence (called a "duration" or "geometric" emission). This naturally models exons (variable length, protein-coding statistics) and introns (variable length, different statistics):

States in a typical GHMM gene model:
- Intergenic (background)
- 5' UTR
- Start codon
- Exon (initial, internal, terminal)
- Intron (with splice site models at boundaries)
- Stop codon
- 3' UTR

Transition probabilities encode the grammar of gene structure: an initial exon can be followed by an intron (not another initial exon). Emission probabilities encode codon usage and signal motif preferences.

The Viterbi algorithm finds the most probable parse of the genomic sequence through the GHMM — the sequence of states that best explains the observed DNA.

The GHMM framework is elegant precisely because it encodes biological knowledge as probabilistic constraints. The gene structure grammar — exon must be followed by intron, final exon must contain stop codon, reading frame must be maintained — is not programmed as hard rules but as transition probabilities that make implausible state sequences very unlikely. The Viterbi algorithm then searches this space efficiently, returning the globally most probable annotation for the entire genomic sequence at once.

## AUGUSTUS

**AUGUSTUS** is the most widely used ab initio gene predictor for eukaryotes:

```bash
# Predict genes with a pre-trained species model
augustus --species=human genome.fa --outfile=predictions.gff3

# Available species include: human, mouse, arabidopsis, drosophila,
# zebrafish, c_elegans, yeast, tomato, rice, and many more

# Train AUGUSTUS on new organism (requires curated gene set)
autoAug.pl --genome=genome.fa --trainingset=training_genes.gb \
           --species=myspecies --workingdir=autoAug_output/
```

AUGUSTUS uses a GHMM with models for:
- Splice site consensus and position-weight matrices
- Branch point signals
- Length distributions of exons and introns
- Codon usage tables
- CpG island models (in vertebrates)

**Training**: species-specific parameters must be estimated from a curated set of genes with experimentally verified structures. Performance degrades substantially when using models from phylogenetically distant species.

This species-specificity is not a limitation of AUGUSTUS specifically — it reflects a genuine biological reality. Codon usage varies enormously across the tree of life; intron length distributions differ by orders of magnitude between yeast and mammals; splice site consensuses are well-conserved but not identical across kingdoms. A model trained on human genes will fail on a genome with unusual intron sizes or non-canonical splice signals. For a newly sequenced organism, generating even a small training set of high-confidence gene models — by sequencing a few transcripts or using conserved single-copy orthologs — dramatically improves prediction accuracy.

## GeneMark

**GeneMark-EP+** and related tools take a different approach:
- For prokaryotes: Markov chain models of codon usage (no introns)
- For eukaryotes: self-training from the genome itself (identifies coding vs. non-coding regions)

```bash
# GeneMark-ES: self-training eukaryotic prediction
perl gmes_petap.pl --ES --sequence genome.fa --cores 8 --soft_mask

# GeneMark-EP+: with protein homology hints
perl gmes_petap.pl --EP+ --sequence genome.fa \
    --EP protein_evidence.faa --cores 8
```

## Accuracy and Limitations

On human genome benchmarks, AUGUSTUS achieves:
- ~80% sensitivity at the nucleotide level
- ~50–60% sensitivity at the gene level (exactly correct exon boundaries)
- ~70–80% specificity at the nucleotide level

These numbers decrease substantially for:
- **Non-model organisms**: no trained species model available
- **Rapidly evolving genes**: unusual codon usage, divergent splice signals
- **Polycistronic loci**: overlapping genes in alternative reading frames
- **Small exons** (< 30 nt): difficult to model statistically
- **Alternative isoforms**: ab initio tools typically predict one isoform per locus

The 50–60% gene-level sensitivity deserves honest attention. It means that roughly half the genes in a human genome benchmark are either missed entirely or have incorrect exon boundaries when predicted ab initio. This is not a catastrophic failure — the predicted genes are substantially correct at the nucleotide level — but it means that ab initio predictions alone are not sufficient for a production-quality genome annotation. They are a starting point, not a finished product.

## Practical Usage in Annotation Pipelines

Ab initio prediction is rarely used alone. It serves as:
1. **Initial evidence layer** in multi-evidence annotation pipelines (MAKER, BRAKER)
2. **Prediction in regions with no RNA-seq coverage** (newly expressed genes, developmental time points not sampled)
3. **Annotation of non-model organisms** where no RNA-seq may be available

The **BRAKER2** pipeline automates the integration of ab initio prediction with RNA-seq and protein evidence:

```bash
braker.pl --species=myorganism \
          --genome=genome.fa \
          --bam=rnaseq_aligned.bam \
          --prot_seq=proteins.faa \
          --softmasking --cores 16
```

## Why This Matters

Ab initio gene prediction is essential for annotating newly sequenced genomes where no experimental data is available. Every newly sequenced genome in NCBI undergoes automated annotation (the NCBI Eukaryotic Genome Annotation Pipeline uses GNOMON, a GHMM-based predictor). For non-model organisms — the vast majority of the ~10 million eukaryotic species — ab initio tools combined with limited RNA-seq provide the primary gene models. Understanding the limitations of ab initio prediction (exon boundary errors, missed small exons, single isoform predictions) is essential for appropriately using such annotations in functional analysis.

The deeper point is that ab initio prediction represents our attempt to distill biological knowledge into a computational prior. The GHMM is, in effect, encoding what we know about gene structure into a probabilistic grammar. Where that knowledge is good — conserved eukaryotic splice signals, start codon context, stop codon positions — the predictor is confident and accurate. Where that knowledge is thin or organism-specific — alternative splicing patterns, non-canonical splice sites, tiny exons — it fails. Ab initio prediction is, in the best sense, a map of what we know about gene structure and where that knowledge runs out.
