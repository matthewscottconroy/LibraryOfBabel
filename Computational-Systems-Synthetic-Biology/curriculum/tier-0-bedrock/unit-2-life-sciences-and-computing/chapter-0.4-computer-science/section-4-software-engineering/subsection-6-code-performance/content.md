# Code Performance

There is a rule of thumb in software engineering attributed to Donald Knuth: "Premature optimization is the root of all evil." The wisdom behind it is that most performance problems turn out to be concentrated in a small fraction of the code — and you almost certainly cannot guess in advance which fraction. Optimize the wrong place and you have wasted time, added complexity, and possibly introduced bugs, for no improvement in runtime. The corollary, though, is equally important: when you do need to optimize, optimize with data. Profile first, then act.

The first rule of optimization is: don't. The second rule is: don't yet. Write correct, clear code first. Profile it. Then optimize the bottleneck. Most programs spend 90% of their time in 10% of their code — optimizing the other 90% is wasted effort. This section is about how to find that 10% and what to do about it.

## Profiling: Finding the Bottleneck

Before optimizing anything, measure where the time goes.

### cProfile: Whole-Program Profiling

```python
# Command-line profiling
python -m cProfile -o profile.stats my_analysis.py

# Analyze results
import pstats
p = pstats.Stats("profile.stats")
p.sort_stats("cumulative")
p.print_stats(20)  # top 20 functions by cumulative time
```

Or interactively using **snakeviz** for a visualization:
```bash
pip install snakeviz
python -m cProfile -o profile.stats my_analysis.py
snakeviz profile.stats  # opens browser visualization
```

### line_profiler: Line-by-Line Within a Function

```python
# Decorate the function you suspect is slow
from line_profiler import profile

@profile
def count_kmers(seq: str, k: int) -> dict:
    counts = {}
    for i in range(len(seq) - k + 1):
        kmer = seq[i:i+k]      # <-- line A
        counts[kmer] = counts.get(kmer, 0) + 1  # <-- line B
    return counts
```

```bash
kernprof -l -v my_script.py
```

Output shows hits, time per line, and cumulative time — immediately revealing which line inside the function is expensive.

### Timing in Jupyter

```python
# IPython magic
%timeit gc_content("ACGT" * 1000)         # many runs; reports mean ± std
%time compute_pairwise_distances(seqs)    # single run; reports wall time

# Fine-grained: time a specific cell
%%timeit -n 100 -r 10
result = [kmer for kmer in kmers if kmer in reference_set]
```

## Vectorization: Replace Python Loops with NumPy

Python loops over arrays are 10–100× slower than NumPy vectorized operations because:
1. Python objects (boxed integers, floats) have heavy overhead per element
2. NumPy operates in compiled C on contiguous typed arrays
3. Modern CPUs execute multiple floating-point operations per clock cycle (SIMD) — NumPy exploits this; Python loops cannot

```python
import numpy as np

# Slow: Python loop (O(n) Python overhead)
def gc_fraction_loop(sequences: list[str]) -> list[float]:
    return [(s.count("G") + s.count("C")) / len(s) for s in sequences]

# Fast: NumPy approach using character comparison
def gc_fraction_numpy(seqs_encoded: np.ndarray) -> np.ndarray:
    """seqs_encoded: (n_seqs, seq_len) uint8 array where G=71, C=67"""
    return ((seqs_encoded == 71) | (seqs_encoded == 67)).mean(axis=1)

# Encode sequences once
def encode_seqs(seqs: list[str], length: int) -> np.ndarray:
    arr = np.zeros((len(seqs), length), dtype=np.uint8)
    for i, s in enumerate(seqs[:length]):
        arr[i, :len(s)] = np.frombuffer(s.encode(), dtype=np.uint8)
    return arr

# Pairwise distance matrix (vectorized)
def pairwise_hamming(arr: np.ndarray) -> np.ndarray:
    """arr: (n, L) boolean or integer array; returns (n, n) distance matrix."""
    # Broadcasting: (n, 1, L) != (1, n, L) → (n, n, L), mean over L
    return (arr[:, None, :] != arr[None, :, :]).mean(axis=2)
```

**Broadcasting rules** are the key skill: understand which dimensions align automatically and which need `[:, None, :]` or `keepdims=True` to expand correctly.

## Numba: JIT Compilation for Numeric Loops

When vectorization is impossible (algorithm requires sequential access to previous values, like dynamic programming), **Numba** compiles Python functions to machine code using LLVM:

```python
from numba import njit, prange
import numpy as np

@njit(parallel=True)  # enables parallel execution of prange loops
def needleman_wunsch_batch(seqs1: np.ndarray, seqs2: np.ndarray,
                            match: int, mismatch: int, gap: int) -> np.ndarray:
    """Compute NW scores for n pairs of sequences."""
    n = seqs1.shape[0]
    scores = np.zeros(n, dtype=np.float64)
    
    for idx in prange(n):  # prange = parallel range
        s1, s2 = seqs1[idx], seqs2[idx]
        n1, n2 = len(s1), len(s2)
        dp = np.zeros((n1 + 1, n2 + 1), dtype=np.float64)
        for i in range(n1 + 1):
            dp[i, 0] = i * gap
        for j in range(n2 + 1):
            dp[0, j] = j * gap
        for i in range(1, n1 + 1):
            for j in range(1, n2 + 1):
                diag = dp[i-1, j-1] + (match if s1[i-1] == s2[j-1] else mismatch)
                dp[i, j] = max(diag, dp[i-1, j] + gap, dp[i, j-1] + gap)
        scores[idx] = dp[n1, n2]
    return scores
```

First call incurs compilation overhead (~1–5 s); subsequent calls are fast (compiled machine code). Speedup over pure Python: typically 10–200×.

**`@njit`** (no-Python mode): requires NumPy arrays, not Python lists; all types must be inferable at compile time; no Python objects. This is the strictest and fastest mode.

## Multiprocessing and Dask

For CPU-bound embarrassingly parallel tasks (many independent computations), use Python's `multiprocessing` or the higher-level `concurrent.futures`:

```python
from concurrent.futures import ProcessPoolExecutor
from multiprocessing import cpu_count

def process_sample(sample: str) -> dict:
    """CPU-intensive per-sample computation."""
    reads = load_reads(sample)
    kmers = count_kmers(reads, k=21)
    return {"sample": sample, "n_kmers": len(kmers), "total": sum(kmers.values())}

# Run on all CPUs
samples = ["SRR001", "SRR002", "SRR003", ..., "SRR500"]
with ProcessPoolExecutor(max_workers=cpu_count()) as pool:
    results = list(pool.map(process_sample, samples))
```

**Dask** extends this to out-of-memory datasets and cluster computation:

```python
import dask.dataframe as dd

# Read many large TSV files as a lazily evaluated dataframe
df = dd.read_csv("results/*.tsv", sep="\t")

# Operations build a DAG but don't execute
grouped = df.groupby("gene_id")["count"].sum()

# Compute triggers execution (parallel across files, across workers)
result = grouped.compute()
```

## GPU Acceleration: CuPy and JAX

For extremely large array computations (training ML models, computing large distance matrices), **GPUs** provide 10–100× speedup over CPUs:

```python
import cupy as cp  # NumPy API on GPU

# Move data to GPU
expr_gpu = cp.array(expression_matrix)   # (50000, 100000) float32 on GPU

# Compute correlation matrix GPU-side (teraflop-scale computation)
mean = expr_gpu.mean(axis=1, keepdims=True)
std  = expr_gpu.std(axis=1, keepdims=True)
z = (expr_gpu - mean) / (std + 1e-9)
corr = cp.dot(z, z.T) / z.shape[1]

# Move result back to CPU
corr_cpu = cp.asnumpy(corr)
```

**JAX** combines NumPy-compatible array operations with automatic differentiation and JIT compilation:

```python
import jax
import jax.numpy as jnp

@jax.jit  # JIT-compile the function
def softmax_cross_entropy(logits, labels):
    log_probs = jax.nn.log_softmax(logits)
    return -jnp.mean(jnp.sum(labels * log_probs, axis=1))

# Automatic gradient
grad_fn = jax.grad(softmax_cross_entropy)
grads = grad_fn(logits, labels)  # exact gradient, no finite differences
```

## Why This Matters for Computational Biology

Performance determines what is computationally feasible. A pairwise sequence comparison that takes 10 s per pair takes 3 hours for 1,000 pairs and 10 years for 100,000 pairs. Vectorizing it with NumPy might reduce per-pair time to 1 ms — reducing 100,000 pairs to 1.5 minutes. Numba-compiled dynamic programming enables DP at scales (millions of alignments) where Python loops are intractable. GPU-accelerated matrix operations make training protein language models on millions of sequences achievable in days. Profiling before optimizing means you spend your effort where it matters — the 10-line bottleneck function, not the 500-line wrapper around it. Understanding the hierarchy of optimization strategies (vectorize → Numba → Cython → C extension → GPU) means you always apply the least-invasive solution that achieves the required performance.
