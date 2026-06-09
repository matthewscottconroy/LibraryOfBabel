# Tree Search Algorithms

You have a likelihood function and a substitution model. You know how to compute the probability of any given tree. The question now is: which tree? There are so many possible tree topologies that this is one of the hardest combinatorial optimization problems in computational biology. Understanding why this is hard — and what the best tools do about it — is essential for running reliable analyses and interpreting their results.

Finding the maximum likelihood tree requires searching over an enormous discrete space of topologies. For $n$ taxa, the number of possible unrooted bifurcating trees grows super-exponentially:

$$\text{Number of unrooted trees} = (2n-5)!! = 1 \times 3 \times 5 \times \cdots \times (2n-5)$$

For $n = 10$: 945 trees. For $n = 20$: $\approx 2.2 \times 10^{20}$ trees. For $n = 50$: $\approx 3 \times 10^{74}$ trees — a number that dwarfs the number of atoms in the observable universe. Exhaustive evaluation of all topologies is feasible only for $n \leq 10$–12. For larger datasets, heuristic search strategies are mandatory.

This is not a computational nuisance that faster computers will eventually eliminate. Even at a trillion likelihood calculations per second, evaluating all topologies for 50 taxa would require far longer than the age of the universe. Heuristic search is not a compromise — it is the only possible approach.

## Heuristic Search Strategies

**Stepwise Addition (Starting Tree)**: Before local search, a starting tree must be generated. The most common approach is to add taxa one at a time, placing each new taxon in the position that best improves the likelihood (greedy stepwise addition). An NJ or BioNJ tree is also commonly used as the starting tree. The quality of the starting tree matters: a better starting tree means the local search begins closer to the true optimum and requires fewer iterations.

**Nearest Neighbor Interchange (NNI)**: For each internal branch in the tree, swap the two pairs of adjacent subtrees on each side, producing 2 alternative trees per internal branch (there are $n-3$ internal branches for $n$ taxa). Accept the swap if it improves likelihood. NNI is fast (each NNI step evaluates $2(n-3)$ trees) but local — it makes small changes to the tree.

**Subtree Pruning and Regrafting (SPR)**: Prune a subtree from the tree (remove a branch and the subtree attached to it) and regraft it at a different position (reattach by inserting a new internal node on any other branch). For a tree with $n$ taxa, there are $O(n^2)$ possible SPR moves per step. SPR explores more of tree space than NNI and can "escape" local optima more easily.

**Tree Bisection and Reconnection (TBR)**: Bisect the tree by removing an internal branch (producing two subtrees), then reconnect by inserting a new branch between any branch of subtree 1 and any branch of subtree 2. TBR is the most thorough of the standard local moves, exploring $O(n^3)$ possible trees per step.

The progression NNI → SPR → TBR represents increasing exploration radius at increasing computational cost. A key insight is that local search can get stuck in local optima — arrangements that are better than their immediate neighbors but not the global best. This is the fundamental challenge of discrete combinatorial optimization, and it requires explicit strategies to escape.

## IQ-TREE2: The Current Standard

**IQ-TREE2** (Minh et al., 2020) uses a sophisticated stochastic NNI + perturbation strategy that substantially outperforms standard NNI or SPR:

1. **Multiple starting trees**: IQ-TREE2 generates 100 starting trees (different NJ or BIONJ trees, with random starts) and selects the best $L$ for full optimization. This guards against getting stuck in a poor local optimum.

2. **Stochastic NNI** (SNNI): Random NNI moves with random perturbation (randomly rearranging some branches) allow escape from local optima.

3. **Perturbation phase**: When the search stagnates (no improvement in $N$ consecutive NNI rounds), a random perturbation (random SPR or TBR moves) is applied to restart the search from a different point.

4. **Parallel computation**: IQ-TREE2 parallelizes likelihood calculations across multiple cores using OpenMP.

```bash
# IQ-TREE2 standard ML analysis
iqtree2 -s alignment.fasta \
        -m GTR+G4 \
        -B 1000 \           # 1000 ultrafast bootstrap replicates
        -T AUTO \           # Auto-detect number of CPU threads
        -o outgroup \       # Specify outgroup for rooting
        --prefix output
```

## Partitioned Analysis for Multi-Gene Datasets

For phylogenomic datasets comprising multiple genes (each with potentially different evolutionary rates and models), **partitioned analysis** fits a separate substitution model and branch length scalers to each gene partition while constraining them to share the same tree topology:

$$\ln L = \sum_{\text{partitions}} \ln L_{\text{partition}}$$

IQ-TREE's ModelFinder can also determine the optimal partitioning scheme (merging partitions with similar substitution parameters) to avoid overparameterization.

The rationale for partitioned analysis is biological: different genes evolve at different rates and under different constraints. Combining them into a single model forces all genes to share the same substitution parameters, which misspecifies the model for all of them. Partitioned analysis allows each gene to have its own model while still estimating a single shared topology — capturing the signal from all genes without forcing them into a one-size-fits-all framework.

## Ultrafast Bootstrap Approximation (UFBoot2)

Standard nonparametric bootstrap resamples the alignment 1,000 times and infers a full ML tree for each replicate — computationally prohibitive for large phylogenomic datasets. **UFBoot** (Ultrafast Bootstrap, implemented in IQ-TREE2) provides an efficient approximation:

1. Generate bootstrap alignments by resampling columns.
2. Rather than running full ML from scratch on each bootstrap alignment, UFBoot uses the ML tree topology as a starting point and applies a rapid SNNI to optimize it for each replicate.
3. Apply a convergence criterion to stop when bootstrap support values are stable.

UFBoot reduces bootstrap computation time by 10–100× compared to standard bootstrap, making bootstrap support practical for phylogenomic analyses with hundreds of taxa. The `-B 1000` flag in IQ-TREE2 runs 1000 UFBoot replicates.

**Caution**: UFBoot values tend to be slightly overestimated compared to standard bootstrap. IQ-TREE2 recommends using `--bnni` (boot-strap NNI optimization) to improve calibration: this additionally performs NNI optimization on candidate bootstrap trees, substantially improving the correspondence between UFBoot and standard bootstrap values.

```bash
iqtree2 -s alignment.fasta -m GTR+G4 -B 1000 --bnni -T 8 -o outgroup
```

## Why This Matters

Tree search is where phylogenetics meets computational optimization — the algorithms used to navigate tree space determine whether the analysis finds the true ML tree or gets stuck in a local optimum, making IQ-TREE2's stochastic search strategy with multiple starting points a substantial practical improvement over simpler NNI or SPR search that dominated earlier tools like PhyML and RAxML. For most practical analyses, IQ-TREE2 with GTR+Γ, model selection, and UFBoot with --bnni is the current gold standard.
