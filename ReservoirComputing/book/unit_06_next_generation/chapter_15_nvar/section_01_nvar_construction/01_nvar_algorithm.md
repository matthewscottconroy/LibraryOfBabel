# 15.1.1 The NVAR Architecture: Polynomial Features and the Lorenz Benchmark

## The Algorithm

The Next-Generation Reservoir Computer (NVAR), introduced by Gauthier et al. [Gauthier2021], dispenses with the reservoir entirely. Its prediction at time $t$ is a polynomial function of the recent input history:

$$\hat{y}_t = W^{out} \mathbf{o}_t$$

where $\mathbf{o}_t$ is a vector of nonlinear features constructed from the most recent $k+1$ observations:

$$\mathbf{o}_t = P(u_t, u_{t-1}, \ldots, u_{t-k})$$

The function $P$ is a polynomial feature map: it includes the raw inputs, all pairwise products, all degree-3 products, and so on, up to some maximum degree $d$.

**Precise construction.** Let $\mathbf{u}_{t:k} = [u_t^\top, u_{t-1}^\top, \ldots, u_{t-k}^\top]^\top \in \mathbb{R}^{(k+1)n}$ be the stacked recent history (where $n$ is the dimension of the observable). The NVAR feature vector is:

$$\mathbf{o}_t = \left[\mathbf{u}_{t:k}^\top, \;\text{(all degree-2 monomials of } \mathbf{u}_{t:k}\text{)}^\top, \;\text{(all degree-3 monomials of } \mathbf{u}_{t:k}\text{)}^\top, \ldots \right]^\top$$

The total number of degree-$d$ monomials of a $(k+1)n$-dimensional vector is $\binom{(k+1)n + d}{d}$. The total feature dimension is $\sum_{j=1}^d \binom{(k+1)n+j-1}{j}$.

**The readout** $W^{out}$ is a matrix trained by ridge regression:

$$W^{out} = \mathbf{Y} \mathbf{O}^\top (\mathbf{O}\mathbf{O}^\top + \lambda I)^{-1}$$

where $\mathbf{O} \in \mathbb{R}^{D \times T}$ is the matrix of feature vectors ($D$ is the feature dimension) and $\mathbf{Y} \in \mathbb{R}^{m \times T}$ is the matrix of target outputs.

**No warmup required.** Unlike a standard ESN, NVAR has no transient state to wash out: the features are computed directly from the input, so prediction begins at step $k$ (as soon as $k+1$ observations are available).

## The Lorenz System Benchmark

Gauthier et al. [Gauthier2021] evaluate NVAR on the Lorenz system:

$$\dot{x} = \sigma(y - x), \quad \dot{y} = x(\rho - z) - y, \quad \dot{z} = xy - \beta z$$

with standard parameters $\sigma = 10$, $\rho = 28$, $\beta = 8/3$. The system is integrated with step size $\Delta t = 0.025$ seconds and the observable is all three components: $\mathbf{u}_t = [x_t, y_t, z_t]^\top \in \mathbb{R}^3$.

The task is **one-step-ahead prediction**: given $\mathbf{u}_t$, predict $\mathbf{u}_{t+1}$.

**NVAR configuration.** Gauthier et al. use:
- $k = 1$ (two time steps of history: $\mathbf{u}_t$ and $\mathbf{u}_{t-1}$)
- Degree $d = 2$ (linear and quadratic features only)
- Total feature dimension: $6 + \binom{6+1}{2} = 6 + 21 = 27$ features (using $[u_t^\top, u_{t-1}^\top]$ as the base $6$-dim vector)
- $\lambda = 10^{-4}$ (ridge parameter)
- Training data: 10,000 steps (250 seconds)
- Test data: 1,000 steps (25 seconds)

**Results.** The NVAR with these settings achieves a valid prediction time of approximately $5.0$ Lyapunov times on the Lorenz system. The standard ESN (with $N = 500$ neurons, optimally tuned) achieves approximately $4.9$–$5.5$ Lyapunov times. The NVAR matches ESN performance with a 27-dimensional feature vector, versus 500-dimensional reservoir states.

## Why So Few Features?

The result is initially surprising: 27 features suffice to match 500 reservoir neurons. The explanation is that the Lorenz system, despite its chaotic appearance, is low-dimensional: it lives on an attractor of fractal dimension approximately 2.05 [Grassberger1983]. The dynamics are described by a three-dimensional ordinary differential equation with quadratic nonlinearities. When $\Delta t$ is small, the one-step-ahead map $\mathbf{u}_{t+1} = F(\mathbf{u}_t)$ is well-approximated by a polynomial of low degree in $\mathbf{u}_t$. The additional history (using $\mathbf{u}_{t-1}$) captures the second-order corrections due to the integration step.

The key insight: **NVAR is efficient when the target functional is well-approximated by a low-degree polynomial in recent inputs**. This is precisely the case for low-dimensional chaotic systems with polynomial dynamics at short integration steps.

## Implementing NVAR: Complete Python Code

```python
import numpy as np
from scipy.integrate import odeint
from itertools import combinations_with_replacement

def lorenz(state, t, sigma=10.0, rho=28.0, beta=8/3):
    x, y, z = state
    return [sigma*(y-x), x*(rho-z)-y, x*y-beta*z]

def generate_lorenz(T_total, dt=0.025, seed=42):
    rng = np.random.default_rng(seed)
    t = np.arange(0, T_total*dt, dt)
    state0 = [rho_0 + rng.normal(0, 0.01) for rho_0 in [0.0, 1.0, 1.05]]
    traj = odeint(lorenz, state0, t)
    return traj  # shape (T_total, 3)

def make_polynomial_features(U, degree=2):
    """
    U: shape (T, n) — input history stacked as rows
    Returns: O, shape (T, D) — polynomial feature matrix
    """
    T, n = U.shape
    features = [U]  # degree 1
    indices = list(range(n))
    for d in range(2, degree + 1):
        combos = list(combinations_with_replacement(indices, d))
        poly = np.column_stack([np.prod(U[:, list(c)], axis=1) for c in combos])
        features.append(poly)
    return np.hstack(features)

def nvar_predict(traj, k=1, degree=2, reg=1e-4, n_train=10000):
    """
    NVAR one-step prediction on Lorenz.
    Returns train NRMSE, test NRMSE, valid prediction time.
    """
    T, n = traj.shape
    
    # Build stacked history: U[t] = [u_t, u_{t-1}, ..., u_{t-k}]
    U_stacked = np.hstack([traj[k-i:T-i] for i in range(k+1)])  # (T-k, n*(k+1))
    targets = traj[k+1:]  # one-step-ahead targets, shape (T-k-1, n)
    U_stacked = U_stacked[:-1]  # align with targets
    
    # Polynomial features
    O = make_polynomial_features(U_stacked, degree=degree)  # (T-k-1, D)
    
    # Split train/test
    O_train, O_test = O[:n_train], O[n_train:]
    Y_train, Y_test = targets[:n_train], targets[n_train:]
    
    # Ridge regression
    W_out = np.linalg.solve(O_train.T @ O_train + reg * np.eye(O_train.shape[1]),
                             O_train.T @ Y_train)
    
    # Predictions
    Y_pred_test = O_test @ W_out
    
    # NRMSE
    nrmse = np.sqrt(np.mean((Y_test - Y_pred_test)**2)) / np.std(Y_test)
    
    return W_out, O_test, Y_pred_test, Y_test, nrmse

# Example usage:
# traj = generate_lorenz(T_total=15000)
# W_out, O_test, Y_pred, Y_true, nrmse = nvar_predict(traj, k=1, degree=2)
# print(f"Test NRMSE: {nrmse:.4f}")
# Feature dimension: 6 + C(6+1,2) = 6 + 21 = 27
```

## Valid Prediction Time

For autonomous (closed-loop) prediction, the NVAR feeds its own prediction back as input. The **valid prediction time** (VPT) is defined as the first time the normalized prediction error exceeds a threshold $\epsilon_{VPT}$ (typically 0.4 or some multiple of the standard deviation of the attractor):

$$T_{VPT} = \min\left\{t : \frac{\|\hat{\mathbf{u}}_t - \mathbf{u}_t\|}{\sigma_u} > \epsilon_{VPT}\right\}$$

This is typically expressed in units of the largest Lyapunov exponent $\lambda_1$: $T_{VPT}^{Ly} = \lambda_1 \cdot T_{VPT}$. For the Lorenz system, $\lambda_1 \approx 0.906$ nats per second, or $\approx 0.023$ per integration step at $\Delta t = 0.025$.

Gauthier et al. [Gauthier2021] report VPT of approximately 5 Lyapunov times for NVAR with $k=1$, $d=2$, matching or slightly exceeding a tuned ESN with 500 nodes.

---

## References

- [Gauthier2021] Gauthier, D.J., Bollt, E., Griffith, A., & Barbosa, W.A.S. (2021). Next generation reservoir computing. *Nature Communications*, 12, 5564.
- [Grassberger1983] Grassberger, P. & Procaccia, I. (1983). Characterization of strange attractors. *Physical Review Letters*, 50(5), 346–349.
- [Lorenz1963] Lorenz, E.N. (1963). Deterministic nonperiodic flow. *Journal of Atmospheric Sciences*, 20(2), 130–141.
