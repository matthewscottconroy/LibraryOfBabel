# Entropy and Mutual Information

In 1948, a 32-year-old mathematician at Bell Labs named Claude Shannon published a paper that would eventually be recognized as the founding document of the information age. Shannon was trying to solve a practical engineering problem: given a noisy communication channel, how much information can you reliably transmit? His solution required a new concept — a way to quantify information as a mathematical quantity. He defined it in terms of the probability of messages, and called the resulting measure "entropy," borrowing the word from thermodynamics. The choice of name was not accidental: the mathematical form was identical to the Boltzmann-Gibbs entropy of statistical mechanics.

What Shannon could not have anticipated is how profoundly his framework would illuminate biology. DNA is a communication channel — from genotype to phenotype. A transcription factor binding to DNA is decoding a message written in sequence. A signaling pathway transmits information from the cell surface to the nucleus. A genome contains information about how to build an organism. Information theory, originally invented for telephone engineers, turns out to be one of the most natural languages for quantitative molecular biology.

## Shannon Entropy

The **Shannon entropy** of a discrete random variable $X$ with probability mass function $p(x)$ is:

$$H(X) = -\sum_{x \in \mathcal{X}} p(x) \log_2 p(x)$$

Entropy is measured in **bits** (when using $\log_2$) or **nats** (when using $\ln$). It quantifies the average uncertainty or information content of $X$.

**Key properties:**
- $H(X) \geq 0$, with equality iff $X$ is deterministic (one outcome has probability 1)
- $H(X)$ is maximized when $X$ is uniform: $H_{\max} = \log_2 |\mathcal{X}|$ bits
- $H(X)$ is concave in $p$ — averaging distributions increases entropy

**Biological example — nucleotide position entropy:** Consider a column in a multiple sequence alignment. If all sequences have G at that position: $H = -1 \cdot \log_2 1 = 0$ bits — completely conserved. If A, C, G, T appear with equal frequency: $H = -4 \cdot \frac{1}{4} \log_2 \frac{1}{4} = 2$ bits — completely random. The **information content** (conservation) at position $i$ is $R_i = \log_2 4 - H_i = 2 - H_i$ bits — the basis of sequence logos.

**Sequence logos** visualize transcription factor binding sites as stacks of letters where the total height at each position equals the information content in bits, and the height of individual letters is proportional to their frequency. A perfect consensus position has 2 bits; a highly degenerate position has near 0 bits. The logo is a visual representation of which nucleotides are informationally constrained and which are free to vary — a direct window into the binding specificity of the factor.

## Joint Entropy and Conditional Entropy

For two random variables $X$ and $Y$:

**Joint entropy:** $H(X, Y) = -\sum_{x,y} p(x,y) \log_2 p(x,y)$

**Conditional entropy:** $H(Y|X) = -\sum_{x,y} p(x,y) \log_2 p(y|x)$ — the remaining uncertainty about $Y$ after observing $X$.

**Chain rule:** $H(X, Y) = H(X) + H(Y|X) = H(Y) + H(X|Y)$

If $X$ and $Y$ are independent: $H(X, Y) = H(X) + H(Y)$ — joint entropy equals the sum of marginal entropies. Dependence reduces joint entropy. This is intuitive: knowing that two genes are co-regulated reduces your uncertainty about the state of either one.

## Mutual Information

**Mutual information (MI)** measures how much information two variables share — the reduction in uncertainty about $Y$ from knowing $X$:

$$I(X; Y) = H(X) + H(Y) - H(X, Y) = H(X) - H(X|Y) = H(Y) - H(Y|X)$$

Equivalently:

$$I(X; Y) = \sum_{x,y} p(x,y) \log_2 \frac{p(x,y)}{p(x)p(y)}$$

**Properties:**
- $I(X; Y) \geq 0$, with equality iff $X$ and $Y$ are independent
- $I(X; Y) = I(Y; X)$ — mutual information is symmetric
- $I(X; Y) \leq \min(H(X), H(Y))$
- $I(X; X) = H(X)$ — self-information equals entropy

MI captures any statistical dependence, not just linear correlation. Two genes whose expressions are related by a nonlinear transformation will have zero Pearson correlation but positive mutual information. This property — detecting arbitrary dependencies, not just linear ones — is what makes MI a particularly powerful tool for biological network inference, where regulatory relationships are almost never linear.

## Kullback-Leibler Divergence

The **KL divergence** (relative entropy) from distribution $Q$ to distribution $P$ is:

$$D_{\text{KL}}(P \| Q) = \sum_x p(x) \log_2 \frac{p(x)}{q(x)}$$

$D_{\text{KL}}(P\|Q) \geq 0$ (Gibbs inequality), with equality iff $P = Q$. It is **not** symmetric: $D_{\text{KL}}(P\|Q) \neq D_{\text{KL}}(Q\|P)$ in general.

KL divergence measures how much information is lost when approximating $P$ with $Q$. It appears naturally in:
- Bayesian model comparison: $D_{\text{KL}}(\text{posterior} \| \text{prior})$ measures how much the data has moved you from your prior beliefs
- Variational inference: approximating an intractable posterior $P(\theta|\mathbf{x})$ with a tractable family $Q(\theta)$ by minimizing $D_{\text{KL}}(Q \| P)$
- Model evaluation: comparing a model's predicted distribution to the observed data distribution

The **Jensen-Shannon divergence** $JSD(P\|Q) = \frac{1}{2} D_{\text{KL}}(P\|M) + \frac{1}{2} D_{\text{KL}}(Q\|M)$ where $M = (P+Q)/2$ is symmetric and bounded in $[0, 1]$ — often preferred for comparing biological distributions.

## Why This Matters for Computational Biology

Information theory quantifies what data "says" about biology in a rigorous, model-free way. Mutual information is used in ARACNE and other network inference algorithms to identify co-regulated gene pairs without assuming a linear relationship. The information content of binding sites quantifies transcription factor specificity — a TF with 12 bits of information content recognizes a 6-bp specific sequence ($2^{12/2} = 64$ sequences out of $4^6 = 4096$ possible). KL divergence measures evolutionary divergence between populations, the informativeness of a prior, and the quality of an approximate inference algorithm. Entropy of codon usage quantifies how skewed a genome's codon preferences are, with implications for recombinant protein expression. These tools appear wherever quantitative reasoning about biological information is needed.

```python
import numpy as np
from scipy.stats import entropy as scipy_entropy

def shannon_entropy(p, base=2):
    """Shannon entropy in bits (or nats if base=np.e)."""
    p = np.array(p)
    p = p[p > 0]  # remove zeros (0 * log 0 = 0 by convention)
    return -np.sum(p * np.log(p) / np.log(base))

def mutual_information(joint_prob):
    """Compute MI from a joint probability table."""
    p_x = joint_prob.sum(axis=1)
    p_y = joint_prob.sum(axis=0)
    H_X = shannon_entropy(p_x)
    H_Y = shannon_entropy(p_y)
    H_XY = shannon_entropy(joint_prob.flatten())
    return H_X + H_Y - H_XY

# Example: nucleotide position entropies (sequence logo)
positions = [
    [0.97, 0.01, 0.01, 0.01],  # highly conserved G
    [0.60, 0.10, 0.20, 0.10],  # moderately conserved
    [0.25, 0.25, 0.25, 0.25],  # completely degenerate
]

print("Position\tEntropy (bits)\tInfo content (bits)")
for i, freq in enumerate(positions):
    H = shannon_entropy(freq)
    IC = 2.0 - H  # max is log2(4) = 2 bits for DNA
    print(f"  {i+1}\t\t{H:.3f}\t\t{IC:.3f}")

# Mutual information between two co-expressed genes
# Joint distribution of (gene1 on/off, gene2 on/off)
joint = np.array([[0.50, 0.05],   # gene1=off: gene2=off, gene2=on
                  [0.05, 0.40]])  # gene1=on:  gene2=off, gene2=on
joint = joint / joint.sum()

MI = mutual_information(joint)
print(f"\nMutual information between co-expressed genes: {MI:.3f} bits")
print(f"(0 = independent, max ~1 = perfectly correlated)")

# KL divergence: TF binding site vs random
binding_site_freq = np.array([0.05, 0.80, 0.10, 0.05])  # consensus C
background_freq = np.array([0.25, 0.25, 0.25, 0.25])     # uniform background

kl_div = np.sum(binding_site_freq * np.log2(binding_site_freq / background_freq))
print(f"\nKL divergence (binding site || background): {kl_div:.3f} bits")
```
