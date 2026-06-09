# Vectorization with NumPy

Consider what happens when you write a Python loop over a million-element array. For each element, Python's interpreter must check the type of the variable, look up the operation in a dispatch table, allocate a result object, manage reference counts, and only then perform the actual arithmetic. The computation itself — the addition, the multiplication, the exponential — takes nanoseconds. The interpreter overhead around it takes microseconds. At a million elements, this overhead alone can consume half a second on fast hardware, and it has nothing to do with the biology you are trying to compute.

NumPy avoids this by letting you describe operations at the level of entire arrays. When you write `xn / (1 + xn)` where `xn` is a NumPy array, Python sends that single high-level instruction down to a compiled C loop that processes all elements without ever returning to the interpreter. Modern processors can execute this loop using SIMD (Single Instruction, Multiple Data) instructions that process 4–16 numbers simultaneously. The difference in speed is not marginal: vectorized NumPy is typically **50–500× faster** than equivalent pure Python loops.

**Vectorization** is the practice of replacing explicit Python loops with NumPy array operations that execute in compiled C or Fortran code. In computational biology — where gene expression matrices contain millions of entries, contact maps require computing distances over thousands of residue pairs, and Monte Carlo simulations evaluate functions over thousands of parameter samples — vectorization is not an optimization technique, it is the baseline requirement for usable code.

## The Core Concept: Eliminating Python Loops

Every Python loop over a large numerical array is a performance red flag. The vectorized replacement is almost always possible and almost always available in NumPy:

```python
import numpy as np
import time

n = 1_000_000
x = np.random.standard_normal(n)

# Slow: explicit Python loop
def hill_function_loop(x, K=1.0, n=2.0):
    result = np.zeros(len(x))
    for i in range(len(x)):
        xn = (x[i] / K) ** n
        result[i] = xn / (1 + xn)
    return result

# Fast: vectorized NumPy
def hill_function_vec(x, K=1.0, n=2.0):
    xn = (x / K) ** n          # element-wise power
    return xn / (1 + xn)       # element-wise division

# Benchmark
start = time.perf_counter()
r_loop = hill_function_loop(x)
t_loop = time.perf_counter() - start

start = time.perf_counter()
r_vec = hill_function_vec(x)
t_vec = time.perf_counter() - start

print(f"Loop:       {t_loop*1000:.1f} ms")
print(f"Vectorized: {t_vec*1000:.1f} ms")
print(f"Speedup:    {t_loop/t_vec:.0f}x")
print(f"Max error:  {np.max(np.abs(r_loop - r_vec)):.2e}")
```

Typical output:
```
Loop:       412.3 ms
Vectorized:   3.8 ms
Speedup:    109x
```

## Broadcasting: Operating on Arrays of Different Shapes

**Broadcasting** is NumPy's mechanism for performing operations on arrays with compatible but different shapes, without explicit replication:

```python
# Broadcast rules:
# 1. Prepend 1s to shape of smaller array
# 2. Dimensions of size 1 are stretched to match larger array
# 3. Shapes must be equal or one must be 1

# Example: compute pairwise Euclidean distances (n_points × n_points matrix)
n = 500
coords = np.random.standard_normal((n, 3))  # n points in 3D

# Naive: double loop (O(n²) Python overhead)
dist_matrix_slow = np.zeros((n, n))
for i in range(n):
    for j in range(n):
        diff = coords[i] - coords[j]
        dist_matrix_slow[i, j] = np.sqrt(np.sum(diff**2))

# Vectorized via broadcasting: shape magic
# coords[:, None, :] has shape (n, 1, 3)
# coords[None, :, :] has shape (1, n, 3)
# Difference broadcasts to (n, n, 3)
diff = coords[:, None, :] - coords[None, :, :]   # (n, n, 3)
dist_matrix = np.sqrt(np.sum(diff**2, axis=2))   # (n, n)

# Even faster: scipy's pairwise distance
from scipy.spatial.distance import cdist
dist_matrix_scipy = cdist(coords, coords, metric='euclidean')

print(f"Max broadcasting error: {np.max(np.abs(dist_matrix - dist_matrix_scipy)):.2e}")
```

## Application: Contact Map Computation

In structural biology, the **contact map** records which residue pairs are within a distance threshold across MD trajectory frames:

```python
import numpy as np

def compute_contact_map_vectorized(ca_positions, cutoff=8.0):
    """
    Vectorized contact map for a single trajectory frame.
    
    ca_positions: (n_residues, 3) Cα positions
    Returns: (n_residues, n_residues) binary contact matrix
    """
    # Broadcasting to compute all pairwise distances at once
    # Shape: (n_res, 1, 3) - (1, n_res, 3) = (n_res, n_res, 3)
    diff = ca_positions[:, None, :] - ca_positions[None, :, :]
    # Sum of squares over xyz dimension, then sqrt
    dist = np.sqrt(np.sum(diff**2, axis=-1))
    # Binary contact matrix
    return dist < cutoff

def compute_contact_frequency_vectorized(trajectory, cutoff=8.0):
    """
    Compute average contact frequency over many frames.
    
    trajectory: (n_frames, n_residues, 3)
    """
    n_frames, n_res, _ = trajectory.shape
    
    # Process all frames simultaneously: (n_frames, n_res, 1, 3) - (n_frames, 1, n_res, 3)
    # This requires O(n_frames * n_res^2) memory — use chunked approach for large trajectories
    
    contact_sum = np.zeros((n_res, n_res))
    
    chunk_size = 100  # process 100 frames at a time
    for start in range(0, n_frames, chunk_size):
        chunk = trajectory[start:start+chunk_size]   # (chunk, n_res, 3)
        diff = chunk[:, :, None, :] - chunk[:, None, :, :]  # (chunk, n_res, n_res, 3)
        dist = np.sqrt(np.sum(diff**2, axis=-1))             # (chunk, n_res, n_res)
        contact_sum += (dist < cutoff).sum(axis=0)
    
    return contact_sum / n_frames

# Test
n_frames, n_res = 1000, 200
trajectory = np.random.standard_normal((n_frames, n_res, 3)) * 5.0

start = time.perf_counter()
freq = compute_contact_frequency_vectorized(trajectory, cutoff=8.0)
print(f"Contact map ({n_frames} frames, {n_res} residues): {(time.perf_counter()-start):.2f} s")
```

## Vectorizing Across Parameter Sets

In parameter scanning or ensemble simulation, the same computation is needed for many parameter combinations. Vectorize across parameters to evaluate all simultaneously:

```python
def hill_response_surface(alpha_values, K_values, n_fixed=2.0, x_input=5.0):
    """
    Compute Hill function output for all (alpha, K) combinations.
    Returns a 2D array: grid[i, j] = output for alpha[i], K[j]
    
    Broadcasting: alpha[:, None] has shape (n_alpha, 1)
                  K[None, :]     has shape (1,       n_K)
    """
    alpha = np.array(alpha_values)   # shape: (n_alpha,)
    K = np.array(K_values)           # shape: (n_K,)
    
    xn = (x_input / K[None, :])**n_fixed    # (1, n_K) broadcasts
    response = alpha[:, None] * xn / (1 + xn)  # (n_alpha, n_K)
    return response

alpha_range = np.linspace(1, 100, 200)
K_range = np.linspace(0.1, 20, 150)

# Compute 200×150 = 30,000 parameter combinations at once
grid = hill_response_surface(alpha_range, K_range)
print(f"Response surface shape: {grid.shape}")  # (200, 150)
print(f"Max response: {grid.max():.2f}")
```

## Universal Functions (ufuncs) and Custom ufuncs

NumPy **ufuncs** are element-wise operations that support broadcasting, reduction, and type casting. Common biological ufuncs:

```python
# All these operate element-wise over arrays of any shape:
x = np.linspace(0, 10, 1000)
y = np.exp(-x)           # vectorized exponential
z = np.log(x + 1)        # vectorized log
w = np.abs(x - 5)        # vectorized absolute value
u = np.where(x > 5, x, 0)  # vectorized conditional

# Reduction ufuncs: reduce along axis
expr_matrix = np.random.lognormal(0, 1, (20000, 100))  # genes × samples
gene_means = expr_matrix.mean(axis=1)    # mean over samples (axis=1)
sample_vars = expr_matrix.var(axis=0)   # variance over genes (axis=0)
```

## Common Vectorization Patterns

| Pattern | Slow (loop) | Fast (vectorized) |
|---------|-------------|-------------------|
| Element-wise math | `for i: result[i] = f(x[i])` | `result = f(x)` |
| Pairwise distances | double loop | `cdist()` or broadcasting |
| Conditional update | `for i: if c: x[i]+=1` | `x[x > threshold] += 1` |
| Accumulation | `for i: s += x[i]` | `s = x.sum()` |
| Matrix product | nested loops | `A @ B` or `np.dot` |

## Why This Matters

Vectorization is the gateway to usable computational biology in Python. A gene expression matrix with 20,000 genes and 500 samples contains 10 million values; any loop-based computation over it will take minutes where the vectorized equivalent takes milliseconds. For trajectory analysis in MD simulations, for genome-wide statistics, and for any multi-sample computation, vectorization is the minimum viable approach.
