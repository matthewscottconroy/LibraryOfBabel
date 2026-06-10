# Section 8.8: Systematic Hyperparameter Optimization

## 8.8.1 The Optimization Problem

Given a validation loss $\mathcal{L}(\boldsymbol{\theta})$ — the error of a reservoir with hyperparameters $\boldsymbol{\theta} = (\rho, \sigma_{in}, \alpha, \lambda, \ldots)$ on a held-out validation sequence — the task is to find

$$\boldsymbol{\theta}^* = \arg\min_{\boldsymbol{\theta} \in \Theta} \mathcal{L}(\boldsymbol{\theta}),$$

where $\Theta$ is the hyperparameter space. Each evaluation of $\mathcal{L}(\boldsymbol{\theta})$ requires constructing and training a new reservoir, which takes from milliseconds (small $N$) to minutes (large $N$, long sequences). The challenge is to minimize the number of evaluations while finding a near-optimal configuration.

## 8.8.2 Grid Search

Grid search evaluates $\mathcal{L}$ on a Cartesian product of candidate values:

$$\Theta_{grid} = \{\rho_1, \ldots, \rho_G\} \times \{\sigma_1, \ldots, \sigma_G\} \times \cdots$$

with $G$ values per dimension and $d$ dimensions, requiring $G^d$ evaluations. For $G = 10$ and $d = 4$, this is $10{,}000$ evaluations. Grid search is simple and embarrassingly parallelizable.

**Failure mode.** Grid search assumes that the optimal hyperparameters are near a grid point. For continuous hyperparameter spaces with irregular optimal regions, the grid resolution required to guarantee near-optimality is exponential in $d$. Moreover, if the loss depends strongly on only $d_{eff} < d$ dimensions, grid search wastes $G^{d - d_{eff}}$ evaluations on the unimportant dimensions.

**When to use grid search:** For $d \leq 2$ primary hyperparameters when a coarse scan of the landscape is desired (e.g., plotting $\mathcal{L}(\rho, \sigma_{in})$ as a 2D heatmap for visualization and understanding).

## 8.8.3 Random Search

Random search [Bergstra2012] samples $k$ configurations independently and uniformly at random from $\Theta$:

$$\boldsymbol{\theta}_1, \ldots, \boldsymbol{\theta}_k \sim \text{Uniform}(\Theta).$$

**Competitive guarantee.** If the loss depends on only $d_{eff}$ of the $d$ dimensions, random search with $k$ samples finds a configuration within $\varepsilon$ of the optimum in those dimensions with probability $1 - \delta$ whenever $k \geq \log(\delta) / \log(1 - \varepsilon^{d_{eff}})$. For $d_{eff} = 1$, $\varepsilon = 0.05$, $\delta = 0.05$: $k \geq \log(0.05)/\log(0.95) \approx 59$ — independent of $d$.

This is the key advantage: **random search is efficient regardless of the number of hyperparameters**, as long as only a few matter. Bergstra and Bengio [Bergstra2012] demonstrated empirically that random search with 60 evaluations matches or outperforms grid search with $10^4$ evaluations on standard machine learning benchmarks.

**In practice.** Use log-uniform distributions for scale hyperparameters ($\rho$, $\sigma_{in}$, $\lambda$, which vary over orders of magnitude) and uniform distributions for bounded parameters ($\alpha \in (0, 1]$). For $N$ evaluations in budget, log-spacing is implicitly achieved by sampling $\log \rho \sim \text{Uniform}[\log \rho_{min}, \log \rho_{max}]$.

## 8.8.4 Bayesian Optimization

Bayesian optimization [Snoek2012] fits a probabilistic surrogate model to the observations $\{(\boldsymbol{\theta}_i, \mathcal{L}(\boldsymbol{\theta}_i))\}_{i=1}^k$ and uses the model to decide where to evaluate next. The surrogate is typically a *Gaussian process* (GP):

$$\mathcal{L}(\boldsymbol{\theta}) \sim \mathcal{GP}(\mu(\boldsymbol{\theta}),\, k_{GP}(\boldsymbol{\theta}, \boldsymbol{\theta}')),$$

where $\mu$ is the prior mean function (often constant) and $k_{GP}$ is the kernel (Matérn 5/2 or squared exponential are standard). After observing $\{(\boldsymbol{\theta}_i, \mathcal{L}_i)\}$, the posterior mean and variance at any new point $\boldsymbol{\theta}$ are:

$$\mu_{post}(\boldsymbol{\theta}) = \mathbf{k}^\top (K + \sigma^2 I)^{-1} \mathbf{y},$$
$$\sigma_{post}^2(\boldsymbol{\theta}) = k_{GP}(\boldsymbol{\theta}, \boldsymbol{\theta}) - \mathbf{k}^\top (K + \sigma^2 I)^{-1} \mathbf{k},$$

where $K_{ij} = k_{GP}(\boldsymbol{\theta}_i, \boldsymbol{\theta}_j)$ is the kernel gram matrix, $\mathbf{k}_i = k_{GP}(\boldsymbol{\theta}, \boldsymbol{\theta}_i)$, and $\mathbf{y} = (\mathcal{L}_1, \ldots, \mathcal{L}_k)$.

**Acquisition function.** The next evaluation point is chosen to maximize an *acquisition function* that balances exploration (evaluating in uncertain regions) and exploitation (evaluating near the current best):

$$\boldsymbol{\theta}_{k+1} = \arg\max_{\boldsymbol{\theta}} \text{EI}(\boldsymbol{\theta}),$$

where the *expected improvement* (EI) [MockusBayesianOpt] is:

$$\text{EI}(\boldsymbol{\theta}) = \mathbb{E}[\max(f^+ - \mathcal{L}(\boldsymbol{\theta}), 0)],$$

with $f^+ = \min_i \mathcal{L}_i$ the current best observed loss. For a GP surrogate:

$$\text{EI}(\boldsymbol{\theta}) = (f^+ - \mu_{post}(\boldsymbol{\theta}))\, \Phi\!\left(\frac{f^+ - \mu_{post}(\boldsymbol{\theta})}{\sigma_{post}(\boldsymbol{\theta})}\right) + \sigma_{post}(\boldsymbol{\theta})\, \phi\!\left(\frac{f^+ - \mu_{post}(\boldsymbol{\theta})}{\sigma_{post}(\boldsymbol{\theta})}\right),$$

where $\Phi$ and $\phi$ are the CDF and PDF of the standard normal. This closed-form expression is differentiable and can be maximized by gradient-based methods.

**Libraries.** Hyperopt [Bergstra2013] and Optuna [Akiba2019] provide practical Bayesian optimization (and tree-structured Parzen estimator variants) with simple Python interfaces. Scikit-optimize provides GP-based Bayesian optimization.

**When Bayesian optimization pays off:** For $d \leq 8$ hyperparameters with expensive evaluations (>10 seconds each) and a total budget of 50-200 evaluations. For very cheap evaluations (milliseconds), random search is competitive.

## 8.8.5 BOHB: Bayesian Optimization with Hyperband

BOHB [Falkner2018] combines Bayesian optimization with Hyperband early stopping:

1. Sample candidate configurations using a kernel density estimator-based Bayesian model (the Tree-structured Parzen Estimator).
2. Evaluate candidates on progressively larger training budgets (Hyperband: start with small $T$, double until full budget).
3. Terminate poor-performing configurations early (Hyperband's successive halving) to save compute.

BOHB is particularly effective for reservoir computing because performance on a short sequence (small $T$) is a reasonable predictor of performance on the full sequence (large $T$), and early termination saves evaluation cost. The Bayesian component ensures that the configurations worth running to completion are explored efficiently.

## 8.8.6 CMA-ES: Evolutionary Strategy

The Covariance Matrix Adaptation Evolution Strategy (CMA-ES) [HansenOstermeier2001] is a derivative-free optimization algorithm well-suited to hyperparameter spaces of moderate dimension ($d \leq 20$):

1. Maintain a multivariate Gaussian distribution $\mathcal{N}(\mathbf{m}, \sigma^2 C)$ over hyperparameters, where $\mathbf{m}$ is the current mean, $\sigma$ the step size, and $C$ the covariance matrix.
2. Sample $\lambda_{pop}$ candidate configurations from this distribution.
3. Evaluate each candidate and rank by performance.
4. Update $\mathbf{m}$, $\sigma$, and $C$ using the top $\mu < \lambda_{pop}$ performers.

The key innovation of CMA-ES is the adaptive covariance $C$: it learns the correlation structure of the loss landscape, effectively rotating the search distribution to align with the loss contours. This allows CMA-ES to handle ill-conditioned landscapes where hyperparameters have complex interactions.

For reservoir hyperparameter optimization, CMA-ES typically converges in $O(d^2)$ evaluations — competitive with Bayesian optimization for moderate $d$ and without the $O(k^3)$ cost of GP fitting. It is the preferred method for evolutionary approaches to reservoir design, where the fitness landscape is smooth in hyperparameter space but may have variable conditioning.

## 8.8.7 Summary and Recommendations

| Method | Best Use | # Evaluations | Key Limitation |
|---|---|---|---|
| Grid search | $d \leq 2$, visualization | $G^d$ | Exponential cost |
| Random search | $d \leq 6$, fast eval | 50-100 | No sequential learning |
| Bayesian opt. (GP) | $d \leq 8$, slow eval | 30-200 | $O(k^3)$ GP fitting |
| BOHB | $d \leq 10$, variable budget | 50-500 | Complex implementation |
| CMA-ES | $d \leq 20$, moderate eval | $O(d^2)$ | No surrogate reuse |

For a first exploration: random search with 60-100 evaluations. For refinement: Bayesian optimization (Optuna) starting from the best random search configuration. For large $d$ or evolutionary reservoir design: CMA-ES.

---

## References

- **[Akiba2019]** T. Akiba, S. Sano, T. Yanase, T. Ohta, and M. Koyama. "Optuna: A next-generation hyperparameter optimization framework." *KDD*, pp. 2623-2631, 2019.
- **[Bergstra2012]** J. Bergstra and Y. Bengio. "Random search for hyper-parameter optimization." *Journal of Machine Learning Research*, 13:281-305, 2012.
- **[Bergstra2013]** J. Bergstra, D. Yamins, and D. D. Cox. "Making a science of model search: Hyperparameter optimization in hundreds of dimensions for vision architectures." *ICML*, 2013.
- **[Falkner2018]** S. Falkner, A. Klein, and F. Hutter. "BOHB: Robust and efficient hyperparameter optimization at scale." *ICML*, 2018.
- **[HansenOstermeier2001]** N. Hansen and A. Ostermeier. "Completely derandomized self-adaptation in evolution strategies." *Evolutionary Computation*, 9(2):159-195, 2001.
- **[MockusBayesianOpt]** J. Mockus. *Bayesian Approach to Global Optimization*. Kluwer Academic Publishers, 1989.
- **[Snoek2012]** J. Snoek, H. Larochelle, and R. P. Adams. "Practical Bayesian optimization of machine learning algorithms." *NIPS*, 2012.
