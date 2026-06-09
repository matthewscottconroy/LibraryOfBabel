# Parallelization in Computational Biology

The most satisfying class of computational problems in systems biology is the embarrassingly parallel kind. You want to explore how a signaling network behaves across a grid of 10,000 parameter combinations — each combination requires a separate ODE solve, and no solve depends on any other. You want to estimate the statistical distribution of protein noise by running 100,000 Gillespie trajectories. You want to build a bootstrap confidence interval on a correlation coefficient by resampling 10,000 times. In each case, the computations are completely independent. You have a 16-core workstation sitting idle. There is no reason — other than not knowing how — to do these computations sequentially.

Modern CPUs have 8–128 cores; HPC clusters have thousands. **Parallelization** exploits this by running independent computations simultaneously. In computational biology, the most common parallelization targets are **embarrassingly parallel** problems: parameter scans (run the same ODE model with 10,000 different parameter sets), Monte Carlo simulations (run 10,000 Gillespie trajectories), bootstrap resampling, and cross-validation folds. These require no inter-process communication and scale linearly with the number of cores.

## Python's GIL and Why It Matters

Python's **Global Interpreter Lock (GIL)** prevents multiple threads from executing Python bytecode simultaneously. This means `threading.Thread` cannot parallelize pure Python computation. The two solutions are:

1. **`multiprocessing`**: spawn separate Python processes (each with its own memory space and GIL); true parallelism for Python code
2. **NumPy/SciPy/Numba operations**: release the GIL internally; threading works for NumPy-heavy code

## ProcessPoolExecutor: The Standard Parallel Workhorse

```python
from concurrent.futures import ProcessPoolExecutor, as_completed
import numpy as np
from scipy.integrate import solve_ivp
import time

def repressilator(t, u, alpha, n, beta):
    m1, m2, m3, p1, p2, p3 = u
    alpha0 = 1e-4
    dm1 = -m1 + alpha / (1 + p3**n) + alpha0
    dm2 = -m2 + alpha / (1 + p1**n) + alpha0
    dm3 = -m3 + alpha / (1 + p2**n) + alpha0
    dp1 = -beta * (p1 - m1)
    dp2 = -beta * (p2 - m2)
    dp3 = -beta * (p3 - m3)
    return [dm1, dm2, dm3, dp1, dp2, dp3]

def simulate_one(params):
    """Run a single ODE simulation — this function runs in a worker process."""
    alpha, n, beta, seed = params
    u0 = [0.1, 0.2, 0.3, 0.1, 0.2, 0.3]
    try:
        sol = solve_ivp(
            lambda t, u: repressilator(t, u, alpha, n, beta),
            (0, 200), u0,
            method='Radau', rtol=1e-6, atol=1e-9
        )
        if sol.success:
            return {'alpha': alpha, 'n': n, 'beta': beta,
                    'max_protein': sol.y[3:].max(), 'oscillatory': True}
        return None
    except Exception:
        return None

# Generate parameter combinations
rng = np.random.default_rng(42)
n_sims = 1000
params_list = [
    (rng.uniform(10, 500),   # alpha
     rng.uniform(1.5, 4.0),  # n
     rng.uniform(0.1, 5.0),  # beta
     int(rng.integers(0, 10000)))  # seed
    for _ in range(n_sims)
]

# Sequential baseline
start = time.time()
results_seq = [simulate_one(p) for p in params_list[:100]]  # subset for timing
t_seq = time.time() - start
print(f"Sequential (100 sims): {t_seq:.1f} s")

# Parallel with 8 workers
start = time.time()
with ProcessPoolExecutor(max_workers=8) as executor:
    futures = [executor.submit(simulate_one, p) for p in params_list]
    results_par = [f.result() for f in as_completed(futures)]
t_par = time.time() - start
print(f"Parallel 8 workers (1000 sims): {t_par:.1f} s")
print(f"Throughput: {n_sims/t_par:.0f} simulations/second")

# Filter successful results
valid = [r for r in results_par if r is not None]
print(f"Successful simulations: {len(valid)}/{n_sims}")
```

## joblib: The scikit-learn Standard

`joblib` provides a simpler interface for parallelizing loops and is the backend used by all scikit-learn parallel operations:

```python
from joblib import Parallel, delayed
import numpy as np

def compute_bootstrap_correlation(expression_matrix, gene_a_idx, gene_b_idx, seed):
    """Compute Pearson correlation on a bootstrap resample."""
    rng = np.random.default_rng(seed)
    n_samples = expression_matrix.shape[1]
    indices = rng.integers(0, n_samples, size=n_samples)
    
    x = expression_matrix[gene_a_idx, indices]
    y = expression_matrix[gene_b_idx, indices]
    return np.corrcoef(x, y)[0, 1]

# Gene expression matrix: 20000 genes × 100 samples
expr = np.random.lognormal(0, 1, (200, 100))  # small example

# Bootstrap CI for correlation between gene 0 and gene 1
n_bootstrap = 10000

# Parallel bootstrap
correlations = Parallel(n_jobs=-1, verbose=0)(  # n_jobs=-1: use all CPUs
    delayed(compute_bootstrap_correlation)(expr, 0, 1, seed)
    for seed in range(n_bootstrap)
)

correlations = np.array(correlations)
ci_low, ci_high = np.percentile(correlations, [2.5, 97.5])
print(f"Bootstrap 95% CI: [{ci_low:.3f}, {ci_high:.3f}]")
```

## Dask: Scaling Beyond a Single Machine

**Dask** provides a familiar NumPy/Pandas API that scales to larger-than-memory datasets and distributed computation across HPC nodes:

```python
import dask.dataframe as dd
import dask.array as da
from dask.distributed import Client, as_completed

# Dask DataFrame: process RNA-seq data larger than RAM
# Each CSV shard is read lazily and processed in parallel
df = dd.read_csv('expression_data/shard_*.csv')
gene_means = df.groupby('gene_id')['TPM'].mean().compute()

# Dask Array: distributed NumPy for large matrices
# Chunked: each chunk fits in memory and is processed independently
expr_large = da.from_array(expression_matrix_hdf5, chunks=(1000, 100))
correlation = da.corrcoef(expr_large)  # lazy computation
result = correlation.compute()  # execute the computation graph

# Connect to a Dask cluster (local scheduler for single machine)
client = Client(n_workers=8, memory_limit='4GB')
print(client.dashboard_link)  # web dashboard at http://localhost:8787

# Submit individual tasks to Dask cluster
futures = [client.submit(simulate_one, p) for p in params_list]
results = client.gather(futures)  # collect all results
```

## HPC Parallelization with SLURM

On a computing cluster with SLURM job scheduler, parameterize the job array:

```bash
#!/bin/bash
#SBATCH --job-name=ode-scan
#SBATCH --array=0-999          # 1000 independent jobs
#SBATCH --cpus-per-task=1      # 1 CPU per job
#SBATCH --mem=2G
#SBATCH --time=01:00:00
#SBATCH --output=logs/job_%A_%a.out

# Pass array index to Python script
python simulate_one_params.py --index $SLURM_ARRAY_TASK_ID
```

```python
# simulate_one_params.py
import argparse
import numpy as np

parser = argparse.ArgumentParser()
parser.add_argument('--index', type=int, required=True)
args = parser.parse_args()

# Load pre-generated parameter sets
params_all = np.load('parameter_grid.npy')
params = params_all[args.index]

# Run simulation with these parameters
result = simulate_one(params)

# Save result indexed by job array index
np.save(f'results/result_{args.index:04d}.npy', result)
```

Snakemake or Nextflow orchestrate these array jobs, merge results, and handle retries — see Chapter 4.5.

## Thread-Level Parallelism for NumPy Operations

For CPU-bound code that is already NumPy-vectorized, the GIL is released during NumPy C-level calls, enabling threading:

```python
from concurrent.futures import ThreadPoolExecutor

def compute_pca_for_sample(sample_data):
    """PCA on a single sample — releases GIL in NumPy SVD."""
    from sklearn.decomposition import PCA
    pca = PCA(n_components=10)
    return pca.fit_transform(sample_data)

# Thread pool for NumPy-heavy work (GIL released in C calls)
with ThreadPoolExecutor(max_workers=8) as executor:
    pca_results = list(executor.map(compute_pca_for_sample, sample_list))
```

## GPU Parallelization with CuPy

For massively parallel numerical operations (matrix factorizations, convolutions, stochastic simulations with thousands of simultaneous trajectories), a GPU offers 1000+ cores:

```python
# CuPy: drop-in NumPy replacement for GPU
import cupy as cp

# Transfer matrix to GPU
expr_gpu = cp.asarray(expression_matrix)

# All NumPy operations now run on GPU
corr_gpu = cp.corrcoef(expr_gpu)  # GPU-accelerated correlation matrix

# Transfer result back to CPU
corr_cpu = cp.asnumpy(corr_gpu)
```

## Why This Matters

The boundary between a feasible and an infeasible computational experiment in systems biology is usually parallelization. A parameter sensitivity analysis with 10,000 ODE solves takes 3 hours sequentially and 25 minutes on 8 cores. An ABC-SMC posterior estimation requiring $10^6$ simulations is impractical without a cluster. Mastering embarrassingly parallel computation — via `ProcessPoolExecutor`, `joblib`, Dask, or SLURM arrays — transforms the scope of biological questions you can address computationally.
