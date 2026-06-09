# UPGMA

If you were building a phylogeny by hand and had no computer, you would probably do something very much like UPGMA. You would look at the distances between your sequences, join the most similar pair first, and work outward from there. It is the most intuitive possible algorithm for converting a matrix of pairwise differences into a tree. And it illustrates, in the clearest possible way, what all distance methods are doing — and why they can fail.

**UPGMA** (Unweighted Pair Group Method with Arithmetic Mean) is the simplest phylogenetic tree-building algorithm. It is an agglomerative hierarchical clustering method applied to a pairwise distance matrix. While rarely the method of choice for serious phylogenetic analysis today, UPGMA illustrates the fundamental concepts of tree-building from distances and remains useful for hierarchical clustering in other contexts (e.g., clustering gene expression data).

## The UPGMA Algorithm

UPGMA builds a tree by iterative merging:

1. Start with each taxon as its own cluster.
2. Find the two clusters $i$ and $j$ with the smallest distance $D_{ij}$.
3. Merge $i$ and $j$ into a new cluster $(ij)$. Place an internal node at height $D_{ij}/2$ (half the merge distance).
4. Update the distance from the new cluster $(ij)$ to each remaining cluster $k$ as the arithmetic mean:

$$D_{(ij)k} = \frac{n_i \cdot D_{ik} + n_j \cdot D_{jk}}{n_i + n_j}$$

where $n_i$ and $n_j$ are the numbers of taxa in clusters $i$ and $j$. For the unweighted version (UPGMA), this simplifies to the average of distances from all members of the merged cluster to $k$.

5. Repeat steps 2–4 until all taxa are merged into a single cluster.

The key step is 3: placing the internal node at height $D_{ij}/2$ means UPGMA assumes both $i$ and $j$ have evolved the same total distance from their common ancestor. Both branches connecting $i$ and $j$ to their shared node have the same length. This is the molecular clock assumption baked directly into the algorithm.

## Worked Example with 4 Taxa

Consider taxa A, B, C, D with distance matrix:

|   | A | B | C | D |
|---|---|---|---|---|
| A | 0 | 4 | 6 | 7 |
| B | 4 | 0 | 6 | 7 |
| C | 6 | 6 | 0 | 5 |
| D | 7 | 7 | 5 | 0 |

**Step 1**: Minimum distance = 4 (A, B). Merge A and B into (AB). Internal node height = 4/2 = 2.

Updated distances: $D_{(AB)C} = (D_{AC} + D_{BC})/2 = (6+6)/2 = 6$; $D_{(AB)D} = (7+7)/2 = 7$.

New matrix:

|      | (AB) | C | D |
|---|---|---|---|
| (AB) | 0    | 6 | 7 |
| C    | 6    | 0 | 5 |
| D    | 7    | 5 | 0 |

**Step 2**: Minimum = 5 (C, D). Merge C and D. Internal node height = 5/2 = 2.5.

Updated: $D_{(AB)(CD)} = (D_{(AB)C} + D_{(AB)D})/2 = (6+7)/2 = 6.5$.

**Step 3**: Merge (AB) and (CD). Internal node height = 6.5/2 = 3.25.

Result: ((A,B),(C,D)) — sisters A+B and sisters C+D, with (AB) diverging from (CD) at height 3.25.

## The Ultrametric Assumption

UPGMA produces an **ultrametric tree** — a tree where all tips are equidistant from the root (height 3.25 in our example). This is because UPGMA places internal nodes at half the merge distance, assuming all lineages have evolved at the same rate (**molecular clock assumption**).

If this assumption is violated — if some lineages have evolved faster than others — UPGMA will give an **incorrect tree topology**, not merely incorrect branch lengths. This is a fundamental limitation, and it is worth emphasizing: the error is not just in the branch lengths. The groupings themselves can be wrong. A lineage that has evolved particularly fast will accumulate large distances to all other taxa, causing UPGMA to group it together with the most distant taxa rather than with its true relatives. In real datasets, evolutionary rates vary substantially between lineages (due to differences in generation time, effective population size, and selective pressures), making UPGMA frequently incorrect for practical phylogenetic inference.

## When UPGMA Is Appropriate

Despite its limitations, UPGMA is still useful in specific contexts:

- **Hierarchical clustering in non-phylogenetic applications**: UPGMA is standard for clustering gene expression heatmaps, distance matrices of ecological samples, or any data where an ultrametric dendrogram is an appropriate representation. The molecular clock assumption is irrelevant in these contexts.
- **Quick visualization of roughly clock-like data**: For highly similar sequences from a single closely related group where rate variation is minimal, UPGMA gives a reasonable quick first look.
- **Inferring the rough topology for ultrametric reference**: In Bayesian analyses where a strict clock is assumed (BEAST2 with strict clock model), a UPGMA starting tree is sometimes used as the initial tree.

## Why This Matters

UPGMA illustrates the core logic of distance-based tree building — hierarchical clustering by merging closest pairs — and its failure mode (topology error when the clock assumption is violated) motivates neighbor-joining (which relaxes the clock) and maximum likelihood methods (which do not require any clock assumption), making UPGMA a conceptually important stepping stone even if rarely the final method of choice. If you understand why UPGMA fails, you understand what every subsequent advance in phylogenetics was designed to fix.
