# Stacking Reservoir Outputs

## The Stacking Concept

Simple averaging assigns equal weight to all ensemble members. Stacking (stacked generalization) instead trains a meta-learner to optimally combine the outputs of multiple base reservoirs [Wolpert 1992]. The meta-learner can learn that some base reservoirs are more reliable on certain input regimes, or that certain linear combinations reduce bias more effectively than equal weighting.

The stacking architecture consists of two levels. **Level 0:** $M$ heterogeneous base reservoirs $\{R_1, \ldots, R_M\}$, each trained on the full training set to produce predictions $\hat{y}^{(m)}_t$. **Level 1:** A meta-learner trained on the matrix of base predictions $[\hat{y}^{(1)}_t, \ldots, \hat{y}^{(M)}_t]^\top$ to produce the final prediction $\hat{y}_t$ [Wolpert 1992].

## Heterogeneous Base Reservoirs

The base reservoirs should differ substantially to provide complementary information. Standard choices for generating diversity include:

- **Spectral radius variation:** $R_m$ uses $\rho_m \in \{0.5, 0.7, 0.9, 0.95, 0.99\}$ — covering timescales from short-memory to long-memory.
- **Size variation:** $R_m$ uses $N_m \in \{50, 100, 200, 500, 1000\}$ — covering feature richness from compact to high-dimensional.
- **Input scaling variation:** $R_m$ uses $\sigma_m^{\text{in}} \in \{0.1, 0.5, 1.0, 2.0\}$ — covering operating regimes from near-linear to saturated nonlinear.
- **Architecture variation:** mix standard ESN, deep ESN (Section 13.2), and delay-line reservoirs.

Heterogeneous ensembles consistently outperform homogeneous ones because the base learners' failure modes are less correlated [Breiman 1996].

## The Meta-Learner

Let $\hat{\mathbf{z}}_t = [\hat{y}^{(1)}_t, \ldots, \hat{y}^{(M)}_t]^\top \in \mathbb{R}^M$ be the level-0 prediction vector. The level-1 meta-learner is a function $g : \mathbb{R}^M \to \mathbb{R}^{d_{\text{out}}}$. Standard choices are:

**Linear meta-learner:** $\hat{y}_t = \mathbf{v}^\top \hat{\mathbf{z}}_t + b$, trained by ridge regression on the out-of-fold predictions. This is a weighted average with learned weights $\mathbf{v}$, and includes simple averaging as the special case $\mathbf{v} = \frac{1}{M}\mathbf{1}$.

**Ridge regression meta-learner:** Same as linear but with $L_2$ regularization $\lambda_{\text{meta}}$, which prevents the meta-learner from over-relying on any single base reservoir.

**Small MLP meta-learner:** A two-layer network with $\sim 10$ hidden units, useful when the relationship between base predictions and the target is nonlinear (e.g., the target is better predicted by the product of two base predictions than their sum).

## Cross-Validation Stacking

A critical pitfall in stacking is overfitting: if the base reservoirs and meta-learner are both trained on the same data, the meta-learner can learn to identify which base reservoirs are overfitting and up-weight them — a systematic bias toward base reservoirs that memorize training data. The solution is cross-validation stacking [Wolpert 1992].

**Algorithm:**
1. Divide the training set into $K$ folds.
2. For each fold $k$: train each base reservoir $R_m$ on the remaining $K-1$ folds, produce out-of-fold predictions $\hat{y}^{(m)}_t$ for $t$ in fold $k$.
3. After all folds, collect the full $T \times M$ matrix of out-of-fold predictions $\hat{\mathbf{Z}}$.
4. Train the meta-learner on $\hat{\mathbf{Z}}$ vs. $\mathbf{Y}^*$.
5. Retrain each base reservoir on the full training set for final deployment.

The out-of-fold predictions are unbiased estimates of the base reservoirs' generalization errors, because each fold's predictions come from a reservoir that has not seen that fold during training. This prevents the meta-learner's training data from being contaminated by in-sample predictions [Wolpert 1992].

For time-series data, standard $K$-fold cross-validation may be inappropriate due to temporal dependencies. The recommended alternative is **time-series cross-validation** (a.k.a. expanding window validation): for fold $k$, train on $\{1, \ldots, t_k\}$ and validate on $\{t_k+1, \ldots, t_{k+1}\}$, preserving temporal order.

## Theoretical Guarantees

The stacking error bound provides a formal guarantee for linear meta-learners. Let $\epsilon_m$ be the MSE of base reservoir $m$ and let $\epsilon^*_m$ be the best-achievable MSE in the class $\mathcal{F}$ of meta-learners. Then for a linear meta-learner trained by ridge regression with parameter $\lambda$:

$$\mathbb{E}[\epsilon_{\text{stack}}] \leq \min_{m} \epsilon_m + O\!\left(\frac{M \log M}{T}\right).$$

That is, the stacking generalization error is asymptotically no worse than the best base reservoir, with a penalty of $O(M \log M / T)$ for the meta-learning overhead [Breiman 1996]. For $T \gg M \log M$ — easily satisfied in time-series applications — stacking is guaranteed to be at least as good as the best base reservoir.

---

## References

- Wolpert, D. H. (1992). Stacked generalization. *Neural Networks*, 5(2), 241–259.
- Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.
