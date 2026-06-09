# Dynamic Programming

In 1970, Saul Needleman and Christian Wunsch published a paper in the *Journal of Molecular Biology* describing an algorithm for aligning protein sequences. In 1981, Temple Smith and Michael Waterman published a variation for finding locally similar subsequences. These two papers collectively define the computational foundation of sequence alignment — the single most important operation in bioinformatics, underlying BLAST, HMMER, read aligners, and protein structure comparison. The technique they both used is dynamic programming.

Dynamic programming (DP) is the algorithmic technique that makes sequence alignment possible. Understanding DP means understanding why these algorithms work, how to implement them, and how to design new DP solutions for novel alignment or optimization problems.

## The Core Idea

**Dynamic programming** solves problems by breaking them into overlapping subproblems, computing each subproblem once, and storing the result. The two conditions that make DP applicable:

1. **Optimal substructure**: The optimal solution to the whole problem contains optimal solutions to subproblems
2. **Overlapping subproblems**: The same subproblems appear repeatedly

Without overlapping subproblems, divide-and-conquer suffices (no need to store results). With overlapping subproblems, DP provides exponential speedup over naive recursion.

The key insight is this: when you ask "what is the best alignment of sequence A to sequence B?", you are really asking "what is the best alignment of A[1..n] to B[1..m]?", which depends on the best alignment of A[1..n-1] to B[1..m-1] (among other subproblems). These subproblems overlap — the same partial alignment appears in many contexts. Computing them once and storing the answer is the essence of DP.

**Memoization** (top-down): recursive implementation with a cache (dictionary) storing computed results.
**Tabulation** (bottom-up): fill a table iteratively in dependency order. Usually faster in practice (no recursion overhead, better cache locality).

## Needleman-Wunsch: Global Sequence Alignment

**Goal**: Align two sequences $S_1$ of length $n$ and $S_2$ of length $m$ end-to-end (global), maximizing alignment score.

**Scoring**: Match: $+\sigma$; Mismatch: $-\mu$; Gap open/extend: linear gaps use penalty $-g$ per gap character.

**DP formulation**: Let $F[i][j]$ = optimal score for aligning $S_1[1..i]$ with $S_2[1..j]$.

**Recurrence** (linear gap penalty):

$$F[i][j] = \max \begin{cases} F[i-1][j-1] + s(S_1[i], S_2[j]) & \text{(match/mismatch)} \\ F[i-1][j] - g & \text{(gap in } S_2\text{)} \\ F[i][j-1] - g & \text{(gap in } S_1\text{)} \end{cases}$$

**Initialization**: $F[0][0] = 0$; $F[i][0] = -ig$; $F[0][j] = -jg$

**Worked example**: Align `ACGT` vs. `AGT` with match=+1, mismatch=-1, gap=-2.

|   |   | A | G | T |
|---|---|---|---|---|
|   | 0 | -2 | -4 | -6 |
| A | -2 | **1** | -1 | -3 |
| C | -4 | -1 | **0** | -2 |
| G | -6 | -3 | **0** | -1 |
| T | -8 | -5 | -2 | **1** |

Traceback from $F[4][3] = 1$: diagonally for A-A, diagonally (mismatch) for C-G → gap in S2, diagonally for G-G, diagonally for T-T.

Alignment:
```
S1: A C G T
S2: A - G T
```
Score: 1 + (-2) + 1 + 1 = 1. ✓

**Complexity**: $O(nm)$ time and space. For the full DP table. For $n = m = 1000$ nt: $10^6$ cells — trivial. For $n = m = 10^9$ (whole-genome): $10^{18}$ cells — impossible. Long-range alignment uses heuristic seed-chain-extend algorithms.

## Smith-Waterman: Local Sequence Alignment

**Goal**: Find the highest-scoring alignment between any subsequence of $S_1$ and any subsequence of $S_2$ — useful when one sequence is much longer (e.g., aligning a read to a genome) or when sequences share a conserved domain but not global similarity.

The intuition: we want to find the best-matching region between two sequences without penalizing for the flanks. A globin domain buried inside a much longer protein should align perfectly to other globins even though the full protein has no similarity outside that domain.

**Modification**: Add a zero option to the recurrence (allows starting a new alignment at any position) and initialize all borders to 0 (no penalty for partial prefixes):

$$H[i][j] = \max \begin{cases} 0 & \text{(start fresh)} \\ H[i-1][j-1] + s(S_1[i], S_2[j]) \\ H[i-1][j] - g \\ H[i][j-1] - g \end{cases}$$

The traceback starts from the maximum value in the entire table (not just the corner).

**Same complexity**: $O(nm)$ time. Smith-Waterman is implemented with SIMD (SSE2/AVX) instructions in tools like SSEARCH and the striped Smith-Waterman library — achieving throughput of ~1 billion cell updates per second on modern hardware.

## Longest Common Subsequence

**Problem**: Find the length of the longest common subsequence (LCS) of two sequences. Used in diff tools and edit distance computation.

Let $L[i][j]$ = LCS of $S_1[1..i]$ and $S_2[1..j]$:

$$L[i][j] = \begin{cases} L[i-1][j-1] + 1 & \text{if } S_1[i] = S_2[j] \\ \max(L[i-1][j], L[i][j-1]) & \text{otherwise} \end{cases}$$

**Edit distance** (Levenshtein) is directly related: minimum number of insertions, deletions, and substitutions to transform $S_1$ into $S_2$. DP recurrence is identical structure with different scoring.

## Viterbi Algorithm for HMMs

Hidden Markov Models (HMMs) are used in gene prediction, profile alignment, and protein family modeling. The **Viterbi algorithm** is a DP that finds the most probable hidden state sequence given observed emissions.

It turns out that gene prediction and sequence alignment, which look like quite different biological problems, are solved by the same mathematical technique. In both cases, you are finding the optimal path through a state space, where the score decomposes into a sum over independent steps. That decomposability is precisely what enables dynamic programming.

Given: observations $O_1, \ldots, O_T$; states $S$; transition matrix $A$; emission matrix $B$.

$$V_t(j) = \max_i \left[ V_{t-1}(i) \cdot A_{ij} \right] \cdot B_{j}(O_t)$$

In log space (to avoid underflow):

$$\log V_t(j) = \max_i \left[ \log V_{t-1}(i) + \log A_{ij} \right] + \log B_j(O_t)$$

HMM gene finders (GeneMark, AUGUSTUS) use Viterbi to label each codon position as exon, intron, or intergenic. Profile HMMs in HMMER use the Viterbi algorithm for sensitive remote homology detection.

## Python Implementation: Smith-Waterman

```python
import numpy as np

def smith_waterman(s1: str, s2: str, match=2, mismatch=-1, gap=-1):
    n, m = len(s1), len(s2)
    H = np.zeros((n + 1, m + 1), dtype=int)
    
    for i in range(1, n + 1):
        for j in range(1, m + 1):
            score = match if s1[i-1] == s2[j-1] else mismatch
            H[i, j] = max(
                0,
                H[i-1, j-1] + score,
                H[i-1, j] + gap,
                H[i, j-1] + gap
            )
    
    # Traceback from maximum
    max_score = H.max()
    i, j = np.unravel_index(H.argmax(), H.shape)
    
    aligned1, aligned2 = [], []
    while H[i, j] > 0:
        score = match if s1[i-1] == s2[j-1] else mismatch
        if H[i, j] == H[i-1, j-1] + score:
            aligned1.append(s1[i-1])
            aligned2.append(s2[j-1])
            i -= 1; j -= 1
        elif H[i, j] == H[i-1, j] + gap:
            aligned1.append(s1[i-1])
            aligned2.append("-")
            i -= 1
        else:
            aligned1.append("-")
            aligned2.append(s2[j-1])
            j -= 1
    
    return max_score, "".join(reversed(aligned1)), "".join(reversed(aligned2))

score, a1, a2 = smith_waterman("HEAGAWGHEE", "PAWHEAE")
print(f"Score: {score}\n{a1}\n{a2}")
```

## Why This Matters for Computational Biology

Every pairwise alignment, every profile search, every gene prediction algorithm, every read aligner's seed extension step uses dynamic programming. Understanding DP means you can read the original Needleman-Wunsch (1970) and Smith-Waterman (1981) papers and understand them completely — a remarkable fact given their fundamental importance. When BLAST reports an e-value, the underlying score is computed by Smith-Waterman on each seed hit. When AUGUSTUS annotates a genome, it is running Viterbi on an HMM over 50 million positions. When you design a custom scoring function for a domain-specific alignment problem (RNA structure, protein-DNA binding), you are designing a DP recurrence.
