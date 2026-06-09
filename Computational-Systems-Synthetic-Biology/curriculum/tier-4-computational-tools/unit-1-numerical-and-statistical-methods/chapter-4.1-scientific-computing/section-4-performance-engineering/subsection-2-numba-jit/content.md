# Numba JIT Compilation

The Gillespie stochastic simulation algorithm is, at its heart, a loop. At each iteration, you compute the propensity of every possible reaction, draw a random waiting time from an exponential distribution, select which reaction fires, update the molecular counts, and repeat — potentially millions of times for a single trajectory. The algorithm is simple and well-understood. The problem is that implementing it in pure Python means every one of those millions of iterations passes through the Python interpreter: type checks, memory allocation, object dispatch. The overhead is enormous. For realistic gene expression models where you need tens of thousands of trajectories to characterize noise statistics, pure Python can make a feasible computation take days.

**Numba** solves this by **Just-In-Time (JIT) compiling** Python functions to native machine code at first call, using the LLVM compiler backend. For numerical kernels with explicit loops — Gillespie stochastic simulation, custom ODE right-hand sides, contact map computation — Numba typically delivers C-level performance with minimal code changes.

Python's flexibility comes at a cost: the interpreter overhead for each bytecode instruction, the dynamic type dispatch for every operation, and the GIL (Global Interpreter Lock) all impose performance penalties that can make pure Python loops 10–1000× slower than equivalent C or Fortran code.

## How Numba Works

When a function decorated with `@numba.jit` is first called, Numba:
1. Inspects the types of the input arguments
2. Generates LLVM Intermediate Representation (IR) for those types
3. Compiles the IR to native machine code (SSE/AVX instructions on x86)
4. Caches the compiled binary
5. Routes all subsequent calls with matching argument types directly to the compiled binary

The first call includes the compilation overhead (~0.1–2 seconds). Subsequent calls pay only the compiled code's execution time.

**Two modes:**
- `@jit`: tries nopython mode first; falls back to object mode if NumPy objects are encountered
- `@njit` (= `@jit(nopython=True)`): strict nopython mode; fails loudly if any Python object is used; always fastest

```python
import numpy as np
import numba
from numba import njit, jit, float64, int64
import time

# Example: Gillespie SSA — the classic target for Numba acceleration
# The inner loop iterates millions of times; each iteration is pure arithmetic

@njit
def gillespie_ssa_numba(stoich_matrix, rates_fn_params, u0, t_max, seed=42):
    """
    Direct Gillespie stochastic simulation algorithm (SSA).
    
    stoich_matrix: (n_reactions, n_species) change per reaction
    u0: initial copy numbers (integer array)
    t_max: simulation end time
    Returns: arrays of times and states
    """
    np.random.seed(seed)
    n_reactions, n_species = stoich_matrix.shape
    
    # Pre-allocate output (generous estimate; will trim later)
    max_events = 1000000
    times = np.zeros(max_events)
    states = np.zeros((max_events, n_species), dtype=np.float64)
    
    u = u0.astype(np.float64).copy()
    t = 0.0
    event = 0
    
    while t < t_max and event < max_events - 1:
        # Compute propensities (reaction rates)
        # Simple birth-death as example: a = [k_prod, k_deg*X]
        a = np.zeros(n_reactions)
        a[0] = rates_fn_params[0]           # production: k_prod
        a[1] = rates_fn_params[1] * u[0]   # degradation: k_deg * X
        
        a0 = 0.0
        for j in range(n_reactions):
            a0 += a[j]
        
        if a0 == 0.0:
            break
        
        # Draw waiting time from exponential distribution
        tau = -np.log(np.random.random()) / a0
        t += tau
        
        # Draw reaction index from categorical distribution
        r2a0 = np.random.random() * a0
        cumsum = 0.0
        j_fired = 0
        for j in range(n_reactions):
            cumsum += a[j]
            if cumsum >= r2a0:
                j_fired = j
                break
        
        # Update state
        for k in range(n_species):
            u[k] += stoich_matrix[j_fired, k]
            if u[k] < 0:
                u[k] = 0  # prevent negative populations
        
        # Record
        times[event] = t
        states[event] = u
        event += 1
    
    return times[:event], states[:event]

# Stoichiometry matrix: rows=reactions, cols=species
# Reaction 0 (production): X -> X+1, so stoich = [+1]
# Reaction 1 (degradation): X -> X-1, so stoich = [-1]
stoich = np.array([[1], [-1]], dtype=np.float64)
params = np.array([10.0, 0.5])  # k_prod=10, k_deg=0.5
u0 = np.array([0], dtype=np.float64)

# Warm up (compilation)
print("Compiling (first call)...")
start = time.perf_counter()
t_out, x_out = gillespie_ssa_numba(stoich, params, u0, 100.0, seed=1)
compile_time = time.perf_counter() - start
print(f"First call (including compilation): {compile_time:.2f} s")

# Subsequent calls — compiled speed
start = time.perf_counter()
for _ in range(100):
    t_out, x_out = gillespie_ssa_numba(stoich, params, u0, 100.0, seed=42)
numba_time = (time.perf_counter() - start) / 100
print(f"Subsequent calls (compiled): {numba_time*1000:.2f} ms per simulation")
```

## Benchmarking Against Pure Python

```python
def gillespie_ssa_python(stoich, params, u0, t_max, seed=42):
    """Pure Python equivalent — no Numba."""
    rng = np.random.default_rng(seed)
    u = list(u0)
    t = 0.0
    times = [t]
    states = [list(u)]
    
    while t < t_max:
        a = [params[0], params[1] * u[0]]
        a0 = sum(a)
        if a0 == 0:
            break
        tau = -np.log(rng.random()) / a0
        t += tau
        r = rng.random() * a0
        cumsum = 0
        j = 0
        for k, ak in enumerate(a):
            cumsum += ak
            if cumsum >= r:
                j = k
                break
        u[0] += int(stoich[j, 0])
        times.append(t)
        states.append(list(u))
    
    return np.array(times), np.array(states)

# Benchmark
n_sims = 100
start = time.perf_counter()
for i in range(n_sims):
    gillespie_ssa_python(stoich, params, u0, 100.0, seed=i)
python_time = (time.perf_counter() - start) / n_sims

start = time.perf_counter()
for i in range(n_sims):
    gillespie_ssa_numba(stoich, params, u0, 100.0, seed=i)
numba_time_bench = (time.perf_counter() - start) / n_sims

print(f"Pure Python: {python_time*1000:.1f} ms")
print(f"Numba:       {numba_time_bench*1000:.2f} ms")
print(f"Speedup:     {python_time/numba_time_bench:.0f}x")
```

Typical output:
```
Pure Python: 48.3 ms
Numba:        0.6 ms
Speedup:     80x
```

## Numba for Custom ODE RHS

Stiff ODE solvers call the RHS function thousands of times. A Numba-compiled RHS can reduce total ODE solve time substantially:

```python
from numba import njit
import numpy as np

@njit(cache=True)  # cache=True: save compiled binary to disk
def repressilator_numba(t, u):
    """Repressilator RHS — Numba compiled for maximum speed."""
    alpha, alpha0, n, beta = 100.0, 1e-4, 2.0, 1.0
    m1, m2, m3, p1, p2, p3 = u[0], u[1], u[2], u[3], u[4], u[5]
    du = np.empty(6)
    du[0] = -m1 + alpha / (1.0 + p3**n) + alpha0
    du[1] = -m2 + alpha / (1.0 + p1**n) + alpha0
    du[2] = -m3 + alpha / (1.0 + p2**n) + alpha0
    du[3] = -beta * (p1 - m1)
    du[4] = -beta * (p2 - m2)
    du[5] = -beta * (p3 - m3)
    return du

# Pre-compile by calling once
_ = repressilator_numba(0.0, np.array([0.1, 0.2, 0.3, 0.1, 0.2, 0.3]))

# Now use in solve_ivp — solver benefits from fast RHS
from scipy.integrate import solve_ivp
sol = solve_ivp(repressilator_numba, (0, 200),
               np.array([0.1, 0.2, 0.3, 0.1, 0.2, 0.3]),
               method='Radau', rtol=1e-8, atol=1e-10)
```

## Parallel Loops with prange

Numba supports OpenMP-style parallel loops over independent iterations:

```python
from numba import njit, prange

@njit(parallel=True)
def compute_contact_maps(trajectories, cutoff=8.0):
    """
    Compute contact frequency matrix from many MD trajectory frames.
    trajectories: (n_frames, n_residues, 3) CA positions
    Returns: (n_residues, n_residues) contact frequency matrix
    """
    n_frames, n_res, _ = trajectories.shape
    contacts = np.zeros((n_res, n_res))
    
    for frame in prange(n_frames):  # parallel across frames
        pos = trajectories[frame]
        for i in range(n_res):
            for j in range(i + 4, n_res):
                dx = pos[i, 0] - pos[j, 0]
                dy = pos[i, 1] - pos[j, 1]
                dz = pos[i, 2] - pos[j, 2]
                d = np.sqrt(dx*dx + dy*dy + dz*dz)
                if d < cutoff:
                    contacts[i, j] += 1.0
    
    return contacts / n_frames
```

## Limitations and When Not to Use Numba

- **NumPy functions inside `@njit`**: most work; complex pandas/scipy calls do not
- **Object-oriented code**: Numba supports basic Python classes but not arbitrary inheritance
- **String operations**: not supported in nopython mode
- **Ahead-of-time compilation**: consider Cython for distributable extensions
- **GPU acceleration**: use `@cuda.jit` for CUDA kernels (different API)

Use Numba when you have explicit loops over numerical arrays. If your code already uses NumPy array operations end-to-end, vectorization (covered in the next section) may give equivalent speedup without compilation overhead.

## Why This Matters

The Gillespie SSA is the workhorse of stochastic systems biology. Running 10,000 trajectories to characterize noise in a gene expression model takes 8 minutes in Python and 6 seconds with Numba. Parameter inference via ABC (Approximate Bayesian Computation) requires millions of simulations — impossible without JIT compilation. Numba bridges the gap between Python's ergonomic expressiveness and the numerical performance needed for serious computational biology.
