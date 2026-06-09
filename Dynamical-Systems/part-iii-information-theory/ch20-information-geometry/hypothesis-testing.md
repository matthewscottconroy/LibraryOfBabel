# 20.7 Hypothesis Testing and Information Geometry

Hypothesis testing — deciding from data which of two distributions generated the data — has a clean information-geometric description. The error exponents are geodesic distances under the appropriate connections.

**Theorem 20.7.1 (Stein's Lemma / Sanov's Theorem).** Let $X_1, \ldots, X_n$ be i.i.d. from $P$. The optimal exponent for the type II error probability in testing $H_0: P = Q_0$ vs $H_1: P = Q_1$ (while keeping type I error $\leq \alpha$) is:
$$-\frac{1}{n}\log P_{\text{type II}} \to D_{\text{KL}}(Q_0 \| Q_1) \quad \text{as } n \to \infty.$$

In plain terms: if the true distribution is $Q_0$ and we fix the probability of incorrectly rejecting $H_0$ (type I error) at level $\alpha$, then the probability of incorrectly accepting $H_0$ when $H_1$ is true (type II error) decays exponentially at rate $D_{\text{KL}}(Q_0 \| Q_1)$.

Notice the asymmetry: it is $D_{\text{KL}}(Q_0 \| Q_1)$, not $D_{\text{KL}}(Q_1 \| Q_0)$. The two error types have different exponents, controlled by the two asymmetric KL divergences. Fixing type I error to be small makes type II error decay at rate $D_{\text{KL}}(Q_0 \| Q_1)$; fixing type II error makes type I error decay at $D_{\text{KL}}(Q_1 \| Q_0)$.

**Theorem 20.7.2 (Chernoff Information).** The Chernoff information $C^* = -\min_{0 \leq \lambda \leq 1} \log\sum_x p(x)^\lambda q(x)^{1-\lambda}$ is the optimal exponent for the minimum total error probability (minimizing the maximum of type I and type II errors) in testing $H_0: P$ vs $H_1: Q$.

The Chernoff information minimizes a $\lambda$-mixture divergence — which is exactly a Rényi divergence (Section 17.3) at the optimal $\lambda$. The optimal $\lambda$ is the one that balances the two error types.

Information geometry gives a unified geometric picture: $D_{\text{KL}}(P\|Q)$ is the "distance" from $P$ to $Q$ along the $m$-geodesic (mixture geodesic), and the Chernoff information is the "shortest $\lambda$-divergence" along the geodesic connecting $P$ and $Q$.

**Sanov's Theorem** generalizes this to testing whether the empirical distribution falls in a set $\Gamma$: the probability that the empirical distribution $\hat{p}_n \in \Gamma$ decays at rate $\min_{q \in \Gamma} D_{\text{KL}}(q \| p)$. The minimizing $q$ is the closest point in $\Gamma$ to $p$ in KL divergence — an $m$-projection problem. The Pythagorean theorem then gives the exact geometry of the error exponent as a function of which constraint set $\Gamma$ is being tested.

These connections make information geometry not just a formal framework but a computational tool: if you want to compute hypothesis testing error exponents, you solve a KL divergence minimization problem, which is a projection in the statistical manifold. The dually flat geometry of exponential families makes this particularly tractable — the projections have closed-form solutions in terms of the natural and mean parameters.
