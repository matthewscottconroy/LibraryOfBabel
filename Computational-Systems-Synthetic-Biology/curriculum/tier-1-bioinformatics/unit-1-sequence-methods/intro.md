# Unit 1: Sequence Methods

The genome is the cell's complete instruction manual — four billion years of evolutionary editing, compressed into a molecule that fits inside a nucleus a few micrometers across. Every protein the cell makes, every regulatory decision it executes, every developmental program it follows traces back to a sequence of nucleotides in that molecule. To understand how cells work, you need to read the genome. To read the genome, you need the tools covered in this unit.

Start with a specific sequence: `ATGAAAGATCTGCGTTTTACGCTTGTCATTGTTGTCGGCATTGTTTTAG`. Is it a gene? What organism does it come from? What does the protein it encodes do? How does it differ from homologs in related species? These questions — deceptively simple to state, computationally demanding to answer — are where bioinformatics began, and they remain the foundation on which the rest of the discipline rests.

## Why Sequence Is Where We Start

Before you can ask what a gene does, you must identify it. Before you can identify it, you must have sequenced the genome that contains it. Before you can compare it to known genes from other organisms, you must align the sequences. Before you can call variants in a patient's genome, you must have mapped sequencing reads to a reference. Before you can measure gene expression, you must have a genome annotation that defines what a gene is.

Every one of these steps is a sequence analysis problem. Sequence is not just one approach among many in bioinformatics; it is the foundation layer on which everything else is built. The genome provides the parts list. Sequence analysis provides the tools for reading that parts list, comparing it across organisms, identifying what is conserved versus what varies, and connecting the raw sequence to biological function. Without these foundations, the more sophisticated analyses of gene regulation, protein function, and systems behavior have no reliable substrate to stand on.

The computational methods developed to work with biological sequences represent some of the most elegant applications of dynamic programming, probabilistic modeling, and graph theory you will encounter anywhere. More importantly, they work: sequence alignment and database search, genome assembly and gene annotation, protein family classification — these are not research-grade curiosities but production tools used to process thousands of genomes every day. Every algorithm here evolved under selective pressure from biological necessity: BLAST was invented because Needleman-Wunsch was too slow for growing sequence databases. De Bruijn graph assembly was adopted because earlier methods broke down at the read counts that sequencing machines were producing. Understanding this history matters — it tells you what each method is optimized for and where its failure modes lie.

## Chapter 1.1: Sequence Analysis

The first chapter covers the core methods for comparing, aligning, and analyzing biological sequences. It builds from the mathematical foundations — what alignment means, how it is scored, why dynamic programming solves it efficiently — through the practical tools that process real sequencing data at scale.

The Needleman-Wunsch and Smith-Waterman algorithms are beautiful examples of dynamic programming applied to biology. BLAST and HMMER, the database search tools used in essentially every bioinformatics workflow, trade optimality for speed in ways that are worth understanding carefully. Multiple sequence alignment extends pairwise comparison to entire protein families, revealing conservation patterns invisible in any pairwise comparison. And the sequence file formats — FASTA, FASTQ, SAM/BAM, VCF, BED/GFF3/GTF — are the infrastructure through which all of this data flows between tools.

The unifying thread is evolutionary inference. Sequence analysis algorithms work because sequences carry the record of their evolutionary history: conserved positions reveal function under selection, variant positions reflect tolerated change, and statistical models of substitution let us quantify similarity in terms of common ancestry.

## Chapter 1.2: Genomics

The second chapter scales these methods from individual sequences to whole genomes. Assembling a genome from millions of short reads is fundamentally a graph problem — the de Bruijn graph formulation turns what looks like an impossible puzzle into a tractable path-finding problem. Annotating a genome (finding genes, regulatory elements, and other functional features) requires integrating sequence statistics, comparative genomics, and experimental data. Comparing assembled genomes requires dealing with rearrangements, inversions, and duplications that go far beyond simple sequence alignment.

Genomics is not just a bigger version of sequence analysis. It requires new conceptual tools: models of genome organization, population genetics statistics, regulatory element definitions, and the statistical frameworks for connecting genomic variation to phenotypic outcomes.

## The Foundation You Are Building

The sequence methods in this unit are the analytical foundation for every subsequent unit in this course. The systems biology chapters depend on the genome annotations, expression measurements, and variant interpretations built on these sequence methods. The synthetic biology chapters depend on sequence databases, alignment tools, and genome analysis methods developed here. Phylogenetics, structural prediction, and evolutionary analysis all begin with multiple sequence alignments and the databases they populate.

Starting with sequence methods is not just pedagogically sensible. It is conceptually honest: this is where the data comes from, where the first biological interpretations are made, and where errors introduced early propagate through everything downstream. Getting sequence analysis right — understanding the algorithms, their assumptions, their limitations, and their failure modes — is the first and most important investment you can make in becoming a capable computational biologist.
