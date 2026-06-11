# Computational Notes — Part V: Foundations of CS and Mathematics

These notes connect Part V's foundational material to computation. The emphasis is on: demonstrating undecidability concretely, computing optimal transport, verifying Weyl equidistribution, and implementing simple versions of categorical and thermodynamic computations.

---

## 1. Wang Tiles and the Undecidability of Tiling

### Wang Tile Simulation

A Wang tile is a unit square with colored edges (North, East, South, West). A tiling of the plane satisfies: adjacent tiles must share the same color on their shared edge.

```
def can_tile_nxn(tile_set, n):
    """
    Backtracking search for valid tiling of n×n grid.
    Returns True if a valid tiling exists.
    """
    grid = [[None] * n for _ in range(n)]
    
    def is_compatible(tile, row, col):
        # Check North neighbor
        if row > 0 and grid[row-1][col] is not None:
            if tile_set[tile].north != tile_set[grid[row-1][col]].south:
                return False
        # Check West neighbor
        if col > 0 and grid[row][col-1] is not None:
            if tile_set[tile].west != tile_set[grid[row][col-1]].east:
                return False
        return True
    
    def backtrack(pos):
        if pos == n * n:
            return True
        row, col = divmod(pos, n)
        for tile in range(len(tile_set)):
            if is_compatible(tile, row, col):
                grid[row][col] = tile
                if backtrack(pos + 1):
                    return True
                grid[row][col] = None
        return False
    
    return backtrack(0)
```

**Key insight**: The 2D tiling problem for Wang tiles is undecidable — no algorithm can determine, given an arbitrary tile set, whether it tiles the plane. The `can_tile_nxn` function above is a decidable approximation (for finite $n$): but there exist tile sets that tile $n \times n$ for all $n$ but do *not* tile the infinite plane (this cannot happen for Wang tiles, but for corner-rule tiles it can). The undecidability comes from the ability to simulate arbitrary Turing machines.

**Berger's aperiodic tile set**: The existence of an aperiodic tile set (one that tiles the plane but only non-periodically) is the key result. The original set had 20,426 tiles; the minimum is 11 (Jeandel-Rao, 2021).

---

## 2. Optimal Transport: Earth Mover's Distance

### Discrete Optimal Transport

For finite measures $\mu = \sum_i p_i \delta_{x_i}$ and $\nu = \sum_j q_j \delta_{y_j}$ on a metric space $(X, d)$, the Wasserstein-1 distance is:
$$W_1(\mu, \nu) = \min_{\pi \in \Pi(\mu,\nu)} \sum_{i,j} d(x_i, y_j) \pi_{ij}$$
where $\Pi(\mu,\nu)$ is the set of couplings (transport plans $\pi_{ij} \geq 0$ with $\sum_j \pi_{ij} = p_i$, $\sum_i \pi_{ij} = q_j$).

```
import numpy as np
from scipy.optimize import linprog

def wasserstein1(source_positions, source_weights,
                 target_positions, target_weights,
                 cost_matrix=None):
    """
    Compute W_1 by solving the linear programming problem:
    min sum_{i,j} C_{ij} pi_{ij}
    s.t. sum_j pi_{ij} = p_i, sum_i pi_{ij} = q_j, pi >= 0
    """
    n = len(source_weights)
    m = len(target_weights)
    
    if cost_matrix is None:
        # Euclidean distance
        cost_matrix = np.array([[np.linalg.norm(source_positions[i] - target_positions[j])
                                  for j in range(m)] for i in range(n)])
    
    # Flatten for LP
    c = cost_matrix.flatten()
    
    # Equality constraints: row sums = source_weights, col sums = target_weights
    A_eq = np.zeros((n + m, n * m))
    for i in range(n):
        A_eq[i, i*m:(i+1)*m] = 1
    for j in range(m):
        A_eq[n+j, j::m] = 1
    b_eq = np.concatenate([source_weights, target_weights])
    
    result = linprog(c, A_eq=A_eq, b_eq=b_eq, bounds=[(0, None)] * (n*m))
    return result.fun
```

**Applications to dynamics**:
- Measure the Wasserstein distance between the empirical distribution after $n$ steps of a map and the invariant measure: $W_1(\frac{1}{N}\sum_{k=0}^{N-1} \delta_{f^k(x)}, \mu_{\text{inv}})$. This should decrease as $N$ increases (convergence to invariant measure).
- Compare the invariant measures of two nearby dynamical systems — the Wasserstein distance is a natural "distance between systems."

---

## 3. Weyl Equidistribution: Experimental Verification

```
import numpy as np
import matplotlib.pyplot as plt

def weyl_experiment(alpha, N=10000):
    """
    Verify equidistribution of n*alpha mod 1.
    Compare empirical CDF to uniform CDF.
    """
    sequence = [(n * alpha) % 1 for n in range(N)]
    
    # Kolmogorov-Smirnov test against uniform
    from scipy.stats import kstest
    ks_stat, p_value = kstest(sequence, 'uniform')
    
    # Discrepancy D_N = sup |F_N(x) - x|
    sequence_sorted = sorted(sequence)
    discrepancy = max(abs((i+1)/N - sequence_sorted[i]) for i in range(N))
    
    return ks_stat, p_value, discrepancy

# Test for irrational alpha
print(weyl_experiment(np.sqrt(2)))  # Expected: p_value ~ uniform, D_N -> 0
print(weyl_experiment(np.pi))

# Test for rational alpha = 1/3
print(weyl_experiment(1/3))  # Expected: sequence has period 3, NOT equidistributed
```

**Weyl's theorem prediction**: For irrational $\alpha$, the discrepancy $D_N = \sup_x |F_N(x) - x|$ satisfies $D_N \to 0$ as $N \to \infty$. For rational $\alpha = p/q$, the sequence is periodic with period $q$ and $D_N$ stays bounded away from 0.

**Three-distance theorem**: The fractional parts $\{n\alpha\}$ for $n = 0, 1, \ldots, N-1$ partition $[0,1)$ into arcs of at most 3 different lengths. This is a beautiful combinatorial result with an ergodic proof.

---

## 4. Furstenberg Correspondence: Computational Demonstration

### Szemerédi's Theorem via Density

```
def upper_density(A, N):
    """Upper density of set A up to N"""
    return len([a for a in A if a <= N]) / N

def contains_ap(A, k, max_n=1000):
    """Check if A contains an arithmetic progression of length k"""
    A_set = set(A)
    for start in A_set:
        for step in range(1, max_n):
            ap = [start + i*step for i in range(k)]
            if all(a in A_set for a in ap):
                return (start, step)
    return None

# Example: primes contain arbitrary-length APs (Green-Tao theorem)
from sympy import primes, isprime
prime_list = list(primes(1, 10000))
print(f"Primes up to 10000 contain AP of length 5: {contains_ap(set(prime_list), 5)}")

# Example: squares {n^2} have density 0 but we can ask if they contain APs
squares = set(n**2 for n in range(1, 100))
print(f"Squares contain AP of length 3: {contains_ap(squares, 3)}")
# Answer: yes! e.g., 1, 25, 49 is an AP with step 24
```

**Furstenberg's approach**: The correspondence principle converts density statements about $A \subseteq \mathbb{Z}$ to measure-theoretic statements about a dynamical system. The fact that sets of positive density contain long APs (Szemerédi's theorem) follows from multiple recurrence of measure-preserving transformations.

---

## 5. Landauer's Principle: Energy Accounting

```
import numpy as np

def landauer_minimum_energy(bits_erased, temperature_kelvin):
    """
    Minimum energy dissipated to erase bits_erased bits at temperature T.
    E = k_B * T * ln(2) * bits_erased
    """
    k_B = 1.38e-23  # J/K
    return k_B * temperature_kelvin * np.log(2) * bits_erased

# At room temperature (T = 293 K)
T = 293
single_bit = landauer_minimum_energy(1, T)
print(f"Erasing 1 bit at room temperature: {single_bit:.2e} J")
print(f"That's {single_bit / 1.6e-19:.2f} eV")

# A modern CPU erasing 10^18 bits/second
cpu_bits_per_sec = 1e18
cpu_power = cpu_bits_per_sec * landauer_minimum_energy(1, T)
print(f"Landauer minimum power for modern CPU: {cpu_power:.2f} W")
print(f"Actual CPU power: ~100W (about {100/cpu_power:.0f}x Landauer limit)")
```

**Physical significance**: Modern processors dissipate about $10^6 \times$ the Landauer minimum (most energy goes to resistive losses, not logical operations). Reversible computing — in principle — could approach the Landauer limit.

---

## 6. Categorical Entropy: Leinster's Approach

### Diversity and Entropy

Tom Leinster's categorical approach to entropy connects to the Rényi entropy family. The *diversity* of order $q$ of an ecological community with species proportions $p = (p_1, \ldots, p_n)$ is:
$$D_q(p) = \left(\sum_{i=1}^n p_i^q\right)^{1/(1-q)}.$$

At $q = 1$: $D_1(p) = \exp(H_{\text{Shannon}}(p))$ (effective number of species).
At $q = 2$: $D_2(p) = 1/\sum p_i^2 = 1/P_{\text{collision}}$ (Simpson's diversity).

```
def diversity(p, q):
    """Diversity of order q (Rényi entropy exponentiated)"""
    p = np.array(p)
    p = p / p.sum()  # normalize
    if abs(q - 1) < 1e-10:
        # L'Hôpital limit: q -> 1
        return np.exp(-sum(pi * np.log(pi) for pi in p if pi > 0))
    elif q == float('inf'):
        return 1 / max(p)
    else:
        return (sum(pi**q for pi in p)) ** (1/(1-q))

# Shannon entropy as log-diversity
def shannon_entropy(p):
    return np.log(diversity(p, 1))

# Verify the chain rule via the diversity formula
# H(X, Y) = H(X) + H(Y|X) becomes a multiplicative rule for diversity
```

---

## 7. Connection to the Quiz App

Computational notes from Part V generate deeper questions:
- "Using the Weyl equidistribution criterion, prove that $(n^2 \alpha) \pmod 1$ is not equidistributed for any $\alpha$. Then state the correct theorem (Weyl polynomial equidistribution)."
- "Compute the Wasserstein-1 distance between the uniform distribution on $\{1, 3, 5, 7, 9\}$ and the uniform distribution on $\{2, 4, 6, 8, 10\}$ on $\mathbb{R}$."
- "Explain how Landauer's principle resolves the Maxwell's demon paradox. What physical process is the 'hidden cost' in the demon's operation?"
- "The arithmetic progressions $1, 5, 9$ and $1, 25, 49$ are both arithmetic progressions in the squares. What is the density of perfect squares in $\mathbb{N}$? Does Szemerédi's theorem apply?"
