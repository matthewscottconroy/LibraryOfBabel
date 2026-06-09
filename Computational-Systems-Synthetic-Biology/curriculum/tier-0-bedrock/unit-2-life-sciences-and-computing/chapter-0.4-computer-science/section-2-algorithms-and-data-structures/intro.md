# Section 2: Algorithms and Data Structures

In 1990, the human genome project was just beginning. The genome is three billion base pairs long. Aligning a short sequencing read to the full genome by naive string matching — trying every possible position — would require roughly $10^{11}$ operations per read. At a billion operations per second, one read would take 100 seconds. At 30× sequencing depth, with 600 million reads, the entire alignment step would take roughly 2,000 years.

This is not a theoretical concern — it is the central engineering problem of the genomics revolution. The solution was not faster hardware. It was better algorithms and data structures. The Burrows-Wheeler transform, the FM-index, the suffix array: these data structures reduced the alignment problem from $O(nm)$ to $O(m)$ per read, where $m$ is the read length and $n$ is the genome size. That reduction — from quadratic to linear in the genome — is the reason next-generation sequencing became practical. The algorithms are the instrument.

This section covers the algorithmic foundations of computational biology, from the basic containers that organize data in memory to the specialized structures that make genome-scale computation possible.

**Fundamental data structures** — arrays, hash tables, trees, graphs, stacks, and queues — are the vocabulary. Every more specialized algorithm is built from these building blocks. Understanding their time and space complexity, and knowing when to use which, is prerequisite knowledge for reading methods sections, evaluating tool performance, and writing efficient analysis code.

**Algorithm analysis** with Big O notation gives you the ability to predict, before running anything, whether an approach is feasible. For $n = 10^9$ (the human genome), $O(n^2)$ algorithms are not slow — they are impossible. Every clever algorithm in bioinformatics exists because someone recognized an $O(n^2)$ problem and found an $O(n \log n)$ or $O(n)$ solution.

**Sorting and searching** underlie the indexed file formats of genomics. BAM files, VCF files, FASTA files — they are sorted, and their indexes are built on binary search. Understanding merge sort, radix sort, and the $O(n \log n)$ lower bound for comparison-based sorting explains both why SAMtools sort works the way it does and why sorted data enables millisecond random-access queries.

**Dynamic programming** is the technique behind sequence alignment. Needleman-Wunsch, Smith-Waterman, the Viterbi algorithm for HMMs — all are DP algorithms. Understanding the core idea (optimal substructure plus overlapping subproblems) means understanding the mathematical basis of BLAST, read alignment, and gene prediction.

**String algorithms** connect computer science text processing directly to molecular biology. The BWT, FM-index, k-mer methods, and de Bruijn graphs are the algorithmic foundations of short-read alignment and genome assembly. Understanding them explains why BWA-MEM aligns a million reads per minute, why different k-mer sizes produce different assemblies, and why long reads solve problems that short reads fundamentally cannot.

**Probabilistic data structures** — Bloom filters, MinHash sketches, Count-Min sketches — extend what is computationally tractable by accepting small, bounded errors in exchange for dramatic space savings. These are not academic curiosities; they are deployed in jellyfish, Mash, Kraken, and sourmash.

**Graph algorithms** are the language of biological networks. BFS, DFS, Dijkstra, topological sort, minimum spanning trees, and motif detection apply directly to metabolic pathways, protein interaction networks, gene regulatory networks, and genome assembly.
