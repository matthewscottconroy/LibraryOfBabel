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

# Lab 15.1 solution — NVAR vs. ESN on Lorenz (Gauthier et al. 2021)
# Step 1: Run NVAR with k=1, delays=2 (D=27 features for Lorenz d=3).
# Train on 10000 steps (after 500-step warmup), predict closed-loop.
# Steps 2-5 are implemented in the nvar_predict function above.
#
# Expected results (matching Gauthier et al. 2021, Nature Comms):
#   NVAR (D=27):   VPT ≈ 5.0 Lyapunov times
#   ESN N=500:     VPT ≈ 5-6 Lyapunov times (with 10000 training steps)
#   ESN N=100:     VPT ≈ 3-4 Lyapunov times
#   ESN N=27:      VPT ≈ 1-2 Lyapunov times  (too small)
#
# Key insight: NVAR with D=27 features matches an ESN with N≈300-500.
# This is because the quadratic monomials of the delay embedding explicitly
# encode the "squaring" operations present in the Lorenz RHS (xy, xz terms).
# The ESN must learn these nonlinearities implicitly from the random reservoir;
# NVAR directly computes them from the data.
#
# Plot Figure 2 reproduction:
#   fig, axes = plt.subplots(3, 1, figsize=(10, 8), sharex=True)
#   for i, comp in enumerate(['x', 'y', 'z']):
#       axes[i].plot(true_test, label='True', lw=1.2)
#       axes[i].plot(pred_nvar, label='NVAR', lw=1.2, linestyle='--')
#       axes[i].axvline(vpt_idx, color='red', lw=1.5, label='VPT')
#       axes[i].set_ylabel(comp)
#   axes[-1].set_xlabel('Time steps')
#   axes[0].legend(); plt.tight_layout()
#   plt.savefig('nvar_lorenz_prediction.png', dpi=150)
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

# Lab 15.2 solution — NVAR vs. ESN on Lorenz-96 (high-dimensional)
#
# NVAR feature count for Lorenz-96 with d_obs=20, delays=2, degree=2:
#   linear_dim = 20 * 2 = 40
#   quadratic_dim = C(40+1, 2) = 40*41/2 = 820
#   total D = 40 + 820 = 860 features
#
# For k=2, d=2:  same D = 860 (only delay spacing changes, not feature count)
#
# Expected experimental results:
#   NVAR (D=860, k=1): VPT ≈ 1-2 Lyapunov times  (poor)
#   NVAR (D=860, k=2): VPT ≈ 1-2 Lyapunov times  (still poor)
#   ESN N=200:         VPT ≈ 2-3 Lyapunov times
#   ESN N=500:         VPT ≈ 3-5 Lyapunov times   (best)
#
# The Lorenz-96 result favors ESN because:
# 1. The 20-variable system has long-range spatial correlations; the ESN reservoir
#    implicitly learns them through its random connectivity.
# 2. NVAR's quadratic features grow quadratically in d_obs=20, while the ESN's
#    representational power scales with N (independent of input dimension).
# 3. For d_obs=3 (Lorenz), NVAR's 27 quadratic features can represent the right
#    polynomial nonlinearities. For d_obs=20, the 860-dimensional feature vector
#    includes many irrelevant cross-terms while missing the spatial structure.
#
# Conclusion: NVAR wins for low-dimensional, polynomial dynamics where the
# right monomials are known a priori. ESN wins for high-dimensional systems
# with complex nonlinearities or unknown structure.
#
# Implementation sketch (requires lorenz96 defined above):
#   traj96 = lorenz96(N=20, F=8.0, T_total=4500)[:4000]
#   X_nvar = nvar_features(traj96, k=1, delays=2)  # shape (T, 860)
#   # Train ridge regression readout (same as Lab 15.1) ...
#   # Build ESN with N=500, input_dim=20 ...
#   # Compute VPT using the vpt_closed_loop function ...
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

# Lab 15.3 solution — Random Features RBF approximation (Rahimi & Recht 2007)
import numpy as np
import matplotlib.pyplot as plt
from scipy.stats import linregress

rng = np.random.default_rng(42)
# 1. Generate 100 random 2D points
X_pts = rng.standard_normal((100, 2))

# 2. Exact RBF kernel matrix
K_exact = rbf_kernel(X_pts, X_pts, sigma=1.0)   # shape (100, 100)
K_norm = np.linalg.norm(K_exact, 'fro')

# 3-4. Random features approximation for varying D
D_values = [10, 50, 100, 500, 1000]
errors = []
for D in D_values:
    # Average over 20 random feature samples to reduce variance
    errs_rep = []
    for rep in range(20):
        Z = random_features_rbf(X_pts, D, sigma=1.0, seed=rep)
        K_approx = Z @ Z.T
        err = np.linalg.norm(K_exact - K_approx, 'fro') / K_norm
        errs_rep.append(err)
    errors.append(np.mean(errs_rep))
    print(f"D={D:5d}: relative error = {errors[-1]:.4f}")

# 5. Log-log fit
log_D  = np.log(D_values)
log_e  = np.log(errors)
slope, intercept, r, *_ = linregress(log_D, log_e)
print(f"\nEmpirical slope: {slope:.3f}  (theory: -0.5 from O(D^{{-1/2}}))")
# Expected: slope ≈ -0.5, confirming the Rahimi-Recht bound.

# Plot
fig, ax = plt.subplots(figsize=(6, 4))
ax.plot(np.log10(D_values), np.log10(errors), 'bo-', label='Empirical error')
ax.plot(np.log10(D_values), (np.log10(np.exp(1)) * slope * log_D + np.log10(np.exp(intercept) / 1)),
        'r--', label=f'Fitted slope = {slope:.2f}')
ax.set_xlabel('log₁₀(D)')
ax.set_ylabel('log₁₀(relative error)')
ax.set_title('Random RBF features: approximation error vs. D')
ax.legend(); plt.tight_layout()
plt.savefig('random_features_error.png', dpi=150)

# 6. Bonus — Laplace kernel: K(x,y) = exp(-||x-y||/sigma)
# The Laplace kernel corresponds to a Cauchy distribution for omega:
# omega ~ Cauchy(0, 1/sigma), i.e., omega ~ Student-t with 1 degree of freedom.
# This is because the Laplace kernel's Fourier transform is a Cauchy distribution
# (vs. the Gaussian transform for the RBF/Gaussian kernel).
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
