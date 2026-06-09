# Profiling Python Code

Every computational biologist eventually reaches a moment of reckoning: the simulation that worked perfectly on a small test case now needs to run ten thousand times for a parameter sensitivity analysis, and the estimated runtime is three days. The tempting response is to immediately start rewriting code — replacing loops with NumPy operations, adding Numba decorators, reaching for parallelization. But this is almost always the wrong move, because you almost certainly do not know where the time is actually being spent. Intuition about performance is notoriously unreliable. The function you are certain must be the bottleneck is often fine; the bottleneck is somewhere you never thought to look.

Before optimizing, you must measure. Guessing where a program spends its time is almost always wrong — the actual bottleneck is rarely where intuition says it should be. **Profiling** instruments your code to measure where time and memory are actually spent, directing optimization effort where it will have real impact. In computational biology, where simulations may run for hours, systematic profiling can reduce runtimes by factors of 10–1000.

## The Golden Rule: Measure First, Optimize Second

Premature optimization wastes developer time on irrelevant code paths. A well-known principle (often attributed to Knuth) is that 90% of runtime is spent in 10% of the code. Profiling identifies that 10%.

The profiling workflow is:
1. Write correct code first
2. Profile to identify the hotspot
3. Optimize only the hotspot
4. Measure again to verify improvement
5. Repeat

## cProfile: Function-Level Profiling

Python's built-in `cProfile` module instruments every function call and measures cumulative time per function. It is low overhead (~15%) and the right starting point for any profiling session:

```python
import cProfile
import pstats
import io
from scipy.integrate import solve_ivp
import numpy as np

def repressilator(t, u):
    m1, m2, m3, p1, p2, p3 = u
    alpha, n, beta = 100.0, 2.0, 1.0
    dm1 = -m1 + alpha / (1 + p3**n)
    dm2 = -m2 + alpha / (1 + p1**n)
    dm3 = -m3 + alpha / (1 + p2**n)
    dp1 = -beta * (p1 - m1)
    dp2 = -beta * (p2 - m2)
    dp3 = -beta * (p3 - m3)
    return [dm1, dm2, dm3, dp1, dp2, dp3]

def run_parameter_scan(n_runs=200):
    """Simulate repressilator for many parameter sets."""
    results = []
    u0 = [0.1, 0.2, 0.3, 0.1, 0.2, 0.3]
    for i in range(n_runs):
        sol = solve_ivp(repressilator, (0, 100), u0,
                       method='Radau', rtol=1e-6, atol=1e-9)
        results.append(sol.y[3].max())
    return results

# Profile with cProfile
profiler = cProfile.Profile()
profiler.enable()
results = run_parameter_scan(n_runs=200)
profiler.disable()

# Analyze results
stream = io.StringIO()
stats = pstats.Stats(profiler, stream=stream)
stats.sort_stats('cumulative')
stats.print_stats(20)  # top 20 functions by cumulative time
print(stream.getvalue())
```

Typical output for a simulation-heavy function:
```
   ncalls  tottime  percall  cumtime  percall filename:lineno(function)
      200    0.012    0.000   18.423    0.092 solve_ivp (scipy)
   124853    5.234    0.000    5.234    0.000 repressilator
        1    0.001    0.001   18.430   18.430 run_parameter_scan
```

This immediately shows that `solve_ivp` is the bottleneck, spending 18 seconds on 200 calls. The RHS function `repressilator` is called 124,000 times — a clear target for JIT compilation.

## timeit: Micro-Benchmarking

For comparing two implementations of the same function:

```python
import timeit

# Compare Python list vs NumPy array for RHS computation
def rhs_list(t, u):
    """Returns a Python list."""
    m1, m2, m3, p1, p2, p3 = u
    return [-m1 + 100/(1+p3**2), -m2 + 100/(1+p1**2), -m3 + 100/(1+p2**2),
            -(p1-m1), -(p2-m2), -(p3-m3)]

def rhs_numpy(t, u):
    """Returns a NumPy array."""
    m, p = u[:3], u[3:]
    dm = -m + 100 / (1 + np.roll(p, 1)**2)
    dp = -(p - m)
    return np.concatenate([dm, dp])

u_test = np.array([0.1, 0.2, 0.3, 0.1, 0.2, 0.3])

n_reps = 100000
t_list  = timeit.timeit(lambda: rhs_list(0, u_test), number=n_reps)
t_numpy = timeit.timeit(lambda: rhs_numpy(0, u_test), number=n_reps)

print(f"Python list: {t_list*1e6/n_reps:.2f} μs/call")
print(f"NumPy array: {t_numpy*1e6/n_reps:.2f} μs/call")
```

## line_profiler: Line-by-Line Timing

When cProfile shows that a specific function is slow, `line_profiler` breaks it down line by line:

```python
# Install: pip install line-profiler
# Usage in script:
from line_profiler import LineProfiler

def network_rhs(t, u, params):
    """ODE RHS for a large gene regulatory network."""
    alpha = params['alpha']
    K = params['K']
    n = params['n']
    delta = params['delta']
    
    # Line 1: Hill function computation
    x = (u / K) ** n        # vectorized exponentiation
    
    # Line 2: Regulatory inputs (sparse matrix-vector product)
    activation = alpha @ (x / (1 + x))  # regulatory matrix × Hill outputs
    
    # Line 3: Repression
    repression = params['beta'] @ (1 / (1 + x))
    
    # Line 4: Degradation
    du = activation + repression - delta * u
    return du

lp = LineProfiler()
lp_wrapper = lp(network_rhs)
# Run the profiled function
lp_wrapper(0, u0, params)
lp.print_stats()
```

Output shows exact microseconds per line, identifying whether the bottleneck is the Hill function, the matrix product, or array allocation.

## Memory Profiling

For large-scale simulations with many trajectories, memory is often the limiting resource. `memory_profiler` tracks allocation line by line:

```bash
pip install memory-profiler
python -m memory_profiler simulate_ensemble.py
```

```python
from memory_profiler import profile

@profile
def run_gillespie_ensemble(n_trajectories=10000, t_max=100):
    """
    Run many Gillespie simulations — may run out of memory
    if all trajectories are stored simultaneously.
    """
    # BAD: stores all trajectories in memory at once
    all_trajectories = []
    for i in range(n_trajectories):
        t, states = gillespie_ssa(rates, u0, t_max)
        all_trajectories.append((t, states))  # 10,000 arrays in memory!
    return all_trajectories

# BETTER: compute statistics on-the-fly, discard trajectories
@profile  
def run_gillespie_streaming(n_trajectories=10000, t_max=100):
    """Compute mean/variance online without storing all trajectories."""
    t_grid = np.linspace(0, t_max, 1000)
    sum_x = np.zeros(len(t_grid))
    sum_x2 = np.zeros(len(t_grid))
    
    for i in range(n_trajectories):
        t, states = gillespie_ssa(rates, u0, t_max)
        # Interpolate to common grid and accumulate
        x_interp = np.interp(t_grid, t, states[:, 0])
        sum_x  += x_interp
        sum_x2 += x_interp**2
    
    mean = sum_x / n_trajectories
    variance = sum_x2 / n_trajectories - mean**2
    return t_grid, mean, variance
```

## Profiling in Jupyter with %prun and %lprun

In Jupyter notebooks, magic commands provide convenient profiling:

```python
# Function-level profiling
%prun -s cumulative run_parameter_scan(200)

# Line-level profiling (requires line_profiler extension)
%load_ext line_profiler
%lprun -f repressilator run_parameter_scan(200)

# Memory profiling
%load_ext memory_profiler
%memit run_parameter_scan(200)

# Simple timing
%timeit repressilator(0, u0)   # single line
%%timeit                        # entire cell
sol = solve_ivp(repressilator, (0, 100), u0, method='Radau')
```

## Practical Profiling Strategy

1. **Start with `%prun` in Jupyter** to identify the top 5 slow functions.
2. **Isolate the bottleneck** in a standalone function.
3. **Apply `%lprun`** to that function to find the specific slow lines.
4. **Check if the hotspot is:**
   - Python loops → NumPy vectorization or Numba JIT
   - Slow math functions → NumPy ufuncs or JAX
   - Repeated small allocations → pre-allocate arrays
   - I/O → async I/O, chunked reading, HDF5 compression

## Why This Matters

In computational biology, the difference between a 10-second simulation and a 100-millisecond simulation is the difference between exploratory analysis that takes a day and one that takes 15 minutes. Profiling transforms optimization from guesswork into engineering. For any workflow that runs more than a handful of times, invest in profiling before investing in optimization.
