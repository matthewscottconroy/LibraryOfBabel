# Bayesian Statistics

Imagine you are trying to estimate the protein degradation rate in a yeast cell from a Western blot time course. You have six time points, noisy measurements, and a model with three parameters. Standard maximum likelihood estimation will give you point estimates, but it cannot tell you how confident to be in those estimates — or how much the data has actually constrained each parameter compared to your prior beliefs. In a system where you have ten parameters and five measurements, not all parameters will be identifiable, and the ones that are not will have posteriors that look remarkably like their priors. Bayesian inference makes this uncertainty visible rather than hiding it inside a single reported value.

Bayesian statistics offers a coherent framework for updating beliefs in light of data, quantifying uncertainty about parameters as probability distributions, and incorporating prior knowledge. While frequentist inference asks "what is the probability of this data given the null hypothesis?", Bayesian inference asks the more natural question: "given this data, what should I believe about the parameters?" For computational biology, Bayesian methods are indispensable — from variant calling to phylogenetics to ODE model calibration.

## The Bayesian Framework

Bayes' theorem, when applied to inference, takes the form:

$$\underbrace{P(\theta | \mathbf{x})}_{\text{posterior}} = \frac{\underbrace{P(\mathbf{x} | \theta)}_{\text{likelihood}} \cdot \underbrace{P(\theta)}_{\text{prior}}}{\underbrace{P(\mathbf{x})}_{\text{marginal likelihood}}}$$

The **prior** $P(\theta)$ encodes beliefs about $\theta$ before seeing data. The **likelihood** $P(\mathbf{x}|\theta)$ is the probability of the data given parameters — the same quantity maximized in MLE. The **posterior** $P(\theta|\mathbf{x})$ is the updated belief after observing data. The **marginal likelihood** (evidence) $P(\mathbf{x}) = \int P(\mathbf{x}|\theta) P(\theta)\, d\theta$ normalizes the posterior.

The posterior is the complete answer to the inference problem — not a point estimate, but a full probability distribution over all possible parameter values. From the posterior you can compute any summary: the mean, median, mode (the **maximum a posteriori** or MAP estimate), credible intervals, and predictions. This completeness is what makes Bayesian inference so valuable when uncertainty quantification matters — which, in biology with its many unknown parameters and limited data, is most of the time.

## Conjugate Priors

A **conjugate prior** is one where the posterior is in the same distributional family as the prior. Conjugate pairs enable closed-form posterior computation without numerical integration.

**Beta-Binomial conjugacy (most important in genomics):**
- Prior: $\theta \sim \text{Beta}(\alpha, \beta)$ (allele frequency, methylation fraction)
- Likelihood: $x | \theta \sim \text{Binomial}(n, \theta)$ ($x$ alternative allele reads out of $n$ total)
- Posterior: $\theta | x \sim \text{Beta}(\alpha + x, \beta + n - x)$

Interpretation: $\alpha$ and $\beta$ act as "pseudo-counts" of prior successes and failures. As $n \to \infty$, the posterior concentrates around the observed frequency $x/n$, regardless of the prior (data overwhelms prior). This limiting behavior is reassuring: with enough data, a Bayesian analysis agrees with maximum likelihood, regardless of the prior chosen.

**Poisson-Gamma conjugacy:**
- Prior: $\lambda \sim \text{Gamma}(\alpha, \beta)$
- Likelihood: $x_i | \lambda \sim \text{Poisson}(\lambda)$ (count data)
- Posterior: $\lambda | \mathbf{x} \sim \text{Gamma}(\alpha + \sum x_i, \beta + n)$

Used for estimating gene expression rates from count data under simple models.

## Credible Intervals vs. Confidence Intervals

A **Bayesian credible interval** $[a, b]$ with probability $1 - \alpha$ means:

$$P(\theta \in [a, b] | \mathbf{x}) = 1 - \alpha$$

This is exactly what scientists intuitively want: "there is a 95% probability that the parameter lies in this interval, given the data." This stands in contrast to the frequentist confidence interval, which is a random interval that contains the true parameter 95% of the time over hypothetical repeated experiments (but says nothing about the probability of the true parameter).

The **highest posterior density (HPD)** interval is the shortest interval containing the specified posterior probability mass.

This distinction is not merely semantic. When you report a 95% credible interval for a kinetic rate constant in a systems biology model, you are genuinely saying something about your uncertainty in that parameter. The frequentist confidence interval — as useful as it is — cannot support that direct interpretation.

## Markov Chain Monte Carlo

For complex models where the posterior is not conjugate and cannot be computed analytically, **Markov Chain Monte Carlo (MCMC)** generates samples from the posterior by constructing a Markov chain whose stationary distribution is the target posterior.

**Metropolis-Hastings algorithm:**
1. Initialize at $\theta^{(0)}$
2. At each step $t$: propose $\theta^* \sim q(\theta^* | \theta^{(t)})$ (e.g., a random walk)
3. Compute acceptance ratio $A = \min\left(1, \frac{P(\mathbf{x}|\theta^*) P(\theta^*)}{P(\mathbf{x}|\theta^{(t)}) P(\theta^{(t)})} \cdot \frac{q(\theta^{(t)}|\theta^*)}{q(\theta^*|\theta^{(t)})}\right)$
4. Accept: $\theta^{(t+1)} = \theta^*$ with probability $A$; otherwise $\theta^{(t+1)} = \theta^{(t)}$

After a **burn-in** period (discarded initial samples where the chain has not yet converged to the stationary distribution), the remaining samples $\{\theta^{(t)}\}$ approximate draws from the posterior.

**Gibbs sampling** is a special case where each parameter is sampled from its conditional distribution given all others — applicable when conditional distributions are tractable (often with conjugate priors in hierarchical models).

Modern MCMC implementations use **Hamiltonian Monte Carlo (HMC)** and the **No-U-Turn Sampler (NUTS)** — used in Stan, PyMC, and NumPyro — which exploit gradient information to propose large moves and explore high-dimensional posteriors efficiently. The practical consequence is that modern probabilistic programming frameworks can fit Bayesian ODE models with dozens of parameters to time-course data, sampling thousands of posterior draws in minutes on a laptop.

## Bayesian Model Comparison

To compare two models $M_1$ and $M_2$, compute the **Bayes factor**:

$$BF_{12} = \frac{P(\mathbf{x} | M_1)}{P(\mathbf{x} | M_2)} = \frac{\int P(\mathbf{x}|\theta_1, M_1) P(\theta_1|M_1)\, d\theta_1}{\int P(\mathbf{x}|\theta_2, M_2) P(\theta_2|M_2)\, d\theta_2}$$

Bayes factors penalize model complexity automatically (more complex models spread probability over more of parameter space, reducing the marginal likelihood unless the parameters are well-constrained by data). This provides a principled solution to the model selection problem without requiring explicit penalties like AIC/BIC.

The **Bayesian Information Criterion (BIC)** approximates $-2 \log P(\mathbf{x}|M)$ when the prior is approximately flat: $\text{BIC} = k \log n - 2\ell(\hat{\theta})$.

## Why This Matters for Computational Biology

Bayesian statistics is the foundation of several core bioinformatics tools. GATK's variant calling computes posterior genotype probabilities given read data and population-level priors. BEAST and MrBayes use MCMC to infer phylogenetic trees and molecular evolution parameters. DESeq2 uses an empirical Bayes approach (estimating priors from data across all genes) to stabilize variance estimates for genes with low counts. Single-cell analysis methods (Seurat's graph-based clustering, trajectory analysis) include Bayesian components. Perhaps most importantly, ODE-based systems biology modeling increasingly uses Bayesian inference to calibrate parameters and quantify uncertainty — because biology has so many unknown parameters and so little data that the prior matters enormously.

```python
import numpy as np
import matplotlib.pyplot as plt
from scipy import stats

# Bayesian estimation of allele frequency
# Prior: Beta(2, 2) - weak prior toward 0.5
# Data: 15 alternative reads out of 40 total

alpha_prior, beta_prior = 2, 2
x_alt, n_total = 15, 40

# Conjugate update: Beta(alpha + x, beta + n - x)
alpha_post = alpha_prior + x_alt
beta_post = beta_prior + (n_total - x_alt)

theta = np.linspace(0, 1, 300)

prior = stats.beta.pdf(theta, alpha_prior, beta_prior)
likelihood_unnorm = stats.binom.pmf(x_alt, n_total, theta)
posterior = stats.beta.pdf(theta, alpha_post, beta_post)

fig, ax = plt.subplots(figsize=(8, 4))
ax.plot(theta, prior / prior.max(), label=f'Prior: Beta({alpha_prior},{beta_prior})', linestyle='--')
ax.plot(theta, likelihood_unnorm / likelihood_unnorm.max(), label='Likelihood (scaled)', linestyle=':')
ax.plot(theta, posterior / posterior.max(), label=f'Posterior: Beta({alpha_post},{beta_post})', linewidth=2)

# 95% credible interval
ci_low = stats.beta.ppf(0.025, alpha_post, beta_post)
ci_high = stats.beta.ppf(0.975, alpha_post, beta_post)
ax.axvspan(ci_low, ci_high, alpha=0.2, color='green', label=f'95% CI: [{ci_low:.2f}, {ci_high:.2f}]')

ax.axvline(x_alt / n_total, color='red', linestyle='-.', label=f'MLE: {x_alt/n_total:.2f}')
ax.set_xlabel('Allele Frequency θ')
ax.set_ylabel('Scaled Density')
ax.set_title('Bayesian Allele Frequency Estimation')
ax.legend()
plt.tight_layout()
```
