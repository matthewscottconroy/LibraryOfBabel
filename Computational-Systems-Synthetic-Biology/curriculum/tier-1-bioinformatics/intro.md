# Tier 1: Bioinformatics

The sequencing of the human genome was announced in June 2000. The press releases called it the "Book of Life." Reporters wrote about reading the genetic code of humanity. It sounded like the hard part was over.

The hard part had barely started.

The genome is a string of 3.2 billion nucleotides. Protein-coding genes account for less than 2% of it. The rest — regulatory elements, non-coding RNAs, repetitive sequences, conserved regions of unknown function — was largely a mystery in 2000 and remains incompletely understood today. Having the sequence is not the same as understanding it. The raw output of a sequencing machine is an enormous string of letters that, by itself, tells you almost nothing about biology. To extract biological meaning from that string, you need algorithms for comparison, statistical models for inference, computational methods for interpretation, and the biological knowledge to know what questions to ask.

That translation — from raw genomic data to biological understanding — is what this tier is about.

## Bioinformatics as Translation

Bioinformatics is the discipline that transforms molecular data into biological knowledge. It is the reason a researcher can sequence a new pathogen's genome in 48 hours and know within days which genes encode surface proteins, which are related to known virulence factors, and how the pathogen relates to other organisms. It is the reason a cancer biologist can compare the transcriptomes of ten thousand individual tumor cells and identify the rare subpopulation driving drug resistance. It is the reason the protein structure problem — which kept structural biologists occupied for fifty years — was largely solved by a neural network trained on sequences.

These achievements are not accidents of hardware or data volume. They reflect the development, over decades, of algorithms that are both mathematically elegant and biologically grounded. Dynamic programming for sequence alignment, probabilistic hidden Markov models for protein family detection, de Bruijn graphs for genome assembly, maximum likelihood methods for phylogenetic inference, energy-based models for structure prediction — each of these represents a fundamental insight about how biological information is organized and how it can be analyzed.

This tier covers those foundational methods. The goal is not to teach you to run bioinformatics software. It is to teach you why the software works, what assumptions it makes, and how to use it critically.

## The Six Chapters of This Tier

The tier is organized around six chapters, each corresponding to a major domain of bioinformatics.

**Chapter 1.1: Sequence Analysis** establishes the vocabulary and grammar of the entire field. Sequence alignment — the problem of finding the best correspondence between two or more sequences — is the central operation of bioinformatics, the one that everything else depends on. You will learn the Needleman-Wunsch and Smith-Waterman dynamic programming algorithms that solve it exactly, the BLAST heuristic that solves it approximately but orders of magnitude faster, and the profile Hidden Markov Models that extend it to entire protein families. You will also learn the file formats — FASTA, FASTQ, SAM/BAM, VCF, BED/GFF3/GTF — that constitute the data infrastructure of the field.

**Chapter 1.2: Genomics** scales from individual sequences to whole genomes. Assembling a genome from millions of short reads requires graph-theoretic algorithms. Annotating a genome — identifying genes, regulatory elements, and functional features — requires integrating sequence statistics, comparative evidence, and experimental data. And comparing genomes across organisms reveals the macroscopic features of evolution: synteny, rearrangements, gene family expansions and contractions.

**Chapter 1.3: Transcriptomics** addresses the question of which genes are active and when. RNA-seq and single-cell sequencing have made it possible to measure the entire transcriptome in hours. The computational challenge — correcting for technical variation, identifying differentially expressed genes, characterizing cell types by expression profiles — is as demanding as the biological one, and solving it requires statistical methods specifically designed for count data with overdispersion and batch effects.

**Chapter 1.4: Proteomics and Metabolomics** covers the downstream layers of molecular biology. Mass spectrometry-based proteomics can identify and quantify thousands of proteins simultaneously; the computational challenge is peptide identification, protein inference, and quantification across complex samples. Metabolomics characterizes the small-molecule chemistry of the cell, requiring specialized methods for peak detection, metabolite identification, and pathway analysis.

**Chapter 1.5: Structural Bioinformatics** connects sequence to three-dimensional structure. Structure determines function in a way that sequence alone cannot reveal. This chapter covers protein structure prediction (culminating in AlphaFold2), structure comparison, docking and binding site analysis, and the molecular dynamics simulations that capture protein dynamics. The dramatic improvement in structure prediction quality since 2020 has transformed what is computationally possible in structural biology.

**Chapter 1.6: Phylogenetics** addresses evolutionary inference from molecular sequences. How are organisms related? When did lineages diverge? What selective pressures have shaped particular proteins? Answering these questions requires probabilistic models of sequence evolution, tree-building algorithms, and statistical tests for selection. Phylogenetics is at once one of the oldest disciplines in bioinformatics and one of the most actively developing.

## The Unifying Theme: Algorithms Encode Biological Assumptions

A theme runs through all six chapters of this tier: the algorithms encode biological assumptions. Sequence alignment assumes that evolution works by substitution, insertion, and deletion. Differential expression analysis assumes that read counts follow a negative binomial distribution. Phylogenetic reconstruction assumes that sequences evolve according to specific substitution models. Structure prediction assumes that the energy landscape of protein folding has particular properties accessible to deep learning.

Understanding these assumptions is not just intellectual housekeeping. It is what separates a sophisticated user of bioinformatics tools from someone who runs the software and accepts whatever output appears. When assumptions hold, the results are trustworthy. When assumptions are violated — when sequences are too divergent for pairwise alignment to be reliable, when RNA-seq data has severe batch effects, when a protein has no homologs with known structure — you need to know that the standard method will fail, and why.

The goal of this tier is to make you a sophisticated user: someone who understands the tools well enough to know when to trust them, when to be skeptical, and when to reach for a different approach.
