# Chapter 14 — Key Concepts

---

## Bias-Variance Decomposition

The decomposition of a learner's expected generalization error into three components:

$$\text{Expected Error} = \text{Bias}^2 + \text{Variance} + \text{Irreducible Noise}$$

Bias measures systematic deviation from the truth (the average prediction minus the true value). Variance measures sensitivity to the particular training dataset drawn. The irreducible noise $\sigma^2$ cannot be reduced by any method. Ensemble averaging targets the variance component: by averaging multiple predictors, variance is reduced without changing bias.

---

## Bagging (Bootstrap AGGregating)

Breiman's (1996) procedure for variance reduction: train $M$ predictors on $M$ bootstrap samples of the training data, then average their predictions. For regression with squared loss, the variance of the bagged predictor satisfies:

$$\text{Var}[\hat{f}^{avg}] = \frac{V}{M}(1 + (M-1)\rho)$$

where $V$ is the single-predictor variance and $\rho$ is the average inter-predictor error correlation. As $M \to \infty$, the variance floor is $\rho V$. Diversity (low $\rho$) is therefore essential for effective bagging.

---

## Ambiguity Decomposition

The identity $E^{avg} = \bar{E} - \bar{A}$ (Krogh & Vedelsby 1995), where $E^{avg}$ is the ensemble error, $\bar{E}$ is the average individual error, and $\bar{A}$ is the average squared disagreement among ensemble members (the "ambiguity"). This decomposition makes precise the intuition that diversity is beneficial: any increase in ambiguity directly reduces ensemble error, without changing the average individual error.

---

## Ensemble Diversity

The degree to which ensemble members make uncorrelated errors. Diversity in a reservoir ensemble arises from: (1) different random seeds (different $W^{rec}$ realizations), (2) different hyperparameters (different $\alpha$, $\rho_{target}$, density), and (3) different random input projections $W^{in}$. The pairwise error correlation matrix $C_{mn} = \text{Corr}(e^{(m)}, e^{(n)})$ is the standard tool for measuring diversity.

---

## Negative Correlation Learning

A training method that explicitly promotes diversity by including a diversity penalty in each ensemble member's loss function. The penalty penalizes a member for making the same errors as other members, effectively encouraging disagreement. For linear readouts, this is a convex optimization and can be solved efficiently.

---

## Mixture of Experts (MoE)

An ensemble architecture where predictions are combined using input-dependent weights (gating weights) rather than uniform averaging:

$$\hat{y}_t = \sum_m g_m(\mathbf{z}_t) \hat{f}^{(m)}(\mathbf{x}_t), \quad g_m \geq 0, \quad \sum_m g_m = 1$$

The gating function $g_m(\mathbf{z}_t)$, typically implemented via softmax, learns which expert is most reliable for the current input regime. The MoE is trained by EM: the E-step computes expert responsibilities, the M-step updates expert readouts and gating parameters.

---

## Gating Function

The input-dependent weight function $g: \mathbb{R}^d \to \Delta^{M-1}$ in a mixture of experts. For reservoir ensembles, the gating function is typically a softmax of a linear function of a designated "gating state" $\mathbf{z}_t$. The gating state may be one of the expert reservoirs' states, a dedicated gating reservoir state, or the raw input.

---

## Predictive Uncertainty Decomposition

In a probabilistic MoE model, the total predictive variance decomposes as:

$$\text{Total Variance} = \underbrace{\sum_m g_m \sigma_m^2}_{\text{aleatoric}} + \underbrace{\sum_m g_m (W_m\mathbf{x})^2 - \left(\sum_m g_m W_m \mathbf{x}\right)^2}_{\text{epistemic}}$$

The aleatoric component (within-expert noise) cannot be reduced with more data. The epistemic component (expert disagreement) decreases as experts agree — it is a measure of how well the model knows which expert to trust.

---

## Error Correlation

The pairwise correlation $\rho = \text{Corr}(e^{(m)}, e^{(m')})$ between the prediction errors of two ensemble members. Low error correlation is equivalent to high diversity. The ensemble variance reduction factor is $(1 + (M-1)\rho)/M$: for $M = 10$ and $\rho = 0.3$, this is $0.37$, meaning the ensemble has 37% the variance of a single predictor.

---

## Reservoir Ensemble Architecture

The specific instantiation of an ensemble for reservoir computing: $M$ independent random reservoirs $(W_m^{rec}, W_m^{in})$, each with its own readout $W_m^{out}$ trained by ridge regression. Predictions are combined by averaging (bagging), by input-dependent weighting (MoE), or by stacking (training a meta-learner on the ensemble members' predictions).

---

## Stacking

A meta-learning approach to ensemble combination: train a second-level learner (the "meta-learner") to predict the target from the ensemble members' predictions. Unlike bagging (which uses uniform averaging), stacking can learn unequal weights and nonlinear combinations. For reservoir ensembles, the meta-learner is typically a linear or ridge regression model trained on the out-of-sample predictions of the base learners.
