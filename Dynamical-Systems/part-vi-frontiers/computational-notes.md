# Computational Notes — Part VI: Research Frontiers

The research frontiers of Part VI are primarily theoretical, but several have important computational aspects: the numerical study of strange attractors and their SRB measures, the computation of $\ell^2$-Betti numbers, the generation of Mandelbrot set images with renormalization structure visible, and the finite-length information theory of Part 39.

---

## 1. Julia Sets and the Mandelbrot Set

### Computing Julia Sets

For $f_c(z) = z^2 + c$, the Julia set $J_c$ separates bounded orbits from escaping orbits. The *escape time* algorithm:

```
def julia_escape_time(c, z, max_iter=256, escape_radius=2.0):
    """
    Returns iteration count before |z| > escape_radius, or max_iter.
    Points that don't escape are in the filled Julia set K_c.
    """
    for k in range(max_iter):
        if abs(z) > escape_radius:
            return k
        z = z**2 + c
    return max_iter  # presumed inside K_c

def julia_set_image(c, width=800, height=800, x_range=(-2,2), y_range=(-2,2)):
    x_vals = np.linspace(x_range[0], x_range[1], width)
    y_vals = np.linspace(y_range[0], y_range[1], height)
    image = np.zeros((height, width))
    for i, y in enumerate(y_vals):
        for j, x in enumerate(x_vals):
            image[i,j] = julia_escape_time(c, complex(x, y))
    return image
```

**Key Julia sets to visualize**:
- $c = 0$: Julia set is the unit circle (all orbits either converge to 0 or escape to ∞)
- $c = -2$: Julia set is the interval $[-2,2]$ (real Julia set)
- $c = -0.7 + 0.27i$: Dendrite (topologically a tree)
- $c = -0.1 + 0.651i$: Siegel disk (invariant disk around a neutral fixed point)
- $c = 0.355 + 0.355i$: Cantor set (totally disconnected Julia set, $c \notin \mathcal{M}$)

### Computing the Mandelbrot Set

The Mandelbrot set $\mathcal{M} = \{c : f_c^n(0) \not\to \infty\}$ uses the same escape time algorithm but iterates the critical point $z_0 = 0$:

```
def mandelbrot(c, max_iter=256):
    """Returns escape time for critical point 0 under z -> z^2 + c"""
    z = 0
    for k in range(max_iter):
        if abs(z) > 2:
            return k
        z = z**2 + c
    return max_iter  # presumed in M
```

**Renormalization structure**: The "baby Mandelbrot sets" (small copies of $\mathcal{M}$ in the parameter plane) are the regions where the map is renormalizable. To see them: zoom into the neck of the period-3 bulb at $c \approx -1.755$. The renormalization operator "zooms in" on the small copy, revealing a map that looks like the original Mandelbrot set.

**Computing period of a hyperbolic component**: To determine if $c$ is in a hyperbolic component of period $n$, find the cycle $f_c^n(0) = 0$ (the critical point is periodic) using Newton's method on $f_c^n(0)$.

---

## 2. Sofic Entropy: A Simple Example

### Bernoulli Shift Entropy

Bowen's sofic entropy reduces to KS entropy for amenable groups. For a Bernoulli shift $\Gamma \curvearrowright (X_0^{\Gamma}, \mu_0^{\Gamma})$ over a countable group $\Gamma$:
- KS entropy (for $\Gamma = \mathbb{Z}$): $h = H(\mu_0) = -\sum_x p(x) \log p(x)$
- Sofic entropy (for sofic $\Gamma$): same formula

```
def bernoulli_entropy(base_distribution):
    """
    Entropy of Bernoulli shift = entropy of the base measure.
    For amenable groups, this is the KS entropy.
    For sofic groups, sofic entropy = KS entropy for Bernoulli shifts.
    """
    p = np.array(base_distribution)
    p = p / p.sum()  # normalize
    return -sum(pi * np.log(pi) for pi in p if pi > 0)

# Two Bernoulli shifts over Z with different base measures
# are isomorphic iff they have the same entropy (Ornstein)
h1 = bernoulli_entropy([0.5, 0.5])   # log 2 ≈ 0.693
h2 = bernoulli_entropy([0.3, 0.7])   # binary entropy ≈ 0.611
h3 = bernoulli_entropy([0.5, 0.5])   # same as h1

print(f"h1 = {h1:.4f}, h2 = {h2:.4f}, h3 = {h3:.4f}")
print(f"Shifts 1 and 3 are isomorphic (same entropy)")
print(f"Shift 2 is NOT isomorphic to 1 or 3 (different entropy)")
```

This demonstrates Ornstein's theorem numerically: entropy is the complete invariant.

---

## 3. Finite-Length Information Theory

### Normal Approximation to Channel Coding Rate

The Polyanskiy-Poor-Verdú (2010) second-order rate approximation:
$$R^*(n, \varepsilon) \approx C - \sqrt{V/n} \cdot Q^{-1}(\varepsilon)$$

```
from scipy.special import erfinv
import numpy as np

def normal_approximation_rate(C, V, n, epsilon):
    """
    Second-order approximation to the maximum coding rate
    at block length n and error probability epsilon.
    """
    # Q^{-1}(epsilon) = sqrt(2) * erfinv(1 - 2*epsilon)
    Q_inv = np.sqrt(2) * erfinv(1 - 2*epsilon)
    return C - np.sqrt(V/n) * Q_inv

def bsc_channel_dispersion(p):
    """
    Channel dispersion of BSC(p).
    V = p(1-p) * (log((1-p)/p))^2 + ... 
    Actually V = Var[log(p(Y|X)/p(Y))] for optimal input
    """
    # For BSC with capacity-achieving uniform input:
    # V = p*(1-p)*(log((1-p)/p))^2  # simplified formula
    if p == 0 or p == 1:
        return 0
    return p * (1-p) * (np.log2((1-p)/p))**2

# Example: BSC with p = 0.1
p = 0.1
C = 1 - (-p*np.log2(p) - (1-p)*np.log2(1-p))  # 1 - H(p)
V = bsc_channel_dispersion(p)
print(f"BSC capacity C = {C:.4f} bits")
print(f"Channel dispersion V = {V:.4f} bits^2")
print(f"\nMaximum rates for epsilon = 0.01:")
for n in [100, 500, 1000, 5000]:
    R = normal_approximation_rate(C, V, n, 0.01)
    print(f"  n = {n:5d}: R* ≈ {R:.4f} bits (gap to capacity: {C-R:.4f})")
```

**Key insight**: For small $n$, the gap $C - R^*$ is significant. For $n = 100$ and $\varepsilon = 0.01$, the maximum rate may be 20-30% below capacity.

---

## 4. Cost in Orbit Equivalence

### Computing Cost of a Free Group Action

The cost of a free ergodic action of a free group $F_r$ on $r$ generators is $r$ (Gaboriau's theorem). For an amenable group, the cost is always 1 (Ornstein-Weiss). These can be seen computationally by counting the minimum number of generators needed for graphings.

```
def estimate_cost(orbit_equivalence_relation_sample, num_vertices):
    """
    Estimate cost by minimum graphing (greedy algorithm).
    A graphing is a set of partial bijections whose union generates the relation.
    Cost = (total number of edges in minimum graphing) / (number of vertices)
    """
    # This is a simplified model for finite approximations
    # True cost requires taking limits over finite approximations (Følner sequence)
    pass

# For Z-action (rotation by alpha):
# The orbit relation on n points is approximately a single cycle of length n
# Minimum graphing: a single cycle has n edges
# Cost = n/n = 1 (as expected for amenable group)

# For F_2-action (Cayley graph of F_2):
# Each vertex has 4 neighbors; minimum spanning forest has n-1 edges per component
# Cost approaches r = 2 for free group on 2 generators
```

---

## 5. Connection to the Quiz App

Frontier-level computational questions:
- "For $c = -0.7 + 0.27i$, compute the first 10 iterates of $f_c$ starting at $z_0 = 0$. Is $0$ in the Mandelbrot set? Is this Julia set connected or a Cantor set?"
- "For a BSC with $p = 0.2$ and block length $n = 1000$, compute the capacity $C$, channel dispersion $V$, and the maximum achievable rate for error probability $\varepsilon = 0.05$ using the normal approximation."
- "Two Bernoulli shifts over $\mathbb{Z}$: one with base measure $(1/3, 2/3)$ and one with base measure $(1/4, 1/4, 1/2)$. Compute their entropies. Are they isomorphic?"
- "The Feigenbaum constant $\delta \approx 4.669$ is universal. Compute the bifurcation values $\mu_1, \mu_2, \mu_3, \mu_4$ for the logistic map and estimate $\delta$ from the ratios."
