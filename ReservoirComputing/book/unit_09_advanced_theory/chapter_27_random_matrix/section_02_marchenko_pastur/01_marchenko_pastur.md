# Section 27.2: The Marchenko-Pastur Law

## 27.2.1 The State Matrix and Its Singular Values

When a reservoir of $N$ units runs for $T$ time steps, it produces a state matrix $X \in \mathbb{R}^{N \times T}$ whose columns are the reservoir states at each time step:
$$X = [x(1)\ |\ x(2)\ |\ \cdots\ |\ x(T)].$$
The output weights $W_{\text{out}} \in \mathbb{R}^{m \times N}$ are trained by linear regression:
$$W_{\text{out}} = Y X^\dagger = Y X^\top (XX^\top)^{-1},$$
where $Y \in \mathbb{R}^{m \times T}$ is the target output matrix and $X^\dagger$ is the Moore-Penrose pseudoinverse of $X$ (when $T \geq N$ and $X$ has full rank).

The singular values of $X$ are the square roots of the eigenvalues of the sample covariance matrix $\hat{\Sigma} = \frac{1}{T} X X^\top \in \mathbb{R}^{N \times N}$. They determine:
1. The *effective rank* of the reservoir representation: how many independent directions the reservoir states span.
2. The conditioning of the linear regression problem: a large ratio $\sigma_{\max}/\sigma_{\min}$ means the regression is ill-conditioned.
3. The *information capacity* of the reservoir: the number of independent linear functionals of the input that can be computed simultaneously.

For a random input-driven reservoir, the state matrix $X$ behaves (in first approximation) like a random matrix with dependent but approximately i.i.d. columns. The Marchenko-Pastur law describes the singular value distribution of such matrices.

## 27.2.2 The Marchenko-Pastur Distribution

**Definition 27.2.1 (Marchenko-Pastur Distribution).** Let $c > 0$ (the *aspect ratio* or *shape parameter*). The *Marchenko-Pastur distribution* with parameter $c$ is the probability measure on $[0, \infty)$ with density:

$$\rho_{MP}(x; c) = \frac{1}{2\pi c x} \sqrt{(x_+ - x)(x - x_-)}\, \mathbf{1}_{[x_-, x_+]}(x) + \max(1 - c^{-1}, 0)\, \delta_0$$

where the edge values are:
$$x_\pm = (1 \pm \sqrt{c})^2.$$

The point mass at 0 appears only when $c > 1$ (i.e., $N > T$), with weight $(1 - 1/c)$, reflecting the fact that there are $N - T$ zero singular values when the matrix has fewer columns than rows.

The density integrates to $\min(1, 1/c)$, and together with the point mass at 0, the total mass is 1.

**Moments of the Marchenko-Pastur distribution:**
$$\int x^k\, d\rho_{MP}(x;c) = \sum_{j=0}^{k-1} \frac{1}{j+1}\binom{k}{j}\binom{k}{j+1} c^j.$$

The first few moments are:
- $\mathbb{E}[X] = 1$ (the mean singular value squared is 1, reflecting normalization).
- $\mathbb{E}[X^2] = 1 + c$ (variance of squared singular values is $c$).
- $\mathbb{E}[X^3] = 1 + 3c + c^2$.

## 27.2.3 The Marchenko-Pastur Theorem

**Theorem 27.2.1 (Marchenko-Pastur Law, 1967).** *Let $Z$ be an $N \times T$ matrix with i.i.d. entries of mean 0 and variance $\sigma^2$. Set $c_N = N/T$ and assume $c_N \to c \in (0, \infty)$ as $N, T \to \infty$. Then the empirical spectral distribution of the sample covariance matrix*
$$\hat{\Sigma}_N = \frac{1}{T} Z Z^\top$$
*converges weakly to the Marchenko-Pastur distribution $\rho_{MP}(\cdot; c)$ scaled by $\sigma^2$:*
$$\frac{1}{N}\sum_{i=1}^N \delta_{\lambda_i(\hat{\Sigma}_N)} \xrightarrow{w} \rho_{MP}(\cdot/\sigma^2; c) \quad \text{a.s.}$$

## 27.2.4 The Stieltjes Transform and Free Probability

The modern proof of the Marchenko-Pastur law uses the *Stieltjes transform* (also called the Cauchy transform):

**Definition 27.2.2 (Stieltjes Transform).** For a probability measure $\mu$ on $\mathbb{R}$, its Stieltjes transform is
$$m_\mu(z) = \int \frac{1}{x - z}\, d\mu(x), \quad z \in \mathbb{C} \setminus \text{supp}(\mu).$$

The measure $\mu$ can be recovered from $m_\mu$ via the Stieltjes inversion formula:
$$\mu(dx) = -\frac{1}{\pi} \lim_{\varepsilon \to 0^+} \text{Im}(m_\mu(x + i\varepsilon))\, dx.$$

**Theorem 27.2.2 (Stieltjes Transform of Marchenko-Pastur).** *The Stieltjes transform $m = m(z)$ of $\rho_{MP}(\cdot; c)$ satisfies the functional equation:*
$$m = \frac{1}{-z + \frac{c}{1 + m}}.$$

*Equivalently:* $zm^2 + (z - 1 + c)m + 1 = 0$ — wait, the correct form is: $-z + c/(1+m) = 1/m$, giving $-z(1+m) + c = (1+m)/m$, so $-zm - z + c = 1/m + 1$, and cross-multiplying: $m(-zm - z + c - 1) = 1$, giving $-zm^2 + (c - 1 - z)m - 1 = 0$, or $zm^2 + (1 + z - c)m + 1 = 0$.

The two roots of this quadratic in $m$ are:
$$m(z) = \frac{-(1+z-c) \pm \sqrt{(1+z-c)^2 - 4z}}{2z}.$$

The correct root is chosen by the condition $\text{Im}(m(z)) > 0$ for $\text{Im}(z) > 0$.

**Free Probability Intuition.** The Marchenko-Pastur law has a natural interpretation in free probability theory. For large $N$, the matrix $Z/\sqrt{T}$ can be treated as a "free random variable" in the sense of Voiculescu [Voiculescu1991]. The eigenvalue distribution of $ZZ^\top/T$ is the *free multiplicative convolution* of the empirical distribution of the rows of $Z/\sqrt{T}$ (which is a unit variance empirical distribution) with itself.

The key free probability formula is: if $A$ and $B$ are large random matrices that are asymptotically free, then the eigenvalue distribution of $AB$ is determined by the eigenvalue distributions of $A$ and $B$ alone, via the $S$-transform:
$$S_{AB}(z) = S_A(z) S_B(z).$$

For the sample covariance $\hat{\Sigma} = ZZ^\top/T$, the relevant $S$-transform calculation recovers the Marchenko-Pastur distribution.

## 27.2.5 Proof of the Marchenko-Pastur Law via Stieltjes Transform

We sketch the proof for $\sigma^2 = 1$. The idea is to show that the Stieltjes transform of the empirical distribution of $\hat{\Sigma}_N = ZZ^\top/T$ converges to the solution of the fixed-point equation above.

**Step 1: Resolvent identity.** The Stieltjes transform of $\hat{\Sigma}$ is
$$m_N(z) = \frac{1}{N}\text{tr}((\hat{\Sigma} - zI)^{-1}) = \frac{1}{N}\sum_{i=1}^N \frac{1}{\lambda_i - z}.$$

**Step 2: Self-consistent equation.** Using the rank-one update formula and the independence structure of $Z$, one derives that $m_N(z)$ approximately satisfies the Marchenko-Pastur functional equation. For $Z$ with i.i.d. columns $z_1, \ldots, z_T$ of the matrix:

$$\hat{\Sigma} = \frac{1}{T}\sum_{t=1}^T z_t z_t^\top.$$

By the Sherman-Morrison-Woodbury identity:
$$(\hat{\Sigma} - zI)^{-1} = \left(\frac{1}{T}\sum_t z_t z_t^\top - zI\right)^{-1}.$$

Decomposing $\hat{\Sigma} = \frac{1}{T}(ZZ^\top)$ and using the matrix identity $(AB - zI)^{-1}$ vs.$(BA - zI)^{-1}$:
$$\frac{1}{N}\text{tr}(\hat{\Sigma}^k - zI)^{-1} \approx \frac{1}{N}\text{tr}\!\left(\!\left(-z + c \cdot \frac{1}{1 + m_N(z)}\right)^{-1} I\right) \approx \frac{1}{-z + c/(1+m_N(z))}.$$

This is the fixed-point equation. The full proof requires controlling the error in the approximation, which can be done using concentration inequalities and the resolvent identity. See [Anderson2010] Chapter 3 for the complete argument.

## 27.2.6 Application to Reservoir Capacity

The Marchenko-Pastur law directly governs the capacity of the reservoir's linear readout.

**Definition 27.2.3 (Linear Memory Capacity).** The *linear memory capacity* at lag $k$ is:
$$C_L(k) = \frac{[\text{Cov}(y(t), x(t-k))]^2}{\text{Var}(y(t))\text{Var}(x(t-k))},$$
summed over all output units, measuring how well the reservoir can recall input $k$ steps in the past using a linear readout.

The total linear capacity $C_L = \sum_{k=0}^\infty C_L(k)$ is bounded above by the reservoir dimension $N$ [JaegerHaas2004]. The Marchenko-Pastur law tells us how close to this bound a random reservoir comes:

**Proposition 27.2.1.** *For a random reservoir with $N$ units driven by $T$ time steps of Gaussian input, with aspect ratio $c = N/T$, the effective rank of the state matrix $X$ is*
$$r_{\text{eff}} = \frac{[\text{tr}(\hat{\Sigma})]^2}{\text{tr}(\hat{\Sigma}^2)} \approx \frac{N}{1 + c}.$$

**Proof.** Under the Marchenko-Pastur law, $\frac{1}{N}\text{tr}(\hat{\Sigma}) \to 1$ and $\frac{1}{N}\text{tr}(\hat{\Sigma}^2) \to 1 + c$. Therefore $r_{\text{eff}} = N^2/[N(1+c)] = N/(1+c)$. $\blacksquare$

This result has a direct practical implication: the reservoir state matrix has effective rank $N/(1+c)$, not $N$. When $T = N$ (aspect ratio $c = 1$), the effective rank is $N/2$ — only half the reservoir units contribute effectively to the linear regression. To achieve full effective rank $N$, we need $T \gg N$ (long training sequences).

**Conditioning of the Regression.** The condition number of $\hat{\Sigma}$ is the ratio of its largest to smallest eigenvalue. Under the Marchenko-Pastur law:
$$\kappa(\hat{\Sigma}) = \frac{x_+}{x_-} = \frac{(1 + \sqrt{c})^2}{(1 - \sqrt{c})^2}.$$

For $c$ close to 1 (square state matrix), $\kappa(\hat{\Sigma}) \to \infty$ — the regression is extremely ill-conditioned. This is the statistical justification for ridge regression: the regularization $\lambda$ effectively inflates the smallest singular values, improving conditioning. The optimal $\lambda$ under the Marchenko-Pastur law can be computed explicitly [LedoitWolf2004].

**Optimal Aspect Ratio.** The condition number is minimized at $c \to 0$ (many more time steps than reservoir units), but the effective rank is then maximized only as $T \to \infty$. In practice, a good balance is $c \approx 0.1$, giving condition number $\approx (1 + \sqrt{0.1})^2 / (1 - \sqrt{0.1})^2 \approx 5.5$ — well-conditioned enough for reliable regression.
