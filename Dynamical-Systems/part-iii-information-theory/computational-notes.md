# Computational Notes — Part III: Information Theory

These notes connect the information theory of Part III to computational experiments. The emphasis is on: implementing entropy computations from scratch, building intuition for the AEP through simulation, implementing coding algorithms, and connecting to the information-theoretic properties of dynamical systems.

---

## 1. Computing Shannon Entropy

### Empirical Entropy from Samples

Given a sequence of samples $x_1, \ldots, x_N$ from an unknown distribution, estimate the entropy:

```
def empirical_entropy(samples, base=2):
    """Estimate Shannon entropy from samples"""
    from collections import Counter
    counts = Counter(samples)
    N = len(samples)
    H = 0.0
    for count in counts.values():
        p = count / N
        H -= p * log(p, base)
    return H
```

**Experiments**:
- Generate 10000 samples from a fair coin (Bernoulli(0.5)): estimated $H \approx 1$ bit.
- Generate from Bernoulli(0.1): estimated $H \approx 0.469$ bits (exact: $-0.1\log 0.1 - 0.9\log 0.9$).
- Observe how the estimate improves with sample size $N$.

**The plug-in estimator** is biased: $\mathbb{E}[\hat{H}] \leq H(X)$ for finite samples (due to Jensen's inequality applied to $-\log$). The bias is approximately $(k-1)/(2N\ln 2)$ where $k$ is the alphabet size.

### Conditional Entropy and Mutual Information

```
def conditional_entropy(joint_counts):
    """H(Y|X) from joint frequency counts"""
    # joint_counts[i][j] = count of (X=i, Y=j)
    H_XY = empirical_entropy(flatten(joint_counts))
    H_X = empirical_entropy(sum_rows(joint_counts))
    return H_XY - H_X

def mutual_information(joint_counts):
    H_X = empirical_entropy(sum_rows(joint_counts))
    H_Y = empirical_entropy(sum_cols(joint_counts))
    H_XY = empirical_entropy(flatten(joint_counts))
    return H_X + H_Y - H_XY
```

**Verify the data processing inequality**: For a Markov chain $X \to Y \to Z$ (i.e., $Z$ depends on $X$ only through $Y$): $I(X;Z) \leq I(X;Y)$. Simulate a chain and verify computationally.

---

## 2. The Asymptotic Equipartition Property (AEP)

### Simulating the AEP

For an i.i.d. source with distribution $p$ and entropy $H = H(X)$:

```
def simulate_aep(p, n, num_trials=1000):
    """
    Verify: -(1/n) log P(X^n) -> H almost surely
    """
    empirical_log_probs = []
    for _ in range(num_trials):
        # Generate n samples
        symbols = random.choice(len(p), size=n, p=p)
        # Compute log probability
        log_prob = sum(log2(p[s]) for s in symbols)
        empirical_log_probs.append(-log_prob / n)
    return mean(empirical_log_probs), std(empirical_log_probs)
```

**Expected result**: mean converges to $H(p)$, standard deviation $\approx \sqrt{\text{Var}[-\log p(X)] / n}$.

### Typical Set

```
def is_typical(x_sequence, p, epsilon=0.1):
    """Check if x^n is in the epsilon-typical set"""
    n = len(x_sequence)
    log_prob = sum(log2(p[x]) for x in x_sequence)
    H = entropy(p)  # true entropy
    return abs(-log_prob/n - H) <= epsilon

def count_typical(p, n, epsilon=0.1, num_samples=10000):
    """Verify: typical set has probability close to 1"""
    typical_count = 0
    for _ in range(num_samples):
        x = random.choice(len(p), size=n, p=p)
        if is_typical(x, p, epsilon):
            typical_count += 1
    return typical_count / num_samples
```

**Observe**: as $n$ grows, the fraction of sequences in the typical set approaches 1, but the typical set has size $\approx 2^{nH}$ out of $k^n$ total sequences.

---

## 3. Source Coding: Huffman Codes

### Building a Huffman Code

```
def huffman_code(probabilities):
    """Build optimal prefix-free code"""
    import heapq
    # Build min-heap of (probability, symbol)
    heap = [(p, i, [i]) for i, p in enumerate(probabilities)]
    heapq.heapify(heap)
    codebook = [''] * len(probabilities)
    while len(heap) > 1:
        p1, _, symbols1 = heapq.heappop(heap)
        p2, _, symbols2 = heapq.heappop(heap)
        # Merge: symbols1 get '0' prefix, symbols2 get '1' prefix
        for s in symbols1:
            codebook[s] = '0' + codebook[s]
        for s in symbols2:
            codebook[s] = '1' + codebook[s]
        heapq.heappush(heap, (p1+p2, symbols1[0], symbols1+symbols2))
    return codebook

def average_length(codebook, probabilities):
    return sum(p * len(c) for p, c in zip(probabilities, codebook))
```

**Verify the source coding theorem**: For distribution $p$:
$$H(X) \leq \bar{L}_{\text{Huffman}} \leq H(X) + 1.$$

For a binary uniform distribution ($p = (1/2, 1/2)$): $\bar{L} = 1 = H$. For $p = (1/2, 1/4, 1/8, 1/8)$: $\bar{L} = 1.75$ bits, $H = 1.75$ bits — exact!

---

## 4. Channel Capacity and the BSC

### Binary Symmetric Channel (BSC)

For a BSC with crossover probability $p$ (each bit is flipped with probability $p$):
- Capacity: $C = 1 - h(p)$ where $h(p) = -p\log_2 p - (1-p)\log_2(1-p)$ (binary entropy)

```
def bsc_capacity(p):
    """Capacity of binary symmetric channel"""
    if p == 0 or p == 1:
        return 1.0
    return 1 + p * log2(p) + (1-p) * log2(1-p)

def simulate_bsc(message, p):
    """Apply BSC noise"""
    return [bit ^ (random() < p) for bit in message]
```

**Repetition code simulation**: Use a rate-$1/3$ repetition code (send each bit 3 times, decode by majority vote) on a BSC with $p = 0.1$.
- Effective error rate: $P_e = 3p^2(1-p) + p^3 \approx 0.028$ (better than $p = 0.1$ alone).
- Rate: $R = 1/3 < C = 0.531$. There exist much better codes approaching capacity.

---

## 5. Kolmogorov Complexity and Compressibility

### Approximating Kolmogorov Complexity

True Kolmogorov complexity is uncomputable, but compression algorithms provide upper bounds:
$$K(x) \leq |z| + O(1) \text{ where } z = \text{compress}(x).$$

```
def complexity_upper_bound(x):
    """Upper bound on K(x) via compression"""
    import zlib
    return len(zlib.compress(x.encode()))

def is_random(string, threshold=0.95):
    """Heuristic test: string is 'random' if it doesn't compress"""
    original = len(string)
    compressed = len(zlib.compress(string.encode()))
    return compressed / original > threshold
```

**Experiments**:
- Compare complexity of: `"010101010101..."` (periodic), `"01101001..."` (Thue-Morse sequence), actual random bits.
- Verify: the logistic map at $\mu = 4$ produces high-complexity sequences; at $\mu = 2.5$ (fixed point), the sequences compress to essentially zero.

**Connection to chaos**: For the doubling map $x \mapsto 2x \pmod 1$, the first $n$ binary digits of $x$ encode the itinerary of the first $n$ steps. A Martin-Löf random initial condition $x$ produces an uncompressible binary sequence — this is Fouché's theorem made computational.

---

## 6. Information Geometry: Fisher Information

### Fisher Information Matrix

For a parametric family $p(x|\theta)$, the Fisher information matrix is:
$$I(\theta)_{ij} = \mathbb{E}_\theta\left[\frac{\partial \log p(x|\theta)}{\partial \theta_i} \frac{\partial \log p(x|\theta)}{\partial \theta_j}\right].$$

```
def fisher_information(log_pdf_gradient, theta, num_samples=10000):
    """Estimate Fisher information by Monte Carlo"""
    n = len(theta)
    I = zeros((n, n))
    for _ in range(num_samples):
        x = sample_from(theta)
        grad = log_pdf_gradient(x, theta)
        I += outer(grad, grad)
    return I / num_samples
```

**Cramér-Rao bound**: For any unbiased estimator $\hat\theta$ of $\theta$ from $n$ samples:
$$\text{Var}[\hat\theta_i] \geq (I(\theta)^{-1})_{ii} / n.$$

**Experiment**: For the Gaussian family $\mathcal{N}(\mu, \sigma^2)$ with $\theta = (\mu, \sigma^2)$:
- $I(\theta) = \text{diag}(1/\sigma^2, 1/(2\sigma^4))$ (exact)
- Cramér-Rao bound: $\text{Var}[\hat\mu] \geq \sigma^2/n$ and $\text{Var}[\hat\sigma^2] \geq 2\sigma^4/n$
- Verify that the MLE $\hat\mu = \bar{X}$ and $\hat\sigma^2 = \frac{1}{n}\sum (X_i - \bar{X})^2$ achieve these bounds.

---

## 7. Entropy Rate of Dynamical Systems

### Estimating Entropy Rate from Orbits

For a dynamical system $f: [0,1] \to [0,1]$ with a partition $\mathcal{P} = \{I_1, \ldots, I_k\}$, estimate the entropy rate by the empirical entropy of the word distribution:

```
def entropy_rate_estimate(f, x0, n, partition):
    """Estimate h_mu via word frequencies"""
    from collections import defaultdict
    
    # Generate symbolic orbit
    x = x0
    orbit = []
    for k in range(n):
        # Find which partition atom x belongs to
        atom = which_atom(x, partition)
        orbit.append(atom)
        x = f(x)
    
    # Count word frequencies for different lengths L
    rates = []
    for L in range(1, 8):
        words = defaultdict(int)
        for i in range(len(orbit) - L):
            word = tuple(orbit[i:i+L])
            words[word] += 1
        total = sum(words.values())
        H_L = -sum((c/total)*log2(c/total) for c in words.values())
        rates.append(H_L / L)
    
    return rates  # should converge to h_mu(f, P)
```

**Expected results**:
- Doubling map $x \mapsto 2x \pmod 1$ with 2-partition: estimated rate $\to \log 2$
- Logistic map $f_4$ with 2-partition: estimated rate $\to \log 2$ (isomorphic to doubling map)
- Tent map: same

---

## 8. Von Neumann Entropy and Quantum States

### Density Matrices and Entropy

```
import numpy as np

def von_neumann_entropy(rho):
    """S(rho) = -Tr(rho log rho) in bits"""
    eigenvalues = np.linalg.eigvalsh(rho)
    eigenvalues = eigenvalues[eigenvalues > 1e-12]  # remove zeros
    return -np.sum(eigenvalues * np.log2(eigenvalues))

def partial_trace(rho, dim_A, dim_B, trace_over='B'):
    """Partial trace of bipartite density matrix"""
    rho_matrix = rho.reshape(dim_A, dim_B, dim_A, dim_B)
    if trace_over == 'B':
        return np.trace(rho_matrix, axis1=1, axis2=3)
    else:
        return np.trace(rho_matrix, axis1=0, axis2=2)
```

**Experiments**:
- Bell state $|\Phi^+\rangle = (|00\rangle + |11\rangle)/\sqrt{2}$: pure state ($S = 0$), but partial trace gives maximally mixed qubit ($S = 1$ bit). This is maximal entanglement.
- Product state $|01\rangle$: partial trace gives $|0\rangle\langle 0|$ with $S = 0$ — unentangled.
- Verify strong subadditivity: $S(AB) + S(BC) \geq S(B) + S(ABC)$ for random 3-qubit states.

---

## 9. Connection to the Quiz App

The quiz app generates questions about information theory. Computational notes add questions like:
- "What is the entropy of a uniform distribution over 8 symbols? Over 4 symbols? How does entropy scale with the number of symbols?"
- "For a BSC with crossover probability $p = 0.1$, what is the capacity? What is the maximum rate at which you can transmit reliably?"
- "State the AEP. If a source has entropy $H = 3$ bits/symbol and you observe $n = 1000$ symbols, approximately how many typical sequences are there?"
- "The Huffman code for distribution $(1/2, 1/4, 1/8, 1/8)$ assigns codewords of lengths $(1, 2, 3, 3)$. Verify that the average length equals the entropy."
