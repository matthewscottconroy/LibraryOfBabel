# Appendix C: Numerical Methods for Reservoir Computing

This appendix covers the numerical methods used in reservoir computing implementations: ODE integration for continuous-time reservoirs, eigenvalue computation for spectral analysis, linear system solvers for readout training, and practical guidelines for reproducible experiments.

---

## C.1 ODE Integration

### Why ODE Integration Matters

Continuous-time reservoir networks (Maass's LSM, rate-coded neural models, physical systems) evolve according to ordinary differential equations (ODEs). The most common form in RC is:

$$\tau \frac{d\mathbf{x}}{dt} = -\mathbf{x}(t) + \tanh(W_{\text{res}}\mathbf{x}(t) + W_{\text{in}}\mathbf{u}(t))$$

or more generally $\dot{\mathbf{x}} = f(\mathbf{x}(t), \mathbf{u}(t))$. Discretizing this ODE introduces truncation error; understanding the error is essential for choosing the right integration scheme.

### Euler's Method

**Derivation**: The Taylor expansion of $\mathbf{x}(t + h)$ around $t$ gives:

$$\mathbf{x}(t+h) = \mathbf{x}(t) + h f(\mathbf{x}(t), \mathbf{u}(t)) + \frac{h^2}{2} \frac{d^2\mathbf{x}}{dt^2} + O(h^3)$$

Euler's method truncates after the first-order term:

$$\mathbf{x}_{n+1} = \mathbf{x}_n + h f(\mathbf{x}_n, \mathbf{u}_n)$$

**Error analysis**: The local truncation error (error per step) is $O(h^2)$; the global error (accumulated over $T/h$ steps) is $O(h)$ — first-order accuracy. For smooth dynamics, the error grows proportionally to the step size.

**Stability**: The Euler method is stable for the test equation $\dot{x} = \lambda x$ only when $|1 + h\lambda| \leq 1$. For purely real $\lambda < 0$ (stable dynamics), this requires $h \leq 2/|\lambda|$. For reservoir networks with eigenvalues near the stability boundary, Euler may require small $h$ to remain stable.

### Runge-Kutta 4 (RK4)

RK4 achieves fourth-order accuracy by computing four slope estimates per step:

$$k_1 = f(\mathbf{x}_n, t_n)$$
$$k_2 = f\left(\mathbf{x}_n + \frac{h}{2}k_1, t_n + \frac{h}{2}\right)$$
$$k_3 = f\left(\mathbf{x}_n + \frac{h}{2}k_2, t_n + \frac{h}{2}\right)$$
$$k_4 = f\left(\mathbf{x}_n + h k_3, t_n + h\right)$$
$$\mathbf{x}_{n+1} = \mathbf{x}_n + \frac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4)$$

**Derivation**: RK4 is constructed to match the Taylor series of the true solution through order $h^4$. We derive the weights $b_1, b_2, b_3, b_4$ and nodes $c_2, c_3, c_4$ by requiring:

$$\frac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4) = hf + \frac{h^2}{2}f' + \frac{h^3}{6}f'' + \frac{h^4}{24}f''' + O(h^5)$$

where primes denote total time derivatives. Writing $k_2 = f + \frac{h}{2}(f_t + f_x f) + O(h^2)$, etc., and matching coefficients of each power of $h$ yields the classic Butcher tableau for RK4. The result is that RK4 has:

- Local truncation error $O(h^5)$
- Global error $O(h^4)$
- 4 function evaluations per step (vs. 1 for Euler)
- Practical accuracy often 10,000$\times$ better than Euler at the same step count

**When to use RK4 vs. Euler in RC**:

| Scenario | Recommendation |
|---|---|
| Mackey-Glass DDE (smooth, stiff) | RK4 with $h = 0.1$ |
| Lorenz system (chaotic) | RK4 with $h \leq 0.01$ |
| Standard ESN (discrete-time) | Euler is exact (already discrete) |
| Soft-body reservoir (mechanical) | RK4 or adaptive (RK45) |
| Rate-coded neuron model | RK4 with $h = \Delta t/10$ |

### Adaptive Step-Size Integration (RK45)

For stiff ODEs or those requiring error control, adaptive step-size methods (like Dormand-Prince RK45 as in `scipy.integrate.solve_ivp`) adjust $h$ to maintain a target local error tolerance. The error is estimated by comparing a 4th and 5th order RK solution at each step.

In Python:

```python
from scipy.integrate import solve_ivp
import numpy as np

def mackey_glass_dde(t, y, history, tau=17, beta=0.2, gamma=0.1, n=10):
    """
    Mackey-Glass DDE: dy/dt = beta*y(t-tau)/(1+y(t-tau)^n) - gamma*y
    Approximated as ODE using linear interpolation for y(t-tau).
    """
    y_tau = history(t - tau)
    return beta * y_tau / (1 + y_tau**n) - gamma * y

def integrate_lorenz(T=100, dt=0.01, sigma=10, rho=28, beta=8/3):
    """Integrate Lorenz system using RK4."""
    def f(t, xyz):
        x, y, z = xyz
        return [sigma*(y-x), x*(rho-z)-y, x*y-beta*z]
    
    sol = solve_ivp(f, [0, T], [0.0, 1.0, 1.05],
                   method='RK45', dense_output=True,
                   rtol=1e-8, atol=1e-10)
    t_eval = np.arange(0, T, dt)
    return t_eval, sol.sol(t_eval).T
```

---

## C.2 Eigenvalue Computation

### Power Iteration

**Algorithm**: Given $A \in \mathbb{R}^{n \times n}$, to find the dominant eigenvalue $\lambda_1$ (largest in absolute value):

1. Initialize $\mathbf{v}^{(0)}$ randomly (unit vector)
2. Iterate: $\mathbf{w}^{(k)} = A\mathbf{v}^{(k-1)}$, $\mathbf{v}^{(k)} = \mathbf{w}^{(k)} / \|\mathbf{w}^{(k)}\|$
3. Eigenvalue estimate: $\lambda_1^{(k)} = (\mathbf{v}^{(k-1)})^\top A \mathbf{v}^{(k-1)}$ (Rayleigh quotient)

**Convergence Analysis**: Let $A$ have eigenvalues $|\lambda_1| > |\lambda_2| \geq \ldots \geq |\lambda_n|$ with eigenvectors $\mathbf{v}_1, \ldots, \mathbf{v}_n$. Write the initial vector as $\mathbf{v}^{(0)} = \sum_i c_i \mathbf{v}_i$ (assuming $c_1 \neq 0$). Then:

$$A^k \mathbf{v}^{(0)} = \lambda_1^k \left(c_1 \mathbf{v}_1 + \sum_{i \geq 2} c_i \left(\frac{\lambda_i}{\lambda_1}\right)^k \mathbf{v}_i\right)$$

Since $|\lambda_i/\lambda_1| < 1$ for $i \geq 2$, the non-dominant components decay exponentially. After normalization:

$$\mathbf{v}^{(k)} \to \mathbf{v}_1 \text{ at rate } \left|\frac{\lambda_2}{\lambda_1}\right|^k$$

The convergence rate is the ratio $|\lambda_2/\lambda_1|^2$ (for the Rayleigh quotient estimate). If $\lambda_1 \approx \lambda_2$, convergence is slow.

**RC Application**: Computing the spectral radius $\rho(W)$ during reservoir initialization. For $N = 1000$, direct eigenvalue computation costs $O(N^3) \approx 10^9$ flops. Power iteration converges in $\sim 100$ iterations, each costing $O(N^2) \approx 10^6$ flops — a 1000$\times$ speedup.

```python
def spectral_radius_power_iteration(W, n_iter=100, tol=1e-6):
    """Estimate spectral radius of W using power iteration."""
    n = W.shape[0]
    v = np.random.randn(n)
    v /= np.linalg.norm(v)
    rho_prev = 0.0
    
    for _ in range(n_iter):
        w = W @ v
        rho = np.linalg.norm(w)
        v = w / rho
        if abs(rho - rho_prev) < tol:
            break
        rho_prev = rho
    
    return rho
```

**Note**: Power iteration finds the dominant eigenvalue in *magnitude*, which may be complex for a real asymmetric matrix like a reservoir weight matrix. For such matrices, use `np.max(np.abs(np.linalg.eigvals(W)))` for exact computation or the power iteration variant for the complex case.

---

## C.3 Solving Linear Systems: The Conjugate Gradient Method

### Context

Readout training requires solving $(X^\top X + \lambda I)\mathbf{w} = X^\top \mathbf{y}$, a symmetric positive-definite (SPD) linear system of size $N \times N$. Direct methods (Cholesky, LU) cost $O(N^3)$. For large $N$, iterative methods are preferable.

### Conjugate Gradient (CG) Overview

CG minimizes the quadratic $q(\mathbf{w}) = \frac{1}{2}\mathbf{w}^\top A\mathbf{w} - \mathbf{b}^\top \mathbf{w}$ (equivalent to solving $A\mathbf{w} = \mathbf{b}$) using $A$-conjugate search directions.

**Algorithm**:
1. Initialize $\mathbf{w}_0 = \mathbf{0}$, $\mathbf{r}_0 = \mathbf{b}$, $\mathbf{d}_0 = \mathbf{r}_0$
2. For $k = 0, 1, 2, \ldots$:
   - $\alpha_k = \|\mathbf{r}_k\|^2 / (\mathbf{d}_k^\top A \mathbf{d}_k)$
   - $\mathbf{w}_{k+1} = \mathbf{w}_k + \alpha_k \mathbf{d}_k$
   - $\mathbf{r}_{k+1} = \mathbf{r}_k - \alpha_k A\mathbf{d}_k$
   - $\beta_k = \|\mathbf{r}_{k+1}\|^2 / \|\mathbf{r}_k\|^2$
   - $\mathbf{d}_{k+1} = \mathbf{r}_{k+1} + \beta_k \mathbf{d}_k$

**Key properties**:
- Each iteration costs one matrix-vector product $A\mathbf{d}_k$: $O(N^2)$ for dense $A$, $O(N)$ for sparse $A$
- Exact solution in at most $N$ iterations (finite termination)
- In practice, convergence to accuracy $\epsilon$ in $O(\sqrt{\kappa(A)}\log(1/\epsilon))$ iterations, where $\kappa(A) = \lambda_{\max}/\lambda_{\min}$ is the condition number

For reservoir readout with $A = X^\top X + \lambda I$: the condition number is $\kappa(A) = (\sigma_1^2 + \lambda)/(\sigma_r^2 + \lambda)$, controlled by the regularization $\lambda$. Larger $\lambda$ gives better conditioning and faster CG convergence.

```python
def conjugate_gradient(A, b, tol=1e-8, max_iter=None):
    """Solve Ax = b where A is SPD."""
    n = len(b)
    if max_iter is None:
        max_iter = n
    
    x = np.zeros(n)
    r = b.copy()
    d = r.copy()
    r_sq = r @ r
    
    for k in range(max_iter):
        Ad = A @ d
        alpha = r_sq / (d @ Ad)
        x += alpha * d
        r -= alpha * Ad
        r_sq_new = r @ r
        
        if np.sqrt(r_sq_new) < tol:
            break
        
        beta = r_sq_new / r_sq
        d = r + beta * d
        r_sq = r_sq_new
    
    return x
```

---

## C.4 Reproducibility in Reservoir Computing Experiments

Reservoir computing experiments involve randomness at multiple stages: reservoir weight initialization, input weight initialization, and (for stochastic training methods) gradient estimation. Reproducibility requires controlling all sources of randomness.

### Random Seed Management

**Rule**: Every experiment should be runnable from a single master seed, with all other seeds derived deterministically.

```python
import numpy as np
import hashlib

def make_seed(base_seed: int, *args) -> int:
    """
    Derive a deterministic seed from a base seed and experiment parameters.
    Ensures that different parameter combinations get different seeds
    but the same combination always gets the same seed.
    """
    key = str(base_seed) + '|' + '|'.join(str(a) for a in args)
    h = hashlib.sha256(key.encode()).hexdigest()
    return int(h[:8], 16) % (2**31)


class ReproducibleESN:
    """
    ESN with full reproducibility control.
    
    Every random decision is made by a dedicated RNG with a
    deterministically derived seed.
    """
    def __init__(self, master_seed: int = 42, N: int = 500,
                 rho: float = 0.95, s_in: float = 0.5):
        self.master_seed = master_seed
        self.N = N
        
        # Separate RNGs for each component
        reservoir_seed = make_seed(master_seed, 'reservoir', N)
        input_seed     = make_seed(master_seed, 'input', N)
        
        rng_res = np.random.RandomState(reservoir_seed)
        rng_in  = np.random.RandomState(input_seed)
        
        # Reservoir weights
        W = rng_res.randn(N, N)
        W[rng_res.rand(N, N) > 0.1] = 0.0
        ev = np.linalg.eigvals(W)
        self.W_res = W * rho / (np.max(np.abs(ev)) + 1e-10)
        
        # Input weights (set later when input dim is known)
        self._rng_in = rng_in
        self._s_in = s_in
        self.W_in = None
    
    def init_input_weights(self, d_in: int) -> None:
        self.W_in = self._s_in * self._rng_in.randn(self.N, d_in)
```

### Experiment Logging

Good experiment logs record:

1. **Software versions**: NumPy, SciPy, Python version, OS
2. **Hardware**: CPU/GPU model (matrix operations may differ in floating-point order)
3. **All hyperparameters**: reservoir size, spectral radius, leaking rate, ridge parameter, washout length, random seed
4. **Dataset split**: which samples are in train/validation/test
5. **Results**: all relevant metrics, not just the reported ones
6. **Timestamp and git commit hash**: for traceability

A minimal logging framework using MLflow (detailed in Appendix D):

```python
import mlflow
import platform

def log_esn_experiment(params: dict, metrics: dict, run_name: str = None):
    """
    Log an ESN experiment to MLflow with full provenance information.
    """
    with mlflow.start_run(run_name=run_name):
        # System info
        mlflow.log_param("python_version", platform.python_version())
        mlflow.log_param("numpy_version", np.__version__)
        mlflow.log_param("platform", platform.platform())
        
        # Experiment parameters
        for k, v in params.items():
            mlflow.log_param(k, v)
        
        # Results
        for k, v in metrics.items():
            mlflow.log_metric(k, v)
        
        # Git commit (if in a git repo)
        try:
            import subprocess
            commit = subprocess.check_output(
                ['git', 'rev-parse', 'HEAD'], stderr=subprocess.DEVNULL
            ).decode().strip()
            mlflow.log_param("git_commit", commit[:8])
        except Exception:
            pass


# Example usage
params = {
    'n_reservoir': 500,
    'spectral_radius': 0.95,
    'leaking_rate': 0.3,
    'ridge_alpha': 1e-4,
    'seed': 42,
    'task': 'NARMA10',
}
metrics = {
    'nmse_train': 0.023,
    'nmse_test': 0.031,
    'memory_capacity': 12.4,
}
log_esn_experiment(params, metrics, run_name="NARMA10_baseline")
```

### Common Reproducibility Pitfalls

**Pitfall 1: Forgetting global NumPy state**. `np.random.seed(42)` sets the *global* RNG state; any library call that uses NumPy random (including some SciPy functions) will consume random numbers and change the state. Always use `np.random.RandomState` objects rather than the global RNG.

**Pitfall 2: Inconsistent washout**. The washout period discards the first $W_{\text{washout}}$ timesteps of each run. If the washout is applied inconsistently (e.g., included in one experiment's training data but not another's), results are not comparable.

**Pitfall 3: Train/test leakage**. The reservoir state at the start of the test set is the state at the end of the training set. This is correct and necessary (the reservoir should carry information from training into the test period). But the target sequence must not leak: test targets must not appear in the training loss.

**Pitfall 4: Spectral radius computation precision**. `np.linalg.eigvals` uses a double-precision algorithm. For very large reservoirs ($N > 5000$), round-off error in eigenvalue computation may cause the achieved spectral radius to differ from the target by $\sim 10^{-10}$–$10^{-8}$. This is negligible for most purposes, but should be verified for numerical precision studies.

**Pitfall 5: Not reporting the standard deviation**. A single run of an RC experiment with a specific random seed may not be representative. Always report mean and standard deviation over multiple seeds (at least 5). Results that are statistically indistinguishable (overlapping confidence intervals) should not be ranked.

### Recommended Experimental Protocol

1. Fix all hyperparameters first (using a separate validation set or GCV — never the test set).
2. Run $M = 10$–$30$ independent experiments with different seeds (seed from 0 to $M-1$).
3. Report mean ± std of all metrics over the $M$ runs.
4. Log all runs to MLflow (or equivalent) for traceability.
5. If comparing methods, use the same random seeds for each method to reduce variance from data splits.
6. Report the performance distribution (e.g., boxplot), not just mean and std — distributions with the same mean and std can have very different shapes.
