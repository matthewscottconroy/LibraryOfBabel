# Unconstrained Optimization

You have built a model of the repressilator. You have written down the six coupled ODEs, implemented them in Python, and confirmed that the solution oscillates for the parameter values in Elowitz and Leibler's original paper. Now you want to go further: you have your own noisy fluorescence time-series data from a new synthetic circuit, and you want to infer the parameters — the production rates, the Hill coefficients, the degradation rates — that make the model match your data. This is a parameter estimation problem, and it is, at heart, a problem of finding the minimum of a function. The function is the discrepancy between your model's output and your data, measured as a sum of squared residuals. The variables are the parameters. You want to find the parameter values that make the discrepancy as small as possible.

Unconstrained optimization — finding the minimum of a function $f(\theta)$ over all of $\mathbb{R}^n$ — appears throughout computational biology: fitting ODE model parameters to experimental time courses, minimizing force field energy during molecular energy minimization, training neural network weights, and calibrating statistical models to sequencing data. The choice of optimizer depends on whether gradients are available, how smooth the objective is, and how many parameters are involved.

## Gradient Descent and Its Variants

**Gradient descent** is the conceptual foundation of almost all continuous optimization:

$$\theta_{k+1} = \theta_k - \alpha \nabla f(\theta_k)$$

The **learning rate** $\alpha$ controls step size. If $\alpha$ is too large, the iteration diverges; too small, and convergence is glacially slow. For a quadratic function $f(\theta) = \frac{1}{2}\theta^T A \theta$, the optimal learning rate is $\alpha = 2/(\lambda_{\max} + \lambda_{\min})$, but this requires knowledge of the Hessian eigenvalues.

**Stochastic gradient descent (SGD)** computes the gradient on a random mini-batch of data points rather than the full dataset. For training on $n$ data points with mini-batch size $b$:

- Full gradient: $O(n)$ cost per step, exact
- Mini-batch: $O(b)$ cost per step, noisy estimate; $b \ll n$
- The noise in the gradient can actually help escape sharp local minima

**Adam (Adaptive Moment Estimation)** is the dominant optimizer for neural network training. It maintains per-parameter learning rates that adapt based on first and second moment estimates of the gradient:

$$m_{k+1} = \beta_1 m_k + (1-\beta_1) g_k$$
$$v_{k+1} = \beta_2 v_k + (1-\beta_2) g_k^2$$
$$\theta_{k+1} = \theta_k - \frac{\alpha}{\sqrt{\hat{v}_{k+1}} + \epsilon} \hat{m}_{k+1}$$

where $\hat{m}$ and $\hat{v}$ are bias-corrected estimates, and typical defaults are $\beta_1 = 0.9$, $\beta_2 = 0.999$, $\epsilon = 10^{-8}$.

## Quasi-Newton Methods: L-BFGS-B

For smooth objective functions where gradients are available but the Hessian is too expensive to compute, **quasi-Newton methods** build an approximation to the inverse Hessian using gradient history. **L-BFGS-B** (Limited-memory Broyden-Fletcher-Goldfarb-Shanno with Bounds) is the standard choice for parameter estimation in biological models:

- Uses the last $m$ gradient vectors (typically $m = 10$) to approximate $H^{-1}$
- Memory: $O(mn)$ instead of $O(n^2)$ for full BFGS
- Handles bound constraints: $\ell \leq \theta \leq u$
- Superlinear convergence near the solution

```python
import numpy as np
from scipy.optimize import minimize
from scipy.integrate import solve_ivp

# Parameter estimation: fit repressilator model to noisy data
def repressilator(t, u, alpha, alpha0, n, beta):
    m1, m2, m3, p1, p2, p3 = u
    dm1 = -m1 + alpha / (1 + p3**n) + alpha0
    dm2 = -m2 + alpha / (1 + p1**n) + alpha0
    dm3 = -m3 + alpha / (1 + p2**n) + alpha0
    dp1 = -beta * (p1 - m1)
    dp2 = -beta * (p2 - m2)
    dp3 = -beta * (p3 - m3)
    return [dm1, dm2, dm3, dp1, dp2, dp3]

# Generate synthetic "data" with known parameters + noise
true_params = [100, 1e-4, 2.0, 1.0]  # alpha, alpha0, n, beta
u0 = [0.1, 0.2, 0.3, 0.1, 0.2, 0.3]
t_data = np.linspace(0, 100, 50)

sol_true = solve_ivp(lambda t, u: repressilator(t, u, *true_params),
                     (0, 100), u0, t_eval=t_data, method='Radau',
                     rtol=1e-8, atol=1e-10)
data = sol_true.y[3:, :] + 0.5 * np.random.default_rng(42).standard_normal((3, 50))

def objective(log_params):
    """
    Sum of squared residuals between simulated and observed protein data.
    Optimize in log-space to enforce positivity without explicit bounds.
    """
    alpha, alpha0, n, beta = np.exp(log_params)
    try:
        sol = solve_ivp(lambda t, u: repressilator(t, u, alpha, alpha0, n, beta),
                       (0, 100), u0, t_eval=t_data, method='Radau',
                       rtol=1e-6, atol=1e-8)
        if not sol.success:
            return 1e10
        residuals = sol.y[3:, :] - data
        return np.sum(residuals**2)
    except Exception:
        return 1e10

# Initial guess: perturbed true parameters
x0 = np.log(true_params) + 0.3 * np.random.standard_normal(4)

result = minimize(
    objective,
    x0,
    method='L-BFGS-B',
    options={'maxiter': 500, 'ftol': 1e-12, 'gtol': 1e-8}
)

fitted_params = np.exp(result.x)
print(f"True:   alpha={true_params[0]:.1f}, n={true_params[2]:.2f}, beta={true_params[3]:.3f}")
print(f"Fitted: alpha={fitted_params[0]:.1f}, n={fitted_params[2]:.2f}, beta={fitted_params[3]:.3f}")
print(f"Final objective: {result.fun:.4f}")
```

## Nelder-Mead Simplex: Gradient-Free Search

When the objective is noisy, discontinuous, or implemented in a way that makes gradients inaccessible, **Nelder-Mead** provides a robust gradient-free alternative. It maintains a simplex of $n+1$ points and transforms it through reflection, expansion, contraction, and shrinkage operations.

```python
from scipy.optimize import minimize

def noisy_objective(params):
    """Objective with measurement noise — gradients unreliable."""
    alpha, n, beta = params
    if alpha <= 0 or n <= 0 or beta <= 0:
        return 1e10
    sol = solve_ivp(lambda t, u: repressilator(t, u, alpha, 1e-4, n, beta),
                    (0, 100), u0, t_eval=t_data, method='Radau',
                    rtol=1e-4, atol=1e-6)
    residuals = sol.y[3:, :] - data
    # Add artificial noise to simulate experimental uncertainty
    noise = 0.1 * np.random.standard_normal()
    return np.sum(residuals**2) + noise

result_nm = minimize(
    noisy_objective,
    x0=[90, 2.1, 0.9],   # starting simplex vertex
    method='Nelder-Mead',
    options={'maxiter': 2000, 'xatol': 1e-4, 'fatol': 1e-4, 'adaptive': True}
)

print(f"Nelder-Mead: {result_nm.x}")
```

**Limitation:** Nelder-Mead scales poorly beyond ~10 dimensions. For higher-dimensional problems without gradients, use differential evolution or Bayesian optimization.

## Powell's Method and COBYLA

For smooth, gradient-free problems with moderate dimensionality (10–100 parameters), **Powell's method** (`method='Powell'`) performs directional line searches along conjugate directions — often faster than Nelder-Mead and more numerically robust. **COBYLA** handles nonlinear inequality constraints without requiring gradients, useful when parameter combinations must satisfy biological constraints (e.g., the steady-state concentration of a metabolite must remain positive).

## Choosing an Optimizer

| Method | Gradient needed | Constraints | Dimensionality | Best for |
|--------|----------------|-------------|----------------|----------|
| L-BFGS-B | Yes | Bounds | Up to ~10⁴ | ODE fitting, ML |
| Adam | Yes | None | Up to ~10⁷ | Neural networks |
| Nelder-Mead | No | None | ≤ 10 | Noisy/discrete objectives |
| Powell | No | None | 10–100 | Smooth, gradient-free |
| Differential evolution | No | Bounds | 10–100 | Global search |

## Why This Matters

Parameter estimation is one of the most computationally intensive tasks in systems biology: fitting an ODE model with 20 parameters to time-series data typically requires thousands of objective function evaluations, each involving a full ODE solve. Choosing the right optimizer — and formulating the objective function correctly (log-space parameters, regularization, proper noise models) — can be the difference between a successful fit in minutes and an intractable search.
