# 14.2.1 Bagging: Bootstrap Aggregating and Variance Reduction

## The Bagging Procedure

Breiman introduced bagging (Bootstrap AGGregating) in 1996 [Breiman1996] as a general method for reducing the variance of any learning algorithm. The procedure is:

1. Given training data $\mathcal{D} = \{(\mathbf{x}_t, y_t)\}_{t=1}^T$, draw $M$ bootstrap samples $\mathcal{D}^{(1)}, \ldots, \mathcal{D}^{(M)}$, each of size $T$, by sampling $\mathcal{D}$ with replacement.
2. Train a predictor $\hat{f}^{(m)}$ on each bootstrap sample $\mathcal{D}^{(m)}$.
3. Combine predictions by averaging: $\hat{f}^{bag}(\mathbf{x}) = \frac{1}{M}\sum_{m=1}^M \hat{f}^{(m)}(\mathbf{x})$.

For regression with squared loss, the bagged predictor has lower variance than any individual predictor.

## The Variance Reduction Theorem

**Theorem (Breiman 1996).** Let $\hat{f}^{(1)}, \ldots, \hat{f}^{(M)}$ be predictors trained on independent datasets $\mathcal{D}^{(1)}, \ldots, \mathcal{D}^{(M)}$, each drawn from the same distribution. Let $\hat{f}^{avg} = \frac{1}{M}\sum_m \hat{f}^{(m)}$. Then:

$$\text{Var}[\hat{f}^{avg}(\mathbf{x})] \leq \text{Var}[\hat{f}^{(1)}(\mathbf{x})]$$

with equality if and only if the predictors are perfectly correlated: $\hat{f}^{(m)}(\mathbf{x}) = \hat{f}^{(m')}(\mathbf{x})$ almost surely for all $m \neq m'$.

**Proof.** Let $\hat{f}^{(m)}(\mathbf{x}) = \mu + e^{(m)}$, where $\mu = \mathbb{E}[\hat{f}^{(1)}(\mathbf{x})]$ is the common mean (same for all $m$ since datasets are i.i.d.) and $e^{(m)}$ is zero-mean error with $\mathbb{E}[(e^{(m)})^2] = V$. Then:

$$\text{Var}[\hat{f}^{avg}] = \text{Var}\!\left[\frac{1}{M}\sum_m (\mu + e^{(m)})\right] = \frac{1}{M^2}\text{Var}\!\left[\sum_m e^{(m)}\right]$$

$$= \frac{1}{M^2}\left(\sum_m \text{Var}[e^{(m)}] + \sum_{m \neq m'} \text{Cov}(e^{(m)}, e^{(m')})\right)$$

$$= \frac{1}{M^2}\left(MV + M(M-1)\rho V\right) = \frac{V}{M}\left(1 + (M-1)\rho\right)$$

where $\rho = \text{Corr}(e^{(m)}, e^{(m')})$ is the correlation between the errors of any two predictors. Since $\rho \leq 1$:

$$\text{Var}[\hat{f}^{avg}] = \frac{V}{M}(1 + (M-1)\rho) \leq \frac{V}{M} \cdot M = V = \text{Var}[\hat{f}^{(1)}]$$

As $M \to \infty$:

$$\text{Var}[\hat{f}^{avg}] \to \rho V$$

**The floor is $\rho V$, not zero.** This is the key insight: even with infinitely many predictors, the variance cannot be reduced below $\rho V$, the variance times the inter-predictor correlation. If all predictors make the same correlated error ($\rho = 1$), bagging provides no variance reduction. If all predictors are independent ($\rho = 0$), variance reduces as $V/M$, going to zero as $M \to \infty$. $\square$

## Bagging for Reservoir Computing

In the reservoir computing context, bagging takes on a slightly different form because:

1. The primary source of randomness is not the training data (which is fixed as a time series) but the **random reservoir initialization**: the random weight matrices $W^{rec}$ and $W^{in}$.

2. Different random seeds produce genuinely different reservoirs with different (but correlated) errors.

The bagged reservoir computer is therefore:

1. Generate $M$ independent random reservoirs $(W_m^{rec}, W_m^{in})$.
2. Run each reservoir on the same training data, obtaining states $X_m \in \mathbb{R}^{T \times N}$.
3. Train a readout $\hat{W}_m^{out}$ for each reservoir by ridge regression.
4. At test time, average predictions: $\hat{y}_t = \frac{1}{M}\sum_m \hat{W}_m^{out} \mathbf{x}_t^{(m)}$.

The variance reduction theorem applies: since different random reservoirs provide (approximately) independent readouts, the ensemble variance is $\approx \rho V / M + (1-\rho)V/M = V/M$ for small inter-predictor correlation $\rho$.

## Empirical Correlation Between Random Reservoirs

How correlated are the errors of two random reservoirs trained on the same task? This depends on:

- **Task complexity**: for very simple tasks (linear regression on a smooth target), all reservoirs tend to find similar solutions, giving $\rho \approx 1$ and little benefit from bagging.
- **Reservoir size**: smaller reservoirs have higher variance and lower bias, making their errors more diverse. Larger reservoirs have lower variance and higher bias, making ensemble benefits less pronounced.
- **Hyperparameter diversity**: reservoirs with different spectral radii, leaking rates, and densities tend to have lower inter-predictor correlation.

The general finding [Rodan2011, Lun2019] is that for moderately sized reservoirs ($N \approx 100$), the error correlation between two independent random reservoirs is typically $\rho \approx 0.3$–$0.6$ for challenging time series tasks. With $M = 10$ reservoirs, this gives a variance reduction by a factor of approximately $1 + (M-1)\rho)/M \approx 0.37$–$0.56$ relative to a single reservoir.

## Bootstrap Sampling for Reservoir Computing

The standard bootstrap (resampling training observations) does not directly apply to time series, because sequential observations are temporally correlated: resampling them independently breaks the temporal structure that the reservoir is designed to exploit.

For time series, the appropriate analog is the **block bootstrap**: randomly select $M$ contiguous blocks from the training sequence, each of length $T/M$, and concatenate them to form a bootstrap training series. This preserves short-range temporal correlations within each block while introducing variability across bootstrap samples.

Alternatively, and more commonly in RC practice, diversity is achieved through **reservoir randomness** rather than data resampling: use the same full training sequence for every ensemble member, but initialize each member with a different random seed. This is not technically bagging (which requires dataset variability) but achieves similar variance reduction through model variability.

---

## References

- [Breiman1996] Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.
- [Geman1992] Geman, S., Bienenstock, E., & Doursat, R. (1992). Neural networks and the bias/variance dilemma. *Neural Computation*, 4(1), 1–58.
- [Rodan2011] Rodan, A. & Tino, P. (2011). Minimum complexity echo state network. *IEEE Transactions on Neural Networks*, 22(1), 131–144.
- [Lun2019] Lun, S., Wang, S., Zhang, G., & Sheng, A. (2019). A new echo state network with variable memory length. *Information Sciences*, 370, 103–127.
