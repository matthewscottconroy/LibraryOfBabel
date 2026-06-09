# 20.1 Statistical Manifolds

## 20.1.1 Parametric Families

The starting point is simple: a family of probability distributions parametrized by a smooth parameter.

**Definition 20.1.1 (Statistical Model).** A *statistical model* (or *statistical manifold*) is a family of probability distributions $\mathcal{S} = \{p_\theta : \theta \in \Theta\}$ where $\Theta \subseteq \mathbb{R}^n$ is open and $\theta \mapsto p_\theta$ is smooth.

Examples:
- Gaussian family: $p_{\mu,\sigma^2} = N(\mu, \sigma^2)$, parametrized by $\theta = (\mu, \sigma^2) \in \mathbb{R} \times \mathbb{R}_{>0}$
- Exponential family: $p_\theta(x) = \exp(\theta \cdot T(x) - \psi(\theta)) h(x)$ for sufficient statistic $T$

The parameter $\theta$ gives coordinates on the manifold $\mathcal{S}$, and as $\theta$ varies smoothly, $p_\theta$ varies smoothly in some sense. But which metric makes this a Riemannian manifold? There are many possible choices — and the information-geometric answer is the Fisher information metric.

## 20.1.2 The Fisher Information Metric

The Fisher information matrix measures how sensitively the distribution changes as you move in parameter space. If $p_\theta$ and $p_{\theta + d\theta}$ are very different distributions, then a small change in $\theta$ carries a lot of "information" — distinguishing them from data is easy. If they are very similar, then even a large change in $\theta$ may be hard to detect from data.

**Definition 20.1.2 (Fisher Information Matrix).** The *Fisher information matrix* at $\theta \in \Theta$ is:
$$g_{ij}(\theta) = E_\theta\left[\frac{\partial \ell}{\partial \theta_i} \cdot \frac{\partial \ell}{\partial \theta_j}\right] = -E_\theta\left[\frac{\partial^2 \ell}{\partial \theta_i \partial \theta_j}\right],$$
where $\ell(\theta; x) = \log p_\theta(x)$ is the log-likelihood.

The two expressions are equal under mild regularity conditions (differentiation under the integral sign). The first says Fisher information is the covariance of the *score* (gradient of the log-likelihood). The second says it is the negative expected Hessian of the log-likelihood.

**Theorem 20.1.3.** The Fisher information matrix $g(\theta) = (g_{ij}(\theta))$ is positive semidefinite and defines a Riemannian metric on $\mathcal{S}$ (the *Fisher-Rao metric*).

This metric is canonical in a strong sense: it is the *unique* Riemannian metric on the space of distributions that is invariant under sufficient statistics (Chentsov's theorem). Any "reasonable" metric that doesn't change when you apply a sufficient statistic must be a multiple of the Fisher metric. This is the information-geometric analogue of Shannon's axiomatic uniqueness of entropy.

## 20.1.3 The Cramér-Rao Bound

The Fisher metric's most important application is a lower bound on statistical estimation:

**Theorem 20.1.4 (Cramér-Rao Bound).** For any unbiased estimator $\hat\theta(X_1, \ldots, X_n)$ of $\theta$ from $n$ i.i.d. samples:
$$\text{Cov}[\hat\theta] \geq \frac{1}{n} g(\theta)^{-1}$$
(in the matrix sense: $\text{Cov}[\hat\theta] - g^{-1}/n$ is positive semidefinite).

The Fisher metric sets a fundamental lower bound on how accurately any estimator can recover $\theta$ from data. No matter how clever your estimation algorithm, the variance of your estimate cannot be smaller than $g(\theta)^{-1}/n$.

This bound is tight: the maximum likelihood estimator achieves it asymptotically.

**Achievability (MLE Efficiency).** The maximum likelihood estimator (MLE) $\hat\theta_{\text{MLE}} = \arg\max_\theta \sum_{i=1}^n \log p_\theta(x_i)$ achieves the Cramér-Rao bound asymptotically:
$$\sqrt{n}(\hat\theta_{\text{MLE}} - \theta) \xrightarrow{d} N(0, g(\theta)^{-1}).$$

The MLE is asymptotically efficient: no unbiased estimator can have smaller asymptotic variance. In the language of information geometry, the MLE is the *$e$-projection* of the empirical distribution onto the parametric family — a concept we will make precise in Section 20.3.

The Cramér-Rao bound unifies a vast literature on statistical estimation theory. It applies to all parametric models, all sample sizes (with the right form), and gives the fundamental tradeoff between statistical accuracy and amount of data. Every time someone says "you need $n$ samples to estimate $\theta$ to precision $\varepsilon$", they are implicitly invoking the Cramér-Rao bound with $n \approx 1/(\varepsilon^2 g(\theta))$.
