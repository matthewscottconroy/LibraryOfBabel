# 23.2 The Shannon-McMillan-Breiman Theorem

The AEP for i.i.d. sources says: $-\frac{1}{n}\log p(X_1, \ldots, X_n) \to h$ in probability. The law of large numbers gives this immediately, since $-\log p(X_1, \ldots, X_n) = \sum_{k=1}^n (-\log p(X_k))$ and the summands are i.i.d.

For a general stationary ergodic source, we can't write the probability as a product. But we can use the chain rule to write the log-probability as a sum of conditional informations:
$$-\log \mu(\xi_0^{n-1}(x)) = \sum_{k=0}^{n-1} \left(-\log \mu(X_k | X_0, \ldots, X_{k-1})\right).$$

Now we're summing things that aren't i.i.d. — but for an ergodic system, the long-run average of any integrable function converges to its expectation. That's Birkhoff's theorem.

The Shannon-McMillan-Breiman theorem is the result of running Birkhoff's theorem on the information function.

**Theorem 23.2.1 (Shannon-McMillan-Breiman).** Let $(X, \mathcal{B}, \mu, f)$ be an ergodic MPT and $\xi = \{A_1, \ldots, A_k\}$ a finite generating partition. Let $\xi_0^{n-1}(x) = A_{i_0} \cap f^{-1}A_{i_1} \cap \cdots \cap f^{-(n-1)}A_{i_{n-1}}$ be the atom of $\bigvee_{k=0}^{n-1}f^{-k}\xi$ containing $x$. Then:
$$-\frac{1}{n}\log \mu(\xi_0^{n-1}(x)) \to h_\mu(f) \quad \mu\text{-a.e.}$$

In words: the information content of the $n$-step orbit-coding of $x$ grows like $n \cdot h_\mu(f)$, for almost every $x$. The concentration is almost-sure, not just in probability — this is the ergodic-theoretic strengthening.

**Proof Sketch:**
1. The information function of the $n$-step partition is $I_n(x) = -\log \mu(\xi_0^{n-1}(x))$.
2. By the chain rule: $I_n(x) = \sum_{k=0}^{n-1} I(x | f^{-k}\xi \vee \cdots \vee f^{-(k-1)}\xi)$ (sum of conditional informations).
3. The conditional information $i_k(x) = -\log \mu(A_{i_k}(f^k(x)) | \xi_{k+1}^{n-1}(f^k(x)))$ is "almost" a function of $f^k(x)$ for large $k$.
4. Apply Birkhoff's theorem to show $\frac{1}{n}I_n(x) \to E[i_0] = h_\mu(f, \xi) = h_\mu(f)$ a.e.

The critical step is step 3. For large $k$, conditioning on the long past changes the conditional information very little — the process is mixing enough that distant past observations contribute negligible additional information. Making this precise requires the martingale convergence theorem to handle the growing $\sigma$-algebra of the past. Once the conditional information is "approximately a function of the present," Birkhoff applies.

Shannon (1948) proved a weaker version (convergence in mean) for i.i.d. processes. McMillan (1953) proved $L^1$ convergence for stationary ergodic processes. Breiman (1957) strengthened this to almost-sure convergence, using the martingale convergence theorem — hence the name Shannon-McMillan-Breiman.

**The SMB Theorem as Ergodic AEP:**

The almost-sure convergence immediately implies the existence of a typical set — a concept that now works for any ergodic source.

**Definition 23.2.2.** The *ergodic typical set* at level $n$ and tolerance $\varepsilon$ is:
$$A_\varepsilon^{(n)} = \left\{x \in X : \left|-\frac{1}{n}\log\mu(\xi_0^{n-1}(x)) - h\right| < \varepsilon\right\}.$$

By SMB:
1. $\mu(A_\varepsilon^{(n)}) \to 1$ as $n \to \infty$ for all $\varepsilon > 0$
2. The number of typical atoms: $|A_\varepsilon^{(n)} \cap \mathcal{P}^n| \leq 2^{n(h+\varepsilon)}$ and $\geq (1-\delta)2^{n(h-\varepsilon)}$ for large $n$
3. Each typical atom has measure $\approx 2^{-nh}$

These three facts are exactly the AEP in its standard formulation — but now proved for all stationary ergodic sources, not just i.i.d. ones.

The geometric picture is the same as in Chapter 16, but now for general ergodic sources. Almost all of the probability mass concentrates on about $2^{nh}$ atoms, each with measure about $2^{-nh}$. Everything outside this typical set has negligible probability. The system is "essentially $k$-ary" with $k = 2^h$, even if the actual alphabet is much larger or the correlations are complex.

This is why data compression works for real sources — not just for i.i.d. models. The entropy rate $h$ is the fundamental limit, and the typical set is the set of sequences that actually appear.
