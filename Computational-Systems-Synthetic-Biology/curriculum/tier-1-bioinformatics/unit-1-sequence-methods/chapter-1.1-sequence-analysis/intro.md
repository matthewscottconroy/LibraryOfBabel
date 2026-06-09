# Chapter 1.1: Sequence Analysis

Imagine you are at a bench somewhere — it does not matter where, the story is universal. You have been working for months on a protein from a pathogenic bacterium that disrupts epithelial cell junctions. You run a BLAST search on the protein sequence, mostly out of habit, expecting to see familiar relatives. Instead, you get a hit you did not expect: 92% identity over 340 amino acids to a known toxin from a completely different organism, with an E-value of $10^{-187}$. That is not a marginal hit. That is a near-certain identification. Your protein is not merely related to a toxin — it functionally *is* one, differing by fewer than 30 residues.

In that moment, your experimental direction changes entirely. The biochemical assays you planned, the overexpression constructs you were designing — all of it reconfigures around the new understanding that the sequence alignment just delivered. You now know which residues are essential (the ones conserved across both organisms), what structural scaffold likely underlies the protein (the toxin's fold has been solved), and which organisms produce related proteins (the ones that appear in the BLAST output). None of this required a single additional experiment. It required a database search that took forty-five seconds.

This chapter is about how that forty-five seconds works — and why it can be trusted.

## What This Chapter Covers

Sequence analysis is the oldest and most foundational discipline in bioinformatics. It encompasses the algorithms, data structures, and file formats that enable the comparison, organization, and interpretation of biological sequences. The chapter is organized in five sections.

**Pairwise sequence alignment** (Section 1) develops the mathematical foundations: what it means to align two sequences, how alignment scores are computed, the dynamic programming algorithms that find optimal alignments exactly (Needleman-Wunsch for global, Smith-Waterman for local), substitution matrices that give alignment scores biological meaning, and BLAST as the practical implementation that makes database-scale search tractable.

**Multiple sequence alignment** (Section 2) extends pairwise alignment to many sequences simultaneously — revealing conservation patterns, providing input for phylogenetic inference, and enabling the construction of profile HMMs that represent entire protein families. The computational challenge is NP-hard; this section covers the progressive and iterative heuristics that make it tractable, quality assessment, and the profile Hidden Markov Models that represent the state of the art in remote homology detection.

**File formats** (Section 3) covers the five formats you will encounter in almost every sequencing analysis: FASTA for sequences, FASTQ for raw reads, SAM/BAM for alignments, VCF for variants, and BED/GFF3/GTF for genomic feature annotations. Each format encodes specific biological information, has its own coordinate conventions, and harbors specific pitfalls for the unwary.

**Short read mapping and assembly** (Section 4) bridges raw sequencing reads and biological interpretation. Read mapping places each read in a reference genome using the Burrows-Wheeler Transform; genome assembly reconstructs sequences de novo using de Bruijn graphs. Both are mathematically elegant solutions to problems that appeared intractable until the right representation was found.

**Practical tools reference** (Section 5) consolidates the best-in-class tools for each task — with specific commands, parameter guidance, and the reasoning needed to select the right tool for a given analysis.

## The Logic of Sequence Analysis

Every topic in this chapter connects back to a simple biological fact: evolution acts on sequences. Sequences that share a common ancestor carry the record of their shared history as a pattern of similarity — conserved where function required conservation, diverged where change was tolerated. The algorithms of sequence analysis are methods for reading that record.

The scoring functions in alignment algorithms encode evolutionary models. The substitution matrices encode the relative rates of amino acid replacements. The E-value in BLAST quantifies whether a similarity is too large to be explained by chance. Profile HMMs model the position-specific variation pattern of an entire protein family. At every level, sequence analysis is statistical inference about evolutionary relationships.

Understanding this underlying logic — not just the commands to run tools, but the reasoning behind their design — is what enables you to interpret results critically, recognize when an analysis might be misleading, and make principled choices when standard protocols do not quite fit your problem. The BLAST hit that started this chapter is trustworthy precisely because we understand the statistics behind its E-value. If you had not known what an E-value means and how it scales with database size, you might have dismissed a genuine homolog or trusted a spurious one. Sequence analysis done correctly is powerful. Done without understanding, it is a source of confident errors.
