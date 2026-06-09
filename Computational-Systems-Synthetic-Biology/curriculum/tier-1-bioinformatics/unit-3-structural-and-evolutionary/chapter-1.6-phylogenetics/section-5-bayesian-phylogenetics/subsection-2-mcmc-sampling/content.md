# MCMC Sampling in Phylogenetics

The Bayesian posterior distribution in phylogenetics is a probability distribution over a space of staggering complexity: every possible combination of tree topology, branch lengths, and model parameters. Computing this distribution analytically is completely out of reach — the space is too large, and the integral for the normalizing constant is intractable. What you need instead is a method that can sample from this distribution without computing it explicitly. That is precisely what Markov Chain Monte Carlo achieves, and it is one of the most powerful computational ideas in modern statistics.

The posterior distribution in Bayesian phylogenetics lives in an astronomically large space — it is a distribution over all possible tree topologies, branch lengths, and model parameters simultaneously. Direct computation of the posterior is analytically intractable for any dataset of practical size. **Markov Chain Monte Carlo (MCMC)** methods solve this problem by constructing a random walk through parameter space that, at convergence, samples from the posterior distribution — without ever computing the normalizing constant $P(D)$.

## The Metropolis-Hastings Algorithm

**Metropolis-Hastings (MH)** is the fundamental MCMC algorithm for Bayesian phylogenetics. Starting from an arbitrary tree topology and parameter values, MH generates a sequence of samples from the posterior:

1. **Propose a new state**: From the current state $x = (T, \boldsymbol{\ell}, \boldsymbol{\theta})$, propose a new state $x'$ by applying one of several **proposal moves** (see below). The proposal is drawn from a proposal distribution $q(x' \mid x)$.

2. **Compute the acceptance ratio**:

$$\alpha = \min\left(1, \frac{P(D \mid x') P(x')}{P(D \mid x) P(x)} \cdot \frac{q(x \mid x')}{q(x' \mid x)}\right)$$

This ratio compares the posterior density at the proposed state to the current state, corrected for any asymmetry in the proposal distribution. Crucially, the normalizing constant $P(D)$ cancels in the ratio.

3. **Accept or reject**: Accept the proposed state $x'$ with probability $\alpha$. If accepted, move to $x'$; if rejected, remain at $x$.

4. **Repeat**: After many iterations, the chain converges to sampling from the posterior distribution.

The cancellation of $P(D)$ in the acceptance ratio is the key insight: you never need to compute the integral over all trees. You only need to compare the posterior density at two nearby points — the current state and the proposed state. The ratio is computable because both the likelihood and the prior can be evaluated for any specific tree.

## Proposal Moves in Phylogenetics

Different proposal moves perturb different aspects of the current state:

**Topological moves (change the tree topology)**:
- **NNI on topology**: Propose a NNI swap around a randomly chosen internal branch.
- **SPR move**: Prune and regraft a subtree.
- **TBR move**: Bisect and reconnect with a new branch.
These are discrete moves in the topology space.

**Branch length updates**:
- **Multiplier proposal**: Scale a randomly chosen branch length by $e^{\lambda(U - 0.5)}$ where $U \sim \text{Uniform}(0,1)$ and $\lambda$ controls the step size. This is a Hastings ratio ≠ 1 move.
- **ALL branch scale**: Scale all branch lengths simultaneously by the same factor.

**Model parameter updates**:
- **κ (Ti/Tv)**: Propose a new value from a sliding window.
- **Base frequencies** ($\pi$): Propose using a Dirichlet distribution or a simplex slide move.
- **Γ shape** (α): Multiplier proposal.

The MCMC chain typically runs with a **mixture** of proposal moves: some proportion of steps are topology moves (producing accepted changes infrequently), others are parameter moves (accepted more frequently). Tuning the proportion and step size of each move type is critical for efficient sampling.

## Convergence Diagnostics

A crucial challenge: how do you know when the MCMC chain has converged to sampling from the posterior? Running a chain for more iterations does not guarantee convergence — the chain might be stuck in a local region of parameter space.

**ESS (Effective Sample Size)**: The number of "effectively independent" samples in the MCMC trace, accounting for autocorrelation. Because successive MCMC samples are correlated (the chain moves slowly), the effective information content is less than the nominal chain length. ESS > 200 for all parameters is the standard threshold; ESS < 100 indicates insufficient mixing. ESS is computed by **Tracer** (a standalone GUI tool by Andrew Rambaut) from the MCMC log file.

**PSRF (Potential Scale Reduction Factor)**: Compares the within-chain variance to the between-chain variance when running multiple independent chains. PSRF ≈ 1.0 indicates chains have converged to the same distribution; PSRF > 1.05–1.10 indicates non-convergence — the chains are sampling from different regions of the posterior.

**Tracer visualization**: Plot the parameter trace (value vs. MCMC generation) for all continuous parameters. A well-mixed chain shows no trend, rapid oscillation, and a mean that is constant over the sampled region. A chain still converging shows a long-term trend. A chain with poor mixing shows very slow oscillation (stuck in one region).

## Mixing vs. Convergence

**Convergence** means the chain has reached the stationary distribution (the posterior). **Mixing** means the chain is efficiently exploring the distribution rather than moving slowly through it.

A chain can converge (reach the right distribution) but mix poorly (move through it slowly), producing highly autocorrelated samples with low ESS. Increasing the step sizes of proposal moves generally improves mixing but decreases acceptance rates. Optimal acceptance rates for continuous parameters: ~23–50% (depending on dimension).

It turns out that a common mistake is to run a chain, see that the trace "looks flat," and declare convergence. You also need to check that the chain has actually explored the full posterior — that it has visited the range of plausible topologies and parameter values, not just one region of them. Running multiple independent chains with different starting points and checking that they agree is the only robust way to verify this.

## Burn-in Period

The early portion of an MCMC chain (before convergence) must be discarded — the **burn-in**. During burn-in, the chain is influenced by its (arbitrary) starting values. Typical burn-in: 10–25% of the total chain length. After burn-in removal, the remaining samples are used to compute the posterior summary (consensus tree, parameter means and credible intervals).

## Running Multiple Independent Chains

Always run at least **2 independent chains** (ideally 4) starting from different random starting points. Comparing PSRF across chains tests convergence. If two chains exploring different starting points converge to the same parameter estimates and tree topology, confidence in convergence is much higher than from a single chain. MrBayes runs 2 chains in two independent runs by default (`nruns = 2`).

## Why This Matters

MCMC is the computational engine that makes Bayesian phylogenetics possible — without it, the posterior distribution over the enormous tree space would be completely inaccessible — but the method's validity depends on convergence, which requires careful diagnostic checking; a non-converged MCMC analysis produces meaningless posterior probabilities that can be dangerously over-confident in wrong tree topologies. Every published Bayesian phylogenetic analysis should report ESS values, PSRF values, and Tracer diagnostics — and if these are missing from a paper you are reading, that is a warning sign that the convergence may not have been properly verified.
