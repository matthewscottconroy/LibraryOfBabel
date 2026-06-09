# Dynamic Programming Alignments

Dynamic programming (DP) is the algorithmic engine behind pairwise sequence alignment. Two classical algorithms — Needleman-Wunsch for global alignment and Smith-Waterman for local alignment — solve the alignment problem exactly in polynomial time by exploiting the **optimal substructure** of the problem: the best alignment of two sequences through any cell of the DP table necessarily uses the best alignment of the subsequences ending at that cell.

This might sound abstract, but the intuition is concrete. Imagine you have already determined the best possible alignment of the first 10 amino acids of protein A with the first 10 amino acids of protein B. Now you want to extend to position 11. You do not need to re-examine all possible alignments of the first 10 characters — you already know the best one. The optimal alignment through position (11, 11) builds on the already-optimal alignment through (10, 10), or (10, 11), or (11, 10), whichever leads to the highest total score. This is optimal substructure, and it is what makes DP correct.

## The Core Insight: Optimal Substructure

Any optimal alignment of sequences $A[1..m]$ and $B[1..n]$ must end in one of three ways:
1. $A[m]$ aligned to $B[n]$ (diagonal move)
2. $A[m]$ aligned to a gap (gap in $B$; moving left)
3. A gap aligned to $B[n]$ (gap in $A$; moving up)

This means the optimal alignment through $(m,n)$ is determined by whichever of these three choices — when combined with the already-optimal alignment of the preceding positions — gives the best score. This recursive structure makes DP exact and efficient.

## Needleman-Wunsch: Global Alignment

The **Needleman-Wunsch** (NW) algorithm finds the optimal global alignment of two sequences. Developed in 1970, it was one of the first applications of dynamic programming to biology, and it established the algorithmic template that underlies nearly all modern alignment tools. Define $H(i,j)$ as the score of the best alignment of $A[1..i]$ and $B[1..j]$:

**Initialization:**
$$H(0,0) = 0$$
$$H(i,0) = i \cdot d \quad \text{for } i = 1, \ldots, m$$
$$H(0,j) = j \cdot d \quad \text{for } j = 1, \ldots, n$$

where $d$ is the linear gap penalty.

**Recursion:**
$$H(i,j) = \max \begin{cases} H(i-1,j-1) + \sigma(A[i], B[j]) & \text{(match/mismatch)} \\ H(i-1,j) + d & \text{(gap in } B) \\ H(i,j-1) + d & \text{(gap in } A) \end{cases}$$

**Traceback**: start at $H(m,n)$ and follow the path back to $H(0,0)$.

The initialization encodes the assumption of global alignment: the entire first sequence must align to the entire second. Gaps at the beginning cost the same as gaps anywhere else. This is appropriate when you are comparing two full-length orthologs and expect them to align from end to end.

### Worked Example

Align `ACGT` and `AGT` with match = +2, mismatch = -1, gap = -2:

|   |   | A | G | T |
|---|---|---|---|---|
|   | 0 |-2 |-4 |-6 |
| A |-2 | 2 | 0 |-2 |
| C |-4 | 0 | 1 |-1 |
| G |-6 |-2 | 2 | 0 |
| T |-8 |-4 | 0 | 4 |

Traceback from $H(4,3) = 4$:
- $(4,3) \to (3,2)$: diagonal, T/T match → score +2
- $(3,2) \to (2,1)$: diagonal, G/G match → score +2
- $(2,1) \to (1,1)$: up (delete C, gap)
- $(1,1) \to (0,0)$: diagonal A:A

Alignment:
```
A C G T
A - G T
```
Score: 2 + (-2) + 2 + 2 = **4** ✓

## Smith-Waterman: Local Alignment

The **Smith-Waterman** (SW) algorithm finds the highest-scoring local alignment — the best-matching substring pair between $A$ and $B$. Introduced in 1981, it solved a problem that NW could not: how to find a conserved domain embedded in otherwise divergent proteins, without the flanking dissimilarity contaminating the alignment score.

The modification is minimal but profound: add a floor of zero to the recursion.

**Recursion:**
$$H(i,j) = \max \begin{cases} 0 & \text{(reset: start new alignment)} \\ H(i-1,j-1) + \sigma(A[i], B[j]) \\ H(i-1,j) + d \\ H(i,j-1) + d \end{cases}$$

**Traceback**: start at the **maximum value anywhere in the table** (not necessarily the bottom-right), and trace back until a zero is encountered.

The biological motivation: we might be searching for a conserved kinase domain shared by two proteins that are otherwise unrelated. The domain may appear anywhere in each sequence, and flanking regions should not penalize the score. The zero floor means that any alignment extension that would make the score negative is simply abandoned — you start fresh. The maximum cell in the table represents the end of the best local match, and the traceback to a zero cell recovers exactly the boundaries of that match.

## Affine Gap Penalties: The Gotoh Algorithm

Linear gap penalties treat each position in a gap identically. But biologically, a deletion of five bases is typically one mutation event, not five. **Affine gap penalties** better model this:

$$\gamma(\ell) = g_o + (\ell - 1) \cdot g_e$$

where $g_o$ is the gap-open penalty and $g_e$ is the gap-extension penalty ($g_o > g_e > 0$).

The **Gotoh algorithm** extends NW/SW to handle affine gaps using three matrices. Each matrix tracks the best score for a different kind of ending state:

- $H(i,j)$: best alignment score ending at $(i,j)$
- $E(i,j)$: best score where $A[i]$ is aligned to a gap (gap in $B$, extends right)
- $F(i,j)$: best score where $B[j]$ is aligned to a gap (gap in $A$, extends down)

$$E(i,j) = \max \begin{cases} E(i,j-1) + g_e \\ H(i,j-1) + g_o \end{cases}$$

$$F(i,j) = \max \begin{cases} F(i-1,j) + g_e \\ H(i-1,j) + g_o \end{cases}$$

$$H(i,j) = \max \begin{cases} H(i-1,j-1) + \sigma(A[i], B[j]) \\ E(i,j) \\ F(i,j) \end{cases}$$

The key insight in the Gotoh formulation: $E(i,j)$ distinguishes between continuing an existing gap (cost $g_e$) and opening a new one (cost $g_o$). Because $g_o > g_e$, the algorithm will prefer to extend existing gaps rather than open new ones — which is precisely the biological behavior we want to encourage.

This runs in $O(mn)$ time and $O(mn)$ space, same as the basic DP.

## Complexity and Space Reduction

Both NW and SW run in $O(mn)$ time. Storing the full $m \times n$ DP table requires $O(mn)$ space — for two proteins of length 1000, that is $10^6$ cells, easily tractable. For comparing reads against a chromosome, this becomes impractical.

**Hirschberg's algorithm** computes the optimal alignment in $O(mn)$ time but only $O(\min(m,n))$ space using divide-and-conquer: it finds the midpoint column of the optimal path using only two rows, then recursively handles each half.

## When to Use Which Algorithm

| Scenario | Algorithm | Reason |
|----------|-----------|--------|
| Two full-length orthologous proteins | Needleman-Wunsch | Both should align end-to-end |
| Domain search within long protein | Smith-Waterman | Best local match ignoring flanks |
| Read vs. genome | Heuristic (BWA, BWT) | Too large for exact DP |
| Close paralogs, conserved regions | SW with BLOSUM62 | Local, protein-aware scoring |

The EMBOSS Needle (NW) and EMBOSS Water (SW) tools implement exact DP alignment for small-scale use and are available freely at the EBI web server. For most large-scale work, heuristic methods are necessary — but understanding which type of alignment you want (global or local) is the first decision you will make, regardless of which tool you use.

## Why This Matters

Needleman-Wunsch and Smith-Waterman are rarely run directly in modern bioinformatics — the data volumes are too large. But they remain the gold standard against which all heuristic aligners are calibrated. The EMBOSS Needle and Water tools implement them for small-scale exact alignment. Crucially, understanding DP alignments is required to interpret the output of heuristic aligners: CIGAR strings, alignment scores, and gap patterns all make sense only if you understand the underlying DP mechanics. Every genome browser, every read pileup, every multiple alignment ultimately traces its logic back to these two algorithms. When a variant caller reports an indel at a particular position, or when BLAST returns a local alignment with gaps in unexpected places, the correct interpretation requires you to understand what DP alignment is doing behind the scenes.
