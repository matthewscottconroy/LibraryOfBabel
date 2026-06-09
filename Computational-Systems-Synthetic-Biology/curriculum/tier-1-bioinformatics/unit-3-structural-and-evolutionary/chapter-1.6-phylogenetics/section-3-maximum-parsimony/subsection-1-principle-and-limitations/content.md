# Maximum Parsimony

There is something deeply appealing about parsimony as a scientific principle. If two explanations account equally well for the data, prefer the simpler one. Applied to phylogenetics, this becomes: prefer the evolutionary history that requires the fewest changes. It is elegant, computationally tractable, and philosophically defensible. It was also, for several decades, the dominant approach in molecular systematics. And then Joseph Felsenstein proved, mathematically, that under certain realistic conditions, it converges to the wrong answer. Understanding both the appeal and the failure of parsimony is essential for reading three decades of phylogenetic literature — and for recognizing when older analyses need to be revisited.

**Maximum parsimony (MP)** is a character-based phylogenetic method that selects the tree topology requiring the fewest evolutionary changes to explain the observed sequence data. Rooted in the philosophical principle of **Occam's Razor** (among competing explanations, prefer the simplest), parsimony was the dominant phylogenetic method from the 1960s through the 1990s and remains influential in morphological systematics.

## The Parsimony Principle

Given an alignment and a proposed tree topology, the **parsimony length** (or tree length) is the minimum number of character state changes (substitutions) required to explain all observed characters at the tips of the tree, allowing ancestral states to be assigned freely at internal nodes. Maximum parsimony selects the tree with the shortest parsimony length — the tree requiring the fewest evolutionary events.

**Underlying assumption**: Evolution is parsimonious — convergent evolution (independent origin of the same character state in different lineages) and parallel evolution are rare relative to retention of ancestral states.

## Parsimony-Informative Sites

Not all alignment positions contribute to differentiating between tree topologies. Only **parsimony-informative sites** affect the parsimony score:

A site is parsimony-informative if it has **at least 2 taxa sharing each of at least 2 different character states**. Concretely: the pattern must require more substitutions on some topologies than others.

Example (4 taxa A, B, C, D):
- Pattern AAGG (A and B have A; C and D have G): Informative — this pattern supports the topology ((A,B),(C,D)) over alternatives.
- Pattern AAGC (A, B have A; C has G; D has C): Also informative — equally parsimonious on some topologies.
- Pattern AAAG (3 taxa have A; one has G): **Invariant for some topologies** — non-informative for distinguishing topologies (one substitution no matter which tree).
- Pattern AAAA (all the same): Constant site — uninformative.

The concept of parsimony-informative sites is actually useful beyond parsimony itself. Even in likelihood analyses, sites with no variation contribute nothing to topology estimation. Knowing what fraction of your alignment is parsimony-informative gives you a rough sense of how much phylogenetic signal is present in the data.

## The Fitch Algorithm for Computing Tree Length

The **Fitch algorithm** (Fitch, 1971) computes the minimum number of substitutions required for a given tree topology and character (single alignment column) in two passes:

**First pass (bottom-up, parsimony)**: For each internal node, compute the set of states consistent with the character states at its descendants with minimum cost:
- If the intersection of descendant state sets is non-empty: the node's state set = intersection (no substitution needed at this node).
- If the intersection is empty: the node's state set = union (one substitution is required to transition between the two child state sets; the parsimony score increments by 1).

**Second pass (top-down, traceback)**: Choose actual states at internal nodes to minimize overall cost (the traceback step).

The total parsimony length for a tree equals the sum of substitution costs over all sites and all internal branches.

The Fitch algorithm is elegant and efficient — it reduces ancestral state reconstruction to a straightforward tree traversal. It also illustrates the core computational structure that reappears in Felsenstein's pruning algorithm for likelihood: both traverse the tree bottom-up, computing a quantity at each node from its children. The conceptual parallel is deep.

## Tree Search: Why Exhaustive Search Fails

The number of possible unrooted tree topologies grows super-exponentially with the number of taxa:

$$\text{Number of unrooted trees} = \frac{(2n-5)!}{2^{n-3}(n-3)!} = (2n-5)!!$$

For $n = 10$ taxa: 945 trees (feasible). For $n = 20$: ~2.2 × 10^20 trees (infeasible). For $n = 50$: a number larger than the number of atoms in the universe.

This is one of the most important facts in computational phylogenetics: the number of possible trees is so vast that no computer could ever evaluate them all for datasets of practical size. Every phylogenetic method — parsimony, maximum likelihood, Bayesian MCMC — must navigate this space using heuristic strategies. The challenge is not computing the score of any given tree; it is finding the best-scoring tree without evaluating them all.

Heuristic search strategies explore tree space without examining every topology:

**Stepwise addition**: Build a starting tree by adding one taxon at a time to the position that minimizes the parsimony length.

**SPR (Subtree Pruning and Regrafting)**: Prune a subtree from the main tree and reattach it at a different position, accepting the move if parsimony length decreases.

**TBR (Tree Bisection Reconnection)**: Bisect the tree into two parts at any branch, then reconnect with a new internal branch in all possible ways, accepting improvements. TBR is more thorough than SPR but slower.

**NNI (Nearest Neighbor Interchange)**: Swap subtrees adjacent to a given internal branch. Fastest but most local of the heuristics.

## Long-Branch Attraction: The Felsenstein Zone

The most important failure mode of parsimony is **long-branch attraction (LBA)**, described analytically by Felsenstein (1978). In the "Felsenstein zone" — parameter combinations where two lineages evolve rapidly and two slowly — parsimony systematically places the two fast-evolving lineages as sisters, even if the true tree has them as not sister.

The mechanism: fast-evolving lineages accumulate many independently derived changes. These convergent changes appear as shared derived states (synapomorphies), misleading parsimony into inferring a relationship that is not based on common ancestry.

It turns out this is not a rare edge case. Many real biological datasets fall in the Felsenstein zone. Ancient lineages that have been evolving in unusual environments (parasites, endosymbionts, organisms with elevated mutation rates) often have long branches. When two such long-branched taxa appear in the same dataset, parsimony will group them together even if they are not closely related.

Maximum likelihood methods with rate-variation models are substantially more robust to LBA because they explicitly account for the probability of convergent evolution at any site.

## When Parsimony Is Appropriate

- **Morphological characters**: In paleontology, parsimony with step matrices (different costs for different character state transitions) remains standard for morphological data.
- **Indels (insertions/deletions)**: Binary coding of gaps as present/absent and parsimony analysis is common.
- **Slowly evolving, closely related taxa**: When divergence is low and the LBA zone is far from the true parameters, parsimony is approximately equivalent to ML.

## Why This Matters

Understanding maximum parsimony — its logic, its informative site concept, and especially its failure mode (long-branch attraction) — is essential for interpreting the phylogenetics literature from the 1980s–2000s and for recognizing when older parsimony analyses may need to be revisited with likelihood-based methods that correctly model the substitution process. Many of the "resolved" relationships published during the parsimony era were later overturned by ML analyses. The history of phylogenetics is in part a history of recognizing where parsimony goes wrong — and building something better.
