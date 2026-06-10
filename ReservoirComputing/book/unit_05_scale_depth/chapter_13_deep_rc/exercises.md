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

# TODO: Run with L=4 layers, N=50 neurons, rho_target=0.9
# alpha_list = [0.5, 0.25, 0.1, 0.05]
# Plot the mean absolute state at each layer vs. time
# Measure the time at which each layer's response decays to e^{-1} of its peak
# Compare to the theoretical prediction tau^(ell) ≈ ell / alpha_ell
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

# TODO:
# 1. Generate 3000 steps of multiscale_signal. Use first 2000 for training.
# 2. Build and run: (a) 1-layer ESN with N=200, alpha=0.1
#                   (b) 4-layer deep ESN with N=50 per layer, alpha=[0.5,0.25,0.1,0.05]
#                   (c) 4-layer deep ESN with N=50 per layer, uniform alpha=0.1
# 3. Train readouts via ridge regression on the training set.
# 4. Evaluate NRMSE on the test set (steps 2000-3000).
# 5. For the deep ESN, also train readouts using only layer 1, layer 4, or all layers.
#    Which single layer gives the best result?
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

# TODO:
# 1. Create a synthetic dataset: 100 graphs from two classes.
#    Class 0: sparse graphs (p=0.1), Class 1: dense graphs (p=0.4).
#    Node features: random uniform in [0,1].
# 2. Run each graph through a Graph ESN for T=10 steps (same graph at each step).
# 3. Pool node states by mean-pooling to get a graph-level vector.
# 4. Train a linear classifier (logistic regression with ridge) on the pooled vectors.
# 5. Report classification accuracy on a held-out test set of 50 graphs.
# 6. Compare to a baseline that uses only node features (no graph structure).
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

# TODO:
# Grid search over:
# L in {1, 2, 4}
# uniform alpha in {0.05, 0.1, 0.2, 0.5}
# rho_target in {0.7, 0.9, 0.95}
# N_per_layer in {50, 100} (so total neurons = L * N_per_layer)
#
# For each configuration, run 3 random seeds and record mean NRMSE on NARMA-10.
# Plot a heatmap: L vs. alpha (best rho for each).
# Question: For L=1 vs. L=4 at their best alpha, how much does depth help?
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
