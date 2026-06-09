# Probability Theory

In 2002, Michael Elowitz and colleagues published a landmark experiment: they placed two fluorescent reporter genes — one cyan, one yellow — under the control of identical promoters in *E. coli* cells, and imaged the cells one by one. The striking result was that genetically identical cells in the same environment showed wildly different ratios of cyan to yellow fluorescence. If the two reporters were identical, their expression should track each other perfectly in a deterministic world. They did not. Gene expression, it turned out, is fundamentally stochastic — individual mRNA molecules are produced in discrete bursts, degraded randomly, and translated into proteins by a noisy process. The noise is not experimental artifact; it is physics.

Biology is irreducibly stochastic. Gene expression bursts are random; mutations arise at random positions; cells make stochastic decisions about whether to divide, differentiate, or die. Probability theory provides the mathematical language for quantifying and reasoning about uncertainty — and it is essential for understanding the noise, variability, and randomness that are fundamental properties of living systems rather than experimental artifacts.

## Sample Spaces and Events

A **probability space** consists of three components:
- A **sample space** $\Omega$: the set of all possible outcomes (e.g., every possible sequence of 100 nucleotides)
- A **sigma-algebra** $\mathcal{F}$: a collection of subsets of $\Omega$ called events (e.g., "the sequence contains a start codon")
- A **probability measure** $P: \mathcal{F} \to [0,1]$ satisfying the Kolmogorov axioms

**Kolmogorov's axioms:**
1. $P(A) \geq 0$ for all events $A$
2. $P(\Omega) = 1$
3. For mutually exclusive events $A_1, A_2, \ldots$: $P\left(\bigcup_i A_i\right) = \sum_i P(A_i)$ (countable additivity)

All of probability theory follows from these three axioms. From them, one derives $P(\emptyset) = 0$, $P(A^c) = 1 - P(A)$, and the union rule $P(A \cup B) = P(A) + P(B) - P(A \cap B)$. The remarkable thing about this axiomatic foundation is that it makes no commitment about what probability *means* — it is equally consistent with a frequentist interpretation (long-run relative frequency), a Bayesian interpretation (degree of belief), and a physical interpretation (quantum mechanical probability). The mathematics works the same way regardless of your philosophical stance.

## Conditional Probability and Independence

**Conditional probability** is the probability of event $A$ given that event $B$ has occurred:

$$P(A | B) = \frac{P(A \cap B)}{P(B)}, \quad P(B) > 0$$

This is not just a formula — it represents the fundamental act of updating beliefs in light of evidence. In biology: given that a cell has divided (event $B$), what is the probability that it expresses a particular fate marker (event $A$)?

The **multiplication rule** follows directly: $P(A \cap B) = P(A|B) P(B) = P(B|A) P(A)$.

**Total probability theorem:** If $\{B_1, B_2, \ldots, B_n\}$ partitions $\Omega$:

$$P(A) = \sum_{i=1}^n P(A | B_i) P(B_i)$$

**Independence:** Events $A$ and $B$ are **independent** if $P(A \cap B) = P(A) P(B)$, equivalently $P(A|B) = P(A)$. Independence is an assumption that must be justified — in genomics, nearby SNPs are correlated due to linkage disequilibrium and are decidedly not independent. Many statistical methods assume independence and fail when it is violated; part of being a careful computational biologist is knowing which assumptions your tools are making.

## Bayes' Theorem

**Bayes' theorem** is derived by combining conditional probability and total probability:

$$P(B_i | A) = \frac{P(A | B_i) P(B_i)}{\sum_j P(A | B_j) P(B_j)}$$

In the language of probabilistic inference:
- $P(B_i)$: the **prior** probability of hypothesis $B_i$ before seeing data
- $P(A | B_i)$: the **likelihood** of observing data $A$ under hypothesis $B_i$
- $P(B_i | A)$: the **posterior** probability of $B_i$ after observing $A$

**Biological example:** Suppose a genetic variant is present in 1% of the population ($P(B) = 0.01$) and causes a disease with probability 0.8 given the variant ($P(A|B) = 0.8$). The disease also occurs in 5% of people without the variant ($P(A|B^c) = 0.05$). Given that a patient has the disease, what is the probability they carry the variant?

$$P(B|A) = \frac{0.8 \times 0.01}{0.8 \times 0.01 + 0.05 \times 0.99} = \frac{0.008}{0.008 + 0.0495} \approx 0.139$$

Even though the variant strongly predisposes to disease, only ~14% of disease patients carry it — because the variant is rare. This calculation, sometimes called the base rate neglect problem, is counterintuitive to almost everyone encountering it for the first time. It is a fundamental calculation in clinical genetics and GWAS interpretation, and it illustrates why the prior probability matters so much whenever you are asking whether a rare event is responsible for an observed outcome.

## Random Variables

A **random variable** $X$ is a function $X: \Omega \to \mathbb{R}$ that assigns a real number to each outcome. Random variables formalize biological measurements:
- $X$ = number of mRNA molecules per cell (discrete)
- $X$ = time until next transcriptional burst (continuous)
- $X$ = fluorescence intensity measurement (continuous)

The **cumulative distribution function (CDF)** is $F(x) = P(X \leq x)$.

For **discrete** random variables, the **probability mass function (PMF)** gives $p(x) = P(X = x)$.

For **continuous** random variables, the **probability density function (PDF)** $f(x)$ satisfies:

$$P(a \leq X \leq b) = \int_a^b f(x)\, dx, \quad \int_{-\infty}^{\infty} f(x)\, dx = 1$$

## Expectation, Variance, and Covariance

The **expected value** (mean) of $X$:

$$E[X] = \sum_x x \cdot p(x) \quad \text{(discrete)}, \qquad E[X] = \int_{-\infty}^{\infty} x f(x)\, dx \quad \text{(continuous)}$$

**Variance:** $\text{Var}(X) = E[(X - E[X])^2] = E[X^2] - (E[X])^2$. The square root is the standard deviation $\sigma = \sqrt{\text{Var}(X)}$.

**Covariance:** $\text{Cov}(X, Y) = E[(X - E[X])(Y - E[Y])] = E[XY] - E[X]E[Y]$.

**Correlation:** $\rho_{XY} = \text{Cov}(X,Y) / (\sigma_X \sigma_Y)$, bounded in $[-1, 1]$.

Covariance and correlation are fundamental in genomics: co-expression analysis, genetic correlation between traits, and quality control metrics all rely on these concepts. And yet correlation has a celebrated limitation: it only detects linear relationships. Two genes connected by a nonlinear regulatory relationship — one activating the other through a sigmoidal Hill function — may show substantial mutual information but near-zero Pearson correlation. This is why information-theoretic measures are complementary to correlation in network inference.

Key identities: $E[aX + b] = aE[X] + b$; $\text{Var}(aX) = a^2 \text{Var}(X)$; for independent $X, Y$: $\text{Var}(X + Y) = \text{Var}(X) + \text{Var}(Y)$.

## Why This Matters for Computational Biology

Probability theory is the language in which all uncertainty in biology is expressed. Sequencing a genome produces probabilistic base calls (Phred quality scores are $-10 \log_{10} P(\text{error})$). Calling variants requires computing $P(\text{variant} | \text{reads})$. Training a classifier to distinguish cell types requires modeling class-conditional probabilities. Understanding that intrinsic noise in gene expression follows a specific probability distribution — and knowing which distribution and why — is what separates a biologist who understands the data from one who just runs the pipeline.

```python
import numpy as np
from scipy import stats

# Bayes' theorem for variant interpretation
def posterior_prob(prior_variant, sensitivity, false_positive_rate):
    """P(variant | disease) given prior P(variant), P(disease|variant), P(disease|no variant)"""
    p_disease = sensitivity * prior_variant + false_positive_rate * (1 - prior_variant)
    posterior = (sensitivity * prior_variant) / p_disease
    return posterior

# Example from text
p_posterior = posterior_prob(0.01, 0.80, 0.05)
print(f"P(variant | disease) = {p_posterior:.3f}")

# Expected value and variance of a simple gene expression model
# mRNA count ~ Poisson(lambda) where lambda = production_rate / degradation_rate
lam = 10  # mean mRNA count
dist = stats.poisson(lam)
print(f"\nPoisson({lam}): mean = {dist.mean():.1f}, variance = {dist.var():.1f}")
# Note: mean == variance for Poisson -- a diagnostic for over-dispersion in real data!
```
