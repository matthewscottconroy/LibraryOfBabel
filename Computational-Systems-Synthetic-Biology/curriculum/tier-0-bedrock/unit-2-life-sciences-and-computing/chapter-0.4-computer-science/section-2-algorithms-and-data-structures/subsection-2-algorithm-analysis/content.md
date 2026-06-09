# Algorithm Analysis

Before you write a single line of code for a large-scale analysis, there is a question you should be able to answer: will this finish? Not "will it be fast" — but will it actually complete in a reasonable time, on the hardware you have, within this decade? It turns out that for many biologically motivated algorithms, the answer depends entirely on the mathematical structure of the approach, and you can determine the answer with a few minutes of analysis before running anything.

Computational biology operates at scales that make algorithm choice critical. Aligning 30× coverage of a human genome means processing ~100 billion bases. Genome assembly uses hundreds of gigabytes of RAM. Protein structure prediction scales with protein length in ways that determine whether a calculation is feasible at all. **Big O notation** is the vocabulary for analyzing and communicating algorithm efficiency. Knowing it lets you predict whether an algorithm will run in seconds, hours, or longer than the age of the universe before you waste compute time finding out.

## Big O Notation: Formal Definition

Big O notation describes the **asymptotic upper bound** on the growth rate of a function. Formally:

$f(n) = O(g(n))$ if there exist constants $c > 0$ and $n_0$ such that $f(n) \leq c \cdot g(n)$ for all $n \geq n_0$.

In algorithm analysis, $n$ is the input size (number of items, string length, graph nodes, etc.) and $f(n)$ is the number of elementary operations (time complexity) or memory units (space complexity).

**Intuition**: Big O captures the dominant term and ignores constants. An algorithm that performs $3n^2 + 7n + 12$ operations is $O(n^2)$ because for large $n$, the $3n^2$ term dominates everything else.

## Complexity Classes

From fastest to slowest growth:

| Complexity | Name | Example |
|---|---|---|
| $O(1)$ | Constant | Hash table lookup, array index |
| $O(\log n)$ | Logarithmic | Binary search, BST operations |
| $O(n)$ | Linear | Single array traversal, linear scan |
| $O(n \log n)$ | Linearithmic | Merge sort, FFT |
| $O(n^2)$ | Quadratic | Naive all-vs-all comparison |
| $O(n^3)$ | Cubic | Naive matrix multiplication |
| $O(2^n)$ | Exponential | Brute-force subset enumeration |
| $O(n!)$ | Factorial | Brute-force TSP |

**Key calibration**: For $n = 10^9$ (roughly human genome size in bp):
- $O(\log n)$: ~30 operations — instant
- $O(n)$: $10^9$ operations — seconds to minutes
- $O(n \log n)$: ~$3 \times 10^{10}$ — minutes to hours
- $O(n^2)$: $10^{18}$ — longer than the age of the universe

This is why genome-wide all-vs-all comparison ($O(n^2)$) is infeasible and why everything in genomics depends on clever indexing and approximations that reduce complexity to $O(n \log n)$ or better. The entire edifice of modern sequencing analysis — the FM-index, the de Bruijn graph, the MinHash sketch — exists because someone looked at a naively $O(n^2)$ problem and found a cleverer approach.

## Best, Average, and Worst Case

Big O typically describes worst-case complexity, but average-case matters in practice:

- **Quicksort**: $O(n \log n)$ average, $O(n^2)$ worst case (sorted input with naive pivot). In practice, with randomized pivot selection, worst case is astronomically unlikely.
- **Hash table lookup**: $O(1)$ average, $O(n)$ worst case (all keys collide — pathological).
- **Binary search**: $O(\log n)$ worst case always.

For bioinformatics, BWA-MEM uses suffix arrays for exact-match seeding ($O(m)$ per read, where $m$ is read length) then extends seeds with Smith-Waterman ($O(m^2)$ per seed, but with SIMD acceleration and short seeds, this is fast in practice).

## Space Complexity

Space complexity uses the same Big O notation. It measures additional memory used by the algorithm (excluding input):

- **In-place sort**: $O(1)$ extra space (e.g., heapsort)
- **Merge sort**: $O(n)$ extra space (temporary arrays)
- **Dynamic programming table for Needleman-Wunsch**: $O(nm)$ space (length $n$ vs. length $m$ sequences)
- **Hirschberg's linear-space alignment**: $O(\min(n, m))$ space (reduces by dividing the problem into smaller subproblems)

Space complexity is often the binding constraint in bioinformatics. A 64 GB RAM server cannot hold an $O(n^2)$ DP table for $n = 10^9$.

## Amortized Analysis

Some operations are expensive occasionally but cheap on average. **Amortized analysis** averages the cost over a sequence of operations:

**Dynamic array doubling**: When you append to a Python list and it runs out of capacity, Python doubles the internal array and copies all elements — an $O(n)$ operation. But this happens rarely enough that the amortized cost per append is $O(1)$.

**Proof**: Starting from capacity 1, $n$ appends cause $\log_2 n$ doublings with costs $1, 2, 4, \ldots, n/2, n$. Total work = $2n - 1$ extra copies over $n$ appends → $O(1)$ amortized per append.

## Reduction and Problem Hardness

**Reduction**: Showing that problem A reduces to problem B means that if you can solve B, you can solve A (by transforming the input). Reductions establish relative difficulty.

- If A reduces to B in polynomial time, and B is in P (polynomial time), then A is in P.
- **NP-complete** problems: No known polynomial algorithm; every NP problem reduces to them. Example relevant to biology: de novo protein design (exactly what sequence has target structure) is NP-hard in simplified lattice models.
- **NP-hard problems in bioinformatics**: Shortest superstring (genome assembly), de novo multiple sequence alignment (optimal), protein folding in some models.

In practice, bioinformatics handles NP-hard problems with:
- **Greedy heuristics**: accept locally optimal choices; fast but not guaranteed optimal
- **Dynamic programming over restricted structure**: Needleman-Wunsch is polynomial because it imposes a sequential structure on alignments
- **Approximation algorithms**: guaranteed within a factor of optimal
- **Probabilistic/heuristic methods**: BLAST, BLAST-like tools use seeds + extension; not optimal but fast and sufficient

## Worked Example: Comparing Two Implementations

Two implementations of all-pairwise k-mer similarity among $N = 500$ bacterial genomes:

**Implementation A**: For each pair, compute Jaccard similarity by comparing k-mer sets.
- Pairs: $\binom{500}{2} = 124,750$
- k-mer comparison per pair: $O(K)$ where $K \sim 10^6$ (genome size / k)
- Total: $124,750 \times 10^6 \approx 10^{11}$ operations → impractical

**Implementation B**: MinHash — compress each genome to a signature of $s = 1000$ hash values (sketches), then compare signatures.
- Sketch construction: $N \times O(K)$ = $500 \times 10^6 = 5 \times 10^8$ operations → done in seconds
- Pairwise comparison: $\binom{500}{2} \times O(s)$ = $124,750 \times 1000 \approx 10^8$ operations → seconds
- Total: $\sim 10^9$ operations vs. $10^{11}$ for exact → 100× faster with guaranteed approximation error bounds

This is the mash/sourmash approach for genome distance estimation — complexity analysis explains exactly why sketching works.

## Why This Matters for Computational Biology

Algorithm analysis explains why the tools you use are designed the way they are — and enables you to predict whether a new approach will scale to your data. When a collaborator proposes running pairwise alignment on $10^4$ sequences, you can immediately calculate $\binom{10^4}{2} \times O(L^2) \approx 10^8 \times L^2$ and determine if it is feasible. When choosing between STAR (RNA-seq aligner, FM-index-based, fast) and HISAT2 (graph FM-index, splice-aware) for your dataset, understanding their complexity differences informs the choice. When writing your own analysis, recognizing that a nested loop over genomic positions is $O(n^2)$ tells you to reach for an interval tree instead.
