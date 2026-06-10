# Section 10.3: Gaussian Process Readout

## 10.3.1 Gaussian Processes: A Brief Review

A Gaussian process (GP) is a distribution over functions. A random function $f: \mathcal{X} \to \mathbb{R}$ is a GP with mean function $m(\mathbf{x}) = \mathbb{E}[f(\mathbf{x})]$ and covariance function (kernel) $k(\mathbf{x}, \mathbf{x}') = \text{Cov}[f(\mathbf{x}), f(\mathbf{x}')]$ if any finite collection of function values $\{f(\mathbf{x}_1), \ldots, f(\mathbf{x}_n)\}$ is jointly Gaussian:

$$\begin{pmatrix} f(\mathbf{x}_1) \\ \vdots \\ f(\mathbf{x}_n) \end{pmatrix} \sim \mathcal{N}\!\left(\begin{pmatrix} m(\mathbf{x}_1) \\ \vdots \\ m(\mathbf{x}_n) \end{pmatrix},\; K\right),$$

where $K_{ij} = k(\mathbf{x}_i, \mathbf{x}_j)$.

We write $f \sim \mathcal{GP}(m, k)$.

For the reservoir readout, the "input" $\mathbf{x}$ is the reservoir state $\mathbf{r}(t) \in \mathbb{R}^N$, and the function $f$ is the readout function. The GP prior over $f$ encodes our beliefs about the readout before seeing data.

## 10.3.2 The GP Readout Model

**Model.** Let $\mathbf{r}_1, \ldots, \mathbf{r}_T$ be the reservoir states at times $1, \ldots, T$ (after washout), and let $y_1, \ldots, y_T$ be the corresponding target outputs. We model:

$$y_t = f(\mathbf{r}_t) + \varepsilon_t, \quad \varepsilon_t \sim \mathcal{N}(0, \sigma_n^2),$$

where $f \sim \mathcal{GP}(0, k)$ is a zero-mean GP prior on the readout function, and $\sigma_n^2$ is the noise variance.

The most common choice for the kernel in reservoir applications is the **linear kernel** (which recovers Bayesian ridge regression) or the **squared exponential (RBF) kernel**:

$$k_{SE}(\mathbf{r}, \mathbf{r}') = \sigma_f^2 \exp\!\left(-\frac{\|\mathbf{r} - \mathbf{r}'\|^2}{2\ell^2}\right).$$

More relevant for reservoir computing is the **ARD linear kernel**:

$$k_{ARD}(\mathbf{r}, \mathbf{r}') = \sum_{i=1}^N \frac{r_i r_i'}{l_i^2},$$

where $l_i > 0$ are per-dimension length scales. Neurons with small $l_i$ contribute strongly to the kernel (they are relevant); neurons with large $l_i$ are effectively ignored (they are irrelevant).

## 10.3.3 Bayesian Ridge Regression as a Special Case

Before deriving the full GP predictive distribution, let us see how Bayesian ridge regression fits in.

**Prior.** If we take $f(\mathbf{r}) = \mathbf{w}^\top \mathbf{r}$ with $\mathbf{w} \sim \mathcal{N}(\mathbf{0}, \sigma_w^2 I_N)$, then

$$k(\mathbf{r}, \mathbf{r}') = \mathbb{E}[\mathbf{w}^\top \mathbf{r} \cdot \mathbf{w}^\top \mathbf{r}'] = \sigma_w^2 \mathbf{r}^\top \mathbf{r}'.$$

This is a linear kernel! Bayesian linear regression with a Gaussian prior on the weights is equivalent to a GP with a linear kernel.

The correspondence: ridge regression parameter $\lambda$ corresponds to $\sigma_n^2 / \sigma_w^2$ in the GP framework. This immediately shows that GP regression *subsumes* ridge regression as a special case — but GP regression also provides the posterior distribution over $\mathbf{w}$, not just the MAP estimate.

## 10.3.4 The Predictive Distribution: Full Derivation

Let $R \in \mathbb{R}^{T \times N}$ be the state matrix (rows are reservoir states $\mathbf{r}_t^\top$) and $\mathbf{y} \in \mathbb{R}^T$ be the target vector. We want to predict the output $y^* = f(\mathbf{r}^*)$ at a new reservoir state $\mathbf{r}^*$.

**Step 1: Joint distribution.**

Under the GP model, the joint distribution of $(y_1, \ldots, y_T, y^*)$ is:

$$\begin{pmatrix} \mathbf{y} \\ y^* \end{pmatrix} \sim \mathcal{N}\!\left(\mathbf{0},\; \begin{pmatrix} K(R,R) + \sigma_n^2 I_T & \mathbf{k}(R, \mathbf{r}^*) \\ \mathbf{k}(\mathbf{r}^*, R) & k(\mathbf{r}^*, \mathbf{r}^*) \end{pmatrix}\right),$$

where $(K(R,R))_{ij} = k(\mathbf{r}_i, \mathbf{r}_j)$, and $\mathbf{k}(R, \mathbf{r}^*) = (k(\mathbf{r}_1, \mathbf{r}^*), \ldots, k(\mathbf{r}_T, \mathbf{r}^*))^\top \in \mathbb{R}^T$.

**Step 2: Conditional distribution.**

By the Schur complement formula for Gaussian conditionals (partitioned Gaussian identity):

$$p(y^* \mid \mathbf{y}) = \mathcal{N}(\mu^*, {\sigma^*}^2),$$

where

$$\mu^* = \mathbf{k}(\mathbf{r}^*, R)^\top \bigl(K(R,R) + \sigma_n^2 I\bigr)^{-1} \mathbf{y},$$

$${\sigma^*}^2 = k(\mathbf{r}^*, \mathbf{r}^*) - \mathbf{k}(\mathbf{r}^*, R)^\top \bigl(K(R,R) + \sigma_n^2 I\bigr)^{-1} \mathbf{k}(R, \mathbf{r}^*).$$

**Interpretation:**

- $\mu^*$ is the predictive mean — a weighted sum of training targets, with weights $\mathbf{k}(\mathbf{r}^*, R)^\top (K + \sigma_n^2 I)^{-1}$. Training points whose reservoir states are "similar" to $\mathbf{r}^*$ (as measured by the kernel $k$) receive higher weight.

- ${\sigma^*}^2$ is the predictive variance — the prior variance $k(\mathbf{r}^*, \mathbf{r}^*)$ minus the variance "explained" by the data. If $\mathbf{r}^*$ is very close to a training state (high kernel value), the variance is small (the model is confident). If $\mathbf{r}^*$ is far from all training states (low kernel values), the variance is close to the prior variance (the model knows it doesn't know).

**Step 3: Connection to ridge regression.**

For the linear kernel $k(\mathbf{r}, \mathbf{r}') = \sigma_w^2 \mathbf{r}^\top \mathbf{r}'$, the kernel matrix is $K(R,R) = \sigma_w^2 R R^\top$. Using the matrix inversion lemma (Woodbury identity):

$$(K + \sigma_n^2 I_T)^{-1} = (\sigma_w^2 R R^\top + \sigma_n^2 I_T)^{-1}.$$

Apply the identity $(A + UCV)^{-1} = A^{-1} - A^{-1}U(C^{-1} + VA^{-1}U)^{-1}VA^{-1}$ with $A = \sigma_n^2 I_T$, $U = R$, $C = \sigma_w^2 I_N$, $V = R^\top$:

$$(K + \sigma_n^2 I_T)^{-1} = \frac{1}{\sigma_n^2}\left(I_T - R(\sigma_n^2/\sigma_w^2 \cdot I_N + R^\top R)^{-1} R^\top\right).$$

The predictive mean becomes:

$$\mu^* = \sigma_w^2 \mathbf{r}^{*\top} R^\top \cdot \frac{1}{\sigma_n^2}\left(I_T - R\left(\frac{\sigma_n^2}{\sigma_w^2} I + R^\top R\right)^{-1} R^\top\right) \mathbf{y}$$

$$= \mathbf{r}^{*\top} \left(R^\top R + \frac{\sigma_n^2}{\sigma_w^2} I_N\right)^{-1} R^\top \mathbf{y} = \mathbf{r}^{*\top} \hat{\mathbf{w}},$$

where $\hat{\mathbf{w}} = (R^\top R + \lambda I)^{-1} R^\top \mathbf{y}$ is exactly the ridge regression solution with $\lambda = \sigma_n^2/\sigma_w^2$. The GP predictive mean coincides with ridge regression for the linear kernel. The GP provides, in addition, the predictive variance ${\sigma^*}^2$ — which ridge regression does not.

## 10.3.5 Automatic Relevance Determination (ARD)

The **ARD kernel** allows each reservoir neuron to have its own relevance:

$$k_{ARD}(\mathbf{r}, \mathbf{r}') = \sigma_f^2 \exp\!\left(-\frac{1}{2}\sum_{i=1}^N \frac{(r_i - r_i')^2}{l_i^2}\right).$$

Each length scale $l_i$ controls how much variation in neuron $i$'s activation changes the kernel value. Small $l_i$ means neuron $i$ is highly relevant (small differences in $r_i$ strongly affect the kernel). Large $l_i$ means neuron $i$ is irrelevant (it is effectively ignored by the kernel).

**Learning the ARD length scales.** The length scales $\{l_i\}$ are *hyperparameters of the GP* (not hyperparameters of the reservoir!) and are learned by maximizing the **marginal likelihood** (also called the log evidence):

$$\log p(\mathbf{y} \mid R, \boldsymbol{\theta}) = -\frac{1}{2}\mathbf{y}^\top (K + \sigma_n^2 I)^{-1} \mathbf{y} - \frac{1}{2}\log\det(K + \sigma_n^2 I) - \frac{T}{2}\log(2\pi),$$

where $\boldsymbol{\theta} = (l_1, \ldots, l_N, \sigma_f^2, \sigma_n^2)$ are the GP hyperparameters.

The marginal likelihood balances data fit (the first term: how well the model explains the training targets) against model complexity (the second term: the log-determinant of the kernel matrix, which penalizes models that use many effective parameters). This is an automatic Occam's razor.

**Gradient of the marginal likelihood.** To optimize $\boldsymbol{\theta}$, we need $\partial \log p(\mathbf{y}) / \partial \theta_j$ for each hyperparameter. Using the identity $\partial \log\det(A)/\partial\theta = \text{tr}(A^{-1} \partial A/\partial\theta)$:

$$\frac{\partial \log p(\mathbf{y})}{\partial \theta_j} = \frac{1}{2}\text{tr}\!\left(({\boldsymbol\alpha}{\boldsymbol\alpha}^\top - C^{-1})\frac{\partial K}{\partial\theta_j}\right),$$

where $C = K + \sigma_n^2 I$ and $\boldsymbol\alpha = C^{-1}\mathbf{y}$.

For the ARD kernel:

$$\frac{\partial K_{mn}}{\partial l_i} = K_{mn} \cdot \frac{(r_{m,i} - r_{n,i})^2}{l_i^3},$$

where $r_{m,i}$ is the $i$-th component of the $m$-th reservoir state. This gradient is computable in $O(T^2 N)$ time, given $C^{-1}$.

## 10.3.6 When GP Readouts Outperform Ridge Regression

The GP readout with a linear kernel is equivalent to Bayesian ridge regression and therefore has exactly the same predictive mean. The advantage of GP over ridge regression is:

1. **Uncertainty quantification.** If the task requires not just predictions but prediction intervals (e.g., for anomaly detection, safety-critical control), the GP provides calibrated uncertainty estimates that ridge regression cannot.

2. **Nonlinear kernels.** For the squared exponential or ARD kernel, the GP can capture nonlinear relationships between reservoir states and outputs that the linear readout misses. This is most beneficial when the reservoir is small (low $N$) and the relevant information is not fully linearized in the state space.

3. **Automatic hyperparameter optimization.** The marginal likelihood provides a principled way to set both the regularization and the relevance weights, without cross-validation.

4. **Sparse observations.** When the training set is small, the GP posterior is substantially different from ridge regression: the predictive variance is large, reflecting genuine uncertainty. Ridge regression in this regime is numerically unstable and overfit-prone; the GP degrades gracefully.

**When ridge regression is preferable:**
- Large training sets ($T > 10000$): GP inference costs $O(T^3)$ (due to the matrix inversion), while ridge regression costs $O(TN^2 + N^3)$. For large $T$ with $T \gg N$, ridge regression is much faster.
- Low noise: if $\sigma_n^2$ is small, the predictive variance is small regardless, and the uncertainty quantification advantage of GP is less important.
- The relationship between reservoir states and outputs is approximately linear: the GP with linear kernel gives no benefit over ridge regression, and the overhead of GP inference is wasted.

## 10.3.7 Computational Considerations

**Standard GP inference** costs $O(T^3)$ due to the Cholesky decomposition of the $T \times T$ kernel matrix. For typical reservoir experiments with $T = 5000$ training steps and $N = 100$ neurons, this is feasible.

**Sparse GP approximations** reduce the cost to $O(TM^2)$ where $M \ll T$ is the number of "inducing points" — representative training states that summarize the full dataset. The inducing points are optimized jointly with the GP hyperparameters. For reservoir computing, a natural choice of inducing points is a $k$-means clustering of the training states.

**Practical code sketch:**

```python
import numpy as np

def gp_predict(R_train, y_train, r_test, sigma_f=1.0, ell=1.0, sigma_n=0.1):
    """
    GP prediction with RBF kernel at test point r_test.
    R_train: (T, N) training states
    y_train: (T,) training targets
    r_test:  (N,) test state
    Returns: predictive mean, predictive variance
    """
    T, N = R_train.shape
    
    # Kernel function (RBF)
    def rbf(X, Y, sf=sigma_f, l=ell):
        # X: (m, N), Y: (n, N) -> (m, n) kernel matrix
        from scipy.spatial.distance import cdist
        sq_dists = cdist(X, Y, 'sqeuclidean')
        return sf**2 * np.exp(-0.5 * sq_dists / l**2)
    
    # Build training kernel matrix
    K_train = rbf(R_train, R_train)
    C = K_train + sigma_n**2 * np.eye(T)
    
    # Solve for alpha = C^{-1} y using Cholesky
    L = np.linalg.cholesky(C)
    alpha = np.linalg.solve(L.T, np.linalg.solve(L, y_train))
    
    # Test covariances
    k_test = rbf(R_train, r_test.reshape(1, -1)).ravel()  # (T,)
    k_star = rbf(r_test.reshape(1,-1), r_test.reshape(1,-1))[0,0]
    
    # Predictive mean and variance
    mu_star = k_test @ alpha
    v = np.linalg.solve(L, k_test)
    sigma2_star = k_star - v @ v
    
    return mu_star, max(0.0, sigma2_star)

def gp_marginal_likelihood(R_train, y_train, sigma_f, ell, sigma_n):
    """Compute log marginal likelihood for RBF GP."""
    T = R_train.shape[0]
    from scipy.spatial.distance import cdist
    sq_dists = cdist(R_train, R_train, 'sqeuclidean')
    K = sigma_f**2 * np.exp(-0.5 * sq_dists / ell**2) + sigma_n**2 * np.eye(T)
    
    L = np.linalg.cholesky(K)
    alpha = np.linalg.solve(L.T, np.linalg.solve(L, y_train))
    
    log_lik = -0.5 * y_train @ alpha
    log_lik -= np.sum(np.log(np.diag(L)))
    log_lik -= 0.5 * T * np.log(2 * np.pi)
    return log_lik
```

## 10.3.8 ARD for Reservoir Neuron Selection

The ARD kernel's per-neuron length scales $\{l_i\}$ act as a soft feature selector: after optimization, neurons with large $l_i$ are effectively ignored by the kernel. This is a form of *automatic feature selection* that identifies which reservoir neurons are most predictive for the task.

This has a direct connection to the capacity analysis of Chapter 7: neurons that are most informative about the target $y_t$ are precisely those whose activation carries the most capacity for the relevant target function. The ARD process is a Bayesian way of doing what the capacity analysis does analytically: identifying the informative dimensions of the reservoir state.

For a reservoir with $N = 200$ neurons trained on a task, the ARD will typically find that $20$–$50$ neurons are highly relevant (small $l_i$), while the rest are noise. This is unsurprising given the $MC \leq N$ bound: with $N = 200$, only a fraction of the capacity is used for any specific task. The ARD identifies which fraction.

---

*The GP readout provides principled uncertainty quantification and automatic feature selection — two capabilities the standard linear readout lacks. The next chapter moves from adapting the readout to adapting the reservoir itself, with FORCE learning.*
