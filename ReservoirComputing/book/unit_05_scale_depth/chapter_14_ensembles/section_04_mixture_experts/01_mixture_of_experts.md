# 14.4.1 Mixture of Experts with Gated Reservoirs

## Beyond Uniform Averaging

Bagging averages ensemble members with equal weights, regardless of the current input. This is appropriate when the ensemble members are equally good across the input space. But in many time series tasks, different reservoir types are better suited to different regimes of the input signal: a slow reservoir (small $\alpha$) excels during slowly-varying phases; a fast reservoir (large $\alpha$) captures sudden transitions.

The **mixture of experts** (MoE) framework [Jacobs1991] addresses this by assigning input-dependent weights to ensemble members. Instead of:

$$\hat{y}_t = \frac{1}{M}\sum_m \hat{f}^{(m)}(\mathbf{x}_t)$$

the MoE predictor uses:

$$\hat{y}_t = \sum_m g_m(\mathbf{x}_t) \hat{f}^{(m)}(\mathbf{x}_t)$$

where $g_m(\mathbf{x}_t) \geq 0$ and $\sum_m g_m(\mathbf{x}_t) = 1$ are the **gating weights** — input-dependent mixing coefficients. The gating function $g: \mathbb{R}^d \to \Delta^{M-1}$ (where $\Delta^{M-1}$ is the $M-1$ dimensional simplex) determines which expert is trusted most for the current input.

## Gating Architecture for Reservoir Ensembles

For reservoir ensembles, the gating function can itself be a linear function of some reservoir state, making the entire system trainable with convex methods. A common choice is the **softmax gate**:

$$g_m(\mathbf{x}_t) = \frac{\exp(\mathbf{v}_m^\top \mathbf{z}_t)}{\sum_{m'} \exp(\mathbf{v}_{m'}^\top \mathbf{z}_t)}$$

where $\mathbf{z}_t$ is a "gating state" — either one of the reservoir states or a dedicated gating reservoir — and $\mathbf{v}_m \in \mathbb{R}^{N_g}$ are learnable gating parameters.

**Architecture options:**

1. **Shared gating reservoir:** A single additional reservoir (the "gating reservoir") drives the gating function. This reservoir may be designed to detect the current input regime (e.g., high-volatility vs. low-volatility in a financial time series).

2. **Expert-state gating:** The gating weights are functions of the experts' own states: $\mathbf{z}_t = [\mathbf{x}_t^{(1)\top}, \ldots, \mathbf{x}_t^{(M)\top}]^\top$. The gating network learns which expert's state best represents the current input.

3. **Input-direct gating:** The gating function is a direct function of the current input $\mathbf{u}_t$, bypassing the reservoir. This is simpler and works well when the relevant regime information is directly observable (e.g., a high/low volatility indicator in finance).

## Training the Mixture of Experts

The joint optimization of expert readouts $\{W_m^{out}\}$ and gating parameters $\{v_m\}$ is generally non-convex. The standard approach is the **expectation-maximization (EM) algorithm** [Jordan1994]:

**E-step:** Given current parameters, compute the "responsibility" of each expert for each training point:

$$r_{m,t} = \frac{g_m(\mathbf{z}_t) p(y_t | \hat{f}^{(m)}(\mathbf{x}_t), \sigma^2)}{\sum_{m'} g_{m'}(\mathbf{z}_t) p(y_t | \hat{f}^{(m')}(\mathbf{x}_t), \sigma^2)}$$

where $p(y_t | \hat{f}^{(m)}, \sigma^2) = \mathcal{N}(y_t; \hat{f}^{(m)}(\mathbf{x}_t), \sigma^2)$ is the Gaussian likelihood.

**M-step:** Update expert readouts using weighted ridge regression:

$$W_m^{out} = \left(\sum_t r_{m,t} \mathbf{x}_t^{(m)} \mathbf{x}_t^{(m)\top} + \lambda I\right)^{-1} \sum_t r_{m,t} y_t \mathbf{x}_t^{(m)\top}$$

and update gating parameters by maximizing the weighted likelihood with respect to $\{v_m\}$.

**In practice for RC:** Since reservoir states are fixed, the E-step and M-step are both tractable closed-form or well-conditioned optimization problems. The entire MoE training procedure requires at most a few EM iterations to converge, making it computationally inexpensive relative to training a deep network.

## Probabilistic Interpretation

The MoE model has a natural probabilistic interpretation:

$$p(y_t | \mathbf{x}_t, \mathbf{z}_t) = \sum_m g_m(\mathbf{z}_t) \mathcal{N}(y_t; W_m^{out}\mathbf{x}_t, \sigma_m^2)$$

This is a Gaussian mixture model where the mixing weights are state-dependent. The model allows different experts to have different noise variances $\sigma_m^2$, which can model the fact that some regimes are intrinsically noisier than others.

**Predictive uncertainty.** A key advantage of the probabilistic MoE is that it provides calibrated predictive uncertainty:

$$\text{Var}[y_t | \mathbf{x}_t, \mathbf{z}_t] = \sum_m g_m \sigma_m^2 + \sum_m g_m (W_m^{out}\mathbf{x}_t)^2 - \left(\sum_m g_m W_m^{out}\mathbf{x}_t\right)^2$$

The first term is the average within-expert variance (aleatoric uncertainty). The second and third terms give the between-expert variance (epistemic uncertainty from disagreement). High between-expert variance signals that the experts disagree — a useful indicator of unreliable prediction regions.

## When Does MoE Outperform Bagging?

The MoE framework is most beneficial when:

1. **The target function is piecewise smooth**: different regimes benefit from different reservoir dynamics.
2. **The regime is identifiable from the input/state**: the gating function can detect which regime the system is in.
3. **The experts are complementary, not redundant**: if all experts have similar dynamics, input-dependent gating provides no additional benefit.

For reservoir ensembles, the most natural MoE application is **multi-regime time series**: financial data with volatility clusters, speech signals with alternating voiced/unvoiced segments, or physiological signals with sleep/wake cycles. In each case, one expert specializes in each regime, and the gating function learns to recognize which regime is currently active.

---

## References

- [Jacobs1991] Jacobs, R.A., Jordan, M.I., Nowlan, S.J., & Hinton, G.E. (1991). Adaptive mixtures of local experts. *Neural Computation*, 3(1), 79–87.
- [Jordan1994] Jordan, M.I. & Jacobs, R.A. (1994). Hierarchical mixtures of experts and the EM algorithm. *Neural Computation*, 6(2), 181–214.
- [Masoudnia2014] Masoudnia, S. & Ebrahimpour, R. (2014). Mixture of experts: A literature survey. *Artificial Intelligence Review*, 42(2), 275–293.
- [Breiman1996] Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.
