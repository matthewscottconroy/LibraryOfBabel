# Mixture of Experts for Reservoir Computing

## The Expert Specialization Idea

Simple ensembles and stacking combine base reservoirs through a fixed or globally learned combination. Mixture of experts (MoE) takes a more adaptive approach: different reservoirs (experts) specialize on different regions of the input space or different dynamical regimes, and a gating network selects which experts to activate based on the current input [Jacobs et al. 1991].

The intuition is that a single reservoir with fixed hyperparameters may excel in one operating regime (e.g., slow variations) but fail in another (e.g., rapid fluctuations). Rather than averaging over these regimes, the MoE framework learns to route each input to the expert best suited to it. The gating network acts as a soft decision tree over the expert space.

## Model Specification

Let there be $K$ expert reservoirs $\{R_1, \ldots, R_K\}$, each with state $\mathbf{x}_t^{(k)}$ and readout $\hat{y}_t^{(k)} = \mathbf{w}_k^{\text{out} \top} \mathbf{x}_t^{(k)}$. The gating network produces a soft assignment vector:

$$\mathbf{g}(\mathbf{u}_t) = \text{softmax}(\mathbf{V} \mathbf{u}_t + \mathbf{c}) \in \Delta^{K-1},$$

where $\mathbf{V} \in \mathbb{R}^{K \times d_{\text{in}}}$ and $\mathbf{c} \in \mathbb{R}^K$. The mixture output is:

$$\hat{y}_t = \sum_{k=1}^K g_k(\mathbf{u}_t) \cdot \hat{y}_t^{(k)}.$$

The gating network is linear in the input $\mathbf{u}_t$ but could also receive the current reservoir state $\mathbf{x}_t$ or the concatenated expert states [Jordan & Jacobs 1994].

## EM Training

Training the MoE model is naturally formulated as an expectation-maximization (EM) problem [Jacobs et al. 1991]. Treat the expert assignment as a latent variable $z_t \in \{1, \ldots, K\}$ with prior $p(z_t = k) = g_k(\mathbf{u}_t)$ and likelihood $p(y_t \mid z_t = k) = \mathcal{N}(y_t; \hat{y}_t^{(k)}, \sigma^2)$.

**E-step:** Compute the soft assignments (posterior probabilities):

$$h_{t,k} = p(z_t = k \mid y_t, \mathbf{u}_t) = \frac{g_k(\mathbf{u}_t) \mathcal{N}(y_t; \hat{y}_t^{(k)}, \sigma^2)}{\sum_j g_j(\mathbf{u}_t) \mathcal{N}(y_t; \hat{y}_t^{(j)}, \sigma^2)}.$$

The soft assignment $h_{t,k}$ is large when expert $k$ produces a prediction close to $y_t$ at time $t$.

**M-step:** Update each expert's readout by weighted ridge regression:

$$\hat{\mathbf{w}}_k^{\text{out}} = \left(\sum_t h_{t,k} \mathbf{x}_t^{(k)} \mathbf{x}_t^{(k) \top} + \lambda \mathbf{I}\right)^{-1} \sum_t h_{t,k} y_t \mathbf{x}_t^{(k)}.$$

Update the gating parameters by weighted logistic regression: maximize $\sum_t \sum_k h_{t,k} \log g_k(\mathbf{u}_t)$ with respect to $\mathbf{V}$ and $\mathbf{c}$.

This EM procedure converges to a local maximum of the marginal likelihood $\prod_t \sum_k g_k(\mathbf{u}_t) \mathcal{N}(y_t; \hat{y}_t^{(k)}, \sigma^2)$ [Jacobs et al. 1991].

## Hard Assignment (Winner-Takes-All)

As an alternative to soft EM, hard assignment selects the single most active expert at each time step:

$$\hat{y}_t = \hat{y}_t^{(k^*)}, \qquad k^* = \arg\max_k g_k(\mathbf{u}_t).$$

This is computationally cheaper (no weighting needed) and provides interpretable specialization. The hard-assignment gating partitions the input space into $K$ regions, one per expert, creating a piecewise-linear (in the gating space) function approximator. Training proceeds by alternating between assigning each time step to its winner expert and retraining each expert's readout on its assigned time steps [Jordan & Jacobs 1994].

## Application to Multiscale Signals

Mixture of expert reservoirs is particularly well-suited to multiscale signals where different frequency ranges require qualitatively different processing. Consider a signal with alternating high-frequency and low-frequency episodes (e.g., epileptic EEG or market regime shifts). Expert $R_1$ is configured with small $\alpha_1$ (fast timescale), expert $R_2$ with large $\alpha_2$ (slow timescale). The gating network learns to activate $R_1$ during high-frequency episodes and $R_2$ during low-frequency episodes.

This specialization is not achievable by a single reservoir or by simple averaging: a single reservoir must choose one timescale, and averaging mixtures timescales when only one is appropriate. The MoE framework allows the model to switch behavior adaptively based on the current input statistics [Jordan & Jacobs 1994].

## Comparison with Deep ESN

The MoE and deep ESN architectures address hierarchical structure in complementary ways. Deep ESN handles signals with simultaneous multiple timescales (all scales present at once, requiring parallel processing). MoE handles signals with temporally separated multiple timescales (different scales dominant at different times, requiring selective routing). For signals with both properties, a hybrid is appropriate: a deep ESN as each expert, with a gating network selecting the relevant deep-ESN configuration.

---

## References

- Jacobs, R. A., Jordan, M. I., Nowlan, S. J., & Hinton, G. E. (1991). Adaptive mixtures of local experts. *Neural Computation*, 3(1), 79–87.
- Jordan, M. I., & Jacobs, R. A. (1994). Hierarchical mixtures of experts and the EM algorithm. *Neural Computation*, 6(2), 181–214.
