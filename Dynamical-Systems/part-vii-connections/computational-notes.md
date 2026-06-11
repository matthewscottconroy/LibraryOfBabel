# Computational Notes — Part VII: Connections to Existing Work

These notes implement the concrete computational connections between the dynamical systems curriculum and the reader's existing projects: the Collatz explorer, quantum computing work, and HoTT/formal verification.

---

## 1. The Collatz Map: From Computation to Ergodic Theory

### Basic Collatz Computations

```python
def collatz(n):
    """The Collatz map: n -> n/2 if even, 3n+1 if odd"""
    if n % 2 == 0:
        return n // 2
    else:
        return 3*n + 1

def collatz_orbit(n, max_steps=10000):
    """Compute orbit until reaching 1 (or max_steps)"""
    orbit = [n]
    while n != 1 and len(orbit) < max_steps:
        n = collatz(n)
        orbit.append(n)
    return orbit

def collatz_stopping_time(n):
    """Number of steps to reach 1"""
    steps = 0
    while n != 1:
        n = collatz(n)
        steps += 1
    return steps
```

### Parity Sequence and Entropy Rate

The Collatz parity sequence $b_k(n) = n_k \pmod 2$ (where $n_k = C^k(n)$):

```python
def collatz_parity_sequence(n, steps):
    """Generate binary sequence from Collatz orbit parities"""
    seq = []
    for _ in range(steps):
        seq.append(n % 2)
        n = collatz(n)
        if n == 1:
            break
    return seq

# Estimate entropy rate via LZ complexity
def lz_complexity(sequence):
    """Estimate entropy rate via Lempel-Ziv"""
    seen = set()
    count = 0
    i = 0
    current = []
    while i < len(sequence):
        current.append(sequence[i])
        i += 1
        key = tuple(current)
        if key not in seen:
            seen.add(key)
            count += 1
            current = []
    if current:
        count += 1
    return count

# Experiment: entropy rate of Collatz parity sequences
for n in [27, 100, 1000, 9999]:
    seq = collatz_parity_sequence(n, 1000)
    c = lz_complexity(seq)
    n_bits = len(seq)
    print(f"n={n:5d}: LZ rate ≈ {c * np.log2(c) / n_bits:.3f} bits/step")
# Expected: close to 1.0 (random-looking parities)
```

### 2-Adic Extension

The Collatz map extends to $\mathbb{Z}_2$ (2-adic integers). On $\mathbb{Z}_2$, the map $T(x) = (x + (3x+1) \cdot [x \text{ is odd}])/2$ can be approximated by working modulo $2^k$:

```python
def collatz_mod(n, mod):
    """Collatz map modulo 2^k, extended to Z_2"""
    if n % 2 == 0:
        return (n // 2) % mod
    else:
        return ((3*n + 1) // 2) % mod

def collatz_adic_orbit(x, k, steps):
    """
    Orbit of x in Z_2 / 2^k Z_2 (approximation to 2-adic orbit).
    x should be given as an integer modulo 2^k.
    """
    mod = 2**k
    orbit = [x % mod]
    for _ in range(steps):
        x = collatz_mod(x, mod)
        orbit.append(x)
    return orbit

# Check: for k=5, the orbit structure modulo 32
# Should see eventual periodicity (since Z_2/32Z_2 is finite)
for start in range(32):
    orbit = collatz_adic_orbit(start, 5, 100)
    # Find period
    seen = {}
    for i, x in enumerate(orbit):
        if x in seen:
            period = i - seen[x]
            print(f"x={start:2d} mod 32: period {period} from step {seen[x]}")
            break
        seen[x] = i
```

### Stopping Time Distribution

```python
import matplotlib.pyplot as plt

stopping_times = [collatz_stopping_time(n) for n in range(1, 10001)]
plt.hist(stopping_times, bins=50, edgecolor='black')
plt.xlabel('Stopping time')
plt.ylabel('Count')
plt.title('Distribution of Collatz stopping times for n = 1 to 10000')

# The distribution is approximately log-normal:
# log(stopping_time) ~ N(mu, sigma^2)
log_times = [np.log(t) for t in stopping_times if t > 0]
print(f"Mean log stopping time: {np.mean(log_times):.2f}")
print(f"SD log stopping time:   {np.std(log_times):.2f}")
# Consistent with ergodic behavior of random walk on log(n)
```

---

## 2. Quantum Computing: Dynamics and Information

### Quantum State Evolution

Quantum computation is unitary dynamics on a complex Hilbert space. A qubit state is $|\psi\rangle = \alpha|0\rangle + \beta|1\rangle$ with $|\alpha|^2 + |\beta|^2 = 1$.

```python
import numpy as np

# Single qubit gates as 2x2 unitary matrices
X = np.array([[0,1],[1,0]])   # Pauli X (NOT gate)
Y = np.array([[0,-1j],[1j,0]])  # Pauli Y
Z = np.array([[1,0],[0,-1]])   # Pauli Z
H = np.array([[1,1],[1,-1]]) / np.sqrt(2)  # Hadamard
CNOT = np.array([[1,0,0,0],[0,1,0,0],[0,0,0,1],[0,0,1,0]])  # 2-qubit

def apply_gate(state, gate):
    """Apply unitary gate to quantum state"""
    return gate @ state

def measure_probability(state, outcome):
    """Probability of measuring outcome (0 or 1) on a single qubit"""
    return abs(state[outcome])**2
```

### Von Neumann Entropy and Entanglement

```python
def von_neumann_entropy(rho, base=2):
    """S(rho) = -Tr(rho log rho)"""
    eigenvalues = np.linalg.eigvalsh(rho)
    eigenvalues = eigenvalues[eigenvalues > 1e-15]
    if base == 2:
        return -np.sum(eigenvalues * np.log2(eigenvalues))
    else:
        return -np.sum(eigenvalues * np.log(eigenvalues))

def partial_trace_B(rho, dim_A, dim_B):
    """Trace out subsystem B from bipartite state rho"""
    rho_A = np.zeros((dim_A, dim_A), dtype=complex)
    for j in range(dim_B):
        for k in range(dim_A):
            for l in range(dim_A):
                rho_A[k,l] += rho[k*dim_B + j, l*dim_B + j]
    return rho_A

# Bell state: (|00> + |11>) / sqrt(2)
bell_state = np.array([1, 0, 0, 1]) / np.sqrt(2)
rho_bell = np.outer(bell_state, bell_state.conj())
rho_A = partial_trace_B(rho_bell, 2, 2)

print(f"Bell state entanglement entropy: {von_neumann_entropy(rho_A):.4f} bits")
# Expected: 1 bit (maximally entangled)

# Product state: |0> tensor |1>
product_state = np.array([0, 1, 0, 0])  # |01>
rho_prod = np.outer(product_state, product_state)
rho_A_prod = partial_trace_B(rho_prod, 2, 2)
print(f"Product state entanglement: {von_neumann_entropy(rho_A_prod):.4f} bits")
# Expected: 0 bits (unentangled)
```

### Quantum Chaos: Level Statistics

The Berry-Tabor/BGS conjecture connects classical chaos to quantum spectral statistics:
- Classically integrable system → energy level spacings follow Poisson distribution
- Classically chaotic system → spacings follow GUE (Gaussian Unitary Ensemble) distribution

```python
def level_spacing_distribution(eigenvalues, normalized=True):
    """
    Compute level spacing distribution from eigenvalue spectrum.
    Compare to Poisson (integrable) or Wigner-Dyson (chaotic).
    """
    eigenvalues = np.sort(eigenvalues)
    spacings = np.diff(eigenvalues)
    if normalized:
        # Normalize by mean spacing
        spacings = spacings / np.mean(spacings)
    return spacings

def wigner_dyson_pdf(s):
    """GUE level spacing distribution (Wigner surmise)"""
    return (32/pi**2) * s**2 * np.exp(-4*s**2/pi)

def poisson_pdf(s):
    """Poisson level spacing distribution (integrable)"""
    return np.exp(-s)

# Example: random Hermitian matrix (GUE) -> Wigner-Dyson statistics
n = 200
A = np.random.randn(n, n) + 1j * np.random.randn(n, n)
H_gue = (A + A.conj().T) / (2*np.sqrt(n))
eigenvalues = np.linalg.eigvalsh(H_gue)
spacings = level_spacing_distribution(eigenvalues)
# Histogram should match Wigner-Dyson distribution
```

---

## 3. HoTT and Lean: Formalizing Dynamical Systems

### Type-Theoretic Encoding of Dynamics

In Lean 4, a dynamical system is a type-theoretic structure:

```lean4
-- Lean 4 pseudocode (not executable, illustrative)

-- A dynamical system as a type
structure DynamicalSystem (α : Type*) where
  f : α → α

-- Orbit of a point
def orbit (ds : DynamicalSystem α) (x : α) : ℕ → α
  | 0     => x
  | n + 1 => ds.f (orbit ds x n)

-- Periodic point of period n
def isPeriodic (ds : DynamicalSystem α) (x : α) (n : ℕ) : Prop :=
  orbit ds x n = x

-- Two systems are topologically conjugate
structure TopologicalConjugacy (ds1 : DynamicalSystem α) (ds2 : DynamicalSystem β) where
  h : α → β
  h_inv : β → α
  h_homeomorphism : Homeomorphism h h_inv
  h_equivariant : ∀ x, h (ds1.f x) = ds2.f (h x)
```

### Formal Proof of a Simple Dynamical Theorem

```lean4
-- Formalize: the doubling map on [0,1) has topological entropy log 2
-- Step 1: Count (n, ε)-separated sets
-- This is part of the Mathlib formalization effort

-- Simpler: prove the orbit of an irrational under rotation is infinite
theorem rotation_orbit_infinite (α : ℝ) (h : Irrational α) (x : ℝ) :
    Set.Infinite {n : ℤ | ∃ k : ℤ, x + n * α = k} := by
  -- Proof that αℤ is dense in ℝ/ℤ when α is irrational
  sorry  -- placeholder; real proof uses Archimedean property
```

### Connecting to Mathlib

The Lean 4 mathematical library Mathlib already contains:
- `MeasureTheory.MeasurePreservingMap`: measure-preserving transformations
- `MeasureTheory.Ergodic`: basic ergodicity definitions  
- `Dynamics.TopologicalDynamics`: basic topological dynamics
- Work in progress: `ErgodicTheory.BirkhoffAverage`

```python
# To explore Mathlib formalization status:
# https://leanprover-community.github.io/mathlib4_docs/Mathlib/MeasureTheory/Measure/MeasureSpace.html

# Key theorem to check:
# - Has the Birkhoff Ergodic Theorem been formalized? (Yes, as of 2024)
# - Has the Shannon-McMillan-Breiman theorem been formalized? (Partially)
# - Has the Ornstein isomorphism theorem been formalized? (Not yet)
```

---

## 4. Integration with the Quiz App

The quiz app subject.toml connects chapters to quiz topics. Part VII connects to:
- **Collatz** → discrete dynamics, ergodic theory on ℤ_2, algorithmic information theory
- **Quantum Computing** → quantum information, entropy, unitary dynamics
- **HoTT** → type-theoretic proof, formal verification, topos theory

Example quiz questions for Part VII:
- "The Collatz stopping time for $n$ is empirically distributed like a log-normal distribution. What does this suggest about the Collatz map viewed as a random walk on $\log n$?"
- "The Bell state $|\Phi^+\rangle = (|00\rangle + |11\rangle)/\sqrt{2}$ has entanglement entropy 1 bit. What does this mean operationally (in terms of what Alice and Bob can do with this state)?"
- "In HoTT, the Univalence Axiom says $A = B$ iff there is an equivalence $A \simeq B$. What is the dynamical systems analogue of this principle?"
- "For the logistic map itinerary sequence starting from $x_0 = 0.2$, compute the first 20 bits and estimate the LZ complexity. Is this initial condition Collatz-like or random-like?"
