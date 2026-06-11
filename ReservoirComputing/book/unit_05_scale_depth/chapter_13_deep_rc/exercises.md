# Chapter 13 — Exercises, Thought Experiments, and Labs

---

## Conceptual Exercises

**Exercise 13.1 — Inductive ESP Proof**

Let $\{W_\ell^{rec}\}_{\ell=1}^L$ be the recurrent matrices of a deep ESN. Assume $\rho(W_\ell^{rec}) < 1$ for each $\ell$, and that tanh is the activation function.

(a) For a single leaky layer with $\alpha \in (0,1]$, show that the update map $F: \mathbb{R}^N \to \mathbb{R}^N$ defined by $F(\mathbf{x}) = (1-\alpha)\mathbf{x} + \alpha\tanh(W^{rec}\mathbf{x} + \mathbf{c})$ (for fixed input $\mathbf{c}$) is a contraction in the Euclidean norm when $\|(1-\alpha)I + \alpha W^{rec}\|_2 < 1$.

(b) Show that $\|(1-\alpha)I + \alpha W^{rec}\|_2 \leq (1-\alpha) + \alpha\rho(W^{rec})$ when $W^{rec}$ is normal.

(c) Complete the inductive argument for the deep ESP: if layer $\ell-1$ satisfies the ESP and its state is bounded, then layer $\ell$ also satisfies the ESP.

**Exercise 13.2 — Timescale Calculation**

A deep ESN has $L = 4$ layers with equal leaking rates $\alpha = 0.05$.

(a) Compute the effective memory time constant $\tau^{(\ell)}$ at each layer using the formula $\tau^{(\ell)} \approx \ell/\alpha$.

(b) At what frequency (in cycles per step) does the low-pass filter at each layer cut off? Use the approximation $\omega_c \approx \alpha_\ell$.

(c) If the input signal has significant frequency content at periods 5, 20, 80, and 320 steps, which layers are most relevant for each frequency component?

(d) Now set $\alpha_\ell = 0.2 \cdot (0.5)^{\ell-1}$ (geometrically decreasing). Recompute the time constants. Compare to the equal-$\alpha$ case.

**Exercise 13.3 — Concatenated vs. Last-Layer Readout**

Argue theoretically (using the timescale hierarchy) why reading from all layers simultaneously should outperform reading from the last layer alone for a task that requires both short-term and long-term memory.

Conversely, describe a task for which reading from only the last layer might suffice, and explain why.

**Exercise 13.4 — Graph ESP Condition**

A Graph ESN has $W^{rec}$ with spectral radius $\rho(W^{rec}) = 0.8$, $W^{nb}$ with spectral radius $\rho(W^{nb}) = 0.3$, and leaking rate $\alpha = 0.5$.

(a) What is the maximum spectral norm $\|A\|_2$ of the adjacency matrix for the Graph ESP sufficient condition to hold?

(b) For a ring graph on $n$ nodes, the adjacency matrix has spectral radius $\rho(A) = 2$. Can this graph support a Graph ESN with the parameters above? If not, what rescaling of $W^{nb}$ is needed?

(c) For a complete graph on $n$ nodes, $\rho(A) = n-1$. How does the maximum permissible $\rho(W^{nb})$ scale with network size?

**Exercise 13.5 — Depth vs. Width Tradeoff**

A practitioner has a computation budget of $N_{total} = 500$ neurons total.

(a) Compare the following architectures for a task requiring temporal integration over 50 time steps:
   - 1 layer with $N_1 = 500$ neurons, $\alpha_1 = 0.02$
   - 5 layers with $N_\ell = 100$ neurons each, $\alpha_\ell = 0.1$ for all $\ell$
   - 5 layers with decreasing $\alpha_\ell \in \{0.3, 0.15, 0.07, 0.035, 0.017\}$

   Compute the readout dimension for each architecture.

(b) The deep architectures have more readout parameters. How should this affect the regularization strength in ridge regression?

(c) For what training set sizes might the single-layer architecture be preferable to the deep architectures, despite the theoretical advantages of depth?

---

## Lab Exercises

**Lab 13.1 — Timescale Visualization**

*Objective:* Empirically confirm the timescale hierarchy by measuring the impulse response at each layer.

```python
import numpy as np
import matplotlib.pyplot as plt

def deep_esn_impulse(L, N, alpha_list, rho_target, T=500, seed=42):
    """
    Run a deep ESN on an impulse input and record all layer states.
    Returns: states shape (L, T)
    """
    rng = np.random.default_rng(seed)
    layers = []
    for ell in range(L):
        W_rec = rng.standard_normal((N, N)) * (1.0 / (N**0.5))
        W_rec = W_rec / np.max(np.abs(np.linalg.eigvals(W_rec))) * rho_target
        N_in = N if ell > 0 else 1
        W_in = rng.standard_normal((N, N_in)) * 0.1
        layers.append((W_rec, W_in, alpha_list[ell]))
    
    # Run impulse: u[0] = 1, u[t] = 0 for t > 0
    u = np.zeros(T)
    u[0] = 1.0
    
    states = np.zeros((L, T))
    x = [np.zeros(N) for _ in range(L)]
    
    for t in range(T):
        new_x = []
        for ell, (W_rec, W_in, alpha) in enumerate(layers):
            inp = x[ell-1] if ell > 0 else np.array([u[t]])
            pre_act = W_rec @ x[ell] + W_in @ inp
            new_x.append((1 - alpha) * x[ell] + alpha * np.tanh(pre_act))
        x = new_x
        for ell in range(L):
            states[ell, t] = np.mean(np.abs(x[ell]))
    
    return states

# Lab 13.1 solution — impulse response across layers
import numpy as np
import matplotlib.pyplot as plt

L = 4
N = 50
rho_target = 0.9
alpha_list = [0.5, 0.25, 0.1, 0.05]   # decreasing leak rates: deeper layers are slower
T = 300

layers = build_deep_esn(L=L, N=N, rho_target=rho_target, alpha_list=alpha_list)
states = impulse_response(layers, T=T)  # shape (L, T)

fig, axes = plt.subplots(1, 2, figsize=(12, 5))

# Left: impulse response curves
for ell in range(L):
    axes[0].plot(states[ell], label=f'Layer {ell+1}, α={alpha_list[ell]}')
axes[0].set_xlabel('Time step')
axes[0].set_ylabel('Mean |state|')
axes[0].set_title('Impulse response per layer')
axes[0].legend()

# Right: compare empirical vs theoretical time constant
# Empirical: first time states[ell] drops below states[ell].max() / e
tau_empirical = []
tau_theoretical = []
for ell in range(L):
    peak = states[ell].max()
    threshold = peak / np.e
    # Find the first index after the peak where the response falls below threshold
    peak_idx = int(np.argmax(states[ell]))
    below = np.where(states[ell][peak_idx:] < threshold)[0]
    tau_e = (peak_idx + below[0]) if len(below) > 0 else T
    tau_empirical.append(tau_e)
    # Theoretical: tau^(ell) ≈ 1 / alpha_ell for layer ell
    # (ignoring spectral radius correction; full formula: tau ≈ -1/ln(1-alpha_ell))
    tau_t = -1.0 / np.log(1.0 - alpha_list[ell])
    tau_theoretical.append(tau_t)

axes[1].bar(np.arange(L) - 0.2, tau_empirical, 0.4, label='Empirical τ')
axes[1].bar(np.arange(L) + 0.2, tau_theoretical, 0.4, label='Theoretical τ = -1/ln(1-α)')
axes[1].set_xticks(np.arange(L))
axes[1].set_xticklabels([f'Layer {ell+1}' for ell in range(L)])
axes[1].set_ylabel('Time constant (steps)')
axes[1].set_title('Empirical vs. theoretical time constants')
axes[1].legend()

plt.tight_layout()
plt.savefig('deep_esn_impulse.png', dpi=150)
plt.show()

# Observation: each deeper layer has a longer time constant because α decreases.
# The theoretical formula τ ≈ -1/ln(1-α) is exact for the linear (tanh→identity) limit
# and is a good approximation in the small-α regime.  For α=0.05, τ_theoretical ≈ 19.5 steps.
```

1. Run the experiment with uniform $\alpha = 0.1$ across 4 layers and plot the impulse response at each layer.
2. Repeat with geometrically decreasing $\alpha_\ell = 0.4 \cdot (0.5)^{\ell-1}$.
3. Measure the empirical time constant at each layer by fitting an exponential to the decay.
4. Compare measured vs. theoretical time constants. Where does the theory agree, and where does it deviate?

**Lab 13.2 — Deep ESN on Multiscale Time Series**

*Objective:* Demonstrate that deep ESNs outperform single-layer ESNs when the target has multiscale structure.

```python
def multiscale_signal(T, seed=42):
    """Generate a target with both fast (period 5) and slow (period 50) components."""
    rng = np.random.default_rng(seed)
    t = np.arange(T)
    fast = np.sin(2 * np.pi * t / 5.0)
    slow = np.sin(2 * np.pi * t / 50.0)
    noise = rng.standard_normal(T) * 0.1
    u = fast + slow + noise  # input
    y = fast * slow + noise * 0.5  # target: product of fast and slow
    return u, y

def train_esn_readout(states, y_train, reg=1e-4):
    """Ridge regression readout."""
    X = states  # shape (T, N_total)
    W = np.linalg.solve(X.T @ X + reg * np.eye(X.shape[1]), X.T @ y_train)
    return W

# Lab 13.2 solution — multiscale signal comparison
T_total = 3000
T_train = 2000
T_washout = 200
reg = 1e-4

u_full, y_full = multiscale_signal(T_total)
u_train, y_train = u_full[:T_train], y_full[:T_train]
u_test,  y_test  = u_full[T_train:], y_full[T_train:]

def run_deep_esn_states(layers, u_seq, T_washout=200):
    """Run deep ESN, concatenate all-layer states after washout."""
    T = len(u_seq)
    L = len(layers)
    N_each = layers[0][0].shape[0]
    all_states = []
    x = [np.zeros(N_each) for _ in range(L)]
    for t in range(T):
        new_x = []
        for ell, (W_rec, W_in, alpha) in enumerate(layers):
            inp = x[ell-1] if ell > 0 else np.array([u_seq[t]])
            pre_act = W_rec @ x[ell] + W_in @ inp
            new_x.append((1 - alpha) * x[ell] + alpha * np.tanh(pre_act))
        x = new_x
        if t >= T_washout:
            all_states.append(np.concatenate(x))
    return np.array(all_states)  # shape (T - T_washout, L*N)

def nrmse(pred, true):
    return np.sqrt(np.mean((pred - true)**2)) / (np.std(true) + 1e-8)

# (a) 1-layer ESN with N=200, alpha=0.1
layers_1 = build_deep_esn(L=1, N=200, rho_target=0.9, alpha_list=[0.1])
states_1_train = run_deep_esn_states(layers_1, u_train, T_washout)
states_1_test  = run_deep_esn_states(layers_1, u_test, 0)
W1 = train_esn_readout(states_1_train, y_train[T_washout:])
nrmse_1 = nrmse(states_1_test @ W1, y_test)

# (b) 4-layer deep ESN with multiscale alphas
layers_4m = build_deep_esn(L=4, N=50, rho_target=0.9, alpha_list=[0.5, 0.25, 0.1, 0.05])
states_4m_train = run_deep_esn_states(layers_4m, u_train, T_washout)
states_4m_test  = run_deep_esn_states(layers_4m, u_test, 0)
W4m = train_esn_readout(states_4m_train, y_train[T_washout:])
nrmse_4m = nrmse(states_4m_test @ W4m, y_test)

# (c) 4-layer deep ESN with uniform alpha=0.1
layers_4u = build_deep_esn(L=4, N=50, rho_target=0.9, alpha_list=[0.1]*4)
states_4u_train = run_deep_esn_states(layers_4u, u_train, T_washout)
states_4u_test  = run_deep_esn_states(layers_4u, u_test, 0)
W4u = train_esn_readout(states_4u_train, y_train[T_washout:])
nrmse_4u = nrmse(states_4u_test @ W4u, y_test)

print(f"NRMSE — 1-layer ESN (N=200, α=0.1):          {nrmse_1:.4f}")
print(f"NRMSE — 4-layer deep ESN (multiscale α):      {nrmse_4m:.4f}")
print(f"NRMSE — 4-layer deep ESN (uniform α=0.1):     {nrmse_4u:.4f}")
# Expected: multiscale deep ESN wins, because layer 1 (α=0.5) tracks the fast
# component (period 5) while layers 3-4 (α=0.05-0.1) integrate the slow component.

# Layer ablation study for the multiscale deep ESN
N_per = 50
for ell in range(4):
    idx_start, idx_end = ell * N_per, (ell + 1) * N_per
    W_single = train_esn_readout(states_4m_train[:, idx_start:idx_end], y_train[T_washout:])
    nr = nrmse(states_4m_test[:, idx_start:idx_end] @ W_single, y_test)
    print(f"  Layer {ell+1} alone: NRMSE = {nr:.4f}")
# Expected: intermediate layers (2-3) give the best single-layer result since they
# capture both timescales through hierarchical integration.
```

**Lab 13.3 — Graph ESN for Temporal Graph Classification**

*Objective:* Implement a simple Graph ESN and apply it to a synthetic graph classification task.

```python
import numpy as np

def random_graph(n_nodes, p_edge, seed=None):
    """Generate an Erdős-Rényi random graph."""
    rng = np.random.default_rng(seed)
    A = (rng.random((n_nodes, n_nodes)) < p_edge).astype(float)
    A = np.triu(A, 1)
    A = A + A.T  # symmetric
    return A

def graph_esn_step(H, x_feat, A, W_rec, W_in, W_nb, alpha):
    """
    Single step of a Graph ESN.
    H: (n_nodes, N_hidden) - current node states
    x_feat: (n_nodes, d_feat) - node features
    A: (n_nodes, n_nodes) - adjacency matrix
    Returns updated H.
    """
    n = H.shape[0]
    # Neighbor aggregation: (n_nodes, N_hidden)
    D_inv = np.diag(1.0 / (A.sum(axis=1) + 1e-8))
    nb_agg = D_inv @ A @ H  # mean-normalized aggregation
    
    pre_act = H @ W_rec.T + x_feat @ W_in.T + nb_agg @ W_nb.T
    H_new = (1 - alpha) * H + alpha * np.tanh(pre_act)
    return H_new

# Lab 13.3 solution — Graph ESN classification
rng_g = np.random.default_rng(7)
n_nodes = 15
d_feat  = 4
N_hidden = 32
T_steps  = 10
alpha_g  = 0.3

# Random graph ESN matrices (fixed, not trained)
W_rec_g = rng_g.standard_normal((N_hidden, N_hidden))
W_rec_g *= 0.9 / np.max(np.abs(np.linalg.eigvals(W_rec_g)))
W_in_g  = rng_g.uniform(-0.5, 0.5, (N_hidden, d_feat))
W_nb_g  = rng_g.standard_normal((N_hidden, N_hidden)) * 0.5

def encode_graph(A, x_feat, W_rec, W_in, W_nb, alpha, T):
    """Run Graph ESN for T steps, return mean-pooled final state."""
    H = np.zeros((A.shape[0], W_rec.shape[0]))
    for _ in range(T):
        H = graph_esn_step(H, x_feat, A, W_rec, W_in, W_nb, alpha)
    return H.mean(axis=0)  # mean-pool over nodes -> shape (N_hidden,)

# Build dataset: 150 total graphs (100 train, 50 test)
graphs = []
labels = []
for cls, p_edge in enumerate([0.1, 0.4]):  # class 0: sparse, class 1: dense
    for i in range(75):
        A_i = random_graph(n_nodes, p_edge, seed=rng_g.integers(1e6))
        x_i = rng_g.uniform(0, 1, (n_nodes, d_feat))
        graphs.append((A_i, x_i))
        labels.append(cls)

features = np.array([encode_graph(A, x, W_rec_g, W_in_g, W_nb_g, alpha_g, T_steps)
                     for A, x in graphs])
labels_arr = np.array(labels)

# Split 100 train / 50 test (interleaved to balance classes)
idx_train = list(range(0, 75, 1))[:50] + list(range(75, 150, 1))[:50]
idx_test  = list(range(0, 75, 1))[50:] + list(range(75, 150, 1))[50:]
X_tr, y_tr = features[idx_train], labels_arr[idx_train]
X_te, y_te = features[idx_test],  labels_arr[idx_test]

# Linear classifier via ridge regression (binary: predict sign(w^T x - 0.5))
lam = 1e-3
w_cls = np.linalg.solve(X_tr.T @ X_tr + lam * np.eye(N_hidden), X_tr.T @ (2*y_tr - 1))
acc_graph_esn = np.mean(np.sign(X_te @ w_cls) == (2*y_te - 1))

# Baseline: use only mean node features (no graph structure)
feat_base = np.array([x.mean(axis=0) for _, x in graphs])
w_base = np.linalg.solve(feat_base[idx_train].T @ feat_base[idx_train]
                          + lam * np.eye(d_feat),
                          feat_base[idx_train].T @ (2*y_tr - 1))
acc_baseline = np.mean(np.sign(feat_base[idx_test] @ w_base) == (2*y_te - 1))

print(f"Graph ESN accuracy: {acc_graph_esn:.2%}")
print(f"Node-feature baseline: {acc_baseline:.2%}")
# Expected: Graph ESN >> baseline because density information is encoded in the
# neighbor-aggregation term; random node features alone cannot distinguish classes.
```

**Lab 13.4 — Hyperparameter Search for Deep ESN**

*Objective:* Understand the interaction between depth, leaking rates, and spectral radius on NARMA-10 performance.

```python
from itertools import product

def narma10(T, seed=42):
    rng = np.random.default_rng(seed)
    u = rng.uniform(0, 0.5, T)
    y = np.zeros(T)
    for t in range(10, T):
        y[t] = (0.3 * y[t-1] 
                + 0.05 * y[t-1] * np.sum(y[t-10:t])
                + 1.5 * u[t-10] * u[t-1] + 0.1)
    return u, y

# Lab 13.4 solution — grid search for deep ESN on NARMA-10
from itertools import product as iproduct

T_narma = 3000
T_train_n = 2000
T_wash_n  = 100
u_n, y_n = narma10(T_narma)

L_list       = [1, 2, 4]
alpha_list_g = [0.05, 0.1, 0.2, 0.5]
rho_list     = [0.7, 0.9, 0.95]
N_list       = [50, 100]
seeds        = [0, 1, 2]

def run_esn_narma(L, alpha, rho, N_per, seed):
    """Return NRMSE on NARMA-10 for a given deep ESN configuration."""
    layers = build_deep_esn(L=L, N=N_per, rho_target=rho,
                            alpha_list=[alpha]*L, seed=seed)
    states_tr = run_deep_esn_states(layers, u_n[:T_train_n], T_wash_n)
    y_tr_cut  = y_n[T_wash_n:T_train_n]
    W = np.linalg.solve(states_tr.T @ states_tr + 1e-4 * np.eye(states_tr.shape[1]),
                        states_tr.T @ y_tr_cut)
    states_te = run_deep_esn_states(layers, u_n[T_train_n:], 0)
    pred      = states_te @ W
    return nrmse(pred, y_n[T_train_n:])

# Collect results (use N_per=50 for speed; repeat with 100 for final table)
results_grid = {}
for L, alpha, rho, N_per in iproduct(L_list, alpha_list_g, rho_list, [50]):
    nrmse_seeds = [run_esn_narma(L, alpha, rho, N_per, s) for s in seeds]
    results_grid[(L, alpha, rho)] = np.mean(nrmse_seeds)

# Find best rho for each (L, alpha) and build heatmap data
import pandas as pd
heatmap = np.zeros((len(L_list), len(alpha_list_g)))
for i, L in enumerate(L_list):
    for j, alpha in enumerate(alpha_list_g):
        best = min(results_grid[(L, alpha, rho)] for rho in rho_list)
        heatmap[i, j] = best

print("NRMSE heatmap (L x alpha, best over rho):")
df = pd.DataFrame(heatmap, index=[f'L={L}' for L in L_list],
                  columns=[f'α={a}' for a in alpha_list_g])
print(df.to_string(float_format='{:.4f}'.format))

# Best single-layer vs. best 4-layer
best_L1 = min(heatmap[0])
best_L4 = min(heatmap[2])
print(f"\nBest L=1 NRMSE: {best_L1:.4f}")
print(f"Best L=4 NRMSE: {best_L4:.4f}")
print(f"Depth improvement: {(best_L1 - best_L4)/best_L1:.1%}")
# Expected finding: L=4 with alpha=[0.1] or [0.05] outperforms L=1 by ~15-25%
# because the hierarchical structure provides both fast (α=0.5 first layer) and
# slow (α=0.05 last layer) memory needed for NARMA-10's 10-step dependencies.
```

---

## Thought Experiments

**Thought Experiment 13.A — Infinite Depth**

Consider the limit $L \to \infty$ with each layer becoming infinitely thin ($\alpha_\ell \to 0$ as $\ell \to \infty$, scaled so that $\sum_\ell \alpha_\ell$ stays finite). What would such an infinitely deep ESN compute?

(a) Argue that in this limit, the architecture approaches a continuous-time dynamical system. What is the analog of the leaking rate in the continuous-time setting?

(b) In the continuous limit, what is the impulse response shape? Compare to the result for finite depth.

(c) What would happen to the ESP in this limit? Is the contraction condition still satisfiable?

**Thought Experiment 13.B — Asymmetric Depth**

A practitioner has a task with the following structure: the target at time $t$ depends linearly on inputs from $t-1$ to $t-5$ (short range) and nonlinearly on inputs from $t-30$ to $t-50$ (long range). 

(a) Design a deep ESN architecture that is specifically tailored to this task. How many layers? What leaking rates?

(b) Would a single-layer ESN with $\alpha$ chosen to cover the long-range dependency ($\alpha \approx 1/50$) perform well on this task? Why or why not?

(c) Is the concatenated readout essential here, or could you get away with reading from only the top or bottom layers?

---

## Further Investigation

**Investigation 13.1.** Gallicchio et al. [Gallicchio2017b] claim that deep ESNs have better "input separation" — the ability to distinguish different input sequences. Design an experiment to test this claim. For two nearly identical input sequences that differ only in a single input value at time $t = 0$, measure how far apart the states are at each layer and at each subsequent time step. Does separation increase with depth?

---

## References

- [Gallicchio2017a] Gallicchio, C. & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
- [Gallicchio2017b] Gallicchio, C., Micheli, A., & Pedrelli, L. (2017). Deep reservoir computing: A critical experimental analysis. *Neurocomputing*, 268, 87–99.
- [Gallicchio2020] Gallicchio, C. & Micheli, A. (2020). Fast and deep graph neural networks. *Proceedings of AAAI*, 34(04), 3898–3905.
