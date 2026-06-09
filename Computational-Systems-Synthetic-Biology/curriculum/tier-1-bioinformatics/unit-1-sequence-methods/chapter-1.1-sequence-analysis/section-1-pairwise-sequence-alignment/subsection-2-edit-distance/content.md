# Edit Distance

Before the elegant probabilistic machinery of substitution matrices and gap penalties, there is a simpler and more fundamental concept: **edit distance**. Edit distance quantifies how different two strings are by counting the minimum number of elementary operations required to transform one into the other. It is the mathematical bedrock on which alignment scoring is built — and understanding it makes the more sophisticated machinery feel like a natural generalization rather than an arbitrary recipe.

Think of it this way: when you compare the word "KITTEN" to the word "SITTING," you intuitively recognize that they are similar but not identical. Edit distance formalizes that intuition and gives it a number. For biological sequences, the same idea applies — two protein sequences that differ at 10 positions with three small insertions have an edit distance that summarizes how much evolutionary work separated them from their common ancestor. That number is not merely a similarity measure; it is a compressed history.

## Definition

The **Levenshtein distance** $d(A, B)$ between strings $A$ and $B$ is the minimum number of single-character operations — insertions, deletions, and substitutions — required to transform $A$ into $B$.

Formally, given:
- $A = a_1 a_2 \ldots a_m$
- $B = b_1 b_2 \ldots b_n$

the edit distance is defined recursively. Let $d(i, j)$ be the edit distance between the first $i$ characters of $A$ and the first $j$ characters of $B$:

$$d(i,0) = i \quad \text{(delete all of } A[1..i]\text{)}$$
$$d(0,j) = j \quad \text{(insert all of } B[1..j]\text{)}$$
$$d(i,j) = \begin{cases}
d(i-1, j-1) & \text{if } a_i = b_j \\
1 + \min \begin{cases} d(i-1, j) & \text{(deletion)} \\ d(i, j-1) & \text{(insertion)} \\ d(i-1, j-1) & \text{(substitution)} \end{cases} & \text{if } a_i \neq b_j
\end{cases}$$

The recursion has a natural geometric interpretation. Each cell $(i,j)$ of the dynamic programming table represents the subproblem of aligning the first $i$ characters of $A$ to the first $j$ characters of $B$. Moving diagonally corresponds to a match or substitution; moving left corresponds to inserting a character; moving up corresponds to deleting a character. The edit distance is the minimum-cost path from the top-left corner to the bottom-right corner of this table.

## Worked Example

Compute the edit distance between `KITTEN` and `SITTING`:

|   |   | S | I | T | T | I | N | G |
|---|---|---|---|---|---|---|---|---|
|   | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
| K | 1 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
| I | 2 | 2 | 1 | 2 | 3 | 4 | 5 | 6 |
| T | 3 | 3 | 2 | 1 | 2 | 3 | 4 | 5 |
| T | 4 | 4 | 3 | 2 | 1 | 2 | 3 | 4 |
| E | 5 | 5 | 4 | 3 | 2 | 2 | 3 | 4 |
| N | 6 | 6 | 5 | 4 | 3 | 3 | 2 | 3 |

The edit distance is $d(6,7) = 3$:
1. K → S (substitution)
2. E → I (substitution)
3. append G (insertion)

Fill in the table from top-left to bottom-right, applying the recurrence at each cell. Read the path backward from the bottom-right to reconstruct the sequence of operations. This traceback is exactly the same operation you will perform to read out a sequence alignment from the Needleman-Wunsch DP table.

## Relationship to Sequence Alignment

Edit distance and sequence alignment are formally equivalent when the alignment scoring scheme assigns:
- Match cost = 0
- Mismatch cost = 1
- Gap penalty = 1 (linear)

In this equivalence, **minimizing edit distance** corresponds exactly to **maximizing alignment score** with these cost parameters. The dynamic programming table computed for edit distance is structurally identical to the Needleman-Wunsch alignment table.

Bioinformatics generalizes edit distance in two important ways, and it is worth pausing to appreciate why each generalization exists.

**1. Non-uniform substitution costs**: Not all substitutions are equal. Replacing leucine (L) with isoleucine (I) (both hydrophobic, similar size) should be penalized less than replacing leucine with glutamate (E) (charge reversal). Substitution matrices encode these differential costs derived from evolutionary data. Edit distance treats all substitutions as equivalent; biological alignment does not.

**2. Affine gap penalties**: A single 5-base deletion is one biological event, yet Levenshtein distance would penalize it as 5 separate operations. This misrepresents the biology. A single recombination or replication slippage event can produce a multi-base indel in one mutational step. **Affine gap penalties** charge a large cost to open a gap and a smaller cost to extend it:

$$\gamma(\ell) = g_\text{open} + (l-1) \times g_\text{extend}$$

This better models the evolutionary reality that indels tend to occur as single events affecting contiguous positions.

## Variants of Edit Distance

Different biological problems call for different definitions of "edit":

- **Hamming distance**: only substitutions allowed; sequences must be same length. Used for comparing fixed-length sequences (e.g., barcodes, k-mers).
- **Longest Common Subsequence (LCS)**: only insertions and deletions, no substitutions. The complement of LCS length gives a dissimilarity measure.
- **Damerau-Levenshtein**: adds transpositions (swapping adjacent characters) to the operation set — useful for spell checking but less common in bioinformatics.

Each variant is appropriate for a different biological context. Hamming distance is the right tool for comparing UMI barcodes in single-cell sequencing — all barcodes are the same length, and you are simply asking how many positions differ. LCS is natural when thinking about conserved subsequences without positional constraints. Levenshtein distance is the general-purpose tool when both substitutions and indels are expected.

## Computational Complexity

The standard dynamic programming computation of Levenshtein distance runs in $O(mn)$ time and $O(mn)$ space, where $m$ and $n$ are the lengths of the two strings. For aligning short sequences (hundreds of bases), this is trivial. For aligning reads against a 3 Gb genome, it is computationally prohibitive — which motivates the heuristic methods (BWT-FM index, BLAST seeds) discussed in later sections.

Space can be reduced to $O(\min(m,n))$ using Hirschberg's divide-and-conquer algorithm, which computes the optimal alignment in linear space by only keeping two rows of the DP table at a time and recursively determining the midpoint of the optimal path. This is a beautiful result: you can compute the full optimal alignment without storing the full table, just by being clever about when you commit to decisions.

## Python Implementation

```python
def levenshtein(a: str, b: str) -> int:
    m, n = len(a), len(b)
    # Use only two rows to save space
    prev = list(range(n + 1))
    curr = [0] * (n + 1)
    for i in range(1, m + 1):
        curr[0] = i
        for j in range(1, n + 1):
            if a[i-1] == b[j-1]:
                curr[j] = prev[j-1]
            else:
                curr[j] = 1 + min(prev[j],    # deletion
                                  curr[j-1],   # insertion
                                  prev[j-1])   # substitution
        prev, curr = curr, prev
    return prev[n]

print(levenshtein("ACGT", "AGT"))   # Output: 1
print(levenshtein("KITTEN", "SITTING"))  # Output: 3
```

## Why This Matters

Edit distance provides the most elementary and assumption-free measure of sequence dissimilarity. It is used in practical tools wherever exact or near-exact matching is needed at speed: short-read aligners use bounded edit distance search to find mapping positions; barcode demultiplexing allows mismatches defined by Hamming distance; error correction in sequencing uses edit distance to cluster reads into consensus sequences. Understanding edit distance as the origin of all alignment scoring schemes provides the conceptual anchor for understanding why alignment parameters matter and what they represent. When you later choose a gap-open penalty for a protein alignment or set an edit distance threshold for a demultiplexing step, you are making an explicit statement about evolutionary or technical processes — and that statement should be grounded in the biology, not chosen arbitrarily.
