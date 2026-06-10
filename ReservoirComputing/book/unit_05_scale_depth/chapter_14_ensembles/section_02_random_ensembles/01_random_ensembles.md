# Random Reservoir Ensembles

## Bagging and Reservoir Bagging

Bagging (Bootstrap AGGregating) trains each base learner on a bootstrap resample of the training data — a random sample of size $T$ drawn with replacement from the original $T$ training points [Breiman 1996]. Applied to reservoirs, bagging has a natural interpretation: draw $B \subset \{1, \ldots, T\}$ with replacement, form the sub-state matrix $\mathbf{X}_B$ and sub-target matrix $\mathbf{Y}_B^*$, and solve the ridge regression

$$\hat{\mathbf{W}}_m^{\text{out}} = (\mathbf{X}_B^\top \mathbf{X}_B + \lambda \mathbf{I})^{-1} \mathbf{X}_B^\top \mathbf{Y}_B^*.$$

Each bootstrap resample produces a different readout trained on a different subset of the time-series trajectory. Averaging $M$ such readouts reduces the readout variance while keeping the reservoir states fixed. The bootstrap samples decorrelate the readouts through their different training subsets.

However, for time-series data, naive bootstrap resampling destroys temporal structure: the bootstrap sample is not a valid time series, and the ridge regression readout trained on it may overfit temporal artifacts. The preferred method is **block bootstrap resampling**: divide the time series into contiguous blocks of length $b$, resample blocks with replacement, and concatenate them to form a bootstrap trajectory of length $\approx T$ [Breiman 1996]. Block size $b$ should be chosen to exceed the effective memory length of the reservoir.

## Random Subspace Method

The random subspace method (Ho 1998) trains each ensemble member on a random subset of input dimensions. For a reservoir with $d_{\text{in}}$-dimensional input, each base reservoir uses a randomly selected subset of $d' < d_{\text{in}}$ input dimensions. The reservoir receives $\mathbf{u}'_t = \mathbf{P}_m \mathbf{u}_t$, where $\mathbf{P}_m \in \{0,1\}^{d' \times d_{\text{in}}}$ is a random row-selector matrix.

This approach is most effective for high-dimensional inputs where individual input dimensions are weakly correlated. Different subspaces provide complementary views of the input, and the ensemble prediction $\hat{y} = \frac{1}{M} \sum_m \hat{y}_m$ combines these views. For $d_{\text{in}} = 100$ and $d' = 50$, the number of possible subsets is $\binom{100}{50} \approx 10^{29}$, far more than any feasible ensemble, so random selection effectively covers the space [Ho 1998].

## Random Hyperparameter Ensembles

A particularly natural ensembling strategy for reservoir computing is to draw hyperparameters from a prior distribution and average predictions across the resulting reservoirs. Let the hyperparameter vector be $\boldsymbol{\theta} = (\rho, \sigma_{\text{in}}, \alpha, N)$, with prior $p(\boldsymbol{\theta})$. For each $m = 1, \ldots, M$, draw $\boldsymbol{\theta}_m \sim p(\boldsymbol{\theta})$, construct the reservoir, and train the readout. The ensemble prediction is

$$\hat{y}_t = \frac{1}{M} \sum_{m=1}^M \hat{f}_m(\mathbf{u}_t; \boldsymbol{\theta}_m).$$

This is a Monte Carlo approximation to the Bayesian model average:

$$\hat{y}_t^{\text{Bayes}} = \int \hat{f}(\mathbf{u}_t; \boldsymbol{\theta}) p(\boldsymbol{\theta} \mid \mathcal{D}) \, d\boldsymbol{\theta},$$

which marginalizes over reservoir hyperparameters weighted by their posterior given data $\mathcal{D}$. For uniform priors, this reduces to averaging over the prior. The hyperparameter ensemble is robust to misspecification of any single hyperparameter setting and implicitly performs model selection through averaging [Breiman 1996].

## The Ambiguity Decomposition

Krogh & Vedelsby [1995] derived an elegant decomposition of ensemble error. Define the ensemble prediction $\bar{f} = \frac{1}{M}\sum_m f_m$ and the ambiguity of member $m$ as

$$a_m = (f_m - \bar{f})^2.$$

The ensemble ambiguity is the average $\bar{a} = \frac{1}{M}\sum_m a_m$. The key identity is:

$$E_{\text{ensemble}} = \bar{E} - \bar{a},$$

where $E_{\text{ensemble}} = (f^* - \bar{f})^2$ is the ensemble error, $\bar{E} = \frac{1}{M}\sum_m (f^* - f_m)^2$ is the average individual error, and $\bar{a} \geq 0$ is the average ambiguity. This holds pointwise (not just in expectation).

The decomposition has an important implication: the ensemble error is always less than the average individual error, by exactly the ensemble disagreement. To minimize ensemble error, one should maximize ambiguity (maximize disagreement among members) while keeping individual errors low. This motivates explicit diversity promotion [Krogh & Vedelsby 1995].

## Maximizing Diversity: Anti-Correlated Reservoirs

For reservoir ensembles, one strategy for maximizing ambiguity is to construct pairs of anti-correlated reservoirs. If $\mathbf{W}^{\text{rec}}_2 = -\mathbf{W}^{\text{rec}}_1$ (sign-flipped weights) and $\mathbf{W}^{\text{in}}_2 = -\mathbf{W}^{\text{in}}_1$, then under $\tanh$ nonlinearity, $\mathbf{x}_t^{(2)} = -\mathbf{x}_t^{(1)}$ (exactly anti-correlated). The ensemble of 2 such reservoirs has perfectly decorrelated outputs when the individual readouts have equal magnitude, achieving the maximum variance reduction of $\sigma^2/2$.

## Negative Correlation Learning

A more general approach is negative correlation learning, which adds an explicit penalty on inter-member correlation to the training objective [Krogh & Vedelsby 1995]:

$$\mathcal{L}_m = \|f^* - f_m\|^2 + \lambda_{\text{NCL}} (f_m - \bar{f}) \sum_{j \neq m} (f_j - \bar{f}).$$

The penalty term pushes each member's prediction away from the current ensemble mean, increasing ambiguity. The tradeoff parameter $\lambda_{\text{NCL}} \in [0, 1]$ balances individual accuracy against ensemble diversity.

---

## References

- Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.
- Krogh, A., & Vedelsby, J. (1995). Neural network ensembles, cross validation, and active learning. *Advances in Neural Information Processing Systems*, 7, 231–238.
- Ho, T. K. (1998). The random subspace method for constructing decision forests. *IEEE Transactions on Pattern Analysis and Machine Intelligence*, 20(8), 832–844.
