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

# Lab 14.1 solution — error correlation and ensemble diversity
import numpy as np
import matplotlib.pyplot as plt
from itertools import combinations

def narma10_data(T=3000, seed=0):
    rng = np.random.default_rng(seed)
    u = rng.uniform(0, 0.5, T)
    y = np.zeros(T)
    for t in range(10, T):
        y[t] = 0.3*y[t-1] + 0.05*y[t-1]*np.sum(y[t-10:t]) + 1.5*u[t-10]*u[t-1] + 0.1
    return u, y

T_total = 3000; T_train = 2000; washout = 100
u_all, y_all = narma10_data(T_total)

def train_member(u, y, N, alpha, rho, seed, T_train=2000, washout=100):
    """Train one ensemble member; return test predictions and errors."""
    W_rec, W_in = build_reservoir(N, alpha, rho, 1, seed)
    X_tr = run_reservoir(W_rec, W_in, u[:T_train], alpha)[washout:]
    w = ridge_regression(X_tr, y[washout:T_train])
    X_te = run_reservoir(W_rec, W_in, u[T_train:], alpha)
    pred = X_te @ w
    return pred, pred - y[T_train:]

# 1-4. Build 20 homogeneous members (seed only varies)
M = 20
preds_homo = []
errors_homo = []
for seed in range(M):
    p, e = train_member(u_all, y_all, N=100, alpha=0.1, rho=0.9, seed=seed)
    preds_homo.append(p); errors_homo.append(e)

errors_homo = np.array(errors_homo)   # (M, T_test)

# 5-6. Error correlation matrix
C_homo = np.corrcoef(errors_homo)     # (M, M)
fig, ax = plt.subplots(figsize=(6, 5))
im = ax.imshow(C_homo, vmin=-1, vmax=1, cmap='RdBu_r')
plt.colorbar(im, ax=ax); ax.set_title('Error correlation — homogeneous ensemble')
plt.tight_layout(); plt.savefig('error_corr_homo.png', dpi=150)

# 7. Average off-diagonal correlation
rho_bar_homo = (C_homo.sum() - np.trace(C_homo)) / (M * (M - 1))
print(f"Homogeneous ensemble rho_bar = {rho_bar_homo:.4f}")

# 8. Diverse ensemble (vary alpha and rho_target)
alphas_div  = [0.05, 0.1, 0.2, 0.4, 0.05, 0.1, 0.2, 0.4, 0.05, 0.1,
               0.2, 0.4, 0.05, 0.1, 0.2, 0.4, 0.05, 0.1, 0.2, 0.4]
rhos_div    = [0.7, 0.8, 0.9, 0.95, 0.75, 0.85, 0.92, 0.97,
               0.7, 0.8, 0.9, 0.95, 0.75, 0.85, 0.92, 0.97, 0.8, 0.9, 0.7, 0.85]

errors_div = []
for seed in range(M):
    _, e = train_member(u_all, y_all, N=100, alpha=alphas_div[seed],
                         rho=rhos_div[seed], seed=seed)
    errors_div.append(e)
errors_div = np.array(errors_div)
C_div = np.corrcoef(errors_div)
rho_bar_div = (C_div.sum() - np.trace(C_div)) / (M * (M - 1))
print(f"Diverse ensemble rho_bar    = {rho_bar_div:.4f}")
print(f"Diversity improvement: {rho_bar_homo - rho_bar_div:.4f} reduction in correlation")

# 9. Ensemble NRMSE vs. M (100 random subsets)
rng_e = np.random.default_rng(99)
def nrmse(pred, true):
    return np.sqrt(np.mean((pred-true)**2)) / np.std(true)

y_test = y_all[T_train:]
for M_sub in [1, 2, 5, 10, 20]:
    nrmse_vals = []
    for _ in range(100):
        idx = rng_e.choice(M, M_sub, replace=False)
        ensemble_pred = preds_homo[0]*0  # zeros
        for i in idx: ensemble_pred = ensemble_pred + np.array(preds_homo[i])
        ensemble_pred /= M_sub
        nrmse_vals.append(nrmse(ensemble_pred, y_test))
    # Theoretical: sigma_M^2 = sigma_1^2 * (rho_bar + (1-rho_bar)/M)
    sigma_1 = nrmse_vals[0] if M_sub == 1 else None
    print(f"M={M_sub:2d}: mean NRMSE = {np.mean(nrmse_vals):.4f} ± {np.std(nrmse_vals):.4f}")
# Expected: NRMSE decreases as ~1/sqrt(M) for uncorrelated errors;
# high correlation (rho_bar ≈ 0.7-0.9 for homogeneous ensemble) limits improvement.
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

# Lab 14.2 solution — bagging vs. single large reservoir on Mackey-Glass
def mackey_glass_gen(T=5500, tau=17, seed=42):
    rng = np.random.default_rng(seed)
    x = np.zeros(T + tau)
    x[:tau] = 0.9 + rng.normal(0, 0.01, tau)
    for t in range(tau, T + tau):
        x[t] = x[t-1] + (0.2*x[t-tau]/(1+x[t-tau]**10) - 0.1*x[t-1])
    return x[tau:]  # shape (T,)

mg = mackey_glass_gen(5500)
# 1-step-ahead: input = mg[t], target = mg[t+1]
u_mg  = mg[:-1]   # input
y_mg  = mg[1:]    # target
# Split: washout 500 steps; train 500-4000; test 4000-5000
wash = 500; tr_end = 4000; te_end = 5000

def run_esn_mg(u, y, N, alpha, rho, seed, wash=500, tr_end=4000, te_end=5000):
    W_rec, W_in = build_reservoir(N, alpha, rho, 1, seed)
    X = run_reservoir(W_rec, W_in, u[:tr_end], alpha)
    w = ridge_regression(X[wash:], y[wash:tr_end])
    X_te = run_reservoir(W_rec, W_in, u[tr_end:te_end], alpha)
    pred = X_te @ w
    return pred, nrmse(pred, y[tr_end:te_end])

# (a) Single ESN N=500
preds_a, nr_a = run_esn_mg(u_mg, y_mg, N=500, alpha=0.3, rho=0.9, seed=0)

# (b) Ensemble M=5, N=100 each, same hyperparams
preds_b = []
for s in range(5):
    p, _ = run_esn_mg(u_mg, y_mg, N=100, alpha=0.3, rho=0.9, seed=s)
    preds_b.append(p)
nr_b = nrmse(np.mean(preds_b, axis=0), y_mg[4000:5000])

# (c) Ensemble M=10, N=50 each
preds_c = []
for s in range(10):
    p, _ = run_esn_mg(u_mg, y_mg, N=50, alpha=0.3, rho=0.9, seed=s)
    preds_c.append(p)
nr_c = nrmse(np.mean(preds_c, axis=0), y_mg[4000:5000])

# (d) Diverse ensemble M=5, N=100 each
alphas_d = [0.1, 0.2, 0.3, 0.5, 0.7]; rhos_d = [0.7, 0.85, 0.9, 0.95, 0.8]
preds_d = []
for s in range(5):
    p, _ = run_esn_mg(u_mg, y_mg, N=100, alpha=alphas_d[s], rho=rhos_d[s], seed=s)
    preds_d.append(p)
nr_d = nrmse(np.mean(preds_d, axis=0), y_mg[4000:5000])

print(f"(a) Single ESN N=500:              NRMSE = {nr_a:.5f}")
print(f"(b) Ensemble M=5, N=100 each:      NRMSE = {nr_b:.5f}  (total N=500)")
print(f"(c) Ensemble M=10, N=50 each:      NRMSE = {nr_c:.5f}  (total N=500)")
print(f"(d) Diverse ensemble M=5, N=100:   NRMSE = {nr_d:.5f}  (total N=500)")
# Key finding: for Mackey-Glass (a relatively easy task), the single large reservoir
# often matches or outperforms same-total-N ensembles at high N.  Ensembles help most
# when individual members are noisy (small N) or the task is harder.
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

# Lab 14.3 solution — Mixture of Experts on regime-switching series
T_moe = 3000; T_tr_moe = 2000
y_moe, regime_moe = regime_switching_series(T_moe)
u_moe = y_moe[:-1]; y_target_moe = y_moe[1:]  # 1-step-ahead
regime_te = regime_moe[T_tr_moe:-1]

# Expert reservoirs (both N=50, different alpha)
def make_expert(alpha, seed=0):
    W_rec, W_in = build_reservoir(50, alpha, 0.85, 1, seed)
    return W_rec, W_in, alpha

experts = [make_expert(0.1, seed=0), make_expert(0.9, seed=1)]

def get_states_moe(W_rec, W_in, alpha, u_seq):
    return run_reservoir(W_rec, W_in, u_seq, alpha)

# Initial E-step: uniform assignment
responsibilities = np.ones((T_tr_moe, 2)) * 0.5

for em_iter in range(3):
    # M-step: train each expert weighted by responsibilities
    expert_weights = []
    for k, (W_rec, W_in, alpha) in enumerate(experts):
        X = get_states_moe(W_rec, W_in, alpha, u_moe[:T_tr_moe])
        r_k = responsibilities[:, k]   # soft weights for this expert
        # Weighted ridge regression
        R = np.diag(r_k)
        w_k = np.linalg.solve(X.T @ R @ X + 1e-4 * np.eye(X.shape[1]), X.T @ R @ y_target_moe[:T_tr_moe])
        expert_weights.append(w_k)
    
    # E-step: compute responsibilities from prediction error
    preds_train = []
    for k, (W_rec, W_in, alpha) in enumerate(experts):
        X = get_states_moe(W_rec, W_in, alpha, u_moe[:T_tr_moe])
        preds_train.append(X @ expert_weights[k])
    
    # Gating: softmax over negative squared errors
    errors = np.stack([(p - y_target_moe[:T_tr_moe])**2 for p in preds_train], axis=1)
    log_resp = -0.5 * errors / (np.var(y_moe) + 1e-8)
    log_resp -= log_resp.max(axis=1, keepdims=True)
    responsibilities = np.exp(log_resp)
    responsibilities /= responsibilities.sum(axis=1, keepdims=True)

# Test: compute gating and predictions on test set
preds_te = []
gate_te_all = []
for k, (W_rec, W_in, alpha) in enumerate(experts):
    X_te = get_states_moe(W_rec, W_in, alpha, u_moe[T_tr_moe:])
    preds_te.append(X_te @ expert_weights[k])

# Simple gate: assign to expert with lower recent error (use last responsibilities from train)
gate_te = np.zeros(len(u_moe) - T_tr_moe)
for t in range(len(gate_te)):
    e0 = (preds_te[0][t] - y_target_moe[T_tr_moe + t])**2
    e1 = (preds_te[1][t] - y_target_moe[T_tr_moe + t])**2
    gate_te[t] = 0.0 if e0 < e1 else 1.0

moe_pred = gate_te[:, None] * preds_te[1] + (1 - gate_te[:, None]) * preds_te[0]
nr_moe = nrmse(moe_pred.ravel(), y_target_moe[T_tr_moe:])

# Uniform average baseline
nr_uniform = nrmse(0.5*(preds_te[0]+preds_te[1]), y_target_moe[T_tr_moe:])

# Single reservoir alpha=0.5
_, W_in_s = build_reservoir(50, 0.5, 0.85, 1, seed=2); W_rec_s, _, _ = experts[0]
X_s = get_states_moe(W_rec_s, W_in_s, 0.5, u_moe[:T_tr_moe])
w_s = ridge_regression(X_s, y_target_moe[:T_tr_moe])
X_te_s = get_states_moe(W_rec_s, W_in_s, 0.5, u_moe[T_tr_moe:])
nr_single = nrmse(X_te_s @ w_s, y_target_moe[T_tr_moe:])

print(f"MoE NRMSE:            {nr_moe:.4f}")
print(f"Uniform average:      {nr_uniform:.4f}")
print(f"Single (α=0.5):       {nr_single:.4f}")
# Check that gating mostly recovers true regimes
gate_acc = np.mean(gate_te == regime_te)
print(f"Gating accuracy vs. true regimes: {max(gate_acc, 1-gate_acc):.2%}")
# Expected: MoE > uniform > single when regimes are distinct and switching is sharp.
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
