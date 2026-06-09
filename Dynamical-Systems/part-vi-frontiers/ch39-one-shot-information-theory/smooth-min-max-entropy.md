# 39.2 Smooth Min- and Max-Entropy

## 39.2.1 Min-Entropy and Max-Entropy

Before smoothing, let's understand the two extreme entropies. Min-entropy measures worst-case randomness — how concentrated the distribution is on its most probable element. Max-entropy measures the support — how many elements are possible at all.

**Definition 39.2.1.** For a probability distribution $P$ on $\mathcal{X}$:
- *Min-entropy*: $H_\infty(X) = -\log \max_x P(x)$ (measures the "worst-case" randomness)
- *Max-entropy*: $H_0(X) = \log |\text{supp}(P)|$ (measures the "support size")

For quantum states $\rho$:
- $H_{\min}(\rho) = -\log \lambda_{\max}(\rho)$ (negative log of largest eigenvalue)
- $H_{\max}(\rho) = \log \text{rank}(\rho)$ (log of rank)

Min-entropy is the Rényi entropy $H_\infty$ — the limit as $\alpha \to \infty$. Max-entropy is the Rényi entropy $H_0$ — the limit as $\alpha \to 0$. Shannon entropy sits between them: $H_\infty \leq H \leq H_0$.

**Remark 39.2.2.** For i.i.d. sources $X^n = (X_1, \ldots, X_n)$: by LLN, $H_\infty(X^n) \approx nH(X)$ for large $n$. The asymptotic $H_\infty/n \to H(X)$ recovers Shannon entropy.

In the i.i.d. limit, min-entropy, max-entropy, and Shannon entropy all coincide (per symbol). The Asymptotic Equipartition Property (AEP) from Chapter 16 is the statement that the "typical set" dominates, so min-entropy per symbol $\to H$ and max-entropy per symbol $\to H$.

## 39.2.2 Smooth Entropy

For finite resources, we need to allow a small error $\varepsilon$ — we're allowed to "give up" on a small fraction of probability mass. This is the smoothing.

**Definition 39.2.3 (Renner, 2005).** The *$\varepsilon$-smooth min-entropy* of $\rho$ is:
$$H_{\min}^\varepsilon(\rho) = \max_{\tilde\rho: \|\tilde\rho - \rho\|_1 \leq \varepsilon} H_{\min}(\tilde\rho).$$

The *$\varepsilon$-smooth max-entropy* is:
$$H_{\max}^\varepsilon(\rho) = \min_{\tilde\rho: \|\tilde\rho - \rho\|_1 \leq \varepsilon} H_{\max}(\tilde\rho).$$

The smoothing allows us to "cheat" on an $\varepsilon$-fraction of probability mass.

The smooth min-entropy $H_{\min}^\varepsilon(\rho)$ is the highest min-entropy you can achieve by slightly modifying $\rho$ (within trace distance $\varepsilon$). You're allowed to redistribute up to $\varepsilon$ probability mass to make the distribution less concentrated. This gives a more optimistic bound on extractable randomness.

**Theorem 39.2.4 (AEP for Smooth Entropy).** For i.i.d. sources $\rho^{\otimes n}$ as $n \to \infty$:
$$\frac{1}{n}H_{\min}^{\varepsilon}(\rho^{\otimes n}) \to S(\rho), \quad \frac{1}{n}H_{\max}^{\varepsilon}(\rho^{\otimes n}) \to S(\rho)$$
for any fixed $\varepsilon > 0$, where $S(\rho)$ is the von Neumann entropy. The smooth entropies collapse to $S(\rho)$ in the i.i.d. limit.

This is the quantum AEP for smooth entropy. For large $n$, everything collapses to the von Neumann entropy. For finite $n$, the smooth entropies differ from $S(\rho)$ by $O(\sqrt{n})$ correction terms that encode finite-blocklength effects.
