# 14.1.1 The Bias-Variance Decomposition

## Setup

Let $\mathcal{D} = \{(\mathbf{x}_1, y_1), \ldots, (\mathbf{x}_T, y_T)\}$ be a training set drawn from some distribution $p(\mathbf{x}, y)$. We want to learn a predictor $\hat{f}: \mathbb{R}^d \to \mathbb{R}$ that generalizes well to new test points. In the reservoir computing context, $\mathbf{x}_t$ is the reservoir state and $y_t$ is the target output.

The expected squared error of a predictor $\hat{f}$ trained on dataset $\mathcal{D}$ at a new test point $\mathbf{x}^*$ with true output $y^* = f(\mathbf{x}^*) + \varepsilon$ (where $\varepsilon$ is mean-zero noise with variance $\sigma^2$) is:

$$\mathbb{E}_\mathcal{D}\!\left[(y^* - \hat{f}(\mathbf{x}^*))^2\right] = \text{Bias}^2\!\left[\hat{f}(\mathbf{x}^*)\right] + \text{Var}\!\left[\hat{f}(\mathbf{x}^*)\right] + \sigma^2$$

where:
$$\text{Bias}\!\left[\hat{f}(\mathbf{x}^*)\right] = \mathbb{E}_\mathcal{D}\!\left[\hat{f}(\mathbf{x}^*)\right] - f(\mathbf{x}^*)$$
$$\text{Var}\!\left[\hat{f}(\mathbf{x}^*)\right] = \mathbb{E}_\mathcal{D}\!\left[\left(\hat{f}(\mathbf{x}^*) - \mathbb{E}_\mathcal{D}[\hat{f}(\mathbf{x}^*)]\right)^2\right]$$

The expectation $\mathbb{E}_\mathcal{D}$ is over the randomness of the training dataset. The bias measures how far the expected prediction is from the truth. The variance measures how sensitive the predictor is to the particular training set drawn.

**Proof of the decomposition.** Let $\bar{f}(\mathbf{x}^*) = \mathbb{E}_\mathcal{D}[\hat{f}(\mathbf{x}^*)]$ denote the expected prediction. Then:

$$\mathbb{E}_\mathcal{D}[(y^* - \hat{f})^2] = \mathbb{E}_\mathcal{D}[(f^* + \varepsilon - \hat{f})^2]$$

Expanding and using $\mathbb{E}[\varepsilon] = 0$, $\mathbb{E}[\varepsilon^2] = \sigma^2$, and the independence of $\varepsilon$ from $\hat{f}$:

$$= \mathbb{E}_\mathcal{D}[(f^* - \hat{f})^2] + \sigma^2$$

$$= \mathbb{E}_\mathcal{D}[(f^* - \bar{f} + \bar{f} - \hat{f})^2] + \sigma^2$$

$$= (f^* - \bar{f})^2 + 2(f^* - \bar{f})\underbrace{\mathbb{E}_\mathcal{D}[\bar{f} - \hat{f}]}_{=0} + \mathbb{E}_\mathcal{D}[(\bar{f} - \hat{f})^2] + \sigma^2$$

$$= \underbrace{(f^* - \bar{f})^2}_{\text{Bias}^2} + \underbrace{\mathbb{E}_\mathcal{D}[(\hat{f} - \bar{f})^2]}_{\text{Variance}} + \underbrace{\sigma^2}_{\text{Noise}}$$

$\square$

## Interpreting the Terms in Reservoir Computing

**Bias.** For a reservoir computer, the bias has two sources:

1. **Reservoir bias**: the particular random realization $W^{rec}$ may not have the right spectral structure to represent the target function efficiently. A reservoir that is too large and too ordered may not mix inputs well; one that is too small may not have enough capacity.

2. **Readout bias**: with ridge regularization $\lambda$, the optimal readout solution $\hat{W}^{out} = (X^\top X + \lambda I)^{-1} X^\top \mathbf{y}$ is biased toward zero. The bias grows with $\lambda$ and decreases with training set size.

**Variance.** The variance has sources:
1. **Training data variance**: a different training sequence (different noise realization) produces a different readout.
2. **Reservoir randomness**: a different random realization of $W^{rec}$ produces different states, leading to a different optimal readout. This is the component that ensembles target.

**Noise.** The irreducible error due to stochastic variation in the training targets. No method can reduce this term.

## The Bias-Variance Tradeoff in Ridge Regression

For the specific case of ridge regression on reservoir states, we can be more precise. Let $X \in \mathbb{R}^{T \times N}$ be the matrix of reservoir states and $\mathbf{y} \in \mathbb{R}^T$ the targets. The ridge regression solution is:

$$\hat{\mathbf{w}} = (X^\top X + \lambda I)^{-1} X^\top \mathbf{y}$$

Using the singular value decomposition $X = U\Sigma V^\top$ with singular values $\sigma_1 \geq \sigma_2 \geq \cdots \geq \sigma_N$, the prediction at a new test state $\mathbf{x}^*$ is:

$$\hat{f}(\mathbf{x}^*) = \mathbf{x}^{*\top} \hat{\mathbf{w}} = \sum_{k=1}^N \frac{\sigma_k^2}{\sigma_k^2 + \lambda} (u_k^\top \mathbf{y})(v_k^\top \mathbf{x}^*)$$

The shrinkage factor $\frac{\sigma_k^2}{\sigma_k^2 + \lambda}$ approaches 1 for large singular values (low-variance directions well-covered by data) and approaches 0 for small singular values (high-variance directions). This creates a bias for small-$\sigma_k$ components and reduces variance for all components.

The integrated bias-variance tradeoff over the test distribution is:

$$\text{Total Error} = \lambda^2 \sum_k \frac{(v_k^\top \mathbf{w}^*)^2}{(\sigma_k^2 + \lambda)^2}\sigma_k^2 + \sigma^2 \sum_k \frac{\sigma_k^2}{(\sigma_k^2 + \lambda)^2}$$

where $\mathbf{w}^*$ is the true optimal readout. The first sum is the integrated bias (grows with $\lambda$); the second is the integrated variance (decreases with $\lambda$). The optimal $\lambda^*$ balances these two terms and depends on $\sigma^2$ and the spectrum of $X^\top X$ — both of which are properties of the specific reservoir realization.

## The Ensemble Perspective on Bias and Variance

The key insight motivating ensembles is:

- **Averaging predictions** reduces variance without changing bias.
- **Different reservoirs** have different biases (arising from different random structures).
- If reservoir biases are not perfectly correlated, averaging across reservoirs can also reduce bias.

The variance reduction is guaranteed (Theorem 14.2.1). The bias reduction is not guaranteed in general, but occurs in practice when reservoirs are initialized with different structural parameters (spectral radius, density, leaking rate) as well as different random seeds.

---

## References

- [Geman1992] Geman, S., Bienenstock, E., & Doursat, R. (1992). Neural networks and the bias/variance dilemma. *Neural Computation*, 4(1), 1–58.
- [Breiman1996] Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.
- [Hastie2009] Hastie, T., Tibshirani, R., & Friedman, J. (2009). *The Elements of Statistical Learning*, 2nd ed. Springer. Chapter 7.
