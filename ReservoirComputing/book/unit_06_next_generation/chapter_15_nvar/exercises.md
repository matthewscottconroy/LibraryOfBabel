# Chapter 15 — Exercises, Thought Experiments, and Labs

---

## Conceptual Exercises

**Exercise 15.1 — Feature Count Scaling**

(a) Compute the NVAR feature dimension $D$ for the following configurations. Include only degree-1 and degree-2 terms.
   - $n = 1$, $k = 5$: $D = ?$
   - $n = 3$, $k = 1$: $D = ?$ (verify: should be 27)
   - $n = 3$, $k = 2$: $D = ?$
   - $n = 10$, $k = 1$: $D = ?$
   - $n = 64$, $k = 1$: $D = ?$

(b) For $n = 3$ (Lorenz observable), $k = 1$, compute $D$ for degrees $d = 1, 2, 3, 4$. At what degree does $D > 500$?

(c) Suppose you have $T = 10000$ training points. For what parameter values $(n, k, d)$ is the system $O \in \mathbb{R}^{T \times D}$ underdetermined ($D > T$)? What does this imply for the importance of ridge regularization?

**Exercise 15.2 — Volterra Connection**

Show that the NVAR with $n = 1$, $k = 1$, $d = 2$ has the following feature vector:

$$\mathbf{o}_t = [u_t, u_{t-1}, u_t^2, u_t u_{t-1}, u_{t-1}^2]^\top$$

(a) Express this as a second-order Volterra approximation: identify the kernels $h_1(0), h_1(1), h_2(0,0), h_2(0,1), h_2(1,1)$ in terms of the readout weights $W^{out}$.

(b) For the system $y_t = u_t + 0.5 u_{t-1}^2$, what is the exact Volterra representation? Does the NVAR feature vector with $k=1$, $d=2$ include all necessary features?

(c) For the system $y_t = u_t^3 u_{t-2}$, what is the minimum $(k, d)$ needed for NVAR to represent this exactly?

**Exercise 15.3 — The Rahimi-Recht Approximation**

(a) Show that the RBF kernel $k(\mathbf{x}, \mathbf{y}) = \exp(-\|\mathbf{x} - \mathbf{y}\|^2 / 2\sigma^2)$ is shift-invariant. What is its Fourier transform $\hat{k}(\boldsymbol{\omega})$?

(b) Verify that $\mathbb{E}_{\boldsymbol{\omega} \sim \mathcal{N}(0, \sigma^{-2}I)}[\cos(\boldsymbol{\omega} \cdot \mathbf{x})\cos(\boldsymbol{\omega} \cdot \mathbf{y})] + \mathbb{E}[\sin(\boldsymbol{\omega} \cdot \mathbf{x})\sin(\boldsymbol{\omega} \cdot \mathbf{y})] = k(\mathbf{x}, \mathbf{y})$. (This verifies the Rahimi-Recht construction for the RBF kernel.)

(c) What distribution $\hat{k}(\boldsymbol{\omega})$ corresponds to the Laplace kernel $k(\mathbf{x}, \mathbf{y}) = \exp(-\|\mathbf{x} - \mathbf{y}\|_1 / \sigma)$?

**Exercise 15.4 — NVAR vs. ESN on NARMA**

The NARMA-10 system has memory depth 10. The input is 1-dimensional.

(a) What is the minimum $k_{max}$ for NVAR to have any hope of solving NARMA-10? Why?

(b) With $k_{max} = 10$ and $d = 2$, compute the NVAR feature dimension.

(c) The NARMA-10 output depends on $y_{t-1}$ (the previous output), not just the input. How does this affect NVAR's ability to solve it? What modification to NVAR would be needed?

(d) An ESN with $N = 100$, $\alpha = 0.2$ routinely achieves NRMSE $\approx 0.15$ on NARMA-10. Does NVAR with the parameters in (b) achieve comparable performance? (Note: the feedback issue in (c) is a genuine limitation.)

**Exercise 15.5 — Kernel Comparison**

(a) For a 1-dimensional linear system $y_t = \sum_{k=0}^\infty \rho^k u_{t-k}$, what is the optimal Volterra kernel? At what order does the NVAR need to be truncated to represent this system with error $< \varepsilon$, as a function of $\rho$ and $\varepsilon$?

(b) For a 1-dimensional linear ESN with spectral radius $\rho$ and unit readout, show that the inner product $x_t^{(\mathbf{u})} \cdot x_t^{(\mathbf{v})} / N \to \sum_{k=0}^\infty \rho^{2k} u_{t-k} v_{t-k}$ as $N \to \infty$ for Gaussian $W^{in}$. What kernel on input time series does this define?

(c) Compare the two kernels from (a) and (b). Are they the same? What does this say about which method is better for linear temporal dependencies?

---

## Lab Exercises

**Lab 15.1 — Reproducing Gauthier et al. 2021**

*Objective:* Reproduce the core Lorenz prediction results of [Gauthier2021] using the starter code from Section 15.1.

```python
# This lab uses the nvar_predict function from Section 15.1

import numpy as np
from scipy.integrate import odeint
# (import make_polynomial_features and nvar_predict from section 15.1)

# Step 1: Generate 15000 steps of Lorenz (dt=0.025)
# Step 2: Run NVAR with k=1, d=2, reg=1e-4, n_train=10000
# Step 3: Compute one-step-ahead NRMSE on test set
# Step 4: Run in closed-loop mode for 1000 steps. Plot x-coordinate prediction vs truth.
# Step 5: Compute valid prediction time (threshold = 0.4 * std(u_test))

def closed_loop_nvar(u_init, W_out, k=1, degree=2, T=1000):
    """
    Run NVAR in closed-loop mode for T steps.
    u_init: shape (k+1, n) - initial history
    Returns predictions: shape (T, n)
    """
    n = u_init.shape[1]
    history = list(u_init.copy())  # list of n-dim vectors, most recent last
    preds = []
    for _ in range(T):
        # Build stacked history
        u_stacked = np.concatenate(history[::-1])  # [u_t, u_{t-1}, ..., u_{t-k}]
        o = make_polynomial_features(u_stacked[None, :], degree=degree)[0]
        y_next = o @ W_out
        preds.append(y_next.copy())
        history.pop(0)
        history.append(y_next)
    return np.array(preds)

# TODO:
# 1. Run NVAR with the configuration from the paper.
# 2. Compute VPT as a function of initial condition (average over 50 random restarts).
# 3. Compare to ESN with N=500, N=100, N=27 (same feature count).
# 4. Plot: VPT vs. N for ESN, and the NVAR result as a horizontal line.
# 5. Reproduce Figure 2 from Gauthier et al.: prediction trajectory with divergence time marked.
```

**Lab 15.2 — NVAR vs. ESN on High-Dimensional Input**

*Objective:* Demonstrate that ESN outperforms NVAR on a high-dimensional, long-memory task.

```python
def lorenz96(N=20, F=8.0, T_total=5000, dt=0.025, seed=42):
    """
    Lorenz-96 model: du_i/dt = (u_{i+1} - u_{i-2})u_{i-1} - u_i + F
    N variables, standard 'turbulent' forcing F=8.
    """
    from scipy.integrate import odeint
    rng = np.random.default_rng(seed)
    
    def l96(y, t):
        dy = np.zeros(N)
        for i in range(N):
            dy[i] = (y[(i+1)%N] - y[(i-2)%N]) * y[(i-1)%N] - y[i] + F
        return dy
    
    y0 = F * np.ones(N) + rng.normal(0, 0.01, N)
    t = np.arange(0, T_total * dt, dt)
    traj = odeint(l96, y0, t)
    return traj[500:]  # discard transient

# TODO:
# 1. Generate 4000 steps of Lorenz-96 with N=20 (20-dim observable).
# 2. Compare:
#    NVAR: k=1, d=2. Compute feature dimension D. Train on 3000 steps.
#    NVAR: k=2, d=2. Compute D. Same training.
#    ESN: N=200, alpha=0.1, rho=0.9. Train on 3000 steps.
#    ESN: N=500, alpha=0.1, rho=0.9.
# 3. Compute closed-loop VPT for each (average over 20 random initial conditions).
# 4. Report: which method wins at N=20? How does NVAR feature count compare to ESN size?
# 5. Discuss: the Lorenz-96 result should favor ESN. Does it? By how much?
```

**Lab 15.3 — Random Features Approximation**

*Objective:* Verify the Rahimi-Recht approximation empirically.

```python
def rbf_kernel(X, Y, sigma=1.0):
    """Exact RBF kernel matrix, shape (len(X), len(Y))."""
    dists = np.sum((X[:, None] - Y[None, :])**2, axis=-1)
    return np.exp(-dists / (2 * sigma**2))

def random_features_rbf(X, D, sigma=1.0, seed=42):
    """Approximate RBF kernel via D random cosine features."""
    rng = np.random.default_rng(seed)
    n = X.shape[1]
    omega = rng.normal(0, 1/sigma, (D, n))
    b = rng.uniform(0, 2*np.pi, D)
    Z = np.sqrt(2/D) * np.cos(X @ omega.T + b)  # shape (len(X), D)
    return Z

# TODO:
# 1. Generate 100 random 2D points X from N(0, I).
# 2. Compute the exact RBF kernel matrix K_exact = rbf_kernel(X, X).
# 3. For D in {10, 50, 100, 500, 1000}, compute Z = random_features_rbf(X, D)
#    and the approximate kernel K_approx = Z @ Z.T.
# 4. For each D, compute the relative approximation error:
#    err = ||K_exact - K_approx||_F / ||K_exact||_F
# 5. Plot: log(err) vs. log(D). Fit a line. What slope do you get?
#    Compare to the theoretical O(D^{-1/2}) prediction.
# 6. Bonus: Repeat with the Laplace kernel. What distribution should you use for omega?
```

---

## Thought Experiments

**Thought Experiment 15.A — The Nature of Randomness**

An ESN uses random weights $W^{rec}$ and $W^{in}$. NVAR uses deterministic polynomial features. Yet both are trained by linear regression on the resulting features.

(a) In what sense is the randomness of ESN a virtue? Does it provide anything that deterministic features cannot?

(b) Rahimi and Recht show that random features approximate a kernel. Does this mean that a carefully chosen deterministic feature set (like NVAR's polynomial features) could outperform random features for the same total feature count? Under what conditions?

(c) The deep ESN has multiple layers of random projections. How does this change the implicit kernel being approximated? Does depth in the random feature mapping help?

**Thought Experiment 15.B — The Limits of Polynomial Prediction**

Gauthier et al. show that NVAR with $d=2$ predicts the Lorenz system well. The Lorenz system has quadratic nonlinearities. Suppose you had a system with transcendental (non-polynomial) dynamics, such as $\dot{x} = \sin(x)$.

(a) Does the one-step map of this system have a polynomial approximation at small $\Delta t$? (Hint: Taylor expand $\sin$ around the current state.) At what degree does the Taylor approximation become accurate?

(b) For such a system, would NVAR or ESN be more natural? Justify in terms of the kernel each method implicitly uses.

---

## References

- [Gauthier2021] Gauthier, D.J., Bollt, E., Griffith, A., & Barbosa, W.A.S. (2021). Next generation reservoir computing. *Nature Communications*, 12, 5564.
- [Rahimi2007] Rahimi, A. & Recht, B. (2007). Random features for large-scale kernel machines. *NIPS 2007*.
- [Bollt2021] Bollt, E. (2021). On explaining the surprising success of reservoir computing forecaster of chaos? *Chaos*, 31(1), 013108.
- [Pathak2018] Pathak, J. et al. (2018). Model-free prediction of large spatiotemporally chaotic systems from data. *Physical Review Letters*, 120(2), 024102.
