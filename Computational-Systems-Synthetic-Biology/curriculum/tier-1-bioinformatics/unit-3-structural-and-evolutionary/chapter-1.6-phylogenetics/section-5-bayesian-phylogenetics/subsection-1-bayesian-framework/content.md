# Bayesian Phylogenetics Framework

Maximum likelihood gives you the best tree — the topology and branch lengths that maximize the probability of the data. But it does not tell you how uncertain that answer is. Is the best tree barely better than the second-best, or dramatically more probable? Is there a whole range of trees that explain the data almost as well? These questions matter enormously for biological interpretation: a clade that is supported by a tree with slightly better likelihood than thousands of competitors deserves less confidence than one that dominates the entire distribution of plausible trees.

Bayesian phylogenetics was developed to answer these questions directly. Rather than finding a single best tree, it computes a probability distribution over all possible trees — the **posterior probability distribution** — which simultaneously encodes the best estimate and all the uncertainty around it.

**Bayesian phylogenetics** provides a principled statistical framework for quantifying uncertainty in phylogenetic inference. Rather than finding a single best tree (as in ML), Bayesian methods compute a probability distribution over all possible trees — the **posterior probability distribution** — which directly quantifies how probable each tree and model parameter combination is given the observed sequence data.

## Bayes' Theorem Applied to Phylogenetics

Bayes' theorem provides the mathematical foundation:

$$P(T, \boldsymbol{\ell}, \boldsymbol{\theta} \mid D) = \frac{P(D \mid T, \boldsymbol{\ell}, \boldsymbol{\theta}) \cdot P(T, \boldsymbol{\ell}, \boldsymbol{\theta})}{P(D)}$$

In words:

**Posterior** ∝ **Likelihood** × **Prior**

where:
- $P(T, \boldsymbol{\ell}, \boldsymbol{\theta} \mid D)$: The **posterior distribution** — probability of tree $T$, branch lengths $\boldsymbol{\ell}$, and model parameters $\boldsymbol{\theta}$ given the alignment $D$. This is what we want to compute.
- $P(D \mid T, \boldsymbol{\ell}, \boldsymbol{\theta})$: The **likelihood** — probability of the alignment given the tree and model parameters. This is the same quantity maximized in ML analysis.
- $P(T, \boldsymbol{\ell}, \boldsymbol{\theta})$: The **prior** — our belief about tree topologies, branch lengths, and model parameters before seeing the data.
- $P(D) = \int P(D \mid \cdot) P(\cdot) d(\cdot)$: The **marginal likelihood** (normalizing constant) — extremely difficult to compute analytically but not required when using MCMC sampling.

The denominator $P(D)$ is the marginal likelihood, and it requires integrating over all possible trees, branch lengths, and model parameters — a sum over an astronomically large space. This is why Bayesian phylogenetics requires MCMC: you can sample from the posterior without computing the normalizing constant, because the Metropolis-Hastings acceptance ratio involves only the ratio of posterior densities, in which $P(D)$ cancels.

## Prior Distributions in Phylogenetics

**Topology prior**: A uniform prior over all possible unrooted tree topologies is standard — each topology is equally probable a priori. This means the posterior probability of a topology directly reflects how well it explains the data, without topology-specific prior weighting.

**Branch length prior**: An exponential prior is standard: $P(\ell_i) = \text{Exp}(\text{mean} = 0.1)$ — expressing the belief that most branches are short (few substitutions), but allowing for long branches. Some analyses use a gamma prior.

**Model parameter priors**: Dirichlet distributions for base frequencies ($\pi$), log-normal or exponential for the kappa (Ti/Tv), and uniform for the gamma shape parameter $\alpha$.

Prior choice matters more than many practitioners appreciate. If your prior strongly constrains branch lengths to be short and the data support a very long branch, the posterior will be a compromise between the data and the prior — and with short alignments where the data are weak, the prior can dominate. In practice, MrBayes and BEAST2 have carefully calibrated default priors that work well for most analyses, but you should always verify that the priors are not overly influential by checking whether the posterior differs substantially from the prior.

## Posterior Probability of a Clade

The key output of Bayesian phylogenetic analysis is the **posterior probability of each clade** — the fraction of sampled trees in which that clade appears. This is computed directly from the MCMC sample:

$$PP(\text{clade}_k) = \frac{\text{number of sampled trees containing clade}_k}{\text{total trees sampled}}$$

A posterior probability of 0.98 means that 98% of sampled trees support that clade — a direct probabilistic statement about evolutionary relationships, given the data and model. This is one of the most compelling features of the Bayesian approach: the output is directly interpretable as probability.

## Comparison: ML Bootstrap vs. Bayesian Posterior Probability

A common question is how to interpret ML bootstrap support (BS) vs. Bayesian posterior probabilities (PP):

| Feature | ML Bootstrap Support | Bayesian Posterior Probability |
|---|---|---|
| Definition | Frequency of clade across bootstrap resampled datasets | Probability of clade given data and model |
| Scale | 0–100% | 0–1 |
| Typical threshold for "strong support" | ≥ 70–80% | ≥ 0.95 |
| Accounts for model uncertainty | No (uses fixed model) | Yes (integrates over parameters) |
| Computation | Relatively fast | MCMC (slower) |
| Interpretation | Resampling-based confidence measure | True statistical probability |

**Key caveat**: Bayesian posterior probabilities tend to be systematically higher than ML bootstrap values for the same clades. A PP of 0.95 does NOT have the same meaning as a BS of 95. In practice, even a PP of 0.95 may correspond to weaker phylogenetic evidence than expected if the model is misspecified (the posterior converges confidently to a wrong tree if the model is wrong). Bayesian methods inherit all model assumptions from the likelihood, and the posterior probability only measures probability relative to those assumptions.

## The Role of the Prior

Bayesian phylogenetics requires explicit specification of priors. Well-calibrated priors are important: an overly informative prior (e.g., strongly constraining branch lengths) can dominate the posterior even in the presence of large amounts of data. Conversely, completely flat priors on unbounded parameters can cause improper posteriors (non-integrable distributions).

In practice, priors used in MrBayes and BEAST2 are the result of decades of experience and validation; the defaults are generally appropriate for standard analyses.

## Why This Matters

The Bayesian framework provides the most rigorous statistical approach to phylogenetic uncertainty — the posterior probability distribution encompasses tree topology, branch lengths, and model parameters simultaneously — enabling formal propagation of uncertainty from phylogeny into downstream analyses such as ancestral sequence reconstruction, divergence time estimation, and diversification rate analysis. When you want to say "I am 95% confident that whales and hippos are sister taxa," you need a Bayesian framework to make that statement meaningful. Maximum likelihood gives you the best tree; Bayesian phylogenetics tells you how confident you should be in it.
