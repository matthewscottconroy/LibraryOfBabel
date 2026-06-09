# Statistical Inference

The 2002 paper that first genome-wide association study reported no significant findings — not because there were no genetic variants influencing the phenotype, but because the authors had not corrected for testing hundreds of thousands of markers simultaneously. At a naive threshold of $p < 0.05$, a GWAS would declare thousands of false positive associations by chance alone. The solution — testing at a threshold of $p < 5 \times 10^{-8}$, corresponding to a Bonferroni correction for roughly one million independent tests — became standard only after statisticians and biologists worked out the multiple testing mathematics together. The same problem is central to RNA-seq differential expression analysis, ChIP-seq peak calling, and virtually every high-throughput genomics method.

Statistical inference is the process of drawing conclusions about population parameters from sample data. In genomics, you rarely observe the "true" biological process — you observe noisy measurements from a limited number of cells, samples, or organisms. Inference provides the rigorous framework for separating signal from noise, quantifying uncertainty, and deciding what claims the data actually support.

## Maximum Likelihood Estimation

**Maximum likelihood estimation (MLE)** finds the parameter values that make the observed data most probable. Given data $\mathbf{x} = (x_1, \ldots, x_n)$ assumed i.i.d. from distribution $f(x; \theta)$, the **likelihood function** is:

$$\mathcal{L}(\theta; \mathbf{x}) = \prod_{i=1}^n f(x_i; \theta)$$

The MLE $\hat{\theta}$ maximizes $\mathcal{L}(\theta)$, or equivalently the **log-likelihood** $\ell(\theta) = \sum_i \log f(x_i; \theta)$ (log converts products to sums, which is more tractable).

**Worked example — estimating degradation rate from time-course data:** Suppose you measure protein concentration $[P](t_i)$ at time points $t_1, \ldots, t_n$, expecting exponential decay $[P](t) = A e^{-\delta t}$. The measurement model is $[P](t_i) = A e^{-\delta t_i} + \varepsilon_i$ where $\varepsilon_i \sim N(0, \sigma^2)$. The log-likelihood is:

$$\ell(A, \delta, \sigma) = -\frac{n}{2}\log(2\pi\sigma^2) - \frac{1}{2\sigma^2}\sum_i \left([P](t_i) - A e^{-\delta t_i}\right)^2$$

Maximizing this over $A$ and $\delta$ is equivalent to **nonlinear least squares** — minimizing the sum of squared residuals. This connection between MLE and least squares holds whenever the noise is Gaussian, and explains why least squares is such a natural fitting procedure.

## Method of Moments

The **method of moments (MOM)** equates theoretical moments (functions of parameters) to sample moments. For a distribution with mean $\mu(\theta)$ and variance $\sigma^2(\theta)$, solve:

$$\bar{x} = \mu(\hat{\theta}), \quad s^2 = \sigma^2(\hat{\theta})$$

MOM is often simpler than MLE and gives closed-form estimators. For the negative binomial distribution of RNA-seq counts: match sample mean $\bar{x}$ to $\mu$ and sample variance $s^2$ to $\mu + \mu^2/r$, giving $\hat{r} = \bar{x}^2/(s^2 - \bar{x})$ — the moment estimator of overdispersion. This is essentially how tools like DESeq2 get their initial dispersion estimates before the more sophisticated empirical Bayes shrinkage step.

## Confidence Intervals

A **95% confidence interval (CI)** for parameter $\theta$ is a random interval $[L(\mathbf{x}), U(\mathbf{x})]$ such that in repeated sampling, 95% of constructed intervals contain the true $\theta$:

$$P(L(\mathbf{x}) \leq \theta \leq U(\mathbf{x})) = 0.95$$

**Important:** The CI is not a probability statement about $\theta$ — $\theta$ is a fixed (unknown) constant in frequentist statistics. The probability refers to the random interval over hypothetical repeated experiments. This distinction is subtle but matters for interpretation: you cannot say "there is a 95% chance that the true parameter is in this interval." What you can say is that the procedure that generated this interval would, in repeated experiments, produce an interval containing the true value 95% of the time.

For a sample mean $\bar{X}$ with known $\sigma$: $\bar{X} \pm z_{\alpha/2} \cdot \sigma/\sqrt{n}$ where $z_{0.025} \approx 1.96$.

For unknown $\sigma$ (the usual case), use the $t$-distribution with $n-1$ degrees of freedom.

## Hypothesis Testing

**Hypothesis testing** provides a formal procedure for deciding whether data is inconsistent with a null hypothesis.

**Framework:**
1. State $H_0$ (null hypothesis): the default, "no effect" model (e.g., $\delta = 0$: gene is not differentially expressed)
2. State $H_1$ (alternative): what you suspect is true ($\delta \neq 0$)
3. Compute a **test statistic** $T$ that measures evidence against $H_0$
4. Compute the **p-value**: $p = P(T \geq t_{\text{obs}} | H_0)$ — the probability of seeing a test statistic at least as extreme as observed if $H_0$ is true
5. Reject $H_0$ if $p < \alpha$ (significance level, typically $\alpha = 0.05$)

**Type I error** ($\alpha$): rejecting $H_0$ when it is true (false positive)
**Type II error** ($\beta$): failing to reject $H_0$ when $H_1$ is true (false negative)
**Power** = $1 - \beta$: the probability of correctly detecting a true effect

Common tests:
- **$t$-test:** comparing means between two groups (gene expression between conditions)
- **$\chi^2$ test:** testing associations in contingency tables (variant × phenotype)
- **$F$-test / ANOVA:** comparing means across multiple groups
- **Wilcoxon rank-sum test:** non-parametric alternative to $t$-test, robust to non-normality

## Multiple Testing Correction

In genomics, you perform thousands of tests simultaneously — $\sim 20,000$ genes in a differential expression analysis, $\sim 10^7$ SNPs in a GWAS. At $\alpha = 0.05$, you expect $0.05 \times 20,000 = 1000$ false positives by chance alone. This is not a hypothetical concern — it is a real problem that produced years of irreproducible results in early candidate-gene association studies before the field adopted stringent correction standards.

**Bonferroni correction:** Set $\alpha^* = \alpha / m$ where $m$ is the number of tests. Controls the **family-wise error rate (FWER)**: the probability of any false positive. Very conservative — appropriate when any false positive is unacceptable.

**Benjamini-Hochberg (BH) procedure:** Controls the **false discovery rate (FDR)**: the expected fraction of rejected hypotheses that are false positives. Procedure:
1. Sort p-values $p_{(1)} \leq p_{(2)} \leq \cdots \leq p_{(m)}$
2. Find the largest $k$ such that $p_{(k)} \leq \frac{k}{m} \cdot q$ (FDR threshold $q$)
3. Reject $H_{(1)}, \ldots, H_{(k)}$

BH is standard in genomics because it adapts to the proportion of true nulls and is much more powerful than Bonferroni. A typical RNA-seq study uses $q = 0.05$, meaning at most 5% of "significant" genes are expected to be false discoveries. The key insight of the BH procedure is that it is not just asking "is this p-value small enough?" — it is asking "given how many tests we are running and how many we are calling significant, what fraction of those are likely to be false?"

## Why This Matters for Computational Biology

Every genomics result you will ever report relies on statistical inference. Differential expression analysis is MLE of the negative binomial model followed by a likelihood ratio test. Variant calling is Bayesian posterior computation. GWAS is millions of association tests with stringent multiple testing correction ($p < 5 \times 10^{-8}$ for GWAS genome-wide significance, a Bonferroni correction for $\sim 10^6$ independent tests). Peak calling in ChIP-seq and ATAC-seq requires modeling background noise and computing p-values. Understanding the statistical machinery — not just running the tools — allows you to interpret results correctly and recognize when assumptions are violated.

```python
import numpy as np
from scipy import stats

# Differential expression: t-test between two conditions
np.random.seed(42)
n_genes = 1000
n_samples_per_group = 5

# Simulate log-expression values
# 50 genes are truly differentially expressed (effect size = 1)
control = np.random.randn(n_genes, n_samples_per_group)
treatment = np.random.randn(n_genes, n_samples_per_group)
treatment[:50, :] += 1.5  # 50 truly DE genes

# Perform t-test for each gene
pvalues = np.array([stats.ttest_ind(control[i], treatment[i]).pvalue
                    for i in range(n_genes)])

# Multiple testing correction
alpha = 0.05
# Bonferroni
bonf_significant = np.sum(pvalues < alpha / n_genes)

# Benjamini-Hochberg (FDR)
from statsmodels.stats.multitest import multipletests
rejected_bh, pvals_adj, _, _ = multipletests(pvalues, alpha=0.05, method='fdr_bh')
bh_significant = np.sum(rejected_bh)

print(f"Uncorrected significant: {np.sum(pvalues < 0.05)}")
print(f"Bonferroni significant: {bonf_significant}")
print(f"BH FDR-corrected significant: {bh_significant}")
print(f"True positives in BH: {np.sum(rejected_bh[:50])}")
print(f"False positives in BH: {np.sum(rejected_bh[50:])}")
```
