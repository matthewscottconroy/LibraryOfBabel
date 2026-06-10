# Section 8.7: Regularization of the Readout

## 8.7.1 Why Regularization Is Necessary

The readout weight vector $\mathbf{w}$ is trained by minimizing the sum of squared errors between the reservoir output and the target, subject to a penalty on the weights. Without such a penalty — in the limit $\lambda \to 0$ — the readout performs *ordinary least squares* (OLS) regression on the state matrix. OLS has excellent statistical properties when the number of training samples $T$ greatly exceeds the number of features $N$, but reservoir computing often operates in regimes where $T \sim N$ or even $T < N$. In these regimes, OLS overfits: the readout memorizes noise in the training sequence rather than learning the underlying signal.

Regularization introduces a deliberate bias to reduce variance. The bias-variance tradeoff (Section 5.8) guarantees that the optimal prediction error is achieved at some $\lambda^* > 0$, not at $\lambda = 0$.

## 8.7.2 Ridge Regression

Ridge regression [Tikhonov1963, HoerlKennard1970] adds an $\ell_2$ penalty on the weight magnitude:

$$\hat{\mathbf{w}}(\lambda) = \arg\min_{\mathbf{w}} \|X^\top \mathbf{w} - \mathbf{y}\|^2 + \lambda \|\mathbf{w}\|^2,$$

with closed-form solution

$$\hat{\mathbf{w}}(\lambda) = (XX^\top + \lambda I)^{-1} X \mathbf{y}.$$

In terms of the SVD $X = USV^\top$ (where $S = \text{diag}(s_1, \ldots, s_{\min(N,T)})$):

$$\hat{\mathbf{w}}(\lambda) = \sum_{i} \frac{s_i}{s_i^2 + \lambda} (\mathbf{v}_i^\top \mathbf{y}) \mathbf{u}_i.$$

Each singular component is shrunk by the factor $s_i^2 / (s_i^2 + \lambda)$. Components with $s_i \gg \lambda^{1/2}$ are nearly unshrunk; components with $s_i \ll \lambda^{1/2}$ are strongly suppressed. The regularization selectively discards low-energy directions (small $s_i$) in the state space, which are typically the noisy, unstable directions.

**Effect of $\lambda$:**
- **Too small ($\lambda \to 0$):** OLS; fits training noise; high variance on test data.
- **Optimal ($\lambda = \lambda^*$):** Minimal expected test error; balances bias and variance.
- **Too large ($\lambda \to \infty$):** $\hat{\mathbf{w}} \to \mathbf{0}$; predicts the constant zero; maximum bias, zero variance.

The optimal $\lambda^*$ depends on the noise level $\sigma_\varepsilon^2$ and the signal-to-noise ratio of the state matrix. For Gaussian noise, $\lambda^* \approx \sigma_\varepsilon^2 N / \|\mathbf{w}^*\|^2$ (the ratio of noise variance to signal power), which motivates the Bayesian interpretation below.

## 8.7.3 Generalized Cross-Validation

The most computationally efficient method for selecting $\lambda$ without a separate validation set is *generalized cross-validation* (GCV) [GolubHeathWahba1979]:

$$\text{GCV}(\lambda) = \frac{\|\mathbf{y} - H(\lambda)\mathbf{y}\|^2}{\left(1 - \frac{1}{T}\text{tr}(H(\lambda))\right)^2},$$

where $H(\lambda) = X^\top(XX^\top + \lambda I)^{-1}X \in \mathbb{R}^{T \times T}$ is the hat (projection) matrix. The effective degrees of freedom is $\text{tr}(H(\lambda)) = \sum_i s_i^2/(s_i^2 + \lambda)$, which ranges from $\min(N,T)$ at $\lambda = 0$ to 0 as $\lambda \to \infty$.

**Derivation.** GCV is derived as an approximation to leave-one-out cross-validation (LOO-CV). The LOO prediction error for removing time step $t$ is

$$\hat{e}_t^{LOO} = \frac{y_t - (H(\lambda)\mathbf{y})_t}{1 - H(\lambda)_{tt}}.$$

GCV approximates $H(\lambda)_{tt}$ by its average $\text{tr}(H(\lambda))/T$, giving the GCV formula above. This approximation is accurate when the leverage scores $H(\lambda)_{tt}$ are approximately constant — which is guaranteed for time series with i.i.d. states [GolubHeathWahba1979].

**Efficient computation.** Given the SVD $X = USV^\top$, compute $\hat{\mathbf{y}} = US^2(S^2 + \lambda I)^{-1}U^\top \mathbf{y}$ in $O(T^2)$ time (after the $O(NT^2)$ SVD). The GCV criterion is then evaluated in $O(T)$ time for each $\lambda$, and the minimum is found by a 1D search (e.g., golden section or Brent's method).

## 8.7.4 LASSO: Sparse Readout Weights

When the task requires the readout to depend on only a small subset of reservoir neurons (e.g., when the task is simple enough that most neurons are irrelevant), $\ell_1$ regularization (LASSO) [Tibshirani1996] is preferable:

$$\hat{\mathbf{w}}^{LASSO}(\lambda) = \arg\min_{\mathbf{w}} \|X^\top \mathbf{w} - \mathbf{y}\|^2 + \lambda \|\mathbf{w}\|_1.$$

LASSO induces *sparsity*: many readout weights are exactly zero at the optimal $\hat{\mathbf{w}}^{LASSO}$. This is in contrast to ridge, which shrinks all weights toward zero but rarely sets any to exactly zero.

The LASSO solution does not have a closed form and requires iterative algorithms (coordinate descent, ISTA/FISTA). However, for moderate $N$ ($< 1000$), these algorithms converge quickly (50-200 iterations), and the solution path over all $\lambda$ can be computed efficiently using the LARS algorithm.

**When to use LASSO:** When interpretability is desired (which neurons matter?), when the reservoir is overcomplete ($N \gg$ effective task complexity), or when the readout will be deployed on hardware with a strict budget for connections.

## 8.7.5 Elastic Net

The elastic net [ZouHastie2005] combines $\ell_1$ and $\ell_2$ penalties:

$$\hat{\mathbf{w}}^{EN}(\lambda_1, \lambda_2) = \arg\min_{\mathbf{w}} \|X^\top \mathbf{w} - \mathbf{y}\|^2 + \lambda_1 \|\mathbf{w}\|_1 + \lambda_2 \|\mathbf{w}\|^2.$$

This achieves sparse solutions (from $\ell_1$) while also regularizing correlated features together (from $\ell_2$). In reservoirs, many neurons are highly correlated (they receive similar inputs through the recurrent dynamics), so LASSO may arbitrarily select one neuron and discard the others. The elastic net retains all correlated neurons at similar weights, producing more stable and interpretable solutions.

The elastic net is the recommended regularizer when both sparsity and stability are desired. The ratio $\lambda_1/\lambda_2$ controls the degree of sparsity (large ratio $\to$ more sparse, small ratio $\to$ ridge-like).

## 8.7.6 Bayesian Interpretation

Ridge regression has a natural Bayesian interpretation [MacKay1992]: it corresponds to MAP estimation under a Gaussian prior on the weights,

$$p(\mathbf{w}) = \mathcal{N}(\mathbf{0}, \tau^2 I),$$

with Gaussian likelihood $p(\mathbf{y} | \mathbf{w}) = \mathcal{N}(X^\top \mathbf{w}, \sigma_\varepsilon^2 I)$. The MAP estimate is

$$\hat{\mathbf{w}}_{MAP} = \arg\max_{\mathbf{w}} [p(\mathbf{y}|\mathbf{w}) p(\mathbf{w})] = (XX^\top + \lambda I)^{-1} X\mathbf{y},$$

with $\lambda = \sigma_\varepsilon^2 / \tau^2$ — the ratio of noise variance to prior variance.

This interpretation clarifies the meaning of $\lambda$: it encodes the practitioner's prior belief about the scale of the readout weights. Large $\lambda$ (small prior variance $\tau^2$) asserts strong prior belief that the weights should be near zero. Small $\lambda$ (large $\tau^2$) asserts that large weights are plausible, reducing regularization.

The Bayesian framework also provides a principled method for selecting $\lambda$ by maximizing the *marginal likelihood* (evidence):

$$\log p(\mathbf{y}|\lambda) = -\frac{1}{2}\mathbf{y}^\top(X^\top X / \sigma_\varepsilon^2 + \lambda I / \sigma_\varepsilon^2)^{-1}\mathbf{y}/\sigma_\varepsilon^2 - \frac{1}{2}\log\det(\cdots) - \frac{T}{2}\log(2\pi\sigma_\varepsilon^2).$$

This is equivalent to GCV for large $T$ [MacKay1992].

---

## References

- **[GolubHeathWahba1979]** G. H. Golub, M. Heath, and G. Wahba. "Generalized cross-validation as a method for choosing a good ridge parameter." *Technometrics*, 21(2):215-223, 1979.
- **[HoerlKennard1970]** A. E. Hoerl and R. W. Kennard. "Ridge regression: Biased estimation for nonorthogonal problems." *Technometrics*, 12(1):55-67, 1970.
- **[MacKay1992]** D. J. C. MacKay. "Bayesian interpolation." *Neural Computation*, 4(3):415-447, 1992.
- **[Tibshirani1996]** R. Tibshirani. "Regression shrinkage and selection via the lasso." *Journal of the Royal Statistical Society: Series B*, 58(1):267-288, 1996.
- **[Tikhonov1963]** A. N. Tikhonov. "Solution of incorrectly formulated problems and the regularization method." *Soviet Mathematics Doklady*, 4:1035-1038, 1963.
- **[ZouHastie2005]** H. Zou and T. Hastie. "Regularization and variable selection via the elastic net." *Journal of the Royal Statistical Society: Series B*, 67(2):301-320, 2005.
