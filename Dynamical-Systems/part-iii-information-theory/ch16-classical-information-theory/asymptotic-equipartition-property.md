# 16.2 The Asymptotic Equipartition Property

## 16.2.1 Typical Sequences

Here is a striking fact about probability that took the entire field of information theory to properly formalize. If you flip a fair coin 1000 times, you almost certainly get a sequence with roughly 500 heads and 500 tails. Not exactly, but close — the probability of being very far from 500-500 is astronomically small. The vast majority of the $2^{1000}$ possible sequences are "typical" in this sense, and you can almost completely ignore the rest.

This is the law of large numbers in action. But Shannon's genius was to ask what this means for *information*: if most sequences look roughly alike, maybe you don't need to distinguish all $2^{1000}$ of them. Maybe you only need to deal with the typical ones.

The *asymptotic equipartition property* (AEP) makes this precise. For an i.i.d. source, the empirical probability of a long sequence concentrates sharply around $2^{-nH}$, where $H$ is the entropy. This is the law of large numbers applied to the random variable $-\log p(X)$.

**Theorem 16.2.1 (Weak AEP — Shannon 1948).** Let $X_1, X_2, \ldots$ be i.i.d. with distribution $p(x)$ and entropy $H = H(X_1)$. Then:
$$-\frac{1}{n}\log p(X_1, \ldots, X_n) \to H \quad \text{in probability.}$$

*Proof:* The key observation is that for an i.i.d. sequence:
$$-\frac{1}{n}\log p(X_1, \ldots, X_n) = \frac{1}{n}\sum_{k=1}^n (-\log p(X_k)).$$
By the Weak Law of Large Numbers, this converges in probability to $E[-\log p(X)] = H$. $\square$

This is elegant in its simplicity, but what does it really mean? It means that, with high probability, the sequence $(X_1, \ldots, X_n)$ has probability close to $2^{-nH}$. Not $2^{-nH}$ exactly — the actual probability varies — but within a factor of $2^{n\varepsilon}$ of $2^{-nH}$ for any $\varepsilon > 0$ and large enough $n$.

This motivates the definition of the typical set:

**Definition 16.2.2 (Typical Set).** The *$\varepsilon$-typical set* is:
$$A_\varepsilon^{(n)} = \left\{(x_1,\ldots,x_n) \in \mathcal{X}^n : \left|-\frac{1}{n}\log p(x_1,\ldots,x_n) - H\right| < \varepsilon\right\}.$$

The typical set contains sequences whose "empirical entropy" is within $\varepsilon$ of the true entropy $H$. It turns out to have three remarkable properties:

**Theorem 16.2.3 (Properties of the Typical Set).**
1. *High probability*: $P((X_1,\ldots,X_n) \in A_\varepsilon^{(n)}) \geq 1 - \delta$ for $n$ large enough (depending on $\varepsilon, \delta$).
2. *Not too large*: $|A_\varepsilon^{(n)}| \leq 2^{n(H+\varepsilon)}$ (at most exponentially many typical sequences).
3. *Not too small*: $|A_\varepsilon^{(n)}| \geq (1-\delta)2^{n(H-\varepsilon)}$ for large $n$.

Let's unpack what these three properties tell us together. Of the $|\mathcal{X}|^n$ possible sequences of length $n$, almost all the probability mass (at least $1 - \delta$) sits on a set of size at most $2^{n(H+\varepsilon)}$. Since $H \leq \log|\mathcal{X}|$, this is dramatically smaller than the full space when the source has entropy below maximum. And each sequence in the typical set has probability roughly $2^{-nH}$ — they are nearly "equiprobable," which is where the name comes from.

**Interpretation:** The typical set has $\approx 2^{nH}$ members. The non-typical sequences are collectively unlikely. This is the *data compression principle*: we only need to encode typical sequences, which requires $nH$ bits — exactly the entropy.

This single insight — that only about $2^{nH}$ sequences matter — is the foundation of data compression. We'll build on it in the next section to prove that entropy is not just an interesting quantity but an exact, achievable compression limit.

The AEP also connects to the dynamical systems perspective that motivates this book. For an ergodic dynamical system (say, a measure-preserving transformation), the Shannon-McMillan-Breiman theorem (the ergodic generalization of the AEP) says that the typical sequences are exactly those that arise from "generic" orbits — orbits that explore the space according to the invariant measure. The entropy $H$ is then the *metric entropy* of the system, and the typical set is the set of sequences you actually see when you watch the system evolve. We will revisit this connection extensively in Part IV.
