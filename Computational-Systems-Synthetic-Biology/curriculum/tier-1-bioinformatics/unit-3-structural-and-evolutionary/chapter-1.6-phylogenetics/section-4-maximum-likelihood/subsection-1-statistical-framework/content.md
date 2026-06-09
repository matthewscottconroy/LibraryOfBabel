# Maximum Likelihood Framework

In 1981, Joe Felsenstein published a paper that changed phylogenetics forever. The paper, "Evolutionary trees from DNA sequences: a maximum likelihood approach," did something that seemed obvious in retrospect but had never been done: it asked what tree topology, branch lengths, and model parameters make the observed sequence data most probable. Not most parsimonious. Not most similar-looking. Most probable, under an explicit probabilistic model of how sequences evolve.

This was a transformation from ad hoc criteria to rigorous statistical inference. Parsimony asks: what tree minimizes changes? Maximum likelihood asks: what tree best explains the data? The distinction matters enormously when evolutionary change is not rare — when sites mutate multiple times, when some lineages evolve faster than others, when the assumptions of parsimony break down. ML is consistent where parsimony is not, and it provides a natural framework for model comparison, hypothesis testing, and uncertainty quantification that parsimony simply cannot offer.

**Maximum likelihood (ML)** is the dominant paradigm for modern molecular phylogenetics. Rather than minimizing ad hoc criteria (as in parsimony or distance methods), ML explicitly defines a probabilistic model of molecular evolution and finds the tree and model parameters that make the observed data most probable. This statistical foundation provides rigorous model comparison, consistent parameter estimation, and natural integration with hypothesis testing.

## The Likelihood of a Tree

The ML objective is to find the tree topology $T$, branch lengths $\boldsymbol{\ell}$, and model parameters $\boldsymbol{\theta}$ that maximize the probability of observing the alignment $D$:

$$L(T, \boldsymbol{\ell}, \boldsymbol{\theta}) = P(D \mid T, \boldsymbol{\ell}, \boldsymbol{\theta})$$

For computational convenience, the **log-likelihood** $\ell = \ln L$ is maximized (sum of log-probabilities over sites vs. product of probabilities):

$$\ln L = \sum_{c=1}^{N_{\text{sites}}} \ln P(D_c \mid T, \boldsymbol{\ell}, \boldsymbol{\theta})$$

where $D_c$ is the character state at site $c$ across all taxa.

The factorization over sites is not an approximation — it follows from the assumption that sites evolve independently, which is violated in practice (covariation between positions exists, especially in RNA and protein secondary structures) but is tractable and generally robust enough for most analyses.

## Felsenstein's Pruning Algorithm

Computing the likelihood of a site pattern given a tree is non-trivial because we must sum over all possible ancestral states at internal nodes (which are unobserved). **Felsenstein's pruning algorithm** (Felsenstein, 1981) computes this efficiently using dynamic programming.

Define $L_v(s)$ = the probability of the observed data in the subtree rooted at node $v$, given that node $v$ is in state $s$. For a leaf node with observed state $A$: $L_v(A) = 1$, $L_v(s) = 0$ for $s \neq A$.

For an internal node $v$ with children $u$ and $w$ connected by branches of length $t_{vu}$ and $t_{vw}$:

$$L_v(s) = \left[\sum_{s'} P(s \to s' \mid t_{vu}) L_u(s')\right] \times \left[\sum_{s'} P(s \to s' \mid t_{vw}) L_w(s')\right]$$

where $P(s \to s' \mid t)$ is the substitution model probability of state $s$ changing to $s'$ over branch length $t$.

At the root, the site likelihood is:

$$P(D_c) = \sum_s \pi_s L_{\text{root}}(s)$$

where $\pi_s$ is the equilibrium frequency of state $s$. The recursion processes the tree from leaves to root in $O(n)$ time per site.

The genius of this algorithm is that it sums over all possible ancestral states at all internal nodes — an exponentially large set of possibilities — in linear time, by exploiting the conditional independence structure of the tree. This is dynamic programming applied to phylogenetics, and it made maximum likelihood computationally feasible.

## Comparison to Parsimony

The key differences from parsimony:

1. **ML uses branch lengths**: Parsimony only counts changes; ML weights each branch by the probability of substitution over that branch length, implicitly accounting for the fact that longer branches are expected to accumulate more change.

2. **ML has a substitution model**: The matrix $P(s \to s' \mid t)$ encodes the substitution model parameters (relative rates, base frequencies), which can be estimated from the data. Parsimony assumes all changes are equally costly.

3. **ML sums over ancestral states**: Rather than minimizing the number of reconstructed states (as Fitch parsimony does), ML marginalizes over all possible ancestral configurations weighted by their probability under the model.

4. **ML can account for rate variation**: By mixing Felsenstein's computation across different rate categories (see Subsection 2, Γ model), ML correctly handles the fact that some sites evolve faster than others. Parsimony has no analogous mechanism.

5. **Statistical consistency**: Under correct model specification, ML is a consistent estimator — it converges to the true tree as data size increases. Parsimony is inconsistent in the Felsenstein zone.

Points 4 and 5 are the most important. Rate variation across sites is biologically real and pervasive, and ignoring it produces systematically wrong results. The Γ model for rate variation, combined with the GTR substitution model, is the direct solution to parsimony's long-branch attraction problem.

## The Likelihood Surface and Tree Search

The likelihood is a function of both the discrete tree topology and the continuous branch lengths/model parameters. For a fixed topology, branch lengths and model parameters are optimized numerically (gradient ascent, Brent's method). The challenging part is searching over the discrete space of topologies (see Subsection 5 on tree search algorithms).

For $n = 20$ taxa, there are ~$2 \times 10^{20}$ unrooted topologies. Heuristic search (NNI, SPR, TBR) explores a small fraction of this space by starting from a plausible tree (often NJ-based) and iteratively improving it.

## Hypothesis Testing with Likelihood Ratios

A major advantage of the ML framework is the ability to formally test biological hypotheses by comparing the log-likelihoods of nested models:

$$\Lambda = 2(\ln L_1 - \ln L_0) \sim \chi^2_{df}$$

where $df$ = number of additional parameters in model 1 vs. model 0. This **likelihood ratio test (LRT)** is used to: (1) test molecular clock hypothesis (constrained vs. unconstrained branch lengths); (2) compare substitution models (JC vs. HKY vs. GTR); (3) test for positive selection (ω < 1 constrained vs. ω estimated freely).

The LRT is asymptotically exact under the null hypothesis when models are nested. This gives phylogenetics a formal hypothesis-testing framework that parsimony and distance methods simply lack. You can ask not just "which tree is best?" but "is this tree significantly better than the alternative?" — and get a p-value.

## Why This Matters

The maximum likelihood framework transformed phylogenetics from a field dominated by ad hoc methods to a rigorous statistical science — providing consistent estimators, principled model selection, and formal hypothesis testing — and remains the foundation for modern phylogenomic analyses, Bayesian methods, and molecular clock dating. Every analysis in the sections that follow — Bayesian phylogenetics, molecular clocks, dN/dS selection analysis — builds directly on the likelihood framework introduced here.
