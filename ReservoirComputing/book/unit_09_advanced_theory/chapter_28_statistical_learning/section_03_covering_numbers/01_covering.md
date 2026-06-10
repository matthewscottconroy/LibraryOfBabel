# Covering Numbers, Metric Entropy, and Reservoir Function Classes

## 28.3.1 The Metric Geometry of Hypothesis Classes

Rademacher complexity provides distribution-dependent generalization bounds, but computing it requires knowing the distribution of reservoir states. A complementary approach based on **covering numbers** yields bounds that depend on the metric geometry of the function class — and connect generalization directly to the spectral properties of the reservoir weight matrix.

The core idea is simple: if two functions $f, g \in \mathcal{F}$ are close everywhere (in some norm), then the generalization behavior of $f$ can be inferred from that of $g$. If we can cover $\mathcal{F}$ with a small number of balls of radius $\varepsilon$, then we can effectively replace $\mathcal{F}$ with a finite class of size $N(\mathcal{F}, \varepsilon, \|\cdot\|)$ and apply finite-class PAC bounds.

## 28.3.2 Covering Numbers

**Definition 28.6 ($\varepsilon$-Cover).** Given a metric space $(\mathcal{F}, d)$ and $\varepsilon > 0$, an **$\varepsilon$-cover** is a set $\mathcal{C} \subseteq \mathcal{F}$ such that for every $f \in \mathcal{F}$, there exists $g \in \mathcal{C}$ with $d(f, g) \leq \varepsilon$.

**Definition 28.7 (Covering Number).** The **covering number** $\mathcal{N}(\mathcal{F}, \varepsilon, d)$ is the minimum cardinality of any $\varepsilon$-cover of $\mathcal{F}$ in metric $d$.

The dual notion is the **packing number** $\mathcal{M}(\mathcal{F}, \varepsilon, d)$: the maximum number of points in $\mathcal{F}$ with pairwise distance $> \varepsilon$. These satisfy the sandwich bound:

$$
\mathcal{M}(\mathcal{F}, 2\varepsilon, d) \leq \mathcal{N}(\mathcal{F}, \varepsilon, d) \leq \mathcal{M}(\mathcal{F}, \varepsilon, d).
$$

**Definition 28.8 (Metric Entropy).** The **metric entropy** of $\mathcal{F}$ at scale $\varepsilon$ is $\log \mathcal{N}(\mathcal{F}, \varepsilon, d)$.

For the purposes of generalization bounds, we typically use the $L_\infty$ covering number over the sample:

$$
\mathcal{N}_\infty(\mathcal{F}, \varepsilon, S) = \mathcal{N}\!\left(\mathcal{F}, \varepsilon, \sup_{x \in S}|f(x) - g(x)|\right).
$$

## 28.3.3 Generalization Bound via Covering Numbers

The following bound was established by [Haussler 1992] and refined by [Bartlett 1998]:

**Theorem 28.5 (Covering Number Generalization Bound).** For a function class $\mathcal{F}$ with range $[0, 1]$, for any $\varepsilon > 0$ and $m$ i.i.d. training examples, with probability at least $1 - \delta$:

$$
\sup_{f \in \mathcal{F}}\left[\mathcal{L}_{\mathcal{D}}(f) - \hat{\mathcal{L}}_S(f)\right] \leq \varepsilon + \sqrt{\frac{2\log \mathcal{N}(\mathcal{F}, \varepsilon/2, \|\cdot\|_\infty) + 2\log(1/\delta)}{m}}.
$$

Optimizing over $\varepsilon$ yields a bound that depends on the metric entropy integral, connecting to Dudley's result below.

## 28.3.4 Dudley's Integral Bound

The most powerful connection between metric entropy and complexity is Dudley's integral bound [Dudley 1967, 1978]:

**Theorem 28.6 (Dudley's Entropy Integral).** The Rademacher complexity of $\mathcal{F}$ satisfies

$$
\mathcal{R}_m(\mathcal{F}) \leq \frac{12}{\sqrt{m}} \int_0^{\mathrm{diam}(\mathcal{F})/2} \sqrt{\log \mathcal{N}(\mathcal{F}, \varepsilon, L_2)} \, d\varepsilon,
$$

where the integral is taken over the $L_2(\mathcal{D})$ metric on functions.

*Proof idea.* The proof proceeds via a chaining argument [Talagrand 2014]. Define a sequence of $\varepsilon_j$-nets with $\varepsilon_j = 2^{-j}\,\mathrm{diam}(\mathcal{F})$; each $f \in \mathcal{F}$ is approximated by its nearest point in the net at each resolution. The supremum of the Rademacher process over $\mathcal{F}$ is bounded by a telescoping sum of Gaussian widths, each controlled by the metric entropy at scale $\varepsilon_j$. Summing the series gives the integral. $\square$

The integral $\int_0^\infty \sqrt{\log \mathcal{N}(\mathcal{F}, \varepsilon)} \, d\varepsilon$ is sometimes called the **Dudley integral** or **entropy integral** of $\mathcal{F}$.

## 28.3.5 Covering Numbers for Reservoir Function Classes

The reservoir function class is determined by the geometry of the set of reservoir states $\mathcal{X}_N = \{\mathbf{x}(u) : u \in \mathcal{U}\} \subseteq \mathbb{R}^N$. For the linear readout class $\mathcal{F}_B = \{\mathbf{x} \mapsto \mathbf{w}^T \mathbf{x} : \|\mathbf{w}\|_2 \leq B\}$:

$$
\mathcal{N}(\mathcal{F}_B, \varepsilon, L_\infty(\mathcal{X}_N)) = \mathcal{N}\!\left(\{\mathbf{w} : \|\mathbf{w}\|_2 \leq B\},\; \frac{\varepsilon}{R},\; \|\cdot\|_2\right),
$$

where $R = \sup_{\mathbf{x} \in \mathcal{X}_N} \|\mathbf{x}\|_2$. For the $\ell^2$ ball of radius $B$ in $\mathbb{R}^N$:

$$
\log \mathcal{N}\!\left(B\mathbb{B}_2^N, \varepsilon, \|\cdot\|_2\right) \leq N \log\!\left(\frac{2B}{\varepsilon} + 1\right) \leq N \log\!\frac{3B}{\varepsilon}. \tag{28.1}
$$

This is a classical volumetric estimate [Pisier 1989].

Substituting (28.1) into Dudley's bound:

$$
\mathcal{R}_m(\mathcal{F}_B) \leq \frac{12}{\sqrt{m}} \int_0^{BR} \sqrt{N \log\!\frac{3BR}{\varepsilon}} \, d\varepsilon = \frac{12BR\sqrt{N}}{\sqrt{m}} \int_0^1 \sqrt{\log\frac{3}{\varepsilon}} \, d\varepsilon = O\!\left(\frac{BR\sqrt{N}}{\sqrt{m}}\right). \tag{28.2}
$$

This recovers the $\sqrt{N}$ dependence predicted by the VC bound, but with the additional prefactor $R = \sup\|\mathbf{x}\|_2$.

## 28.3.6 Spectral Control of Metric Entropy

The bound (28.2) is still $O(\sqrt{N})$. However, if the reservoir states are not spread throughout $\mathbb{R}^N$ but concentrated near a lower-dimensional subspace, the covering number is dramatically smaller.

Suppose the empirical covariance $\Sigma = \frac{1}{T}\sum_t \mathbf{x}(t)\mathbf{x}(t)^T$ has eigenvalues $\lambda_1 \geq \lambda_2 \geq \cdots \geq \lambda_N \geq 0$. The effective dimension is the **stable rank**:

$$
r(\Sigma) = \frac{\mathrm{tr}(\Sigma)}{\|\Sigma\|_{\mathrm{op}}} = \frac{\sum_i \lambda_i}{\lambda_1}.
$$

**Theorem 28.7 (Spectral Covering Bound).** If the reservoir states lie within an ellipsoid defined by $\Sigma$, the covering number of the readout function class satisfies

$$
\log \mathcal{N}(\mathcal{F}_B, \varepsilon, L_2) \leq r(\Sigma) \log\!\frac{2B\|\Sigma\|_{\mathrm{op}}^{1/2}}{\varepsilon},
$$

and Dudley's bound gives $\mathcal{R}_m(\mathcal{F}_B) = O(B\sqrt{r(\Sigma)/m})$.

*Proof sketch.* Transform to the eigenbasis of $\Sigma$. In the transformed space, the function class is a product of intervals of width $\sqrt{\lambda_i}$. The covering number is a product of covering numbers of each interval, and the log-covering number sums to $\sum_i \log(2B\sqrt{\lambda_i}/\varepsilon)$. Bounding this sum by $r(\Sigma)\log(2B\|\Sigma\|_{\mathrm{op}}^{1/2}/\varepsilon)$ gives the result [Zhou 2002]. $\square$

**Corollary 28.8.** If the reservoir dynamics compress the input signal into an approximately $d$-dimensional subspace ($r(\Sigma) \approx d \ll N$), then the generalization complexity is that of a $d$-dimensional model, not an $N$-dimensional one.

This result is crucial for understanding why large reservoirs can generalize well: if the task requires only $d \ll N$ effective dimensions, the extra neurons do not increase the generalization complexity.

## 28.3.7 Practical Estimation

In practice, the stable rank $r(\Sigma)$ can be estimated from the training state matrix $\mathbf{X} \in \mathbb{R}^{T \times N}$ via its singular values $\sigma_1 \geq \sigma_2 \geq \cdots$:

$$
r(\mathbf{X}/\sqrt{T}) = \frac{\sum_i \sigma_i^2}{\sigma_1^2}.
$$

A fast decay of singular values indicates that the reservoir compresses the input into a low-dimensional representation, and the generalization bound is correspondingly tight. Slow decay (all singular values comparable) indicates that the reservoir uses all $N$ dimensions, and the bound approaches the $O(\sqrt{N})$ worst case.

## References

- Bartlett, P. L. (1998). The sample complexity of pattern classification with neural networks: the size of the weights is more important than the size of the network. *IEEE Transactions on Information Theory*, 44(2), 525–536.
- Bartlett, P. L. (2020). Benign overfitting in linear regression. In *Advances in Neural Information Processing Systems*. (Survey version.)
- Dudley, R. M. (1967). The sizes of compact subsets of Hilbert space and continuity of Gaussian processes. *Journal of Functional Analysis*, 1(3), 290–330.
- Dudley, R. M. (1978). Central limit theorems for empirical measures. *The Annals of Probability*, 6(6), 899–929.
- Haussler, D. (1992). Decision-theoretic generalizations of the PAC model for neural net and other learning applications. *Information and Computation*, 100(1), 78–150.
- Pisier, G. (1989). *The Volume of Convex Bodies and Banach Space Geometry*. Cambridge University Press.
- Talagrand, M. (2014). *Upper and Lower Bounds for Stochastic Processes*. Springer.
- Zhou, D.-X. (2002). The covering number in learning theory. *Journal of Complexity*, 18(3), 739–767.
