# Double Descent and Overparameterized Reservoir Readouts

## 28.4.1 The Classical Bias-Variance Tradeoff

Classical statistical learning theory predicts a U-shaped test error curve as model complexity increases. In the **underparameterized regime** — where the number of parameters $N$ is small relative to the number of training examples $T$ — increasing $N$ reduces bias at the cost of higher variance. In the **overparameterized regime** — where $N > T$ — the model can interpolate the training data exactly, and classical theory predicts severe overfitting. The optimal model complexity was thought to lie somewhere between these extremes, at the minimum of the U-shaped curve.

Mathematically, for ordinary least squares (OLS) regression with $N$ parameters and $T$ samples, the bias and variance satisfy:

$$
\mathrm{Bias}^2 \propto \left\|\mathbf{I} - \mathbf{X}(\mathbf{X}^T\mathbf{X})^{-1}\mathbf{X}^T\right\|_\mathrm{op}^2,\quad \mathrm{Variance} \propto \frac{N\sigma^2}{T - N - 1} \text{ for } N < T.
$$

At $N = T$, the variance diverges (interpolation threshold), and the OLS solution is not unique for $N > T$. Classical analysis recommends regularization (ridge regression) to stay in the underparameterized regime.

## 28.4.2 The Double Descent Phenomenon

Modern large-scale machine learning contradicts the classical picture. [Belkin et al. 2019] documented that test error often *decreases again* after an initial rise at the interpolation threshold $N = T$:

$$
\text{Test error} = \begin{cases}
\text{U-shaped (bias-variance)} & N < T \\
\text{Peak (interpolation threshold)} & N \approx T \\
\text{Decreasing again} & N > T
\end{cases}
$$

This **double descent** curve was observed empirically in neural networks, random forests, and linear regression. The phenomenon was explained theoretically in the linear regression setting by [Bartlett et al. 2020] via the **benign overfitting** theorem.

## 28.4.3 SVD Analysis of the Linear Readout

The double descent phenomenon admits a clean analysis for the linear readout via the singular value decomposition (SVD). Let $\mathbf{X} \in \mathbb{R}^{T \times N}$ be the state matrix (rows are reservoir states at each time step), $\mathbf{y} \in \mathbb{R}^T$ the target vector.

**Case 1: $N < T$ (underparameterized).** The OLS solution is unique:

$$
\hat{\mathbf{w}}_{\mathrm{OLS}} = (\mathbf{X}^T\mathbf{X})^{-1}\mathbf{X}^T\mathbf{y},
$$

provided $\mathbf{X}$ has full column rank. Let $\mathbf{X} = \mathbf{U}\mathbf{\Sigma}\mathbf{V}^T$ be the SVD with $\sigma_1 \geq \cdots \geq \sigma_N > 0$. Then:

$$
\hat{\mathbf{w}}_{\mathrm{OLS}} = \mathbf{V}\mathbf{\Sigma}^{-1}\mathbf{U}^T\mathbf{y} = \sum_{i=1}^N \frac{\langle \mathbf{u}_i, \mathbf{y}\rangle}{\sigma_i} \mathbf{v}_i.
$$

The test error on new data $(\mathbf{x}', y')$ involves both the signal component (useful for prediction) and the noise component (amplified by small $\sigma_i$).

**Case 2: $N > T$ (overparameterized).** There are infinitely many solutions satisfying $\mathbf{X}\mathbf{w} = \mathbf{y}$. Gradient descent (initialized at $\mathbf{w} = \mathbf{0}$) converges to the **minimum-norm interpolating solution**:

$$
\hat{\mathbf{w}}_{\min} = \mathbf{X}^T(\mathbf{X}\mathbf{X}^T)^{-1}\mathbf{y} = \mathbf{V}\mathbf{\Sigma}^T(\mathbf{\Sigma}\mathbf{\Sigma}^T)^{-1}\mathbf{U}^T\mathbf{y} = \sum_{i=1}^T \frac{\langle \mathbf{u}_i, \mathbf{y}\rangle}{\sigma_i}\mathbf{v}_i.
$$

This solution uses only the top $T$ right singular vectors of $\mathbf{X}$. The remaining $N - T$ dimensions contribute zero to $\hat{\mathbf{w}}_{\min}$, regardless of $N$.

**Risk decomposition.** The expected test error of $\hat{\mathbf{w}}_{\min}$ decomposes as [Bartlett et al. 2020]:

$$
\mathbb{E}[\|\mathbf{x}^T\hat{\mathbf{w}}_{\min} - y\|^2] = \underbrace{\|\mathbf{w}^*_\perp\|_{\Sigma}^2}_{\text{bias: tail of true }{\mathbf{w}^*}} + \underbrace{\sigma^2 \cdot \frac{T \cdot \lambda_1(\Sigma)^2}{\left(\sum_{i > T}\lambda_i(\Sigma)\right)^2}}_{\text{variance: depends on tail eigenvalues}},
$$

where $\mathbf{w}^*_\perp$ is the component of the true parameter vector orthogonal to the top-$T$ subspace of $\Sigma$, and $\sigma^2$ is the noise variance.

## 28.4.4 Benign Overfitting in Reservoir Readouts

The main result of [Bartlett et al. 2020] establishes conditions under which the minimum-norm interpolating solution generalizes:

**Theorem 28.9 (Benign Overfitting [Bartlett et al. 2020]).** Suppose $\mathbf{x} \sim \mathcal{N}(0, \Sigma)$ with eigenvalues $\lambda_1 \geq \lambda_2 \geq \cdots \geq \lambda_N \geq 0$. Define the **effective dimension**

$$
k^*(T) = \max\left\{k : \sum_{i > k}\lambda_i \geq k\lambda_k\right\}.
$$

If $k^*(T) \gg T$ (many eigenvalues are small but their sum is large), then $\hat{\mathbf{w}}_{\min}$ generalizes: $\mathbb{E}[\|\mathbf{x}^T\hat{\mathbf{w}}_{\min} - y\|^2] \to \sigma^2$ as $T \to \infty$, even though the fit is perfect on training data.

**Interpretation for reservoir computing.** Large reservoirs typically have a few large eigenvalues (capturing the main signal dimensions) and many small eigenvalues (corresponding to "noise modes" in the reservoir state). The benign overfitting condition $k^*(T) \gg T$ is satisfied when the tail of the eigenvalue spectrum is flat (polynomially decaying): $\lambda_i \sim C/i^{\alpha}$ for $\alpha \leq 1$.

Reservoir states tend to have this structure when:
- The spectral radius $\rho(W^{\mathrm{rec}}) \lesssim 1$ (dynamics near the edge of chaos compress many input dimensions equally)
- The reservoir is sparse (many near-zero connections lead to many low-variance modes)
- The input has long temporal correlations (the reservoir integrates correlated signals, diffusing energy across many eigenvalues)

## 28.4.5 The Interpolation Threshold in Practice

The interpolation threshold $N = T$ corresponds to the worst-case generalization. For reservoir readouts, this means that reservoirs of size $N \approx T$ are the most dangerous configuration. Both smaller and larger reservoirs can generalize better:

- $N \ll T$: classical bias-variance regime; regularization manages variance
- $N \gg T$: overparameterized regime; minimum-norm solution is benign if eigenvalue spectrum satisfies Theorem 28.9

**Practical recommendation.** When operating with limited data ($T$ fixed), either:
(a) Use a small reservoir $N \ll T$ with appropriate regularization; or
(b) Use a very large reservoir $N \gg T$ with minimum-norm readout (minimal or zero ridge penalty) — but verify that the reservoir eigenvalue spectrum decays slowly.

Avoid the regime $N \approx T$ unless strong regularization is applied.

## 28.4.6 Empirical Evidence in Reservoir Systems

[Gonon & Ortega 2020] and subsequent work have demonstrated the double-descent curve empirically for ESNs on NARMA-10 and chaotic time series tasks. Test error peaks near $N = T$, then decreases for $N > T$. This is consistent with Theorem 28.9 when the ESN state covariance has a slowly decaying spectrum — which is typical for standard random ESN initializations with spectral radius near 1.

## References

- Bartlett, P. L., Montanari, A., and Rakhlin, A. (2020). Benign overfitting in linear regression. *Proceedings of the National Academy of Sciences*, 117(48), 30063–30070.
- Belkin, M., Hsu, D., Ma, S., and Mandal, S. (2019). Reconciling modern machine-learning practice and the classical bias-variance trade-off. *Proceedings of the National Academy of Sciences*, 116(32), 15849–15854.
- Gonon, L. and Ortega, J.-P. (2020). Reservoir computing universality with stochastic inputs. *IEEE Transactions on Neural Networks and Learning Systems*, 31(1), 100–112.
- Hastie, T., Montanari, A., Rosset, S., and Tibshirani, R. J. (2022). Surprises in high-dimensional ridgeless least squares interpolation. *The Annals of Statistics*, 50(2), 949–986.
- Mei, S. and Montanari, A. (2022). The generalization error of random features regression: Precise asymptotics and the double descent curve. *Communications on Pure and Applied Mathematics*, 75(4), 667–766.
