# String Algorithms

There is a beautiful accident of history at the heart of computational biology: the molecule of life is a string. DNA is a sequence of nucleotides drawn from a four-letter alphabet. Proteins are sequences of amino acids from a twenty-letter alphabet. This means that the entire machinery of computer science string algorithms — developed for text search, data compression, and pattern matching — turns out to be directly applicable to the central problems of molecular biology. Not by analogy. Directly. A genome is a string. Reading alignment is substring search. Assembly is string reconstruction. Gene prediction is pattern matching with a probabilistic model.

Bioinformatics is fundamentally string processing — genomes are strings over a 4-letter alphabet, reads are short strings that must be matched against long strings, and most computational problems reduce to finding patterns in sequences. The string algorithms underlying modern genome alignment (BWA, Bowtie2) and assembly (SPAdes) are among the most elegant in computer science. Understanding them explains why these tools work and what their limitations are.

## Exact String Matching: Naive to KMP

**Naive algorithm**: For pattern $P$ of length $m$ and text $T$ of length $n$, try matching $P$ at every position in $T$: $O(nm)$ worst case.

**Knuth-Morris-Pratt (KMP)**: Preprocesses the pattern to build a **failure function** (or prefix function) that records how much of the pattern can be reused after a mismatch. Never backtracks in the text: $O(n + m)$ worst case.

**Boyer-Moore**: Precomputes bad character and good suffix heuristics; skips large portions of the text. $O(n/m)$ on average for random text — sublinear. Used in `grep` for short patterns.

For bioinformatics, exact matching is often too restrictive (real reads have sequencing errors, and we want approximate matches). But exact matching is used internally in larger algorithms for seed finding.

## Suffix Arrays: Indexing All Suffixes

A **suffix array** (SA) for string $T$ of length $n$ is the sorted array of all $n$ suffixes of $T$. Each entry $SA[i]$ gives the starting position of the $i$-th lexicographically smallest suffix.

**Example**: $T$ = `BANANA$` (appending `$` as a unique sentinel smaller than all other characters)

Suffixes sorted: `$`, `A$`, `ANA$`, `ANANA$`, `BANANA$`, `NA$`, `NANA$`

$SA$ = [6, 5, 3, 1, 0, 4, 2]

**Construction**: Naively $O(n^2)$ or $O(n^2 \log n)$; advanced algorithms (Skew/DC3, SA-IS) achieve $O(n)$.

**Query**: Binary search the SA to find all occurrences of pattern $P$ in $O(m \log n)$ time; with the LCP array, reduce to $O(m + k)$ where $k$ is the number of matches.

Suffix arrays are the basis of most modern genome indexes.

## Burrows-Wheeler Transform (BWT) and FM-Index

The **BWT** is a reversible string transformation that clusters similar characters together, enabling better compression AND efficient substring search — the combination underlying Bowtie, BWA, and all modern short-read aligners.

It turns out that compressibility and searchability are not competing properties — in the BWT, they emerge from the same structural feature. The BWT of a text clusters together all characters that appear in the same context, which makes it highly compressible. And this clustering also makes it possible to search the transformed string efficiently without uncompressing it.

### BWT Construction

1. Form all cyclic rotations of $T\$$ (append sentinel `$`)
2. Sort all rotations lexicographically
3. The BWT is the last column of the sorted rotations matrix

**Example**: $T$ = `BANANA$`

Sorted rotations:
```
$BANANA  →  A
A$BANAN  →  N
ANA$BAN  →  N
ANANA$B  →  B
BANANA$  →  $
NA$BANA  →  A
NANA$BA  →  A
```

BWT($T$) = `ANNB$AA`

The BWT clusters characters: all occurrences of the same character that are followed by the same character in the original string appear together in the BWT. This makes it highly compressible (run-length encoding on `ANNB$AA` works well).

### FM-Index: Searching the BWT

The **FM-index** combines the BWT with an occurrence table (`Occ[c][i]` = number of character `c` in `BWT[1..i]`) and a count array (`C[c]` = number of characters in $T$ that are less than `c`) to enable substring search:

**Backward search** for pattern $P = P[m] P[m-1] \ldots P[1]$:

```
sp = 0; ep = n  # current suffix array range
for i from m down to 1:
    c = P[i]
    sp = C[c] + Occ[c][sp - 1] + 1
    ep = C[c] + Occ[c][ep]
    if sp > ep: return "not found"
return SA[sp..ep]  # all match positions
```

Each step takes $O(1)$ with precomputed Occ table → total $O(m)$ for a pattern of length $m$, regardless of genome size. BWA and Bowtie2 align millions of reads per minute because each read query is $O(m)$ against a prebuilt FM-index of the reference.

**Approximate matching**: BWA-backtrack (old algorithm) extends FM-index to allow mismatches by branching at each mismatch, exploring all possible corrections — exponential in worst case but fast in practice for few mismatches. BWA-MEM (current standard) uses seed finding (exact matches) + Smith-Waterman extension.

## k-mer Methods

A **k-mer** is a subsequence of length $k$. k-mer counting is fundamental to:
- **Genome assembly**: k-mer overlap graphs (de Bruijn graphs)
- **Error correction**: k-mers with low frequency are likely sequencing errors
- **Metagenomics**: k-mer profiles for taxonomic classification
- **Genome comparison**: k-mer Jaccard similarity

**k-mer counting** with a hash table:

```python
from collections import Counter

def count_kmers(seq: str, k: int) -> Counter:
    return Counter(seq[i:i+k] for i in range(len(seq) - k + 1))

# For canonical k-mers (DNA is double-stranded, count both strands together)
from Bio.Seq import Seq
def canonical_kmer(kmer: str) -> str:
    rc = str(Seq(kmer).reverse_complement())
    return min(kmer, rc)  # lexicographically smaller = canonical

def count_canonical_kmers(seq: str, k: int) -> Counter:
    return Counter(canonical_kmer(seq[i:i+k]) for i in range(len(seq) - k + 1))
```

Genome-scale k-mer counting (billions of k-mers) requires tools like jellyfish or KMC that use hash tables with compact k-mer encoding (2 bits per base for DNA → k-mers fit in 64-bit integers for k ≤ 32).

## de Bruijn Graphs: The Core of Genome Assembly

A **de Bruijn graph** for a set of sequences is a directed graph where:
- Nodes are all unique (k-1)-mers in the reads
- A directed edge connects node $u$ to node $v$ if there is a k-mer in the reads whose first $k-1$ bases are $u$ and last $k-1$ bases are $v$

**Assembly**: An **Eulerian path** through the de Bruijn graph (visiting every edge exactly once) corresponds to the assembled genome sequence.

For a genome of length $G$ with $N$ reads of length $L$ and k-mer size $k$:
- Each read contributes $L - k + 1$ k-mers
- Total edges: $N(L - k + 1)$
- Nodes: $\leq \min(4^{k-1}, N(L-k+1))$

Finding Eulerian paths is $O(V + E)$ — linear. The difficulty in real assembly is **repeats**: repeated sequences create ambiguous branches in the de Bruijn graph. Resolving them requires paired-end reads (linking nodes far apart in the graph) and long reads (spanning the repeat). This is not an algorithmic limitation but a reflection of a genuine biological ambiguity: without long-range information, there is simply no way to determine how many times a repeated element appears or in what order.

## Why This Matters for Computational Biology

Short-read sequencing depends entirely on FM-index alignment. Every RNA-seq analysis, ChIP-seq experiment, and variant call begins with BWA or STAR, which implement the BWT/FM-index algorithms described here. Understanding BWT explains the fundamental reason why genome alignment is possible in reasonable time — without it, each read would require an $O(nm)$ Smith-Waterman scan over the reference. de Bruijn graphs explain genome assembly — why different k-mer sizes produce different assemblies, why repeats cause mis-assemblies, and why long-read technologies fundamentally solve the problem that short reads cannot. k-mer methods underpin metagenomics, population genomics, and rapid genome comparison — tools like sourmash, Mash, and CLARK are k-mer algorithms.
