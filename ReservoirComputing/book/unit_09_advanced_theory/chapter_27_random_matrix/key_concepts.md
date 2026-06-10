# Chapter 27: Key Concepts

**Empirical Spectral Distribution (ESD).** For an $N \times N$ matrix $A$ with eigenvalues $\lambda_1, \ldots, \lambda_N$, the ESD is $\mu_N = \frac{1}{N}\sum_{i=1}^N \delta_{\lambda_i}$. Convergence of $\mu_N$ as $N \to \infty$ is the central object of study in random matrix theory.

**Wigner Matrix.** A symmetric random matrix with i.i.d. off-diagonal entries (mean 0, variance $\sigma^2$) and i.i.d. diagonal entries. The normalized Wigner matrix $\widetilde{W} = W/(\sigma\sqrt{N})$ has ESD converging to the semicircle distribution.

**Semicircle Distribution.** The probability measure on $[-R, R]$ with density $\rho_{sc}(x;R) = \frac{2}{\pi R^2}\sqrt{R^2-x^2}$. Its $2k$-th moment is the $k$-th Catalan number $C_k$ times $R^{2k}$. The limiting spectral distribution of normalized symmetric random matrices.

**Catalan Numbers.** $C_k = \frac{1}{k+1}\binom{2k}{k}$. The sequence $1, 1, 2, 5, 14, 42, \ldots$ counting Dyck paths, non-crossing pair partitions, and many other combinatorial objects. Their appearance in the moments of the semicircle distribution is the key to the method-of-moments proof.

**Method of Moments.** Proving convergence of measures by showing that all moments converge. For the Wigner law, the $2k$-th moment of the ESD is $\frac{1}{N}\mathbb{E}[\text{tr}(\widetilde{W}^{2k})]$, computed as a sum over closed paths weighted by products of entries. Only non-crossing pair partitions contribute at leading order, yielding Catalan numbers.

**Non-Crossing Pair Partition.** A pairing of $\{1, \ldots, 2k\}$ into $k$ pairs such that no two pairs "cross": no $a < b < c < d$ with $\{a,c\}$ and $\{b,d\}$ both in the partition. The $k$-th Catalan number counts non-crossing pair partitions of $2k$ elements. Their combinatorial property — can be traced via Dyck paths — is why they arise in random matrix theory.

**Circular Law.** For non-symmetric random matrices with i.i.d. entries of variance $1/N$, the eigenvalues (complex) converge to a uniform distribution on the unit disk. For real matrices with off-diagonal entries of variance $\sigma^2/N$, the eigenvalues fill the disk of radius $\sigma$.

**Tracy-Widom Distribution.** The distribution of the fluctuations of the largest eigenvalue of a Wigner matrix, after centering and scaling. The largest eigenvalue converges to $2\sigma$ at rate $N^{-2/3}$, and the centered/scaled fluctuation follows the Tracy-Widom law. Relevant for controlling the actual spectral radius of random reservoirs.

**Sample Covariance Matrix.** $\hat{\Sigma} = \frac{1}{T}XX^\top$ where $X \in \mathbb{R}^{N \times T}$ is the state matrix. Its eigenvalue distribution is governed by the Marchenko-Pastur law when $X$ has approximately i.i.d. columns.

**Marchenko-Pastur Distribution.** The limiting spectral distribution of $\frac{1}{T}ZZ^\top$ when $Z \in \mathbb{R}^{N \times T}$ has i.i.d. entries and $N/T \to c$. Has density $\rho_{MP}(x;c) = \frac{1}{2\pi cx}\sqrt{(x_+-x)(x-x_-)}$ on $[x_-, x_+]$ with $x_\pm = (1\pm\sqrt{c})^2$, plus a point mass at 0 when $c > 1$.

**Aspect Ratio $c$.** The ratio $c = N/T$ (reservoir size to training length). Determines the Marchenko-Pastur distribution. When $c = 1$ (square state matrix), the distribution is maximally spread. When $c \to 0$ ($T \gg N$), the distribution concentrates near $x = 1$.

**Stieltjes Transform.** $m_\mu(z) = \int(x-z)^{-1}d\mu(x)$ for $z \notin \text{supp}(\mu)$. The Marchenko-Pastur distribution is characterized by the fixed-point equation $m = (-z + c/(1+m))^{-1}$. The Stieltjes transform approach is the modern method for proving convergence of ESDs.

**Free Probability.** A noncommutative probability theory (Voiculescu) in which the standard concept of independence is replaced by *freeness*. Free probability provides the algebraic framework for computing eigenvalue distributions of sums and products of large random matrices. The Marchenko-Pastur law follows from the $S$-transform formula for free multiplicative convolution.

**Effective Rank.** $r_{\text{eff}} = [\text{tr}(\hat{\Sigma})]^2 / \text{tr}(\hat{\Sigma}^2) \approx N/(1+c)$ under the Marchenko-Pastur law. Measures the "effective number of dimensions" in the reservoir state representation. Quantifies how much of the reservoir's $N$-dimensional capacity is actually used.

**Hoeffding's Inequality.** For bounded i.i.d. random variables in $[a, b]$, the sample mean concentrates as $\mathbb{P}(|\bar{X} - \mu| \geq \varepsilon) \leq 2\exp(-2T\varepsilon^2/(b-a)^2)$. The simplest and most widely applied concentration inequality.

**Bernstein's Inequality.** A sharper concentration inequality for sub-exponential random variables: $\mathbb{P}(\sum X_i \geq t) \leq \exp(-t^2/(2\sigma^2 + 2bt))$. Interpolates between Gaussian ($t$ small) and exponential ($t$ large) tails.

**Matrix Bernstein Inequality.** Extension of Bernstein to random matrices: bounds the spectral norm deviation of a sum of independent mean-zero matrices. Key tool: $\mathbb{P}(\|\sum Z_t\|_{\text{op}} \geq \varepsilon) \leq 2N\exp(-\varepsilon^2/2(\sigma^2 + R\varepsilon/3))$. Introduces a factor of $N$ (the dimension) compared to the scalar case.

**Sub-Gaussian Variable.** A random variable $X$ with $\mathbb{P}(|X| \geq t) \leq 2e^{-t^2/(2\sigma^2)}$ for all $t$. Bounded variables are sub-Gaussian with $\sigma^2 = (b-a)^2/4$. Products of sub-Gaussian variables are sub-exponential.

**Linear Memory Capacity.** A measure of how many independent linear functionals of the input history a reservoir can compute. Bounded by the reservoir dimension $N$. Under the Marchenko-Pastur law, the capacity achievable with $T$ training samples is approximately $N \cdot T/(N+T)$.
