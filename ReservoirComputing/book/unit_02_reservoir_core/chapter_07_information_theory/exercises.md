# Chapter 7 Exercises

## Analytical Exercises

**Exercise 7.1 (Prove $MC \leq N$, alternate approach).**
This exercise walks you through an alternative proof of Theorem 7.2.1 using the Cauchy–Schwarz inequality directly.

(a) Let $\mathbf{c}_k = \mathbb{E}[\mathbf{r}(t) u_{t-k}]$ and $R_{\mathbf{rr}} = \mathbb{E}[\mathbf{r}(t)\mathbf{r}(t)^\top]$. Use the matrix Cauchy–Schwarz inequality $(\mathbf{c}_k^\top R_{\mathbf{rr}}^{-1} \mathbf{c}_k)(\mathbf{c}_k^\top R_{\mathbf{rr}} \mathbf{c}_k) \geq (\mathbf{c}_k^\top \mathbf{c}_k)^2$ to obtain a different bound on $MC_k$.

(b) Show that $\sum_{k=1}^\infty \mathbf{c}_k \mathbf{c}_k^\top \preceq R_{\mathbf{rr}}$ using the argument that any function $g$ expressible as a linear combination of delayed inputs is also expressible as a linear functional of the reservoir state (since the reservoir state encodes the input history).

(c) From part (b), conclude that $\operatorname{tr}(R_{\mathbf{rr}}^{-1/2} (\sum_k \mathbf{c}_k \mathbf{c}_k^\top) R_{\mathbf{rr}}^{-1/2}) \leq N$.

(d) Use the cyclic trace property to show this equals $MC$ and thus complete the proof.

---

**Exercise 7.2 (Scalar reservoir memory capacity).**
Consider a scalar linear reservoir $r(t) = w \cdot r(t-1) + w^{in} \cdot u_t$ with $|w| < 1$ and i.i.d. input $u_t \sim \mathcal{U}(-1,1)$.

(a) Compute the stationary variance $\sigma_r^2 = \text{Var}[r(t)]$ in terms of $w$ and $w^{in}$.

(b) Show that $MC_k = (1 - w^2) w^{2k}$ for $k \geq 1$.

(c) Sum the geometric series to show $MC = w^2$.

(d) Interpret: what is the maximum memory capacity of a scalar reservoir? How does this relate to the $MC \leq N = 1$ bound?

(e) What happens to the memory profile $\{MC_k\}$ as $w \to 1^-$? Sketch the profile for $w = 0.5$, $w = 0.9$, and $w = 0.99$.

---

**Exercise 7.3 (Capacity of a nonlinear reservoir).**
A single tanh neuron: $r(t) = \tanh(w\,r(t-1) + w^{in} u_t)$.

(a) Explain why the total information-processing capacity $C_{total}$ of this neuron satisfies $C_{total} \leq 1$.

(b) With $w = 0$, the neuron has no recurrent connections. What is $MC_k$ for $k \geq 2$? What is $MC_1$? (Hint: compute $R^2$ for predicting $u_{t-1}$ from $r(t) = \tanh(w^{in} u_{t-1})$. What does this depend on?)

(c) Explain why $MC_1$ for the tanh neuron is strictly less than 1 for any nonzero $w^{in}$, but approaches 1 as $w^{in} \to 0$. (Hint: for small $w^{in}$, $\tanh(w^{in} u) \approx w^{in} u$.)

(d) Argue informally that capacity "used" for the nonlinear relationship between $r(t)$ and $u_{t-1}$ represents capacity *not* available for linear memory. This is the fundamental nonlinearity-vs-memory trade-off.

---

**Exercise 7.4 (Capacity decomposition for quadratic inputs).**
Let the input be i.i.d. $\mathcal{N}(0,1)$ and consider the target functions $\{1, u_t, u_t^2 - 1, u_{t-1}, u_{t-1}^2 - 1, u_t u_{t-1}, \ldots\}$ (Hermite polynomial basis, with appropriate normalization).

(a) Verify that the functions $H_0 = 1$, $H_1 = u_t$, $H_2 = (u_t^2 - 1)/\sqrt{2}$ are orthonormal: $\mathbb{E}[H_i H_j] = \delta_{ij}$.

(b) For a linear reservoir driven by $\mathcal{N}(0,1)$ inputs, show that the capacity for any quadratic target $u_{t-j} u_{t-k}$ ($j \neq k$) is zero. Why?

(c) For a nonlinear reservoir with tanh activation, explain why quadratic capacities can be nonzero. Give a qualitative argument in terms of the Taylor expansion of tanh.

(d) Sketch what the capacity profile (capacity vs. function degree and delay) would look like for (i) a linear reservoir, (ii) a reservoir with mild tanh nonlinearity, (iii) a heavily saturated reservoir.

---

**Exercise 7.5 (Transfer entropy connection).**
Transfer entropy from $X$ to $Y$ given $Z$ is defined as $T_{X \to Y \mid Z} = I(Y_t; X_{t-1:t-\infty} \mid Y_{t-1:t-\infty})$, where $I(\cdot;\cdot\mid\cdot)$ denotes conditional mutual information.

(a) In the reservoir context, set $Y = r_i$ (the $i$-th reservoir neuron) and $X = u$ (the input). Write out the transfer entropy $T_{u \to r_i}$ explicitly.

(b) For a linear Gaussian system, show that transfer entropy relates to the memory capacity contributions of neuron $i$. Specifically, argue that $T_{u \to r_i} = -\frac{1}{2}\log(1 - \sum_k c_{ik}^2 / \sigma_{r_i}^2)$ where $c_{ik} = \mathbb{E}[r_i(t) u_{t-k}]$ and $\sigma_{r_i}^2 = \text{Var}[r_i]$.

(c) How does transfer entropy differ from memory capacity? When might you prefer one over the other as a diagnostic tool?

---

**Exercise 7.6 (Task design requiring high MC and high nonlinearity).**
Design a synthetic task that simultaneously requires (a) memory of inputs from at least 20 timesteps ago, and (b) a nonlinear transformation of those inputs.

(a) Formalize your task as a target function $f(u_{t-1}, u_{t-2}, \ldots)$. Compute or estimate its decomposition into the orthonormal basis: what is the minimum capacity in linear memory functions? What is the minimum capacity in nonlinear components?

(b) For a reservoir of size $N = 100$ with $\rho = 0.95$, estimate whether the reservoir has sufficient capacity for your task. What does your analysis predict about the achievable performance?

(c) How would you adjust reservoir hyperparameters to better match your task's capacity requirements?

---

## Thought Experiments

**Thought Experiment 7.1: The Information-Theoretic Meaning of the Echo State Property.**

The echo state property (ESP) states that the reservoir state is uniquely determined by the input history: $\mathbf{r}(t) = F(u_t, u_{t-1}, u_{t-2}, \ldots)$ for some function $F$ independent of initial conditions.

(a) Without the ESP, what would "memory capacity" even mean? Suppose two different initial conditions lead to different reservoir trajectories under the same input. Can we still define $MC_k$ as the $R^2$ of predicting $u_{t-k}$ from $\mathbf{r}(t)$? What goes wrong?

(b) The ESP ensures that the reservoir state is a *deterministic* function of the input history (plus noise, if present). This means the state carries *no additional entropy* — all its randomness comes from the input. Is this a strength or a limitation from an information-theoretic viewpoint?

(c) Imagine a reservoir that violates the ESP in a mild way: it has a very long (but finite) transient, so that two nearby initial conditions eventually converge, but only after 10,000 timesteps. How would this affect the capacity measurement in practice? What would you see in the $MC_k$ profile?

---

**Thought Experiment 7.2: What Would Infinite Memory Capacity Require?**

The bound $MC \leq N$ means that to have infinite memory capacity, you would need an infinitely large reservoir. But consider the following scenarios:

(a) A delay line: a chain of $N$ neurons, each passing its activation to the next. This achieves $MC = N$ (each neuron stores exactly one input). What does the memory profile $\{MC_k\}$ look like? What is $MC_k$ for $k \leq N$ and for $k > N$?

(b) Now imagine the delay line is infinite. You have infinitely many neurons, each storing one past input perfectly. Is this a valid reservoir computing architecture? Does it satisfy the ESP?

(c) The $MC \leq N$ bound was derived under the assumption of finite $N$. What happens if we allow $N \to \infty$ but require that the total "cost" (measured, say, by the Frobenius norm of $W$) remains bounded? Does infinite memory capacity become possible?

(d) Ergodic theory provides another angle: a measure-preserving dynamical system can, in principle, encode its entire past trajectory in its current state (the Kolmogorov–Sinai entropy measures its information production rate). Is there a connection between the reservoir's memory capacity and the KS entropy of the reservoir dynamics?

---

**Thought Experiment 7.3: The Memory of a Chaotic System.**

A chaotic system (like the Lorenz system) has positive Lyapunov exponents, meaning nearby trajectories diverge exponentially fast. This means that the current state carries, in principle, *vanishing* information about the initial conditions after a long time — the butterfly effect erases past history.

(a) Does a chaotic reservoir therefore have *zero* memory capacity? Carefully distinguish between memory of *external inputs* and memory of *initial conditions*.

(b) For an externally driven chaotic system, the ESP is the statement that the drive overwhelms the internal dynamics — the driven trajectory "forgets" its initial condition and tracks the drive. Explain how this is consistent with positive Lyapunov exponents of the *autonomous* system.

(c) There is a subtle tension: strong chaos means rapid forgetting of initial conditions (good for ESP), but rapid forgetting also means rapid forgetting of inputs (bad for memory capacity). How does the spectral radius relate to this tension? At $\rho = 1$, the reservoir is "at the edge" — is this where memory capacity is maximized?

(d) In the literature, reservoirs operating "at the edge of chaos" (near the transition between stable and chaotic regimes) are sometimes reported to have improved computational power. Does the memory capacity framework support or contradict this claim? What would you need to measure to test it?

---

## Lab Exercises

**Lab 7.1: Implementing the Dambre et al. Capacity Measurement.**

Implement the full information-processing capacity measurement from [Dambre2012] for a standard ESN.

```python
import numpy as np
import scipy.special  # for Legendre polynomials
from itertools import product

def legendre_poly(n, x):
    """Evaluate the n-th Legendre polynomial at x (x in [-1,1])."""
    return scipy.special.eval_legendre(n, x)

def build_basis_functions(max_degree, max_delay):
    """
    Build a dictionary of basis functions for uniform input on [-1,1].
    Each basis function is a product of Legendre polynomials at different delays.
    Returns list of (degrees, delays) pairs and normalization constants.
    """
    basis = []
    # Single-variable: L_n(u_{t-k}) for n >= 1, k >= 0
    for k in range(max_delay):
        for n in range(1, max_degree + 1):
            # Normalization: E[L_n(u)^2] = 1/(2n+1) for uniform on [-1,1]
            norm = np.sqrt(2*n + 1)
            basis.append({'type': 'single', 'degree': n, 'delay': k, 'norm': norm})
    return basis

def compute_capacity_profile(W, w_in, rho_target, 
                              max_degree=3, max_delay=50, 
                              T=5000, washout=500):
    """
    Compute the capacity profile for each basis function.
    Returns array of capacities and total capacity.
    """
    N = W.shape[0]
    # Scale spectral radius
    eigs = np.abs(np.linalg.eigvals(W))
    W_scaled = W * rho_target / eigs.max()
    
    # Generate i.i.d. uniform input
    u = np.random.uniform(-1, 1, T + washout)
    
    # Run reservoir
    r = np.zeros(N)
    states = np.zeros((T, N))
    for t in range(T + washout):
        r = np.tanh(W_scaled @ r + w_in * u[t])
        if t >= washout:
            states[t - washout] = r
    
    # Center states
    states -= states.mean(axis=0)
    
    # Compute covariance matrix (with regularization)
    R_rr = (states.T @ states) / T + 1e-6 * np.eye(N)
    R_inv = np.linalg.inv(R_rr)
    
    # Compute capacity for each basis function
    basis = build_basis_functions(max_degree, max_delay)
    capacities = []
    
    for bf in basis:
        k = bf['delay']
        n = bf['degree']
        norm = bf['norm']
        
        if k >= T:
            capacities.append(0.0)
            continue
        
        # Evaluate basis function on input
        target = norm * legendre_poly(n, u[washout + k:washout + T])
        aligned_states = states[:T - k]
        
        # Cross-covariance
        c = (aligned_states.T @ target) / len(target)
        
        # Capacity
        cap = float(c @ R_inv @ c / np.var(target))
        cap = np.clip(cap, 0, 1)
        capacities.append(cap)
    
    return basis, capacities, sum(capacities)

# Run the experiment
np.random.seed(42)
N = 100
# Sparse random reservoir
W = np.random.randn(N, N) * (np.random.rand(N, N) < 0.1)
w_in = np.random.uniform(-0.1, 0.1, N)

for rho in [0.5, 0.8, 0.9, 0.95]:
    basis, caps, total = compute_capacity_profile(W, w_in, rho)
    print(f"rho={rho:.2f}: Total capacity = {total:.2f}")
    # Separate linear (degree=1) from nonlinear components
    linear_caps = [c for bf, c in zip(basis, caps) if bf['degree'] == 1]
    nonlinear_caps = [c for bf, c in zip(basis, caps) if bf['degree'] > 1]
    print(f"  Linear MC = {sum(linear_caps):.2f}, Nonlinear = {sum(nonlinear_caps):.2f}")
```

**Tasks:**
1. Run this code for $\rho \in \{0.5, 0.8, 0.9, 0.95, 0.99\}$ and tabulate the results.
2. Plot the capacity profile: for degree-1 functions, plot $C_{u_{t-k}}$ vs. $k$ for each $\rho$. Overlay the theoretical prediction $\propto \rho^{2k}$ and assess the fit.
3. How does the *nonlinear* capacity vary with $\rho$? When is the reservoir allocating more capacity to nonlinear functions?
4. Increase the reservoir size to $N = 500$ and repeat. Does the ratio of linear to nonlinear capacity change?

---

**Lab 7.2: Memory Capacity vs. Spectral Radius Sweep.**

```python
import numpy as np
import matplotlib.pyplot as plt

def measure_MC(W, w_in, rho, K=100, T=5000, washout=500):
    """Measure memory capacity MC = sum_k MC_k."""
    N = W.shape[0]
    eigs = np.abs(np.linalg.eigvals(W))
    W_sc = W * rho / eigs.max()
    
    u = np.random.uniform(-1, 1, T + washout + K)
    r = np.zeros(N)
    states = []
    for t in range(T + washout + K):
        r = np.tanh(W_sc @ r + w_in * u[t])
        if t >= washout:
            states.append(r.copy())
    states = np.array(states)
    
    MC_total = 0.0
    MC_profile = []
    T_eff = T
    for k in range(1, K + 1):
        X = states[k:k + T_eff]
        y = u[washout:washout + T_eff]
        # Ordinary least squares
        w_out, _, _, _ = np.linalg.lstsq(X, y, rcond=None)
        y_hat = X @ w_out
        corr = np.corrcoef(y_hat, y)[0, 1]
        mc_k = corr ** 2
        MC_profile.append(mc_k)
        MC_total += mc_k
    return MC_total, np.array(MC_profile)

np.random.seed(0)
N = 100
W = np.random.randn(N, N) * (np.random.rand(N, N) < 0.1)
w_in = np.random.uniform(-0.2, 0.2, N)

rho_vals = np.linspace(0.1, 0.99, 30)
mc_vals = []
for rho in rho_vals:
    mc, _ = measure_MC(W, w_in, rho)
    mc_vals.append(mc)
    print(f"rho={rho:.3f}: MC={mc:.2f}")

plt.figure(figsize=(8, 5))
plt.plot(rho_vals, mc_vals, 'b-o', markersize=4)
plt.axhline(N, color='r', linestyle='--', label=f'Theoretical max N={N}')
plt.xlabel(r'Spectral radius $\rho$')
plt.ylabel('Memory capacity MC')
plt.title('Memory Capacity vs. Spectral Radius')
plt.legend()
plt.tight_layout()
plt.savefig('mc_vs_rho.pdf')
plt.show()
```

**Tasks:**
1. Run the experiment. At what $\rho$ does $MC$ saturate or reach a maximum?
2. Repeat for $N = 50$, $N = 100$, $N = 200$. Does the shape of the $MC(\rho)$ curve scale with $N$?
3. Replace the tanh activation with a linear activation. Does $MC$ now approach $N$ as $\rho \to 1$? Why does tanh reduce MC?
4. Try different input scaling $\sigma_{in} \in \{0.01, 0.1, 1.0\}$. How does this interact with $\rho$ to determine MC?

---

**Lab 7.3: Transfer Entropy Visualization.**

Implement a visualization of transfer entropy between the input and individual reservoir neurons.

```python
import numpy as np
from scipy.stats import gaussian_kde

def mutual_information_knn(X, Y, k=5):
    """
    Estimate mutual information I(X;Y) using the Kraskov et al. k-NN estimator.
    X, Y: 1D arrays of equal length.
    """
    from scipy.spatial import cKDTree
    import scipy.special as sp
    
    N = len(X)
    XY = np.column_stack([X, Y])
    
    tree_xy = cKDTree(XY)
    tree_x  = cKDTree(X.reshape(-1, 1))
    tree_y  = cKDTree(Y.reshape(-1, 1))
    
    # Find k-th nearest neighbors in joint space
    dists, _ = tree_xy.query(XY, k=k+1)
    eps = dists[:, -1]  # distance to k-th neighbor
    
    # Count neighbors within eps in marginal spaces
    nx = np.array([len(tree_x.query_ball_point([x], r=e)) - 1 
                   for x, e in zip(X, eps)])
    ny = np.array([len(tree_y.query_ball_point([y], r=e)) - 1 
                   for y, e in zip(Y, eps)])
    
    mi = sp.digamma(k) - np.mean(sp.digamma(nx + 1) + sp.digamma(ny + 1)) \
         + sp.digamma(N)
    return max(0, mi)

def transfer_entropy(source, target, lag=1, k=5):
    """
    Estimate TE from source to target at given lag.
    TE(X->Y) = I(Y_t; X_{t-lag} | Y_{t-1})
    Approximated as I(Y_t, X_{t-lag}) - I(Y_t, Y_{t-1}) (simplification)
    """
    T = min(len(source), len(target))
    s = source[:-lag]
    t_curr = target[lag:]
    t_past = target[:-lag]
    T = min(len(s), len(t_curr), len(t_past))
    
    # Approximate TE using conditional MI
    joint_target = np.column_stack([t_curr[:T], t_past[:T]])
    mi_full = mutual_information_knn(s[:T], joint_target[:, 0], k=k)
    mi_cond = mutual_information_knn(t_past[:T], joint_target[:, 0], k=k)
    return max(0, mi_full - mi_cond)

# Run on a small reservoir
np.random.seed(1)
N = 20
W = np.random.randn(N, N) * (np.random.rand(N, N) < 0.1)
eigs = np.abs(np.linalg.eigvals(W))
W = W * 0.9 / eigs.max()
w_in = np.random.uniform(-0.2, 0.2, N)

T = 1000
u = np.random.uniform(-1, 1, T + 100)
r = np.zeros(N)
states = []
for t in range(T + 100):
    r = np.tanh(W @ r + w_in * u[t])
    if t >= 100:
        states.append(r.copy())
states = np.array(states)

# Compute TE from input to each neuron at various lags
lags = [1, 2, 5, 10]
TE_matrix = np.zeros((N, len(lags)))
for j, lag in enumerate(lags):
    for i in range(N):
        TE_matrix[i, j] = transfer_entropy(u[100:], states[:, i], lag=lag)
    print(f"Lag {lag}: mean TE = {TE_matrix[:, j].mean():.4f}")

import matplotlib.pyplot as plt
fig, axes = plt.subplots(1, len(lags), figsize=(12, 4))
for j, (lag, ax) in enumerate(zip(lags, axes)):
    ax.bar(range(N), TE_matrix[:, j])
    ax.set_title(f'TE at lag {lag}')
    ax.set_xlabel('Neuron index')
    ax.set_ylabel('Transfer entropy (nats)')
plt.suptitle('Transfer Entropy from Input to Reservoir Neurons')
plt.tight_layout()
plt.savefig('transfer_entropy.pdf')
plt.show()
```

**Tasks:**
1. Which neurons receive the most transfer entropy from the input at lag 1? Do these correlate with the magnitude of their input weights $|w^{in}_i|$?
2. How does the TE profile change with lag? Which neurons still show high TE at lag 10?
3. Repeat with $\rho = 0.5$ vs. $\rho = 0.99$. How does the spectral radius affect the distribution of TE across neurons and lags?

---

## Programming Projects

**Project 7.A: Capacity-Optimal Reservoir Design.**

The goal of this project is to design a reservoir that maximizes a *specific* component of the capacity profile.

1. Given a target task (e.g., $k$-step nonlinear prediction: $y_t = u_{t-10}^2 + u_{t-5} u_{t-10}$), identify which capacity components are required.
2. Implement the full Dambre capacity measurement for this task.
3. Use a simple optimization loop (gradient-free, e.g., Nelder–Mead or random search) to tune $\rho$, $\sigma_{in}$, and reservoir connectivity to maximize the capacity components required by your task.
4. Evaluate: does a reservoir designed to maximize task-relevant capacity perform better on the task than a random reservoir of the same size?

Write a short report (1–2 pages) describing your methodology, results, and the limitations of using capacity as a proxy for task performance.

---

**Project 7.B: Capacity as a Function of Reservoir Architecture.**

Compare the capacity profiles of four reservoir architectures: (1) random sparse ESN, (2) fully connected random ESN, (3) delay-line reservoir, (4) small-world network reservoir. For each:

1. Measure the full capacity profile (linear memory, quadratic, cross-delay quadratic) using the Dambre framework.
2. Plot the capacity profile as a heatmap over (function degree, delay).
3. Measure the total capacity as a function of $N$ for each architecture. Does $C_{total}$ grow linearly with $N$ for all architectures? Are there architectures that are more efficient?
4. Test each architecture on three tasks with different capacity requirements: (a) linear prediction, (b) nonlinear classification of current input, (c) long-range nonlinear dependency task.
5. Assess whether the capacity profile predicts task performance.
