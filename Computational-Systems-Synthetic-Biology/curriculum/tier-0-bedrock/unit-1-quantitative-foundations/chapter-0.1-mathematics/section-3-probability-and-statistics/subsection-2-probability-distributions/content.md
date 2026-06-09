# Probability Distributions

When DESeq2 was first published in 2010, it solved a problem that had been quietly undermining RNA-seq analysis: the counts of sequencing reads mapping to a gene were being modeled as Poisson-distributed, but the actual data had much more variance than the Poisson model predicted. Using a Poisson model for overdispersed count data inflates false discovery rates — you see differential expression everywhere, most of it noise. The solution was to use the negative binomial distribution, which has a separate parameter controlling the excess variance beyond the Poisson. That one distributional choice made DESeq2 dramatically more reliable than its predecessors.

Probability distributions are the specific probability laws that describe the random variables we encounter in biology. Knowing which distribution governs a biological process — and why — is more than theoretical knowledge: it determines which statistical tests are valid, which models fit the data, and how to detect when something has gone wrong.

## Discrete Distributions

**Bernoulli distribution:** A single binary outcome — success (1) with probability $p$, failure (0) with probability $1-p$. The simplest model for a binary biological decision: will a cell divide or not? Will a particular mRNA be translated? $E[X] = p$, $\text{Var}(X) = p(1-p)$.

**Binomial distribution:** The number of successes in $n$ independent Bernoulli trials, each with probability $p$:

$$P(X = k) = \binom{n}{k} p^k (1-p)^{n-k}$$

Used in allele frequency modeling (Hardy-Weinberg), sequencing coverage (k reads covering a position given n total reads), and variant calling (k alternative allele reads out of n total reads at a site).

**Poisson distribution:** Counts the number of events in a fixed interval when events occur at constant rate $\lambda$ and independently:

$$P(X = k) = \frac{\lambda^k e^{-\lambda}}{k!}, \quad k = 0, 1, 2, \ldots$$

**Key property:** $E[X] = \text{Var}(X) = \lambda$ — mean equals variance. This is a diagnostic: if observed variance greatly exceeds the mean (overdispersion), the Poisson model is inadequate and a negative binomial distribution is needed.

**Biological applications of the Poisson:**
- mRNA molecule counts in a cell (in the simplest gene expression model)
- Mutations per genome per generation (for rare mutations under the Poisson process)
- Read counts at a genomic position in ChIP-seq, ATAC-seq, or RNA-seq (approximately)
- Number of rare binding events per unit time

**Negative binomial distribution:** Generalizes the Poisson by allowing variance to exceed the mean. Parameterized by mean $\mu$ and dispersion parameter $r$:

$$P(X = k) = \binom{k+r-1}{k} \left(\frac{r}{\mu+r}\right)^r \left(\frac{\mu}{\mu+r}\right)^k$$

$\text{Var}(X) = \mu + \mu^2/r$. As $r \to \infty$, the negative binomial approaches Poisson($\mu$). **The negative binomial is the standard distribution for RNA-seq read counts** — it accounts for biological variability (cell-to-cell differences in expression level) on top of the Poisson technical noise from sequencing. DESeq2 and edgeR both model counts as negative binomial. The dispersion parameter $r$ reflects how much genes vary in expression across biological replicates — and it turns out to vary substantially across genes, which is why these tools estimate it separately for each gene.

**Geometric distribution:** The number of trials until the first success ($P(X=k) = (1-p)^{k-1} p$). Models the length of a run of nucleotides before a particular motif, or the waiting time until a rare mutation event (measured in generations).

## Continuous Distributions

**Uniform distribution:** $X \sim \text{Uniform}(a, b)$ with $f(x) = 1/(b-a)$. Used for uninformative priors, random number generation, and as a baseline for significance testing.

**Normal (Gaussian) distribution:** The most important continuous distribution:

$$f(x) = \frac{1}{\sigma\sqrt{2\pi}} \exp\left(-\frac{(x-\mu)^2}{2\sigma^2}\right)$$

By the **Central Limit Theorem**, the mean of $n$ i.i.d. random variables with finite variance converges in distribution to a normal as $n \to \infty$. This justifies many statistical tests. Log-transformed gene expression values are approximately normally distributed, motivating standard linear model approaches (limma uses this). The CLT is not just a mathematical curiosity — it explains why so many biological quantities that are sums or averages of many independent processes (measurement errors, aggregated noise) have approximately normal distributions.

**Exponential distribution:** Models waiting times between events in a Poisson process:

$$f(x) = \lambda e^{-\lambda x}, \quad x \geq 0$$

$E[X] = 1/\lambda$. The exponential distribution is **memoryless**: $P(X > s + t | X > s) = P(X > t)$. In gene circuit modeling, the time between transcriptional bursts is often modeled as exponential; in Markov chain Monte Carlo, the holding time in a Markov state is exponential.

**Beta distribution:** Defined on $[0, 1]$, parameterized by $\alpha, \beta > 0$:

$$f(x) = \frac{x^{\alpha-1}(1-x)^{\beta-1}}{B(\alpha,\beta)}, \quad x \in [0,1]$$

The beta distribution is the natural model for **allele frequencies**, methylation fractions, and any probability parameter. It is the conjugate prior for the binomial — if allele frequency has a Beta($\alpha$, $\beta$) prior and you observe $k$ copies in $n$ total, the posterior is Beta($\alpha + k$, $\beta + n - k$). This elegant updating rule is why the beta-binomial model is so widely used in methylation analysis and population genetics.

**Gamma distribution:** Models positive-valued quantities, especially waiting times for multiple events:

$$f(x) = \frac{\lambda^\alpha x^{\alpha-1} e^{-\lambda x}}{\Gamma(\alpha)}, \quad x > 0$$

With shape $\alpha = 1$, reduces to the exponential. Appears as the prior for Poisson rate parameters and in models of mRNA degradation timescales.

## Multivariate Distributions

**Multivariate normal:** $\mathbf{X} \sim \mathcal{N}(\boldsymbol{\mu}, \Sigma)$ where $\boldsymbol{\mu} \in \mathbb{R}^p$ is the mean vector and $\Sigma \in \mathbb{R}^{p \times p}$ is the positive definite covariance matrix. Gene expression profiles are often modeled as multivariate normal after transformation. The density is:

$$f(\mathbf{x}) = \frac{1}{(2\pi)^{p/2}|\Sigma|^{1/2}} \exp\left(-\frac{1}{2}(\mathbf{x}-\boldsymbol{\mu})^T \Sigma^{-1} (\mathbf{x}-\boldsymbol{\mu})\right)$$

**Dirichlet distribution:** The multivariate generalization of the Beta, defined on the probability simplex. Models distributions over $K$ categories — for example, the relative abundances of taxa in a microbiome sample, or the mixture proportions of cell types. When you perform cell type deconvolution from bulk RNA-seq, the inferred mixture proportions follow a Dirichlet distribution in the Bayesian formulation.

## Why This Matters for Computational Biology

Choosing the right distribution is not an academic exercise — it determines whether your analysis is valid. Using a Poisson model when data is overdispersed inflates false discovery rates dramatically. Assuming normality for count data leads to incorrect inference. The distributions covered here appear constantly: the negative binomial in differential expression analysis, the beta in methylation modeling, the Poisson in mutation burden calculations, and the multivariate normal throughout dimensionality reduction and clustering. Understanding each distribution's assumptions and when they are violated is foundational to responsible computational biology.

```python
import numpy as np
import matplotlib.pyplot as plt
from scipy import stats

# Compare Poisson vs Negative Binomial for RNA-seq counts
# Simulate 10,000 gene expression values

mu = 50     # mean expression
r = 5       # dispersion (lower = more overdispersed)

poisson_samples = stats.poisson.rvs(mu, size=10000)
negbinom_samples = stats.nbinom.rvs(r, r/(r+mu), size=10000)

fig, axes = plt.subplots(1, 2, figsize=(12, 4))

axes[0].hist(poisson_samples, bins=50, density=True, alpha=0.7, label='Poisson')
axes[0].hist(negbinom_samples, bins=50, density=True, alpha=0.7, label='Neg. Binom.')
axes[0].set_xlabel('Count'); axes[0].set_ylabel('Density')
axes[0].set_title(f'RNA-seq count distribution (μ={mu})')
axes[0].legend()

# Mean-variance relationship
means = np.logspace(0, 3, 100)
poisson_var = means
negbinom_var = means + means**2 / r

axes[1].loglog(means, poisson_var, label='Poisson: Var = μ')
axes[1].loglog(means, negbinom_var, label=f'NB: Var = μ + μ²/{r}')
axes[1].set_xlabel('Mean'); axes[1].set_ylabel('Variance')
axes[1].set_title('Mean-Variance Relationship')
axes[1].legend()

print(f"Poisson: mean={poisson_samples.mean():.1f}, var={poisson_samples.var():.1f}")
print(f"NegBinom: mean={negbinom_samples.mean():.1f}, var={negbinom_samples.var():.1f}")
plt.tight_layout()
```
