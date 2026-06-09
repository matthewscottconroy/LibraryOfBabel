# Exercises — Chapter 20

The exercises develop the core computations and geometric intuitions of information geometry. Exercise 20.5 is implementation-oriented.

**Exercise 20.1.** Compute the Fisher information $g(\theta)$ for: (a) $\text{Bernoulli}(\theta)$; (b) $N(\mu, \sigma^2)$; (c) $\text{Poisson}(\lambda)$. Verify that for the Gaussian, $g$ is diagonal in natural coordinates.

**Exercise 20.2.** Show that the Cramér-Rao bound is tight for the MLE of the Bernoulli parameter: compute the variance of $\hat\theta = \bar{X}_n$ and compare to $1/(n g(\theta))$.

**Exercise 20.3.** For the exponential family, prove that $\nabla_\theta \psi = E_\theta[T(X)]$ and $\nabla^2_\theta \psi = \text{Cov}_\theta[T(X)]$. Conclude that $\psi$ is convex.

**Exercise 20.4.** (Pythagorean Theorem) Prove that the KL divergence between a Gaussian $q = N(\mu, \sigma^2)$ and the MLE projection onto a Gaussian family $\mathcal{E}$ (given i.i.d. data) satisfies the Pythagorean decomposition.

**Exercise 20.5.** (EM) Implement the EM algorithm for Gaussian mixture models on a simple 1D dataset. Interpret each E-step and M-step geometrically as alternating projections in the space of distributions.
