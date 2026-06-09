# JAX: Differentiable Programming for Biology

What if you could take the gradient of a simulation? Not a surrogate model of the simulation, not a finite-difference approximation — the exact gradient, computed by tracing the actual computation graph of your numerical solver, your loss function, and your parameter-to-prediction mapping? For parameter estimation in ODE models, this would mean gradient-based optimization with exact sensitivity information: instead of evaluating the objective function thousands of times to approximate a gradient, you compute the gradient exactly in a single pass. For training neural networks on biological time-series data, it would mean seamless backpropagation through ODE solvers. For protein design, it would mean gradients through molecular simulations.

**JAX** (Google, 2018) is a Python library that combines NumPy-like syntax with **automatic differentiation (AD)**, **JIT compilation to XLA** (Accelerated Linear Algebra, which targets CPU/GPU/TPU), and **functional vectorization**. These three capabilities together enable a programming paradigm — **differentiable programming** — where gradients of arbitrary numerical computations are available automatically. For computational biology, JAX unlocks gradient-based parameter estimation for ODE models, training Neural ODEs, and high-performance Monte Carlo likelihood evaluations.

## The Three Core Transformations

JAX provides composable function transformations:

| Transform | Symbol | What it does |
|-----------|--------|-------------|
| `jax.grad` | $\nabla$ | Computes gradient of scalar-valued function |
| `jax.jit` | — | JIT compiles function to XLA (CPU/GPU/TPU) |
| `jax.vmap` | — | Vectorizes function over a batch dimension |
| `jax.jacobian` | $J$ | Computes full Jacobian matrix |
| `jax.hessian` | $H$ | Computes Hessian matrix |

These transforms **compose**: `jax.jit(jax.vmap(jax.grad(f)))` JIT compiles a batched gradient computation.

## Automatic Differentiation: Exact Gradients for Free

JAX implements **reverse-mode AD** (backpropagation) via `jax.grad` and **forward-mode AD** via `jax.jacfwd`. These compute exact gradients — not finite-difference approximations — by tracing the computation graph:

```python
import jax
import jax.numpy as jnp
import numpy as np

# Enable 64-bit precision (important for biology)
from jax import config
config.update("jax_enable_x64", True)

# Define a biological cost function: sum of squared residuals
# from ODE parameter estimation
def hill_response(x, alpha, K, n):
    """Hill function: fraction active."""
    return alpha * (x / K)**n / (1 + (x / K)**n)

def loss(params, x_data, y_data):
    """MSE loss for Hill function fitting."""
    alpha, log_K, log_n = params
    K = jnp.exp(log_K)  # enforce positivity via log-space
    n = jnp.exp(log_n)
    y_pred = hill_response(x_data, alpha, K, n)
    return jnp.mean((y_pred - y_data)**2)

# Generate synthetic data
x_data = jnp.linspace(0.1, 20, 50)
true_params = jnp.array([1.0, jnp.log(5.0), jnp.log(2.0)])
y_data = hill_response(x_data, 1.0, 5.0, 2.0) + 0.05 * jax.random.normal(
    jax.random.PRNGKey(0), shape=(50,)
)

# Gradient of loss w.r.t. params (exact, not finite difference)
grad_loss = jax.grad(loss, argnums=0)   # gradient w.r.t. first argument

# Verify gradient at initial point
params0 = jnp.array([0.8, jnp.log(4.0), jnp.log(1.8)])
g = grad_loss(params0, x_data, y_data)
print(f"Gradient at initial point: {g}")

# Gradient-based optimization with Adam
def adam_optimize(loss_fn, params, x_data, y_data, n_steps=2000, lr=0.01):
    """JAX-compatible Adam optimizer from scratch."""
    grad_fn = jax.jit(jax.grad(loss_fn, argnums=0))
    
    # Adam state
    m = jnp.zeros_like(params)  # first moment
    v = jnp.zeros_like(params)  # second moment
    beta1, beta2, eps = 0.9, 0.999, 1e-8
    
    loss_history = []
    
    for step in range(1, n_steps + 1):
        g = grad_fn(params, x_data, y_data)
        m = beta1 * m + (1 - beta1) * g
        v = beta2 * v + (1 - beta2) * g**2
        m_hat = m / (1 - beta1**step)
        v_hat = v / (1 - beta2**step)
        params = params - lr * m_hat / (jnp.sqrt(v_hat) + eps)
        
        if step % 200 == 0:
            l = loss_fn(params, x_data, y_data)
            loss_history.append(float(l))
            print(f"Step {step:4d}: loss={l:.6f}")
    
    return params, loss_history

fitted_params, history = adam_optimize(loss, params0, x_data, y_data)
alpha_fit = float(fitted_params[0])
K_fit = float(jnp.exp(fitted_params[1]))
n_fit = float(jnp.exp(fitted_params[2]))
print(f"\nFitted: alpha={alpha_fit:.3f}, K={K_fit:.3f}, n={n_fit:.3f}")
print(f"True:   alpha=1.000, K=5.000, n=2.000")
```

## jit: JIT Compilation to XLA

`jax.jit` compiles a Python function using the XLA compiler:

```python
@jax.jit
def ode_rhs(state, t, params):
    """
    Repressilator ODE right-hand side — JIT compiled.
    JAX traces the computation graph on first call; subsequent calls use compiled code.
    """
    alpha, alpha0, n, beta = params[0], params[1], params[2], params[3]
    m = state[:3]
    p = state[3:]
    p_rolled = jnp.roll(p, 1)  # p[j] for j = (i-1) mod 3
    dm = -m + alpha / (1 + p_rolled**n) + alpha0
    dp = -beta * (p - m)
    return jnp.concatenate([dm, dp])

# First call: traces and compiles (slow)
state0 = jnp.array([0.1, 0.2, 0.3, 0.1, 0.2, 0.3])
params = jnp.array([100.0, 1e-4, 2.0, 1.0])

import time
start = time.perf_counter()
rhs_val = ode_rhs(state0, 0.0, params)
print(f"First call (compilation): {(time.perf_counter()-start)*1000:.1f} ms")

# Second call: compiled (fast)
start = time.perf_counter()
for _ in range(10000):
    rhs_val = ode_rhs(state0, 0.0, params)
print(f"Compiled ({10000} calls): {(time.perf_counter()-start)*1000:.1f} ms total")
```

## vmap: Vectorization Over Batches

`jax.vmap` (vectorized map) transforms a function that operates on a single input into one that operates on a batch of inputs simultaneously — without writing any loops:

```python
# Evaluate ODE RHS for 1000 different parameter sets simultaneously
@jax.jit
def rhs_single(params):
    """RHS at fixed state for a single parameter set."""
    state = jnp.array([1.0, 1.0, 1.0, 0.5, 0.5, 0.5])
    return ode_rhs(state, 0.0, params)

# Batch version: apply over first axis (batch of parameter sets)
rhs_batched = jax.jit(jax.vmap(rhs_single))

# 1000 random parameter sets
rng = jax.random.PRNGKey(42)
params_batch = jax.random.uniform(rng, shape=(1000, 4),
                                   minval=jnp.array([10, 1e-6, 1.5, 0.5]),
                                   maxval=jnp.array([500, 1e-3, 4.0, 5.0]))

# Evaluate all 1000 simultaneously
start = time.perf_counter()
rhs_all = rhs_batched(params_batch)
rhs_all.block_until_ready()  # wait for async GPU computation to complete
print(f"1000 batch evaluations: {(time.perf_counter()-start)*1000:.1f} ms")
print(f"Output shape: {rhs_all.shape}")  # (1000, 6)
```

## Neural ODEs: A JAX Use Case

**Neural ODEs** parameterize the ODE right-hand side with a neural network, enabling end-to-end gradient-based training through ODE integration. JAX + `diffrax` makes this tractable:

```python
# pip install diffrax optax equinox
import diffrax
import equinox as eqx
import optax

class NeuralODE(eqx.Module):
    """ODE whose RHS is a small neural network."""
    mlp: eqx.nn.MLP
    
    def __init__(self, n_species, hidden=32, key=None):
        self.mlp = eqx.nn.MLP(
            in_size=n_species, out_size=n_species,
            width_size=hidden, depth=2, key=key
        )
    
    def __call__(self, t, y, args):
        return self.mlp(y)

# Training: minimize MSE between predicted and observed trajectories
@eqx.filter_jit
def train_step(model, y_obs, t_obs, optimizer_state, optimizer):
    def loss_fn(model):
        solution = diffrax.diffeqsolve(
            diffrax.ODETerm(model),
            diffrax.Dopri5(),
            t0=t_obs[0], t1=t_obs[-1],
            dt0=0.1,
            y0=y_obs[0],
            saveat=diffrax.SaveAt(ts=t_obs)
        )
        return jnp.mean((solution.ys - y_obs)**2)
    
    loss, grads = eqx.filter_value_and_grad(loss_fn)(model)
    updates, new_state = optimizer.update(grads, optimizer_state)
    new_model = eqx.apply_updates(model, updates)
    return new_model, new_state, loss
```

## Why This Matters

JAX changes the scope of what is computationally tractable in biology:
- **Parameter estimation** for ODE models becomes gradient-based (millions of times faster than derivative-free methods for smooth objectives)
- **Neural ODEs** learn mechanistic dynamics from time-series data
- **Differentiable MD** enables gradients through physical simulations for protein design
- **GPU/TPU acceleration** is transparent — the same code runs on a laptop CPU or a data center TPU
- **Composable transforms** (`grad`, `jit`, `vmap`) replace boilerplate with mathematical abstraction

For anyone moving between pure numerical simulation and machine learning for biology, JAX is the unifying framework.
