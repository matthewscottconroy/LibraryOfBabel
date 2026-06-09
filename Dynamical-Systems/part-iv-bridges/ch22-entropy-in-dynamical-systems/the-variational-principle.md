# 22.3 The Variational Principle

This is the main theorem of the chapter. It says that topological entropy and KS entropy — two quantities defined in completely different ways, one using orbit geometry and the other using invariant measures — are related by a supremum. The KS entropy of any invariant measure is at most the topological entropy, and the topological entropy is achieved (or approached) by some measure.

If you know Shannon's information theory, you've seen this structure before. The channel capacity is the maximum mutual information over all input distributions. The variational principle says: topological entropy is the channel capacity of the dynamical system, and KS entropy is the mutual information for a specific input measure.

**Theorem 22.3.1 (Variational Principle — Goodwyn 1969, Dinaburg 1970, Bowen 1971).** For a continuous map $f$ on a compact metric space:
$$h_{\text{top}}(f) = \sup_\mu h_\mu(f),$$
where the supremum is over all $f$-invariant Borel probability measures $\mu$.

The proof has two parts: the upper bound (every invariant measure has KS entropy at most $h_{\text{top}}$) and the lower bound (some measure achieves $h_{\text{top}}$, or comes arbitrarily close). Let's sketch both.

*(proof sketch)*

**Upper bound ($h_\mu \leq h_{\text{top}}$):** For any finite partition $\xi$ and invariant measure $\mu$, the information $H(\bigvee_{k=0}^{n-1} f^{-k}\xi)$ is bounded by $\log s_n(\varepsilon)$ for $\varepsilon$ = the minimum partition diameter. Taking the limit gives $h_\mu(f, \xi) \leq h_{\text{top}}(f)$.

The argument is: if two points $x, y$ land in the same atom of the $n$-step joined partition $\bigvee_{k=0}^{n-1} f^{-k}\xi$, then $d(f^k(x), f^k(y)) \leq \text{diam}(\xi)$ for all $k = 0, \ldots, n-1$. So points that are $(n, \varepsilon)$-separated must land in different atoms. This means the number of atoms of positive $\mu$-measure is at most $s_n(\varepsilon)$, and so $H(\bigvee_{k=0}^{n-1} f^{-k}\xi) \leq \log s_n(\varepsilon)$. Dividing by $n$ and taking limits gives the bound.

**Lower bound (achieved by some $\mu$):** For each $n$, take a maximal $(n,\varepsilon)$-separated set $E_n$ and put a uniform measure on it. The weak limit of the Cesàro averages of these measures is an invariant measure achieving entropy $\geq h_{\text{top}}$.

The Cesàro averaging is necessary to produce an *invariant* measure. The construction is: let $\mu_n = \frac{1}{|E_n|} \sum_{x \in E_n} \delta_x$ (uniform on separated set), then let $\nu_n = \frac{1}{n}\sum_{k=0}^{n-1} f^k_* \mu_n$ (time average). Any weak limit $\mu$ of $\{\nu_n\}$ is $f$-invariant, and a careful counting argument shows $h_\mu(f) \geq h_{\text{top}}(f) - \varepsilon$. Since this holds for all $\varepsilon$, we get $h_\mu(f) \geq h_{\text{top}}(f)$. Combined with the upper bound, $h_\mu(f) = h_{\text{top}}(f)$.

The variational principle has immediate consequences. It tells us that topological entropy is a *property of the map*, not of any particular measure — it's the best any invariant measure can do. The measure that achieves this best — the *measure of maximal entropy* — is special: it's the one that "sees" the most complexity in the system.

For the full $k$-shift, the measure of maximal entropy is the uniform Bernoulli measure (each symbol equally likely, independent). This makes perfect sense from the information-theoretic side: the capacity-achieving input distribution for a noiseless channel is the uniform distribution.

The variational principle also tells us something deep about the relationship between geometry and measure theory in dynamical systems. A purely topological quantity (the number of distinguishable orbits) is controlled by a measure-theoretic quantity (KS entropy). The bridge is made possible by the fact that separating sets — the geometric objects — can be used to construct invariant measures.
