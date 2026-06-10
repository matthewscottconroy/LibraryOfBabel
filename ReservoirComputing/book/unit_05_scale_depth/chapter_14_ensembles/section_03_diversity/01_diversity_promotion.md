# 14.3.1 Diversity Promotion and the Ambiguity Decomposition

## Why Diversity Matters

The variance reduction theorem shows that the benefit of an ensemble depends critically on the inter-predictor correlation $\rho$. When $\rho$ is small — when ensemble members make uncorrelated errors — the variance reduction is large. When $\rho$ is near 1 — when ensemble members all fail in the same way — the ensemble provides no benefit.

This creates the central challenge of ensemble design: **how do you create diverse predictors that generalize well individually and make different errors?** A trivially diverse ensemble — one member always predicts 0, one always predicts 1, one predicts randomly — has low correlation but terrible performance. Useful diversity must be *error diversity*: disagreements that are uncorrelated with the target, not disagreements that increase error.

The ambiguity decomposition formalizes this intuition precisely.

## The Ambiguity Decomposition

**Theorem (Krogh & Vedelsby 1995).** Let $\hat{f}^{(1)}, \ldots, \hat{f}^{(M)}$ be ensemble members with equal weights. Let $\hat{f}^{avg} = \frac{1}{M}\sum_m \hat{f}^{(m)}$. Define:

- **Ensemble error:** $E^{avg} = \mathbb{E}_{\mathbf{x}}[(y(\mathbf{x}) - \hat{f}^{avg}(\mathbf{x}))^2]$
- **Average member error:** $\bar{E} = \frac{1}{M}\sum_m \mathbb{E}_{\mathbf{x}}[(y(\mathbf{x}) - \hat{f}^{(m)}(\mathbf{x}))^2]$
- **Ambiguity:** $\bar{A} = \frac{1}{M}\sum_m \mathbb{E}_{\mathbf{x}}[(\hat{f}^{(m)}(\mathbf{x}) - \hat{f}^{avg}(\mathbf{x}))^2]$

Then:

$$E^{avg} = \bar{E} - \bar{A}$$

**Proof.** At each point $\mathbf{x}$, let $\bar{f} = \hat{f}^{avg}(\mathbf{x})$ and $y = y(\mathbf{x})$. Then:

$$\frac{1}{M}\sum_m (y - \hat{f}^{(m)})^2 = \frac{1}{M}\sum_m\left[(y - \bar{f}) + (\bar{f} - \hat{f}^{(m)})\right]^2$$

$$= (y - \bar{f})^2 + 2(y - \bar{f})\underbrace{\frac{1}{M}\sum_m(\bar{f} - \hat{f}^{(m)})}_{=0} + \frac{1}{M}\sum_m(\hat{f}^{(m)} - \bar{f})^2$$

Therefore:

$$(y - \bar{f})^2 = \frac{1}{M}\sum_m(y - \hat{f}^{(m)})^2 - \frac{1}{M}\sum_m(\hat{f}^{(m)} - \bar{f})^2$$

Integrating over $\mathbf{x}$:

$$E^{avg} = \bar{E} - \bar{A}$$

$\square$

**Interpretation.** The ensemble error equals the average member error minus the ambiguity. Ambiguity is the average squared disagreement among ensemble members — exactly the kind of diversity we want. The decomposition says:

1. Any method that increases ambiguity (makes members more diverse in their predictions) will reduce ensemble error, provided it doesn't simultaneously increase individual member errors.
2. You cannot increase ambiguity for free: there is a tradeoff between individual performance and diversity.
3. The "sweet spot" for ensemble design is to maximize $\bar{A}$ subject to the constraint that $\bar{E}$ remains bounded.

## Sources of Diversity in Random Reservoirs

Random reservoirs are naturally diverse because different random initializations produce genuinely different dynamical systems with different representational properties. The sources of diversity include:

**1. Spectral structure diversity.** Two random matrices $W_1^{rec}$ and $W_2^{rec}$ with the same spectral radius but different eigenvalue distributions have different memory curves, different mixing properties, and different sensitivities to various frequency components of the input. This is the primary source of diversity in standard ensembles of identically-parameterized reservoirs.

**2. Hyperparameter diversity (explicit).** Intentionally varying $\alpha$, $\rho_{target}$, and density $p$ across ensemble members creates reservoirs with genuinely different timescales and capacities. A reservoir with $\alpha = 0.1$ (slow, long memory) and one with $\alpha = 0.9$ (fast, short memory) will represent very different features of the same input, and their errors will be correspondingly less correlated.

**3. Input connectivity diversity.** Different random $W^{in}$ matrices project the input into different subspaces of the reservoir state space, producing different effective bases for temporal representation.

**4. Readout mask diversity.** One can train different ensemble members on different random subsets of reservoir neurons, amplifying diversity by restricting each member's view of the full state.

## Measuring Diversity: The Pairwise Correlation Matrix

A practical tool for assessing ensemble diversity is the $M \times M$ error correlation matrix $C$ where $C_{mn} = \text{Corr}(e^{(m)}, e^{(n)})$ and $e^{(m)}_t = y_t - \hat{y}_t^{(m)}$ is the error of member $m$ at time $t$.

An ideal ensemble has $C_{mn} \approx 0$ for $m \neq n$ (diagonal correlation matrix). In practice, for random reservoirs with the same hyperparameters, $C_{mn}$ is typically in the range $[0.3, 0.6]$.

The ensemble variance reduction factor is:

$$\frac{\text{Var}[\hat{f}^{avg}]}{\text{Var}[\hat{f}^{(1)}]} = \frac{1 + (M-1)\bar{\rho}}{M}$$

where $\bar{\rho} = \frac{1}{M(M-1)}\sum_{m \neq m'} C_{mn}$ is the average off-diagonal correlation. For $\bar{\rho} = 0.5$ and $M = 10$, this gives $\approx 0.55$ — a 45% variance reduction.

## Promoting Diversity Through Negative Correlation Learning

Negative correlation (NC) learning [Liu1999] is a training method that explicitly promotes diversity by adding a penalty term to the loss function of each ensemble member:

$$L^{(m)} = (y - \hat{f}^{(m)})^2 + \lambda \sum_{m' \neq m} ((\hat{f}^{(m)} - \hat{f}^{avg})(\hat{f}^{(m')} - \hat{f}^{avg}))$$

The second term penalizes ensemble members for making the same error in the same direction. In the reservoir context, NC learning can be applied during readout training (keeping reservoirs fixed) by adding a regularization term that encourages the readout weights to produce outputs that are uncorrelated with the outputs of other ensemble members. Since all readouts are linear, this is a convex optimization problem and can be solved efficiently.

---

## References

- [Krogh1995] Krogh, A. & Vedelsby, J. (1995). Neural network ensembles, cross validation, and active learning. In *Advances in Neural Information Processing Systems*, 7.
- [Breiman1996] Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.
- [Liu1999] Liu, Y. & Yao, X. (1999). Ensemble learning via negative correlation. *Neural Networks*, 12(10), 1399–1404.
- [Opitz1999] Opitz, D. & Maclin, R. (1999). Popular ensemble methods: An empirical study. *Journal of Artificial Intelligence Research*, 11, 169–198.
