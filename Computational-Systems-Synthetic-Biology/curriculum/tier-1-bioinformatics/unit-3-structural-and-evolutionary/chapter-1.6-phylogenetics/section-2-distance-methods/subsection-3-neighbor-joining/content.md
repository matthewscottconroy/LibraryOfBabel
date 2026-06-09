# Neighbor-Joining (NJ)

UPGMA fails when lineages evolve at different rates — and in real biology, they almost always do. A bacterium with a generation time of 20 minutes and a whale with a generation time of 10 years will accumulate mutations at entirely different rates per calendar year. The molecular clock is an approximation at best. What you need is a method that can recover the correct tree topology even when branches have very different total lengths. That is exactly what Naruya Saitou and Masatoshi Nei achieved in 1987 with neighbor-joining.

**Neighbor-joining (NJ)** (Saitou & Nei, 1987) is the most widely used distance-based tree-building method in molecular phylogenetics. It overcomes the primary weakness of UPGMA by not assuming a molecular clock, producing unrooted trees where different lineages can have different total rates of substitution. Despite being nearly 40 years old, NJ remains widely used due to its speed, simplicity, and reasonably good performance on well-behaved data.

## The Q-Matrix Criterion

NJ selects neighbors (pairs of taxa to join) based on a **transformed distance** called the Q-matrix, which minimizes the total branch length of the tree:

For taxa $i$ and $j$ among $n$ remaining taxa:

$$Q_{ij} = (n-2)D_{ij} - \sum_{k} D_{ik} - \sum_{k} D_{jk}$$

The pair $(i, j)$ with the minimum $Q_{ij}$ is joined. This criterion corrects for the "long-branch effect" in UPGMA: a taxon with a very long branch (many total substitutions from all others) will have large distances to all others but will not be incorrectly joined with the next-most-distant taxon, because the Q-correction accounts for the cumulative distances from other taxa. The subtracted terms $\sum_k D_{ik}$ and $\sum_k D_{jk}$ represent the "isolation" of each taxon — how far it is from everything else. By correcting for isolation, NJ avoids the UPGMA trap of grouping long-branch taxa together simply because they are both far from everything.

## The NJ Algorithm

1. Compute the Q-matrix for the initial distance matrix.
2. Find the pair $(i, j)$ with the minimum $Q_{ij}$.
3. Create a new internal node $u$ and connect $i$ and $j$ to it with branch lengths:

$$\ell_{iu} = \frac{D_{ij}}{2} + \frac{\sum_k D_{ik} - \sum_k D_{jk}}{2(n-2)}, \quad \ell_{ju} = D_{ij} - \ell_{iu}$$

4. Remove $i$ and $j$ from the matrix and add the new node $u$ with distances to remaining taxa:

$$D_{uk} = \frac{D_{ik} + D_{jk} - D_{ij}}{2}$$

5. Repeat until only 3 taxa (nodes) remain; connect these directly.

The key feature: branch lengths $\ell_{iu}$ and $\ell_{ju}$ can be different — $i$ and $j$ do not need to have evolved at the same rate from their common ancestor $u$. This is why NJ does not require a molecular clock. The asymmetric branch length calculation is NJ's fundamental advance over UPGMA.

## Star Decomposition

NJ starts from an implicit "star" tree (all taxa connected to a single internal node) and successively "decomposes" the star by finding and joining the pair of neighbors that most reduces the total tree length. This star decomposition analogy gives the method its conceptual framework: at each step, we replace the star relationship between two taxa with a branching relationship, building the tree from the outside (tips) inward.

## Computational Complexity

NJ runs in $O(n^3)$ time for $n$ taxa: at each step, the $n \times n$ Q-matrix must be computed (requiring $O(n^2)$ operations), and this is repeated $n-2$ times. For $n = 10{,}000$ taxa (a common scale for bacterial phylogenomics), this is $10^{12}$ operations — computationally demanding but feasible with optimized implementations. **FastNJ** and **RapidNJ** reduce this to $O(n^2)$ or better using approximate nearest-neighbor search.

## Conditions for NJ to Recover the Correct Tree

NJ produces the correct tree topology when the input distances are **additive** — meaning the true distance equals the sum of branch lengths along the path between two taxa in the true tree. If distances are additive (or approximately additive), NJ is guaranteed to recover the true unrooted topology.

In practice, distances are always estimated with error (finite-site sampling), and evolutionary models are simplifications of the true substitution process. NJ is consistent (converges to the correct tree as data size → ∞) under reasonable model conditions.

## Limitations of NJ

**Distance-based = information loss**: Reducing the full alignment to a pairwise distance matrix loses information — particularly about which specific sites changed and in what evolutionary context. Character-based methods (ML, Bayesian) use all site patterns and are more powerful. You might expect the distance summary to be a lossless compression, but it turns out that different site patterns that produce the same pairwise distance can support different tree topologies, and a distance matrix cannot distinguish between them.

**Long-branch attraction (LBA)**: When two lineages have very long branches (many substitutions), NJ (and parsimony) tends to group them together artifactually — they share many independently derived states by chance. ML methods using rate-variation models (Γ distribution) are more robust to LBA.

**Single distance matrix**: NJ cannot partition the alignment into independently evolving partitions (as in partitioned ML analysis), losing the ability to handle heterogeneous substitution rates across genes.

## Practical Use

Despite limitations, NJ remains valuable:
- **Quick reference tree** for quality checking before full ML analysis.
- **Guide tree** for MUSCLE or MAFFT progressive alignment.
- **Rapid approximate phylogeny** for thousands of taxa where ML is computationally prohibitive.

```r
library(ape)
library(phangorn)
alignment <- read.phyDat("sequences.fasta", format = "fasta")
dist_matrix <- dist.ml(alignment, model = "JC69")
nj_tree <- NJ(dist_matrix)
plot(nj_tree)
```

## Why This Matters

Neighbor-joining demonstrates that correct tree reconstruction is possible even without assuming a molecular clock, and its star-decomposition logic provides intuition for how all distance methods balance pairwise distances to find the best tree — insight directly applicable to understanding why distance methods can fail when distances are non-additive due to model misspecification or rate heterogeneity. NJ is also the historical bridge between the simple clustering of UPGMA and the character-based methods of maximum likelihood: it keeps the computational speed of distance approaches while abandoning their most unrealistic assumption. Understanding NJ prepares you to understand exactly what you gain by moving to maximum likelihood.
