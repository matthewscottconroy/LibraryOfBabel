# Section 27.3: Concentration Inequalities

## 27.3.1 Why Concentration?

The Wigner and Marchenko-Pastur laws describe limiting behavior as $N \to \infty$. For finite reservoirs, we need quantitative bounds on deviations from the limit. Concentration inequalities provide exactly this: they say "with high probability, the empirical quantity is within $\varepsilon$ of its expectation, with probability at least $1 - 2e^{-c\varepsilon^2 N}$."

These bounds have three uses in reservoir computing:
1. They quantify how much the empirical spectral distribution of a random reservoir deviates from the semicircle law at finite $N$.
2. They bound the deviation of the empirical capacity $\hat{C}_L$ from its expectation $C_L$, given finite training data.
3. They underpin the generalization bounds of Chapter 26 (Section 26.5.5).

We develop the inequalities in increasing order of sophistication: Hoeffding (scalar, bounded), Bernstein (scalar, sub-exponential), and matrix Bernstein (matrix-valued, bounded).

## 27.3.2 Scalar Concentration: Hoeffding's Inequality

**Theorem 27.3.1 (Hoeffding's Inequality).** *Let $X_1, \ldots, X_n$ be independent random variables with $a_i \leq X_i \leq b_i$ almost surely. Let $S = \sum_{i=1}^n X_i$ and $\mu = \mathbb{E}[S]$. For any $t > 0$:*
$$\mathbb{P}(S - \mu \geq t) \leq \exp\!\left(-\frac{2t^2}{\sum_{i=1}^n (b_i - a_i)^2}\right).$$

**Proof.** We use the *Chernoff method*: for any $s > 0$,
$$\mathbb{P}(S - \mu \geq t) \leq e^{-st} \mathbb{E}[e^{s(S-\mu)}] = e^{-st} \prod_{i=1}^n \mathbb{E}[e^{s(X_i - \mathbb{E}[X_i])}].$$

For each term, we use the *Hoeffding lemma*: if $Y$ is a random variable with $\mathbb{E}[Y] = 0$ and $a \leq Y \leq b$, then
$$\mathbb{E}[e^{sY}] \leq \exp\!\left(\frac{s^2(b-a)^2}{8}\right).$$

**Proof of Hoeffding Lemma.** By convexity of $e^{sx}$ on $[a,b]$:
$$e^{sx} \leq \frac{b-x}{b-a} e^{sa} + \frac{x-a}{b-a} e^{sb}.$$
Taking expectation with $\mathbb{E}[Y] = 0$:
$$\mathbb{E}[e^{sY}] \leq \frac{-a}{b-a} e^{sa} + \frac{b}{b-a} e^{sb} \cdot \frac{-a}{b} = \frac{b}{b-a} e^{sa} \cdot (-a/b) + \ldots$$

More cleanly: let $p = -a/(b-a) \in [0,1]$ and write $\mathbb{E}[e^{sY}] \leq (1-p)e^{sa} + pe^{sb}$ by linearity and the fact that $\mathbb{E}[Y] = 0$ gives $\mathbb{E}[(b-Y)] = b$ and $\mathbb{E}[(Y-a)] = -a$... 

Actually: since $\mathbb{E}[Y] = 0$ and $a \leq Y \leq b$:
$$\mathbb{E}[e^{sY}] \leq \frac{-a}{b-a}e^{sb} + \frac{b}{b-a}e^{sa} =: \phi(s).$$
We need to show $\phi(s) \leq e^{s^2(b-a)^2/8}$. Taking $\log\phi(s)$ and expanding around $s=0$: $\log\phi(0) = 0$, $(\log\phi)'(0) = 0$ (since $\mathbb{E}[Y]=0$), and $(\log\phi)''(s) \leq (b-a)^2/4$ by direct computation. Therefore $\log\phi(s) \leq s^2(b-a)^2/8$. $\blacksquare$

Applying the Hoeffding lemma to each factor and optimizing over $s$ (set $s = 4t/\sum_i(b_i-a_i)^2$) gives the stated bound. $\blacksquare$

**Application to Reservoir Computing.** Suppose we compute the empirical linear capacity $\hat{C}_L = \frac{1}{T}\sum_{t=1}^T f(x(t), u(t))$ where $f$ is bounded: $|f| \leq B$. Then by Hoeffding (treating the $T$ observations as approximately independent):
$$\mathbb{P}(|\hat{C}_L - C_L| \geq \varepsilon) \leq 2\exp\!\left(-\frac{2T^2 \varepsilon^2}{T \cdot (2B)^2}\right) = 2\exp\!\left(-\frac{T\varepsilon^2}{2B^2}\right).$$

This says: with $T = O(B^2 \log(1/\delta)/\varepsilon^2)$ samples, the empirical capacity is within $\varepsilon$ of the true capacity with probability at least $1-\delta$.

## 27.3.3 Bernstein's Inequality

Hoeffding requires bounded random variables. Bernstein's inequality is sharper when the variables are sub-exponential (heavy-tailed but with finite exponential moments).

**Definition 27.3.1 (Sub-exponential Random Variable).** A zero-mean random variable $X$ is *sub-exponential* with parameters $(\nu^2, b)$ if:
$$\mathbb{E}[e^{sX}] \leq e^{s^2 \nu^2 / 2} \quad \text{for all } |s| \leq 1/b.$$

Equivalently, $\mathbb{P}(|X| \geq t) \leq 2e^{-t/(2b)}$ for $t \geq 0$ (the tail decays exponentially, not sub-Gaussian).

**Theorem 27.3.2 (Bernstein's Inequality).** *Let $X_1, \ldots, X_n$ be independent, zero-mean random variables. Suppose each $X_i$ has $\mathbb{E}[X_i^k] \leq \frac{k! \sigma_i^2 b^{k-2}}{2}$ for all $k \geq 2$ (a Bernstein condition). Let $\sigma^2 = \sum_i \sigma_i^2$. Then:*
$$\mathbb{P}\!\left(\sum_i X_i \geq t\right) \leq \exp\!\left(-\frac{t^2/2}{\sigma^2 + bt}\right).$$

The Bernstein inequality interpolates between the sub-Gaussian regime ($t \ll \sigma^2/b$, giving Gaussian tails $e^{-t^2/(2\sigma^2)}$) and the sub-exponential regime ($t \gg \sigma^2/b$, giving exponential tails $e^{-t/(2b)}$).

**Proof.** Using the Chernoff method again, with moment generating function:
$$\mathbb{E}[e^{sX_i}] = 1 + \sum_{k=2}^\infty \frac{s^k}{k!}\mathbb{E}[X_i^k] \leq 1 + \frac{s^2 \sigma_i^2/2}{1 - sb} \leq \exp\!\left(\frac{s^2\sigma_i^2/2}{1-sb}\right)$$
for $0 < s < 1/b$. Multiplying over $i$ and optimizing $s = t/(\sigma^2 + bt)$ gives the stated bound. $\blacksquare$

**Application: Reservoir State Variance.** Reservoir states $x(t)$ are not bounded (if the activation is $\tanh$, they lie in $(-1,1)^N$, but if the activation is ReLU or linear, they can be unbounded). For states in $(-1,1)^N$, use Hoeffding. For sub-exponential states, Bernstein applies.

More concretely: the sample covariance estimator $\hat{\Sigma} = \frac{1}{T}\sum_t x(t)x(t)^\top$ has entries $\hat{\Sigma}_{ij} = \frac{1}{T}\sum_t x_i(t)x_j(t)$. If $|x_i(t)| \leq 1$ (bounded activations), the products $x_i(t)x_j(t) \in [-1,1]$ and Hoeffding gives:
$$\mathbb{P}(|\hat{\Sigma}_{ij} - \Sigma_{ij}| \geq \varepsilon) \leq 2e^{-T\varepsilon^2/2}.$$
For all $N^2$ entries simultaneously (union bound):
$$\mathbb{P}(\|\hat{\Sigma} - \Sigma\|_\infty \geq \varepsilon) \leq 2N^2 e^{-T\varepsilon^2/2}.$$
Setting this to $\delta$: $T = O(\log(N/\delta)/\varepsilon^2)$ samples suffice. For spectral norm bounds, we need the matrix Bernstein inequality.

## 27.3.4 Matrix Bernstein Inequality

The matrix version of Bernstein's inequality bounds the spectral norm deviation of a sum of random matrices. This is the tool needed for bounding deviations of the sample covariance $\hat{\Sigma}$ from $\Sigma$ in spectral norm.

**Theorem 27.3.3 (Matrix Bernstein Inequality, Tropp 2012).** *Let $Z_1, \ldots, Z_T$ be independent, zero-mean random symmetric matrices of size $N \times N$. Suppose $\|Z_t\|_{\text{op}} \leq R$ almost surely for all $t$. Define the matrix variance:*
$$\sigma^2 = \left\|\sum_{t=1}^T \mathbb{E}[Z_t^2]\right\|_{\text{op}}.$$

*Then for any $\varepsilon > 0$:*
$$\mathbb{P}\!\left(\left\|\sum_{t=1}^T Z_t\right\|_{\text{op}} \geq \varepsilon\right) \leq 2N \exp\!\left(-\frac{\varepsilon^2/2}{\sigma^2 + R\varepsilon/3}\right).$$

The extra factor of $N$ (the dimension) compared to the scalar case reflects the union-bound cost of controlling all $N$ eigenvalues simultaneously.

**Proof sketch.** The key tool is the *matrix Laplace transform* method. For symmetric matrices, the Chernoff bound gives:
$$\mathbb{P}\!\left(\lambda_{\max}\!\left(\sum Z_t\right) \geq \varepsilon\right) \leq e^{-s\varepsilon} \mathbb{E}\!\left[\text{tr}\exp\!\left(s\sum_t Z_t\right)\right]$$
for $s > 0$. The key lemma is the *Golden-Thompson inequality* (for commuting matrices) and its non-commutative replacement (the *Lieb concavity theorem*): for independent mean-zero matrices,
$$\mathbb{E}\!\left[\text{tr}\exp\!\left(s\sum_t Z_t\right)\right] \leq \text{tr}\exp\!\left(\sum_t \log\mathbb{E}[e^{sZ_t}]\right).$$
This is not exactly right for non-commuting matrices, so one uses the *Golden-Thompson inequality* $\text{tr}(e^{A+B}) \leq \text{tr}(e^A e^B)$ iteratively, or alternatively uses the *matrix cumulant generating function* approach of Tropp [Tropp2012]. The matrix Bernstein condition on $Z_t$ gives:
$$\mathbb{E}[e^{sZ_t}] \preceq \exp\!\left(\frac{s^2 \sigma_t^2/2}{1-sR}\right) I.$$
Multiplying, taking the trace, and optimizing $s$ gives the result. The factor $N$ comes from $\text{tr}(I) = N$. $\blacksquare$

## 27.3.5 Application: Bounding Reservoir Capacity Deviation

**Theorem 27.3.4 (Reservoir Capacity Concentration).** *Let an $N$-unit ESN be driven by a stationary input process for $T$ time steps. Suppose reservoir states satisfy $\|x(t)\|_2 \leq B_x$ almost surely. The empirical linear information capacity*
$$\hat{C} = \text{tr}\!\left(\hat{\Sigma}^{-1/2} \hat{\Sigma}_{xu} \hat{\Sigma}_u^{-1} \hat{\Sigma}_{xu}^\top \hat{\Sigma}^{-1/2}\right)$$
*(an $N \times N$ matrix trace involving the cross-covariance between states and inputs) satisfies:*
$$\mathbb{P}(|\hat{C} - C| \geq \varepsilon) \leq 2N \exp\!\left(-\frac{c T \varepsilon^2}{B_x^4 N^2}\right)$$
*for a universal constant $c > 0$.*

**Proof sketch.** Write $\hat{C}$ as a trace involving $\hat{\Sigma} = \frac{1}{T}XX^\top$. Each summand $Z_t = x(t)x(t)^\top/T - \Sigma/T$ is a rank-1 zero-mean matrix with $\|Z_t\|_{\text{op}} \leq B_x^2/T$ and matrix variance $\sigma^2 = \|\sum_t \mathbb{E}[Z_t^2]\|_{\text{op}} \leq B_x^4/T$. Applying matrix Bernstein to $\hat{\Sigma} - \Sigma = \sum_t Z_t$ gives $\|\hat{\Sigma} - \Sigma\|_{\text{op}} \leq O(B_x^2 \sqrt{N/T})$ with high probability. A perturbation analysis then bounds $|\hat{C} - C|$ in terms of $\|\hat{\Sigma} - \Sigma\|_{\text{op}}$. $\blacksquare$

**Corollary 27.3.1.** *To achieve $|\hat{C} - C| \leq \varepsilon$ with probability at least $1-\delta$:*
$$T = \Omega\!\left(\frac{B_x^4 N^2 \log(N/\delta)}{\varepsilon^2}\right).$$

The quadratic dependence on $N$ is unfortunate: large reservoirs need quadratically more training data to estimate their capacity accurately. This is the *curse of dimensionality* for capacity estimation. In practice, one often uses cross-validation to estimate the appropriate reservoir size rather than computing the capacity directly.

## 27.3.6 The Marchenko-Pastur Law and Sample Covariance Concentration

Combining the Marchenko-Pastur law with concentration:

**Theorem 27.3.5 (Sample Covariance Concentration around MP).** *Under the conditions of Theorem 27.2.1, with $T = cN$:*
$$\mathbb{P}\!\left(\left|\frac{1}{N}\text{tr}(\phi(\hat{\Sigma})) - \int \phi(x)\, d\rho_{MP}(x;c)\right| \geq \varepsilon\right) \leq C e^{-c' N \varepsilon^2}$$
*for any Lipschitz test function $\phi$, where $C, c'$ are universal constants.*

This says the empirical spectral statistics of the state matrix concentrate around the Marchenko-Pastur predictions at a rate $O(1/\sqrt{N})$. For $N = 100$, the deviation is at most $O(0.1)$; for $N = 1000$, it is $O(0.03)$. The MP law is an excellent approximation at practical reservoir sizes.
