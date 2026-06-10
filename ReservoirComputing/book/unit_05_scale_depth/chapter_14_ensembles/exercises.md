# Chapter 14 — Exercises, Thought Experiments, and Labs

---

## Conceptual Exercises

**Exercise 14.1 — The Variance Reduction Ceiling**

Derive the variance of a bagged ensemble of $M$ predictors when the inter-predictor correlation is $\rho$.

(a) Show that $\text{Var}[\hat{f}^{avg}] = \frac{V}{M}(1 + (M-1)\rho)$.

(b) For $\rho = 0.5$ and $V = 1$, compute the ensemble variance for $M \in \{1, 2, 5, 10, 20, \infty\}$. Plot the result.

(c) Find the value of $M$ that achieves 90% of the maximum possible variance reduction (i.e., 90% of the way from $V$ to $\rho V$).

(d) Show that for $\rho > 0$, increasing $M$ beyond some saturation point provides diminishing returns. Quantify this saturation.

**Exercise 14.2 — Ambiguity Decomposition**

Verify the ambiguity decomposition for a concrete three-member ensemble. Suppose at a particular test point $\mathbf{x}^*$, the three expert predictions are $\hat{y}_1 = 0.8$, $\hat{y}_2 = 1.2$, $\hat{y}_3 = 0.7$, and the true value is $y^* = 1.0$.

(a) Compute the ensemble average prediction $\hat{y}^{avg}$.
(b) Compute the individual errors $(y^* - \hat{y}_m)^2$ for each member.
(c) Compute the average individual error $\bar{E}$.
(d) Compute the ambiguity $\bar{A} = \frac{1}{3}\sum_m (\hat{y}_m - \hat{y}^{avg})^2$.
(e) Verify that $E^{avg} = \bar{E} - \bar{A}$.

**Exercise 14.3 — Negative Correlation Learning**

Consider two reservoir computers with predictions $\hat{y}_1$ and $\hat{y}_2$ on the same input. The NC learning loss for ensemble member 1 is:

$$L_1 = (y - \hat{y}_1)^2 + \lambda (\hat{y}_1 - \hat{y}^{avg})(\hat{y}_2 - \hat{y}^{avg})$$

(a) Show that minimizing $L_1$ with respect to $\hat{y}_1$ introduces a penalty that pushes $\hat{y}_1$ away from the ensemble average when $\hat{y}_2$ is on the same side.

(b) In the linear readout setting, let $\hat{y}_m = \mathbf{w}_m^\top \mathbf{x}$ for reservoir state $\mathbf{x}$. Write out the gradient of $L_1$ with respect to $\mathbf{w}_1$.

(c) Show that the NC regularization is equivalent to adding a negative-correlation term to the ridge regression objective. Solve the resulting linear system.

**Exercise 14.4 — Mixture of Experts Training**

A two-expert MoE has experts $f_1(\mathbf{x}) = w_1^\top \mathbf{x}$ and $f_2(\mathbf{x}) = w_2^\top \mathbf{x}$ with gating weights $g_1(\mathbf{z}) = \sigma(\mathbf{v}^\top \mathbf{z})$ and $g_2 = 1 - g_1$ (sigmoid gating).

(a) Write the EM E-step formula for the responsibility $r_{1,t}$ of expert 1 for training point $(\mathbf{x}_t, y_t, \mathbf{z}_t)$.

(b) Write the M-step update for $w_1$ as a weighted ridge regression. What is the effective training set for expert 1?

(c) Describe qualitatively what happens when two experts specialize on different halves of the input range. What does convergence look like? What initial conditions favor successful specialization?

**Exercise 14.5 — Ensemble vs. Single Large Reservoir**

A practitioner has a compute budget for $N_{total} = 1000$ neurons total. Compare:

(a) A single reservoir with $N = 1000$ neurons, $\alpha = 0.2$, $\rho_{target} = 0.9$.
(b) An ensemble of $M = 10$ reservoirs, each with $N = 100$ neurons, same hyperparameters.
(c) An ensemble of $M = 10$ reservoirs with diverse hyperparameters: $\alpha \in \{0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 0.3, 0.1, 0.05, 0.2\}$ and $\rho_{target}$ drawn uniformly from $[0.7, 0.99]$.

For each, estimate: (1) the effective readout dimension, (2) the expected variance reduction relative to a single $N=100$ reservoir, (3) the expected bias compared to the single $N=1000$ reservoir.

In which regime (large training set, small training set, high-noise, low-noise) would you prefer each option?

---

## Lab Exercises

**Lab 14.1 — Measuring Ensemble Diversity and Error Correlation**

*Objective:* Empirically measure the pairwise error correlation matrix for an ensemble of random reservoirs.

```python
import numpy as np
from itertools import combinations

def build_reservoir(N, alpha, rho_target, input_dim, seed):
    rng = np.random.default_rng(seed)
    W_rec = rng.standard_normal((N, N)) * 0.1
    eigvals = np.linalg.eigvals(W_rec)
    W_rec = W_rec / np.max(np.abs(eigvals)) * rho_target
    W_in = rng.standard_normal((N, input_dim)) * 0.3
    return W_rec, W_in

def run_reservoir(W_rec, W_in, u, alpha):
    T = len(u)
    N = W_rec.shape[0]
    X = np.zeros((T, N))
    x = np.zeros(N)
    for t in range(T):
        x = (1 - alpha) * x + alpha * np.tanh(W_rec @ x + W_in @ np.atleast_1d(u[t]))
        X[t] = x
    return X

def ridge_regression(X, y, reg=1e-4):
    return np.linalg.solve(X.T @ X + reg * np.eye(X.shape[1]), X.T @ y)

# TODO:
# 1. Generate 3000 steps of NARMA-10. Train on first 2000, test on last 1000.
# 2. Build M=20 reservoirs with N=100, alpha=0.1, rho_target=0.9,
#    varying only the random seed.
# 3. For each reservoir, run it on the training data and train a readout.
# 4. Compute predictions on the test set and the prediction errors.
# 5. Compute the 20x20 error correlation matrix C.
# 6. Plot C as a heatmap.
# 7. Compute the average off-diagonal correlation rho_bar.
# 8. Now build M=20 reservoirs with diverse hyperparameters (vary alpha and rho_target).
#    Recompute rho_bar. How much does diversity help?
# 9. For each ensemble size M in {1,2,5,10,20}, compute the ensemble NRMSE
#    (average over 100 random subsets of M members). Compare to the theoretical
#    prediction from the variance reduction formula.
```

**Lab 14.2 — Bagging vs. Single Large Reservoir**

*Objective:* Compare bagging ensembles to single reservoirs on the Mackey-Glass task.

```python
def mackey_glass(T, tau=17, seed=42):
    """Generate Mackey-Glass time series."""
    rng = np.random.default_rng(seed)
    x = np.zeros(T + tau)
    x[:tau] = 0.9 + rng.normal(0, 0.01, tau)
    for t in range(tau, T + tau):
        x[t] = x[t-1] + (0.2 * x[t-tau] / (1 + x[t-tau]**10) - 0.1 * x[t-1])
    return x[tau:]

# TODO:
# 1. Generate 5000 steps of Mackey-Glass. Use 1-step-ahead prediction as the task.
#    Train set: steps 500-4000. Test set: steps 4000-5000.
# 2. Train: (a) Single ESN, N=500, alpha=0.3, rho=0.9
#           (b) Ensemble of M=5 ESNs, N=100 each, same hyperparams, different seeds
#           (c) Ensemble of M=10 ESNs, N=50 each, same hyperparams, different seeds
#           (d) Ensemble of M=5 ESNs with diverse hyperparams (N=100 each)
# 3. For each, report test NRMSE and 95% confidence intervals over 10 random seeds.
# 4. Plot: NRMSE vs. total neuron count for single reservoirs of size 50, 100, 200, 500.
#    Overlay ensemble results at equivalent total neuron counts.
# 5. Discuss: for fixed neuron budget, when does an ensemble outperform a single large reservoir?
```

**Lab 14.3 — Mixture of Experts on a Regime-Switching Task**

*Objective:* Build a two-expert MoE for a task with two clearly distinct regimes.

```python
def regime_switching_series(T, seed=42):
    """
    Regime 0: AR(1) with coefficient 0.9 (slow dynamics)
    Regime 1: AR(1) with coefficient 0.1 (fast dynamics)
    Regimes switch every 100 steps.
    """
    rng = np.random.default_rng(seed)
    y = np.zeros(T)
    regime = np.zeros(T, dtype=int)
    y[0] = rng.normal()
    for t in range(1, T):
        r = (t // 100) % 2
        regime[t] = r
        if r == 0:
            y[t] = 0.9 * y[t-1] + rng.normal(0, 0.1)
        else:
            y[t] = 0.1 * y[t-1] + rng.normal(0, 0.5)
    return y, regime

# TODO:
# 1. Generate 3000 steps. Use 1-step-ahead prediction. Train on 2000, test on 1000.
# 2. Build two reservoirs specialized for each regime:
#    Expert 1: alpha=0.1 (slow, good for regime 0)
#    Expert 2: alpha=0.9 (fast, good for regime 1)
# 3. Implement simple EM training for the MoE (2 iterations is sufficient).
# 4. Plot the gating weights g_1(t) over the test set. Do they recover the true regimes?
# 5. Compare MoE NRMSE to: (a) uniform ensemble average, (b) single reservoir, alpha=0.5.
# 6. Report: how much does input-dependent gating help over uniform averaging?
```

---

## Thought Experiments

**Thought Experiment 14.A — The Perfect Ensemble**

Imagine you have $M$ reservoir computers that make completely independent errors: $\rho = 0$. You take the arithmetic mean of their predictions.

(a) How does the ensemble error scale with $M$?
(b) In the limit $M \to \infty$, what does the ensemble predict? What is its error?
(c) Is this achievable in practice? What prevents it?

**Thought Experiment 14.B — Diversity Without Accuracy**

Consider an ensemble where each member is a constant predictor: $\hat{f}^{(m)}(\mathbf{x}) = c_m$ for some constant $c_m$ drawn uniformly from $[0, 2]$. Individual accuracy is poor, but diversity is high.

(a) What is the bias of each member for a target $y^* = 1$?
(b) What is the variance?
(c) What is the ensemble average prediction? What is its bias and variance?
(d) Does high diversity compensate for high individual bias? What does the ambiguity decomposition say?

---

## References

- [Breiman1996] Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.
- [Breiman2001] Breiman, L. (2001). Random forests. *Machine Learning*, 45(1), 5–32.
- [Geman1992] Geman, S., Bienenstock, E., & Doursat, R. (1992). Neural networks and the bias/variance dilemma. *Neural Computation*, 4(1), 1–58.
- [Krogh1995] Krogh, A. & Vedelsby, J. (1995). Neural network ensembles, cross validation, and active learning. *NIPS*, 7.
- [Jacobs1991] Jacobs, R.A. et al. (1991). Adaptive mixtures of local experts. *Neural Computation*, 3(1), 79–87.
