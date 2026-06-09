# Jacobian Provision for Stiff ODE Solvers

Consider what happens inside a Radau solver at each time step. It needs to solve a nonlinear system of equations using Newton's method, and Newton's method requires computing the Jacobian — the matrix of partial derivatives of your right-hand side function with respect to each species. If you do not provide that Jacobian explicitly, the solver has to estimate it numerically: it perturbs each state variable by a tiny amount, re-evaluates the right-hand side, and divides the difference by the perturbation size. For a system with $n$ species, that means $n$ extra function evaluations per Newton step, per time step.

Now imagine you are fitting a 20-species signaling model to experimental data, which requires solving the ODE a hundred thousand times during the parameter search. Each solve takes $m$ time steps; each step takes several Newton iterations; each iteration evaluates $\mathbf{f}$ twenty extra times for the finite-difference Jacobian. The accumulated cost is enormous. Providing an analytical Jacobian eliminates this overhead entirely and improves both speed and reliability.

Every implicit stiff solver — BDF, Radau, or LSODA — requires the **Jacobian matrix** $J_{ij} = \partial f_i / \partial u_j$ to perform Newton iterations at each step. By default, SciPy approximates the Jacobian using finite differences, evaluating $\mathbf{f}$ twice for each column of $J$. For a system of $n$ ODEs, this costs $n$ extra function evaluations per Newton iteration, per time step. Providing an analytical Jacobian eliminates this overhead entirely and improves both speed and reliability.

## What the Jacobian Represents

For a biological ODE system $\dot{\mathbf{u}} = \mathbf{f}(\mathbf{u})$, the Jacobian evaluated at a point $\mathbf{u}^*$ is the matrix of partial derivatives:

$$J = \begin{pmatrix}
\partial f_1/\partial u_1 & \partial f_1/\partial u_2 & \cdots \\
\partial f_2/\partial u_1 & \partial f_2/\partial u_2 & \cdots \\
\vdots & & \ddots
\end{pmatrix}$$

Implicit solvers solve $(I - h\gamma J)\delta = -F$ at each Newton step, where $\gamma$ is a method-dependent constant. The Jacobian is factorized (LU decomposition) once and reused for multiple Newton iterations, amortizing the $O(n^3)$ cost.

The Jacobian also communicates the **sparsity structure** of the system to the solver: most biological network models are sparse — each species interacts with only a few others — so the Jacobian has mostly zeros. Providing this sparsity allows the solver to use sparse factorization, reducing the cost from $O(n^3)$ to $O(n)$ for sparse systems.

## Analytical Jacobian for a Gene Expression Model

Consider the lac operon model with three species: mRNA ($m$), protein ($p$), and inducer ($I$):

$$\dot{m} = \frac{\alpha}{1 + (p/K)^n} - \delta_m m$$
$$\dot{p} = \beta m - \delta_p p$$
$$\dot{I} = k_{\text{in}} - k_{\text{out}} I$$

The Jacobian is:

$$J = \begin{pmatrix}
-\delta_m & -\frac{\alpha n (p/K)^{n-1}/K}{(1+(p/K)^n)^2} & 0 \\
\beta & -\delta_p & 0 \\
0 & 0 & -k_{\text{out}}
\end{pmatrix}$$

```python
import numpy as np
from scipy.integrate import solve_ivp
import time

def lac_operon(t, u, alpha=100, K=10, n=2, 
               delta_m=1.0, beta=2.0, delta_p=0.5,
               k_in=0.1, k_out=0.3):
    """Lac operon ODE: u = [mRNA, protein, inducer]"""
    m, p, I = u
    hill = 1 / (1 + (p / K)**n)
    dm = alpha * hill - delta_m * m
    dp = beta * m - delta_p * p
    dI = k_in - k_out * I
    return [dm, dp, dI]

def lac_jacobian(t, u, alpha=100, K=10, n=2,
                 delta_m=1.0, beta=2.0, delta_p=0.5,
                 k_in=0.1, k_out=0.3):
    """
    Analytical Jacobian of lac_operon.
    Returns 3x3 matrix J[i,j] = df_i/du_j
    """
    m, p, I = u
    # Derivative of Hill function w.r.t. p
    x = (p / K)**n
    dhill_dp = -alpha * n * x / (p * (1 + x)**2)

    J = np.array([
        [-delta_m,  dhill_dp,  0        ],
        [ beta,    -delta_p,   0        ],
        [ 0,        0,        -k_out    ]
    ])
    return J

u0 = [0.0, 0.0, 0.0]
t_span = (0, 50)

# Without Jacobian: finite difference approximation
start = time.perf_counter()
sol_no_jac = solve_ivp(lac_operon, t_span, u0,
                        method='Radau', rtol=1e-8, atol=1e-10)
t_no_jac = time.perf_counter() - start

# With analytical Jacobian
start = time.perf_counter()
sol_with_jac = solve_ivp(lac_operon, t_span, u0,
                          method='Radau', rtol=1e-8, atol=1e-10,
                          jac=lac_jacobian)
t_with_jac = time.perf_counter() - start

print(f"Without Jacobian: {sol_no_jac.nfev:4d} evals, {t_no_jac*1000:.2f} ms")
print(f"With Jacobian:    {sol_with_jac.nfev:4d} evals, {t_with_jac*1000:.2f} ms")
print(f"Speedup: {t_no_jac/t_with_jac:.1f}x")
```

## Using JAX for Automatic Jacobian Computation

For large or complex systems where writing the Jacobian by hand is error-prone, JAX provides **automatic differentiation** that computes the exact Jacobian analytically:

```python
import jax
import jax.numpy as jnp
from scipy.integrate import solve_ivp
import numpy as np

# Define RHS in JAX
def repressilator_jax(u, alpha=100.0, alpha0=1e-4, n=2.0, beta=1.0):
    m1, m2, m3, p1, p2, p3 = u
    dm1 = -m1 + alpha / (1 + p3**n) + alpha0
    dm2 = -m2 + alpha / (1 + p1**n) + alpha0
    dm3 = -m3 + alpha / (1 + p2**n) + alpha0
    dp1 = -beta * (p1 - m1)
    dp2 = -beta * (p2 - m2)
    dp3 = -beta * (p3 - m3)
    return jnp.array([dm1, dm2, dm3, dp1, dp2, dp3])

# JAX computes exact Jacobian via forward-mode AD
jac_fn = jax.jit(jax.jacobian(repressilator_jax))

def rhs_scipy(t, u):
    return np.array(repressilator_jax(jnp.array(u)))

def jac_scipy(t, u):
    return np.array(jac_fn(jnp.array(u)))

u0 = [0.1, 0.2, 0.3, 0.1, 0.2, 0.3]
sol = solve_ivp(rhs_scipy, (0, 200), u0,
               method='Radau', rtol=1e-8, atol=1e-10,
               jac=jac_scipy)

print(f"Solution with JAX Jacobian: {sol.nfev} evaluations")
```

## Sparse Jacobians for Large Networks

For a network of 100 species where each species interacts with at most 5 others, the Jacobian is sparse with at most 500 non-zero entries out of 10,000. Providing this sparse structure allows the solver to use sparse LU factorization:

```python
from scipy.sparse import csc_matrix
from scipy.integrate import solve_ivp

def sparse_jac(t, u):
    """Return Jacobian as scipy sparse matrix for a large network."""
    # ... compute non-zero entries ...
    rows = [...]  # row indices of non-zero elements
    cols = [...]  # column indices
    data = [...]  # values
    n = len(u)
    return csc_matrix((data, (rows, cols)), shape=(n, n))

# SciPy Radau/BDF accept sparse Jacobians natively
sol = solve_ivp(network_rhs, t_span, u0,
               method='BDF',          # BDF preferred for large sparse systems
               jac=sparse_jac,
               jac_sparsity=sparsity_pattern,  # binary pattern for finite-diff fallback
               rtol=1e-6, atol=1e-9)
```

The `jac_sparsity` argument — a binary matrix with 1s where the Jacobian is non-zero — enables **compressed finite differences**: instead of $n$ separate perturbations, groups of non-overlapping columns can be perturbed simultaneously, reducing the cost from $O(n)$ to $O(\text{colors})$ function evaluations.

## Why This Matters

In parameter estimation workflows, a stiff ODE is typically solved hundreds of thousands of times. The difference between a Jacobian-free run (many wasted function evaluations per step) and an analytical or AD-computed Jacobian (minimal evaluations) translates directly into wall-clock time. For a 20-species signaling network solved $10^5$ times, providing the Jacobian can reduce total simulation time from hours to minutes — the difference between a feasible parameter search and an infeasible one.
