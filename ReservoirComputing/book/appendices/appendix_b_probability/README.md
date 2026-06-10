# Appendix B: Probability and Statistics Reference

This appendix develops the probabilistic tools needed for Bayesian interpretations of readout training, cross-validation in RC experiments, and concentration inequalities for finite-sample guarantees.

---

## B.1 Gaussian Random Vectors

### Multivariate Gaussian

A random vector $\mathbf{x} \in \mathbb{R}^d$ has a **multivariate Gaussian distribution** $\mathbf{x} \sim \mathcal{N}(\boldsymbol{\mu}, \Sigma)$ if its density is:

$$p(\mathbf{x}) = \frac{1}{(2\pi)^{d/2}|\Sigma|^{1/2}} \exp\left(-\frac{1}{2}(\mathbf{x} - \boldsymbol{\mu})^\top \Sigma^{-1}(\mathbf{x} - \boldsymbol{\mu})\right)$$

where $\boldsymbol{\mu} = \mathbb{E}[\mathbf{x}] \in \mathbb{R}^d$ is the mean and $\Sigma = \mathbb{E}[(\mathbf{x}-\boldsymbol{\mu})(\mathbf{x}-\boldsymbol{\mu})^\top] \in \mathbb{R}^{d \times d}$ is the positive-definite covariance matrix.

### Key Properties

**Linear transformations**: If $\mathbf{x} \sim \mathcal{N}(\boldsymbol{\mu}, \Sigma)$ and $\mathbf{y} = A\mathbf{x} + \mathbf{b}$, then $\mathbf{y} \sim \mathcal{N}(A\boldsymbol{\mu} + \mathbf{b}, A\Sigma A^\top)$.

**Marginals**: For a jointly Gaussian vector partitioned as $\mathbf{x} = (\mathbf{x}_1, \mathbf{x}_2)$ with:

$$\begin{pmatrix}\mathbf{x}_1 \\ \mathbf{x}_2\end{pmatrix} \sim \mathcal{N}\left(\begin{pmatrix}\boldsymbol{\mu}_1 \\ \boldsymbol{\mu}_2\end{pmatrix}, \begin{pmatrix}\Sigma_{11} & \Sigma_{12} \\ \Sigma_{21} & \Sigma_{22}\end{pmatrix}\right)$$

the marginal of $\mathbf{x}_1$ is $\mathbf{x}_1 \sim \mathcal{N}(\boldsymbol{\mu}_1, \Sigma_{11})$.

**Conditionals**: The conditional distribution is:

$$\mathbf{x}_1 | \mathbf{x}_2 \sim \mathcal{N}\left(\boldsymbol{\mu}_1 + \Sigma_{12}\Sigma_{22}^{-1}(\mathbf{x}_2 - \boldsymbol{\mu}_2),\ \Sigma_{11} - \Sigma_{12}\Sigma_{22}^{-1}\Sigma_{21}\right)$$

The conditional mean $\boldsymbol{\mu}_{1|2} = \boldsymbol{\mu}_1 + \Sigma_{12}\Sigma_{22}^{-1}(\mathbf{x}_2 - \boldsymbol{\mu}_2)$ is linear in $\mathbf{x}_2$ — the Gaussian conditional mean is the best linear predictor.

### Gaussian Random Weights in RC

The standard ESN initialization draws reservoir weights $W \sim \mathcal{N}(0, \sigma_W^2)$ (i.i.d., then scaled to target spectral radius). Input weights $W_{\text{in}} \sim \mathcal{N}(0, \sigma_{\text{in}}^2)$. Under this initialization, for a fixed input $\mathbf{u}$, the reservoir state $\mathbf{x} = \tanh(W\mathbf{x}_{\text{prev}} + W_{\text{in}}\mathbf{u})$ has a distribution that can be analyzed using random matrix theory (Chapter 6).

---

## B.2 Bayesian Linear Regression — Full Derivation

Bayesian linear regression provides a probabilistic treatment of the readout training problem, yielding both a point estimate and uncertainty quantification.

### Model

**Data**: $\{(\mathbf{x}_t, y_t)\}_{t=1}^T$ where $\mathbf{x}_t \in \mathbb{R}^N$ is the reservoir state and $y_t \in \mathbb{R}$ is the target.

**Likelihood**: $y_t | \mathbf{x}_t, \mathbf{w} \sim \mathcal{N}(\mathbf{w}^\top\mathbf{x}_t, \beta^{-1})$, i.e., $y_t = \mathbf{w}^\top\mathbf{x}_t + \epsilon_t$ with $\epsilon_t \sim \mathcal{N}(0, \beta^{-1})$. Here $\beta$ is the noise precision.

**Prior**: $\mathbf{w} \sim \mathcal{N}(\mathbf{0}, \alpha^{-1}I)$ where $\alpha$ is the weight precision hyperparameter.

### Posterior Distribution

The posterior $p(\mathbf{w} | \{y_t\}, \{x_t\})$ is proportional to the likelihood times the prior:

$$p(\mathbf{w} | Y, X) \propto p(Y | X, \mathbf{w}) \cdot p(\mathbf{w})$$

where $X = [\mathbf{x}_1, \ldots, \mathbf{x}_T]^\top \in \mathbb{R}^{T \times N}$ is the design matrix and $Y = [y_1, \ldots, y_T]^\top$.

**Step 1**: Write the log posterior:

$$\log p(\mathbf{w} | Y, X) = -\frac{\beta}{2}\sum_t (y_t - \mathbf{w}^\top\mathbf{x}_t)^2 - \frac{\alpha}{2}\|\mathbf{w}\|^2 + \text{const}$$

$$= -\frac{1}{2}\mathbf{w}^\top(\beta X^\top X + \alpha I)\mathbf{w} + \beta\mathbf{w}^\top X^\top Y + \text{const}$$

**Step 2**: Complete the square. Let $\Lambda_N = \beta X^\top X + \alpha I$ (posterior precision matrix). The log posterior is:

$$-\frac{1}{2}(\mathbf{w} - \mathbf{m}_N)^\top \Lambda_N (\mathbf{w} - \mathbf{m}_N) + \text{const}$$

where the **posterior mean** is:

$$\mathbf{m}_N = \Lambda_N^{-1}\beta X^\top Y = (\beta X^\top X + \alpha I)^{-1}\beta X^\top Y$$

**Step 3**: Read off the posterior:

$$\mathbf{w} | Y, X \sim \mathcal{N}(\mathbf{m}_N, \Lambda_N^{-1})$$

### Connection to Ridge Regression

Compare $\mathbf{m}_N$ to the ridge regression solution:

$$\hat{\mathbf{w}}_{\text{ridge}} = (X^\top X + \lambda I)^{-1}X^\top Y$$

Setting $\lambda = \alpha/\beta$ (prior-to-noise ratio), we have $\mathbf{m}_N = \hat{\mathbf{w}}_{\text{ridge}}$. The Bayesian posterior mean **is** the ridge regression estimate. The ridge regression is exactly Bayesian linear regression with Gaussian prior and likelihood — a fact that justifies the probabilistic interpretation of the regularization parameter: $\lambda = \alpha/\beta$ is the ratio of weight precision to noise precision.

### Predictive Distribution

For a new input $\mathbf{x}_*$, the predictive distribution is:

$$y_* | \mathbf{x}_*, Y, X \sim \mathcal{N}(\mathbf{m}_N^\top\mathbf{x}_*, \sigma_*^2)$$

where:

$$\sigma_*^2 = \beta^{-1} + \mathbf{x}_*^\top \Lambda_N^{-1} \mathbf{x}_*$$

The predictive variance has two components: $\beta^{-1}$ (irreducible noise) and $\mathbf{x}_*^\top\Lambda_N^{-1}\mathbf{x}_*$ (uncertainty from limited data). The latter decreases as $T \to \infty$ at a rate determined by the design matrix.

**RC Application**: The predictive variance $\sigma_*^2$ gives a principled uncertainty estimate for reservoir-based predictions — useful for anomaly detection (Chapter 22) and active learning.

### Marginal Likelihood (Evidence) for Hyperparameter Optimization

The marginal likelihood (integrating over $\mathbf{w}$):

$$p(Y | X, \alpha, \beta) = \int p(Y | X, \mathbf{w}, \beta)p(\mathbf{w} | \alpha) \, d\mathbf{w}$$

has a closed form for Gaussian likelihood and prior:

$$\log p(Y | X, \alpha, \beta) = \frac{N}{2}\log\alpha + \frac{T}{2}\log\beta - \frac{1}{2}\log|\Lambda_N| - \frac{\beta}{2}\|Y - X\mathbf{m}_N\|^2 - \frac{\alpha}{2}\|\mathbf{m}_N\|^2 - \frac{T}{2}\log(2\pi)$$

Maximizing this over $(\alpha, \beta)$ — **evidence optimization** or **empirical Bayes** — gives principled hyperparameter selection without cross-validation (Chapter 9).

---

## B.3 Cross-Validation Methods

### k-Fold Cross-Validation

The data is divided into $k$ equal-sized folds $\{F_1, \ldots, F_k\}$. For each fold $j$:
1. Train on $\{F_i : i \neq j\}$ (size $\approx (1 - 1/k)T$)
2. Evaluate on $F_j$

The $k$-fold CV estimate of generalization error is:

$$\hat{E}_{\text{CV}} = \frac{1}{k}\sum_{j=1}^k \hat{E}_j$$

**Bias-variance trade-off**: 
- Small $k$ (e.g., $k=2$): more bias (each model sees less data), less variance
- Large $k$ (e.g., $k=T$, LOO): less bias, more variance and computational cost

Standard choice: $k = 5$ or $k = 10$.

### Leave-One-Out Cross-Validation (LOO-CV)

LOO-CV ($k = T$) trains on all but one observation and evaluates on the left-out one, repeating $T$ times:

$$\hat{E}_{\text{LOO}} = \frac{1}{T}\sum_{t=1}^T (y_t - \hat{y}_{t,-t})^2$$

where $\hat{y}_{t,-t}$ is the prediction for $y_t$ from the model trained on all data except observation $t$.

**The LOO shortcut for ridge regression**: Instead of re-training $T$ times, LOO error can be computed from a single model fit using the **hat matrix** $H = X(X^\top X + \lambda I)^{-1}X^\top$:

$$y_t - \hat{y}_{t,-t} = \frac{y_t - \hat{y}_t}{1 - H_{tt}}$$

where $\hat{y}_t = H_{tt}y_t + \sum_{s \neq t} H_{ts}y_s$ is the in-sample prediction. This gives:

$$\hat{E}_{\text{LOO}} = \frac{1}{T}\sum_t \left(\frac{y_t - \hat{y}_t}{1 - H_{tt}}\right)^2$$

computable in $O(TN^2)$ (cost of one model fit) instead of $O(T^2 N^2)$ (cost of $T$ model fits).

### Generalized Cross-Validation (GCV) — Derivation

The GCV approximation replaces $H_{tt}$ (which varies by observation) with its average $\text{tr}(H)/T$:

$$\hat{E}_{\text{GCV}}(\lambda) = \frac{1}{T}\sum_t \left(\frac{y_t - \hat{y}_t}{1 - \text{tr}(H)/T}\right)^2 = \frac{\|Y - X\hat{\mathbf{w}}\|^2/T}{\left(1 - \text{tr}(H)/T\right)^2}$$

where $\hat{\mathbf{w}} = (X^\top X + \lambda I)^{-1}X^\top Y$ is the ridge estimate.

**Derivation**: The trace of the hat matrix is:

$$\text{tr}(H) = \text{tr}(X(X^\top X + \lambda I)^{-1}X^\top) = \sum_{i=1}^N \frac{\sigma_i^2}{\sigma_i^2 + \lambda}$$

where $\sigma_1, \ldots, \sigma_N$ are the singular values of $X$ (from its SVD $X = U\Sigma V^\top$). The quantity $\text{tr}(H)$ is the **effective degrees of freedom** of the ridge regression fit: it ranges from 0 (when $\lambda \to \infty$) to $N$ (when $\lambda \to 0$).

The GCV formula is:

$$\hat{E}_{\text{GCV}}(\lambda) = \frac{\frac{1}{T}\|Y - \hat{Y}_\lambda\|^2}{\left(1 - \frac{d_{\text{eff}}(\lambda)}{T}\right)^2}$$

where $d_{\text{eff}}(\lambda) = \sum_i \sigma_i^2/(\sigma_i^2 + \lambda)$ and $\hat{Y}_\lambda = X\hat{\mathbf{w}}_\lambda$ is the fitted vector.

**Properties of GCV**:
1. Rotationally invariant: $\hat{E}_{\text{GCV}}$ depends on $X$ only through its singular values.
2. Asymptotically equivalent to LOO-CV for ridge regression under mild conditions.
3. Computationally efficient: $O(TN)$ once the SVD of $X$ is available.
4. Achieves near-optimal $\lambda$ selection.

**RC Application**: GCV provides fast hyperparameter selection for the ridge regression regularization parameter $\lambda$ in reservoir readout training. On a ridge regression path (varying $\lambda$ on a grid), GCV evaluation costs $O(TN)$ per $\lambda$ value after a one-time SVD of $X$ — far cheaper than $k$-fold CV.

---

## B.4 Concentration Inequalities

Concentration inequalities bound the probability that a random variable deviates from its expectation by more than a specified amount. They are the mathematical tools for finite-sample learning theory.

### Hoeffding's Inequality

**Setting**: $X_1, \ldots, X_n$ are independent random variables with $a_i \leq X_i \leq b_i$ almost surely.

**Statement**: For any $t > 0$:

$$P\left(\sum_{i=1}^n X_i - \sum_{i=1}^n \mathbb{E}[X_i] \geq t\right) \leq \exp\left(-\frac{2t^2}{\sum_{i=1}^n(b_i-a_i)^2}\right)$$

**Proof sketch**: The key step is Hoeffding's lemma: for $a \leq X \leq b$ with $\mathbb{E}[X] = 0$, for any $s \in \mathbb{R}$:

$$\mathbb{E}[e^{sX}] \leq \exp\left(\frac{s^2(b-a)^2}{8}\right)$$

This follows from the convexity of $e^{sx}$: since $a \leq X \leq b$, we can write $X = \theta b + (1-\theta) a$ for $\theta = (X-a)/(b-a) \in [0,1]$, giving:

$$e^{sX} \leq \theta e^{sb} + (1-\theta)e^{sa}$$

Taking expectations and using $\mathbb{E}[X] = 0$ (so $\mathbb{E}[\theta] = -a/(b-a)$):

$$\mathbb{E}[e^{sX}] \leq \frac{-a}{b-a}e^{sb} + \frac{b}{b-a}e^{sa} \triangleq \phi(s(b-a))$$

A Taylor expansion of $\phi$ at 0 shows $\phi(u) \leq e^{u^2/8}$, giving Hoeffding's lemma. Applying the Markov inequality to $e^{s\sum X_i}$ and optimizing over $s$ gives the full bound. $\square$

**RC Application**: Bound on training error vs. generalization error: if the per-sample loss $\ell_t \in [0, B]$ (bounded), then the training error concentrates around the expected loss:

$$P(|\hat{E}_{\text{train}} - E_{\text{gen}}| \geq \epsilon) \leq 2\exp\left(-\frac{2T\epsilon^2}{B^2}\right)$$

For $B = 1$ (normalized loss), to achieve $\epsilon = 0.05$ with probability $\geq 0.95$: $T \geq \frac{\ln(40)}{2(0.05)^2} \approx 740$ training samples.

### Bernstein's Inequality

Bernstein's inequality improves on Hoeffding when the variance of $X_i$ is small relative to its range.

**Setting**: Independent $X_1, \ldots, X_n$ with $\mathbb{E}[X_i] = 0$, $|X_i| \leq M$ almost surely, and $\text{Var}(X_i) \leq \sigma_i^2$.

**Statement**:

$$P\left(\sum_{i=1}^n X_i \geq t\right) \leq \exp\left(-\frac{t^2/2}{\sum_{i=1}^n \sigma_i^2 + Mt/3}\right)$$

**Proof sketch**: Bernstein's condition: $\mathbb{E}[|X_i|^k] \leq \frac{k!}{2} \sigma_i^2 M^{k-2}$ for all $k \geq 2$. This follows from $|X_i| \leq M$. Using the moment generating function bound derived from this condition:

$$\mathbb{E}[e^{sX_i}] \leq \exp\left(\frac{\sigma_i^2 s^2 / 2}{1 - Ms/3}\right) \text{ for } s < 3/M$$

Applying Markov's inequality to $e^{s\sum X_i}$ and optimizing $s = t/(\sum\sigma_i^2 + Mt/3)$ gives the Bernstein bound. $\square$

**Comparison with Hoeffding**: Bernstein's bound is tighter when $t \ll \sum\sigma_i^2 / M$ (small deviation regime), and matches Hoeffding's bound in the large-deviation regime. For RC analysis where $\sigma_i^2 \ll M^2$ (the loss has small variance), Bernstein gives significantly better constants.

**RC Application**: Bernstein's inequality is used to analyze the concentration of the empirical ridge regression loss, accounting for the variance of the loss (which is smaller than its range). This gives tighter sample complexity bounds for reservoir readout training than Hoeffding alone.

### McDiarmid's Inequality (Bounded Differences)

For a function $f: \mathcal{X}^n \to \mathbb{R}$ satisfying the **bounded differences condition**:

$$\sup_{x_1,\ldots,x_n,x_i'} |f(x_1,\ldots,x_i,\ldots,x_n) - f(x_1,\ldots,x_i',\ldots,x_n)| \leq c_i$$

for independent inputs $X_1, \ldots, X_n$:

$$P(f(X_1,\ldots,X_n) - \mathbb{E}[f] \geq t) \leq \exp\left(-\frac{2t^2}{\sum_i c_i^2}\right)$$

This is a generalization of Hoeffding to functions of multiple random variables. It is used to bound the generalization error of reservoir learning algorithms as a function of the training data.
