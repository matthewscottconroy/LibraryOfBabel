# Why Ensemble Multiple Reservoirs?

## The Single-Reservoir Variance Problem

A single reservoir is a random object: its weights $\mathbf{W}^{\text{rec}}$ are drawn once from a random distribution and fixed for all time. Different random seeds produce qualitatively different reservoirs — different spectral properties, different input projections, different sets of computable features. The trained readout is a linear combination of the features offered by this particular random realization. If the reservoir happens to be poorly matched to the task (inadequate timescale, poor separation of relevant input classes), the readout cannot compensate, and prediction error is high. A different random seed might yield a much better reservoir.

This sensitivity to random initialization is the fundamental source of variance in reservoir computing. The estimator $\hat{f}_1(\mathbf{u})$ produced by one reservoir is a high-variance approximation of the true target function $f^*(\mathbf{u})$, and averaging multiple independent estimates can dramatically reduce this variance.

## The Bias-Variance Decomposition for Ensembles

Let $\hat{f}_1, \hat{f}_2, \ldots, \hat{f}_M$ be $M$ independent reservoir predictors trained on the same dataset. Define the ensemble prediction as the simple average:

$$\hat{f}^{\text{avg}}(\mathbf{u}) = \frac{1}{M} \sum_{m=1}^M \hat{f}_m(\mathbf{u}).$$

The expected squared error decomposes as follows. For any test point $\mathbf{u}$:

$$\mathbb{E}\!\left[(\hat{f}^{\text{avg}}(\mathbf{u}) - f^*(\mathbf{u}))^2\right] = \text{Bias}^2 + \frac{1}{M} \text{Var}(\hat{f}_1) + \frac{M-1}{M} \text{Cov}(\hat{f}_1, \hat{f}_2).$$

This decomposition, credited to Geman et al. [1992], is derived as follows. Write $\hat{f}^{\text{avg}} = f^* + (\bar{f} - f^*) + (\hat{f}^{\text{avg}} - \bar{f})$, where $\bar{f} = \mathbb{E}[\hat{f}_m]$ is the mean predictor. Then

$$\text{MSE}(\hat{f}^{\text{avg}}) = \underbrace{(\bar{f} - f^*)^2}_{\text{Bias}^2} + \underbrace{\mathbb{E}[(\hat{f}^{\text{avg}} - \bar{f})^2]}_{\text{Variance of average}}.$$

For the variance term, since $\hat{f}^{\text{avg}} - \bar{f} = \frac{1}{M}\sum_m (\hat{f}_m - \bar{f})$:

$$\mathbb{E}[(\hat{f}^{\text{avg}} - \bar{f})^2] = \frac{1}{M^2} \sum_{m,m'} \text{Cov}(\hat{f}_m, \hat{f}_{m'}) = \frac{1}{M}\sigma^2 + \frac{M-1}{M} \rho \sigma^2,$$

where $\sigma^2 = \text{Var}(\hat{f}_m)$ and $\rho = \text{Cor}(\hat{f}_m, \hat{f}_{m'})$ for $m \neq m'$ [Breiman 1996]. Thus

$$\text{Var}(\hat{f}^{\text{avg}}) = \sigma^2 \left(\frac{1}{M} + \frac{M-1}{M} \rho\right).$$

**Key result:** If the base predictors are independent ($\rho = 0$), the ensemble variance is $\sigma^2/M$ — variance reduces by exactly $M$. If they are positively correlated, the gain is smaller. If they are negatively correlated ($\rho < 0$), the ensemble variance is below $\sigma^2/M$ — a superlinear reduction [Breiman 1996].

## When Ensembling is Most Useful

The variance reduction is largest when (1) individual reservoirs have high variance (small $N$, large effective spectral radius, noisy data) and (2) the base reservoirs are mutually uncorrelated (different random seeds produce independent features). The bias is not reduced by averaging: if all reservoirs are systematically biased (e.g., they all lack sufficient memory to capture the relevant history), averaging will not help.

Ensembling is therefore most useful when:
- $N$ is small relative to task complexity, making single-reservoir variance large.
- Training data is limited, making the learned readout itself noisy.
- The target function is complex and the random reservoir occasionally fails to provide adequate features.

It is less useful when:
- $N$ is large enough that any single reservoir achieves low variance.
- All reservoirs share a common failure mode (systematic bias).
- The computational cost $M \times$ (single reservoir cost) is prohibitive.

## Computational Cost

The ensemble cost scales linearly in $M$: state collection is $O(M N^2 T)$ and ridge regression is $O(M N^3)$. For $M = 10$ and $N = 500$, this is a 10× increase over the single-reservoir baseline. Since the individual reservoirs are independent, ensembles are embarrassingly parallelizable, and the wall-clock time can be kept to a single-reservoir level with $M$ processors [Breiman 1996].

---

## References

- Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.
- Geman, S., Bienenstock, E., & Doursat, R. (1992). Neural networks and the bias/variance dilemma. *Neural Computation*, 4(1), 1–58.
