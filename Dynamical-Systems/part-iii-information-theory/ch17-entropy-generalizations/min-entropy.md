# 17.2 Min-Entropy

Shannon entropy tells you, on average, how uncertain you are about $X$. But for cryptographic applications, "on average" is the wrong measure of security. An adversary does not get to average over many trials — they make one guess, and you need that guess to be wrong. The right quantity is *min-entropy*, the Rényi entropy at $\alpha = \infty$.

**Definition 17.2.1 (Min-Entropy).** The *min-entropy* of $X$ is:
$$H_\infty(X) = -\log \max_x p(x) = -\log \|p\|_\infty.$$

Min-entropy is the "worst-case" entropy: it measures the probability of the most likely outcome. A high min-entropy means even the most probable outcome is improbable — the source is "hard to guess." A low min-entropy means there is some outcome that dominates, and an adversary can succeed by always guessing that outcome.

To see why min-entropy is the right quantity, consider: an adversary who knows the distribution $p$ will always guess $\arg\max p(x)$. They succeed with probability $\max_x p(x) = 2^{-H_\infty(X)}$. So $H_\infty(X)$ is, literally, the negative log of the adversary's best success probability.

## 17.2.1 Conditional Min-Entropy

In practice, the adversary often has side information $Y$. The right quantity is then the *conditional min-entropy*, but this needs care: the naive definition would be $-\log \max_{x,y} p(x|y)$, which ignores the distribution of $Y$. The correct definition averages over $Y$:

**Definition 17.2.2 (Conditional Min-Entropy).** The *conditional min-entropy* of $X$ given $Y$ is:
$$H_\infty(X|Y) = -\log \sum_y p(y) \max_x p(x|y) = -\log \mathbb{E}_Y[\max_x p(X|Y)].$$

*Note: This is NOT $-\log \max_{x,y} p(x|y)$.* The correct definition uses an average over $Y$.

This definition captures the right security notion: an adversary who observes $Y$ will guess $\arg\max_x p(x|Y)$ for each value of $Y$, succeeding with probability $\max_x p(x|Y)$. The conditional min-entropy is the negative log of their *expected* success probability. High conditional min-entropy means the adversary cannot succeed reliably even with side information.

## 17.2.2 Smooth Min-Entropy

In one-shot information theory — where we analyze a single use of a protocol rather than its asymptotic behavior — the exact min-entropy can be too pessimistic. A distribution might have one outlier outcome with high probability that dominates $H_\infty$ but is itself negligible. The fix is to allow a small perturbation:

**Definition 17.2.3 (Smooth Min-Entropy).** The *$\varepsilon$-smooth min-entropy* is:
$$H_\infty^\varepsilon(X) = \max_{\tilde{p}: \|p - \tilde{p}\|_1 \leq \varepsilon} H_\infty(\tilde{p}).$$

We maximize the min-entropy over all distributions $\tilde{p}$ within $\varepsilon$ (in total variation) of the true distribution $p$. The smooth min-entropy is the min-entropy of the "best nearby distribution" — essentially, we allow ourselves to ignore a set of probability $\varepsilon$ and compute min-entropy on the rest.

Smooth min-entropy is the correct quantity for *privacy amplification* — the process of extracting nearly-uniform random bits from a partially random source.

**Theorem 17.2.4 (Operational Meaning — Privacy Amplification).** Given $n$ uses of a source with smooth min-entropy $H_\infty^\varepsilon(X^n)$, one can extract $k \leq H_\infty^\varepsilon(X^n) - 2\log(1/\varepsilon)$ bits that are statistically close to uniform, using a 2-universal hash function.

In other words: take any source whose smooth min-entropy is $k + 2\log(1/\varepsilon)$, apply a random hash function, and you get $k$ bits that no computationally unbounded adversary can predict better than chance (up to error $\varepsilon$). The hash function does not need to know the source distribution — the theorem holds universally.

This is the foundation of quantum key distribution security proofs, randomness extraction, and many protocols in cryptography. The smooth min-entropy framework (developed by Renner in his 2005 PhD thesis) essentially created the field of one-shot information theory — the analysis of protocols from a single sample rather than many.
